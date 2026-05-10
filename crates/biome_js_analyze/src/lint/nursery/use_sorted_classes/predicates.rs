//! CSS value-type predicates for `NamedTyped` branches.
//!
//! Hand-ported from Tailwind v4.2.2:
//! - <https://github.com/tailwindlabs/tailwindcss/blob/v4.2.2/packages/tailwindcss/src/utils/infer-data-type.ts>
//!
//! Only `is_number` / `is_percentage` / `is_ratio` are kept — those are
//! the value types `NamedTyped` carries in the preset. The other types
//! (`Color`, `Length`, `Angle`, `Image`, ...) are reachable only through
//! `ArbitraryTyped`, which `sort_v4` does not yet route through, and
//! once it does the parser nodes from #10299 will replace any text
//! scanning we'd add here.

/// `[+-]?\d*\.?\d+(?:[eE][+-]?\d+)?` anchored over the full string.
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

pub fn is_number(value: &str) -> bool {
    is_number_str(value)
}

pub fn is_percentage(value: &str) -> bool {
    value
        .strip_suffix('%')
        .is_some_and(|prefix| !prefix.is_empty() && is_number_str(prefix))
}

pub fn is_ratio(value: &str) -> bool {
    let Some(slash) = value.find('/') else {
        return false;
    };
    // `^<num>\s*/\s*<num>$` — whitespace allowed only around the slash.
    let left = value[..slash].trim_end();
    let right = value[slash + 1..].trim_start();
    is_number_str(left) && is_number_str(right)
}

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
        assert!(!is_number(""));
        assert!(!is_number("abc"));
        assert!(!is_number("1px"));
        assert!(!is_number("1."));
        assert!(!is_number("1e"));
    }

    #[test]
    fn percentage() {
        assert!(is_percentage("0%"));
        assert!(is_percentage("50%"));
        assert!(is_percentage("100%"));
        assert!(is_percentage("-25%"));
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
}
