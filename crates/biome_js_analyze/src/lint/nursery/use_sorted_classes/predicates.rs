//! CSS value-type predicates.
//!
//! Hand-ported from Tailwind v4.2.2:
//! - <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts>
//! - <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/is-color.ts>
//! - <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/math-operators.ts>
//! - <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/segment.ts>
//!
//! These are the 17 predicates that `ValueType::matches` dispatches to
//! from `tailwind_preset_v4.rs`.

use phf::{phf_set, Set};
use smallvec::SmallVec;

// region: shared scanners

const LENGTH_UNITS: &[&str] = &[
    "cm", "mm", "Q", "in", "pc", "pt", "px", "em", "ex", "ch", "rem", "lh", "rlh", "vw", "vh",
    "vmin", "vmax", "vb", "vi", "svw", "svh", "lvw", "lvh", "dvw", "dvh", "cqw", "cqh", "cqi",
    "cqb", "cqmin", "cqmax",
];

const ANGLE_UNITS: &[&str] = &["deg", "rad", "grad", "turn"];

const GRADIENT_FN_PREFIXES: &[&str] = &[
    "linear-gradient(",
    "radial-gradient(",
    "conic-gradient(",
    "repeating-linear-gradient(",
    "repeating-radial-gradient(",
    "repeating-conic-gradient(",
];

const IMAGE_FN_PREFIXES: &[&str] = &["element(", "image(", "image-set(", "cross-fade("];

/// Case-insensitive prefixes — `is_color` matches CSS color functions
/// regardless of letter casing (`rgb(...)`, `RGB(...)`, etc.).
const COLOR_FN_PREFIXES: &[&str] = &[
    "rgb(",
    "rgba(",
    "hsl(",
    "hsla(",
    "hwb(",
    "color(",
    "lab(",
    "lch(",
    "oklab(",
    "oklch(",
    "light-dark(",
    "color-mix(",
];

/// Mirrors Tailwind's `HAS_NUMBER` pattern anchored over the full string:
/// `[+-]?\d*\.?\d+(?:[eE][+-]?\d+)?`. The integer / fractional pieces are
/// each optional individually, but at least one digit must appear before
/// or after the optional dot.
fn is_number_str(s: &str) -> bool {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut i = 0;
    if i < n && matches!(bytes[i], b'+' | b'-') {
        i += 1;
    }
    let int_start = i;
    while i < n && bytes[i].is_ascii_digit() {
        i += 1;
    }
    let int_digits = i - int_start;
    let has_dot = i < n && bytes[i] == b'.';
    if has_dot {
        i += 1;
    }
    let frac_start = i;
    while i < n && bytes[i].is_ascii_digit() {
        i += 1;
    }
    let frac_digits = i - frac_start;
    if has_dot {
        if frac_digits == 0 {
            return false;
        }
    } else if int_digits == 0 {
        return false;
    }
    if i < n && matches!(bytes[i], b'e' | b'E') {
        i += 1;
        if i < n && matches!(bytes[i], b'+' | b'-') {
            i += 1;
        }
        let exp_start = i;
        while i < n && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i == exp_start {
            return false;
        }
    }
    i == n
}

/// `<number><unit>` where `<unit>` is one of the supplied suffixes.
fn is_number_with_unit(value: &str, units: &[&str]) -> bool {
    units.iter().any(|unit| {
        value
            .strip_suffix(unit)
            .is_some_and(|prefix| !prefix.is_empty() && is_number_str(prefix))
    })
}

fn starts_with_any(value: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|prefix| value.starts_with(prefix))
}

fn starts_with_any_ascii_ci(value: &str, prefixes: &[&str]) -> bool {
    let v_bytes = value.as_bytes();
    prefixes.iter().any(|prefix| {
        let p_bytes = prefix.as_bytes();
        v_bytes.len() >= p_bytes.len() && v_bytes[..p_bytes.len()].eq_ignore_ascii_case(p_bytes)
    })
}

// endregion: shared scanners

// region: shared helpers

