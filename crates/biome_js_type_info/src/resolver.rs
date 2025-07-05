use std::{borrow::Cow, fmt::Debug};

use biome_js_syntax::AnyJsExpression;
use biome_js_type_info_macros::Resolvable;
use biome_rowan::Text;

use crate::{
    GLOBAL_UNKNOWN_ID, NUM_PREDEFINED_TYPES, ScopeId, TypeData, TypeId, TypeImportQualifier,
    TypeInstance, TypeMember, TypeMemberKind, TypeReference, TypeReferenceQualifier, TypeofValue,
    Union,
    globals::{GLOBAL_UNDEFINED_ID, global_type_name},
};

const NUM_MODULE_ID_BITS: i32 = 30;
const MODULE_ID_MASK: u32 = 0x3fff_ffff; // Lower 30 bits.
const LEVEL_MASK: u32 = 0xc000_0000; // Upper 2 bits.

/// Type ID combined with the level at which the type was resolved.
///
/// `ResolvedTypeId` uses `u32` for its first field so that it can fit the
/// module ID and the resolver level together in 4 bytes, making the struct as
/// a whole still fit in 8 bytes without alignment issues.
///
/// **FIXME:** The second field, that is normally used for storing a `TypeId`,
///            is used instead to store a `BindingId` or a `ModuleId` if the
///            `ResolverId` is of level `TypeResolverLevel::Import`. See
///            [`TypeResolverLevel`] for details.
///            It would be cleaner and safer to avoid this by using an enum for
///            `ResolvedTypeId` instead, but I don't see a way to limit the size
///            of such an enum to 8 bytes, given how we use the [`ResolverId`].
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct ResolvedTypeId(ResolverId, TypeId);

impl Debug for ResolvedTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.level() == TypeResolverLevel::Global {
            if self.1.index() < NUM_PREDEFINED_TYPES {
                f.write_str(global_type_name(self.1))
            } else {
                let id = self.1.index() - NUM_PREDEFINED_TYPES;
                f.write_fmt(format_args!("Global TypeId({id})"))
            }
        } else {
            f.write_fmt(format_args!("{:?} {:?}", self.0, self.1))
        }
    }
}

impl ResolvedTypeId {
    #[inline]
    pub const fn new(level: TypeResolverLevel, id: TypeId) -> Self {
        Self(ResolverId::from_level(level), id)
    }

    /// Applies the module ID of `self` to `id`.
    #[inline]
    pub const fn apply_module_id(self, id: Self) -> Self {
        self.0.apply_module_id(id)
    }

    /// Applies the module ID of `self` to `reference`.
    #[inline]
    pub fn apply_module_id_to_reference(self, reference: &TypeReference) -> Cow<TypeReference> {
        self.0.apply_module_id_to_reference(reference)
    }

    #[inline]
    pub const fn id(self) -> TypeId {
        self.1
    }

    #[inline]
    pub const fn index(self) -> usize {
        self.1.index()
    }

    #[inline]
    pub const fn is_global(self) -> bool {
        matches!(self.level(), TypeResolverLevel::Global)
    }

    #[inline]
    pub const fn is_at_module_level(self) -> bool {
        matches!(self.level(), TypeResolverLevel::Thin)
    }

    #[inline]
    pub const fn level(self) -> TypeResolverLevel {
        self.0.level()
    }

    #[inline]
    pub const fn module_id(self) -> ModuleId {
        self.0.module_id()
    }

    #[inline]
    pub const fn resolver_id(self) -> ResolverId {
        self.0
    }

    #[inline]
    pub const fn with_module_id(self, module_id: ModuleId) -> Self {
        Self(self.0.with_module_id(module_id), self.1)
    }
}

/// Combines a [`TypeResolverLevel`] and [`ModuleId`] into a single field to
/// uniquely identify where a resolver has resolved a specific type.
///
/// Combined with a [`TypeId`] this makes a [`ResolvedTypeId`].
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct ResolverId(u32);

impl Debug for ResolverId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.level() {
            TypeResolverLevel::Full => f.write_fmt(format_args!("Full")),
            TypeResolverLevel::Thin => {
                f.write_fmt(format_args!("Module({:?})", self.module_id().index()))
            }
            TypeResolverLevel::Import => f.write_fmt(format_args!("Import")),
            TypeResolverLevel::Global => f.write_fmt(format_args!("Global")),
        }
    }
}

