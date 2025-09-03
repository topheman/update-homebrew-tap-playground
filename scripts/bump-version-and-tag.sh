#!/bin/bash

# Script to bump patch version, build, and create git commit/tag
set -e  # Exit on any error

echo "🚀 Starting version bump process..."

# Read current version from Cargo.toml
current_version=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
echo "📋 Current version: $current_version"

# Extract major, minor, and patch components
IFS='.' read -r major minor patch <<< "$current_version"

# Increment patch version
new_patch=$((patch + 1))
new_version="$major.$minor.$new_patch"

echo "📈 New version will be: $new_version"

# Update version in Cargo.toml
sed -i.bak "s/version = \"$current_version\"/version = \"$new_version\"/" Cargo.toml
echo "✅ Updated Cargo.toml to version $new_version"

# Clean up backup file
rm Cargo.toml.bak

# Build the project
echo "🔨 Building project..."
cargo build
echo "✅ Build completed successfully"

# Git operations
echo "📝 Staging changes..."
git add .

echo "💾 Committing changes..."
git commit -m "publish v$new_version"

echo "🏷️  Creating tag..."
git tag "$new_version"

echo "🎉 Version bump completed successfully!"
echo "   Version: $current_version → $new_version"
echo "   Commit: $(git rev-parse --short HEAD)"
echo "   Tag: $new_version"
