use crate::db::queries::{
    NormalizeTypeInput, infer_call_expression_return_type, infer_module_types,
};
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{JsExport, JsImport, JsModuleInfo, JsOwnExport, ModuleDb, ResolvedPath};
use biome_css_syntax::TextRange;
use biome_js_semantic::JsDeclarationKind;
use biome_js_type_info::{
    CallArgumentType as RawCallArgumentType, ConstructorParameter as RawConstructorParameter,
    DestructureField as RawDestructureField, FunctionParameter as RawFunctionParameter,
    FunctionParameterBinding as RawFunctionParameterBinding, GLOBAL_RESOLVER, ImportSymbol,
    Literal as RawLiteral, Path, RawTypeData, ResolvedTypeId, ReturnType as RawReturnType,
    TupleElementType as RawTupleElementType, TypeId, TypeImportQualifier,
    TypeMember as RawTypeMember, TypeMemberAccessibility, TypeMemberKind as RawTypeMemberKind,
    TypeOperator as RawTypeOperator, TypeReference, TypeReferenceQualifier, TypeResolver,
    TypeResolverLevel, TypeofExpression as RawTypeofExpression,
    interned_types::{
        AssertsReturnType as InferredAssertsReturnType,
        ConstructorParameter as InferredConstructorParameter,
        FunctionParameter as InferredFunctionParameter,
        FunctionParameterBinding as InferredFunctionParameterBinding,
        InternedClass as InferredInternedClass, InternedConstructor as InferredInternedConstructor,
        InternedFunction as InferredInternedFunction,
        InternedGenericTypeParameter as InferredInternedGenericTypeParameter,
        InternedInterface as InferredInternedInterface, InternedLiteral as InferredInternedLiteral,
        InternedMergedReference as InferredInternedMergedReference,
        InternedModule as InferredInternedModule, InternedNamespace as InferredNamespace,
        InternedObject as InferredInternedObject, InternedTuple as InferredInternedTuple,
        InternedTypeOperatorType as InferredInternedTypeOperatorType, Literal as InferredLiteral,
        LocalTypeHandle, LocalTypeId, ModuleKey,
        NamedFunctionParameter as InferredNamedFunctionParameter,
        PatternFunctionParameter as InferredPatternFunctionParameter,
        PredicateReturnType as InferredPredicateReturnType, ReturnType as InferredReturnType,
        TupleElementType as InferredTupleElementType, TypeData as InferredTypeData,
        TypeMember as InferredTypeMember, TypeMemberKind as InferredTypeMemberKind,
        TypeSubstitution as InferredTypeSubstitution,
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

        for _ in 0..MAX_LOCAL_TYPE_RESOLUTION_STEPS {
            let InferredTypeData::Local(local) = ty else {
                return ty;
            };

            let module_key = local.module(db);
            let type_id = local.type_id(db);
            if !seen.insert((module_key, type_id)) {
                return ty;
            }

            ty = self
                .type_for_local_handle(db, local)
                .unwrap_or(InferredTypeData::Unknown);
        }

        ty
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
        let mut pending = vec![MemberLookupState::new(ty, MemberLookup::Any, false)];
        let mut found = Vec::new();
        let mut remaining_steps = MAX_MEMBER_LOOKUP_STEPS;

        while let Some(mut state) = pending.pop() {
            let lookup = state.lookup;
            let collect = state.collect;
            let ty = self.resolve_type_iterative(db, state.ty);
            let (ty, lookup) = match ty {
                InferredTypeData::InstanceOf(instance) => {
                    let target = self.resolve_type_iterative(db, instance.ty(db));
                    state.substitutions = substitutions_for_instance(
                        db,
                        target,
                        instance.type_parameters(db),
                        &state.substitutions,
                    );
                    (target, MemberLookup::Instance)
                }
                ty => (ty, lookup),
            };

            if !seen.insert((ty, lookup, state.substitutions.clone())) {
                continue;
            }

            // Deduplicated entries above don't count against the budget, so
            // the limit measures distinct types visited, not queue churn.
            if remaining_steps == 0 {
                break;
            }
            remaining_steps -= 1;

            if let Some(member_ty) = self.find_own_member_type(db, ty, name, lookup) {
                let member_ty = apply_substitutions(db, member_ty, &state.substitutions);
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
                        pending.push(MemberLookupState {
                            ty: apply_substitutions(db, extends, &state.substitutions),
                            lookup,
                            collect,
                            substitutions: state.substitutions.clone(),
                        });
                    }
                }
                InferredTypeData::Interface(interface) => {
                    pending.extend(interface.extends(db).iter().rev().copied().map(|ty| {
                        MemberLookupState {
                            ty: apply_substitutions(db, ty, &state.substitutions),
                            lookup,
                            collect,
                            substitutions: state.substitutions.clone(),
                        }
                    }));
                }
                InferredTypeData::Generic(generic) => {
                    if let Some(constraint) = generic.constraint(db) {
                        pending.push(MemberLookupState {
                            ty: apply_substitutions(db, constraint, &state.substitutions),
                            lookup,
                            collect,
                            substitutions: state.substitutions.clone(),
                        });
                    }
                }
                InferredTypeData::Intersection(intersection) => {
                    pending.extend(intersection.types(db).iter().rev().copied().map(|ty| {
                        MemberLookupState {
                            ty: apply_substitutions(db, ty, &state.substitutions),
                            lookup,
                            collect: true,
                            substitutions: state.substitutions.clone(),
                        }
                    }));
                }
                InferredTypeData::MergedReference(reference) => {
                    pending.extend(reference.targets(db).map(|ty| MemberLookupState {
                        ty: apply_substitutions(db, ty, &state.substitutions),
                        lookup,
                        collect: true,
                        substitutions: state.substitutions.clone(),
                    }));
                }
                InferredTypeData::Object(object) => {
                    if let Some(prototype) = object.prototype(db) {
                        pending.push(MemberLookupState {
                            ty: apply_substitutions(db, prototype, &state.substitutions),
                            lookup,
                            collect,
                            substitutions: state.substitutions.clone(),
                        });
                    }
                }
                InferredTypeData::Union(union) => {
                    pending.extend(union.types(db).iter().rev().copied().map(|ty| {
                        MemberLookupState {
                            ty: apply_substitutions(db, ty, &state.substitutions),
                            lookup,
                            collect: true,
                            substitutions: state.substitutions.clone(),
                        }
                    }));
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

#[derive(Clone, Copy)]
enum StaticMemberMode {
    Class,
    Instance,
}

impl StaticMemberMode {
    fn allows_named_member(self, kind: &InferredTypeMemberKind<'_>) -> bool {
        match self {
            StaticMemberMode::Class => kind.is_static() && !kind.is_constructor(),
            StaticMemberMode::Instance => !kind.is_static(),
        }
    }

    fn allows_index_signature(self) -> bool {
        matches!(self, StaticMemberMode::Instance)
    }
}

#[derive(Clone)]
struct MemberLookupState<'db> {
    ty: InferredTypeData<'db>,
    lookup: MemberLookup,
    collect: bool,
    substitutions: Vec<InferredTypeSubstitution<'db>>,
}

impl<'db> MemberLookupState<'db> {
    fn new(ty: InferredTypeData<'db>, lookup: MemberLookup, collect: bool) -> Self {
        Self {
            ty,
            lookup,
            collect,
            substitutions: Vec::new(),
        }
    }
}

fn substitutions_for_instance<'db>(
    db: &'db dyn ModuleDb,
    target: InferredTypeData<'db>,
    type_parameters: &[InferredTypeData<'db>],
    inherited: &[InferredTypeSubstitution<'db>],
) -> Vec<InferredTypeSubstitution<'db>> {
    let Some(declared_parameters) = declared_type_parameters(db, target) else {
        return inherited.to_vec();
    };
    if declared_parameters.is_empty() {
        return inherited.to_vec();
    }

    let mut substitutions = inherited.to_vec();
    for (declared, replacement) in declared_parameters.iter().zip(type_parameters) {
        let declared = apply_substitutions(db, *declared, inherited);
        let replacement = apply_substitutions(db, *replacement, inherited);
        let declared_instance = InferredTypeData::instance_of(db, declared, Box::default());
        if declared_instance != declared {
            substitutions.push(InferredTypeSubstitution {
                generic: declared_instance,
                replacement,
            });
        }
        substitutions.push(InferredTypeSubstitution {
            generic: declared,
            replacement,
        });
    }

    substitutions
}

fn declared_type_parameters<'db>(
    db: &'db dyn ModuleDb,
    target: InferredTypeData<'db>,
) -> Option<&'db [InferredTypeData<'db>]> {
    match target {
        InferredTypeData::Class(class) => Some(class.type_parameters(db)),
        InferredTypeData::Function(function) => Some(function.type_parameters(db)),
        InferredTypeData::InstanceOf(instance) => Some(instance.type_parameters(db)),
        InferredTypeData::Interface(interface) => Some(interface.type_parameters(db)),
        _ => None,
    }
}

fn apply_substitutions<'db>(
    db: &'db dyn ModuleDb,
    mut ty: InferredTypeData<'db>,
    substitutions: &[InferredTypeSubstitution<'db>],
) -> InferredTypeData<'db> {
    for substitution in substitutions {
        ty = ty.substitute_type(db, *substitution);
    }
    ty
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

