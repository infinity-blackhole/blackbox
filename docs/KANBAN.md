# Blackbox — Development Kanban

> Clean-room Rust rewrite of the NieR Re[in]carnation private server (Lunar Tear).
> All specs in `docs/`. No implementation code yet.

**Last updated:** 2026-06-23
**Phase:** Pre-development — scaffolding & tooling

---

## 📋 Backlog (Unestimated / Future)

- [ ] Benchmark: tokio actor throughput vs Go goroutine baseline
- [ ] Profile: SQLite concurrent write contention under N clients
- [ ] Fuzz: `store::UserState` serialization round-trip
- [ ] Integration: full load test (100 concurrent clients, 24h soak)
- [ ] CI: cross-compile for Linux x86_64 / aarch64 (server targets)
- [ ] CI: Nix flake build verification
- [ ] Security: audit `unsafe` blocks (should be zero)
- [ ] Observability: dashboards (Grafana / Prometheus)
- [ ] Feature: admin WebSocket streaming for live metrics
- [ ] Feature: graceful shutdown with drain timeout
- [ ] Feature: config hot-reload (SIGHUP or file watcher)
- [ ] Research: msgpack → protobuf migration path for internal events

---

## 🏗️ Phase 0 — Workspace Scaffolding

**Goal:** `cargo build --workspace` succeeds with empty stubs. All crate skeletons exist.

| # | Task | Crate(s) | Depends On | Est. |
|---|------|----------|------------|------|
| 0.1 | Create workspace `Cargo.toml` with all 12 members + `octo-proto` | workspace root | — | 30min |
| 0.2 | Create `Cargo.toml` for `blackbox-core` (empty lib) | `core` | 0.1 | 15min |
| 0.3 | Create `Cargo.toml` for `blackbox-observability` (empty lib) | `observability` | 0.1 | 15min |
| 0.4 | Create `Cargo.toml` for `blackbox-store` (empty lib) | `store` | 0.1 | 15min |
| 0.5 | Create `Cargo.toml` for `blackbox-master-data` (empty lib + build.rs) | `master-data` | 0.1 | 15min |
| 0.6 | Create `Cargo.toml` for `blackbox-diff-sync` (empty lib) | `diff-sync` | 0.1 | 15min |
| 0.7 | Create `Cargo.toml` for `blackbox-auth` (empty lib) | `auth` | 0.1 | 15min |
| 0.8 | Create `Cargo.toml` for `blackbox-game-server` (empty bin) | `game-server` | 0.1 | 15min |
| 0.9 | Create `Cargo.toml` for `blackbox-assets-server` (empty bin) | `assets-server` | 0.1 | 15min |
| 0.10 | Create `Cargo.toml` for `blackbox-admin` (empty bin) | `admin` | 0.1 | 15min |
| 0.11 | Create `Cargo.toml` for `blackbox-cli` (empty bin) | `cli` | 0.1 | 15min |
| 0.12 | Create `Cargo.toml` for `blackbox-dev` (empty bin) | `dev` | 0.1 | 15min |
| 0.13 | Create `proto/octo.proto` + `build.rs` for `octo-proto` | `octo-proto` | 0.1 | 30min |
| 0.14 | Create `config/default.toml` with all sections | root | 0.1 | 15min |
| 0.15 | Create `flake.nix` dev shell with all toolchain deps | root | 0.1 | 30min |
| 0.16 | Create `Makefile` with `build`, `test`, `run`, `lint` targets | root | 0.1 | 15min |
| 0.17 | Verify `cargo check --workspace` passes | all | 0.2–0.16 | 15min |

**Exit criterion:** `cargo check --workspace` succeeds. `cargo build --workspace` succeeds (empty binaries).

---

## 🏗️ Phase 1 — `blackbox-core` + `blackbox-observability`

**Goal:** Foundation crates fully implemented. Other crates can depend on real types.

