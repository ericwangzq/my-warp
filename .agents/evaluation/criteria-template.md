# Evaluation Criteria Template

Use this template when evaluating an implementation against its Sprint Contract.

## Dimensions

### 1. Functional Completeness (30%)

For each testable behavior in the Sprint Contract:

| # | Behavior | Status | Notes |
|---|----------|--------|-------|
| 1 | <behavior description> | ✅ Pass / ❌ Fail / ⚠️ Partial | <details> |
| 2 | ... | ... | ... |

**Score**: ___ / 30

### 2. Code Quality (25%)

| Check | Status | Notes |
|-------|--------|-------|
| `cargo fmt` passes | ✅ / ❌ | |
| `cargo clippy` passes (no warnings) | ✅ / ❌ | |
| Exhaustive matching (no `_` wildcards) | ✅ / ❌ | |
| No nested terminal model locks | ✅ / ❌ / N/A | |
| Follows WARP.md coding style | ✅ / ❌ | |
| No unnecessary abstractions | ✅ / ❌ | |
| Unused params removed (not `_`-prefixed) | ✅ / ❌ | |
| Inline format args used | ✅ / ❌ | |
| Existing comments preserved | ✅ / ❌ | |

**Score**: ___ / 25

### 3. Test Coverage (20%)

| Check | Status | Notes |
|-------|--------|-------|
| Unit tests for new logic | ✅ / ❌ | |
| Tests in separate `_tests.rs` files | ✅ / ❌ | |
| All existing tests pass | ✅ / ❌ | |
| Edge cases covered | ✅ / ❌ | |
| Integration tests (if applicable) | ✅ / ❌ / N/A | |

**Score**: ___ / 20

### 4. Architecture (15%)

| Check | Status | Notes |
|-------|--------|-------|
| Respects crate boundaries | ✅ / ❌ | |
| Uses Entity-Handle pattern correctly | ✅ / ❌ / N/A | |
| Feature flag usage (runtime preferred) | ✅ / ❌ / N/A | |
| No circular dependencies introduced | ✅ / ❌ | |
| Platform-specific code properly gated | ✅ / ❌ / N/A | |

**Score**: ___ / 15

### 5. Safety (10%)

| Check | Status | Notes |
|-------|--------|-------|
| No hardcoded secrets | ✅ / ❌ | |
| No security regressions | ✅ / ❌ | |
| Input validation at boundaries | ✅ / ❌ / N/A | |
| Secret detection hook passes | ✅ / ❌ | |

**Score**: ___ / 10
