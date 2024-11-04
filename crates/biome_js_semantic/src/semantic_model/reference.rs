use biome_js_syntax::{AnyJsFunction, AnyJsIdentifierUsage, JsCallExpression};

use super::*;
use std::rc::Rc;

/// Provides all information regarding to a specific reference.
#[derive(Debug)]
pub struct Reference {
    pub(crate) data: Rc<SemanticModelData>,
    pub(crate) id: ReferenceId,
}

impl Reference {
    pub(crate) fn find_next(&self) -> Option<Reference> {
        let id = self.data.next_reference(self.id)?;
        Some(Reference {
            data: self.data.clone(),
            id,
        })
    }

    pub(crate) fn find_next_read(&self) -> Option<Reference> {
        let references = &self.data.binding(self.id.binding_id()).references;
        let mut index = self.id.index() + 1;
        while index < references.len() {
            if references[index].is_read() {
                return Some(Reference {
                    data: self.data.clone(),
                    id: ReferenceId::new(self.id.binding_id(), index),
                });
            } else {
                index += 1;
            }
        }
        None
    }

    pub(crate) fn find_next_write(&self) -> Option<Reference> {
        let references = &self.data.binding(self.id.binding_id()).references;
        let mut index = self.id.index() + 1;
        while index < references.len() {
            if references[index].is_write() {
                return Some(Reference {
                    data: self.data.clone(),
                    id: ReferenceId::new(self.id.binding_id(), index),
                });
            } else {
                index += 1;
            }
        }
        None
    }

    /// Returns the range of this reference
    pub fn range_start(&self) -> TextSize {
        self.data.reference(self.id).range_start
    }

    /// Returns the scope of this reference
    pub fn scope(&self) -> Scope {
        let start = self.range_start();
        let id = self.data.scope(TextRange::new(
            start,
            // SAFETY: A reference name has at least a length of 1 byte.
            start + TextSize::from(1),
        ));
        Scope {
            data: self.data.clone(),
            id,
        }
    }

    /// Returns the node of this reference
    pub fn syntax(&self) -> &JsSyntaxNode {
        &self.data.binding_node_by_start[&self.range_start()]
    }

    /// Returns the binding of this reference
    pub fn binding(&self) -> Option<Binding> {
        Some(Binding {
            data: self.data.clone(),
            id: self.id.binding_id(),
        })
    }

    /// Returns if the declaration of this reference is hoisted or not
    pub fn is_using_hoisted_declaration(&self) -> bool {
        let reference = &self.data.reference(self.id);
        match reference.ty {
            SemanticModelReferenceType::Read { hoisted } => hoisted,
            SemanticModelReferenceType::Write { hoisted } => hoisted,
        }
    }

    /// Returns if this reference is just reading its binding
    pub fn is_read(&self) -> bool {
        let reference = self.data.reference(self.id);
        matches!(reference.ty, SemanticModelReferenceType::Read { .. })
    }

    /// Returns if this reference is writing its binding
    pub fn is_write(&self) -> bool {
        let reference = self.data.reference(self.id);
        matches!(reference.ty, SemanticModelReferenceType::Write { .. })
    }

    /// Returns this reference as a [FunctionCall] if possible
    pub fn as_call(&self) -> Option<FunctionCall> {
        let call = self.syntax().ancestors().find(|x| {
            !matches!(
                x.kind(),
                JsSyntaxKind::JS_REFERENCE_IDENTIFIER | JsSyntaxKind::JS_IDENTIFIER_EXPRESSION
            )
        });

        match call {
            Some(node) if node.kind() == JsSyntaxKind::JS_CALL_EXPRESSION => Some(FunctionCall {
                data: self.data.clone(),
                id: self.id,
            }),
            _ => None,
        }
    }
}

/// Provides all information regarding a specific function or method call.
#[derive(Debug)]
pub struct FunctionCall {
    pub(crate) data: Rc<SemanticModelData>,
    pub(crate) id: ReferenceId,
}

impl FunctionCall {
    /// Returns the start of the range of this reference
    pub fn range_start(&self) -> TextSize {
        self.data.reference(self.id).range_start
    }

    /// Returns the node of this reference
    pub fn syntax(&self) -> &JsSyntaxNode {
        &self.data.binding_node_by_start[&self.range_start()]
    }

    /// Returns the typed AST node of this reference
    pub fn tree(&self) -> JsCallExpression {
        let node = self.syntax();
        let call = node.ancestors().find(|x| {
            !matches!(
                x.kind(),
                JsSyntaxKind::JS_REFERENCE_IDENTIFIER | JsSyntaxKind::JS_IDENTIFIER_EXPRESSION
            )
        });
        debug_assert!(matches!(&call,
            Some(call) if call.kind() == JsSyntaxKind::JS_CALL_EXPRESSION
        ));
        JsCallExpression::unwrap_cast(call.unwrap())
    }
}

pub struct AllCallsIter {
    pub(crate) references: AllBindingReadReferencesIter,
}

impl Iterator for AllCallsIter {
    type Item = FunctionCall;

    fn next(&mut self) -> Option<Self::Item> {
        for reference in self.references.by_ref() {
            if let Some(call) = reference.as_call() {
                return Some(call);
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct SemanticModelUnresolvedReference {
    pub(crate) range: TextRange,
}

#[derive(Debug)]
pub struct UnresolvedReference {
    pub(crate) data: Rc<SemanticModelData>,
    pub(crate) id: u32,
}

impl UnresolvedReference {
    pub fn syntax(&self) -> &JsSyntaxNode {
        let reference = &self.data.unresolved_reference(self.id);
        &self.data.binding_node_by_start[&reference.range.start()]
    }

    pub fn tree(&self) -> AnyJsIdentifierUsage {
        AnyJsIdentifierUsage::unwrap_cast(self.syntax().clone())
    }

    pub fn range(&self) -> TextRange {
        self.data.unresolved_reference(self.id).range
    }
}

/// Marker trait that groups all "AstNode" that have declarations
pub trait HasDeclarationAstNode: AstNode<Language = JsLanguage> {
    #[inline(always)]
    fn node(&self) -> &Self {
        self
    }
}

impl HasDeclarationAstNode for JsReferenceIdentifier {}
impl HasDeclarationAstNode for JsIdentifierAssignment {}
impl HasDeclarationAstNode for JsxReferenceIdentifier {}

/// Extension method to allow any node that is a declaration to easily
/// get all of its references.
pub trait ReferencesExtensions {
    fn all_references(&self, model: &SemanticModel) -> AllBindingReferencesIter
    where
        Self: IsBindingAstNode,
    {
        model.as_binding(self).all_references()
    }

    fn all_reads(&self, model: &SemanticModel) -> AllBindingReadReferencesIter
    where
        Self: IsBindingAstNode,
    {
        model.as_binding(self).all_reads()
    }

    fn all_writes(&self, model: &SemanticModel) -> AllBindingWriteReferencesIter
    where
        Self: IsBindingAstNode,
    {
        model.as_binding(self).all_writes()
    }
}

impl<T: IsBindingAstNode> ReferencesExtensions for T {}

pub trait CallsExtensions {
    fn all_calls(&self, model: &SemanticModel) -> Option<AllCallsIter>;
}

impl CallsExtensions for AnyJsFunction {
    fn all_calls(&self, model: &SemanticModel) -> Option<AllCallsIter> {
        model.all_calls(self)
    }
}
