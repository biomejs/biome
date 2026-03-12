use super::*;
use crate::format_semantic_model::FormatSemanticModelContext;
use biome_js_syntax::{
    AnyJsDeclaration, JsImport, JsVariableKind, TextRange, TsTypeParameter, TsTypeParameterName,
    binding_ext::AnyJsIdentifierBinding,
};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

/// The kind of JavaScript/TypeScript declaration.
///
/// This categorizes bindings by how they were declared, which determines
/// their hoisting behavior, whether they declare a type, a value, or both,
/// and other semantic properties.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum JsDeclarationKind {
    /// A `class` declaration.
    ///
    /// Declares both a type and a value.
    Class,

    /// An `enum` declaration.
    ///
    /// Declares both a type and a value.
    Enum,

    /// A generic type parameter, declared in angle brackets.
    ///
    /// For example: `<T>`.
    ///
    /// Declares only a type.
    Generic,

    /// A `function` or `var` declaration.
    ///
    /// Declares only a value, and is hoisted to the function scope.
    HoistedValue,

    /// An `import` declaration.
    ///
    /// Declares both a type and a value.
    Import,

    /// An `import type` declaration.
    ///
    /// Declares only a type.
    ImportType,

    /// An interface declaration.
    ///
    /// Declares only a type.
    Interface,

    /// A module declaration (`declare module "foo"`).
    Module,

    /// A namespace declaration (`namespace Foo { ... }`).
    ///
    /// Declares both a namespace and a value.
    Namespace,

    /// A type declaration (`type Foo = ...`).
    ///
    /// Declares only a type.
    Type,

    /// A bogus declaration or a declaration kind we don't handle yet.
    #[default]
    Unknown,

    /// A `using` declaration.
    ///
    /// Declares only a value.
    Using,

    /// A `let` or `const` declaration.
    ///
    /// Declares only a value.
    Value,
}

impl JsDeclarationKind {
    /// Returns whether this declaration declares a namespace.
    #[inline]
    pub fn declares_namespace(&self) -> bool {
        matches!(self, Self::Namespace)
    }

    /// Returns whether this declaration declares a type.
    #[inline]
    pub fn declares_type(&self) -> bool {
        matches!(
            self,
            Self::Class
                | Self::Enum
                | Self::Generic
                | Self::Import
                | Self::ImportType
                | Self::Interface
                | Self::Type
                | Self::Unknown
        )
    }

    /// Returns whether this declaration declares a runtime value.
    #[inline]
    pub fn declares_value(&self) -> bool {
        matches!(
            self,
            Self::Class
                | Self::Enum
                | Self::HoistedValue
                | Self::Import
                | Self::Namespace
                | Self::Unknown
                | Self::Using
                | Self::Value
        )
    }

    /// Determines the declaration kind by walking up the AST from a binding node.
    pub fn from_node(node: &JsSyntaxNode) -> Self {
        for ancestor in node.ancestors() {
            if TsTypeParameter::can_cast(ancestor.kind()) {
                return Self::Generic;
            }

            if let Some(declaration) = AnyJsDeclaration::cast_ref(&ancestor) {
                return match declaration {
                    AnyJsDeclaration::JsClassDeclaration(_) => Self::Class,
                    AnyJsDeclaration::JsFunctionDeclaration(_) => Self::HoistedValue,
                    AnyJsDeclaration::JsVariableDeclaration(decl) => match decl.variable_kind() {
                        Ok(JsVariableKind::Const | JsVariableKind::Let) => Self::Value,
                        Ok(JsVariableKind::Using) => Self::Using,
                        Ok(JsVariableKind::Var) => Self::HoistedValue,
                        Err(_) => Self::Unknown,
                    },
                    AnyJsDeclaration::TsDeclareFunctionDeclaration(_) => Self::HoistedValue,
                    AnyJsDeclaration::TsEnumDeclaration(_) => Self::Enum,
                    AnyJsDeclaration::TsExternalModuleDeclaration(_) => Self::Module,
                    AnyJsDeclaration::TsInterfaceDeclaration(_) => Self::Interface,
                    AnyJsDeclaration::TsModuleDeclaration(decl) => {
                        if decl
                            .module_or_namespace()
                            .is_ok_and(|token| token.text_trimmed() == "namespace")
                        {
                            Self::Namespace
                        } else {
                            Self::Module
                        }
                    }
                    AnyJsDeclaration::TsTypeAliasDeclaration(_) => Self::Type,
                    AnyJsDeclaration::TsGlobalDeclaration(_)
                    | AnyJsDeclaration::TsImportEqualsDeclaration(_) => Self::Unknown,
                };
            }

            if let Some(import) = JsImport::cast(ancestor) {
                return match import.import_clause() {
                    Ok(import_clause) if import_clause.type_token().is_some() => Self::ImportType,
                    _ => Self::Import,
                };
            }
        }

        Self::Unknown
    }

