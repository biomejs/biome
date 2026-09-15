use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::no_tailwind_restyled_components::{
    TailwindAppearanceCategory, TailwindComponentAllowance,
};
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwValue, CssFunction, CssGenericComponentValueList, CssIdentifier,
    TailwindSyntaxKind, TwCandidateList,
};

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
    let group = if base == "text" {
        if matches!(
            named,
            Some("left" | "right" | "center" | "justify" | "start" | "end")
        ) {
            return None;
        }
        if named.is_some_and(is_text_size) || value.is_some_and(is_length_value) {
            "font-size"
        } else if matches!(named, Some("wrap" | "nowrap" | "balance" | "pretty")) {
            "text-wrap"
        } else if matches!(named, Some("ellipsis" | "clip")) {
            "text-overflow"
        } else {
            "text-color"
        }
    } else if base == "bg" {
        if matches!(named, Some("none"))
            || named.is_some_and(|name| name.starts_with("gradient-to-"))
            || value.is_some_and(is_image_value)
        {
            "bg-image"
        } else if matches!(named, Some("fixed" | "local" | "scroll")) {
            "bg-attachment"
        } else if matches!(named, Some("cover" | "contain" | "auto")) {
            "bg-size"
        } else if matches!(named, Some("center" | "top" | "bottom" | "left" | "right")) {
            "bg-position"
        } else if matches!(
            named,
            Some(
                "repeat" | "no-repeat" | "repeat-x" | "repeat-y" | "repeat-round" | "repeat-space"
            )
        ) {
            "bg-repeat"
        } else {
            "bg-color"
        }
    } else if [
        "bg-linear",
        "bg-radial",
        "bg-conic",
        "bg-gradient",
        "bg-gradient-to",
    ]
    .contains(&base)
    {
        "bg-image"
    } else if base == "stroke" {
        if value.is_some_and(is_length_value)
            || named.is_some_and(|name| name.parse::<f64>().is_ok())
        {
            "stroke-w"
        } else {
            "stroke"
        }
    } else if in_family(base, "border-spacing") {
        base
    } else if base == "border" && matches!(named, Some("collapse" | "separate")) {
        return None;
    } else if in_family(base, "border")
        || in_family(base, "divide")
        || ["ring", "ring-offset", "inset-ring", "outline", "decoration"].contains(&base)
    {
        let has_shape = value.is_none()
            || named.is_some_and(|name| {
                name.parse::<f64>().is_ok()
                    || matches!(
                        name,
                        "solid"
                            | "dashed"
                            | "dotted"
                            | "double"
                            | "hidden"
                            | "none"
                            | "inset"
                            | "wavy"
                            | "reverse"
                    )
            })
            || value.is_some_and(is_length_value);
        if base == "decoration" {
            if has_shape {
                "text-decoration-thickness"
            } else {
                "text-decoration-color"
            }
        } else if has_shape {
            "border-w"
        } else {
            "border-color"
        }
    } else if ["shadow", "inset-shadow", "text-shadow", "drop-shadow"].contains(&base) {
        if value.is_none()
            || named.is_some_and(|name| is_shadow_size(name) || matches!(name, "inner" | "none"))
            || value.is_some_and(is_length_value)
        {
            base
        } else {
            "shadow-color"
        }
    } else if matches!(base, "from" | "via" | "to") {
        if matches!(value, Some(AnyTwValue::TwPercentageValue(_))) {
            "gradient-from-pos"
        } else {
            "gradient-from"
        }
    } else if base == "caret" {
        "caret-color"
    } else if base == "placeholder" {
        "placeholder-color"
    } else if base == "font" {
        "font-family"
    } else if matches!(base, "italic") || (base == "not" && named == Some("italic")) {
        "font-style"
    } else if matches!(base, "underline" | "overline")
        || matches!(
            (base, named),
            ("no", Some("underline")) | ("line", Some("through"))
        )
    {
        "text-decoration"
    } else if matches!(base, "uppercase" | "lowercase" | "capitalize")
        || (base == "normal" && named == Some("case"))
    {
        "text-transform"
    } else if base == "antialiased" || (base == "subpixel" && named == Some("antialiased")) {
        "font-smoothing"
    } else if base == "truncate" {
        "text-overflow"
    } else {
        base
    };
    GROUP_CATEGORY.get(group).copied()
}

