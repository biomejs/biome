//! Callable-member classification requests.
//!
//! Member classification normalizes the receiver and selected member while
//! preserving indeterminate results for structure that cannot be inferred.

use biome_js_type_info::InferredType;
use biome_rowan::TextRange;

use crate::ModuleInfo;

use super::super::{
    Sealed, TypeInferenceClassification, TypeInferenceCodeReference, TypeInferenceRequest,
    TypeInferenceRequestContext, TypeInferenceRequestMetadata, TypeInferenceRequestOrigin,
};

/// Classifies whether an expression has a callable member.
///
/// Unavailable or structurally indeterminate receiver and member types produce
/// an indeterminate result.
pub struct CallableMemberRequest<'name> {
    module: ModuleInfo,
    expression: TextRange,
    member_name: &'name str,
}

impl<'name> CallableMemberRequest<'name> {
    /// Uses `expression` as the receiver lookup and profile origin.
    pub const fn new(module: ModuleInfo, expression: TextRange, member_name: &'name str) -> Self {
        Self {
            module,
            expression,
            member_name,
        }
    }
}

impl Sealed for CallableMemberRequest<'_> {}

impl TypeInferenceRequestMetadata for CallableMemberRequest<'_> {
    const ID: &'static str = "request.callable-member";
    const LABEL: &'static str = "Callable member check";
}

impl<'db> TypeInferenceRequest<'db> for CallableMemberRequest<'_> {
    type Output = TypeInferenceClassification;

    const IMPLEMENTATION: TypeInferenceCodeReference =
        TypeInferenceCodeReference::new(file!(), line!(), "CallableMemberRequest::execute");

    fn origin(&self) -> TypeInferenceRequestOrigin {
        TypeInferenceRequestOrigin::new(self.module, self.expression)
    }

    fn execute(self, context: &TypeInferenceRequestContext<'db>) -> Self::Output {
        let Some(ty) = context.expression_type(self.module, self.expression) else {
            return TypeInferenceClassification::Indeterminate;
        };
        let ty = context.normalize_type(self.module, ty);
        if ty.is_indeterminate() {
            return TypeInferenceClassification::Indeterminate;
        }
        let Some(member_ty) = context.value_member_type(ty, self.member_name) else {
            return TypeInferenceClassification::NoMatch;
        };
        let member_ty = context.normalize_type(self.module, member_ty);

        match InferredType::new(context.db(), member_ty).is_callable() {
            Some(true) => TypeInferenceClassification::Match,
            Some(false) => TypeInferenceClassification::NoMatch,
            None => TypeInferenceClassification::Indeterminate,
        }
    }
}
