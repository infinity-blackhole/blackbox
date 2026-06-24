# Component Schema

## Workspace Structure

```text
blackbox/
├── Cargo.toml                    # workspace root
├── AGENTS.md
├── flake.nix                     # Nix dev shell
├── Makefile                      # Build/test/deploy shortcuts
├── config/
│   └── default.toml              # Default configuration
├── proto/
│   ├── apb/api/                  # gRPC service definitions
│   │   ├── user.proto
│   │   ├── quest.proto
│   │   ├── gacha.proto
│   │   └── ... (28 services)
│   └── octo.proto                # Asset catalog protobuf
├── schemas.json                  # Single source of truth for all 607 entity schemas
├── crates/
│   ├── blackbox-core/            # Shared types, errors, config, clock
│   ├── blackbox-master-data/     # Schema codegen, binary parsing, master data
│   │                            # CLI
│   ├── blackbox-diff-sync/       # Event-driven incremental state sync
│   ├── blackbox-store/           # sqlx SQLite user data persistence
│   ├── blackbox-auth/            # Auth library (token validation, Facebook resolve)
│   ├── blackbox-game-server/     # tonic gRPC game server + kameo actors
│   ├── blackbox-assets-server/   # axum HTTP asset CDN
│   ├── blackbox-admin/           # Admin API (hot reload)
│   ├── blackbox-observability/   # OpenTelemetry + tracing
│   ├── blackbox-cli/             # CLI (wizard, dev)
│   └── blackbox-dev/             # Dev runner (spawns all services)
```

## Schema Definition

All entity types are defined in a single `schemas.json` file at the project
root. This file is the single source of truth for all 607 master data tables.

### Format

```json
{
  "m_quest": {
    "class": "EntityMQuest",
    "columns": [
      [0, "int", "QuestId"],
      [1, "int", "NameQuestTextId"],
      [2, "int", "PictureBookNameQuestTextId"],
      [3, "QuestType", "QuestType"],
      [4, "int", "ChapterId"],
      [5, "DifficultyType", "DifficultyType"]
    ]
  }
}
```

Each entry: `{ class: "EntityMXxx", columns: [[index, type, name], ...] }`

- **index**: positional index in the msgpack array (maps directly to column
  position)
- **type**: `"int"` → `i32`, `"long"` → `i64`, `"string"` → `String`, `"bool"` →
  `bool`, `"float"` → `f32`, `"double"` → `f64`, enum name → that enum type
- **name**: field name in the generated Rust struct

### Type Mapping

| JSON Type    | Rust Type  | Notes                                                    |
| ------------ | ---------- | -------------------------------------------------------- |
| `"int"`      | `i32`      |                                                          |
| `"long"`     | `i64`      | Used for timestamps (Datetime fields)                    |
| `"string"`   | `String`   |                                                          |
| `"bool"`     | `bool`     |                                                          |
| `"float"`    | `f32`      |                                                          |
| `"double"`   | `f64`      |                                                          |
| `"EnumName"` | `EnumName` | Generates `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` |

### Schema Parsing & Code Generation

```text
schemas.json (single file, 607 tables)
    ↓ parsed by blackbox-master-data/src/schema/parser.rs
SchemaIR (intermediate representation)
    ↓ codegen by blackbox-master-data/src/schema/codegen.rs
    ↓
crates/blackbox-master-data/src/schema/generated/structs.rs
        (struct + enum definitions)
crates/blackbox-master-data/src/schema/generated/deserialize.rs
        (positional msgpack Deserialize)
crates/blackbox-master-data/src/schema/generated/enums.rs
        (From<i32> for enum types)
    ↓
crates/blackbox-master-data/src/schema/mod.rs (include! all generated)
```

The generated `Deserialize` impl reads msgpack arrays positionally by index,
matching the binary format exactly. The generated code is written to
`src/schema/generated/` (gitignored) and included at compile time. This gives
full rust-analyzer support with autocomplete and type checking.

The `tools/gen-entities` crate provides a standalone binary for offline code
generation (CI checks, IDE integration).

---

## Component Responsibilities

### `blackbox-core` — Foundation

