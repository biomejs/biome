use super::*;
use rome_js_syntax::{binding_ext::AnyJsIdentifierBinding, TextRange, TsTypeParameterName};
use std::rc::Rc;

/// Internal type with all the semantic data of a specific binding
#[derive(Debug)]
pub(crate) struct SemanticModelBindingData {
    pub id: BindingIndex,
    pub range: TextRange,
    pub references: Vec<SemanticModelReference>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum SemanticModelReferenceType {
    Read { hoisted: bool },
    Write { hoisted: bool },
}

/// Internal type with all the semantic data of a specific reference
#[derive(Debug)]
pub(crate) struct SemanticModelReference {
    pub(crate) index: ReferenceIndex,
    pub(crate) range: TextRange,
    pub(crate) ty: SemanticModelReferenceType,
}

impl SemanticModelReference {
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        matches!(self.ty, SemanticModelReferenceType::Read { .. })
    }

    #[inline(always)]
    pub fn is_write(&self) -> bool {
        matches!(self.ty, SemanticModelReferenceType::Write { .. })
    }
}

pub type AllBindingReferencesIter =
    std::iter::Successors<Reference, fn(&Reference) -> Option<Reference>>;
pub type AllBindingReadReferencesIter =
    std::iter::Successors<Reference, fn(&Reference) -> Option<Reference>>;
pub type AllBindingWriteReferencesIter =
    std::iter::Successors<Reference, fn(&Reference) -> Option<Reference>>;

/// Provides access to all semantic data of a specific binding.
pub struct Binding {
    pub(crate) data: Rc<SemanticModelData>,
    pub(crate) index: BindingIndex,
}

impl std::fmt::Debug for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Binding").field("id", &self.index).finish()
    }
}

impl Binding {
    /// Returns the scope of this binding
    pub fn scope(&self) -> Scope {
        let binding = self.data.binding(self.index);
        let id = self.data.scope(&binding.range); //TODO declaration can have its scope id
        Scope {
            data: self.data.clone(),
            id,
        }
    }

    /// Returns the syntax node associated with this binding.
    pub fn syntax(&self) -> &JsSyntaxNode {
        let binding = self.data.binding(self.index);
        &self.data.node_by_range[&binding.range]
    }

    /// Returns the typed AST node associated with this binding.
    pub fn tree(&self) -> AnyJsIdentifierBinding {
        let node = self.syntax();
        let binding = AnyJsIdentifierBinding::cast_ref(node);
        debug_assert!(binding.is_some());
        binding.unwrap()
    }

    /// Returns an iterator to all references of this binding.
    pub fn all_references(&self) -> AllBindingReferencesIter {
        let binding = self.data.binding(self.index);
        let first = binding.references.first().map(|reference| Reference {
            data: self.data.clone(),
            index: reference.index,
        });
        std::iter::successors(first, Reference::find_next)
    }

    /// Returns an iterator to all reads references of this binding.
    pub fn all_reads(&self) -> AllBindingReadReferencesIter {
        let binding = self.data.binding(self.index);
        let first = binding
            .references
            .iter()
            .find(|x| x.is_read())
            .map(|reference| Reference {
                data: self.data.clone(),
                index: reference.index,
            });
        std::iter::successors(first, Reference::find_next_read)
    }

    /// Returns an iterator to all write references of this binding.
    pub fn all_writes(&self) -> AllBindingWriteReferencesIter {
        let binding = self.data.binding(self.index);
        let first = binding
            .references
            .iter()
            .find(|x| x.is_write())
            .map(|reference| Reference {
                data: self.data.clone(),
                index: reference.index,
            });
        std::iter::successors(first, Reference::find_next_write)
    }

    pub fn is_imported(&self) -> bool {
        super::is_imported(self.syntax())
    }
}

/// Marker trait that groups all "AstNode" that are bindings
pub trait IsBindingAstNode: AstNode<Language = JsLanguage> {
    #[inline(always)]
    fn node(&self) -> &Self {
        self
    }
}

impl IsBindingAstNode for JsIdentifierBinding {}
impl IsBindingAstNode for TsIdentifierBinding {}
impl IsBindingAstNode for AnyJsIdentifierBinding {}
impl IsBindingAstNode for TsTypeParameterName {}

/// Extension method to allow nodes that have declaration to easily
/// get its binding.
pub trait BindingExtensions {
    /// Returns the [Binding] that declared the symbol this reference references.
    fn binding(&self, model: &SemanticModel) -> Option<Binding>
    where
        Self: HasDeclarationAstNode,
    {
        model.binding(self)
    }
}

impl<T: HasDeclarationAstNode> BindingExtensions for T {}
