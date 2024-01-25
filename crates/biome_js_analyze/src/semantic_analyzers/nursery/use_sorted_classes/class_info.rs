//! Each CSS class needs to be processed to determine the information that will be used to sort it.
//! This information includes:
//! - The layer it belongs to (e.g. `components` or `utilities`).
//! - The index of the utility within the layer.
//! - The total variants weight that results from the combination of all the variants.
//! - The text of the class itself.
//! It is generated according to the information contained in a `SortConfig`, which includes:
//! - The list of layers, in order.
//! - The list of utilities, in order, for each layer.
//! - The list of variants, in order of importance (which is used to compute the variants weight).
//! - Other options, such as prefix and separator.

use super::{
    class_lexer::{tokenize_class, ClassSegmentStructure},
    sort_config::SortConfig,
};
use crate::semantic_analyzers::nursery::use_sorted_classes::sort_config::UtilityLayer;

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

impl UtilityMatch {
    /// Checks if a utility matches a target, and returns the result.
    fn from(target: &str, utility_text: &str) -> UtilityMatch {
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
        assert_eq!(UtilityMatch::from("px-2$", "px-2"), UtilityMatch::Exact);
        // TODO: support negative values
        // assert_eq!(UtilityMatch::from("px-2$", "-px-2"), UtilityMatch::Exact);
        assert_eq!(UtilityMatch::from("px-2$", "not-px-2"), UtilityMatch::None);
        assert_eq!(UtilityMatch::from("px-2$", "px-2-"), UtilityMatch::None);
        assert_eq!(UtilityMatch::from("px-2$", "px-4"), UtilityMatch::None);
        assert_eq!(UtilityMatch::from("px-2$", "px-2$"), UtilityMatch::None);
        assert_eq!(UtilityMatch::from("px-2$", "px-2-"), UtilityMatch::None);
        assert_eq!(UtilityMatch::from("px-2$", "px-2.5"), UtilityMatch::None);
        assert_eq!(UtilityMatch::from("px-2$", "px-2.5$"), UtilityMatch::None);
        assert_eq!(UtilityMatch::from("px-2$", "px-2.5-"), UtilityMatch::None);
    }

    #[test]
    fn test_partial_match() {
        assert_eq!(UtilityMatch::from("px-", "px-2"), UtilityMatch::Partial);
        // TODO: support negative values
        // assert_eq!(UtilityMatch::from("px-", "-px-2"), UtilityMatch::Partial);
        assert_eq!(UtilityMatch::from("px-", "px-2.5"), UtilityMatch::Partial);
        assert_eq!(
            UtilityMatch::from("px-", "px-anything"),
            UtilityMatch::Partial
        );
        assert_eq!(
            UtilityMatch::from("px-", "px-%$>?+=-"),
            UtilityMatch::Partial
        );
        assert_eq!(UtilityMatch::from("px-", "px-"), UtilityMatch::None);
        // TODO: support negative values
        // assert_eq!(UtilityMatch::from("px-", "-px-"), UtilityMatch::None);
        assert_eq!(UtilityMatch::from("px-", "not-px-2"), UtilityMatch::None);
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
    let mut layer: &str = "<no match>";
    let mut match_index: usize = 0;
    let mut last_size: usize = 0;

    // Iterate over each layer, looking for a match.
    for layer_data in utility_config.iter() {
        // Iterate over each target in the layer, looking for a match.
        for (index, target) in layer_data.classes.iter().enumerate() {
            match UtilityMatch::from(target, utility_text) {
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
                    let target_size = target.chars().count();
                    if target_size > last_size {
                        layer = &layer_data.name;
                        match_index = index;
                        last_size = target_size;
                    }
                }
                _ => {}
            }
        }
    }
    if layer != "<no match>" {
        return Some(UtilityInfo {
            layer,
            index: match_index,
        });
    }
    None
}

#[cfg(test)]
mod get_utility_info_tests {
    use super::*;
    use crate::semantic_analyzers::nursery::use_sorted_classes::sort_config::UtilityLayer;

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

// classes
// -------

/// Sort-related information about a CSS class.
#[derive(Debug, Eq, PartialEq)]
pub struct ClassInfo {
    /// The full text of the class itself.
    pub text: String,
    /// The total variants weight that results from the combination of all the variants.
    pub variant_weight: Option<u64>, // TODO: this will need to be Option<bitvec>
    /// The layer the utility belongs to.
    pub layer_index: usize,
    /// The index of the utility within the layer.
    pub utility_index: usize,
}

/// Computes sort-related information about a CSS class. If the class is not recognized as a utility,
/// it is considered a custom class instead and `None` is returned.
pub fn get_class_info(class_name: &str, sort_config: &SortConfig) -> Option<ClassInfo> {
    let utility_data = tokenize_class(class_name)?;
    let utility_info = get_utility_info(sort_config.utilities, &utility_data.utility);
    if let Some(utility_info) = utility_info {
        return Some(ClassInfo {
            text: class_name.to_string(),
            variant_weight: if utility_data.variants.is_empty() {
                None
            } else {
                // TODO: return None if there is an unknown variant.
                Some(0) // TODO: actually compute variant weight
            },
            layer_index: *sort_config.layer_index_map.get(&utility_info.layer)?,
            utility_index: utility_info.index,
        });
    }
    // If there is no utility info, the class is not recognized.
    None
}

#[cfg(test)]
mod get_class_info_tests {
    use super::*;
    use crate::semantic_analyzers::nursery::use_sorted_classes::sort_config::UtilityLayer;

    #[test]
    fn test_get_class_info() {
        const UTILITIES_CONFIG: [UtilityLayer; 2] = [
            UtilityLayer {
                name: "layer0",
                classes: &["px-", "py-", "block$"],
            },
            UtilityLayer {
                name: "layer1",
                classes: &["mx-", "my-", "inline$"],
            },
        ];
        let sort_config = SortConfig::new(UTILITIES_CONFIG.as_slice(), vec![]);
        assert_eq!(
            get_class_info("px-2", &sort_config),
            Some(ClassInfo {
                text: "px-2".to_string(),
                variant_weight: None,
                layer_index: 0,
                utility_index: 0,
            })
        );
        assert_eq!(
            get_class_info("py-2", &sort_config),
            Some(ClassInfo {
                text: "py-2".to_string(),
                variant_weight: None,
                layer_index: 0,
                utility_index: 1,
            })
        );
        assert_eq!(
            get_class_info("block", &sort_config),
            Some(ClassInfo {
                text: "block".to_string(),
                variant_weight: None,
                layer_index: 0,
                utility_index: 2,
            })
        );
        assert_eq!(
            get_class_info("mx-2", &sort_config),
            Some(ClassInfo {
                text: "mx-2".to_string(),
                variant_weight: None,
                layer_index: 1,
                utility_index: 0,
            })
        );
        assert_eq!(
            get_class_info("my-2", &sort_config),
            Some(ClassInfo {
                text: "my-2".to_string(),
                variant_weight: None,
                layer_index: 1,
                utility_index: 1,
            })
        );
        assert_eq!(
            get_class_info("inline", &sort_config),
            Some(ClassInfo {
                text: "inline".to_string(),
                variant_weight: None,
                layer_index: 1,
                utility_index: 2,
            })
        );
        assert_eq!(
            get_class_info("[arbitrary:css]", &sort_config),
            Some(ClassInfo {
                text: "[arbitrary:css]".to_string(),
                variant_weight: None,
                layer_index: 2,
                utility_index: 0,
            })
        );
        assert_eq!(get_class_info("unknown", &sort_config), None);
    }
}