impl ResolverId {
    #[inline]
    pub const fn from_level(level: TypeResolverLevel) -> Self {
        Self((level as u32) << NUM_MODULE_ID_BITS)
    }

    #[inline]
    pub const fn from_resolved(id: ResolvedTypeId) -> Self {
        id.0
    }

    /// The scope level is closest to whichever resolver uses the
    /// [`ResolverId`], and it won't attempt to apply any module ID, so
    /// it's a safe default in many contexts.
    #[inline]
    pub const fn scope() -> Self {
        Self::from_level(TypeResolverLevel::Full)
    }

    /// Applies the module ID of `self` to the given `id`.
    #[inline]
    pub const fn apply_module_id(self, id: ResolvedTypeId) -> ResolvedTypeId {
        match (self.level(), id.level()) {
            (TypeResolverLevel::Thin, TypeResolverLevel::Thin) => {
                id.with_module_id(self.module_id())
            }
            _ => id,
        }
    }

    /// Applies the module ID of `self` to the given `data`.
    #[inline]
    pub fn apply_module_id_to_data(self, mut data: TypeData) -> TypeData {
        match self.level() {
            TypeResolverLevel::Thin => {
                let module_id = self.module_id();
                data.update_all_references(|reference| reference.set_module_id(module_id));
                data
            }
            _ => data,
        }
    }

    /// Applies the module ID of `self` to the given `reference`.
    #[inline]
    pub fn apply_module_id_to_reference(self, reference: &TypeReference) -> Cow<TypeReference> {
        match reference {
            TypeReference::Resolved(id) => {
                Cow::Owned(TypeReference::Resolved(self.apply_module_id(*id)))
            }
            other => Cow::Borrowed(other),
        }
    }

    #[inline]
    pub const fn is_global(self) -> bool {
        matches!(self.level(), TypeResolverLevel::Global)
    }

    #[inline]
    pub const fn is_at_module_level(self) -> bool {
        matches!(self.level(), TypeResolverLevel::Thin)
    }

    #[inline]
    pub const fn level(self) -> TypeResolverLevel {
        TypeResolverLevel::from_u2(self.0 >> NUM_MODULE_ID_BITS)
    }

    #[inline]
    pub const fn module_id(self) -> ModuleId {
        ModuleId(self.0 & MODULE_ID_MASK)
    }

    #[inline]
    pub const fn with_module_id(self, module_id: ModuleId) -> Self {
        if self.is_at_module_level() {
            // Clear the bits of the old module ID, while preserving the resolver
            // level, and OR with the bits from the new module ID.
            Self((self.0 & LEVEL_MASK) | module_id.0)
        } else {
            self
        }
    }
}

/// Indicates the level within which a symbol has been or can be resolved.
///
/// The level is used by type resolvers to determine _where_ to look up a given
/// [`TypeId`]. They can look up types within their own registered types, within
/// modules they may have access to and/or decide to forward resolution to
/// another resolver that may be able to handle the level.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeResolverLevel {
    /// Used for full inference where resolution across modules takes place.
    Full,

    /// Used for resolving types that exist within the same module as from which
    /// the resolution took place.
    ///
    /// A [`ResolvedTypeId`] that uses this level may have a [`ModuleId`] stored
    /// as well. However, we **don't** store such module IDs as part of a
    /// module's type information, because a module is unaware of its own ID.
    /// Instead, we rely on the module resolver to attach the module ID at
    /// resolution time.
    Thin,

    /// Used for two disjoint purposes, though both are related to the handling
    /// of imports:
    ///
    /// * The module info collector uses this level for marking types that exist
    ///   across modules that are beyond the capability of the current resolver
    ///   to resolve. Any resolved IDs with this level are **NOT** allowed to
    ///   leave the resolver. Instead, any references at this level are
    ///   converted to [`TypeReference::Import`] before storing them in the
    ///   module info.
    /// * The module resolver uses this level for creating [`ResolvedTypeId`]s
    ///   that resolve to an ad-hoc namespace for a given module that is created
    ///   using the `import * as namespace` syntax.
    ///
    /// **Important:** [`ResolvedTypeId`]s of this level do not store a `TypeId`
    ///                where one is normally expected. Instead, the module info
    ///                collector stores a `BindingId` in its place, while the
    ///                module resolver stores a `ModuleId` there.
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
            0 => Self::Full,
            1 => Self::Thin,
            2 => Self::Import,
            3 => Self::Global,
            _ => panic!("invalid bits passed to TypeResolverLevel"),
        }
    }
}

