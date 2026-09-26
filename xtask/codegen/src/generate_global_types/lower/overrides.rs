//! Hand-written projections for globals whose shape type inference depends on.
//!
//! Type inference and lint rules rely on the reduced shapes of `Array`, `Promise`,
//! `Error`, `Disposable`, and `AsyncDisposable`, including synthetic helper globals such
//! as `Array.prototype.filter`. Each override validates the declarations it projects so
//! that a TypeScript change which breaks the projection fails codegen. Every other global
//! is lowered from its declarations by the generic lowerer.

use biome_js_syntax::{JsFormalParameter, TsFunctionType};
use biome_rowan::SyntaxResult;

use super::generic::GenericLowerer;
use super::ids::{GlobalIds, GlobalSlotKind};
use super::*;

/// Lowers every override whose declarations are present. The first global of each
/// override replaces the generic lowering of the same identity; the rest are helpers.
pub(super) fn lower_overrides<'a>(
    manifest: &GlobalManifest,
    ids: &'a GlobalIds,
    lowerer: &mut GenericLowerer<'a>,
) -> Result<Vec<LoweredGlobal>> {
    let mut globals = Vec::new();
    lower_array_globals(manifest, ids, lowerer, &mut globals)?;
    lower_promise_globals(manifest, &mut lowerer.sources, &mut globals)?;
    lower_error_globals(manifest, &mut lowerer.sources, &mut globals)?;
    lower_disposable_global(
        manifest,
        ids,
        &mut lowerer.sources,
        &mut globals,
        DISPOSABLE_GLOBAL,
    )?;
    lower_disposable_global(
        manifest,
        ids,
        &mut lowerer.sources,
        &mut globals,
        ASYNC_DISPOSABLE_GLOBAL,
    )?;
    Ok(globals)
}

/// Lowers `Error` and its constructor and call helpers when present.
fn lower_error_globals(
    manifest: &GlobalManifest,
    source_cache: &mut ParsedSourceCache,
    globals: &mut Vec<LoweredGlobal>,
) -> Result<()> {
    let Some(error_group) = manifest.global_group("Error") else {
        return Ok(());
    };
    if !error_group.has_role(GlobalDeclarationRole::Type) {
        bail!("Error global must have a type-side declaration");
    }
    if !error_group.has_role(GlobalDeclarationRole::Value) {
        bail!("Error global must have a value-side declaration");
    }
    ensure_error_value_references_constructor(error_group.declarations(), source_cache)?;

    let Some(error_constructor_group) = manifest.global_group("ErrorConstructor") else {
        bail!("Error global value side references missing ErrorConstructor group");
    };
    if !error_constructor_group.has_role(GlobalDeclarationRole::Type) {
        bail!("ErrorConstructor must have a type-side declaration");
    }

    let mut members = Vec::new();
    let mut saw_error_interface = false;
    for record in error_group.declarations() {
        match &record.kind {
            DeclarationKind::Interface => {
                saw_error_interface = true;
            }
            DeclarationKind::TypeAlias => {
                bail!("type aliases are not supported in the Error global")
            }
            DeclarationKind::DeclareFunction
            | DeclarationKind::VariableDeclarator { .. }
            | DeclarationKind::ImportEquals => {
                continue;
            }
        }
        let declaration = source_cache
            .find_interface_declaration(record)?
            .with_context(|| {
                format!(
                    "failed to find interface declaration {} at {:?}",
                    record.declared_name.text(),
                    record.text_range
                )
            })?;
        lower_error_interface_members(&declaration, &mut members)?;
    }
    if !saw_error_interface {
        bail!("Error global must include an interface declaration");
    }

    let ErrorConstructorSignatures {
        constructor,
        call,
        prototype,
    } = lower_error_constructor_signatures(error_constructor_group.declarations(), source_cache)?;

    members.push(LoweredTypeMember {
        name: Text::from("constructor"),
        kind: LoweredMemberKind::Constructor,
        type_reference: LoweredTypeReference::Predefined("GLOBAL_ERROR_CONSTRUCT_ID"),
    });
    members.push(LoweredTypeMember {
        name: Text::from("call"),
        kind: LoweredMemberKind::CallSignature,
        type_reference: LoweredTypeReference::Predefined("GLOBAL_ERROR_CALL_ID"),
    });
    if let Some(prototype) = prototype {
        members.push(prototype);
    }

    globals.push(LoweredGlobal {
        roles: GlobalRoles::default(),
        local_types: Box::default(),
        name: Text::from("Error"),
        id_constant: "ERROR_ID_GLOBAL_TYPE_ID".into(),
        data: LoweredTypeData::Class(LoweredClass {
            extends: None,
            implements: Box::default(),
            name: Text::from("Error"),
            type_parameters: Box::default(),
            members: members.into_boxed_slice(),
        }),
    });
    globals.push(LoweredGlobal {
        roles: GlobalRoles::default(),
        local_types: Box::default(),
        name: Text::from("Error.constructor"),
        id_constant: "ERROR_CONSTRUCT_ID_GLOBAL_TYPE_ID".into(),
        data: LoweredTypeData::Constructor(constructor),
    });
    globals.push(LoweredGlobal {
        roles: GlobalRoles::default(),
        local_types: Box::default(),
        name: Text::from("Error.call"),
        id_constant: "ERROR_CALL_ID_GLOBAL_TYPE_ID".into(),
        data: LoweredTypeData::Function(call),
    });

    Ok(())
}

/// Lowered pieces extracted from `interface ErrorConstructor`.
struct ErrorConstructorSignatures {
    constructor: LoweredConstructor,
    call: LoweredFunction,
    prototype: Option<LoweredTypeMember>,
}

/// Describes how one disposable interface (`Disposable`/`AsyncDisposable`) and its dispose
/// helper are lowered. Every field is a static string because it feeds generated Rust source.
#[derive(Clone, Copy)]
struct DisposableGlobalSpec {
    /// Interface name in the `.d.ts` source and the manifest group key.
    interface_name: &'static str,
    /// `GlobalTypeId` constant the lowered interface registers into.
    global_id_constant: &'static str,
    /// Display name of the single computed member (e.g. `[Symbol.dispose]`).
    member_name: &'static str,
    /// `GLOBAL_*` reference the computed member key must resolve to.
    symbol_id: &'static str,
    /// Display name of the synthesized dispose helper global.
    helper_name: &'static str,
    /// `GlobalTypeId` constant the dispose helper registers into.
    helper_id_constant: &'static str,
    /// `GLOBAL_*` reference the member's value type points at (the helper).
    helper_type_id: &'static str,
    /// Whether the helper returns `void` or `PromiseLike<void>`.
    return_kind: DisposableReturnKind,
}

/// Return shape of a dispose helper, mapping the `.d.ts` signature to the lowered return type.
#[derive(Clone, Copy)]
enum DisposableReturnKind {
    Void,
    PromiseLikeVoid,
}

impl DisposableReturnKind {
    /// Whether the lowered dispose helper is an `async` function.
    fn helper_is_async(self) -> bool {
        matches!(self, Self::PromiseLikeVoid)
    }

    /// Predefined ID constant the lowered return type must resolve to.
    fn return_type_id(self) -> &'static str {
        match self {
            Self::Void => "GLOBAL_VOID_ID",
            Self::PromiseLikeVoid => "GLOBAL_INSTANCEOF_PROMISE_ID",
        }
    }
}

/// Lowering spec for the `Disposable` interface and its `[Symbol.dispose](): void` helper.
const DISPOSABLE_GLOBAL: DisposableGlobalSpec = DisposableGlobalSpec {
    interface_name: "Disposable",
    global_id_constant: "DISPOSABLE_ID_GLOBAL_TYPE_ID",
    member_name: "[Symbol.dispose]",
    symbol_id: "GLOBAL_SYMBOL_DISPOSE_ID",
    helper_name: "Disposable[Symbol.dispose]",
    helper_id_constant: "DISPOSABLE_DISPOSE_ID_GLOBAL_TYPE_ID",
    helper_type_id: "GLOBAL_DISPOSABLE_DISPOSE_ID",
    return_kind: DisposableReturnKind::Void,
};

/// Lowering spec for `AsyncDisposable` and its `[Symbol.asyncDispose](): PromiseLike<void>` helper.
const ASYNC_DISPOSABLE_GLOBAL: DisposableGlobalSpec = DisposableGlobalSpec {
    interface_name: "AsyncDisposable",
    global_id_constant: "ASYNC_DISPOSABLE_ID_GLOBAL_TYPE_ID",
    member_name: "[Symbol.asyncDispose]",
    symbol_id: "GLOBAL_SYMBOL_ASYNC_DISPOSE_ID",
    helper_name: "AsyncDisposable[Symbol.asyncDispose]",
    helper_id_constant: "ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID_GLOBAL_TYPE_ID",
    helper_type_id: "GLOBAL_ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID",
    return_kind: DisposableReturnKind::PromiseLikeVoid,
};

/// Array members retained in the resolver's reduced projection.
#[derive(Clone, Copy)]
enum SelectedArrayMember {
    Filter,
    ForEach,
    Length,
    Map,
}

