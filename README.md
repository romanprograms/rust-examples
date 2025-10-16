# Rust Workspace Starter

One repo, many crates (apps + libs) via Cargo workspaces.

## Quick start
```bash
# build everything
cargo build --workspace

# run a specific binary
cargo run -p api
cargo run -p cli -- Alice

# tests, fmt, clippy
cargo t
cargo fmt
cargo c
```

## Layout
- `crates/` reusable libraries (`core-utils`, `db`)
- `apps/` binaries (`api`, `cli`)
- `tools/` internal tooling (`schema-gen`)
- shared deps & lints live in root `Cargo.toml` under `[workspace.*]`

## Notes
- `db` crate exposes a feature flag `postgres` (no external deps here for simplicity).
- `api` uses Tokio runtime and calls into both libraries.