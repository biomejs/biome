//! Bounded transformations of nested types.
//!
//! A transformation visits each type and its slots. The traversal is private;
//! callers use operations such as generic substitution.

use crate::interned_types::{TypeData, TypeDataSlotRebuilder, TypeDb};
use rustc_hash::FxHashSet;

pub(crate) const MAX_TYPE_SUBSTITUTION_STEPS: usize = 1024;

/// Replaces `generic` with `replacement` outside nested declarations of the
/// same generic parameter.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeSubstitution<'db> {
    /// Type to replace.
    pub generic: TypeData<'db>,
    /// Replacement type.
    pub replacement: TypeData<'db>,
}

impl<'db> TypeSubstitution<'db> {
    /// Removes an empty generic instantiation used as a substitution pattern.
    fn binder_generic(self, db: &'db dyn TypeDb) -> TypeData<'db> {
        if let TypeData::InstanceOf(instance) = self.generic
            && instance.type_parameters(db).is_empty()
            && matches!(instance.ty(db), TypeData::Generic(_))
        {
            instance.ty(db)
        } else {
            self.generic
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeTransformError {
    /// The transformation exceeded its step limit.
    StepLimitExceeded,
    /// The replacements did not match the extracted slots.
    InvalidRebuild,
}

impl std::fmt::Display for TypeTransformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::StepLimitExceeded => "type transformation exceeded its step limit",
            Self::InvalidRebuild => "replacement types could not rebuild their parent",
        })
    }
}

impl std::error::Error for TypeTransformError {}

/// Result of transforming a type and its nested slots.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "a type transformation failure must be handled"]
pub enum TypeTransformResult<T> {
    /// Every required type was transformed.
    Transformed(T),
    /// The transformation exceeded its step limit.
    LimitExceeded,
    /// The replacements did not match the extracted slots.
    InvalidRebuild,
}

impl<T> TypeTransformResult<T> {
    /// Applies `map` to a transformed value while preserving failures.
    pub fn map<U>(self, map: impl FnOnce(T) -> U) -> TypeTransformResult<U> {
        match self {
            Self::Transformed(value) => TypeTransformResult::Transformed(map(value)),
            Self::LimitExceeded => TypeTransformResult::LimitExceeded,
            Self::InvalidRebuild => TypeTransformResult::InvalidRebuild,
        }
    }

    /// Applies `map` to the transformed value, or returns `default` on failure.
    pub fn map_or<U>(self, default: U, map: impl FnOnce(T) -> U) -> U {
        match self {
            Self::Transformed(value) => map(value),
            Self::LimitExceeded | Self::InvalidRebuild => default,
        }
    }

    /// Applies `map` to the transformed value, or computes a fallback on failure.
    pub fn map_or_else<U>(self, default: impl FnOnce() -> U, map: impl FnOnce(T) -> U) -> U {
        match self {
            Self::Transformed(value) => map(value),
            Self::LimitExceeded | Self::InvalidRebuild => default(),
        }
    }

    /// Returns the transformed value.
    ///
    /// # Panics
    ///
    /// Panics if the transformation failed.
    pub fn unwrap(self) -> T {
        self.expect("type transformation failed")
    }

    /// Returns the transformed value using `message` if the transformation failed.
    ///
    /// # Panics
    ///
    /// Panics if the transformation failed.
    pub fn expect(self, message: &str) -> T {
        match self {
            Self::Transformed(value) => value,
            Self::LimitExceeded => panic!("{message}: step limit exceeded"),
            Self::InvalidRebuild => panic!("{message}: invalid slot reconstruction"),
        }
    }

    /// Returns whether every required type was transformed.
    pub const fn is_transformed(&self) -> bool {
        matches!(self, Self::Transformed(_))
    }

    /// Converts this domain-specific result into a standard [`Result`].
    pub fn into_result(self) -> Result<T, TypeTransformError> {
        match self {
            Self::Transformed(value) => Ok(value),
            Self::LimitExceeded => Err(TypeTransformError::StepLimitExceeded),
            Self::InvalidRebuild => Err(TypeTransformError::InvalidRebuild),
        }
    }
}

impl<T> From<Result<T, TypeTransformError>> for TypeTransformResult<T> {
    fn from(result: Result<T, TypeTransformError>) -> Self {
        match result {
            Ok(value) => Self::Transformed(value),
            Err(TypeTransformError::StepLimitExceeded) => Self::LimitExceeded,
            Err(TypeTransformError::InvalidRebuild) => Self::InvalidRebuild,
        }
    }
}

