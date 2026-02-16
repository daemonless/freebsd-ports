#!/bin/sh
#
# dbuild CI wrapper for Woodpecker
# Locates dbuild and runs the full pipeline: build → test → push → sbom
#
# Usage: sh dbuild-ci.sh [VARIANT]
#
# Version pinning (in .woodpecker.yaml):
#   /v2/dbuild-ci.sh       → latest v2.x (recommended)
#   /v2.0.0/dbuild-ci.sh   → exact version
#   /main/dbuild-ci.sh     → development
#
# Env vars:
#   DBUILD_REF   - override the dbuild version to fetch (default: v2)
#   DBUILD_PATH  - explicit path to dbuild checkout (skips fetch)
#   GITHUB_TOKEN - registry authentication
#   GITHUB_ACTOR - registry username (optional)
#

set -e

DBUILD_REF="${DBUILD_REF:-v2}"

# ── Locate dbuild ────────────────────────────────────────────────────

if [ -n "$DBUILD_PATH" ] && [ -d "$DBUILD_PATH/dbuild" ]; then
    DBUILD_DIR="$DBUILD_PATH"
elif [ -d "../dbuild/dbuild" ]; then
    DBUILD_DIR="$(cd ../dbuild && pwd)"
else
    echo "Fetching dbuild ${DBUILD_REF}..."
    mkdir -p /tmp/dbuild
    fetch -qo /tmp/dbuild.tar.gz \
        "https://github.com/daemonless/dbuild/archive/${DBUILD_REF}.tar.gz"
    tar -xzf /tmp/dbuild.tar.gz -C /tmp/dbuild --strip-components=1
    DBUILD_DIR="/tmp/dbuild"
fi

export PYTHONPATH="$DBUILD_DIR"
DBUILD="python3 -m dbuild"

echo "=== dbuild CI ==="
echo "dbuild ref:  $DBUILD_REF"
echo "dbuild path: $DBUILD_DIR"
$DBUILD --version

# ── Variant filter (optional) ────────────────────────────────────────

VARIANT_FLAG=""
if [ -n "$1" ]; then
    VARIANT_FLAG="--variant $1"
fi

# ── Pipeline ─────────────────────────────────────────────────────────

# Build
$DBUILD -v build $VARIANT_FLAG

# Test (capture exit code so artifacts are preserved on failure)
cit_exit=0
$DBUILD -v test $VARIANT_FLAG --json cit-result.json || cit_exit=$?

if [ $cit_exit -ne 0 ]; then
    echo "Tests failed with exit code $cit_exit -- skipping push and sbom"
    exit $cit_exit
fi

# Push (push.py handles CI detection and PR skip)
$DBUILD -v push $VARIANT_FLAG

# SBOM
$DBUILD -v sbom $VARIANT_FLAG

echo "=== dbuild CI complete ==="
