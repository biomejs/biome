/// Simple trait to merge two types of the same type
pub trait MergeWith<T>
where
    T: Default + PartialEq,
{
    /// Merges one type with another
    fn merge_with(&mut self, other: T);

    /// Merges one type with another, if the condition is met
    fn merge_with_if(&mut self, other: T, condition: bool) {
        if condition {
            self.merge_with(other)
        }
    }

    /// Merges `other` only if its value is not equal to its [Default].
    fn merge_with_if_not_default(&mut self, other: T)
    where
        T: Default;
}

impl<M> MergeWith<Option<M>> for M
where
    M: MergeWith<M> + Default + PartialEq,
{
    fn merge_with(&mut self, other: Option<M>) {
        if let Some(other) = other {
            self.merge_with(other);
        }
    }

    fn merge_with_if_not_default(&mut self, other: Option<M>)
    where
        M: Default,
    {
        if let Some(other) = other {
            if other != M::default() {
                self.merge_with(other);
            }
        }
    }
}