const MATH_FUNCTIONS: &[&str] = &[
    "calc(", "min(", "max(", "clamp(", "mod(", "rem(", "sin(", "cos(", "tan(", "asin(", "acos(",
    "atan(", "atan2(", "pow(", "sqrt(", "hypot(", "log(", "exp(", "round(",
];

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/math-operators.ts#L41>
/// (`hasMathFn`).
fn has_math_fn(input: &str) -> bool {
    if !input.contains('(') {
        return false;
    }
    MATH_FUNCTIONS.iter().any(|f| input.contains(f))
}

/// Iterator that splits `input` on `sep` only at nesting depth zero,
/// respecting parens / brackets / braces, escapes, and quoted strings.
///
/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/segment.ts#L29>
/// (`segment`).
struct Segments<'a> {
    input: &'a str,
    bytes: &'a [u8],
    sep: u8,
    pos: usize,
    last: usize,
    stack: SmallVec<[u8; 8]>,
    done: bool,
}

impl<'a> Iterator for Segments<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.done {
            return None;
        }
        while self.pos < self.bytes.len() {
            let c = self.bytes[self.pos];
            if self.stack.is_empty() && c == self.sep {
                let part = &self.input[self.last..self.pos];
                self.pos += 1;
                self.last = self.pos;
                return Some(part);
            }
            match c {
                b'\\' => {
                    self.pos = (self.pos + 2).min(self.bytes.len());
                    continue;
                }
                b'\'' | b'"' => {
                    let quote = c;
                    self.pos += 1;
                    while self.pos < self.bytes.len() {
                        if self.bytes[self.pos] == b'\\' {
                            self.pos = (self.pos + 2).min(self.bytes.len());
                            continue;
                        }
                        if self.bytes[self.pos] == quote {
                            self.pos += 1;
                            break;
                        }
                        self.pos += 1;
                    }
                    continue;
                }
                b'(' => self.stack.push(b')'),
                b'[' => self.stack.push(b']'),
                b'{' => self.stack.push(b'}'),
                b')' | b']' | b'}' if self.stack.last() == Some(&c) => {
                    self.stack.pop();
                }
                _ => {}
            }
            self.pos += 1;
        }
        self.done = true;
        Some(&self.input[self.last..])
    }
}

fn segment<'a>(input: &'a str, sep: u8) -> Segments<'a> {
    Segments {
        input,
        bytes: input.as_bytes(),
        sep,
        pos: 0,
        last: 0,
        stack: SmallVec::new(),
        done: false,
    }
}

// endregion: shared helpers

// region: primitive predicates

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L63>
/// (`isUrl`).
pub fn is_url(value: &str) -> bool {
    value.starts_with("url(") && value.ends_with(')')
}

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L173>
/// (`isNumber`).
pub fn is_number(value: &str) -> bool {
    is_number_str(value) || has_math_fn(value)
}

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L181>
/// (`isPercentage`).
pub fn is_percentage(value: &str) -> bool {
    let matches_pattern = value
        .strip_suffix('%')
        .is_some_and(|prefix| !prefix.is_empty() && is_number_str(prefix));
    matches_pattern || has_math_fn(value)
}

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L189>
/// (`isFraction`; mapped to the `ratio` data-type by Tailwind).
pub fn is_ratio(value: &str) -> bool {
    if let Some(slash) = value.find('/') {
        // `^<num>\s*/\s*<num>$` — whitespace allowed only around the slash,
        // not at the outer ends.
        let left = value[..slash].trim_end();
        let right = value[slash + 1..].trim_start();
        if is_number_str(left) && is_number_str(right) {
            return true;
        }
    }
    has_math_fn(value)
}

/// `Number.isInteger(num) && num >= 0 && String(num) === String(value)`.
/// Equivalent to: optional `0` or a non-zero digit followed by digits;
/// no sign, no leading zeros, no decimals, no exponent.
///
/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L346>
/// (`isPositiveInteger`).
pub fn is_integer(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    if !bytes.iter().all(|b| b.is_ascii_digit()) {
        return false;
    }
    if bytes.len() > 1 && bytes[0] == b'0' {
        return false;
    }
    true
}

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L236>
/// (`isLength`).
pub fn is_length(value: &str) -> bool {
    is_number_with_unit(value, LENGTH_UNITS) || has_math_fn(value)
}

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L324>
/// (`isAngle`).
pub fn is_angle(value: &str) -> bool {
    is_number_with_unit(value, ANGLE_UNITS)
}

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L339>
/// (`isVector`).
pub fn is_vector(value: &str) -> bool {
    // `^<num> +<num> +<num>$` — three numbers separated by 1+ spaces, no
    // leading/trailing whitespace.
    let bytes = value.as_bytes();
    let n = bytes.len();
    if n == 0 {
        return false;
    }
    let mut numbers = 0;
    let mut i = 0;
    loop {
        let start = i;
        while i < n && bytes[i] != b' ' {
            i += 1;
        }
        // Empty token guards leading whitespace and double spaces.
        if i == start {
            return false;
        }
        if !is_number_str(&value[start..i]) {
            return false;
        }
        numbers += 1;
        if i == n {
            break;
        }
        while i < n && bytes[i] == b' ' {
            i += 1;
        }
        // Trailing whitespace not allowed by the original anchor.
        if i == n {
            return false;
        }
    }
    numbers == 3
}

