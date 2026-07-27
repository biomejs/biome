//! Expected argument-type requests for calls and constructors.
//!
//! Both requests resolve the callee and source arguments before delegating
//! signature selection to the call-query family. They remain distinct request
//! types because call and constructor signature selection produce different
//! results and use different tracked queries.

use biome_js_type_info::interned_types::{
    CallArgumentType as InferredCallArgumentType, TypeData as InferredTypeData,
};
use biome_rowan::TextRange;

use crate::ModuleInfo;

use super::super::{
    Sealed, TypeInferenceCodeReference, TypeInferenceRequest, TypeInferenceRequestContext,
    TypeInferenceRequestMetadata, TypeInferenceRequestOrigin,
};

/// Source location and spread state of one call argument.
pub struct TypeInferenceArgument {
    range: TextRange,
    is_spread: bool,
}

impl TypeInferenceArgument {
    /// `range` identifies the argument expression, excluding spread punctuation.
    pub const fn new(range: TextRange, is_spread: bool) -> Self {
        Self { range, is_spread }
    }

    /// Returns the source range of the argument expression.
    pub const fn range(&self) -> TextRange {
        self.range
    }

    fn is_spread(&self) -> bool {
        self.is_spread
    }
}

struct ExpectedArgumentInput {
    module: ModuleInfo,
    origin: TextRange,
    callee: TextRange,
    arguments: Box<[TypeInferenceArgument]>,
    argument_index: usize,
}

impl ExpectedArgumentInput {
    fn new(
        module: ModuleInfo,
        origin: TextRange,
        callee: TextRange,
        arguments: Box<[TypeInferenceArgument]>,
        argument_index: usize,
    ) -> Self {
        Self {
            module,
            origin,
            callee,
            arguments,
            argument_index,
        }
    }

    fn origin(&self) -> TypeInferenceRequestOrigin {
        TypeInferenceRequestOrigin::new(self.module, self.origin)
    }

    fn resolve<'db>(
        self,
        context: &TypeInferenceRequestContext<'db>,
    ) -> Option<(
        InferredTypeData<'db>,
        Box<[InferredCallArgumentType<'db>]>,
        usize,
    )> {
        let callee = context.expression_type(self.module, self.callee)?;
        let arguments = self
            .arguments
            .iter()
            .map(|argument| {
                let ty = context.expression_type(self.module, argument.range())?;
                Some(if argument.is_spread() {
                    InferredCallArgumentType::Spread(ty)
                } else {
                    InferredCallArgumentType::Argument(ty)
                })
            })
            .collect::<Option<Vec<_>>>()?
            .into_boxed_slice();
        Some((callee, arguments, self.argument_index))
    }
}

/// Resolves the expected type of one call argument.
///
/// Returns `None` when the callee or a sibling argument is unavailable, or no
/// supported call signature provides an expected type. An indeterminate
/// expected type remains `Unknown`.
pub struct ExpectedCallArgumentTypeRequest(ExpectedArgumentInput);

impl ExpectedCallArgumentTypeRequest {
    /// Uses all source arguments for call-signature selection and attributes the request to `origin`.
    pub fn new(
        module: ModuleInfo,
        origin: TextRange,
        callee: TextRange,
        arguments: Box<[TypeInferenceArgument]>,
        argument_index: usize,
    ) -> Self {
        Self(ExpectedArgumentInput::new(
            module,
            origin,
            callee,
            arguments,
            argument_index,
        ))
    }
}

impl Sealed for ExpectedCallArgumentTypeRequest {}

impl TypeInferenceRequestMetadata for ExpectedCallArgumentTypeRequest {
    const ID: &'static str = "request.expected-call-argument-type";
    const LABEL: &'static str = "Expected call argument type";
}

impl<'db> TypeInferenceRequest<'db> for ExpectedCallArgumentTypeRequest {
    type Output = Option<InferredTypeData<'db>>;

    const IMPLEMENTATION: TypeInferenceCodeReference = TypeInferenceCodeReference::new(
        file!(),
        line!(),
        "ExpectedCallArgumentTypeRequest::execute",
    );

    fn origin(&self) -> TypeInferenceRequestOrigin {
        self.0.origin()
    }

    fn execute(self, context: &TypeInferenceRequestContext<'db>) -> Self::Output {
        let (callee, arguments, argument_index) = self.0.resolve(context)?;
        context.call_argument_type(callee, arguments, argument_index)
    }
}

/// Resolves the expected type of one constructor argument.
///
/// Returns `None` when the callee or a sibling argument is unavailable, or no
/// supported constructor signature provides an expected type. An indeterminate
/// expected type remains `Unknown`.
pub struct ExpectedConstructorArgumentTypeRequest(ExpectedArgumentInput);

impl ExpectedConstructorArgumentTypeRequest {
    /// Uses all source arguments for constructor-signature selection and attributes the request to `origin`.
    pub fn new(
        module: ModuleInfo,
        origin: TextRange,
        callee: TextRange,
        arguments: Box<[TypeInferenceArgument]>,
        argument_index: usize,
    ) -> Self {
        Self(ExpectedArgumentInput::new(
            module,
            origin,
            callee,
            arguments,
            argument_index,
        ))
    }
}

impl Sealed for ExpectedConstructorArgumentTypeRequest {}

impl TypeInferenceRequestMetadata for ExpectedConstructorArgumentTypeRequest {
    const ID: &'static str = "request.expected-constructor-argument-type";
    const LABEL: &'static str = "Expected constructor argument type";
}

impl<'db> TypeInferenceRequest<'db> for ExpectedConstructorArgumentTypeRequest {
    type Output = Option<InferredTypeData<'db>>;

    const IMPLEMENTATION: TypeInferenceCodeReference = TypeInferenceCodeReference::new(
        file!(),
        line!(),
        "ExpectedConstructorArgumentTypeRequest::execute",
    );

    fn origin(&self) -> TypeInferenceRequestOrigin {
        self.0.origin()
    }

    fn execute(self, context: &TypeInferenceRequestContext<'db>) -> Self::Output {
        let (callee, arguments, argument_index) = self.0.resolve(context)?;
        context.constructor_argument_type(callee, arguments, argument_index)
    }
}