**Dependencies:** `tokio`, `bytes`, `serde`, `thiserror`, `tracing`, `config`,
`time`

**Provides:**

- `AppConfig` — server addresses, ports, database paths, admin token, master
  data path. Loaded from TOML + env vars via `config` crate.
- `MasterDataCatalogs` — the hot-swappable aggregate of all loaded catalogs.
  Stored as `Arc<MasterDataCatalogs>` behind a `tokio::sync::watch` channel.
- `GameClock` — `fn now() -> OffsetDateTime` / `fn now_millis() -> i64`.
  Injectable for tests (system clock vs. frozen clock).
- `LunarError` — top-level error enum with `thiserror`, covering crypto, IO, DB,
  gRPC, deserialization, and domain errors.
- Constants: AES key/IV, LZ4 ext code (99), original resource URL string.

**Relationships:** Depended on by ALL other crates. Zero internal dependencies.

---

### `blackbox-master-data` — Schema Codegen, Binary Parsing, Master Data CLI

**Dependencies:** `aes`, `cbc`, `lz4`, `rmp-serde`, `serde`, `serde_json`,
`bytes`, `blackbox-core`, `quote`, `syn`, `proc-macro2`, `clap`

**Provides:**

Three modules in one crate:

**`blackbox_master_data::schema`** — Schema parsing and Rust code generation:

- `parse_schemas(path: &Path) -> Result<SchemaIR>` — reads `schemas.json`,
  validates, produces IR
- `generate_schemas(ir: &SchemaIR) -> TokenStream` — generates Rust source
  (structs + Deserialize)
- `SchemaIR` — intermediate representation (`tables: Vec<TableDef>`,
  `enums: Vec<EnumDef>`)
- `TableDef` — `table_key`, `class_name`, `columns: Vec<ColumnDef>`
- `ColumnDef` — `index: u32`, `name: String`, `raw_type: String`,
  `is_enum: bool`
- `EnumDef` — `name`, `variants: Vec<(String, i32)>`
- `validate()` — checks for duplicate table keys, duplicate column indices,
  unknown enum references

**`blackbox_master_data::binary`** — Binary parsing pipeline:

- `decrypt(data: &[u8]) -> Vec<u8>` — AES-128-CBC, PKCS7 unpad, hardcoded key/IV
- `parse_toc(data: &[u8]) -> HashMap<String, (usize, usize)>` — msgpack map →
  table offsets
- `decompress_table(raw: &[u8]) -> Result<Vec<Vec<serde_json::Value>>>` — detect
  LZ4 ext type (code 99), decompress, msgpack deserialize
- `load_catalogs(path: &Path) -> Result<MasterDataCatalogs>` — full pipeline
- `reload(path: &Path, sender: &watch::Sender>)` — rebuild + atomic swap + mtime
  bump

**`blackbox_master_data::cli`** — CLI commands:

- `dump --input bin.e/ --output ./dump/` — dump all tables to JSON files
- `patch --input bin.e/ --output patched.bin.e` — extend time-gated content to
  2030
- `inspect --input bin.e/ --table m_quest` — print table summary
- `validate --input bin.e/` — validate binary integrity
- `search --input bin.e/ --table m_quest --column QuestId --value 101` — search
  rows
- `gen-entities --input schemas.json --output src/schema/generated/` — generate
  Rust code from schemas

**Binary target:** `blackbox-masterdata` — single binary with all subcommands.
The `gen-entities` subcommand is the standalone code generator for CI and IDE
integration.

**Build script (`build.rs`):**

- Calls `parse_schemas("schemas.json")` → `generate_schemas()` → writes
  generated code to `src/schema/generated/`
- Triggers re-run when `schemas.json` changes

**Relationships:** Depends on `blackbox-core`. Used by `blackbox-game-server`
(startup, reload), `blackbox-admin` (reload webhook), and `blackbox-cli` (CLI
binary).

---

### `blackbox-diff-sync` — Event-Driven Incremental State Sync

**Dependencies:** `serde`, `serde_json`, `prost`, `blackbox-core`,
`blackbox-store`

**Provides:**

- `DiffEntry` — a single table mutation descriptor: table name, action
  (insert/update/delete), key fields, changed values.
