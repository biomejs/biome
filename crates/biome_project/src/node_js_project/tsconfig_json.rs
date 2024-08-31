use crate::{LanguageRoot, Manifest};
use biome_deserialize::json::deserialize_from_json_ast;
use biome_deserialize::Deserialized;
use biome_deserialize_macros::Deserializable;
use biome_json_syntax::JsonLanguage;
use rustc_hash::FxHashMap;

#[derive(Debug, Default, Clone, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub struct TsConfigJson {
    base_url: Option<String>,
    paths: FxHashMap<String, Vec<String>>,
}

impl Manifest for TsConfigJson {
    type Language = JsonLanguage;

    fn deserialize_manifest(root: &LanguageRoot<Self::Language>) -> Deserialized<Self> {
        deserialize_from_json_ast::<TsConfigJson>(root, "")
    }
}
