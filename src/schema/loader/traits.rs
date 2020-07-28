pub(super) trait BatchFnItem {
    type Key;
    fn key(&self) -> Self::Key;
}
