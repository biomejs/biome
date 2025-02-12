use biome_js_syntax::JsTemplateElement;
use biome_rowan::{AstNode, TextRange, TextSize, TokenText};
use std::cmp::Ordering;

use super::{
    any_class_string_like::AnyClassStringLike,
    class_info::{get_class_info, ClassInfo},
    sort_config::SortConfig,
};

impl ClassInfo {
    /// Compare based on the existence of variants. Classes with variants go last.
    /// Returns `None` if both or none of the classes has variants.
    fn cmp_variants_weight_existence(&self, other: &ClassInfo) -> Option<Ordering> {
        match (&self.variant_weight, &other.variant_weight) {
            (Some(_), Some(_)) => None,
            (Some(_), _) => Some(Ordering::Greater),
            (_, Some(_)) => Some(Ordering::Less),
            (None, None) => None,
        }
    }

    /// Compare based on layer indexes. Classes with lower indexes go first.
    /// Returns `None` if the indexes are equal.
    fn cmp_layers(&self, other: &ClassInfo) -> Option<Ordering> {
        let result = self.layer_index.cmp(&other.layer_index);
        if result != Ordering::Equal {
            return Some(result);
        }
        None
    }

    /// Compare based on variants weight. Classes with lower weight go first.
    /// First compare variants weight length. Only if their equal compare their actual weight.
    /// Returns `None` if they have the same weight.
    fn cmp_variants_weight(&self, other: &ClassInfo) -> Option<Ordering> {
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
    fn cmp_arbitrary_variants_existence(&self, other: &ClassInfo) -> Option<Ordering> {
        match (&self.arbitrary_variants, &other.arbitrary_variants) {
            (Some(_), Some(_)) => None,
            (Some(_), _) => Some(Ordering::Greater),
            (_, Some(_)) => Some(Ordering::Less),
            (None, None) => None,
        }
    }

    /// Compare arbitrary variants based on their length and then lexicographically
    fn cmp_arbitrary_variants(&self, other: &ClassInfo) -> Option<Ordering> {
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
    fn cmp_utilities(&self, other: &ClassInfo) -> Option<Ordering> {
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

/// Sort the given class string according to the given sort config.
/// ignore_prefix and ignore_postfix are used to ignore the first and last class respectively.
pub fn sort_class_name(
    class_name: &TokenText,
    sort_config: &SortConfig,
    ignore_prefix: bool,
    ignore_postfix: bool,
) -> String {
    // Obtain classes by splitting the class string by whitespace.
    let mut classes_iter = class_name.split_whitespace();
    let class_str_prefix = if ignore_prefix {
        classes_iter.next()
    } else {
        None
    };
    let class_str_postfix = if ignore_postfix {
        classes_iter.next_back()
    } else {
        None
    };

    // Collect the remaining classes into a vector if needed.
    let classes: Vec<&str> = classes_iter.collect();
    let classes_len = classes.len();

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
            .map(|class_info| class_info.text.as_str()),
    );

    // Add the first class back if it was ignored.
    if let Some(class_str_prefix) = class_str_prefix {
        sorted_classes.insert(0, class_str_prefix);
    }

    // Add the last class back if it was ignored.
    if let Some(class_str_postfix) = class_str_postfix {
        sorted_classes.push(class_str_postfix);
    }

    let mut result = sorted_classes.join(" ");

    if classes_len > 0 || ignore_postfix || ignore_prefix {
        // restore front space
        if class_name.starts_with(' ') {
            result.insert(0, ' ');
        }

        // restore final space
        if class_name.ends_with(' ') {
            result.push(' ');
        }
    }

    result
}

// Get the range of the class name to be sorted.
pub fn get_sort_class_name_range(
    class_name: &TokenText,
    range: &TextRange,
    ignore_prefix: bool,
    ignore_postfix: bool,
) -> Option<TextRange> {
    let mut class_iter = class_name.split_whitespace();
    let first_class_len = class_iter.next().map_or(0, |s| s.len()) as u32;
    let last_class_len = class_iter.next_back().map_or(0, |s| s.len()) as u32;
    let offset_prefix = if ignore_prefix { first_class_len } else { 0 };
    let offset_postfix = if ignore_postfix { last_class_len } else { 0 };

    let start = range.start() + TextSize::from(offset_prefix);
    let end = range.end() - TextSize::from(offset_postfix);

    if end < start {
        return None;
    }

    Some(TextRange::new(start, end))
}

// Returns whether the given node should be ignored prefix when sorting.
pub fn should_ignore_prefix(node: &AnyClassStringLike) -> bool {
    if let Some(value) = node.value() {
        // For example, for <div class={`${variable}mx-2 m-5`} />, we should ignore "${variable}mx-2" as a sorting item because it is an indivisible whole.
        let ignore_prefix = if let AnyClassStringLike::JsTemplateChunkElement(_template) = node {
            !value.starts_with(' ')
                && node
                    .syntax()
                    .prev_sibling()
                    .is_some_and(|sibling| JsTemplateElement::can_cast(sibling.kind()))
        } else {
            false
        };
        ignore_prefix
    } else {
        false
    }
}

// Returns whether the given node should be ignored postfix when sorting.
pub fn should_ignore_postfix(node: &AnyClassStringLike) -> bool {
    if let Some(value) = node.value() {
        // For example, for <div class={`mx-2 m-5${variable}`} />, we should ignore "m-5${variable}" as a sorting item because it is an indivisible whole.
        let ignore_postfix = if let AnyClassStringLike::JsTemplateChunkElement(_template) = node {
            !value.ends_with(' ')
                && node
                    .syntax()
                    .next_sibling()
                    .is_some_and(|sibling| JsTemplateElement::can_cast(sibling.kind()))
        } else {
            false
        };
        ignore_postfix
    } else {
        false
    }
}
