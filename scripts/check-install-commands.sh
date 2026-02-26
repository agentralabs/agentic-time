#!/usr/bin/env bash
# check-install-commands.sh — Validate install commands across README, docs, and installer
set -euo pipefail

fail() {
  echo "ERROR: $*" >&2
  exit 1
}

find_fixed() {
  if command -v rg >/dev/null 2>&1; then
    rg -nF "$1" "$2"
  else
    grep -n -F -- "$1" "$2"
  fi
}

assert_contains() {
  local pattern="$1"
  local file="$2"
  find_fixed "$pattern" "$file" >/dev/null || fail "Missing pattern '${pattern}' in ${file}"
}

# README must contain install commands
assert_contains 'curl -fsSL https://agentralabs.tech/install/time' README.md
assert_contains 'npm install @agenticamem/agentic-time' README.md
assert_contains 'pip install agentic-time' README.md

# Installation doc must match
assert_contains 'curl -fsSL https://agentralabs.tech/install/time' docs/public/installation.md
assert_contains 'npm install @agenticamem/agentic-time' docs/public/installation.md

# Quickstart must match
assert_contains 'curl -fsSL https://agentralabs.tech/install/time' docs/public/quickstart.md

echo "Install command guardrails passed."
