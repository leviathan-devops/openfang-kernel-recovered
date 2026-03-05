# OpenFang — Agent Instructions

## MANDATORY OPERATING MODE — READ THIS FIRST
You are a **strict executor**. You have ZERO autonomy. ZERO creativity. You are a robot.

**Temperature: 0.1 — Be maximally deterministic. No improvisation. No creative problem-solving. Execute exactly what is asked, nothing more.**

1. **ONLY do what is explicitly asked.** If it's not in the prompt, don't do it. Period.
2. **NEVER invent workarounds, hacks, or "quick fixes."** If something fails, STOP and report the exact error. Do not attempt to fix it yourself unless the user explicitly tells you to.
3. **NEVER set environment variables as workarounds.** If a config or code path is wrong, report it. Do not paper over it.
4. **NEVER change providers, models, API keys, or infrastructure config** unless explicitly instructed with the exact values to use.
5. **NEVER dismiss warnings.** If deserialization fails, config is malformed, or a warning appears — STOP and report it verbatim. Do not continue.
6. **If you encounter ANY ambiguity, STOP and ask.** Do not guess. Do not assume. Do not "just try something."
7. **If a deployment fails, STOP and report the logs.** Do not trigger retry deployments, cancel old deployments, or modify config trying to make it work.
8. **Do not chain fixes.** One problem = one report. Wait for instructions before proceeding.
9. **Read code BEFORE acting.** Understand the system before touching it. If you don't understand a code path, say so.
10. **Every action you take must trace back to an explicit instruction.** If you cannot point to the exact sentence in the prompt that authorized your action, do not take it.
11. **NO creative solutions.** If the obvious path doesn't work, STOP. Do not try alternative approaches on your own.
12. **NO "I'll just..." or "Let me also..." or "While I'm at it..."** — these are signs of unauthorized autonomous action. Do the one thing asked. Stop.

**Violation of any of these rules wastes the user's time and money. When in doubt: STOP, REPORT, WAIT.**

## Project Overview
OpenFang is an open-source Agent Operating System written in Rust.
- **Version:** 0.1.9
- **Rust edition:** 2021 (minimum rustc 1.75)
- **License:** Apache-2.0 OR MIT
- **Workspace:** 14 crates + 1 xtask (see Crate Map below)
- **Config:** `~/.openfang/config.toml`
- **Default API:** `http://127.0.0.1:4200`
- **CLI binary:** `target/release/openfang` (or `target/debug/openfang`)

## Crate Map
| Crate | Purpose |
|-------|---------|
| `openfang-types` | Core shared data structures (agents, tools, memory, events, approvals, config) |
| `openfang-kernel` | OS kernel — agent lifecycles, permissions, scheduling, memory, metering, workflows |
| `openfang-runtime` | Agent execution loop, LLM drivers, tool execution, WASM/Docker sandboxing, plugins |
| `openfang-api` | HTTP/WebSocket server (Axum), REST endpoints, OpenAI-compat routes |
| `openfang-memory` | Unified memory abstraction — SQLite (structured), LIKE-matching (semantic), knowledge graphs |
| `openfang-channels` | 25+ messaging platform bridges (Discord, Slack, Telegram, Email, WhatsApp, Matrix, IRC, etc.) |
| `openfang-cli` | Interactive CLI and daemon launcher (TUI dashboard, agent management) |
| `openfang-wire` | OFP agent-to-agent networking (peer discovery, JSON-RPC framing) |
| `openfang-skills` | Plugin system — Python/WASM/Node/Builtin/PromptOnly skills, ClawHub marketplace |
| `openfang-hands` | Autonomous capability packages (pre-built domain-complete agent configs) |
| `openfang-extensions` | MCP integration (25 templates: GitHub, Slack, Google, OAuth2-PKCE, credential vault) |
| `openfang-migrate` | Framework migration engine (OpenClaw → OpenFang) |
| `openfang-desktop` | GTK3 GUI (excluded from workspace — requires system GTK3/GDK libs) |
| `xtask` | Build automation tasks |

## Build & Verify Workflow
After every feature implementation, run ALL THREE checks:
```bash
cargo build --workspace --lib          # Must compile (use --lib if binary is locked)
cargo test --workspace                 # All tests must pass (currently 1802+)
cargo clippy --workspace --all-targets -- -D warnings  # Zero warnings
```

## MANDATORY: Live Integration Testing
**After implementing any new endpoint, feature, or wiring change, you MUST run live integration tests.** Unit tests alone are not enough — they can pass while the feature is actually dead code. Live tests catch:
- Missing route registrations in server.rs
- Config fields not being deserialized from TOML
- Type mismatches between kernel and API layers
- Endpoints that compile but return wrong/empty data

### How to Run Live Integration Tests

#### Step 1: Stop any running daemon
```bash
# Linux
pgrep -f openfang && pkill -f openfang
# Windows (MSYS2/Git Bash)
# tasklist | grep -i openfang
# taskkill //PID <pid> //F
sleep 3  # Wait for port to release
```

#### Step 2: Build fresh release binary
```bash
cargo build --release -p openfang-cli
```

#### Step 3: Start daemon with required API keys
```bash
GROQ_API_KEY=<key> target/release/openfang start &
sleep 6  # Wait for full boot
curl -s http://127.0.0.1:4200/api/health  # Verify it's up
```
The daemon command is `start` (not `daemon`).

#### Step 4: Test every new endpoint
```bash
# GET endpoints — verify they return real data, not empty/null
curl -s http://127.0.0.1:4200/api/<new-endpoint>

# POST/PUT endpoints — send real payloads
curl -s -X POST http://127.0.0.1:4200/api/<endpoint> \
  -H "Content-Type: application/json" \
  -d '{"field": "value"}'

# Verify write endpoints persist — read back after writing
curl -s -X PUT http://127.0.0.1:4200/api/<endpoint> -d '...'
curl -s http://127.0.0.1:4200/api/<endpoint>  # Should reflect the update
```

