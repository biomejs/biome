//! Core sorting logic for Tailwind CSS utility classes.
//!
//! This module is language-agnostic. It takes a plain `&str` class string and returns the sorted
//! result. Language-specific concerns (e.g., JS template literal boundaries, concatenation
//! expressions) are handled by the caller via `SortContext`.

use std::cmp::Ordering;

use crate::{
    class_info::{ClassInfo, get_class_info},
    sort_config::SortConfig,
};

impl ClassInfo {
    /// Compare based on the existence of variants. Classes with variants go last.
    /// Returns `None` if both or none of the classes has variants.
    fn cmp_variants_weight_existence(&self, other: &Self) -> Option<Ordering> {
        match (&self.variant_weight, &other.variant_weight) {
            (Some(_), Some(_)) => None,
            (Some(_), _) => Some(Ordering::Greater),
            (_, Some(_)) => Some(Ordering::Less),
            (None, None) => None,
        }
    }

    /// Compare based on layer indexes. Classes with lower indexes go first.
    /// Returns `None` if the indexes are equal.
    fn cmp_layers(&self, other: &Self) -> Option<Ordering> {
        let result = self.layer_index.cmp(&other.layer_index);
        if result != Ordering::Equal {
            return Some(result);
        }
        None
    }

    /// Compare based on variants weight. Classes with lower weight go first.
    /// First compare variants weight length. Only if their equal compare their actual weight.
    /// Returns `None` if they have the same weight.
    fn cmp_variants_weight(&self, other: &Self) -> Option<Ordering> {
        let current_weight = self.variant_weight.as_ref()?;
        let other_weight = other.variant_weight.as_ref()?;

        let mut result = current_weight.len().cmp(&other_weight.len());
        if result == Ordering::Equal {
            result = current_weight.cmp(other_weight);
        }

        if result != Ordering::Equal {
            return Some(result);
        }
        None
    }

    /// Compare based on the existence of arbitrary variants. Classes with arbitrary variants go last.
    /// Returns `None` if both or none of the classes has arbitrary variants.
    fn cmp_arbitrary_variants_existence(&self, other: &Self) -> Option<Ordering> {
        match (&self.arbitrary_variants, &other.arbitrary_variants) {
            (Some(_), Some(_)) => None,
            (Some(_), _) => Some(Ordering::Greater),
            (_, Some(_)) => Some(Ordering::Less),
            (None, None) => None,
        }
    }

    /// Compare arbitrary variants based on their length and then lexicographically
    fn cmp_arbitrary_variants(&self, other: &Self) -> Option<Ordering> {
        let a = self.arbitrary_variants.as_ref()?;
        let b = other.arbitrary_variants.as_ref()?;

        let mut result = a.len().cmp(&b.len());
        if result == Ordering::Equal {
            result = a.cmp(b);
        }

        if result != Ordering::Equal {
            return Some(result);
        }
        None
    }

    /// Compare based on utility index. Classes with lower indexes go first.
    /// Returns `None` if the indexes are equal.
    fn cmp_utilities(&self, other: &Self) -> Option<Ordering> {
        let result = self.utility_index.cmp(&other.utility_index);
        if result != Ordering::Equal {
            return Some(result);
        }
        None
    }
}

// TODO: implement through Ord/PartialOrd trait.

// See: https://github.com/tailwindlabs/tailwindcss/blob/970f2ca704dda95cf328addfe67b81d6679c8755/src/lib/offsets.js#L206
// This comparison function follows a very similar logic to the one in Tailwind CSS, with some
// simplifications and necessary differences.
fn compare_classes(a: &ClassInfo, b: &ClassInfo) -> Ordering {
    // Classes with arbitrary variants go last
    if let Some(has_arbitrary_variants) = a.cmp_arbitrary_variants_existence(b) {
        return has_arbitrary_variants;
    }

    // Compare arbitrary variants
    if let Some(arbitrary_variants_order) = a.cmp_arbitrary_variants(b) {
        return arbitrary_variants_order;
    }

    // Classes with variants go last.
    if let Some(has_variants_order) = a.cmp_variants_weight_existence(b) {
        return has_variants_order;
    }

    // Compare utility layers.
    if let Some(layers_order) = a.cmp_layers(b) {
        return layers_order;
    }

    // TODO: sort screens at this point.

    // Compare variant weights.
    if let Some(variants_weight_order) = a.cmp_variants_weight(b) {
        return variants_weight_order;
    }

    // Compare utility indexes.
    if let Some(utilities_order) = a.cmp_utilities(b) {
        return utilities_order;
    }

    Ordering::Equal
}

