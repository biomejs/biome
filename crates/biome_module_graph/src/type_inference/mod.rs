//! Type-inference operations used by analyzers.
//!
//! An analyzer submits a request for one result, such as the type of an
//! expression or whether a function returns a Promise. The request records the
//! analyzer and source location that need the result. Its implementation uses a
//! shared context to run the database queries needed to answer it. Profiling can
//! therefore attribute all of those queries to the original analyzer request.

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
