//! Callable parameter types expected by calls and constructors.
//!
//! These requests support analyzers that inspect callbacks passed as arguments.
//! They select a call or constructor signature from the other arguments, then
//! return the requested parameter only when its type is callable. Calls and
//! constructors use separate request types because their signatures differ.

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
            .enumerate()
            .map(|(index, argument)| {
                let ty = if index == self.argument_index && !argument.is_spread() {
                    InferredTypeData::Unknown
                } else {
                    context.expression_type(self.module, argument.range())?
                };
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

/// Returns the callable type expected for one call argument.
///
/// Signature selection uses the types of all arguments except the requested
/// non-spread argument. The requested argument is omitted because its expected
/// type is the result of this request. A requested spread argument is resolved
/// because its tuple shape determines how arguments map to parameters.
///
/// Returns `None` when the callee or another required argument type is
/// unavailable, no supported signature matches, the requested index is out of
/// bounds, or the selected parameter does not have one unambiguous callable
/// type.
///
/// Requesting argument 1 uses `"sync"` to select the first overload and returns
/// `() => void`. Requesting argument 0 uses the callback to select the second
/// overload and returns `None` because the `"async"` parameter is not callable.
///
/// ```ts
/// declare function schedule(kind: "sync", task: () => void): void;
/// declare function schedule(kind: "async", task: () => Promise<void>): void;
/// schedule("sync", async () => {});
/// ```
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

/// Returns the callable type expected for one constructor argument.
///
/// All source arguments participate in signature selection, except that the
/// requested argument's type is ignored. A requested spread is still resolved
/// because its tuple shape determines parameter mapping. Returns `None` when
/// the callee or another argument type is unavailable, no supported constructor
/// matches, the requested index is out of bounds, or the selected parameter is
/// not one unambiguous callable type.
///
/// In this construction, `"sync"` selects the first signature. A request for
/// argument 1 returns `() => void`.
///
/// ```ts
/// declare const Job: {
///     new (kind: "sync", task: () => void): object;
///     new (kind: "async", task: () => Promise<void>): object;
/// };
/// new Job("sync", async () => {});
/// ```
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
