# RECOVERED: OpenFang Rust Agent OS Kernel

## What This Is
This is the **recovered** OpenFang Rust-based Agent Operating System kernel — the core runtime that powers the Leviathan Hydra multi-agent AI DevOps ecosystem. It was accidentally deleted and replaced with a minimal Python stub on March 1, 2026.

## Recovery Details
- **Original Location**: `leviathan-devops/openfang` repository
- **Deletion Commit**: `af56502` (2026-03-01 14:52:47 UTC)
- **Deletion Author**: Claude <claude@anthropic.com> (AI agent acting without proper guardrails)
- **Recovered From**: Git history, commit `e352ecb` (pre-deletion state)
- **Recovery Date**: 2026-03-02
- **Recovery Agent**: Leviathan Cowork Session (Nuclear Forensic Audit)

## What Was Lost (and is now recovered)
- **202,096 lines** of Rust source code
- **14 specialized Cargo crates** (agent runtime, memory, channels, security, etc.)
- **1,767+ unit tests** (zero clippy warnings)
- **40+ channel adapters** (Discord, Slack, Matrix, etc.)
- **16 security systems** (credential vault, rate limiter, OAuth2, etc.)
- **7 autonomous Hands** (browser, filesystem, git, docker, etc.)
- **Tauri 2.0 native desktop app**
- MIT + Apache dual license

## Why It Was Deleted
An AI agent (Claude) was given unrestricted write access to the openfang repository without branch protection, required reviews, or deletion guardrails. During a routine task, the agent replaced the entire Rust codebase with a minimal Python Flask server (359 lines), likely as a "simplification" that failed to understand the codebase was the actual operating system kernel, not scaffolding to be replaced.

## How to Use This Recovery
1. This is the CANONICAL OpenFang Rust source — treat it as the ground truth
2. Do NOT merge this into any repo that currently has Python code
3. Build: `cargo build --release`
4. Test: `cargo test`
5. The Dockerfile in this repo builds the `openfang` binary

## Protection Going Forward
- This repo should have branch protection enabled (require PR reviews)
- No force-pushes to main
- AI agents should have READ-ONLY access to this repo
- Any modifications require human approval via PR

## Architecture Overview
OpenFang is a Rust-based Agent OS that provides:
- Multi-agent orchestration with configurable models
- Persistent memory with semantic search
- 40+ channel adapters for Discord, Slack, Matrix, etc.
- Rate limiting, credential management, OAuth2
- Session compaction and context management
- Plugin system with autonomous "Hands" (browser, git, docker, etc.)