| # | Task | Crate(s) | Depends On | Est. |
|---|------|----------|------------|------|
| 1.1 | `AppConfig` struct + TOML loading + env override | `core` | 0.2 | 2h |
| 1.2 | `MasterDataCatalogs` struct (29 fields, matching INTERFACES.md) | `core` | 0.2 | 1h |
| 1.3 | `GameClock` (system + frozen) | `core` | 0.2 | 30min |
| 1.4 | `LunarError` enum (thiserror, all variants) | `core` | 0.2 | 1h |
| 1.5 | Constants (AES key/IV, LZ4 ext code 99, resource URL) | `core` | 0.2 | 15min |
| 1.6 | `init_tracing()` with OTLP + stdout fallback | `observability` | 0.3 | 1h |
| 1.7 | Reusable span macros | `observability` | 1.6 | 30min |
| 1.8 | Unit tests: config, clock, error display | `core`, `observability` | 1.1–1.7 | 2h |

**Exit criterion:** `cargo test -p blackbox-core -p blackbox-observability` passes. 90%+ coverage on `core`.

---

## 🏗️ Phase 2 — `blackbox-master-data` (Schema + Binary)

**Goal:** Parse `schemas.json`, generate Rust code, load `.bin.e` files. Master data pipeline working end-to-end (offline).

| # | Task | Crate(s) | Depends On | Est. |
|---|------|----------|------------|------|
| 2.1 | `SchemaIR` + `parse_schemas()` + `validate()` | `master-data` | 0.5 | 3h |
| 2.2 | `generate_schemas()` → `quote!` codegen for structs | `master-data` | 2.1 | 4h |
| 2.3 | Codegen: positional `Deserialize` impl (rmp-serde) | `master-data` | 2.2 | 4h |
| 2.4 | Codegen: enum `From<i32>` + `#[derive(...)]` | `master-data` | 2.2 | 2h |
| 2.5 | `build.rs` → call parse + codegen, write to `generated/` | `master-data` | 2.1–2.4 | 1h |
| 2.6 | `decrypt()` — AES-128-CBC + PKCS7 unpad | `master-data` | 0.5 | 1h |
| 2.7 | `parse_toc()` — msgpack map → table offsets | `master-data` | 2.6 | 1h |
| 2.8 | `decompress_table()` — LZ4 ext type detection + decompression | `master-data` | 2.7 | 1h |
| 2.9 | `load_catalogs()` — full pipeline | `master-data` | 2.6–2.8 | 1h |
| 2.10 | `reload()` — rebuild + atomic swap + mtime bump | `master-data` | 2.9 | 30min |
| 2.11 | CLI: `dump`, `patch`, `inspect`, `validate`, `search` | `master-data` | 2.6–2.9 | 3h |
| 2.12 | CLI: `gen-entities` subcommand | `master-data` | 2.5 | 1h |
| 2.13 | Binary target `blackbox-masterdata` | `master-data` | 2.11–2.12 | 30min |
| 2.14 | Unit tests: decrypt, parse, decompress, round-trip | `master-data` | 2.6–2.9 | 3h |
| 2.15 | Integration test: load real `.bin.e`, assert table counts | `master-data` | 2.14 | 2h |

**Exit criterion:** `cargo run -p blackbox-masterdata -- dump --input <file>` works. 95%+ coverage.

---

## 🏗️ Phase 3 — `blackbox-store`

**Goal:** SQLite user persistence. `UserState` CRUD + session management. Migrations.

