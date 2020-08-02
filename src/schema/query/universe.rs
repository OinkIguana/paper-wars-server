use super::{Archetype, Context, Contributor, Map, Pagination, QueryWrapper, UniverseVersion};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use juniper::FieldResult;
use uuid::Uuid;

pub struct Universe {
    id: Uuid,
}

impl QueryWrapper for Universe {
    type Model = data::Universe;

    fn load(&self, context: &Context) -> anyhow::Result<Self::Model> {
        context
            .universes()
            .load(self.id)
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
    fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load(context)?.id)
    }

    /// The name of the universe. This should be compared case-insensitively.
    fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context)?.name.to_string())
    }

    /// When this universe was created.
    fn created_at(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context)?.created_at)
    }

    /// The accounts who contribute to the development of this universe.
    fn contributors(
        &self,
        context: &Context,
        search: Option<data::ContributorSearch>,
    ) -> FieldResult<Pagination<Contributor>> {
        let search = search.unwrap_or_default().for_universe(self.id);
        let items = context
            .contributors()
            .search(&search)?
            .into_iter()
            .map(|contributor| Contributor::new(contributor.universe_id, contributor.account_id));
        Ok(Pagination::new(search, items))
    }

    /// Archetypes which belong to this universe.
    fn archetypes(&self, context: &Context) -> FieldResult<Vec<Archetype>> {
        Ok(context
            .archetypes()
            .for_universe(&self.id)
            .into_iter()
            .map(|archetype| Archetype::new(archetype.id))
            .collect())
    }

    /// Maps which belong to this universe.
    fn maps(&self, context: &Context) -> FieldResult<Vec<Map>> {
        Ok(context
            .maps()
            .for_universe(&self.load(context)?.id)
            .into_iter()
            .map(|map| Map::new(map.id))
            .collect())
    }

    /// Versions of this universe.
    fn versions(&self, context: &Context) -> FieldResult<Vec<UniverseVersion>> {
        Ok(context
            .universe_versions()
            .for_universe(&self.load(context)?.id)
            .into_iter()
            .map(|version| UniverseVersion::new(version.universe_id, version.version))
            .collect())
    }

    /// The highest version number for this universe.
    #[graphql(arguments(unreleased(default = false)))]
    fn version_number(&self, context: &Context, unreleased: bool) -> FieldResult<Option<i32>> {
        Ok(context
            .universe_versions()
            .load_current(self.load(context)?.id, unreleased)?
            .map(|version| version.version))
    }
}

#[juniper::graphql_object(Context = Context, name = "UniversePagination")]
impl Pagination<Universe> {
    fn items(&self) -> &[Universe] {
        self.items()
    }

    fn total(&self) -> i32 {
        self.total()
    }

    fn start(&self, context: &Context) -> juniper::FieldResult<Option<String>> {
        self.start(context)
    }

    fn end(&self, context: &Context) -> juniper::FieldResult<Option<String>> {
        self.end(context)
    }
}
