//! Analyzer-facing request flows for Salsa-backed type inference.
//!
//! A request represents the semantic result a consumer asks the type engine to
//! compute. Request implementations compose tracked query primitives through a
//! shared context, keeping consumer intent separate from query implementation.
//! This boundary gives profiling and contributor tooling one stable place to
//! attach caller, source, and implementation metadata.

mod classification;
mod context;
#[doc(hidden)]
pub mod profiling;
mod registry;
mod request;
mod requests;

pub use classification::TypeInferenceClassification;
pub(crate) use registry::{TypeInferenceQueryKind, TypeInferenceWholeModuleReason};
pub use request::{
    TypeInferenceCaller, TypeInferenceCodeReference, TypeInferenceRequest,
    TypeInferenceRequestContext, TypeInferenceRequestMetadata, TypeInferenceRequestOrigin,
    execute_type_inference_request,
};
pub use requests::{
    ArrayOfPromisesClassificationRequest, CallableMemberRequest, ExpectedCallArgumentTypeRequest,
    ExpectedConstructorArgumentTypeRequest, FunctionReturnTypeRequest, MemberReturnTypeRequest,
    NormalizedBindingTypeRequest, NormalizedExpressionTypeRequest, PromiseClassificationRequest,
    PromiseReturningFunctionClassificationRequest, TypeInferenceArgument, TypeInferenceSource,
};

pub(crate) use request::Sealed;
