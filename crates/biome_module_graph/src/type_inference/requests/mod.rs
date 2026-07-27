//! Type-inference requests available to analyzers.

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
