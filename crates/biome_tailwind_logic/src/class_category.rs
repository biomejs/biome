//! Sorts Tailwind utility classes into the categories that rules use to decide
//! which styles are allowed.

use biome_rowan::{AstNode, SyntaxKindSet};
use biome_rule_options::tailwind_utility_category::TailwindUtilityCategory;
use biome_string_case::StrLikeExtension;
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwValue, CssFunction, CssGenericComponentValueList, CssIdentifier,
    CssNumber, CssPercentage, CssRegularDimension, CssUnknownDimension, CssUrlFunction,
    TailwindLanguage, TwCssVariableValue,
};

/// Values of `text-*` that set alignment, which is layout.
const TEXT_ALIGN_VALUES: &[&str] = &["center", "end", "justify", "left", "right", "start"];

/// Values of `text-*` that set wrapping or overflow.
const TEXT_TYPOGRAPHY_VALUES: &[&str] =
    &["balance", "clip", "ellipsis", "nowrap", "pretty", "wrap"];

const TEXT_SIZES: &[&str] = &[
    "2xl", "3xl", "4xl", "5xl", "6xl", "7xl", "8xl", "9xl", "base", "lg", "sm", "xl", "xs",
];

/// Values of `bg-*` that set background attachment, position, repeat, or size.
const BG_EFFECT_VALUES: &[&str] = &[
    "auto",
    "bottom",
    "center",
    "contain",
    "cover",
    "fixed",
    "left",
    "local",
    "no-repeat",
    "none",
    "repeat",
    "repeat-round",
    "repeat-space",
    "repeat-x",
    "repeat-y",
    "right",
    "scroll",
    "top",
];

/// Values of border, divide, ring, outline, and decoration utilities that set shape instead of color.
const LINE_SHAPE_VALUES: &[&str] = &[
    "dashed", "dotted", "double", "hidden", "inset", "none", "reverse", "solid", "wavy",
];

const SHADOW_SIZES: &[&str] = &["2xl", "2xs", "lg", "md", "sm", "xl", "xs"];

/// Utility bases that always set typography.
const TYPOGRAPHY_BASES: &[&str] = &[
    "antialiased",
    "capitalize",
    "font",
    "italic",
    "lowercase",
    "overline",
    "truncate",
    "underline",
    "uppercase",
];

const IMAGE_FUNCTIONS: &[&str] = &[
    "conic-gradient",
    "image",
    "image-set",
    "linear-gradient",
    "radial-gradient",
    "repeating-conic-gradient",
    "repeating-linear-gradient",
    "repeating-radial-gradient",
    "url",
];

/// Shorthand properties that set color when they don't restyle anything else.
const COLOR_SHORTHANDS: &[&str] = &[
    "background",
    "border",
    "border-bottom",
    "border-left",
    "border-right",
    "border-top",
    "outline",
    "text-decoration",
];

/// Shorthand properties that combine line width, line style, and color.
const LINE_SHORTHANDS: &[&str] = &[
    "border",
    "border-bottom",
    "border-left",
    "border-right",
    "border-top",
    "outline",
];

/// Keywords of `text-decoration` that restyle more than the color, compared case-insensitively.
const TEXT_DECORATION_KEYWORDS: &[&str] = &[
    "auto",
    "dashed",
    "dotted",
    "double",
    "from-font",
    "line-through",
    "none",
    "overline",
    "solid",
    "underline",
    "wavy",
];

/// Keywords of line shorthands that restyle more than the color, compared case-insensitively.
const LINE_KEYWORDS: &[&str] = &[
    "auto", "dashed", "dotted", "double", "groove", "hidden", "inset", "medium", "none", "outset",
    "ridge", "solid", "thick", "thin",
];

/// Property families that restyle a component. Each family includes its hyphenated longhands.
const RESTYLING_FAMILIES: &[&str] = &[
    "animation",
    "backdrop-filter",
    "border-radius",
    "box-shadow",
    "filter",
    "font",
    "mask",
    "mix-blend-mode",
    "opacity",
    "text-shadow",
    "transition",
];

/// Properties that restyle a component, matched exactly instead of as families.
const RESTYLING_PROPERTIES: &[&str] = &[
    "background-image",
    "letter-spacing",
    "line-height",
    "outline-style",
    "outline-width",
    "stroke-width",
    "text-decoration-line",
    "text-decoration-style",
    "text-decoration-thickness",
    "text-transform",
];

