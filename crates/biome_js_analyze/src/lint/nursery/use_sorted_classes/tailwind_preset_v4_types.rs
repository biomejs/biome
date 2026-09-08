//! Hand-written types referenced by the codegen-emitted
//! `tailwind_preset_v4` data. New Tailwind namespaces / value types
//! surface as a compile error against an unknown variant — add it
//! (and its `keys()` arm for `ThemeNamespace`) below.

use super::tailwind_preset_v4::{
    THEME_KEYS_ANIMATE, THEME_KEYS_ASPECT, THEME_KEYS_BACKGROUND_IMAGE, THEME_KEYS_BLUR,
    THEME_KEYS_BREAKPOINT, THEME_KEYS_COLOR, THEME_KEYS_CONTAINER, THEME_KEYS_DROP_SHADOW,
    THEME_KEYS_EASE, THEME_KEYS_FONT, THEME_KEYS_FONT_WEIGHT, THEME_KEYS_INSET_SHADOW,
    THEME_KEYS_LEADING, THEME_KEYS_PERSPECTIVE, THEME_KEYS_RADIUS, THEME_KEYS_SHADOW,
    THEME_KEYS_SPACING, THEME_KEYS_TEXT, THEME_KEYS_TEXT_SHADOW, THEME_KEYS_TRACKING,
};

// Named-path typed value categories. Matching is dispatched by the consumer
// on parser node kind (TwNumberValue / TwPercentageValue / TwModifier+number),
// not by CSS data-type predicates.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum NamedValueType {
    Number,
    Percentage,
    Ratio,
}

// CSS data types (from infer-data-type.ts). Bracketed arbitrary values use
// AST predicates — see sort_v4::resolve_arbitrary_branch.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum CssDataType {
    Color,
    Length,
    Percentage,
    Number,
    Integer,
    Ratio,
    Angle,
    Url,
    Position,
    BgSize,
    LineWidth,
    Image,
    AbsoluteSize,
    RelativeSize,
    Vector,
}

// Theme namespaces (from default theme.css).
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ThemeNamespace {
    Color,
    Spacing,
    Text,
    TextShadow,
    Font,
    FontWeight,
    Leading,
    Tracking,
    Breakpoint,
    Container,
    Radius,
    Shadow,
    InsetShadow,
    DropShadow,
    Blur,
    Perspective,
    Aspect,
    Ease,
    Animate,
    BackgroundImage,
}

