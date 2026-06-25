use blackbox_diff_sync::DiffEntry;

/// Core game events dispatched through the system.
///
/// Each command emits one or more events. Event handlers react by mutating
/// state and producing diff entries.
#[derive(Debug, Clone)]
pub enum GameEvent {
    /// User registered for the first time.
    UserRegistered {
        user_id: i64,
        uuid: String,
    },
    /// User logged in.
    UserLogin {
        user_id: i64,
    },
    /// User logged out.
    UserLogout {
        user_id: i64,
    },
    /// User changed their name.
    UserNameChanged {
        user_id: i64,
        old_name: String,
        new_name: String,
    },
    /// Quest started.
    QuestStarted {
        user_id: i64,
        quest_id: i32,
        deck_id: String,
    },
    /// Quest completed.
    QuestCompleted {
        user_id: i64,
        quest_id: i32,
        score: i64,
    },
    /// Stamina consumed.
    StaminaConsumed {
        user_id: i64,
        amount: i32,
    },
    /// Gacha drawn.
    GachaDrawn {
        user_id: i64,
        gacha_id: i32,
        draw_count: i32,
    },
    /// Deck updated.
    DeckUpdated {
        user_id: i64,
        deck_id: String,
    },
}

impl GameEvent {
    /// Return the user_id associated with this event, if any.
    pub fn user_id(&self) -> Option<i64> {
        match self {
            Self::UserRegistered { user_id, .. } => Some(*user_id),
            Self::UserLogin { user_id } => Some(*user_id),
            Self::UserLogout { user_id } => Some(*user_id),
            Self::UserNameChanged { user_id, .. } => Some(*user_id),
            Self::QuestStarted { user_id, .. } => Some(*user_id),
            Self::QuestCompleted { user_id, .. } => Some(*user_id),
            Self::StaminaConsumed { user_id, .. } => Some(*user_id),
            Self::GachaDrawn { user_id, .. } => Some(*user_id),
            Self::DeckUpdated { user_id, .. } => Some(*user_id),
        }
    }
}