enum GlobalTypeWork<'a> {
    TypeId(TypeId),
    Raw(&'a RawTypeData),
    Reference(&'a TypeReference),
    RebuildTypeId(TypeId),
    RebuildClass {
        type_parameters: usize,
        has_extends: bool,
        implements: usize,
        members: usize,
        name: Option<Text>,
    },
    RebuildConstructor {
        type_parameters: usize,
        parameters: usize,
        has_return_type: bool,
    },
    RebuildFunction {
        type_parameters: usize,
        parameters: usize,
        is_async: bool,
        name: Option<Text>,
    },
    RebuildInterface {
        type_parameters: usize,
        extends: usize,
        members: usize,
        name: Text,
    },
    RebuildObject {
        has_prototype: bool,
        members: usize,
    },
    RebuildModule {
        members: usize,
        name: Text,
    },
    RebuildNamespace {
        members: usize,
        path: Path,
    },
    RebuildTuple(usize),
    RebuildGeneric {
        has_constraint: bool,
        has_default: bool,
        name: Text,
    },
    RebuildInstance(usize),
    RebuildIntersection(usize),
    RebuildUnion(usize),
    RebuildTypeOperator(RawTypeOperator),
    RebuildLiteralObject(usize),
    RebuildMergedReference {
        has_ty: bool,
        has_value_ty: bool,
        has_namespace_ty: bool,
    },
    RebuildTypeMember(&'a RawTypeMemberKind),
    RebuildConstructorParameter(Option<TypeMemberAccessibility>),
    RebuildFunctionParameter(&'a RawFunctionParameter),
    RebuildFunctionParameterBinding(Text),
    RebuildReturnType(&'a RawReturnType),
    RebuildTupleElement {
        name: Option<Text>,
        is_optional: bool,
        is_rest: bool,
    },
}

enum GlobalTypeValue<'db> {
    Type(InferredTypeData<'db>),
    Member(InferredTypeMember<'db>),
    ConstructorParameter(InferredConstructorParameter<'db>),
    FunctionParameter(InferredFunctionParameter<'db>),
    FunctionParameterBinding(InferredFunctionParameterBinding<'db>),
    ReturnType(InferredReturnType<'db>),
    TupleElement(InferredTupleElementType<'db>),
}

fn resolve_global_type_id<'db>(
    db: &'db dyn ModuleDb,
    type_id: TypeId,
    resolved_globals: &mut FxHashMap<TypeId, InferredTypeData<'db>>,
) -> InferredTypeData<'db> {
    resolve_global_type_id_with_resolver(db, GLOBAL_RESOLVER.as_ref(), type_id, resolved_globals)
}

fn resolve_global_type_id_with_resolver<'db, 'a>(
    db: &'db dyn ModuleDb,
    resolver: &'a dyn TypeResolver,
    type_id: TypeId,
    resolved_globals: &mut FxHashMap<TypeId, InferredTypeData<'db>>,
) -> InferredTypeData<'db> {
    if let Some(ty) = resolved_globals.get(&type_id) {
        return *ty;
    }

    let mut stack = vec![GlobalTypeWork::TypeId(type_id)];
    let mut values = Vec::new();
    let mut active = FxHashSet::default();

    // The stack walks finite borrowed raw type trees. `TypeId` references
    // terminate through the memo table or the active-set cycle placeholder.
    while let Some(work) = stack.pop() {
        match work {
            GlobalTypeWork::TypeId(type_id) => {
                if let Some(ty) = resolved_globals.get(&type_id) {
                    values.push(GlobalTypeValue::Type(*ty));
                } else if active.contains(&type_id) {
                    values.push(GlobalTypeValue::Type(global_cycle_placeholder(
                        db, resolver, type_id,
                    )));
                } else {
                    active.insert(type_id);
                    stack.push(GlobalTypeWork::RebuildTypeId(type_id));
                    stack.push(GlobalTypeWork::Raw(resolver.get_by_id(type_id)));
                }
            }
            GlobalTypeWork::Raw(raw) => push_global_raw_type(db, raw, &mut stack, &mut values),
            GlobalTypeWork::Reference(reference) => {
                push_global_reference(resolver, reference, &mut stack, &mut values)
            }
            GlobalTypeWork::RebuildTypeId(type_id) => {
                let ty = pop_global_type(&mut values);
                active.remove(&type_id);
                if active.is_empty() {
                    resolved_globals.insert(type_id, ty);
                }
                values.push(GlobalTypeValue::Type(ty));
            }
            GlobalTypeWork::RebuildClass {
                type_parameters,
                has_extends,
                implements,
                members,
                name,
            } => {
                let members = pop_global_members(&mut values, members);
                let implements = pop_global_types(&mut values, implements);
                let extends = has_extends.then(|| pop_global_type(&mut values));
                let type_parameters = pop_global_types(&mut values, type_parameters);
                values.push(GlobalTypeValue::Type(InferredTypeData::Class(
                    InferredInternedClass::new(
                        db,
                        type_parameters.into_boxed_slice(),
                        extends,
                        implements.into_boxed_slice(),
                        members.into_boxed_slice(),
                        name,
                    ),
                )));
            }
            GlobalTypeWork::RebuildConstructor {
                type_parameters,
                parameters,
                has_return_type,
            } => {
                let return_type = has_return_type.then(|| pop_global_type(&mut values));
                let parameters = pop_global_constructor_parameters(&mut values, parameters);
                let type_parameters = pop_global_types(&mut values, type_parameters);
                values.push(GlobalTypeValue::Type(InferredTypeData::Constructor(
                    InferredInternedConstructor::new(
                        db,
                        type_parameters.into_boxed_slice(),
                        parameters.into_boxed_slice(),
                        return_type,
                    ),
                )));
            }
            GlobalTypeWork::RebuildFunction {
                type_parameters,
                parameters,
                is_async,
                name,
            } => {
                let return_type = pop_global_return_type(&mut values);
                let parameters = pop_global_function_parameters(&mut values, parameters);
                let type_parameters = pop_global_types(&mut values, type_parameters);
                values.push(GlobalTypeValue::Type(InferredTypeData::Function(
                    InferredInternedFunction::new(
                        db,
                        type_parameters.into_boxed_slice(),
                        parameters.into_boxed_slice(),
                        return_type,
                        is_async,
                        name,
                    ),
                )));
            }
            GlobalTypeWork::RebuildInterface {
                type_parameters,
                extends,
                members,
                name,
            } => {
                let members = pop_global_members(&mut values, members);
                let extends = pop_global_types(&mut values, extends);
                let type_parameters = pop_global_types(&mut values, type_parameters);
                values.push(GlobalTypeValue::Type(InferredTypeData::Interface(
                    InferredInternedInterface::new(
                        db,
                        type_parameters.into_boxed_slice(),
                        extends.into_boxed_slice(),
                        members.into_boxed_slice(),
                        name,
                    ),
                )));
            }
            GlobalTypeWork::RebuildObject {
                has_prototype,
                members,
            } => {
                let members = pop_global_members(&mut values, members);
                let prototype = has_prototype.then(|| pop_global_type(&mut values));
                values.push(GlobalTypeValue::Type(InferredTypeData::Object(
                    InferredInternedObject::new(db, prototype, members.into_boxed_slice()),
                )));
            }
            GlobalTypeWork::RebuildModule { members, name } => {
                let members = pop_global_members(&mut values, members);
                values.push(GlobalTypeValue::Type(InferredTypeData::Module(
                    InferredInternedModule::new(db, members.into_boxed_slice(), name),
                )));
            }
            GlobalTypeWork::RebuildNamespace { members, path } => {
                let members = pop_global_members(&mut values, members);
                values.push(GlobalTypeValue::Type(InferredTypeData::Namespace(
                    InferredNamespace::new(db, members.into_boxed_slice(), path),
                )));
            }
            GlobalTypeWork::RebuildTuple(elements) => {
                let elements = pop_global_tuple_elements(&mut values, elements);
                values.push(GlobalTypeValue::Type(InferredTypeData::Tuple(
                    InferredInternedTuple::new(db, elements.into_boxed_slice()),
                )));
            }
            GlobalTypeWork::RebuildGeneric {
                has_constraint,
                has_default,
                name,
            } => {
                let default = has_default.then(|| pop_global_type(&mut values));
                let constraint = has_constraint.then(|| pop_global_type(&mut values));
                values.push(GlobalTypeValue::Type(InferredTypeData::Generic(
                    InferredInternedGenericTypeParameter::new(db, constraint, default, name),
                )));
            }
            GlobalTypeWork::RebuildInstance(type_parameters) => {
                let type_parameters = pop_global_types(&mut values, type_parameters);
                let ty = pop_global_type(&mut values);
                values.push(GlobalTypeValue::Type(InferredTypeData::instance_of(
                    db,
                    ty,
                    type_parameters.into_boxed_slice(),
                )));
            }
            GlobalTypeWork::RebuildIntersection(types) => {
                let types = pop_global_types(&mut values, types);
                values.push(GlobalTypeValue::Type(
                    InferredTypeData::intersection_from_types(db, types),
                ));
            }
            GlobalTypeWork::RebuildUnion(types) => {
                let types = pop_global_types(&mut values, types);
                values.push(GlobalTypeValue::Type(InferredTypeData::union_from_types(
                    db, types,
                )));
            }
            GlobalTypeWork::RebuildTypeOperator(operator) => {
                let ty = pop_global_type(&mut values);
                values.push(GlobalTypeValue::Type(InferredTypeData::TypeOperator(
                    InferredInternedTypeOperatorType::new(db, ty, operator),
                )));
            }
            GlobalTypeWork::RebuildLiteralObject(members) => {
                let members = pop_global_members(&mut values, members);
                values.push(GlobalTypeValue::Type(InferredTypeData::Literal(
                    InferredInternedLiteral::new(
                        db,
                        InferredLiteral::Object(members.into_boxed_slice()),
                    ),
                )));
            }
            GlobalTypeWork::RebuildMergedReference {
                has_ty,
                has_value_ty,
                has_namespace_ty,
            } => {
                let namespace_ty = has_namespace_ty.then(|| pop_global_type(&mut values));
                let value_ty = has_value_ty.then(|| pop_global_type(&mut values));
                let ty = has_ty.then(|| pop_global_type(&mut values));
                values.push(GlobalTypeValue::Type(InferredTypeData::MergedReference(
                    InferredInternedMergedReference::new(db, ty, value_ty, namespace_ty),
                )));
            }
            GlobalTypeWork::RebuildTypeMember(kind) => {
                let member_ty = pop_global_type(&mut values);
                let key_ty = raw_member_kind_reference(kind).map(|_| pop_global_type(&mut values));
                values.push(GlobalTypeValue::Member(InferredTypeMember {
                    kind: global_member_kind_from_raw(kind, key_ty),
                    ty: member_ty,
                }));
            }
            GlobalTypeWork::RebuildConstructorParameter(accessibility) => {
                let parameter = pop_global_function_parameter(&mut values);
                values.push(GlobalTypeValue::ConstructorParameter(
                    InferredConstructorParameter {
                        parameter,
                        accessibility,
                    },
                ));
            }
            GlobalTypeWork::RebuildFunctionParameter(parameter) => match parameter {
                RawFunctionParameter::Named(parameter) => {
                    let ty = pop_global_type(&mut values);
                    values.push(GlobalTypeValue::FunctionParameter(
                        InferredFunctionParameter::Named(InferredNamedFunctionParameter {
                            name: parameter.name.clone(),
                            ty,
                            is_optional: parameter.is_optional,
                            is_rest: parameter.is_rest,
                        }),
                    ));
                }
                RawFunctionParameter::Pattern(parameter) => {
                    let ty = pop_global_type(&mut values);
                    let bindings = pop_global_function_parameter_bindings(
                        &mut values,
                        parameter.bindings.len(),
                    );
                    values.push(GlobalTypeValue::FunctionParameter(
                        InferredFunctionParameter::Pattern(InferredPatternFunctionParameter {
                            bindings: bindings.into_boxed_slice(),
                            ty,
                            is_optional: parameter.is_optional,
                            is_rest: parameter.is_rest,
                        }),
                    ));
                }
            },
            GlobalTypeWork::RebuildFunctionParameterBinding(name) => {
                let ty = pop_global_type(&mut values);
                values.push(GlobalTypeValue::FunctionParameterBinding(
                    InferredFunctionParameterBinding { name, ty },
                ));
            }
            GlobalTypeWork::RebuildReturnType(return_type) => match return_type {
                RawReturnType::Type(_) => {
                    let ty = pop_global_type(&mut values);
                    values.push(GlobalTypeValue::ReturnType(InferredReturnType::Type(ty)));
                }
                RawReturnType::Predicate(predicate) => {
                    let ty = pop_global_type(&mut values);
                    values.push(GlobalTypeValue::ReturnType(InferredReturnType::Predicate(
                        InferredPredicateReturnType {
                            parameter_name: predicate.parameter_name.clone(),
                            ty,
                        },
                    )));
                }
                RawReturnType::Asserts(asserts) => {
                    let ty = pop_global_type(&mut values);
                    values.push(GlobalTypeValue::ReturnType(InferredReturnType::Asserts(
                        InferredAssertsReturnType {
                            parameter_name: asserts.parameter_name.clone(),
                            ty,
                        },
                    )));
                }
            },
            GlobalTypeWork::RebuildTupleElement {
                name,
                is_optional,
                is_rest,
            } => {
                let ty = pop_global_type(&mut values);
                values.push(GlobalTypeValue::TupleElement(InferredTupleElementType {
                    ty,
                    name,
                    is_optional,
                    is_rest,
                }));
            }
        }
    }

    debug_assert_eq!(values.len(), 1, "global type converter stack imbalance");
    pop_global_type(&mut values)
}

fn global_cycle_placeholder<'db>(
    db: &'db dyn ModuleDb,
    resolver: &dyn TypeResolver,
    type_id: TypeId,
) -> InferredTypeData<'db> {
    // Until globals have identity handles like local types, recursive edges are
    // represented by shallow placeholders. This preserves nominal checks but
    // caps structural member traversal through recursive global cycles.
    match resolver.get_by_id(type_id) {
        RawTypeData::Class(class) => InferredTypeData::Class(InferredInternedClass::new(
            db,
            Box::default(),
            None,
            Box::default(),
            Box::default(),
            class.name.clone(),
        )),
        RawTypeData::Interface(interface) => {
            InferredTypeData::Interface(InferredInternedInterface::new(
                db,
                Box::default(),
                Box::default(),
                Box::default(),
                interface.name.clone(),
            ))
        }
        RawTypeData::Generic(generic) => InferredTypeData::Generic(
            InferredInternedGenericTypeParameter::new(db, None, None, generic.name.clone()),
        ),
        RawTypeData::BigInt => InferredTypeData::BigInt,
        RawTypeData::Boolean => InferredTypeData::Boolean,
        RawTypeData::Conditional => InferredTypeData::Conditional,
        RawTypeData::Global => InferredTypeData::Global,
        RawTypeData::Null => InferredTypeData::Null,
        RawTypeData::Number => InferredTypeData::Number,
        RawTypeData::String => InferredTypeData::String,
        RawTypeData::Symbol => InferredTypeData::Symbol,
        RawTypeData::Undefined => InferredTypeData::Undefined,
        RawTypeData::AnyKeyword => InferredTypeData::AnyKeyword,
        RawTypeData::NeverKeyword => InferredTypeData::NeverKeyword,
        RawTypeData::ObjectKeyword => InferredTypeData::ObjectKeyword,
        RawTypeData::ThisKeyword => InferredTypeData::ThisKeyword,
        RawTypeData::UnknownKeyword => InferredTypeData::UnknownKeyword,
        RawTypeData::VoidKeyword => InferredTypeData::VoidKeyword,
        RawTypeData::Unknown
        | RawTypeData::ImportNamespace(_)
        | RawTypeData::Constructor(_)
        | RawTypeData::Function(_)
        | RawTypeData::Module(_)
        | RawTypeData::Namespace(_)
        | RawTypeData::Object(_)
        | RawTypeData::Tuple(_)
        | RawTypeData::Intersection(_)
        | RawTypeData::Union(_)
        | RawTypeData::TypeOperator(_)
        | RawTypeData::Literal(_)
        | RawTypeData::InstanceOf(_)
        | RawTypeData::Reference(_)
        | RawTypeData::MergedReference(_)
        | RawTypeData::TypeofExpression(_)
        | RawTypeData::TypeofType(_)
        | RawTypeData::TypeofValue(_) => InferredTypeData::Unknown,
    }
}

fn push_global_raw_type<'a, 'db>(
    db: &'db dyn ModuleDb,
    raw: &'a RawTypeData,
    stack: &mut Vec<GlobalTypeWork<'a>>,
    values: &mut Vec<GlobalTypeValue<'db>>,
) {
    match raw {
        RawTypeData::Unknown => values.push(GlobalTypeValue::Type(InferredTypeData::Unknown)),
        RawTypeData::Global => values.push(GlobalTypeValue::Type(InferredTypeData::Global)),
        RawTypeData::BigInt => values.push(GlobalTypeValue::Type(InferredTypeData::BigInt)),
        RawTypeData::Boolean => values.push(GlobalTypeValue::Type(InferredTypeData::Boolean)),
        RawTypeData::Null => values.push(GlobalTypeValue::Type(InferredTypeData::Null)),
        RawTypeData::Number => values.push(GlobalTypeValue::Type(InferredTypeData::Number)),
        RawTypeData::String => values.push(GlobalTypeValue::Type(InferredTypeData::String)),
        RawTypeData::Symbol => values.push(GlobalTypeValue::Type(InferredTypeData::Symbol)),
        RawTypeData::Undefined => values.push(GlobalTypeValue::Type(InferredTypeData::Undefined)),
        RawTypeData::Conditional => {
            values.push(GlobalTypeValue::Type(InferredTypeData::Conditional));
        }
        RawTypeData::ImportNamespace(_)
        | RawTypeData::TypeofExpression(_)
        | RawTypeData::TypeofType(_)
        | RawTypeData::TypeofValue(_) => {
            values.push(GlobalTypeValue::Type(InferredTypeData::Unknown));
        }
        RawTypeData::Class(class) => {
            stack.push(GlobalTypeWork::RebuildClass {
                type_parameters: class.type_parameters.len(),
                has_extends: class.extends.is_some(),
                implements: class.implements.len(),
                members: class.members.len(),
                name: class.name.clone(),
            });
            push_raw_members(stack, &class.members);
            push_references(stack, &class.implements);
            if let Some(extends) = &class.extends {
                stack.push(GlobalTypeWork::Reference(extends));
            }
            push_references(stack, &class.type_parameters);
        }
        RawTypeData::Constructor(constructor) => {
            stack.push(GlobalTypeWork::RebuildConstructor {
                type_parameters: constructor.type_parameters.len(),
                parameters: constructor.parameters.len(),
                has_return_type: constructor.return_type.is_some(),
            });
            if let Some(return_type) = &constructor.return_type {
                stack.push(GlobalTypeWork::Reference(return_type));
            }
            push_constructor_parameters(stack, &constructor.parameters);
            push_references(stack, &constructor.type_parameters);
        }
        RawTypeData::Function(function) => {
            stack.push(GlobalTypeWork::RebuildFunction {
                type_parameters: function.type_parameters.len(),
                parameters: function.parameters.len(),
                is_async: function.is_async,
                name: function.name.clone(),
            });
            push_return_type(stack, &function.return_type);
            push_function_parameters(stack, &function.parameters);
            push_references(stack, &function.type_parameters);
        }
        RawTypeData::Interface(interface) => {
            stack.push(GlobalTypeWork::RebuildInterface {
                type_parameters: interface.type_parameters.len(),
                extends: interface.extends.len(),
                members: interface.members.len(),
                name: interface.name.clone(),
            });
            push_raw_members(stack, &interface.members);
            push_references(stack, &interface.extends);
            push_references(stack, &interface.type_parameters);
        }
        RawTypeData::Module(module) => {
            stack.push(GlobalTypeWork::RebuildModule {
                members: module.members.len(),
                name: module.name.clone(),
            });
            push_raw_members(stack, &module.members);
        }
        RawTypeData::Namespace(namespace) => {
            stack.push(GlobalTypeWork::RebuildNamespace {
                members: namespace.members.len(),
                path: namespace.path.clone(),
            });
            push_raw_members(stack, &namespace.members);
        }
        RawTypeData::Object(object) => {
            stack.push(GlobalTypeWork::RebuildObject {
                has_prototype: object.prototype.is_some(),
                members: object.members.len(),
            });
            push_raw_members(stack, &object.members);
            if let Some(prototype) = &object.prototype {
                stack.push(GlobalTypeWork::Reference(prototype));
            }
        }
        RawTypeData::Tuple(tuple) => {
            stack.push(GlobalTypeWork::RebuildTuple(tuple.elements().len()));
            push_tuple_elements(stack, tuple.elements());
        }
        RawTypeData::Generic(generic) => {
            let has_constraint = generic.constraint.is_known();
            let has_default = generic.default.is_known();
            stack.push(GlobalTypeWork::RebuildGeneric {
                has_constraint,
                has_default,
                name: generic.name.clone(),
            });
            if has_default {
                stack.push(GlobalTypeWork::Reference(&generic.default));
            }
            if has_constraint {
                stack.push(GlobalTypeWork::Reference(&generic.constraint));
            }
        }
        RawTypeData::Intersection(intersection) => {
            stack.push(GlobalTypeWork::RebuildIntersection(
                intersection.types().len(),
            ));
            push_references(stack, intersection.types());
        }
        RawTypeData::Union(union) => {
            stack.push(GlobalTypeWork::RebuildUnion(union.types().len()));
            push_references(stack, union.types());
        }
        RawTypeData::TypeOperator(type_operator) => {
            stack.push(GlobalTypeWork::RebuildTypeOperator(type_operator.operator));
            stack.push(GlobalTypeWork::Reference(&type_operator.ty));
        }
        RawTypeData::Literal(literal) => push_global_literal(db, literal.as_ref(), stack, values),
        RawTypeData::InstanceOf(instance) => {
            stack.push(GlobalTypeWork::RebuildInstance(
                instance.type_parameters.len(),
            ));
            push_references(stack, &instance.type_parameters);
            stack.push(GlobalTypeWork::Reference(&instance.ty));
        }
        RawTypeData::Reference(reference) => stack.push(GlobalTypeWork::Reference(reference)),
        RawTypeData::MergedReference(reference) => {
            stack.push(GlobalTypeWork::RebuildMergedReference {
                has_ty: reference.ty.is_some(),
                has_value_ty: reference.value_ty.is_some(),
                has_namespace_ty: reference.namespace_ty.is_some(),
            });
            if let Some(namespace_ty) = &reference.namespace_ty {
                stack.push(GlobalTypeWork::Reference(namespace_ty));
            }
            if let Some(value_ty) = &reference.value_ty {
                stack.push(GlobalTypeWork::Reference(value_ty));
            }
            if let Some(ty) = &reference.ty {
                stack.push(GlobalTypeWork::Reference(ty));
            }
        }
        RawTypeData::AnyKeyword => values.push(GlobalTypeValue::Type(InferredTypeData::AnyKeyword)),
        RawTypeData::NeverKeyword => {
            values.push(GlobalTypeValue::Type(InferredTypeData::NeverKeyword));
        }
        RawTypeData::ObjectKeyword => {
            values.push(GlobalTypeValue::Type(InferredTypeData::ObjectKeyword));
        }
        RawTypeData::ThisKeyword => {
            values.push(GlobalTypeValue::Type(InferredTypeData::ThisKeyword));
        }
        RawTypeData::UnknownKeyword => {
            values.push(GlobalTypeValue::Type(InferredTypeData::UnknownKeyword));
        }
        RawTypeData::VoidKeyword => {
            values.push(GlobalTypeValue::Type(InferredTypeData::VoidKeyword))
        }
    }
}

fn push_global_reference<'a, 'db>(
    resolver: &'a dyn TypeResolver,
    reference: &'a TypeReference,
    stack: &mut Vec<GlobalTypeWork<'a>>,
    values: &mut Vec<GlobalTypeValue<'db>>,
) {
    match reference {
        TypeReference::Resolved(resolved_id)
            if resolved_id.level() == TypeResolverLevel::Global =>
        {
            stack.push(GlobalTypeWork::TypeId(resolved_id.id()));
        }
        TypeReference::Qualifier(qualifier) => {
            if let Some(resolved_id) = resolver.resolve_qualifier(qualifier) {
                stack.push(GlobalTypeWork::TypeId(resolved_id.id()));
            } else {
                values.push(GlobalTypeValue::Type(InferredTypeData::Unknown));
            }
        }
        TypeReference::Resolved(_) | TypeReference::Import(_) => {
            values.push(GlobalTypeValue::Type(InferredTypeData::Unknown));
        }
    }
}

fn push_global_literal<'a, 'db>(
    db: &'db dyn ModuleDb,
    literal: &'a RawLiteral,
    stack: &mut Vec<GlobalTypeWork<'a>>,
    values: &mut Vec<GlobalTypeValue<'db>>,
) {
    match literal {
        RawLiteral::BigInt(value) => values.push(GlobalTypeValue::Type(InferredTypeData::Literal(
            InferredInternedLiteral::new(db, InferredLiteral::BigInt(value.clone())),
        ))),
        RawLiteral::Boolean(value) => {
            values.push(GlobalTypeValue::Type(InferredTypeData::Literal(
                InferredInternedLiteral::new(db, InferredLiteral::Boolean(value.clone())),
            )))
        }
        RawLiteral::Number(value) => values.push(GlobalTypeValue::Type(InferredTypeData::Literal(
            InferredInternedLiteral::new(db, InferredLiteral::Number(value.clone())),
        ))),
        RawLiteral::Object(object) => {
            stack.push(GlobalTypeWork::RebuildLiteralObject(object.members().len()));
            push_raw_members(stack, object.members());
        }
        RawLiteral::RegExp(value) => values.push(GlobalTypeValue::Type(InferredTypeData::Literal(
            InferredInternedLiteral::new(db, InferredLiteral::RegExp(value.clone())),
        ))),
        RawLiteral::String(value) => values.push(GlobalTypeValue::Type(InferredTypeData::Literal(
            InferredInternedLiteral::new(db, InferredLiteral::String(value.clone())),
        ))),
        RawLiteral::Template(value) => {
            values.push(GlobalTypeValue::Type(InferredTypeData::Literal(
                InferredInternedLiteral::new(db, InferredLiteral::Template(value.clone())),
            )))
        }
    }
}

fn push_references<'a>(stack: &mut Vec<GlobalTypeWork<'a>>, references: &'a [TypeReference]) {
    for reference in references.iter().rev() {
        stack.push(GlobalTypeWork::Reference(reference));
    }
}

fn push_raw_members<'a>(stack: &mut Vec<GlobalTypeWork<'a>>, members: &'a [RawTypeMember]) {
    for member in members.iter().rev() {
        stack.push(GlobalTypeWork::RebuildTypeMember(&member.kind));
        stack.push(GlobalTypeWork::Reference(&member.ty));
        if let Some(key_ty) = raw_member_kind_reference(&member.kind) {
            stack.push(GlobalTypeWork::Reference(key_ty));
        }
    }
}

fn push_constructor_parameters<'a>(
    stack: &mut Vec<GlobalTypeWork<'a>>,
    parameters: &'a [RawConstructorParameter],
) {
    for parameter in parameters.iter().rev() {
        stack.push(GlobalTypeWork::RebuildConstructorParameter(
            parameter.accessibility,
        ));
        push_function_parameter(stack, &parameter.parameter);
    }
}

fn push_function_parameters<'a>(
    stack: &mut Vec<GlobalTypeWork<'a>>,
    parameters: &'a [RawFunctionParameter],
) {
    for parameter in parameters.iter().rev() {
        push_function_parameter(stack, parameter);
    }
}

fn push_function_parameter<'a>(
    stack: &mut Vec<GlobalTypeWork<'a>>,
    parameter: &'a RawFunctionParameter,
) {
    stack.push(GlobalTypeWork::RebuildFunctionParameter(parameter));
    match parameter {
        RawFunctionParameter::Named(parameter) => {
            stack.push(GlobalTypeWork::Reference(&parameter.ty));
        }
        RawFunctionParameter::Pattern(parameter) => {
            stack.push(GlobalTypeWork::Reference(&parameter.ty));
            push_function_parameter_bindings(stack, &parameter.bindings);
        }
    }
}