impl SelectedArrayMember {
    /// Returns the TypeScript spelling used in diagnostics.
    fn name(self) -> &'static str {
        match self {
            Self::Filter => "filter",
            Self::ForEach => "forEach",
            Self::Length => "length",
            Self::Map => "map",
        }
    }
}

/// Return shape required from a selected Array callback declaration.
#[derive(Clone, Copy)]
enum ArrayCallbackReturn<'a> {
    Reference(&'a str),
    Unknown,
    Void,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum PromiseMemberLocation {
    Instance,
    Static,
}

/// Connects a selected TypeScript method to its predefined resolver helper.
#[derive(Clone, Copy)]
struct PromiseMethodSpecification {
    source_name: &'static str,
    global_name: &'static str,
    id_constant: &'static str,
    member_type_id: &'static str,
    location: PromiseMemberLocation,
}

const PROMISE_METHOD_COUNT: usize = 10;

const PROMISE_METHOD_SPECIFICATIONS: [PromiseMethodSpecification; PROMISE_METHOD_COUNT] = [
    PromiseMethodSpecification {
        source_name: "catch",
        global_name: "Promise.prototype.catch",
        id_constant: "PROMISE_CATCH_ID_GLOBAL_TYPE_ID",
        member_type_id: "GLOBAL_PROMISE_CATCH_ID",
        location: PromiseMemberLocation::Instance,
    },
    PromiseMethodSpecification {
        source_name: "finally",
        global_name: "Promise.prototype.finally",
        id_constant: "PROMISE_FINALLY_ID_GLOBAL_TYPE_ID",
        member_type_id: "GLOBAL_PROMISE_FINALLY_ID",
        location: PromiseMemberLocation::Instance,
    },
    PromiseMethodSpecification {
        source_name: "then",
        global_name: "Promise.prototype.then",
        id_constant: "PROMISE_THEN_ID_GLOBAL_TYPE_ID",
        member_type_id: "GLOBAL_PROMISE_THEN_ID",
        location: PromiseMemberLocation::Instance,
    },
    PromiseMethodSpecification {
        source_name: "all",
        global_name: "Promise.all",
        id_constant: "PROMISE_ALL_ID_GLOBAL_TYPE_ID",
        member_type_id: "GLOBAL_PROMISE_ALL_ID",
        location: PromiseMemberLocation::Static,
    },
    PromiseMethodSpecification {
        source_name: "allSettled",
        global_name: "Promise.allSettled",
        id_constant: "PROMISE_ALL_SETTLED_ID_GLOBAL_TYPE_ID",
        member_type_id: "GLOBAL_PROMISE_ALL_SETTLED_ID",
        location: PromiseMemberLocation::Static,
    },
    PromiseMethodSpecification {
        source_name: "any",
        global_name: "Promise.any",
        id_constant: "PROMISE_ANY_ID_GLOBAL_TYPE_ID",
        member_type_id: "GLOBAL_PROMISE_ANY_ID",
        location: PromiseMemberLocation::Static,
    },
    PromiseMethodSpecification {
        source_name: "race",
        global_name: "Promise.race",
        id_constant: "PROMISE_RACE_ID_GLOBAL_TYPE_ID",
        member_type_id: "GLOBAL_PROMISE_RACE_ID",
        location: PromiseMemberLocation::Static,
    },
    PromiseMethodSpecification {
        source_name: "reject",
        global_name: "Promise.reject",
        id_constant: "PROMISE_REJECT_ID_GLOBAL_TYPE_ID",
        member_type_id: "GLOBAL_PROMISE_REJECT_ID",
        location: PromiseMemberLocation::Static,
    },
    PromiseMethodSpecification {
        source_name: "resolve",
        global_name: "Promise.resolve",
        id_constant: "PROMISE_RESOLVE_ID_GLOBAL_TYPE_ID",
        member_type_id: "GLOBAL_PROMISE_RESOLVE_ID",
        location: PromiseMemberLocation::Static,
    },
    PromiseMethodSpecification {
        source_name: "try",
        global_name: "Promise.try",
        id_constant: "PROMISE_TRY_ID_GLOBAL_TYPE_ID",
        member_type_id: "GLOBAL_PROMISE_TRY_ID",
        location: PromiseMemberLocation::Static,
    },
];

/// Validates selected members across merged declarations and builds the resolver projection.
///
/// Static members and constructor signatures are lowered from `ArrayConstructor`.
fn lower_array_globals<'a>(
    manifest: &GlobalManifest,
    ids: &'a GlobalIds,
    lowerer: &mut GenericLowerer<'a>,
    globals: &mut Vec<LoweredGlobal>,
) -> Result<()> {
    let Some(array_group) = manifest.global_group("Array") else {
        return Ok(());
    };
    let source_cache = &mut lowerer.sources;
    if !array_group.has_role(GlobalDeclarationRole::Type) {
        bail!("Array global must have a type-side declaration");
    }

    let mut saw_interface = false;
    let mut saw_filter = false;
    let mut saw_for_each = false;
    let mut saw_length = false;
    let mut saw_map = false;

    for record in array_group.declarations() {
        match &record.kind {
            DeclarationKind::Interface => {
                saw_interface = true;
            }
            DeclarationKind::TypeAlias => {
                bail!("type aliases are not supported in the Array global")
            }
            DeclarationKind::VariableDeclarator { .. } => continue,
            DeclarationKind::DeclareFunction | DeclarationKind::ImportEquals => {
                bail!("unsupported value-side Array declaration")
            }
        }

        let declaration = source_cache
            .find_interface_declaration(record)?
            .with_context(|| {
                format!(
                    "failed to find interface declaration {} at {:?}",
                    record.declared_name.text(),
                    record.text_range
                )
            })?;
        let array_type_parameter = validate_array_interface_type_parameter(&declaration)?;

        for member in declaration.members() {
            match member {
                AnyTsTypeMember::TsMethodSignatureTypeMember(method) => {
                    let Some(selected) = selected_array_member(method.name()?)? else {
                        continue;
                    };
                    match selected {
                        SelectedArrayMember::Filter => {
                            if method.type_parameters().is_some() {
                                continue;
                            }
                            if saw_filter {
                                bail!("Array has multiple non-generic filter overloads");
                            }
                            validate_array_filter(&method, array_type_parameter.text())?;
                            saw_filter = true;
                        }
                        SelectedArrayMember::ForEach => {
                            if saw_for_each {
                                bail!("Array has multiple forEach methods");
                            }
                            validate_array_for_each(&method, array_type_parameter.text())?;
                            saw_for_each = true;
                        }
                        SelectedArrayMember::Length => {
                            bail!("Array.length must be a property")
                        }
                        SelectedArrayMember::Map => {
                            if saw_map {
                                bail!("Array has multiple map methods");
                            }
                            validate_array_map(&method, array_type_parameter.text())?;
                            saw_map = true;
                        }
                    }
                }
                AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
                    let Some(selected) = selected_array_member(property.name()?)? else {
                        continue;
                    };
                    if !matches!(selected, SelectedArrayMember::Length) {
                        bail!("Array.{} must be a method", selected.name());
                    }
                    if saw_length {
                        bail!("Array has multiple length properties");
                    }
                    validate_array_length(&property)?;
                    saw_length = true;
                }
                AnyTsTypeMember::TsGetterSignatureTypeMember(getter) => {
                    reject_selected_array_accessor(getter.name()?)?;
                }
                AnyTsTypeMember::TsSetterSignatureTypeMember(setter) => {
                    reject_selected_array_accessor(setter.name()?)?;
                }
                AnyTsTypeMember::JsBogusMember(_)
                | AnyTsTypeMember::JsMetavariable(_)
                | AnyTsTypeMember::TsCallSignatureTypeMember(_)
                | AnyTsTypeMember::TsConstructSignatureTypeMember(_)
                | AnyTsTypeMember::TsIndexSignatureTypeMember(_) => {}
            }
        }
    }

    if !saw_interface {
        bail!("Array global must include an interface declaration");
    }
    if !saw_filter {
        bail!("Array is missing a non-generic filter overload");
    }
    if !saw_for_each {
        bail!("Array is missing forEach");
    }
    if !saw_length {
        bail!("Array is missing length");
    }
    if !saw_map {
        bail!("Array is missing map");
    }

    let (mut class, local_types) = match ids.type_slot("Array").map(|index| ids.slot(index)) {
        Some(slot) if matches!(slot.kind(), GlobalSlotKind::Class { .. }) => {
            lowerer.lower_class_statics(slot)?
        }
        _ => (
            LoweredClass {
                name: Text::from("Array"),
                type_parameters: Box::default(),
                extends: None,
                implements: Box::default(),
                members: Box::default(),
            },
            Box::default(),
        ),
    };
    // The projection's member types refer to the predefined `T` rather than a local
    // type parameter, so the class must declare that same `T`.
    class.type_parameters = Box::new([LoweredTypeReference::Predefined("GLOBAL_T_ID")]);
    class.members = [
        ("filter", "GLOBAL_ARRAY_FILTER_ID"),
        ("forEach", "GLOBAL_ARRAY_FOREACH_ID"),
        ("map", "GLOBAL_ARRAY_MAP_ID"),
        ("length", "GLOBAL_NUMBER_KEYWORD_ID"),
    ]
    .into_iter()
    .map(|(name, type_id)| LoweredTypeMember {
        name: Text::from(name),
        kind: LoweredMemberKind::Named { optional: false },
        type_reference: LoweredTypeReference::Predefined(type_id),
    })
    .chain(class.members.iter().cloned())
    .collect();
    globals.push(LoweredGlobal {
        roles: GlobalRoles::default(),
        local_types,
        name: Text::from("Array"),
        id_constant: "ARRAY_ID_GLOBAL_TYPE_ID".into(),
        data: LoweredTypeData::Class(class),
    });
    globals.push(array_method_global(
        "Array.prototype.filter",
        "ARRAY_FILTER_ID_GLOBAL_TYPE_ID",
        Box::default(),
        "GLOBAL_CONDITIONAL_CALLBACK_ID",
        "GLOBAL_INSTANCEOF_ARRAY_T_ID",
    ));
    globals.push(array_method_global(
        "Array.prototype.forEach",
        "ARRAY_FOREACH_ID_GLOBAL_TYPE_ID",
        Box::default(),
        "GLOBAL_VOID_CALLBACK_ID",
        "GLOBAL_VOID_ID",
    ));
    globals.push(array_method_global(
        "Array.prototype.map",
        "ARRAY_MAP_ID_GLOBAL_TYPE_ID",
        Box::new([LoweredTypeReference::Predefined("GLOBAL_U_ID")]),
        "GLOBAL_MAP_CALLBACK_ID",
        "GLOBAL_INSTANCEOF_ARRAY_U_ID",
    ));

    Ok(())
}