fn is_text_size(name: &str) -> bool {
    matches!(
        name,
        "xs" | "sm"
            | "base"
            | "lg"
            | "xl"
            | "2xl"
            | "3xl"
            | "4xl"
            | "5xl"
            | "6xl"
            | "7xl"
            | "8xl"
            | "9xl"
    )
}

fn is_shadow_size(name: &str) -> bool {
    matches!(name, "2xs" | "xs" | "sm" | "md" | "lg" | "xl" | "2xl")
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
        matches!(
            value.kind(),
            TailwindSyntaxKind::CSS_REGULAR_DIMENSION
                | TailwindSyntaxKind::CSS_UNKNOWN_DIMENSION
                | TailwindSyntaxKind::CSS_NUMBER
                | TailwindSyntaxKind::CSS_PERCENTAGE
        ) || value
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
        .any(|node| node.kind() == TailwindSyntaxKind::CSS_URL_FUNCTION)
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
                    .is_some_and(|name| {
                        matches!(
                            name.text_trimmed(),
                            "url"
                                | "image"
                                | "image-set"
                                | "linear-gradient"
                                | "radial-gradient"
                                | "conic-gradient"
                                | "repeating-linear-gradient"
                                | "repeating-radial-gradient"
                                | "repeating-conic-gradient"
                        )
                    })
            })
}

fn property_category(
    property: &str,
    values: &CssGenericComponentValueList,
) -> Option<TailwindAppearanceCategory> {
    if property == "color" || property.ends_with("-color") || matches!(property, "fill" | "stroke")
    {
        return GROUP_CATEGORY.get("text-color").copied();
    }
    if property.starts_with("padding")
        || in_family(property, "gap")
        || matches!(property, "row-gap" | "column-gap")
        || in_family(property, "border-spacing")
    {
        return GROUP_CATEGORY.get("p").copied();
    }
    if !is_restyling_property(property, values) {
        return matches!(
            property,
            "background"
                | "border"
                | "border-top"
                | "border-right"
                | "border-bottom"
                | "border-left"
                | "outline"
                | "text-decoration"
        )
        .then(|| GROUP_CATEGORY.get("text-color").copied())
        .flatten();
    }
    let group = if property.starts_with("font")
        || property.starts_with("text-decoration")
        || property == "text-transform"
    {
        "font-family"
    } else if property.starts_with("border")
        || property.starts_with("outline")
        || property == "stroke-width"
    {
        "border-w"
    } else if property.starts_with("animation") || property.starts_with("transition") {
        "transition"
    } else {
        "filter"
    };
    GROUP_CATEGORY.get(group).copied()
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
                        [
                            "none",
                            "underline",
                            "overline",
                            "line-through",
                            "solid",
                            "double",
                            "dotted",
                            "dashed",
                            "wavy",
                            "auto",
                            "from-font",
                        ]
                        .iter()
                        .any(|keyword| token.text_trimmed().eq_ignore_ascii_case(keyword))
                    })
                });
    }
    if property == "border"
        || [
            "border-top",
            "border-right",
            "border-bottom",
            "border-left",
            "outline",
        ]
        .contains(&property)
    {
        return contains_length(values)
            || values
                .syntax()
                .children()
                .filter_map(CssIdentifier::cast)
                .any(|identifier| {
                    identifier.ident_token().is_ok_and(|token| {
                        [
                            "none", "hidden", "dotted", "dashed", "solid", "double", "groove",
                            "ridge", "inset", "outset", "auto", "thin", "medium", "thick",
                        ]
                        .iter()
                        .any(|keyword| token.text_trimmed().eq_ignore_ascii_case(keyword))
                    })
                });
    }
    [
        "font",
        "border-radius",
        "box-shadow",
        "text-shadow",
        "opacity",
        "filter",
        "backdrop-filter",
        "animation",
        "transition",
        "mask",
        "mix-blend-mode",
    ]
    .iter()
    .any(|family| in_family(property, family))
        || matches!(
            property,
            "background-image"
                | "text-transform"
                | "text-decoration-line"
                | "text-decoration-style"
                | "text-decoration-thickness"
                | "outline-width"
                | "outline-style"
                | "stroke-width"
        )
        || property.starts_with("border-")
            && (property.ends_with("-width")
                || property.ends_with("-style")
                || property.ends_with("-radius"))
}

#[cfg(test)]
mod tests {
    use super::restyled_component_ranges;
    use biome_tailwind_parser::parse_tailwind;

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
