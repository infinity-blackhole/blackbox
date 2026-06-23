# Interface Contracts

## Trait Definitions

### `UserRepository`

```rust
#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create_user(&self, uuid: &str, platform: ClientPlatform) -> Result<i64>;
    async fn get_user_by_uuid(&self, uuid: &str) -> Result<i64>;
    async fn load_user(&self, user_id: i64) -> Result<UserState>;
    async fn update_user(&self, user_id: i64, f: impl FnOnce(&mut UserState)) -> Result<UserState>;
    async fn default_user_id(&self) -> Result<i64>;
    async fn set_facebook_id(&self, user_id: i64, fb_id: i64) -> Result<()>;
    async fn get_user_by_facebook_id(&self, fb_id: i64) -> Result<i64>;
    async fn get_facebook_id(&self, user_id: i64) -> Result<i64>;
    async fn clear_facebook_id(&self, user_id: i64) -> Result<()>;
    async fn update_uuid(&self, user_id: i64, new_uuid: &str) -> Result<()>;
}
```

### `SessionRepository`

```rust
#[async_trait]
pub trait SessionRepository: Send + Sync + 'static {
    async fn create_session(&self, uuid: &str, ttl: Duration) -> Result<SessionState>;
    async fn resolve_user_id(&self, session_key: &str) -> Result<i64>;
}
```

### `AssetResolver`

```rust
#[async_trait]
pub trait AssetResolver: Send + Sync + 'static {
    async fn resolve(
        &self,
        object_id: &str,
        asset_type: AssetType,
        revision: &str,
        platform: Platform,
    ) -> Result<AssetResolution>;

    async fn prewarm(&self, revision: &str, platform: Platform);
}
```

### `MasterDataSource`

```rust
pub trait MasterDataSource: Send + Sync + 'static {
    fn get(&self) -> Arc<MasterDataCatalogs>;
    async fn reload(&self) -> Result<()>;
}
```

## Key Structs

### `UserState`

```rust
pub struct UserState {
    pub user_id: i64,
    pub uuid: String,
    pub player_id: i64,
    pub os_type: i32,
    pub platform_type: i32,
    pub register_datetime: i64,
    pub game_start_datetime: i64,
    pub birth_year: i32,
    pub birth_month: i32,
    pub backup_token: String,
    pub charge_money_this_month: i64,
    pub facebook_id: i64,
    pub setting: UserSettingState,
    pub status: UserStatusState,
    pub gem: UserGemState,
    pub profile: UserProfileState,
    pub login: UserLoginState,
    pub login_bonus: UserLoginBonusState,
    pub tutorials: HashMap<i32, TutorialProgressState>,
    pub main_quest: MainQuestState,
    pub event_quest: EventQuestState,
    pub extra_quest: ExtraQuestState,
    pub side_story_quests: HashMap<i32, SideStoryQuestProgress>,
    pub characters: HashMap<i32, CharacterState>,
    pub costumes: HashMap<String, CostumeState>,
    pub weapons: HashMap<String, WeaponState>,
    pub companions: HashMap<String, CompanionState>,
    pub deck_characters: HashMap<String, DeckCharacterState>,
    pub decks: HashMap<DeckKey, DeckState>,
    pub quests: HashMap<i32, UserQuestState>,
    pub missions: HashMap<i32, UserMissionState>,
    pub consumable_items: HashMap<i32, i32>,
    pub materials: HashMap<i32, i32>,
    pub parts: HashMap<String, PartsState>,
    pub gimmick: GimmickState,
    pub gacha: GachaState,
    pub shop_items: HashMap<i32, UserShopItemState>,
    pub explore_scores: HashMap<i32, ExploreScoreState>,
    pub character_boards: HashMap<i32, CharacterBoardState>,
    pub big_hunt_progress: BigHuntProgress,
    // ... ~120 fields total
}

impl UserState {
    pub fn ensure_maps(&mut self);
}
```

### `MasterDataCatalogs`

