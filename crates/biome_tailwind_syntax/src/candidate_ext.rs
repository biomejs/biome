use crate::TwFullCandidate;

impl TwFullCandidate {
    /// Returns whether a leading or trailing `!` marks the candidate as important.
    pub fn is_important(&self) -> bool {
        self.excl_token().is_some() || self.legacy_important_token().is_some()
    }
}