/// Validates the selected declarations before emitting the resolver's reduced Promise projection.
fn lower_promise_globals(
    manifest: &GlobalManifest,
    source_cache: &mut ParsedSourceCache,
    globals: &mut Vec<LoweredGlobal>,
) -> Result<()> {
    let Some(promise_group) = manifest.global_group("Promise") else {
        return Ok(());
    };
    if !promise_group.has_role(GlobalDeclarationRole::Type) {
        bail!("Promise global must have a type-side declaration");
    }
    if !promise_group.has_role(GlobalDeclarationRole::Value) {
        bail!("Promise global must have a value-side declaration");
    }
    validate_promise_constructor_reference(promise_group.declarations(), source_cache)?;

    let mut saw_promise_interface = false;
    let mut saw_methods = [false; PROMISE_METHOD_COUNT];
    for record in promise_group.declarations() {
        match &record.kind {
            DeclarationKind::Interface => saw_promise_interface = true,
            DeclarationKind::VariableDeclarator { .. } => continue,
            DeclarationKind::TypeAlias => {
                bail!("type aliases are not supported in the Promise global")
            }
            DeclarationKind::DeclareFunction | DeclarationKind::ImportEquals => {
                bail!("unsupported value-side Promise declaration")
            }
        }

        let declaration = source_cache
            .find_interface_declaration(record)?
            .with_context(|| {
                format!(
                    "failed to find interface declaration {} at {:?}",
                    record.declared_name.text(),
                    record.text_range
                )
            })?;
        validate_promise_interface(&declaration)?;
        validate_promise_methods(
            &declaration,
            PromiseMemberLocation::Instance,
            &mut saw_methods,
        )?;
    }
    if !saw_promise_interface {
        bail!("Promise global must include an interface declaration");
    }

    let Some(constructor_group) = manifest.global_group("PromiseConstructor") else {
        bail!("Promise global value side references missing PromiseConstructor group");
    };
    if !constructor_group.has_role(GlobalDeclarationRole::Type) {
        bail!("PromiseConstructor must have a type-side declaration");
    }

    let mut saw_constructor_interface = false;
    let mut saw_construct_signature = false;
    for record in constructor_group.declarations() {
        match &record.kind {
            DeclarationKind::Interface => saw_constructor_interface = true,
            DeclarationKind::TypeAlias => {
                bail!("type aliases are not supported in PromiseConstructor")
            }
            DeclarationKind::DeclareFunction
            | DeclarationKind::VariableDeclarator { .. }
            | DeclarationKind::ImportEquals => {
                bail!("value-side PromiseConstructor declarations are not supported")
            }
        }

        let declaration = source_cache
            .find_interface_declaration(record)?
            .with_context(|| {
                format!(
                    "failed to find interface declaration {} at {:?}",
                    record.declared_name.text(),
                    record.text_range
                )
            })?;
        if declaration.extends_clause().is_some() {
            bail!("PromiseConstructor extends clauses are not supported");
        }
        if declaration.type_parameters().is_some() {
            bail!("PromiseConstructor type parameters are not supported");
        }

        validate_promise_methods(
            &declaration,
            PromiseMemberLocation::Static,
            &mut saw_methods,
        )?;
        for member in declaration.members() {
            if let AnyTsTypeMember::TsConstructSignatureTypeMember(member) = member {
                validate_promise_construct_signature(&member)?;
                saw_construct_signature = true;
            }
        }
    }
    if !saw_constructor_interface {
        bail!("PromiseConstructor must include an interface declaration");
    }
    if !saw_construct_signature {
        bail!("PromiseConstructor is missing a construct signature");
    }
    for (index, specification) in PROMISE_METHOD_SPECIFICATIONS.iter().enumerate() {
        if !saw_methods[index] {
            bail!("Promise is missing {}", specification.global_name);
        }
    }

    let members = std::iter::once(LoweredTypeMember {
        name: Text::from("constructor"),
        kind: LoweredMemberKind::Constructor,
        type_reference: LoweredTypeReference::Predefined("GLOBAL_PROMISE_CONSTRUCT_ID"),
    })
    .chain(PROMISE_METHOD_SPECIFICATIONS.map(promise_member))
    .collect();
    globals.push(LoweredGlobal {
        roles: GlobalRoles::default(),
        local_types: Box::default(),
        name: Text::from("Promise"),
        id_constant: "PROMISE_ID_GLOBAL_TYPE_ID".into(),
        data: LoweredTypeData::Class(LoweredClass {
            extends: None,
            implements: Box::default(),
            name: Text::from("Promise"),
            type_parameters: Box::new([LoweredTypeReference::Predefined("GLOBAL_T_ID")]),
            members,
        }),
    });
    globals.push(LoweredGlobal {
        roles: GlobalRoles::default(),
        local_types: Box::default(),
        name: Text::from("Promise.constructor"),
        id_constant: "PROMISE_CONSTRUCT_ID_GLOBAL_TYPE_ID".into(),
        data: LoweredTypeData::Function(LoweredFunction {
            is_async: false,
            type_parameters: Box::default(),
            name: Some(Text::from("Promise.constructor")),
            parameters: Box::new([LoweredFunctionParameter {
                binding: LoweredFunctionParameterBinding::Pattern,
                type_reference: LoweredTypeReference::Predefined("GLOBAL_VOID_CALLBACK_ID"),
                is_optional: false,
                is_rest: false,
            }]),
            return_type: LoweredTypeReference::Predefined("GLOBAL_VOID_ID"),
        }),
    });
    for specification in PROMISE_METHOD_SPECIFICATIONS {
        globals.push(promise_method_global(specification));
    }

    Ok(())
}

/// Requires each merged Promise declaration to use one unconstrained type parameter.
fn validate_promise_interface(declaration: &TsInterfaceDeclaration) -> Result<()> {
    if declaration.extends_clause().is_some() {
        bail!("Promise interface extends clauses are not supported");
    }
    single_type_parameter_name(declaration.type_parameters(), "Promise interface")?;
    Ok(())
}

/// Checks selected methods while allowing overloads and unrelated declaration members.
fn validate_promise_methods(
    declaration: &TsInterfaceDeclaration,
    location: PromiseMemberLocation,
    saw_methods: &mut [bool; PROMISE_METHOD_COUNT],
) -> Result<()> {
    for member in declaration.members() {
        match member {
            AnyTsTypeMember::TsMethodSignatureTypeMember(method) => {
                validate_selected_promise_method(&method, location, saw_methods)?;
            }
            AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
                reject_selected_promise_non_method(property.name()?, location)?;
            }
            AnyTsTypeMember::TsGetterSignatureTypeMember(getter) => {
                reject_selected_promise_non_method(getter.name()?, location)?;
            }
            AnyTsTypeMember::TsSetterSignatureTypeMember(setter) => {
                reject_selected_promise_non_method(setter.name()?, location)?;
            }
            AnyTsTypeMember::JsBogusMember(_)
            | AnyTsTypeMember::JsMetavariable(_)
            | AnyTsTypeMember::TsCallSignatureTypeMember(_)
            | AnyTsTypeMember::TsConstructSignatureTypeMember(_)
            | AnyTsTypeMember::TsIndexSignatureTypeMember(_) => {}
        }
    }
    Ok(())
}

