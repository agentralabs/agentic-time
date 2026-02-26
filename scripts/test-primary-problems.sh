#!/usr/bin/env bash
# test-primary-problems.sh — Validate primary problem coverage documentation exists
set -euo pipefail

fail() {
  echo "ERROR: $*" >&2
  exit 1
}

assert_file() {
  [ -f "$1" ] || fail "Missing required file: $1"
}

find_fixed() {
  if command -v rg >/dev/null 2>&1; then
    rg -nF "$1" "$2"
  else
    grep -n -F -- "$1" "$2"
  fi
}

assert_contains() {
  find_fixed "$1" "$2" >/dev/null || fail "Missing pattern '$1' in $2"
}

# Primary problem coverage doc must exist
assert_file "docs/public/primary-problem-coverage.md"
assert_file "docs/public/initial-problem-coverage.md"

# Must contain at least the expected problem sections
assert_contains "Deadline Amnesia" docs/public/primary-problem-coverage.md
assert_contains "No Duration Tracking" docs/public/primary-problem-coverage.md
assert_contains "Schedule Blindness" docs/public/primary-problem-coverage.md

echo "Primary problem regression guardrails passed."
