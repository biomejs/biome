use crate::{prelude::*, ts::bindings::type_parameters::FormatTsTypeParametersOptions};
use biome_formatter::{format_args, write};
use biome_js_syntax::AnyJsClass;

pub struct FormatClass<'a> {
    class: &'a AnyJsClass,
}

impl FormatClass<'_> {
    fn should_group(&self, comments: &JsComments) -> FormatResult<bool> {
        if let Some(id) = self.class.id() {
            if comments.has_trailing_comments(id.syntax()) {
                return Ok(true);
            }
        }

        if let Some(type_parameters) = self.class.type_parameters() {
            if comments.has_trailing_comments(type_parameters.syntax()) {
                return Ok(true);
            }
        }

        if let Some(extends) = self.class.extends_clause() {
            if comments.has_trailing_comments(extends.syntax()) {
                return Ok(true);
            }
        }

        if self.class.implements_clause().is_some() {
            return Ok(true);
        }

        Ok(false)
    }
}

impl<'a> From<&'a AnyJsClass> for FormatClass<'a> {
    fn from(class: &'a AnyJsClass) -> Self {
        Self { class }
    }
}

impl Format<JsFormatContext> for FormatClass<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let decorators = self.class.decorators();
        let abstract_token = self.class.abstract_token();
        let id = self.class.id();
        let extends = self.class.extends_clause();
        let implements_clause = self.class.implements_clause();
        let type_parameters = self.class.type_parameters();
        let class_token = self.class.class_token()?;
        let members = self.class.members();

        let group_mode = self.should_group(f.comments())?;

        write!(f, [decorators.format()])?;

        if let Some(abstract_token) = abstract_token {
            write!(f, [abstract_token.format(), space()])?;
        }

        write!(f, [class_token.format()])?;

        let indent_only_heritage = type_parameters.as_ref().is_some_and(|type_parameters| {
            !f.comments()
                .has_trailing_line_comment(type_parameters.syntax())
        }) && !(extends.is_some() && implements_clause.is_some());

        let type_parameters_id = if indent_only_heritage && implements_clause.is_some() {
            Some(f.group_id("type_parameters"))
        } else {
            None
        };

        let head = format_with(|f| {
            if let Some(id) = &id {
                write!(f, [space(), id.format()])?;
            }

            if let Some(type_parameters) = &type_parameters {
                write!(
                    f,
                    [type_parameters
                        .format()
                        .with_options(FormatTsTypeParametersOptions {
                            group_id: type_parameters_id,
                            is_type_or_interface_decl: false
                        })]
                )?;
            }

            Ok(())
        });

        let format_heritage_clauses = format_with(|f| {
            if let Some(extends) = &extends {
                if group_mode {
                    write!(f, [soft_line_break_or_space(), group(&extends.format())])?;
                } else {
                    write!(f, [space(), extends.format()])?;
                }
            }

            if let Some(implements_clause) = &implements_clause {
                if indent_only_heritage {
                    write!(
                        f,
                        [
                            if_group_breaks(&space()).with_group_id(type_parameters_id),
                            if_group_fits_on_line(&soft_line_break_or_space())
                                .with_group_id(type_parameters_id)
                        ]
                    )?;
                } else {
                    write!(f, [soft_line_break_or_space()])?;
                }

                write!(f, [implements_clause.format()])?;
            }

            Ok(())
        });

        if group_mode {
            let indented = format_with(|f| {
                if indent_only_heritage {
                    write!(f, [head, indent(&format_heritage_clauses)])
                } else {
                    write!(f, [indent(&format_args![head, format_heritage_clauses])])
                }
            });

            let heritage_id = f.group_id("heritageGroup");
            write!(
                f,
                [group(&indented).with_group_id(Some(heritage_id)), space()]
            )?;

            if !members.is_empty() {
                write!(
                    f,
                    [if_group_breaks(&hard_line_break()).with_group_id(Some(heritage_id))]
                )?;
            }
        } else {
            write!(f, [head, format_heritage_clauses, space()])?;
        }

        if members.is_empty() {
            write!(
                f,
                [
                    self.class.l_curly_token().format(),
                    format_dangling_comments(self.class.syntax()).with_block_indent(),
                    self.class.r_curly_token().format()
                ]
            )
        } else {
            write![
                f,
                [
                    self.class.l_curly_token().format(),
                    block_indent(&members.format()),
                    self.class.r_curly_token().format()
                ]
            ]
        }
    }
}