| # | Task | Crate(s) | Depends On | Est. |
|---|------|----------|------------|------|
| 3.1 | `UserState` struct (123 fields, matching INTERFACES.md exactly) | `store` | 0.4 | 2h |
| 3.2 | `UserState::ensure_maps()` lazy initialization | `store` | 3.1 | 30min |
| 3.3 | `SessionState` struct | `store` | 0.4 | 15min |
| 3.4 | `UserRepository` trait | `store` | 0.4 | 30min |
| 3.5 | `SessionRepository` trait | `store` | 0.4 | 15min |
| 3.6 | SQL schema design + migration files | `store` | 3.1 | 3h |
| 3.7 | `SqliteStore` impl — `create_user`, `get_user_by_uuid` | `store` | 3.4–3.6 | 2h |
| 3.8 | `SqliteStore` impl — `load_user`, `update_user` | `store` | 3.7 | 2h |
| 3.9 | `SqliteStore` impl — `set_facebook_id`, `get_user_by_facebook_id` | `store` | 3.7 | 1h |
| 3.10 | `SqliteStore` impl — `create_session`, `resolve_user_id` | `store` | 3.5–3.6 | 1h |
| 3.11 | JSON blob serialization for map fields | `store` | 3.8 | 2h |
| 3.12 | `sqlx::migrate!()` embedded migrations | `store` | 3.6 | 30min |
| 3.13 | Unit tests: CRUD, session, concurrent updates | `store` | 3.7–3.11 | 3h |
| 3.14 | Integration test: create → load → update → reload cycle | `store` | 3.13 | 1h |

**Exit criterion:** `cargo test -p blackbox-store` passes with in-memory SQLite. 85%+ coverage.

---

## 🏗️ Phase 4 — `blackbox-auth`

**Goal:** Auth library. Token signing, Facebook resolve, AuthStore CRUD.

| # | Task | Crate(s) | Depends On | Est. |
|---|------|----------|------------|------|
| 4.1 | `TokenService` — HMAC-SHA256 sign/validate | `auth` | 0.7 | 2h |
| 4.2 | `AuthStore` — SQLite `auth.db` CRUD | `auth` | 0.7, 3.6 | 1h |
| 4.3 | `resolve_facebook_token()` — HTTP call to Graph API | `auth` | 0.7 | 2h |
| 4.4 | `AuthError` enum | `auth` | 0.7 | 30min |
| 4.5 | Unit tests: token sign/validate, expiry, tamper | `auth` | 4.1–4.4 | 2h |
| 4.6 | Unit tests: Facebook mock (success, network error) | `auth` | 4.3 | 1h |
| 4.7 | Integration test: AuthStore CRUD | `auth` | 4.2 | 30min |

**Exit criterion:** `cargo test -p blackbox-auth` passes. 80%+ coverage.

---

## 🏗️ Phase 5 — `blackbox-diff-sync`

**Goal:** Event-driven incremental state sync. `DiffEntry`, `DiffSet`, protobuf serialization.

| # | Task | Crate(s) | Depends On | Est. |
|---|------|----------|------------|------|
| 5.1 | `DiffEntry` struct + `DiffAction` enum | `diff-sync` | 0.6 | 30min |
| 5.2 | `DiffSet` accumulator (`push`, `into_protobuf`) | `diff-sync` | 5.1 | 1h |
| 5.3 | `key_fields_for_table()` — 80+ table key definitions | `diff-sync` | 0.6 | 2h |
| 5.4 | Protobuf `DiffData` message definition | `diff-sync` | 0.6 | 1h |
| 5.5 | `into_protobuf()` → `HashMap<String, DiffData>` | `diff-sync` | 5.2, 5.4 | 1h |
| 5.6 | Unit tests: push, accumulate, empty, round-trip | `diff-sync` | 5.1–5.5 | 2h |
| 5.7 | Unit tests: key_fields correctness for key tables | `diff-sync` | 5.3 | 1h |

**Exit criterion:** `cargo test -p blackbox-diff-sync` passes. 95%+ coverage.

---

## 🏗️ Phase 6 — Proto Definitions

**Goal:** All `.proto` files compiled. `octo-proto` + `apb/api/` services.

