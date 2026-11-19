//! Tells whether code can run at all, from the flow of its execution root.

use super::super::{
    Sealed, TypeInferenceClassification, TypeInferenceCodeReference, TypeInferenceRequest,
    TypeInferenceRequestContext, TypeInferenceRequestMetadata, TypeInferenceRequestOrigin,
};
use crate::ModuleInfo;
use biome_rowan::TextRange;

/// Classifies whether no path reaches a statement, as TypeScript decides
/// reachability.
///
/// [`TypeInferenceClassification::Match`] means that the statement is
/// unreachable, because every path to it ends first: at `return`, `throw`,
/// `break`, or `continue`, in a branch that a condition written as the
/// literal `true` or `false` never takes, at a call that never returns
/// because its signature's return type annotation is `never` or because it
/// asserts an argument that is always false, or on the path that matches no
/// case of an exhaustive `switch`. [`TypeInferenceClassification::NoMatch`]
/// means that some path reaches it. A narrowed type, even `never`, never
/// makes code unreachable.
///
/// The statement is the innermost one that contains `range`, and a statement
/// nested in an unreachable one is unreachable too. A range outside any
/// statement, disabled inference, a root over a work limit, and a dependency
/// cycle produce [`TypeInferenceClassification::Indeterminate`].
///
/// The read of `x` is unreachable after the `switch`, whose cases cover
/// every value of `x`:
///
/// ```ts
/// function f(x: "a" | "b") {
///     switch (x) {
///         case "a": return 1;
///         case "b": return 2;
///     }
///     x;
/// }
/// ```
pub struct UnreachableCodeClassificationRequest {
    module: ModuleInfo,
    range: TextRange,
}

impl UnreachableCodeClassificationRequest {
    /// Uses `range` as both the classification input and profile origin.
    pub const fn new(module: ModuleInfo, range: TextRange) -> Self {
        Self { module, range }
    }
}

impl Sealed for UnreachableCodeClassificationRequest {}

impl TypeInferenceRequestMetadata for UnreachableCodeClassificationRequest {
    const ID: &'static str = "request.unreachable-code-classification";
    const LABEL: &'static str = "Unreachable code classification";
}

impl<'db> TypeInferenceRequest<'db> for UnreachableCodeClassificationRequest {
    type Output = TypeInferenceClassification;

    const IMPLEMENTATION: TypeInferenceCodeReference = TypeInferenceCodeReference::new(
        file!(),
        line!(),
        "UnreachableCodeClassificationRequest::execute",
    );

    fn origin(&self) -> TypeInferenceRequestOrigin {
        TypeInferenceRequestOrigin::new(self.module, self.range)
    }

    fn execute(self, context: &TypeInferenceRequestContext<'db>) -> Self::Output {
        context.classify_statement_as_unreachable(self.module, self.range)
    }
}