// endregion: primitive predicates

// region: color

/// Longest entry in `NAMED_COLORS` is `"lightgoldenrodyellow"` (20 chars).
/// Round up to a small power of two to keep the stack buffer cheap.
const NAMED_COLOR_BUF_LEN: usize = 32;

static NAMED_COLORS: Set<&'static str> = phf_set! {
    "accentcolor",
    "accentcolortext",
    "activetext",
    "aliceblue",
    "antiquewhite",
    "aqua",
    "aquamarine",
    "azure",
    "beige",
    "bisque",
    "black",
    "blanchedalmond",
    "blue",
    "blueviolet",
    "brown",
    "burlywood",
    "buttonborder",
    "buttonface",
    "buttontext",
    "cadetblue",
    "canvas",
    "canvastext",
    "chartreuse",
    "chocolate",
    "coral",
    "cornflowerblue",
    "cornsilk",
    "crimson",
    "currentcolor",
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
    "field",
    "fieldtext",
    "firebrick",
    "floralwhite",
    "forestgreen",
    "fuchsia",
    "gainsboro",
    "ghostwhite",
    "gold",
    "goldenrod",
    "gray",
    "graytext",
    "green",
    "greenyellow",
    "grey",
    "highlight",
    "highlighttext",
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
    "lime",
    "limegreen",
    "linen",
    "linktext",
    "magenta",
    "mark",
    "marktext",
    "maroon",
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
    "navy",
    "oldlace",
    "olive",
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
    "purple",
    "rebeccapurple",
    "red",
    "rosybrown",
    "royalblue",
    "saddlebrown",
    "salmon",
    "sandybrown",
    "seagreen",
    "seashell",
    "selecteditem",
    "selecteditemtext",
    "sienna",
    "silver",
    "skyblue",
    "slateblue",
    "slategray",
    "slategrey",
    "snow",
    "springgreen",
    "steelblue",
    "tan",
    "teal",
    "thistle",
    "tomato",
    "transparent",
    "turquoise",
    "violet",
    "visitedtext",
    "wheat",
    "white",
    "whitesmoke",
    "yellow",
    "yellowgreen",
};

/// Case-insensitive named-color lookup that avoids a heap allocation by
/// lowercasing into a fixed-size stack buffer. Inputs that are
/// obviously too long (longer than the longest named color) short-
/// circuit immediately.
fn named_color_match(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.is_empty() || bytes.len() > NAMED_COLOR_BUF_LEN {
        return false;
    }
    let mut buf = [0u8; NAMED_COLOR_BUF_LEN];
    for (i, &b) in bytes.iter().enumerate() {
        buf[i] = b.to_ascii_lowercase();
    }
    // `to_ascii_lowercase` only changes ASCII A-Z bytes; UTF-8
    // continuation bytes pass through untouched, so the byte slice
    // remains valid UTF-8 (the input was a `&str`).
    let lowered = std::str::from_utf8(&buf[..bytes.len()])
        .expect("ascii-lowercased &str remains valid UTF-8");
    NAMED_COLORS.contains(lowered)
}

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/is-color.ts#L200>
/// (`isColor`).
pub fn is_color(value: &str) -> bool {
    if value.as_bytes().first() == Some(&b'#') {
        return true;
    }
    if starts_with_any_ascii_ci(value, COLOR_FN_PREFIXES) {
        return true;
    }
    named_color_match(value)
}

// endregion: color

