---
name: sprint-contract
description: Negotiate "done" criteria before implementation begins. Use when starting any substantial feature to align Generator, architect-lead, and Evaluator on scope and acceptance criteria.
---

# sprint-contract

Negotiate a Sprint Contract that defines exactly what "done" means before any code is written.

## Why

Without upfront agreement on scope and acceptance criteria:
- Generator builds more or less than what was intended
- Evaluator grades against unclear standards
- Scope creep happens silently
- "Is it done?" becomes a subjective debate

## Contract Structure

```markdown
# Sprint Contract: <Feature Name>

## Ticket
<Linear ticket or GitHub issue reference>

## Scope
<1-3 sentence description of what will be built>

### In Scope
- <Specific deliverable 1>
- <Specific deliverable 2>

### Out of Scope
- <Explicitly excluded item 1>
- <Explicitly excluded item 2>

## Testable Behaviors
Each behavior must be objectively verifiable (yes/no, not "looks good").

1. <When X happens, Y should result>
2. <Given A, if B occurs, then C>
3. <System should handle edge case D by doing E>

## Acceptance Criteria
- [ ] All testable behaviors pass
- [ ] `cargo fmt` passes
- [ ] `cargo clippy` passes with no warnings
- [ ] All existing tests pass
- [ ] New unit tests cover added logic
- [ ] No hardcoded secrets
- [ ] Exhaustive matching used (no wildcards)

## Specialist Assignments
| Specialist | Tasks |
|-----------|-------|
| <domain>-dev | <What they implement> |

## Cross-Domain Boundaries
<Any interfaces or contracts between specialists>

## Estimated Scope
- Files changed: ~<N>
- New tests: ~<N>
- Complexity: Low / Medium / High
```

## Negotiation Workflow

1. **Generator drafts** the contract based on Planner's task list
2. **architect-lead reviews** for architectural soundness:
   - Are crate boundaries respected?
   - Are existing patterns followed?
   - Are cross-domain interfaces clean?
3. **Evaluator reviews** for testability:
   - Is every behavior objectively verifiable?
   - Are there missing edge cases?
   - Can acceptance criteria be automated?
4. **All three approve** → Contract is saved and implementation begins
5. **Scope changes** during implementation → Re-negotiate the contract

## Storage

Active contracts are stored in:
```
production/session-state/active-contract.md
```

Completed contracts are archived to:
```
production/session-logs/contract_<timestamp>.md
```

## Constraints

- A contract must exist before implementation starts for any substantial feature
- Small bug fixes and trivial changes do not need contracts
- If scope changes during implementation, the contract must be updated first
- The Evaluator grades strictly against the contract — not against "intent"
