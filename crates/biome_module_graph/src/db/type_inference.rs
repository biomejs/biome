use crate::db::queries::{NormalizeTypeInput, infer_module_types};
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{JsExport, JsImport, JsModuleInfo, JsOwnExport, ModuleDb, ResolvedPath};
use biome_css_syntax::TextRange;
use biome_js_semantic::JsDeclarationKind;
use biome_js_type_info::{
    GLOBAL_RESOLVER, ImportSymbol, Path, ResolvedTypeId, TypeId, TypeImportQualifier,
    TypeReference, TypeResolver, TypeResolverLevel,
    interned_types::{
        InternedNamespace as InferredNamespace, InternedObject as InferredObject,
        Literal as InferredLiteral, LocalTypeHandle, LocalTypeId, ModuleKey,
        TypeData as InferredTypeData, TypeMember as InferredTypeMember,
        TypeMemberKind as InferredTypeMemberKind,
    },
};
use biome_rowan::Text;
use rustc_hash::{FxHashMap, FxHashSet};
use salsa::plumbing::{AsId, FromId};

/// Unlike the other limits, this one guards actual stack recursion: each level
/// of `ResolutionCtx::resolve` clones a raw type and runs the conversion walk,
/// so the frames are heavy. Named declarations already short-circuit to
/// `TypeData::Local` handles, which leaves only structural nesting within a
/// single declaration on the stack — real-world code stays well below this.
const MAX_RAW_TYPE_RESOLUTION_DEPTH: usize = 64;
const MAX_LOCAL_TYPE_RESOLUTION_STEPS: usize = 1024;
const MAX_MEMBER_LOOKUP_STEPS: usize = 1024;
const MAX_SCOPE_RESOLUTION_STEPS: usize = 1024;
const MAX_EXPORT_RESOLUTION_STEPS: usize = 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq, salsa::Update)]
pub struct BindingTypeData<'db> {
    pub ty: InferredTypeData<'db>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InferredModuleTypes<'db> {
    pub module_key: ModuleKey,
    pub named_type_ids: Box<[LocalTypeId]>,
    pub types: Box<[InferredTypeData<'db>]>,
    pub expressions: FxHashMap<TextRange, InferredTypeData<'db>>,
    pub binding_type_data: FxHashMap<TextRange, BindingTypeData<'db>>,
}

// SAFETY: This struct does not borrow from the database. It owns the ranges, and
// the types are small handles created by Salsa. Comparing the old maps with the
// new maps is safe; if they differ, replacing the old maps exposes the same data
// as updating each entry one by one.
unsafe impl salsa::Update for InferredModuleTypes<'_> {
    unsafe fn maybe_update(old_pointer: *mut Self, new_value: Self) -> bool {
        let old_value = unsafe { &mut *old_pointer };
        if *old_value == new_value {
            false
        } else {
            *old_value = new_value;
            true
        }
    }
}

impl<'db> InferredModuleTypes<'db> {
    pub fn resolve_type(
        &self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        self.resolve_type_iterative(db, ty)
    }