// region: line-width / position / bg-size / image

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L69>
/// (`isLineWidth`).
pub fn is_line_width(value: &str) -> bool {
    segment(value, b' ').all(|part| {
        is_length(part) || is_number(part) || part == "thin" || part == "medium" || part == "thick"
    })
}

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L242>
/// (`isBackgroundPosition`).
pub fn is_position(value: &str) -> bool {
    let mut count = 0usize;
    for part in segment(value, b' ') {
        match part {
            "center" | "top" | "right" | "bottom" | "left" => {
                count += 1;
                continue;
            }
            _ => {}
        }
        if part.starts_with("var(") {
            continue;
        }
        if is_length(part) || is_percentage(part) {
            count += 1;
            continue;
        }
        return false;
    }
    count > 0
}

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L285>
/// (`isBackgroundSize`).
pub fn is_bg_size(value: &str) -> bool {
    let mut count = 0usize;
    for size in segment(value, b',') {
        let size = size.trim();
        if size == "cover" || size == "contain" {
            count += 1;
            continue;
        }
        // bg-size accepts at most two space-separated tokens; collect
        // into stack-allocated SmallVec to count without heap alloc.
        let values: SmallVec<[&str; 2]> = segment(size, b' ').collect();
        if values.len() != 1 && values.len() != 2 {
            return false;
        }
        // Mirrors Tailwind: an invalid token in a multi-segment list is
        // silently skipped instead of failing the whole value (so e.g.
        // `cover, foo` returns true). Don't add `return false` here —
        // diverging would change predicate semantics versus Tailwind.
        if values
            .iter()
            .all(|v| *v == "auto" || is_length(v) || is_percentage(v))
        {
            count += 1;
        }
    }
    count > 0
}

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L85>
/// (`isImage`).
pub fn is_image(value: &str) -> bool {
    let mut count = 0usize;
    for part in segment(value, b',') {
        let part = part.trim();
        if part.starts_with("var(") {
            continue;
        }
        if is_url(part) {
            count += 1;
            continue;
        }
        if starts_with_any(part, GRADIENT_FN_PREFIXES)
            || starts_with_any(part, IMAGE_FN_PREFIXES)
        {
            count += 1;
            continue;
        }
        return false;
    }
    count > 0
}

// `is_family_name` and `is_generic_name` are intentionally not ported.
// Tailwind v4 only consults these types from the `font-` utility, which
// routes both of them — and any other unrecognized arbitrary value — to the
// same property (`font-family`). The codegen's branch-dedupe step therefore
// always collapses ArbitraryTyped(FamilyName)/ArbitraryTyped(GenericName)
// into the existing Arbitrary fallback, so a runtime predicate would never
// be called. Re-introduce both if a future utility disambiguates property
// by these types. Reference:
// <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L114>

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L148>
/// (`isAbsoluteSize`).
pub fn is_absolute_size(value: &str) -> bool {
    matches!(
        value,
        "xx-small"
            | "x-small"
            | "small"
            | "medium"
            | "large"
            | "x-large"
            | "xx-large"
            | "xxx-large"
    )
}

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L161>
/// (`isRelativeSize`).
pub fn is_relative_size(value: &str) -> bool {
    matches!(value, "larger" | "smaller")
}

// endregion: line-width / position / bg-size / image