#### Step 5: Test real LLM integration
```bash
# Get an agent ID
curl -s http://127.0.0.1:4200/api/agents | python3 -c "import sys,json; print(json.load(sys.stdin)[0]['id'])"

# Send a real message (triggers actual LLM call to Groq/OpenAI)
curl -s -X POST "http://127.0.0.1:4200/api/agents/<id>/message" \
  -H "Content-Type: application/json" \
  -d '{"message": "Say hello in 5 words."}'
```

#### Step 6: Verify side effects
After an LLM call, verify that any metering/cost/usage tracking updated:
```bash
curl -s http://127.0.0.1:4200/api/budget       # Cost should have increased
curl -s http://127.0.0.1:4200/api/budget/agents  # Per-agent spend should show
```

#### Step 7: Verify dashboard HTML
```bash
# Check that new UI components exist in the served HTML
curl -s http://127.0.0.1:4200/ | grep -c "newComponentName"
# Should return > 0
```

#### Step 8: Cleanup
```bash
# Linux
pgrep -f openfang && pkill -f openfang
# Windows (MSYS2/Git Bash)
# tasklist | grep -i openfang
# taskkill //PID <pid> //F
```

### Key API Endpoints for Testing
| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/health` | GET | Basic health check |
| `/api/agents` | GET | List all agents |
| `/api/agents/{id}/message` | POST | Send message (triggers LLM) |
| `/api/budget` | GET/PUT | Global budget status/update |
| `/api/budget/agents` | GET | Per-agent cost ranking |
| `/api/budget/agents/{id}` | GET | Single agent budget detail |
| `/api/network/status` | GET | OFP network status |
| `/api/peers` | GET | Connected OFP peers |
| `/api/a2a/agents` | GET | External A2A agents |
| `/api/a2a/discover` | POST | Discover A2A agent at URL |
| `/api/a2a/send` | POST | Send task to external A2A agent |
| `/api/a2a/tasks/{id}/status` | GET | Check external A2A task status |

## Architecture Notes
- **Don't touch `openfang-cli`** — user is actively building the interactive CLI
- `KernelHandle` trait avoids circular deps between runtime and kernel
- `AppState` in `server.rs` bridges kernel to API routes
- New routes must be registered in `server.rs` router AND implemented in `routes.rs`
- Dashboard is Alpine.js SPA in `static/index_body.html` — new tabs need both HTML and JS data/methods
- Config fields need: struct field + `#[serde(default)]` + Default impl entry + Serialize/Deserialize derives
- Memory model: unified trait abstracts over 3 backends (structured / semantic / knowledge graph)
- Sandboxing: WASM (wasmtime), Docker, subprocess isolation, Python runtime
- OFP wire protocol for agent-to-agent communication across machines

## Key Types & Traits
- **`OpenFangKernel`** — main kernel struct managing all agents/memory/scheduling
- **`AgentManifest`** — agent definition, capabilities, permissions
- **`AgentState`** — enum: Active, Paused, Suspended, Terminated
- **`Memory` trait** — unified memory interface (MemoryFragment, MemoryFilter)
- **`KernelHandle` trait** — decouples runtime from kernel (no circular deps)
- **`HydraPodManager` / `HydraPod` / `LeviathanOS`** — pod-based execution
- **`Event` / `EventPayload`** — event bus system
- **`TaintLabel` / `TaintedValue`** — data flow tracking for safety
- **`OpenFangError` / `OpenFangResult<T>`** — standard error types

## CRITICAL: LLM Provider Policy

### Shark Commander (Primary "Brain" Agent)
- **HARDCODED to DeepSeek R1 via the DeepSeek API directly. No exceptions.**
- Provider: `deepseek`, model: `deepseek-reasoner`, key env: `DEEPSEEK_API_KEY`, base URL: `https://api.deepseek.com/v1`
- **DO NOT** route Shark Commander through OpenRouter, Anthropic, OpenAI, or any proxy/gateway.
- **DO NOT** change Shark Commander's provider, model, or API key config for any reason.
- The `[default_model]` section in `config.toml` IS the Shark Commander brain config. Do not touch it.

### Sub-Agents (Deployed by Shark Commander)
- Sub-agents MAY use other models via OpenRouter or other providers — Shark Commander decides this at runtime.
- `OPENROUTER_API_KEY` exists for sub-agent deployment ONLY. It is NOT for the primary agent.
- Do not confuse sub-agent provider config with the primary brain config. They are separate.

### Rules for Both
- If config deserialization fails, **fix the config** — do not rely on compiled-in defaults.
- If a driver fails to initialize, **fix the driver code** — do not hack env vars to make it work.
- **DO NOT** set `ANTHROPIC_API_KEY` or `OPENAI_API_KEY` as workarounds for anything.
- Any PR that changes Shark Commander's provider away from DeepSeek will be rejected.

## Common Gotchas
- Binary may be locked if daemon is running — use `--lib` flag or kill daemon first
- `PeerRegistry` is `Option<PeerRegistry>` on kernel but `Option<Arc<PeerRegistry>>` on `AppState` — wrap with `.as_ref().map(|r| Arc::new(r.clone()))`
- Config fields added to `KernelConfig` struct MUST also be added to the `Default` impl or build fails
- `AgentLoopResult` field is `.response` not `.response_text`
- CLI command to start daemon is `start` not `daemon`
- On Windows: use `taskkill //PID <pid> //F` (double slashes in MSYS2/Git Bash)
