//! AST predicates for Tailwind v4 typed arbitrary value classification.
//!
//! Each CssDataType predicate receives a `CssGenericComponentValueList` and
//! returns whether the parsed arbitrary value satisfies that type. The caller
//! walks utility branches in preset order, mirroring Tailwind's
//! `infer-data-type.ts` priority model without collapsing the value to one
//! global type first.

use biome_rowan::{AstNode, AstNodeList};
use biome_tailwind_syntax::{
    AnyCssDimension, AnyCssFunction, AnyCssGenericComponentValue, AnyCssValue,
    CssFunction, CssGenericComponentValueList,
};

use super::tailwind_preset_v4_types::CssDataType;

const LENGTH_UNITS: &[&str] = &[
    "cm", "mm", "Q", "in", "pc", "pt", "px", "em", "ex", "ch", "rem", "lh", "rlh", "vw",
    "vh", "vmin", "vmax", "vb", "vi", "svw", "svh", "lvw", "lvh", "dvw", "dvh", "cqw", "cqh",
    "cqi", "cqb", "cqmin", "cqmax",
];

const ANGLE_UNITS: &[&str] = &["deg", "rad", "grad", "turn"];

const MATH_FUNCTIONS: &[&str] = &[
    "calc", "min", "max", "clamp", "mod", "rem", "sin", "cos", "tan", "asin", "acos", "atan",
    "atan2", "pow", "sqrt", "hypot", "log", "exp", "round",
];

const COLOR_FUNCTIONS: &[&str] = &[
    "rgb",
    "rgba",
    "hsl",
    "hsla",
    "hwb",
    "color",
    "lab",
    "lch",
    "oklab",
    "oklch",
    "light-dark",
    "color-mix",
];

const NAMED_COLORS: &[&str] = &[
    "black",
    "silver",
    "gray",
    "white",
    "maroon",
    "red",
    "purple",
    "fuchsia",
    "green",
    "lime",
    "olive",
    "yellow",
    "navy",
    "blue",
    "teal",
    "aqua",
    "aliceblue",
    "antiquewhite",
    "aquamarine",
    "azure",
    "beige",
    "bisque",
    "blanchedalmond",
    "blueviolet",
    "brown",
    "burlywood",
    "cadetblue",
    "chartreuse",
    "chocolate",
    "coral",
    "cornflowerblue",
    "cornsilk",
    "crimson",
    "cyan",
    "darkblue",
    "darkcyan",
    "darkgoldenrod",
    "darkgray",
    "darkgreen",
    "darkgrey",
    "darkkhaki",
    "darkmagenta",
    "darkolivegreen",
    "darkorange",
    "darkorchid",
    "darkred",
    "darksalmon",
    "darkseagreen",
    "darkslateblue",
    "darkslategray",
    "darkslategrey",
    "darkturquoise",
    "darkviolet",
    "deeppink",
    "deepskyblue",
    "dimgray",
    "dimgrey",
    "dodgerblue",
    "firebrick",
    "floralwhite",
    "forestgreen",
    "gainsboro",
    "ghostwhite",
    "gold",
    "goldenrod",
    "greenyellow",
    "grey",
    "honeydew",
    "hotpink",
    "indianred",
    "indigo",
    "ivory",
    "khaki",
    "lavender",
    "lavenderblush",
    "lawngreen",
    "lemonchiffon",
    "lightblue",
    "lightcoral",
    "lightcyan",
    "lightgoldenrodyellow",
    "lightgray",
    "lightgreen",
    "lightgrey",
    "lightpink",
    "lightsalmon",
    "lightseagreen",
    "lightskyblue",
    "lightslategray",
    "lightslategrey",
    "lightsteelblue",
    "lightyellow",
    "limegreen",
    "linen",
    "magenta",
    "mediumaquamarine",
    "mediumblue",
    "mediumorchid",
    "mediumpurple",
    "mediumseagreen",
    "mediumslateblue",
    "mediumspringgreen",
    "mediumturquoise",
    "mediumvioletred",
    "midnightblue",
    "mintcream",
    "mistyrose",
    "moccasin",
    "navajowhite",
    "oldlace",
    "olivedrab",
    "orange",
    "orangered",
    "orchid",
    "palegoldenrod",
    "palegreen",
    "paleturquoise",
    "palevioletred",
    "papayawhip",
    "peachpuff",
    "peru",
    "pink",
    "plum",
    "powderblue",
    "rebeccapurple",
    "rosybrown",
    "royalblue",
    "saddlebrown",
    "salmon",
    "sandybrown",
    "seagreen",
    "seashell",
    "sienna",
    "skyblue",
    "slateblue",
    "slategray",
    "slategrey",
    "snow",
    "springgreen",
    "steelblue",
    "tan",
    "thistle",
    "tomato",
    "turquoise",
    "violet",
    "wheat",
    "whitesmoke",
    "yellowgreen",
    "transparent",
    "currentcolor",
    "canvas",
    "canvastext",
    "linktext",
    "visitedtext",
    "activetext",
    "buttonface",
    "buttontext",
    "buttonborder",
    "field",
    "fieldtext",
    "highlight",
    "highlighttext",
    "selecteditem",
    "selecteditemtext",
    "mark",
    "marktext",
    "graytext",
    "accentcolor",
    "accentcolortext",
];

