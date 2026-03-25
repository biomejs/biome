use std::sync::Arc;

use biome_js_semantic::{JsDeclarationKind, ScopeId};
use biome_js_syntax::TextRange;
use biome_js_type_info::TypeReference;
use biome_rowan::{Text, TextSize};

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
    pub fn is_write(&self) -> bool {
        matches!(self.kind, JsBindingReferenceKind::Write { .. })
    }
}

/// Provides access to all semantic data of a specific binding.
pub struct JsBinding {
    data: Arc<JsModuleInfoInner>,
    semantic_binding: biome_js_semantic::Binding,
}

impl std::fmt::Debug for JsBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self
            .semantic_binding
            .tree()
            .name_token()
            .ok()
            .map(|t| t.text_trimmed().to_string());
        f.debug_struct("JsBinding").field("name", &name).finish()
    }
}

impl JsBinding {
    pub(crate) fn from_semantic_binding(
        data: Arc<JsModuleInfoInner>,
        semantic_binding: biome_js_semantic::Binding,
    ) -> Self {
        Self {
            data,
            semantic_binding,
        }
    }

    /// Returns whether the binding is exported.
    pub fn is_exported(&self) -> bool {
        // Check if there are export ranges in the type augmentation data
        let binding_range = self.semantic_binding.syntax().text_trimmed_range();
        self.data
            .binding_type_data
            .get(&binding_range)
            .is_some_and(|data| !data.export_ranges.is_empty())
    }

    /// Returns whether the binding is imported.
    pub fn is_imported(&self) -> bool {
        self.semantic_binding.is_imported()
    }

    /// Returns the binding's name.
    pub fn name(&self) -> Text {
        self.semantic_binding
            .tree()
            .name_token()
            .ok()
            .map(|t| t.token_text_trimmed().into())
            .unwrap_or_default()
    }

    /// Returns the scope of this binding.
    pub fn scope(&self) -> JsScope {
        JsScope {
            info: self.data.clone(),
            scope: self.semantic_binding.scope(),
        }
    }

    /// Returns the binding's type.
    ///
    /// Returns an owned TypeReference since we may need to return
    /// a default unknown type when no augmentation data exists.
    pub fn ty(&self) -> TypeReference {
        // Look up type augmentation data by binding range
        let binding_range = self.semantic_binding.syntax().text_trimmed_range();
        self.data
            .binding_type_data
            .get(&binding_range)
            .map_or_else(TypeReference::unknown, |data| data.ty.clone())
    }
}
