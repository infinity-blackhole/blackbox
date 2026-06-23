# Blackbox — Lunar Tear Rust Rewrite

Clean-room Rust reimplementation of the NieR Re[in]carnation private server.

**Language:** Rust 2024 edition (MSRV TBD, targeting latest stable)
**License:** AGPL-3.0
**Repo:** `github.com/infinity-blackhole/blackbox`

## Stack

| Concern | Crate |
|---------|-------|
| Async runtime | `tokio` (full features) |
| gRPC | `tonic` + `prost` (via `blackbox-game-server`) |
| HTTP (CDN/Auth/Admin) | `axum` + `tower` |
| Actor model | `kameo` |
| Database | `sqlx` (SQLite, compile-time checked) |
| Serialization | `serde` + `rmp-serde` + `prost` |
| Binary buffers | `bytes` |
| Crypto | `aes` + `cbc` + `ring` |
| LZ4 | `lz4` |
| Config | `config` (TOML + env) |
| Logging | `tracing` + `tracing-subscriber` |
| Errors | `thiserror` + `anyhow` |
| Time | `time` |
| Protobuf codegen | `prost-build` |
| Testing | `tokio-test`, `insta` (snapshots), `proptest` |

## Commit Style

- Plain-text capitalized title, no conventional-commit prefix
- Body wrapped at 80 columns
- GPG-sign all commits
- 1 commit == 1 PR via ghstack

## Protect `main`

- Require 1 approving review
- Require linear history
- Require signed commits
- Squash+rebase merge only
