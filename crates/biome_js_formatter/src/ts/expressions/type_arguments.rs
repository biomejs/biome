use crate::utils::should_hug_type;
use crate::{prelude::*, utils::is_object_like_type};
use biome_formatter::FormatError::SyntaxError;
use biome_formatter::{format_args, write};
use biome_js_syntax::{
    AnyJsExpression, AnyTsType, JsSyntaxKind, JsVariableDeclarator, TsTypeArguments,
    TsTypeArgumentsFields,
};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeArguments;

impl FormatNodeRule<TsTypeArguments> for FormatTsTypeArguments {
    fn fmt_fields(&self, node: &TsTypeArguments, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeArgumentsFields {
            l_angle_token,
            ts_type_argument_list,
            r_angle_token,
        } = node.as_fields();

        if ts_type_argument_list.is_empty() {
            return Err(SyntaxError);
        }

        // We want to check if we are inside something like this:
        // const foo: SomeThing<{ [P in "x" | "y"]: number }> = func();
        //
        //                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //                      |_________________________|
        //                      that's where we start from
        let is_arrow_function_variables = {
            match ts_type_argument_list.first() {
                // first argument is not mapped type or object type
                Some(Ok(ty)) if is_object_like_type(&ty) && ts_type_argument_list.len() == 1 => {
                    false
                }
                Some(Ok(ty)) => {
                    // we then go up until we can find a potential type annotation,
                    // meaning four levels up
                    let maybe_type_annotation = ty.syntax().ancestors().nth(4);

                    let initializer = maybe_type_annotation
                        .and_then(|maybe_type_annotation| {
                            if maybe_type_annotation.kind() == JsSyntaxKind::TS_TYPE_ANNOTATION {
                                maybe_type_annotation.parent()
                            } else {
                                None
                            }
                        })
                        // is so, we try to cast the parent into a variable declarator
                        .and_then(JsVariableDeclarator::cast)
                        // we extract the initializer
                        .and_then(|variable_declarator| variable_declarator.initializer());

                    if let Some(initializer) = initializer {
                        // we verify if we have an arrow function expression
                        let expression = initializer.expression()?;
                        matches!(expression, AnyJsExpression::JsArrowFunctionExpression(_))
                    } else {
                        false
                    }
                }

                _ => false,
            }
        };

        let first_argument_can_be_hugged_or_is_null_type = match ts_type_argument_list.first() {
            _ if ts_type_argument_list.len() != 1 => false,
            Some(Ok(AnyTsType::TsNullLiteralType(_))) => true,
            Some(Ok(ty)) => should_hug_type(&ty, f),
            _ => false,
        };

        let should_inline = !is_arrow_function_variables
            && (ts_type_argument_list.len() == 0 || first_argument_can_be_hugged_or_is_null_type);

        if should_inline {
            write!(
                f,
                [
                    l_angle_token.format(),
                    ts_type_argument_list.format(),
                    r_angle_token.format()
                ]
            )
        } else {
            write!(
                f,
                [group(&format_args![
                    l_angle_token.format(),
                    soft_block_indent(&ts_type_argument_list.format()),
                    r_angle_token.format()
                ])]
            )
        }
    }
}
