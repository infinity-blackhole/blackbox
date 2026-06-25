//! Generated Rust types for octo-proto.
//!
//! This file provides fallback types when protoc is not available.
//! When the `codegen` feature is enabled, prost-build generates code from
//! the .proto files at `proto/` and this file is replaced.

#![allow(missing_docs, clippy::derive_partial_eq_eq)]

use serde::{Deserialize, Serialize};

// ============================================================================
// Core types from octo.proto
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Database {
    pub id: String,
    pub data: String,
    pub updated_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub key: String,
    pub value: Vec<u8>,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url {
    pub id: String,
    pub url: String,
    pub asset_bundle: String,
    pub hash: String,
    pub size: i32,
    pub priority: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UrlMap {
    pub entries: ::std::collections::HashMap<String, Url>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UrlList {
    pub urls: Vec<Url>,
}

// ============================================================================
// Diff types from data.proto
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffAction {
    Insert = 0,
    Update = 1,
    Delete = 2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiffEntry {
    pub table_name: String,
    pub key_field: String,
    pub key_value: String,
    pub field_name: String,
    pub old_value: String,
    pub new_value: String,
    pub action: DiffAction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiffData {
    pub entries: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiffSyncPacket {
    pub tables: ::std::collections::HashMap<String, DiffData>,
}

// ============================================================================
// Common domain types
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RewardItem {
    pub item_id: i32,
    pub count: i32,
    pub rarity: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserData {
    pub uuid: String,
    pub user_id: i64,
    pub player_id: i64,
    pub name: String,
    pub level: i32,
    pub exp: i32,
    pub gem: i64,
    pub coin: i64,
    pub stamina: i32,
    pub register_time: i64,
    pub last_login_time: i64,
}

// ============================================================================
// User service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUserRequest {
    pub uuid: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub user: Option<UserData>,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub uuid: String,
    pub fields: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateUserResponse {
    pub user: Option<UserData>,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginRequest {
    pub uuid: String,
    pub token: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginResponse {
    pub session_key: String,
    pub user_id: String,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogoutRequest {
    pub uuid: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogoutResponse {
    pub success: bool,
}

// ============================================================================
// Quest service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartQuestRequest {
    pub quest_id: i32,
    pub deck_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartQuestResponse {
    pub quest_session_id: String,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndQuestRequest {
    pub quest_session_id: String,
    pub score: i64,
    pub clear_time: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndQuestResponse {
    pub rewards: Vec<RewardItem>,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetQuestRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetQuestResponse {
    pub active_quests: Vec<QuestInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetChapterRequest {
    pub chapter_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetChapterResponse {
    pub chapter: Option<ChapterInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuestInfo {
    pub quest_id: i32,
    pub status: i32,
    pub progress: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChapterInfo {
    pub chapter_id: i32,
    pub quests: Vec<QuestInfo>,
}

// ============================================================================
// Gacha service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrawGachaRequest {
    pub gacha_id: i32,
    pub draw_count: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrawGachaResponse {
    pub results: Vec<GachaResult>,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGachaRateRequest {
    pub gacha_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGachaRateResponse {
    pub rates: Vec<RateInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGachaListRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGachaListResponse {
    pub gachas: Vec<GachaInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GachaResult {
    pub item_id: i32,
    pub rarity: i32,
    pub is_new: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateInfo {
    pub rarity: i32,
    pub rate: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GachaInfo {
    pub gacha_id: i32,
    pub name: String,
    pub status: i32,
}

// ============================================================================
// Battle service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartBattleRequest {
    pub dungeon_id: i32,
    pub deck_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartBattleResponse {
    pub battle_session_id: String,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndBattleRequest {
    pub battle_session_id: String,
    pub result: i32,
    pub score: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndBattleResponse {
    pub rewards: Vec<RewardItem>,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBattleResultRequest {
    pub battle_session_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBattleResultResponse {
    pub result: i32,
    pub score: i64,
}

// ============================================================================
// Config service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetConfigRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetConfigResponse {
    pub config: Option<GameConfig>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMaintenanceRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMaintenanceResponse {
    pub is_maintenance: bool,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetVersionRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetVersionResponse {
    pub required_version: String,
    pub current_version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameConfig {
    pub max_stamina: i32,
    pub stamina_recovery_seconds: i32,
    pub max_friends: i32,
}

// ============================================================================
// Tutorial service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTutorialRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTutorialResponse {
    pub tutorial_step: i32,
    pub completed: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateTutorialRequest {
    pub user_id: i64,
    pub step: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateTutorialResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkipTutorialRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkipTutorialResponse {
    pub diffs: Vec<DiffEntry>,
}

// ============================================================================
// Gift service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGiftsRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGiftsResponse {
    pub gifts: Vec<GiftInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaimGiftRequest {
    pub gift_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaimGiftResponse {
    pub rewards: Vec<RewardItem>,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendGiftRequest {
    pub target_user_id: i64,
    pub gift_item_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendGiftResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GiftInfo {
    pub gift_id: i32,
    pub from_user_id: i64,
    pub item_id: i32,
    pub sent_at: i64,
}

// ============================================================================
// GamePlay service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGamePlayDataRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGamePlayDataResponse {
    pub stamina: i32,
    pub stamina_recovery_time: i64,
    pub level: i32,
    pub exp: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateGamePlayRequest {
    pub user_id: i64,
    pub stamina: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateGamePlayResponse {
    pub diffs: Vec<DiffEntry>,
}

// ============================================================================
// Gimmick service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGimmickRequest {
    pub gimmick_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGimmickResponse {
    pub gimmick: Option<GimmickInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerGimmickRequest {
    pub gimmick_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerGimmickResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimmickInfo {
    pub gimmick_id: i32,
    pub is_active: bool,
}

// ============================================================================
// Notification service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNotificationsRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNotificationsResponse {
    pub notifications: Vec<NotificationInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkReadRequest {
    pub notification_ids: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkReadResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationInfo {
    pub notification_id: i32,
    pub title: String,
    pub body: String,
    pub is_read: bool,
}

// ============================================================================
// CageOrnament service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCageOrnamentRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCageOrnamentResponse {
    pub status: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaimCageOrnamentRequest {
    pub reward_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaimCageOrnamentResponse {
    pub rewards: Vec<RewardItem>,
    pub diffs: Vec<DiffEntry>,
}

// ============================================================================
// Deck service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDecksRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDecksResponse {
    pub decks: Vec<DeckData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateDeckRequest {
    pub user_id: i64,
    pub deck_id: String,
    pub character_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateDeckResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDeckRequest {
    pub user_id: i64,
    pub deck_slot: i32,
    pub deck_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDeckResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeckData {
    pub deck_id: String,
    pub name: String,
    pub characters: Vec<String>,
}

// ============================================================================
// Friend service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFriendsRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFriendsResponse {
    pub friends: Vec<FriendData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddFriendRequest {
    pub user_id: i64,
    pub target_user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddFriendResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveFriendRequest {
    pub user_id: i64,
    pub target_user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveFriendResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFriendRequestsRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFriendRequestsResponse {
    pub incoming: Vec<FriendData>,
    pub outgoing: Vec<FriendData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FriendData {
    pub user_id: i64,
    pub name: String,
    pub level: i32,
    pub last_login: i64,
}

// ============================================================================
// LoginBonus service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoginBonusRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoginBonusResponse {
    pub day: i32,
    pub rewards: Vec<LoginBonusReward>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaimLoginBonusRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaimLoginBonusResponse {
    pub rewards: Vec<RewardItem>,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginBonusReward {
    pub day: i32,
    pub item_id: i32,
    pub count: i32,
}

// ============================================================================
// NaviCutIn service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNaviCutInRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNaviCutInResponse {
    pub events: Vec<NaviCutInEvent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkNaviCutInPlayedRequest {
    pub event_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkNaviCutInPlayedResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NaviCutInEvent {
    pub event_id: i32,
    pub character_id: i32,
    pub is_played: bool,
}

// ============================================================================
// ContentsStory service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetContentsStoryRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetContentsStoryResponse {
    pub stories: Vec<ContentsStoryData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateStoryProgressRequest {
    pub story_id: i32,
    pub chapter_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateStoryProgressResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentsStoryData {
    pub story_id: i32,
    pub chapters: Vec<StoryChapter>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoryChapter {
    pub chapter_id: i32,
    pub status: i32,
}

// ============================================================================
// Dokan service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDokanRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDokanResponse {
    pub messages: Vec<DokanMessage>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfirmDokanRequest {
    pub message_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfirmDokanResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DokanMessage {
    pub message_id: i32,
    pub title: String,
    pub body: String,
}

// ============================================================================
// PortalCage service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPortalCageRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPortalCageResponse {
    pub status: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenPortalCageRequest {
    pub cage_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenPortalCageResponse {
    pub rewards: Vec<RewardItem>,
    pub diffs: Vec<DiffEntry>,
}

// ============================================================================
// CharacterViewer service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCharacterViewerRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCharacterViewerResponse {
    pub characters: Vec<ViewerCharacter>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterCharacterRequest {
    pub character_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterCharacterResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerCharacter {
    pub character_id: i32,
    pub count: i32,
}

// ============================================================================
// Mission service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMissionsRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMissionsResponse {
    pub missions: Vec<Mission>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaimMissionRequest {
    pub mission_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaimMissionResponse {
    pub rewards: Vec<RewardItem>,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDailyMissionsRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDailyMissionsResponse {
    pub missions: Vec<Mission>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWeeklyMissionsRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWeeklyMissionsResponse {
    pub missions: Vec<Mission>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mission {
    pub mission_id: i32,
    pub title: String,
    pub progress: i32,
    pub goal: i32,
    pub mission_type: i32,
}

// ============================================================================
// Shop service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetShopRequest {
    pub shop_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetShopResponse {
    pub shop: Option<ShopData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseItemRequest {
    pub product_id: i32,
    pub count: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseItemResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProductsRequest {
    pub shop_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProductsResponse {
    pub products: Vec<ShopProduct>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShopData {
    pub shop_id: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShopProduct {
    pub product_id: i32,
    pub item_id: i32,
    pub price: i32,
    pub count: i32,
}

// ============================================================================
// Costume service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCostumesRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCostumesResponse {
    pub costumes: Vec<Costume>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipCostumeRequest {
    pub character_id: i32,
    pub costume_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipCostumeResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwakenCostumeRequest {
    pub costume_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwakenCostumeResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Costume {
    pub costume_id: String,
    pub character_id: i32,
    pub is_equipped: bool,
}

// ============================================================================
// Movie service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMoviesRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMoviesResponse {
    pub movies: Vec<Movie>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkMoviePlayedRequest {
    pub movie_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkMoviePlayedResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Movie {
    pub movie_id: i32,
    pub title: String,
    pub is_played: bool,
}

// ============================================================================
// Omikuji service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrawOmikujiRequest {
    pub draw_type: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrawOmikujiResponse {
    pub result: Option<OmikujiResult>,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOmikujiStatusRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOmikujiStatusResponse {
    pub draws_remaining: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OmikujiResult {
    pub rank: String,
    pub rewards: Vec<RewardItem>,
}

// ============================================================================
// Weapon service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWeaponsRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWeaponsResponse {
    pub weapons: Vec<Weapon>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipWeaponRequest {
    pub character_id: i32,
    pub weapon_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipWeaponResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwakenWeaponRequest {
    pub weapon_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwakenWeaponResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnhanceWeaponRequest {
    pub weapon_id: String,
    pub material_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnhanceWeaponResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weapon {
    pub weapon_id: String,
    pub name: String,
    pub level: i32,
    pub character_id: i32,
}

// ============================================================================
// Explore service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetExploreRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetExploreResponse {
    pub explores: Vec<ExploreInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartExploreRequest {
    pub explore_id: i32,
    pub deck_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartExploreResponse {
    pub explore_session_id: String,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndExploreRequest {
    pub explore_session_id: String,
    pub score: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndExploreResponse {
    pub rewards: Vec<RewardItem>,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExploreInfo {
    pub explore_id: i32,
    pub status: i32,
}

// ============================================================================
// CharacterBoard service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCharacterBoardRequest {
    pub character_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCharacterBoardResponse {
    pub board: Option<CharacterBoard>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivateBoardRequest {
    pub character_id: i32,
    pub board_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivateBoardResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterBoard {
    pub board_id: i32,
    pub name: String,
    pub is_active: bool,
}

// ============================================================================
// Parts service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPartsRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPartsResponse {
    pub parts: Vec<PartsInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipPartsRequest {
    pub character_id: i32,
    pub parts_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipPartsResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnhancePartsRequest {
    pub parts_id: String,
    pub material_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnhancePartsResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartsInfo {
    pub parts_id: String,
    pub name: String,
    pub level: i32,
}

// ============================================================================
// Character service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersResponse {
    pub characters: Vec<Character>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnhanceCharacterRequest {
    pub character_id: i32,
    pub material_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnhanceCharacterResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwakenCharacterRequest {
    pub character_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwakenCharacterResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RebirthCharacterRequest {
    pub character_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RebirthCharacterResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Character {
    pub character_id: i32,
    pub name: String,
    pub level: i32,
    pub exp: i32,
}

// ============================================================================
// Companion service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCompanionsRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCompanionsResponse {
    pub companions: Vec<Companion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCompanionRequest {
    pub character_id: i32,
    pub companion_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCompanionResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Companion {
    pub companion_id: String,
    pub name: String,
    pub level: i32,
}

// ============================================================================
// Material service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMaterialsRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMaterialsResponse {
    pub materials: Vec<Material>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseMaterialRequest {
    pub material_id: i32,
    pub count: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseMaterialResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Material {
    pub material_id: i32,
    pub name: String,
    pub count: i32,
}

// ============================================================================
// ConsumableItem service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetConsumableItemsRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetConsumableItemsResponse {
    pub items: Vec<ConsumableItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseItemRequest {
    pub item_id: i32,
    pub target_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseItemResponse {
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsumableItem {
    pub item_id: i32,
    pub name: String,
    pub count: i32,
}

// ============================================================================
// SideStoryQuest service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSideStoryQuestsRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSideStoryQuestsResponse {
    pub quests: Vec<SideStoryQuest>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartSideStoryRequest {
    pub quest_id: i32,
    pub deck_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartSideStoryResponse {
    pub quest_session_id: String,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndSideStoryRequest {
    pub quest_session_id: String,
    pub score: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndSideStoryResponse {
    pub rewards: Vec<RewardItem>,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SideStoryQuest {
    pub quest_id: i32,
    pub title: String,
    pub status: i32,
}

// ============================================================================
// BigHunt service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBigHuntRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBigHuntResponse {
    pub hunts: Vec<BigHunt>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartBigHuntRequest {
    pub hunt_id: i32,
    pub deck_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartBigHuntResponse {
    pub hunt_session_id: String,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndBigHuntRequest {
    pub hunt_session_id: String,
    pub damage: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndBigHuntResponse {
    pub rewards: Vec<RewardItem>,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBigHuntProgressRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBigHuntProgressResponse {
    pub total_damage: i64,
    pub personal_rank: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BigHunt {
    pub hunt_id: i32,
    pub boss_name: String,
    pub status: i32,
}

// ============================================================================
// Reward service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRewardsRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRewardsResponse {
    pub pending_rewards: Vec<RewardItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaimRewardRequest {
    pub reward_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaimRewardResponse {
    pub rewards: Vec<RewardItem>,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRewardHistoryRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRewardHistoryResponse {
    pub history: Vec<RewardItem>,
}

// ============================================================================
// Labyrinth service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLabyrinthRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLabyrinthResponse {
    pub seasons: Vec<LabyrinthSeason>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartLabyrinthRequest {
    pub season_id: i32,
    pub stage_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartLabyrinthResponse {
    pub session_id: String,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndLabyrinthRequest {
    pub session_id: String,
    pub result: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndLabyrinthResponse {
    pub rewards: Vec<RewardItem>,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabyrinthSeason {
    pub season_id: i32,
    pub name: String,
    pub status: i32,
}

// ============================================================================
// Banner service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBannersRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBannersResponse {
    pub banners: Vec<Banner>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Banner {
    pub banner_id: i32,
    pub image_url: String,
    pub link_url: String,
    pub start_time: i64,
    pub end_time: i64,
}

// ============================================================================
// Admin service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUsersRequest {
    pub offset: i32,
    pub limit: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUsersResponse {
    pub users: Vec<UserData>,
    pub total: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BanUserRequest {
    pub user_id: i64,
    pub reason: String,
    pub duration_hours: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BanUserResponse {
    pub success: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendNoticeRequest {
    pub title: String,
    pub body: String,
    pub target_users: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendNoticeResponse {
    pub sent_count: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSystemInfoRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSystemInfoResponse {
    pub version: String,
    pub uptime_seconds: i64,
    pub active_users: i32,
}

// ============================================================================
// CharacterReward service
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCharacterRewardsRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCharacterRewardsResponse {
    pub rewards: Vec<CharacterReward>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaimCharacterRewardRequest {
    pub character_id: i32,
    pub reward_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaimCharacterRewardResponse {
    pub rewards: Vec<RewardItem>,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterReward {
    pub reward_id: i32,
    pub character_id: i32,
    pub is_claimed: bool,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_roundtrip() {
        let db = Database {
            id: "user:123".to_string(),
            data: r#"{"name":"Alice"}"#.to_string(),
            updated_at: 1700000000,
        };
        let json = serde_json::to_string(&db).unwrap();
        let restored: Database = serde_json::from_str(&json).unwrap();
        assert_eq!(db, restored);
    }

    #[test]
    fn test_diff_entry_roundtrip() {
        let entry = DiffEntry {
            table_name: "user".to_string(),
            key_field: "id".to_string(),
            key_value: "42".to_string(),
            field_name: "level".to_string(),
            old_value: "10".to_string(),
            new_value: "11".to_string(),
            action: DiffAction::Update,
        };
        let json = serde_json::to_string(&entry).unwrap();
        let restored: DiffEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry, restored);
    }

    #[test]
    fn test_diff_sync_packet() {
        let packet = DiffSyncPacket {
            tables: {
                let mut m = std::collections::HashMap::new();
                m.insert(
                    "user".to_string(),
                    DiffData {
                        entries: vec![DiffEntry {
                            table_name: "user".to_string(),
                            key_field: "id".to_string(),
                            key_value: "1".to_string(),
                            field_name: "name".to_string(),
                            old_value: String::new(),
                            new_value: "Alice".to_string(),
                            action: DiffAction::Insert,
                        }],
                    },
                );
                m
            },
        };
        let json = serde_json::to_string(&packet).unwrap();
        let restored: DiffSyncPacket = serde_json::from_str(&json).unwrap();
        assert_eq!(packet, restored);
    }

    #[test]
    fn test_user_data_roundtrip() {
        let user = UserData {
            uuid: "abc-123".to_string(),
            user_id: 42,
            player_id: 100,
            name: "TestUser".to_string(),
            level: 50,
            exp: 99999,
            gem: 5000,
            coin: 100000,
            stamina: 100,
            register_time: 1600000000,
            last_login_time: 1700000000,
        };
        let json = serde_json::to_string(&user).unwrap();
        let restored: UserData = serde_json::from_str(&json).unwrap();
        assert_eq!(user, restored);
    }

    #[test]
    fn test_reward_item() {
        let item = RewardItem {
            item_id: 1001,
            count: 5,
            rarity: 3,
        };
        let json = serde_json::to_string(&item).unwrap();
        let restored: RewardItem = serde_json::from_str(&json).unwrap();
        assert_eq!(item, restored);
    }
}