/// Identifier that indicates which module a type is defined in.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Resolvable)]
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

/// [`TypeData`] reference combined with a [`ResolverId`] to preserve the
/// context in which the data was resolved.
///
/// Whenever type data is returned by a resolver, it is common to want to
/// resolve further references from that type data. In order to resolve such
/// references accurately, the resolver needs to know the context in which the
/// type data itself was found. The embedded [`ResolverId`] contains this
/// information.
///
/// The [`TypeData`] reference can be converted into an owned [`TypeData`]
/// instance using [`Self::to_data()`]. This will update all references to
/// explicitly refer to the correct module ID.
#[derive(Clone, Copy, Debug)]
pub struct ResolvedTypeData<'a> {
    id: ResolverId,
    data: &'a TypeData,
}

impl<'a> From<(ResolvedTypeId, &'a TypeData)> for ResolvedTypeData<'a> {
    #[inline]
    fn from((id, data): (ResolvedTypeId, &'a TypeData)) -> Self {
        (ResolverId::from_resolved(id), data).into()
    }
}

impl<'a> From<(ResolverId, &'a TypeData)> for ResolvedTypeData<'a> {
    #[inline]
    fn from((id, data): (ResolverId, &'a TypeData)) -> Self {
        Self { id, data }
    }
}

impl<'a> ResolvedTypeData<'a> {
    /// Applies the module ID from the embedded [`ResolverId`] to the given
    /// `id`.
    #[inline]
    pub fn apply_module_id(self, id: ResolvedTypeId) -> ResolvedTypeId {
        self.id.apply_module_id(id)
    }

    /// Applies the module ID from the embedded [`ResolverId`] to the given
    /// `data`.
    #[inline]
    pub fn apply_module_id_to_data(self, data: TypeData) -> TypeData {
        self.id.apply_module_id_to_data(data)
    }

    /// Applies the module ID from the embedded [`ResolverId`] to the given
    /// `reference`.
    #[inline]
    pub fn apply_module_id_to_reference(self, reference: &TypeReference) -> Cow<TypeReference> {
        self.id.apply_module_id_to_reference(reference)
    }

    /// Returns a reference to the raw data.
    ///
    /// **Be careful:** If you intend to invoke the resolver on the data, it may
    /// not be aware of the context in which the data was resolved, and further
    /// references may be resolved from the wrong context. If you wish to call
    /// the resolver on the data, use [`Self::to_data()`] instead.
    pub fn as_raw_data(self) -> &'a TypeData {
        self.data
    }

    #[inline]
    pub fn resolver_id(self) -> ResolverId {
        self.id
    }

    #[inline]
    pub fn should_flatten_instance(self, instance: &TypeInstance) -> bool {
        self.as_raw_data().should_flatten_instance(instance)
    }

    /// Converts the resolved data to owned [`TypeData`] with the module ID from
    /// the [`ResolverId`] applied to all its references.
    pub fn to_data(self) -> TypeData {
        match self.id.level() {
            TypeResolverLevel::Thin => {
                let mut data = self.data.clone();
                let module_id = self.id.module_id();
                data.update_all_references(|reference| reference.set_module_id(module_id));
                data
            }
            _ => self.data.clone(),
        }
    }
}

/// [`TypeMember`] reference combined with a [`ResolverId`] to preserve the
/// context in which the member was resolved.
#[derive(Clone, Debug)]
pub struct ResolvedTypeMember<'a> {
    id: ResolverId,
    member: Cow<'a, TypeMember>,
}

impl<'a> From<(ResolverId, &'a TypeMember)> for ResolvedTypeMember<'a> {
    #[inline]
    fn from((id, member): (ResolverId, &'a TypeMember)) -> Self {
        Self {
            id,
            member: Cow::Borrowed(member),
        }
    }
}

