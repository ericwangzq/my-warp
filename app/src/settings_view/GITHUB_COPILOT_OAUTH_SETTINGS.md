# GitHub Copilot OAuth Settings UI

This document explains the Settings-side implementation for GitHub Copilot OAuth.

## Entry Points

The UI is implemented in `app/src/settings_view/ai_page.rs`.

Key types and actions:

- `GitHubCopilotOAuthStatus`
- `AISettingsPageAction::ConnectGitHubCopilot`
- `AISettingsPageAction::DisconnectGitHubCopilot`
- `AISettingsPageAction::CopyGitHubCopilotDeviceCode`
- `render_github_copilot_oauth_row()`
- `render_github_copilot_device_code_row()`

The row is rendered inside `ApiKeysWidget::render_api_keys_section()`.

## Connect Flow

1. User clicks `Connect`.
2. Settings dispatches `ConnectGitHubCopilot`.
3. The view creates `GitHubCopilotOAuth`.
4. The OAuth client calls GitHub's device authorization endpoint.
5. Warp opens either `verification_uri_complete` or `verification_uri`.
6. Warp displays the `user_code` in the settings row.
7. User can click `Copy` to copy the device code.
8. Warp polls GitHub for the OAuth token.
9. On success, `write_github_oauth_token()` stores the GitHub OAuth token.
10. Settings reconciles model preferences and emits `LLMPreferencesEvent::UpdatedAvailableLLMs`.

The poll callback checks that the current UI status still matches the same `device_code`. This prevents a canceled or superseded OAuth flow from writing a token into the UI state.

## Disconnect Flow

1. User clicks `Disconnect`.
2. Settings dispatches `DisconnectGitHubCopilot`.
3. `remove_github_oauth_token()` deletes the token.
4. Settings resets status to `Idle`.
5. LLM preferences are reconciled so selected models can be cleared if needed.

During an in-progress device flow, the same action is used as `Cancel`. The background poll may still finish, but its result is ignored if the UI status no longer matches the original device code.

## Copy Device Code

GitHub device OAuth requires the user to enter an 8-character code in the browser. The code is displayed in monospace text and has an explicit `Copy` button:

```rust
ctx.clipboard().write(ClipboardContent::plain_text(code))
```

Do not rely on text selection for this code. Warp UI text elements are not guaranteed to be selectable in settings rows.

## Storage And Keychain

For GitHub Copilot OAuth token storage:

- Debug/OSS uses local state: `paths::state_dir()/github_copilot_oauth_token`.
- Non-debug/non-OSS uses Warp secure storage.

This avoids local-development Keychain prompts for the Copilot token. It does not change every other secure-storage use in Warp.

If a developer still sees a Keychain prompt after building a local debug/OSS bundle, first check whether the prompt is coming from a generic Warp secure-storage caller rather than the Copilot token. Repeated local code signing can invalidate macOS Keychain ACL trust for existing items, which is different from the Copilot token path.

## AI Enablement

`app/src/settings/ai.rs` treats local GitHub Copilot OAuth as sufficient to enable local AI entry points when Warp AI is otherwise enabled. This is necessary for local subscription mode because the user may not be logged into Warp, but still has a valid GitHub Copilot OAuth token.

Do not remove the regular logged-in check for Warp cloud AI. The Copilot bypass is specifically for the local GitHub Copilot route.

## UI Style Notes

The row follows existing Settings UI patterns:

- Uses `ui_builder().button(ButtonVariant::Secondary, ...)`
- Uses existing settings text/description styles
- Does not introduce custom one-off button themes
- Keeps actions in the same API Keys section where provider credentials live

If the UI needs to expand, prefer adding more state to `GitHubCopilotOAuthStatus` over introducing a separate modal. The current device-flow UX is intentionally inline with the existing settings page.
