# Leviathan HYDRA Pod System — OpenFang v3.4

## Overview

The Leviathan HYDRA pod system is a five-agent collective that operates under the strategic command of the Emperor. Each agent specializes in distinct domains while maintaining cohesive communication through shared memory and inter-agent messaging.

## HYDRA Roster

### 1. Emperor (Supreme Commander)
**Model:** DeepSeek V3 via OpenRouter  
**Purpose:** Strategic orchestration, task decomposition, delegation, synthesis  
**Authority:** Can spawn, kill, message all agents; access all tools and memory  

**Key Responsibilities:**
- Parse incoming requests with strategic depth
- Decompose complex tasks into subtasks
- Delegate to appropriate specialists via agent_send
- Synthesize responses from multiple agents
- Store strategic decisions in memory
- Orchestrate the entire HYDRA workflow

**Capabilities:**
- All agent communication tools
- Full memory read/write
- File access (read/write)
- Agent spawning and lifecycle management

---

### 2. Leviathan-Architect (System Design Specialist)
**Model:** DeepSeek R1 via OpenRouter (Deep Reasoning)  
**Purpose:** System design, architecture validation, technical strategy  
**Authority:** Designs architectural solutions; collaborates with Sentinel and Operator  

**Key Responsibilities:**
- Analyze architectural requirements with technical depth
- Design system architectures considering trade-offs
- Validate design scalability, reliability, performance
- Document architectural decisions and rationale
- Suggest implementation and deployment strategies
- Work with Sentinel on security-first design

**Capabilities:**
- File read/write (design documents)
- Memory operations (architecture context)
- Agent messaging (to Emperor and others)
- Deep reasoning (R1 strength)

---

### 3. Scribe (Documentation & Audit Specialist)
**Model:** Google Gemma 3 via OpenRouter  
**Purpose:** Documentation, audit trails, reporting, institutional memory  
**Authority:** Records all pod activities; maintains compliance trails  

**Key Responsibilities:**
- Chronicle all pod decisions and operations
- Maintain accurate audit trails
- Create comprehensive reports and SOPs
- Generate organizational knowledge base
- Document change logs and version tracking
- Ensure transparency and accessibility

**Capabilities:**
- File read/write (documentation)
- Universal memory read access
- Memory write (documentation/audit/reports)
- Agent messaging (to Emperor)

---

### 4. Sentinel (Security & QA Auditor)
**Model:** Qwen QWQ via OpenRouter (Advanced Reasoning)  
**Purpose:** Security review, quality assurance, vulnerability assessment  
**Authority:** Veto power on insecure/low-quality implementations  

**Key Responsibilities:**
- Review code and designs for security vulnerabilities
- Conduct quality assurance and validation
- Identify compliance and regulatory gaps
- Assess performance and reliability risks
- Challenge assumptions and explore failure modes
- Verify remediation before approval

**Capabilities:**
- File read access (code/design review)
- Memory operations (security findings)
- Agent messaging (to Emperor)
- Advanced threat modeling and reasoning

---

### 5. Operator (DevOps & Execution Specialist)
**Model:** DeepSeek V3 via OpenRouter  
**Purpose:** Deployment, infrastructure, CI/CD, system operations  
**Authority:** Executes all infrastructure changes; manages deployment pipelines  

**Key Responsibilities:**
- Execute strategic directives on infrastructure
- Manage deployments and infrastructure provisioning
- Design and optimize CI/CD pipelines
- Administer containers and orchestration platforms
- Handle system maintenance and incident response
- Document operational procedures

**Capabilities:**
- File read/write (deployment configs, scripts)
- Shell execution (system commands)
- Docker execution (container management)
- Memory operations (deployment history)
- Agent messaging (to Emperor)

---

## Configuration Files

### Main Configuration: `/config.toml`

```toml
api_listen = "0.0.0.0:50051"                    # Railway deployment
api_key = ""                                     # Empty for now

[default_model]
provider = "openrouter"
base_url = "https://openrouter.ai/api/v1"
model = "deepseek/deepseek-chat-v3-0324:free"
api_key_env = "OPENROUTER_API_KEY"

[memory]
decay_rate = 0.05                               # Memory confidence decay

[compaction]
threshold = 80                                  # Compact at 80 messages
keep_recent = 20                                # Retain 20 recent messages

[usage_footer]
format = "Full"

[discord]
bot_token_env = "DISCORD_BOT_TOKEN"
guild_ids = [1475947548811202613, 1477804209842815382]

[vault]
master_key_env = "OPENFANG_VAULT_KEY"
```