enum TypeTransformEvent<'db> {
    Enter(TypeData<'db>),
    Rebuild {
        rebuilder: TypeDataSlotRebuilder<'db>,
    },
    Exit {
        source: TypeData<'db>,
        transformed: TypeData<'db>,
    },
}

pub(crate) enum TypeTransformAction<'db> {
    Descend(TypeData<'db>),
    Replace(TypeData<'db>),
}

pub(crate) trait TypeTransform<'db> {
    /// Chooses whether to visit a type's slots or replace the type directly.
    fn enter(&mut self, db: &'db dyn TypeDb, ty: TypeData<'db>) -> TypeTransformAction<'db>;

    /// Transforms a type after its slots have been transformed.
    fn leave(&mut self, db: &'db dyn TypeDb, ty: TypeData<'db>) -> TypeData<'db>;
}

pub(crate) struct TypeDataTransformer {
    remaining_steps: usize,
}

impl TypeDataTransformer {
    pub(crate) fn new(max_steps: usize) -> Self {
        Self {
            remaining_steps: max_steps,
        }
    }

    /// Transforms `root` and its nested slots.
    ///
    /// Each visited type consumes one step. Reusing this transformer shares the
    /// remaining steps across roots. A type already being transformed is reused
    /// without consuming another step.
    ///
    /// Slot extraction and reconstruction must agree on slot count and order.
    pub(crate) fn transform<'db>(
        &mut self,
        root: TypeData<'db>,
        db: &'db dyn TypeDb,
        operation: &mut impl TypeTransform<'db>,
    ) -> TypeTransformResult<TypeData<'db>> {
        let mut stack = Vec::from([TypeTransformEvent::Enter(root)]);
        let mut results = Vec::new();
        let mut active = FxHashSet::default();

        while let Some(event) = stack.pop() {
            match event {
                TypeTransformEvent::Enter(source) => {
                    if active.contains(&source) {
                        results.push(source);
                        continue;
                    }
                    if self.remaining_steps == 0 {
                        return TypeTransformResult::LimitExceeded;
                    }
                    self.remaining_steps -= 1;

                    let transformed = match operation.enter(db, source) {
                        TypeTransformAction::Replace(transformed) => {
                            results.push(transformed);
                            continue;
                        }
                        TypeTransformAction::Descend(transformed) => transformed,
                    };
                    let slots = transformed.type_slots(db);
                    let slot_count = slots.len();
                    if slot_count == 0 {
                        results.push(operation.leave(db, transformed));
                        continue;
                    }

                    active.insert(source);
                    active.insert(transformed);
                    let queued_entries = stack
                        .iter()
                        .filter(|event| matches!(event, TypeTransformEvent::Enter(_)))
                        .count();
                    let available_steps = self.remaining_steps.saturating_sub(queued_entries);
                    let new_types = slots.iter().filter(|ty| !active.contains(ty)).count();
                    if new_types > available_steps {
                        return TypeTransformResult::LimitExceeded;
                    }

                    let (rebuilder, pending_types) = slots.into_parts();
                    stack.push(TypeTransformEvent::Exit {
                        source,
                        transformed,
                    });
                    stack.push(TypeTransformEvent::Rebuild { rebuilder });
                    stack.extend(
                        pending_types
                            .into_iter()
                            .rev()
                            .map(TypeTransformEvent::Enter),
                    );
                }
                TypeTransformEvent::Rebuild { rebuilder } => {
                    let slot_count = rebuilder.len();
                    let Some(start) = results.len().checked_sub(slot_count) else {
                        return TypeTransformResult::InvalidRebuild;
                    };
                    let replacements = results.split_off(start);
                    match rebuilder.rebuild(db, replacements) {
                        TypeTransformResult::Transformed(rebuilt) => {
                            results.push(operation.leave(db, rebuilt));
                        }
                        TypeTransformResult::LimitExceeded => {
                            return TypeTransformResult::LimitExceeded;
                        }
                        TypeTransformResult::InvalidRebuild => {
                            return TypeTransformResult::InvalidRebuild;
                        }
                    }
                }
                TypeTransformEvent::Exit {
                    source,
                    transformed,
                } => {
                    active.remove(&source);
                    active.remove(&transformed);
                }
            }
        }

        match results.as_slice() {
            [result] => TypeTransformResult::Transformed(*result),
            _ => TypeTransformResult::InvalidRebuild,
        }
    }
}

pub(crate) struct TypeSubstituter<'db> {
    substitution: TypeSubstitution<'db>,
    binder_generic: TypeData<'db>,
}

