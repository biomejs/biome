use crate::{prelude::*, ts::bindings::type_parameters::FormatTsTypeParametersOptions};

use biome_formatter::{format_args, write};
use biome_js_syntax::{TsInterfaceDeclaration, TsInterfaceDeclarationFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsInterfaceDeclaration;

impl FormatNodeRule<TsInterfaceDeclaration> for FormatTsInterfaceDeclaration {
    fn fmt_fields(&self, node: &TsInterfaceDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let TsInterfaceDeclarationFields {
            interface_token,
            id,
            type_parameters,
            extends_clause,
            members,
            l_curly_token,
            r_curly_token,
        } = node.as_fields();

        let l_curly_token = l_curly_token?;
        let r_curly_token = r_curly_token?;
        let id = id?;

        let should_indent_extends_only = type_parameters
            .as_ref()
            .is_some_and(|params| !f.comments().has_trailing_line_comment(params.syntax()));

        let type_parameter_group = if should_indent_extends_only && extends_clause.is_some() {
            Some(f.group_id("type_parameters"))
        } else {
            None
        };

        let format_id = format_with(|f| {
            write!(f, [id.format(),])?;

            if let Some(type_parameters) = &type_parameters {
                write!(
                    f,
                    [type_parameters
                        .format()
                        .with_options(FormatTsTypeParametersOptions {
                            group_id: type_parameter_group,
                            is_type_or_interface_decl: true
                        })]
                )?;
            }

            Ok(())
        });

        let format_extends = format_with(|f| {
            if let Some(extends_clause) = &extends_clause {
                if should_indent_extends_only {
                    write!(
                        f,
                        [
                            if_group_breaks(&space()).with_group_id(type_parameter_group),
                            if_group_fits_on_line(&soft_line_break_or_space())
                                .with_group_id(type_parameter_group),
                        ]
                    )?;
                } else {
                    write!(f, [soft_line_break_or_space()])?;
                }

                write!(f, [extends_clause.format(), space()])?;
            }

            Ok(())
        });

        let content = format_with(|f| {
            write![f, [interface_token.format(), space()]]?;

            let id_has_trailing_comments = f.comments().has_trailing_comments(id.syntax());
            if id_has_trailing_comments || extends_clause.is_some() {
                if should_indent_extends_only {
                    write!(
                        f,
                        [group(&format_args!(format_id, indent(&format_extends)))]
                    )?;
                } else {
                    write!(
                        f,
                        [group(&indent(&format_args!(format_id, format_extends)))]
                    )?;
                }
            } else {
                write!(f, [format_id, format_extends])?;
            }

            write!(f, [space(), l_curly_token.format()])?;

            if members.is_empty() {
                write!(
                    f,
                    [format_dangling_comments(node.syntax()).with_block_indent()]
                )?;
            } else {
                write!(f, [block_indent(&members.format())])?;
            }

            write!(f, [r_curly_token.format()])
        });

        write![f, [group(&content)]]
    }

    fn fmt_dangling_comments(
        &self,
        _: &TsInterfaceDeclaration,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Handled inside of `fmt_fields`
        Ok(())
    }
}
