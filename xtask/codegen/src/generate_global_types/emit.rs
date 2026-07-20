use std::path::Path;

use anyhow::{Result, bail};

use super::lower::{
    LoweredClass, LoweredConstructor, LoweredFunction, LoweredFunctionParameter, LoweredGlobal,
    LoweredGlobalTypes, LoweredInterface, LoweredMemberKind, LoweredTypeData, LoweredTypeMember,
    LoweredTypeReference,
};

/// Relative path of the generated global types module from the workspace root.
const OUTPUT_RELATIVE_PATH: &str = "crates/biome_js_type_info/src/generated/global_types.rs";

/// Stable generated global emission order for generated globals with fixed IDs.
///
/// Must stay ordered by ascending `GlobalTypeId` index so the emitted
/// `MIGRATED_PREDEFINED_IDS` stays sorted for the runtime `binary_search`.
const GLOBAL_ID_EMIT_ORDER: [&str; 7] = [
    "DISPOSABLE_ID_GLOBAL_TYPE_ID",
    "DISPOSABLE_DISPOSE_ID_GLOBAL_TYPE_ID",
    "ASYNC_DISPOSABLE_ID_GLOBAL_TYPE_ID",
    "ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID_GLOBAL_TYPE_ID",
    "ERROR_ID_GLOBAL_TYPE_ID",
    "ERROR_CONSTRUCTOR_ID_GLOBAL_TYPE_ID",
    "ERROR_CALL_ID_GLOBAL_TYPE_ID",
];

/// Emits the global types module with LF-normalized output.
pub(super) fn emit_global_types(
    pin: &crate::generate_global_types::SourcePin,
    workspace_root: &Path,
    lowered: &LoweredGlobalTypes,
) -> anyhow::Result<crate::UpdateResult> {
    let path = workspace_root.join(OUTPUT_RELATIVE_PATH);
    let formatted =
        xtask_glue::reformat_with_command(generated_body(pin, lowered)?, "just gen-global-types")?;
    crate::update(&path, &formatted, &xtask_glue::Mode::Overwrite)
}

/// Renders the unformatted Rust body for the generated module.
fn generated_body(
    pin: &crate::generate_global_types::SourcePin,
    lowered: &LoweredGlobalTypes,
) -> Result<String> {
    let migrated_ids = render_migrated_ids(lowered)?;
    let registrations = render_registrations(lowered)?;

    Ok(format!(
        r#"// Generated from microsoft/TypeScript {typescript_tag} (git commit {typescript_sha}).

/// Predefined global IDs whose `TypeData` is supplied by this generated module.
pub(crate) const MIGRATED_PREDEFINED_IDS: &[crate::globals::GlobalTypeId] = &[
{migrated_ids}];

/// Registers all generated global type data into the resolver builder.
pub(crate) fn set_generated_global_type_data(builder: &mut crate::globals_builder::GlobalsResolverBuilder) {{
{registrations}}}
"#,
        typescript_tag = pin.tag(),
        typescript_sha = pin.sha(),
    ))
}

/// Builds the migrated ID slice body.
fn render_migrated_ids(lowered: &LoweredGlobalTypes) -> Result<String> {
    let mut ids = String::new();
    for global in sorted_globals(lowered)? {
        ids.push_str("    crate::globals::");
        ids.push_str(global.id_constant());
        ids.push_str(",\n");
    }
    Ok(ids)
}

/// Builds the resolver registration statements.
fn render_registrations(lowered: &LoweredGlobalTypes) -> Result<String> {
    let mut registrations = String::new();
    for global in sorted_globals(lowered)? {
        registrations.push_str("    let data = ");
        registrations.push_str(&render_type_data(global.data()));
        registrations.push_str(";\n");
        registrations.push_str("    builder.set_type_data(crate::globals::");
        registrations.push_str(global.id_constant());
        registrations.push_str(", data);\n");
    }
    Ok(registrations)
}

