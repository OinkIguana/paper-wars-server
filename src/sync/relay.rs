use std::sync::{Arc, RwLock};

#[derive(Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Relay<T>(Arc<RwLock<T>>);
