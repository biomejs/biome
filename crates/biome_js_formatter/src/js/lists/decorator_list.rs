use crate::prelude::*;
use crate::utils::format_modifiers::should_expand_decorators;
use biome_formatter::write;
use biome_js_syntax::JsSyntaxKind::{
    JS_CLASS_EXPRESSION, JS_FORMAL_PARAMETER, JS_REST_PARAMETER, TS_PROPERTY_PARAMETER,
};
use biome_js_syntax::{
    AnyJsDeclarationClause, AnyJsExportClause, AnyJsExportDefaultDeclaration, JsDecoratorList,
    JsExport,
};
use biome_rowan::SyntaxNodeOptionExt;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsDecoratorList;
impl FormatRule<JsDecoratorList> for FormatJsDecoratorList {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsDecoratorList, f: &mut JsFormatter) -> FormatResult<()> {
        if node.is_empty() {
            return Ok(());
        }

        // we need to rearrange decorators to be before export if we have decorators before class and after export
        if let Some(export) = node.parent::<JsExport>() {
            let mut join = f.join_nodes_with_hardline();

            // write decorators before export first
            for decorator in node {
                join.entry(decorator.syntax(), &format_or_verbatim(decorator.format()));
            }

            // try to find class decorators
            let class_decorators = match export.export_clause()? {
                AnyJsExportClause::AnyJsDeclarationClause(
                    AnyJsDeclarationClause::JsClassDeclaration(class),
                ) => {
                    // @before export @after class Foo {}
                    Some(class.decorators())
                }
                AnyJsExportClause::JsExportDefaultDeclarationClause(export_default_declaration) => {
                    match export_default_declaration.declaration()? {
                        AnyJsExportDefaultDeclaration::JsClassExportDefaultDeclaration(class) => {
                            // @before export default @after class Foo {}
                            Some(class.decorators())
                        }
                        _ => None,
                    }
                }
                _ => None,
            };

            // write decorators after export
            if let Some(class_decorators) = class_decorators {
                for decorator in class_decorators {
                    join.entry(decorator.syntax(), &format_or_verbatim(decorator.format()));
                }
            }

            join.finish()?;

            write!(f, [hard_line_break()])
        } else if matches!(node.syntax().parent().kind(), Some(JS_CLASS_EXPRESSION)) {
            write!(f, [expand_parent()])?;
            f.join_with(&soft_line_break_or_space())
                .entries(node.iter().formatted())
                .finish()?;

            write!(f, [soft_line_break_or_space()])
        } else {
            let is_parameter_decorators = matches!(
                node.syntax().parent().kind(),
                Some(JS_FORMAL_PARAMETER | JS_REST_PARAMETER | TS_PROPERTY_PARAMETER)
            );

            if is_parameter_decorators {
                let should_expand = should_expand_decorators(node);

                if should_expand {
                    write!(f, [expand_parent()])?;
                }
            } else {
                // If the parent node is an export declaration and the decorator
                // was written before the export, the export will be responsible
                // for printing the decorators.
                let export = node.syntax().grand_parent().and_then(|grand_parent| {
                    JsExport::cast_ref(&grand_parent)
                        .or_else(|| grand_parent.parent().and_then(JsExport::cast))
                });
                let is_export = export.is_some();

                let has_decorators_before_export =
                    export.is_some_and(|export| !export.decorators().is_empty());

                if has_decorators_before_export {
                    return Ok(());
                }

                if is_export {
                    write!(f, [hard_line_break()])?;
                } else {
                    write!(f, [expand_parent()])?;
                }
            }

            f.join_with(&soft_line_break_or_space())
                .entries(node.iter().formatted())
                .finish()?;

            write!(f, [soft_line_break_or_space()])
        }
    }
}
