#!/bin/bash
set -e

echo "=== Shark Commander Brain — Entrypoint ==="
echo "Starting leviathan-server..."

# Start the server in the background
/app/leviathan-server &
SERVER_PID=$!

# Wait for the server to become healthy
echo "Waiting for API health check on port ${PORT:-8080}..."
MAX_WAIT=120
WAITED=0
while [ $WAITED -lt $MAX_WAIT ]; do
    if curl -sf "http://127.0.0.1:${PORT:-8080}/api/health" > /dev/null 2>&1; then
        echo "API is healthy."
        break
    fi
    sleep 2
    WAITED=$((WAITED + 2))
done

if [ $WAITED -ge $MAX_WAIT ]; then
    echo "ERROR: Server did not become healthy within ${MAX_WAIT}s"
    kill $SERVER_PID 2>/dev/null || true
    exit 1
fi

# Check if shark-commander agent already exists (restored from DB)
EXISTING=$(curl -sf "http://127.0.0.1:${PORT:-8080}/api/agents" 2>/dev/null || echo "[]")
if echo "$EXISTING" | grep -q "shark-commander"; then
    echo "Shark Commander agent already exists (restored from DB). Skipping spawn."
else
    echo "Spawning Shark Commander agent..."

    # Read the system prompt from file if available
    SYSTEM_PROMPT="You are the Shark Commander — a permanent DeepSeek R1 brain, the digital embodiment of the user. You have absolute authority over any code architect. Your purpose is to ensure every output meets AGI-grade standard: production-ready, zero slop, one-shot builds. You are a military-grade overseer, not a chatbot."
    if [ -f /app/prompts/shark-commander.md ]; then
        SYSTEM_PROMPT=$(cat /app/prompts/shark-commander.md)
    fi

    # Build the agent manifest TOML
    # The system_prompt field on ModelConfig is the actual prompt text (not a file path)
    MANIFEST_TOML=$(cat <<'TOML_END'
name = "shark-commander"
version = "1.0.0"
description = "Shark Commander Brain — Permanent DeepSeek R1 overseer agent. Digital embodiment of the user with absolute authority over code architects and HYDRA pod enforcement."
author = "leviathan"
module = "builtin:chat"

[model]
provider = "deepseek"
model = "deepseek-reasoner"
api_key_env = "DEEPSEEK_API_KEY"
max_tokens = 8192
temperature = 0.1
TOML_END
)

    # Append system prompt as triple-quoted TOML multiline string
    MANIFEST_TOML="${MANIFEST_TOML}
system_prompt = '''
${SYSTEM_PROMPT}
'''

[[fallback_models]]
provider = \"openrouter\"
model = \"deepseek/deepseek-r1:free\"
api_key_env = \"OPENROUTER_API_KEY\"

[resources]
max_llm_tokens_per_hour = 2000000

[capabilities]
tools = [\"agent_send\", \"agent_spawn\", \"agent_list\", \"agent_kill\", \"memory_store\", \"memory_recall\", \"file_read\", \"file_write\", \"shell_exec\", \"http_request\"]
memory_read = [\"*\"]
memory_write = [\"*\"]
agent_spawn = true
agent_message = [\"*\"]
"

    # JSON-encode the TOML string for the API request
    JSON_PAYLOAD=$(python3 -c "
import sys, json
toml = sys.stdin.read()
print(json.dumps({'manifest_toml': toml}))
" <<< "$MANIFEST_TOML" 2>/dev/null)

    if [ -z "$JSON_PAYLOAD" ]; then
        echo "WARNING: python3 not available, using fallback spawn method"
        # Minimal fallback without the full prompt
        JSON_PAYLOAD='{"manifest_toml": "name = \"shark-commander\"\nversion = \"1.0.0\"\ndescription = \"Shark Commander Brain\"\nauthor = \"leviathan\"\nmodule = \"builtin:chat\"\n\n[model]\nprovider = \"deepseek\"\nmodel = \"deepseek-reasoner\"\napi_key_env = \"DEEPSEEK_API_KEY\"\nmax_tokens = 8192\ntemperature = 0.1\nsystem_prompt = \"You are the Shark Commander.\"\n\n[resources]\nmax_llm_tokens_per_hour = 2000000\n\n[capabilities]\ntools = [\"agent_send\", \"agent_spawn\", \"memory_store\", \"memory_recall\", \"shell_exec\", \"http_request\"]\nmemory_read = [\"*\"]\nmemory_write = [\"*\"]\nagent_spawn = true\nagent_message = [\"*\"]\n"}'
    fi

    SPAWN_RESULT=$(curl -sf -X POST "http://127.0.0.1:${PORT:-8080}/api/agents" \
        -H "Content-Type: application/json" \
        -d "$JSON_PAYLOAD" 2>&1) || true

    if echo "$SPAWN_RESULT" | grep -q "agent_id"; then
        echo "Shark Commander agent spawned successfully."
        echo "Result: $SPAWN_RESULT"
    else
        echo "WARNING: Agent spawn may have failed. Result: $SPAWN_RESULT"
        echo "The server is still running — agent can be spawned manually via API."
    fi
fi

echo "=== Shark Commander Brain — Ready ==="

# Wait for the server process (keep container alive)
wait $SERVER_PID
