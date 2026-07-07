#!/bin/bash
# Release script for clearedforpush
# Usage: ./scripts/release.sh 0.2.0

set -euo pipefail

VERSION="${1:-}"

if [ -z "$VERSION" ]; then
  echo "Usage: $0 <version>"
  echo "Example: $0 0.2.0"
  exit 1
fi

TAG="v${VERSION}"

echo "Releasing clearedforpush ${TAG}..."

# Check we're on main
BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$BRANCH" != "main" ]; then
  echo "Error: must be on 'main' branch (currently on '${BRANCH}')"
  exit 1
fi

# Check clean working directory
if [ -n "$(git status --porcelain)" ]; then
  echo "Error: working directory is not clean"
  exit 1
fi

# Update version in Cargo.toml
sed -i "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml

# Update Cargo.lock
cargo check

# Update PKGBUILD version
sed -i "s/^pkgver=.*/pkgver=${VERSION}/" pkg/aur/PKGBUILD

# Commit version bump
git add Cargo.toml Cargo.lock pkg/aur/PKGBUILD
git commit -m "chore: bump version to ${VERSION}"

# Create annotated tag
git tag -a "${TAG}" -m "Release ${TAG}"

echo ""
echo "Done! To publish:"
echo "  git push origin main"
echo "  git push origin ${TAG}"
echo ""
echo "The release workflow will:"
echo "  1. Build binaries for all platforms"
echo "  2. Create a GitHub release with changelog"
echo "  3. Publish to crates.io"
