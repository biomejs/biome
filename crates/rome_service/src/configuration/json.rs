use crate::configuration::merge::MergeWith;
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to JSON files
#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct JsonConfiguration {
    /// Parsing options
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(json_parser), optional)]
    pub parser: Option<JsonParser>,
}

impl JsonConfiguration {
    pub const KNOWN_KEYS: &'static [&'static str] = &["parser"];
}

impl MergeWith<JsonConfiguration> for JsonConfiguration {
    fn merge_with(&mut self, other: JsonConfiguration) {
        if let Some(other_parser) = other.parser {
            let parser = self.parser.get_or_insert_with(JsonParser::default);
            parser.merge_with(other_parser);
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct JavascriptOrganizeImports {}

/// Options that changes how the JSON parser behaves
#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsonParser {
    #[bpaf(hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Allow parsing comments in `.json` files
    pub allow_comments: Option<bool>,
}

impl JsonParser {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] = &["allowComments"];
}

impl MergeWith<JsonParser> for JsonParser {
    fn merge_with(&mut self, other: JsonParser) {
        if let Some(allow_comments) = other.allow_comments {
            self.allow_comments = Some(allow_comments);
        }
    }
}
