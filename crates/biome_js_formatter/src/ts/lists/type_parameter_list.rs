use crate::context::trailing_commas::FormatTrailingCommas;
use crate::prelude::*;
use biome_js_syntax::{JsSyntaxKind, TsTypeParameterList};
use biome_rowan::{AstSeparatedList, SyntaxNodeOptionExt};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeParameterList;

impl FormatRule<TsTypeParameterList> for FormatTsTypeParameterList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsTypeParameterList, f: &mut JsFormatter) -> FormatResult<()> {
        // Type parameter lists of arrow function expressions have to include at least one comma
        // to avoid any ambiguity with JSX elements.
        // Thus, we have to add a trailing comma when there is a single type parameter.
        // The comma can be omitted in the case where the single parameter has a constraint,
        // i.i. an `extends` clause.
        let trailing_separator = if node.len() == 1
            // This only concern sources that allow JSX or a restricted standard variant.
            && !f.options().source_type().variant().is_standard()
            && node.syntax().grand_parent().kind()
                == Some(JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION)
            // Ignore Type parameter with an `extends` clause or a default type.
            && !node.first().and_then(|param| param.ok())
                .is_some_and(|type_parameter| type_parameter.constraint().is_some() || type_parameter.default().is_some())
        {
            TrailingSeparator::Mandatory
        } else {
            FormatTrailingCommas::ES5.trailing_separator(f.options())
        };

        f.join_with(&soft_line_break_or_space())
            .entries(
                node.format_separated(",")
                    .with_trailing_separator(trailing_separator),
            )
            .finish()
    }
}
