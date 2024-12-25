# buco (Builder at Compile Time)

A zero-overhead, compile-time builder pattern implementation for Rust that enforces correctness through the type system while preserving IDE support and detailed error messages.

## Description

buco enhances Rust's struct construction by:

- Moving builder pattern validation to compile-time
- Preventing field omission errors through type-level state machines
- Supporting flexible field ordering without runtime overhead
- Providing clear, actionable compiler errors
- Handling optional fields with configurable strictness levels

Perfect for ensuring robust struct initialization while maintaining excellent developer experience.

## Installation

Add buco to your `Cargo.toml`:

```toml
[dependencies]
buco = "0.2.0"
```

## Usage

### Basic Example
```rust
use buco::Builder;

#[derive(Builder)]
struct Config {
    host: String,
    port: u16,
    timeout: u64,
}

fn main() {
    let config = Config::builder()
        .set_host("localhost".into())
        .set_port(8080)
        .set_timeout(30)
        .build();
        
    // All fields must be set - won't compile otherwise
}
```

### Optional Fields
```rust
#[derive(Builder)]
struct Server {
    host: String,
    port: u16,
    timeout: Option<u64>,  // Optional field
}

// Optional fields can be omitted in non-strict mode
let server = Server::builder()
    .set_host("localhost".into())
    .set_port(8080)
    .build();
```

### Strict Mode
```rust
#[derive(Builder)]
#[buco(strict)]
struct StrictConfig {
    name: String,
    optional: Option<String>,  // Must be explicitly set in strict mode
}
```

## Features

- **Compile-Time Validation**:
  - Missing field detection at compile time
  - Invalid field order detection
  - Prevention of duplicate field assignment
  - Type safety for each field

- **Flexible Building**:
  - Fields can be set in any order
  - Optional field support
  - Configurable strict mode
  - Clear error messages

- **Zero Runtime Cost**:
  - All checks performed at compile time
  - No runtime performance impact
  - No heap allocations from builder pattern
  - Direct struct construction

- **Developer Experience**:
  - Full IDE support
  - Descriptive error messages
  - Type hints for missing fields
  - Rust-analyzer friendly

## Error Examples

buco provides clear, actionable error messages:

```rust
// Missing required field
let config = Config::builder()
    .set_host("localhost".into())
    .build();  
// Error: no method named `build` found for struct `ConfigBuilder<()>`
// Help: You need to set `port` and `timeout`

// Trying to set a field twice
let config = Config::builder()
    .set_host("localhost".into())
    .set_host("127.0.0.1".into());  
// Error: no method named `set_host` found for struct `ConfigBuilder<HostField>`
```

## Contributing Guidelines

1. **Issue First**: Create or find an issue before starting work
2. **Testing**: 
   - Run unit tests: `cargo test`
   - Run compile fail tests: `cargo test sanity_tests`
   - Add tests for new features
3. **Code Style**: Follow Rust formatting guidelines
4. **Documentation**: Update docs and examples for changes

## Development

### Running Tests
```bash
# Run all tests
cargo test

# Run only compile-fail tests
cargo test sanity_tests
```

### Project Structure
```
buco/
├── src/            # Main library code
├── buco_derive/    # Procedural macro implementation
└── build_tests/    # Compilation test cases
```

### Build Test Categories
- Enum/union rejection tests
- Struct builder tests
- Partial completion tests
- Optional field tests
- Strict mode tests

## Limitations

- Only works with structs (enums and unions not supported)
- Struct fields must be named (tuple structs not supported)
- Generic types must satisfy necessary trait bounds

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Built with 🦀 in Rust