/// Dispatches lowered data to its Rust expression renderer.
fn render_type_data(data: &LoweredTypeData) -> String {
    match data {
        LoweredTypeData::Class(class) => render_class(class),
        LoweredTypeData::Constructor(constructor) => render_constructor(constructor),
        LoweredTypeData::Function(function) => render_function(function),
        LoweredTypeData::Interface(interface) => render_interface(interface),
    }
}

/// Builds a `TypeData::Class` expression.
fn render_class(class: &LoweredClass) -> String {
    format!(
        "crate::TypeData::Class(Box::new(crate::Class {{
            name: Some(biome_rowan::Text::new_static({name})),
            type_parameters: Box::default(),
            extends: None,
            implements: Box::default(),
            members: Box::new([{members}]),
        }}))",
        name = rust_string_literal(class.name()),
        members = render_members(class.members()),
    )
}

/// Builds a `TypeData::Interface` expression.
fn render_interface(interface: &LoweredInterface) -> String {
    format!(
        "crate::TypeData::Interface(Box::new(crate::Interface {{
            name: biome_rowan::Text::new_static({name}),
            type_parameters: Box::default(),
            extends: Box::default(),
            members: Box::new([{members}]),
        }}))",
        name = rust_string_literal(interface.name()),
        members = render_members(interface.members()),
    )
}

/// Builds a `TypeData::Constructor` expression.
fn render_constructor(constructor: &LoweredConstructor) -> String {
    let return_type = constructor.return_type().map_or_else(
        || "None".to_string(),
        |return_type| format!("Some({})", render_type_reference(return_type)),
    );

    format!(
        "crate::TypeData::Constructor(Box::new(crate::Constructor {{
            type_parameters: Box::default(),
            parameters: Box::new([{parameters}]),
            return_type: {return_type},
        }}))",
        parameters = render_constructor_parameters(constructor.parameters()),
    )
}

/// Builds a `TypeData::Function` expression.
fn render_function(function: &LoweredFunction) -> String {
    let name = function.name().map_or_else(
        || "None".to_string(),
        |name| {
            format!(
                "Some(biome_rowan::Text::new_static({}))",
                rust_string_literal(name)
            )
        },
    );

    format!(
        "crate::TypeData::Function(Box::new(crate::Function {{
            is_async: {is_async},
            type_parameters: Box::default(),
            name: {name},
            parameters: Box::new([{parameters}]),
            return_type: crate::ReturnType::Type({return_type}),
        }}))",
        is_async = function.is_async(),
        parameters = render_function_parameters(function.parameters()),
        return_type = render_type_reference(function.return_type()),
    )
}

/// Builds the class member expression list.
fn render_members(members: &[LoweredTypeMember]) -> String {
    let mut rendered = String::new();
    for member in members {
        rendered.push_str(&render_member(member));
        rendered.push(',');
    }
    rendered
}

/// Builds one `TypeMember` expression.
fn render_member(member: &LoweredTypeMember) -> String {
    format!(
        "crate::TypeMember {{
            kind: {kind},
            ty: {type_reference},
        }}",
        kind = render_member_kind(member),
        type_reference = render_type_reference(member.type_reference()),
    )
}

/// Builds the `TypeMemberKind` expression for a member.
fn render_member_kind(member: &LoweredTypeMember) -> String {
    match member.kind() {
        LoweredMemberKind::Named {
            optional: false,
            readonly: false,
        } => {
            format!(
                "crate::TypeMemberKind::Named(biome_rowan::Text::new_static({}))",
                rust_string_literal(member.name()),
            )
        }
        LoweredMemberKind::Named {
            optional: true,
            readonly: false,
        } => {
            format!(
                "crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static({}))",
                rust_string_literal(member.name()),
            )
        }
        LoweredMemberKind::Named {
            optional: false,
            readonly: true,
        } => {
            format!(
                "crate::TypeMemberKind::ReadonlyNamed(biome_rowan::Text::new_static({}))",
                rust_string_literal(member.name()),
            )
        }
        LoweredMemberKind::Named {
            optional: true,
            readonly: true,
        } => {
            format!(
                "crate::TypeMemberKind::ReadonlyNamedOptional(biome_rowan::Text::new_static({}))",
                rust_string_literal(member.name()),
            )
        }
        LoweredMemberKind::NamedStatic { readonly: false } => {
            format!(
                "crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static({}))",
                rust_string_literal(member.name()),
            )
        }
        LoweredMemberKind::NamedStatic { readonly: true } => {
            format!(
                "crate::TypeMemberKind::ReadonlyNamedStatic(biome_rowan::Text::new_static({}))",
                rust_string_literal(member.name()),
            )
        }
        LoweredMemberKind::Constructor => "crate::TypeMemberKind::Constructor".to_string(),
        LoweredMemberKind::CallSignature => "crate::TypeMemberKind::CallSignature".to_string(),
        LoweredMemberKind::ComputedValue { key_reference } => {
            format!(
                "crate::TypeMemberKind::ComputedValue({})",
                render_type_reference(key_reference)
            )
        }
    }
}