fn push_function_parameter_bindings<'a>(
    stack: &mut Vec<GlobalTypeWork<'a>>,
    bindings: &'a [RawFunctionParameterBinding],
) {
    for binding in bindings.iter().rev() {
        stack.push(GlobalTypeWork::RebuildFunctionParameterBinding(
            binding.name.clone(),
        ));
        stack.push(GlobalTypeWork::Reference(&binding.ty));
    }
}

fn push_return_type<'a>(stack: &mut Vec<GlobalTypeWork<'a>>, return_type: &'a RawReturnType) {
    stack.push(GlobalTypeWork::RebuildReturnType(return_type));
    match return_type {
        RawReturnType::Type(ty) => stack.push(GlobalTypeWork::Reference(ty)),
        RawReturnType::Predicate(predicate) => stack.push(GlobalTypeWork::Reference(&predicate.ty)),
        RawReturnType::Asserts(asserts) => stack.push(GlobalTypeWork::Reference(&asserts.ty)),
    }
}

fn push_tuple_elements<'a>(
    stack: &mut Vec<GlobalTypeWork<'a>>,
    elements: &'a [RawTupleElementType],
) {
    for element in elements.iter().rev() {
        stack.push(GlobalTypeWork::RebuildTupleElement {
            name: element.name.clone(),
            is_optional: element.is_optional,
            is_rest: element.is_rest,
        });
        stack.push(GlobalTypeWork::Reference(&element.ty));
    }
}

