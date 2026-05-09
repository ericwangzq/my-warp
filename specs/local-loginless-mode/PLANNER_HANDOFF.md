# Planner Handoff

## Request

Remove Warp official account login restrictions from this fork so normal local usage does not require any Warp official backend account.

## Product Spec

See `specs/local-loginless-mode/PRODUCT.md`.

## Planner Interpretation

The safe product boundary is a local-loginless fork mode:

- Local functionality should run without Warp official account login.
- Official Warp cloud/account features should be hidden, disabled, or gracefully unavailable.
- The implementation must not bypass Warp official server-side authorization, billing, entitlements, quotas, private APIs, team membership, or cloud service access controls.

## Current Code Surfaces Found

Primary identity/auth surfaces:

- `app/src/auth/auth_state.rs`
  - `AuthState::initialize` chooses test user, API key, `WARP_USER_SECRET`, persisted Firebase user, or logged-out state.
  - `AuthState::should_use_test_user` already treats `test`, `skip_login`, and integration channel as test-user modes.
  - `AuthState::is_logged_in`, `is_anonymous_or_logged_out`, `is_anonymous_user_feature_gated`, and `is_anonymous_user_past_object_limit` drive many account gates.

- `app/src/auth/auth_manager.rs`
  - Owns login, device authorization, Firebase user refresh, anonymous user creation, account linking, persistence, and post-login side effects.
  - Emits `AttemptedLoginGatedFeature` when gated features are used.

- `app/src/auth/login_slide.rs`
  - Handles onboarding login flow and "login later".
  - `FeatureFlag::SkipFirebaseAnonymousUser` already avoids creating anonymous Firebase users and emits `SkippedLogin`.

- `app/src/auth/auth_view_body.rs`
  - Uses `FeatureFlag::ForceLogin` to decide whether loginless flow is allowed.

- `app/src/bin/local.rs`
  - Local channel enables debug, dogfood, and preview feature sets, but does not force login.

- `app/src/bin/preview.rs`
  - Preview channel explicitly enables `FeatureFlag::ForceLogin`.

Known gate surfaces:

- `app/src/workspace/action.rs`
  - `WorkspaceAction::blocked_for_anonymous_user` blocks team drive and session sharing actions.

- `app/src/workspace/view.rs`
  - Contains account prompt routing, anonymous signup/sign-in buttons, reauth prompts, and login-gated action handling.

- `app/src/drive/index.rs`
  - Blocks anonymous users from share/team actions.
  - Checks anonymous object limits before creating local-looking Drive objects.

- `app/src/ai/blocklist/agent_view/agent_input_footer/mod.rs`
  - Disables remote-control/session-sharing chip when logged out or anonymous.

- `app/src/ai/blocklist/prompt/prompt_alert.rs`
  - Shows anonymous request-limit soft/hard gates.

- `app/src/settings_view/privacy_page.rs`, `app/src/settings/initializer.rs`, `app/src/settings/cloud_preferences_syncer.rs`
  - Privacy and cloud preference flows may fetch or sync account-scoped settings.

Backend/client surfaces:

- `crates/warp_server_client/src/auth`
- `app/src/server/server_api`
- `app/src/server/sync_queue.rs`
- `app/src/server/cloud_objects/update_manager.rs`
- `app/src/workspaces/update_manager.rs`

## Proposed Sprint Contract

### Scope

1. Introduce an explicit local-loginless fork mode or default local identity path.
2. Make fresh startup enter a usable local workspace without account, Firebase, anonymous user, or official backend login.
3. Remove or reroute login prompts from default local flows.
4. Disable or hide official Warp cloud/account features that cannot work without official backend authorization.
5. Preserve local persistence and user-owned third-party credential flows.
6. Add focused tests around identity state and representative UI gates.

### Non-scope

1. Do not implement fake official cloud access.
2. Do not bypass server-side entitlements, billing, quotas, or team membership.
3. Do not remove unrelated third-party auth flows such as AWS, GitHub, SSH, MCP, or user-provided AI provider credentials.
4. Do not do a broad visual redesign of account-related UI beyond removing login prompts from local-loginless mode.

### Acceptance Criteria

1. A clean local run does not show any Warp official login, signup, anonymous signup, login-later, or reauth modal before the user can use a terminal.
2. Local settings, local terminal sessions, local code editor, and local user-owned AI/provider settings remain usable without an official account.
3. The app does not call official account creation, user fetch, token refresh, account linking, SSO linking, or login notification endpoints during normal local startup.
4. Account-gated official cloud actions do not open login UI. They are hidden or show an unavailable-in-this-build message.
5. Anonymous-user quotas do not block local object creation.
6. The user menu/avatar area does not imply an official Warp account exists.
7. Existing tests that depend on test users still pass or are updated to target explicit local-loginless/test identity behavior.
8. At least one focused test verifies `AuthState` local-loginless semantics.
9. At least one focused UI/model test verifies that a representative login-gated action no longer opens login UI in local-loginless mode.

## Suggested Specialist Split

- `architect-lead`: decide whether local-loginless mode should be a compile-time cargo feature, runtime feature flag, channel behavior, or fork-default identity model.
- `rust-core-dev`: adjust auth state semantics, feature flags/channel setup, and tests.
- `ui-dev`: remove/reroute login prompts, user menu account affordances, onboarding login surfaces, and gated feature messaging.
- `infra-dev`: disable account backend calls and cloud sync pollers in local-loginless mode without breaking local persistence.
- `ai-agent-dev`: preserve local/user-provided AI flows while disabling official cloud agent, cloud conversations, shared remote-control, and anonymous request-limit gates.

## Key Risks

1. Treating local user as fully logged in may accidentally enable official cloud actions that still fail server-side. Prefer explicit local identity semantics over globally returning `true` for `is_logged_in`.
2. Reusing `User::test()` in production could leak test identifiers into telemetry, cloud requests, or persistence. If used as a bridge, it should be guarded and not represent an official account.
3. Removing auth prompts without disabling dependent background pollers may create retry loops against official APIs.
4. Drive and workspace objects mix local persistence with cloud sync assumptions; implementation should identify which object types are truly local before removing gates.
5. The app has multiple login-like flows. Do not remove SSH/AWS/GitHub/MCP/provider credential prompts that are unrelated to Warp official accounts.

## Recommended Next Step

Ask `architect-lead` for a TECH.md decision on the identity model before generator implementation. The main architectural question is whether this fork should:

1. Add a new explicit `LocalLoginless` identity variant/state.
2. Reuse existing `skip_login` behavior in default local builds.
3. Add a new fork-specific feature flag that disables official account flows and cloud-backed features.

The planner recommendation is option 1 or 3, not plain reuse of `User::test()`, because the product behavior needs to distinguish local identity from a real authenticated Warp account.