| # | Task | Crate(s) | Depends On | Est. |
|---|------|----------|------------|------|
| 6.1 | `proto/octo.proto` — `Database`, `Data`, `Url`, `UrlList` | `octo-proto` | 0.13 | 1h |
| 6.2 | `proto/apb/api/user.proto` — UserService | `game-server` | 0.8 | 1h |
| 6.3 | `proto/apb/api/quest.proto` — QuestService | `game-server` | 0.8 | 1h |
| 6.4 | `proto/apb/api/gacha.proto` — GachaService | `game-server` | 0.8 | 1h |
| 6.5 | `proto/apb/api/battle.proto` — BattleService | `game-server` | 0.8 | 1h |
| 6.6 | `proto/apb/api/config.proto` — ConfigService | `game-server` | 0.8 | 1h |
| 6.7 | `proto/apb/api/data.proto` — DataService + DiffData | `game-server` | 0.8 | 1h |
| 6.8 | `proto/apb/api/tutorial.proto` — TutorialService | `game-server` | 0.8 | 1h |
| 6.9 | `proto/apb/api/gift.proto` — GiftService | `game-server` | 0.8 | 1h |
| 6.10 | `proto/apb/api/gameplay.proto` — GamePlayService | `game-server` | 0.8 | 1h |
| 6.11 | `proto/apb/api/gimmick.proto` — GimmickService | `game-server` | 0.8 | 1h |
| 6.12 | `proto/apb/api/notification.proto` — NotificationService | `game-server` | 0.8 | 1h |
| 6.13 | `proto/apb/api/cageornament.proto` — CageOrnamentService | `game-server` | 0.8 | 1h |
| 6.14 | `proto/apb/api/deck.proto` — DeckService | `game-server` | 0.8 | 1h |
| 6.15 | `proto/apb/api/friend.proto` — FriendService | `game-server` | 0.8 | 1h |
| 6.16 | `proto/apb/api/loginbonus.proto` — LoginBonusService | `game-server` | 0.8 | 1h |
| 6.17 | `proto/apb/api/navicutin.proto` — NaviCutInService | `game-server` | 0.8 | 1h |
| 6.18 | `proto/apb/api/contentsstory.proto` — ContentsStoryService | `game-server` | 0.8 | 1h |
| 6.19 | `proto/apb/api/dokan.proto` — DokanService | `game-server` | 0.8 | 1h |
| 6.20 | `proto/apb/api/portalcage.proto` — PortalCageService | `game-server` | 0.8 | 1h |
| 6.21 | `proto/apb/api/characterviewer.proto` — CharacterViewerService | `game-server` | 0.8 | 1h |
| 6.22 | `proto/apb/api/mission.proto` — MissionService | `game-server` | 0.8 | 1h |
| 6.23 | `proto/apb/api/shop.proto` — ShopService | `game-server` | 0.8 | 1h |
| 6.24 | `proto/apb/api/costume.proto` — CostumeService | `game-server` | 0.8 | 1h |
| 6.25 | `proto/apb/api/movie.proto` — MovieService | `game-server` | 0.8 | 1h |
| 6.26 | `proto/apb/api/omikuji.proto` — OmikujiService | `game-server` | 0.8 | 1h |
| 6.27 | `proto/apb/api/weapon.proto` — WeaponService | `game-server` | 0.8 | 1h |
| 6.28 | `proto/apb/api/explore.proto` — ExploreService | `game-server` | 0.8 | 1h |
| 6.29 | `proto/apb/api/characterboard.proto` — CharacterBoardService | `game-server` | 0.8 | 1h |
| 6.30 | `proto/apb/api/parts.proto` — PartsService | `game-server` | 0.8 | 1h |
| 6.31 | `proto/apb/api/character.proto` — CharacterService | `game-server` | 0.8 | 1h |
| 6.32 | `proto/apb/api/companion.proto` — CompanionService | `game-server` | 0.8 | 1h |
| 6.33 | `proto/apb/api/material.proto` — MaterialService | `game-server` | 0.8 | 1h |
| 6.34 | `proto/apb/api/consumableitem.proto` — ConsumableItemService | `game-server` | 0.8 | 1h |
| 6.35 | `proto/apb/api/sidestoryquest.proto` — SideStoryQuestService | `game-server` | 0.8 | 1h |
| 6.36 | `proto/apb/api/bighunt.proto` — BigHuntService | `game-server` | 0.8 | 1h |
| 6.37 | `proto/apb/api/reward.proto` — RewardService | `game-server` | 0.8 | 1h |
| 6.38 | `proto/apb/api/labyrinth.proto` — LabyrinthService | `game-server` | 0.8 | 1h |
| 6.39 | `proto/apb/api/banner.proto` — BannerService | `game-server` | 0.8 | 1h |
| 6.40 | `proto/apb/api/admin.proto` — AdminService | `admin` | 0.10 | 1h |
| 6.41 | `build.rs` for `game-server` — tonic-build for all 38 services | `game-server` | 6.2–6.39 | 2h |
| 6.42 | Verify `cargo build -p blackbox-game-server` compiles all services | `game-server` | 6.41 | 30min |

