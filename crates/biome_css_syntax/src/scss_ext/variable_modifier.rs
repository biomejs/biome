use crate::{CssSyntaxKind, ScssVariableModifier};

impl ScssVariableModifier {
    /// Returns true if this modifier is `!default`.
    pub fn is_default(&self) -> bool {
        matches!(
            self.value().ok().map(|token| token.kind()),
            Some(CssSyntaxKind::DEFAULT_KW)
        )
    }

    /// Returns true if this modifier is `!global`.
    pub fn is_global(&self) -> bool {
        matches!(
            self.value().ok().map(|token| token.kind()),
            Some(CssSyntaxKind::GLOBAL_KW)
        )
    }

    /// Returns true if this modifier is neither `!default` nor `!global`.
    /// This includes bogus or invalid modifiers.
    pub fn is_unknown(&self) -> bool {
        !self.is_default() && !self.is_global()
    }
}
