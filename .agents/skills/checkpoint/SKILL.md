---
name: checkpoint
description: Save current session progress and generate a handoff artifact for later resumption. Use before long breaks, context window limits, or task switching.
---

# checkpoint

Save the current session state so work can be resumed later.

## When to Use

- Context window approaching capacity
- Switching to a different task mid-sprint
- Before a long break
- When the session-compact hook fires automatically

## Steps

1. **Capture current state**
   - Active branch and recent commits
   - Uncommitted changes (`git diff --stat`)
   - Current task description and progress
   - Active Sprint Contract (if any)

2. **Generate handoff artifact**

   Create `production/session-state/checkpoint_<timestamp>.md` with:

   ```markdown
   # Checkpoint: <timestamp>

   ## Task
   <What was being worked on>

   ## Progress
   - [x] Completed items
   - [ ] Remaining items

   ## Branch State
   - Branch: <name>
   - Last commit: <hash> <message>
   - Uncommitted changes: <list>

   ## Active Sprint Contract
   <Contract details or "None">

   ## Context
   <Key decisions made, approaches tried, blockers encountered>

   ## Next Steps
   <What to do when resuming>
   ```

3. **Run session-compact hook**
   ```bash
   bash .agents/hooks/session-compact.sh
   ```

4. **Confirm to user**
   ```
   ✅ Checkpoint saved: production/session-state/checkpoint_<timestamp>.md
   Resume with the `resume` skill.
   ```

## Outputs

- `production/session-state/checkpoint_<timestamp>.md` — Full handoff artifact
- `production/session-state/compact_<timestamp>.md` — Auto-generated compact summary
