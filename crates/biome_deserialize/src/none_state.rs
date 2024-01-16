pub trait NoneState {
    /// Returns the "none state" for a deserializable type.
    ///
    /// This represents the state in which no fields have been set and the
    /// defaults have not yet been filled in. I.e. every field is `None`.
    fn none() -> Self;
}