impl ThemeNamespace {
    pub fn keys(self) -> &'static phf::Set<&'static str> {
        match self {
            Self::Color => &THEME_KEYS_COLOR,
            Self::Spacing => &THEME_KEYS_SPACING,
            Self::Text => &THEME_KEYS_TEXT,
            Self::TextShadow => &THEME_KEYS_TEXT_SHADOW,
            Self::Font => &THEME_KEYS_FONT,
            Self::FontWeight => &THEME_KEYS_FONT_WEIGHT,
            Self::Leading => &THEME_KEYS_LEADING,
            Self::Tracking => &THEME_KEYS_TRACKING,
            Self::Breakpoint => &THEME_KEYS_BREAKPOINT,
            Self::Container => &THEME_KEYS_CONTAINER,
            Self::Radius => &THEME_KEYS_RADIUS,
            Self::Shadow => &THEME_KEYS_SHADOW,
            Self::InsetShadow => &THEME_KEYS_INSET_SHADOW,
            Self::DropShadow => &THEME_KEYS_DROP_SHADOW,
            Self::Blur => &THEME_KEYS_BLUR,
            Self::Perspective => &THEME_KEYS_PERSPECTIVE,
            Self::Aspect => &THEME_KEYS_ASPECT,
            Self::Ease => &THEME_KEYS_EASE,
            Self::Animate => &THEME_KEYS_ANIMATE,
            Self::BackgroundImage => &THEME_KEYS_BACKGROUND_IMAGE,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VariantKind {
    Static,
    Functional,
    Compound,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VariantCompare {
    Default,
    BreakpointAsc,
    BreakpointDesc,
    ContainerAsc,
    ContainerDesc,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VariantEntry {
    pub kind: VariantKind,
    pub order: u16,
    pub compare: VariantCompare,
    /// Tailwind's `Compounds` bitflags for what this variant emits:
    /// `1` = at-rules (media / container queries), `2` = style rules
    /// (selectors), `0` = neither.
    pub compounds: u8,
    /// The `Compounds` bitflags a compound variant (`group-*`, `has-*`)
    /// accepts as its nested variant; `0` for non-compound variants.
    pub compounds_with: u8,
}

#[derive(Copy, Clone)]
pub struct UtilityEntry {
    /// Index into `SIGNATURE_POOL` — the ascending property-order
    /// indices this utility's declarations touch.
    pub sig: u16,
    /// Total declaration count, Tailwind's tie-break after the
    /// signature (wider utilities sort first).
    pub count: u8,
    /// Whether Tailwind registers a negative form (`-m-px` exists,
    /// `-flex` does not).
    pub has_negative: bool,
}

/// The `/modifier` a named branch accepts. A color branch takes an opacity
/// modifier (`bg-red-500/50`); a font-size branch takes a line-height
/// modifier (`text-lg/8`, `text-lg/loose`); every other branch takes none,
/// so a modifier makes the candidate invalid (`w-1/foo`, `p-4/2`).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ModifierKind {
    None,
    Opacity,
    LineHeight,
}

// Named-path dispatch branches inside a functional utility's compileFn.
// After the branch discriminant each variant carries a `ModifierKind` (the
// `/modifier` it accepts) then the `(u16, u8)` placement: a `SIGNATURE_POOL`
// index and the declaration count.
//
// - Theme:    theme-namespace lookup (`text-lg` ↔ `--text-lg`).
// - Keyword:  hardcoded keyword set baked into the compileFn
//             (`origin-top`, `accent-current`). First field is an index
//             into `KEYWORD_POOL`.
// - Typed:    predicate match for bare value patterns (`p-4` Number,
//             `from-25%` Percentage, `w-1/2` Ratio).
#[derive(Copy, Clone)]
pub enum NamedBranch {
    Theme(ThemeNamespace, ModifierKind, u16, u8),
    Keyword(u16, ModifierKind, u16, u8),
    Typed(NamedValueType, ModifierKind, u16, u8),
}

// Arbitrary-path dispatch branches inside a functional utility's compileFn.
// The trailing `(u16, u8)` payload is the placement, as in `NamedBranch`.
//
// - Typed:     predicate match used for utilities whose property differs by
//              CSS data type (`from-[#fff]` → `--tw-gradient-from`,
//              `from-[10px]` → `--tw-gradient-from-position`).
// - Fallback:  type-blind fallback used when the utility emits the same
//              property regardless of CSS data type (`p-[10px]`, `p-[#fff]`
//              → `padding`). Resolved after every `Typed` branch.
#[derive(Copy, Clone)]
pub enum ArbitraryBranch {
    Typed(CssDataType, ModifierKind, u16, u8),
    Fallback(ModifierKind, u16, u8),
}

#[derive(Copy, Clone)]
pub struct FunctionalEntry {
    pub named_branches: &'static [NamedBranch],
    pub arbitrary_branches: &'static [ArbitraryBranch],
    /// Placement of the bare basename when the utility compiles without
    /// a value (`border`, `ring`, `shadow` have defaults; `w` does
    /// not), as a (`SIGNATURE_POOL` index, declaration count) pair.
    pub bare: Option<(u16, u8)>,
    /// Placements of the bare basename with a modifier, split by
    /// modifier shape because the compiled declarations differ:
    /// a numeric or bracketed modifier (`shadow/50` adds an opacity
    /// declaration) versus a bare-word modifier (`@container/sidebar`
    /// names the container). `None` when the modifier invalidates the
    /// candidate, which is the case for almost every utility.
    pub bare_opacity: Option<(u16, u8)>,
    pub bare_name: Option<(u16, u8)>,
    pub negative: Option<Negative>,
}

#[derive(Copy, Clone)]
pub enum Negative {
    SameBranches,
    Distinct {
        named_branches: &'static [NamedBranch],
        arbitrary_branches: &'static [ArbitraryBranch],
    },
}
