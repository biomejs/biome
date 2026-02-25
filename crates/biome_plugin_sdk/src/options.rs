//! Lightweight JSON option extractors for WASM plugins.
//!
//! Since `serde` is not available in the guest environment, these helpers
//! provide simple extraction of primitive values from a JSON options string.
//!
//! All functions take a JSON string (expected to be a top-level object) and
//! a key name, returning `None` if the key is not found or has an
//! incompatible type.
//!
//! # Example
//!
//! ```ignore
//! use biome_plugin_sdk::options;
//!
//! fn configure(options_json: String) {
//!     let pattern = options::get_string(&options_json, "pattern")
//!         .unwrap_or_else(|| "default".to_string());
//!     let max_depth = options::get_number(&options_json, "maxDepth")
//!         .unwrap_or(3.0);
//!     let strict = options::get_bool(&options_json, "strict")
//!         .unwrap_or(false);
//!     let prefixes = options::get_string_array(&options_json, "prefixes")
//!         .unwrap_or_default();
//! }
//! ```

/// Extract a string value for a given key from a JSON object.
///
/// Handles `{"key": "value"}` with proper escaped-quote handling.
/// Only `\"` and `\\` escapes are interpreted; other JSON escapes
/// (e.g. `\n`, `\uXXXX`) are passed through literally.
pub fn get_string(json: &str, key: &str) -> Option<String> {
    let value_start = find_value_start(json, key)?;
    let rest = &json[value_start..];
    let rest = rest.strip_prefix('"')?;
    parse_json_string(rest)
}

/// Extract a number value for a given key from a JSON object.
///
/// Returns the value as `f64`. Handles integers and floats, including
/// negative numbers and scientific notation.
pub fn get_number(json: &str, key: &str) -> Option<f64> {
    let value_start = find_value_start(json, key)?;
    let rest = &json[value_start..];
    let end = rest
        .find(|c: char| c == ',' || c == '}' || c == ']' || c.is_whitespace())
        .unwrap_or(rest.len());
    rest[..end].trim().parse::<f64>().ok()
}

/// Extract a boolean value for a given key from a JSON object.
pub fn get_bool(json: &str, key: &str) -> Option<bool> {
    let value_start = find_value_start(json, key)?;
    let rest = &json[value_start..];
    if rest.starts_with("true") {
        Some(true)
    } else if rest.starts_with("false") {
        Some(false)
    } else {
        None
    }
}

/// Extract a string array value for a given key from a JSON object.
///
/// Returns `None` if the key is not found. Returns an empty `Vec` if
/// the value is `[]`.
pub fn get_string_array(json: &str, key: &str) -> Option<Vec<String>> {
    let value_start = find_value_start(json, key)?;
    let rest = &json[value_start..];
    let rest = rest.strip_prefix('[')?;

    let mut result = Vec::new();
    let mut pos = 0;
    let bytes = rest.as_bytes();

    while pos < bytes.len() {
        match bytes[pos] {
            b'"' => {
                // Parse a string element.
                pos += 1;
                let s = parse_json_string(&rest[pos..])?;
                // Advance past the string content + closing quote.
                pos += count_json_string_bytes(&rest[pos..])? + 1;
                result.push(s);
            }
            b']' => return Some(result),
            b',' | b' ' | b'\t' | b'\n' | b'\r' => pos += 1,
            _ => return None,
        }
    }

    None
}