- `DiffSet` — accumulator for `DiffEntry` across event handlers within one
  request cycle.
- `into_protobuf(self) -> HashMap<String, DiffData>` — serializes the
  accumulated diff to the wire-format `DiffData` protobuf.
- `key_fields_for_table(table: &TableId) -> Option<&[&str]>` — composite key
  definitions for 80+ tables.

**Design — Approach C (Per-Event Inline Delta):**

The diff-sync crate does NOT compute diffs by comparing before/after snapshots.
Instead, each kameo event handler returns `Vec<DiffEntry>` alongside its state
mutation. The command handler accumulates these into a `DiffSet` and attaches it
to the gRPC response.

Flow:

```text
1. gRPC command handler receives request
2. Validates request (stamina, prerequisites, etc.)
3. Emits GameEvent to kameo event bus
4. Event handlers process the event:
   - Update DB (store)
   - Return Vec<DiffEntry> describing what changed
5. Handler accumulates all DiffEntry into DiffSet
6. DiffSet::into_protobuf() → HashMap<String, DiffData>
7. Attach to gRPC response + x-apb-update-user-data-names trailer
```

**Relationships:** Depends on `blackbox-core`, `blackbox-store`. Used by
`blackbox-game-server` (event handlers and command layer).

---

### `blackbox-store` — User Data Persistence

**Dependencies:** `sqlx` (runtime-tokio, sqlite, migrate), `tokio`, `serde`,
`blackbox-core`

**Provides:**

- `UserRepository` trait — `create_user`, `get_user_by_uuid`, `load_user`,
  `update_user`, `set_facebook_id`, etc.
- `SessionRepository` trait — `create_session`, `resolve_user_id`.
- `SqliteStore` — sqlx-backed implementation of both traits.
- `UserState` — the ~120-field aggregate. All map fields are `HashMap<K, V>`
  with `ensure_maps()` lazy initialization.
- `SessionState` — session key, user ID, UUID, expiry.
- Embedded migrations via `sqlx::migrate!()`.

**Storage model:** Single row per user with JSON blob columns for nested
structures. Indexed: `uuid`, `facebook_id`, `player_id`.

**Relationships:** Depends on `blackbox-core`. Used by `blackbox-game-server`,
`blackbox-auth`, `blackbox-diff-sync`.

---

### `blackbox-auth` — Auth Library

**Dependencies:** `sqlx` (runtime-tokio, sqlite), `tokio`, `hmac`, `sha2`,
`blackbox-core`

**Provides:**

- `TokenService` — HMAC-based token signing/validation.
- `AuthStore` — sqlx SQLite for auth accounts (separate `auth.db`).
- `resolve_facebook_token(token: &str) -> Result<FacebookId>` — calls Facebook
  Graph API to validate tokens and extract the user ID.
- `AuthError` — token validation failures (expired, invalid, network error).

**Design:** Pure library — no HTTP server. Embedded directly into
`blackbox-game-server`. The `AuthStore` uses the same `auth.db` SQLite database.
The `resolve_facebook_token` function performs the HTTP call to Facebook's `/me`
endpoint internally rather than delegating to a separate auth server.

**Relationships:** Depends on `blackbox-core`. Used by `blackbox-game-server`
(UserService actor).

---

### `blackbox-game-server` — Game Server

**Dependencies:** `tonic`, `prost`, `tokio`, `kameo`, `tracing`,
`blackbox-core`, `blackbox-store`, `blackbox-diff-sync`, `blackbox-master-data`,
`blackbox-auth`

**Provides:**

- 38 gRPC service implementations as kameo actors, each holding a
  `watch::Receiver` for hot reloads and a `SqliteStore` reference.
- **Event bus** — central kameo event dispatcher. Command handlers emit
  `GameEvent`, event handlers process asynchronously and return
  `Vec<DiffEntry>`.
