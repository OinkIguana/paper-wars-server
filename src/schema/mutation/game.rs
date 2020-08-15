use super::{Context, Game, Mutation};
use data::*;
use diesel::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(juniper::GraphQLInputObject)]
pub struct CreateGame {
    name: String,
    universe: Uuid,
    map: Uuid,
    seed: String,
    players: Vec<Uuid>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct GameInvitation {
    id: Uuid,
}

impl Mutation {
    pub(super) fn create_game(
        &self,
        context: &Context,
        CreateGame {
            name,
            universe,
            map,
            seed,
            players,
        }: CreateGame,
    ) -> anyhow::Result<Game> {
        let account_id = context.try_authenticated_account()?;
        let mut seed = base64::decode(seed)?;
        seed.resize(32, 0);
        anyhow::ensure!(
            players.contains(&account_id),
            "You cannot create a game where you are not one of the players",
        );
        let game = context.transaction(|conn| {
            let universe_version = universe_versions::table
                .select(max(universe_versions::version))
                .filter(universe_versions::universe_id.eq(universe))
                .filter(universe_versions::released_at.is_not_null())
                .get_result::<Option<i32>>(conn)?
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "This universe ({}) does not exist, or has not been released",
                        universe
                    )
                })?;
            let map_exists = universe_version_maps::table
                .filter(universe_version_maps::universe_id.eq(universe))
                .filter(universe_version_maps::universe_version.eq(universe_version))
                .filter(universe_version_maps::map_id.eq(map));
            let map_exists = select(exists(map_exists)).get_result::<bool>(conn)?;
            anyhow::ensure!(
                map_exists,
                "This map ({}) is not available in the current version ({}) of the universe ({})",
                map,
                universe_version,
                universe
            );

            let game: data::Game = insert_into(games::table)
                .values((
                    games::name.eq(name),
                    games::universe_id.eq(universe),
                    games::universe_version.eq(universe_version),
                    games::map_id.eq(map),
                    games::map_seed.eq(seed),
                ))
                .returning(games::all_columns)
                .get_result(conn)?;

            for (i, player) in players.into_iter().enumerate() {
                let player_exists =
                    select(exists(accounts::table.find(player))).get_result::<bool>(conn)?;
                anyhow::ensure!(
                    player_exists,
                    "A player you have invited ({}) could not be found",
                    player,
                );
                let engagement = if player == account_id {
                    PlayerEngagement::Host
                } else {
                    PlayerEngagement::Pending
                };
                insert_into(players::table)
                    .values((
                        players::game_id.eq(game.id),
                        players::account_id.eq(player),
                        players::turn_order.eq(i as i32),
                        players::engagement.eq(engagement),
                    ))
                    .execute(conn)?;
            }
            Ok(game)
        })?;

        let query = Game::new(game.id);
        context.games().prime(game);
        Ok(query)
    }

    pub(super) fn respond_to_game_invitation(
        &self,
        context: &Context,
        GameInvitation { id }: GameInvitation,
        accepted: bool,
    ) -> anyhow::Result<Game> {
        let account_id = context.try_authenticated_account()?;
        let game = context.transaction(|conn| {
            let player: data::Player = players::table
                .filter(players::account_id.eq(account_id))
                .filter(players::game_id.eq(id))
                .get_result(conn)?;
            anyhow::ensure!(
                player.engagement == PlayerEngagement::Pending,
                "You have already responded to this invitation",
            );
            let engagement = if accepted {
                PlayerEngagement::Player
            } else {
                PlayerEngagement::Declined
            };
            update(&player)
                .set(players::engagement.eq(engagement))
                .execute(conn)?;

            let game: data::Game = games::table.find(id).get_result(conn)?;
            Ok(game)
        })?;

        let query = Game::new(game.id);
        context.games().prime(game);
        Ok(query)
    }
}
