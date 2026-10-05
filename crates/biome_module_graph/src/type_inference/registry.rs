//! Stable categories used by type-inference profiling.
//!
//! Query families identify subject modules rather than individual query bodies.
//! Whole-module reasons identify the production transitions where an
//! analyzer-facing request widens from selective queries to complete tables.

/// Stable category shared by tracked queries in one inference module.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum TypeInferenceQueryKind {
    /// Looks up collected expression, binding, and local type data.
    Lookups,
    /// Resolves named, default, and re-exported module exports.
    Exports,
    /// Classifies Promise-like expressions, collections, and call results.
    Promises,
    /// Selects call or constructor signatures and projects argument types.
    Calls,
    /// Recursively resolves and simplifies inferred type structures.
    Normalization,
    /// Produces complete inferred-type tables for a module.
    ModuleTypes,
}

impl TypeInferenceQueryKind {
    /// Returns the stable profile ID for this query family.
    pub(crate) const fn id(self) -> &'static str {
        match self {
            Self::Lookups => "query.lookups",
            Self::Exports => "query.exports",
            Self::Promises => "query.promises",
            Self::Calls => "query.calls",
            Self::Normalization => "query.normalization",
            Self::ModuleTypes => "query.module-types",
        }
    }

    /// Returns the label rendered in human-readable profiles.
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Lookups => "Lookups",
            Self::Exports => "Exports",
            Self::Promises => "Promises",
            Self::Calls => "Calls",
            Self::Normalization => "Normalization",
            Self::ModuleTypes => "Module types",
        }
    }
}

/// Reason an analyzer-facing request widened to complete module tables.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum TypeInferenceWholeModuleReason {
    /// The requested import chain exhausted the on-demand traversal budget.
    ImportDepthLimit,
    /// Complete inference of one module required another module's complete tables.
    InternalDependency,
}

impl TypeInferenceWholeModuleReason {
    /// Returns the stable profile ID for this widening reason.
    pub(crate) const fn id(self) -> &'static str {
        match self {
            Self::ImportDepthLimit => "whole-module.import-depth-limit",
            Self::InternalDependency => "whole-module.internal-dependency",
        }
    }

    /// Returns the label rendered in human-readable profiles.
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::ImportDepthLimit => "Import depth limit",
            Self::InternalDependency => "Internal dependency",
        }
    }
}
