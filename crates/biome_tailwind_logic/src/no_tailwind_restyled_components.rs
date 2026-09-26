use biome_rowan::{AstNode, AstSeparatedList, SyntaxKindSet, TextRange};
use biome_rule_options::no_tailwind_restyled_components::{
    TailwindAppearanceCategory, TailwindComponentAllowance,
};
use biome_string_case::StrLikeExtension;
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwValue, CssFunction, CssGenericComponentValueList, CssIdentifier,
    CssNumber, CssPercentage, CssRegularDimension, CssUnknownDimension, CssUrlFunction,
    TailwindLanguage, TwCandidateList,
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
static GROUP_CATEGORY: phf::Map<&'static str, TailwindAppearanceCategory> = phf::phf_map! {
    "gap" => TailwindAppearanceCategory::Spacing,
    "gap-x" => TailwindAppearanceCategory::Spacing,
    "gap-y" => TailwindAppearanceCategory::Spacing,
    "p" => TailwindAppearanceCategory::Spacing,
    "px" => TailwindAppearanceCategory::Spacing,
    "py" => TailwindAppearanceCategory::Spacing,
    "ps" => TailwindAppearanceCategory::Spacing,
    "pe" => TailwindAppearanceCategory::Spacing,
    "pbs" => TailwindAppearanceCategory::Spacing,
    "pbe" => TailwindAppearanceCategory::Spacing,
    "pt" => TailwindAppearanceCategory::Spacing,
    "pr" => TailwindAppearanceCategory::Spacing,
    "pb" => TailwindAppearanceCategory::Spacing,
    "pl" => TailwindAppearanceCategory::Spacing,
    "space-x" => TailwindAppearanceCategory::Spacing,
    "space-x-reverse" => TailwindAppearanceCategory::Spacing,
    "space-y" => TailwindAppearanceCategory::Spacing,
    "space-y-reverse" => TailwindAppearanceCategory::Spacing,
    "font-size" => TailwindAppearanceCategory::Typography,
    "font-smoothing" => TailwindAppearanceCategory::Typography,
    "font-style" => TailwindAppearanceCategory::Typography,
    "font-weight" => TailwindAppearanceCategory::Typography,
    "font-stretch" => TailwindAppearanceCategory::Typography,
    "font-family" => TailwindAppearanceCategory::Typography,
    "font-features" => TailwindAppearanceCategory::Typography,
    "fvn-normal" => TailwindAppearanceCategory::Typography,
    "fvn-ordinal" => TailwindAppearanceCategory::Typography,
    "fvn-slashed-zero" => TailwindAppearanceCategory::Typography,
    "fvn-figure" => TailwindAppearanceCategory::Typography,
    "fvn-spacing" => TailwindAppearanceCategory::Typography,
    "fvn-fraction" => TailwindAppearanceCategory::Typography,
    "tracking" => TailwindAppearanceCategory::Typography,
    "line-clamp" => TailwindAppearanceCategory::Typography,
    "leading" => TailwindAppearanceCategory::Typography,
    "list-image" => TailwindAppearanceCategory::Typography,
    "list-style-position" => TailwindAppearanceCategory::Typography,
    "list-style-type" => TailwindAppearanceCategory::Typography,
    "placeholder-color" => TailwindAppearanceCategory::Color,
    "text-color" => TailwindAppearanceCategory::Color,
    "text-decoration" => TailwindAppearanceCategory::Typography,
    "text-decoration-style" => TailwindAppearanceCategory::Typography,
    "text-decoration-thickness" => TailwindAppearanceCategory::Typography,
    "text-decoration-color" => TailwindAppearanceCategory::Color,
    "underline-offset" => TailwindAppearanceCategory::Typography,
    "text-transform" => TailwindAppearanceCategory::Typography,
    "text-overflow" => TailwindAppearanceCategory::Typography,
    "text-wrap" => TailwindAppearanceCategory::Typography,
    "indent" => TailwindAppearanceCategory::Typography,
    "hyphens" => TailwindAppearanceCategory::Typography,
    "bg-attachment" => TailwindAppearanceCategory::Effects,
    "bg-clip" => TailwindAppearanceCategory::Effects,
    "bg-origin" => TailwindAppearanceCategory::Effects,
    "bg-position" => TailwindAppearanceCategory::Effects,
    "bg-repeat" => TailwindAppearanceCategory::Effects,
    "bg-size" => TailwindAppearanceCategory::Effects,
    "bg-image" => TailwindAppearanceCategory::Effects,
    "bg-color" => TailwindAppearanceCategory::Color,
    "gradient-from-pos" => TailwindAppearanceCategory::Effects,
    "gradient-via-pos" => TailwindAppearanceCategory::Effects,
    "gradient-to-pos" => TailwindAppearanceCategory::Effects,
    "gradient-from" => TailwindAppearanceCategory::Color,
    "gradient-via" => TailwindAppearanceCategory::Color,
    "gradient-to" => TailwindAppearanceCategory::Color,
    "rounded" => TailwindAppearanceCategory::Shape,
    "rounded-s" => TailwindAppearanceCategory::Shape,
    "rounded-e" => TailwindAppearanceCategory::Shape,
    "rounded-t" => TailwindAppearanceCategory::Shape,
    "rounded-r" => TailwindAppearanceCategory::Shape,
    "rounded-b" => TailwindAppearanceCategory::Shape,
    "rounded-l" => TailwindAppearanceCategory::Shape,
    "rounded-ss" => TailwindAppearanceCategory::Shape,
    "rounded-se" => TailwindAppearanceCategory::Shape,
    "rounded-ee" => TailwindAppearanceCategory::Shape,
    "rounded-es" => TailwindAppearanceCategory::Shape,
    "rounded-tl" => TailwindAppearanceCategory::Shape,
    "rounded-tr" => TailwindAppearanceCategory::Shape,
    "rounded-br" => TailwindAppearanceCategory::Shape,
    "rounded-bl" => TailwindAppearanceCategory::Shape,
    "border-w" => TailwindAppearanceCategory::Shape,
    "border-w-x" => TailwindAppearanceCategory::Shape,
    "border-w-y" => TailwindAppearanceCategory::Shape,
    "border-w-s" => TailwindAppearanceCategory::Shape,
    "border-w-e" => TailwindAppearanceCategory::Shape,
    "border-w-bs" => TailwindAppearanceCategory::Shape,
    "border-w-be" => TailwindAppearanceCategory::Shape,
    "border-w-t" => TailwindAppearanceCategory::Shape,
    "border-w-r" => TailwindAppearanceCategory::Shape,
    "border-w-b" => TailwindAppearanceCategory::Shape,
    "border-w-l" => TailwindAppearanceCategory::Shape,
    "divide-x" => TailwindAppearanceCategory::Shape,
    "divide-x-reverse" => TailwindAppearanceCategory::Shape,
    "divide-y" => TailwindAppearanceCategory::Shape,
    "divide-y-reverse" => TailwindAppearanceCategory::Shape,
    "border-style" => TailwindAppearanceCategory::Shape,
    "divide-style" => TailwindAppearanceCategory::Shape,
    "border-color" => TailwindAppearanceCategory::Color,
    "border-color-x" => TailwindAppearanceCategory::Color,
    "border-color-y" => TailwindAppearanceCategory::Color,
    "border-color-s" => TailwindAppearanceCategory::Color,
    "border-color-e" => TailwindAppearanceCategory::Color,
    "border-color-bs" => TailwindAppearanceCategory::Color,
    "border-color-be" => TailwindAppearanceCategory::Color,
    "border-color-t" => TailwindAppearanceCategory::Color,
    "border-color-r" => TailwindAppearanceCategory::Color,
    "border-color-b" => TailwindAppearanceCategory::Color,
    "border-color-l" => TailwindAppearanceCategory::Color,
    "divide-color" => TailwindAppearanceCategory::Color,
    "outline-style" => TailwindAppearanceCategory::Shape,
    "outline-offset" => TailwindAppearanceCategory::Shape,
    "outline-w" => TailwindAppearanceCategory::Shape,
    "outline-color" => TailwindAppearanceCategory::Color,
    "shadow" => TailwindAppearanceCategory::Effects,
    "shadow-color" => TailwindAppearanceCategory::Color,
    "inset-shadow" => TailwindAppearanceCategory::Effects,
    "inset-shadow-color" => TailwindAppearanceCategory::Color,
    "ring-w" => TailwindAppearanceCategory::Shape,
    "ring-w-inset" => TailwindAppearanceCategory::Shape,
    "ring-color" => TailwindAppearanceCategory::Color,
    "ring-offset-w" => TailwindAppearanceCategory::Shape,
    "ring-offset-color" => TailwindAppearanceCategory::Color,
    "inset-ring-w" => TailwindAppearanceCategory::Shape,
    "inset-ring-color" => TailwindAppearanceCategory::Color,
    "text-shadow" => TailwindAppearanceCategory::Effects,
    "text-shadow-color" => TailwindAppearanceCategory::Color,
    "opacity" => TailwindAppearanceCategory::Effects,
    "mix-blend" => TailwindAppearanceCategory::Effects,
    "bg-blend" => TailwindAppearanceCategory::Effects,
    "mask-clip" => TailwindAppearanceCategory::Effects,
    "mask-composite" => TailwindAppearanceCategory::Effects,
    "mask-image-linear-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-linear-from-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-linear-to-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-linear-from-color" => TailwindAppearanceCategory::Color,
    "mask-image-linear-to-color" => TailwindAppearanceCategory::Color,
    "mask-image-t-from-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-t-to-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-t-from-color" => TailwindAppearanceCategory::Color,
    "mask-image-t-to-color" => TailwindAppearanceCategory::Color,
    "mask-image-r-from-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-r-to-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-r-from-color" => TailwindAppearanceCategory::Color,
    "mask-image-r-to-color" => TailwindAppearanceCategory::Color,
    "mask-image-b-from-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-b-to-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-b-from-color" => TailwindAppearanceCategory::Color,
    "mask-image-b-to-color" => TailwindAppearanceCategory::Color,
    "mask-image-l-from-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-l-to-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-l-from-color" => TailwindAppearanceCategory::Color,
    "mask-image-l-to-color" => TailwindAppearanceCategory::Color,
    "mask-image-x-from-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-x-to-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-x-from-color" => TailwindAppearanceCategory::Color,
    "mask-image-x-to-color" => TailwindAppearanceCategory::Color,
    "mask-image-y-from-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-y-to-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-y-from-color" => TailwindAppearanceCategory::Color,
    "mask-image-y-to-color" => TailwindAppearanceCategory::Color,
    "mask-image-radial" => TailwindAppearanceCategory::Effects,
    "mask-image-radial-from-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-radial-to-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-radial-from-color" => TailwindAppearanceCategory::Color,
    "mask-image-radial-to-color" => TailwindAppearanceCategory::Color,
    "mask-image-radial-shape" => TailwindAppearanceCategory::Effects,
    "mask-image-radial-size" => TailwindAppearanceCategory::Effects,
    "mask-image-radial-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-conic-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-conic-from-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-conic-to-pos" => TailwindAppearanceCategory::Effects,
    "mask-image-conic-from-color" => TailwindAppearanceCategory::Color,
    "mask-image-conic-to-color" => TailwindAppearanceCategory::Color,
    "mask-mode" => TailwindAppearanceCategory::Effects,
    "mask-origin" => TailwindAppearanceCategory::Effects,
    "mask-position" => TailwindAppearanceCategory::Effects,
    "mask-repeat" => TailwindAppearanceCategory::Effects,
    "mask-size" => TailwindAppearanceCategory::Effects,
    "mask-type" => TailwindAppearanceCategory::Effects,
    "mask-image" => TailwindAppearanceCategory::Effects,
    "filter" => TailwindAppearanceCategory::Effects,
    "blur" => TailwindAppearanceCategory::Effects,
    "brightness" => TailwindAppearanceCategory::Effects,
    "contrast" => TailwindAppearanceCategory::Effects,
    "drop-shadow" => TailwindAppearanceCategory::Effects,
    "drop-shadow-color" => TailwindAppearanceCategory::Color,
    "grayscale" => TailwindAppearanceCategory::Effects,
    "hue-rotate" => TailwindAppearanceCategory::Effects,
    "invert" => TailwindAppearanceCategory::Effects,
    "saturate" => TailwindAppearanceCategory::Effects,
    "sepia" => TailwindAppearanceCategory::Effects,
    "backdrop-filter" => TailwindAppearanceCategory::Effects,
    "backdrop-blur" => TailwindAppearanceCategory::Effects,
    "backdrop-brightness" => TailwindAppearanceCategory::Effects,
    "backdrop-contrast" => TailwindAppearanceCategory::Effects,
    "backdrop-grayscale" => TailwindAppearanceCategory::Effects,
    "backdrop-hue-rotate" => TailwindAppearanceCategory::Effects,
    "backdrop-invert" => TailwindAppearanceCategory::Effects,
    "backdrop-opacity" => TailwindAppearanceCategory::Effects,
    "backdrop-saturate" => TailwindAppearanceCategory::Effects,
    "backdrop-sepia" => TailwindAppearanceCategory::Effects,
    "border-spacing" => TailwindAppearanceCategory::Spacing,
    "border-spacing-x" => TailwindAppearanceCategory::Spacing,
    "border-spacing-y" => TailwindAppearanceCategory::Spacing,
    "transition" => TailwindAppearanceCategory::Motion,
    "transition-behavior" => TailwindAppearanceCategory::Motion,
    "duration" => TailwindAppearanceCategory::Motion,
    "ease" => TailwindAppearanceCategory::Motion,
    "delay" => TailwindAppearanceCategory::Motion,
    "animate" => TailwindAppearanceCategory::Motion,
    "accent" => TailwindAppearanceCategory::Color,
    "caret-color" => TailwindAppearanceCategory::Color,
    "scrollbar-thumb-color" => TailwindAppearanceCategory::Color,
    "scrollbar-track-color" => TailwindAppearanceCategory::Color,
    "fill" => TailwindAppearanceCategory::Color,
    "stroke-w" => TailwindAppearanceCategory::Shape,
    "stroke" => TailwindAppearanceCategory::Color,
};