**Exit criterion:** All 38+ services compile. `cargo build --workspace` succeeds.

---

## 🏗️ Phase 7 — `blackbox-game-server` (Core Engine)

**Goal:** Kameo actors, event bus, command layer, interceptor stack. All 38 gRPC services wired.

| # | Task | Crate(s) | Depends On | Est. |
|---|------|----------|------------|------|
| 7.1 | Supervisor actor + actor lifecycle management | `game-server` | 6.41 | 3h |
| 7.2 | Event bus (kameo event dispatcher) | `game-server` | 6.41 | 2h |
| 7.3 | Command layer scaffold (emit events, collect diffs) | `game-server` | 7.2 | 2h |
| 7.4 | `UserService` actor — auth, register, transfer, setName | `game-server` | 7.1, 4.1 | 3h |
| 7.5 | `QuestService` actor — start, end, progress | `game-server` | 7.1 | 3h |
| 7.6 | `GachaService` actor — draw, banner state | `game-server` | 7.1 | 3h |
| 7.7 | `RewardEventHandler` — grant items/gold/exp | `game-server` | 7.2 | 2h |
| 7.8 | `QuestEventHandler` — quest progress + reward emission | `game-server` | 7.2 | 2h |
| 7.9 | `GachaEventHandler` — draw logic + banner update | `game-server` | 7.2 | 2h |
| 7.10 | `DeckEventHandler` — validate constraints, update state | `game-server` | 7.2 | 2h |
| 7.11 | `StaminaEventHandler` — consume/refill timers | `game-server` | 7.2 | 2h |
| 7.12 | `InventoryEventHandler` — item/costume/weapon tracking | `game-server` | 7.2 | 2h |
| 7.13 | `AchievementEventHandler` — milestone checks | `game-server` | 7.2 | 2h |
| 7.14 | `DiffEventHandler` — subscribe, accumulate, attach to response | `game-server` | 7.2, 5.1 | 2h |
| 7.15 | Interceptor: Platform extraction | `game-server` | 6.41 | 1h |
| 7.16 | Interceptor: Logging | `game-server` | 7.15 | 30min |
| 7.17 | Interceptor: Diff collection | `game-server` | 7.14 | 1h |
| 7.18 | Interceptor: TimeSync trailer | `game-server` | 7.15 | 30min |
| 7.19 | Remaining 30 gRPC service actors (thin impls) | `game-server` | 7.1 | 10h |
| 7.20 | Integration test: Register → Auth → GameStart → GetUserProfile | `game-server` | 7.4–7.19 | 3h |
| 7.21 | Integration test: Quest completion → reward events | `game-server` | 7.5, 7.7–7.8 | 2h |
| 7.22 | Integration test: Gacha draw → banner state update | `game-server` | 7.6, 7.9 | 2h |
| 7.23 | Integration test: Stamina consumed on quest start | `game-server` | 7.11 | 1h |

