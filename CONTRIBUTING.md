# Contributing to AgenticTime

Thank you for your interest in contributing to AgenticTime! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/agentic-time.git`
3. Create a feature branch: `git checkout -b my-feature`
4. Make your changes
5. Run the tests (see below)
6. Commit and push
7. Open a pull request

## Development Setup

This is a Cargo workspace monorepo. All Rust crates are under `crates/`.

### Rust Workspace

```bash
# Build everything (core + CLI + MCP + FFI)
cargo build

# Run all tests
cargo test --workspace

# Core library only
cargo test -p agentic-time
cargo bench -p agentic-time

# CLI tool only
cargo test -p agentic-time-cli

# MCP server only
cargo test -p agentic-time-mcp

# FFI bindings only
cargo test -p agentic-time-ffi

# Run the CLI
cargo run -p agentic-time-cli -- deadline list

# Run the MCP server
cargo run -p agentic-time-mcp
```

### Python SDK

```bash
cd python/
python3 -m venv .venv
source .venv/bin/activate
pip install -e ".[dev]"
pytest tests/ -v
```

## Ways to Contribute

### Report Bugs

File an issue with:
- Steps to reproduce
- Expected behavior
- Actual behavior
- System info (OS, Python version, Rust version)

### Add an MCP Tool

1. Add tool definition in `crates/agentic-time-mcp/src/tools.rs`
2. Add dispatch case in `dispatch()`
3. Implement handler function
4. Add tests
5. Update README

### Write Examples

1. Add a new example in `examples/`
2. Ensure it runs without errors
3. Add a docstring explaining what it demonstrates

### Improve Documentation

All docs are in `docs/`. Fix typos, add examples, clarify explanations.

## Code Guidelines

- **Rust**: Follow standard Rust conventions. Run `cargo clippy` and `cargo fmt`.
- **Python**: Follow PEP 8. Use type hints. Run `mypy` for type checking.
- **Tests**: Every feature needs tests. We maintain 80+ tests across the stack.
- **Documentation**: Update docs when changing public APIs.

## Commit Messages

Use clear, descriptive commit messages:
- `Add: new decay alert threshold tool`
- `Fix: schedule conflict detection with recurring events`
- `Update: improve PERT estimation precision`
- `Docs: add sequence workflow examples`

## Pull Request Guidelines

- Keep PRs focused
- Include tests for new functionality
- Update documentation if needed
- Ensure all tests pass before submitting
- Write a clear PR description

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
