use biome_rowan::{TextRange, TokenText};
use std::cmp::Ordering;

use super::{
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

fn compare_classes(a: &ClassInfo, b: &ClassInfo) -> Ordering {
    if let Some(has_arbitrary_variants) = a.cmp_arbitrary_variants_existence(b) {
        return has_arbitrary_variants;
    }

    if let Some(arbitrary_variants_order) = a.cmp_arbitrary_variants(b) {
        return arbitrary_variants_order;
    }

    if let Some(has_variants_order) = a.cmp_variants_weight_existence(b) {
        return has_variants_order;
    }

    if let Some(layers_order) = a.cmp_layers(b) {
        return layers_order;
    }

    if let Some(variants_weight_order) = a.cmp_variants_weight(b) {
        return variants_weight_order;
    }

    if let Some(utilities_order) = a.cmp_utilities(b) {
        return utilities_order;
    }

    Ordering::Equal
}

/// Sort a slice of class strings according to the given sort config.
pub fn sort_class_name_from_slice(classes: &[&str], sort_config: &SortConfig) -> String {
    let mut sorted_classes = Vec::new();
    let mut classes_info = Vec::new();
    for class in classes {
        match get_class_info(class, sort_config) {
            Some(class_info) => {
                classes_info.push(class_info);
            }
            None => {
                sorted_classes.push(*class);
            }
        }
    }

    classes_info.sort_unstable_by(|a, b| a.text.cmp(&b.text));
    classes_info.sort_by(compare_classes);

    sorted_classes.extend(
        classes_info
            .iter()
            .map(|class_info| class_info.text.as_ref()),
    );

    sorted_classes.join(" ")
}

/// Sort the given class string according to the given sort config.
pub fn sort_class_name(class_name: &TokenText, sort_config: &SortConfig) -> String {
    let classes: Vec<&str> = class_name.split_whitespace().collect();
    sort_class_name_from_slice(&classes, sort_config)
}

/// Get the range of the class name to be sorted.
pub fn get_sort_class_name_range(
    class_name: &TokenText,
    range: &TextRange,
) -> Option<TextRange> {
    let _ = class_name;
    Some(*range)
}
