# GroPilot Showcase
The GroPilot showcase app with Rust integration!

### Prerequisites
The Android Studio and Android NDK.

### Rusty-stuff
If you wanna work with Rust, talk to Nimesh! Otherwise ignore this section!

You NEED Linux and Rust (2024 edition) v1.85 at least. You need to install aarch64-linux-android rust target as well.

Configure the (absolute) path of the NDK-clang in `rust/.cargo/config.toml` file. This config is common for all crates in `rust` folder. Replace the existing linker path with your NDK linker.

You can link to native android dependencies to crates individually using `build.rustflags` key in individual crate's config.toml.

Use this command to compile a rust crate:
```
cargo b --release --target aarch64-linux-android
```

Move the generated libs from `target/aarch64-linux-android/release` folders to the `main/jniLibs/arm64-v8a` folders

Failure to do any of these will lead to a crashy app.

And sorry, Android Studio cant help managing rust sources. Prefer VSCode/RustRover for Rust dev.

### Building the app
Connect your Android device to your laptop. Enable USB debugging in Developer settings.

Make sure your Android device is in "file transfer" or "media transfer" mode.
Use AI/Tutorials if its too hard.

You can build and install the app on-device with this command OR (see below):
```
./gradlew installDebug
```

Alternatively, you can select the physical device and hit the run button on Android Studio.
(I dont prefer the emulator)
(No need to break your head)

There's the app!