    pub fn find_member_type(
        &self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
        name: &str,
    ) -> Option<InferredTypeData<'db>> {
        self.find_member_type_iterative(db, ty, name)
    }

    fn resolve_type_iterative(
        &self,
        db: &'db dyn ModuleDb,
        mut ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        let mut seen = FxHashSet::default();
        let mut remaining_steps = MAX_LOCAL_TYPE_RESOLUTION_STEPS;
        loop {
            let InferredTypeData::Local(local) = ty else {
                return ty;
            };

            if remaining_steps == 0 {
                return ty;
            }
            remaining_steps -= 1;

            let module_key = local.module(db);
            let type_id = local.type_id(db);
            if !seen.insert((module_key, type_id)) {
                return ty;
            }

            ty = self
                .type_for_local_handle(db, local)
                .unwrap_or(InferredTypeData::Unknown);
        }
    }

    fn type_for_local_handle(
        &self,
        db: &'db dyn ModuleDb,
        local: LocalTypeHandle<'db>,
    ) -> Option<InferredTypeData<'db>> {
        let module_key = local.module(db);
        let type_id = local.type_id(db);
        if module_key == self.module_key {
            return self.types.get(type_id.index()).copied();
        }

        let module = module_for_key(db, module_key)?;
        super::queries::infer_module_types(db, module)
            .and_then(|types| types.types.get(type_id.index()).copied())
    }

    fn find_member_type_iterative(
        &self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
        name: &str,
    ) -> Option<InferredTypeData<'db>> {
        let mut seen = FxHashSet::default();
        let mut pending = vec![(ty, MemberLookup::Any, false)];
        let mut found = Vec::new();
        let mut remaining_steps = MAX_MEMBER_LOOKUP_STEPS;

        while let Some((ty, lookup, collect)) = pending.pop() {
            let ty = self.resolve_type_iterative(db, ty);
            let (ty, lookup) = match ty {
                InferredTypeData::InstanceOf(instance) => (
                    self.resolve_type_iterative(db, instance.ty(db)),
                    MemberLookup::Instance,
                ),
                ty => (ty, lookup),
            };

            if !seen.insert((ty, lookup)) {
                continue;
            }

            // Deduplicated entries above don't count against the budget, so
            // the limit measures distinct types visited, not queue churn.
            if remaining_steps == 0 {
                break;
            }
            remaining_steps -= 1;

            if let Some(member_ty) = self.find_own_member_type(db, ty, name, lookup) {
                if collect {
                    found.push(member_ty);
                    continue;
                }
                return Some(member_ty);
            }

            match ty {
                InferredTypeData::Class(class) => {
                    if let Some(mut extends) = class.extends(db) {
                        if matches!(lookup, MemberLookup::Any) {
                            extends = class_side_type(db, extends);
                        }
                        pending.push((extends, lookup, collect));
                    }
                }
                InferredTypeData::Interface(interface) => {
                    pending.extend(
                        interface
                            .extends(db)
                            .iter()
                            .rev()
                            .copied()
                            .map(|ty| (ty, lookup, collect)),
                    );
                }
                InferredTypeData::Generic(generic) => {
                    if let Some(constraint) = generic.constraint(db) {
                        pending.push((constraint, lookup, collect));
                    }
                }
                InferredTypeData::Intersection(intersection) => {
                    pending.extend(
                        intersection
                            .types(db)
                            .iter()
                            .rev()
                            .copied()
                            .map(|ty| (ty, lookup, true)),
                    );
                }
                InferredTypeData::MergedReference(reference) => {
                    pending.extend(reference.targets(db).map(|ty| (ty, lookup, true)));
                }
                InferredTypeData::Object(object) => {
                    if let Some(prototype) = object.prototype(db) {
                        pending.push((prototype, lookup, collect));
                    }
                }
                InferredTypeData::Union(union) => {
                    pending.extend(
                        union
                            .types(db)
                            .iter()
                            .rev()
                            .copied()
                            .map(|ty| (ty, lookup, true)),
                    );
                }
                _ => {}
            }
        }

        collected_type_result(db, found)
    }

    fn find_own_member_type(
        &self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
        name: &str,
        lookup: MemberLookup,
    ) -> Option<InferredTypeData<'db>> {
        match ty {
            InferredTypeData::Class(class) => find_member_type(
                db,
                class.members(db),
                name,
                lookup,
                matches!(lookup, MemberLookup::Instance),
            ),
            InferredTypeData::Interface(interface) => {
                find_member_type(db, interface.members(db), name, lookup, true)
            }
            InferredTypeData::Literal(literal) => match literal.literal(db) {
                InferredLiteral::Object(members) => {
                    find_member_type(db, members, name, lookup, true)
                }
                _ => None,
            },
            InferredTypeData::Module(module) => {
                find_member_type(db, module.members(db), name, lookup, true)
            }
            InferredTypeData::Namespace(namespace) => {
                find_member_type(db, namespace.members(db), name, lookup, true)
            }
            InferredTypeData::Object(object) => {
                find_member_type(db, object.members(db), name, lookup, true)
            }
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum MemberLookup {
    Any,
    Instance,
}

fn class_side_type<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) -> InferredTypeData<'db> {
    match ty {
        InferredTypeData::InstanceOf(instance) => instance.ty(db),
        ty => ty,
    }
}

