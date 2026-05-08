#!/usr/bin/env bash
# Hook: session-start
# Initializes context at the beginning of an agent session.
# Outputs a structured summary for the agent to consume.

set -euo pipefail

PROJECT_ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$PROJECT_ROOT"

echo "=== Session Start ==="
echo "Project: $(basename "$PROJECT_ROOT")"
echo "Date: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo ""

# Git status
echo "--- Git Status ---"
BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
echo "Branch: $BRANCH"

DIRTY=$(git status --porcelain 2>/dev/null | wc -l | tr -d ' ')
echo "Uncommitted changes: $DIRTY files"

if [[ "$DIRTY" -gt 0 ]]; then
  echo "Modified files:"
  git status --porcelain 2>/dev/null | head -10
  if [[ "$DIRTY" -gt 10 ]]; then
    echo "  ... and $((DIRTY - 10)) more"
  fi
fi
echo ""

# Check for active sprint contracts
echo "--- Active Session State ---"
if [[ -d "production/session-state" ]]; then
  STATE_FILES=$(find production/session-state -name "*.json" -o -name "*.md" 2>/dev/null | wc -l | tr -d ' ')
  if [[ "$STATE_FILES" -gt 0 ]]; then
    echo "Active state files: $STATE_FILES"
    find production/session-state -name "*.json" -o -name "*.md" 2>/dev/null | head -5
  else
    echo "No active session state."
  fi
else
  echo "No session state directory."
fi
echo ""

# Recent specs
echo "--- Recent Specs ---"
if [[ -d "specs" ]]; then
  RECENT_SPECS=$(find specs -name "PRODUCT.md" -o -name "TECH.md" 2>/dev/null | head -5)
  if [[ -n "$RECENT_SPECS" ]]; then
    echo "$RECENT_SPECS"
  else
    echo "No specs found."
  fi
else
  echo "No specs directory."
fi

echo ""
echo "=== Session Ready ==="
