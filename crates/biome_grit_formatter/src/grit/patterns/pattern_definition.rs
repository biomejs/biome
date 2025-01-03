use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_grit_syntax::{GritPatternDefinition, GritPatternDefinitionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternDefinition;
impl FormatNodeRule<GritPatternDefinition> for FormatGritPatternDefinition {
    fn fmt_fields(&self, node: &GritPatternDefinition, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPatternDefinitionFields {
            visibility_token,
            pattern_token,
            name,
            l_paren_token,
            args,
            r_paren_token,
            language,
            body,
        } = node.as_fields();

        if let Some(visibility) = visibility_token {
            write!(f, [visibility.format(), space()])?;
        }
        write!(
            f,
            [
                pattern_token.format(),
                space(),
                name.format(),
                l_paren_token.format(),
                group(&format_args![args.format()]),
                r_paren_token.format(),
                space()
            ]
        )?;

        if let Some(language) = language {
            write!(f, [language.format(), space()])?;
        }

        write!(f, [body.format()])
    }
}
