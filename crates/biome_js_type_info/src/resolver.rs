use std::fmt::Debug;

use biome_rowan::Text;

use crate::{
    Class, DestructureField, Function, GenericTypeParameter, TypeData, TypeId, TypeImportQualifier,
    TypeInstance, TypeMember, TypeReference, TypeReferenceQualifier, TypeofDestructureExpression,
    TypeofExpression, TypeofValue, Union, globals::GLOBAL_UNDEFINED_ID,
};

const NUM_MODULE_ID_BITS: i32 = 30;
const MODULE_ID_MASK: u32 = 0x3fff_ffff; // Lower 30 bits.
const LEVEL_MASK: u32 = 0xc000_0000; // Upper 2 bits.

/// Type ID combined with the level at which the type was resolved.
///
/// `ResolvedTypeId` uses `u32` for its first field so that it can fit the
/// module ID and the resolver level together in 4 bytes, making the struct as
/// a whole still fit in 8 bytes without alignment issues.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct ResolvedTypeId(u32, TypeId);

impl Debug for ResolvedTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.1;
        match self.level() {
            TypeResolverLevel::AdHoc => f.write_fmt(format_args!("AdHoc {id:?}")),
            TypeResolverLevel::Module => f.write_fmt(format_args!(
                "Module({:?}) {id:?}",
                self.module_id().index()
            )),
            TypeResolverLevel::Import => f.write_fmt(format_args!("Import {id:?}")),
            TypeResolverLevel::Global => f.write_fmt(format_args!("Global {id:?}")),
        }
    }
}

impl ResolvedTypeId {
    pub const fn new(level: TypeResolverLevel, id: TypeId) -> Self {
        let first = (level as u32) << NUM_MODULE_ID_BITS;
        Self(first, id)
    }

    pub const fn id(self) -> TypeId {
        self.1
    }

    pub const fn index(self) -> usize {
        self.1.index()
    }

    pub const fn is_global(self) -> bool {
        matches!(self.level(), TypeResolverLevel::Global)
    }

    pub const fn is_at_module_level(self) -> bool {
        matches!(self.level(), TypeResolverLevel::Module)
    }

    pub const fn level(self) -> TypeResolverLevel {
        TypeResolverLevel::from_u2(self.0 >> NUM_MODULE_ID_BITS)
    }

    pub const fn module_id(self) -> ModuleId {
        ModuleId(self.0 & MODULE_ID_MASK)
    }

    pub const fn with_module_id(self, module_id: ModuleId) -> Self {
        if self.is_at_module_level() {
            // Clear the bits of the old module ID, while preserving the resolver
            // level, and OR with the bits from the new module ID.
            Self((self.0 & LEVEL_MASK) | module_id.0, self.1)
        } else {
            self
        }
    }
}

/// Identifier that indicates which module a type is defined in.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModuleId(u32);

impl ModuleId {
    pub const fn new(index: usize) -> Self {
        // Top two bits are reserved to fit in resolver level.
        debug_assert!(index < MODULE_ID_MASK as usize);

        Self(index as u32)
    }

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// Indicates the level within which a symbol has been resolved.
///
/// The level is used by type resolvers to determine _where_ to look up a given
/// [`TypeId`]. They can look up types within their own registered types, within
/// modules they may have access to and/or decide to forward resolution to
/// another resolver that may be able to handle the level.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeResolverLevel {
    /// Used for ad-hoc inference that is not cached except in the ad-hoc
    /// resolver itself (which is discarded after use).
    AdHoc,

    /// Used for resolving types that exist within the same module as from which
    /// the resolution took place.
    ///
    /// A [`ResolvedTypeId`] that uses this level may have a [`ModuleId`] stored
    /// as well. However, we **don't** store such module IDs as part of a
    /// module's type information, because a module is unaware of its own ID.
    /// Instead, we rely on the resolver to attach the module ID at resolution
    /// time.
    Module,

    /// Used for resolving types that exist across modules within the project.
    ///
    /// Currently, we don't store resolved IDs with this level in the module
    /// info. Instead, we use it during a module's type collection to flag
    /// resolved types that require imports from other modules. Such resolved
    /// IDs then get converted to [`TypeReference::Import`] before storing
    /// them in the module info.
    ///
    /// **Important:** [`ResolvedTypeId`]s of this level store a `BindingId` in
    ///                the field that is used for `TypeId`s normally.
    Import,

    /// Used for language- and environment-level globals.
    Global,
}

