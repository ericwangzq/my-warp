// This client is exposed for future local/provider routing, but Warp's built-in Agent currently
// sends requests through the multi-agent server path.
#![allow(dead_code)]

use std::{env, fs, sync::Arc, time::Duration};

use chrono::{DateTime, TimeZone, Utc};
use parking_lot::Mutex;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use warp_core::{
    channel::{Channel, ChannelState},
    paths,
};
use warpui::AppContext;
use warpui_extras::secure_storage::{self, AppContextExt as _};

const GITHUB_COPILOT_OAUTH_STORAGE_KEY: &str = "github_copilot_oauth_token";
const GITHUB_COPILOT_OAUTH_DEV_STORAGE_FILE: &str = "github_copilot_oauth_token";
const GITHUB_COPILOT_OAUTH_CLIENT_ID: &str = "01ab8ac9400c4e429b23";
const DEFAULT_GITHUB_DEVICE_CODE_URL: &str = "https://github.com/login/device/code";
const DEFAULT_GITHUB_ACCESS_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const DEFAULT_COPILOT_TOKEN_URL: &str = "https://api.github.com/copilot_internal/v2/token";
const DEFAULT_COPILOT_CHAT_COMPLETIONS_URL: &str = "https://api.githubcopilot.com/chat/completions";
const DEFAULT_COPILOT_OPENAI_BASE_URL: &str = "https://api.githubcopilot.com";
const DEFAULT_COPILOT_MODEL: &str = "gpt-4o";
const TOKEN_REFRESH_SKEW_SECONDS: i64 = 60;

#[derive(Clone)]
pub struct GitHubCopilotClient {
    http_client: reqwest::Client,
    github_token: String,
    token_url: String,
    chat_completions_url: String,
    token_cache: Arc<Mutex<Option<CachedCopilotToken>>>,
}

