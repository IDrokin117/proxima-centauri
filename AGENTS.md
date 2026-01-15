# Agents Documentation - Proxy Centauri

## About the Project

**Proxy Centauri** - a high-performance Rust-based HTTP CONNECT proxy server with user authentication and traffic statistics.

### Tech Stack
- **Language**: Rust (edition 2024)
- **Async Runtime**: Tokio 1.48.0 (multi-threaded)
- **Binary Name**: `procent`
- **HTTP Parsing**: httparse 1.10.1
- **Logging**: tracing + tracing-subscriber
- **Error Handling**: anyhow

### Project Structure
```
proxy-centauri/
├── proxy_centauri/
│   ├── bin/
│   │   └── main.rs              # Application entry point
│   ├── lib/
│   │   ├── lib.rs               # Library root, re-exports
│   │   ├── server.rs            # Server orchestration, accept loop
│   │   ├── handler.rs           # Connection handling, HTTP parsing
│   │   ├── tunnel.rs            # TCP bidirectional tunneling
│   │   ├── auth.rs              # Authentication & user database
│   │   ├── config.rs            # Configuration management
│   │   ├── statistics.rs        # Per-user traffic statistics
│   │   ├── http_utils/          # HTTP utilities
│   │   │   ├── mod.rs
│   │   │   ├── request.rs       # Request helpers (test only)
│   │   │   └── response.rs      # Response enum (ProxyResponse)
│   │   └── tests/               # Integration tests
│   │       ├── mod.rs
│   │       ├── integration_tests.rs
│   │       └── common/
│   │           ├── mod.rs
│   │           └── request.rs   # Test request helpers
│   └── Cargo.toml               # Package manifest
├── Cargo.toml                   # Workspace manifest
├── README.md                    # Comprehensive project documentation
└── AGENTS.md                    # This file
```

### Core Features
- ✅ **HTTP CONNECT tunneling** - Full support for HTTPS proxying
- ✅ **User authentication** - Basic auth with credentials database
- ✅ **Traffic statistics** - Per-user ingress/egress monitoring
- ✅ **Configurable** - Environment-based configuration (.env)
- ✅ **Well tested** - Comprehensive integration test suite with RAII cleanup
- ✅ **High performance** - Tokio work-stealing scheduler, zero-copy bidirectional streaming

### Implemented Components

#### Authentication
- `auth.rs` contains `Database` struct with hardcoded users
- Supports Basic auth (base64-encoded credentials)
- Users: `procent:o953zY7lnkYMEl5D`, `admin:12345`

#### HTTP Responses
- `ProxyResponse` enum with type-safe responses:
  - `ConnectionEstablished` (200)
  - `Unauthorized` (401)
  - `MethodNotAllowed` (405)
  - `ProxyAuthRequired` (407)

#### Statistics
- Real-time per-user traffic tracking
- Ingress/egress byte counters (u128)
- Logged every 10 seconds

#### Testing
- Each test spawns isolated server on unique port (9100+)
- RAII `TestServer` guard with automatic cleanup
- Tests: auth required, unauthorized, method not allowed, successful connect, cleanup

### Repository
- **GitHub**: https://github.com/IDrokin117/proxy-centauri
- **License**: MIT
- **Author**: Igor Drokin

---

## Project Working Rules

### 1. DO NOT MAKE CHANGES BY DEFAULT
**IMPORTANT**: When working with code, agents should only suggest examples of changes, but NOT apply them automatically. All changes must be approved by the user.

**Correct Approach**:
- Analyze the request
- Suggest solution options with code examples
- Wait for user confirmation
- Only apply changes after confirmation

**Incorrect Approach**:
- Automatically modify files without approval
- Refactor code without request
- Add "improvements" without explicit permission

### 2. USE ASYNC/AWAIT WITH TOKIO
All asynchronous code must use Tokio runtime. Avoid blocking operations in async context.

**Important patterns**:
- Use `tokio::spawn` for concurrent tasks (NOT std::thread)
- Use `tokio::io::copy_bidirectional` for proxying
- Never call `.await` on spawned tasks unless necessary (breaks parallelism)
- Tasks are lightweight, threads are expensive

### 3. FOLLOW RUST BEST PRACTICES
- Use `clippy` for code linting
- Follow Rust naming conventions
- Apply idiomatic Rust patterns
- Handle errors through `Result<T, E>`
- Use `anyhow` for error context
- Prefer `&str` over `String` when possible

### 4. MODULAR ARCHITECTURE
**Current module structure**:
- `config.rs` - Configuration and initialization (uses `Once` for tracing)
- `server.rs` - Server lifecycle, accept loop
- `handler.rs` - Request parsing, routing, authentication
- `tunnel.rs` - Bidirectional TCP streaming
- `auth.rs` - Authentication logic + database
- `statistics.rs` - Traffic monitoring
- `http_utils/` - HTTP request/response helpers

**Rules**:
- Keep modules focused on single responsibility
- Minimize cross-module dependencies
- Use `pub(crate)` for internal APIs
- Public API only in `lib.rs`

### 5. TESTING
- Write integration tests in `tests/` directory
- Use `TestServer` RAII guard for test isolation
- Each test gets unique port via `AtomicU16` counter
- Test responses using `ProxyResponse` enum
- Run tests with `cargo test --lib`