/// Builds constructor parameter expressions.
fn render_constructor_parameters(parameters: &[LoweredFunctionParameter]) -> String {
    let mut rendered = String::new();
    for parameter in parameters {
        rendered.push_str("crate::ConstructorParameter { parameter: ");
        rendered.push_str(&render_function_parameter(parameter));
        rendered.push_str(", accessibility: None, is_readonly: false },");
    }
    rendered
}

/// Builds function parameter expressions.
fn render_function_parameters(parameters: &[LoweredFunctionParameter]) -> String {
    let mut rendered = String::new();
    for parameter in parameters {
        rendered.push_str(&render_function_parameter(parameter));
        rendered.push(',');
    }
    rendered
}

/// Builds one named parameter expression.
fn render_function_parameter(parameter: &LoweredFunctionParameter) -> String {
    format!(
        "crate::FunctionParameter::Named(crate::NamedFunctionParameter {{
            name: biome_rowan::Text::new_static({name}),
            ty: {type_reference},
            is_optional: {is_optional},
            is_rest: {is_rest},
        }})",
        name = rust_string_literal(parameter.name()),
        type_reference = render_type_reference(parameter.type_reference()),
        is_optional = parameter.is_optional(),
        is_rest = parameter.is_rest(),
    )
}

/// Builds a generated `TypeReference` expression.
fn render_type_reference(reference: &LoweredTypeReference) -> String {
    match reference {
        LoweredTypeReference::Predefined(id) => {
            format!("crate::globals::{id}.into()")
        }
    }
}

/// Quotes a string for generated Rust source.
fn rust_string_literal(value: &str) -> String {
    format!("{value:?}")
}

fn global_with_id_constant<'a>(
    lowered: &'a LoweredGlobalTypes,
    id_constant: &str,
) -> Result<&'a LoweredGlobal> {
    for global in lowered.globals() {
        if global.id_constant() == id_constant {
            return Ok(global);
        }
    }

    bail!("generated global output is missing {id_constant}");
}

/// Returns generated globals in the sorted predefined-ID order.
fn sorted_globals(lowered: &LoweredGlobalTypes) -> Result<[&LoweredGlobal; 7]> {
    for global in lowered.globals() {
        if !GLOBAL_ID_EMIT_ORDER.contains(&global.id_constant()) {
            bail!(
                "generated global {} targets {}, but the ID is missing from GLOBAL_ID_EMIT_ORDER",
                global.name(),
                global.id_constant()
            );
        }
    }

    Ok([
        global_with_id_constant(lowered, GLOBAL_ID_EMIT_ORDER[0])?,
        global_with_id_constant(lowered, GLOBAL_ID_EMIT_ORDER[1])?,
        global_with_id_constant(lowered, GLOBAL_ID_EMIT_ORDER[2])?,
        global_with_id_constant(lowered, GLOBAL_ID_EMIT_ORDER[3])?,
        global_with_id_constant(lowered, GLOBAL_ID_EMIT_ORDER[4])?,
        global_with_id_constant(lowered, GLOBAL_ID_EMIT_ORDER[5])?,
        global_with_id_constant(lowered, GLOBAL_ID_EMIT_ORDER[6])?,
    ])
}