impl TypeResolverLevel {
    /// Creates `TypeResolverLevel` from the two least significant bits of a
    /// `u32`.
    ///
    /// Only the two least significant bits may be set in order to let the type
    /// fit into `ResolvedTypeId`. If more bits become necessary, we may need to
    /// rebalance the layout of `ResolvedTypeId`.
    ///
    /// Note: `from_u2` is not a typo. Even though `u2` isn't a real type, it's
    ///       named like this to make you, dear reader, more aware of the size
    ///       constraint ;)
    pub const fn from_u2(bits: u32) -> Self {
        match bits {
            0 => Self::AdHoc,
            1 => Self::Module,
            2 => Self::Import,
            3 => Self::Global,
            _ => panic!("invalid bits passed to TypeResolverLevel"),
        }
    }
}

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
    /// Returns the resolution level handled by this resolver.
    fn level(&self) -> TypeResolverLevel;

    /// Attempts to find the given type data if it already exists within the
    /// resolver.
    fn find_type(&self, type_data: &TypeData) -> Option<TypeId>;

    /// Returns type data by its ID.
    ///
    /// **Warning:** The given [`TypeId`] must be a type ID that was created by
    ///              this resolver.
    fn get_by_id(&self, id: TypeId) -> &TypeData;

    /// Returns type data by its resolved ID, if possible.
    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<&TypeData>;

    /// Returns the [`TypeReference`] to refer to a [`TypeId`] belonging to this
    /// resolver.
    fn reference_to_id(&self, id: TypeId) -> TypeReference {
        TypeReference::Resolved(ResolvedTypeId::new(self.level(), id))
    }

    /// Returns a reference to the given type data, if possible.
    fn reference_to_data(&self, type_data: &TypeData) -> Option<TypeReference> {
        match type_data {
            TypeData::Reference(reference) => Some(reference.as_ref().clone()),
            other => self.find_type(other).map(|id| self.reference_to_id(id)),
        }
    }

    /// Returns a reference to the given type data, registering the data if
    /// necessary.
    fn reference_to_registered_data(&mut self, type_data: TypeData) -> TypeReference {
        match self.reference_to_data(&type_data) {
            Some(type_data) => type_data,
            None => {
                let id = self.register_type(type_data);
                self.reference_to_id(id)
            }
        }
    }

    /// Registers a type within the level handled by this resolver.
    ///
    /// If the given `type_data` is already registered, this may return an
    /// existing [`TypeId`].
    fn register_type(&mut self, type_data: TypeData) -> TypeId;

    /// Registers a type within the level handled by this resolver, and
    /// immediately returns the [`TypeData`].
    fn register_and_get(&mut self, type_data: TypeData) -> &TypeData {
        let type_id = self.register_type(type_data);
        self.get_by_id(type_id)
    }

    /// Registers a type within the level handled by this resolver, and returns
    /// a [`ResolvedTypeId`].
    fn register_and_resolve(&mut self, type_data: TypeData) -> ResolvedTypeId {
        let type_id = self.register_type(type_data);
        ResolvedTypeId::new(self.level(), type_id)
    }

    /// Resolves a type reference and immediately returns the associated
    /// [`TypeData`] if found.
    fn resolve_and_get(&self, ty: &TypeReference) -> Option<&TypeData> {
        match self
            .resolve_reference(ty)
            .and_then(|id| self.get_by_resolved_id(id))
        {
            Some(TypeData::Reference(type_data)) => self.resolve_and_get(type_data),
            other => other,
        }
    }

    /// Same as [`TypeResolver::resolve_and_get()`], but applies the given
    /// `module_id` during resolution.
    fn resolve_and_get_with_module_id(
        &self,
        ty: &TypeReference,
        module_id: ModuleId,
    ) -> Option<&TypeData> {
        match self
            .resolve_reference(ty)
            .and_then(|id| self.get_by_resolved_id(id.with_module_id(module_id)))
        {
            Some(TypeData::Reference(type_data)) => {
                self.resolve_and_get_with_module_id(type_data, module_id)
            }
            other => other,
        }
    }

    /// Resolves a type reference and returns the [`ResolvedTypeId`] if found.
    ///
    /// If not found, the reference is registered within the level handled by
    /// this resolved and a new [`ResolvedTypeId`] is returned.
    fn resolve_or_register(&mut self, ty: &TypeReference) -> ResolvedTypeId {
        match self.resolve_reference(ty) {
            Some(resolved_id) => resolved_id,
            None => self.register_and_resolve(TypeData::Reference(Box::new(ty.clone()))),
        }
    }

    /// Resolves the given import qualifier, registering the result into this
    /// resolver's type array if necessary.
    fn resolve_import(&mut self, _qualifier: &TypeImportQualifier) -> Option<ResolvedTypeId> {
        None
    }

    /// Resolves a type reference.
    fn resolve_reference(&self, ty: &TypeReference) -> Option<ResolvedTypeId>;

    /// Resolves a type by its reference `qualifier`.
    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<ResolvedTypeId>;

    /// Resolves the type of a value by its `identifier`.
    fn resolve_type_of(&self, identifier: &Text) -> Option<ResolvedTypeId>;

    // #region Utilities for test inspection

    /// Returns the resolver's fallback, if it has one.
    fn fallback_resolver(&self) -> Option<&dyn TypeResolver> {
        None
    }

    /// Returns all types registered in this resolver.
    fn registered_types(&self) -> &[TypeData];

    // #endregion

    // #region Registration utilities

    fn assign_type_parameters(
        &mut self,
        type_data: &TypeData,
        type_parameters: &[TypeReference],
    ) -> Option<TypeId> {
        match type_data {
            TypeData::Class(class) => Some(
                self.register_type(TypeData::Class(Box::new(Class {
                    type_parameters: class
                        .type_parameters
                        .iter()
                        .enumerate()
                        .map(|(i, param)| GenericTypeParameter {
                            name: param.name.clone(),
                            ty: type_parameters
                                .get(i)
                                .cloned()
                                .unwrap_or_else(|| param.ty.clone()),
                        })
                        .collect(),
                    ..class.as_ref().clone()
                }))),
            ),
            // TODO: Which other types do we need to handle here?
            _ => None,
        }
    }

    fn destructuring_of(
        &mut self,
        ty: TypeReference,
        destructure_field: DestructureField,
    ) -> TypeData {
        self.register_and_get(TypeData::TypeofExpression(Box::new(
            TypeofExpression::Destructure(TypeofDestructureExpression {
                ty,
                destructure_field,
            }),
        )))
        .clone()
    }

    fn optional(&mut self, ty: TypeReference) -> TypeId {
        self.union_of(ty, GLOBAL_UNDEFINED_ID.into())
    }

    fn register_type_from_member(
        &mut self,
        object: &TypeData,
        member: &TypeMember,
    ) -> ResolvedTypeId {
        match member {
            TypeMember::CallSignature(member) => {
                self.register_and_resolve(TypeData::Function(Box::new(Function {
                    is_async: false,
                    type_parameters: member.type_parameters.clone(),
                    name: None,
                    parameters: member.parameters.clone(),
                    return_type: member.return_type.clone(),
                })))
            }
            TypeMember::Constructor(member) => match &member.return_type {
                Some(reference) => self.resolve_or_register(reference),
                None => self.register_and_resolve(object.clone()),
            },
            TypeMember::Method(member) => {
                let id = self.register_type(TypeData::Function(Box::new(Function {
                    is_async: member.is_async,
                    type_parameters: member.type_parameters.clone(),
                    name: Some(member.name.clone()),
                    parameters: member.parameters.clone(),
                    return_type: member.return_type.clone(),
                })));
                let id = if member.is_optional {
                    self.optional(self.reference_to_id(id))
                } else {
                    id
                };
                ResolvedTypeId::new(self.level(), id)
            }
            TypeMember::Property(member) => {
                if member.is_optional {
                    ResolvedTypeId::new(self.level(), self.optional(member.ty.clone()))
                } else {
                    self.resolve_or_register(&member.ty)
                }
            }
        }
    }

    fn type_from_member(&mut self, object: &TypeData, member: &TypeMember) -> TypeData {
        let resolved_id = self.register_type_from_member(object, member);
        self.get_by_resolved_id(resolved_id)
            .expect("resolved ID must be registered")
            .clone()
    }

    fn type_reference_from_member(&mut self, object: &TypeData, member: &TypeMember) -> TypeData {
        let resolved_id = self.register_type_from_member(object, member);
        self.get_by_resolved_id(resolved_id)
            .expect("resolved ID must be registered")
            .clone()
    }

    fn undefined(&mut self) -> TypeId {
        self.register_type(TypeData::Undefined)
    }

    fn union_of(&mut self, left: TypeReference, right: TypeReference) -> TypeId {
        self.register_type(TypeData::Union(Box::new(Union(Box::new([left, right])))))
    }

    fn unknown(&mut self) -> TypeId {
        self.register_type(TypeData::Unknown)
    }

    fn void(&mut self) -> TypeId {
        self.register_type(TypeData::VoidKeyword)
    }

    // #endregion
}

