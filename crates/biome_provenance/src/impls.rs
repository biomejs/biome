//! Implementations of ProvenanceTrackable for primitive types and collections
//!
//! This module provides manual implementations similar to how biome_deserialize
//! implements Deserializable for built-in types.

use crate::{ProvenanceContext, ProvenanceSourceNode, ProvenanceTrackable, ProvenanceValue};
use std::collections::{BTreeSet, HashSet};
use std::hash::Hash;

// ============================================================================
// Primitive types
// ============================================================================

impl ProvenanceTrackable for String {
    fn from_source_with_provenance(
        source: &impl ProvenanceSourceNode,
        ctx: &mut ProvenanceContext,
    ) -> Option<Self> {
        match source.source_value()? {
            ProvenanceValue::String(s) => {
                ctx.capture(ProvenanceValue::String(s.clone()), source.range());
                Some(s)
            }
            _ => None,
        }
    }
}

impl ProvenanceTrackable for bool {
    fn from_source_with_provenance(
        source: &impl ProvenanceSourceNode,
        ctx: &mut ProvenanceContext,
    ) -> Option<Self> {
        match source.source_value()? {
            ProvenanceValue::Boolean(b) => {
                ctx.capture(ProvenanceValue::Boolean(b), source.range());
                Some(b)
            }
            _ => None,
        }
    }
}

// Macro to implement ProvenanceTrackable for numeric types
macro_rules! impl_numeric {
    ($($ty:ty),*) => {
        $(
            impl ProvenanceTrackable for $ty {
                fn from_source_with_provenance(
                    source: &impl ProvenanceSourceNode,
                    ctx: &mut ProvenanceContext,
                ) -> Option<Self> {
                    match source.source_value()? {
                        ProvenanceValue::Number(n) => {
                            ctx.capture(ProvenanceValue::Number(n.clone()), source.range());
                            n.parse().ok()
                        }
                        _ => None,
                    }
                }
            }
        )*
    };
}

impl_numeric!(u8, u16, u32, u64, u128, usize);
impl_numeric!(i8, i16, i32, i64, i128, isize);
impl_numeric!(f32, f64);

// ============================================================================
// Collections
// ============================================================================

impl<T: ProvenanceTrackable> ProvenanceTrackable for Option<T> {
    fn from_source_with_provenance(
        source: &impl ProvenanceSourceNode,
        ctx: &mut ProvenanceContext,
    ) -> Option<Self> {
        // Option fails if the value is present but invalid
        // If null, return Some(None)
        // If valid value, return Some(Some(value))
        match source.source_value()? {
            ProvenanceValue::Null => {
                ctx.capture(ProvenanceValue::Null, source.range());
                Some(None)
            }
            _ => {
                // Try to deserialize as T
                T::from_source_with_provenance(source, ctx).map(Some)
            }
        }
    }
}

impl<T: ProvenanceTrackable> ProvenanceTrackable for Vec<T> {
    fn from_source_with_provenance(
        source: &impl ProvenanceSourceNode,
        ctx: &mut ProvenanceContext,
    ) -> Option<Self> {
        if !source.is_array() {
            return None;
        }

        let mut result = Vec::new();
        let mut all_succeeded = true;

        source.traverse_array(&mut |index, element| {
            ctx.push_index(index);
            match T::from_source_with_provenance(element, ctx) {
                Some(value) => result.push(value),
                None => all_succeeded = false,
            }
            ctx.pop();
        });

        if all_succeeded {
            Some(result)
        } else {
            // If any element failed, the whole array fails
            None
        }
    }
}

impl<T: ProvenanceTrackable> ProvenanceTrackable for Box<T> {
    fn from_source_with_provenance(
        source: &impl ProvenanceSourceNode,
        ctx: &mut ProvenanceContext,
    ) -> Option<Self> {
        T::from_source_with_provenance(source, ctx).map(Box::new)
    }
}

impl<T: ProvenanceTrackable + Eq + Hash> ProvenanceTrackable for HashSet<T> {
    fn from_source_with_provenance(
        source: &impl ProvenanceSourceNode,
        ctx: &mut ProvenanceContext,
    ) -> Option<Self> {
        if !source.is_array() {
            return None;
        }

        let mut result = HashSet::new();
        let mut all_succeeded = true;

        source.traverse_array(&mut |index, element| {
            ctx.push_index(index);
            match T::from_source_with_provenance(element, ctx) {
                Some(value) => {
                    result.insert(value);
                }
                None => all_succeeded = false,
            }
            ctx.pop();
        });

        if all_succeeded { Some(result) } else { None }
    }
}

