# Build toolchain

This runbook reproduces the CI build for the Rust workspace, Android ABI, and
Tauri shell. It deliberately builds Android on Linux: the NDK/cross-linker path
is stable there and keeps Windows CI focused on its native desktop target.

## Pinned inputs

- Rust `1.96.0`, installed through `rustup`; required components are `clippy`
  and `rustfmt`.
- Node.js `24` with npm for the Tauri frontend.
- Android SDK command-line tools plus NDK `28.2.13676358`; API 26 is the
  minimum Android level because it is a widely supported baseline and is used
  in the generated arm64 linker target.
- JDK 17 for the Android binding test. Gradle itself is never installed
  system-wide: `apps/android/` ships a pinned Gradle wrapper (`gradlew`,
  `gradlew.bat`, `gradle/wrapper/`) targeting Gradle `8.11.1`, the minimum and
  default version required by Android Gradle Plugin `8.9.x`. Always invoke
  `./gradlew`, never a `gradle` binary from `PATH` — an unpinned system
  `gradle` is not reproducible across machines or CI.

The dependency choices follow the official UniFFI binding/Gradle guidance, the
Android NDK ABI and API-target guidance, and Tauri v2 prerequisites. UniFFI is
`0.32.0`, Tauri Rust is `2.11.5`, and the Tauri CLI is `2.11.4`.

## Rust core (Windows or Linux)

From the repository root:

```powershell
rustup toolchain install 1.96.0 --component clippy --component rustfmt
cargo build --locked --workspace
cargo test --locked --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo install cargo-deny --locked
cargo deny check
```

`coblox-node --version` prints the same version supplied by `coblox-core`.

## Android arm64 and Kotlin bindings (Linux)

Install the Android SDK command-line tools, NDK `28.2.13676358`, and JDK 17.
Set `ANDROID_NDK_HOME` to the NDK directory (e.g.
`%ANDROID_HOME%\ndk\28.2.13676358` on Windows or
`$ANDROID_HOME/ndk/28.2.13676358` on Linux), then run:

```bash
rustup target add aarch64-linux-android
cargo install cargo-ndk --version 4.1.2 --locked
./scripts/build-android.sh
```

`scripts/build-android.sh` calls `cargo ndk` with `--platform 26` spelled out
in full. In cargo-ndk 4.x a short `-p` is forwarded straight to `cargo` and
read as `--package`, so `-p 26` fails with `unknown package: 26`; `--platform`
(or the short `-P`, uppercase) is the only correct spelling of the API-level
flag.

The script produces
`apps/android/core/src/main/jniLibs/arm64-v8a/libcoblox_ffi.so`, builds a host
library only for the JVM test, generates Kotlin from the UDL, and runs
`./gradlew :core:testDebugUnitTest` to execute `coreVersion()`. `./gradlew` is
the pinned wrapper checked into `apps/android/` — it downloads Gradle
`8.11.1` on first use and needs no system-wide Gradle install. The JVM test is
intentional: it verifies the actual generated binding and native FFI boundary
without an emulator; the Android ABI itself is cross-compiled separately. Do
not substitute the host library for the packaged Android `.so`.

## Tauri (Windows or Linux)

Install the current system prerequisites from the [Tauri v2 prerequisites]
(https://v2.tauri.app/start/prerequisites/): Visual Studio C++ Build Tools and
WebView2 on Windows; WebKitGTK/GTK/AppIndicator/RSVG development packages on
Ubuntu. Then:

```bash
cd apps/desktop
npm ci
npm run build
npm run tauri -- build --no-bundle
```

`--no-bundle` skips packaging entirely and compiles the app binary only; it
replaces an earlier `--bundles none`, which the Tauri v2 CLI rejects (`none`
is not a valid bundle target — Windows only accepts `msi`/`nsis`, so
`--bundles none` fails immediately with "invalid value 'none'"). The one-page
shell invokes the Rust `core_version` Tauri command and displays the returned
`0.1.0`. Bundling and signing are intentionally excluded from this
specification.

## Common failures

- **Android linker cannot be found:** confirm `ANDROID_NDK_HOME` points to the
  NDK root, not the SDK root, and rerun `cargo ndk`.
- **Linux Tauri fails in pkg-config:** install the listed `libwebkit2gtk`, GTK,
  AppIndicator, RSVG, and `patchelf` packages.
- **Windows Tauri linker errors:** select the MSVC Rust host and install the
  Desktop development with C++ workload.
- **Generated Kotlin cannot load the native library:** run the Gradle task,
  rather than a test directly; it stages the host `coblox_ffi` library and sets
  `jna.library.path`.