pub fn value_matches_type(list: &CssGenericComponentValueList, vt: CssDataType) -> bool {
    if starts_with_var_function(list) {
        return false;
    }

    match vt {
        CssDataType::Position => return is_position(list),
        CssDataType::BgSize => return is_bg_size(list),
        CssDataType::LineWidth => return is_line_width(list),
        CssDataType::Image => return is_image(list),
        CssDataType::Vector => return is_vector(list),
        _ => {}
    }

    let Some(value) = single_value(list) else {
        return false;
    };

    match vt {
        CssDataType::Color => is_color(&value),
        CssDataType::Length => is_length(&value),
        CssDataType::Percentage => is_percentage(&value),
        CssDataType::Number => is_number(&value),
        CssDataType::Integer => is_positive_integer(&value),
        CssDataType::Ratio => is_ratio(&value),
        CssDataType::Angle => is_angle(&value),
        CssDataType::Url => is_url(&value),
        CssDataType::AbsoluteSize => is_identifier_one_of(
            &value,
            &[
                "xx-small", "x-small", "small", "medium", "large", "x-large", "xx-large",
                "xxx-large",
            ],
        ),
        CssDataType::RelativeSize => is_identifier_one_of(&value, &["larger", "smaller"]),
        CssDataType::Position
        | CssDataType::BgSize
        | CssDataType::LineWidth
        | CssDataType::Image
        | CssDataType::Vector => unreachable!(),
    }
}

fn single_value(list: &CssGenericComponentValueList) -> Option<AnyCssValue> {
    let mut iter = list.iter();
    let first = iter.next()?;
    if iter.next().is_some() {
        return None;
    }

    match first {
        AnyCssGenericComponentValue::AnyCssValue(value) => Some(value),
        AnyCssGenericComponentValue::CssGenericDelimiter(_) => None,
    }
}

fn identifier_matches(value: &AnyCssValue, mut predicate: impl FnMut(&str) -> bool) -> bool {
    match value {
        AnyCssValue::CssIdentifier(ident) => ident
            .ident_token()
            .ok()
            .is_some_and(|token| predicate(token.text_trimmed())),
        AnyCssValue::CssDashedIdentifier(ident) => ident
            .ident_token()
            .ok()
            .is_some_and(|token| predicate(token.text_trimmed())),
        _ => false,
    }
}

fn is_identifier_one_of(value: &AnyCssValue, accepted: &[&str]) -> bool {
    identifier_matches(value, |text| {
        accepted
            .iter()
            .any(|keyword| text.eq_ignore_ascii_case(keyword))
    })
}

fn starts_with_var_function(list: &CssGenericComponentValueList) -> bool {
    list.iter().next().is_some_and(|component| match component {
        AnyCssGenericComponentValue::AnyCssValue(value) => is_var_function(&value),
        AnyCssGenericComponentValue::CssGenericDelimiter(_) => false,
    })
}

fn css_function_name_matches(
    function: &CssFunction,
    mut predicate: impl FnMut(&str) -> bool,
) -> bool {
    function
        .name()
        .ok()
        .and_then(|name| name.ident_token().ok())
        .is_some_and(|token| predicate(token.text_trimmed()))
}