impl<T: ProvenanceTrackable + Ord> ProvenanceTrackable for BTreeSet<T> {
    fn from_source_with_provenance(
        source: &impl ProvenanceSourceNode,
        ctx: &mut ProvenanceContext,
    ) -> Option<Self> {
        if !source.is_array() {
            return None;
        }

        let mut result = BTreeSet::new();
        let mut all_succeeded = true;

        source.traverse_array(&mut |index, element| {
            ctx.push_index(index);
            match T::from_source_with_provenance(element, ctx) {
                Some(value) => {
                    result.insert(value);
                }
                None => all_succeeded = false,
            }
            ctx.pop();
        });

        if all_succeeded { Some(result) } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_json_parser::{JsonParserOptions, parse_json};
    use camino::Utf8PathBuf;

    fn parse_and_extract<T: ProvenanceTrackable>(
        json: &str,
    ) -> Option<(T, Vec<crate::ProvenanceEntry>)> {
        let parsed = parse_json(json, JsonParserOptions::default());
        let root = parsed.tree();
        let value = root.value().ok()?;

        let mut ctx = ProvenanceContext::new(Utf8PathBuf::from("test.json"), 0);
        let result = T::from_source_with_provenance(&value, &mut ctx)?;
        let entries = ctx.into_entries();

        Some((result, entries))
    }

    #[test]
    fn test_string() {
        let (value, entries) = parse_and_extract::<String>(r#""hello""#).unwrap();
        assert_eq!(value, "hello");
        assert_eq!(entries.len(), 1);
        assert_eq!(
            entries[0].source_value,
            ProvenanceValue::String("hello".to_string())
        );
    }

    #[test]
    fn test_bool_true() {
        let (value, entries) = parse_and_extract::<bool>(r#"true"#).unwrap();
        assert!(value);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].source_value, ProvenanceValue::Boolean(true));
    }

    #[test]
    fn test_bool_false() {
        let (value, entries) = parse_and_extract::<bool>(r#"false"#).unwrap();
        assert!(!value);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].source_value, ProvenanceValue::Boolean(false));
    }

    #[test]
    fn test_u32() {
        let (value, entries) = parse_and_extract::<u32>(r#"42"#).unwrap();
        assert_eq!(value, 42);
        assert_eq!(entries.len(), 1);
        assert_eq!(
            entries[0].source_value,
            ProvenanceValue::Number("42".to_string())
        );
    }

    #[test]
    fn test_f64() {
        let (value, entries) = parse_and_extract::<f64>(r#"3.14"#).unwrap();
        assert!((value - 3.14).abs() < 0.001);
        assert_eq!(entries.len(), 1);
        assert_eq!(
            entries[0].source_value,
            ProvenanceValue::Number("3.14".to_string())
        );
    }

    #[test]
    fn test_option_some() {
        let (value, entries) = parse_and_extract::<Option<String>>(r#""hello""#).unwrap();
        assert_eq!(value, Some("hello".to_string()));
        assert_eq!(entries.len(), 1);
        assert_eq!(
            entries[0].source_value,
            ProvenanceValue::String("hello".to_string())
        );
    }

    #[test]
    fn test_option_null() {
        let (value, entries) = parse_and_extract::<Option<String>>(r#"null"#).unwrap();
        assert_eq!(value, None);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].source_value, ProvenanceValue::Null);
    }

    #[test]
    fn test_vec_string() {
        let (value, entries) =
            parse_and_extract::<Vec<String>>(r#"["apple", "banana", "cherry"]"#).unwrap();
        assert_eq!(value, vec!["apple", "banana", "cherry"]);
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].field_path, "[0]");
        assert_eq!(
            entries[0].source_value,
            ProvenanceValue::String("apple".to_string())
        );
        assert_eq!(entries[1].field_path, "[1]");
        assert_eq!(entries[2].field_path, "[2]");
    }

    #[test]
    fn test_vec_u32() {
        let (value, entries) = parse_and_extract::<Vec<u32>>(r#"[1, 2, 3]"#).unwrap();
        assert_eq!(value, vec![1, 2, 3]);
        assert_eq!(entries.len(), 3);
        assert_eq!(
            entries[0].source_value,
            ProvenanceValue::Number("1".to_string())
        );
        assert_eq!(
            entries[1].source_value,
            ProvenanceValue::Number("2".to_string())
        );
        assert_eq!(
            entries[2].source_value,
            ProvenanceValue::Number("3".to_string())
        );
    }

    #[test]
    fn test_vec_fails_on_invalid_element() {
        // Array with one invalid element should fail
        let result = parse_and_extract::<Vec<u32>>(r#"[1, "invalid", 3]"#);
        assert!(result.is_none());
    }

    #[test]
    fn test_box() {
        let (value, entries) = parse_and_extract::<Box<String>>(r#""boxed""#).unwrap();
        assert_eq!(*value, "boxed");
        assert_eq!(entries.len(), 1);
    }
}
