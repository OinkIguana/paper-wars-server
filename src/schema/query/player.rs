use super::{Context, QueryWrapper};
use anyhow::anyhow;
use juniper::FieldResult;
use uuid::Uuid;

pub struct Player {
    game_id: Uuid,
    account_id: Uuid,
}

impl QueryWrapper for Player {
    type Model = data::Player;

    fn load(&self, context: &Context) -> anyhow::Result<Self::Model> {
        context
            .players()
            .load((self.game_id, self.account_id))
            .ok_or_else(|| {
                anyhow!(
                    "Game {} player {} does not exist",
                    self.game_id,
                    self.account_id
                )
            })
    }
}

impl Player {
    pub fn new(game_id: Uuid, account_id: Uuid) -> Self {
        Self {
            game_id,
            account_id,
        }
    }

    fn load_account(&self, context: &Context) -> anyhow::Result<data::Account> {
        context
            .accounts()
            .load(self.account_id)
            .ok_or_else(|| anyhow!("Account {} does not exist", self.account_id))
    }
}

#[juniper::graphql_object(Context = Context)]
impl Player {
    /// The ID of the player's account.
    fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load(context)?.account_id)
    }

    /// The name of the player.
    fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load_account(context)?.name.to_string())
    }

    /// The place this player's turn occurs in the order of the game.
    fn turn_order(&self, context: &Context) -> FieldResult<i32> {
        Ok(self.load(context)?.turn_order)
    }

    /// The game state that is specific to this player.
    fn state(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context)?.state.to_string())
    }
}
