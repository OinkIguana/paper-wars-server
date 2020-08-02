use super::{Context, Contributor, Mutation};
use anyhow::anyhow;
use data::{contributors, ContributorRole};
use diesel::dsl::*;
use diesel::prelude::*;
use juniper::FieldResult;
use uuid::Uuid;

#[derive(juniper::GraphQLInputObject)]
pub struct InviteContributor {
    account_id: Uuid,
    universe_id: Uuid,
}

#[derive(juniper::GraphQLInputObject)]
pub struct RespondToContributorInvitation {
    universe_id: Uuid,
    accepted: bool,
}

impl Mutation {
    pub(super) fn invite_contributor(
        &self,
        context: &Context,
        contributor: InviteContributor,
    ) -> FieldResult<Contributor> {
        let account_id = context.try_authenticated_account()?;
        let invitation = context.transaction(|conn| {
            self.assert_universe_owner(context, contributor.universe_id, account_id)?;
            let existing_contributor = contributors::table
                .filter(contributors::account_id.eq(contributor.account_id))
                .filter(contributors::universe_id.eq(contributor.universe_id))
                .filter(contributors::role.ne(ContributorRole::Declined));
            let contributor_exists = select(exists(existing_contributor)).get_result(conn)?;
            if contributor_exists {
                return Err(anyhow!(
                    "That account ({}) is already a contributor to this universe ({})",
                    contributor.account_id,
                    contributor.universe_id,
                ));
            }

            let invitation: data::Contributor = insert_into(contributors::table)
                .values((
                    contributors::universe_id.eq(contributor.universe_id),
                    contributors::account_id.eq(contributor.account_id),
                    contributors::role.eq(ContributorRole::Pending),
                ))
                .returning(contributors::all_columns)
                .get_result(conn)?;
            Ok(invitation)
        })?;
        let query = Contributor::new(invitation.universe_id, invitation.account_id);
        context.contributors().prime(invitation);
        Ok(query)
    }

    pub(super) fn respond_to_contributor_invitation(
        &self,
        context: &Context,
        invitation: RespondToContributorInvitation,
    ) -> FieldResult<Contributor> {
        let account_id = context.try_authenticated_account()?;
        let contributor = context.transaction(|conn| {
            let mut contributor = context
                .contributors()
                .load((invitation.universe_id, account_id))
                .ok_or_else(|| {
                    anyhow!(
                        "You ({}) havenot been invited to contribute to this universe ({}).",
                        account_id,
                        invitation.universe_id
                    )
                })?;
            contributor.role = if invitation.accepted {
                ContributorRole::Contributor
            } else {
                ContributorRole::Declined
            };
            update(&contributor)
                .set(contributors::role.eq(contributor.role))
                .execute(conn)?;
            Ok(contributor)
        })?;
        let query = Contributor::new(contributor.universe_id, contributor.account_id);
        context.contributors().prime(contributor);
        Ok(query)
    }
}
