//! Each CSS class needs to be processed to determine the information that will be used to sort it.
//! This information includes:
//! - The layer it belongs to (e.g. `components` or `utilities`).
//! - The index of the utility within the layer.
//! - The total variants weight that results from the combination of all the variants.
//! - The text of the class itself.
//! - The arbitrary variants of the class.
//!     It is generated according to the information contained in a `SortConfig`, which includes:
//! - The list of layers, in order.
//! - The list of utilities, in order, for each layer.
//! - The list of variants, in order of importance (which is used to compute the variants weight).
//! - Other options, such as prefix and separator.

use std::{cmp::Ordering, collections::HashMap};

use bitvec::{order::Lsb0, vec::BitVec};

use super::{
    class_lexer::{tokenize_class, ClassSegmentStructure},
    sort_config::{build_variant_weight, SortConfig, VariantsConfig},
};
use crate::lint::nursery::use_sorted_classes::sort_config::UtilityLayer;

// utilities
// ---------

/// The result of matching a utility against a target.
#[derive(Debug, Eq, PartialEq)]
enum UtilityMatch {
    /// The utility matches an exact target.
    Exact,
    /// The utility matches a partial target.
    Partial,
    /// The utility does not match the target.
    None,
}

impl From<(&str, &str)> for UtilityMatch {
    /// Checks if a utility matches a target, and returns the result.
    fn from((target, utility_text): (&str, &str)) -> UtilityMatch {
        // If the target ends with `$`, then it's an exact target.
        if target.ends_with('$') {
            // Check if the utility matches the target (without the final `$`) exactly.
            if utility_text == &target[..target.len() - 1] {
                return UtilityMatch::Exact;
            }
            return UtilityMatch::None;
        }
        // Check if the utility starts with the (partial) target.
        if utility_text.starts_with(target) && utility_text != target {
            return UtilityMatch::Partial;
        }
        // If all of the above checks fail, there is no match.
        UtilityMatch::None
    }
}

#[cfg(test)]
mod utility_match_tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        assert_eq!(UtilityMatch::from(("px-2$", "px-2")), UtilityMatch::Exact);
        // TODO: support negative values
        // assert_eq!(UtilityMatch::from(("px-2$", "-px-2")), UtilityMatch::Exact);
        assert_eq!(
            UtilityMatch::from(("px-2$", "not-px-2")),
            UtilityMatch::None
        );
        assert_eq!(UtilityMatch::from(("px-2$", "px-2-")), UtilityMatch::None);
        assert_eq!(UtilityMatch::from(("px-2$", "px-4")), UtilityMatch::None);
        assert_eq!(UtilityMatch::from(("px-2$", "px-2$")), UtilityMatch::None);
        assert_eq!(UtilityMatch::from(("px-2$", "px-2-")), UtilityMatch::None);
        assert_eq!(UtilityMatch::from(("px-2$", "px-2.5")), UtilityMatch::None);
        assert_eq!(UtilityMatch::from(("px-2$", "px-2.5$")), UtilityMatch::None);
        assert_eq!(UtilityMatch::from(("px-2$", "px-2.5-")), UtilityMatch::None);
    }

    #[test]
    fn test_partial_match() {
        assert_eq!(UtilityMatch::from(("px-", "px-2")), UtilityMatch::Partial);
        // TODO: support negative values
        // assert_eq!(UtilityMatch::from(("px-", "-px-2")), UtilityMatch::Partial);
        assert_eq!(UtilityMatch::from(("px-", "px-2.5")), UtilityMatch::Partial);
        assert_eq!(
            UtilityMatch::from(("px-", "px-anything")),
            UtilityMatch::Partial
        );
        assert_eq!(
            UtilityMatch::from(("px-", "px-%$>?+=-")),
            UtilityMatch::Partial
        );
        assert_eq!(UtilityMatch::from(("px-", "px-")), UtilityMatch::None);
        // TODO: support negative values
        // assert_eq!(UtilityMatch::from(("px-", "-px-")), UtilityMatch::None);
        assert_eq!(UtilityMatch::from(("px-", "not-px-2")), UtilityMatch::None);
    }
}