```rust
pub struct MasterDataCatalogs {
    pub game_config: EntityMGameConfig,
    pub parts: PartsCatalog,
    pub quest: QuestCatalog,
    pub gacha_entries: Vec<GachaCatalogEntry>,
    pub gacha_medals: HashMap<i32, GachaMedalInfo>,
    pub gacha_pool: GachaCatalog,
    pub shop: ShopCatalog,
    pub dup_exchange: HashMap<i32, Vec<DupExchangeEntry>>,
    pub condition_resolver: ConditionResolver,
    pub cage_ornament: CageOrnamentCatalog,
    pub login_bonus: LoginBonusCatalog,
    pub character_viewer: CharacterViewerCatalog,
    pub omikuji: OmikujiCatalog,
    pub material: MaterialCatalog,
    pub consumable_item: ConsumableItemCatalog,
    pub costume: CostumeCatalog,
    pub weapon: WeaponCatalog,
    pub explore: ExploreCatalog,
    pub gimmick: GimmickCatalog,
    pub character_board: CharacterBoardCatalog,
    pub character_rebirth: CharacterRebirthCatalog,
    pub companion: CompanionCatalog,
    pub side_story: SideStoryCatalog,
    pub big_hunt: BigHuntCatalog,
    pub tower: TowerCatalog,
    pub labyrinth: LabyrinthCatalog,
    pub campaign: CampaignCatalog,
    // Derived handlers (not from tables):
    pub quest_handler: QuestHandler,
    pub gacha_handler: GachaHandler,
}
```

### `SessionState`

```rust
pub struct SessionState {
    pub session_key: String,
    pub user_id: i64,
    pub uuid: String,
    pub expire_at: OffsetDateTime,
}
```

### `DiffEntry` and `DiffSet`

```rust
/// A single table mutation descriptor produced by an event handler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffEntry {
    pub table: TableId,
    pub action: DiffAction,
    pub key_fields: HashMap<String, serde_json::Value>,
    pub values: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DiffAction {
    Insert,
    Update,
    Delete,
}

/// Accumulator for DiffEntry across event handlers within one request cycle.
#[derive(Debug, Default)]
pub struct DiffSet {
    entries: Vec<DiffEntry>,
}

impl DiffSet {
    pub fn push(&mut self, entry: DiffEntry);
    pub fn into_protobuf(self) -> HashMap<String, DiffData>;
}

/// Key fields for each table (80+ definitions).
pub fn key_fields_for_table(table: &TableId) -> Option<&[&str]>;
```

### `AppConfig`

```rust
pub struct AppConfig {
    pub game_server: GameServerConfig,
    pub assets_server: AssetsServerConfig,
    pub admin: AdminConfig,
    pub sqlite: SqliteConfig,
    pub master_data: MasterDataConfig,
}

pub struct GameServerConfig {
    pub listen: SocketAddr,
    pub public_addr: SocketAddr,
}

pub struct AssetsServerConfig {
    pub listen: SocketAddr,
    pub public_addr: SocketAddr,
    pub assets_dir: PathBuf,
}

pub struct AdminConfig {
    pub listen: SocketAddr,
    pub token: Option<String>,
}

pub struct SqliteConfig {
    pub game_db: PathBuf,
    pub auth_db: PathBuf,
}

pub struct MasterDataConfig {
    pub path: PathBuf,
}
```

## gRPC Service Contracts

All services are defined in `proto/apb/api/` and compiled via `tonic-build` + `prost`.

### Package Structure

