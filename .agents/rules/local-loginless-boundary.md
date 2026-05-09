# Rule: Local Loginless Boundary

This fork is adding local-loginless behavior so the local app can be used without a Warp official backend account. Keep this as a local product mode, not an authorization bypass.

## Rules

1. Do not fake a Warp official account, Firebase account, team membership, billing state, entitlement, cloud quota, or server permission.
2. Do not bypass Warp official server-side authorization checks. If a feature requires official Warp backend access, disable it, hide it, or show a local-unavailable message.
3. Do not reuse `User::test()` as a production local-loginless identity. Local identity must be distinguishable from an official account.
4. Do not make `AuthState::is_logged_in()` mean "local user exists" unless the auth architecture spec is updated and approved. It currently means official credentials exist.
5. Preserve unrelated third-party credential flows. AWS, GitHub, SSH, MCP, BYOK, and provider-specific auth prompts are not Warp official account login.
6. Keep local functionality available where it can operate without official backend services: terminal, local settings, local persistence, local editor, and local or user-owned AI/provider workflows.
7. Gate official cloud/account surfaces through explicit capability helpers rather than scattered ad hoc checks.
8. Keep `specs/local-loginless-mode/PRODUCT.md` and `specs/local-loginless-mode/TECH.md` current when behavior, identity semantics, capability gating, or validation strategy changes.

## Required Reading

Before implementing or reviewing local-loginless work, read:

- `specs/local-loginless-mode/PRODUCT.md`
- `specs/local-loginless-mode/TECH.md`
- `specs/local-loginless-mode/PLANNER_HANDOFF.md`

## Examples

```rust
// BAD: treats local mode as a real official account and may enable cloud paths.
if auth_state.is_logged_in() {
    open_share_dialog();
}

// GOOD: checks whether official cloud capability is actually available.
if official_warp_cloud_enabled(ctx) {
    open_share_dialog();
} else {
    show_local_unavailable_message(ctx);
}
```

## Source

`specs/local-loginless-mode/PRODUCT.md` and `specs/local-loginless-mode/TECH.md`
