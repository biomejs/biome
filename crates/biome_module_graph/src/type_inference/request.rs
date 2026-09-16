//! Contracts and execution for type-inference requests.
//!
//! Each request defines one result and the source location that needs it.
//! Executing a request also records which analyzer made it and where the request
//! is implemented, so a profile can connect nested database work to its cause.

use biome_rowan::TextRange;

use crate::{ModuleDb, ModuleInfo};

/// Stable source coordinate for an inference implementation.
///
/// Profiles render this coordinate so a maintainer can locate the implementation
/// that produced a request, query, or whole-module record in the corresponding
/// Biome version. Construct references beside the implementation they identify
/// with `file!()` and `line!()`.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypeInferenceCodeReference {
    file: &'static str,
    line: u32,
    symbol: &'static str,
}

impl TypeInferenceCodeReference {
    /// Creates a source coordinate for `symbol` at the given Rust source location.
    pub(crate) const fn new(file: &'static str, line: u32, symbol: &'static str) -> Self {
        Self { file, line, symbol }
    }

    /// Returns the source file captured by `file!()`.
    pub(crate) const fn file(self) -> &'static str {
        self.file
    }

    /// Returns the one-based source line captured by `line!()`.
    pub(crate) const fn line(self) -> u32 {
        self.line
    }

    /// Returns the implementation symbol named by this coordinate.
    pub(crate) const fn symbol(self) -> &'static str {
        self.symbol
    }
}

/// Analyzer or service that initiated a type-inference request.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypeInferenceCaller {
    group: &'static str,
    name: &'static str,
}

impl TypeInferenceCaller {
    /// Creates an attribution from a caller group and name.
    pub const fn new(group: &'static str, name: &'static str) -> Self {
        Self { group, name }
    }

    /// Returns the caller group, such as a lint group.
    pub(crate) const fn group(self) -> &'static str {
        self.group
    }

    /// Returns the caller name within its group.
    pub(crate) const fn name(self) -> &'static str {
        self.name
    }
}

/// Source location that initiated a type-inference request.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct TypeInferenceRequestOrigin {
    module: ModuleInfo,
    range: TextRange,
}

impl TypeInferenceRequestOrigin {
    /// Creates an exact request origin in `module`.
    pub(crate) const fn new(module: ModuleInfo, range: TextRange) -> Self {
        Self { module, range }
    }

    /// Returns the module containing the request origin.
    pub(crate) const fn module(self) -> ModuleInfo {
        self.module
    }

    /// Returns the zero-based, half-open UTF-8 byte range of the request origin.
    pub(crate) const fn range(self) -> TextRange {
        self.range
    }
}

mod private {
    //! Request types cross crate boundaries, but only `biome_module_graph` may
    //! define them. This keeps the execution and profiling rules under this
    //! crate's control.

    /// Supertrait that restricts `TypeInferenceRequest` implementations to this crate.
    pub trait Sealed {}
}

/// Profile identity declared by one request type.
///
/// A runtime option cannot select a different identity. Inputs that produce
/// different result contracts require distinct request types that each
/// implement this trait.
pub trait TypeInferenceRequestMetadata {
    /// Stable maintenance identifier printed in verbose profiles.
    const ID: &'static str;

    /// Concise human-readable label printed in profiles.
    const LABEL: &'static str;
}

/// Defines one type-inference result available to analyzers.
///
/// Implementations use [`TypeInferenceRequestContext`] for database operations.
/// Consumers use [`execute_type_inference_request`] so profiling can attribute
/// the complete operation to one caller and source location.
pub trait TypeInferenceRequest<'db>: private::Sealed + TypeInferenceRequestMetadata {
    /// Result returned to the analyzer-facing caller.
    type Output;

    /// Source coordinate of this request type's canonical implementation.
    const IMPLEMENTATION: TypeInferenceCodeReference;

    /// Returns the exact source location that caused this request.
    fn origin(&self) -> TypeInferenceRequestOrigin;

    /// Executes the request by composing operations from `context`.
    fn execute(self, context: &TypeInferenceRequestContext<'db>) -> Self::Output;
}

/// Database available to every operation in one inference request.
pub struct TypeInferenceRequestContext<'db> {
    db: &'db dyn ModuleDb,
}

impl<'db> TypeInferenceRequestContext<'db> {
    pub(crate) const fn new(db: &'db dyn ModuleDb) -> Self {
        Self { db }
    }

    pub(crate) const fn db(&self) -> &'db dyn ModuleDb {
        self.db
    }
}

/// Executes a declared type-inference request.
///
/// This is the only production entry point for request implementations. Query
/// profiles inherit the caller and source origin from the profiling scope when
/// profiling is active. The disabled path executes the request without
/// constructing profiling metadata or timers.
pub fn execute_type_inference_request<'db, R>(
    db: &'db dyn ModuleDb,
    caller: TypeInferenceCaller,
    request: R,
) -> R::Output
where
    R: TypeInferenceRequest<'db>,
{
    let context = TypeInferenceRequestContext::new(db);
    if !super::profiling::is_recording() {
        return request.execute(&context);
    }

    super::profiling::profile_request::<R, _>(
        db,
        caller,
        request.origin(),
        R::IMPLEMENTATION,
        || request.execute(&context),
    )
}

pub(crate) use private::Sealed;