/// Language-agnostic context for boundary handling.
///
/// This allows callers to control how the sort function treats the edges of the class string.
/// For example, in a JS template literal like `` `${var}px-2 m-4` ``, the first class `px-2`
/// may be glued to a variable and should be excluded from sorting.
#[derive(Debug, Clone, Copy, Default)]
pub struct SortContext {
    /// Skip the first class from sorting (e.g., it's glued to a preceding expression).
    pub ignore_prefix: bool,
    /// Skip the last class from sorting (e.g., it's glued to a following expression).
    pub ignore_suffix: bool,
    /// Keep a leading space in the result (e.g., to maintain a variable boundary).
    pub keep_leading_space: bool,
    /// Keep a trailing space in the result (e.g., to maintain a variable boundary).
    pub keep_trailing_space: bool,
}

/// Sort the given class string according to the given sort config.
///
/// This is the main entry point for sorting. It is language-agnostic — callers provide
/// a plain `&str` and an optional `SortContext` for edge handling.
///
/// Returns the sorted class string, or an empty string if the input is empty/whitespace-only.
pub fn sort_class_name(
    class_name: &str,
    sort_config: &SortConfig,
    sort_context: &Option<SortContext>,
) -> String {
    let (ignore_prefix, ignore_suffix) = sort_context
        .as_ref()
        .map_or((false, false), |ctx| (ctx.ignore_prefix, ctx.ignore_suffix));

    // Obtain classes by splitting the class string by whitespace.
    let mut classes_iter = class_name.split_whitespace();
    let class_str_prefix = if ignore_prefix {
        classes_iter.next()
    } else {
        None
    };
    let class_str_suffix = if ignore_suffix {
        classes_iter.next_back()
    } else {
        None
    };

    // Collect the remaining classes into a vector if needed.
    let classes: Vec<&str> = classes_iter.collect();

    // Separate custom classes from recognized classes, and compute the recognized classes' info.
    // Custom classes always go first, in the order that they appear in.
    let mut sorted_classes = Vec::new();
    let mut classes_info = Vec::new();
    for class in classes {
        match get_class_info(class, sort_config) {
            Some(class_info) => {
                classes_info.push(class_info);
            }
            None => {
                sorted_classes.push(class);
            }
        }
    }

    // TODO: make this the last step of compare instead?

    // Pre-sort the recognized classes lexico-graphically.
    classes_info.sort_unstable_by(|a, b| a.text.cmp(&b.text));

    // Sort recognized classes using the compare function. Needs to be a stable sort to
    // preserve the lexico-graphical order as a fallback.
    classes_info.sort_by(compare_classes);

    // Add the sorted classes to the result.
    sorted_classes.extend(
        classes_info
            .iter()
            .map(|class_info| class_info.text.as_ref()),
    );

    // Add the first class back if it was ignored.
    if let Some(class_str_prefix) = class_str_prefix {
        sorted_classes.insert(0, class_str_prefix);
    }

    // Add the last class back if it was ignored.
    if let Some(class_str_suffix) = class_str_suffix {
        sorted_classes.push(class_str_suffix);
    }

    let mut result = sorted_classes.join(" ");

    // Edge space handling
    if let Some(ctx) = sort_context {
        if ctx.keep_leading_space {
            result.insert(0, ' ');
        }
        if ctx.keep_trailing_space {
            result.push(' ');
        }
    }

    result
}

/// Get the byte offsets of the actual sortable range within the class string.
///
/// When some classes are ignored (prefix/suffix), this returns the byte range
/// that excludes those classes, useful for diagnostic highlighting.
pub fn get_sort_class_name_range(
    class_name: &str,
    sort_context: &Option<SortContext>,
) -> Option<(usize, usize)> {
    let mut class_iter = class_name.split_whitespace();
    let first_class_len = class_iter.next().map_or(0, |s| s.len());
    let last_class_len = class_iter.next_back().map_or(0, |s| s.len());

    let (ignore_prefix, ignore_suffix) = sort_context
        .as_ref()
        .map_or((false, false), |ctx| (ctx.ignore_prefix, ctx.ignore_suffix));
    let offset_prefix = if ignore_prefix { first_class_len } else { 0 };
    let offset_suffix = if ignore_suffix { last_class_len } else { 0 };

    let start = offset_prefix;
    let end = class_name.len().saturating_sub(offset_suffix);

    if end <= start {
        return None;
    }

    Some((start, end))
}
