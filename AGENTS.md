# Agents Documentation - Proxy Centauri

## About the Project

**Proxy Centauri** - a Rust-based proxy server providing a single entry point for various proxy providers.

### Tech Stack
- **Language**: Rust (edition 2024)
- **Async Runtime**: Tokio 1.48.0
- **Binary Name**: `procent`

### Project Structure
```
proxy-centauri/
├── proxy_centauri/
│   ├── bin/          # Application entry point (main.rs)
│   ├── lib/          # Library code (lib.rs)
│   └── Cargo.toml    # Package manifest
├── Cargo.toml        # Workspace manifest
└── README.md         # Project documentation
```

### Core Features
- Single entry point for various proxy providers
- User-based configuration
- Detailed statistics aggregation
- Fast request routing

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

### 3. FOLLOW RUST BEST PRACTICES
- Use `clippy` for code linting
- Follow Rust naming conventions
- Apply idiomatic Rust patterns
- Handle errors through `Result<T, E>`

### 4. DOCUMENT CODE
- Add doc comments (`///`) for public APIs
- Write clear comments for complex logic
- Update documentation when functionality changes

### 5. TESTING
- Write unit tests for new functionality
- Use integration tests to verify component interactions
- Test edge cases and error handling

### 6. PERFORMANCE
- Minimize memory allocations
- Use zero-copy approaches where possible
- Profile critical code sections
- Avoid unnecessary data cloning

### 7. SECURITY
- Validate user input data
- Avoid unsafe operations unless necessary
- Use secure libraries for network operations
- Protect against injection attacks

### 8. CODE STRUCTURE
- Separate logic into modules by responsibility
- Use lib/ for library code
- Use bin/ for application entry point
- Minimize dependencies between modules

---

## Current Status

**Development Status**: Work in Progress ⚠️

**Current Branch**: feature/initial
**Main Branch**: main

**Recent Changes**:
- Updated README.md with project description
- Initialized base project
- Added Tokio 1.48.0 dependency

---

## Notes for AI Agents

When working with this project:
1. Always read the code before suggesting changes
2. Propose minimal changes that solve the specific task
3. Don't add redundant abstractions without necessity
4. Clarify requirements if they are ambiguous
5. Remember the first rule: EXAMPLES ONLY, NO CHANGES