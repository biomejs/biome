use biome_rowan::Text;

use crate::{Type, TypeInner, TypeReferenceQualifier};

const MAX_RESOLUTION_DEPTH: usize = 16;

/// Trait for implementing type resolution.
///
/// In Biome, we define three levels of type inference:
/// - **Local inference.** Constrained to the expression or statement from which
///   the type is inferred. Doesn't perform any type resolution.
/// - **Thin**, or module-level, type inference. Can perform type resolution as
///   long as the referenced types are defined in the same module.
/// - **Full inference**. Can perform type resolution across modules.
///
/// Since both thin inference and full inference rely on type resolution, we
/// also have two layers of type *resolution*, both of which implement this
/// trait.
pub trait TypeResolver {
    /// Resolves a type by its `qualifier`.
    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<Type>;

    /// Resolves the type of a value by its `identifier`.
    fn resolve_type_of(&self, identifier: &Text) -> Option<Type>;
}

impl Type {
    /// Attempts to resolve all unresolved types using the given resolver.
    pub fn resolve(&mut self, resolver: &dyn TypeResolver) {
        let stack = &[&**self];
        *self = if self.needs_resolving(resolver, stack) {
            self.resolved(resolver, stack)
        } else {
            self.flattened(resolver, stack)
        };
    }
}

/// Trait to be implemented by `Type` and its subtypes to aid the resolver.
pub(crate) trait Resolvable {
    /// Returns whether (part of) the type can and should be resolved.
    fn needs_resolving(&self, resolver: &dyn TypeResolver, stack: &[&TypeInner]) -> bool;

    /// Returns the resolved version of this type.
    ///
    /// You should first call `Self::needs_resolving()` to avoid making
    /// needless clones.
    fn resolved(&self, resolver: &dyn TypeResolver, stack: &[&TypeInner]) -> Self;
}

impl Resolvable for Type {
    fn needs_resolving(&self, resolver: &dyn TypeResolver, stack: &[&TypeInner]) -> bool {
        let inner = &**self;
        if stack.len() >= MAX_RESOLUTION_DEPTH
            || stack[0..stack.len().saturating_sub(1)].contains(&inner)
        {
            return false;
        }

        let mut stack = stack.to_vec();
        stack.push(inner);
        inner.needs_resolving(resolver, &stack)
    }

    /// Returns the resolved version of this type.
    ///
    /// You should first call `Self::needs_resolving()` to avoid making
    /// needless clones.
    fn resolved(&self, resolver: &dyn TypeResolver, stack: &[&TypeInner]) -> Self {
        let inner = &**self;
        if stack.len() >= MAX_RESOLUTION_DEPTH
            || stack[0..stack.len().saturating_sub(1)].contains(&inner)
        {
            return self.clone();
        }

        let mut stack = stack.to_vec();
        stack.push(inner);
        Self::from(inner.resolved(resolver, &stack)).flattened(resolver, &stack)
    }
}

macro_rules! derive_primitive_resolved {
    ($($ty:ty),+) => {
        $(impl Resolvable for $ty {
            fn needs_resolving(&self, _resolver: &dyn TypeResolver, _stack: &[&TypeInner]) -> bool {
                false
            }

            fn resolved(&self, _resolver: &dyn TypeResolver, _stack: &[&TypeInner]) -> Self {
                *self
            }
        })+
    };
}

derive_primitive_resolved!(bool, f64, u64, usize);
