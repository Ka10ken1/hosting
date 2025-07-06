# OpenCode.md

## Build/Lint/Test Commands
- Build: `cargo build`
- Run: `cargo run`
- Test: `cargo test`
- Lint: Use `cargo clippy` for linting
- Check formatting: `cargo fmt -- --check`
- Format code: `cargo fmt`
- Run a single test: `cargo test <test_name>`

## Code Style Guidelines

### Imports
- Group `std` imports first, followed by external crates, and then local modules.
- Use explicit imports rather than wildcard imports for clarity.

### Formatting
- Use `cargo fmt` to adhere to the Rust style guide.
- Limit lines to 100 characters.

### Types
- Use `Option` and `Result` for fallible operations.
- Prefer explicit types for public functions for readability.

### Naming Conventions
- Use `snake_case` for functions and variables.
- Use `PascalCase` for types and structs.
- Avoid abbreviations in names to enhance clarity.

### Error Handling
- Use the `?` operator for propagating errors.
- For recoverable errors, prefer custom error types implementing `std::error::Error`.
- Log critical errors and gracefully handle them.

### Testing
- Organize tests in a `tests` module.
- Write separate unit and integration tests as appropriate.

### Additional Notes
- Serve assets with the settings in `Trunk.toml`.
- Keep `static/` directory for static web assets.
