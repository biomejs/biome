//! The following structures define the config required to compute sort-related information about
//! CSS classes (`ClassInfo`) that is later used to compare and sort them. A sort config includes:
//! - The list of layers, in order.
//! - The list of utilities, in order, for each layer.
//! - The list of variants, in order of importance (which is used to compute the variants weight).
//! - Other options, such as prefix and separator.

use std::collections::HashMap;

use bitvec::{order::Lsb0, vec::BitVec};

use super::presets::ConfigPreset;

/// A utility layer, containing its name and an ordered list of classes.
pub struct UtilityLayer {
    pub name: &'static str,
    pub classes: &'static [&'static str],
}

/// An owned version of UtilityLayer for dynamic class lists.
pub struct UtilityLayerOwned {
    pub name: String,
    pub classes: Vec<String>,
}

pub fn build_variant_weight(size: usize) -> BitVec<u8, Lsb0> {
    let mut bit_vec = BitVec::new();
    let iterable = vec![false; size];
    bit_vec.extend(iterable);
    bit_vec.push(true);
    bit_vec
}

/// The utilities config, contains an ordered list of utility layers.
pub type UtilitiesConfig = &'static [UtilityLayer];

/// The variants config, contains an ordered list of variants.
pub type VariantsConfig = &'static [&'static str];

/// The sort config, containing the utility config and the variant config.
pub struct SortConfig {
    pub utilities: &'static [UtilityLayer],
    pub variants: VariantsConfig,
    pub layer_index_map: HashMap<&'static str, usize>,
    /// Optional owned utilities for dynamic custom classes
    pub utilities_owned: Option<Vec<UtilityLayerOwned>>,
    /// Optional owned layer index map for custom classes
    pub layer_index_map_owned: Option<HashMap<String, usize>>,
}

impl SortConfig {
    /// Creates a new sort config.
    pub fn new(preset: &ConfigPreset) -> Self {
        // Compute the layer index map.
        let mut layer_index_map: HashMap<&'static str, usize> = HashMap::new();
        let mut index = 0;
        for layer in preset.utilities.iter() {
            layer_index_map.insert(layer.name, index);
            index += 1;
        }
        layer_index_map.insert("arbitrary", index);

        Self {
            utilities: preset.utilities,
            variants: preset.variants,
            layer_index_map,
            utilities_owned: None,
            layer_index_map_owned: None,
        }
    }

    /// Gets the layer index, regardless of owned/static backing.
    #[allow(dead_code)]
    pub fn layer_index(&self, name: &str) -> Option<usize> {
        if let Some(map) = &self.layer_index_map_owned {
            map.get(name).copied()
        } else {
            self.layer_index_map.get(name).copied()
        }
    }

    /// Creates a new sort config with custom classes merged into the preset.
    pub fn with_custom_classes(
        preset: &ConfigPreset,
        custom_components: Option<&[Box<str>]>,
        custom_utilities: Option<&[Box<str>]>,
    ) -> Self {
        // Early return if no custom classes provided
        if custom_components.is_none() && custom_utilities.is_none() {
            return Self::new(preset);
        }

        // Build owned utility layers by merging preset with custom classes
        let utilities_owned: Vec<UtilityLayerOwned> = preset
            .utilities
            .iter()
            .map(|layer| {
                let mut classes: Vec<String> =
                    layer.classes.iter().copied().map(String::from).collect();

                // Append custom classes to the end of their respective layer
                match layer.name {
                    "components" => {
                        if let Some(custom) = custom_components {
                            classes.extend(custom.iter().map(|s| s.to_string()));
                        }
                    }
                    "utilities" => {
                        if let Some(custom) = custom_utilities {
                            classes.extend(custom.iter().map(|s| s.to_string()));
                        }
                    }
                    _ => {} // No custom classes for other layers
                }

                UtilityLayerOwned {
                    name: layer.name.to_string(),
                    classes,
                }
            })
            .collect();

        // Compute owned layer index map
        let layer_index_map_owned: HashMap<String, usize> = utilities_owned
            .iter()
            .enumerate()
            .map(|(idx, layer)| (layer.name.clone(), idx))
            .chain(std::iter::once((
                "arbitrary".to_string(),
                utilities_owned.len(),
            )))
            .collect();

        Self {
            utilities: preset.utilities,
            variants: preset.variants,
            layer_index_map: HashMap::new(), // Unused when utilities_owned is Some
            utilities_owned: Some(utilities_owned),
            layer_index_map_owned: Some(layer_index_map_owned),
        }
    }
}
