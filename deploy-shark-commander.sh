#!/usr/bin/env bash
# =============================================================
# SHARK COMMANDER — One-Shot Railway Deployment Script
# =============================================================
# Run this from your local machine after cloning the repo.
# Prerequisites: Railway CLI (npm i -g @railway/cli) OR use dashboard.
#
# REQUIRED: Set these env vars BEFORE running this script:
#   export DEEPSEEK_API_KEY="<your-deepseek-key>"
#   export OPENROUTER_API_KEY="<your-openrouter-key>"
#   export DISCORD_TOKEN="<your-discord-bot-token>"
#   export OPENFANG_VAULT_KEY="<your-vault-key>"
#   export RAILWAY_TOKEN="<your-railway-token>"
#
# OPTION A: Run this script (fastest)
# --------------------------------
# 1. Install Railway CLI:
#    npm i -g @railway/cli
#
# 2. Export your env vars (see above)
#
# 3. Run:
#    ./deploy-shark-commander.sh
#
# OPTION B: Railway Dashboard (manual)
# --------------------------------
# 1. Go to https://railway.app/dashboard
# 2. New Project -> Deploy from GitHub Repo
# 3. Select: leviathan-devops/openfang-kernel-recovered
# 4. Branch: claude/update-claude-md-dksAa (or merge to main first)
# 5. Railway will detect Dockerfile.leviathan via railway.toml
# 6. Go to Variables tab and add:
#    DEEPSEEK_API_KEY, OPENROUTER_API_KEY, DISCORD_TOKEN,
#    DISCORD_BOT_TOKEN, OPENFANG_VAULT_KEY, RUST_LOG=info,
#    OPENFANG_CONFIG=/app/config.toml, OPENFANG_DATA_DIR=/app/data
# 7. Deploy. Wait 8-10 min for Rust build. Check logs.
# 8. Once healthy, your Discord bot should come online.
#
# VERIFICATION:
# --------------------------------
# After deployment, check:
#   curl https://<your-railway-url>/api/health
#   curl https://<your-railway-url>/api/agents
#
# The bot should send "Shark Commander online. Ready to pilot." in Discord.
#
# =============================================================

set -euo pipefail

echo "=== Shark Commander Deployment ==="
echo ""

# Validate required env vars
for var in DEEPSEEK_API_KEY OPENROUTER_API_KEY DISCORD_TOKEN; do
    if [ -z "${!var:-}" ]; then
        echo "[!] ERROR: $var is not set. Export it before running this script."
        exit 1
    fi
done

# Check if Railway CLI is installed
if command -v railway &> /dev/null; then
    echo "[+] Railway CLI found"

    # Check if already linked to a project
    if railway status &> /dev/null 2>&1; then
        echo "[+] Already linked to a Railway project"
    else
        echo "[*] Creating new Railway project..."
        railway init --name shark-commander-brain
    fi

    echo "[*] Setting environment variables..."
    railway variables set \
        DEEPSEEK_API_KEY="$DEEPSEEK_API_KEY" \
        OPENROUTER_API_KEY="$OPENROUTER_API_KEY" \
        DISCORD_TOKEN="$DISCORD_TOKEN" \
        DISCORD_BOT_TOKEN="$DISCORD_TOKEN" \
        OPENFANG_VAULT_KEY="${OPENFANG_VAULT_KEY:-shark-commander-vault-key-2026}" \
        RUST_LOG="info" \
        OPENFANG_CONFIG="/app/config.toml" \
        OPENFANG_DATA_DIR="/app/data"

    echo "[*] Deploying to Railway..."
    railway up

    echo ""
    echo "=== Deployment initiated ==="
    echo "Check Railway dashboard for build progress (8-10 min for Rust build)"
    echo "Once live, your Shark Commander bot should appear in Discord."
else
    echo "[!] Railway CLI not found."
    echo "[!] Install it: npm i -g @railway/cli"
    echo "[!] Or use the Railway dashboard (see comments in this script)"
    exit 1
fi
