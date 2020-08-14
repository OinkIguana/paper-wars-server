use super::Context;

#[derive(Debug)]
pub struct OperationResult<T>(anyhow::Result<T>);

impl<T> From<anyhow::Result<T>> for OperationResult<T> {
    fn from(value: anyhow::Result<T>) -> Self {
        Self(value)
    }
}

impl<T> OperationResult<T> {
    pub fn success(&self) -> Option<&T> {
        self.0.as_ref().ok()
    }

    pub fn error(&self) -> Option<String> {
        self.0.as_ref().err().map(|error| error.to_string())
    }
}

#[juniper::graphql_object(Context = Context, name = "Result")]
impl OperationResult<bool> {
    pub fn success(&self) -> Option<&bool> {
        self.success()
    }

    pub fn error(&self) -> Option<String> {
        self.error()
    }
}