/// Sort-related information about a utility.
#[derive(Debug, Eq, PartialEq)]
struct UtilityInfo {
    /// The layer the utility belongs to.
    layer: &'static str,
    /// The index of the utility within the layer.
    index: usize,
}

/// Computes sort-related information about a CSS utility. If the utility is not recognized,
/// `None` is returned.
fn get_utility_info(
    utility_config: &[UtilityLayer],
    utility_data: &ClassSegmentStructure,
) -> Option<UtilityInfo> {
    // Arbitrary CSS utilities always go in the "arbitrary" layer, at index 0.
    // This layer is always at the end, and the order of the utilities in it is not
    // determined at this point, so they all have the same index.
    if utility_data.arbitrary {
        return Some(UtilityInfo {
            layer: "arbitrary",
            index: 0,
        });
    }

    let utility_text = utility_data.text.as_str();
    let mut layer: Option<&str> = None;
    let mut match_index: usize = 0;
    let mut last_size: usize = 0;

    // Iterate over each layer, looking for a match.
    for layer_data in utility_config.iter() {
        // Iterate over each target in the layer, looking for a match.
        for (index, &target) in layer_data.classes.iter().enumerate() {
            match UtilityMatch::from((target, utility_text)) {
                UtilityMatch::Exact => {
                    // Exact matches can be returned immediately.
                    return Some(UtilityInfo {
                        layer: layer_data.name,
                        index,
                    });
                }
                UtilityMatch::Partial => {
                    // Multiple partial matches can occur, so we need to keep looking to find
                    // the longest target that matches. For example, if the utility text is
                    // `gap-x-4`, and there are targets like `gap-` and `gap-x-`, we want to
                    // make sure that the `gap-x-` target is matched as it is more specific,
                    // regardless of the order in which the targets are defined.
                    let target_size = target.len();
                    if target_size > last_size {
                        layer = Some(layer_data.name);
                        match_index = index;
                        last_size = target_size;
                    }
                }
                UtilityMatch::None => {}
            }
        }
    }
    if let Some(layer_match) = layer {
        return Some(UtilityInfo {
            layer: layer_match,
            index: match_index,
        });
    }
    None
}

#[cfg(test)]
mod get_utility_info_tests {
    use super::*;
    use crate::lint::nursery::use_sorted_classes::sort_config::UtilityLayer;

    #[test]
    fn test_exact_match() {
        let utility_config = vec![UtilityLayer {
            name: "layer",
            classes: &["px-2$"],
        }];
        let utility_data = ClassSegmentStructure {
            text: "px-2".to_string(),
            arbitrary: false,
        };
        assert_eq!(
            get_utility_info(utility_config.as_slice(), &utility_data),
            Some(UtilityInfo {
                layer: "layer",
                index: 0,
            })
        );
        let utility_data = ClassSegmentStructure {
            text: "px-4".to_string(),
            arbitrary: false,
        };
        assert_eq!(
            get_utility_info(utility_config.as_slice(), &utility_data),
            None
        );
    }

    #[test]
    fn test_partial_match() {
        let utility_config = vec![UtilityLayer {
            name: "layer",
            classes: &["px-"],
        }];
        let utility_data = ClassSegmentStructure {
            text: "px-2".to_string(),
            arbitrary: false,
        };
        assert_eq!(
            get_utility_info(utility_config.as_slice(), &utility_data),
            Some(UtilityInfo {
                layer: "layer",
                index: 0,
            })
        );
        let utility_data = ClassSegmentStructure {
            text: "not-px-2".to_string(),
            arbitrary: false,
        };
        assert_eq!(
            get_utility_info(utility_config.as_slice(), &utility_data),
            None
        );
    }

    #[test]
    fn test_partial_match_longest() {
        let utility_config = vec![UtilityLayer {
            name: "layer",
            classes: &["border-", "border-t-"],
        }];
        let utility_data = ClassSegmentStructure {
            text: "border-t-2".to_string(),
            arbitrary: false,
        };
        assert_eq!(
            get_utility_info(utility_config.as_slice(), &utility_data),
            Some(UtilityInfo {
                layer: "layer",
                index: 1,
            })
        );
    }

