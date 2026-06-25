use blackbox_diff_sync::{DiffEntry, DiffSet};

use crate::event::GameEvent;
use crate::error::GameError;

/// Command context passed to command handlers.
pub struct CommandContext {
    /// User ID issuing the command.
    pub user_id: i64,
    /// Current diff accumulator for this command.
    pub diffs: DiffSet,
}

impl CommandContext {
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            diffs: DiffSet::new(),
        }
    }

    /// Emit a diff entry for a state change.
    pub fn emit_diff(&mut self, entry: DiffEntry) {
        self.diffs.push(entry);
    }

    /// Consume the accumulated diffs (called by interceptor).
    pub fn take_diffs(&mut self) -> DiffSet {
        std::mem::take(&mut self.diffs)
    }
}

/// Command trait — business logic units that mutate state and produce diffs.
pub trait Command: Send + Sync {
    type Response;

    fn execute(&self, ctx: &mut CommandContext) -> Result<Self::Response, GameError>;
}

/// Command executor — runs commands and collects their diffs.
#[derive(Debug, Clone)]
pub struct CommandExecutor;

impl CommandExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn run<C: Command>(
        &self,
        command: &C,
        ctx: &mut CommandContext,
    ) -> Result<(C::Response, DiffSet), GameError> {
        let response = command.execute(ctx)?;
        let diffs = ctx.take_diffs();
        Ok((response, diffs))
    }
}

impl Default for CommandExecutor {
    fn default() -> Self {
        Self::new()
    }
}
