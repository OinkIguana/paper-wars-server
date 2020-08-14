use super::Mutation;
use data::*;
use diesel::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

impl Mutation {
    pub fn assert_universe_owner(
        &self,
        universe_id: Uuid,
        account_id: Uuid,
        conn: &DbConnection,
    ) -> anyhow::Result<()> {
        let universe_owner = contributors::table
            .filter(contributors::universe_id.eq(universe_id))
            .filter(contributors::account_id.eq(account_id))
            .filter(contributors::role.eq(ContributorRole::Owner));
        let is_universe_owner: bool = select(exists(universe_owner)).get_result(conn)?;
        anyhow::ensure!(
            is_universe_owner,
            "You ({}) are not the owner of this universe ({})",
            account_id,
            universe_id,
        );
        Ok(())
    }

    pub fn assert_universe_contributor(
        &self,
        universe_id: Uuid,
        account_id: Uuid,
        conn: &DbConnection,
    ) -> anyhow::Result<()> {
        let contributor = contributors::table
            .filter(contributors::universe_id.eq(universe_id))
            .filter(contributors::account_id.eq(account_id))
            .filter(
                contributors::role
                    .eq(ContributorRole::Owner)
                    .or(contributors::role.eq(ContributorRole::Contributor)),
            );
        let is_contributor: bool = select(exists(contributor)).get_result(conn)?;
        anyhow::ensure!(
            is_contributor,
            "You ({}) are not a contributor to this universe ({})",
            account_id,
            universe_id,
        );
        Ok(())
    }
}
