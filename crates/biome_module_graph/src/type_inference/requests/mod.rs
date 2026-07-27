//! Registered analyzer-facing inference request implementations.
//!
//! Requests are grouped by the semantic result they return. Analyzer rules
//! reuse these types rather than defining rule-specific inference flows.

mod call;
mod expression;
mod member;
mod promise;
mod return_type;

pub use call::{
    ExpectedCallArgumentTypeRequest, ExpectedConstructorArgumentTypeRequest, TypeInferenceArgument,
};
pub use expression::{NormalizedBindingTypeRequest, NormalizedExpressionTypeRequest};
pub use member::CallableMemberRequest;
pub use promise::{
    ArrayOfPromisesClassificationRequest, PromiseClassificationRequest,
    PromiseReturningFunctionClassificationRequest,
};
pub use return_type::{FunctionReturnTypeRequest, MemberReturnTypeRequest, TypeInferenceSource};
