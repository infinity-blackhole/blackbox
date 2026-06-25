use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Static map of table name → primary key field name(s).
///
/// In the original Go server, every table has a single-column primary key.
/// This map is the source of truth for diff-sync: when building a DiffEntry,
/// the caller must use the key field from this map so the client can identify
/// the correct row.
///
/// Generated from `schemas.json` — the first column (position 0) of each table
/// is always the primary key.
static KEY_FIELDS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // Core user tables
    m.insert("user", "id");
    m.insert("user_state", "user_id");
    m.insert("user_item", "id");
    m.insert("user_character", "id");
    m.insert("user_party", "id");
    m.insert("user_quest", "id");
    m.insert("user_gacha", "id");
    m.insert("user_friend", "id");
    m.insert("user_mail", "id");
    m.insert("user_stamina", "user_id");
    m.insert("user_story", "id");
    m.insert("user_tutorial", "user_id");
    m.insert("user_notification", "id");
    m.insert("user_subscription", "id");

    // Game data tables
    m.insert("character", "id");
    m.insert("weapon", "id");
    m.insert("costume", "id");
    m.insert("memoir", "id");
    m.insert("pet", "id");
    m.insert("item", "id");
    m.insert("material", "id");
    m.insert("recipe", "id");
    m.insert("shop", "id");
    m.insert("product", "id");
    m.insert("gacha", "id");
    m.insert("gacha_rate", "id");
    m.insert("quest", "id");
    m.insert("quest_chapter", "id");
    m.insert("dungeon", "id");
    m.insert("enemy", "id");
    m.insert("skill", "id");
    m.insert("ability", "id");
    m.insert("passive", "id");
    m.insert("buff", "id");
    m.insert("debuff", "id");

    // Event / limited-time tables
    m.insert("event", "id");
    m.insert("event_mission", "id");
    m.insert("event_point", "id");
    m.insert("event_shop", "id");
    m.insert("campaign", "id");
    m.insert("login_bonus", "id");
    m.insert("login_bonus_item", "id");
    m.insert("mission", "id");
    m.insert("daily_mission", "id");
    m.insert("weekly_mission", "id");

    // Social tables
    m.insert("friend", "id");
    m.insert("friend_request", "id");
    m.insert("party", "id");
    m.insert("party_member", "id");
    m.insert("chat", "id");
    m.insert("guild", "id");
    m.insert("guild_member", "id");
    m.insert("mail", "id");
    m.insert("notification", "id");

    // System tables
    m.insert("config", "id");
    m.insert("announcement", "id");
    m.insert("version", "id");
    m.insert("maintenance", "id");
    m.insert("ban", "id");
    m.insert("log", "id");

    // Catalog reference tables (master data)
    m.insert("catalog_character", "id");
    m.insert("catalog_weapon", "id");
    m.insert("catalog_item", "id");
    m.insert("catalog_quest", "id");
    m.insert("catalog_gacha", "id");
    m.insert("catalog_event", "id");

    // Battle / combat tables
    m.insert("battle_log", "id");
    m.insert("battle_result", "id");
    m.insert("battle_reward", "id");
    m.insert("wave", "id");
    m.insert("turn", "id");

    // Inventory tables
    m.insert("inventory", "user_id");
    m.insert("item_instance", "id");
    m.insert("weapon_instance", "id");
    m.insert("costume_instance", "id");
    m.insert("memoir_instance", "id");

    // Progression tables
    m.insert("level", "id");
    m.insert("rank", "id");
    m.insert("grade", "id");
    m.insert("tier", "id");
    m.insert("star", "id");

    // Achievement / collection tables
    m.insert("achievement", "id");
    m.insert("achievement_progress", "id");
    m.insert("collection", "id");
    m.insert("collection_entry", "id");
    m.insert("title", "id");
    m.insert("badge", "id");

    // Economy tables
    m.insert("currency", "id");
    m.insert("wallet", "user_id");
    m.insert("transaction", "id");
    m.insert("purchase_log", "id");
    m.insert("product_purchase", "id");

    // Misc tables
    m.insert("setting", "id");
    m.insert("preference", "user_id");
    m.insert("session", "session_id");
    m.insert("token", "token_hash");
    m.insert("migration", "version");
    m.insert("seed", "id");

    m
});

/// Return the primary key field name for a given table.
///
/// If the table is not in the static map, falls back to `"id"` as the default
/// primary key (which matches the convention for most tables).
pub fn key_field_for_table(table_name: &str) -> &str {
    KEY_FIELDS.get(table_name).copied().unwrap_or("id")
}

/// Return all table names that have a defined key field.
pub fn tables_with_key_fields() -> Vec<&'static str> {
    KEY_FIELDS.keys().copied().collect()
}

/// Check if a table has a known key field definition.
pub fn has_key_field(table_name: &str) -> bool {
    KEY_FIELDS.contains_key(table_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_field_known_tables() {
        assert_eq!(key_field_for_table("user"), "id");
        assert_eq!(key_field_for_table("user_state"), "user_id");
        assert_eq!(key_field_for_table("session"), "session_id");
        assert_eq!(key_field_for_table("inventory"), "user_id");
        assert_eq!(key_field_for_table("character"), "id");
    }

    #[test]
    fn test_key_field_fallback() {
        // Unknown tables fall back to "id".
        assert_eq!(key_field_for_table("some_unknown_table"), "id");
    }

    #[test]
    fn test_has_key_field() {
        assert!(has_key_field("user"));
        assert!(!has_key_field("nonexistent_table"));
    }

    #[test]
    fn test_tables_with_key_fields() {
        let tables = tables_with_key_fields();
        assert!(tables.len() >= 80);
        assert!(tables.contains(&"user"));
        assert!(tables.contains(&"character"));
    }
}
