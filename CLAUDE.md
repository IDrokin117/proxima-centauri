# Proxy Centauri

HTTP CONNECT proxy server with authentication and rate limiting.

## Tech
- Rust (edition 2024), Tokio, httparse, anyhow, tracing

## Structure
```
proxima_centauri/lib/
├── server.rs      # Accept loop
├── handler.rs     # Request handling, auth, limits
├── tunnel.rs      # Bidirectional TCP streaming
├── auth.rs        # User database
├── statistics.rs  # Traffic/concurrency tracking, Limiter
├── config.rs      # Configuration
├── http_utils/    # ProxyResponse enum
└── tests/         # Integration tests
```

## Rules

1. **Wait for confirmation before making changes**
2. **DO NOT add comments** — code should be self-documenting
3. **Minimal changes** — solve the task, nothing more
4. **Always run tests** after changes: `cargo test --lib`
5. **Use async/await** — never block, use `tokio::` types
6. **Error handling** — `anyhow::Result`, `bail!`, no `unwrap()` on user data

## Commands
```bash
cargo check          # Compile check
cargo test --lib     # Run tests
cargo clippy         # Lint
cargo run            # Run server
```

## Auth
Users: `procent:o953zY7lnkYMEl5D`, `admin:12345`