fn module_for_key(db: &dyn ModuleDb, module_key: ModuleKey) -> Option<ModuleInfo> {
    let module = ModuleInfo::from_id(module_key.as_id());
    let current = db.module_for_path(module.path(db))?;
    (ModuleKey::new(current.as_id()) == module_key).then_some(current)
}

fn find_member_type<'db>(
    db: &'db dyn ModuleDb,
    members: &[InferredTypeMember<'db>],
    name: &str,
    lookup: MemberLookup,
    allow_index_signature: bool,
) -> Option<InferredTypeData<'db>> {
    let named_member = members
        .iter()
        .find(|member| {
            !(matches!(lookup, MemberLookup::Instance) && member.kind.is_static())
                && member.kind.has_name(name)
        })
        .map(|member| member.ty);
    if named_member.is_some() {
        return named_member;
    }

    allow_index_signature.then(|| {
        members.iter().find_map(|member| {
            member
                .kind
                .index_signature_type()
                .is_some_and(|ty| ty.is_string_key_type(db) || ty.is_string_literal_key(db, name))
                .then_some(member.ty)
        })
    })?
}

pub(super) fn collected_type_result<'db>(
    db: &'db dyn ModuleDb,
    types: Vec<InferredTypeData<'db>>,
) -> Option<InferredTypeData<'db>> {
    if types.is_empty() {
        None
    } else {
        Some(InferredTypeData::union_from_types(db, types))
    }
}

pub(super) fn infer_module_types_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _module: ModuleInfo,
) -> Option<InferredModuleTypes<'db>> {
    None
}

pub(super) fn normalize_type_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _input: NormalizeTypeInput<'db>,
) -> InferredTypeData<'db> {
    InferredTypeData::Unknown
}

struct ResolutionCtx<'db, 'a> {
    db: &'db dyn ModuleDb,
    module_key: ModuleKey,
    js_info: &'a JsModuleInfo,
    import_types: &'a FxHashMap<ResolvedPath, InferredModuleTypes<'db>>,
    named_type_ids: FxHashSet<TypeId>,
    resolved: FxHashMap<TypeId, InferredTypeData<'db>>,
    in_progress: FxHashSet<TypeId>,
    resolution_depth: usize,
}

struct NamespaceExportCollection<'db> {
    members: Vec<InferredTypeMember<'db>>,
    seen_names: FxHashSet<String>,
    seen_modules: FxHashSet<ModuleKey>,
    stack: Vec<(ModuleInfo, bool)>,
    remaining_steps: usize,
}

impl NamespaceExportCollection<'_> {
    fn new() -> Self {
        Self {
            members: Vec::new(),
            seen_names: FxHashSet::default(),
            seen_modules: FxHashSet::default(),
            stack: Vec::new(),
            remaining_steps: MAX_EXPORT_RESOLUTION_STEPS,
        }
    }
}

