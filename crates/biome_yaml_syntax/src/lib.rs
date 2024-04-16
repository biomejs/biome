#[macro_use]
mod generated;
mod syntax_node;

pub use self::generated::*;
pub use syntax_node::*;

use biome_rowan::RawSyntaxKind;

impl From<u16> for YamlSyntaxKind {
    fn from(d: u16) -> YamlSyntaxKind {
        assert!(d <= (YamlSyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, YamlSyntaxKind>(d) }
    }
}

impl From<YamlSyntaxKind> for u16 {
    fn from(k: YamlSyntaxKind) -> u16 {
        k as u16
    }
}

impl biome_rowan::SyntaxKind for YamlSyntaxKind {
    const EOF: Self = YamlSyntaxKind::EOF;
    const TOMBSTONE: Self = YamlSyntaxKind::TOMBSTONE;

    fn is_bogus(&self) -> bool {
        matches!(self, YamlSyntaxKind::YAML_BOGUS)
    }

    fn to_bogus(&self) -> Self {
        match self {
            YamlSyntaxKind::YAML_SCALAR | YamlSyntaxKind::YAML_BOGUS_VALUE => {
                YamlSyntaxKind::YAML_BOGUS_VALUE
            }
            _ => YamlSyntaxKind::YAML_BOGUS,
        }
    }

    #[inline]
    fn to_raw(&self) -> RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

    #[inline]
    fn from_raw(raw: RawSyntaxKind) -> Self {
        Self::from(raw.0)
    }

    fn is_root(&self) -> bool {
        matches!(self, YamlSyntaxKind::YAML_ROOT)
    }

    fn is_list(&self) -> bool {
        YamlSyntaxKind::is_list(*self)
    }

    fn to_string(&self) -> Option<&'static str> {
        YamlSyntaxKind::to_string(self)
    }
}