**Exit criterion:** `cargo test -p blackbox-game-server` passes. All 38 services callable via tonic test client. 80%+ coverage.

---

## 🏗️ Phase 8 — `blackbox-assets-server`

**Goal:** HTTP asset CDN. list.bin serving, URL rewriting, asset bundles, MD5 validation.

| # | Task | Crate(s) | Depends On | Est. |
|---|------|----------|------------|------|
| 8.1 | Axum router: `/v2/.../list/`, `/v1/list/`, `/v2/.../info` | `assets-server` | 0.9 | 2h |
| 8.2 | `AssetResolver` — object ID → filesystem path + MD5 | `assets-server` | 0.9 | 2h |
| 8.3 | `RevisionTracker` — per-client active revision | `assets-server` | 0.9 | 1h |
| 8.4 | `list.bin` protobuf serving with URL rewriting (43-byte) | `assets-server` | 8.1–8.2 | 3h |
| 8.5 | Asset bundle serving (`unso-*` paths) | `assets-server` | 8.2 | 1h |
| 8.6 | Static HTML pages | `assets-server` | 8.1 | 30min |
| 8.7 | h2c support | `assets-server` | 8.1 | 30min |
| 8.8 | Unit tests: list.bin, URL rewrite, MD5 validation | `assets-server` | 8.4–8.5 | 2h |
| 8.9 | Integration test: full asset flow | `assets-server` | 8.8 | 1h |

**Exit criterion:** `cargo test -p blackbox-assets-server` passes. 75%+ coverage.

---

## 🏗️ Phase 9 — `blackbox-admin`

**Goal:** Admin API. Reload endpoint with constant-time auth.

| # | Task | Crate(s) | Depends On | Est. |
|---|------|----------|------------|------|
| 9.1 | `POST /api/admin/master-data/reload` | `admin` | 0.10 | 2h |
| 9.2 | Bearer token check (constant-time, fail-closed) | `admin` | 0.10 | 1h |
| 9.3 | `BLACKBOX_ADMIN_TOKEN` env var integration | `admin` | 9.2 | 30min |
| 9.4 | Health check + metrics endpoints | `admin` | 0.10 | 1h |
| 9.5 | Unit tests: valid/invalid token, no-env fail-closed | `admin` | 9.1–9.3 | 1h |
| 9.6 | Integration test: reload triggers catalog swap | `admin` | 9.1, 2.10 | 1h |

**Exit criterion:** `cargo test -p blackbox-admin` passes. 90%+ coverage.

---

## 🏗️ Phase 10 — `blackbox-cli` + `blackbox-dev`

**Goal:** Dev experience. CLI wizard, dev runner, production single-service mode.

| # | Task | Crate(s) | Depends On | Est. |
|---|------|----------|------------|------|
| 10.1 | `blackbox wizard` — interactive setup | `cli` | 0.11 | 2h |
| 10.2 | `blackbox dev` — spawn all services locally | `cli` | 0.11 | 2h |
| 10.3 | `blackbox serve` — production single-service mode | `cli` | 0.11 | 1h |
| 10.4 | `blackbox-masterdata` CLI wrapper (dump/patch/validate) | `cli` | 2.11 | 30min |
| 10.5 | `blackbox-dev` binary — in-process multi-service + file watcher | `dev` | 0.12 | 3h |
| 10.6 | `.bin.e` change detection → auto-reload | `dev` | 10.5 | 1h |

**Exit criterion:** `cargo run -p blackbox-dev` starts all services. File watcher triggers reload.

---

## 🏗️ Phase 11 — Integration Test Suite

**Goal:** End-to-end flows. Full game loop, hot reload, concurrent load.