pub(super) fn resolve_raw_types<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    js_info: &JsModuleInfo,
    import_types: &FxHashMap<ResolvedPath, InferredModuleTypes<'db>>,
) -> InferredModuleTypes<'db> {
    let module_key = ModuleKey::new(module.as_id());
    let named_type_ids = named_type_ids(js_info);
    let mut ctx = ResolutionCtx {
        db,
        module_key,
        js_info,
        import_types,
        named_type_ids,
        resolved: FxHashMap::default(),
        in_progress: FxHashSet::default(),
        resolution_depth: 0,
    };

    let mut named_type_ids = ctx
        .named_type_ids
        .iter()
        .map(|type_id| LocalTypeId::new(type_id.index()))
        .collect::<Vec<_>>();
    named_type_ids.sort_unstable();

    let types = (0..js_info.raw_types.len())
        .map(|index| ctx.resolve_raw_type_id(TypeId::new(index)))
        .collect();

    let expressions = js_info
        .raw_expressions
        .iter()
        .map(|(range, reference)| (*range, ctx.resolve(reference)))
        .collect();

    let binding_type_data = js_info
        .raw_binding_types
        .iter()
        .map(|(range, reference)| {
            (
                *range,
                BindingTypeData {
                    ty: ctx.resolve(reference),
                },
            )
        })
        .collect();

    InferredModuleTypes {
        module_key,
        named_type_ids: named_type_ids.into_boxed_slice(),
        types,
        expressions,
        binding_type_data,
    }
}

fn named_type_ids(js_info: &JsModuleInfo) -> FxHashSet<TypeId> {
    js_info
        .raw_binding_types
        .iter()
        .filter_map(|(range, reference)| {
            let binding = js_info.semantic_model.as_binding_by_range(*range)?;
            if !is_named_type_declaration(binding.declaration_kind()) {
                return None;
            }
            let TypeReference::Resolved(resolved_id) = reference else {
                return None;
            };
            (resolved_id.level() == TypeResolverLevel::Thin).then(|| resolved_id.id())
        })
        .collect()
}

fn is_named_type_declaration(declaration_kind: JsDeclarationKind) -> bool {
    matches!(
        declaration_kind,
        JsDeclarationKind::Class
            | JsDeclarationKind::Enum
            | JsDeclarationKind::Interface
            | JsDeclarationKind::Module
            | JsDeclarationKind::Namespace
            | JsDeclarationKind::Type
    )
}