- **Event handlers** — specialized actors for each domain:
  - `QuestEventHandler` — updates quest progress, emits reward events on
    completion
  - `RewardEventHandler` — grants items, gold, exp, materials
  - `GachaEventHandler` — processes gacha draws, updates banner states, grants
    items
  - `DeckEventHandler` — validates deck constraints, updates deck state
  - `StaminaEventHandler` — checks/consume stamina, handles refill timers
  - `AchievementEventHandler` — checks milestones, grants achievement rewards
  - `InventoryEventHandler` — tracks item counts, weapon/costume/character
    states
  - `DiffEventHandler` — subscribes to events, accumulates diff entries
- **Command layer** — gRPC handlers that validate, emit events, collect diffs,
  return responses.
- Interceptor stack (tower middleware):
  1. **Platform** — parse `x-apb-platform` header.
  2. **Logging** — `tracing::info!` per RPC.
  3. **Diff** — collect `DiffEntry` from event handlers, attach to response.
  4. **TimeSync** — attach `x-apb-response-datetime` trailer.
- `CurrentUserId` — extract session key from gRPC metadata → resolve via
  `SessionRepository`.
- `UserService` — uses `blackbox-auth::AuthStore` and
  `blackbox-auth::resolve_facebook_token` directly.
- Supervisor actor managing all service and event handler actor lifecycles.

**Event flow:**

```text
gRPC Request
  → Command Handler (validate + emit GameEvent)
    → Event Bus dispatches to handlers
      → QuestEventHandler: store.update_quest() + returns diff entries
      → RewardEventHandler: store.grant_items() + returns diff entries
      → StaminaEventHandler: store.consume_stamina() + returns diff entries
    → Command Handler collects Vec<DiffEntry>
    → DiffSet::into_protobuf() → attach to response
  → gRPC Response with DiffUserData
```

**Actor hierarchy:**

```text
GameServer (supervisor)
├── UserService (gRPC + kameo actor)
├── QuestService (gRPC + kameo actor)
├── ... (38 gRPC service actors)
├── QuestEventHandler (event handler)
├── RewardEventHandler (event handler)
├── GachaEventHandler (event handler)
├── StaminaEventHandler (event handler)
├── InventoryEventHandler (event handler)
├── AchievementEventHandler (event handler)
└── DiffEventHandler (event handler)
```

**Relationships:** Depends on `blackbox-core`, `blackbox-store`,
`blackbox-diff-sync`, `blackbox-master-data`, `blackbox-auth`. Top-level server
— no reverse dependencies.

---

### `blackbox-assets-server` — Asset CDN

**Dependencies:** `axum`, `tokio`, `bytes`, `http`, `tower`, `tower-http`,
`tracing`, `blackbox-core`, `octo-proto`

**Provides:**

- Axum router: `/v2/.../list/`, `/v1/list/`, `/v2/.../info`, `/master-data/*`,
  `unso-*` asset bundles, static HTML pages.
- `RevisionTracker` — per-client active revision.
- `AssetResolver` — object ID → filesystem path, MD5 validation.
- `list.bin` URL replacement (43-byte constraint).
- h2c support.

**Relationships:** Depends on `blackbox-core`, `octo-proto`. Standalone service.

---

### `blackbox-admin` — Admin API

**Dependencies:** `axum`, `tokio`, `blackbox-core`, `blackbox-master-data`

**Provides:**

- `POST /api/admin/master-data/reload` — constant-time Bearer token check, calls
  `master_data::reload()`.
- Only binds when `BLACKBOX_ADMIN_TOKEN` env var is set (fail-closed).
- Defaults to `127.0.0.1:8082`.

**Relationships:** Depends on `blackbox-core`, `blackbox-master-data`.
Standalone.

---

### `octo-proto` — Generated Protobuf

**Dependencies:** `prost`, `prost-types`

**Provides:** `Database`, `Data`, `Url`, `UrlList` from `octo.proto`.

**Relationships:** Depended on by `blackbox-assets-server`. No internal
dependencies.

---

### `blackbox-observability` — OpenTelemetry

**Dependencies:** `opentelemetry`, `opentelemetry_sdk`, `tracing-opentelemetry`,
`tracing`

**Provides:**

- `init_tracing(config) -> Result<Guard>` — initializes OpenTelemetry tracing
  with OTLP exporter or stdout fallback.
- Reusable span macros.

