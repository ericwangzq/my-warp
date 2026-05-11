# AGENTS.md

Agent-agnostic entry point for AI coding assistants working in this repository.

我的工作语言是中文，交流和任务交付优先使用中文。

## Project Overview

Rust-based terminal emulator with a custom UI framework (WarpUI). Cargo workspace with 60+ crates. Main binary in `app/`, UI framework in `crates/warpui/`. Cross-platform: macOS, Windows, Linux, WASM.

For full engineering details, see [WARP.md](WARP.md).

## Quick Reference

### Build & Run

```bash
cargo run                              # Build and run
./script/presubmit                     # MUST pass before any PR (fmt + clippy + tests)
cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2  # Tests
cargo fmt                              # Format
cargo clippy --workspace --all-targets --all-features --tests -- -D warnings  # Lint
```

### Key Rules

- **Exhaustive matching**: Never use `_` wildcard in match statements. List all variants explicitly.
- **Terminal model locking**: Never nest `model.lock()` calls — causes UI deadlocks. Pass locked refs down the stack.
- **No secrets**: Never hardcode tokens, keys, or credentials in source code.
- **UI reuse**: Use existing `ActionButtonTheme` impls. Do not create feature-specific button themes.
- **Context parameter naming**: Always name context params `ctx` and place them last (except when a closure is last).
- **Unused params**: Remove completely, do not prefix with `_`.
- **Inline format args**: Use `eprintln!("{message}")` not `eprintln!("{}", message)`.
- **Preserve comments**: Do not remove existing comments when making unrelated changes.
- **Local loginless boundary**: This fork's local-loginless work must not fake or bypass Warp official backend account, billing, team, cloud, sharing, or entitlement checks. See `.agents/rules/local-loginless-boundary.md`.

### Feature Flags

```rust
// Add variant to warp_core/src/features.rs
FeatureFlag::YourFlag.is_enabled()  // Prefer runtime checks over #[cfg(...)]
// Rollout: DOGFOOD_FLAGS → PREVIEW_FLAGS → RELEASE_FLAGS
```

### Testing

- Unit tests in separate `${filename}_tests.rs` files, included via `#[cfg(test)] mod tests;`
- Integration tests in `crates/integration/`
- Use `cargo nextest` for parallel execution

### Specs

Significant features require specs before implementation:

```
specs/<ticket-number>/PRODUCT.md   # User-facing behavior
specs/<ticket-number>/TECH.md      # Implementation plan
```

See `.agents/skills/spec-driven-implementation/SKILL.md` for the full workflow.

### Active Specs

Current cross-agent work:

- `specs/local-loginless-mode/PRODUCT.md` — desired user-facing behavior for making this fork usable without Warp official account login.
- `specs/local-loginless-mode/TECH.md` — architecture and implementation plan for local-loginless mode.
- `specs/local-loginless-mode/PLANNER_HANDOFF.md` — planner handoff with code surfaces, sprint contract, risks, and specialist split.

Any agent implementing, reviewing, or evaluating auth/account/cloud changes must read these files first and keep them current when behavior or architecture changes.

## Agent Architecture (Fusion)

This project uses a GAN-inspired multi-agent architecture with three layers:

```
Decision:  planner, architect-lead     → WHAT to build
Execution: generator → [domain]-dev    → HOW to build
Evaluation: evaluator                  → Grade the result
```

### Core Agents (`.agents/agents/`)

| Agent | Role |
|-------|------|
| `planner` | Decomposes requirements, produces PRODUCT.md, manages priorities |
| `generator` | Implements features, coordinates domain specialists |
| `evaluator` | Objectively grades implementation quality. Never implements what it evaluates |
| `architect-lead` | Reviews TECH.md decisions, enforces architectural consistency |

### Domain Specialists (`.agents/agents/`)

| Specialist | Scope |
|------------|-------|
| `rust-core-dev` | `crates/warp_core`, `crates/warp_terminal`, `crates/command` |
| `ui-dev` | `crates/warpui*`, `crates/ui_components`, `app/src` UI code |
| `ai-agent-dev` | `crates/ai`, `crates/warp_completer` |
| `infra-dev` | `crates/http_client`, `crates/websocket`, `crates/persistence` |

## Available Skills (`.agents/skills/`)

### Spec & Implementation
- `spec-driven-implementation` — Full spec-first workflow
- `write-product-spec` — Generate PRODUCT.md
- `write-tech-spec` — Generate TECH.md
- `implement-specs` — Build from approved specs

### Feature Lifecycle
- `add-feature-flag` / `promote-feature` / `remove-feature-flag`

### Quality
- `fix-errors` — Compilation, clippy, test failures
- `rust-unit-tests` — Write and run unit tests
- `warp-integration-test` — End-to-end integration tests
- `warp-ui-guidelines` — UI coding guidelines

### Workflow
- `create-pr` / `review-pr` / `review-pr-local`
- `diagnose-ci-failures` — CI failure analysis
- `resolve-merge-conflicts`
- `add-telemetry`

### Session Management
- `start` — Initialize session, load context, check git status
- `checkpoint` — Save progress, generate handoff artifact
- `resume` — Restore context from handoff artifact
- `sprint-contract` — Negotiate "done" criteria before implementation

## Workflow: Sprint Contract

Before implementing any substantial feature:

1. **Generator** proposes a sprint contract (scope, testable behaviors, acceptance criteria)
2. **architect-lead** reviews architectural soundness
3. **Evaluator** reviews testability of criteria
4. All three approve → implementation begins
5. After implementation → Evaluator grades against the contract

## Directory Layout

```
.agents/
├── agents/          # Agent definitions (planner, generator, evaluator, specialists)
├── skills/          # Skill definitions (SKILL.md per skill)
├── hooks/           # Lifecycle and safety hooks
├── rules/           # Automated code quality rules
└── evaluation/      # Evaluation criteria and report templates
production/
├── session-state/   # Persisted session state
└── session-logs/    # Session log archive
```
