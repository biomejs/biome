use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_graphql_syntax::{
    GraphqlSyntaxToken, GraphqlUnionMemberTypes, GraphqlUnionMemberTypesFields,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlUnionMemberTypes;
impl FormatNodeRule<GraphqlUnionMemberTypes> for FormatGraphqlUnionMemberTypes {
    fn fmt_fields(
        &self,
        node: &GraphqlUnionMemberTypes,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlUnionMemberTypesFields {
            eq_token,
            bitwise_or_token,
            members,
        } = node.as_fields();

        write!(
            f,
            [
                space(),
                eq_token.format(),
                if_group_fits_on_line(&space()),
                soft_block_indent(&format_args![
                    FormatTypeLeadingSeparator {
                        separator: "|",
                        leading_separator: bitwise_or_token.as_ref(),
                    },
                    members.format(),
                ])
            ]
        )
    }
}

pub struct FormatTypeLeadingSeparator<'a> {
    separator: &'static str,
    leading_separator: Option<&'a GraphqlSyntaxToken>,
}

impl Format<GraphqlFormatContext> for FormatTypeLeadingSeparator<'_> {
    fn fmt(&self, f: &mut GraphqlFormatter) -> FormatResult<()> {
        match &self.leading_separator {
            Some(token) => {
                let content = format_with(|f| {
                    write!(f, [soft_line_break_or_space(), token.format(), space()])
                });
                write!(f, [format_only_if_breaks(token, &content)])
            }
            None => {
                let content = format_with(|f| {
                    write!(
                        f,
                        [soft_line_break_or_space(), text(self.separator), space()]
                    )
                });

                write!(f, [if_group_breaks(&content)])
            }
        }
    }
}
