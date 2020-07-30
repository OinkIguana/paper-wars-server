pub trait BatchFnItem {
    type Key;
    fn key(&self) -> Self::Key;
}
