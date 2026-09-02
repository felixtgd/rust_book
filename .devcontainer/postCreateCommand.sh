#!/bin/bash
set -euo pipefail

# git pull for automatic intraday updates
git pull origin $(git branch --show-current)

# GitHub CLI extensions ISSUE
gh extension install https://github.com/nektos/gh-act
gh extension install github/gh-copilot

# Run dotfiles setup from mounted host repo when available
DOTFILES_INSTALL="/workspaces/.dotfiles/install.sh"
if [ -f "${DOTFILES_INSTALL}" ]; then
	bash "${DOTFILES_INSTALL}"
fi
