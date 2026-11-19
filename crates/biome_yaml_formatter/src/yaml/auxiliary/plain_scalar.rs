use crate::flow_scalar::FormatFlowScalar;
use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlPlainScalar, YamlPlainScalarFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlPlainScalar;
impl FormatNodeRule<YamlPlainScalar> for FormatYamlPlainScalar {
    fn fmt_fields(&self, node: &YamlPlainScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlPlainScalarFields { value_token } = node.as_fields();
        write!(
            f,
            [FormatFlowScalar {
                token: &value_token?
            }]
        )
    }
}
