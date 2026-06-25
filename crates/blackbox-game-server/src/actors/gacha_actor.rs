use kameo::Actor;
use blackbox_diff_sync::DiffEntry;

/// Gacha actor — manages gacha draws and banner state.
#[derive(Actor)]
pub struct GachaActor {
    state: GachaActorState,
}

#[derive(Debug, Default)]
struct GachaActorState {
    total_draws: i64,
}

impl GachaActor {
    pub fn new() -> Self {
        Self {
            state: GachaActorState::default(),
        }
    }
}

/// Draw from a gacha.
pub struct DrawGacha {
    pub user_id: i64,
    pub gacha_id: i32,
    pub draw_count: i32,
}

impl kameo::message::Message<DrawGacha> for GachaActor {
    type Reply = Result<Vec<DiffEntry>, crate::error::GameError>;

    async fn handle(
        &mut self,
        msg: DrawGacha,
        _ctx: kameo::message::Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        self.state.total_draws += msg.draw_count as i64;
        let diffs = vec![DiffEntry::insert(
            "user_gacha",
            "id",
            &format!("{}_{}", msg.user_id, msg.gacha_id),
            "draw_count",
            &msg.draw_count.to_string(),
        )];
        Ok(diffs)
    }
}

/// Stamina actor — manages stamina consumption and recovery.
#[derive(Actor)]
pub struct StaminaActor {
    state: StaminaActorState,
}

#[derive(Debug, Default)]
struct StaminaActorState {
    current_stamina: i32,
}

impl StaminaActor {
    pub fn new() -> Self {
        Self {
            state: StaminaActorState {
                current_stamina: 100,
            },
        }
    }
}

/// Consume stamina.
pub struct ConsumeStamina {
    pub user_id: i64,
    pub amount: i32,
}

impl kameo::message::Message<ConsumeStamina> for StaminaActor {
    type Reply = Result<Vec<DiffEntry>, crate::error::GameError>;

    async fn handle(
        &mut self,
        msg: ConsumeStamina,
        _ctx: kameo::message::Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        self.state.current_stamina = (self.state.current_stamina - msg.amount).max(0);
        let diffs = vec![DiffEntry::update(
            "user_stamina",
            "user_id",
            &msg.user_id.to_string(),
            "current",
            &(self.state.current_stamina + msg.amount).to_string(),
            &self.state.current_stamina.to_string(),
        )];
        Ok(diffs)
    }
}

/// Deck actor — manages deck composition.
#[derive(Actor)]
pub struct DeckActor {
    state: DeckActorState,
}

#[derive(Debug, Default)]
struct DeckActorState {
    deck_count: i32,
}

impl DeckActor {
    pub fn new() -> Self {
        Self {
            state: DeckActorState::default(),
        }
    }
}

/// Update deck.
pub struct UpdateDeck {
    pub user_id: i64,
    pub deck_id: String,
    pub character_ids: Vec<String>,
}

impl kameo::message::Message<UpdateDeck> for DeckActor {
    type Reply = Result<Vec<DiffEntry>, crate::error::GameError>;

    async fn handle(
        &mut self,
        msg: UpdateDeck,
        _ctx: kameo::message::Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        self.state.deck_count += 1;
        let diffs = vec![DiffEntry::update(
            "user_party",
            "id",
            &msg.deck_id,
            "characters",
            "",
            &msg.character_ids.join(","),
        )];
        Ok(diffs)
    }
}

/// Inventory actor — manages items, costumes, weapons.
#[derive(Actor)]
pub struct InventoryActor {
    state: InventoryActorState,
}

#[derive(Debug, Default)]
struct InventoryActorState {
    item_count: i32,
}

impl InventoryActor {
    pub fn new() -> Self {
        Self {
            state: InventoryActorState::default(),
        }
    }
}

/// Grant item to user.
pub struct GrantItem {
    pub user_id: i64,
    pub item_id: i32,
    pub count: i32,
}

impl kameo::message::Message<GrantItem> for InventoryActor {
    type Reply = Result<Vec<DiffEntry>, crate::error::GameError>;

    async fn handle(
        &mut self,
        msg: GrantItem,
        _ctx: kameo::message::Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        self.state.item_count += msg.count;
        let diffs = vec![DiffEntry::insert(
            "user_item",
            "id",
            &format!("{}_{}", msg.user_id, msg.item_id),
            "count",
            &msg.count.to_string(),
        )];
        Ok(diffs)
    }
}
