use super::{
    class_parser::{parse_class, ClassSegmentStructure},
    sort_config::{SortConfig, UtilitiesConfig},
};

// utilities
// ---------

enum UtilityMatch {
    Exact,
    Partial,
    None,
}

impl UtilityMatch {
    fn from(target: &String, utility_text: &str) -> UtilityMatch {
        // If the target ends with `$`, then it's an exact target.
        if target.ends_with('$') {
            // Check if the utility matches the target (without the final `$`) exactly.
            if utility_text == &target[..target.len() - 1] {
                return UtilityMatch::Exact;
            }
            return UtilityMatch::None;
        }
        // Check the utility starts with the (partial) target.
        if utility_text.starts_with(target) && utility_text != target {
            return UtilityMatch::Partial;
        }
        UtilityMatch::None
    }
}

struct UtilityInfo {
    layer: String,
    index: usize,
}

fn get_utility_info(
    utility_config: &UtilitiesConfig,
    utility_data: &ClassSegmentStructure,
) -> Option<UtilityInfo> {
    // Arbitrary CSS utilities always go in the "arbitrary" layer, at index 0.
    // This layer is always at the end, and the order of the utilities in it is not
    // determined at this point, so they all have the same index.
    if utility_data.arbitrary {
        return Some(UtilityInfo {
            layer: "arbitrary".to_string(),
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
                        layer: layer_data.name.clone(),
                        index: index + 1,
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
            layer: layer.to_string(),
            index: match_index,
        });
    }
    None
}

// classes
// -------

/// Information about a CSS class.
#[derive(Debug)]
pub struct ClassInfo {
    pub text: String,
    pub variant_weight: Option<u64>, // TODO: this will need to be Option<u64>
    pub layer_index: usize,
    pub utility_index: usize,
}

/// Computes the information about a CSS class.
pub fn get_class_info(class_name: &str, sort_config: &SortConfig) -> Option<ClassInfo> {
    let utility_data = parse_class(class_name);
    let utility_info = get_utility_info(&sort_config.utilities, &utility_data.utility);
    if let Some(utility_info) = utility_info {
        return Some(ClassInfo {
            text: class_name.to_string(),
            variant_weight: if utility_data.variants.is_empty() {
                None
            } else {
                Some(0) // TODO: actually compute variant weight
            },
            layer_index: *sort_config.layer_index_map.get(&utility_info.layer)?,
            utility_index: utility_info.index,
        });
    }
    // If there is no utility info, that means the class is not recognized as a utility,
    // and it is a custom class instead.
    None
}
