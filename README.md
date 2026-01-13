# Proxy Centauri 🚀

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> A high-performance proxy server with user-based authentication and traffic statistics.

**Proxy Centauri** (aka **Procent**) is a proxy server that provides a single entry point for routing traffic through multiple proxy providers with per-user configuration and detailed statistics aggregation.

## ⚠️ Disclaimer

> [!WARNING]
> This project is currently under active development. APIs and features may change.

## ✨ Features

- 🔐 **User-based authentication** - Basic auth with credentials database
- 📊 **Detailed statistics** - Per-user ingress/egress traffic monitoring
- ⚡ **High performance** - Built with Tokio async runtime
- 🧪 **Well tested** - Comprehensive integration test suite
- 🔧 **Configurable** - Environment-based configuration

## 🚀 Quick Start

### Prerequisites

- Rust 1.75 or higher
- Cargo

### Installation

```bash
git clone https://github.com/yourusername/proxy-centauri.git
cd proxy-centauri
cargo build --release
```

### Configuration

Create a `.env` file in the project root:

```env
PROXY_PORT=9090
PROXY_HOST=127.0.0.1
```

### Running

```bash
cargo run --release
```

The proxy server will start on `127.0.0.1:9090` (or your configured address).

## 📖 Usage

### Connecting through the proxy

Configure your client to use the proxy:

```bash
# Using curl
curl -x http://procent:o953zY7lnkYMEl5D@127.0.0.1:9090 https://example.com

# Using environment variables
export HTTP_PROXY=http://procent:o953zY7lnkYMEl5D@127.0.0.1:9090
export HTTPS_PROXY=http://procent:o953zY7lnkYMEl5D@127.0.0.1:9090
```

### Default credentials

- Username: `procent`
- Password: `o953zY7lnkYMEl5D`

Or:
- Username: `admin`
- Password: `12345`

> [!NOTE]
> Credentials are currently hardcoded in `lib/auth.rs`. In production, use a proper authentication backend.


### Project Structure

```
proxy_centauri/
├── bin/
│   └── main.rs           # Application entry point
└── lib/
    ├── lib.rs            # Library root
    ├── server.rs         # Server orchestration
    ├── handler.rs        # Connection handling logic
    ├── tunnel.rs         # TCP tunneling
    ├── auth.rs           # Authentication & database
    ├── config.rs         # Configuration management
    ├── statistics.rs     # Traffic statistics
    ├── http_utils/       # HTTP utilities
    │   ├── request.rs    # Request helpers
    │   └── response.rs   # Response types
    └── tests/            # Integration tests
```

## 🧪 Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_proxy_auth_required
```

### Test Coverage

- ✅ Proxy authentication required (407)
- ✅ Unauthorized access (401)
- ✅ Method not allowed (405)
- ✅ Successful CONNECT tunnel (200)
- ✅ Malformed request handling
- ✅ Server cleanup on drop

## 📊 Statistics

The proxy tracks per-user traffic statistics:

```
User `procent` stats:
    ingress: 1056 bytes
    egress: 5618 bytes
```

Statistics are logged every 10 seconds during runtime.

## 🛠️ Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Check without building
cargo check
```

### Linting

```bash
cargo clippy --all-targets --all-features
```

### Formatting

```bash
cargo fmt --all
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 Roadmap

- [ ] Dynamic user management API
- [ ] Persistent statistics storage
- [ ] HTTP/2 support
- [ ] Rate limiting per user
- [ ] Docker containerization
- [ ] Prometheus metrics export
- [ ] Configuration hot-reload
- [ ] Multiple backend proxy support

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Tokio](https://tokio.rs/) - Async runtime for Rust
- Uses [httparse](https://github.com/seanmonstar/httparse) - HTTP parsing
- Logging via [tracing](https://github.com/tokio-rs/tracing)

---

**Made with ❤️ and Rust**