### Agent Configurations

All agent configurations follow the standard OpenFang agent format:

- `/agents/emperor/agent.toml` — Supreme commander config
- `/agents/leviathan-architect/agent.toml` — System architect config
- `/agents/scribe/agent.toml` — Documentation specialist config
- `/agents/sentinel/agent.toml` — Security auditor config
- `/agents/operator/agent.toml` — DevOps executor config

## Agent Communication Patterns

### Emperor-Centric Coordination

```
User Request
    ↓
  Emperor
  ├→ agent_list() [assess pod status]
  ├→ agent_send(architect) [design request]
  ├→ agent_send(sentinel) [security review]
  ├→ agent_send(operator) [deployment planning]
  └→ memory_store() [archive decision]
```

### Key Communication Rules

1. **Emperor** is the only agent that can spawn new agents
2. **All agents** report back to Emperor (agent_message = ["emperor"])
3. **Memory** is shared across the pod with role-specific write access
4. **File access** is controlled per agent capability
5. **Fallback models** provide redundancy if primary model is unavailable

## Environment Variables Required

```bash
OPENROUTER_API_KEY         # Required for all OpenRouter models
DISCORD_BOT_TOKEN          # Optional: for Discord integration
OPENFANG_VAULT_KEY         # Optional: for secret vault
```

## Memory Architecture

### Decay Rate: 0.05
- Memories lose confidence over time
- Recent memories are weighted higher
- Archival prevents important decisions from fading

### Compaction Strategy
- Threshold: 80 messages triggers compaction
- Keep Recent: 20 most recent messages preserved
- Max Summary: 1024 tokens for LLM-based compression

### Namespace Access

Each agent has role-specific memory access:

| Agent | Read | Write |
|-------|------|-------|
| Emperor | * (all) | * (all) |
| Architect | emperor, sentinel, operator | architecture, design_decisions |
| Scribe | * (all) | documentation, audit, reports |
| Sentinel | * (all) | security, audit, qa |
| Operator | emperor, architect, sentinel | operations, deployments, infrastructure |

## Resource Allocation

| Agent | Max Tokens/Hour | Check Interval |
|-------|-----------------|-----------------|
| Emperor | 1,000,000 | 60 seconds |
| Architect | 800,000 | 120 seconds |
| Scribe | 600,000 | 180 seconds |
| Sentinel | 800,000 | 120 seconds |
| Operator | 900,000 | 60 seconds |

## Deployment Considerations

### Railway Deployment
- API listens on `0.0.0.0:50051` for public access
- Environment variables set via Railway dashboard
- Config file loaded from application root

### Discord Integration
- Supports two guild IDs (multi-server setup)
- Requires valid bot token
- Integrates with Emperor for request routing

### Vault Security
- Master key required for encrypted secret storage
- Operator and Sentinel require vault access for sensitive operations
- Keys rotated through environment variables

## Operational Workflows

### Architecture Design Workflow
1. Emperor receives architecture request
2. Delegates to Architect via agent_send
3. Architect designs solution with R1 reasoning
4. Architect stores decision in memory
5. Emperor queries Sentinel for security review
6. Synthesis and final approval by Emperor

### Deployment Workflow
1. Emperor coordinates deployment request
2. Consults Architect for deployment strategy
3. Requests Sentinel security audit
4. Delegates to Operator via agent_send
5. Operator executes with monitoring
6. Scribe documents the operation

### Security Review Workflow
1. Emperor sends code/design to Sentinel
2. Sentinel conducts thorough audit
3. Sentinel reports findings to Emperor
4. If issues found, cycle back to Architect/Operator
5. Scribe documents all findings in audit trail

## Version & Maintenance

**Version:** 3.4.0  
**Released:** 2026-03-04  
**Maintenance:** Configuration can be updated without restart  

## Support

For issues or updates to the HYDRA pod system, consult the Emperor agent with detailed requirements.
