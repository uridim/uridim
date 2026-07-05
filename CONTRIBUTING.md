# Contributing to Uridim

Uridim is in early development.

## Development requirements

- Rust stable toolchain
- Cargo
- Git

## Build

```bash
cargo build --workspace
```

## Test

```bash
cargo test --workspace --all-targets
```

## Format

```bash
cargo fmt --all
```

## Lint

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Pull requests

- Use a feature branch instead of main.
- Keep unrelated changes separate.
- Include tests when behavior changes.
- Avoid cosmetic changes to unrelated files.
- Keep documentation concise and useful.
- Ensure CI passes before requesting review.

Example branch names:

- feat/project-detection
- fix/tui-resize
- refactor/collector-model
- docs/runtime-architecture

## Commit messages

`type(scope): subject`

Examples:

```bash
feat(project): detect repository root
fix(tui): preserve selection after refresh
refactor(core): separate sampling from aggregation
test(project): add nested workspace fixture
docs(runtime): describe capability loading
```

Types:
- build
- ci
- docs
- feat
- fix
- perf
- refactor
- revert
- test