/// Marks a selected method after checking the Promise return retained by the projection.
fn validate_selected_promise_method(
    method: &TsMethodSignatureTypeMember,
    location: PromiseMemberLocation,
    saw_methods: &mut [bool; PROMISE_METHOD_COUNT],
) -> Result<()> {
    let Some((index, specification)) = selected_promise_method(method.name()?, location)? else {
        return Ok(());
    };
    if method.optional_token().is_some() {
        bail!("{} must not be optional", specification.global_name);
    }
    let return_type = method
        .return_type_annotation()
        .with_context(|| format!("{} is missing a return type", specification.global_name))?
        .ty()
        .with_context(|| format!("{} has a malformed return type", specification.global_name))?;
    let return_type = regular_return_type(return_type, specification.global_name)?;
    validate_promise_reference(&return_type, specification.global_name)?;
    saw_methods[index] = true;
    Ok(())
}

/// Resolves a literal member name only within its instance or static declaration side.
fn selected_promise_method(
    name: AnyJsObjectMemberName,
    location: PromiseMemberLocation,
) -> Result<Option<(usize, PromiseMethodSpecification)>> {
    let AnyJsObjectMemberName::JsLiteralMemberName(name) = name else {
        return Ok(None);
    };
    let name = name.name()?;
    Ok(PROMISE_METHOD_SPECIFICATIONS
        .iter()
        .copied()
        .enumerate()
        .find(|(_, specification)| {
            specification.location == location && specification.source_name == name.text()
        }))
}

/// Rejects properties and accessors only when their names belong to the selected projection.
fn reject_selected_promise_non_method(
    name: AnyJsObjectMemberName,
    location: PromiseMemberLocation,
) -> Result<()> {
    if let Some((_, specification)) = selected_promise_method(name, location)? {
        bail!("{} must be a method", specification.global_name);
    }
    Ok(())
}

/// Requires the global value declaration to retain its constructor interface.
fn validate_promise_constructor_reference(
    records: &[DeclarationRecord],
    source_cache: &mut ParsedSourceCache,
) -> Result<()> {
    let mut saw_value = false;
    for record in records {
        let DeclarationKind::VariableDeclarator { .. } = &record.kind else {
            continue;
        };
        let declarator = source_cache
            .find_variable_declarator(record)?
            .with_context(|| {
                format!(
                    "failed to find variable declaration {} at {:?}",
                    record.declared_name.text(),
                    record.text_range
                )
            })?;
        let Some(AnyTsVariableAnnotation::TsTypeAnnotation(annotation)) =
            declarator.variable_annotation()
        else {
            bail!("declare var Promise is missing a type annotation");
        };
        validate_reference_type(
            &annotation.ty()?,
            "PromiseConstructor",
            "declare var Promise",
        )?;
        saw_value = true;
    }
    if !saw_value {
        bail!("Promise global must include declare var Promise");
    }
    Ok(())
}

/// Requires the executor and return shapes represented by the synthetic constructor helper.
fn validate_promise_construct_signature(member: &TsConstructSignatureTypeMember) -> Result<()> {
    single_type_parameter_name(member.type_parameters(), "Promise constructor")?;

    let mut parameters = member.parameters()?.items().into_iter();
    let executor = required_formal_parameter(
        parameters.next(),
        "Promise constructor",
        "one executor parameter",
    )?;
    if parameters.next().is_some() || executor.question_mark_token().is_some() {
        bail!("Promise constructor must have one required executor parameter");
    }
    let executor_type = executor
        .type_annotation()
        .context("Promise constructor executor is missing a type annotation")?
        .ty()
        .context("Promise constructor executor has a malformed type annotation")?;
    let AnyTsType::TsFunctionType(executor) = executor_type else {
        bail!("Promise constructor executor must be a function type");
    };
    if executor.type_parameters().is_some() {
        bail!("Promise constructor executor must not be generic");
    }
    let executor_return_type = regular_return_type(executor.return_type()?, "Promise constructor")?;
    if !matches!(executor_return_type, AnyTsType::TsVoidType(_)) {
        bail!("Promise constructor executor must return void");
    }

    let return_type = member
        .type_annotation()
        .context("Promise constructor is missing a return type")?
        .ty()
        .context("Promise constructor has a malformed return type")?;
    validate_promise_reference(&return_type, "Promise constructor")
}

/// Requires `Promise<...>` without constraining the declaration's projected type argument.
fn validate_promise_reference(type_node: &AnyTsType, owner: &str) -> Result<()> {
    let AnyTsType::TsReferenceType(reference) = type_node else {
        bail!("{owner} must return Promise");
    };
    let name = reference
        .name()
        .with_context(|| format!("{owner} has a missing return type name"))?;
    let biome_js_syntax::AnyTsName::JsReferenceIdentifier(identifier) = name else {
        bail!("{owner} must return Promise");
    };
    if identifier.value_token()?.token_text_trimmed().text() != "Promise" {
        bail!("{owner} must return Promise");
    }
    let type_arguments = reference
        .type_arguments()
        .with_context(|| format!("{owner} must return Promise with one type argument"))?;
    let mut arguments = type_arguments.ts_type_argument_list().into_iter();
    let Some(argument) = arguments.next() else {
        bail!("{owner} must return Promise with one type argument");
    };
    argument.with_context(|| format!("{owner} has a malformed Promise type argument"))?;
    if arguments.next().is_some() {
        bail!("{owner} must return Promise with one type argument");
    }
    Ok(())
}

/// Preserves whether lookup reaches the helper through an instance or the constructor.
fn promise_member(specification: PromiseMethodSpecification) -> LoweredTypeMember {
    let kind = match specification.location {
        PromiseMemberLocation::Instance => LoweredMemberKind::Named { optional: false },
        PromiseMemberLocation::Static => LoweredMemberKind::NamedStatic,
    };
    LoweredTypeMember {
        name: Text::from(specification.source_name),
        kind,
        type_reference: LoweredTypeReference::Predefined(specification.member_type_id),
    }
}

/// Keeps the resolver's parameter-free callable projection for selected Promise methods.
fn promise_method_global(specification: PromiseMethodSpecification) -> LoweredGlobal {
    LoweredGlobal {
        roles: GlobalRoles::default(),
        local_types: Box::default(),
        name: Text::from(specification.global_name),
        id_constant: specification.id_constant.into(),
        data: LoweredTypeData::Function(LoweredFunction {
            is_async: false,
            type_parameters: Box::default(),
            name: Some(Text::from(specification.global_name)),
            parameters: Box::default(),
            return_type: LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_PROMISE_ID"),
        }),
    }
}

/// Builds the resolver's single-callback shape, omitting the validated `thisArg`.
fn array_method_global(
    name: &'static str,
    id_constant: &'static str,
    type_parameters: Box<[LoweredTypeReference]>,
    parameter_type_id: &'static str,
    return_type_id: &'static str,
) -> LoweredGlobal {
    LoweredGlobal {
        roles: GlobalRoles::default(),
        local_types: Box::default(),
        name: Text::from(name),
        id_constant: id_constant.into(),
        data: LoweredTypeData::Function(LoweredFunction {
            is_async: false,
            type_parameters,
            name: Some(Text::from(name)),
            parameters: Box::new([LoweredFunctionParameter {
                binding: LoweredFunctionParameterBinding::Pattern,
                type_reference: LoweredTypeReference::Predefined(parameter_type_id),
                is_optional: false,
                is_rest: false,
            }]),
            return_type: LoweredTypeReference::Predefined(return_type_id),
        }),
    }
}

/// Requires unextended `Array<T>` with no modifiers, constraint, or default on `T`.
fn validate_array_interface_type_parameter(declaration: &TsInterfaceDeclaration) -> Result<Text> {
    if declaration.extends_clause().is_some() {
        bail!("Array interface extends clauses are not supported");
    }
    single_type_parameter_name(declaration.type_parameters(), "Array interface")
}

/// Recognizes only the Array members represented by the resolver projection.
fn selected_array_member(name: AnyJsObjectMemberName) -> Result<Option<SelectedArrayMember>> {
    let AnyJsObjectMemberName::JsLiteralMemberName(name) = name else {
        return Ok(None);
    };
    Ok(match name.name()?.text() {
        "filter" => Some(SelectedArrayMember::Filter),
        "forEach" => Some(SelectedArrayMember::ForEach),
        "length" => Some(SelectedArrayMember::Length),
        "map" => Some(SelectedArrayMember::Map),
        _ => None,
    })
}

/// Rejects selected members expressed as unsupported getter or setter declarations.
fn reject_selected_array_accessor(name: AnyJsObjectMemberName) -> Result<()> {
    if let Some(member) = selected_array_member(name)? {
        bail!(
            "Array.{} has an unsupported accessor declaration",
            member.name()
        );
    }
    Ok(())
}

/// Requires `length` to be a mutable, required `number` property.
fn validate_array_length(property: &TsPropertySignatureTypeMember) -> Result<()> {
    if property.readonly_token().is_some() {
        bail!("Array.length must not be readonly");
    }
    if property.optional_token().is_some() {
        bail!("Array.length must not be optional");
    }
    let type_node = property
        .type_annotation()
        .context("Array.length is missing a type annotation")?
        .ty()
        .context("Array.length has a malformed type annotation")?;
    if !matches!(type_node, AnyTsType::TsNumberType(_)) {
        bail!("Array.length must be number");
    }
    Ok(())
}

