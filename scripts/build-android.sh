#!/usr/bin/env bash
set -euo pipefail

: "${ANDROID_NDK_HOME:?Set ANDROID_NDK_HOME to the Android NDK location.}"
# NOTE: `--platform` must be spelled out. cargo-ndk 4.x forwards a short `-p`
# straight through to `cargo` as `--package`, so `-p 26` used to be silently
# read as "build the package named 26" and panic with `unknown package: 26`.
cargo ndk -t arm64-v8a --platform 26 -o apps/android/core/src/main/jniLibs build -p coblox-ffi --release
cargo build --locked -p coblox-ffi --release
(cd apps/android && ./gradlew :core:testDebugUnitTest --no-daemon)