/// Trait to be implemented by `TypeData` and its subtypes to aid the resolver.
pub trait Resolvable: Sized {
    /// Returns the resolved version of this type.
    fn resolved(&self, resolver: &mut dyn TypeResolver) -> Self;

    /// Returns the resolved version of this type, and applies a custom mapper
    /// function on all instances of [`TypeReference`].
    fn resolved_with_mapped_references(
        &self,
        map: impl Copy + Fn(TypeReference, &mut dyn TypeResolver) -> TypeReference,
        resolver: &mut dyn TypeResolver,
    ) -> Self;

    /// Returns the resolved version of this type, and applies the given
    /// `module_id` to any returned module-level type references.
    fn resolved_with_module_id(
        &self,
        module_id: ModuleId,
        resolver: &mut dyn TypeResolver,
    ) -> Self {
        self.resolved_with_mapped_references(
            |reference, _| reference.with_module_id(module_id),
            resolver,
        )
    }

    /// Returns the instance with all module-level references augmented with the
    /// given `module_id`.
    ///
    /// Does not perform any resolving in the process.
    fn with_module_id(self, module_id: ModuleId) -> Self;
}

impl Resolvable for TypeReference {
    fn resolved(&self, resolver: &mut dyn TypeResolver) -> Self {
        match self {
            Self::Qualifier(qualifier) => {
                let resolved_id = resolver.resolve_qualifier(qualifier);
                match resolved_id {
                    Some(resolved_id) => Self::Resolved(resolved_id),
                    None => {
                        // If we can't resolve the qualifier as is, attempt to
                        // resolve it without type parameters. If it can be
                        // resolved that way, we create an instantiation for it
                        // and resolve to there.
                        qualifier
                            .has_known_type_parameters()
                            .then(|| {
                                resolver.resolve_qualifier(&qualifier.without_type_parameters())
                            })
                            .flatten()
                            .and_then(|resolved_id| {
                                let parameters = resolver
                                    .get_by_resolved_id(resolved_id)
                                    .and_then(TypeData::type_parameters)?;
                                let resolved_id: ResolvedTypeId = resolver.register_and_resolve(
                                    TypeData::InstanceOf(Box::new(TypeInstance {
                                        ty: resolved_id.into(),
                                        type_parameters: GenericTypeParameter::merge_types(
                                            parameters,
                                            &qualifier.type_parameters,
                                        ),
                                    })),
                                );
                                Some(resolved_id.into())
                            })
                            .unwrap_or_else(|| {
                                Self::Qualifier(TypeReferenceQualifier {
                                    path: qualifier.path.clone(),
                                    type_parameters: self.resolved_params(resolver),
                                })
                            })
                    }
                }
            }
            Self::Import(import) => {
                let resolved_id = resolver.resolve_import(import);
                match resolved_id {
                    Some(resolved_id) => Self::Resolved(resolved_id),
                    None => self.clone(),
                }
            }
            other => other.clone(),
        }
    }

