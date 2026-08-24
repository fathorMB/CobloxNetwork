#!/usr/bin/env bash
set -euo pipefail

: "${ANDROID_NDK_HOME:?Set ANDROID_NDK_HOME to the Android NDK location.}"
cargo ndk -t arm64-v8a -p 26 -o apps/android/core/src/main/jniLibs build -p coblox-ffi --release
cargo build --locked -p coblox-ffi --release
(cd apps/android && gradle :core:testDebugUnitTest --no-daemon)
