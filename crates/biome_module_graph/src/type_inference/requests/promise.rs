//! Checks for Promise-like values without resolving complete types.
//!
//! The requests return `Indeterminate` rather than guessing when the selected
//! part of a type is unavailable or ambiguous.

use super::super::{
    Sealed, TypeInferenceClassification, TypeInferenceCodeReference, TypeInferenceRequest,
    TypeInferenceRequestContext, TypeInferenceRequestMetadata, TypeInferenceRequestOrigin,
};
use crate::ModuleInfo;
use biome_rowan::TextRange;

/// Classifies whether an expression is Promise-like.
///
/// A missing expression, disabled inference, an unresolved or ambiguous export,
/// an `unknown` type, a dependency cycle, or an exhausted work limit produces
/// [`TypeInferenceClassification::Indeterminate`]. An unresolved import path or
/// unavailable imported module produces [`TypeInferenceClassification::NoMatch`].
///
/// This expression is indeterminate because its declared type gives no evidence
/// for or against a Promise shape.
///
/// ```ts
/// declare const value: unknown;
/// value;
/// ```
pub struct PromiseClassificationRequest {
    module: ModuleInfo,
    expression: TextRange,
}

impl PromiseClassificationRequest {
    /// Uses `expression` as both the classification input and profile origin.
    pub const fn new(module: ModuleInfo, expression: TextRange) -> Self {
        Self { module, expression }
    }
}

impl Sealed for PromiseClassificationRequest {}

impl TypeInferenceRequestMetadata for PromiseClassificationRequest {
    const ID: &'static str = "request.promise-classification";
    const LABEL: &'static str = "Promise classification";
}

impl<'db> TypeInferenceRequest<'db> for PromiseClassificationRequest {
    type Output = TypeInferenceClassification;

    const IMPLEMENTATION: TypeInferenceCodeReference =
        TypeInferenceCodeReference::new(file!(), line!(), "PromiseClassificationRequest::execute");

    fn origin(&self) -> TypeInferenceRequestOrigin {
        TypeInferenceRequestOrigin::new(self.module, self.expression)
    }

    fn execute(self, context: &TypeInferenceRequestContext<'db>) -> Self::Output {
        context.classify_expression_as_promise(self.module, self.expression)
    }
}

/// Classifies whether an expression is an array of Promise-like values.
///
/// A missing expression, disabled inference, an unresolved or ambiguous export,
/// an `unknown` outer type, an ambiguous overloaded call, a dependency cycle,
/// or an exhausted work limit produces
/// [`TypeInferenceClassification::Indeterminate`]. An unresolved import path or
/// unavailable imported module produces [`TypeInferenceClassification::NoMatch`].
///
/// This expression is indeterminate because its outer type is unknown.
///
/// ```ts
/// declare const values: unknown;
/// values;
/// ```
pub struct ArrayOfPromisesClassificationRequest {
    module: ModuleInfo,
    expression: TextRange,
}

impl ArrayOfPromisesClassificationRequest {
    /// Uses `expression` as both the classification input and profile origin.
    pub const fn new(module: ModuleInfo, expression: TextRange) -> Self {
        Self { module, expression }
    }
}

impl Sealed for ArrayOfPromisesClassificationRequest {}

impl TypeInferenceRequestMetadata for ArrayOfPromisesClassificationRequest {
    const ID: &'static str = "request.array-of-promises-classification";
    const LABEL: &'static str = "Array of Promises classification";
}

impl<'db> TypeInferenceRequest<'db> for ArrayOfPromisesClassificationRequest {
    type Output = TypeInferenceClassification;

    const IMPLEMENTATION: TypeInferenceCodeReference = TypeInferenceCodeReference::new(
        file!(),
        line!(),
        "ArrayOfPromisesClassificationRequest::execute",
    );

    fn origin(&self) -> TypeInferenceRequestOrigin {
        TypeInferenceRequestOrigin::new(self.module, self.expression)
    }

    fn execute(self, context: &TypeInferenceRequestContext<'db>) -> Self::Output {
        context.classify_expression_as_array_of_promises(self.module, self.expression)
    }
}

/// Classifies whether calling an expression returns a Promise-like value.
///
/// A missing expression, disabled inference, an unresolved or ambiguous export,
/// an `unknown` callable, any recognized overload set, a dependency cycle, or
/// an exhausted work limit produces [`TypeInferenceClassification::Indeterminate`].
/// Overload return types are not compared. An unresolved import path or
/// unavailable imported module produces [`TypeInferenceClassification::NoMatch`].
///
/// This function is indeterminate because it has an overload set.
///
/// ```ts
/// declare function load(): void;
/// declare function load(id: string): Promise<void>;
/// load;
/// ```
pub struct PromiseReturningFunctionClassificationRequest {
    module: ModuleInfo,
    expression: TextRange,
}

impl PromiseReturningFunctionClassificationRequest {
    /// Uses `expression` as both the classification input and profile origin.
    pub const fn new(module: ModuleInfo, expression: TextRange) -> Self {
        Self { module, expression }
    }
}

impl Sealed for PromiseReturningFunctionClassificationRequest {}

impl TypeInferenceRequestMetadata for PromiseReturningFunctionClassificationRequest {
    const ID: &'static str = "request.promise-returning-function-classification";
    const LABEL: &'static str = "Promise-returning function classification";
}

impl<'db> TypeInferenceRequest<'db> for PromiseReturningFunctionClassificationRequest {
    type Output = TypeInferenceClassification;

    const IMPLEMENTATION: TypeInferenceCodeReference = TypeInferenceCodeReference::new(
        file!(),
        line!(),
        "PromiseReturningFunctionClassificationRequest::execute",
    );

    fn origin(&self) -> TypeInferenceRequestOrigin {
        TypeInferenceRequestOrigin::new(self.module, self.expression)
    }

    fn execute(self, context: &TypeInferenceRequestContext<'db>) -> Self::Output {
        context.classify_expression_as_promise_returning_function(self.module, self.expression)
    }
}