const LENGTH_KINDS: SyntaxKindSet<TailwindLanguage> = CssRegularDimension::KIND_SET
    .union(CssUnknownDimension::KIND_SET)
    .union(CssNumber::KIND_SET)
    .union(CssPercentage::KIND_SET);

// Class-group categories from https://github.com/shadcn-ui/lint/blob/main/packages/lint/src/grammar/categories.ts.
// Groups categorized as layout (null) are omitted.
static GROUP_CATEGORY: phf::Map<&'static str, TailwindUtilityCategory> = phf::phf_map! {
    "gap" => TailwindUtilityCategory::Spacing,
    "gap-x" => TailwindUtilityCategory::Spacing,
    "gap-y" => TailwindUtilityCategory::Spacing,
    "p" => TailwindUtilityCategory::Spacing,
    "px" => TailwindUtilityCategory::Spacing,
    "py" => TailwindUtilityCategory::Spacing,
    "ps" => TailwindUtilityCategory::Spacing,
    "pe" => TailwindUtilityCategory::Spacing,
    "pbs" => TailwindUtilityCategory::Spacing,
    "pbe" => TailwindUtilityCategory::Spacing,
    "pt" => TailwindUtilityCategory::Spacing,
    "pr" => TailwindUtilityCategory::Spacing,
    "pb" => TailwindUtilityCategory::Spacing,
    "pl" => TailwindUtilityCategory::Spacing,
    "space-x" => TailwindUtilityCategory::Spacing,
    "space-x-reverse" => TailwindUtilityCategory::Spacing,
    "space-y" => TailwindUtilityCategory::Spacing,
    "space-y-reverse" => TailwindUtilityCategory::Spacing,
    "font-size" => TailwindUtilityCategory::Typography,
    "font-smoothing" => TailwindUtilityCategory::Typography,
    "font-style" => TailwindUtilityCategory::Typography,
    "font-weight" => TailwindUtilityCategory::Typography,
    "font-stretch" => TailwindUtilityCategory::Typography,
    "font-family" => TailwindUtilityCategory::Typography,
    "font-features" => TailwindUtilityCategory::Typography,
    "fvn-normal" => TailwindUtilityCategory::Typography,
    "fvn-ordinal" => TailwindUtilityCategory::Typography,
    "fvn-slashed-zero" => TailwindUtilityCategory::Typography,
    "fvn-figure" => TailwindUtilityCategory::Typography,
    "fvn-spacing" => TailwindUtilityCategory::Typography,
    "fvn-fraction" => TailwindUtilityCategory::Typography,
    "tracking" => TailwindUtilityCategory::Typography,
    "line-clamp" => TailwindUtilityCategory::Typography,
    "leading" => TailwindUtilityCategory::Typography,
    "list-image" => TailwindUtilityCategory::Typography,
    "list-style-position" => TailwindUtilityCategory::Typography,
    "list-style-type" => TailwindUtilityCategory::Typography,
    "placeholder-color" => TailwindUtilityCategory::Color,
    "text-color" => TailwindUtilityCategory::Color,
    "text-decoration" => TailwindUtilityCategory::Typography,
    "text-decoration-style" => TailwindUtilityCategory::Typography,
    "text-decoration-thickness" => TailwindUtilityCategory::Typography,
    "text-decoration-color" => TailwindUtilityCategory::Color,
    "underline-offset" => TailwindUtilityCategory::Typography,
    "text-transform" => TailwindUtilityCategory::Typography,
    "text-overflow" => TailwindUtilityCategory::Typography,
    "text-wrap" => TailwindUtilityCategory::Typography,
    "indent" => TailwindUtilityCategory::Typography,
    "hyphens" => TailwindUtilityCategory::Typography,
    "bg-attachment" => TailwindUtilityCategory::Effects,
    "bg-clip" => TailwindUtilityCategory::Effects,
    "bg-origin" => TailwindUtilityCategory::Effects,
    "bg-position" => TailwindUtilityCategory::Effects,
    "bg-repeat" => TailwindUtilityCategory::Effects,
    "bg-size" => TailwindUtilityCategory::Effects,
    "bg-image" => TailwindUtilityCategory::Effects,
    "bg-color" => TailwindUtilityCategory::Color,
    "gradient-from-pos" => TailwindUtilityCategory::Effects,
    "gradient-via-pos" => TailwindUtilityCategory::Effects,
    "gradient-to-pos" => TailwindUtilityCategory::Effects,
    "gradient-from" => TailwindUtilityCategory::Color,
    "gradient-via" => TailwindUtilityCategory::Color,
    "gradient-to" => TailwindUtilityCategory::Color,
    "rounded" => TailwindUtilityCategory::Shape,
    "rounded-s" => TailwindUtilityCategory::Shape,
    "rounded-e" => TailwindUtilityCategory::Shape,
    "rounded-t" => TailwindUtilityCategory::Shape,
    "rounded-r" => TailwindUtilityCategory::Shape,
    "rounded-b" => TailwindUtilityCategory::Shape,
    "rounded-l" => TailwindUtilityCategory::Shape,
    "rounded-ss" => TailwindUtilityCategory::Shape,
    "rounded-se" => TailwindUtilityCategory::Shape,
    "rounded-ee" => TailwindUtilityCategory::Shape,
    "rounded-es" => TailwindUtilityCategory::Shape,
    "rounded-tl" => TailwindUtilityCategory::Shape,
    "rounded-tr" => TailwindUtilityCategory::Shape,
    "rounded-br" => TailwindUtilityCategory::Shape,
    "rounded-bl" => TailwindUtilityCategory::Shape,
    "border-w" => TailwindUtilityCategory::Shape,
    "border-w-x" => TailwindUtilityCategory::Shape,
    "border-w-y" => TailwindUtilityCategory::Shape,
    "border-w-s" => TailwindUtilityCategory::Shape,
    "border-w-e" => TailwindUtilityCategory::Shape,
    "border-w-bs" => TailwindUtilityCategory::Shape,
    "border-w-be" => TailwindUtilityCategory::Shape,
    "border-w-t" => TailwindUtilityCategory::Shape,
    "border-w-r" => TailwindUtilityCategory::Shape,
    "border-w-b" => TailwindUtilityCategory::Shape,
    "border-w-l" => TailwindUtilityCategory::Shape,
    "divide-x" => TailwindUtilityCategory::Shape,
    "divide-x-reverse" => TailwindUtilityCategory::Shape,
    "divide-y" => TailwindUtilityCategory::Shape,
    "divide-y-reverse" => TailwindUtilityCategory::Shape,
    "border-style" => TailwindUtilityCategory::Shape,
    "divide-style" => TailwindUtilityCategory::Shape,
    "border-color" => TailwindUtilityCategory::Color,
    "border-color-x" => TailwindUtilityCategory::Color,
    "border-color-y" => TailwindUtilityCategory::Color,
    "border-color-s" => TailwindUtilityCategory::Color,
    "border-color-e" => TailwindUtilityCategory::Color,
    "border-color-bs" => TailwindUtilityCategory::Color,
    "border-color-be" => TailwindUtilityCategory::Color,
    "border-color-t" => TailwindUtilityCategory::Color,
    "border-color-r" => TailwindUtilityCategory::Color,
    "border-color-b" => TailwindUtilityCategory::Color,
    "border-color-l" => TailwindUtilityCategory::Color,
    "divide-color" => TailwindUtilityCategory::Color,
    "outline-style" => TailwindUtilityCategory::Shape,
    "outline-offset" => TailwindUtilityCategory::Shape,
    "outline-w" => TailwindUtilityCategory::Shape,
    "outline-color" => TailwindUtilityCategory::Color,
    "shadow" => TailwindUtilityCategory::Effects,
    "shadow-color" => TailwindUtilityCategory::Color,
    "inset-shadow" => TailwindUtilityCategory::Effects,
    "inset-shadow-color" => TailwindUtilityCategory::Color,
    "ring-w" => TailwindUtilityCategory::Shape,
    "ring-w-inset" => TailwindUtilityCategory::Shape,
    "ring-color" => TailwindUtilityCategory::Color,
    "ring-offset-w" => TailwindUtilityCategory::Shape,
    "ring-offset-color" => TailwindUtilityCategory::Color,
    "inset-ring-w" => TailwindUtilityCategory::Shape,
    "inset-ring-color" => TailwindUtilityCategory::Color,
    "text-shadow" => TailwindUtilityCategory::Effects,
    "text-shadow-color" => TailwindUtilityCategory::Color,
    "opacity" => TailwindUtilityCategory::Effects,
    "mix-blend" => TailwindUtilityCategory::Effects,
    "bg-blend" => TailwindUtilityCategory::Effects,
    "mask-clip" => TailwindUtilityCategory::Effects,
    "mask-composite" => TailwindUtilityCategory::Effects,
    "mask-image-linear-pos" => TailwindUtilityCategory::Effects,
    "mask-image-linear-from-pos" => TailwindUtilityCategory::Effects,
    "mask-image-linear-to-pos" => TailwindUtilityCategory::Effects,
    "mask-image-linear-from-color" => TailwindUtilityCategory::Color,
    "mask-image-linear-to-color" => TailwindUtilityCategory::Color,
    "mask-image-t-from-pos" => TailwindUtilityCategory::Effects,
    "mask-image-t-to-pos" => TailwindUtilityCategory::Effects,
    "mask-image-t-from-color" => TailwindUtilityCategory::Color,
    "mask-image-t-to-color" => TailwindUtilityCategory::Color,
    "mask-image-r-from-pos" => TailwindUtilityCategory::Effects,
    "mask-image-r-to-pos" => TailwindUtilityCategory::Effects,
    "mask-image-r-from-color" => TailwindUtilityCategory::Color,
    "mask-image-r-to-color" => TailwindUtilityCategory::Color,
    "mask-image-b-from-pos" => TailwindUtilityCategory::Effects,
    "mask-image-b-to-pos" => TailwindUtilityCategory::Effects,
    "mask-image-b-from-color" => TailwindUtilityCategory::Color,
    "mask-image-b-to-color" => TailwindUtilityCategory::Color,
    "mask-image-l-from-pos" => TailwindUtilityCategory::Effects,
    "mask-image-l-to-pos" => TailwindUtilityCategory::Effects,
    "mask-image-l-from-color" => TailwindUtilityCategory::Color,
    "mask-image-l-to-color" => TailwindUtilityCategory::Color,
    "mask-image-x-from-pos" => TailwindUtilityCategory::Effects,
    "mask-image-x-to-pos" => TailwindUtilityCategory::Effects,
    "mask-image-x-from-color" => TailwindUtilityCategory::Color,
    "mask-image-x-to-color" => TailwindUtilityCategory::Color,
    "mask-image-y-from-pos" => TailwindUtilityCategory::Effects,
    "mask-image-y-to-pos" => TailwindUtilityCategory::Effects,
    "mask-image-y-from-color" => TailwindUtilityCategory::Color,
    "mask-image-y-to-color" => TailwindUtilityCategory::Color,
    "mask-image-radial" => TailwindUtilityCategory::Effects,
    "mask-image-radial-from-pos" => TailwindUtilityCategory::Effects,
    "mask-image-radial-to-pos" => TailwindUtilityCategory::Effects,
    "mask-image-radial-from-color" => TailwindUtilityCategory::Color,
    "mask-image-radial-to-color" => TailwindUtilityCategory::Color,
    "mask-image-radial-shape" => TailwindUtilityCategory::Effects,
    "mask-image-radial-size" => TailwindUtilityCategory::Effects,
    "mask-image-radial-pos" => TailwindUtilityCategory::Effects,
    "mask-image-conic-pos" => TailwindUtilityCategory::Effects,
    "mask-image-conic-from-pos" => TailwindUtilityCategory::Effects,
    "mask-image-conic-to-pos" => TailwindUtilityCategory::Effects,
    "mask-image-conic-from-color" => TailwindUtilityCategory::Color,
    "mask-image-conic-to-color" => TailwindUtilityCategory::Color,
    "mask-mode" => TailwindUtilityCategory::Effects,
    "mask-origin" => TailwindUtilityCategory::Effects,
    "mask-position" => TailwindUtilityCategory::Effects,
    "mask-repeat" => TailwindUtilityCategory::Effects,
    "mask-size" => TailwindUtilityCategory::Effects,
    "mask-type" => TailwindUtilityCategory::Effects,
    "mask-image" => TailwindUtilityCategory::Effects,
    "filter" => TailwindUtilityCategory::Effects,
    "blur" => TailwindUtilityCategory::Effects,
    "brightness" => TailwindUtilityCategory::Effects,
    "contrast" => TailwindUtilityCategory::Effects,
    "drop-shadow" => TailwindUtilityCategory::Effects,
    "drop-shadow-color" => TailwindUtilityCategory::Color,
    "grayscale" => TailwindUtilityCategory::Effects,
    "hue-rotate" => TailwindUtilityCategory::Effects,
    "invert" => TailwindUtilityCategory::Effects,
    "saturate" => TailwindUtilityCategory::Effects,
    "sepia" => TailwindUtilityCategory::Effects,
    "backdrop-filter" => TailwindUtilityCategory::Effects,
    "backdrop-blur" => TailwindUtilityCategory::Effects,
    "backdrop-brightness" => TailwindUtilityCategory::Effects,
    "backdrop-contrast" => TailwindUtilityCategory::Effects,
    "backdrop-grayscale" => TailwindUtilityCategory::Effects,
    "backdrop-hue-rotate" => TailwindUtilityCategory::Effects,
    "backdrop-invert" => TailwindUtilityCategory::Effects,
    "backdrop-opacity" => TailwindUtilityCategory::Effects,
    "backdrop-saturate" => TailwindUtilityCategory::Effects,
    "backdrop-sepia" => TailwindUtilityCategory::Effects,
    "border-spacing" => TailwindUtilityCategory::Spacing,
    "border-spacing-x" => TailwindUtilityCategory::Spacing,
    "border-spacing-y" => TailwindUtilityCategory::Spacing,
    "transition" => TailwindUtilityCategory::Motion,
    "transition-behavior" => TailwindUtilityCategory::Motion,
    "duration" => TailwindUtilityCategory::Motion,
    "ease" => TailwindUtilityCategory::Motion,
    "delay" => TailwindUtilityCategory::Motion,
    "animate" => TailwindUtilityCategory::Motion,
    "accent" => TailwindUtilityCategory::Color,
    "caret-color" => TailwindUtilityCategory::Color,
    "scrollbar-thumb-color" => TailwindUtilityCategory::Color,
    "scrollbar-track-color" => TailwindUtilityCategory::Color,
    "fill" => TailwindUtilityCategory::Color,
    "stroke-w" => TailwindUtilityCategory::Shape,
    "stroke" => TailwindUtilityCategory::Color,
};

