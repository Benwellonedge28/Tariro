# Contributing to Tariro AGI

Thank you for your interest in contributing to Tariro AGI! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a welcoming and respectful environment for all contributors.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- A text editor or IDE with Rust support (VS Code + rust-analyzer recommended)

### Setting Up Development Environment

```bash
# Fork the repository on GitHub

# Clone your fork
git clone https://github.com/YOUR_USERNAME/Tariro.git
cd Tariro

# Add upstream remote
git remote add upstream https://github.com/Benwellonedge28/Tariro.git

# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Development Workflow

```bash
# Create a feature branch
git checkout -b feature/your-feature-name

# Make your changes
# ...

# Run tests
cargo test --all

# Run lints
cargo fmt --check --config-path rustfmt.toml
cargo clippy --config clippy.toml -- -D warnings

# Commit your changes
git commit -m "feat: add your feature description"

# Push to your fork
git push origin feature/your-feature-name

# Open a Pull Request
```

## Pull Request Guidelines

1. **Scope**: Keep PRs focused on a single concern
2. **Description**: Include a clear description of changes
3. **Tests**: Include tests for new functionality
4. **Documentation**: Update docs for any API changes
5. **CI**: Ensure all CI checks pass

### PR Template

```markdown
## Description
Brief description of the changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe testing performed

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] All CI checks pass
```

## Code Style

We use:
- **rustfmt** with `rustfmt.toml` configuration
- **clippy** with `clippy.toml` configuration

Run before committing:
```bash
cargo fmt --config-path rustfmt.toml
cargo clippy --config clippy.toml -- -D warnings
```

## Testing Guidelines

### Unit Tests
- Place tests in `#[cfg(test)]` modules within source files
- Use descriptive test names: `test_feature_name_when_condition`

### Integration Tests
- Place in `tests/` directory
- Use `tariro::*` to import library items

### Running Tests
```bash
# All tests
cargo test --all

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Benchmarks
cargo bench
```

## Commit Message Guidelines

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

## Documentation

- Public APIs should have documentation comments
- Use `///` for documentation
- Include examples where helpful
- Update README.md for significant changes

## Questions?

- Open an issue for bugs
- Start a discussion for questions
- Email: samuelmukandara@gmail.com

## License

By contributing, you agree your contributions will be licensed under the MIT or Apache-2.0 license.