/// Locate the byte offset where the value for `key` starts in a JSON object.
///
/// Scans through the JSON, skipping over string values to avoid matching
/// the key inside a nested string value.
fn find_value_start(json: &str, key: &str) -> Option<usize> {
    let search = format!("\"{key}\"");
    let bytes = json.as_bytes();
    let search_bytes = search.as_bytes();

    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'"' {
            // Check if this position matches our key.
            if i + search_bytes.len() <= bytes.len()
                && &bytes[i..i + search_bytes.len()] == search_bytes
            {
                // Found the key — skip past `"key"`, then whitespace and `:`.
                let after_key = &json[i + search.len()..];
                let after_colon = after_key.trim_start().strip_prefix(':')?;
                let trimmed = after_colon.trim_start();
                let offset = json.len() - trimmed.len();
                return Some(offset);
            }

            // Skip this entire string (it's not our key, or it's a value).
            i += 1;
            while i < bytes.len() {
                if bytes[i] == b'\\' {
                    i += 2;
                } else if bytes[i] == b'"' {
                    i += 1;
                    break;
                } else {
                    i += 1;
                }
            }
            continue;
        }
        i += 1;
    }
    None
}

/// Parse a JSON string starting after the opening quote.
/// Returns the unescaped content.
fn parse_json_string(s: &str) -> Option<String> {
    let bytes = s.as_bytes();
    let mut result = String::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            match bytes[i + 1] {
                b'"' => result.push('"'),
                b'\\' => result.push('\\'),
                b'/' => result.push('/'),
                b'n' => result.push('\n'),
                b'r' => result.push('\r'),
                b't' => result.push('\t'),
                b'b' => result.push('\u{0008}'),
                b'f' => result.push('\u{000C}'),
                other => {
                    result.push('\\');
                    result.push(other as char);
                }
            }
            i += 2;
        } else if bytes[i] == b'"' {
            return Some(result);
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }
    None
}

/// Count the number of bytes in a JSON string body (excluding the closing quote).
fn count_json_string_bytes(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            i += 2;
        } else if bytes[i] == b'"' {
            return Some(i);
        } else {
            i += 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_string() {
        let json = r#"{"pattern": "^(is|has)[A-Z]", "other": "val"}"#;
        assert_eq!(
            get_string(json, "pattern"),
            Some("^(is|has)[A-Z]".to_string())
        );
        assert_eq!(get_string(json, "other"), Some("val".to_string()));
        assert_eq!(get_string(json, "missing"), None);
    }

    #[test]
    fn test_get_string_with_escapes() {
        let json = r#"{"msg": "hello \"world\""}"#;
        assert_eq!(get_string(json, "msg"), Some("hello \"world\"".to_string()));
    }

    #[test]
    fn test_get_number() {
        let json = r#"{"count": 42, "rate": 2.72, "neg": -5}"#;
        assert_eq!(get_number(json, "count"), Some(42.0));
        assert_eq!(get_number(json, "rate"), Some(2.72));
        assert_eq!(get_number(json, "neg"), Some(-5.0));
        assert_eq!(get_number(json, "missing"), None);
    }

    #[test]
    fn test_get_bool() {
        let json = r#"{"strict": true, "verbose": false}"#;
        assert_eq!(get_bool(json, "strict"), Some(true));
        assert_eq!(get_bool(json, "verbose"), Some(false));
        assert_eq!(get_bool(json, "missing"), None);
    }

    #[test]
    fn test_get_string_array() {
        let json = r#"{"prefixes": ["is", "has", "should"]}"#;
        assert_eq!(
            get_string_array(json, "prefixes"),
            Some(vec![
                "is".to_string(),
                "has".to_string(),
                "should".to_string()
            ])
        );
    }

    #[test]
    fn test_get_string_array_empty() {
        let json = r#"{"items": []}"#;
        assert_eq!(get_string_array(json, "items"), Some(vec![]));
    }

    #[test]
    fn test_get_string_array_missing() {
        let json = r#"{"other": "val"}"#;
        assert_eq!(get_string_array(json, "items"), None);
    }

    #[test]
    fn test_key_not_in_value() {
        // The key "a" appears inside a value string — should not match.
        let json = r#"{"b": "key a is here", "a": "correct"}"#;
        assert_eq!(get_string(json, "a"), Some("correct".to_string()));
    }

    #[test]
    fn test_whitespace_variations() {
        let json = r#"{  "key"  :  "value"  }"#;
        assert_eq!(get_string(json, "key"), Some("value".to_string()));
    }
}
