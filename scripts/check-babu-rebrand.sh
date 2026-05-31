#!/usr/bin/env bash
set -euo pipefail

pattern="$(printf 'jar%s' 'vis')"
failed=0

if git ls-files | grep -i -- "$pattern"; then
  echo "Found old assistant name in tracked file or folder paths." >&2
  failed=1
fi

if git grep -I -n -i -- "$pattern" -- .; then
  echo "Found old assistant name in tracked text content." >&2
  failed=1
fi

if (( failed )); then
  exit 1
fi

echo "Babu rebrand check passed: no tracked paths or text contain the old assistant name."