#[derive(Debug, Clone)]
struct CachedCopilotToken {
    token: String,
    expires_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct CopilotTokenResponse {
    token: String,
    expires_at: serde_json::Value,
}

#[derive(Clone)]
pub struct GitHubCopilotOAuth {
    http_client: reqwest::Client,
    client_id: String,
    device_code_url: String,
    access_token_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubDeviceAuthorization {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    #[serde(default)]
    pub verification_uri_complete: Option<String>,
    pub expires_in: u64,
    #[serde(default = "default_device_flow_interval")]
    pub interval: u64,
}

#[derive(Debug, Deserialize)]
struct GitHubAccessTokenResponse {
    access_token: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest {
    #[serde(default = "default_model")]
    pub model: String,
    pub messages: Vec<ChatCompletionMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: Option<String>,
    pub model: Option<String>,
    pub choices: Vec<ChatCompletionChoice>,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChoice {
    pub index: Option<u32>,
    pub message: Option<ChatCompletionResponseMessage>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponseMessage {
    pub role: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum GitHubCopilotClientError {
    #[error("GITHUB_TOKEN is not set")]
    MissingGitHubToken,

    #[error("invalid request header: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    #[error("failed to exchange GitHub token for Copilot token: HTTP {status}: {body}")]
    TokenExchangeFailed {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("Copilot token response did not include a valid expires_at value: {0}")]
    InvalidTokenExpiry(String),

    #[error("GitHub Copilot OAuth failed: {0}")]
    OAuth(String),

    #[error("Copilot chat completion request failed: HTTP {status}: {body}")]
    ChatCompletionFailed {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

impl GitHubCopilotClient {
    pub fn from_secure_storage(ctx: &AppContext) -> Result<Self, GitHubCopilotClientError> {
        let github_token = read_github_oauth_token(ctx)?;
        Self::new(github_token)
    }

    pub fn from_env() -> Result<Self, GitHubCopilotClientError> {
        let github_token =
            env::var("GITHUB_TOKEN").map_err(|_| GitHubCopilotClientError::MissingGitHubToken)?;
        Self::new(github_token)
    }

    pub fn from_local_dev_storage_or_env() -> Result<Self, GitHubCopilotClientError> {
        if use_local_dev_storage() {
            if let Ok(github_token) = read_github_oauth_token_from_local_dev_storage() {
                return Self::new(github_token);
            }
        }
        Self::from_env()
    }

    pub fn new(github_token: String) -> Result<Self, GitHubCopilotClientError> {
        Self::new_with_urls(
            github_token,
            DEFAULT_COPILOT_TOKEN_URL.to_string(),
            DEFAULT_COPILOT_CHAT_COMPLETIONS_URL.to_string(),
        )
    }

    fn new_with_urls(
        github_token: String,
        token_url: String,
        chat_completions_url: String,
    ) -> Result<Self, GitHubCopilotClientError> {
        Ok(Self {
            http_client: reqwest::Client::builder()
                .default_headers(default_headers()?)
                .build()?,
            github_token,
            token_url,
            chat_completions_url,
            token_cache: Arc::new(Mutex::new(None)),
        })
    }

    pub fn default_model() -> &'static str {
        DEFAULT_COPILOT_MODEL
    }

    /// Base URL to use with OpenAI-compatible clients when they can provide
    /// per-request dynamic authorization headers.
    pub fn openai_compatible_base_url(&self) -> &'static str {
        DEFAULT_COPILOT_OPENAI_BASE_URL
    }

    pub async fn get_openai_compatible_auth_token(
        &self,
    ) -> Result<String, GitHubCopilotClientError> {
        self.copilot_token().await
    }

    pub async fn chat_completion(
        &self,
        messages: Vec<ChatCompletionMessage>,
    ) -> Result<ChatCompletionResponse, GitHubCopilotClientError> {
        self.chat_completion_with_request(&ChatCompletionRequest {
            model: Self::default_model().to_string(),
            messages,
            temperature: None,
            stream: None,
        })
        .await
    }

    pub async fn chat_completion_with_request<R>(
        &self,
        request: &R,
    ) -> Result<ChatCompletionResponse, GitHubCopilotClientError>
    where
        R: Serialize + ?Sized,
    {
        self.chat_completion_json(request).await
    }

    pub async fn chat_completion_json<R, T>(
        &self,
        request: &R,
    ) -> Result<T, GitHubCopilotClientError>
    where
        R: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let token = self.copilot_token().await?;
        let response = self
            .http_client
            .post(&self.chat_completions_url)
            .bearer_auth(token)
            .json(request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GitHubCopilotClientError::ChatCompletionFailed {
                status: response.status(),
                body: response.text().await.unwrap_or_default(),
            });
        }

        Ok(response.json().await?)
    }

    async fn copilot_token(&self) -> Result<String, GitHubCopilotClientError> {
        if let Some(token) = self.cached_valid_token() {
            return Ok(token);
        }

        let response = self
            .http_client
            .get(&self.token_url)
            .header(AUTHORIZATION, format!("token {}", self.github_token))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GitHubCopilotClientError::TokenExchangeFailed {
                status: response.status(),
                body: response.text().await.unwrap_or_default(),
            });
        }

        let response: CopilotTokenResponse = response.json().await?;
        let cached = CachedCopilotToken {
            token: response.token,
            expires_at: parse_expires_at(&response.expires_at)?,
        };
        let token = cached.token.clone();
        *self.token_cache.lock() = Some(cached);
        Ok(token)
    }

    fn cached_valid_token(&self) -> Option<String> {
        let cached = self.token_cache.lock().clone()?;
        let refresh_at = cached.expires_at - chrono::Duration::seconds(TOKEN_REFRESH_SKEW_SECONDS);
        (Utc::now() < refresh_at).then_some(cached.token)
    }
}

impl GitHubCopilotOAuth {
    pub fn new() -> Result<Self, GitHubCopilotClientError> {
        let client_id = env::var("GITHUB_COPILOT_OAUTH_CLIENT_ID")
            .unwrap_or_else(|_| GITHUB_COPILOT_OAUTH_CLIENT_ID.to_string());
        Self::new_with_urls(
            client_id,
            DEFAULT_GITHUB_DEVICE_CODE_URL.to_string(),
            DEFAULT_GITHUB_ACCESS_TOKEN_URL.to_string(),
        )
    }

    fn new_with_urls(
        client_id: String,
        device_code_url: String,
        access_token_url: String,
    ) -> Result<Self, GitHubCopilotClientError> {
        Ok(Self {
            http_client: reqwest::Client::builder()
                .default_headers(default_headers()?)
                .build()?,
            client_id,
            device_code_url,
            access_token_url,
        })
    }