fn raw_member_kind_reference(kind: &RawTypeMemberKind) -> Option<&TypeReference> {
    match kind {
        RawTypeMemberKind::ComputedValue(ty)
        | RawTypeMemberKind::ConstAssertedComputedValue(ty)
        | RawTypeMemberKind::ConstAssertedIndexSignature(ty)
        | RawTypeMemberKind::IndexSignature(ty) => Some(ty),
        RawTypeMemberKind::CallSignature
        | RawTypeMemberKind::ConstAssertedCallSignature
        | RawTypeMemberKind::ConstAssertedConstructor
        | RawTypeMemberKind::ConstAssertedGetter(_)
        | RawTypeMemberKind::ConstAssertedNamed(_)
        | RawTypeMemberKind::ConstAssertedNamedOptional(_)
        | RawTypeMemberKind::ConstAssertedNamedStatic(_)
        | RawTypeMemberKind::Constructor
        | RawTypeMemberKind::Getter(_)
        | RawTypeMemberKind::Named(_)
        | RawTypeMemberKind::NamedOptional(_)
        | RawTypeMemberKind::NamedStatic(_) => None,
    }
}

fn global_member_kind_from_raw<'db>(
    kind: &RawTypeMemberKind,
    key_ty: Option<InferredTypeData<'db>>,
) -> InferredTypeMemberKind<'db> {
    match kind {
        RawTypeMemberKind::CallSignature => InferredTypeMemberKind::CallSignature,
        RawTypeMemberKind::ComputedValue(_) => {
            InferredTypeMemberKind::ComputedValue(key_ty.unwrap_or(InferredTypeData::Unknown))
        }
        RawTypeMemberKind::ConstAssertedCallSignature => {
            InferredTypeMemberKind::ConstAssertedCallSignature
        }
        RawTypeMemberKind::ConstAssertedComputedValue(_) => {
            InferredTypeMemberKind::ConstAssertedComputedValue(
                key_ty.unwrap_or(InferredTypeData::Unknown),
            )
        }
        RawTypeMemberKind::ConstAssertedConstructor => {
            InferredTypeMemberKind::ConstAssertedConstructor
        }
        RawTypeMemberKind::ConstAssertedGetter(name) => {
            InferredTypeMemberKind::ConstAssertedGetter(name.clone())
        }
        RawTypeMemberKind::ConstAssertedIndexSignature(_) => {
            InferredTypeMemberKind::ConstAssertedIndexSignature(
                key_ty.unwrap_or(InferredTypeData::Unknown),
            )
        }
        RawTypeMemberKind::ConstAssertedNamed(name) => {
            InferredTypeMemberKind::ConstAssertedNamed(name.clone())
        }
        RawTypeMemberKind::ConstAssertedNamedOptional(name) => {
            InferredTypeMemberKind::ConstAssertedNamedOptional(name.clone())
        }
        RawTypeMemberKind::ConstAssertedNamedStatic(name) => {
            InferredTypeMemberKind::ConstAssertedNamedStatic(name.clone())
        }
        RawTypeMemberKind::Constructor => InferredTypeMemberKind::Constructor,
        RawTypeMemberKind::Getter(name) => InferredTypeMemberKind::Getter(name.clone()),
        RawTypeMemberKind::IndexSignature(_) => {
            InferredTypeMemberKind::IndexSignature(key_ty.unwrap_or(InferredTypeData::Unknown))
        }
        RawTypeMemberKind::Named(name) => InferredTypeMemberKind::Named(name.clone()),
        RawTypeMemberKind::NamedOptional(name) => {
            InferredTypeMemberKind::NamedOptional(name.clone())
        }
        RawTypeMemberKind::NamedStatic(name) => InferredTypeMemberKind::NamedStatic(name.clone()),
    }
}

