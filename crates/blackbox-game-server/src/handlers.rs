//! Stub gRPC service handlers.
//!
//! These are placeholder implementations that return UNIMPLEMENTED for all RPCs.
//! Full business logic is implemented incrementally as actors are wired up.

use tonic::{Request, Response, Status};

/// User service handler.
#[derive(Debug, Clone)]
pub struct UserServiceHandler;

impl UserServiceHandler {
    pub fn new() -> Self { Self }
}

/// Quest service handler.
#[derive(Debug, Clone)]
pub struct QuestServiceHandler;

impl QuestServiceHandler {
    pub fn new() -> Self { Self }
}

/// Gacha service handler.
#[derive(Debug, Clone)]
pub struct GachaServiceHandler;

impl GachaServiceHandler {
    pub fn new() -> Self { Self }
}

/// Battle service handler.
#[derive(Debug, Clone)]
pub struct BattleServiceHandler;

impl BattleServiceHandler {
    pub fn new() -> Self { Self }
}

/// Config service handler.
#[derive(Debug, Clone)]
pub struct ConfigServiceHandler;

impl ConfigServiceHandler {
    pub fn new() -> Self { Self }
}

/// Tutorial service handler.
#[derive(Debug, Clone)]
pub struct TutorialServiceHandler;

impl TutorialServiceHandler {
    pub fn new() -> Self { Self }
}

/// Gift service handler.
#[derive(Debug, Clone)]
pub struct GiftServiceHandler;

impl GiftServiceHandler {
    pub fn new() -> Self { Self }
}

/// GamePlay service handler.
#[derive(Debug, Clone)]
pub struct GamePlayServiceHandler;

impl GamePlayServiceHandler {
    pub fn new() -> Self { Self }
}

/// Gimmick service handler.
#[derive(Debug, Clone)]
pub struct GimmickServiceHandler;

impl GimmickServiceHandler {
    pub fn new() -> Self { Self }
}

/// Notification service handler.
#[derive(Debug, Clone)]
pub struct NotificationServiceHandler;

impl NotificationServiceHandler {
    pub fn new() -> Self { Self }
}

/// CageOrnament service handler.
#[derive(Debug, Clone)]
pub struct CageOrnamentServiceHandler;

impl CageOrnamentServiceHandler {
    pub fn new() -> Self { Self }
}

/// Deck service handler.
#[derive(Debug, Clone)]
pub struct DeckServiceHandler;

impl DeckServiceHandler {
    pub fn new() -> Self { Self }
}

/// Friend service handler.
#[derive(Debug, Clone)]
pub struct FriendServiceHandler;

impl FriendServiceHandler {
    pub fn new() -> Self { Self }
}

/// LoginBonus service handler.
#[derive(Debug, Clone)]
pub struct LoginBonusServiceHandler;

impl LoginBonusServiceHandler {
    pub fn new() -> Self { Self }
}

/// NaviCutIn service handler.
#[derive(Debug, Clone)]
pub struct NaviCutInServiceHandler;

impl NaviCutInServiceHandler {
    pub fn new() -> Self { Self }
}

/// ContentsStory service handler.
#[derive(Debug, Clone)]
pub struct ContentsStoryServiceHandler;

impl ContentsStoryServiceHandler {
    pub fn new() -> Self { Self }
}

/// Dokan service handler.
#[derive(Debug, Clone)]
pub struct DokanServiceHandler;

impl DokanServiceHandler {
    pub fn new() -> Self { Self }
}

/// PortalCage service handler.
#[derive(Debug, Clone)]
pub struct PortalCageServiceHandler;

impl PortalCageServiceHandler {
    pub fn new() -> Self { Self }
}

/// CharacterViewer service handler.
#[derive(Debug, Clone)]
pub struct CharacterViewerServiceHandler;

impl CharacterViewerServiceHandler {
    pub fn new() -> Self { Self }
}

