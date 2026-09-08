//! Database operations available to type-inference requests.
//!
//! Request implementations use these methods instead of constructing database
//! query inputs themselves. This keeps shared lookup and result-processing
//! behavior consistent between requests.

use biome_js_type_info::interned_types::{
    CallArgumentType as InferredCallArgumentType, TypeData as InferredTypeData,
};
use biome_rowan::TextRange;

use crate::db::queries::{
    BindingTypeInput, ExpressionTypeInput, find_member_type, find_value_member_type,
    infer_binding_type, infer_export_type, infer_expression_function_returns_promise,
    infer_expression_is_array_of_promises, infer_expression_is_promise, infer_expression_type,
    resolve_callable_type,
};
use crate::{
    CallArgumentTypeInput, ModuleInfo, NormalizeTypeInput, SymbolFromModuleInfo,
    infer_call_argument_type, infer_constructor_argument_type, normalize_type,
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

    pub(crate) fn classify_expression_as_promise(
        &self,
        module: ModuleInfo,
        range: TextRange,
    ) -> TypeInferenceClassification {
        let db = self.db();
        infer_expression_is_promise(db, ExpressionTypeInput::new(db, module, range))
    }

    pub(crate) fn classify_expression_as_array_of_promises(
        &self,
        module: ModuleInfo,
        range: TextRange,
    ) -> TypeInferenceClassification {
        let db = self.db();
        infer_expression_is_array_of_promises(db, ExpressionTypeInput::new(db, module, range))
    }

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

    /// Resolves local references inside `ty` and simplifies its structure.
    ///
    /// Local-handle cycles retain the repeated symbolic handle. Salsa query
    /// cycles, failed structural rebuilds, and normalization work-limit
    /// exhaustion return `Unknown`.
    pub(crate) fn normalize_type(
        &self,
        module: ModuleInfo,
        ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        let db = self.db();
        normalize_type(db, NormalizeTypeInput::new(db, module, ty))
    }

    /// Resolves a named member through own properties, inheritance, and compound types.
    ///
    /// Members found in separate union, intersection, or merged-reference
    /// branches are combined. Lookup returns `Unknown` when it reaches its work
    /// limit without completing traversal.
    pub(crate) fn member_type(
        &self,
        ty: InferredTypeData<'db>,
        member_name: &str,
    ) -> Option<InferredTypeData<'db>> {
        find_member_type(self.db(), ty, member_name)
    }

    /// Resolves a named member available on a runtime value.
    ///
    /// Static members are considered for class values and instance members for
    /// object-like values. Lookup returns `Unknown` when it reaches its work
    /// limit without completing traversal.
    pub(crate) fn value_member_type(
        &self,
        ty: InferredTypeData<'db>,
        member_name: &str,
    ) -> Option<InferredTypeData<'db>> {
        find_value_member_type(self.db(), ty, member_name)
    }

    /// Selects a call signature and returns one callable parameter type.
    ///
    /// Returns `None` when no signature can be selected or the selected
    /// parameter type is not one unambiguous callable type.
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

    /// Selects a constructor signature and returns one callable parameter type.
    ///
    /// Returns `None` when no signature can be selected or the selected
    /// parameter type is not one unambiguous callable type.
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