fn any_css_function_name_matches(
    function: &AnyCssFunction,
    mut predicate: impl FnMut(&str) -> bool,
) -> bool {
    match function {
        AnyCssFunction::CssFunction(function) => css_function_name_matches(function, predicate),
        AnyCssFunction::CssUrlFunction(_) => predicate("url"),
    }
}

fn function_name_matches(value: &AnyCssValue, predicate: impl FnMut(&str) -> bool) -> bool {
    let Some(function) = value.as_any_css_function() else {
        return false;
    };
    any_css_function_name_matches(function, predicate)
}

fn function_name_is(value: &AnyCssValue, name: &str) -> bool {
    function_name_matches(value, |function_name| {
        function_name.eq_ignore_ascii_case(name)
    })
}

fn function_name_in_case_insensitive(value: &AnyCssValue, names: &[&str]) -> bool {
    function_name_matches(value, |function_name| {
        names
            .iter()
            .any(|name| function_name.eq_ignore_ascii_case(name))
    })
}

fn is_var_function(value: &AnyCssValue) -> bool {
    function_name_is(value, "var")
}

fn has_math_fn(value: &AnyCssValue) -> bool {
    if let AnyCssValue::AnyCssFunction(AnyCssFunction::CssFunction(function)) = value
        && css_function_name_matches(function, |function_name| {
            MATH_FUNCTIONS
                .iter()
                .any(|name| function_name.eq_ignore_ascii_case(name))
        })
    {
        return true;
    }

    value.syntax().descendants().any(|node| {
        CssFunction::cast_ref(&node).is_some_and(|function| {
            css_function_name_matches(&function, |function_name| {
                MATH_FUNCTIONS
                    .iter()
                    .any(|name| function_name.eq_ignore_ascii_case(name))
            })
        })
    })
}

fn unary_argument(value: &AnyCssValue) -> Option<AnyCssValue> {
    let AnyCssValue::CssUnaryExpression(unary) = value else {
        return None;
    };
    let operator = unary.operator().ok()?;
    let text = operator.text_trimmed();
    if text != "-" && text != "+" {
        return None;
    }

    unary.argument().ok()
}

fn is_dimension_with_unit(value: &AnyCssValue, units: &[&str]) -> bool {
    let AnyCssValue::AnyCssDimension(dimension) = value else {
        return false;
    };

    let unit_token = match dimension {
        AnyCssDimension::CssRegularDimension(dimension) => dimension.unit_token(),
        AnyCssDimension::CssUnknownDimension(dimension) => dimension.unit_token(),
    };
    let Ok(unit_token) = unit_token else {
        return false;
    };
    let unit = unit_token.text_trimmed();

    units.contains(&unit)
}

fn is_signed_dimension_with_unit(value: &AnyCssValue, units: &[&str]) -> bool {
    is_dimension_with_unit(value, units)
        || unary_argument(value).is_some_and(|argument| is_dimension_with_unit(&argument, units))
}

fn is_length(value: &AnyCssValue) -> bool {
    is_signed_dimension_with_unit(value, LENGTH_UNITS) || has_math_fn(value)
}

fn is_angle(value: &AnyCssValue) -> bool {
    is_signed_dimension_with_unit(value, ANGLE_UNITS)
}

fn is_percentage_literal(value: &AnyCssValue) -> bool {
    value.as_css_percentage().is_some()
        || unary_argument(value).is_some_and(|argument| argument.as_css_percentage().is_some())
}

fn is_percentage(value: &AnyCssValue) -> bool {
    is_percentage_literal(value) || has_math_fn(value)
}

fn is_number_literal(value: &AnyCssValue) -> bool {
    value.as_css_number().is_some()
        || unary_argument(value).is_some_and(|argument| argument.as_css_number().is_some())
}

fn is_number(value: &AnyCssValue) -> bool {
    is_number_literal(value) || has_math_fn(value)
}

fn is_positive_integer(value: &AnyCssValue) -> bool {
    let AnyCssValue::CssNumber(number) = value else {
        return false;
    };
    let Ok(token) = number.value_token() else {
        return false;
    };
    let text = token.text_trimmed();
    if !is_canonical_positive_integer_text(text) {
        return false;
    }

    text.parse::<u64>().is_ok()
}

