//! Normalized function and member return-type requests.
//!
//! A request may start from an expression, binding, or default export, but all
//! sources share callable extraction and normalization semantics.

use biome_js_type_info::interned_types::{
    ReturnType as InferredReturnType, TypeData as InferredTypeData,
};
use biome_rowan::TextRange;

use crate::ModuleInfo;

use super::super::{
    Sealed, TypeInferenceCodeReference, TypeInferenceRequest, TypeInferenceRequestContext,
    TypeInferenceRequestMetadata, TypeInferenceRequestOrigin,
};

/// Collected entry used as the starting point for a request.
pub enum TypeInferenceSource {
    /// Type collected for an expression range.
    Expression(TextRange),
    /// Type collected for a binding range.
    Binding(TextRange),
    /// Type exported as the module's default export.
    DefaultExport,
}

impl TypeInferenceSource {
    fn resolve<'db>(
        self,
        context: &TypeInferenceRequestContext<'db>,
        module: ModuleInfo,
    ) -> Option<InferredTypeData<'db>> {
        match self {
            Self::Expression(range) => context.expression_type(module, range),
            Self::Binding(range) => context.binding_type(module, range),
            Self::DefaultExport => context.default_export_type(module),
        }
    }
}

/// Resolves and normalizes the return type of one function.
///
/// Returns `None` when the source is unavailable, is not callable, or has no
/// concrete type return. Indeterminate return types remain `Unknown`.
pub struct FunctionReturnTypeRequest {
    module: ModuleInfo,
    origin: TextRange,
    source: TypeInferenceSource,
}

impl FunctionReturnTypeRequest {
    /// Creates a request for `source`, attributed to `origin`.
    pub const fn new(module: ModuleInfo, origin: TextRange, source: TypeInferenceSource) -> Self {
        Self {
            module,
            origin,
            source,
        }
    }
}

impl Sealed for FunctionReturnTypeRequest {}

impl TypeInferenceRequestMetadata for FunctionReturnTypeRequest {
    const ID: &'static str = "request.function-return-type";
    const LABEL: &'static str = "Function return type";
}

impl<'db> TypeInferenceRequest<'db> for FunctionReturnTypeRequest {
    type Output = Option<InferredTypeData<'db>>;

    const IMPLEMENTATION: TypeInferenceCodeReference =
        TypeInferenceCodeReference::new(file!(), line!(), "FunctionReturnTypeRequest::execute");

    fn origin(&self) -> TypeInferenceRequestOrigin {
        TypeInferenceRequestOrigin::new(self.module, self.origin)
    }

    fn execute(self, context: &TypeInferenceRequestContext<'db>) -> Self::Output {
        let function_ty = self.source.resolve(context, self.module)?;
        let function = function_ty.callable_function(context.db())?;
        let InferredReturnType::Type(return_ty) = function.return_type(context.db()) else {
            return None;
        };
        Some(context.normalize_type(self.module, *return_ty))
    }
}

/// Resolves and normalizes the return type of one class or object member.
///
/// A callable member with a concrete type return yields that return type;
/// otherwise the request yields the normalized member type. Missing or
/// structurally unresolvable parents and members return `None`; a resolved
/// indeterminate type remains `Unknown`.
pub struct MemberReturnTypeRequest<'name> {
    module: ModuleInfo,
    origin: TextRange,
    parent: TypeInferenceSource,
    member_name: &'name str,
}

impl<'name> MemberReturnTypeRequest<'name> {
    /// Creates a return-type request for `member_name` on `parent`.
    pub const fn new(
        module: ModuleInfo,
        origin: TextRange,
        parent: TypeInferenceSource,
        member_name: &'name str,
    ) -> Self {
        Self {
            module,
            origin,
            parent,
            member_name,
        }
    }
}

impl Sealed for MemberReturnTypeRequest<'_> {}

impl TypeInferenceRequestMetadata for MemberReturnTypeRequest<'_> {
    const ID: &'static str = "request.member-return-type";
    const LABEL: &'static str = "Member return type";
}

impl<'db> TypeInferenceRequest<'db> for MemberReturnTypeRequest<'_> {
    type Output = Option<InferredTypeData<'db>>;

    const IMPLEMENTATION: TypeInferenceCodeReference =
        TypeInferenceCodeReference::new(file!(), line!(), "MemberReturnTypeRequest::execute");

    fn origin(&self) -> TypeInferenceRequestOrigin {
        TypeInferenceRequestOrigin::new(self.module, self.origin)
    }

    fn execute(self, context: &TypeInferenceRequestContext<'db>) -> Self::Output {
        let parent_ty = self.parent.resolve(context, self.module)?;
        let parent_ty = context.normalize_type(self.module, parent_ty);
        let member_ty = context.member_type(parent_ty, self.member_name)?;
        let return_ty = member_ty
            .callable_function(context.db())
            .and_then(|function| {
                let InferredReturnType::Type(return_ty) = function.return_type(context.db()) else {
                    return None;
                };
                Some(*return_ty)
            });

        Some(context.normalize_type(self.module, return_ty.unwrap_or(member_ty)))
    }
}
