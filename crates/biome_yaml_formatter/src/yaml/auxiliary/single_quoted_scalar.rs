use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlSingleQuotedScalar, YamlSingleQuotedScalarFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlSingleQuotedScalar;
impl FormatNodeRule<YamlSingleQuotedScalar> for FormatYamlSingleQuotedScalar {
    fn fmt_fields(&self, node: &YamlSingleQuotedScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlSingleQuotedScalarFields { value_token } = node.as_fields();
        write!(f, [value_token?.format()])
    }
}
