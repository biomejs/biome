//! AST predicates for Tailwind v4 ArbitraryTyped value classification.
//!
//! Each ValueType predicate receives a `CssGenericComponentValueList` and
//! returns whether the parsed arbitrary value satisfies that type. The caller
//! walks utility branches in preset order, mirroring Tailwind's
//! `infer-data-type.ts` priority model without collapsing the value to one
//! global type first.

use biome_rowan::{AstNode, AstNodeList};
use biome_string_case::StrLikeExtension;
use biome_tailwind_syntax::{
    AnyCssDimension, AnyCssFunction, AnyCssGenericComponentValue, AnyCssValue,
    CssGenericComponentValueList,
};

use super::tailwind_preset_v4_types::ValueType;

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

pub fn value_matches_type(list: &CssGenericComponentValueList, vt: ValueType) -> bool {
    if starts_with_var_function(list) {
        return false;
    }

    match vt {
        ValueType::Position => return is_position(list),
        ValueType::BgSize => return is_bg_size(list),
        ValueType::LineWidth => return is_line_width(list),
        ValueType::Image => return is_image(list),
        ValueType::Vector => return is_vector(list),
        _ => {}
    }

    let Some(value) = single_value(list) else {
        return false;
    };

    match vt {
        ValueType::Color => is_color(&value),
        ValueType::Length => is_length(&value),
        ValueType::Percentage => is_percentage(&value),
        ValueType::Number => is_number(&value),
        ValueType::Integer => is_positive_integer(&value),
        ValueType::Ratio => is_ratio(&value),
        ValueType::Angle => is_angle(&value),
        ValueType::Url => is_url(&value),
        ValueType::AbsoluteSize => is_identifier_one_of(
            &value,
            &[
                "xx-small", "x-small", "small", "medium", "large", "x-large", "xx-large",
                "xxx-large",
            ],
        ),
        ValueType::RelativeSize => is_identifier_one_of(&value, &["larger", "smaller"]),
        ValueType::Position
        | ValueType::BgSize
        | ValueType::LineWidth
        | ValueType::Image
        | ValueType::Vector => unreachable!(),
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

fn value_text(value: &AnyCssValue) -> String {
    value.syntax().text_trimmed().to_string()
}

fn ident_text(value: &AnyCssValue) -> Option<String> {
    match value {
        AnyCssValue::CssIdentifier(ident) => {
            Some(ident.ident_token().ok()?.text_trimmed().to_string())
        }
        AnyCssValue::CssDashedIdentifier(ident) => {
            Some(ident.ident_token().ok()?.text_trimmed().to_string())
        }
        _ => None,
    }
}

fn is_identifier_one_of(value: &AnyCssValue, accepted: &[&str]) -> bool {
    let Some(text) = ident_text(value) else {
        return false;
    };
    accepted.iter().any(|accepted| text == *accepted)
}

fn starts_with_var_function(list: &CssGenericComponentValueList) -> bool {
    list.iter().next().is_some_and(|component| match component {
        AnyCssGenericComponentValue::AnyCssValue(value) => is_var_function(&value),
        AnyCssGenericComponentValue::CssGenericDelimiter(_) => false,
    })
}

fn function_name(value: &AnyCssValue) -> Option<String> {
    let function = value.as_any_css_function()?;
    match function {
        AnyCssFunction::CssFunction(function) => Some(
            function
                .name()
                .ok()?
                .ident_token()
                .ok()?
                .text_trimmed()
                .to_string(),
        ),
        AnyCssFunction::CssUrlFunction(_) => Some("url".to_string()),
    }
}

fn function_name_is(value: &AnyCssValue, name: &str) -> bool {
    function_name(value).is_some_and(|function_name| function_name == name)
}

fn function_name_in(value: &AnyCssValue, names: &[&str]) -> bool {
    let Some(function_name) = function_name(value) else {
        return false;
    };
    names.iter().any(|name| function_name == *name)
}

fn function_name_in_case_insensitive(value: &AnyCssValue, names: &[&str]) -> bool {
    let Some(function_name) = function_name(value) else {
        return false;
    };
    names
        .iter()
        .any(|name| function_name.eq_ignore_ascii_case(name))
}

fn is_var_function(value: &AnyCssValue) -> bool {
    function_name_is(value, "var")
}

fn has_math_fn(value: &AnyCssValue) -> bool {
    let text = value_text(value);
    text.contains('(')
        && MATH_FUNCTIONS
            .iter()
            .any(|name| text.contains(&format!("{name}(")))
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

fn dimension_unit(value: &AnyCssValue) -> Option<String> {
    let AnyCssValue::AnyCssDimension(dimension) = value else {
        return None;
    };
    match dimension {
        AnyCssDimension::CssRegularDimension(dimension) => {
            Some(dimension.unit_token().ok()?.text_trimmed().to_string())
        }
        AnyCssDimension::CssUnknownDimension(dimension) => {
            Some(dimension.unit_token().ok()?.text_trimmed().to_string())
        }
    }
}

fn is_dimension_with_unit(value: &AnyCssValue, units: &[&str]) -> bool {
    let Some(unit) = dimension_unit(value) else {
        return false;
    };
    units.iter().any(|accepted| unit == *accepted)
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
    let Ok(parsed) = text.parse::<u64>() else {
        return false;
    };

    parsed.to_string() == text
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
    let Some(text) = ident_text(value) else {
        return false;
    };
    let text = text.to_ascii_lowercase_cow();

    NAMED_COLORS.contains(&text.as_ref())
}

fn is_url(value: &AnyCssValue) -> bool {
    function_name_is(value, "url")
}

fn is_gradient_function(value: &AnyCssValue) -> bool {
    function_name_in(
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
    function_name_in(value, &["element", "image", "cross-fade", "image-set"])
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

    fn parse_value(text: &str) -> CssGenericComponentValueList {
        let source = format!("x-[{text}]");
        let parsed = parse_tailwind(&source);
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
        assert!(value_matches_type(&parse_value("10px"), ValueType::Length));
        assert!(value_matches_type(
            &parse_value("calc(100%-1rem)"),
            ValueType::Length
        ));
        assert!(!value_matches_type(&parse_value("45deg"), ValueType::Length));
    }

    #[test]
    fn angle_matches_only_angle_dimensions() {
        assert!(value_matches_type(&parse_value("45deg"), ValueType::Angle));
        assert!(value_matches_type(&parse_value("0.5turn"), ValueType::Angle));
        assert!(!value_matches_type(
            &parse_value("calc(45deg+5deg)"),
            ValueType::Angle
        ));
    }

    #[test]
    fn percentage_number_integer_and_ratio_match_expected_shapes() {
        assert!(value_matches_type(&parse_value("50%"), ValueType::Percentage));
        assert!(value_matches_type(
            &parse_value("calc(50%+1px)"),
            ValueType::Percentage
        ));
        assert!(value_matches_type(&parse_value("-3.5"), ValueType::Number));
        assert!(value_matches_type(&parse_value("3"), ValueType::Integer));
        assert!(!value_matches_type(&parse_value("3.5"), ValueType::Integer));
        assert!(!value_matches_type(&parse_value("-3"), ValueType::Integer));
        assert!(value_matches_type(&parse_value("16/9"), ValueType::Ratio));
        assert!(value_matches_type(
            &parse_value("calc(16/9)"),
            ValueType::Ratio
        ));
    }

    // endregion: numeric

    // region: color / image

    #[test]
    fn color_matches_hash_functions_and_named_colors() {
        assert!(value_matches_type(&parse_value("#abc"), ValueType::Color));
        assert!(value_matches_type(
            &parse_value("rgb(0,0,0)"),
            ValueType::Color
        ));
        assert!(value_matches_type(
            &parse_value("color-mix(in_oklab,red,blue)"),
            ValueType::Color
        ));
        assert!(value_matches_type(&parse_value("rebeccapurple"), ValueType::Color));
        assert!(value_matches_type(
            &parse_value("currentColor"),
            ValueType::Color
        ));
        assert!(!value_matches_type(&parse_value("10px"), ValueType::Color));
    }

    #[test]
    fn url_and_image_match_expected_functions() {
        assert!(value_matches_type(
            &parse_value("url('/a.png')"),
            ValueType::Url
        ));
        assert!(value_matches_type(
            &parse_value("url('/a.png')"),
            ValueType::Image
        ));
        assert!(value_matches_type(
            &parse_value("linear-gradient(red,blue)"),
            ValueType::Image
        ));
        assert!(value_matches_type(
            &parse_value("image-set(url(a.png)_1x,url(b.png)_2x)"),
            ValueType::Image
        ));
        assert!(!value_matches_type(
            &parse_value("linear-gradient(red,blue)"),
            ValueType::Url
        ));
        assert!(!value_matches_type(&parse_value("red"), ValueType::Image));
    }

    // endregion: color / image

    // region: keyword-ish

    #[test]
    fn line_width_matches_single_or_multi_width_values() {
        assert!(value_matches_type(&parse_value("thin"), ValueType::LineWidth));
        assert!(value_matches_type(&parse_value("2px"), ValueType::LineWidth));
        assert!(value_matches_type(
            &parse_value("1px 2px"),
            ValueType::LineWidth
        ));
        assert!(!value_matches_type(&parse_value("solid"), ValueType::LineWidth));
    }

    #[test]
    fn font_size_keywords_match_their_specific_value_types() {
        assert!(value_matches_type(
            &parse_value("xx-small"),
            ValueType::AbsoluteSize
        ));
        assert!(value_matches_type(
            &parse_value("larger"),
            ValueType::RelativeSize
        ));
        assert!(!value_matches_type(
            &parse_value("small"),
            ValueType::RelativeSize
        ));
    }

    // endregion: keyword-ish

    // region: multi-value

    #[test]
    fn position_matches_keywords_lengths_percentages_and_vars() {
        assert!(value_matches_type(&parse_value("top"), ValueType::Position));
        assert!(value_matches_type(
            &parse_value("top left"),
            ValueType::Position
        ));
        assert!(value_matches_type(
            &parse_value("50% 10px"),
            ValueType::Position
        ));
        assert!(value_matches_type(
            &parse_value("top var(--pos)"),
            ValueType::Position
        ));
        assert!(!value_matches_type(
            &parse_value("var(--pos)"),
            ValueType::Position
        ));
        assert!(!value_matches_type(
            &parse_value("var(--pos) top"),
            ValueType::Position
        ));
        assert!(!value_matches_type(&parse_value("foo"), ValueType::Position));
    }

    #[test]
    fn background_size_matches_css_background_size_shapes() {
        assert!(value_matches_type(&parse_value("cover"), ValueType::BgSize));
        assert!(value_matches_type(&parse_value("auto"), ValueType::BgSize));
        assert!(value_matches_type(
            &parse_value("200px 100%"),
            ValueType::BgSize
        ));
        assert!(value_matches_type(
            &parse_value("200px_100%"),
            ValueType::BgSize
        ));
        assert!(value_matches_type(
            &parse_value("cover,contain"),
            ValueType::BgSize
        ));
        assert!(!value_matches_type(
            &parse_value("200px 100% 50%"),
            ValueType::BgSize
        ));
    }

    #[test]
    fn vector_matches_exactly_three_numbers() {
        assert!(value_matches_type(&parse_value("1 2 3"), ValueType::Vector));
        assert!(!value_matches_type(&parse_value("1 2"), ValueType::Vector));
        assert!(!value_matches_type(&parse_value("1px 2 3"), ValueType::Vector));
    }

    // endregion: multi-value
}
