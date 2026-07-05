# Uridim Developer Architecture

Uridim is a terminal-native, project-aware observability and diagnostics environment.

## Purpose

This document explains:

1. how Uridim's major components fit together
2. where code and runtime assets belong
3. the architectural constraints that should remain stable as the project evolves

Module-specific details should live near the relevant module whenever possible.

## Current architecture

Uridim begins as a Rust workspace.

At this stage, Uridim intentionally avoids speculative subsystems. New crates and major directories should be introduced only when real implementation pressure justifies them.

## Project layout

- `crates/` — compiled Rust components
- `runtime/` — shipped runtime assets, built-in capabilities, and documentation
- `test/` — test infrastructure, fixtures, benchmarks, and integration environments
- `scripts/` — development, maintenance, and release automation

## Architectural principles

1. Uridim must remain useful without third-party plugins.
2. Core code must not depend directly on specific technologies such as Docker or PostgreSQL.
3. User interfaces must not perform system collection directly.
4. Platform-specific behavior should remain behind explicit boundaries.
5. New abstractions should be introduced after repeated implementation pressure appears.
6. The repository tree should communicate responsibility and ownership.
7. Module-specific architecture should be documented near the module.

## Current dependency graph

```text
uridim executable
      │
      ▼
future core APIs
```