/// Validates the selected non-generic `filter` overload that preserves the element type.
fn validate_array_filter(
    method: &TsMethodSignatureTypeMember,
    array_type_parameter: &str,
) -> Result<()> {
    validate_array_method_parameters(
        method,
        "Array.filter",
        array_type_parameter,
        ArrayCallbackReturn::Unknown,
    )?;
    validate_array_method_array_return(method, "Array.filter", array_type_parameter)
}

/// Requires a non-generic `forEach` method and callback that both return `void`.
fn validate_array_for_each(
    method: &TsMethodSignatureTypeMember,
    array_type_parameter: &str,
) -> Result<()> {
    if method.type_parameters().is_some() {
        bail!("Array.forEach must not be generic");
    }
    validate_array_method_parameters(
        method,
        "Array.forEach",
        array_type_parameter,
        ArrayCallbackReturn::Void,
    )?;
    validate_array_method_void_return(method, "Array.forEach")
}

/// Requires `map` to use one type parameter for both callback and array results.
fn validate_array_map(
    method: &TsMethodSignatureTypeMember,
    array_type_parameter: &str,
) -> Result<()> {
    let map_type_parameter = single_type_parameter_name(method.type_parameters(), "Array.map")?;
    validate_array_method_parameters(
        method,
        "Array.map",
        array_type_parameter,
        ArrayCallbackReturn::Reference(map_type_parameter.text()),
    )?;
    validate_array_method_array_return(method, "Array.map", map_type_parameter.text())
}

/// Requires a callback followed by an optional `any`-typed `thisArg`.
fn validate_array_method_parameters(
    method: &TsMethodSignatureTypeMember,
    owner: &str,
    array_type_parameter: &str,
    callback_return: ArrayCallbackReturn,
) -> Result<()> {
    if method.optional_token().is_some() {
        bail!("{owner} must not be optional");
    }

    let mut parameters = method.parameters()?.items().into_iter();
    let expected_parameters = "a callback and an optional thisArg parameter";
    let callback_parameter =
        required_formal_parameter(parameters.next(), owner, expected_parameters)?;
    let this_argument_parameter =
        required_formal_parameter(parameters.next(), owner, expected_parameters)?;
    if parameters.next().is_some() {
        bail!("{owner} must have a callback and an optional thisArg parameter");
    }

    if callback_parameter.question_mark_token().is_some() {
        bail!("{owner} callback must not be optional");
    }
    let callback_type = callback_parameter
        .type_annotation()
        .with_context(|| format!("{owner} callback is missing a type annotation"))?
        .ty()
        .with_context(|| format!("{owner} callback has a malformed type annotation"))?;
    let AnyTsType::TsFunctionType(callback) = callback_type else {
        bail!("{owner} callback must be a function type");
    };
    validate_array_callback(&callback, owner, array_type_parameter, callback_return)?;

    if this_argument_parameter.question_mark_token().is_none() {
        bail!("{owner} thisArg must be optional");
    }
    let this_argument_type = this_argument_parameter
        .type_annotation()
        .with_context(|| format!("{owner} thisArg is missing a type annotation"))?
        .ty()
        .with_context(|| format!("{owner} thisArg has a malformed type annotation"))?;
    if !matches!(this_argument_type, AnyTsType::TsAnyType(_)) {
        bail!("{owner} thisArg must be any");
    }

    Ok(())
}

/// Extracts a plain formal parameter without decorators or an initializer.
fn required_formal_parameter(
    parameter: Option<SyntaxResult<AnyJsParameter>>,
    owner: &str,
    expected_parameters: &str,
) -> Result<JsFormalParameter> {
    let Some(parameter) = parameter else {
        bail!("{owner} must have {expected_parameters}");
    };
    let AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(parameter)) =
        parameter.with_context(|| format!("{owner} has a malformed parameter"))?
    else {
        bail!("{owner} has an unsupported parameter");
    };
    if !parameter.decorators().is_empty() || parameter.initializer().is_some() {
        bail!("{owner} has an unsupported parameter");
    }
    Ok(parameter)
}

/// Requires a non-generic callback with the selected parameters and return shape.
fn validate_array_callback(
    callback: &TsFunctionType,
    owner: &str,
    array_type_parameter: &str,
    callback_return: ArrayCallbackReturn,
) -> Result<()> {
    if callback.type_parameters().is_some() {
        bail!("{owner} callback must not be generic");
    }

    let mut parameters = callback.parameters()?.items().into_iter();
    let expected_parameters = "a callback with three required parameters";
    let value_parameter = required_formal_parameter(parameters.next(), owner, expected_parameters)?;
    let index_parameter = required_formal_parameter(parameters.next(), owner, expected_parameters)?;
    let array_parameter = required_formal_parameter(parameters.next(), owner, expected_parameters)?;
    if parameters.next().is_some() {
        bail!("{owner} must have a callback with three required parameters");
    }
    let value_type = required_array_callback_parameter_type(&value_parameter, owner)?;
    validate_reference_type(&value_type, array_type_parameter, owner)?;
    let index_type = required_array_callback_parameter_type(&index_parameter, owner)?;
    if !matches!(index_type, AnyTsType::TsNumberType(_)) {
        bail!("{owner} callback index parameter must be number");
    }
    let array_type = required_array_callback_parameter_type(&array_parameter, owner)?;
    validate_array_type(&array_type, array_type_parameter, owner)?;

    let return_type = regular_return_type(callback.return_type()?, owner)?;
    match callback_return {
        ArrayCallbackReturn::Reference(type_parameter) => {
            validate_reference_type(&return_type, type_parameter, owner)?;
        }
        ArrayCallbackReturn::Unknown => {
            if !matches!(return_type, AnyTsType::TsUnknownType(_)) {
                bail!("{owner} callback must return unknown");
            }
        }
        ArrayCallbackReturn::Void => {
            if !matches!(return_type, AnyTsType::TsVoidType(_)) {
                bail!("{owner} callback must return void");
            }
        }
    }

    Ok(())
}

/// Returns the type annotation after rejecting an optional callback parameter.
fn required_array_callback_parameter_type(
    parameter: &JsFormalParameter,
    owner: &str,
) -> Result<AnyTsType> {
    if parameter.question_mark_token().is_some() {
        bail!("{owner} callback must have three required parameters");
    }
    parameter
        .type_annotation()
        .with_context(|| format!("{owner} callback parameter is missing a type annotation"))?
        .ty()
        .with_context(|| format!("{owner} callback parameter has a malformed type annotation"))
}

/// Requires a regular array return whose element matches the selected type parameter.
fn validate_array_method_array_return(
    method: &TsMethodSignatureTypeMember,
    owner: &str,
    element_type_parameter: &str,
) -> Result<()> {
    let return_type = method
        .return_type_annotation()
        .with_context(|| format!("{owner} is missing a return type"))?
        .ty()
        .with_context(|| format!("{owner} has a malformed return type"))?;
    let return_type = regular_return_type(return_type, owner)?;
    validate_array_type(&return_type, element_type_parameter, owner)
}

/// Requires an ordinary `void` return rather than a predicate or assertion.
fn validate_array_method_void_return(
    method: &TsMethodSignatureTypeMember,
    owner: &str,
) -> Result<()> {
    let return_type = method
        .return_type_annotation()
        .with_context(|| format!("{owner} is missing a return type"))?
        .ty()
        .with_context(|| format!("{owner} has a malformed return type"))?;
    let return_type = regular_return_type(return_type, owner)?;
    if !matches!(return_type, AnyTsType::TsVoidType(_)) {
        bail!("{owner} must return void");
    }
    Ok(())
}

/// Requires array syntax whose element references the expected type parameter.
fn validate_array_type(
    type_node: &AnyTsType,
    element_type_parameter: &str,
    owner: &str,
) -> Result<()> {
    let AnyTsType::TsArrayType(array_type) = type_node else {
        bail!("{owner} must use an array type");
    };
    validate_reference_type(&array_type.element_type()?, element_type_parameter, owner)
}

/// Requires an unqualified type reference without type arguments.
fn validate_reference_type(type_node: &AnyTsType, expected_name: &str, owner: &str) -> Result<()> {
    let AnyTsType::TsReferenceType(reference) = type_node else {
        bail!("{owner} must reference {expected_name}");
    };
    if reference.type_arguments().is_some() {
        bail!("{owner} must reference {expected_name} without type arguments");
    }
    let biome_js_syntax::AnyTsName::JsReferenceIdentifier(identifier) = reference
        .name()
        .with_context(|| format!("{owner} has a missing type reference name"))?
    else {
        bail!("{owner} must reference {expected_name}");
    };
    if identifier.value_token()?.token_text_trimmed().text() != expected_name {
        bail!("{owner} must reference {expected_name}");
    }
    Ok(())
}

