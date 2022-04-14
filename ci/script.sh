#!/usr/bin/env bash

set -euxo pipefail

if [ -n "${TARGET:-}" ]; then
    cargo check --target $TARGET
fi
