use std::fs;

use serde::{Deserialize, Serialize};
use warp_core::{
    channel::{Channel, ChannelState},
    paths,
};
use warp_graphql::scalars::time::ServerTimestamp;
use warpui::AppContext;
use warpui_extras::secure_storage::{self, AppContextExt};

use crate::auth::{
    user::{AnonymousUserType, FirebaseAuthTokens, PersonalObjectLimits, UserMetadata},
    UserUid,
};

const USER_STORAGE_KEY: &str = "User";
const USER_DEV_STORAGE_FILE: &str = "persisted_user";

/// Helper function to set `true` as the default for a serde field on PersistedUser.
fn default_as_true() -> bool {
    true
}

/// The persisted representation of a user, serialized to/from the user's keychain.
/// This struct must remain backwards compatible with the existing keychain JSON format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedUser {
    /// Information about the user's authentication through Firebase.
    #[serde(rename = "id_token")]
    pub auth_tokens: FirebaseAuthTokens,
    /// DO NOT USE! This used to be one of the two places we stored the user's Firebase refresh
    /// token. Now, all callers should go through `auth_tokens` to access it.
    #[serde(default)]
    #[deprecated = "use auth_tokens.refresh_token instead"]
    pub refresh_token: String,
    /// The Firebase UID of this user.
    pub local_id: UserUid,
    /// Metadata about the user.
    #[serde(flatten)]
    pub metadata: UserMetadata,
    /// Whether or not the user is onboarded.
    #[serde(default = "default_as_true")]
    pub is_onboarded: bool,
    /// Whether or not the user needs to link their account via SSO due to an organization setting.
    #[serde(default)]
    pub needs_sso_link: bool,
    /// What type of anonymous user this user is. May be `None` if they are not anonymous.
    #[serde(default)]
    pub anonymous_user_type: Option<AnonymousUserType>,
    #[serde(default)]
    pub linked_at: Option<ServerTimestamp>,
    #[serde(default)]
    pub personal_object_limits: Option<PersonalObjectLimits>,
    /// Whether or not this user is on what we consider a "work" domain.
    #[serde(default)]
    pub is_on_work_domain: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum UserPersistenceError {
    /// The persisted user was not successfully read from or written to disk.
    #[error("secure storage error")]
    SecureStorageError(#[from] secure_storage::Error),

    #[error("local user storage error")]
    LocalStorageError(#[from] std::io::Error),

    /// The persisted user on disk could not be decoded into a valid struct.
    #[error("failed to serialize or deserialize a PersistedUser struct")]
    SerializationError(#[from] serde_json::Error),
}

impl PersistedUser {
    pub fn from_secure_storage(ctx: &AppContext) -> Result<PersistedUser, UserPersistenceError> {
        if use_local_dev_storage() {
            let value = fs::read_to_string(local_dev_storage_path())?;
            return Ok(serde_json::from_str::<PersistedUser>(&value)?);
        }

        let value = ctx.secure_storage().read_value(USER_STORAGE_KEY)?;
        Ok(serde_json::from_str::<PersistedUser>(&value)?)
    }

    pub fn write_to_secure_storage(&self, ctx: &AppContext) -> Result<(), UserPersistenceError> {
        let serialized_user = serde_json::to_string(self)?;
        if use_local_dev_storage() {
            let path = local_dev_storage_path();
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            return Ok(fs::write(path, serialized_user)?);
        }

        Ok(ctx
            .secure_storage()
            .write_value(USER_STORAGE_KEY, &serialized_user)?)
    }

    pub fn remove_from_secure_storage(ctx: &AppContext) -> Result<(), UserPersistenceError> {
        if use_local_dev_storage() {
            return Ok(match fs::remove_file(local_dev_storage_path()) {
                Ok(()) => (),
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => (),
                Err(err) => return Err(err.into()),
            });
        }

        Ok(ctx.secure_storage().remove_value(USER_STORAGE_KEY)?)
    }
}

fn use_local_dev_storage() -> bool {
    (cfg!(debug_assertions) && !cfg!(test)) || ChannelState::channel() == Channel::Oss
}

fn local_dev_storage_path() -> std::path::PathBuf {
    paths::state_dir().join(USER_DEV_STORAGE_FILE)
}

#[cfg(test)]
#[path = "user_persistence_test.rs"]
mod tests;