    /// Returns whether this is an import declaration (either regular or type-only).
    #[inline]
    pub fn is_import_declaration(&self) -> bool {
        matches!(self, Self::Import | Self::ImportType)
    }

    /// Returns whether this is a type-only import declaration.
    #[inline]
    pub fn is_import_type_declaration(&self) -> bool {
        matches!(self, Self::ImportType)
    }
}

/// Reference to one or two bindings.
///
/// Tracks whether the bindings refer to a type or the type of a value.
/// This is necessary because in TypeScript, a single name can refer to
/// up to three different things: a type, a value, and a namespace.
/// For example, a `class` declaration creates both a type and a value.
/// Declaration merging (e.g., `interface Foo` + `const Foo`) creates
/// separate bindings for the type and value slots.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TsBindingReference {
    /// The binding only declares a type.
    Type(BindingId),
    /// The binding only declares a value (the type of the value).
    ValueType(BindingId),
    /// The binding declares both a type and a value (e.g., a class or enum).
    TypeAndValueType(BindingId),
    /// The binding declares both a namespace and a value.
    NamespaceAndValueType(BindingId),
    /// The binding results from declaration merging, with separate
    /// binding IDs for the type, value, and namespace slots.
    Merged {
        ty: Option<BindingId>,
        value_ty: Option<BindingId>,
        namespace_ty: Option<BindingId>,
    },
}

impl TsBindingReference {
    /// Creates a `TsBindingReference` from a binding ID and its declaration kind.
    pub fn from_binding_and_declaration_kind(
        binding_id: BindingId,
        declaration_kind: JsDeclarationKind,
    ) -> Self {
        match (
            declaration_kind.declares_namespace(),
            declaration_kind.declares_type(),
            declaration_kind.declares_value(),
        ) {
            (true, _, _) => Self::NamespaceAndValueType(binding_id),
            (_, true, true) => Self::TypeAndValueType(binding_id),
            (_, true, false) => Self::Type(binding_id),
            (_, false, _) => Self::ValueType(binding_id),
        }
    }

    /// Returns the namespace binding or type binding, in that order.
    ///
    /// Returns `None` if the reference only references a value type.
    pub fn namespace_ty_or_ty(self) -> Option<BindingId> {
        match self {
            Self::Type(binding_id)
            | Self::TypeAndValueType(binding_id)
            | Self::NamespaceAndValueType(binding_id) => Some(binding_id),
            Self::Merged {
                ty, namespace_ty, ..
            } => namespace_ty.or(ty),
            _ => None,
        }
    }

    /// Returns the value type binding, or the type binding if the value type
    /// binding is unknown.
    pub fn value_ty_or_ty(self) -> BindingId {
        match self {
            Self::ValueType(binding_id)
            | Self::TypeAndValueType(binding_id)
            | Self::NamespaceAndValueType(binding_id) => binding_id,
            Self::Merged {
                ty,
                value_ty,
                namespace_ty,
            } => value_ty
                .or(namespace_ty)
                .or(ty)
                .expect("a merged reference must have at least two fields set to `Some`"),
            Self::Type(binding_id) => binding_id,
        }
    }

