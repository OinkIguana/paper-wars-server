use super::Context;

#[async_trait::async_trait]
pub trait QueryWrapper {
    type Model;

    async fn load(&self, context: &Context) -> anyhow::Result<Self::Model>;
}
