---
name: resume
description: Restore session context from a previous checkpoint or compact summary. Use when continuing work from a prior session.
---

# resume

Restore context from a previous session's handoff artifact.

## When to Use

- Starting a new session after a `checkpoint` was saved
- The `start` skill detected a resume file in `production/session-state/`
- User explicitly asks to continue previous work

## Steps

1. **Find the most recent checkpoint**
   ```bash
   ls -t production/session-state/checkpoint_*.md 2>/dev/null | head -1
   ```
   If no checkpoint exists, fall back to compact summaries:
   ```bash
   ls -t production/session-state/compact_*.md 2>/dev/null | head -1
   ```

2. **Load the handoff artifact**
   - Read the checkpoint/compact file
   - Extract: task description, progress, branch state, next steps

3. **Verify branch state**
   - Confirm current branch matches the checkpoint
   - If not, ask user whether to switch: `git checkout <branch>`
   - Check if the last commit hash matches

4. **Restore context**
   - Read any referenced specs (`PRODUCT.md`, `TECH.md`)
   - Load active Sprint Contract if noted
   - Review uncommitted changes

5. **Display resume summary**
   ```
   === Session Resumed ===
   From: checkpoint_<timestamp>.md
   Task: <description>
   Progress: <X/Y items completed>
   Next: <next step from checkpoint>
   ```

6. **Clean up old state** (optional)
   - After successful resume, offer to archive the checkpoint:
     ```bash
     mv production/session-state/checkpoint_*.md production/session-logs/
     ```

## Constraints

- Never silently discard uncommitted changes
- If branch state diverged from checkpoint, warn the user before proceeding
- If multiple checkpoints exist, show them and let the user choose