/// Returns ranges of appearance utilities not covered by the supplied allowances.
/// Utilities without a mapped category are ignored.
pub fn restyled_component_ranges(
    candidates: &TwCandidateList,
    allowances: &[&TailwindComponentAllowance],
) -> Vec<TextRange> {
    candidates
        .iter()
        .flatten()
        .filter_map(|candidate| candidate.as_tw_full_candidate().cloned())
        .filter_map(|full| {
            let category = match full.candidate().ok()? {
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
                _ => None,
            }?;
            if allowances.iter().any(|allow| {
                allow.categories.contains(&category)
                    || allow
                        .classes
                        .iter()
                        .any(|class| full.syntax().text_trimmed() == class.as_ref())
            }) {
                return None;
            }
            Some(full.range())
        })
        .collect()
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

fn utility_category(base: &str, value: Option<&AnyTwValue>) -> Option<TailwindAppearanceCategory> {
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
            return Some(TailwindAppearanceCategory::Typography);
        }
        return Some(TailwindAppearanceCategory::Color);
    }
    if base == "bg" {
        if named.is_some_and(|name| name.starts_with("gradient-to-"))
            || value.is_some_and(is_image_value)
            || named.is_some_and(|name| contains(BG_EFFECT_VALUES, name))
        {
            return Some(TailwindAppearanceCategory::Effects);
        }
        return Some(TailwindAppearanceCategory::Color);
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
        return Some(TailwindAppearanceCategory::Effects);
    }
    if base == "stroke" {
        if value.is_some_and(is_length_value)
            || named.is_some_and(|name| name.parse::<f64>().is_ok())
        {
            return Some(TailwindAppearanceCategory::Shape);
        }
        return Some(TailwindAppearanceCategory::Color);
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
                TailwindAppearanceCategory::Typography
            } else {
                TailwindAppearanceCategory::Shape
            });
        }
        return Some(TailwindAppearanceCategory::Color);
    }
    if ["shadow", "inset-shadow", "text-shadow", "drop-shadow"].contains(&base) {
        if value.is_none()
            || named.is_some_and(|name| {
                contains(SHADOW_SIZES, name) || matches!(name, "inner" | "none")
            })
            || value.is_some_and(is_length_value)
        {
            return Some(TailwindAppearanceCategory::Effects);
        }
        return Some(TailwindAppearanceCategory::Color);
    }
    if matches!(base, "from" | "via" | "to") {
        return Some(if matches!(value, Some(AnyTwValue::TwPercentageValue(_))) {
            TailwindAppearanceCategory::Effects
        } else {
            TailwindAppearanceCategory::Color
        });
    }
    if matches!(base, "caret" | "placeholder") {
        return Some(TailwindAppearanceCategory::Color);
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
        return Some(TailwindAppearanceCategory::Typography);
    }
    GROUP_CATEGORY.get(base).copied()
}