impl<'db> TypeSubstituter<'db> {
    pub(crate) fn new(db: &'db dyn TypeDb, substitution: TypeSubstitution<'db>) -> Self {
        Self {
            substitution,
            binder_generic: substitution.binder_generic(db),
        }
    }

    pub(crate) fn substitute(
        &mut self,
        transformer: &mut TypeDataTransformer,
        db: &'db dyn TypeDb,
        ty: TypeData<'db>,
    ) -> TypeTransformResult<TypeData<'db>> {
        transformer.transform(ty, db, self)
    }
}

impl<'db> TypeTransform<'db> for TypeSubstituter<'db> {
    fn enter(&mut self, db: &'db dyn TypeDb, ty: TypeData<'db>) -> TypeTransformAction<'db> {
        if ty.declares_generic(db, self.binder_generic) {
            TypeTransformAction::Replace(ty)
        } else if ty == self.substitution.generic {
            TypeTransformAction::Replace(self.substitution.replacement)
        } else {
            TypeTransformAction::Descend(ty)
        }
    }

    fn leave(&mut self, _db: &'db dyn TypeDb, ty: TypeData<'db>) -> TypeData<'db> {
        ty
    }
}

impl<'db> TypeData<'db> {
    /// Substitutes a generic throughout this type, excluding nested types that
    /// declare the same generic parameter.
    pub fn substitute_type(
        self,
        db: &'db dyn TypeDb,
        substitution: TypeSubstitution<'db>,
    ) -> TypeTransformResult<Self> {
        let mut transformer = TypeDataTransformer::new(MAX_TYPE_SUBSTITUTION_STEPS);
        TypeSubstituter::new(db, substitution).substitute(&mut transformer, db, self)
    }

    /// Substitutes inside the body of this type while preserving generic
    /// parameters declared by the root and by nested binders.
    pub fn substitute_type_in_root_body(
        self,
        db: &'db dyn TypeDb,
        substitution: TypeSubstitution<'db>,
    ) -> TypeTransformResult<Self> {
        if !self.declares_generic(db, substitution.binder_generic(db)) {
            return self.substitute_type(db, substitution);
        }

        let root_type_parameter_count = self.declared_type_parameters(db).map_or(0, <[_]>::len);
        let slots = self.type_slots(db);
        let mut replacements = Vec::with_capacity(slots.len());
        let mut transformer = TypeDataTransformer::new(MAX_TYPE_SUBSTITUTION_STEPS);
        let mut substituter = TypeSubstituter::new(db, substitution);
        for (index, ty) in slots.iter().enumerate() {
            if index < root_type_parameter_count {
                replacements.push(ty);
            } else {
                match substituter.substitute(&mut transformer, db, ty) {
                    TypeTransformResult::Transformed(ty) => replacements.push(ty),
                    TypeTransformResult::LimitExceeded => {
                        return TypeTransformResult::LimitExceeded;
                    }
                    TypeTransformResult::InvalidRebuild => {
                        return TypeTransformResult::InvalidRebuild;
                    }
                }
            }
        }
        slots.rebuild(db, replacements)
    }

    /// Returns the generic parameters declared directly by this type.
    ///
    /// Classes, constructors, functions, and interfaces are generic binders.
    /// `Some(&[])` identifies one of those binders without parameters, while
    /// `None` identifies a type that cannot declare generic parameters.
    fn declared_type_parameters(self, db: &'db dyn TypeDb) -> Option<&'db [Self]> {
        match self {
            Self::Class(class) => Some(class.type_parameters(db)),
            Self::Constructor(constructor) => Some(constructor.type_parameters(db)),
            Self::Function(function) => Some(function.type_parameters(db)),
            Self::Interface(interface) => Some(interface.type_parameters(db)),
            _ => None,
        }
    }

    fn declares_generic(self, db: &'db dyn TypeDb, generic: Self) -> bool {
        self.declared_type_parameters(db)
            .is_some_and(|parameters| parameters.contains(&generic))
    }
}

#[cfg(test)]
mod tests {
    use super::TypeTransformResult;

    #[test]
    #[should_panic(expected = "type transformation failed: step limit exceeded")]
    fn unwrap_panics_on_limit_exceeded() {
        let result: TypeTransformResult<()> = TypeTransformResult::LimitExceeded;
        result.unwrap();
    }

    #[test]
    #[should_panic(expected = "substitution failed: invalid slot reconstruction")]
    fn expect_panics_on_invalid_rebuild() {
        let result: TypeTransformResult<()> = TypeTransformResult::InvalidRebuild;
        result.expect("substitution failed");
    }
}
