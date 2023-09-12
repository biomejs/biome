/// Simple trait to merge two types of the same type
pub trait MergeWith<T> {
    /// Merges one type with another
    fn merge_with(&mut self, other: T);

    /// Merges one type with another, if the condition is met
    fn merge_with_if(&mut self, other: T, condition: bool) {
        if condition {
            self.merge_with(other)
        }
    }
}
