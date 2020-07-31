use super::Context;
use anyhow::anyhow;
use juniper::FieldResult;
use uuid::Uuid;

pub struct Player {
    game_id: Uuid,
    account_id: Uuid,
}

impl Player {
    pub fn new(game_id: Uuid, account_id: Uuid) -> Self {
        Self {
            game_id,
            account_id,
        }
    }

    async fn load(&self, context: &Context) -> anyhow::Result<data::Player> {
        context
            .players()
            .load((self.game_id, self.account_id))
            .await
            .ok_or_else(|| {
                anyhow!(
                    "Game {} player {} does not exist",
                    self.game_id,
                    self.account_id
                )
            })
    }

    async fn load_account(&self, context: &Context) -> anyhow::Result<data::Account> {
        context
            .accounts()
            .load(self.account_id)
            .await
            .ok_or_else(|| anyhow!("Account {} does not exist", self.account_id))
    }
}

#[juniper::graphql_object(Context = Context)]
impl Player {
    /// The ID of the player's account.
    async fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load(context).await?.account_id)
    }

    /// The name of the player.
    async fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load_account(context).await?.name.to_string())
    }

    /// The place this player's turn occurs in the order of the game.
    async fn turn_order(&self, context: &Context) -> FieldResult<i32> {
        Ok(self.load(context).await?.turn_order)
    }

    /// The game state that is specific to this player.
    async fn state(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context).await?.state.to_string())
    }
}
