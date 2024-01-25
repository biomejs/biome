use biome_rowan::TokenText;
use std::cmp::Ordering;

use super::{
    class_info::{get_class_info, ClassInfo},
    sort_config::SortConfig,
};

impl ClassInfo {
    /// Compare based on the existence of variants. Classes with variants go last.
    /// Returns `None` if both or none of the classes has variants.
    fn cmp_has_variants(&self, other: &ClassInfo) -> Option<Ordering> {
        if self.variant_weight.is_some() && other.variant_weight.is_some() {
            return None;
        }
        if self.variant_weight.is_some() {
            return Some(Ordering::Greater);
        }
        if other.variant_weight.is_some() {
            return Some(Ordering::Less);
        }
        None
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

    /// Compare based on variants weight. Classes with higher weight go first.
    /// Returns `None` if they have the same weight.
    fn cmp_variants_weight(&self, _other: &ClassInfo) -> Option<Ordering> {
        // TODO: implement variant weight comparison.
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
    // Classes with variants go last.
    if let Some(has_variants_order) = a.cmp_has_variants(b) {
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
pub fn sort_class_name(class_name: &TokenText, sort_config: &SortConfig) -> String {
    // Obtain classes by splitting the class string by whitespace.
    let classes = class_name.split_whitespace().collect::<Vec<&str>>();

    // Separate custom classes from recognized classes, and compute the recognized classes' info.
    // Custom classes always go first, in the order that they appear in.
    let mut sorted_classes: Vec<&str> = Vec::new();
    let mut classes_info: Vec<ClassInfo> = Vec::new();
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

    // Join the classes back into a string.
    sorted_classes.join(" ")
}