/// Rejects predicate and assertion return types.
fn regular_return_type(return_type: AnyTsReturnType, owner: &str) -> Result<AnyTsType> {
    match return_type {
        AnyTsReturnType::AnyTsType(type_node) => Ok(type_node),
        AnyTsReturnType::TsAssertsReturnType(_) | AnyTsReturnType::TsPredicateReturnType(_) => {
            bail!("{owner} must use a regular return type")
        }
    }
}

/// Extracts one unmodified type parameter without a constraint or default.
fn single_type_parameter_name(
    type_parameters: Option<TsTypeParameters>,
    owner: &str,
) -> Result<Text> {
    let type_parameters =
        type_parameters.with_context(|| format!("{owner} must have one type parameter"))?;
    let mut type_parameters = type_parameters.items().into_iter();
    let Some(type_parameter) = type_parameters.next() else {
        bail!("{owner} must have one type parameter");
    };
    let type_parameter =
        type_parameter.with_context(|| format!("{owner} has a malformed type parameter"))?;
    if type_parameters.next().is_some() {
        bail!("{owner} must have one type parameter");
    }
    if !type_parameter.modifiers().is_empty()
        || type_parameter.constraint().is_some()
        || type_parameter.default().is_some()
    {
        bail!("{owner} has an unsupported type parameter");
    }

    Ok(Text::from(
        type_parameter.name()?.ident_token()?.token_text_trimmed(),
    ))
}

/// Lowers a disposable interface into its interface global plus the dispose helper global.
fn lower_disposable_global(
    manifest: &GlobalManifest,
    ids: &GlobalIds,
    source_cache: &mut ParsedSourceCache,
    globals: &mut Vec<LoweredGlobal>,
    spec: DisposableGlobalSpec,
) -> Result<()> {
    let Some(group) = manifest.global_group(spec.interface_name) else {
        return Ok(());
    };
    // The computed key refers to the symbol's generated identity, so the symbol must be
    // declared as a `unique symbol` property of `SymbolConstructor`.
    let symbol = spec
        .member_name
        .trim_start_matches('[')
        .trim_end_matches(']');
    let declares_symbol = ids.value_slot(symbol).is_some_and(|index| {
        let slot = ids.slot(index);
        slot.kind() == &GlobalSlotKind::UniqueSymbol && slot.reference_constant() == spec.symbol_id
    });
    if !declares_symbol {
        bail!(
            "{} requires {symbol} to be declared as a unique symbol",
            spec.interface_name
        );
    }
    if !group.has_role(GlobalDeclarationRole::Type) {
        bail!(
            "{} global must have a type-side declaration",
            spec.interface_name
        );
    }

    let mut lowered_member = None;
    let mut saw_interface = false;
    for record in group.declarations() {
        match &record.kind {
            DeclarationKind::Interface => {
                saw_interface = true;
            }
            DeclarationKind::TypeAlias => {
                bail!("type aliases are not supported in {}", spec.interface_name)
            }
            DeclarationKind::DeclareFunction
            | DeclarationKind::VariableDeclarator { .. }
            | DeclarationKind::ImportEquals => {
                bail!(
                    "value-side {} declarations are not supported",
                    spec.interface_name
                )
            }
        }

        let declaration = source_cache
            .find_interface_declaration(record)?
            .with_context(|| {
                format!(
                    "failed to find interface declaration {} at {:?}",
                    record.declared_name.text(),
                    record.text_range
                )
            })?;
        if declaration.extends_clause().is_some() {
            bail!("{} extends clauses are not supported", spec.interface_name);
        }
        if declaration.type_parameters().is_some() {
            bail!("{} type parameters are not supported", spec.interface_name);
        }

        for member in declaration.members() {
            let lowered = lower_disposable_type_member(member, spec)?;
            if lowered_member.replace(lowered).is_some() {
                bail!("{} has multiple computed members", spec.interface_name);
            }
        }
    }
    if !saw_interface {
        bail!(
            "{} global must include an interface declaration",
            spec.interface_name
        );
    }

    let lowered_member = lowered_member
        .with_context(|| format!("{} is missing {}", spec.interface_name, spec.member_name))?;

    globals.push(LoweredGlobal {
        roles: GlobalRoles::default(),
        local_types: Box::default(),
        name: Text::from(spec.interface_name),
        id_constant: spec.global_id_constant.into(),
        data: LoweredTypeData::Interface(LoweredInterface {
            name: Text::from(spec.interface_name),
            type_parameters: Box::default(),
            extends: Box::default(),
            members: Box::new([lowered_member]),
        }),
    });
    globals.push(LoweredGlobal {
        roles: GlobalRoles::default(),
        local_types: Box::default(),
        name: Text::from(spec.helper_name),
        id_constant: spec.helper_id_constant.into(),
        data: LoweredTypeData::Function(LoweredFunction {
            is_async: spec.return_kind.helper_is_async(),
            type_parameters: Box::default(),
            name: None,
            parameters: Box::default(),
            return_type: LoweredTypeReference::Predefined(spec.return_kind.return_type_id()),
        }),
    });

    Ok(())
}

/// Lowers the single member of a disposable interface. Only a computed method signature
/// (`[Symbol.dispose](): void`) is supported; every other member shape bails.
fn lower_disposable_type_member(
    member: AnyTsTypeMember,
    spec: DisposableGlobalSpec,
) -> Result<LoweredTypeMember> {
    match member {
        AnyTsTypeMember::TsMethodSignatureTypeMember(member) => {
            lower_disposable_method_signature(&member, spec)
        }
        AnyTsTypeMember::TsPropertySignatureTypeMember(_) => {
            bail!("properties are not supported in {}", spec.interface_name)
        }
        AnyTsTypeMember::TsCallSignatureTypeMember(_)
        | AnyTsTypeMember::TsConstructSignatureTypeMember(_) => {
            bail!("signatures are not supported in {}", spec.interface_name)
        }
        AnyTsTypeMember::JsBogusMember(_) => {
            bail!("bogus members are not supported in {}", spec.interface_name)
        }
        AnyTsTypeMember::JsMetavariable(_) => {
            bail!("metavariables are not supported in {}", spec.interface_name)
        }
        AnyTsTypeMember::TsGetterSignatureTypeMember(_) => {
            bail!(
                "getter signatures are not supported in {}",
                spec.interface_name
            )
        }
        AnyTsTypeMember::TsIndexSignatureTypeMember(_) => {
            bail!(
                "index signatures are not supported in {}",
                spec.interface_name
            )
        }
        AnyTsTypeMember::TsSetterSignatureTypeMember(_) => {
            bail!(
                "setter signatures are not supported in {}",
                spec.interface_name
            )
        }
    }
}

/// Lowers a `[Symbol.(async)Dispose](): <return>` method into a computed-value member whose
/// key is the well-known symbol and whose value type is the dispose helper. Bails on any
/// deviation from that exact shape (optional, generic, parameterized, wrong key, wrong return).
fn lower_disposable_method_signature(
    member: &TsMethodSignatureTypeMember,
    spec: DisposableGlobalSpec,
) -> Result<LoweredTypeMember> {
    if member.optional_token().is_some() {
        bail!("{} must not be optional", spec.member_name);
    }
    if member.type_parameters().is_some() {
        bail!("{} must not be generic", spec.member_name);
    }
    if member.parameters()?.items().into_iter().next().is_some() {
        bail!("{} must not declare parameters", spec.member_name);
    }

    let computed_member = lower_symbol_computed_member_name(member.name()?)?;
    if computed_member.name.text() != spec.member_name {
        bail!(
            "{} has unsupported computed member {}",
            spec.interface_name,
            computed_member.name
        );
    }
    if computed_member.key_reference != LoweredTypeReference::Predefined(spec.symbol_id) {
        bail!(
            "{} has unsupported computed key for {}",
            spec.interface_name,
            spec.member_name
        );
    }

    let return_type = member
        .return_type_annotation()
        .with_context(|| format!("{} is missing a return type", spec.member_name))?
        .ty()
        .with_context(|| format!("{} has malformed return type", spec.member_name))
        .and_then(|return_type_node| lower_disposable_return_type(&return_type_node, spec))?;
    if return_type != LoweredTypeReference::Predefined(spec.return_kind.return_type_id()) {
        bail!("{} has unsupported return type", spec.member_name);
    }

    Ok(LoweredTypeMember {
        name: computed_member.name,
        kind: LoweredMemberKind::ComputedValue {
            key_reference: computed_member.key_reference,
        },
        type_reference: LoweredTypeReference::Predefined(spec.helper_type_id),
    })
}

/// A lowered `[Symbol.<name>]` computed key: its display name and the `GLOBAL_*` symbol reference.
struct ComputedMemberName {
    name: Text,
    key_reference: LoweredTypeReference,
}

