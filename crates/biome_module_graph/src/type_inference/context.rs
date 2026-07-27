//! Query operations available to analyzer-facing requests.
//!
//! Request implementations compose these operations instead of constructing
//! Salsa keys directly. This keeps query selection and post-processing shared
//! when several request types need the same inference primitive.

use biome_js_type_info::interned_types::{
    CallArgumentType as InferredCallArgumentType, TypeData as InferredTypeData,
};
use biome_rowan::TextRange;

use crate::{
    BindingTypeInput, CallArgumentTypeInput, ExpressionTypeInput, ModuleInfo, NormalizeTypeInput,
    SymbolFromModuleInfo, find_member_type, find_value_member_type, infer_binding_type,
    infer_call_argument_type, infer_constructor_argument_type, infer_export_type,
    infer_expression_function_returns_promise, infer_expression_is_array_of_promises,
    infer_expression_is_promise, infer_expression_type, normalize_type, resolve_callable_type,
};

use super::{TypeInferenceClassification, TypeInferenceRequestContext};

impl<'db> TypeInferenceRequestContext<'db> {
    /// Resolves the collected type of an expression.
    ///
    /// Returns `None` when the module cannot provide inference or the expression
    /// was not collected. Query cycles produce `Unknown` rather than `None`.
    pub(crate) fn expression_type(
        &self,
        module: ModuleInfo,
        range: TextRange,
    ) -> Option<InferredTypeData<'db>> {
        let db = self.db();
        infer_expression_type(db, ExpressionTypeInput::new(db, module, range))
    }

    /// Classifies an expression as Promise-like.
    pub(crate) fn classify_expression_as_promise(
        &self,
        module: ModuleInfo,
        range: TextRange,
    ) -> TypeInferenceClassification {
        let db = self.db();
        infer_expression_is_promise(db, ExpressionTypeInput::new(db, module, range))
    }

    /// Classifies an expression as an array of Promise-like values.
    pub(crate) fn classify_expression_as_array_of_promises(
        &self,
        module: ModuleInfo,
        range: TextRange,
    ) -> TypeInferenceClassification {
        let db = self.db();
        infer_expression_is_array_of_promises(db, ExpressionTypeInput::new(db, module, range))
    }

    /// Classifies whether calling an expression returns a Promise-like value.
    pub(crate) fn classify_expression_as_promise_returning_function(
        &self,
        module: ModuleInfo,
        range: TextRange,
    ) -> TypeInferenceClassification {
        let db = self.db();
        infer_expression_function_returns_promise(db, ExpressionTypeInput::new(db, module, range))
    }

    /// Resolves the collected type of a binding.
    ///
    /// Returns `None` when the module cannot provide inference or the binding
    /// was not collected. Query cycles produce `Unknown` rather than `None`.
    pub(crate) fn binding_type(
        &self,
        module: ModuleInfo,
        range: TextRange,
    ) -> Option<InferredTypeData<'db>> {
        let db = self.db();
        infer_binding_type(db, BindingTypeInput::new(db, module, range))
    }

    /// Resolves the module's default export type.
    ///
    /// Returns `None` when the module cannot provide inference. Missing,
    /// ambiguous, cyclic, or indeterminate exports produce `Unknown`.
    pub(crate) fn default_export_type(&self, module: ModuleInfo) -> Option<InferredTypeData<'db>> {
        let db = self.db();
        infer_export_type(db, SymbolFromModuleInfo::new(db, "default", module))
    }

    /// Recursively resolves and simplifies `ty` in `module`.
    pub(crate) fn normalize_type(
        &self,
        module: ModuleInfo,
        ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        let db = self.db();
        normalize_type(db, NormalizeTypeInput::new(db, module, ty))
    }

    /// Resolves a named member from any supported structural type.
    ///
    /// Returns `None` when the type has no resolvable member with that name.
    pub(crate) fn member_type(
        &self,
        ty: InferredTypeData<'db>,
        member_name: &str,
    ) -> Option<InferredTypeData<'db>> {
        find_member_type(self.db(), ty, member_name)
    }

    /// Resolves a named member from a runtime value shape.
    ///
    /// Returns `None` when the value has no resolvable member with that name.
    pub(crate) fn value_member_type(
        &self,
        ty: InferredTypeData<'db>,
        member_name: &str,
    ) -> Option<InferredTypeData<'db>> {
        find_value_member_type(self.db(), ty, member_name)
    }

    /// Selects a call signature and resolves one expected argument type.
    ///
    /// Returns `None` when no signature can be selected conclusively.
    pub(crate) fn call_argument_type(
        &self,
        callee: InferredTypeData<'db>,
        arguments: Box<[InferredCallArgumentType<'db>]>,
        argument_index: usize,
    ) -> Option<InferredTypeData<'db>> {
        let db = self.db();
        let input = CallArgumentTypeInput::new(db, callee, arguments, argument_index);
        let ty = infer_call_argument_type(db, input)?;
        resolve_callable_type(db, ty)
    }

    /// Selects a constructor signature and resolves one expected argument type.
    ///
    /// Returns `None` when no signature can be selected conclusively.
    pub(crate) fn constructor_argument_type(
        &self,
        callee: InferredTypeData<'db>,
        arguments: Box<[InferredCallArgumentType<'db>]>,
        argument_index: usize,
    ) -> Option<InferredTypeData<'db>> {
        let db = self.db();
        let input = CallArgumentTypeInput::new(db, callee, arguments, argument_index);
        let ty = infer_constructor_argument_type(db, input)?;
        resolve_callable_type(db, ty)
    }
}
