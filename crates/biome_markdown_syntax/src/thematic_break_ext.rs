use crate::MdThematicBreakChar;
use biome_rowan::SyntaxResult;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MdThematicBreakMarker {
    Hyphen,
    Star,
    Underscore,
}

impl MdThematicBreakMarker {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hyphen => "-",
            Self::Star => "*",
            Self::Underscore => "_",
        }
    }
}

impl MdThematicBreakChar {
    /// Returns the marker style used by this MdThematicBreakChar node.
    pub fn marker(&self) -> SyntaxResult<MdThematicBreakMarker> {
        let token = self.value()?;
        Ok(match token.kind() {
            T![-] => MdThematicBreakMarker::Hyphen,
            T![*] => MdThematicBreakMarker::Star,
            T!["_"] => MdThematicBreakMarker::Underscore,
            _ => unreachable!(),
        })
    }
}