impl From<(ResolverId, TypeMember)> for ResolvedTypeMember<'_> {
    #[inline]
    fn from((id, member): (ResolverId, TypeMember)) -> Self {
        Self {
            id,
            member: Cow::Owned(member),
        }
    }
}

impl<'a> ResolvedTypeMember<'a> {
    /// Applies the module ID from the embedded [`ResolverId`] to the given
    /// `data`.
    #[inline]
    pub fn apply_module_id_to_data(&self, data: TypeData) -> TypeData {
        self.id.apply_module_id_to_data(data)
    }

    /// Applies the module ID from the embedded [`ResolverId`] to the given
    /// `reference`.
    #[inline]
    pub fn apply_module_id_to_reference<'r>(
        &self,
        reference: &'r TypeReference,
    ) -> Cow<'r, TypeReference> {
        self.id.apply_module_id_to_reference(reference)
    }

    /// Returns a reference to the raw type member.
    ///
    /// **Be careful:** If you intend to invoke the resolver on the member's
    /// data, it may not be aware of the context in which the member was
    /// resolved, and further references may be resolved from the wrong context.
    /// If you wish to call the resolver on the member's data, use
    /// [`Self::to_member()`] instead.
    pub fn as_raw_member(&'a self) -> &'a TypeMember {
        self.member.as_ref()
    }

    /// Returns a reference to the type of the member if we dereference it.
    ///
    /// This means if the member represents a getter or setter, it will
    /// dereference to the type of the property being get or set.
    pub fn deref_ty(&self, resolver: &dyn TypeResolver) -> Cow<TypeReference> {
        if self.is_getter() {
            resolver
                .resolve_and_get(&self.ty())
                .and_then(|resolved| match resolved.as_raw_data() {
                    TypeData::Function(function) => {
                        function.return_type.as_type().map(|return_ty| {
                            resolved
                                .apply_module_id_to_reference(return_ty)
                                .into_owned()
                        })
                    }
                    _ => None,
                })
                .map_or(Cow::Owned(GLOBAL_UNKNOWN_ID.into()), Cow::Owned)
        } else {
            self.ty()
        }
    }

    #[inline]
    pub fn has_name(&self, name: &str) -> bool {
        self.member.has_name(name)
    }

    #[inline]
    pub fn is_getter(&self) -> bool {
        self.member.is_getter()
    }

    pub fn is_index_signature_with_ty(&self, predicate: impl Fn(&TypeReference) -> bool) -> bool {
        self.member.is_index_signature_with_ty(|reference| {
            predicate(&self.apply_module_id_to_reference(reference))
        })
    }

    #[inline]
    pub fn is_static(&self) -> bool {
        self.member.is_static()
    }

    #[inline]
    pub fn kind(&self) -> &TypeMemberKind {
        &self.member.kind
    }

    #[inline]
    pub fn name(&self) -> Option<Text> {
        self.member.name()
    }

    /// Converts the resolved type member to an owned [`TypeMember`] with the
    /// module ID from the [`ResolverId`] applied to all its references.
    pub fn to_member(self) -> TypeMember {
        match self.id.level() {
            TypeResolverLevel::Thin => {
                let mut member = self.member.into_owned();
                let module_id = self.id.module_id();
                member.update_all_references(|reference| reference.set_module_id(module_id));
                member
            }
            _ => self.member.into_owned(),
        }
    }

    /// Returns a reference to the type of the member.
    pub fn ty(&self) -> Cow<TypeReference> {
        self.apply_module_id_to_reference(&self.member.ty)
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
    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<ResolvedTypeData>;

    /// Returns the [`TypeReference`] to refer to a [`TypeId`] belonging to this
    /// resolver.
    fn reference_to_id(&self, id: TypeId) -> TypeReference {
        TypeReference::Resolved(ResolvedTypeId::new(self.level(), id))
    }

    /// Returns a reference to the given type data, if possible.
    fn reference_to_data(&self, type_data: &TypeData) -> Option<TypeReference> {
        match type_data {
            TypeData::Reference(reference) => Some(reference.clone()),
            other => self.find_type(other).map(|id| self.reference_to_id(id)),
        }
    }

    /// Returns a reference to the given type data, registering the data if
    /// necessary.
    fn reference_to_registered_data(&mut self, type_data: &TypeData) -> TypeReference {
        match type_data {
            TypeData::Reference(reference) => reference.clone(),
            _ => {
                let id = self.register_type(Cow::Borrowed(type_data));
                self.reference_to_id(id)
            }
        }
    }

    /// Returns a reference to the given owned type data, registering the data
    /// in the process.
    fn reference_to_owned_data(&mut self, type_data: TypeData) -> TypeReference {
        match type_data {
            TypeData::Reference(reference) => reference,
            _ => {
                let id = self.register_type(Cow::Owned(type_data));
                self.reference_to_id(id)
            }
        }
    }

    /// Returns a reference to the given `expression` in the given scope with
    /// the given ID.
    fn reference_to_resolved_expression(
        &mut self,
        scope_id: ScopeId,
        expression: &AnyJsExpression,
    ) -> TypeReference {
        let data = self.resolve_expression(scope_id, expression);
        match data {
            Cow::Owned(TypeData::Reference(reference)) => reference,
            Cow::Borrowed(TypeData::Reference(reference)) => reference.clone(),
            data => {
                let data = Cow::Owned(data.into_owned());
                let id = self.register_type(data);
                self.reference_to_id(id)
            }
        }
    }

    /// Registers a type within the level handled by this resolver.
    ///
    /// If the given `type_data` is already registered, this may return an
    /// existing [`TypeId`].
    fn register_type(&mut self, type_data: Cow<TypeData>) -> TypeId;

    /// Registers a type within the level handled by this resolver, and
    /// immediately returns the [`TypeData`].
    fn register_and_get(&mut self, type_data: TypeData) -> &TypeData {
        let type_id = self.register_type(Cow::Owned(type_data));
        self.get_by_id(type_id)
    }

    /// Registers a type within the level handled by this resolver, and returns
    /// a [`ResolvedTypeId`].
    fn register_and_resolve(&mut self, type_data: TypeData) -> ResolvedTypeId {
        match type_data {
            TypeData::Reference(TypeReference::Resolved(resolved)) => resolved,
            type_data => {
                let type_id = self.register_type(Cow::Owned(type_data));
                ResolvedTypeId::new(self.level(), type_id)
            }
        }
    }

    /// Resolves a type reference and immediately returns the associated
    /// [`TypeData`] if found.
    fn resolve_and_get(&self, ty: &TypeReference) -> Option<ResolvedTypeData> {
        match self
            .resolve_reference(ty)
            .and_then(|id| self.get_by_resolved_id(id))
        {
            Some(ResolvedTypeData {
                data: TypeData::Reference(reference),
                id,
            }) if reference != ty => {
                self.resolve_and_get(&id.apply_module_id_to_reference(reference))
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
            None => self.register_and_resolve(TypeData::reference(ty.clone())),
        }
    }

    /// Resolves the given import qualifier.
    fn resolve_import(&self, _qualifier: &TypeImportQualifier) -> Option<ResolvedTypeId> {
        None
    }

    /// Resolves a named symbol in a given module.
    fn resolve_import_namespace_member(
        &self,
        _module_id: ModuleId,
        _name: &str,
    ) -> Option<ResolvedTypeId> {
        None
    }

    /// Resolves the given `expression` in the given `scope_id` to a type.
    ///
    /// Depending on the resolver, this may return owned type data based on
    /// local inference, or a reference to previously resolved type data.
    fn resolve_expression(
        &mut self,
        scope_id: ScopeId,
        expression: &AnyJsExpression,
    ) -> Cow<TypeData>;

    /// Resolves a type reference.
    fn resolve_reference(&self, ty: &TypeReference) -> Option<ResolvedTypeId>;

    /// Resolves a type by its reference `qualifier`.
    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<ResolvedTypeId>;

    /// Resolves the type of a value by its `identifier` in a specific scope.
    fn resolve_type_of(&self, identifier: &Text, scope_id: ScopeId) -> Option<ResolvedTypeId>;

    /// Maps from one resolved ID to another.
    ///
    /// Some resolvers may wish to map resolved IDs that reference other
    /// resolvers to their own resolved types. They can reimplement this method
    /// to do so.
    fn mapped_resolved_id(&self, resolved_id: ResolvedTypeId) -> ResolvedTypeId {
        resolved_id
    }

    // #region Utilities for test inspection

    /// Returns the resolver's fallback, if it has one.
    fn fallback_resolver(&self) -> Option<&dyn TypeResolver> {
        None
    }

    /// Returns all types registered in this resolver.
    fn registered_types(&self) -> Vec<&TypeData>;

    // #endregion

    // #region Registration utilities

    fn optional(&mut self, ty: TypeReference) -> TypeId {
        self.register_type(Cow::Owned(TypeData::Union(Box::new(Union(Box::new([
            ty,
            GLOBAL_UNDEFINED_ID.into(),
        ]))))))
    }

    // #endregion
}

/// Trait to be implemented by `TypeData` and its subtypes to aid the resolver.
pub trait Resolvable: Sized {
    /// Returns the resolved version of this type.
    fn resolved(&self, resolver: &mut dyn TypeResolver) -> Option<Self>;

    /// Updates all references using the given callback.
    fn update_all_references(&mut self, updater: impl Copy + Fn(&mut TypeReference));
}

impl Resolvable for TypeReference {
    fn resolved(&self, resolver: &mut dyn TypeResolver) -> Option<Self> {
        match self {
            Self::Qualifier(qualifier) => {
                let resolved_id = resolver.resolve_qualifier(qualifier);
                match resolved_id {
                    Some(resolved_id) => Some(Self::Resolved(resolved_id)),
                    None if qualifier.has_known_type_parameters() => Some({
                        // If we can't resolve the qualifier as is, attempt to
                        // resolve it without type parameters. If it can be
                        // resolved that way, we create an instantiation for it
                        // and resolve to there.
                        resolver
                            .resolve_qualifier(&qualifier.without_type_parameters())
                            .and_then(|resolved_id| {
                                let resolved = resolver
                                    .get_by_resolved_id(resolved_id)
                                    .map(|data| data.to_data());
                                let parameters =
                                    resolved.as_ref().and_then(|data| data.type_parameters())?;
                                let resolved_id: ResolvedTypeId = resolver.register_and_resolve(
                                    TypeData::instance_of(TypeInstance {
                                        ty: resolved_id.into(),
                                        type_parameters: Self::merge_parameters(
                                            parameters,
                                            &qualifier.type_parameters,
                                        ),
                                    }),
                                );
                                Some(resolved_id.into())
                            })
                            .unwrap_or_else(|| {
                                Self::from(TypeReferenceQualifier {
                                    path: qualifier.path.clone(),
                                    type_parameters: self.resolved_params(resolver),
                                    scope_id: qualifier.scope_id,
                                    type_only: qualifier.type_only,
                                    excluded_binding_id: qualifier.excluded_binding_id,
                                })
                            })
                    }),
                    None => None,
                }
            }
            Self::Import(import) => resolver.resolve_import(import).map(Self::Resolved),
            _ => None,
        }
    }

    fn update_all_references(&mut self, updater: impl Copy + Fn(&mut Self)) {
        updater(self)
    }
}

impl Resolvable for TypeofValue {
    fn resolved(&self, resolver: &mut dyn TypeResolver) -> Option<Self> {
        let ty = if self.ty == TypeReference::Unknown {
            let resolved_id = resolver
                .resolve_type_of(&self.identifier, self.scope_id.unwrap_or(ScopeId::GLOBAL))?;
            TypeReference::Resolved(resolved_id)
        } else {
            self.ty.resolved(resolver)?
        };

        Some(Self {
            identifier: self.identifier.clone(),
            ty,
            scope_id: self.scope_id,
        })
    }

    fn update_all_references(&mut self, updater: impl Copy + Fn(&mut TypeReference)) {
        updater(&mut self.ty)
    }
}

macro_rules! derive_primitive_resolved {
    ($($ty:ty),+) => {
        $(impl Resolvable for $ty {
            fn resolved(&self, _resolver: &mut dyn TypeResolver) -> Option<Self> {
                None
            }

            fn update_all_references(&mut self, _updater: impl Copy + Fn(&mut TypeReference)) {}
        })+
    };
}

derive_primitive_resolved!(bool, f64, u32, u64, usize);