/// Mission service handler.
#[derive(Debug, Clone)]
pub struct MissionServiceHandler;

impl MissionServiceHandler {
    pub fn new() -> Self { Self }
}

/// Shop service handler.
#[derive(Debug, Clone)]
pub struct ShopServiceHandler;

impl ShopServiceHandler {
    pub fn new() -> Self { Self }
}

/// Costume service handler.
#[derive(Debug, Clone)]
pub struct CostumeServiceHandler;

impl CostumeServiceHandler {
    pub fn new() -> Self { Self }
}

/// Movie service handler.
#[derive(Debug, Clone)]
pub struct MovieServiceHandler;

impl MovieServiceHandler {
    pub fn new() -> Self { Self }
}

/// Omikuji service handler.
#[derive(Debug, Clone)]
pub struct OmikujiServiceHandler;

impl OmikujiServiceHandler {
    pub fn new() -> Self { Self }
}

/// Weapon service handler.
#[derive(Debug, Clone)]
pub struct WeaponServiceHandler;

impl WeaponServiceHandler {
    pub fn new() -> Self { Self }
}

/// Explore service handler.
#[derive(Debug, Clone)]
pub struct ExploreServiceHandler;

impl ExploreServiceHandler {
    pub fn new() -> Self { Self }
}

/// CharacterBoard service handler.
#[derive(Debug, Clone)]
pub struct CharacterBoardServiceHandler;

impl CharacterBoardServiceHandler {
    pub fn new() -> Self { Self }
}

/// Parts service handler.
#[derive(Debug, Clone)]
pub struct PartsServiceHandler;

impl PartsServiceHandler {
    pub fn new() -> Self { Self }
}

/// Character service handler.
#[derive(Debug, Clone)]
pub struct CharacterServiceHandler;

impl CharacterServiceHandler {
    pub fn new() -> Self { Self }
}

/// Companion service handler.
#[derive(Debug, Clone)]
pub struct CompanionServiceHandler;

impl CompanionServiceHandler {
    pub fn new() -> Self { Self }
}

/// Material service handler.
#[derive(Debug, Clone)]
pub struct MaterialServiceHandler;

impl MaterialServiceHandler {
    pub fn new() -> Self { Self }
}

/// ConsumableItem service handler.
#[derive(Debug, Clone)]
pub struct ConsumableItemServiceHandler;

impl ConsumableItemServiceHandler {
    pub fn new() -> Self { Self }
}

/// SideStoryQuest service handler.
#[derive(Debug, Clone)]
pub struct SideStoryQuestServiceHandler;

impl SideStoryQuestServiceHandler {
    pub fn new() -> Self { Self }
}

/// BigHunt service handler.
#[derive(Debug, Clone)]
pub struct BigHuntServiceHandler;

impl BigHuntServiceHandler {
    pub fn new() -> Self { Self }
}

/// Reward service handler.
#[derive(Debug, Clone)]
pub struct RewardServiceHandler;

impl RewardServiceHandler {
    pub fn new() -> Self { Self }
}

/// Labyrinth service handler.
#[derive(Debug, Clone)]
pub struct LabyrinthServiceHandler;

impl LabyrinthServiceHandler {
    pub fn new() -> Self { Self }
}

/// Banner service handler.
#[derive(Debug, Clone)]
pub struct BannerServiceHandler;

impl BannerServiceHandler {
    pub fn new() -> Self { Self }
}

/// Admin service handler.
#[derive(Debug, Clone)]
pub struct AdminServiceHandler;

impl AdminServiceHandler {
    pub fn new() -> Self { Self }
}

/// CharacterReward service handler.
#[derive(Debug, Clone)]
pub struct CharacterRewardServiceHandler;

impl CharacterRewardServiceHandler {
    pub fn new() -> Self { Self }
}

/// Return a unimplemented status for stub handlers.
pub fn unimplemented_status() -> Status {
    Status::unimplemented("Full gRPC handlers require protoc code generation")
}
