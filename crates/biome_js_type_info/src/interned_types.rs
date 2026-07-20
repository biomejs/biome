//! Salsa-backed type data for the resolved type-inference world.
//!
//! The existing `type_data` module remains the raw, collector-side representation
//! for now. This module introduces the interned resolved representation that
//! later phases will wire into module inference.

use biome_js_syntax::numbers::canonicalize_js_bigint_literal;
use biome_rowan::Text;
use rustc_hash::FxHashSet;
use std::ops::ControlFlow;

use crate::{
    ScopeId,
    builders::{IntersectionBuilder, UnionBuilder},
    globals_ids::{
        ARRAY_ID_GLOBAL_TYPE_ID, ASYNC_DISPOSABLE_ID_GLOBAL_TYPE_ID, DATE_ID_GLOBAL_TYPE_ID,
        DISPOSABLE_ID_GLOBAL_TYPE_ID, ERROR_ID_GLOBAL_TYPE_ID, GLOBAL_ARRAY_ID,
        GLOBAL_ASYNC_DISPOSABLE_ID, GLOBAL_BOOLEAN_ID, GLOBAL_CONDITIONAL_ID, GLOBAL_DATE_ID,
        GLOBAL_DISPOSABLE_ID, GLOBAL_ERROR_ID, GLOBAL_GLOBAL_ID, GLOBAL_MAP_ID, GLOBAL_NUMBER_ID,
        GLOBAL_PROMISE_ID, GLOBAL_REGEXP_ID, GLOBAL_SET_ID, GLOBAL_STRING_ID,
        GLOBAL_SYMBOL_ASYNC_DISPOSE_ID, GLOBAL_SYMBOL_DISPOSE_ID, GLOBAL_SYMBOL_ID,
        GLOBAL_UNDEFINED_ID, GLOBAL_UNKNOWN_ID, GLOBAL_VOID_ID, GLOBAL_WEAK_MAP_ID, GlobalTypeId,
        MAP_ID_GLOBAL_TYPE_ID, PROMISE_ID_GLOBAL_TYPE_ID, REGEXP_ID_GLOBAL_TYPE_ID,
        SET_ID_GLOBAL_TYPE_ID, SYMBOL_ID_GLOBAL_TYPE_ID, WEAK_MAP_ID_GLOBAL_TYPE_ID,
    },
    literal::{BooleanLiteral, NumberLiteral, RegexpLiteral, StringLiteral},
    type_data as raw,
};

pub type RawTypeData = raw::TypeData;
pub type ReferenceResolver<'db, 'resolver> =
    dyn FnMut(&raw::TypeReference) -> TypeData<'db> + 'resolver;
const MAX_GENERIC_REPLACEMENT_STEPS: usize = 64;
const MAX_TYPE_SUBSTITUTION_STEPS: usize = 1024;

pub fn well_known_symbol_name(reference: &raw::TypeReference) -> Option<Text> {
    match reference {
        raw::TypeReference::Resolved(id) if *id == GLOBAL_SYMBOL_DISPOSE_ID => {
            Some(Text::new_static("Symbol.dispose"))
        }
        raw::TypeReference::Resolved(id) if *id == GLOBAL_SYMBOL_ASYNC_DISPOSE_ID => {
            Some(Text::new_static("Symbol.asyncDispose"))
        }
        raw::TypeReference::Qualifier(qualifier) => {
            let mut parts = qualifier.path.iter();
            match (parts.next(), parts.next(), parts.next()) {
                (Some(symbol), Some(member), None)
                    if symbol.text() == "Symbol"
                        && matches!(member.text(), "dispose" | "asyncDispose") =>
                {
                    Some(Text::new_owned(format!("Symbol.{}", member.text()).into()))
                }
                _ => None,
            }
        }
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeSubstitution<'db> {
    pub generic: TypeData<'db>,
    pub replacement: TypeData<'db>,
}

#[derive(Clone, Copy, Debug)]
enum StructuralTypeMapItem<'db> {
    Enter(TypeData<'db>),
    Rebuild(TypeData<'db>, usize),
    Exit(TypeData<'db>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StructuralMapError {
    /// The caller must degrade the incomplete result to `Unknown`.
    StepLimitExceeded,
    /// A child sequence could not rebuild the original structural shape.
    InvalidRebuild,
}

impl std::fmt::Display for StructuralMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::StepLimitExceeded => "structural type traversal exceeded its step limit",
            Self::InvalidRebuild => "structural type children could not rebuild their parent",
        })
    }
}

impl std::error::Error for StructuralMapError {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, salsa::Update)]
pub struct ModuleKey {
    id: salsa::Id,
}

impl ModuleKey {
    pub fn new(id: salsa::Id) -> Self {
        Self { id }
    }

    pub fn as_id(self) -> salsa::Id {
        self.id
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, salsa::Update)]
pub struct LocalTypeId {
    index: u32,
}

impl LocalTypeId {
    pub fn new(index: usize) -> Self {
        Self {
            index: index as u32,
        }
    }

    pub const fn index(self) -> usize {
        self.index as usize
    }
}

