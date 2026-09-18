use crate::{CssRegularDimension, decode_css_identifier};
use biome_string_case::StrLikeExtension;

impl CssRegularDimension {
    /// Returns `true` when the decoded unit is present in the sorted, lowercase `units` list.
    pub fn matches_unit(&self, units: &[&str]) -> bool {
        self.unit_token().is_ok_and(|token| {
            let unit = decode_css_identifier(token.text_trimmed());
            units
                .binary_search(&unit.to_ascii_lowercase_cow().as_ref())
                .is_ok()
        })
    }
}