    pub async fn start_device_authorization(
        &self,
    ) -> Result<GitHubDeviceAuthorization, GitHubCopilotClientError> {
        let response = self
            .http_client
            .post(&self.device_code_url)
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("scope", "user:email"),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GitHubCopilotClientError::OAuth(format!(
                "failed to start device flow: HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        Ok(response.json().await?)
    }

    pub async fn wait_for_access_token(
        &self,
        authorization: &GitHubDeviceAuthorization,
    ) -> Result<String, GitHubCopilotClientError> {
        let started_at = Utc::now();
        let mut interval = authorization.interval.max(1);

        loop {
            if Utc::now() - started_at > chrono::Duration::seconds(authorization.expires_in as i64)
            {
                return Err(GitHubCopilotClientError::OAuth(
                    "GitHub device authorization expired".to_string(),
                ));
            }

            warpui::r#async::Timer::after(Duration::from_secs(interval)).await;

            let response = self
                .http_client
                .post(&self.access_token_url)
                .form(&[
                    ("client_id", self.client_id.as_str()),
                    ("device_code", authorization.device_code.as_str()),
                    ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                ])
                .send()
                .await?;

            if !response.status().is_success() {
                return Err(GitHubCopilotClientError::OAuth(format!(
                    "failed to poll device flow: HTTP {}: {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                )));
            }

            let token_response: GitHubAccessTokenResponse = response.json().await?;
            if let Some(access_token) = token_response.access_token {
                return Ok(access_token);
            }

            match token_response.error.as_deref() {
                Some("authorization_pending") => {}
                Some("slow_down") => interval += 5,
                Some("expired_token") => {
                    return Err(GitHubCopilotClientError::OAuth(
                        "GitHub device authorization expired".to_string(),
                    ));
                }
                Some("access_denied") => {
                    return Err(GitHubCopilotClientError::OAuth(
                        "GitHub device authorization was denied".to_string(),
                    ));
                }
                Some(error) => {
                    return Err(GitHubCopilotClientError::OAuth(format!(
                        "{error}: {}",
                        token_response.error_description.unwrap_or_default()
                    )));
                }
                None => {
                    return Err(GitHubCopilotClientError::OAuth(
                        "GitHub device flow response did not include access_token or error"
                            .to_string(),
                    ));
                }
            }
        }
    }

    pub async fn authorize(&self) -> Result<String, GitHubCopilotClientError> {
        let authorization = self.start_device_authorization().await?;
        println!(
            "Open {} and enter code: {}",
            authorization.verification_uri, authorization.user_code
        );
        if let Some(url) = authorization.verification_uri_complete.as_deref() {
            open_browser(url);
        } else {
            open_browser(&authorization.verification_uri);
        }
        self.wait_for_access_token(&authorization).await
    }
}

pub fn read_github_oauth_token(ctx: &AppContext) -> Result<String, GitHubCopilotClientError> {
    if use_local_dev_storage() {
        return read_github_oauth_token_from_local_dev_storage();
    }

    ctx.secure_storage()
        .read_value(GITHUB_COPILOT_OAUTH_STORAGE_KEY)
        .map_err(|err| match err {
            secure_storage::Error::NotFound => GitHubCopilotClientError::MissingGitHubToken,
            err => GitHubCopilotClientError::OAuth(format!("failed to read GitHub token: {err}")),
        })
}

fn read_github_oauth_token_from_local_dev_storage() -> Result<String, GitHubCopilotClientError> {
    fs::read_to_string(local_dev_storage_path()).map_err(|err| match err.kind() {
        std::io::ErrorKind::NotFound => GitHubCopilotClientError::MissingGitHubToken,
        _ => GitHubCopilotClientError::OAuth(format!("failed to read GitHub token: {err}")),
    })
}

pub fn write_github_oauth_token(
    ctx: &AppContext,
    token: &str,
) -> Result<(), GitHubCopilotClientError> {
    if use_local_dev_storage() {
        let path = local_dev_storage_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                GitHubCopilotClientError::OAuth(format!(
                    "failed to create local GitHub token directory: {err}"
                ))
            })?;
        }
        return fs::write(path, token).map_err(|err| {
            GitHubCopilotClientError::OAuth(format!("failed to store GitHub token: {err}"))
        });
    }

    ctx.secure_storage()
        .write_value(GITHUB_COPILOT_OAUTH_STORAGE_KEY, token)
        .map_err(|err| {
            GitHubCopilotClientError::OAuth(format!("failed to store GitHub token: {err}"))
        })
}

pub fn remove_github_oauth_token(ctx: &AppContext) -> Result<(), GitHubCopilotClientError> {
    if use_local_dev_storage() {
        return fs::remove_file(local_dev_storage_path()).map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => GitHubCopilotClientError::MissingGitHubToken,
            _ => GitHubCopilotClientError::OAuth(format!("failed to remove GitHub token: {err}")),
        });
    }

