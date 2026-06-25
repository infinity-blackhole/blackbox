use kameo::Actor;
use blackbox_diff_sync::DiffEntry;

/// User actor — manages user state including authentication, registration,
/// and profile updates.
#[derive(Actor)]
pub struct UserActor {
    state: UserActorState,
}

#[derive(Debug, Default)]
struct UserActorState {
    registered_user_ids: Vec<i64>,
    name_change_count: i32,
}

impl UserActor {
    pub fn new() -> Self {
        Self {
            state: UserActorState::default(),
        }
    }
}

/// Register a new user.
pub struct RegisterUser {
    pub uuid: String,
    pub platform: i32,
}

impl kameo::message::Message<RegisterUser> for UserActor {
    type Reply = Result<Vec<DiffEntry>, crate::error::GameError>;

    async fn handle(
        &mut self,
        msg: RegisterUser,
        _ctx: kameo::message::Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        let user_id = self.state.registered_user_ids.len() as i64 + 1;
        self.state.registered_user_ids.push(user_id);

        let diffs = vec![
            DiffEntry::insert("user", "id", &user_id.to_string(), "uuid", &msg.uuid),
            DiffEntry::insert(
                "user",
                "id",
                &user_id.to_string(),
                "platform",
                &msg.platform.to_string(),
            ),
        ];

        tracing::info!(user_id, uuid = %msg.uuid, "user registered");
        Ok(diffs)
    }
}

/// Set user name command.
pub struct SetUserName {
    pub user_id: i64,
    pub new_name: String,
}

impl kameo::message::Message<SetUserName> for UserActor {
    type Reply = Result<Vec<DiffEntry>, crate::error::GameError>;

    async fn handle(
        &mut self,
        msg: SetUserName,
        _ctx: kameo::message::Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        self.state.name_change_count += 1;

        let diffs = vec![DiffEntry::update(
            "user",
            "id",
            &msg.user_id.to_string(),
            "name",
            "",
            &msg.new_name,
        )];

        tracing::info!(user_id = msg.user_id, name = %msg.new_name, "user name changed");
        Ok(diffs)
    }
}