// region: tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        assert!(is_number("0"));
        assert!(is_number("1"));
        assert!(is_number("1.5"));
        assert!(is_number("-2"));
        assert!(is_number("+3"));
        assert!(is_number("1e2"));
        assert!(is_number("1E-3"));
        assert!(is_number(".5"));
        assert!(is_number("calc(1 + 2)"));
        assert!(!is_number(""));
        assert!(!is_number("abc"));
        assert!(!is_number("1px"));
        assert!(!is_number("1."));
        assert!(!is_number("1e"));
    }

    #[test]
    fn integer() {
        assert!(is_integer("0"));
        assert!(is_integer("1"));
        assert!(is_integer("100"));
        assert!(!is_integer(""));
        assert!(!is_integer("-1"));
        assert!(!is_integer("+1"));
        assert!(!is_integer("1.0"));
        assert!(!is_integer("01"));
        assert!(!is_integer("1e2"));
    }

    #[test]
    fn percentage() {
        assert!(is_percentage("0%"));
        assert!(is_percentage("50%"));
        assert!(is_percentage("100%"));
        assert!(is_percentage("-25%"));
        assert!(is_percentage("calc(50% + 10px)"));
        assert!(!is_percentage("50"));
        assert!(!is_percentage("%"));
        assert!(!is_percentage("abc%"));
    }

    #[test]
    fn ratio() {
        assert!(is_ratio("16/9"));
        assert!(is_ratio("1/2"));
        assert!(is_ratio("1 / 2"));
        assert!(is_ratio("4/3"));
        assert!(!is_ratio("16"));
        assert!(!is_ratio("16/"));
        assert!(!is_ratio("/9"));
    }

    #[test]
    fn length() {
        assert!(is_length("10px"));
        assert!(is_length("1.5rem"));
        assert!(is_length("100vw"));
        assert!(is_length("2.5cqw"));
        assert!(is_length("calc(1px + 2px)"));
        assert!(!is_length("10"));
        assert!(!is_length("10%"));
        assert!(!is_length("10foo"));
    }

    #[test]
    fn angle() {
        assert!(is_angle("45deg"));
        assert!(is_angle("0.5turn"));
        assert!(is_angle("1rad"));
        assert!(is_angle("200grad"));
        assert!(!is_angle("45"));
        assert!(!is_angle("45px"));
    }

    #[test]
    fn url() {
        assert!(is_url("url(./img.png)"));
        assert!(is_url("url(\"a\")"));
        assert!(!is_url("url"));
        assert!(!is_url("url(abc"));
    }

    #[test]
    fn vector() {
        assert!(is_vector("1 0 0"));
        assert!(is_vector("0.5 0.5 1"));
        assert!(is_vector("-1 0 1"));
        assert!(!is_vector("1 0"));
        assert!(!is_vector("1 0 0 0"));
    }

    #[test]
    fn color() {
        assert!(is_color("#000"));
        assert!(is_color("#abcdef"));
        assert!(is_color("red"));
        assert!(is_color("REBECCAPURPLE"));
        assert!(is_color("rgb(0 0 0)"));
        assert!(is_color("hsl(0deg 0% 0%)"));
        assert!(is_color("oklch(60% 0.2 250)"));
        assert!(is_color("color-mix(in srgb, red, blue)"));
        assert!(!is_color("not-a-color"));
        assert!(!is_color(""));
        // long input — exceeds NAMED_COLOR_BUF_LEN, must short-circuit
        // without panic.
        assert!(!is_color(&"x".repeat(64)));
    }

    #[test]
    fn line_width() {
        assert!(is_line_width("thin"));
        assert!(is_line_width("medium"));
        assert!(is_line_width("thick"));
        assert!(is_line_width("2px"));
        assert!(is_line_width("1"));
        assert!(is_line_width("thin 2px"));
        assert!(!is_line_width("foo"));
    }

    #[test]
    fn position() {
        assert!(is_position("top"));
        assert!(is_position("center"));
        assert!(is_position("top left"));
        assert!(is_position("50% 50%"));
        assert!(is_position("10px 20px"));
        assert!(!is_position("foo"));
        assert!(!is_position(""));
    }

    #[test]
    fn bg_size() {
        assert!(is_bg_size("cover"));
        assert!(is_bg_size("contain"));
        assert!(is_bg_size("auto"));
        assert!(is_bg_size("100px 50px"));
        assert!(is_bg_size("auto auto"));
        assert!(is_bg_size("100% auto"));
        assert!(!is_bg_size("100px 50px 30px"));
        assert!(!is_bg_size("foo"));
    }

    #[test]
    fn image() {
        assert!(is_image("url(./a.png)"));
        assert!(is_image("linear-gradient(red, blue)"));
        assert!(is_image("radial-gradient(red, blue)"));
        assert!(is_image("repeating-conic-gradient(red, blue)"));
        assert!(is_image("image-set(url(a.png))"));
        assert!(!is_image("red"));
        assert!(!is_image(""));
    }

    #[test]
    fn sizes() {
        assert!(is_absolute_size("small"));
        assert!(is_absolute_size("xxx-large"));
        assert!(!is_absolute_size("medium-large"));
        assert!(is_relative_size("larger"));
        assert!(is_relative_size("smaller"));
        assert!(!is_relative_size("large"));
    }
}

// endregion: tests
