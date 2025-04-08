use std::sync::Arc;

use biome_js_semantic::{BindingId, ScopeId};
use biome_js_syntax::{AnyJsDeclaration, JsImport, JsSyntaxNode, JsVariableKind, TextRange};
use biome_js_type_info::Type;
use biome_rowan::{AstNode, TextSize};

use crate::jsdoc_comment::JsdocComment;

use super::{JsModuleInfoInner, scope::JsScope};

/// Internal type with all the semantic data of a specific binding
#[derive(Debug)]
pub struct JsBindingData {
    pub range: TextRange,
    pub references: Vec<JsBindingReference>,
    pub scope_id: ScopeId,
    #[expect(unused)] // TODO: I expect we'll start using this in a bit (famous last words)...
    pub declaration_kind: JsDeclarationKind,
    pub ty: Type,
    pub jsdoc: Option<JsdocComment>,
    pub export_ranges: Vec<TextRange>,
}

#[derive(Clone, Copy, Debug)]
pub enum JsBindingReferenceKind {
    Read { _hoisted: bool },
    Write { _hoisted: bool },
}

/// Internal type with all the semantic data of a specific reference
#[derive(Debug)]
#[expect(unused)]
pub struct JsBindingReference {
    pub range_start: TextSize,
    pub kind: JsBindingReferenceKind,
}

#[expect(unused)]
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
    pub(crate) id: BindingId,
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

    /// Returns the scope of this binding.
    pub fn scope(&self) -> JsScope {
        let binding = self.data.binding(self.id);
        JsScope {
            info: self.data.clone(),
            id: binding.scope_id,
        }
    }
}

#[derive(Clone, Debug)]
pub enum JsDeclarationKind {
    /// A `class` declaration.
    Class,

    /// An `enum` declaration.
    Enum,

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
    Unknown,

    /// A `using` declaration.
    Using,

    /// A `let` or `const` declaration.
    Value,
}

impl JsDeclarationKind {
    pub fn from_node(node: &JsSyntaxNode) -> Self {
        let Some(declaration) = node.ancestors().find_map(AnyJsDeclaration::cast) else {
            return match node.ancestors().find_map(JsImport::cast) {
                Some(import) => match import.import_clause() {
                    Ok(import_clause) if import_clause.type_token().is_some() => Self::ImportType,
                    _ => Self::Import,
                },
                None => Self::Unknown,
            };
        };

        match declaration {
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
            AnyJsDeclaration::TsGlobalDeclaration(_) => {
                // TODO: Handle this
                Self::Unknown
            }
            AnyJsDeclaration::TsImportEqualsDeclaration(_) => {
                // TODO: Handle this
                Self::Unknown
            }
            AnyJsDeclaration::TsInterfaceDeclaration(_) => JsDeclarationKind::Interface,
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
        }
    }
}