fn pop_global_type<'db>(values: &mut Vec<GlobalTypeValue<'db>>) -> InferredTypeData<'db> {
    match values.pop() {
        Some(GlobalTypeValue::Type(ty)) => ty,
        _ => {
            unexpected_global_value("type");
            InferredTypeData::Unknown
        }
    }
}

fn pop_global_types<'db>(
    values: &mut Vec<GlobalTypeValue<'db>>,
    count: usize,
) -> Vec<InferredTypeData<'db>> {
    let mut types = Vec::with_capacity(count);
    for _ in 0..count {
        types.push(pop_global_type(values));
    }
    types.reverse();
    types
}

fn pop_global_members<'db>(
    values: &mut Vec<GlobalTypeValue<'db>>,
    count: usize,
) -> Vec<InferredTypeMember<'db>> {
    let mut members = Vec::with_capacity(count);
    for _ in 0..count {
        match values.pop() {
            Some(GlobalTypeValue::Member(member)) => members.push(member),
            _ => {
                unexpected_global_value("member");
                members.push(InferredTypeMember {
                    kind: InferredTypeMemberKind::Named(Text::new_static("unknown")),
                    ty: InferredTypeData::Unknown,
                });
            }
        }
    }
    members.reverse();
    members
}

fn pop_global_constructor_parameters<'db>(
    values: &mut Vec<GlobalTypeValue<'db>>,
    count: usize,
) -> Vec<InferredConstructorParameter<'db>> {
    let mut parameters = Vec::with_capacity(count);
    for _ in 0..count {
        match values.pop() {
            Some(GlobalTypeValue::ConstructorParameter(parameter)) => parameters.push(parameter),
            _ => {
                unexpected_global_value("constructor parameter");
                parameters.push(InferredConstructorParameter {
                    parameter: InferredFunctionParameter::Pattern(
                        InferredPatternFunctionParameter {
                            bindings: Box::default(),
                            ty: InferredTypeData::Unknown,
                            is_optional: false,
                            is_rest: false,
                        },
                    ),
                    accessibility: None,
                });
            }
        }
    }
    parameters.reverse();
    parameters
}

fn pop_global_function_parameters<'db>(
    values: &mut Vec<GlobalTypeValue<'db>>,
    count: usize,
) -> Vec<InferredFunctionParameter<'db>> {
    let mut parameters = Vec::with_capacity(count);
    for _ in 0..count {
        parameters.push(pop_global_function_parameter(values));
    }
    parameters.reverse();
    parameters
}

fn pop_global_function_parameter<'db>(
    values: &mut Vec<GlobalTypeValue<'db>>,
) -> InferredFunctionParameter<'db> {
    match values.pop() {
        Some(GlobalTypeValue::FunctionParameter(parameter)) => parameter,
        _ => {
            unexpected_global_value("function parameter");
            InferredFunctionParameter::Pattern(InferredPatternFunctionParameter {
                bindings: Box::default(),
                ty: InferredTypeData::Unknown,
                is_optional: false,
                is_rest: false,
            })
        }
    }
}

fn pop_global_function_parameter_bindings<'db>(
    values: &mut Vec<GlobalTypeValue<'db>>,
    count: usize,
) -> Vec<InferredFunctionParameterBinding<'db>> {
    let mut bindings = Vec::with_capacity(count);
    for _ in 0..count {
        match values.pop() {
            Some(GlobalTypeValue::FunctionParameterBinding(binding)) => bindings.push(binding),
            _ => {
                unexpected_global_value("function parameter binding");
                bindings.push(InferredFunctionParameterBinding {
                    name: Text::new_static("unknown"),
                    ty: InferredTypeData::Unknown,
                });
            }
        }
    }
    bindings.reverse();
    bindings
}

fn pop_global_return_type<'db>(values: &mut Vec<GlobalTypeValue<'db>>) -> InferredReturnType<'db> {
    match values.pop() {
        Some(GlobalTypeValue::ReturnType(return_type)) => return_type,
        _ => {
            unexpected_global_value("return type");
            InferredReturnType::Type(InferredTypeData::Unknown)
        }
    }
}

fn pop_global_tuple_elements<'db>(
    values: &mut Vec<GlobalTypeValue<'db>>,
    count: usize,
) -> Vec<InferredTupleElementType<'db>> {
    let mut elements = Vec::with_capacity(count);
    for _ in 0..count {
        match values.pop() {
            Some(GlobalTypeValue::TupleElement(element)) => elements.push(element),
            _ => {
                unexpected_global_value("tuple element");
                elements.push(InferredTupleElementType {
                    ty: InferredTypeData::Unknown,
                    name: None,
                    is_optional: false,
                    is_rest: false,
                });
            }
        }
    }
    elements.reverse();
    elements
}

fn unexpected_global_value(expected: &'static str) {
    debug_assert!(
        false,
        "global type converter expected {expected} on the value stack"
    );
}

fn find_member_type<'db>(
    db: &'db dyn ModuleDb,
    members: &[InferredTypeMember<'db>],
    name: &str,
    lookup: MemberLookup,
    allow_index_signature: bool,
) -> Option<InferredTypeData<'db>> {
    find_member_in_members(
        db,
        members,
        name,
        |kind| !(matches!(lookup, MemberLookup::Instance) && kind.is_static()),
        allow_index_signature,
    )
    .map(|(ty, _)| ty)
}

