//! Normalized types for expressions and bindings.
//!
//! A binding request may attribute profiling to a different source range from
//! the binding it looks up. This allows an analyzer to record the use that
//! caused inference rather than the declaration being inspected.

use biome_js_type_info::interned_types::TypeData as InferredTypeData;
use biome_rowan::TextRange;

use crate::ModuleInfo;

use super::super::{
    Sealed, TypeInferenceCodeReference, TypeInferenceRequest, TypeInferenceRequestContext,
    TypeInferenceRequestMetadata, TypeInferenceRequestOrigin,
};

/// Resolves and normalizes the type of one expression.
///
/// Returns `None` when the module cannot provide inference or the expression
/// was not collected. Salsa query cycles, failed structural rebuilds, and
/// normalization work-limit exhaustion return `Unknown`. A local-handle cycle
/// retains the repeated symbolic handle.
pub struct NormalizedExpressionTypeRequest {
    module: ModuleInfo,
    expression: TextRange,
}

impl NormalizedExpressionTypeRequest {
    /// Uses `expression` as both the lookup key and profile origin.
    pub const fn new(module: ModuleInfo, expression: TextRange) -> Self {
        Self { module, expression }
    }
}

impl Sealed for NormalizedExpressionTypeRequest {}

impl TypeInferenceRequestMetadata for NormalizedExpressionTypeRequest {
    const ID: &'static str = "request.normalized-expression-type";
    const LABEL: &'static str = "Normalized expression type";
}

impl<'db> TypeInferenceRequest<'db> for NormalizedExpressionTypeRequest {
    type Output = Option<InferredTypeData<'db>>;

    const IMPLEMENTATION: TypeInferenceCodeReference = TypeInferenceCodeReference::new(
        file!(),
        line!(),
        "NormalizedExpressionTypeRequest::execute",
    );

    fn origin(&self) -> TypeInferenceRequestOrigin {
        TypeInferenceRequestOrigin::new(self.module, self.expression)
    }

    fn execute(self, context: &TypeInferenceRequestContext<'db>) -> Self::Output {
        let ty = context.expression_type(self.module, self.expression)?;
        Some(context.normalize_type(self.module, ty))
    }
}

/// Resolves and normalizes the type of one binding.
///
/// Returns `None` when the module cannot provide inference or the binding was
/// not collected. Salsa query cycles, failed structural rebuilds, and
/// normalization work-limit exhaustion return `Unknown`. A local-handle cycle
/// retains the repeated symbolic handle.
pub struct NormalizedBindingTypeRequest {
    module: ModuleInfo,
    origin: TextRange,
    binding: TextRange,
}

impl NormalizedBindingTypeRequest {
    /// Keeps the analyzer's `origin` distinct from the collected `binding` key.
    pub const fn new(module: ModuleInfo, origin: TextRange, binding: TextRange) -> Self {
        Self {
            module,
            origin,
            binding,
        }
    }
}

impl Sealed for NormalizedBindingTypeRequest {}

impl TypeInferenceRequestMetadata for NormalizedBindingTypeRequest {
    const ID: &'static str = "request.normalized-binding-type";
    const LABEL: &'static str = "Normalized binding type";
}

impl<'db> TypeInferenceRequest<'db> for NormalizedBindingTypeRequest {
    type Output = Option<InferredTypeData<'db>>;

    const IMPLEMENTATION: TypeInferenceCodeReference =
        TypeInferenceCodeReference::new(file!(), line!(), "NormalizedBindingTypeRequest::execute");

    fn origin(&self) -> TypeInferenceRequestOrigin {
        TypeInferenceRequestOrigin::new(self.module, self.origin)
    }

    fn execute(self, context: &TypeInferenceRequestContext<'db>) -> Self::Output {
        let ty = context.binding_type(self.module, self.binding)?;
        Some(context.normalize_type(self.module, ty))
    }
}
