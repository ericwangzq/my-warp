#!/usr/bin/env bash
# Hook: validate-commit
# Validates commit message format and basic content rules.

set -euo pipefail

COMMIT_MSG_FILE="${1:-}"

if [[ -z "$COMMIT_MSG_FILE" || ! -f "$COMMIT_MSG_FILE" ]]; then
  # When called without a file, read from stdin or skip
  exit 0
fi

MSG=$(cat "$COMMIT_MSG_FILE")
FIRST_LINE=$(echo "$MSG" | head -1)

# Rule 1: First line must not be empty
if [[ -z "$FIRST_LINE" ]]; then
  echo "❌ Commit message must not be empty."
  exit 1
fi

# Rule 2: First line should be <= 72 characters
if [[ ${#FIRST_LINE} -gt 72 ]]; then
  echo "⚠️  Commit subject line is ${#FIRST_LINE} chars (recommended: <= 72)."
  # Warning only, don't block
fi

# Rule 3: No WIP commits to main/master
BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
if [[ "$BRANCH" == "main" || "$BRANCH" == "master" ]]; then
  if echo "$FIRST_LINE" | grep -iqE '^(WIP|wip|fixup!|squash!)'; then
    echo "❌ WIP/fixup/squash commits are not allowed on $BRANCH."
    exit 1
  fi
fi

# Rule 4: Check for debug leftovers in message
if echo "$MSG" | grep -qE '(TODO|FIXME|HACK|XXX)'; then
  echo "⚠️  Commit message contains TODO/FIXME markers. Intentional?"
fi

exit 0