/// Resolves well-known Symbol keys to their predefined identities.
/// Other computed expressions and Symbol properties return errors.
fn lower_symbol_computed_member_name(name: AnyJsObjectMemberName) -> Result<ComputedMemberName> {
    let AnyJsObjectMemberName::JsComputedMemberName(name) = name else {
        bail!("expected computed symbol member name")
    };
    let AnyJsExpression::JsStaticMemberExpression(expression) = name.expression()? else {
        bail!("computed member name must be a static member expression")
    };
    let AnyJsExpression::JsIdentifierExpression(object) = expression.object()? else {
        bail!("computed member object must be Symbol")
    };
    if object.name()?.value_token()?.token_text_trimmed().text() != "Symbol" {
        bail!("computed member object must be Symbol");
    }

    let AnyJsName::JsName(member_name) = expression.member()? else {
        bail!("computed Symbol member must be a public name")
    };
    let member_name = member_name.value_token()?.token_text_trimmed();
    match member_name.text() {
        "dispose" => Ok(ComputedMemberName {
            name: Text::from("[Symbol.dispose]"),
            key_reference: LoweredTypeReference::Predefined("GLOBAL_SYMBOL_DISPOSE_ID"),
        }),
        "asyncDispose" => Ok(ComputedMemberName {
            name: Text::from("[Symbol.asyncDispose]"),
            key_reference: LoweredTypeReference::Predefined("GLOBAL_SYMBOL_ASYNC_DISPOSE_ID"),
        }),
        name => bail!("unsupported Symbol computed member {name}"),
    }
}

/// Lowers a dispose helper's return type according to `spec.return_kind`: a plain `void`, or
/// the `PromiseLike<void>` special case handled by [`lower_promise_like_void_reference`].
fn lower_disposable_return_type(
    return_type_node: &AnyTsReturnType,
    spec: DisposableGlobalSpec,
) -> Result<LoweredTypeReference> {
    match return_type_node {
        AnyTsReturnType::AnyTsType(type_node) => match spec.return_kind {
            DisposableReturnKind::Void => lower_void_reference(type_node, spec),
            DisposableReturnKind::PromiseLikeVoid => lower_promise_like_void_reference(type_node),
        },
        AnyTsReturnType::TsAssertsReturnType(_) | AnyTsReturnType::TsPredicateReturnType(_) => {
            bail!(
                "predicate return types are not supported in {}",
                spec.member_name
            )
        }
    }
}

/// Lowers the `Disposable` dispose helper's `void` return type to `GLOBAL_VOID_ID`.
fn lower_void_reference(
    type_node: &AnyTsType,
    spec: DisposableGlobalSpec,
) -> Result<LoweredTypeReference> {
    if !matches!(type_node, AnyTsType::TsVoidType(_)) {
        bail!("{} return type must be void", spec.member_name);
    }
    Ok(LoweredTypeReference::Predefined("GLOBAL_VOID_ID"))
}

/// Lowers the `AsyncDisposable` dispose helper's `PromiseLike<void>` return type to
/// `GLOBAL_INSTANCEOF_PROMISE_ID`. This is a deliberate approximation: `PromiseLike` is not a
/// migrated global yet, so the helper resolves to `instanceof Promise` exactly like the previous
/// hand-written data did; the exact-shape check keeps any other return type from being lowered.
fn lower_promise_like_void_reference(type_node: &AnyTsType) -> Result<LoweredTypeReference> {
    let AnyTsType::TsReferenceType(reference) = type_node else {
        bail!("AsyncDisposable return type must be PromiseLike<void>");
    };
    let name = reference
        .name()
        .context("missing AsyncDisposable return type name")?;
    let biome_js_syntax::AnyTsName::JsReferenceIdentifier(identifier) = name else {
        bail!("qualified AsyncDisposable return types are not supported");
    };
    if identifier.value_token()?.token_text_trimmed().text() != "PromiseLike" {
        bail!("AsyncDisposable return type must be PromiseLike<void>");
    }

    let type_arguments = reference
        .type_arguments()
        .context("PromiseLike return type is missing type arguments")?;
    let mut arguments = type_arguments.ts_type_argument_list().into_iter();
    let Some(argument) = arguments.next() else {
        bail!("PromiseLike return type is missing void type argument");
    };
    let argument = argument?;
    if arguments.next().is_some() {
        bail!("PromiseLike return type must have one type argument");
    }
    if !matches!(argument, AnyTsType::TsVoidType(_)) {
        bail!("PromiseLike return type must be PromiseLike<void>");
    }

    Ok(LoweredTypeReference::Predefined(
        "GLOBAL_INSTANCEOF_PROMISE_ID",
    ))
}

/// Lowers supported members from `interface Error`.
fn lower_error_interface_members(
    declaration: &TsInterfaceDeclaration,
    members: &mut Vec<LoweredTypeMember>,
) -> Result<()> {
    if declaration.extends_clause().is_some() {
        bail!("Error interface extends clauses are not supported");
    }

    for member in declaration.members() {
        if let Some(lowered) = lower_error_type_member(member)? {
            members.push(lowered);
        }
    }
    Ok(())
}

/// Lowers one supported `Error` instance member.
fn lower_error_type_member(member: AnyTsTypeMember) -> Result<Option<LoweredTypeMember>> {
    match member {
        AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
            let name = lower_object_member_name(property.name()?)?;
            let optional = property.optional_token().is_some();
            let type_reference = property
                .type_annotation()
                .with_context(|| format!("Error member {name} is missing a type annotation"))?
                .ty()
                .with_context(|| format!("Error member {name} has a malformed type annotation"))
                .and_then(|type_node| lower_type_reference(&type_node))?;
            let type_reference = if optional {
                lower_optional_error_member_reference(&name, type_reference)?
            } else {
                type_reference
            };
            Ok(Some(LoweredTypeMember {
                name,
                kind: LoweredMemberKind::Named { optional },
                type_reference,
            }))
        }
        AnyTsTypeMember::TsMethodSignatureTypeMember(_) => {
            bail!("method signatures are not supported in the Error global")
        }
        AnyTsTypeMember::TsCallSignatureTypeMember(_)
        | AnyTsTypeMember::TsConstructSignatureTypeMember(_) => {
            bail!("Error global signatures must be declared on ErrorConstructor")
        }
        AnyTsTypeMember::JsBogusMember(_) => {
            bail!("bogus members are not supported in the Error global")
        }
        AnyTsTypeMember::JsMetavariable(_) => {
            bail!("metavariable members are not supported in the Error global")
        }
        AnyTsTypeMember::TsGetterSignatureTypeMember(_) => {
            bail!("getter signatures are not supported in the Error global")
        }
        AnyTsTypeMember::TsIndexSignatureTypeMember(_) => {
            bail!("index signatures are not supported in the Error global")
        }
        AnyTsTypeMember::TsSetterSignatureTypeMember(_) => {
            bail!("setter signatures are not supported in the Error global")
        }
    }
}

/// Validates supported optional `Error` members.
fn lower_optional_error_member_reference(
    name: &Text,
    type_reference: LoweredTypeReference,
) -> Result<LoweredTypeReference> {
    match (name.text(), type_reference) {
        ("stack", LoweredTypeReference::Predefined("GLOBAL_STRING_KEYWORD_ID")) => {
            Ok(LoweredTypeReference::Predefined("GLOBAL_STRING_KEYWORD_ID"))
        }
        (name, type_reference) => {
            bail!("unsupported optional Error member {name} with type {type_reference:?}")
        }
    }
}

/// Checks that `declare var Error` points at `ErrorConstructor`.
fn ensure_error_value_references_constructor(
    records: &[DeclarationRecord],
    source_cache: &mut ParsedSourceCache,
) -> Result<()> {
    let mut found_value_side = false;
    for record in records {
        match &record.kind {
            DeclarationKind::VariableDeclarator { .. } => {}
            DeclarationKind::DeclareFunction | DeclarationKind::ImportEquals => {
                bail!("unsupported value-side Error declaration {:?}", record.kind)
            }
            DeclarationKind::Interface | DeclarationKind::TypeAlias => {
                continue;
            }
        }
        let declarator = source_cache
            .find_variable_declarator(record)?
            .with_context(|| {
                format!(
                    "failed to find variable declaration {} at {:?}",
                    record.declared_name.text(),
                    record.text_range
                )
            })?;
        let Some(annotation) = declarator.variable_annotation() else {
            bail!("declare var Error is missing a type annotation");
        };
        let AnyTsVariableAnnotation::TsTypeAnnotation(annotation) = annotation else {
            bail!("declare var Error uses unsupported definite assignment annotation");
        };
        let type_reference = lower_type_reference(&annotation.ty()?)?;
        if type_reference != LoweredTypeReference::Predefined("GLOBAL_ERROR_CONSTRUCTOR_ID") {
            bail!("declare var Error must reference ErrorConstructor, got {type_reference:?}");
        }
        found_value_side = true;
    }

    if !found_value_side {
        bail!("Error global must include declare var Error");
    }

    Ok(())
}

