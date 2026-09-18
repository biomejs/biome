/// Result of checking an inferred subject against a requested condition.
///
/// This keeps an inconclusive inference result distinct from a conclusive
/// non-match.
#[derive(Clone, Copy, Debug, Eq, PartialEq, salsa::Update)]
pub enum TypeInferenceClassification {
    /// The available type information proves that the subject matches.
    Match,
    /// The available type information proves that the subject does not match.
    NoMatch,
    /// Type inference cannot determine whether the subject matches.
    Indeterminate,
}
