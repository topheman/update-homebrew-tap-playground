#!/bin/bash

# Script to create a GitHub release if it doesn't exist
# Usage: ./create-release-if-not-exist.sh <tag>
#
# This script works in any context (local, CI/CD, GitHub Actions, etc.)
# Exit codes:
#   0: Success (release exists or was created)
#   1: Error (invalid usage, GitHub CLI not available, or creation failed)

set -e

# Check if tag is provided
if [ $# -eq 0 ]; then
    echo "Error: Tag is required"
    echo "Usage: $0 <tag> [additional_flags...]"
    echo "Example: $0 v1.0.0 --draft --generate-notes"
    exit 1
fi

TAG="$1"
shift  # Remove the first argument (tag), leaving any additional flags
# Use "$@" to preserve quoted arguments properly
ADDITIONAL_FLAGS=("$@")

# Check if GitHub CLI is available
if ! command -v gh &> /dev/null; then
    echo "Error: GitHub CLI (gh) is not installed or not in PATH"
    echo "Please install GitHub CLI: https://cli.github.com/"
    exit 1
fi

# Check if authenticated with GitHub
if ! gh auth status &> /dev/null; then
    echo "Error: Not authenticated with GitHub CLI"
    echo "Please run: gh auth login"
    exit 1
fi

echo "Checking if release '$TAG' exists..."

# Check if release exists
echo "DEBUG: Running: gh release view '$TAG'"
if gh release view "$TAG" >/dev/null 2>&1; then
    echo "Release found."
    echo "Status: EXISTS"
    exit 0
else
    echo "Release not found. Creating release draft..."
    echo "DEBUG: Running: gh release create '$TAG' ${ADDITIONAL_FLAGS[*]}"
    if gh release create "$TAG" "${ADDITIONAL_FLAGS[@]}"; then
        echo "Release draft created successfully"
        echo "Status: CREATED"
        exit 0
    else
        echo "Error: Failed to create release draft"
        exit 1
    fi
fi