#[salsa::db]
pub trait TypeDb: biome_db::Db {
    fn local_type_name(&self, module: ModuleKey, type_id: LocalTypeId) -> Option<Text> {
        let _ = (module, type_id);
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct DivergentType {
    pub id: salsa::Id,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, salsa::Update)]
pub enum TypeData<'db> {
    #[default]
    Unknown,
    Divergent(DivergentType),
    Global,
    BigInt,
    Boolean,
    Null,
    Number,
    String,
    Symbol,
    Undefined,
    Conditional,
    Class(InternedClass<'db>),
    Constructor(InternedConstructor<'db>),
    Function(InternedFunction<'db>),
    Interface(InternedInterface<'db>),
    Module(InternedModule<'db>),
    Namespace(InternedNamespace<'db>),
    Object(InternedObject<'db>),
    Tuple(InternedTuple<'db>),
    Generic(InternedGenericTypeParameter<'db>),
    Local(LocalTypeHandle<'db>),
    GlobalType(GlobalTypeId),
    Intersection(InternedIntersection<'db>),
    Union(InternedUnion<'db>),
    TypeOperator(InternedTypeOperatorType<'db>),
    Literal(InternedLiteral<'db>),
    InstanceOf(InternedTypeInstance<'db>),
    MergedReference(InternedMergedReference<'db>),
    TypeofExpression(InternedTypeofExpression<'db>),
    TypeofType(InternedTypeofType<'db>),
    TypeofValue(InternedTypeofValue<'db>),
    AnyKeyword,
    NeverKeyword,
    ObjectKeyword,
    ThisKeyword,
    UnknownKeyword,
    VoidKeyword,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ConditionalType {
    Anything,
    Falsy,
    FalsyButNotNullish,
    NonNullish,
    Nullish,
    Truthy,
    Unknown,
}

impl ConditionalType {
    pub fn is_falsy(self) -> bool {
        matches!(self, Self::Falsy | Self::FalsyButNotNullish | Self::Nullish)
    }

    pub fn is_inferred(self) -> bool {
        !matches!(self, Self::Unknown)
    }

    pub fn is_non_nullish(self) -> bool {
        matches!(
            self,
            Self::FalsyButNotNullish | Self::NonNullish | Self::Truthy
        )
    }

    pub fn is_nullish(self) -> bool {
        matches!(self, Self::Nullish)
    }

    pub fn is_truthy(self) -> bool {
        matches!(self, Self::Truthy)
    }

    pub fn is_mergeable(self) -> bool {
        !matches!(self, Self::Anything | Self::Unknown)
    }

    pub fn merged_with(self, other: Self) -> Self {
        match (self, other) {
            (Self::Anything, _)
            | (_, Self::Anything)
            | (Self::Falsy | Self::Nullish, Self::NonNullish)
            | (Self::Falsy | Self::FalsyButNotNullish | Self::Nullish, Self::Truthy)
            | (Self::NonNullish, Self::Falsy | Self::Nullish)
            | (Self::Truthy, Self::Falsy | Self::FalsyButNotNullish | Self::Nullish) => {
                Self::Anything
            }
            (Self::Falsy, Self::Falsy | Self::FalsyButNotNullish | Self::Nullish)
            | (Self::FalsyButNotNullish | Self::Nullish, Self::Falsy)
            | (Self::FalsyButNotNullish, Self::Nullish)
            | (Self::Nullish, Self::FalsyButNotNullish) => Self::Falsy,
            (Self::FalsyButNotNullish, Self::FalsyButNotNullish) => Self::FalsyButNotNullish,
            (Self::NonNullish, Self::FalsyButNotNullish | Self::NonNullish | Self::Truthy)
            | (Self::FalsyButNotNullish | Self::Truthy, Self::NonNullish) => Self::NonNullish,
            (Self::Nullish, Self::Nullish) => Self::Nullish,
            (Self::Truthy, Self::Truthy) => Self::Truthy,
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ConditionalSubset {
    Falsy,
    Truthy,
    NonNullish,
}

impl<'db> TypeData<'db> {
    pub fn divergent(id: salsa::Id) -> Self {
        Self::Divergent(DivergentType { id })
    }

    pub fn is_string_key_type(self, db: &'db dyn TypeDb) -> bool {
        match self {
            Self::String => true,
            Self::InstanceOf(instance) => instance.ty(db) == Self::String,
            _ => false,
        }
    }

    pub fn is_string_literal_key(self, db: &'db dyn TypeDb, name: &str) -> bool {
        matches!(
            self,
            Self::Literal(literal)
                if matches!(literal.literal(db), Literal::String(string) if string.as_str() == name)
        )
    }

    pub(crate) fn is_boolean_literal(self, db: &'db dyn TypeDb, value: bool) -> bool {
        matches!(
            self,
            Self::Literal(literal)
                if matches!(literal.literal(db), Literal::Boolean(boolean) if boolean.as_bool() == value)
        )
    }

    pub(crate) fn literal_base_type(self, db: &'db dyn TypeDb) -> Option<Self> {
        let Self::Literal(literal) = self else {
            return None;
        };

        match literal.literal(db) {
            Literal::BigInt(_) => Some(Self::BigInt),
            Literal::Boolean(_) => Some(Self::Boolean),
            Literal::Number(_) => Some(Self::Number),
            Literal::String(_) | Literal::Template(_) => Some(Self::String),
            Literal::Object(_) | Literal::RegExp(_) => None,
        }
    }

    pub(crate) fn is_primitive(self, db: &'db dyn TypeDb) -> bool {
        match self {
            Self::BigInt
            | Self::Boolean
            | Self::Null
            | Self::Number
            | Self::String
            | Self::Symbol
            | Self::Undefined => true,
            Self::Literal(literal) => matches!(
                literal.literal(db),
                Literal::BigInt(_)
                    | Literal::Boolean(_)
                    | Literal::Number(_)
                    | Literal::String(_)
                    | Literal::Template(_)
            ),
            _ => false,
        }
    }

    pub fn conditional_type_shallow(self, db: &'db dyn TypeDb) -> Option<ConditionalType> {
        match self {
            Self::AnyKeyword
            | Self::Conditional
            | Self::NeverKeyword
            | Self::ThisKeyword
            | Self::Unknown
            | Self::UnknownKeyword => Some(ConditionalType::Anything),
            Self::BigInt | Self::Boolean | Self::Interface(_) | Self::Number | Self::String => {
                Some(ConditionalType::NonNullish)
            }
            Self::Class(_)
            | Self::Constructor(_)
            | Self::Function(_)
            | Self::Global
            | Self::Module(_)
            | Self::Namespace(_)
            | Self::Object(_)
            | Self::ObjectKeyword
            | Self::Symbol
            | Self::Tuple(_) => Some(ConditionalType::Truthy),
            Self::Literal(literal) => Some(match literal.literal(db) {
                Literal::BigInt(text) => match canonicalize_js_bigint_literal(text.text()) {
                    Some(text) if text == "0n" => ConditionalType::FalsyButNotNullish,
                    Some(_) => ConditionalType::Truthy,
                    None => ConditionalType::Anything,
                },
                Literal::Boolean(boolean) => {
                    if boolean.as_bool() {
                        ConditionalType::Truthy
                    } else {
                        ConditionalType::FalsyButNotNullish
                    }
                }
                Literal::Number(number) => match number.to_f64() {
                    Some(number) if number == 0. || number.is_nan() => {
                        ConditionalType::FalsyButNotNullish
                    }
                    Some(_) => ConditionalType::Truthy,
                    None => ConditionalType::Anything,
                },
                Literal::Object(_) | Literal::RegExp(_) => ConditionalType::Truthy,
                Literal::String(string) => {
                    if string.as_str().is_empty() {
                        ConditionalType::FalsyButNotNullish
                    } else {
                        ConditionalType::Truthy
                    }
                }
                Literal::Template(_) => ConditionalType::Anything,
            }),
            Self::Null | Self::Undefined | Self::VoidKeyword => Some(ConditionalType::Nullish),
            Self::Divergent(_)
            | Self::Generic(_)
            | Self::GlobalType(_)
            | Self::Local(_)
            | Self::TypeOperator(_)
            | Self::TypeofType(_)
            | Self::TypeofValue(_)
            | Self::InstanceOf(_)
            | Self::Intersection(_)
            | Self::MergedReference(_)
            | Self::TypeofExpression(_)
            | Self::Union(_) => None,
        }
    }

    pub fn should_flatten_instance(self, type_parameters: &[Self]) -> bool {
        match self {
            Self::AnyKeyword
            | Self::BigInt
            | Self::Boolean
            | Self::Conditional
            | Self::Global
            | Self::Literal(_)
            | Self::Module(_)
            | Self::Namespace(_)
            | Self::NeverKeyword
            | Self::Null
            | Self::Number
            | Self::ObjectKeyword
            | Self::String
            | Self::Symbol
            | Self::ThisKeyword
            | Self::Undefined
            | Self::Unknown
            | Self::UnknownKeyword
            | Self::VoidKeyword => true,
            Self::Constructor(_)
            | Self::Function(_)
            | Self::InstanceOf(_)
            | Self::Interface(_)
            | Self::Intersection(_)
            | Self::Object(_)
            | Self::Tuple(_)
            | Self::Union(_) => type_parameters.is_empty(),
            Self::Class(_)
            | Self::Divergent(_)
            | Self::Generic(_)
            | Self::GlobalType(_)
            | Self::Local(_)
            | Self::MergedReference(_)
            | Self::TypeOperator(_)
            | Self::TypeofExpression(_)
            | Self::TypeofType(_)
            | Self::TypeofValue(_) => false,
        }
    }

    pub fn is_promise_instance(self, db: &'db dyn TypeDb) -> bool {
        let Self::InstanceOf(instance) = self else {
            return false;
        };

        instance.ty(db).is_promise_class(db)
    }

    pub fn is_array_class(self, db: &'db dyn TypeDb) -> bool {
        self == Self::GlobalType(ARRAY_ID_GLOBAL_TYPE_ID)
            || self.is_builtin_class_named(db, "Array")
    }

    pub fn is_promise_class(self, db: &'db dyn TypeDb) -> bool {
        self == Self::GlobalType(PROMISE_ID_GLOBAL_TYPE_ID)
            || self.is_builtin_class_named(db, "Promise")
    }

    fn is_builtin_class_named(self, db: &'db dyn TypeDb, expected_name: &str) -> bool {
        match self {
            Self::Class(class) if class.is_builtin(db) => class
                .name(db)
                .as_ref()
                .is_some_and(|name| name.text() == expected_name),
            _ => false,
        }
    }

    pub fn is_generic_reference(self, db: &'db dyn TypeDb) -> bool {
        match self {
            Self::Generic(_) => true,
            Self::InstanceOf(instance) => matches!(instance.ty(db), Self::Generic(_)),
            _ => false,
        }
    }

    pub fn callable_function(self, db: &'db dyn TypeDb) -> Option<InternedFunction<'db>> {
        match self {
            Self::Function(function) => Some(function),
            Self::InstanceOf(instance) => match instance.ty(db) {
                Self::Function(function) => Some(function),
                _ => None,
            },
            _ => None,
        }
    }

    /// Compares this type pattern with an actual argument type and returns the
    /// generic replacements needed to make the pattern match the actual type.
    ///
    /// For example, comparing the pattern `Promise<T>` with the actual type
    /// `Promise<string>` returns a replacement where `generic` is `T` and
    /// `replacement` is `string`.
    ///
    /// The walk is iterative and returns `None` if it cannot finish within a
    /// fixed number of steps. Already-seen edges can still complete after the
    /// budget is consumed, so cyclic shapes terminate without false exhaustion.
    pub fn collect_generic_replacements(
        self,
        db: &'db dyn TypeDb,
        actual: Self,
    ) -> Option<Vec<TypeSubstitution<'db>>> {
        let mut replacements = Vec::new();
        let mut stack = Vec::from([(self, actual)]);
        let mut seen = FxHashSet::default();
        let mut remaining_steps = MAX_GENERIC_REPLACEMENT_STEPS;

        while let Some((pattern, actual)) = stack.pop() {
            if !seen.insert((pattern, actual)) {
                continue;
            }
            if remaining_steps == 0 {
                return None;
            }
            remaining_steps -= 1;

            if pattern.is_generic_reference(db) {
                replacements.push(TypeSubstitution {
                    generic: pattern,
                    replacement: actual,
                });
                continue;
            }

            let (Self::InstanceOf(pattern), Self::InstanceOf(actual)) = (pattern, actual) else {
                continue;
            };

            for (pattern, actual) in pattern
                .type_parameters(db)
                .iter()
                .zip(actual.type_parameters(db))
                .rev()
            {
                stack.push((*pattern, *actual));
            }
            stack.push((pattern.ty(db), actual.ty(db)));
        }

        Some(replacements)
    }

    /// Substitutes a generic throughout a type, stopping below nested binders.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralMapError::StepLimitExceeded`] if traversal exhausts
    /// its budget, or [`StructuralMapError::InvalidRebuild`] if transformed
    /// children cannot reconstruct their parent type.
    pub fn substitute_type(
        self,
        db: &'db dyn TypeDb,
        substitution: TypeSubstitution<'db>,
    ) -> Result<Self, StructuralMapError> {
        let mut remaining_steps = MAX_TYPE_SUBSTITUTION_STEPS;
        self.substitute_type_with_budget(db, substitution, &mut remaining_steps)
    }

    fn substitute_type_with_budget(
        self,
        db: &'db dyn TypeDb,
        substitution: TypeSubstitution<'db>,
        remaining_steps: &mut usize,
    ) -> Result<Self, StructuralMapError> {
        let binder_generic = substitution.binder_generic(db);
        self.try_map_structural_with_budget(
            db,
            remaining_steps,
            |ty| {
                if ty.declares_generic(db, binder_generic) {
                    ControlFlow::Break(ty)
                } else if ty == substitution.generic {
                    ControlFlow::Break(substitution.replacement)
                } else {
                    ControlFlow::Continue(ty)
                }
            },
            |ty| ty,
        )
    }

    /// Substitutes inside the root binder while respecting nested binders.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralMapError::StepLimitExceeded`] if traversal exhausts
    /// its budget, or [`StructuralMapError::InvalidRebuild`] if transformed
    /// children cannot reconstruct their parent type.
    pub fn substitute_type_in_root_body(
        self,
        db: &'db dyn TypeDb,
        substitution: TypeSubstitution<'db>,
    ) -> Result<Self, StructuralMapError> {
        if !self.declares_generic(db, substitution.binder_generic(db)) {
            return self.substitute_type(db, substitution);
        }

        let mut remaining_steps = MAX_TYPE_SUBSTITUTION_STEPS;
        if structural_type_child_count(db, self) > remaining_steps {
            return Err(StructuralMapError::StepLimitExceeded);
        }
        let root_type_parameter_count = declared_type_parameters(db, self).map_or(0, <[_]>::len);
        let children = structural_type_children(db, self)
            .into_iter()
            .enumerate()
            .map(|(index, child)| {
                if index < root_type_parameter_count {
                    Ok(child)
                } else {
                    child.substitute_type_with_budget(db, substitution, &mut remaining_steps)
                }
            })
            .collect::<Result<_, _>>()?;
        rebuild_structural_type(db, self, children).ok_or(StructuralMapError::InvalidRebuild)
    }

    fn declares_generic(self, db: &'db dyn TypeDb, generic: Self) -> bool {
        declared_type_parameters(db, self).is_some_and(|parameters| parameters.contains(&generic))
    }

    /// Iteratively maps this type and all structurally nested types.
    ///
    /// `map` runs before children are visited. Returning `Break` replaces a
    /// complete subtree, while `Continue` descends into the returned type.
    /// `finish` runs after a descended type has been rebuilt.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralMapError::StepLimitExceeded`] when `max_steps` is
    /// exhausted, or [`StructuralMapError::InvalidRebuild`] when mapped children
    /// cannot reconstruct their parent type.
    pub fn try_map_structural(
        self,
        db: &'db dyn TypeDb,
        max_steps: usize,
        map: impl FnMut(Self) -> ControlFlow<Self, Self>,
        finish: impl FnMut(Self) -> Self,
    ) -> Result<Self, StructuralMapError> {
        let mut remaining_steps = max_steps;
        self.try_map_structural_with_budget(db, &mut remaining_steps, map, finish)
    }

    fn try_map_structural_with_budget(
        self,
        db: &'db dyn TypeDb,
        remaining_steps: &mut usize,
        mut map: impl FnMut(Self) -> ControlFlow<Self, Self>,
        mut finish: impl FnMut(Self) -> Self,
    ) -> Result<Self, StructuralMapError> {
        // Count every child before adding it to the stack. This prevents the
        // walk from starting a group of children that it cannot finish with
        // the steps that remain. A child that points to an active parent reuses
        // that parent, so following that link does not use another step.
        let mut stack = Vec::from([StructuralTypeMapItem::Enter(self)]);
        let mut results = Vec::new();
        let mut active = FxHashSet::default();

        while let Some(item) = stack.pop() {
            match item {
                StructuralTypeMapItem::Enter(ty) => {
                    if active.contains(&ty) {
                        results.push(ty);
                        continue;
                    }

                    if *remaining_steps == 0 {
                        return Err(StructuralMapError::StepLimitExceeded);
                    }
                    *remaining_steps -= 1;

                    let source = ty;
                    let ty = match map(ty) {
                        ControlFlow::Break(ty) => {
                            results.push(ty);
                            continue;
                        }
                        ControlFlow::Continue(ty) => ty,
                    };
                    let child_count = structural_type_child_count(db, ty);
                    if child_count == 0 {
                        results.push(finish(ty));
                        continue;
                    }
                    let queued_entries = stack
                        .iter()
                        .filter(|item| matches!(item, StructuralTypeMapItem::Enter(_)))
                        .count();
                    let available_steps = remaining_steps.saturating_sub(queued_entries);
                    if child_count
                        > available_steps
                            .saturating_add(active.len())
                            .saturating_add(1)
                    {
                        return Err(StructuralMapError::StepLimitExceeded);
                    }
                    let children = structural_type_children(db, ty);
                    let new_children = children
                        .iter()
                        .filter(|child| **child != source && !active.contains(*child))
                        .count();
                    if new_children > available_steps {
                        return Err(StructuralMapError::StepLimitExceeded);
                    }

                    active.insert(source);
                    stack.push(StructuralTypeMapItem::Exit(source));
                    stack.push(StructuralTypeMapItem::Rebuild(ty, child_count));
                    stack.extend(children.into_iter().rev().map(StructuralTypeMapItem::Enter));
                }
                StructuralTypeMapItem::Rebuild(ty, child_count) => {
                    let start = results
                        .len()
                        .checked_sub(child_count)
                        .ok_or(StructuralMapError::InvalidRebuild)?;
                    let children = results.split_off(start);
                    let rebuilt = rebuild_structural_type(db, ty, children)
                        .ok_or(StructuralMapError::InvalidRebuild)?;
                    results.push(finish(rebuilt));
                }
                StructuralTypeMapItem::Exit(ty) => {
                    active.remove(&ty);
                }
            }
        }

        match results.as_slice() {
            [result] => Ok(*result),
            _ => Err(StructuralMapError::InvalidRebuild),
        }
    }

    /// Builds the smallest type that represents a list of union variants.
    ///
    /// Duplicate variants are removed. An empty list becomes `never`, and a
    /// single remaining variant is returned directly instead of wrapping it in
    /// a union.
    pub fn union_from_types(db: &'db dyn TypeDb, types: Vec<Self>) -> Self {
        UnionBuilder::new(db).add_all(types).build()
    }

    /// Builds the smallest type that represents a list of intersection variants.
    ///
    /// Nested intersections are flattened, duplicate variants are removed, an
    /// empty list becomes `never`, and a single remaining variant is returned
    /// directly instead of wrapping it in an intersection.
    pub fn intersection_from_types(db: &'db dyn TypeDb, types: Vec<Self>) -> Self {
        IntersectionBuilder::new(db).add_all(types).build()
    }

    pub fn object_from_members(db: &'db dyn TypeDb, members: Vec<TypeMember<'db>>) -> Self {
        crate::builders::object_from_members(db, members)
    }

    pub fn pick_members(
        db: &'db dyn TypeDb,
        members: Vec<TypeMember<'db>>,
        key_names: &[Text],
    ) -> Self {
        crate::builders::pick_members(db, members, key_names)
    }

    pub fn omit_members(
        db: &'db dyn TypeDb,
        members: Vec<TypeMember<'db>>,
        key_names: &[Text],
    ) -> Self {
        crate::builders::omit_members(db, members, key_names)
    }

    pub fn with_all_optional_members(db: &'db dyn TypeDb, members: Vec<TypeMember<'db>>) -> Self {
        crate::builders::with_all_optional_members(db, members)
    }

    pub fn with_all_required_members(db: &'db dyn TypeDb, members: Vec<TypeMember<'db>>) -> Self {
        crate::builders::with_all_required_members(db, members)
    }

    pub fn array_class(_db: &'db dyn TypeDb) -> Self {
        Self::GlobalType(ARRAY_ID_GLOBAL_TYPE_ID)
    }

    pub fn async_disposable_interface(_db: &'db dyn TypeDb) -> Self {
        Self::GlobalType(ASYNC_DISPOSABLE_ID_GLOBAL_TYPE_ID)
    }

    pub fn date_class(_db: &'db dyn TypeDb) -> Self {
        Self::GlobalType(DATE_ID_GLOBAL_TYPE_ID)
    }

    pub fn disposable_interface(_db: &'db dyn TypeDb) -> Self {
        Self::GlobalType(DISPOSABLE_ID_GLOBAL_TYPE_ID)
    }

    pub fn error_class(_db: &'db dyn TypeDb) -> Self {
        Self::GlobalType(ERROR_ID_GLOBAL_TYPE_ID)
    }

    pub fn map_class(_db: &'db dyn TypeDb) -> Self {
        Self::GlobalType(MAP_ID_GLOBAL_TYPE_ID)
    }

    pub fn promise_class(_db: &'db dyn TypeDb) -> Self {
        Self::GlobalType(PROMISE_ID_GLOBAL_TYPE_ID)
    }

    pub fn regexp_class(_db: &'db dyn TypeDb) -> Self {
        Self::GlobalType(REGEXP_ID_GLOBAL_TYPE_ID)
    }

    pub fn set_class(_db: &'db dyn TypeDb) -> Self {
        Self::GlobalType(SET_ID_GLOBAL_TYPE_ID)
    }

    pub fn symbol_class(_db: &'db dyn TypeDb) -> Self {
        Self::GlobalType(SYMBOL_ID_GLOBAL_TYPE_ID)
    }

    pub fn weak_map_class(_db: &'db dyn TypeDb) -> Self {
        Self::GlobalType(WEAK_MAP_ID_GLOBAL_TYPE_ID)
    }

    pub fn instance_of(db: &'db dyn TypeDb, ty: Self, type_parameters: Box<[Self]>) -> Self {
        if type_parameters.is_empty()
            && let Self::InstanceOf(instance) = ty
        {
            return Self::InstanceOf(instance);
        }

        Self::InstanceOf(InternedTypeInstance::new(db, ty, type_parameters))
    }

    pub fn array_instance(db: &'db dyn TypeDb, type_parameters: Box<[Self]>) -> Self {
        Self::instance_of(db, Self::array_class(db), type_parameters)
    }

    pub fn map_instance(db: &'db dyn TypeDb, type_parameters: Box<[Self]>) -> Self {
        Self::instance_of(db, Self::map_class(db), type_parameters)
    }

    pub fn promise_instance(db: &'db dyn TypeDb, type_parameters: Box<[Self]>) -> Self {
        Self::instance_of(db, Self::promise_class(db), type_parameters)
    }

    pub fn set_instance(db: &'db dyn TypeDb, type_parameters: Box<[Self]>) -> Self {
        Self::instance_of(db, Self::set_class(db), type_parameters)
    }

    pub fn weak_map_instance(db: &'db dyn TypeDb, type_parameters: Box<[Self]>) -> Self {
        Self::instance_of(db, Self::weak_map_class(db), type_parameters)
    }

    pub fn from_raw_lossy(db: &'db dyn TypeDb, raw: &RawTypeData) -> Self {
        let mut resolve_reference =
            |reference: &raw::TypeReference| Self::from_raw_reference_lossy(db, reference);
        Self::from_raw_with_resolver(db, raw, false, &mut resolve_reference)
    }

    pub fn from_raw_with_resolver(
        db: &'db dyn TypeDb,
        raw: &RawTypeData,
        is_builtin: bool,
        resolve_reference: &mut ReferenceResolver<'db, '_>,
    ) -> Self {
        match raw {
            raw::TypeData::Unknown => Self::Unknown,
            raw::TypeData::Global => Self::Global,
            raw::TypeData::BigInt => Self::BigInt,
            raw::TypeData::Boolean => Self::Boolean,
            raw::TypeData::Null => Self::Null,
            raw::TypeData::Number => Self::Number,
            raw::TypeData::String => Self::String,
            raw::TypeData::Symbol => Self::Symbol,
            raw::TypeData::Undefined => Self::Undefined,
            raw::TypeData::Conditional => Self::Conditional,
            raw::TypeData::ImportNamespace(_) => Self::Unknown,
            raw::TypeData::Class(class) => Self::Class(InternedClass::new(
                db,
                convert_references(db, &class.type_parameters, resolve_reference),
                class.extends.as_ref().map(&mut *resolve_reference),
                convert_references(db, &class.implements, resolve_reference),
                convert_type_members(db, &class.members, resolve_reference),
                class.name.clone(),
                is_builtin,
            )),
            raw::TypeData::Constructor(constructor) => Self::Constructor(InternedConstructor::new(
                db,
                convert_references(db, &constructor.type_parameters, resolve_reference),
                convert_constructor_parameters(db, &constructor.parameters, resolve_reference),
                constructor
                    .return_type
                    .as_ref()
                    .map(&mut *resolve_reference),
            )),
            raw::TypeData::Function(function) => Self::Function(InternedFunction::new(
                db,
                convert_references(db, &function.type_parameters, resolve_reference),
                convert_function_parameters(db, &function.parameters, resolve_reference),
                convert_return_type(db, &function.return_type, resolve_reference),
                function.is_async,
                function.name.clone(),
            )),
            raw::TypeData::Interface(interface) => Self::Interface(InternedInterface::new(
                db,
                convert_references(db, &interface.type_parameters, resolve_reference),
                convert_references(db, &interface.extends, resolve_reference),
                convert_type_members(db, &interface.members, resolve_reference),
                interface.name.clone(),
            )),
            raw::TypeData::Module(module) => Self::Module(InternedModule::new(
                db,
                convert_type_members(db, &module.members, resolve_reference),
                module.name.clone(),
            )),
            raw::TypeData::Namespace(namespace) => Self::Namespace(InternedNamespace::new(
                db,
                convert_type_members(db, &namespace.members, resolve_reference),
                namespace.path.clone(),
            )),
            raw::TypeData::Object(object) => Self::Object(InternedObject::new(
                db,
                object.prototype.as_ref().map(&mut *resolve_reference),
                convert_type_members(db, &object.members, resolve_reference),
            )),
            raw::TypeData::Tuple(tuple) => Self::Tuple(InternedTuple::new(
                db,
                tuple
                    .elements()
                    .iter()
                    .map(|element| TupleElementType {
                        ty: resolve_reference(&element.ty),
                        name: element.name.clone(),
                        is_optional: element.is_optional,
                        is_rest: element.is_rest,
                    })
                    .collect::<Box<[_]>>(),
            )),
            raw::TypeData::Generic(generic) => Self::Generic(InternedGenericTypeParameter::new(
                db,
                generic
                    .constraint
                    .is_known()
                    .then(|| resolve_reference(&generic.constraint)),
                generic
                    .default
                    .is_known()
                    .then(|| resolve_reference(&generic.default)),
                generic.name.clone(),
            )),
            raw::TypeData::Intersection(intersection) => Self::intersection_from_types(
                db,
                convert_references(db, intersection.types(), resolve_reference).into_vec(),
            ),
            raw::TypeData::Union(union) => Self::union_from_types(
                db,
                convert_references(db, union.types(), resolve_reference).into_vec(),
            ),
            raw::TypeData::TypeOperator(type_operator) => {
                Self::TypeOperator(InternedTypeOperatorType::new(
                    db,
                    resolve_reference(&type_operator.ty),
                    type_operator.operator,
                ))
            }
            raw::TypeData::Literal(literal) => Self::Literal(InternedLiteral::new(
                db,
                convert_literal(db, literal.as_ref(), resolve_reference),
            )),
            raw::TypeData::InstanceOf(instance) => Self::instance_of(
                db,
                resolve_reference(&instance.ty),
                convert_references(db, &instance.type_parameters, resolve_reference),
            ),
            raw::TypeData::Reference(reference) => resolve_reference(reference),
            raw::TypeData::MergedReference(reference) => {
                Self::MergedReference(InternedMergedReference::new(
                    db,
                    reference.ty.as_ref().map(&mut *resolve_reference),
                    reference.value_ty.as_ref().map(&mut *resolve_reference),
                    reference.namespace_ty.as_ref().map(&mut *resolve_reference),
                ))
            }
            raw::TypeData::TypeofExpression(expression) => {
                Self::TypeofExpression(InternedTypeofExpression::new(
                    db,
                    convert_typeof_expression(db, expression.as_ref(), resolve_reference),
                ))
            }
            raw::TypeData::TypeofType(ty) => {
                Self::TypeofType(InternedTypeofType::new(db, resolve_reference(ty)))
            }
            raw::TypeData::TypeofValue(value) => Self::TypeofValue(InternedTypeofValue::new(
                db,
                resolve_reference(&value.ty),
                value.identifier.clone(),
                value.scope_id,
            )),
            raw::TypeData::AnyKeyword => Self::AnyKeyword,
            raw::TypeData::NeverKeyword => Self::NeverKeyword,
            raw::TypeData::ObjectKeyword => Self::ObjectKeyword,
            raw::TypeData::ThisKeyword => Self::ThisKeyword,
            raw::TypeData::UnknownKeyword => Self::UnknownKeyword,
            raw::TypeData::VoidKeyword => Self::VoidKeyword,
        }
    }

    pub fn from_raw_reference_lossy(db: &'db dyn TypeDb, reference: &raw::TypeReference) -> Self {
        match reference {
            raw::TypeReference::Resolved(id) if *id == GLOBAL_UNKNOWN_ID => Self::Unknown,
            raw::TypeReference::Resolved(id) if *id == GLOBAL_UNDEFINED_ID => Self::Undefined,
            raw::TypeReference::Resolved(id) if *id == GLOBAL_VOID_ID => Self::VoidKeyword,
            raw::TypeReference::Resolved(id) if *id == GLOBAL_CONDITIONAL_ID => Self::Conditional,
            raw::TypeReference::Resolved(id) if *id == GLOBAL_NUMBER_ID => Self::Number,
            raw::TypeReference::Resolved(id) if *id == GLOBAL_STRING_ID => Self::String,
            raw::TypeReference::Resolved(id) if *id == GLOBAL_GLOBAL_ID => Self::Global,
            raw::TypeReference::Resolved(id) if *id == GLOBAL_BOOLEAN_ID => Self::Boolean,
            raw::TypeReference::Resolved(id) if *id == GLOBAL_ARRAY_ID => Self::array_class(db),
            raw::TypeReference::Resolved(id) if *id == GLOBAL_ASYNC_DISPOSABLE_ID => {
                Self::async_disposable_interface(db)
            }
            raw::TypeReference::Resolved(id) if *id == GLOBAL_DATE_ID => Self::date_class(db),
            raw::TypeReference::Resolved(id) if *id == GLOBAL_DISPOSABLE_ID => {
                Self::disposable_interface(db)
            }
            raw::TypeReference::Resolved(id) if *id == GLOBAL_ERROR_ID => Self::error_class(db),
            raw::TypeReference::Resolved(id) if *id == GLOBAL_MAP_ID => Self::map_class(db),
            raw::TypeReference::Resolved(id) if *id == GLOBAL_PROMISE_ID => Self::promise_class(db),
            raw::TypeReference::Resolved(id) if *id == GLOBAL_REGEXP_ID => Self::regexp_class(db),
            raw::TypeReference::Resolved(id) if *id == GLOBAL_SET_ID => Self::set_class(db),
            raw::TypeReference::Resolved(id) if *id == GLOBAL_SYMBOL_ID => Self::symbol_class(db),
            raw::TypeReference::Resolved(id) if *id == GLOBAL_WEAK_MAP_ID => {
                Self::weak_map_class(db)
            }
            raw::TypeReference::Resolved(_) => Self::Unknown,
            raw::TypeReference::Qualifier(qualifier) => qualifier
                .type_parameters
                .iter()
                .find(|param| param.is_known())
                .map_or(Self::Unknown, |param| {
                    Self::from_raw_reference_lossy(db, param)
                }),
            raw::TypeReference::Import(_) => Self::Unknown,
        }
    }

    pub fn to_raw_lossy(self, db: &'db dyn TypeDb) -> RawTypeData {
        match self {
            Self::Unknown | Self::Divergent(_) => raw::TypeData::Unknown,
            Self::GlobalType(id) => raw::TypeData::Reference(raw::RawTypeId::Global(id).into()),
            Self::Global => raw::TypeData::Global,
            Self::BigInt => raw::TypeData::BigInt,
            Self::Boolean => raw::TypeData::Boolean,
            Self::Null => raw::TypeData::Null,
            Self::Number => raw::TypeData::Number,
            Self::String => raw::TypeData::String,
            Self::Symbol => raw::TypeData::Symbol,
            Self::Undefined => raw::TypeData::Undefined,
            Self::Conditional => raw::TypeData::Conditional,
            Self::Class(class) => raw::TypeData::Class(Box::new(raw::Class {
                name: class.name(db).clone(),
                type_parameters: raw_references_from_types(db, class.type_parameters(db)),
                extends: self.raw_reference_from_option(db, class.extends(db)),
                implements: raw_references_from_types(db, class.implements(db)),
                members: raw_type_members_from_types(db, class.members(db)),
            })),
            Self::Constructor(constructor) => {
                raw::TypeData::Constructor(Box::new(raw::Constructor {
                    type_parameters: raw_references_from_types(db, constructor.type_parameters(db)),
                    parameters: raw_constructor_parameters_from_types(
                        db,
                        constructor.parameters(db),
                    ),
                    return_type: self.raw_reference_from_option(db, constructor.return_type(db)),
                }))
            }
            Self::Function(function) => raw::TypeData::Function(Box::new(raw::Function {
                is_async: function.is_async(db),
                type_parameters: raw_references_from_types(db, function.type_parameters(db)),
                name: function.name(db).clone(),
                parameters: raw_function_parameters_from_types(db, function.parameters(db)),
                return_type: raw_return_type_from_type(db, function.return_type(db)),
            })),
            Self::Interface(interface) => raw::TypeData::Interface(Box::new(raw::Interface {
                name: interface.name(db).clone(),
                type_parameters: raw_references_from_types(db, interface.type_parameters(db)),
                extends: raw_references_from_types(db, interface.extends(db)),
                members: raw_type_members_from_types(db, interface.members(db)),
            })),
            Self::Module(module) => raw::TypeData::Module(Box::new(raw::Module {
                name: module.name(db).clone(),
                members: raw_type_members_from_types(db, module.members(db)),
            })),
            Self::Namespace(namespace) => raw::TypeData::Namespace(Box::new(raw::Namespace {
                path: namespace.path(db).clone(),
                members: raw_type_members_from_types(db, namespace.members(db)),
            })),
            Self::Object(object) => raw::TypeData::Object(Box::new(raw::Object {
                prototype: self.raw_reference_from_option(db, object.prototype(db)),
                members: raw_type_members_from_types(db, object.members(db)),
            })),
            Self::Tuple(tuple) => raw::TypeData::Tuple(Box::new(raw::Tuple(
                tuple
                    .elements(db)
                    .iter()
                    .map(|element| raw::TupleElementType {
                        ty: element.ty.to_raw_reference_lossy(),
                        name: element.name.clone(),
                        is_optional: element.is_optional,
                        is_rest: element.is_rest,
                    })
                    .collect(),
            ))),
            Self::Generic(generic) => raw::TypeData::Generic(Box::new(raw::GenericTypeParameter {
                name: generic.name(db).clone(),
                constraint: generic.constraint(db).map_or_else(
                    raw::TypeReference::unknown,
                    TypeData::to_raw_reference_lossy,
                ),
                default: generic.default(db).map_or_else(
                    raw::TypeReference::unknown,
                    TypeData::to_raw_reference_lossy,
                ),
            })),
            Self::Local(_) => raw::TypeData::Unknown,
            Self::Intersection(intersection) => raw::TypeData::Intersection(Box::new(
                raw::Intersection(raw_references_from_types(db, intersection.types(db))),
            )),
            Self::Union(union) => raw::TypeData::Union(Box::new(raw::Union(
                raw_references_from_types(db, union.types(db)),
            ))),
            Self::TypeOperator(type_operator) => {
                raw::TypeData::TypeOperator(Box::new(raw::TypeOperatorType {
                    operator: type_operator.operator(db),
                    ty: type_operator.ty(db).to_raw_reference_lossy(),
                }))
            }
            Self::Literal(literal) => {
                raw::TypeData::Literal(Box::new(raw_literal_from_type(db, literal.literal(db))))
            }
            Self::InstanceOf(instance) => raw::TypeData::InstanceOf(Box::new(raw::TypeInstance {
                ty: instance.ty(db).to_raw_reference_lossy(),
                type_parameters: raw_references_from_types(db, instance.type_parameters(db)),
            })),
            Self::MergedReference(reference) => {
                raw::TypeData::MergedReference(Box::new(raw::MergedReference {
                    ty: self.raw_reference_from_option(db, reference.ty(db)),
                    value_ty: self.raw_reference_from_option(db, reference.value_ty(db)),
                    namespace_ty: self.raw_reference_from_option(db, reference.namespace_ty(db)),
                }))
            }
            Self::TypeofExpression(expression) => raw::TypeData::TypeofExpression(Box::new(
                raw_typeof_expression_from_type(db, expression.expression(db)),
            )),
            Self::TypeofType(ty) => {
                raw::TypeData::TypeofType(Box::new(ty.ty(db).to_raw_reference_lossy()))
            }
            Self::TypeofValue(value) => raw::TypeData::TypeofValue(Box::new(raw::TypeofValue {
                identifier: value.identifier(db).clone(),
                ty: value.ty(db).to_raw_reference_lossy(),
                scope_id: value.scope_id(db),
            })),
            Self::AnyKeyword => raw::TypeData::AnyKeyword,
            Self::NeverKeyword => raw::TypeData::NeverKeyword,
            Self::ObjectKeyword => raw::TypeData::ObjectKeyword,
            Self::ThisKeyword => raw::TypeData::ThisKeyword,
            Self::UnknownKeyword => raw::TypeData::UnknownKeyword,
            Self::VoidKeyword => raw::TypeData::VoidKeyword,
        }
    }

    pub fn to_raw_reference_lossy(self) -> raw::TypeReference {
        match self {
            Self::Unknown | Self::Divergent(_) => raw::TypeReference::Resolved(GLOBAL_UNKNOWN_ID),
            Self::Global => raw::TypeReference::Resolved(GLOBAL_GLOBAL_ID),
            Self::Boolean => raw::TypeReference::Resolved(GLOBAL_BOOLEAN_ID),
            Self::Number => raw::TypeReference::Resolved(GLOBAL_NUMBER_ID),
            Self::String => raw::TypeReference::Resolved(GLOBAL_STRING_ID),
            Self::Undefined => raw::TypeReference::Resolved(GLOBAL_UNDEFINED_ID),
            Self::Conditional => raw::TypeReference::Resolved(GLOBAL_CONDITIONAL_ID),
            Self::VoidKeyword => raw::TypeReference::Resolved(GLOBAL_VOID_ID),
            Self::Local(_) => raw::TypeReference::Resolved(GLOBAL_UNKNOWN_ID),
            _ => raw::TypeReference::Resolved(GLOBAL_UNKNOWN_ID),
        }
    }

    fn raw_reference_from_option(
        self,
        _db: &'db dyn TypeDb,
        ty: Option<Self>,
    ) -> Option<raw::TypeReference> {
        ty.map(Self::to_raw_reference_lossy)
    }
}

impl<'db> TypeSubstitution<'db> {
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

fn declared_type_parameters<'db>(
    db: &'db dyn TypeDb,
    ty: TypeData<'db>,
) -> Option<&'db [TypeData<'db>]> {
    if let TypeData::Class(class) = ty {
        Some(class.type_parameters(db))
    } else if let TypeData::Constructor(constructor) = ty {
        Some(constructor.type_parameters(db))
    } else if let TypeData::Function(function) = ty {
        Some(function.type_parameters(db))
    } else if let TypeData::Interface(interface) = ty {
        Some(interface.type_parameters(db))
    } else {
        None
    }
}

fn function_parameter_child_count(parameter: &FunctionParameter<'_>) -> usize {
    match parameter {
        FunctionParameter::Named(_) => 1,
        FunctionParameter::Pattern(parameter) => parameter.bindings.len().saturating_add(1),
    }
}

fn type_member_child_count(member: &TypeMember<'_>) -> usize {
    let key_count = usize::from(matches!(
        &member.kind,
        TypeMemberKind::ComputedValue(_)
            | TypeMemberKind::ComputedValueNamed(_, _)
            | TypeMemberKind::ConstAssertedComputedValue(_)
            | TypeMemberKind::ConstAssertedComputedValueNamed(_, _)
            | TypeMemberKind::ConstAssertedIndexSignature(_)
            | TypeMemberKind::IndexSignature(_)
    ));
    key_count + 1
}

fn typeof_expression_child_count(expression: &TypeofExpression<'_>) -> usize {
    match expression {
        TypeofExpression::Addition(_)
        | TypeofExpression::LogicalAnd(_)
        | TypeofExpression::LogicalOr(_)
        | TypeofExpression::NullishCoalescing(_) => 2,
        TypeofExpression::Conditional(_) => 3,
        TypeofExpression::Call(expression) => expression.arguments.len().saturating_add(1),
        TypeofExpression::New(expression) => expression.arguments.len().saturating_add(1),
        TypeofExpression::Await(_)
        | TypeofExpression::BitwiseNot(_)
        | TypeofExpression::Destructure(_)
        | TypeofExpression::Index(_)
        | TypeofExpression::IterableValueOf(_)
        | TypeofExpression::StaticMember(_)
        | TypeofExpression::Super(_)
        | TypeofExpression::This(_)
        | TypeofExpression::Typeof(_)
        | TypeofExpression::UnaryMinus(_) => 1,
    }
}

#[deny(clippy::wildcard_enum_match_arm)]
fn structural_type_child_count<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> usize {
    match ty {
        TypeData::Class(class) => class
            .type_parameters(db)
            .len()
            .saturating_add(usize::from(class.extends(db).is_some()))
            .saturating_add(class.implements(db).len())
            .saturating_add(class.members(db).iter().map(type_member_child_count).sum()),
        TypeData::Constructor(constructor) => constructor
            .type_parameters(db)
            .len()
            .saturating_add(
                constructor
                    .parameters(db)
                    .iter()
                    .map(|parameter| function_parameter_child_count(&parameter.parameter))
                    .sum(),
            )
            .saturating_add(usize::from(constructor.return_type(db).is_some())),
        TypeData::Function(function) => function
            .type_parameters(db)
            .len()
            .saturating_add(
                function
                    .parameters(db)
                    .iter()
                    .map(function_parameter_child_count)
                    .sum(),
            )
            .saturating_add(1),
        TypeData::Interface(interface) => interface
            .type_parameters(db)
            .len()
            .saturating_add(interface.extends(db).len())
            .saturating_add(
                interface
                    .members(db)
                    .iter()
                    .map(type_member_child_count)
                    .sum(),
            ),
        TypeData::Module(module) => module.members(db).iter().map(type_member_child_count).sum(),
        TypeData::Namespace(namespace) => namespace
            .members(db)
            .iter()
            .map(type_member_child_count)
            .sum(),
        TypeData::Object(object) => usize::from(object.prototype(db).is_some())
            .saturating_add(object.members(db).iter().map(type_member_child_count).sum()),
        TypeData::Tuple(tuple) => tuple.elements(db).len(),
        TypeData::Generic(generic) => {
            usize::from(generic.constraint(db).is_some())
                + usize::from(generic.default(db).is_some())
        }
        TypeData::Intersection(intersection) => intersection.types(db).len(),
        TypeData::Union(union) => union.types(db).len(),
        TypeData::TypeOperator(_) => 1,
        TypeData::Literal(literal) => match literal.literal(db) {
            Literal::Object(members) => members.iter().map(type_member_child_count).sum(),
            Literal::BigInt(_)
            | Literal::Boolean(_)
            | Literal::Number(_)
            | Literal::RegExp(_)
            | Literal::String(_)
            | Literal::Template(_) => 0,
        },
        TypeData::InstanceOf(instance) => instance.type_parameters(db).len().saturating_add(1),
        TypeData::MergedReference(reference) => {
            usize::from(reference.ty(db).is_some())
                + usize::from(reference.value_ty(db).is_some())
                + usize::from(reference.namespace_ty(db).is_some())
        }
        TypeData::TypeofExpression(expression) => {
            typeof_expression_child_count(expression.expression(db))
        }
        TypeData::TypeofType(_) | TypeData::TypeofValue(_) => 1,
        TypeData::Unknown
        | TypeData::Divergent(_)
        | TypeData::Global
        | TypeData::GlobalType(_)
        | TypeData::BigInt
        | TypeData::Boolean
        | TypeData::Null
        | TypeData::Number
        | TypeData::String
        | TypeData::Symbol
        | TypeData::Undefined
        | TypeData::Conditional
        | TypeData::Local(_)
        | TypeData::AnyKeyword
        | TypeData::NeverKeyword
        | TypeData::ObjectKeyword
        | TypeData::ThisKeyword
        | TypeData::UnknownKeyword
        | TypeData::VoidKeyword => 0,
    }
}

#[deny(clippy::wildcard_enum_match_arm)]
fn structural_type_children<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> Vec<TypeData<'db>> {
    let mut children = Vec::new();
    match ty {
        TypeData::Class(class) => {
            children.extend_from_slice(class.type_parameters(db));
            children.extend(class.extends(db));
            children.extend_from_slice(class.implements(db));
            push_type_member_children(class.members(db), &mut children);
        }
        TypeData::Constructor(constructor) => {
            children.extend_from_slice(constructor.type_parameters(db));
            for parameter in constructor.parameters(db) {
                push_function_parameter_children(&parameter.parameter, &mut children);
            }
            children.extend(constructor.return_type(db));
        }
        TypeData::Function(function) => {
            children.extend_from_slice(function.type_parameters(db));
            for parameter in function.parameters(db) {
                push_function_parameter_children(parameter, &mut children);
            }
            push_return_type_children(function.return_type(db), &mut children);
        }
        TypeData::Interface(interface) => {
            children.extend_from_slice(interface.type_parameters(db));
            children.extend_from_slice(interface.extends(db));
            push_type_member_children(interface.members(db), &mut children);
        }
        TypeData::Module(module) => push_type_member_children(module.members(db), &mut children),
        TypeData::Namespace(namespace) => {
            push_type_member_children(namespace.members(db), &mut children);
        }
        TypeData::Object(object) => {
            children.extend(object.prototype(db));
            push_type_member_children(object.members(db), &mut children);
        }
        TypeData::Tuple(tuple) => {
            children.extend(tuple.elements(db).iter().map(|element| element.ty));
        }
        TypeData::Generic(generic) => {
            children.extend(generic.constraint(db));
            children.extend(generic.default(db));
        }
        TypeData::Intersection(intersection) => children.extend_from_slice(intersection.types(db)),
        TypeData::Union(union) => children.extend_from_slice(union.types(db)),
        TypeData::TypeOperator(operator) => children.push(operator.ty(db)),
        TypeData::Literal(literal) => {
            if let Literal::Object(members) = literal.literal(db) {
                push_type_member_children(members, &mut children);
            }
        }
        TypeData::InstanceOf(instance) => {
            children.push(instance.ty(db));
            children.extend_from_slice(instance.type_parameters(db));
        }
        TypeData::MergedReference(reference) => {
            children.extend(reference.ty(db));
            children.extend(reference.value_ty(db));
            children.extend(reference.namespace_ty(db));
        }
        TypeData::TypeofExpression(expression) => {
            push_typeof_expression_children(expression.expression(db), &mut children);
        }
        TypeData::TypeofType(ty) => children.push(ty.ty(db)),
        TypeData::TypeofValue(value) => children.push(value.ty(db)),
        TypeData::Unknown
        | TypeData::Divergent(_)
        | TypeData::Global
        | TypeData::GlobalType(_)
        | TypeData::BigInt
        | TypeData::Boolean
        | TypeData::Null
        | TypeData::Number
        | TypeData::String
        | TypeData::Symbol
        | TypeData::Undefined
        | TypeData::Conditional
        | TypeData::Local(_)
        | TypeData::AnyKeyword
        | TypeData::NeverKeyword
        | TypeData::ObjectKeyword
        | TypeData::ThisKeyword
        | TypeData::UnknownKeyword
        | TypeData::VoidKeyword => {}
    }
    children
}

fn push_function_parameter_children<'db>(
    parameter: &FunctionParameter<'db>,
    children: &mut Vec<TypeData<'db>>,
) {
    match parameter {
        FunctionParameter::Named(parameter) => children.push(parameter.ty),
        FunctionParameter::Pattern(parameter) => {
            children.extend(parameter.bindings.iter().map(|binding| binding.ty));
            children.push(parameter.ty);
        }
    }
}

fn push_return_type_children<'db>(
    return_type: &ReturnType<'db>,
    children: &mut Vec<TypeData<'db>>,
) {
    children.push(match return_type {
        ReturnType::Type(ty) => *ty,
        ReturnType::Predicate(predicate) => predicate.ty,
        ReturnType::Asserts(asserts) => asserts.ty,
    });
}

fn push_type_member_children<'db>(members: &[TypeMember<'db>], children: &mut Vec<TypeData<'db>>) {
    for member in members {
        match &member.kind {
            TypeMemberKind::ComputedValue(ty)
            | TypeMemberKind::ComputedValueNamed(_, ty)
            | TypeMemberKind::ConstAssertedComputedValue(ty)
            | TypeMemberKind::ConstAssertedComputedValueNamed(_, ty)
            | TypeMemberKind::ConstAssertedIndexSignature(ty)
            | TypeMemberKind::IndexSignature(ty) => children.push(*ty),
            TypeMemberKind::CallSignature
            | TypeMemberKind::ConstAssertedCallSignature
            | TypeMemberKind::ConstAssertedConstructor
            | TypeMemberKind::ConstAssertedGetter(_)
            | TypeMemberKind::ConstAssertedNamed(_)
            | TypeMemberKind::ConstAssertedNamedOptional(_)
            | TypeMemberKind::ConstAssertedNamedStatic(_)
            | TypeMemberKind::Constructor
            | TypeMemberKind::Getter(_)
            | TypeMemberKind::Named(_)
            | TypeMemberKind::NamedOptional(_)
            | TypeMemberKind::NamedStatic(_) => {}
        }
        children.push(member.ty);
    }
}

#[deny(clippy::wildcard_enum_match_arm)]
fn push_typeof_expression_children<'db>(
    expression: &TypeofExpression<'db>,
    children: &mut Vec<TypeData<'db>>,
) {
    match expression {
        TypeofExpression::Addition(expression) => {
            children.extend([expression.left, expression.right]);
        }
        TypeofExpression::Await(expression) => children.push(expression.argument),
        TypeofExpression::BitwiseNot(expression) => children.push(expression.argument),
        TypeofExpression::Call(expression) => {
            children.push(expression.callee);
            push_call_argument_children(&expression.arguments, children);
        }
        TypeofExpression::Conditional(expression) => {
            children.extend([expression.test, expression.consequent, expression.alternate]);
        }
        TypeofExpression::Destructure(expression) => children.push(expression.ty),
        TypeofExpression::Index(expression) => children.push(expression.object),
        TypeofExpression::IterableValueOf(expression) => children.push(expression.ty),
        TypeofExpression::LogicalAnd(expression) => {
            children.extend([expression.left, expression.right]);
        }
        TypeofExpression::LogicalOr(expression) => {
            children.extend([expression.left, expression.right]);
        }
        TypeofExpression::New(expression) => {
            children.push(expression.callee);
            push_call_argument_children(&expression.arguments, children);
        }
        TypeofExpression::NullishCoalescing(expression) => {
            children.extend([expression.left, expression.right]);
        }
        TypeofExpression::StaticMember(expression) => children.push(expression.object),
        TypeofExpression::Super(expression) | TypeofExpression::This(expression) => {
            children.push(expression.parent);
        }
        TypeofExpression::Typeof(expression) => children.push(expression.argument),
        TypeofExpression::UnaryMinus(expression) => children.push(expression.argument),
    }
}

fn push_call_argument_children<'db>(
    arguments: &[CallArgumentType<'db>],
    children: &mut Vec<TypeData<'db>>,
) {
    children.extend(arguments.iter().map(|argument| match argument {
        CallArgumentType::Argument(ty) | CallArgumentType::Spread(ty) => *ty,
    }));
}

#[deny(clippy::wildcard_enum_match_arm)]
fn rebuild_structural_type<'db>(
    db: &'db dyn TypeDb,
    ty: TypeData<'db>,
    children: Vec<TypeData<'db>>,
) -> Option<TypeData<'db>> {
    let mut children = children.into_iter();
    let rebuilt = match ty {
        TypeData::Class(class) => TypeData::Class(InternedClass::new(
            db,
            take_types(&mut children, class.type_parameters(db).len())?,
            take_optional_type(&mut children, class.extends(db))?,
            take_types(&mut children, class.implements(db).len())?,
            rebuild_type_members(class.members(db), &mut children)?,
            class.name(db).clone(),
            class.is_builtin(db),
        )),
        TypeData::Constructor(constructor) => TypeData::Constructor(InternedConstructor::new(
            db,
            take_types(&mut children, constructor.type_parameters(db).len())?,
            constructor
                .parameters(db)
                .iter()
                .map(|parameter| {
                    Some(ConstructorParameter {
                        parameter: rebuild_function_parameter(&parameter.parameter, &mut children)?,
                        accessibility: parameter.accessibility,
                    })
                })
                .collect::<Option<Box<[_]>>>()?,
            take_optional_type(&mut children, constructor.return_type(db))?,
        )),
        TypeData::Function(function) => TypeData::Function(InternedFunction::new(
            db,
            take_types(&mut children, function.type_parameters(db).len())?,
            function
                .parameters(db)
                .iter()
                .map(|parameter| rebuild_function_parameter(parameter, &mut children))
                .collect::<Option<Box<[_]>>>()?,
            rebuild_return_type(function.return_type(db), &mut children)?,
            function.is_async(db),
            function.name(db).clone(),
        )),
        TypeData::Interface(interface) => TypeData::Interface(InternedInterface::new(
            db,
            take_types(&mut children, interface.type_parameters(db).len())?,
            take_types(&mut children, interface.extends(db).len())?,
            rebuild_type_members(interface.members(db), &mut children)?,
            interface.name(db).clone(),
        )),
        TypeData::Module(module) => TypeData::Module(InternedModule::new(
            db,
            rebuild_type_members(module.members(db), &mut children)?,
            module.name(db).clone(),
        )),
        TypeData::Namespace(namespace) => TypeData::Namespace(InternedNamespace::new(
            db,
            rebuild_type_members(namespace.members(db), &mut children)?,
            namespace.path(db).clone(),
        )),
        TypeData::Object(object) => TypeData::Object(InternedObject::new(
            db,
            take_optional_type(&mut children, object.prototype(db))?,
            rebuild_type_members(object.members(db), &mut children)?,
        )),
        TypeData::Tuple(tuple) => TypeData::Tuple(InternedTuple::new(
            db,
            tuple
                .elements(db)
                .iter()
                .map(|element| {
                    let mut element = element.clone();
                    element.ty = children.next()?;
                    Some(element)
                })
                .collect::<Option<Box<[_]>>>()?,
        )),
        TypeData::Generic(generic) => TypeData::Generic(InternedGenericTypeParameter::new(
            db,
            take_optional_type(&mut children, generic.constraint(db))?,
            take_optional_type(&mut children, generic.default(db))?,
            generic.name(db).clone(),
        )),
        TypeData::Intersection(intersection) => TypeData::intersection_from_types(
            db,
            take_types(&mut children, intersection.types(db).len())?.into_vec(),
        ),
        TypeData::Union(union) => TypeData::union_from_types(
            db,
            take_types(&mut children, union.types(db).len())?.into_vec(),
        ),
        TypeData::TypeOperator(operator) => TypeData::TypeOperator(InternedTypeOperatorType::new(
            db,
            children.next()?,
            operator.operator(db),
        )),
        TypeData::Literal(literal) => TypeData::Literal(InternedLiteral::new(
            db,
            match literal.literal(db) {
                Literal::Object(members) => {
                    Literal::Object(rebuild_type_members(members, &mut children)?)
                }
                literal @ (Literal::BigInt(_)
                | Literal::Boolean(_)
                | Literal::Number(_)
                | Literal::RegExp(_)
                | Literal::String(_)
                | Literal::Template(_)) => literal.clone(),
            },
        )),
        TypeData::InstanceOf(instance) => TypeData::instance_of(
            db,
            children.next()?,
            take_types(&mut children, instance.type_parameters(db).len())?,
        ),
        TypeData::MergedReference(reference) => {
            TypeData::MergedReference(InternedMergedReference::new(
                db,
                take_optional_type(&mut children, reference.ty(db))?,
                take_optional_type(&mut children, reference.value_ty(db))?,
                take_optional_type(&mut children, reference.namespace_ty(db))?,
            ))
        }
        TypeData::TypeofExpression(expression) => {
            TypeData::TypeofExpression(InternedTypeofExpression::new(
                db,
                rebuild_typeof_expression(expression.expression(db), &mut children)?,
            ))
        }
        TypeData::TypeofType(_) => {
            TypeData::TypeofType(InternedTypeofType::new(db, children.next()?))
        }
        TypeData::TypeofValue(value) => TypeData::TypeofValue(InternedTypeofValue::new(
            db,
            children.next()?,
            value.identifier(db).clone(),
            value.scope_id(db),
        )),
        TypeData::Unknown
        | TypeData::Divergent(_)
        | TypeData::Global
        | TypeData::GlobalType(_)
        | TypeData::BigInt
        | TypeData::Boolean
        | TypeData::Null
        | TypeData::Number
        | TypeData::String
        | TypeData::Symbol
        | TypeData::Undefined
        | TypeData::Conditional
        | TypeData::Local(_)
        | TypeData::AnyKeyword
        | TypeData::NeverKeyword
        | TypeData::ObjectKeyword
        | TypeData::ThisKeyword
        | TypeData::UnknownKeyword
        | TypeData::VoidKeyword => ty,
    };
    children.next().is_none().then_some(rebuilt)
}

fn take_types<'db>(
    children: &mut impl Iterator<Item = TypeData<'db>>,
    count: usize,
) -> Option<Box<[TypeData<'db>]>> {
    (0..count).map(|_| children.next()).collect()
}

fn take_optional_type<'db>(
    children: &mut impl Iterator<Item = TypeData<'db>>,
    original: Option<TypeData<'db>>,
) -> Option<Option<TypeData<'db>>> {
    if original.is_some() {
        Some(Some(children.next()?))
    } else {
        Some(None)
    }
}

fn rebuild_function_parameter<'db>(
    parameter: &FunctionParameter<'db>,
    children: &mut impl Iterator<Item = TypeData<'db>>,
) -> Option<FunctionParameter<'db>> {
    Some(match parameter {
        FunctionParameter::Named(parameter) => FunctionParameter::Named(NamedFunctionParameter {
            name: parameter.name.clone(),
            ty: children.next()?,
            is_optional: parameter.is_optional,
            is_rest: parameter.is_rest,
        }),
        FunctionParameter::Pattern(parameter) => {
            FunctionParameter::Pattern(PatternFunctionParameter {
                bindings: parameter
                    .bindings
                    .iter()
                    .map(|binding| {
                        Some(FunctionParameterBinding {
                            name: binding.name.clone(),
                            ty: children.next()?,
                        })
                    })
                    .collect::<Option<Box<[_]>>>()?,
                ty: children.next()?,
                is_optional: parameter.is_optional,
                is_rest: parameter.is_rest,
            })
        }
    })
}

fn rebuild_return_type<'db>(
    return_type: &ReturnType<'db>,
    children: &mut impl Iterator<Item = TypeData<'db>>,
) -> Option<ReturnType<'db>> {
    Some(match return_type {
        ReturnType::Type(_) => ReturnType::Type(children.next()?),
        ReturnType::Predicate(predicate) => ReturnType::Predicate(PredicateReturnType {
            parameter_name: predicate.parameter_name.clone(),
            ty: children.next()?,
        }),
        ReturnType::Asserts(asserts) => ReturnType::Asserts(AssertsReturnType {
            parameter_name: asserts.parameter_name.clone(),
            ty: children.next()?,
        }),
    })
}

fn rebuild_type_members<'db>(
    members: &[TypeMember<'db>],
    children: &mut impl Iterator<Item = TypeData<'db>>,
) -> Option<Box<[TypeMember<'db>]>> {
    members
        .iter()
        .map(|member| {
            Some(TypeMember {
                kind: rebuild_type_member_kind(&member.kind, children)?,
                ty: children.next()?,
            })
        })
        .collect()
}

fn rebuild_type_member_kind<'db>(
    kind: &TypeMemberKind<'db>,
    children: &mut impl Iterator<Item = TypeData<'db>>,
) -> Option<TypeMemberKind<'db>> {
    Some(match kind {
        TypeMemberKind::CallSignature => TypeMemberKind::CallSignature,
        TypeMemberKind::ComputedValue(_) => TypeMemberKind::ComputedValue(children.next()?),
        TypeMemberKind::ComputedValueNamed(name, _) => {
            TypeMemberKind::ComputedValueNamed(name.clone(), children.next()?)
        }
        TypeMemberKind::ConstAssertedCallSignature => TypeMemberKind::ConstAssertedCallSignature,
        TypeMemberKind::ConstAssertedComputedValue(_) => {
            TypeMemberKind::ConstAssertedComputedValue(children.next()?)
        }
        TypeMemberKind::ConstAssertedComputedValueNamed(name, _) => {
            TypeMemberKind::ConstAssertedComputedValueNamed(name.clone(), children.next()?)
        }
        TypeMemberKind::ConstAssertedConstructor => TypeMemberKind::ConstAssertedConstructor,
        TypeMemberKind::ConstAssertedGetter(name) => {
            TypeMemberKind::ConstAssertedGetter(name.clone())
        }
        TypeMemberKind::ConstAssertedIndexSignature(_) => {
            TypeMemberKind::ConstAssertedIndexSignature(children.next()?)
        }
        TypeMemberKind::ConstAssertedNamed(name) => {
            TypeMemberKind::ConstAssertedNamed(name.clone())
        }
        TypeMemberKind::ConstAssertedNamedOptional(name) => {
            TypeMemberKind::ConstAssertedNamedOptional(name.clone())
        }
        TypeMemberKind::ConstAssertedNamedStatic(name) => {
            TypeMemberKind::ConstAssertedNamedStatic(name.clone())
        }
        TypeMemberKind::Constructor => TypeMemberKind::Constructor,
        TypeMemberKind::Getter(name) => TypeMemberKind::Getter(name.clone()),
        TypeMemberKind::IndexSignature(_) => TypeMemberKind::IndexSignature(children.next()?),
        TypeMemberKind::Named(name) => TypeMemberKind::Named(name.clone()),
        TypeMemberKind::NamedOptional(name) => TypeMemberKind::NamedOptional(name.clone()),
        TypeMemberKind::NamedStatic(name) => TypeMemberKind::NamedStatic(name.clone()),
    })
}

#[deny(clippy::wildcard_enum_match_arm)]
fn rebuild_typeof_expression<'db>(
    expression: &TypeofExpression<'db>,
    children: &mut impl Iterator<Item = TypeData<'db>>,
) -> Option<TypeofExpression<'db>> {
    Some(match expression {
        TypeofExpression::Addition(_) => TypeofExpression::Addition(TypeofAdditionExpression {
            left: children.next()?,
            right: children.next()?,
        }),
        TypeofExpression::Await(_) => TypeofExpression::Await(TypeofAwaitExpression {
            argument: children.next()?,
        }),
        TypeofExpression::BitwiseNot(_) => {
            TypeofExpression::BitwiseNot(TypeofBitwiseNotExpression {
                argument: children.next()?,
            })
        }
        TypeofExpression::Call(expression) => TypeofExpression::Call(TypeofCallExpression {
            callee: children.next()?,
            arguments: rebuild_call_arguments(&expression.arguments, children)?,
        }),
        TypeofExpression::Conditional(_) => {
            TypeofExpression::Conditional(TypeofConditionalExpression {
                test: children.next()?,
                consequent: children.next()?,
                alternate: children.next()?,
            })
        }
        TypeofExpression::Destructure(expression) => {
            TypeofExpression::Destructure(TypeofDestructureExpression {
                ty: children.next()?,
                destructure_field: expression.destructure_field.clone(),
            })
        }
        TypeofExpression::Index(expression) => TypeofExpression::Index(TypeofIndexExpression {
            object: children.next()?,
            index: expression.index,
        }),
        TypeofExpression::IterableValueOf(_) => {
            TypeofExpression::IterableValueOf(TypeofIterableValueOfExpression {
                ty: children.next()?,
            })
        }
        TypeofExpression::LogicalAnd(_) => {
            TypeofExpression::LogicalAnd(TypeofLogicalAndExpression {
                left: children.next()?,
                right: children.next()?,
            })
        }
        TypeofExpression::LogicalOr(_) => TypeofExpression::LogicalOr(TypeofLogicalOrExpression {
            left: children.next()?,
            right: children.next()?,
        }),
        TypeofExpression::New(expression) => TypeofExpression::New(TypeofNewExpression {
            callee: children.next()?,
            arguments: rebuild_call_arguments(&expression.arguments, children)?,
        }),
        TypeofExpression::NullishCoalescing(_) => {
            TypeofExpression::NullishCoalescing(TypeofNullishCoalescingExpression {
                left: children.next()?,
                right: children.next()?,
            })
        }
        TypeofExpression::StaticMember(expression) => {
            TypeofExpression::StaticMember(TypeofStaticMemberExpression {
                object: children.next()?,
                member: expression.member.clone(),
            })
        }
        TypeofExpression::Super(_) => TypeofExpression::Super(TypeofThisOrSuperExpression {
            parent: children.next()?,
        }),
        TypeofExpression::This(_) => TypeofExpression::This(TypeofThisOrSuperExpression {
            parent: children.next()?,
        }),
        TypeofExpression::Typeof(_) => TypeofExpression::Typeof(TypeofTypeofExpression {
            argument: children.next()?,
        }),
        TypeofExpression::UnaryMinus(_) => {
            TypeofExpression::UnaryMinus(TypeofUnaryMinusExpression {
                argument: children.next()?,
            })
        }
    })
}

fn rebuild_call_arguments<'db>(
    arguments: &[CallArgumentType<'db>],
    children: &mut impl Iterator<Item = TypeData<'db>>,
) -> Option<Box<[CallArgumentType<'db>]>> {
    arguments
        .iter()
        .map(|argument| {
            Some(match argument {
                CallArgumentType::Argument(_) => CallArgumentType::Argument(children.next()?),
                CallArgumentType::Spread(_) => CallArgumentType::Spread(children.next()?),
            })
        })
        .collect()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub enum Literal<'db> {
    BigInt(Text),
    Boolean(BooleanLiteral),
    Number(NumberLiteral),
    Object(Box<[TypeMember<'db>]>),
    RegExp(RegexpLiteral),
    String(StringLiteral),
    Template(Text),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub enum ReturnType<'db> {
    Type(TypeData<'db>),
    Predicate(PredicateReturnType<'db>),
    Asserts(AssertsReturnType<'db>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct PredicateReturnType<'db> {
    pub parameter_name: Text,
    pub ty: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct AssertsReturnType<'db> {
    pub parameter_name: Text,
    pub ty: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct ConstructorParameter<'db> {
    pub parameter: FunctionParameter<'db>,
    pub accessibility: Option<raw::TypeMemberAccessibility>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub enum FunctionParameter<'db> {
    Named(NamedFunctionParameter<'db>),
    Pattern(PatternFunctionParameter<'db>),
}

impl<'db> FunctionParameter<'db> {
    pub fn ty(&self) -> TypeData<'db> {
        match self {
            Self::Named(parameter) => parameter.ty,
            Self::Pattern(parameter) => parameter.ty,
        }
    }

    pub fn is_optional(&self) -> bool {
        match self {
            Self::Named(parameter) => parameter.is_optional,
            Self::Pattern(parameter) => parameter.is_optional,
        }
    }

    pub fn is_rest(&self) -> bool {
        match self {
            Self::Named(parameter) => parameter.is_rest,
            Self::Pattern(parameter) => parameter.is_rest,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct NamedFunctionParameter<'db> {
    pub name: Text,
    pub ty: TypeData<'db>,
    pub is_optional: bool,
    pub is_rest: bool,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct PatternFunctionParameter<'db> {
    pub bindings: Box<[FunctionParameterBinding<'db>]>,
    pub ty: TypeData<'db>,
    pub is_optional: bool,
    pub is_rest: bool,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct FunctionParameterBinding<'db> {
    pub name: Text,
    pub ty: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TupleElementType<'db> {
    pub ty: TypeData<'db>,
    pub name: Option<Text>,
    pub is_optional: bool,
    pub is_rest: bool,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeMember<'db> {
    pub kind: TypeMemberKind<'db>,
    pub ty: TypeData<'db>,
}

impl TypeMember<'_> {
    pub(crate) fn name(&self) -> Option<Text> {
        self.kind.name()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub enum TypeMemberKind<'db> {
    CallSignature,
    ComputedValue(TypeData<'db>),
    ComputedValueNamed(Text, TypeData<'db>),
    ConstAssertedCallSignature,
    ConstAssertedComputedValue(TypeData<'db>),
    ConstAssertedComputedValueNamed(Text, TypeData<'db>),
    ConstAssertedConstructor,
    ConstAssertedGetter(Text),
    ConstAssertedIndexSignature(TypeData<'db>),
    ConstAssertedNamed(Text),
    ConstAssertedNamedOptional(Text),
    ConstAssertedNamedStatic(Text),
    Constructor,
    Getter(Text),
    IndexSignature(TypeData<'db>),
    Named(Text),
    NamedOptional(Text),
    NamedStatic(Text),
}

impl<'db> TypeMemberKind<'db> {
    pub fn has_name(&self, name: &str) -> bool {
        match self {
            Self::Constructor | Self::ConstAssertedConstructor => name == "constructor",
            Self::Getter(own_name)
            | Self::ConstAssertedGetter(own_name)
            | Self::ComputedValueNamed(own_name, _)
            | Self::ConstAssertedComputedValueNamed(own_name, _)
            | Self::Named(own_name)
            | Self::ConstAssertedNamed(own_name)
            | Self::NamedOptional(own_name)
            | Self::ConstAssertedNamedOptional(own_name)
            | Self::NamedStatic(own_name)
            | Self::ConstAssertedNamedStatic(own_name) => own_name.text() == name,
            Self::CallSignature
            | Self::ComputedValue(_)
            | Self::ConstAssertedCallSignature
            | Self::ConstAssertedComputedValue(_)
            | Self::ConstAssertedIndexSignature(_)
            | Self::IndexSignature(_) => false,
        }
    }

    pub fn index_signature_type(&self) -> Option<TypeData<'db>> {
        match self {
            Self::IndexSignature(ty) | Self::ConstAssertedIndexSignature(ty) => Some(*ty),
            _ => None,
        }
    }

    pub fn is_static(&self) -> bool {
        matches!(
            self,
            Self::Constructor
                | Self::ConstAssertedConstructor
                | Self::NamedStatic(_)
                | Self::ConstAssertedNamedStatic(_)
        )
    }

    pub fn is_constructor(&self) -> bool {
        matches!(self, Self::Constructor | Self::ConstAssertedConstructor)
    }

    pub fn is_optional(&self) -> bool {
        matches!(
            self,
            Self::NamedOptional(_) | Self::ConstAssertedNamedOptional(_)
        )
    }

    pub fn is_const_asserted(&self) -> bool {
        matches!(
            self,
            Self::ConstAssertedCallSignature
                | Self::ConstAssertedComputedValue(_)
                | Self::ConstAssertedComputedValueNamed(_, _)
                | Self::ConstAssertedConstructor
                | Self::ConstAssertedGetter(_)
                | Self::ConstAssertedIndexSignature(_)
                | Self::ConstAssertedNamed(_)
                | Self::ConstAssertedNamedOptional(_)
                | Self::ConstAssertedNamedStatic(_)
        )
    }

    pub fn with_optional(self) -> Self {
        match self {
            Self::Named(name) => Self::NamedOptional(name),
            Self::ConstAssertedNamed(name) => Self::ConstAssertedNamedOptional(name),
            other => other,
        }
    }

    pub fn without_optional(self) -> Self {
        match self {
            Self::NamedOptional(name) => Self::Named(name),
            Self::ConstAssertedNamedOptional(name) => Self::ConstAssertedNamed(name),
            other => other,
        }
    }

    pub fn is_call_signature(&self) -> bool {
        matches!(self, Self::CallSignature | Self::ConstAssertedCallSignature)
    }

    pub fn name(&self) -> Option<Text> {
        match self {
            Self::CallSignature
            | Self::ComputedValue(_)
            | Self::ConstAssertedCallSignature
            | Self::ConstAssertedComputedValue(_)
            | Self::ConstAssertedIndexSignature(_)
            | Self::IndexSignature(_) => None,
            Self::ConstAssertedConstructor | Self::Constructor => {
                Some(Text::new_static("constructor"))
            }
            Self::ConstAssertedGetter(name)
            | Self::ConstAssertedComputedValueNamed(name, _)
            | Self::ConstAssertedNamed(name)
            | Self::ConstAssertedNamedOptional(name)
            | Self::ConstAssertedNamedStatic(name)
            | Self::Getter(name)
            | Self::ComputedValueNamed(name, _)
            | Self::Named(name)
            | Self::NamedOptional(name)
            | Self::NamedStatic(name) => Some(name.clone()),
        }
    }

    pub fn computed_name(&self) -> Option<&str> {
        match self {
            Self::ComputedValueNamed(name, _) | Self::ConstAssertedComputedValueNamed(name, _) => {
                Some(name.text())
            }
            _ => None,
        }
    }

    pub fn computed_value_type(&self) -> Option<TypeData<'db>> {
        match self {
            Self::ComputedValue(ty)
            | Self::ComputedValueNamed(_, ty)
            | Self::ConstAssertedComputedValue(ty)
            | Self::ConstAssertedComputedValueNamed(_, ty) => Some(*ty),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub enum TypeofExpression<'db> {
    Addition(TypeofAdditionExpression<'db>),
    Await(TypeofAwaitExpression<'db>),
    BitwiseNot(TypeofBitwiseNotExpression<'db>),
    Call(TypeofCallExpression<'db>),
    Conditional(TypeofConditionalExpression<'db>),
    Destructure(TypeofDestructureExpression<'db>),
    Index(TypeofIndexExpression<'db>),
    IterableValueOf(TypeofIterableValueOfExpression<'db>),
    LogicalAnd(TypeofLogicalAndExpression<'db>),
    LogicalOr(TypeofLogicalOrExpression<'db>),
    New(TypeofNewExpression<'db>),
    NullishCoalescing(TypeofNullishCoalescingExpression<'db>),
    StaticMember(TypeofStaticMemberExpression<'db>),
    Super(TypeofThisOrSuperExpression<'db>),
    This(TypeofThisOrSuperExpression<'db>),
    Typeof(TypeofTypeofExpression<'db>),
    UnaryMinus(TypeofUnaryMinusExpression<'db>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofAdditionExpression<'db> {
    pub left: TypeData<'db>,
    pub right: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofAwaitExpression<'db> {
    pub argument: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofBitwiseNotExpression<'db> {
    pub argument: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofCallExpression<'db> {
    pub callee: TypeData<'db>,
    pub arguments: Box<[CallArgumentType<'db>]>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofConditionalExpression<'db> {
    pub test: TypeData<'db>,
    pub consequent: TypeData<'db>,
    pub alternate: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofDestructureExpression<'db> {
    pub ty: TypeData<'db>,
    pub destructure_field: raw::DestructureField,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofIterableValueOfExpression<'db> {
    pub ty: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofLogicalAndExpression<'db> {
    pub left: TypeData<'db>,
    pub right: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofLogicalOrExpression<'db> {
    pub left: TypeData<'db>,
    pub right: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofNewExpression<'db> {
    pub callee: TypeData<'db>,
    pub arguments: Box<[CallArgumentType<'db>]>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub enum CallArgumentType<'db> {
    Argument(TypeData<'db>),
    Spread(TypeData<'db>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofIndexExpression<'db> {
    pub object: TypeData<'db>,
    pub index: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofNullishCoalescingExpression<'db> {
    pub left: TypeData<'db>,
    pub right: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofStaticMemberExpression<'db> {
    pub object: TypeData<'db>,
    pub member: Text,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofThisOrSuperExpression<'db> {
    pub parent: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofTypeofExpression<'db> {
    pub argument: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofUnaryMinusExpression<'db> {
    pub argument: TypeData<'db>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedFunction<'db> {
    #[returns(ref)]
    pub type_parameters: Box<[TypeData<'db>]>,
    #[returns(ref)]
    pub parameters: Box<[FunctionParameter<'db>]>,
    #[returns(ref)]
    pub return_type: ReturnType<'db>,
    pub is_async: bool,
    #[returns(ref)]
    pub name: Option<Text>,
}

impl<'db> InternedFunction<'db> {
    pub(crate) fn intersection_with(self, db: &'db dyn TypeDb, other: Self) -> Self {
        let return_type = match (self.return_type(db), other.return_type(db)) {
            (ReturnType::Type(left), ReturnType::Type(right)) => {
                ReturnType::Type(TypeData::union_from_types(db, Vec::from([*left, *right])))
            }
            _ => ReturnType::Type(TypeData::Boolean),
        };

        Self::new(db, Box::default(), Box::default(), return_type, false, None)
    }

    pub fn returns_promise(self, db: &'db dyn TypeDb) -> bool {
        match self.return_type(db) {
            ReturnType::Type(ty) => ty.is_promise_instance(db),
            ReturnType::Predicate(_) | ReturnType::Asserts(_) => false,
        }
    }
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedClass<'db> {
    #[returns(ref)]
    pub type_parameters: Box<[TypeData<'db>]>,
    pub extends: Option<TypeData<'db>>,
    #[returns(ref)]
    pub implements: Box<[TypeData<'db>]>,
    #[returns(ref)]
    pub members: Box<[TypeMember<'db>]>,
    #[returns(ref)]
    pub name: Option<Text>,
    pub is_builtin: bool,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedConstructor<'db> {
    #[returns(ref)]
    pub type_parameters: Box<[TypeData<'db>]>,
    #[returns(ref)]
    pub parameters: Box<[ConstructorParameter<'db>]>,
    pub return_type: Option<TypeData<'db>>,
}

impl<'db> InternedConstructor<'db> {
    pub fn accepts_argument_count(self, db: &'db dyn TypeDb, argument_count: usize) -> bool {
        let parameters = self.parameters(db);
        let required_count = parameters
            .iter()
            .filter(|parameter| {
                !parameter.parameter.is_optional() && !parameter.parameter.is_rest()
            })
            .count();
        let has_rest = parameters
            .iter()
            .any(|parameter| parameter.parameter.is_rest());

        required_count <= argument_count && (has_rest || argument_count <= parameters.len())
    }
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedInterface<'db> {
    #[returns(ref)]
    pub type_parameters: Box<[TypeData<'db>]>,
    #[returns(ref)]
    pub extends: Box<[TypeData<'db>]>,
    #[returns(ref)]
    pub members: Box<[TypeMember<'db>]>,
    #[returns(ref)]
    pub name: Text,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedObject<'db> {
    pub prototype: Option<TypeData<'db>>,
    #[returns(ref)]
    pub members: Box<[TypeMember<'db>]>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedUnion<'db> {
    #[returns(ref)]
    pub types: Box<[TypeData<'db>]>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedIntersection<'db> {
    #[returns(ref)]
    pub types: Box<[TypeData<'db>]>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedTuple<'db> {
    #[returns(ref)]
    pub elements: Box<[TupleElementType<'db>]>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedModule<'db> {
    #[returns(ref)]
    pub members: Box<[TypeMember<'db>]>,
    #[returns(ref)]
    pub name: Text,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedNamespace<'db> {
    #[returns(ref)]
    pub members: Box<[TypeMember<'db>]>,
    #[returns(ref)]
    pub path: raw::Path,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedLiteral<'db> {
    #[returns(ref)]
    pub literal: Literal<'db>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedTypeInstance<'db> {
    pub ty: TypeData<'db>,
    #[returns(ref)]
    pub type_parameters: Box<[TypeData<'db>]>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedMergedReference<'db> {
    pub ty: Option<TypeData<'db>>,
    pub value_ty: Option<TypeData<'db>>,
    pub namespace_ty: Option<TypeData<'db>>,
}

impl<'db> InternedMergedReference<'db> {
    /// Returns the types stored in this merged reference.
    ///
    /// Missing parts are skipped. The order is type, value, then namespace.
    pub fn targets(self, db: &'db dyn TypeDb) -> impl Iterator<Item = TypeData<'db>> {
        // Preserve the raw merged-reference order: type, value, then namespace.
        // If future lookups need a narrower target set, this is the single place to change it.
        [self.ty(db), self.value_ty(db), self.namespace_ty(db)]
            .into_iter()
            .flatten()
    }
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedGenericTypeParameter<'db> {
    pub constraint: Option<TypeData<'db>>,
    pub default: Option<TypeData<'db>>,
    #[returns(ref)]
    pub name: Text,
}

#[salsa::interned]
#[derive(Debug)]
pub struct LocalTypeHandle<'db> {
    pub module: ModuleKey,
    pub type_id: LocalTypeId,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedTypeOperatorType<'db> {
    pub ty: TypeData<'db>,
    pub operator: raw::TypeOperator,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedTypeofExpression<'db> {
    #[returns(ref)]
    pub expression: TypeofExpression<'db>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedTypeofType<'db> {
    pub ty: TypeData<'db>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedTypeofValue<'db> {
    pub ty: TypeData<'db>,
    #[returns(ref)]
    pub identifier: Text,
    pub scope_id: Option<ScopeId>,
}

fn convert_references<'db>(
    db: &'db dyn TypeDb,
    references: &[raw::TypeReference],
    resolve_reference: &mut ReferenceResolver<'db, '_>,
) -> Box<[TypeData<'db>]> {
    let _ = db;
    references.iter().map(resolve_reference).collect()
}

fn convert_type_members<'db>(
    db: &'db dyn TypeDb,
    members: &[raw::TypeMember],
    resolve_reference: &mut ReferenceResolver<'db, '_>,
) -> Box<[TypeMember<'db>]> {
    members
        .iter()
        .map(|member| TypeMember {
            kind: convert_type_member_kind(db, &member.kind, resolve_reference),
            ty: resolve_reference(&member.ty),
        })
        .collect()
}

fn convert_type_member_kind<'db>(
    db: &'db dyn TypeDb,
    kind: &raw::TypeMemberKind,
    resolve_reference: &mut ReferenceResolver<'db, '_>,
) -> TypeMemberKind<'db> {
    let _ = db;
    match kind {
        raw::TypeMemberKind::CallSignature => TypeMemberKind::CallSignature,
        raw::TypeMemberKind::ComputedValue(ty) => {
            let resolved = resolve_reference(ty);
            well_known_symbol_name(ty).map_or(TypeMemberKind::ComputedValue(resolved), |name| {
                TypeMemberKind::ComputedValueNamed(name, resolved)
            })
        }
        raw::TypeMemberKind::ConstAssertedCallSignature => {
            TypeMemberKind::ConstAssertedCallSignature
        }
        raw::TypeMemberKind::ConstAssertedComputedValue(ty) => {
            let resolved = resolve_reference(ty);
            well_known_symbol_name(ty).map_or(
                TypeMemberKind::ConstAssertedComputedValue(resolved),
                |name| TypeMemberKind::ConstAssertedComputedValueNamed(name, resolved),
            )
        }
        raw::TypeMemberKind::ConstAssertedConstructor => TypeMemberKind::ConstAssertedConstructor,
        raw::TypeMemberKind::ConstAssertedGetter(name) => {
            TypeMemberKind::ConstAssertedGetter(name.clone())
        }
        raw::TypeMemberKind::ConstAssertedIndexSignature(ty) => {
            TypeMemberKind::ConstAssertedIndexSignature(resolve_reference(ty))
        }
        raw::TypeMemberKind::ConstAssertedNamed(name) => {
            TypeMemberKind::ConstAssertedNamed(name.clone())
        }
        raw::TypeMemberKind::ConstAssertedNamedOptional(name) => {
            TypeMemberKind::ConstAssertedNamedOptional(name.clone())
        }
        raw::TypeMemberKind::ConstAssertedNamedStatic(name) => {
            TypeMemberKind::ConstAssertedNamedStatic(name.clone())
        }
        raw::TypeMemberKind::Constructor => TypeMemberKind::Constructor,
        raw::TypeMemberKind::Getter(name) => TypeMemberKind::Getter(name.clone()),
        raw::TypeMemberKind::IndexSignature(ty) => {
            TypeMemberKind::IndexSignature(resolve_reference(ty))
        }
        raw::TypeMemberKind::Named(name) => TypeMemberKind::Named(name.clone()),
        raw::TypeMemberKind::NamedOptional(name) => TypeMemberKind::NamedOptional(name.clone()),
        raw::TypeMemberKind::NamedStatic(name) => TypeMemberKind::NamedStatic(name.clone()),
    }
}

fn convert_constructor_parameters<'db>(
    db: &'db dyn TypeDb,
    parameters: &[raw::ConstructorParameter],
    resolve_reference: &mut ReferenceResolver<'db, '_>,
) -> Box<[ConstructorParameter<'db>]> {
    parameters
        .iter()
        .map(|parameter| ConstructorParameter {
            parameter: convert_function_parameter(db, &parameter.parameter, resolve_reference),
            accessibility: parameter.accessibility,
        })
        .collect()
}

fn convert_function_parameters<'db>(
    db: &'db dyn TypeDb,
    parameters: &[raw::FunctionParameter],
    resolve_reference: &mut ReferenceResolver<'db, '_>,
) -> Box<[FunctionParameter<'db>]> {
    parameters
        .iter()
        .map(|parameter| convert_function_parameter(db, parameter, resolve_reference))
        .collect()
}

fn convert_function_parameter<'db>(
    db: &'db dyn TypeDb,
    parameter: &raw::FunctionParameter,
    resolve_reference: &mut ReferenceResolver<'db, '_>,
) -> FunctionParameter<'db> {
    let _ = db;
    match parameter {
        raw::FunctionParameter::Named(named) => FunctionParameter::Named(NamedFunctionParameter {
            name: named.name.clone(),
            ty: resolve_reference(&named.ty),
            is_optional: named.is_optional,
            is_rest: named.is_rest,
        }),
        raw::FunctionParameter::Pattern(pattern) => {
            FunctionParameter::Pattern(PatternFunctionParameter {
                bindings: pattern
                    .bindings
                    .iter()
                    .map(|binding| FunctionParameterBinding {
                        name: binding.name.clone(),
                        ty: resolve_reference(&binding.ty),
                    })
                    .collect(),
                ty: resolve_reference(&pattern.ty),
                is_optional: pattern.is_optional,
                is_rest: pattern.is_rest,
            })
        }
    }
}

fn convert_return_type<'db>(
    db: &'db dyn TypeDb,
    return_type: &raw::ReturnType,
    resolve_reference: &mut ReferenceResolver<'db, '_>,
) -> ReturnType<'db> {
    let _ = db;
    match return_type {
        raw::ReturnType::Type(ty) => ReturnType::Type(resolve_reference(ty)),
        raw::ReturnType::Predicate(predicate) => ReturnType::Predicate(PredicateReturnType {
            parameter_name: predicate.parameter_name.clone(),
            ty: resolve_reference(&predicate.ty),
        }),
        raw::ReturnType::Asserts(asserts) => ReturnType::Asserts(AssertsReturnType {
            parameter_name: asserts.parameter_name.clone(),
            ty: resolve_reference(&asserts.ty),
        }),
    }
}

fn convert_literal<'db>(
    db: &'db dyn TypeDb,
    literal: &raw::Literal,
    resolve_reference: &mut ReferenceResolver<'db, '_>,
) -> Literal<'db> {
    match literal {
        raw::Literal::BigInt(text) => Literal::BigInt(text.clone()),
        raw::Literal::Boolean(boolean) => Literal::Boolean(boolean.clone()),
        raw::Literal::Number(number) => Literal::Number(number.clone()),
        raw::Literal::Object(object) => Literal::Object(convert_type_members(
            db,
            object.members(),
            resolve_reference,
        )),
        raw::Literal::RegExp(regexp) => Literal::RegExp(regexp.clone()),
        raw::Literal::String(string) => Literal::String(string.clone()),
        raw::Literal::Template(text) => Literal::Template(text.clone()),
    }
}

fn convert_typeof_expression<'db>(
    db: &'db dyn TypeDb,
    expression: &raw::TypeofExpression,
    resolve_reference: &mut ReferenceResolver<'db, '_>,
) -> TypeofExpression<'db> {
    let _ = db;
    match expression {
        raw::TypeofExpression::Addition(expression) => {
            TypeofExpression::Addition(TypeofAdditionExpression {
                left: resolve_reference(&expression.left),
                right: resolve_reference(&expression.right),
            })
        }
        raw::TypeofExpression::Await(expression) => {
            TypeofExpression::Await(TypeofAwaitExpression {
                argument: resolve_reference(&expression.argument),
            })
        }
        raw::TypeofExpression::BitwiseNot(expression) => {
            TypeofExpression::BitwiseNot(TypeofBitwiseNotExpression {
                argument: resolve_reference(&expression.argument),
            })
        }
        raw::TypeofExpression::Call(expression) => TypeofExpression::Call(TypeofCallExpression {
            callee: resolve_reference(&expression.callee),
            arguments: convert_call_arguments(db, &expression.arguments, resolve_reference),
        }),
        raw::TypeofExpression::Conditional(expression) => {
            TypeofExpression::Conditional(TypeofConditionalExpression {
                test: resolve_reference(&expression.test),
                consequent: resolve_reference(&expression.consequent),
                alternate: resolve_reference(&expression.alternate),
            })
        }
        raw::TypeofExpression::Destructure(expression) => {
            TypeofExpression::Destructure(TypeofDestructureExpression {
                ty: resolve_reference(&expression.ty),
                destructure_field: expression.destructure_field.clone(),
            })
        }
        raw::TypeofExpression::Index(expression) => {
            TypeofExpression::Index(TypeofIndexExpression {
                object: resolve_reference(&expression.object),
                index: expression.index,
            })
        }
        raw::TypeofExpression::IterableValueOf(expression) => {
            TypeofExpression::IterableValueOf(TypeofIterableValueOfExpression {
                ty: resolve_reference(&expression.ty),
            })
        }
        raw::TypeofExpression::LogicalAnd(expression) => {
            TypeofExpression::LogicalAnd(TypeofLogicalAndExpression {
                left: resolve_reference(&expression.left),
                right: resolve_reference(&expression.right),
            })
        }
        raw::TypeofExpression::LogicalOr(expression) => {
            TypeofExpression::LogicalOr(TypeofLogicalOrExpression {
                left: resolve_reference(&expression.left),
                right: resolve_reference(&expression.right),
            })
        }
        raw::TypeofExpression::New(expression) => TypeofExpression::New(TypeofNewExpression {
            callee: resolve_reference(&expression.callee),
            arguments: convert_call_arguments(db, &expression.arguments, resolve_reference),
        }),
        raw::TypeofExpression::NullishCoalescing(expression) => {
            TypeofExpression::NullishCoalescing(TypeofNullishCoalescingExpression {
                left: resolve_reference(&expression.left),
                right: resolve_reference(&expression.right),
            })
        }
        raw::TypeofExpression::StaticMember(expression) => {
            TypeofExpression::StaticMember(TypeofStaticMemberExpression {
                object: resolve_reference(&expression.object),
                member: expression.member.clone(),
            })
        }
        raw::TypeofExpression::Super(expression) => {
            TypeofExpression::Super(TypeofThisOrSuperExpression {
                parent: resolve_reference(&expression.parent),
            })
        }
        raw::TypeofExpression::This(expression) => {
            TypeofExpression::This(TypeofThisOrSuperExpression {
                parent: resolve_reference(&expression.parent),
            })
        }
        raw::TypeofExpression::Typeof(expression) => {
            TypeofExpression::Typeof(TypeofTypeofExpression {
                argument: resolve_reference(&expression.argument),
            })
        }
        raw::TypeofExpression::UnaryMinus(expression) => {
            TypeofExpression::UnaryMinus(TypeofUnaryMinusExpression {
                argument: resolve_reference(&expression.argument),
            })
        }
    }
}

fn convert_call_arguments<'db>(
    db: &'db dyn TypeDb,
    arguments: &[raw::CallArgumentType],
    resolve_reference: &mut ReferenceResolver<'db, '_>,
) -> Box<[CallArgumentType<'db>]> {
    let _ = db;
    arguments
        .iter()
        .map(|argument| match argument {
            raw::CallArgumentType::Argument(ty) => {
                CallArgumentType::Argument(resolve_reference(ty))
            }
            raw::CallArgumentType::Spread(ty) => CallArgumentType::Spread(resolve_reference(ty)),
        })
        .collect()
}

fn raw_references_from_types<'db>(
    _db: &'db dyn TypeDb,
    types: &[TypeData<'db>],
) -> Box<[raw::TypeReference]> {
    types.iter().map(|ty| ty.to_raw_reference_lossy()).collect()
}

fn raw_type_members_from_types<'db>(
    db: &'db dyn TypeDb,
    members: &[TypeMember<'db>],
) -> Box<[raw::TypeMember]> {
    members
        .iter()
        .map(|member| raw::TypeMember {
            kind: raw_type_member_kind_from_type(db, &member.kind),
            ty: member.ty.to_raw_reference_lossy(),
        })
        .collect()
}

fn raw_type_member_kind_from_type<'db>(
    _db: &'db dyn TypeDb,
    kind: &TypeMemberKind<'db>,
) -> raw::TypeMemberKind {
    match kind {
        TypeMemberKind::CallSignature => raw::TypeMemberKind::CallSignature,
        TypeMemberKind::ComputedValue(ty) => {
            raw::TypeMemberKind::ComputedValue(ty.to_raw_reference_lossy())
        }
        TypeMemberKind::ComputedValueNamed(_, ty) => {
            raw::TypeMemberKind::ComputedValue(ty.to_raw_reference_lossy())
        }
        TypeMemberKind::ConstAssertedCallSignature => {
            raw::TypeMemberKind::ConstAssertedCallSignature
        }
        TypeMemberKind::ConstAssertedComputedValue(ty) => {
            raw::TypeMemberKind::ConstAssertedComputedValue(ty.to_raw_reference_lossy())
        }
        TypeMemberKind::ConstAssertedComputedValueNamed(_, ty) => {
            raw::TypeMemberKind::ConstAssertedComputedValue(ty.to_raw_reference_lossy())
        }
        TypeMemberKind::ConstAssertedConstructor => raw::TypeMemberKind::ConstAssertedConstructor,
        TypeMemberKind::ConstAssertedGetter(name) => {
            raw::TypeMemberKind::ConstAssertedGetter(name.clone())
        }
        TypeMemberKind::ConstAssertedIndexSignature(ty) => {
            raw::TypeMemberKind::ConstAssertedIndexSignature(ty.to_raw_reference_lossy())
        }
        TypeMemberKind::ConstAssertedNamed(name) => {
            raw::TypeMemberKind::ConstAssertedNamed(name.clone())
        }
        TypeMemberKind::ConstAssertedNamedOptional(name) => {
            raw::TypeMemberKind::ConstAssertedNamedOptional(name.clone())
        }
        TypeMemberKind::ConstAssertedNamedStatic(name) => {
            raw::TypeMemberKind::ConstAssertedNamedStatic(name.clone())
        }
        TypeMemberKind::Constructor => raw::TypeMemberKind::Constructor,
        TypeMemberKind::Getter(name) => raw::TypeMemberKind::Getter(name.clone()),
        TypeMemberKind::IndexSignature(ty) => {
            raw::TypeMemberKind::IndexSignature(ty.to_raw_reference_lossy())
        }
        TypeMemberKind::Named(name) => raw::TypeMemberKind::Named(name.clone()),
        TypeMemberKind::NamedOptional(name) => raw::TypeMemberKind::NamedOptional(name.clone()),
        TypeMemberKind::NamedStatic(name) => raw::TypeMemberKind::NamedStatic(name.clone()),
    }
}

fn raw_constructor_parameters_from_types<'db>(
    db: &'db dyn TypeDb,
    parameters: &[ConstructorParameter<'db>],
) -> Box<[raw::ConstructorParameter]> {
    parameters
        .iter()
        .map(|parameter| raw::ConstructorParameter {
            parameter: raw_function_parameter_from_type(db, &parameter.parameter),
            accessibility: parameter.accessibility,
        })
        .collect()
}

fn raw_function_parameters_from_types<'db>(
    db: &'db dyn TypeDb,
    parameters: &[FunctionParameter<'db>],
) -> Box<[raw::FunctionParameter]> {
    parameters
        .iter()
        .map(|parameter| raw_function_parameter_from_type(db, parameter))
        .collect()
}

fn raw_function_parameter_from_type<'db>(
    _db: &'db dyn TypeDb,
    parameter: &FunctionParameter<'db>,
) -> raw::FunctionParameter {
    match parameter {
        FunctionParameter::Named(named) => {
            raw::FunctionParameter::Named(raw::NamedFunctionParameter {
                name: named.name.clone(),
                ty: named.ty.to_raw_reference_lossy(),
                is_optional: named.is_optional,
                is_rest: named.is_rest,
            })
        }
        FunctionParameter::Pattern(pattern) => {
            raw::FunctionParameter::Pattern(raw::PatternFunctionParameter {
                bindings: pattern
                    .bindings
                    .iter()
                    .map(|binding| raw::FunctionParameterBinding {
                        name: binding.name.clone(),
                        ty: binding.ty.to_raw_reference_lossy(),
                    })
                    .collect(),
                ty: pattern.ty.to_raw_reference_lossy(),
                is_optional: pattern.is_optional,
                is_rest: pattern.is_rest,
            })
        }
    }
}

fn raw_return_type_from_type<'db>(
    _db: &'db dyn TypeDb,
    return_type: &ReturnType<'db>,
) -> raw::ReturnType {
    match return_type {
        ReturnType::Type(ty) => raw::ReturnType::Type(ty.to_raw_reference_lossy()),
        ReturnType::Predicate(predicate) => {
            raw::ReturnType::Predicate(Box::new(raw::PredicateReturnType {
                parameter_name: predicate.parameter_name.clone(),
                ty: predicate.ty.to_raw_reference_lossy(),
            }))
        }
        ReturnType::Asserts(asserts) => {
            raw::ReturnType::Asserts(Box::new(raw::AssertsReturnType {
                parameter_name: asserts.parameter_name.clone(),
                ty: asserts.ty.to_raw_reference_lossy(),
            }))
        }
    }
}

fn raw_literal_from_type<'db>(db: &'db dyn TypeDb, literal: &Literal<'db>) -> raw::Literal {
    match literal {
        Literal::BigInt(text) => raw::Literal::BigInt(text.clone()),
        Literal::Boolean(boolean) => raw::Literal::Boolean(boolean.clone()),
        Literal::Number(number) => raw::Literal::Number(number.clone()),
        Literal::Object(members) => {
            raw::Literal::Object(raw::ObjectLiteral(raw_type_members_from_types(db, members)))
        }
        Literal::RegExp(regexp) => raw::Literal::RegExp(regexp.clone()),
        Literal::String(string) => raw::Literal::String(string.clone()),
        Literal::Template(text) => raw::Literal::Template(text.clone()),
    }
}

fn raw_typeof_expression_from_type<'db>(
    db: &'db dyn TypeDb,
    expression: &TypeofExpression<'db>,
) -> raw::TypeofExpression {
    match expression {
        TypeofExpression::Addition(expression) => {
            raw::TypeofExpression::Addition(raw::TypeofAdditionExpression {
                left: expression.left.to_raw_reference_lossy(),
                right: expression.right.to_raw_reference_lossy(),
            })
        }
        TypeofExpression::Await(expression) => {
            raw::TypeofExpression::Await(raw::TypeofAwaitExpression {
                argument: expression.argument.to_raw_reference_lossy(),
            })
        }
        TypeofExpression::BitwiseNot(expression) => {
            raw::TypeofExpression::BitwiseNot(raw::TypeofBitwiseNotExpression {
                argument: expression.argument.to_raw_reference_lossy(),
            })
        }
        TypeofExpression::Call(expression) => {
            raw::TypeofExpression::Call(raw::TypeofCallExpression {
                callee: expression.callee.to_raw_reference_lossy(),
                arguments: raw_call_arguments_from_types(db, &expression.arguments),
            })
        }
        TypeofExpression::Conditional(expression) => {
            raw::TypeofExpression::Conditional(raw::TypeofConditionalExpression {
                test: expression.test.to_raw_reference_lossy(),
                consequent: expression.consequent.to_raw_reference_lossy(),
                alternate: expression.alternate.to_raw_reference_lossy(),
            })
        }
        TypeofExpression::Destructure(expression) => {
            raw::TypeofExpression::Destructure(raw::TypeofDestructureExpression {
                ty: expression.ty.to_raw_reference_lossy(),
                destructure_field: expression.destructure_field.clone(),
            })
        }
        TypeofExpression::Index(expression) => {
            raw::TypeofExpression::Index(raw::TypeofIndexExpression {
                object: expression.object.to_raw_reference_lossy(),
                index: expression.index,
            })
        }
        TypeofExpression::IterableValueOf(expression) => {
            raw::TypeofExpression::IterableValueOf(raw::TypeofIterableValueOfExpression {
                ty: expression.ty.to_raw_reference_lossy(),
            })
        }
        TypeofExpression::LogicalAnd(expression) => {
            raw::TypeofExpression::LogicalAnd(raw::TypeofLogicalAndExpression {
                left: expression.left.to_raw_reference_lossy(),
                right: expression.right.to_raw_reference_lossy(),
            })
        }
        TypeofExpression::LogicalOr(expression) => {
            raw::TypeofExpression::LogicalOr(raw::TypeofLogicalOrExpression {
                left: expression.left.to_raw_reference_lossy(),
                right: expression.right.to_raw_reference_lossy(),
            })
        }
        TypeofExpression::New(expression) => raw::TypeofExpression::New(raw::TypeofNewExpression {
            callee: expression.callee.to_raw_reference_lossy(),
            arguments: raw_call_arguments_from_types(db, &expression.arguments),
        }),
        TypeofExpression::NullishCoalescing(expression) => {
            raw::TypeofExpression::NullishCoalescing(raw::TypeofNullishCoalescingExpression {
                left: expression.left.to_raw_reference_lossy(),
                right: expression.right.to_raw_reference_lossy(),
            })
        }
        TypeofExpression::StaticMember(expression) => {
            raw::TypeofExpression::StaticMember(raw::TypeofStaticMemberExpression {
                object: expression.object.to_raw_reference_lossy(),
                member: expression.member.clone(),
            })
        }
        TypeofExpression::Super(expression) => {
            raw::TypeofExpression::Super(raw::TypeofThisOrSuperExpression {
                parent: expression.parent.to_raw_reference_lossy(),
            })
        }
        TypeofExpression::This(expression) => {
            raw::TypeofExpression::This(raw::TypeofThisOrSuperExpression {
                parent: expression.parent.to_raw_reference_lossy(),
            })
        }
        TypeofExpression::Typeof(expression) => {
            raw::TypeofExpression::Typeof(raw::TypeofTypeofExpression {
                argument: expression.argument.to_raw_reference_lossy(),
            })
        }
        TypeofExpression::UnaryMinus(expression) => {
            raw::TypeofExpression::UnaryMinus(raw::TypeofUnaryMinusExpression {
                argument: expression.argument.to_raw_reference_lossy(),
            })
        }
    }
}

fn raw_call_arguments_from_types<'db>(
    _db: &'db dyn TypeDb,
    arguments: &[CallArgumentType<'db>],
) -> Box<[raw::CallArgumentType]> {
    arguments
        .iter()
        .map(|argument| match argument {
            CallArgumentType::Argument(ty) => {
                raw::CallArgumentType::Argument(ty.to_raw_reference_lossy())
            }
            CallArgumentType::Spread(ty) => {
                raw::CallArgumentType::Spread(ty.to_raw_reference_lossy())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use salsa::plumbing::FromId;

    #[salsa::db]
    #[derive(Default)]
    struct TestDb {
        storage: salsa::Storage<Self>,
    }

    #[salsa::db]
    impl salsa::Database for TestDb {}

    #[salsa::db]
    impl biome_db::Db for TestDb {
        fn parsed_source_for_path(
            &self,
            _path: &camino::Utf8Path,
        ) -> Option<biome_db::ParsedSource> {
            None
        }
    }

    #[salsa::db]
    impl TypeDb for TestDb {}

    #[derive(Default)]
    struct Sentinels(usize);

    impl Sentinels {
        fn next<'db>(&mut self) -> TypeData<'db> {
            let ty = [
                TypeData::Global,
                TypeData::BigInt,
                TypeData::Boolean,
                TypeData::Null,
                TypeData::Number,
                TypeData::String,
                TypeData::Symbol,
                TypeData::Undefined,
                TypeData::Conditional,
                TypeData::AnyKeyword,
                TypeData::NeverKeyword,
                TypeData::ObjectKeyword,
                TypeData::ThisKeyword,
                TypeData::UnknownKeyword,
                TypeData::VoidKeyword,
                TypeData::Unknown,
            ][self.0];
            self.0 += 1;
            ty
        }
    }

    fn text(value: &'static str) -> Text {
        Text::new_static(value)
    }

    fn boxed<T, const N: usize>(values: [T; N]) -> Box<[T]> {
        Box::new(values)
    }

    fn named_parameter<'db>(sentinels: &mut Sentinels) -> FunctionParameter<'db> {
        FunctionParameter::Named(NamedFunctionParameter {
            name: text("named"),
            ty: sentinels.next(),
            is_optional: false,
            is_rest: false,
        })
    }

    fn pattern_parameter<'db>(sentinels: &mut Sentinels) -> FunctionParameter<'db> {
        FunctionParameter::Pattern(PatternFunctionParameter {
            bindings: [FunctionParameterBinding {
                name: text("binding"),
                ty: sentinels.next(),
            }]
            .into(),
            ty: sentinels.next(),
            is_optional: true,
            is_rest: true,
        })
    }

    fn named_member<'db>(sentinels: &mut Sentinels) -> TypeMember<'db> {
        TypeMember {
            kind: TypeMemberKind::Named(text("member")),
            ty: sentinels.next(),
        }
    }

    fn child_bearing_members<'db>(sentinels: &mut Sentinels) -> Box<[TypeMember<'db>]> {
        [
            TypeMemberKind::ComputedValue(sentinels.next()),
            TypeMemberKind::ComputedValueNamed(text("computed"), sentinels.next()),
            TypeMemberKind::ConstAssertedComputedValue(sentinels.next()),
            TypeMemberKind::ConstAssertedComputedValueNamed(
                text("constComputed"),
                sentinels.next(),
            ),
            TypeMemberKind::ConstAssertedIndexSignature(sentinels.next()),
            TypeMemberKind::IndexSignature(sentinels.next()),
        ]
        .into_iter()
        .map(|kind| TypeMember {
            kind,
            ty: sentinels.next(),
        })
        .collect()
    }

    fn typeof_type<'db>(db: &'db TestDb, expression: TypeofExpression<'db>) -> TypeData<'db> {
        TypeData::TypeofExpression(InternedTypeofExpression::new(db, expression))
    }

    fn assert_identity<'db>(db: &'db TestDb, build: impl FnOnce(&mut Sentinels) -> TypeData<'db>) {
        let ty = build(&mut Sentinels::default());
        let children = structural_type_children(db, ty);
        assert!(!children.is_empty());
        assert_eq!(
            children.iter().copied().collect::<FxHashSet<_>>().len(),
            children.len(),
            "test shape must use distinct children"
        );

        assert_eq!(
            ty.try_map_structural(
                db,
                usize::MAX,
                ControlFlow::Continue,
                std::convert::identity,
            ),
            Ok(ty)
        );
        assert_eq!(rebuild_structural_type(db, ty, children.clone()), Some(ty));
        assert!(rebuild_structural_type(db, ty, children[..children.len() - 1].to_vec()).is_none());
        let mut extra_children = children;
        extra_children.push(TypeData::Unknown);
        assert!(rebuild_structural_type(db, ty, extra_children).is_none());
    }

    #[test]
    fn structural_map_reports_step_limit_exceeded() {
        let db = TestDb::default();
        let ty = TypeData::TypeOperator(InternedTypeOperatorType::new(
            &db,
            TypeData::String,
            raw::TypeOperator::Keyof,
        ));

        assert_eq!(
            ty.try_map_structural(&db, 1, ControlFlow::Continue, std::convert::identity,),
            Err(StructuralMapError::StepLimitExceeded)
        );
    }

    fn typeof_chain<'db>(
        db: &'db TestDb,
        distinct_types: usize,
        leaf: TypeData<'db>,
    ) -> TypeData<'db> {
        assert!(distinct_types > 0);
        (1..distinct_types).fold(leaf, |ty, _| {
            TypeData::TypeofType(InternedTypeofType::new(db, ty))
        })
    }

    fn generic<'db>(db: &'db TestDb) -> TypeData<'db> {
        TypeData::Generic(InternedGenericTypeParameter::new(db, None, None, text("T")))
    }

    #[test]
    fn substitution_reports_direct_step_boundaries() {
        let db = TestDb::default();
        let generic = generic(&db);
        let substitution = TypeSubstitution {
            generic,
            replacement: TypeData::String,
        };

        for distinct_types in [1023, 1024, 1025] {
            let result =
                typeof_chain(&db, distinct_types, generic).substitute_type(&db, substitution);
            if distinct_types <= MAX_TYPE_SUBSTITUTION_STEPS {
                assert!(result.is_ok(), "distinct types {distinct_types}");
            } else {
                assert_eq!(
                    result,
                    Err(StructuralMapError::StepLimitExceeded),
                    "distinct types {distinct_types}"
                );
            }
        }
    }

    #[test]
    fn root_body_substitution_reports_direct_step_boundaries() {
        let db = TestDb::default();
        let generic = generic(&db);
        let substitution = TypeSubstitution {
            generic,
            replacement: TypeData::String,
        };

        for distinct_types in [1023, 1024, 1025] {
            let function = TypeData::Function(InternedFunction::new(
                &db,
                boxed([generic]),
                Box::default(),
                ReturnType::Type(typeof_chain(&db, distinct_types, generic)),
                false,
                None,
            ));
            let result = function.substitute_type_in_root_body(&db, substitution);
            if distinct_types <= MAX_TYPE_SUBSTITUTION_STEPS {
                assert!(result.is_ok(), "distinct types {distinct_types}");
            } else {
                assert_eq!(
                    result,
                    Err(StructuralMapError::StepLimitExceeded),
                    "distinct types {distinct_types}"
                );
            }
        }
    }

    #[test]
    fn root_body_substitution_shares_one_budget_across_children() {
        let db = TestDb::default();
        let generic = generic(&db);
        let substitution = TypeSubstitution {
            generic,
            replacement: TypeData::String,
        };
        let parameter = |name, ty| {
            FunctionParameter::Named(NamedFunctionParameter {
                name: text(name),
                ty,
                is_optional: false,
                is_rest: false,
            })
        };
        let function = TypeData::Function(InternedFunction::new(
            &db,
            boxed([generic]),
            Vec::from([
                parameter("first", typeof_chain(&db, 600, generic)),
                parameter("second", typeof_chain(&db, 600, generic)),
            ])
            .into_boxed_slice(),
            ReturnType::Type(generic),
            false,
            None,
        ));

        assert_eq!(
            function.substitute_type_in_root_body(&db, substitution),
            Err(StructuralMapError::StepLimitExceeded)
        );
    }

    #[test]
    fn root_body_substitution_preserves_the_root_binder() {
        let db = TestDb::default();
        let generic = generic(&db);
        let function = TypeData::Function(InternedFunction::new(
            &db,
            boxed([generic]),
            boxed([FunctionParameter::Named(NamedFunctionParameter {
                name: text("value"),
                ty: generic,
                is_optional: false,
                is_rest: false,
            })]),
            ReturnType::Type(generic),
            false,
            None,
        ));
        let substituted = function
            .substitute_type_in_root_body(
                &db,
                TypeSubstitution {
                    generic,
                    replacement: TypeData::String,
                },
            )
            .expect("root body substitution must complete");
        let TypeData::Function(substituted) = substituted else {
            panic!("expected a function");
        };

        assert_eq!(substituted.type_parameters(&db).as_ref(), &[generic]);
        assert_eq!(substituted.parameters(&db)[0].ty(), TypeData::String);
        assert_eq!(
            substituted.return_type(&db),
            &ReturnType::Type(TypeData::String)
        );
    }

    #[test]
    fn substitution_completes_back_edge_after_budget_is_consumed() {
        let db = TestDb::default();
        // IDs 0..1022 build the path. The next interned value receives ID 1023,
        // which closes the final child back to the root.
        let root_reference = InternedTypeofType::from_id(unsafe { salsa::Id::from_index(1023) });
        let child = (0..MAX_TYPE_SUBSTITUTION_STEPS - 1)
            .fold(TypeData::TypeofType(root_reference), |ty, _| {
                TypeData::TypeofType(InternedTypeofType::new(&db, ty))
            });
        let root = TypeData::TypeofType(InternedTypeofType::new(&db, child));

        assert_eq!(
            root.substitute_type(
                &db,
                TypeSubstitution {
                    generic: TypeData::Number,
                    replacement: TypeData::String,
                }
            ),
            Ok(root)
        );
    }

    #[test]
    fn generic_replacement_collection_completes_seen_back_edges_at_the_limit() {
        let db = TestDb::default();
        let generic = TypeData::Generic(InternedGenericTypeParameter::new(
            &db,
            None,
            None,
            text("T"),
        ));
        let pattern = (0..MAX_GENERIC_REPLACEMENT_STEPS - 3).fold(generic, |ty, _| {
            TypeData::instance_of(&db, TypeData::ObjectKeyword, boxed([ty]))
        });
        let actual = (0..MAX_GENERIC_REPLACEMENT_STEPS - 3).fold(TypeData::String, |ty, _| {
            TypeData::instance_of(&db, TypeData::ObjectKeyword, boxed([ty]))
        });
        let pattern =
            TypeData::instance_of(&db, TypeData::ObjectKeyword, boxed([pattern, pattern]));
        let actual = TypeData::instance_of(&db, TypeData::ObjectKeyword, boxed([actual, actual]));

        let replacements = pattern
            .collect_generic_replacements(&db, actual)
            .expect("an already-seen back edge must complete after the budget is consumed");

        assert_eq!(replacements.len(), 1);
        assert_eq!(replacements[0].replacement, TypeData::String);
    }

    #[test]
    fn structural_map_identity_round_trips_all_child_bearing_shapes() {
        let db = TestDb::default();

        assert_identity(&db, |s| {
            TypeData::Class(InternedClass::new(
                &db,
                boxed([s.next()]),
                Some(s.next()),
                boxed([s.next()]),
                boxed([named_member(s)]),
                Some(text("Class")),
                false,
            ))
        });
        assert_identity(&db, |s| {
            TypeData::Constructor(InternedConstructor::new(
                &db,
                boxed([s.next()]),
                boxed([ConstructorParameter {
                    parameter: named_parameter(s),
                    accessibility: None,
                }]),
                Some(s.next()),
            ))
        });
        assert_identity(&db, |s| {
            TypeData::Function(InternedFunction::new(
                &db,
                boxed([s.next()]),
                boxed([named_parameter(s), pattern_parameter(s)]),
                ReturnType::Type(s.next()),
                false,
                Some(text("function")),
            ))
        });
        assert_identity(&db, |s| {
            TypeData::Function(InternedFunction::new(
                &db,
                Box::default(),
                Box::default(),
                ReturnType::Predicate(PredicateReturnType {
                    parameter_name: text("value"),
                    ty: s.next(),
                }),
                false,
                None,
            ))
        });
        assert_identity(&db, |s| {
            TypeData::Function(InternedFunction::new(
                &db,
                Box::default(),
                Box::default(),
                ReturnType::Asserts(AssertsReturnType {
                    parameter_name: text("value"),
                    ty: s.next(),
                }),
                false,
                None,
            ))
        });
        assert_identity(&db, |s| {
            TypeData::Interface(InternedInterface::new(
                &db,
                boxed([s.next()]),
                boxed([s.next()]),
                boxed([named_member(s)]),
                text("Interface"),
            ))
        });
        assert_identity(&db, |s| {
            TypeData::Module(InternedModule::new(
                &db,
                child_bearing_members(s),
                text("module"),
            ))
        });
        assert_identity(&db, |s| {
            TypeData::Namespace(InternedNamespace::new(
                &db,
                boxed([named_member(s)]),
                raw::Path::from(text("namespace")),
            ))
        });
        assert_identity(&db, |s| {
            TypeData::Object(InternedObject::new(
                &db,
                Some(s.next()),
                boxed([named_member(s)]),
            ))
        });
        assert_identity(&db, |s| {
            TypeData::Tuple(InternedTuple::new(
                &db,
                boxed([
                    TupleElementType {
                        ty: s.next(),
                        name: Some(text("first")),
                        is_optional: false,
                        is_rest: false,
                    },
                    TupleElementType {
                        ty: s.next(),
                        name: None,
                        is_optional: true,
                        is_rest: true,
                    },
                ]),
            ))
        });
        assert_identity(&db, |s| {
            TypeData::Generic(InternedGenericTypeParameter::new(
                &db,
                Some(s.next()),
                Some(s.next()),
                text("T"),
            ))
        });
        assert_identity(&db, |s| {
            TypeData::Intersection(InternedIntersection::new(&db, boxed([s.next(), s.next()])))
        });
        assert_identity(&db, |s| {
            TypeData::Union(InternedUnion::new(&db, boxed([s.next(), s.next()])))
        });
        assert_identity(&db, |s| {
            TypeData::TypeOperator(InternedTypeOperatorType::new(
                &db,
                s.next(),
                raw::TypeOperator::Readonly,
            ))
        });
        assert_identity(&db, |s| {
            TypeData::Literal(InternedLiteral::new(
                &db,
                Literal::Object([named_member(s)].into()),
            ))
        });
        assert_identity(&db, |s| {
            TypeData::InstanceOf(InternedTypeInstance::new(
                &db,
                s.next(),
                boxed([s.next(), s.next()]),
            ))
        });
        assert_identity(&db, |s| {
            TypeData::MergedReference(InternedMergedReference::new(
                &db,
                Some(s.next()),
                Some(s.next()),
                Some(s.next()),
            ))
        });

        assert_identity(&db, |s| {
            typeof_type(
                &db,
                TypeofExpression::Addition(TypeofAdditionExpression {
                    left: s.next(),
                    right: s.next(),
                }),
            )
        });
        assert_identity(&db, |s| {
            typeof_type(
                &db,
                TypeofExpression::Await(TypeofAwaitExpression { argument: s.next() }),
            )
        });
        assert_identity(&db, |s| {
            typeof_type(
                &db,
                TypeofExpression::BitwiseNot(TypeofBitwiseNotExpression { argument: s.next() }),
            )
        });
        assert_identity(&db, |s| {
            typeof_type(
                &db,
                TypeofExpression::Call(TypeofCallExpression {
                    callee: s.next(),
                    arguments: [
                        CallArgumentType::Argument(s.next()),
                        CallArgumentType::Spread(s.next()),
                    ]
                    .into(),
                }),
            )
        });
        assert_identity(&db, |s| {
            typeof_type(
                &db,
                TypeofExpression::Conditional(TypeofConditionalExpression {
                    test: s.next(),
                    consequent: s.next(),
                    alternate: s.next(),
                }),
            )
        });
        assert_identity(&db, |s| {
            typeof_type(
                &db,
                TypeofExpression::Destructure(TypeofDestructureExpression {
                    ty: s.next(),
                    destructure_field: raw::DestructureField::Index(0),
                }),
            )
        });
        assert_identity(&db, |s| {
            typeof_type(
                &db,
                TypeofExpression::Index(TypeofIndexExpression {
                    object: s.next(),
                    index: 1,
                }),
            )
        });
        assert_identity(&db, |s| {
            typeof_type(
                &db,
                TypeofExpression::IterableValueOf(TypeofIterableValueOfExpression { ty: s.next() }),
            )
        });
        assert_identity(&db, |s| {
            typeof_type(
                &db,
                TypeofExpression::LogicalAnd(TypeofLogicalAndExpression {
                    left: s.next(),
                    right: s.next(),
                }),
            )
        });
        assert_identity(&db, |s| {
            typeof_type(
                &db,
                TypeofExpression::LogicalOr(TypeofLogicalOrExpression {
                    left: s.next(),
                    right: s.next(),
                }),
            )
        });
        assert_identity(&db, |s| {
            typeof_type(
                &db,
                TypeofExpression::New(TypeofNewExpression {
                    callee: s.next(),
                    arguments: [
                        CallArgumentType::Argument(s.next()),
                        CallArgumentType::Spread(s.next()),
                    ]
                    .into(),
                }),
            )
        });
        assert_identity(&db, |s| {
            typeof_type(
                &db,
                TypeofExpression::NullishCoalescing(TypeofNullishCoalescingExpression {
                    left: s.next(),
                    right: s.next(),
                }),
            )
        });
        assert_identity(&db, |s| {
            typeof_type(
                &db,
                TypeofExpression::StaticMember(TypeofStaticMemberExpression {
                    object: s.next(),
                    member: text("member"),
                }),
            )
        });
        assert_identity(&db, |s| {
            typeof_type(
                &db,
                TypeofExpression::Super(TypeofThisOrSuperExpression { parent: s.next() }),
            )
        });
        assert_identity(&db, |s| {
            typeof_type(
                &db,
                TypeofExpression::This(TypeofThisOrSuperExpression { parent: s.next() }),
            )
        });
        assert_identity(&db, |s| {
            typeof_type(
                &db,
                TypeofExpression::Typeof(TypeofTypeofExpression { argument: s.next() }),
            )
        });
        assert_identity(&db, |s| {
            typeof_type(
                &db,
                TypeofExpression::UnaryMinus(TypeofUnaryMinusExpression { argument: s.next() }),
            )
        });

        assert_identity(&db, |s| {
            TypeData::TypeofType(InternedTypeofType::new(&db, s.next()))
        });
        assert_identity(&db, |s| {
            TypeData::TypeofValue(InternedTypeofValue::new(&db, s.next(), text("value"), None))
        });
    }
}