**Test patterns**:
```rust
#[tokio::test]
async fn test_something() -> Result<()> {
    let server = TestServer::start().await;  // Auto-cleanup on drop
    let mut socket = TcpStream::connect(server.addr()).await?;
    // ... test logic
    Ok(())
}
```

### 6. PERFORMANCE
- Use `Arc` for shared ownership (NOT `Rc` in async)
- `Arc<Mutex<T>>` only when mutation needed
- Avoid `.clone()` on `Arc` in hot paths
- Use `copy_bidirectional` for zero-copy proxying
- Set `TCP_NODELAY` to reduce latency
- Profile with `Instant::now()` / `elapsed()`

### 7. SECURITY
- Validate all user input (HTTP parsing)
- Use `httparse` for safe HTTP parsing
- Never trust `unwrap()` on user data
- Base64 decode with error handling
- Avoid buffer overflows (use bounded reads)
- Timeout long-running operations

### 8. ERROR HANDLING
- Use `anyhow::Result` for application errors
- Use `bail!` for early returns with context
- Convert stdlib errors via `.into()` or `anyhow!`
- Log errors with `tracing::error!`
- Never use `expect()` in production paths

---

## Current Implementation Status

### ✅ Completed Features
- [x] Basic HTTP CONNECT proxy
- [x] User authentication (Basic auth)
- [x] Per-user traffic statistics
- [x] Environment-based configuration
- [x] Modular architecture (6 modules)
- [x] Integration test suite (6 tests)
- [x] RAII test server with cleanup
- [x] HTTP response enum
- [x] Comprehensive README

### 🚧 In Progress
- [ ] Remove debug timing logs from handler
- [ ] Persistent user database (currently hardcoded)
- [ ] Statistics export API

### 📋 Roadmap
- [ ] Dynamic user management API
- [ ] Persistent statistics storage (DB)
- [ ] Rate limiting per user
- [ ] Docker containerization
- [ ] Prometheus metrics export
- [ ] Configuration hot-reload
- [ ] Multiple backend proxy support
- [ ] HTTP/2 support

---

## Architecture Notes

### Async Runtime Architecture
**Multi-threaded Tokio runtime**:
- Worker threads: N = CPU cores
- Each worker has local scheduler + task queue
- Work-stealing between workers
- Shared I/O reactor (mio-based)
- Tasks migrate between threads

**Important**:
- `tokio::spawn` creates task (NOT thread)
- Tasks are stackless coroutines (state machines)
- `Pin` prevents self-referential struct movement
- Tasks can outlive spawning context

### Connection Flow
1. **Accept** - `TcpListener::accept()` on main loop
2. **Spawn** - Each connection gets own task via `tokio::spawn`
3. **Parse** - `httparse::Request` parses HTTP CONNECT
4. **Auth** - Check credentials in `Database`
5. **Connect** - Open TCP to target server
6. **Tunnel** - `copy_bidirectional` streams data both ways
7. **Stats** - Track bytes transferred
8. **Cleanup** - Connections auto-close on drop

### HTTP CONNECT Protocol
```
Client → Proxy: CONNECT example.com:443 HTTP/1.1
                Proxy-Authorization: Basic <base64>

Proxy → Client: HTTP/1.1 200 Connection Established

[Raw TCP tunnel, TLS happens inside]
```

### Testing Isolation
- Each test spawns server on unique port (9100, 9101, ...)
- `TestServer` implements `Drop` → calls `handle.abort()`
- Port counter is `AtomicU16` (thread-safe)
- Tests run in parallel without conflicts
- OS releases port after task abort

---

## Notes for AI Agents

When working with this project:

1. **Always read before modifying**
   - Use `Read` tool to understand current code
   - Check imports and dependencies
   - Understand module relationships

2. **Propose minimal changes**
   - Solve the specific task, nothing more
   - Don't add abstractions "for the future"
   - Keep it simple and focused

3. **Test your changes**
   - Run `cargo check` for compilation
   - Run `cargo test --lib` for tests
   - Ensure all tests pass

4. **Follow existing patterns**
   - Match current code style
   - Use same error handling approach
   - Follow established module structure

5. **Remember: EXAMPLES FIRST**
   - Show code examples in conversation
   - Wait for user approval
   - Apply changes only after confirmation

6. **Async context awareness**
   - Never block in async functions
   - Use `tokio::` versions of std types
   - Understand task vs thread difference

7. **Security mindset**
   - Validate all external input
   - Handle errors, don't panic
   - Use safe parsing libraries

---

## Quick Commands

```bash
# Build & Run
cargo build --release
cargo run

# Testing
cargo test --lib                    # All tests
cargo test test_proxy_auth_required # Specific test
cargo test -- --nocapture           # Show output

# Code Quality
cargo check                         # Fast compile check
cargo clippy                        # Linting
cargo fmt                           # Format code

# Configuration
# Create .env file:
echo "PROXY_PORT=9090" > .env
echo "PROXY_HOST=127.0.0.1" >> .env
```

---

**Last Updated**: 2026-01-13
**Status**: Active Development 🚀