/// Returns the category of `candidate`.
///
/// Returns [`TailwindUtilityCategory::Layout`] for layout utilities, such as sizing,
/// positioning, and margins, and for candidates that can't be categorized.
pub fn candidate_category(candidate: &AnyTwCandidate) -> TailwindUtilityCategory {
    appearance_category(candidate).unwrap_or(TailwindUtilityCategory::Layout)
}

fn appearance_category(candidate: &AnyTwCandidate) -> Option<TailwindUtilityCategory> {
    match candidate {
        AnyTwCandidate::TwFunctionalCandidate(candidate) => utility_category(
            candidate.base_token().ok()?.text_trimmed(),
            Some(&candidate.value().ok()?),
        ),
        AnyTwCandidate::TwStaticCandidate(candidate) => {
            utility_category(candidate.base_token().ok()?.text_trimmed(), None)
        }
        AnyTwCandidate::TwArbitraryCandidate(candidate) => property_category(
            candidate.property_token().ok()?.text_trimmed(),
            &candidate.value(),
        ),
        AnyTwCandidate::TwBogusCandidate(_) => None,
    }
}

fn in_family(name: &str, family: &str) -> bool {
    name == family
        || name
            .strip_prefix(family)
            .is_some_and(|suffix| suffix.starts_with('-'))
}

