---
name: start
description: Initialize an agent session by loading project context, checking git status, and displaying current work state. Use at the beginning of any coding session.
---

# start

Initialize a new agent session with full project context.

## When to Use

Run this at the start of every coding session to:
- Orient yourself in the project
- Check for uncommitted work or in-progress tasks
- Load any active Sprint Contracts
- Review recent spec activity

## Steps

1. **Run session-start hook**
   ```bash
   bash .agents/hooks/session-start.sh
   ```

2. **Check for resume state**
   - Look in `production/session-state/` for any `compact_*.md` files
   - If found, ask the user if they want to resume the previous session
   - If yes, use the `resume` skill instead

3. **Load project rules**
   - Read `AGENTS.md` for project conventions
   - Read `WARP.md` for engineering guidelines
   - Scan `.agents/rules/` for active rules

4. **Check active work**
   - List open branches: `git branch --sort=-committerdate | head -10`
   - Check for in-progress specs: `find specs -name "PRODUCT.md" -newer $(git log -1 --format=%ci HEAD) 2>/dev/null`
   - Check for active Sprint Contracts in `production/session-state/`

5. **Display session summary**
   ```
   === Session Ready ===
   Branch: <current-branch>
   Uncommitted: <N> files
   Active contract: <yes/no>
   Resume available: <yes/no>
   ```

6. **Ask for task**
   - Prompt the user for what they want to work on
   - If a Sprint Contract is active, suggest continuing that work