impl<'db> ResolutionCtx<'db, '_> {
    fn resolve(&mut self, reference: &TypeReference) -> InferredTypeData<'db> {
        if self.resolution_depth >= MAX_RAW_TYPE_RESOLUTION_DEPTH {
            return InferredTypeData::Unknown;
        }

        self.resolution_depth += 1;
        let resolved = match reference {
            TypeReference::Resolved(resolved_id) => self.resolve_resolved_id(*resolved_id),
            TypeReference::Qualifier(qualifier) => self.resolve_qualifier(qualifier),
            TypeReference::Import(import) => self.resolve_import(import),
        };
        self.resolution_depth -= 1;
        resolved
    }

    fn resolve_resolved_id(&mut self, resolved_id: ResolvedTypeId) -> InferredTypeData<'db> {
        match resolved_id.level() {
            TypeResolverLevel::Thin => self.resolve_raw_type_reference(resolved_id.id()),
            TypeResolverLevel::Global => InferredTypeData::from_raw_lossy(
                self.db,
                GLOBAL_RESOLVER.get_by_id(resolved_id.id()),
            ),
            TypeResolverLevel::Full | TypeResolverLevel::Import => InferredTypeData::Unknown,
        }
    }

    fn resolve_raw_type_reference(&mut self, type_id: TypeId) -> InferredTypeData<'db> {
        if self.named_type_ids.contains(&type_id) {
            return self.local_type(type_id);
        }

        self.resolve_raw_type_id(type_id)
    }

    fn local_type(&self, type_id: TypeId) -> InferredTypeData<'db> {
        InferredTypeData::Local(LocalTypeHandle::new(
            self.db,
            self.module_key,
            LocalTypeId::new(type_id.index()),
        ))
    }

    fn resolve_raw_type_id(&mut self, type_id: TypeId) -> InferredTypeData<'db> {
        if let Some(ty) = self.resolved.get(&type_id) {
            return *ty;
        }

        if !self.in_progress.insert(type_id) {
            return InferredTypeData::Unknown;
        }

        let ty = self.js_info.raw_types.get(type_id.index()).cloned().map_or(
            InferredTypeData::Unknown,
            |raw| {
                let db = self.db;
                InferredTypeData::from_raw_with_resolver(db, &raw, &mut |reference| {
                    self.resolve(reference)
                })
            },
        );

        self.in_progress.remove(&type_id);
        self.resolved.insert(type_id, ty);
        ty
    }

    fn resolve_qualifier(
        &mut self,
        qualifier: &biome_js_type_info::TypeReferenceQualifier,
    ) -> InferredTypeData<'db> {
        let Some(identifier) = qualifier.path.iter().next() else {
            return InferredTypeData::Unknown;
        };

        let mut scope = self
            .js_info
            .semantic_model
            .scope_from_id(qualifier.scope_id);
        let mut remaining_steps = MAX_SCOPE_RESOLUTION_STEPS;
        loop {
            if remaining_steps == 0 {
                return InferredTypeData::Unknown;
            }
            remaining_steps -= 1;

            if let Some(binding) = scope.get_binding(identifier.text()) {
                if binding.is_imported()
                    && let Some(import) = self.js_info.static_imports.get(identifier.text())
                {
                    return self.resolve_import(&TypeImportQualifier {
                        symbol: import.symbol.clone(),
                        resolved_path: import.resolved_path.clone(),
                        type_only: qualifier.type_only,
                    });
                }

                return self
                    .js_info
                    .raw_binding_types
                    .get(&binding.syntax().text_trimmed_range())
                    .cloned()
                    .map_or(InferredTypeData::Unknown, |reference| {
                        self.resolve(&reference)
                    });
            }

            match scope.parent() {
                Some(parent) => scope = parent,
                None => break,
            }
        }

        if let Some(resolved_id) = GLOBAL_RESOLVER.resolve_qualifier(qualifier) {
            return self.resolve_resolved_id(resolved_id);
        }

        if qualifier.is_record() && qualifier.type_parameters.len() == 2 {
            let key_ty = self.resolve(&qualifier.type_parameters[0]);
            let value_ty = self.resolve(&qualifier.type_parameters[1]);
            return InferredTypeData::Object(InferredObject::new(
                self.db,
                None,
                Box::from([InferredTypeMember {
                    kind: InferredTypeMemberKind::IndexSignature(key_ty),
                    ty: value_ty,
                }]),
            ));
        }

        if qualifier.is_array() {
            return InferredTypeData::array_instance(
                self.db,
                qualifier
                    .type_parameters
                    .iter()
                    .map(|parameter| self.resolve(parameter))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            );
        }

        if qualifier.is_map() {
            return InferredTypeData::map_instance(
                self.db,
                qualifier
                    .type_parameters
                    .iter()
                    .map(|parameter| self.resolve(parameter))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            );
        }

        if qualifier.is_promise() {
            return InferredTypeData::promise_instance(
                self.db,
                qualifier
                    .type_parameters
                    .iter()
                    .map(|parameter| self.resolve(parameter))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            );
        }

        if qualifier.is_set() {
            return InferredTypeData::set_instance(
                self.db,
                qualifier
                    .type_parameters
                    .iter()
                    .map(|parameter| self.resolve(parameter))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            );
        }

        InferredTypeData::Unknown
    }

    fn resolve_import(&mut self, qualifier: &TypeImportQualifier) -> InferredTypeData<'db> {
        self.resolve_import_qualifier(qualifier)
    }

    fn resolve_import_qualifier(&self, qualifier: &TypeImportQualifier) -> InferredTypeData<'db> {
        let Some(module) = self.module_for_resolved_path(&qualifier.resolved_path) else {
            return InferredTypeData::Unknown;
        };

        let Some(imported_types) = self.import_types.get(&qualifier.resolved_path) else {
            return infer_module_types(self.db, module)
                .map_or(InferredTypeData::Unknown, |types| {
                    self.resolve_import_symbol(module, &types, &qualifier.symbol)
                });
        };

        self.resolve_import_symbol(module, imported_types, &qualifier.symbol)
    }

    fn module_for_resolved_path(&self, resolved_path: &ResolvedPath) -> Option<ModuleInfo> {
        let path = resolved_path.as_path()?;
        self.db.module_for_path(path)
    }

    fn resolve_import_symbol(
        &self,
        module: ModuleInfo,
        inferred_types: &InferredModuleTypes<'db>,
        symbol: &ImportSymbol,
    ) -> InferredTypeData<'db> {
        match symbol {
            ImportSymbol::All => self.namespace_for_module(module, inferred_types),
            ImportSymbol::Default => self.resolve_export_name(module, inferred_types, "default"),
            ImportSymbol::Named(name) => {
                self.resolve_export_name(module, inferred_types, name.text())
            }
        }
    }

    fn resolve_js_import(&self, import: &JsImport) -> InferredTypeData<'db> {
        self.module_for_resolved_path(&import.resolved_path)
            .and_then(|module| {
                infer_module_types(self.db, module)
                    .map(|types| self.resolve_import_symbol(module, &types, &import.symbol))
            })
            .unwrap_or(InferredTypeData::Unknown)
    }

    fn namespace_for_module(
        &self,
        module: ModuleInfo,
        inferred_types: &InferredModuleTypes<'db>,
    ) -> InferredTypeData<'db> {
        let mut collection = NamespaceExportCollection::new();

        if !self.collect_namespace_members(module, inferred_types, true, &mut collection) {
            return InferredTypeData::Unknown;
        }

        while let Some((module, include_default)) = collection.stack.pop() {
            if collection.remaining_steps == 0 {
                return InferredTypeData::Unknown;
            }
            collection.remaining_steps -= 1;

            let Some(inferred_types) = infer_module_types(self.db, module) else {
                continue;
            };

            if !self.collect_namespace_members(
                module,
                &inferred_types,
                include_default,
                &mut collection,
            ) {
                return InferredTypeData::Unknown;
            }
        }

        InferredTypeData::Namespace(InferredNamespace::new(
            self.db,
            collection.members.into_boxed_slice(),
            Path::from(Text::from(module.path(self.db).to_string())),
        ))
    }

    fn collect_namespace_members(
        &self,
        module: ModuleInfo,
        inferred_types: &InferredModuleTypes<'db>,
        include_default: bool,
        collection: &mut NamespaceExportCollection<'db>,
    ) -> bool {
        let module_key = ModuleKey::new(module.as_id());
        if !collection.seen_modules.insert(module_key) {
            return true;
        }

        let ModuleInfoKind::Js(js_info) = module.kind(self.db) else {
            return true;
        };

        for (name, _) in js_info.exports.iter() {
            if !include_default && name.text() == "default" {
                continue;
            }

            if !collection.seen_names.insert(name.text().to_string()) {
                continue;
            }

            if collection.remaining_steps == 0 {
                return false;
            }
            collection.remaining_steps -= 1;

            collection.members.push(InferredTypeMember {
                kind: InferredTypeMemberKind::Named(name.clone()),
                ty: self.resolve_export_name(module, inferred_types, name.text()),
            });
        }

        for reexport in js_info.blanket_reexports.iter().rev() {
            if let Some(module) = self.module_for_resolved_path(&reexport.import.resolved_path) {
                collection.stack.push((module, false));
            }
        }

        true
    }

    fn resolve_export_name(
        &self,
        module: ModuleInfo,
        inferred_types: &InferredModuleTypes<'db>,
        name: &str,
    ) -> InferredTypeData<'db> {
        let mut stack = Vec::new();
        let mut seen = FxHashSet::default();
        let mut remaining_steps = MAX_EXPORT_RESOLUTION_STEPS;

        if let Some(ty) = self.resolve_export_name_in_module(
            module,
            inferred_types,
            name,
            &mut stack,
            &mut seen,
            &mut remaining_steps,
        ) {
            return ty;
        }

        while let Some((module, name)) = stack.pop() {
            if remaining_steps == 0 {
                return InferredTypeData::Unknown;
            }
            remaining_steps -= 1;

            let Some(inferred_types) = infer_module_types(self.db, module) else {
                continue;
            };

            if let Some(ty) = self.resolve_export_name_in_module(
                module,
                &inferred_types,
                &name,
                &mut stack,
                &mut seen,
                &mut remaining_steps,
            ) {
                return ty;
            }
        }

        InferredTypeData::Unknown
    }

    fn resolve_export_name_in_module(
        &self,
        module: ModuleInfo,
        inferred_types: &InferredModuleTypes<'db>,
        name: &str,
        stack: &mut Vec<(ModuleInfo, String)>,
        seen: &mut FxHashSet<(ModuleKey, String)>,
        remaining_steps: &mut usize,
    ) -> Option<InferredTypeData<'db>> {
        let module_key = ModuleKey::new(module.as_id());
        if !seen.insert((module_key, name.to_string())) {
            return None;
        }

        let ModuleInfoKind::Js(js_info) = module.kind(self.db) else {
            return None;
        };

        match js_info.exports.get(name) {
            Some(JsExport::Own(own_export) | JsExport::OwnType(own_export)) => {
                Some(self.resolve_own_export(inferred_types, own_export))
            }
            Some(JsExport::Reexport(reexport) | JsExport::ReexportType(reexport)) => {
                self.push_reexport_target(reexport.import.clone(), name, stack);
                None
            }
            None => {
                for reexport in js_info.blanket_reexports.iter().rev() {
                    if *remaining_steps == 0 {
                        return Some(InferredTypeData::Unknown);
                    }
                    *remaining_steps -= 1;
                    if let Some(module) =
                        self.module_for_resolved_path(&reexport.import.resolved_path)
                    {
                        stack.push((module, name.to_string()));
                    }
                }
                None
            }
        }
    }

    fn push_reexport_target(
        &self,
        import: JsImport,
        fallback_name: &str,
        stack: &mut Vec<(ModuleInfo, String)>,
    ) {
        let Some(module) = self.module_for_resolved_path(&import.resolved_path) else {
            return;
        };

        match import.symbol {
            ImportSymbol::All => {
                stack.push((module, fallback_name.to_string()));
            }
            ImportSymbol::Default => stack.push((module, "default".to_string())),
            ImportSymbol::Named(name) => stack.push((module, name.text().to_string())),
        }
    }

    fn resolve_own_export(
        &self,
        inferred_types: &InferredModuleTypes<'db>,
        own_export: &JsOwnExport,
    ) -> InferredTypeData<'db> {
        match own_export {
            JsOwnExport::Binding(range) => inferred_types
                .binding_type_data
                .get(range)
                .map_or(InferredTypeData::Unknown, |data| data.ty),
            JsOwnExport::Type(resolved_id) => {
                inferred_type_from_resolved_id(self.db, inferred_types, *resolved_id)
            }
            JsOwnExport::Namespace(reexport) => self.resolve_js_import(&reexport.import),
        }
    }
}

fn inferred_type_from_resolved_id<'db>(
    db: &'db dyn ModuleDb,
    inferred_types: &InferredModuleTypes<'db>,
    resolved_id: ResolvedTypeId,
) -> InferredTypeData<'db> {
    match resolved_id.level() {
        TypeResolverLevel::Thin => {
            let local_type_id = LocalTypeId::new(resolved_id.index());
            if inferred_types.named_type_ids.contains(&local_type_id) {
                InferredTypeData::Local(LocalTypeHandle::new(
                    db,
                    inferred_types.module_key,
                    local_type_id,
                ))
            } else {
                inferred_types
                    .types
                    .get(resolved_id.index())
                    .copied()
                    .unwrap_or(InferredTypeData::Unknown)
            }
        }
        TypeResolverLevel::Global => {
            InferredTypeData::from_raw_lossy(db, GLOBAL_RESOLVER.get_by_id(resolved_id.id()))
        }
        TypeResolverLevel::Full | TypeResolverLevel::Import => InferredTypeData::Unknown,
    }
}
