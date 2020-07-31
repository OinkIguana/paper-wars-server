use super::{Pagination, Archetype, Context, Contributor, Map, UniverseVersion, QueryWrapper};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use juniper::FieldResult;
use uuid::Uuid;

pub struct Universe {
    id: Uuid,
}

#[async_trait::async_trait]
impl QueryWrapper for Universe {
    type Model = data::Universe;

    async fn load(&self, context: &Context) -> anyhow::Result<Self::Model> {
        context
            .universes()
            .load(self.id)
            .await
            .ok_or_else(|| anyhow!("Universe {} does not exist", self.id))
    }
}

impl Universe {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

#[juniper::graphql_object(Context = Context)]
impl Universe {
    /// The ID of the universe.
    async fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load(context).await?.id)
    }

    /// The name of the universe. This should be compared case-insensitively.
    async fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context).await?.name.to_string())
    }

    /// When this universe was created.
    async fn created_at(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context).await?.created_at)
    }

    /// The accounts who contribute to the development of this universe.
    async fn contributors(&self, context: &Context) -> FieldResult<Vec<Contributor>> {
        Ok(context
            .contributors()
            .for_universe(&self.id)
            .await
            .into_iter()
            .map(|contributor| Contributor::new(contributor.universe_id, contributor.account_id))
            .collect())
    }

    /// Archetypes which belong to this universe.
    async fn archetypes(&self, context: &Context) -> FieldResult<Vec<Archetype>> {
        Ok(context
            .archetypes()
            .for_universe(&self.id)
            .await
            .into_iter()
            .map(|archetype| Archetype::new(archetype.id))
            .collect())
    }

    /// Maps which belong to this universe.
    async fn maps(&self, context: &Context) -> FieldResult<Vec<Map>> {
        Ok(context
            .maps()
            .for_universe(&self.load(context).await?.id)
            .await
            .into_iter()
            .map(|map| Map::new(map.id))
            .collect())
    }

    /// Versions of this universe.
    async fn versions(&self, context: &Context) -> FieldResult<Vec<UniverseVersion>> {
        Ok(context
            .universe_versions()
            .for_universe(&self.load(context).await?.id)
            .await
            .into_iter()
            .map(|version| UniverseVersion::new(version.universe_id, version.version))
            .collect())
    }

    /// The highest version number for this universe.
    #[graphql(arguments(unreleased(default = false)))]
    async fn version_number(&self, context: &Context, unreleased: bool) -> FieldResult<Option<i32>> {
        Ok(context
            .universe_versions()
            .load_current(self.load(context).await?.id, unreleased)
            .await?
            .map(|version| version.version))
    }
}

#[juniper::graphql_object(Context = Context, name = "UniversePagination")]
impl Pagination<Universe> {
    async fn items(&self) -> &[Universe] {
        self.items().await
    }

    async fn total(&self) -> i32 {
        self.total().await
    }

    async fn start(&self, context: &Context) -> juniper::FieldResult<Option<String>> {
        self.start(context).await
    }

    async fn end(&self, context: &Context) -> juniper::FieldResult<Option<String>> {
        self.end(context).await
    }
}
