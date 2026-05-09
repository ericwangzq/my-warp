# Copilot Instructions

## Agent Role Routing

When the user mentions one of the following role names, read the corresponding agent definition from `.agents/agents/<name>.md` and adopt that role's responsibilities, constraints, and workflow:

- **planner** → `.agents/agents/planner.md`
- **generator** → `.agents/agents/generator.md`
- **evaluator** → `.agents/agents/evaluator.md`
- **architect-lead** → `.agents/agents/architect-lead.md`
- **rust-core-dev** → `.agents/agents/rust-core-dev.md`
- **ui-dev** → `.agents/agents/ui-dev.md`
- **ai-agent-dev** → `.agents/agents/ai-agent-dev.md`
- **infra-dev** → `.agents/agents/infra-dev.md`

## Project Rules

Always follow the rules in `.agents/rules/`:
- `exhaustive-matching.md` — No `_` wildcards in match statements
- `terminal-model-locking.md` — Never nest `model.lock()` calls
- `no-secrets.md` — Never hardcode secrets
- `ui-reuse.md` — Reuse existing UI components and themes

## Skills

When the user asks to start a session, save progress, resume work, or negotiate a sprint contract, read the corresponding skill from `.agents/skills/<name>/SKILL.md`.

## References

- Project conventions: `AGENTS.md`
- Engineering guide: `WARP.md`
- UI guidelines: `.agents/skills/warp-ui-guidelines/SKILL.md`
