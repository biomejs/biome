use super::*;
use crate::{HasDeclarationAstNode, SemanticModel};
use biome_js_syntax::{
    binding_ext::AnyJsIdentifierBinding, JsIdentifierBinding, JsLanguage, JsSyntaxKind,
};
use biome_rowan::AstNode;

pub(crate) fn is_imported(node: &JsSyntaxNode) -> bool {
    node.ancestors().any(|x| {
        matches!(
            x.kind(),
            JsSyntaxKind::JS_IMPORT
                | JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIERS
                | JsSyntaxKind::JS_DEFAULT_IMPORT_SPECIFIER
        )
    })
}

/// Marker trait that groups all "AstNode" that can be imported or
/// exported
pub trait CanBeImportedExported: AstNode<Language = JsLanguage> {
    type Result;
    fn is_exported(&self, model: &SemanticModel) -> Self::Result;
    fn is_imported(&self, model: &SemanticModel) -> Self::Result;
}

impl CanBeImportedExported for JsIdentifierBinding {
    type Result = bool;

    fn is_exported(&self, model: &SemanticModel) -> Self::Result {
        let range = self.syntax().text_trimmed_range();
        model.data.is_exported(range)
    }

    fn is_imported(&self, _: &SemanticModel) -> Self::Result {
        is_imported(self.syntax())
    }
}

impl CanBeImportedExported for TsIdentifierBinding {
    type Result = bool;

    fn is_exported(&self, model: &SemanticModel) -> Self::Result {
        let range = self.syntax().text_trimmed_range();
        model.data.is_exported(range)
    }

    fn is_imported(&self, _: &SemanticModel) -> Self::Result {
        is_imported(self.syntax())
    }
}

impl CanBeImportedExported for AnyJsIdentifierBinding {
    type Result = bool;

    fn is_exported(&self, model: &SemanticModel) -> Self::Result {
        let range = self.syntax().text_trimmed_range();
        model.data.is_exported(range)
    }

    fn is_imported(&self, _: &SemanticModel) -> Self::Result {
        is_imported(self.syntax())
    }
}

impl<T: HasDeclarationAstNode> CanBeImportedExported for T {
    type Result = Option<bool>;

    fn is_exported(&self, model: &SemanticModel) -> Self::Result {
        let range = self.binding(model)?.syntax().text_trimmed_range();
        Some(model.data.is_exported(range))
    }

    fn is_imported(&self, model: &SemanticModel) -> Self::Result {
        let binding = self.binding(model)?;
        let node = binding.syntax();
        Some(is_imported(node))
    }
}
