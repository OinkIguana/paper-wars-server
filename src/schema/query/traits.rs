use super::Context;

pub trait QueryWrapper {
    type Model;

    fn load(&self, context: &Context) -> anyhow::Result<Self::Model>;
}