fn is_canonical_positive_integer_text(text: &str) -> bool {
    if text == "0" {
        return true;
    }

    let mut bytes = text.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    if !matches!(first, b'1'..=b'9') {
        return false;
    }

    bytes.all(|byte| byte.is_ascii_digit())
}

fn is_ratio(value: &AnyCssValue) -> bool {
    value.as_css_ratio().is_some() || has_math_fn(value)
}

fn is_color(value: &AnyCssValue) -> bool {
    if value.as_css_color().is_some() {
        return true;
    }
    if function_name_in_case_insensitive(value, COLOR_FUNCTIONS) {
        return true;
    }
    identifier_matches(value, |text| {
        NAMED_COLORS
            .iter()
            .any(|color| text.eq_ignore_ascii_case(color))
    })
}

fn is_url(value: &AnyCssValue) -> bool {
    function_name_is(value, "url")
}

fn is_gradient_function(value: &AnyCssValue) -> bool {
    function_name_in_case_insensitive(
        value,
        &[
            "linear-gradient",
            "radial-gradient",
            "conic-gradient",
            "repeating-linear-gradient",
            "repeating-radial-gradient",
            "repeating-conic-gradient",
        ],
    )
}

fn is_image_function(value: &AnyCssValue) -> bool {
    function_name_in_case_insensitive(value, &["element", "image", "cross-fade", "image-set"])
}

fn split_by_comma(list: &CssGenericComponentValueList) -> Option<Vec<Vec<AnyCssValue>>> {
    let mut segments = vec![Vec::new()];
    for component in list.iter() {
        match component {
            AnyCssGenericComponentValue::AnyCssValue(value) => {
                segments.last_mut()?.push(value);
            }
            AnyCssGenericComponentValue::CssGenericDelimiter(delimiter) => {
                let token = delimiter.value().ok()?;
                if token.text_trimmed() != "," {
                    return None;
                }
                segments.push(Vec::new());
            }
        }
    }

    Some(segments)
}

fn is_image(list: &CssGenericComponentValueList) -> bool {
    let Some(segments) = split_by_comma(list) else {
        return false;
    };
    let mut count = 0;

    for segment in segments {
        let [value] = segment.as_slice() else {
            return false;
        };
        if is_var_function(value) {
            continue;
        }
        if is_url(value) || is_gradient_function(value) || is_image_function(value) {
            count += 1;
            continue;
        }
        return false;
    }

    count > 0
}

fn is_line_width(list: &CssGenericComponentValueList) -> bool {
    let mut count = 0;
    for component in list.iter() {
        let AnyCssGenericComponentValue::AnyCssValue(value) = component else {
            return false;
        };
        if is_length(&value)
            || is_number(&value)
            || is_identifier_one_of(&value, &["thin", "medium", "thick"])
        {
            count += 1;
            continue;
        }
        return false;
    }

    count > 0
}

fn is_position(list: &CssGenericComponentValueList) -> bool {
    let mut count = 0;
    for component in list.iter() {
        let AnyCssGenericComponentValue::AnyCssValue(value) = component else {
            return false;
        };
        if is_var_function(&value) {
            continue;
        }
        if is_identifier_one_of(&value, &["center", "top", "right", "bottom", "left"])
            || is_length(&value)
            || is_percentage(&value)
        {
            count += 1;
            continue;
        }
        return false;
    }

    count > 0
}

fn is_bg_size(list: &CssGenericComponentValueList) -> bool {
    let Some(segments) = split_by_comma(list) else {
        return false;
    };
    let mut count = 0;

    for segment in segments {
        let values = segment.as_slice();
        if let [value] = values
            && is_identifier_one_of(value, &["cover", "contain"])
        {
            count += 1;
            continue;
        }

        if !(1..=2).contains(&values.len()) {
            return false;
        }

        if values
            .iter()
            .all(|value| is_identifier_one_of(value, &["auto"]) || is_length(value) || is_percentage(value))
        {
            count += 1;
            continue;
        }

        return false;
    }

    count > 0
}

