pub mod import_groups;
pub mod import_source;

use crate::organize_imports::import_groups::ImportGroups;
pub use crate::shared::sort_order::SortOrder;
use biome_console::markup;
use biome_deserialize::{DeserializableValidator, DeserializationDiagnostic};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_diagnostics::Severity;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
#[deserializable(with_validator)]
pub struct OrganizeImportsOptions {
    /// Groups to change how imports and exports are sorted.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub groups: Option<ImportGroups>,
    /// Order used for sorting identifiers within imports and exports.
    ///
    /// Default: `natural`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub identifier_order: Option<SortOrder>,
    /// If `true`, bare imports such as `import "module"` are sorted with other imports.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub sort_bare_imports: Option<bool>,
}

impl DeserializableValidator for OrganizeImportsOptions {
    fn validate(
        &mut self,
        ctx: &mut impl biome_deserialize::DeserializationContext,
        _name: &str,
        range: biome_rowan::TextRange,
    ) -> bool {
        if self
            .sort_bare_imports
            .is_none_or(|sort_bare_imports| !sort_bare_imports)
            && let Some(groups) = self.groups.as_ref()
            && groups.has_bare_matchers()
        {
            ctx.report(
                DeserializationDiagnostic::new(
                    markup!(
                        "The "<Emphasis>"groups"</Emphasis>" option contains kind matchers set to "<Emphasis>"bare"</Emphasis>" or "<Emphasis>"!bare"</Emphasis>". "
                        "They have no effect because the "<Emphasis>"sortBareImports"</Emphasis>" option is not set to "<Emphasis>"true"</Emphasis>". "
                        "Set "<Emphasis>"sortBareImports"</Emphasis>" to "<Emphasis>"true"</Emphasis>" or remove the bare kind matchers."
                    ),
                )
                .with_range(range)
                .with_custom_severity(Severity::Error)
                .with_note(markup!(
                        "We are assessing an approach to sort a subset of bare imports, using bare kind matchers without setting "<Emphasis>"sortBareImports"</Emphasis>" to "<Emphasis>"true"</Emphasis>". "
                        "See the "<Hyperlink href="https://github.com/biomejs/biome/pull/10190">"Pull Request"</Hyperlink>" for more details."
                    ))
            );
            false
        } else {
            true
        }
    }
}
