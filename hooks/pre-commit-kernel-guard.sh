#!/bin/bash
# OpenFang Kernel Guard — Pre-commit Hook
# Prevents accidental kernel deletion or mass file removal
# Install: cp hooks/pre-commit-kernel-guard.sh .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit

set -e

echo "🔒 OpenFang Kernel Guard — Pre-commit Check"

# Count staged deletions of .rs files
DELETED_RS=$(git diff --cached --diff-filter=D --name-only | grep '\.rs$' | wc -l)
TOTAL_RS=$(find . -name "*.rs" -not -path "./.git/*" | wc -l)

if [ "$DELETED_RS" -gt 10 ]; then
    echo "❌ BLOCKED: $DELETED_RS Rust files staged for deletion"
    echo "   This exceeds the safety threshold of 10 files per commit."
    echo "   If this is intentional, use: git commit --no-verify"
    echo "   WARNING: Mass Rust file deletion previously destroyed 202,096 lines of this kernel."
    exit 1
fi

# Check if more than 20% of Rust files are being deleted
if [ "$TOTAL_RS" -gt 0 ]; then
    PERCENT=$((DELETED_RS * 100 / TOTAL_RS))
    if [ "$PERCENT" -gt 20 ]; then
        echo "❌ BLOCKED: $PERCENT% of Rust files staged for deletion ($DELETED_RS of $TOTAL_RS)"
        echo "   Maximum allowed: 20% per commit"
        exit 1
    fi
fi

# Check for addition of large non-Rust replacement files
ADDED_PY=$(git diff --cached --diff-filter=A --name-only | grep '\.py$' | wc -l)
if [ "$ADDED_PY" -gt 3 ]; then
    echo "⚠️ WARNING: $ADDED_PY Python files being added to Rust kernel repo"
    echo "   This pattern matches the previous kernel replacement attack."
    echo "   Proceed with caution. Use --no-verify to override."
    exit 1
fi

echo "✅ Kernel Guard: Commit approved ($DELETED_RS deletions, $TOTAL_RS total .rs files)"
