pub trait Container<T> {
    fn __contains__(&self, item: &T) -> bool;
}

impl<T: PartialEq> Container<T> for Vec<T> {
    fn __contains__(&self, item: &T) -> bool {
        self.contains(item)
    }
}
