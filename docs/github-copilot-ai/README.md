# GitHub Copilot AI Integration Index

This directory documents the local GitHub Copilot subscription integration added for Warp OSS/debug development.

The goal of the integration is to let a user connect a GitHub Copilot subscription with GitHub OAuth device authorization, then route Warp Agent chat completions through GitHub Copilot's OpenAI-compatible endpoint.

## Read These First

- [AI architecture and routing](../../app/src/ai/GITHUB_COPILOT_ARCHITECTURE.md)
- [Settings UI and OAuth behavior](../../app/src/settings_view/GITHUB_COPILOT_OAUTH_SETTINGS.md)

## Code Map

| Area | File | Responsibility |
| --- | --- | --- |
| OAuth and Copilot API client | `app/src/ai/github_copilot_client.rs` | Starts GitHub device OAuth, stores/reads GitHub OAuth tokens, exchanges GitHub tokens for short-lived Copilot API tokens, calls Copilot chat completions. |
| Model list and preference behavior | `app/src/ai/llms.rs` | Defines `LLMProvider::GitHubCopilot`, Copilot model IDs, Copilot-only model choices, and persists the user's last selected model into the active execution profile. |
| Agent routing | `app/src/ai/agent/api/impl.rs` | Detects `github-copilot/*` model IDs and bypasses Warp's multi-agent server path with a local Copilot chat completion response stream. |
| AI gate | `app/src/settings/ai.rs` | Allows AI features when Warp AI is enabled and either the user is logged in or a local GitHub Copilot OAuth token exists. |
| Settings UI | `app/src/settings_view/ai_page.rs` | Adds the GitHub Copilot OAuth row, device-code display, copy button, connect/disconnect actions, and model-list refresh after connect/disconnect. |

## Current Product Behavior

- Settings -> AI Provider shows a `GitHub Copilot OAuth` connection row in the API Keys section.
- Connecting starts GitHub device OAuth and opens the browser.
- The device code is displayed inside Warp and has a `Copy` button.
- In debug/OSS local development, the GitHub OAuth token is stored in Warp's local state directory, not macOS Keychain.
- After connecting, `/model` exposes GitHub Copilot routed models only. Non-Copilot Warp server/BYOK models are intentionally hidden in this local subscription mode because they do not route through the Copilot token.
- Selecting a model in one session persists it to the active execution profile, so newly-created sessions inherit the last selected model.

## Verification

Fast local checks:

```sh
cargo fmt --check
cargo check -p warp
cargo test -p warp ai::github_copilot_client -- --nocapture
```

Manual real Copilot check:

```sh
unset GITHUB_TOKEN
cargo test -p warp ai::github_copilot_client::tests::manual_real_copilot_chat_completion -- --ignored --nocapture
```

The manual test starts device OAuth if no `GITHUB_TOKEN` exists.

## External Reference

The model list is based on GitHub's supported Copilot model documentation:

https://docs.github.com/en/copilot/reference/ai-models/supported-models

GitHub notes that model availability can vary by plan, client, and organization policy. For that reason, adding a model to Warp's picker does not guarantee the user's GitHub account can call it.