/// Returns whether `name` belongs to one of the sorted `families`, as defined by [`in_family`].
fn in_any_family(name: &str, families: &[&str]) -> bool {
    std::iter::once(name)
        .chain(name.match_indices('-').map(|(index, _)| &name[..index]))
        .any(|prefix| families.binary_search(&prefix).is_ok())
}

fn contains(list: &[&str], value: &str) -> bool {
    list.binary_search(&value).is_ok()
}

fn contains_ignore_ascii_case(list: &[&str], value: &str) -> bool {
    list.binary_search_by(|item| item.cmp_ignore_ascii_case(value))
        .is_ok()
}

fn named_value(value: Option<&AnyTwValue>) -> Option<biome_rowan::TokenText> {
    value?
        .as_tw_named_value()?
        .value_token()
        .ok()
        .map(|token| token.token_text_trimmed())
}

fn utility_category(base: &str, value: Option<&AnyTwValue>) -> Option<TailwindUtilityCategory> {
    let named = named_value(value);
    let named = named.as_ref().map(|text| text.text());
    if value.is_none() && matches!(base, "text" | "bg" | "p" | "from" | "via" | "to") {
        return None;
    }
    if base == "text" {
        if named.is_some_and(|name| contains(TEXT_ALIGN_VALUES, name)) {
            return None;
        }
        if named.is_some_and(|name| {
            contains(TEXT_SIZES, name) || contains(TEXT_TYPOGRAPHY_VALUES, name)
        }) || value.is_some_and(is_length_value)
        {
            return Some(TailwindUtilityCategory::Typography);
        }
        return Some(TailwindUtilityCategory::Color);
    }
    if base == "bg" {
        if named.is_some_and(|name| name.starts_with("gradient-to-"))
            || value.is_some_and(is_image_value)
            || named.is_some_and(|name| contains(BG_EFFECT_VALUES, name))
        {
            return Some(TailwindUtilityCategory::Effects);
        }
        return Some(TailwindUtilityCategory::Color);
    }
    if [
        "bg-linear",
        "bg-radial",
        "bg-conic",
        "bg-gradient",
        "bg-gradient-to",
    ]
    .contains(&base)
    {
        return Some(TailwindUtilityCategory::Effects);
    }
    if base == "stroke" {
        if value.is_some_and(is_length_value)
            || named.is_some_and(|name| name.parse::<f64>().is_ok())
        {
            return Some(TailwindUtilityCategory::Shape);
        }
        return Some(TailwindUtilityCategory::Color);
    }
    if in_family(base, "border-spacing") {
        return GROUP_CATEGORY.get(base).copied();
    }
    if base == "border" && matches!(named, Some("collapse" | "separate")) {
        return None;
    }
    if in_family(base, "border")
        || in_family(base, "divide")
        || ["ring", "ring-offset", "inset-ring", "outline", "decoration"].contains(&base)
    {
        let has_shape = value.is_none()
            || named.is_some_and(|name| {
                name.parse::<f64>().is_ok() || contains(LINE_SHAPE_VALUES, name)
            })
            || value.is_some_and(is_length_value);
        if has_shape {
            return Some(if base == "decoration" {
                TailwindUtilityCategory::Typography
            } else {
                TailwindUtilityCategory::Shape
            });
        }
        return Some(TailwindUtilityCategory::Color);
    }
    if ["shadow", "inset-shadow", "text-shadow", "drop-shadow"].contains(&base) {
        if value.is_none()
            || named.is_some_and(|name| {
                contains(SHADOW_SIZES, name) || matches!(name, "inner" | "none")
            })
            || value.is_some_and(is_length_value)
        {
            return Some(TailwindUtilityCategory::Effects);
        }
        return Some(TailwindUtilityCategory::Color);
    }
    if matches!(base, "from" | "via" | "to") {
        return Some(if matches!(value, Some(AnyTwValue::TwPercentageValue(_))) {
            TailwindUtilityCategory::Effects
        } else {
            TailwindUtilityCategory::Color
        });
    }
    if matches!(base, "caret" | "placeholder") {
        return Some(TailwindUtilityCategory::Color);
    }
    if contains(TYPOGRAPHY_BASES, base)
        || matches!(
            (base, named),
            ("not", Some("italic"))
                | ("no", Some("underline"))
                | ("line", Some("through"))
                | ("normal", Some("case"))
                | ("subpixel", Some("antialiased"))
        )
    {
        return Some(TailwindUtilityCategory::Typography);
    }
    GROUP_CATEGORY.get(base).copied()
}