fn is_length_value(value: &AnyTwValue) -> bool {
    match value {
        AnyTwValue::TwNumberValue(_) | AnyTwValue::TwPercentageValue(_) => true,
        AnyTwValue::TwArbitraryValue(value) => contains_length(&value.value()),
        AnyTwValue::TwCssVariableValue(value) => value
            .value_token()
            .is_ok_and(|token| token.text_trimmed().starts_with("length:")),
        _ => false,
    }
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
        AnyTwValue::TwCssVariableValue(value) => value
            .value_token()
            .is_ok_and(|token| token.text_trimmed().starts_with("image:")),
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
) -> Option<TailwindAppearanceCategory> {
    if property == "color" || property.ends_with("-color") || matches!(property, "fill" | "stroke")
    {
        return Some(TailwindAppearanceCategory::Color);
    }
    if property.starts_with("padding")
        || in_family(property, "gap")
        || matches!(property, "row-gap" | "column-gap")
        || in_family(property, "border-spacing")
    {
        return Some(TailwindAppearanceCategory::Spacing);
    }
    if !is_restyling_property(property, values) {
        return contains(COLOR_SHORTHANDS, property).then_some(TailwindAppearanceCategory::Color);
    }
    let category = if property.starts_with("font")
        || property.starts_with("text-decoration")
        || matches!(
            property,
            "text-transform" | "line-height" | "letter-spacing"
        ) {
        TailwindAppearanceCategory::Typography
    } else if property.starts_with("border")
        || property.starts_with("outline")
        || property == "stroke-width"
    {
        TailwindAppearanceCategory::Shape
    } else if property.starts_with("animation") || property.starts_with("transition") {
        TailwindAppearanceCategory::Motion
    } else {
        TailwindAppearanceCategory::Effects
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
    use biome_tailwind_parser::parse_tailwind;

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

    #[test]
    fn category_allowances() {
        use TailwindAppearanceCategory::{Color, Effects, Motion, Shape, Spacing, Typography};
        use biome_rule_options::no_tailwind_restyled_components::{
            TailwindAllowedComponents, TailwindAppearanceCategory, TailwindComponentAllowance,
        };

        for (input, expected) in [
            ("text-sm", Typography),
            ("text-red-500", Color),
            ("bg-cover", Effects),
            ("bg-red-500", Color),
            ("bg-gradient-to-r", Effects),
            ("stroke-2", Shape),
            ("stroke-red-500", Color),
            ("border-spacing-2", Spacing),
            ("border-2", Shape),
            ("border-red-500", Color),
            ("decoration-2", Typography),
            ("decoration-red-500", Color),
            ("shadow-lg", Effects),
            ("shadow-red-500", Color),
            ("from-50%", Effects),
            ("from-red-500", Color),
            ("caret-red-500", Color),
            ("not-italic", Typography),
            ("[font-size:14px]", Typography),
            ("[line-height:2]", Typography),
            ("[letter-spacing:2px]", Typography),
            ("[border-width:2px]", Shape),
            ("[padding:0]", Spacing),
            ("[color:red]", Color),
            ("[transition:none]", Motion),
            ("[opacity:0.5]", Effects),
        ] {
            let parse = parse_tailwind(input);
            assert!(!parse.has_errors(), "{input}");
            for category in [Color, Effects, Motion, Shape, Spacing, Typography] {
                let allow = TailwindComponentAllowance {
                    components: TailwindAllowedComponents::Name("*".into()),
                    categories: vec![category],
                    classes: vec![],
                };
                let ranges = restyled_component_ranges(&parse.tree().candidates(), &[&allow]);
                assert_eq!(
                    ranges.is_empty(),
                    category == expected,
                    "{input}: {category:?}"
                );
            }
        }
    }

    #[test]
    fn appearance_utilities() {
        for input in [
            "leading-6",
            "tracking-wide",
            "ring-offset-2",
            "outline-offset-4",
            "text-wrap",
            "rounded-none",
            "hover:!rounded-none",
            "dark:rounded-none!",
            "text-sm/6",
            "border-t-2",
            "font-bold",
            "shadow",
            "ring",
            "italic",
            "not-italic",
            "no-underline",
            "line-through",
            "normal-case",
            "subpixel-antialiased",
            "[&>span]:text-sm",
            "bg-[url('a:b')]",
            "[font-size:14px]",
            "hover:[border-radius:0]",
            "backdrop-blur-sm",
            "inset-shadow-sm",
            "text-[14px]",
            "text-[length:var(--size)]",
            "border-[3px]",
            "border-dashed",
            "shadow-lg",
            "shadow-[0_1px_2px_black]",
            "shadow-[inset_0_1px_2px_rgb(0_0_0/0.25)]",
            "[border:solid_1px_red]",
            "[border:none]",
            "[outline:dashed]",
            "[text-decoration:underline_red]",
            "[border:thin_red]",
            "shadow-[rgb(0_0_0)_0_1px]",
            "bg-linear-to-r",
            "bg-gradient-to-r",
            "bg-gradient-to-tl",
            "hover:!bg-gradient-to-b",
            "[border:1px_solid_red]",
            "[background:url(image.png)]",
            "[border-top-width:2px]",
        ] {
            let parse = parse_tailwind(input);
            assert!(!parse.has_errors(), "{input}");
            let ranges = restyled_component_ranges(&parse.tree().candidates(), &[]);
            assert_eq!(ranges.len(), 1, "{input}");
            assert_eq!(&input[ranges[0]], input);
        }
    }

    #[test]
    fn explicit_spacing_and_color_allowances() {
        use biome_rule_options::no_tailwind_restyled_components::{
            TailwindAllowedComponents, TailwindAppearanceCategory, TailwindComponentAllowance,
        };
        let allow = TailwindComponentAllowance {
            components: TailwindAllowedComponents::Name("*".into()),
            categories: vec![
                TailwindAppearanceCategory::Spacing,
                TailwindAppearanceCategory::Color,
            ],
            classes: vec![],
        };
        for input in [
            "",
            "mt-4 w-full",
            "flex grid gap-4",
            "absolute inset-0 z-10",
            "-mx-2",
            "sm:w-[400px]",
            "[margin-top:1rem]",
            "[width:100%]",
            "brand-button",
            "text bg p from",
            "background-card",
            "group peer",
            "[--brand:red]",
            "p-4 px-0 space-x-2",
            "bg-red-500 text-white border-blue-500",
            "hover:!bg-red-500/50 dark:text-white!",
            "text-[#fff] bg-[rgb(0,0,0)]",
            "text-[color:var(--brand)] text-brand",
            "border-[color:var(--brand)]",
            "ring-red-500 outline-blue-500 decoration-red-500 drop-shadow-red-500",
            "shadow-red-500 shadow-[#fff]",
            "shadow-[rgb(0_0_0/0.25)] border-[rgb(0_0_0/0.25)]",
            "fill-current stroke-red-500 accent-blue-500 caret-black",
            "stroke-none",
            "bg-opacity-50",
            "[color:red] [background-color:red] [border-color:red] [padding:0]",
            "[border:red] [outline:rgb(0_0_0)] [text-decoration:red]",
            "border-spacing-2",
            "text-left",
        ] {
            let parse = parse_tailwind(input);
            assert!(!parse.has_errors(), "{input}");
            assert!(
                restyled_component_ranges(&parse.tree().candidates(), &[&allow]).is_empty(),
                "{input}"
            );
        }
    }
    #[test]
    fn no_categories_allowed_by_default() {
        for input in [
            "p-4",
            "gap-2",
            "bg-red-500",
            "text-white",
            "shadow-red-500",
            "caret-black",
            "from-red-500",
            "[color:red]",
            "[padding:0]",
            "[border:red]",
            "[background:red]",
            "rounded-none",
            "font-bold",
            "opacity-50",
            "animate-spin",
        ] {
            let parse = parse_tailwind(input);
            assert!(!parse.has_errors(), "{input}");
            let ranges = restyled_component_ranges(&parse.tree().candidates(), &[]);
            assert_eq!(ranges.len(), 1, "{input}");
        }
        let parse = parse_tailwind("mt-4 w-full flex text-left appearance-none border-collapse");
        assert!(!parse.has_errors());
        assert!(restyled_component_ranges(&parse.tree().candidates(), &[]).is_empty());
    }
}