fn is_vector(list: &CssGenericComponentValueList) -> bool {
    let mut count = 0;
    for component in list.iter() {
        let AnyCssGenericComponentValue::AnyCssValue(value) = component else {
            return false;
        };
        if !is_number_literal(&value) {
            return false;
        }
        count += 1;
    }

    count == 3
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_rowan::AstNodeList;
    use biome_tailwind_parser::parse_tailwind;
    use biome_tailwind_syntax::{AnyTwCandidate, AnyTwValue, CssGenericComponentValueList};

    macro_rules! parse_value {
        ($text:literal) => {
            parse_arbitrary_value(concat!("x-[", $text, "]"))
        };
    }

    fn parse_arbitrary_value(source: &str) -> CssGenericComponentValueList {
        let parsed = parse_tailwind(source);
        let full = parsed.tree().candidates().iter().next().unwrap();
        let full = full.as_tw_full_candidate().unwrap();
        let candidate = full.candidate().unwrap();
        let AnyTwCandidate::TwFunctionalCandidate(functional) = candidate else {
            panic!("expected functional candidate")
        };
        let AnyTwValue::TwArbitraryValue(arbitrary) = functional.value().unwrap() else {
            panic!("expected arbitrary value")
        };

        arbitrary.value()
    }

    // region: numeric

    #[test]
    fn length_matches_dimensions_and_math_functions() {
        assert!(value_matches_type(&parse_value!("10px"), CssDataType::Length));
        assert!(value_matches_type(
            &parse_value!("calc(100%-1rem)"),
            CssDataType::Length
        ));
        assert!(value_matches_type(
            &parse_value!("CALC(100%-1rem)"),
            CssDataType::Length
        ));
        assert!(!value_matches_type(&parse_value!("45deg"), CssDataType::Length));
    }

    #[test]
    fn angle_matches_only_angle_dimensions() {
        assert!(value_matches_type(&parse_value!("45deg"), CssDataType::Angle));
        assert!(value_matches_type(&parse_value!("0.5turn"), CssDataType::Angle));
        assert!(!value_matches_type(
            &parse_value!("calc(45deg+5deg)"),
            CssDataType::Angle
        ));
    }

    #[test]
    fn percentage_number_integer_and_ratio_match_expected_shapes() {
        assert!(value_matches_type(&parse_value!("50%"), CssDataType::Percentage));
        assert!(value_matches_type(
            &parse_value!("calc(50%+1px)"),
            CssDataType::Percentage
        ));
        assert!(value_matches_type(&parse_value!("-3.5"), CssDataType::Number));
        assert!(value_matches_type(&parse_value!("3"), CssDataType::Integer));
        assert!(!value_matches_type(&parse_value!("3.5"), CssDataType::Integer));
        assert!(!value_matches_type(&parse_value!("-3"), CssDataType::Integer));
        assert!(value_matches_type(&parse_value!("16/9"), CssDataType::Ratio));
        assert!(value_matches_type(
            &parse_value!("calc(16/9)"),
            CssDataType::Ratio
        ));
    }

    // endregion: numeric

    // region: color / image

    #[test]
    fn color_matches_hash_functions_and_named_colors() {
        assert!(value_matches_type(&parse_value!("#abc"), CssDataType::Color));
        assert!(value_matches_type(
            &parse_value!("rgb(0,0,0)"),
            CssDataType::Color
        ));
        assert!(value_matches_type(
            &parse_value!("color-mix(in_oklab,red,blue)"),
            CssDataType::Color
        ));
        assert!(value_matches_type(&parse_value!("rebeccapurple"), CssDataType::Color));
        assert!(value_matches_type(
            &parse_value!("currentColor"),
            CssDataType::Color
        ));
        assert!(!value_matches_type(&parse_value!("10px"), CssDataType::Color));
    }

    #[test]
    fn url_and_image_match_expected_functions() {
        assert!(value_matches_type(
            &parse_value!("url('/a.png')"),
            CssDataType::Url
        ));
        assert!(value_matches_type(
            &parse_value!("URL('/a.png')"),
            CssDataType::Url
        ));
        assert!(value_matches_type(
            &parse_value!("url('/a.png')"),
            CssDataType::Image
        ));
        assert!(value_matches_type(
            &parse_value!("linear-gradient(red,blue)"),
            CssDataType::Image
        ));
        assert!(value_matches_type(
            &parse_value!("LINEAR-GRADIENT(red,blue)"),
            CssDataType::Image
        ));
        assert!(value_matches_type(
            &parse_value!("image-set(url(a.png)_1x,url(b.png)_2x)"),
            CssDataType::Image
        ));
        assert!(!value_matches_type(
            &parse_value!("linear-gradient(red,blue)"),
            CssDataType::Url
        ));
        assert!(!value_matches_type(&parse_value!("red"), CssDataType::Image));
    }

    // endregion: color / image

    // region: keyword-ish

    #[test]
    fn line_width_matches_single_or_multi_width_values() {
        assert!(value_matches_type(&parse_value!("thin"), CssDataType::LineWidth));
        assert!(value_matches_type(&parse_value!("THIN"), CssDataType::LineWidth));
        assert!(value_matches_type(&parse_value!("2px"), CssDataType::LineWidth));
        assert!(value_matches_type(
            &parse_value!("1px 2px"),
            CssDataType::LineWidth
        ));
        assert!(!value_matches_type(&parse_value!("solid"), CssDataType::LineWidth));
    }

    #[test]
    fn font_size_keywords_match_their_specific_value_types() {
        assert!(value_matches_type(
            &parse_value!("xx-small"),
            CssDataType::AbsoluteSize
        ));
        assert!(value_matches_type(
            &parse_value!("XX-SMALL"),
            CssDataType::AbsoluteSize
        ));
        assert!(value_matches_type(
            &parse_value!("larger"),
            CssDataType::RelativeSize
        ));
        assert!(value_matches_type(
            &parse_value!("LARGER"),
            CssDataType::RelativeSize
        ));
        assert!(!value_matches_type(
            &parse_value!("small"),
            CssDataType::RelativeSize
        ));
    }

    // endregion: keyword-ish

    // region: multi-value

    #[test]
    fn position_matches_keywords_lengths_percentages_and_vars() {
        assert!(value_matches_type(&parse_value!("top"), CssDataType::Position));
        assert!(value_matches_type(&parse_value!("TOP"), CssDataType::Position));
        assert!(value_matches_type(
            &parse_value!("top left"),
            CssDataType::Position
        ));
        assert!(value_matches_type(
            &parse_value!("50% 10px"),
            CssDataType::Position
        ));
        assert!(value_matches_type(
            &parse_value!("top var(--pos)"),
            CssDataType::Position
        ));
        assert!(!value_matches_type(
            &parse_value!("var(--pos)"),
            CssDataType::Position
        ));
        assert!(!value_matches_type(
            &parse_value!("var(--pos) top"),
            CssDataType::Position
        ));
        assert!(!value_matches_type(&parse_value!("foo"), CssDataType::Position));
    }

    #[test]
    fn background_size_matches_css_background_size_shapes() {
        assert!(value_matches_type(&parse_value!("cover"), CssDataType::BgSize));
        assert!(value_matches_type(&parse_value!("COVER"), CssDataType::BgSize));
        assert!(value_matches_type(&parse_value!("auto"), CssDataType::BgSize));
        assert!(value_matches_type(&parse_value!("AUTO"), CssDataType::BgSize));
        assert!(value_matches_type(
            &parse_value!("200px 100%"),
            CssDataType::BgSize
        ));
        assert!(value_matches_type(
            &parse_value!("200px_100%"),
            CssDataType::BgSize
        ));
        assert!(value_matches_type(
            &parse_value!("cover,contain"),
            CssDataType::BgSize
        ));
        assert!(!value_matches_type(
            &parse_value!("200px 100% 50%"),
            CssDataType::BgSize
        ));
    }

    #[test]
    fn vector_matches_exactly_three_numbers() {
        assert!(value_matches_type(&parse_value!("1 2 3"), CssDataType::Vector));
        assert!(!value_matches_type(&parse_value!("1 2"), CssDataType::Vector));
        assert!(!value_matches_type(&parse_value!("1px 2 3"), CssDataType::Vector));
    }

    // endregion: multi-value
}