    #[test]
    fn test_partial_match_longest_first() {
        let utility_config = vec![UtilityLayer {
            name: "layer",
            classes: &["border-t-", "border-"],
        }];
        let utility_data = ClassSegmentStructure {
            text: "border-t-2".to_string(),
            arbitrary: false,
        };
        assert_eq!(
            get_utility_info(utility_config.as_slice(), &utility_data),
            Some(UtilityInfo {
                layer: "layer",
                index: 0,
            })
        );
    }

    #[test]
    fn test_arbitrary_layer() {
        let utility_config = vec![UtilityLayer {
            name: "layer",
            classes: &["border-t-", "border-"],
        }];
        let utility_data = ClassSegmentStructure {
            text: "[arbitrary:css]".to_string(),
            arbitrary: true,
        };
        assert_eq!(
            get_utility_info(utility_config.as_slice(), &utility_data),
            Some(UtilityInfo {
                layer: "arbitrary",
                index: 0,
            })
        );
    }
}

// variants
// -------

/// The result of matching a variant against a target.
#[derive(Debug, Eq, PartialEq)]
enum VariantMatch {
    /// The variant matches an exact target.
    Exact,
    /// The variant matches a partial target.
    Partial,
    /// The variant does not match the target.
    None,
}

impl From<(&str, &str)> for VariantMatch {
    /// Checks if a variant matches a target, and returns the result.
    fn from((target, variant_text): (&str, &str)) -> VariantMatch {
        // If the target matched exactly the variant text.
        if target == variant_text {
            return VariantMatch::Exact;
        };

        let mut target_chars = target.bytes();
        let mut target_found = true;
        let mut dash_found = false;
        let mut bracket_found = false;
        // Checks if variant text has a custom value thus it starts with the target and it's followed by "-["
        for byte in variant_text.bytes() {
            match (byte, target_chars.next()) {
                (_, Some(target_byte)) => {
                    if target_byte != byte {
                        target_found = false;
                        break;
                    }
                }
                (b'-', None) => {
                    if target_found {
                        dash_found = true;
                    }
                }
                (b'[', None) => {
                    if target_found && dash_found {
                        bracket_found = true;
                    }
                }
                (_, None) => {
                    break;
                }
            }
        }

        if target_found && dash_found && bracket_found {
            return VariantMatch::Exact;
        }

        // Check if the variant starts with the (partial) target.
        if variant_text.starts_with(target) && variant_text != target {
            return VariantMatch::Partial;
        }
        // If all of the above checks fail, there is no match.
        VariantMatch::None
    }
}

#[cfg(test)]
mod variant_match_tests {
    use crate::lint::nursery::use_sorted_classes::class_info::VariantMatch;

    #[test]
    fn test_exact_match() {
        assert_eq!(VariantMatch::from(("hover", "hover")), VariantMatch::Exact);
        assert_eq!(VariantMatch::from(("focus", "focus")), VariantMatch::Exact);
        assert_eq!(
            VariantMatch::from(("group", "group-[.is-published]")),
            VariantMatch::Exact
        );
        assert_eq!(
            VariantMatch::from(("has", "has-[:checked]")),
            VariantMatch::Exact
        );
        assert_eq!(
            VariantMatch::from(("group-has", "group-has-[.custom-class]")),
            VariantMatch::Exact
        );
        assert_eq!(
            VariantMatch::from(("group-aria-disabled", "group-aria-disabled")),
            VariantMatch::Exact
        );
    }

    #[test]
    fn test_partial_match() {
        assert_eq!(
            VariantMatch::from(("group", "group-has-[.custom-class]")),
            VariantMatch::Partial
        );
        assert_eq!(
            VariantMatch::from(("peer", "peer-has-[:checked]")),
            VariantMatch::Partial
        );
    }

    #[test]
    fn test_no_match() {
        assert_eq!(VariantMatch::from(("group", "hover")), VariantMatch::None);
        assert_eq!(
            VariantMatch::from(("group-aria-busy", "group-aria-disabled")),
            VariantMatch::None
        );
    }
}