```
apb.api.user       — UserService (Auth, RegisterUser, TransferUser, SetUserName, ...)
apb.api.quest      — QuestService (StartQuest, EndQuest, ...)
apb.api.gacha      — GachaService (Draw, ...)
apb.api.battle     — BattleService
apb.api.config     — ConfigService (GetConfig)
apb.api.data       — DataService (GetLatestMasterDataVersion, GetUserData)
apb.api.tutorial   — TutorialService
apb.api.gift       — GiftService
apb.api.gameplay   — GamePlayService
apb.api.gimmick    — GimmickService
apb.api.notification — NotificationService
apb.api.cageornament — CageOrnamentService
apb.api.deck       — DeckService
apb.api.friend     — FriendService
apb.api.loginbonus — LoginBonusService
apb.api.navicutin  — NaviCutInService
apb.api.contentsstory — ContentsStoryService
apb.api.dokan      — DokanService
apb.api.portalcage — PortalCageService
apb.api.characterviewer — CharacterViewerService
apb.api.mission    — MissionService
apb.api.shop       — ShopService
apb.api.costume    — CostumeService
apb.api.movie      — MovieService
apb.api.omikuji    — OmikujiService
apb.api.weapon     — WeaponService
apb.api.explore    — ExploreService
apb.api.characterboard — CharacterBoardService
apb.api.parts      — PartsService
apb.api.character  — CharacterService
apb.api.companion  — CompanionService
apb.api.material   — MaterialService
apb.api.consumableitem — ConsumableItemService
apb.api.sidestoryquest — SideStoryQuestService
apb.api.bighunt    — BigHuntService
apb.api.reward     — RewardService
apb.api.labyrinth  — LabyrinthService
apb.api.banner     — BannerService
```

### Admin API (`blackbox-api`)

```
apb.api.admin      — AdminService (ReloadMasterData, HealthCheck, GetMetrics)
```

### Common Response Pattern

Every gRPC response that mutates user state includes:

```protobuf
message XxxResponse {
    // ... response-specific fields ...
    map<string, DiffData> diff_user_data = N;
}
```

The `DiffData` message:

```protobuf
message DiffData {
    string update_records_json = 1;
    string delete_keys_json = 2;
}
```

### Trailer Contracts

| Trailer | Type | When | Description |
|---------|------|------|-------------|
| `x-apb-response-datetime` | `string` (millis) | All RPCs except Auth/Register/Transfer | Server timestamp |
| `x-apb-update-user-data-names` | `string` (comma-separated) | All RPCs with diff data | Changed table names |

### Metadata (Request Headers)

| Header | Type | Description |
|--------|------|-------------|
| `x-apb-session-key` | `string` | Session key from Auth response |
| `x-apb-platform` | `string` | `"android"` or `"ios"` |
| `x-apb-uuid` | `string` | Client UUID |

## Error Mapping

| Domain Error | gRPC Status |
|---|---|
| User not found | `NOT_FOUND` |
| Invalid session | `UNAUTHENTICATED` |
| Registration disabled | `PERMISSION_DENIED` |
| Master data not loaded | `INTERNAL` |
| DB error | `INTERNAL` |
| Invalid request argument | `INVALID_ARGUMENT` |

## Serialization Contracts

### Master Data (msgpack)

- Tables are stored as msgpack arrays of arrays: `[[col0, col1, ...], ...]`
- Each row is deserialized into a struct by positional index
- LZ4 compression uses msgpack ext type code 99
- The ext header contains: `[code: int8, data: bytes]` where data = `[uncompressed_size: int32, lz4_block: bytes]`

### Diff Deltas (JSON)

- `update_records_json`: JSON array of objects with named fields
- `delete_keys_json`: JSON array of composite key objects
- Both use camelCase field names matching the C# entity property names

### User State (JSON blob in SQLite)

- Stored as `serde_json::Value` blob per user
- Map fields use string keys (JSON object) for UUID-keyed maps, numeric string keys for i32-keyed maps
- On load: deserialize JSON → `UserState` struct
- On save: serialize `UserState` → JSON blob

### list.bin (protobuf)

- `Database` message contains `repeated Data asset_bundle_list` and `repeated Data resource_list`
- Each `Data` has: `id`, `filepath`, `name`, `size`, `crc`, `priority`, `tag_id`, `dependencies`, `state`, `md5`, `object_name`, `generation`, `upload_version_id`
- Resource base URL is a fixed-length (43-byte) field that must be replaced in-place