/// Lowers signatures and static members from `interface ErrorConstructor`.
fn lower_error_constructor_signatures(
    records: &[DeclarationRecord],
    source_cache: &mut ParsedSourceCache,
) -> Result<ErrorConstructorSignatures> {
    let mut constructor = None;
    let mut call = None;
    let mut prototype = None;

    for record in records {
        match &record.kind {
            DeclarationKind::Interface => {}
            DeclarationKind::TypeAlias => {
                bail!("type aliases are not supported in ErrorConstructor")
            }
            DeclarationKind::DeclareFunction
            | DeclarationKind::VariableDeclarator { .. }
            | DeclarationKind::ImportEquals => {
                bail!("value-side ErrorConstructor declarations are not supported")
            }
        }
        let declaration = source_cache
            .find_interface_declaration(record)?
            .with_context(|| {
                format!(
                    "failed to find interface declaration {} at {:?}",
                    record.declared_name.text(),
                    record.text_range
                )
            })?;
        if declaration.extends_clause().is_some() {
            bail!("ErrorConstructor extends clauses are not supported");
        }

        for member in declaration.members() {
            match member {
                AnyTsTypeMember::TsConstructSignatureTypeMember(member) => {
                    let lowered = lower_construct_signature(&member)?;
                    if constructor.replace(lowered).is_some() {
                        bail!("ErrorConstructor has multiple construct signatures");
                    }
                }
                AnyTsTypeMember::TsCallSignatureTypeMember(member) => {
                    let lowered = lower_call_signature(&member)?;
                    if call.replace(lowered).is_some() {
                        bail!("ErrorConstructor has multiple call signatures");
                    }
                }
                AnyTsTypeMember::TsPropertySignatureTypeMember(member) => {
                    let lowered = lower_error_constructor_property_member(&member)?;
                    if prototype.replace(lowered).is_some() {
                        bail!("ErrorConstructor has multiple prototype properties");
                    }
                }
                AnyTsTypeMember::TsMethodSignatureTypeMember(_) => {
                    bail!("method signatures are not supported in ErrorConstructor")
                }
                AnyTsTypeMember::JsBogusMember(_) => {
                    bail!("bogus members are not supported in ErrorConstructor")
                }
                AnyTsTypeMember::TsGetterSignatureTypeMember(_) => {
                    bail!("getter signatures are not supported in ErrorConstructor")
                }
                AnyTsTypeMember::TsIndexSignatureTypeMember(_) => {
                    bail!("index signatures are not supported in ErrorConstructor")
                }
                AnyTsTypeMember::TsSetterSignatureTypeMember(_) => {
                    bail!("setter signatures are not supported in ErrorConstructor")
                }
                AnyTsTypeMember::JsMetavariable(_) => {
                    bail!("metavariables are not supported in ErrorConstructor")
                }
            }
        }
    }

    Ok(ErrorConstructorSignatures {
        constructor: constructor.context("ErrorConstructor is missing a construct signature")?,
        call: call.context("ErrorConstructor is missing a call signature")?,
        prototype,
    })
}

/// Lowers supported `ErrorConstructor` static properties.
fn lower_error_constructor_property_member(
    property: &TsPropertySignatureTypeMember,
) -> Result<LoweredTypeMember> {
    let name = lower_object_member_name(property.name()?)?;
    if name.text() != "prototype" {
        bail!("unsupported ErrorConstructor property {name}");
    }
    if property.optional_token().is_some() {
        bail!("ErrorConstructor.prototype must not be optional");
    }
    let type_reference = property
        .type_annotation()
        .context("ErrorConstructor.prototype is missing a type annotation")?
        .ty()
        .context("ErrorConstructor.prototype has malformed type annotation")
        .and_then(|type_node| lower_type_reference(&type_node))
        .map(instance_return_reference)?;

    Ok(LoweredTypeMember {
        name,
        kind: LoweredMemberKind::NamedStatic,
        type_reference,
    })
}

/// Lowers the `new Error(...)` construct signature.
fn lower_construct_signature(
    member: &TsConstructSignatureTypeMember,
) -> Result<LoweredConstructor> {
    if member.type_parameters().is_some() {
        bail!("generic Error constructor signatures are not supported");
    }
    let return_type = member
        .type_annotation()
        .context("ErrorConstructor construct signature is missing a return type")?
        .ty()
        .context("ErrorConstructor construct signature has malformed return type")
        .and_then(|type_node| lower_type_reference(&type_node))?;

    Ok(LoweredConstructor {
        type_parameters: Box::default(),
        parameters: lower_parameters(member.parameters()?)?,
        return_type: Some(return_type),
    })
}

/// Lowers the `Error(...)` call signature.
fn lower_call_signature(member: &TsCallSignatureTypeMember) -> Result<LoweredFunction> {
    if member.type_parameters().is_some() {
        bail!("generic Error call signatures are not supported");
    }
    let return_type = member
        .return_type_annotation()
        .context("ErrorConstructor call signature is missing a return type")?
        .ty()
        .context("ErrorConstructor call signature has malformed return type")
        .and_then(|return_type_node| lower_return_type_reference(&return_type_node))?;

    Ok(LoweredFunction {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(Text::from("Error")),
        parameters: lower_parameters(member.parameters()?)?,
        return_type: instance_return_reference(return_type),
    })
}

/// Lowers function-like parameters for the `ErrorConstructor`.
fn lower_parameters(parameters: JsParameters) -> Result<Box<[LoweredFunctionParameter]>> {
    lower_parameters_with(parameters, &mut lower_type_reference)
}

fn lower_parameters_with(
    parameters: JsParameters,
    lower_reference: &mut impl FnMut(&AnyTsType) -> Result<LoweredTypeReference>,
) -> Result<Box<[LoweredFunctionParameter]>> {
    let mut lowered = Vec::new();
    for parameter in parameters.items() {
        match parameter? {
            AnyJsParameter::AnyJsFormalParameter(parameter) => {
                let AnyJsFormalParameter::JsFormalParameter(parameter) = parameter else {
                    bail!("unsupported function formal parameter");
                };
                if !parameter.decorators().is_empty() || parameter.initializer().is_some() {
                    bail!("unsupported function parameter");
                }
                let name = lower_binding_name(parameter.binding()?)?;
                let is_optional = parameter.question_mark_token().is_some();
                let type_reference = parameter
                    .type_annotation()
                    .context("function parameter is missing a type annotation")?
                    .ty()
                    .context("function parameter has malformed type annotation")
                    .and_then(|type_node| lower_reference(&type_node))?;
                lowered.push(LoweredFunctionParameter {
                    binding: LoweredFunctionParameterBinding::Named(name),
                    type_reference,
                    is_optional,
                    is_rest: false,
                });
            }
            AnyJsParameter::JsRestParameter(parameter) => {
                let binding = match parameter.binding()? {
                    AnyJsBindingPattern::JsArrayBindingPattern(_) => {
                        LoweredFunctionParameterBinding::Pattern
                    }
                    binding => LoweredFunctionParameterBinding::Named(lower_binding_name(binding)?),
                };
                let type_reference = parameter
                    .type_annotation()
                    .context("function rest parameter is missing a type annotation")?
                    .ty()
                    .context("function rest parameter has malformed type annotation")
                    .and_then(|type_node| lower_reference(&type_node))?;
                lowered.push(LoweredFunctionParameter {
                    binding,
                    type_reference,
                    is_optional: false,
                    is_rest: true,
                });
            }
            AnyJsParameter::TsThisParameter(_) => {
                bail!("this parameters are not supported in function")
            }
        }
    }

    Ok(lowered.into_boxed_slice())
}

/// Maps a supported TypeScript type node to a lowered reference.
fn lower_type_reference(type_node: &AnyTsType) -> Result<LoweredTypeReference> {
    if let Some(reference) = lower_primitive_reference(type_node) {
        return Ok(reference);
    }
    match type_node {
        AnyTsType::TsReferenceType(reference) => {
            let name = reference.name().context("missing type reference name")?;
            let biome_js_syntax::AnyTsName::JsReferenceIdentifier(identifier) = name else {
                bail!("qualified type references are not supported in Error global")
            };
            let name = Text::from(identifier.value_token()?.token_text_trimmed());
            Ok(match name.text() {
                "Error" => LoweredTypeReference::Predefined("GLOBAL_ERROR_ID"),
                "ErrorConstructor" => {
                    LoweredTypeReference::Predefined("GLOBAL_ERROR_CONSTRUCTOR_ID")
                }
                _ => bail!("unresolved type reference {name} in Error global"),
            })
        }
        _ => bail!("unsupported type reference in Error global: {type_node:?}"),
    }
}

/// Maps a supported return type node to a lowered reference.
fn lower_return_type_reference(return_type_node: &AnyTsReturnType) -> Result<LoweredTypeReference> {
    match return_type_node {
        AnyTsReturnType::AnyTsType(type_node) => lower_type_reference(type_node),
        AnyTsReturnType::TsAssertsReturnType(_) | AnyTsReturnType::TsPredicateReturnType(_) => {
            bail!("predicate return types are not supported in Error global")
        }
    }
}

/// Converts constructor returns from `Error` to `InstanceOf<Error>`.
fn instance_return_reference(reference: LoweredTypeReference) -> LoweredTypeReference {
    match reference {
        LoweredTypeReference::Predefined("GLOBAL_ERROR_ID") => {
            LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_ERROR_ID")
        }
        reference => reference,
    }
}
