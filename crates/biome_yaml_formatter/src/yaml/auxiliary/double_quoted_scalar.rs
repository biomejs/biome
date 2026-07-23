use crate::flow_scalar::FormatFlowScalar;
use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlDoubleQuotedScalar, YamlDoubleQuotedScalarFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlDoubleQuotedScalar;
impl FormatNodeRule<YamlDoubleQuotedScalar> for FormatYamlDoubleQuotedScalar {
    fn fmt_fields(&self, node: &YamlDoubleQuotedScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlDoubleQuotedScalarFields { value_token } = node.as_fields();
        write!(
            f,
            [FormatFlowScalar {
                token: &value_token?
            }]
        )
    }
}