    ctx.secure_storage()
        .remove_value(GITHUB_COPILOT_OAUTH_STORAGE_KEY)
        .map_err(|err| match err {
            secure_storage::Error::NotFound => GitHubCopilotClientError::MissingGitHubToken,
            err => GitHubCopilotClientError::OAuth(format!("failed to remove GitHub token: {err}")),
        })
}

fn use_local_dev_storage() -> bool {
    cfg!(debug_assertions) || ChannelState::channel() == Channel::Oss
}

fn local_dev_storage_path() -> std::path::PathBuf {
    paths::state_dir().join(GITHUB_COPILOT_OAUTH_DEV_STORAGE_FILE)
}

fn default_model() -> String {
    DEFAULT_COPILOT_MODEL.to_string()
}

fn default_device_flow_interval() -> u64 {
    5
}

fn default_headers() -> Result<HeaderMap, reqwest::header::InvalidHeaderValue> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Warp"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("editor-version", HeaderValue::from_static("vscode/1.95.0"));
    headers.insert(
        "Copilot-Integration-Id",
        HeaderValue::from_static("vscode-chat"),
    );
    Ok(headers)
}

fn parse_expires_at(value: &serde_json::Value) -> Result<DateTime<Utc>, GitHubCopilotClientError> {
    match value {
        serde_json::Value::String(value) => {
            if let Ok(datetime) = DateTime::parse_from_rfc3339(value) {
                return Ok(datetime.with_timezone(&Utc));
            }

            value
                .parse::<i64>()
                .ok()
                .and_then(timestamp_to_datetime)
                .ok_or_else(|| GitHubCopilotClientError::InvalidTokenExpiry(value.clone()))
        }
        serde_json::Value::Number(value) => value
            .as_i64()
            .and_then(timestamp_to_datetime)
            .ok_or_else(|| GitHubCopilotClientError::InvalidTokenExpiry(value.to_string())),
        other => Err(GitHubCopilotClientError::InvalidTokenExpiry(
            other.to_string(),
        )),
    }
}

fn timestamp_to_datetime(timestamp: i64) -> Option<DateTime<Utc>> {
    Utc.timestamp_opt(timestamp, 0).single()
}

fn open_browser(url: &str) {
    #[cfg(target_os = "macos")]
    let command = ("open", url);
    #[cfg(target_os = "windows")]
    let command = ("cmd", &format!("/C start {url}"));
    #[cfg(all(unix, not(target_os = "macos")))]
    let command = ("xdg-open", url);

    let _ = std::process::Command::new(command.0).arg(command.1).spawn();
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Matcher, Server};
    use serde_json::json;