fn find_variant_position(config_variants: VariantsConfig, variant_text: &str) -> Option<usize> {
    let mut variant: Option<&str> = None;
    let mut match_index: usize = 0;
    let mut last_size: usize = 0;

    // Iterate over each variant looking for a match.
    for (index, &target) in config_variants.iter().enumerate() {
        match VariantMatch::from((target, variant_text)) {
            VariantMatch::Exact => {
                // Exact matches can be returned immediately.
                return Some(index);
            }
            VariantMatch::Partial => {
                // Multiple partial matches can occur, so we need to keep looking to find
                // the longest target that matches. For example, if the variant text is
                // `group-aria-[.custom-class]`, and there are targets like `group` and `group-aria`, we want to
                // make sure that the `group-aria` target is matched as it is more specific,
                // so when the target is `group` a Partial match will occur.
                let target_size = target.len();
                if target_size > last_size {
                    variant = Some(target);
                    match_index = index;
                    last_size = target_size;
                }
            }
            VariantMatch::None => {}
        }
    }
    if variant.is_some() {
        return Some(match_index);
    };
    None
}

pub fn compute_variants_weight(
    config_variants: VariantsConfig,
    current_variants: &[&ClassSegmentStructure],
) -> Option<BitVec<u8, Lsb0>> {
    if current_variants.is_empty() {
        return None;
    };
    // Check if it's a known variant
    // If it is then compute weights for each variant on the fly by using index as size
    // TODO: Cache the weights for next run?
    let mut variants_map: HashMap<&str, BitVec<u8, Lsb0>> = HashMap::new();
    for current_variant in current_variants.iter() {
        let variant_name = current_variant.text.as_ref();
        let Some(variant_index) = find_variant_position(config_variants, variant_name) else {
            continue;
        };

        if !variants_map.contains_key(variant_name) {
            variants_map.insert(variant_name, build_variant_weight(variant_index));
        }
    }

    // If there's a custom variant, their weight isn't important
    if variants_map.is_empty() {
        return None;
    }

    // Compute Variants Weight as the BitWise XOR of all the recognized variants' weights
    let variants_weight = variants_map
        .iter()
        .fold(BitVec::<u8, Lsb0>::new(), |acc, (_, val)| {
            let mut accumulator = acc.clone();
            let mut current_weight = val.clone();
            let acc_len = accumulator.len();
            let current_weight_len = current_weight.len();

            match acc_len.cmp(&current_weight_len) {
                Ordering::Less => accumulator.resize(current_weight_len, false),
                Ordering::Greater => current_weight.resize(acc_len, false),
                _ => (),
            }

            accumulator ^ current_weight
        });

    Some(variants_weight)
}

// classes
// -------

/// Sort-related information about a CSS class.
#[derive(Debug, Eq, PartialEq)]
pub struct ClassInfo {
    /// The full text of the class itself.
    pub text: String,
    /// The total variants weight that results from the combination of all the variants.
    pub variant_weight: Option<BitVec<u8, Lsb0>>,
    /// The layer the utility belongs to.
    pub layer_index: usize,
    /// The index of the utility within the layer.
    pub utility_index: usize,
    /// Arbitrary variants
    pub arbitrary_variants: Option<Vec<String>>,
}

/// Computes sort-related information about a CSS class. If the class is not recognized as a utility,
/// it is considered a custom class instead and `None` is returned.
pub fn get_class_info(class_name: &str, sort_config: &SortConfig) -> Option<ClassInfo> {
    let utility_data = tokenize_class(class_name)?;
    let utility_info = get_utility_info(sort_config.utilities, &utility_data.utility);

    // Split up variants into arbitrary and known variants.
    let (arbitrary_variants, current_variants): (
        Vec<&ClassSegmentStructure>,
        Vec<&ClassSegmentStructure>,
    ) = utility_data.variants.iter().partition(|el| el.arbitrary);

    let arbitrary_variants: Vec<String> = arbitrary_variants
        .iter()
        .map(|&variant| variant.text.clone())
        .collect();

    if let Some(utility_info) = utility_info {
        return Some(ClassInfo {
            text: class_name.to_string(),
            variant_weight: compute_variants_weight(sort_config.variants, &current_variants),
            layer_index: *sort_config.layer_index_map.get(&utility_info.layer)?,
            utility_index: utility_info.index,
            arbitrary_variants: if arbitrary_variants.is_empty() {
                None
            } else {
                Some(arbitrary_variants)
            },
        });
    }
    // If there is no utility info, the class is not recognized.
    None
}

