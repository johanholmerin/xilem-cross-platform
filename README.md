# xilem cross-platform

Example setup for using xilem on android, iOS, and desktop

### Instructions

## iOS

> Note: may not work on the Apple iOS Simulator due to lack of capabilites
> [\*](https://developer.apple.com/documentation/metal/developing_metal_apps_that_run_in_simulator#3241608)

```sh
rustup target add aarch64-apple-ios

open xilem_example_mobile.xcodeproj
# Configure signing

# Build
```

### Android

```sh
rustup target add aarch64-linux-android armv7-linux-androideabi
cargo install cargo-apk

cargo apk run -p xilem_example_mobile
```

### Desktop

> Note: only tested on macOS but should work on other systems as well

```sh
cargo run --manifest-path=./desktop/Cargo.toml
```

## Architecture

- The android build uses [cargo-apk](https://github.com/rust-mobile/cargo-apk)
  which, although it's deprecated, is the most stable way of building rust
  projects into android apps. It's configured in `mobile/Cargo.toml` and
  calls the `android_main` function from the `mobile` crate.
- iOS also uses the `mobile` crate but compiles is a library(using `cdylib`)
  which is then called by an Objective-C file in `ios-src`. There's an XCode
  project in `xilem_example_mobile.xcodeproj` that includes the required files
  and which calls `build_rust_deps.sh` during build to compile the rust code.
- There's a separate crate for the desktop build since cargo-apk fails if the
  mobile project contains a binary target. It's a normal cargo project
- The application logic is a separate project so that it can be imported
  by both the mobile and desktop crates. It's based on the xilem mason example.

The setup should work with winit directly and would only require removing xilem
from the app crate and instead creating the event loop and window directly.

## References

- https://github.com/bevyengine/bevy/tree/main/examples/mobile
- https://github.com/ryanmcgrath/cacao/tree/trunk/examples/ios-beta
- https://github.com/linebender/vello#android