fn is_length_value(value: &AnyTwValue) -> bool {
    match value {
        AnyTwValue::TwNumberValue(_) | AnyTwValue::TwPercentageValue(_) => true,
        AnyTwValue::TwArbitraryValue(value) => contains_length(&value.value()),
        AnyTwValue::TwCssVariableValue(value) => has_type_hint(value, "length"),
        _ => false,
    }
}

/// Whether `value` declares the type hint `hint` (`bg-(image:--a)`).
fn has_type_hint(value: &TwCssVariableValue, hint: &str) -> bool {
    value
        .type_hint()
        .and_then(|type_hint| type_hint.name_token().ok())
        .is_some_and(|name| name.text_trimmed() == hint)
}

fn contains_length(values: &CssGenericComponentValueList) -> bool {
    // Color functions contain numeric arguments, so only inspect top-level values.
    values.syntax().children().any(|value| {
        LENGTH_KINDS.matches(value.kind())
            || value
                .first_token()
                .is_some_and(|token| token.text_trimmed() == "length")
    })
}

fn is_image_value(value: &AnyTwValue) -> bool {
    match value {
        AnyTwValue::TwArbitraryValue(value) => is_image(&value.value()),
        AnyTwValue::TwCssVariableValue(value) => has_type_hint(value, "image"),
        _ => false,
    }
}

