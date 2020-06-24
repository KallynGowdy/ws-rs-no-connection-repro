Repro for https://github.com/servo/servo/issues/27043

### Steps

1. [Clone and build the servo repo](https://github.com/servo/servo).
2. Ensure you have the [rust toolchain installed](https://rustup.rs/).
3. Set the `OPENSSL_DIR` environment variable to the OpenSSL install directory created by the servo build.
    -   Use the UWP version.
    -   This will be in the servo repo directory under `.servo/msvc-dependencies/openssl/{version}/x64-windows-uwp`.
4. Run `cargo build`
5. Copy `libssl.dll` and `libcrypto.dll` to the output directory (`./target/debug`).
6. Run `./target/debug/test_ws.exe`.
