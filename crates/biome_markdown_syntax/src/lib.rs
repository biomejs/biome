#[macro_use]
mod generated;
mod syntax_node;

pub use self::generated::*;
use biome_rowan::RawSyntaxKind;
pub use syntax_node::*;


impl From<u16> for MarkdownSyntaxKind {
    fn from(d: u16) -> MarkdownSyntaxKind {
        assert!(d <= (MarkdownSyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, MarkdownSyntaxKind>(d) }
    }
}

impl biome_rowan::SyntaxKind for MarkdownSyntaxKind {
    const TOMBSTONE: Self = MarkdownSyntaxKind::TOMBSTONE;

    const EOF: Self = MarkdownSyntaxKind::EOF;

    fn is_bogus(&self) -> bool {
        matches!(self, MarkdownSyntaxKind::BOGUS)
    }

    fn to_bogus(&self) -> Self {
        todo!()
    }

    fn to_raw(&self) -> biome_rowan::RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

    fn from_raw(raw: biome_rowan::RawSyntaxKind) -> Self {
        Self::from(raw.0)
    }

    fn is_root(&self) -> bool {
        todo!()
    }

    fn is_list(&self) -> bool {
        MarkdownSyntaxKind::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        todo!()
    }

    fn to_string(&self) -> Option<&'static str> {
        MarkdownSyntaxKind::to_string(self)
    }
}
