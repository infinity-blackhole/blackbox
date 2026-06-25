use kameo::Actor;
use blackbox_diff_sync::DiffEntry;

/// Quest actor — manages quest lifecycle.
#[derive(Actor)]
pub struct QuestActor {
    state: QuestActorState,
}

#[derive(Debug, Default)]
struct QuestActorState {
    active_quests: Vec<(i64, i32)>,
}

impl QuestActor {
    pub fn new() -> Self {
        Self {
            state: QuestActorState::default(),
        }
    }
}

/// Start a quest.
pub struct StartQuest {
    pub user_id: i64,
    pub quest_id: i32,
    pub deck_id: String,
}

impl kameo::message::Message<StartQuest> for QuestActor {
    type Reply = Result<Vec<DiffEntry>, crate::error::GameError>;

    async fn handle(
        &mut self,
        msg: StartQuest,
        _ctx: kameo::message::Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        self.state.active_quests.push((msg.user_id, msg.quest_id));
        let diffs = vec![DiffEntry::insert(
            "user_quest",
            "id",
            &format!("{}_{}", msg.user_id, msg.quest_id),
            "status",
            "started",
        )];
        Ok(diffs)
    }
}

/// Complete a quest.
pub struct CompleteQuest {
    pub user_id: i64,
    pub quest_id: i32,
    pub score: i64,
}

impl kameo::message::Message<CompleteQuest> for QuestActor {
    type Reply = Result<Vec<DiffEntry>, crate::error::GameError>;

    async fn handle(
        &mut self,
        msg: CompleteQuest,
        _ctx: kameo::message::Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        self.state.active_quests.retain(|(uid, qid)| {
            !(*uid == msg.user_id && *qid == msg.quest_id)
        });
        let diffs = vec![DiffEntry::update(
            "user_quest",
            "id",
            &format!("{}_{}", msg.user_id, msg.quest_id),
            "status",
            "in_progress",
            "completed",
        )];
        Ok(diffs)
    }
}
