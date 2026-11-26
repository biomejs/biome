//! Hand-written projection for `Promise`, whose shape type inference depends on.
//!
//! Type inference and lint rules rely on the reduced shape of `Promise`, including
//! synthetic helper globals such as `Promise.prototype.then`. The override validates the
//! declarations it projects so that a TypeScript change which breaks the projection fails
//! codegen. Every other global is lowered from its declarations by the generic lowerer.

use biome_js_syntax::JsFormalParameter;
use biome_rowan::SyntaxResult;

use super::generic::GenericLowerer;
use super::*;

/// Lowers the `Promise` override when its declarations are present. The first global
/// replaces the generic lowering of the same identity; the rest are helpers.
pub(super) fn lower_overrides(
    manifest: &GlobalManifest,
    lowerer: &mut GenericLowerer<'_>,
) -> Result<Vec<LoweredGlobal>> {
    let mut globals = Vec::new();
    lower_promise_globals(manifest, &mut lowerer.sources, &mut globals)?;
    Ok(globals)
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
            return_kind: LoweredReturnKind::Type,
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
            return_kind: LoweredReturnKind::Type,
        }),
    }
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
