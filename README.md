Repro for https://github.com/servo/servo/issues/27043 and https://github.com/housleyjk/ws-rs/issues/51

### Steps

(appologies for the weird repro steps)

1. Be running on Windows 10.
2. Ensure you have the [rust toolchain installed](https://rustup.rs/).
3. Ensure [vcpkg](https://github.com/Microsoft/vcpkg) is intalled.
4. Add vcpkg to the path environment variable.
5. [Integrate vcpkg](https://github.com/microsoft/vcpkg/blob/master/docs/users/integration.md)
    -   Run this from a terminal with admin privileges.
6. Install openssl for 64 bit Windows
    -   `vcpkg install openssl --triplet x64-windows-static-md`
8. Run `cargo run`.

It will prompt for which URL to use. One is a normal insecure websocket (`ws`) and the other is secure (`wss`).

For option `0`, a connection with the server will be established and any message you type will be echoed back to you.

For option `1`, a connection will be attempted but will fail for some reason.
From what I can tell from wireshark, no packets are sent when connecting over `wss` while over `ws` the connection will be established correctly.