    /// Creates a union from this binding reference with another.
    ///
    /// If both bindings refer to the same kind of type, the binding ID(s) from
    /// `other` takes precedence.
    pub fn union_with(self, other: Self) -> Self {
        match (self, other) {
            (Self::Type(own_binding_id), Self::ValueType(other_binding_id)) => {
                if own_binding_id == other_binding_id {
                    Self::TypeAndValueType(other_binding_id)
                } else {
                    Self::Merged {
                        ty: Some(own_binding_id),
                        value_ty: Some(other_binding_id),
                        namespace_ty: None,
                    }
                }
            }
            (Self::Type(own_binding_id), Self::NamespaceAndValueType(other_binding_id)) => {
                Self::Merged {
                    ty: Some(own_binding_id),
                    value_ty: Some(other_binding_id),
                    namespace_ty: Some(other_binding_id),
                }
            }
            (
                Self::Type(own_binding_ty),
                Self::Merged {
                    ty,
                    value_ty,
                    namespace_ty,
                },
            ) if ty.is_none() => Self::Merged {
                ty: Some(own_binding_ty),
                value_ty,
                namespace_ty,
            },

            (Self::ValueType(own_binding_id), Self::Type(other_binding_id)) => {
                if own_binding_id == other_binding_id {
                    Self::TypeAndValueType(other_binding_id)
                } else {
                    Self::Merged {
                        ty: Some(other_binding_id),
                        value_ty: Some(own_binding_id),
                        namespace_ty: None,
                    }
                }
            }
            (
                Self::ValueType(own_binding_ty),
                Self::Merged {
                    ty,
                    value_ty,
                    namespace_ty,
                },
            ) if value_ty.is_none() => Self::Merged {
                ty,
                value_ty: Some(own_binding_ty),
                namespace_ty,
            },

            (Self::TypeAndValueType(own_binding_id), Self::Type(other_binding_id)) => {
                Self::Merged {
                    ty: Some(other_binding_id),
                    value_ty: Some(own_binding_id),
                    namespace_ty: None,
                }
            }
            (Self::TypeAndValueType(own_binding_id), Self::ValueType(other_binding_id)) => {
                Self::Merged {
                    ty: Some(own_binding_id),
                    value_ty: Some(other_binding_id),
                    namespace_ty: None,
                }
            }
            (
                Self::TypeAndValueType(own_binding_id),
                Self::NamespaceAndValueType(other_binding_id),
            ) => Self::Merged {
                ty: Some(own_binding_id),
                value_ty: Some(other_binding_id),
                namespace_ty: Some(other_binding_id),
            },
            (
                Self::TypeAndValueType(own_binding_ty),
                Self::Merged {
                    ty,
                    value_ty,
                    namespace_ty,
                },
            ) => Self::Merged {
                ty: ty.or(Some(own_binding_ty)),
                value_ty: value_ty.or(Some(own_binding_ty)),
                namespace_ty,
            },

            (Self::NamespaceAndValueType(own_binding_id), Self::Type(other_binding_id)) => {
                Self::Merged {
                    ty: Some(other_binding_id),
                    value_ty: Some(own_binding_id),
                    namespace_ty: Some(own_binding_id),
                }
            }
            (Self::NamespaceAndValueType(own_binding_id), Self::ValueType(other_binding_id)) => {
                Self::Merged {
                    ty: None,
                    value_ty: Some(other_binding_id),
                    namespace_ty: Some(own_binding_id),
                }
            }
            (
                Self::NamespaceAndValueType(own_binding_id),
                Self::TypeAndValueType(other_binding_id),
            ) => Self::Merged {
                ty: Some(other_binding_id),
                value_ty: Some(other_binding_id),
                namespace_ty: Some(own_binding_id),
            },
            (
                Self::NamespaceAndValueType(own_binding_ty),
                Self::Merged {
                    ty,
                    value_ty,
                    namespace_ty,
                },
            ) => Self::Merged {
                ty,
                value_ty: value_ty.or(Some(own_binding_ty)),
                namespace_ty: namespace_ty.or(Some(own_binding_ty)),
            },

            (
                Self::Merged {
                    value_ty,
                    namespace_ty,
                    ..
                },
                Self::Type(other_binding_ty),
            ) => Self::Merged {
                ty: Some(other_binding_ty),
                value_ty,
                namespace_ty,
            },
            (
                Self::Merged {
                    ty, namespace_ty, ..
                },
                Self::ValueType(other_binding_ty),
            ) => Self::Merged {
                ty,
                value_ty: Some(other_binding_ty),
                namespace_ty,
            },
            (Self::Merged { namespace_ty, .. }, Self::TypeAndValueType(other_binding_ty))
                if namespace_ty.is_some() =>
            {
                Self::Merged {
                    ty: Some(other_binding_ty),
                    value_ty: Some(other_binding_ty),
                    namespace_ty,
                }
            }
            (Self::Merged { ty, .. }, Self::NamespaceAndValueType(other_binding_ty))
                if ty.is_some() =>
            {
                Self::Merged {
                    ty,
                    value_ty: Some(other_binding_ty),
                    namespace_ty: Some(other_binding_ty),
                }
            }
            (
                Self::Merged {
                    ty: own_ty,
                    value_ty: own_value_ty,
                    namespace_ty: own_namespace_ty,
                },
                Self::Merged {
                    ty: other_ty,
                    value_ty: other_value_ty,
                    namespace_ty: other_namespace_ty,
                },
            ) => Self::Merged {
                ty: other_ty.or(own_ty),
                value_ty: other_value_ty.or(own_value_ty),
                namespace_ty: other_namespace_ty.or(own_namespace_ty),
            },

            (_, other) => other,
        }
    }
}

/// Internal type with all the semantic data of a specific binding
#[derive(Debug)]
pub(crate) struct SemanticModelBindingData {
    pub(crate) range: TextRange,
    pub(crate) references: Vec<SemanticModelReference>,
    // We use a SmallVec because most of the time a binding is expected once.
    pub(crate) export_by_start: smallvec::SmallVec<[TextSize; 4]>,
    /// The kind of declaration that introduced this binding.
    pub(crate) declaration_kind: JsDeclarationKind,
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

    /// Returns the kind of declaration that introduced this binding.
    pub fn declaration_kind(&self) -> JsDeclarationKind {
        self.data.binding(self.id).declaration_kind
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
