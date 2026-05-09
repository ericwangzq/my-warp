# Local Loginless Mode

## Summary

This feature removes Warp official account login as a requirement for this fork. A user should be able to launch, onboard, configure, and use the local terminal, editor, local AI workflows, local agent workflows, and local persistence without signing in to Warp's official backend.

The fork should not pretend to have access to Warp-hosted account, billing, team, sharing, or cloud services. Any feature that inherently depends on Warp's official backend should be disabled, hidden, or presented as unavailable in local-loginless mode rather than prompting the user to log in.

## Goals / Non-goals

1. Goal: the default build of this fork starts in a usable local identity state with no Warp account, Firebase account, anonymous Firebase account, or official backend login flow required.

2. Goal: local-first features remain available without account prompts: terminal sessions, panes, tabs, settings, themes, local notebooks/workflows where supported locally, local env var collections where supported locally, local code editing, local project explorer, local command history, and local agent/AI surfaces that use local or user-provided credentials.

3. Goal: all UI that currently says "Log in", "Sign in", "Sign up", "requires login", "anonymous user limit", or similar official account messaging is removed from default local-loginless flows unless it refers to a third-party provider the user explicitly configured, such as AWS, GitHub, OpenAI-compatible providers, or other external tools.

4. Goal: official Warp backend features fail closed and clearly: the app should not attempt to create, refresh, link, or restore a Warp official user account as part of normal startup or normal local usage.

5. Non-goal: bypassing Warp official server-side authorization, billing, team membership, entitlement checks, paid quotas, or private cloud APIs. If a feature only works by calling official Warp services with a real authorized account, that feature is outside local-loginless mode.

6. Non-goal: removing third-party authentication flows unrelated to Warp official accounts. AWS CLI login, GitHub auth, SSH auth, MCP server auth, API-key based custom model providers, and similar user-owned integrations may keep their existing credential flows.

## Behavior

1. On a fresh install or clean profile, the app opens directly into a usable local workspace without showing an account creation, account login, Firebase anonymous signup, login-later confirmation, or forced-login modal.

2. The default identity state is "local user". The local user is considered sufficiently identified for local-only app behavior, but it is not treated as a Warp official account and does not produce Firebase credentials or a Warp backend user token.

3. The local user is considered onboarded by default unless the app still needs a non-account onboarding flow for local preferences. Any onboarding that remains must focus on local setup choices and must not ask the user to create or link a Warp official account.

4. Account prompts must not appear when the user:
   - Starts the app.
   - Opens a new terminal session, pane, or tab.
   - Opens settings.
   - Changes local settings, theme, keybindings, or privacy preferences.
   - Creates or uses local notebooks, workflows, prompts, or env var collections where those objects are available locally.
   - Uses local agent or local AI functionality with user-provided credentials.
   - Reopens the app after restart.

5. Existing persisted official Warp credentials, if present from an earlier build, must not be required for startup. The app may ignore them, preserve them unused, or offer a user-initiated cleanup path, but normal local-loginless behavior must not depend on them.

6. No normal startup path should call official Warp account endpoints for:
   - Firebase anonymous user creation.
   - OAuth/device authorization.
   - Firebase token refresh.
   - Current user fetch.
   - Login notification.
   - Account linking.
   - SSO linking.

7. The app may still call non-account endpoints only when the user explicitly enables a feature that needs them and understands that the feature is cloud-backed. Such calls must not happen as a hidden prerequisite for local terminal use.

8. Any feature that requires a real Warp official account should be disabled or hidden in local-loginless mode. Examples include official team drives, shared sessions backed by Warp services, cloud object sharing, cloud conversations, official cloud agents, official account billing, official account profile management, and organization-specific SSO workflows.

9. When a disabled official cloud feature is still visible because removing it would harm navigation or discoverability, activating it should show a concise local-fork message explaining that the feature requires Warp-hosted services and is unavailable in this build. It must not route the user into a login flow.

10. The user menu/avatar area must not present the current local user as an official Warp account. It should either show a local profile affordance or omit account identity UI entirely.

11. Log out should not be a primary action in local-loginless mode because there is no official account session to end. If any reset action remains, it should be framed as clearing local data or local credentials, not logging out of Warp.

12. Anonymous-user quota prompts and anonymous-user object limits must not apply to the local user. Local objects should not be blocked because an anonymous Firebase user exceeded server-provided limits.

13. Local persistence must remain stable across restarts. The user should not lose local settings, local sessions, local notebooks/workflows/env var collections, or local AI configuration merely because there is no official account.

14. Privacy settings should remain local. Telemetry and crash-reporting choices should be respected according to local settings, but the app should not fetch official per-account privacy settings as part of startup.

15. Error messages for local features must not tell the user to sign in to Warp as the recovery path. Recovery text should describe the actual local problem, such as missing local credentials, unavailable network, unsupported cloud feature, or disabled provider.

16. Third-party provider auth remains provider-specific. For example, if AWS Bedrock credentials are missing, the UI may still ask the user to run an AWS login command because that is not Warp official account login.

17. API-key based operation remains allowed when the key is explicitly supplied by the user. The app should not use an API key to silently recreate a Warp official account dependency.

18. Tests and integration builds that currently use test users may keep test identities if they are still useful, but production local-loginless behavior should not require compiling with test-only assumptions unless the build configuration intentionally enables this fork mode.

19. If the app cannot support a cloud-backed feature without official Warp services, the correct behavior is graceful unavailability, not a partial broken flow, repeated login prompts, or background retry loops.

20. The local-loginless mode must be the default behavior for this fork's local build path. Official-login behavior, if retained for development comparison, must be opt-in and clearly separated from default local usage.
