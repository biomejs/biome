//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_yaml_syntax::AnyYamlDocument;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyYamlDocument;
impl FormatRule<AnyYamlDocument> for FormatAnyYamlDocument {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &AnyYamlDocument, f: &mut YamlFormatter) -> FormatResult<()> {
        match node {
            AnyYamlDocument::YamlBogus(node) => node.format().fmt(f),
            AnyYamlDocument::YamlDocument(node) => node.format().fmt(f),
        }
    }
}