    #[tokio::test]
    async fn exchanges_github_token_and_sends_chat_completion_request() {
        let mut server = Server::new_async().await;
        let expires_at = Utc::now() + chrono::Duration::minutes(5);
        let token_mock = server
            .mock("GET", "/copilot_internal/v2/token")
            .match_header(AUTHORIZATION.as_str(), "token ghu_test")
            .with_status(200)
            .with_body(
                json!({ "token": "copilot-token", "expires_at": expires_at.to_rfc3339() })
                    .to_string(),
            )
            .create_async()
            .await;
        let chat_mock = server
            .mock("POST", "/chat/completions")
            .match_header(AUTHORIZATION.as_str(), "Bearer copilot-token")
            .match_header("editor-version", "vscode/1.95.0")
            .match_header("Copilot-Integration-Id", "vscode-chat")
            .match_body(Matcher::PartialJson(json!({
                "model": "gpt-4o",
                "messages": [{"role": "user", "content": "hello"}]
            })))
            .with_status(200)
            .with_body(
                json!({
                    "id": "chatcmpl-test",
                    "model": "gpt-4o",
                    "choices": [{
                        "index": 0,
                        "message": {"role": "assistant", "content": "hi"},
                        "finish_reason": "stop"
                    }]
                })
                .to_string(),
            )
            .create_async()
            .await;

        let client = GitHubCopilotClient::new_with_urls(
            "ghu_test".to_string(),
            format!("{}/copilot_internal/v2/token", server.url()),
            format!("{}/chat/completions", server.url()),
        )
        .unwrap();

        let response = client
            .chat_completion(vec![ChatCompletionMessage {
                role: "user".to_string(),
                content: "hello".to_string(),
            }])
            .await
            .unwrap();

        assert_eq!(
            response.choices[0]
                .message
                .as_ref()
                .unwrap()
                .content
                .as_deref(),
            Some("hi")
        );
        token_mock.assert_async().await;
        chat_mock.assert_async().await;
    }

    #[tokio::test]
    async fn reuses_cached_token_until_it_is_near_expiry() {
        let mut server = Server::new_async().await;
        let expires_at = Utc::now() + chrono::Duration::minutes(5);
        let token_mock = server
            .mock("GET", "/copilot_internal/v2/token")
            .with_status(200)
            .with_body(
                json!({ "token": "cached-token", "expires_at": expires_at.timestamp() })
                    .to_string(),
            )
            .expect(1)
            .create_async()
            .await;
        let chat_mock = server
            .mock("POST", "/chat/completions")
            .match_header(AUTHORIZATION.as_str(), "Bearer cached-token")
            .with_status(200)
            .with_body(json!({ "choices": [] }).to_string())
            .expect(2)
            .create_async()
            .await;

        let client = GitHubCopilotClient::new_with_urls(
            "ghu_test".to_string(),
            format!("{}/copilot_internal/v2/token", server.url()),
            format!("{}/chat/completions", server.url()),
        )
        .unwrap();
        let request = ChatCompletionRequest {
            model: "gpt-4o".to_string(),
            messages: vec![ChatCompletionMessage {
                role: "user".to_string(),
                content: "hello".to_string(),
            }],
            temperature: None,
            stream: None,
        };

        let _: ChatCompletionResponse = client.chat_completion_json(&request).await.unwrap();
        let _: ChatCompletionResponse = client.chat_completion_json(&request).await.unwrap();

        token_mock.assert_async().await;
        chat_mock.assert_async().await;
    }

    #[test]
    fn parses_numeric_and_rfc3339_expirations() {
        let timestamp = Utc::now().timestamp();
        assert_eq!(
            parse_expires_at(&json!(timestamp)).unwrap().timestamp(),
            timestamp
        );

        let rfc3339 = "2026-05-06T12:34:56Z";
        assert_eq!(
            parse_expires_at(&json!(rfc3339)).unwrap().to_rfc3339(),
            "2026-05-06T12:34:56+00:00"
        );
    }

    #[tokio::test]
    #[ignore = "requires a real GitHub OAuth token in GITHUB_TOKEN and calls GitHub Copilot"]
    async fn manual_real_copilot_chat_completion() {
        let client = match GitHubCopilotClient::from_env() {
            Ok(client) => client,
            Err(GitHubCopilotClientError::MissingGitHubToken) => {
                let oauth = GitHubCopilotOAuth::new().unwrap();
                let github_token = oauth.authorize().await.unwrap();
                GitHubCopilotClient::new(github_token).unwrap()
            }
            Err(err) => panic!("{err}"),
        };
        let response = client
            .chat_completion(vec![ChatCompletionMessage {
                role: "user".to_string(),
                content: "Reply with exactly: OK".to_string(),
            }])
            .await
            .unwrap();

        println!("{response:#?}");
        let content = response
            .choices
            .first()
            .and_then(|choice| choice.message.as_ref())
            .and_then(|message| message.content.as_deref())
            .unwrap_or_default();
        assert!(
            content.contains("OK"),
            "expected Copilot response to contain OK, got: {content:?}"
        );
    }
}
