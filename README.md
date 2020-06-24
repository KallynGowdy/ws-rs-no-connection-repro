Repro for https://github.com/servo/servo/issues/27043

### Steps

(appologies for the weird repro steps)

1. Be running on Windows 10.
2. [Clone and build the servo repo](https://github.com/servo/servo).
    -   You can probably skip this step if you already have a OpenSSL installed on your system.
    -   If you have OpenSSL installed and you cannot reproduce the issue, then go back and do this step to help confirm that it is a versioning issue.
3. Ensure you have the [rust toolchain installed](https://rustup.rs/).
4. Set the `OPENSSL_DIR` environment variable to the OpenSSL install directory created by the servo build.
    -   Use the UWP version.
    -   This will be in the servo repo directory under `.servo/msvc-dependencies/openssl/{version}/x64-windows-uwp`.
5. Run `cargo build`
6. Copy `libssl.dll` and `libcrypto.dll` to the output directory (`./target/debug`).
7. Run `cargo run`.

It will prompt for which URL to use. One is a normal insecure websocket (`ws`) and the other is secure (`wss`).

For option `0`, a connection with the server will be established and any message you type will be echoed back to you.

For option `1`, a connection will be attempted but will fail for some reason.
From what I can tell from wireshark, no packets are sent when connecting over `wss` while over `ws` the connection will be established correctly.