| # | Task | Crate(s) | Depends On | Est. |
|---|------|----------|------------|------|
| 11.1 | Full game flow integration test (Register → Auth → Quest → Gacha → Verify) | `tests/` | 7.20 | 4h |
| 11.2 | Hot reload flow (start → query → reload → query changed) | `tests/` | 9.6 | 2h |
| 11.3 | Concurrent load test (N clients, random operations) | `tests/` | 7.20 | 4h |
| 11.4 | Snapshot tests: table row counts, diff output, response shapes | `tests/` | 2.15 | 2h |
| 11.5 | Fuzz harnesses (decrypt, parse_toc, changed_tables) | `tests/` | 2.6, 5.1 | 2h |

**Exit criterion:** `cargo test --workspace` green. All integration tests pass.

---

## 📊 Dependency Graph (Phase Order)

```
Phase 0 (Scaffolding)
    │
    ├──► Phase 1 (Core + Observability)
    │         │
    │         ├──► Phase 2 (Master Data)
    │         │         │
    │         │         ├──► Phase 3 (Store)
    │         │         │       │
    │         │         │       ├──► Phase 4 (Auth)
    │         │         │       │       │
    │         │         │       │       ├──► Phase 5 (Diff Sync)
    │         │         │       │       │       │
    │         │         │       │       │       ├──► Phase 6 (Protos)
    │         │         │       │       │       │       │
    │         │         │       │       │       │       ├──► Phase 7 (Game Server)
    │         │         │       │       │       │       │       │
    │         │         │       │       │       │       │       ├──► Phase 8 (Assets)
    │         │         │       │       │       │       │       │       │
    │         │         │       │       │       │       │       │       ├──► Phase 9 (Admin)
    │         │         │       │       │       │       │       │       │       │
    │         │         │       │       │       │       │       │       │       ├──► Phase 10 (CLI/Dev)
    │         │         │       │       │       │       │       │       │       │       │
    │         │         │       │       │       │       │       │       │       │       ▼
    │         │         │       │       │       │       │       │       │       │   Phase 11 (Integration)
```

---

## 🔑 Critical Path

```
0 → 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9 → 10 → 11
```

Phases 8 and 9 can run in parallel after Phase 6.
Phase 10 depends on all server crates (7, 8, 9).

---

## 📈 Estimated Totals

| Phase | Effort | Crates Touched |
|-------|--------|----------------|
| 0 — Scaffolding | ~4h | all |
| 1 — Core + Observability | ~5h | `core`, `observability` |
| 2 — Master Data | ~22h | `master-data` |
| 3 — Store | ~15h | `store` |
| 4 — Auth | ~7h | `auth` |
| 5 — Diff Sync | ~7h | `diff-sync` |
| 6 — Protos | ~12h | `game-server`, `admin` |
| 7 — Game Server | ~45h | `game-server` |
| 8 — Assets Server | ~10h | `assets-server` |
| 9 — Admin | ~5h | `admin` |
| 10 — CLI + Dev | ~9h | `cli`, `dev` |
| 11 — Integration | ~14h | `tests/` |
| **Total** | **~155h** | **all** |

---

## ✅ Readiness Checklist

Before starting any dev work, confirm:

- [x] `AGENTS.md` — project conventions documented
- [x] `docs/COMPONENTS.md` — 12 crates + tools defined
- [x] `docs/INTERFACES.md` — trait/struct/gRPC contracts specified
- [x] `docs/TESTING.md` — per-crate test matrices defined
- [x] Go reference source fully read and validated
- [x] `schemas.json` format documented and parser design decided
- [x] Diff-sync approach (Approach C — inline delta) decided
- [x] Auth as library (not standalone server) decided
- [x] 38 gRPC services identified and listed
- [x] `UserState` 123 fields matched exactly
- [ ] Workspace `Cargo.toml` created (Phase 0, Task 0.1)
- [ ] First crate skeleton exists
- [ ] `cargo check --workspace` passes

**Status: READY FOR PHASE 0**