fn is_image(values: &CssGenericComponentValueList) -> bool {
    values
        .syntax()
        .descendants()
        .any(|node| CssUrlFunction::can_cast(node.kind()))
        || values
            .syntax()
            .first_token()
            .is_some_and(|token| token.text_trimmed() == "image")
        || values
            .syntax()
            .descendants()
            .filter_map(CssFunction::cast)
            .any(|function| {
                function
                    .name()
                    .ok()
                    .and_then(|name| name.ident_token().ok())
                    .is_some_and(|name| contains(IMAGE_FUNCTIONS, name.text_trimmed()))
            })
}

fn property_category(
    property: &str,
    values: &CssGenericComponentValueList,
) -> Option<TailwindUtilityCategory> {
    if property == "color" || property.ends_with("-color") || matches!(property, "fill" | "stroke")
    {
        return Some(TailwindUtilityCategory::Color);
    }
    if property.starts_with("padding")
        || in_family(property, "gap")
        || matches!(property, "row-gap" | "column-gap")
        || in_family(property, "border-spacing")
    {
        return Some(TailwindUtilityCategory::Spacing);
    }
    if !is_restyling_property(property, values) {
        return contains(COLOR_SHORTHANDS, property).then_some(TailwindUtilityCategory::Color);
    }
    let category = if property.starts_with("font")
        || property.starts_with("text-decoration")
        || matches!(
            property,
            "text-transform" | "line-height" | "letter-spacing"
        ) {
        TailwindUtilityCategory::Typography
    } else if property.starts_with("border")
        || property.starts_with("outline")
        || property == "stroke-width"
    {
        TailwindUtilityCategory::Shape
    } else if property.starts_with("animation") || property.starts_with("transition") {
        TailwindUtilityCategory::Motion
    } else {
        TailwindUtilityCategory::Effects
    };
    Some(category)
}

