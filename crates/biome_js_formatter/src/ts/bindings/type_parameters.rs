use crate::prelude::*;
use biome_formatter::FormatError::SyntaxError;
use biome_formatter::{format_args, write, FormatRuleWithOptions, GroupId};
use biome_js_syntax::{TsTypeParameters, TsTypeParametersFields};

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct FormatTsTypeParametersOptions {
    pub(crate) group_id: Option<GroupId>,
    pub(crate) is_type_or_interface_decl: bool,
}
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsTypeParameters {
    options: FormatTsTypeParametersOptions,
}

impl FormatRuleWithOptions<TsTypeParameters> for FormatTsTypeParameters {
    type Options = FormatTsTypeParametersOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}

impl FormatNodeRule<TsTypeParameters> for FormatTsTypeParameters {
    fn fmt_fields(&self, node: &TsTypeParameters, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeParametersFields {
            items,
            r_angle_token,
            l_angle_token,
        } = node.as_fields();

        if items.is_empty() && self.options.is_type_or_interface_decl {
            write!(f, [l_angle_token.format(), r_angle_token.format()])
        } else if items.is_empty() {
            return Err(SyntaxError);
        } else {
            write!(
                f,
                [group(&format_args![
                    l_angle_token.format(),
                    soft_block_indent(&items.format()),
                    r_angle_token.format()
                ])
                .with_group_id(self.options.group_id)]
            )
        }
    }
}
