use super::*;
use crate::format_semantic_model::FormatSemanticModelContext;
use biome_js_syntax::{TextRange, TsTypeParameterName, binding_ext::AnyJsIdentifierBinding};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

/// Internal type with all the semantic data of a specific binding
#[derive(Debug)]
pub(crate) struct SemanticModelBindingData {
    pub(crate) range: TextRange,
    pub(crate) references: Vec<SemanticModelReference>,
    // We use a SmallVec because most of the time a binding is expected once.
    pub(crate) export_by_start: smallvec::SmallVec<[TextSize; 4]>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum SemanticModelReferenceType {
    Read { hoisted: bool },
    Write { hoisted: bool },
}

/// Internal type with all the semantic data of a specific reference
#[derive(Debug)]
pub(crate) struct SemanticModelReference {
    pub(crate) range_start: TextSize,
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
    pub(crate) data: Arc<SemanticModelData>,
    pub(crate) id: BindingId,
}

impl std::fmt::Debug for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Binding").field("id", &self.id).finish()
    }
}

impl Display for Binding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let formatted = biome_formatter::format!(FormatSemanticModelContext, [&self])
            .expect("Formatting not to throw any FormatErrors");
        f.write_str(
            formatted
                .print()
                .expect("Expected a valid document")
                .as_code(),
        )
    }
}

impl Binding {
    /// Returns the scope of this binding
    pub fn scope(&self) -> Scope {
        let binding = self.data.binding(self.id);
        Scope {
            data: self.data.clone(),
            id: self.data.scope(binding.range),
        }
    }

    /// Returns the syntax node associated with this binding.
    pub fn syntax(&self) -> JsSyntaxNode {
        let binding = self.data.binding(self.id);
        self.data.binding_node_by_start[&binding.range.start()]
            .to_node(self.data.to_root().syntax())
    }

    /// Returns the typed AST node associated with this binding.
    pub fn tree(&self) -> AnyJsIdentifierBinding {
        AnyJsIdentifierBinding::unwrap_cast(self.syntax().clone())
    }

    /// Returns an iterator to all references of this binding.
    pub fn all_references(&self) -> AllBindingReferencesIter {
        let binding = self.data.binding(self.id);
        let first = if binding.references.is_empty() {
            None
        } else {
            Some(Reference {
                data: self.data.clone(),
                id: ReferenceId::new(self.id, 0),
            })
        };
        std::iter::successors(first, Reference::find_next)
    }

    /// Returns an iterator to all reads references of this binding.
    pub fn all_reads(&self) -> AllBindingReadReferencesIter {
        let binding = self.data.binding(self.id);
        let first = binding
            .references
            .iter()
            .enumerate()
            .find(|(_, x)| x.is_read())
            .map(|(index, _)| Reference {
                data: self.data.clone(),
                id: ReferenceId::new(self.id, index),
            });
        std::iter::successors(first, Reference::find_next_read)
    }

    /// Returns an iterator to all write references of this binding.
    pub fn all_writes(&self) -> AllBindingWriteReferencesIter {
        let binding = self.data.binding(self.id);
        let first = binding
            .references
            .iter()
            .enumerate()
            .find(|(_, x)| x.is_write())
            .map(|(index, _)| Reference {
                data: self.data.clone(),
                id: ReferenceId::new(self.id, index),
            });
        std::iter::successors(first, Reference::find_next_write)
    }

    /// Returns all exports of the binding.
    ///
    /// The node kind is either an identifier binding (if the declaration is
    /// itself an `export` statement) or an identifier usage.
    pub fn exports(&self) -> impl Iterator<Item = JsSyntaxNode> + '_ {
        let binding = self.data.binding(self.id);
        binding.export_by_start.iter().map(|export_start| {
            self.data.binding_node_by_start[export_start].to_node(self.data.to_root().syntax())
        })
    }

    pub fn is_imported(&self) -> bool {
        super::is_imported(&self.syntax())
    }
}

impl PartialEq for Binding {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Binding {}
impl std::hash::Hash for Binding {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
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
