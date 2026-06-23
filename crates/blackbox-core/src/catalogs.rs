use std::sync::Arc;
use std::collections::HashMap;

// Placeholder types for master data catalogs
// These will be replaced by generated types from schemas.json
#[derive(Debug, Clone, Default)]
pub struct EntityMGameConfig;

#[derive(Debug, Clone, Default)]
pub struct PartsCatalog;
#[derive(Debug, Clone, Default)]
pub struct QuestCatalog;
#[derive(Debug, Clone, Default)]
pub struct GachaCatalog;
#[derive(Debug, Clone, Default)]
pub struct GachaCatalogEntry;
#[derive(Debug, Clone, Default)]
pub struct GachaMedalInfo;
#[derive(Debug, Clone, Default)]
pub struct ShopCatalog;
#[derive(Debug, Clone, Default)]
pub struct DupExchangeEntry;
#[derive(Debug, Clone, Default)]
pub struct ConditionResolver;
#[derive(Debug, Clone, Default)]
pub struct CageOrnamentCatalog;
#[derive(Debug, Clone, Default)]
pub struct LoginBonusCatalog;
#[derive(Debug, Clone, Default)]
pub struct CharacterViewerCatalog;
#[derive(Debug, Clone, Default)]
pub struct OmikujiCatalog;
#[derive(Debug, Clone, Default)]
pub struct MaterialCatalog;
#[derive(Debug, Clone, Default)]
pub struct ConsumableItemCatalog;
#[derive(Debug, Clone, Default)]
pub struct CostumeCatalog;
#[derive(Debug, Clone, Default)]
pub struct WeaponCatalog;
#[derive(Debug, Clone, Default)]
pub struct ExploreCatalog;
#[derive(Debug, Clone, Default)]
pub struct GimmickCatalog;
#[derive(Debug, Clone, Default)]
pub struct CharacterBoardCatalog;
#[derive(Debug, Clone, Default)]
pub struct CharacterRebirthCatalog;
#[derive(Debug, Clone, Default)]
pub struct CompanionCatalog;
#[derive(Debug, Clone, Default)]
pub struct SideStoryCatalog;
#[derive(Debug, Clone, Default)]
pub struct BigHuntCatalog;
#[derive(Debug, Clone, Default)]
pub struct TowerCatalog;
#[derive(Debug, Clone, Default)]
pub struct LabyrinthCatalog;
#[derive(Debug, Clone, Default)]
pub struct CampaignCatalog;
#[derive(Debug, Clone, Default)]
pub struct QuestHandler;
#[derive(Debug, Clone, Default)]
pub struct GachaHandler;

/// The aggregate of all loaded master data catalogs.
/// Stored as Arc<MasterDataCatalogs> behind a watch channel for hot reloads.
#[derive(Debug, Clone, Default)]
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
    pub quest_handler: QuestHandler,
    pub gacha_handler: GachaHandler,
}

impl MasterDataCatalogs {
    /// Returns an empty (default) MasterDataCatalogs for initial state before data is loaded.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Returns a new Arc-wrapped MasterDataCatalogs for use in watch channels.
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self::default())
    }
}