**Relationships:** Depended on by `blackbox-game-server`,
`blackbox-assets-server`, `blackbox-admin`, `blackbox-cli`.

---

### `blackbox-cli` — CLI Tools

**Dependencies:** `clap`, `tracing`, `blackbox-core`, `blackbox-master-data`,
`blackbox-observability`

**Provides:**

- `blackbox wizard` — interactive setup.
- `blackbox dev` — spawns all services locally.
- `blackbox serve` — production single-service mode.

**Relationships:** Depends on `blackbox-core`, `blackbox-master-data`,
`blackbox-observability`. Spawns `blackbox-game-server`,
`blackbox-assets-server`, `blackbox-admin` as subprocesses or in-process.

---

### `blackbox-dev` — Dev Runner

**Dependencies:** `tokio`, `blackbox-game-server`, `blackbox-assets-server`,
`blackbox-admin`

**Provides:**

- Single binary that spawns all three services in-process for local development.
- Watches for `.bin.e` changes and triggers reload. **Relationships:** Depends
  on all server crates. Entrypoint for `cargo run`.

---

## Dependency Graph

```text
                    ┌──────────────────┐
                    │    octo-proto     │
                    └────────┬─────────┘
                             │
                    ┌────────▼─────────┐    ┌────────────────────┐
                    │blackbox-master-data│   │blackbox-assets-server│
                    └────────┬─────────┘    └────────────────────┘
                             │
                    ┌────────▼─────────┐
                    │ blackbox-admin   │
                    └──────────────────┘

┌───────┬─────────┐    ┌──────────────────┐    ┌────────────────┐
│blackbox-store  │◄───│blackbox-game-server│──►│  blackbox-auth │
└───────┬─────────┘    └────────┬─────────┘    └────────────────┘
        │                       │
        │              ┌────────▼─────────┐
        └─────────────►│ blackbox-diff-sync│
                       └──────────────────┘

┌──────────────┐    ┌──────────────────┐    ┌──────────────────┐
│ blackbox-cli │    │blackbox-observability│  │  blackbox-core   │◄── all
└──────────────┘    └──────────────────┘    └──────────────────┘

Tool: gen-entities
(standalone wrapper around blackbox-master-data::schema, no reverse dependencies)
```

## Data Flow

```text
Client ──gRPC──► blackbox-game-server
                    ├──► Command Handler (validate + emit GameEvent)
                    │       ├──► QuestEventHandler → store + diff entries
                    │       ├──► RewardEventHandler → store + diff entries
                    │       └──► StaminaEventHandler → store + diff entries
                    ├──► blackbox-diff-sync (accumulate DiffEntry → DiffData)
                    ├──► blackbox-master-data (read catalogs via watch)
                    └──► blackbox-auth (validate FB token via Facebook Graph API)

Client ──HTTP──► blackbox-assets-server
                    ├──► list.bin (protobuf, URL-rewritten)
                    ├──► asset bundles (disk, MD5-validated)
                    └──► .bin.e (master data)

Admin ──HTTP──► blackbox-admin
    └──► blackbox-master-data::reload() → watch::send() →
        game-server actors
```

## Configuration

```toml
[game_server]
listen = "0.0.0.0:443"
public_addr = "127.0.0.1:443"

[assets_server]
listen = "0.0.0.0:8080"
public_addr = "127.0.0.1:8080"
assets_dir = "."

[admin]
listen = "127.0.0.1:8082"

[sqlite]
game_db = "db/game.db"
auth_db = "db/auth.db"

[master_data]
path = "assets/release/20240404193219.bin.e"
```

## Build & Run

```bash
cargo build --release
cargo run -p blackbox-game-server
cargo run -p blackbox-assets-server
cargo run -p blackbox-dev              # all services
cargo test --workspace
cargo run -p blackbox-masterdata -- dump --input
    assets/release/20240404193219.bin.e ./dump
cargo run -p blackbox-masterdata -- patch --input original.bin.e
    --output patched.bin.e
cargo run -p blackbox-masterdata -- inspect --input bin.e/
    --table m_quest
cargo run -p blackbox-masterdata -- validate --input bin.e/
```