fn find_member_in_members<'db>(
    db: &'db dyn ModuleDb,
    members: &[InferredTypeMember<'db>],
    name: &str,
    allows_named_member: impl Fn(&InferredTypeMemberKind<'db>) -> bool,
    allow_index_signature: bool,
) -> Option<(InferredTypeData<'db>, bool)> {
    let named_member = members
        .iter()
        .find(|member| allows_named_member(&member.kind) && member.kind.has_name(name))
        .map(|member| (member.ty, member.kind.is_optional()));
    if named_member.is_some() {
        return named_member;
    }

    allow_index_signature.then(|| {
        members.iter().find_map(|member| {
            member
                .kind
                .index_signature_type()
                .is_some_and(|ty| ty.is_string_key_type(db) || ty.is_string_literal_key(db, name))
                .then_some((member.ty, false))
        })
    })?
}

fn find_member_in_members_for_mode<'db>(
    db: &'db dyn ModuleDb,
    members: &[InferredTypeMember<'db>],
    name: &str,
    mode: StaticMemberMode,
) -> Option<(InferredTypeData<'db>, bool)> {
    find_member_in_members(
        db,
        members,
        name,
        |kind| mode.allows_named_member(kind),
        mode.allows_index_signature(),
    )
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
    resolved_globals: FxHashMap<TypeId, InferredTypeData<'db>>,
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
        resolved_globals: FxHashMap::default(),
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
            TypeResolverLevel::Global => self.resolve_global_type_id(resolved_id.id()),
            TypeResolverLevel::Full | TypeResolverLevel::Import => InferredTypeData::Unknown,
        }
    }

    fn resolve_global_type_id(&mut self, type_id: TypeId) -> InferredTypeData<'db> {
        resolve_global_type_id(self.db, type_id, &mut self.resolved_globals)
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

        let js_info = self.js_info;
        let ty = js_info
            .raw_types
            .get(type_id.index())
            .map_or(InferredTypeData::Unknown, |raw| self.resolve_raw_type(raw));

        self.in_progress.remove(&type_id);
        self.resolved.insert(type_id, ty);
        ty
    }

    fn resolve_raw_type(&mut self, raw: &RawTypeData) -> InferredTypeData<'db> {
        if let RawTypeData::TypeofExpression(expression) = raw
            && let Some(ty) = self.resolve_typeof_expression(expression)
        {
            return ty;
        }

        let db = self.db;
        InferredTypeData::from_raw_with_resolver(db, raw, &mut |reference| self.resolve(reference))
    }

    fn resolve_typeof_expression(
        &mut self,
        expression: &RawTypeofExpression,
    ) -> Option<InferredTypeData<'db>> {
        match expression {
            RawTypeofExpression::Addition(expression) => {
                let left = self.resolve(&expression.left);
                let right = self.resolve(&expression.right);
                self.resolve_addition_expression(left, right)
            }
            RawTypeofExpression::Await(expression) => {
                let argument = self.resolve(&expression.argument);
                self.resolve_await_expression(argument)
            }
            RawTypeofExpression::BitwiseNot(expression) => {
                let argument = self.resolve(&expression.argument);
                Some(self.resolve_number_or_bigint_unary_expression(argument))
            }
            RawTypeofExpression::Call(expression) => {
                let callee = self.resolve(&expression.callee);
                Some(self.resolve_call_expression(callee, &expression.arguments))
            }
            RawTypeofExpression::Destructure(expression) => match &expression.destructure_field {
                RawDestructureField::Index(index) => {
                    let subject = self.resolve(&expression.ty);
                    self.resolve_element_type_at_index(subject, *index)
                }
                RawDestructureField::Name(_)
                | RawDestructureField::RestExcept(_)
                | RawDestructureField::RestFrom(_) => None,
            },
            RawTypeofExpression::Index(expression) => {
                let object = self.resolve(&expression.object);
                self.resolve_element_type_at_index(object, expression.index)
            }
            RawTypeofExpression::IterableValueOf(expression) => {
                let ty = self.resolve(&expression.ty);
                self.resolve_iterable_value_type(ty)
            }
            RawTypeofExpression::New(expression) => {
                let callee = self.resolve(&expression.callee);
                self.resolve_new_expression(callee, expression.arguments.len())
            }
            RawTypeofExpression::StaticMember(expression) => {
                let object = self.resolve(&expression.object);
                self.resolve_static_member_expression(object, expression.member.text())
            }
            RawTypeofExpression::Typeof(expression) => {
                let argument = self.resolve(&expression.argument);
                Some(self.resolve_typeof_operator(argument))
            }
            RawTypeofExpression::UnaryMinus(expression) => {
                let argument = self.resolve(&expression.argument);
                Some(self.resolve_number_or_bigint_unary_expression(argument))
            }
            RawTypeofExpression::Conditional(_)
            | RawTypeofExpression::LogicalAnd(_)
            | RawTypeofExpression::LogicalOr(_)
            | RawTypeofExpression::NullishCoalescing(_)
            | RawTypeofExpression::Super(_)
            | RawTypeofExpression::This(_) => None,
        }
    }

    fn resolve_call_expression(
        &mut self,
        callee: InferredTypeData<'db>,
        arguments: &[RawCallArgumentType],
    ) -> InferredTypeData<'db> {
        let args = arguments
            .iter()
            .map(|argument| match argument {
                RawCallArgumentType::Argument(ty) | RawCallArgumentType::Spread(ty) => {
                    self.resolve(ty)
                }
            })
            .collect::<Vec<_>>();

        infer_call_expression_return_type(self.db, callee, &args)
    }

    fn resolve_new_expression(
        &mut self,
        callee: InferredTypeData<'db>,
        argument_count: usize,
    ) -> Option<InferredTypeData<'db>> {
        let callee = self.resolve_inferred_type(callee);
        let InferredTypeData::Class(class) = callee else {
            return None;
        };

        let constructed_ty = class
            .members(self.db)
            .iter()
            .filter(|member| member.kind.is_constructor())
            .find_map(|member| match self.resolve_inferred_type(member.ty) {
                InferredTypeData::Constructor(constructor)
                    if constructor.accepts_argument_count(self.db, argument_count) =>
                {
                    constructor.return_type(self.db)
                }
                _ => None,
            })
            .unwrap_or(callee);

        Some(InferredTypeData::instance_of(
            self.db,
            constructed_ty,
            Box::default(),
        ))
    }

    fn resolve_await_expression(
        &mut self,
        argument: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        match self.resolve_inferred_type(argument) {
            ty @ (InferredTypeData::BigInt
            | InferredTypeData::Boolean
            | InferredTypeData::Class(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Literal(_)
            | InferredTypeData::Null
            | InferredTypeData::Number
            | InferredTypeData::Object(_)
            | InferredTypeData::String) => Some(ty),
            InferredTypeData::InstanceOf(instance)
                if self
                    .resolve_inferred_type(instance.ty(self.db))
                    .is_promise_class(self.db) =>
            {
                instance
                    .type_parameters(self.db)
                    .first()
                    .map(|ty| self.resolve_inferred_type(*ty))
            }
            _ => None,
        }
    }

    fn resolve_static_member_expression(
        &mut self,
        object: InferredTypeData<'db>,
        member_name: &str,
    ) -> Option<InferredTypeData<'db>> {
        match self.resolve_inferred_type(object) {
            InferredTypeData::Class(class) => find_member_in_members_for_mode(
                self.db,
                class.members(self.db),
                member_name,
                StaticMemberMode::Class,
            )
            .map(|(ty, is_optional)| self.member_type(ty, is_optional)),
            InferredTypeData::InstanceOf(instance) => {
                let target = self.resolve_inferred_type(instance.ty(self.db));
                let substitutions = substitutions_for_instance(
                    self.db,
                    target,
                    instance.type_parameters(self.db),
                    &[],
                );
                self.find_static_member_on_resolved_type(target, member_name)
                    .map(|(ty, is_optional)| {
                        let ty = apply_substitutions(self.db, ty, &substitutions);
                        self.member_type(ty, is_optional)
                    })
            }
            InferredTypeData::Union(union) => {
                let mut types = Vec::new();
                for ty in union.types(self.db) {
                    match self.resolve_inferred_type(*ty) {
                        InferredTypeData::Undefined => {}
                        InferredTypeData::Unknown => types.push(InferredTypeData::Unknown),
                        ty => {
                            if let Some((member_ty, is_optional)) =
                                self.find_static_member_on_resolved_type(ty, member_name)
                            {
                                types.push(self.member_type(member_ty, is_optional));
                            }
                        }
                    }
                }
                collected_type_result(self.db, types).or(Some(InferredTypeData::Unknown))
            }
            ty => self
                .find_static_member_on_resolved_type(ty, member_name)
                .map(|(ty, is_optional)| self.member_type(ty, is_optional)),
        }
    }

    fn find_static_member_on_resolved_type(
        &mut self,
        ty: InferredTypeData<'db>,
        member_name: &str,
    ) -> Option<(InferredTypeData<'db>, bool)> {
        match ty {
            InferredTypeData::Class(class) => find_member_in_members_for_mode(
                self.db,
                class.members(self.db),
                member_name,
                StaticMemberMode::Instance,
            ),
            InferredTypeData::Interface(interface) => find_member_in_members_for_mode(
                self.db,
                interface.members(self.db),
                member_name,
                StaticMemberMode::Instance,
            ),
            InferredTypeData::Literal(literal) => match literal.literal(self.db) {
                InferredLiteral::Object(members) => find_member_in_members_for_mode(
                    self.db,
                    members,
                    member_name,
                    StaticMemberMode::Instance,
                ),
                _ => None,
            },
            InferredTypeData::Module(module) => find_member_in_members_for_mode(
                self.db,
                module.members(self.db),
                member_name,
                StaticMemberMode::Instance,
            ),
            InferredTypeData::Namespace(namespace) => find_member_in_members_for_mode(
                self.db,
                namespace.members(self.db),
                member_name,
                StaticMemberMode::Instance,
            ),
            InferredTypeData::Object(object) => find_member_in_members_for_mode(
                self.db,
                object.members(self.db),
                member_name,
                StaticMemberMode::Instance,
            ),
            _ => None,
        }
    }

    fn member_type(
        &mut self,
        ty: InferredTypeData<'db>,
        is_optional: bool,
    ) -> InferredTypeData<'db> {
        if is_optional {
            InferredTypeData::union_from_types(
                self.db,
                Vec::from([ty, InferredTypeData::Undefined]),
            )
        } else {
            self.resolve_inferred_type(ty)
        }
    }

    fn resolve_element_type_at_index(
        &mut self,
        subject: InferredTypeData<'db>,
        index: usize,
    ) -> Option<InferredTypeData<'db>> {
        match self.resolve_inferred_type(subject) {
            InferredTypeData::Tuple(tuple) => {
                let element = tuple.elements(self.db).get(index)?;
                Some(self.optional_element_type(element.ty, element.is_optional || element.is_rest))
            }
            InferredTypeData::InstanceOf(instance)
                if self
                    .resolve_inferred_type(instance.ty(self.db))
                    .is_array_class(self.db) =>
            {
                instance
                    .type_parameters(self.db)
                    .first()
                    .map(|ty| self.optional_element_type(*ty, true))
            }
            _ => None,
        }
    }

    fn resolve_iterable_value_type(
        &mut self,
        subject: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        let subject = self.resolve_inferred_type(subject);
        let InferredTypeData::InstanceOf(instance) = subject else {
            return None;
        };
        self.resolve_inferred_type(instance.ty(self.db))
            .is_array_class(self.db)
            .then(|| instance.type_parameters(self.db).first().copied())
            .flatten()
    }

    fn optional_element_type(
        &mut self,
        ty: InferredTypeData<'db>,
        is_optional: bool,
    ) -> InferredTypeData<'db> {
        let ty = self.resolve_inferred_type(ty);
        if is_optional {
            InferredTypeData::union_from_types(
                self.db,
                Vec::from([ty, InferredTypeData::Undefined]),
            )
        } else {
            ty
        }
    }

    fn resolve_addition_expression(
        &mut self,
        left: InferredTypeData<'db>,
        right: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        match (
            self.coerced_addition_operand_type(left),
            self.coerced_addition_operand_type(right),
        ) {
            (Some(InferredTypeData::BigInt), Some(InferredTypeData::BigInt)) => {
                Some(InferredTypeData::BigInt)
            }
            (Some(InferredTypeData::Number), Some(InferredTypeData::Number)) => {
                Some(InferredTypeData::Number)
            }
            (Some(InferredTypeData::String), _) | (_, Some(InferredTypeData::String)) => {
                Some(InferredTypeData::String)
            }
            (Some(InferredTypeData::Unknown), Some(InferredTypeData::Unknown)) => {
                Some(InferredTypeData::Unknown)
            }
            _ => None,
        }
    }

    fn coerced_addition_operand_type(
        &mut self,
        ty: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        match self.resolve_inferred_type(ty) {
            InferredTypeData::BigInt => Some(InferredTypeData::BigInt),
            InferredTypeData::Boolean
            | InferredTypeData::Null
            | InferredTypeData::Number
            | InferredTypeData::Undefined => Some(InferredTypeData::Number),
            InferredTypeData::Class(_)
            | InferredTypeData::InstanceOf(_)
            | InferredTypeData::Interface(_)
            | InferredTypeData::Object(_)
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::String => Some(InferredTypeData::String),
            InferredTypeData::Literal(literal) => match literal.literal(self.db) {
                InferredLiteral::BigInt(_) => Some(InferredTypeData::BigInt),
                InferredLiteral::Boolean(_) | InferredLiteral::Number(_) => {
                    Some(InferredTypeData::Number)
                }
                InferredLiteral::Object(_)
                | InferredLiteral::RegExp(_)
                | InferredLiteral::String(_)
                | InferredLiteral::Template(_) => Some(InferredTypeData::String),
            },
            InferredTypeData::Unknown => Some(InferredTypeData::Unknown),
            _ => None,
        }
    }

    fn resolve_number_or_bigint_unary_expression(
        &mut self,
        argument: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        match self.resolve_inferred_type(argument) {
            InferredTypeData::BigInt => InferredTypeData::BigInt,
            _ => InferredTypeData::Number,
        }
    }

    fn resolve_typeof_operator(
        &mut self,
        argument: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        match self.resolve_inferred_type(argument) {
            InferredTypeData::BigInt => self.typeof_string_literal("bigint"),
            InferredTypeData::Boolean => self.typeof_string_literal("boolean"),
            InferredTypeData::Function(_) => self.typeof_string_literal("function"),
            InferredTypeData::Literal(literal) => match literal.literal(self.db) {
                InferredLiteral::BigInt(_) => self.typeof_string_literal("bigint"),
                InferredLiteral::Boolean(_) => self.typeof_string_literal("boolean"),
                InferredLiteral::Object(_) | InferredLiteral::RegExp(_) => {
                    self.typeof_string_literal("object")
                }
                InferredLiteral::Number(_) => self.typeof_string_literal("number"),
                InferredLiteral::String(_) | InferredLiteral::Template(_) => {
                    self.typeof_string_literal("string")
                }
            },
            InferredTypeData::Null => self.typeof_string_literal("object"),
            InferredTypeData::Number => self.typeof_string_literal("number"),
            InferredTypeData::Object(_) | InferredTypeData::Tuple(_) => {
                self.typeof_string_literal("object")
            }
            InferredTypeData::String => self.typeof_string_literal("string"),
            InferredTypeData::Symbol => self.typeof_string_literal("symbol"),
            InferredTypeData::Undefined => self.typeof_string_literal("undefined"),
            _ => self.typeof_return_union(),
        }
    }

    fn typeof_return_union(&self) -> InferredTypeData<'db> {
        InferredTypeData::union_from_types(
            self.db,
            [
                "bigint",
                "boolean",
                "function",
                "number",
                "object",
                "string",
                "symbol",
                "undefined",
            ]
            .into_iter()
            .map(|value| self.typeof_string_literal(value))
            .collect(),
        )
    }

    fn typeof_string_literal(&self, value: &'static str) -> InferredTypeData<'db> {
        // TODO: Replace this with canonical `global_types(db)` literal entries in Phase 6e.
        InferredTypeData::Literal(InferredInternedLiteral::new(
            self.db,
            InferredLiteral::String(Text::new_static(value).into()),
        ))
    }

    fn resolve_qualifier(&mut self, qualifier: &TypeReferenceQualifier) -> InferredTypeData<'db> {
        let Some(identifier) = qualifier.path.iter().next() else {
            return InferredTypeData::Unknown;
        };

        let mut scope = self
            .js_info
            .semantic_model
            .scope_from_id(qualifier.scope_id);
        let mut reached_root_scope = false;
        for _ in 0..MAX_SCOPE_RESOLUTION_STEPS {
            if let Some(binding) = scope.get_binding(identifier.text()) {
                if binding.is_imported()
                    && let Some(import) = self.js_info.static_imports.get(identifier.text())
                {
                    let target = self.resolve_import(&TypeImportQualifier {
                        symbol: import.symbol.clone(),
                        resolved_path: import.resolved_path.clone(),
                        type_only: qualifier.type_only,
                    });
                    return self.apply_qualifier_type_parameters(target, qualifier);
                }

                let target = self
                    .js_info
                    .raw_binding_types
                    .get(&binding.syntax().text_trimmed_range())
                    .cloned()
                    .map_or(InferredTypeData::Unknown, |reference| {
                        self.resolve(&reference)
                    });
                return self.apply_qualifier_type_parameters(target, qualifier);
            }

            match scope.parent() {
                Some(parent) => scope = parent,
                None => {
                    reached_root_scope = true;
                    break;
                }
            }
        }

        if !reached_root_scope {
            return InferredTypeData::Unknown;
        }

        if qualifier.is_record() && qualifier.type_parameters.len() == 2 {
            let key_ty = self.resolve(&qualifier.type_parameters[0]);
            let value_ty = self.resolve(&qualifier.type_parameters[1]);
            return InferredTypeData::object_from_members(
                self.db,
                Vec::from([InferredTypeMember {
                    kind: InferredTypeMemberKind::IndexSignature(key_ty),
                    ty: value_ty,
                }]),
            );
        }

        if (qualifier.is_pick() || qualifier.is_omit()) && qualifier.type_parameters.len() == 2 {
            return self.resolve_pick_or_omit(qualifier);
        }

        if (qualifier.is_partial() || qualifier.is_required())
            && qualifier.type_parameters.len() == 1
        {
            return self.resolve_partial_or_required(qualifier);
        }

        if qualifier.is_readonly() && qualifier.type_parameters.len() == 1 {
            return self.resolve_readonly(qualifier);
        }

        if qualifier.is_array() && qualifier.has_known_type_parameters() {
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

        if qualifier.is_map() && qualifier.has_known_type_parameters() {
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

        if qualifier.is_promise() && qualifier.has_known_type_parameters() {
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

        if qualifier.is_set() && qualifier.has_known_type_parameters() {
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

        if qualifier.is_weak_map() && qualifier.has_known_type_parameters() {
            return InferredTypeData::weak_map_instance(
                self.db,
                qualifier
                    .type_parameters
                    .iter()
                    .map(|parameter| self.resolve(parameter))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            );
        }

        if let Some(ty) = self.resolve_global_member_qualifier(qualifier) {
            return ty;
        }

        if let Some(resolved_id) = GLOBAL_RESOLVER.resolve_qualifier(qualifier) {
            return self.resolve_resolved_id(resolved_id);
        }

        InferredTypeData::Unknown
    }

    fn resolve_global_member_qualifier(
        &mut self,
        qualifier: &TypeReferenceQualifier,
    ) -> Option<InferredTypeData<'db>> {
        let mut parts = qualifier.path.iter();
        let first = parts.next()?;
        let mut target = parts.next().and_then(|member| {
            let base = GLOBAL_RESOLVER
                .resolve_qualifier(&TypeReferenceQualifier {
                    path: Path::from(first.clone()),
                    type_parameters: Box::default(),
                    scope_id: qualifier.scope_id,
                    type_only: qualifier.type_only,
                    excluded_binding_id: qualifier.excluded_binding_id,
                })
                .map(|resolved_id| self.resolve_resolved_id(resolved_id))?;
            self.resolve_static_member_expression(base, member.text())
        })?;

        for member in parts {
            target = self.resolve_static_member_expression(target, member.text())?;
        }

        Some(self.apply_qualifier_type_parameters(target, qualifier))
    }

    fn apply_qualifier_type_parameters(
        &mut self,
        target: InferredTypeData<'db>,
        qualifier: &TypeReferenceQualifier,
    ) -> InferredTypeData<'db> {
        if qualifier.type_parameters.is_empty() {
            return target;
        }

        let Some(declared_parameters) = self.declared_type_parameters(target) else {
            return target;
        };

        let incoming_parameters = qualifier
            .type_parameters
            .iter()
            .map(|parameter| self.resolve(parameter))
            .collect::<Vec<_>>();
        let merged_parameters = declared_parameters
            .iter()
            .enumerate()
            .map(|(index, parameter)| {
                incoming_parameters
                    .get(index)
                    .copied()
                    .unwrap_or(*parameter)
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        InferredTypeData::instance_of(self.db, target, merged_parameters)
    }

    fn declared_type_parameters(
        &mut self,
        target: InferredTypeData<'db>,
    ) -> Option<Box<[InferredTypeData<'db>]>> {
        match self.resolve_inferred_type(target) {
            InferredTypeData::Class(class) => Some(class.type_parameters(self.db).to_vec().into()),
            InferredTypeData::Function(function) => {
                Some(function.type_parameters(self.db).to_vec().into())
            }
            InferredTypeData::InstanceOf(instance) => {
                Some(instance.type_parameters(self.db).to_vec().into())
            }
            InferredTypeData::Interface(interface) => {
                Some(interface.type_parameters(self.db).to_vec().into())
            }
            _ => None,
        }
    }

    fn resolve_pick_or_omit(
        &mut self,
        qualifier: &TypeReferenceQualifier,
    ) -> InferredTypeData<'db> {
        let target_ty = self.resolve(&qualifier.type_parameters[0]);
        let key_ty = self.resolve(&qualifier.type_parameters[1]);
        let Some(key_names) = self.string_literal_keys(key_ty) else {
            return InferredTypeData::Unknown;
        };
        let Some(members) = self.own_members(target_ty) else {
            return InferredTypeData::Unknown;
        };

        if qualifier.is_pick() {
            InferredTypeData::pick_members(self.db, members, &key_names)
        } else {
            InferredTypeData::omit_members(self.db, members, &key_names)
        }
    }

    fn resolve_partial_or_required(
        &mut self,
        qualifier: &TypeReferenceQualifier,
    ) -> InferredTypeData<'db> {
        let target_ty = self.resolve(&qualifier.type_parameters[0]);
        let Some(members) = self.own_members(target_ty) else {
            return InferredTypeData::Unknown;
        };

        if qualifier.is_partial() {
            InferredTypeData::with_all_optional_members(self.db, members)
        } else {
            InferredTypeData::with_all_required_members(self.db, members)
        }
    }

    fn resolve_readonly(&mut self, qualifier: &TypeReferenceQualifier) -> InferredTypeData<'db> {
        let target_ty = self.resolve(&qualifier.type_parameters[0]);
        self.own_members(target_ty)
            .map_or(InferredTypeData::Unknown, |members| {
                InferredTypeData::object_from_members(self.db, members)
            })
    }

    fn own_members(&mut self, ty: InferredTypeData<'db>) -> Option<Vec<InferredTypeMember<'db>>> {
        let mut ty = ty;

        for _ in 0..MAX_LOCAL_TYPE_RESOLUTION_STEPS {
            match self.resolve_inferred_type(ty) {
                InferredTypeData::Class(class) => return Some(class.members(self.db).to_vec()),
                InferredTypeData::Interface(interface) => {
                    return Some(interface.members(self.db).to_vec());
                }
                InferredTypeData::InstanceOf(instance) => ty = instance.ty(self.db),
                InferredTypeData::Literal(literal) => match literal.literal(self.db) {
                    InferredLiteral::Object(members) => return Some(members.to_vec()),
                    _ => return None,
                },
                InferredTypeData::Module(module) => return Some(module.members(self.db).to_vec()),
                InferredTypeData::Namespace(namespace) => {
                    return Some(namespace.members(self.db).to_vec());
                }
                InferredTypeData::Object(object) => return Some(object.members(self.db).to_vec()),
                _ => return None,
            }
        }

        None
    }

    fn string_literal_keys(&mut self, ty: InferredTypeData<'db>) -> Option<Vec<Text>> {
        match self.resolve_inferred_type(ty) {
            InferredTypeData::Literal(literal) => match literal.literal(self.db) {
                InferredLiteral::String(value) => Some(vec![value.as_ref().clone()]),
                _ => None,
            },
            InferredTypeData::Union(union) => Some(
                union
                    .types(self.db)
                    .to_vec()
                    .into_iter()
                    .filter_map(|ty| self.string_literal_key(ty))
                    .collect(),
            ),
            _ => None,
        }
    }

    fn string_literal_key(&mut self, ty: InferredTypeData<'db>) -> Option<Text> {
        match self.resolve_inferred_type(ty) {
            InferredTypeData::Literal(literal) => match literal.literal(self.db) {
                InferredLiteral::String(value) => Some(value.as_ref().clone()),
                _ => None,
            },
            _ => None,
        }
    }

    fn resolve_inferred_type(&mut self, mut ty: InferredTypeData<'db>) -> InferredTypeData<'db> {
        let mut seen = FxHashSet::default();

        for _ in 0..MAX_LOCAL_TYPE_RESOLUTION_STEPS {
            let InferredTypeData::Local(local) = ty else {
                return ty;
            };

            let module_key = local.module(self.db);
            let local_type_id = local.type_id(self.db);
            if !seen.insert((module_key, local_type_id)) {
                return ty;
            }

            ty = if module_key == self.module_key {
                self.resolve_raw_type_id(TypeId::new(local_type_id.index()))
            } else {
                module_for_key(self.db, module_key)
                    .and_then(|module| infer_module_types(self.db, module))
                    .and_then(|types| types.types.get(local_type_id.index()).copied())
                    .unwrap_or(InferredTypeData::Unknown)
            };
        }

        ty
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
        // Shared across direct stack pops and blanket reexport scans.
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
            let mut resolved_globals = FxHashMap::default();
            resolve_global_type_id(db, resolved_id.id(), &mut resolved_globals)
        }
        TypeResolverLevel::Full | TypeResolverLevel::Import => InferredTypeData::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::borrow::Cow;

    use biome_db::ParsedSource;
    use biome_js_semantic::ScopeId;
    use biome_js_syntax::AnyJsExpression;
    use biome_js_type_info::{GlobalsResolver, ResolvedTypeData};
    use camino::Utf8Path;
    use salsa::Storage;

    #[salsa::db]
    #[derive(Default)]
    struct TestDb {
        storage: Storage<Self>,
    }

    #[salsa::db]
    impl salsa::Database for TestDb {}

    #[salsa::db]
    impl biome_db::Db for TestDb {
        fn parsed_source_for_path(&self, _path: &Utf8Path) -> Option<ParsedSource> {
            None
        }
    }

    #[salsa::db]
    impl biome_js_type_info::TypeDb for TestDb {}

    #[salsa::db]
    impl ModuleDb for TestDb {
        fn module_for_path(&self, _path: &Utf8Path) -> Option<ModuleInfo> {
            None
        }

        fn for_each_module(&self, _f: &mut dyn FnMut(&Utf8Path, &ModuleInfoKind)) {}
    }

    struct NoopTypeResolver;

    impl TypeResolver for NoopTypeResolver {
        fn level(&self) -> TypeResolverLevel {
            TypeResolverLevel::Global
        }

        fn find_type(&self, _type_data: &RawTypeData) -> Option<TypeId> {
            None
        }

        fn get_by_id(&self, _id: TypeId) -> &RawTypeData {
            panic!("noop resolver must not resolve type IDs");
        }

        fn get_by_resolved_id(&self, _id: ResolvedTypeId) -> Option<ResolvedTypeData<'_>> {
            None
        }

        fn register_type(&mut self, _type_data: Cow<RawTypeData>) -> TypeId {
            panic!("noop resolver must not register types");
        }

        fn resolve_reference(&self, _ty: &TypeReference) -> Option<ResolvedTypeId> {
            None
        }

        fn resolve_qualifier(&self, _qualifier: &TypeReferenceQualifier) -> Option<ResolvedTypeId> {
            None
        }

        fn resolve_type_of(
            &self,
            _identifier: &Text,
            _scope_id: ScopeId,
        ) -> Option<ResolvedTypeId> {
            None
        }

        fn resolve_expression(
            &mut self,
            _scope_id: ScopeId,
            _expr: &AnyJsExpression,
        ) -> Cow<'_, RawTypeData> {
            Cow::Owned(RawTypeData::Unknown)
        }

        fn registered_types(&self) -> Vec<&RawTypeData> {
            Vec::new()
        }
    }

    fn global_type_id(resolver: &GlobalsResolver, name: &'static str) -> TypeId {
        resolver
            .resolve_qualifier(&TypeReferenceQualifier::from_path(
                ScopeId::GLOBAL,
                Text::new_static(name),
            ))
            .unwrap_or_else(|| panic!("{name} must resolve as a global type"))
            .id()
    }

    fn global_type_ref(type_id: TypeId) -> TypeReference {
        ResolvedTypeId::new(TypeResolverLevel::Global, type_id).into()
    }

    fn is_class_named(db: &dyn ModuleDb, ty: InferredTypeData, name: &str) -> bool {
        matches!(ty, InferredTypeData::Class(class) if class.name(db).as_ref().is_some_and(|class_name| class_name.text() == name))
    }

    #[test]
    fn global_converter_normalizes_compound_rebuilds() {
        let db = TestDb::default();
        let mut resolver = GlobalsResolver::default();
        let unresolved = NoopTypeResolver;
        let promise_id = global_type_id(&resolver, "Promise");
        let array_id = global_type_id(&resolver, "Array");

        let inner_union = RawTypeData::union_of(
            &unresolved,
            Box::new([global_type_ref(promise_id), global_type_ref(array_id)]),
        );
        let inner_union_id = resolver.register_type(Cow::Owned(inner_union));
        let outer_union = RawTypeData::union_of(
            &unresolved,
            Box::new([global_type_ref(inner_union_id), global_type_ref(promise_id)]),
        );
        let outer_union_id = resolver.register_type(Cow::Owned(outer_union));
        let union_ty = resolve_global_type_id_with_resolver(
            &db,
            &resolver,
            outer_union_id,
            &mut FxHashMap::default(),
        );
        let InferredTypeData::Union(union) = union_ty else {
            panic!("outer union must stay a union, got {union_ty:?}");
        };
        assert_eq!(union.types(&db).len(), 2);
        assert!(
            union
                .types(&db)
                .iter()
                .all(|ty| !matches!(ty, InferredTypeData::Union(_)))
        );
        assert!(
            union
                .types(&db)
                .iter()
                .any(|ty| is_class_named(&db, *ty, "Promise"))
        );
        assert!(
            union
                .types(&db)
                .iter()
                .any(|ty| is_class_named(&db, *ty, "Array"))
        );

        let inner_intersection = RawTypeData::intersection_of(Vec::from([
            global_type_ref(promise_id),
            global_type_ref(array_id),
        ]));
        let inner_intersection_id = resolver.register_type(Cow::Owned(inner_intersection));
        let outer_intersection = RawTypeData::intersection_of(Vec::from([
            global_type_ref(inner_intersection_id),
            global_type_ref(promise_id),
        ]));
        let outer_intersection_id = resolver.register_type(Cow::Owned(outer_intersection));
        let intersection_ty = resolve_global_type_id_with_resolver(
            &db,
            &resolver,
            outer_intersection_id,
            &mut FxHashMap::default(),
        );
        match intersection_ty {
            InferredTypeData::Intersection(intersection) => {
                assert_eq!(intersection.types(&db).len(), 2);
                assert!(
                    intersection
                        .types(&db)
                        .iter()
                        .all(|ty| !matches!(ty, InferredTypeData::Intersection(_)))
                );
                assert!(
                    intersection
                        .types(&db)
                        .iter()
                        .any(|ty| is_class_named(&db, *ty, "Promise"))
                );
                assert!(
                    intersection
                        .types(&db)
                        .iter()
                        .any(|ty| is_class_named(&db, *ty, "Array"))
                );
            }
            InferredTypeData::Object(_) => {}
            _ => panic!(
                "outer intersection must become a normalized intersection or object, got {intersection_ty:?}"
            ),
        }
    }

    #[test]
    fn global_converter_does_not_memoize_types_built_under_active_ancestor() {
        let db = TestDb::default();
        let resolver = GlobalsResolver::default();
        let promise_id = global_type_id(&resolver, "Promise");
        let mut resolved_globals = FxHashMap::default();

        let promise_ty =
            resolve_global_type_id_with_resolver(&db, &resolver, promise_id, &mut resolved_globals);

        assert!(is_class_named(&db, promise_ty, "Promise"));
        assert_eq!(resolved_globals.len(), 1);
        assert_eq!(resolved_globals.get(&promise_id), Some(&promise_ty));
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "global type converter expected type on the value stack")]
    fn global_converter_value_mismatch_panics_in_debug() {
        let mut values = vec![GlobalTypeValue::ReturnType(InferredReturnType::Type(
            InferredTypeData::Unknown,
        ))];

        let _ = pop_global_type(&mut values);
    }
}