fn is_restyling_property(property: &str, values: &CssGenericComponentValueList) -> bool {
    if property.ends_with("-color") {
        return false;
    }
    if property == "background" {
        return is_image(values);
    }
    if property == "text-decoration" {
        return contains_length(values)
            || values
                .syntax()
                .children()
                .filter_map(CssIdentifier::cast)
                .any(|identifier| {
                    identifier.ident_token().is_ok_and(|token| {
                        contains_ignore_ascii_case(TEXT_DECORATION_KEYWORDS, token.text_trimmed())
                    })
                });
    }
    if contains(LINE_SHORTHANDS, property) {
        return contains_length(values)
            || values
                .syntax()
                .children()
                .filter_map(CssIdentifier::cast)
                .any(|identifier| {
                    identifier.ident_token().is_ok_and(|token| {
                        contains_ignore_ascii_case(LINE_KEYWORDS, token.text_trimmed())
                    })
                });
    }
    in_any_family(property, RESTYLING_FAMILIES)
        || contains(RESTYLING_PROPERTIES, property)
        || property.starts_with("border-")
            && (property.ends_with("-width")
                || property.ends_with("-style")
                || property.ends_with("-radius"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_are_sorted() {
        for list in [
            TEXT_ALIGN_VALUES,
            TEXT_TYPOGRAPHY_VALUES,
            TEXT_SIZES,
            BG_EFFECT_VALUES,
            LINE_SHAPE_VALUES,
            SHADOW_SIZES,
            TYPOGRAPHY_BASES,
            IMAGE_FUNCTIONS,
            COLOR_SHORTHANDS,
            LINE_SHORTHANDS,
            RESTYLING_FAMILIES,
            RESTYLING_PROPERTIES,
        ] {
            assert!(list.is_sorted(), "{list:?}");
        }
        for list in [TEXT_DECORATION_KEYWORDS, LINE_KEYWORDS] {
            assert!(
                list.is_sorted_by(|a, b| a.cmp_ignore_ascii_case(b).is_le()),
                "{list:?}"
            );
        }
    }
}
