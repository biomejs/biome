use std::sync::Arc;

use biome_js_syntax::{
    AnyJsDeclaration, JsImport, JsSyntaxNode, JsVariableKind, TextRange, TsTypeParameter,
};
use biome_js_type_info::{BindingId, ScopeId, TypeReference};
use biome_rowan::{AstNode, Text, TextSize};

use biome_jsdoc_comment::JsdocComment;

use super::{JsModuleInfoInner, scope::JsScope};

/// Internal type with all the semantic data of a specific binding
#[derive(Clone, Debug)]
pub struct JsBindingData {
    pub name: Text,
    pub references: Vec<JsBindingReference>,
    pub scope_id: ScopeId,
    pub declaration_kind: JsDeclarationKind,
    pub ty: TypeReference,
    pub jsdoc: Option<JsdocComment>,
    pub export_ranges: Vec<TextRange>,
    pub range: TextRange,
}

#[derive(Clone, Copy, Debug)]
pub enum JsBindingReferenceKind {
    Read { _hoisted: bool },
    Write { _hoisted: bool },
}

/// Internal type with all the semantic data of a specific reference
#[derive(Clone, Debug)]
pub struct JsBindingReference {
    pub range_start: TextSize,
    pub kind: JsBindingReferenceKind,
}

impl JsBindingReference {
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        matches!(self.kind, JsBindingReferenceKind::Read { .. })
    }

    #[inline(always)]
    pub fn is_write(&self) -> bool {
        matches!(self.kind, JsBindingReferenceKind::Write { .. })
    }
}

/// Provides access to all semantic data of a specific binding.
pub struct JsBinding {
    pub(crate) data: Arc<JsModuleInfoInner>,
    pub id: BindingId,
}

impl std::fmt::Debug for JsBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Binding").field("id", &self.id).finish()
    }
}

impl JsBinding {
    /// Returns whether the binding is exported.
    pub fn is_exported(&self) -> bool {
        let binding = self.data.binding(self.id);
        !binding.export_ranges.is_empty()
    }

    /// Returns whether the binding is imported.
    pub fn is_imported(&self) -> bool {
        let binding = self.data.binding(self.id);
        binding.declaration_kind.is_import_declaration()
    }

    /// Returns the binding's name.
    pub fn name(&self) -> Text {
        let binding = self.data.binding(self.id);
        binding.name.clone()
    }

    /// Returns the scope of this binding.
    pub fn scope(&self) -> JsScope {
        let binding = self.data.binding(self.id);
        JsScope {
            info: self.data.clone(),
            id: binding.scope_id,
        }
    }

    /// Returns a reference to the binding's type.
    pub fn ty(&self) -> &TypeReference {
        &self.data.binding(self.id).ty
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum JsDeclarationKind {
    /// A `class` declaration.
    Class,

    /// An `enum` declaration.
    Enum,

    /// A generic type parameter, declared in angle brackets.
    ///
    /// For example: `<T>`.
    Generic,

    /// A `function` or `var` declaration.
    HoistedValue,

    /// An `import` declaration.
    Import,

    /// An `import type` declaration.
    ImportType,

    /// An interface declaration.
    Interface,

    /// A module declaration.
    Module,

    /// A namespace declaration.
    Namespace,

    /// A type declaration.
    Type,

    /// A bogus declaration or a declaration kind we don't handle yet.
    #[default]
    Unknown,

    /// A `using` declaration.
    Using,

    /// A `let` or `const` declaration.
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

    #[inline]
    pub fn is_import_declaration(&self) -> bool {
        matches!(self, Self::Import | Self::ImportType)
    }

    #[inline]
    pub fn is_import_type_declaration(&self) -> bool {
        matches!(self, Self::ImportType)
    }
}