    fn resolved_with_mapped_references(
        &self,
        map: impl Copy + Fn(Self, &mut dyn TypeResolver) -> Self,
        resolver: &mut dyn TypeResolver,
    ) -> Self {
        map(self.resolved(resolver), resolver)
    }

    fn with_module_id(self, module_id: ModuleId) -> Self {
        match self {
            Self::Resolved(resolved_type_id) => {
                Self::Resolved(resolved_type_id.with_module_id(module_id))
            }
            other => other,
        }
    }
}

impl Resolvable for TypeofValue {
    fn resolved(&self, resolver: &mut dyn TypeResolver) -> Self {
        let identifier = self.identifier.clone();
        let ty = if self.ty == TypeReference::Unknown {
            resolver
                .resolve_type_of(&identifier)
                .map_or(TypeReference::Unknown, TypeReference::Resolved)
        } else {
            self.ty.resolved(resolver)
        };

        Self { identifier, ty }
    }

    fn resolved_with_mapped_references(
        &self,
        map: impl Copy + Fn(TypeReference, &mut dyn TypeResolver) -> TypeReference,
        resolver: &mut dyn TypeResolver,
    ) -> Self {
        let Self { identifier, ty } = self.resolved(resolver);
        Self {
            identifier,
            ty: map(ty, resolver),
        }
    }

    fn with_module_id(self, module_id: ModuleId) -> Self {
        let Self { identifier, ty } = self;
        Self {
            identifier,
            ty: ty.with_module_id(module_id),
        }
    }
}

macro_rules! derive_primitive_resolved {
    ($($ty:ty),+) => {
        $(impl Resolvable for $ty {
            fn resolved(&self, _resolver: &mut dyn TypeResolver) -> Self {
                *self
            }

            fn resolved_with_mapped_references(
                &self,
                _map: impl Copy + Fn(TypeReference, &mut dyn TypeResolver) -> TypeReference,
                _resolver: &mut dyn TypeResolver,
            ) -> Self {
                *self
            }

            fn with_module_id(self, _module_id: ModuleId) -> Self {
                self
            }
        })+
    };
}

derive_primitive_resolved!(bool, f64, u32, u64, usize);
