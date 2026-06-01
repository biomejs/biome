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

#[derive(Copy, Clone)]
pub struct UtilityEntry {
    pub property_idx: u16,
    pub property_count: u8,
    pub registration_idx: u16,
    pub negative_registration_idx: Option<u16>,
}

// Named-path dispatch branches inside a functional utility's compileFn.
//
// - Theme:    theme-namespace lookup (`text-lg` ↔ `--text-lg`).
// - Keyword:  hardcoded keyword set baked into the compileFn
//             (`origin-top`, `accent-current`). First field is an index
//             into `KEYWORD_POOL`.
// - Typed:    predicate match for bare value patterns (`p-4` Number,
//             `from-25%` Percentage, `w-1/2` Ratio).
#[derive(Copy, Clone)]
pub enum NamedBranch {
    Theme(ThemeNamespace, u16, u8),
    Keyword(u16, u16, u8),
    Typed(NamedValueType, u16, u8),
}

// Arbitrary-path dispatch branches inside a functional utility's compileFn.
//
// - Typed:     predicate match used for utilities whose property differs by
//              CSS data type (`from-[#fff]` → `--tw-gradient-from`,
//              `from-[10px]` → `--tw-gradient-from-position`).
// - Fallback:  type-blind fallback used when the utility emits the same
//              property regardless of CSS data type (`p-[10px]`, `p-[#fff]`
//              → `padding`). Resolved after every `Typed` branch.
#[derive(Copy, Clone)]
pub enum ArbitraryBranch {
    Typed(CssDataType, u16, u8),
    Fallback(u16, u8),
}

#[derive(Copy, Clone)]
pub struct FunctionalEntry {
    pub registration_idx: u16,
    pub named_branches: &'static [NamedBranch],
    pub arbitrary_branches: &'static [ArbitraryBranch],
    pub negative: Option<Negative>,
}

#[derive(Copy, Clone)]
pub enum Negative {
    SameBranches {
        registration_idx: u16,
    },
    Distinct {
        registration_idx: u16,
        named_branches: &'static [NamedBranch],
        arbitrary_branches: &'static [ArbitraryBranch],
    },
}
