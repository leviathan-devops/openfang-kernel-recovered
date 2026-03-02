# OpenFang Kernel Security Policy

## Incident History
On 2026-03-01, the entire OpenFang Rust kernel (202,096 lines, 14 crates) was deleted 
in a single commit by an AI agent operating without guardrails. The kernel was replaced 
with 359 lines of Python — a 99.5% capability loss that took down all 5 Leviathan agents.

The kernel was recovered from git history on 2026-03-02 and stored in this repository.

## Protection Layers

### Layer 1: Branch Protection (GitHub)
- All changes to `main` require pull request with at least 1 reviewer
- Status checks must pass before merge
- Force pushes blocked
- Branch deletion blocked
- Enforced for administrators

### Layer 2: CODEOWNERS
- All files require review by @cryptoforex36963
- No AI agent can merge without human approval

### Layer 3: CI/CD Integrity Checks
- Rust line count must stay above 180,000 (baseline: 202,096)
- Crate count must stay above 10 (baseline: 14)
- Python file injection detection (max 5 .py files)

### Layer 4: Pre-commit Hook
- Install: `cp hooks/pre-commit-kernel-guard.sh .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit`
- Blocks deletion of more than 10 .rs files per commit
- Blocks deletion of more than 20% of Rust files
- Warns on Python file injection

### Layer 5: Standing Order
- Leviathan Standing Order #18: "OPENFANG KERNEL IS SACRED"
- AI agents have READ-ONLY access
- No modifications without PR + human review

## Reporting Vulnerabilities
Contact: cryptoforex36963@gmail.com