#[cfg(test)]
mod get_class_info_tests {
    use bitvec::bitvec;

    use super::*;
    use crate::lint::nursery::use_sorted_classes::{
        presets::ConfigPreset, sort_config::UtilityLayer,
    };

    #[test]
    fn test_get_class_info() {
        const UTILITIES_CONFIG: [UtilityLayer; 2] = [
            UtilityLayer {
                name: "layer0",
                classes: &["px-", "py-", "block$", "bg-"],
            },
            UtilityLayer {
                name: "layer1",
                classes: &["mx-", "my-", "inline$"],
            },
        ];
        let variants: &'static [&'static str; 4] = &["hover", "focus", "focus-visible", "active"];

        let sort_config = SortConfig::new(&ConfigPreset {
            utilities: &UTILITIES_CONFIG,
            variants,
        });

        assert_eq!(
            get_class_info("px-2", &sort_config),
            Some(ClassInfo {
                text: "px-2".to_string(),
                variant_weight: None,
                layer_index: 0,
                utility_index: 0,
                arbitrary_variants: None
            })
        );
        assert_eq!(
            get_class_info("py-2", &sort_config),
            Some(ClassInfo {
                text: "py-2".to_string(),
                variant_weight: None,
                layer_index: 0,
                utility_index: 1,
                arbitrary_variants: None,
            })
        );
        assert_eq!(
            get_class_info("block", &sort_config),
            Some(ClassInfo {
                text: "block".to_string(),
                variant_weight: None,
                layer_index: 0,
                utility_index: 2,
                arbitrary_variants: None
            })
        );
        assert_eq!(
            get_class_info("mx-2", &sort_config),
            Some(ClassInfo {
                text: "mx-2".to_string(),
                variant_weight: None,
                layer_index: 1,
                utility_index: 0,
                arbitrary_variants: None
            })
        );
        assert_eq!(
            get_class_info("my-2", &sort_config),
            Some(ClassInfo {
                text: "my-2".to_string(),
                variant_weight: None,
                layer_index: 1,
                utility_index: 1,
                arbitrary_variants: None
            })
        );
        assert_eq!(
            get_class_info("inline", &sort_config),
            Some(ClassInfo {
                text: "inline".to_string(),
                variant_weight: None,
                layer_index: 1,
                utility_index: 2,
                arbitrary_variants: None
            })
        );
        assert_eq!(
            get_class_info("[arbitrary:css]", &sort_config),
            Some(ClassInfo {
                text: "[arbitrary:css]".to_string(),
                variant_weight: None,
                layer_index: 2,
                utility_index: 0,
                arbitrary_variants: None
            })
        );
        assert_eq!(
            get_class_info("hover:bg-red-500", &sort_config),
            Some(ClassInfo {
                text: "hover:bg-red-500".to_string(),
                variant_weight: Some(bitvec![u8, Lsb0; 1]),
                layer_index: 0,
                utility_index: 3,
                arbitrary_variants: None
            })
        );
        assert_eq!(
            get_class_info("hover:focus:bg-yellow-600", &sort_config),
            Some(ClassInfo {
                text: "hover:focus:bg-yellow-600".to_string(),
                variant_weight: Some(bitvec![u8, Lsb0; 1, 1]),
                layer_index: 0,
                utility_index: 3,
                arbitrary_variants: None
            })
        );
        assert_eq!(
            get_class_info("[&nth-child(2)]:bg-yellow-300", &sort_config),
            Some(ClassInfo {
                text: "[&nth-child(2)]:bg-yellow-300".to_string(),
                variant_weight: None,
                layer_index: 0,
                utility_index: 3,
                arbitrary_variants: Some(vec!["[&nth-child(2)]".to_string()])
            })
        );
        assert_eq!(
            get_class_info("[&nth-child(1)]:focus:bg-yellow-300", &sort_config),
            Some(ClassInfo {
                text: "[&nth-child(1)]:focus:bg-yellow-300".to_string(),
                variant_weight: Some(bitvec![u8, Lsb0; 0, 1]),
                layer_index: 0,
                utility_index: 3,
                arbitrary_variants: Some(vec!["[&nth-child(1)]".to_string()])
            })
        );
        assert_eq!(get_class_info("unknown", &sort_config), None);
    }
}
