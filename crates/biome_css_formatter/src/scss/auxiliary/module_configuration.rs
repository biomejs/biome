use crate::prelude::*;
use biome_css_syntax::{ScssModuleConfiguration, ScssModuleConfigurationFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssModuleConfiguration;

impl FormatNodeRule<ScssModuleConfiguration> for FormatScssModuleConfiguration {
    fn fmt_fields(&self, node: &ScssModuleConfiguration, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssModuleConfigurationFields {
            name,
            colon_token,
            value,
            modifier,
        } = node.as_fields();

        write!(
            f,
            [name.format(), colon_token.format(), space(), value.format()]
        )?;

        if let Some(modifier) = modifier {
            write!(f, [space(), modifier.format()])?;
        }

        Ok(())
    }
}
