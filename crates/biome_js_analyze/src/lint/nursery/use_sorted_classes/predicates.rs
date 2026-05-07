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

use std::sync::LazyLock;

use phf::{phf_set, Set};
use regex::Regex;
use smallvec::SmallVec;

// ── shared regex sources ─────────────────────────────────────────

/// `[+-]?\d*\.?\d+(?:[eE][+-]?\d+)?` — used as the building block of
/// every numeric pattern below.
const HAS_NUMBER: &str = r"[+-]?\d*\.?\d+(?:[eE][+-]?\d+)?";

const LENGTH_UNITS: &[&str] = &[
    "cm", "mm", "Q", "in", "pc", "pt", "px", "em", "ex", "ch", "rem", "lh", "rlh", "vw", "vh",
    "vmin", "vmax", "vb", "vi", "svw", "svh", "lvw", "lvh", "dvw", "dvh", "cqw", "cqh", "cqi",
    "cqb", "cqmin", "cqmax",
];

const ANGLE_UNITS: &[&str] = &["deg", "rad", "grad", "turn"];

static IS_NUMBER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r"^{HAS_NUMBER}$")).unwrap());

static IS_PERCENTAGE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r"^{HAS_NUMBER}%$")).unwrap());

static IS_FRACTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(r"^{HAS_NUMBER}\s*/\s*{HAS_NUMBER}$")).unwrap()
});

static IS_LENGTH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(r"^{HAS_NUMBER}({})$", LENGTH_UNITS.join("|"))).unwrap()
});

static IS_ANGLE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(r"^{HAS_NUMBER}({})$", ANGLE_UNITS.join("|"))).unwrap()
});

static IS_VECTOR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(r"^{HAS_NUMBER} +{HAS_NUMBER} +{HAS_NUMBER}$")).unwrap()
});

static IS_GRADIENT_FN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(repeating-)?(conic|linear|radial)-gradient\(").unwrap());

static IS_IMAGE_FN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:element|image|cross-fade|image-set)\(").unwrap());

static IS_COLOR_FN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^(rgba?|hsla?|hwb|color|(ok)?(lab|lch)|light-dark|color-mix)\(").unwrap()
});

// ── shared helpers ───────────────────────────────────────────────

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

// ── primitive predicates ─────────────────────────────────────────

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L63>
/// (`isUrl`).
pub fn is_url(value: &str) -> bool {
    value.starts_with("url(") && value.ends_with(')')
}

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L173>
/// (`isNumber`).
pub fn is_number(value: &str) -> bool {
    IS_NUMBER.is_match(value) || has_math_fn(value)
}

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L181>
/// (`isPercentage`).
pub fn is_percentage(value: &str) -> bool {
    IS_PERCENTAGE.is_match(value) || has_math_fn(value)
}

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L189>
/// (`isFraction`; mapped to the `ratio` data-type by Tailwind).
pub fn is_ratio(value: &str) -> bool {
    IS_FRACTION.is_match(value) || has_math_fn(value)
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
    IS_LENGTH.is_match(value) || has_math_fn(value)
}

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L324>
/// (`isAngle`).
pub fn is_angle(value: &str) -> bool {
    IS_ANGLE.is_match(value)
}

/// Mirrors <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts#L339>
/// (`isVector`).
pub fn is_vector(value: &str) -> bool {
    IS_VECTOR.is_match(value)
}

// ── color ────────────────────────────────────────────────────────

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
    if IS_COLOR_FN.is_match(value) {
        return true;
    }
    named_color_match(value)
}

// ── line-width / position / bg-size / image ──────────────────────

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
        if IS_GRADIENT_FN.is_match(part) || IS_IMAGE_FN.is_match(part) {
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

// ── tests ────────────────────────────────────────────────────────

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
