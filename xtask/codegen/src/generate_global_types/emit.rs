use std::path::Path;

use biome_string_case::StrLikeExtension;

use super::lower::{
    LoweredClass, LoweredConstructor, LoweredFunction, LoweredFunctionParameter,
    LoweredFunctionParameterBinding, LoweredGlobal, LoweredGlobalTypes, LoweredInterface,
    LoweredMemberKind, LoweredTypeData, LoweredTypeMember, LoweredTypeReference,
};

/// Relative path of the generated global types module from the workspace root.
const OUTPUT_RELATIVE_PATH: &str = "crates/biome_js_type_info/src/generated/global_types.rs";

/// Emits the global types module with LF-normalized output.
pub(super) fn emit_global_types(
    pin: &crate::generate_global_types::SourcePin,
    workspace_root: &Path,
    lowered: &LoweredGlobalTypes,
) -> anyhow::Result<crate::UpdateResult> {
    let path = workspace_root.join(OUTPUT_RELATIVE_PATH);
    let formatted = xtask_glue::reformat_with_command(
        render_global_types(pin, lowered),
        "just gen-global-types",
    )?;
    crate::update(&path, &formatted, &xtask_glue::Mode::Overwrite)
}

/// Renders the unformatted Rust source of the generated module.
pub fn render_global_types(
    pin: &crate::generate_global_types::SourcePin,
    lowered: &LoweredGlobalTypes,
) -> String {
    let globals = lowered.globals();
    let mut ids = String::new();
    let mut names = String::new();
    let mut builder_table = String::new();
    let mut builders = String::new();
    let mut type_globals = Vec::new();
    let mut value_globals = Vec::new();
    for (index, global) in globals.iter().enumerate() {
        let id = global.id_constant();
        let reference = global.reference_constant();
        let offset = if index == 0 {
            String::new()
        } else {
            format!(" + {index}")
        };
        ids.push_str(&format!(
            "pub(crate) const {id}: GlobalTypeId = GlobalTypeId::new(TypeId::new(FIRST_GENERATED_ID{offset}));\n\
             pub(crate) const {reference}: RawTypeId = RawTypeId::Global({id});\n"
        ));
        names.push_str(&format!("{:?},\n", global.name()));
        let builder = builder_name(global);
        builder_table.push_str(&format!("{builder},\n"));
        builders.push_str(&format!(
            "fn {builder}() -> crate::TypeData {{ {} }}\n",
            render_type_data(global.data())
        ));
        let roles = global.roles();
        if roles.type_name {
            type_globals.push((global.name(), id));
        }
        if roles.value_name {
            value_globals.push((global.name(), id));
        }
    }
    let local_types = render_local_types(globals);

    format!(
        r#"// Generated from microsoft/TypeScript {typescript_tag} (git commit {typescript_sha}).

/// Identities of generated globals, allocated after the hand-written manifest.
#[expect(dead_code, reason = "Rust code only names the globals it inspects")]
pub(crate) mod ids {{
    use crate::globals::GlobalTypeId;
    use crate::{{RawTypeId, TypeId}};

    const FIRST_GENERATED_ID: usize = crate::globals_ids::PREDEFINED_ID_ROWS.len();

    {ids}
}}

/// Names of generated globals in ID order.
pub(crate) const GENERATED_GLOBAL_NAMES: &[&str] = &[{names}];

/// Globals that type annotations can name, sorted by name.
pub(crate) const TYPE_GLOBALS: &[(&str, crate::globals::GlobalTypeId)] = &[{type_globals}];

/// Globals that expressions can name, sorted by name.
pub(crate) const VALUE_GLOBALS: &[(&str, crate::globals::GlobalTypeId)] = &[{value_globals}];

/// Builds each generated global's type data, in ID order after the manifest.
pub(crate) static GENERATED_GLOBAL_BUILDERS: [fn() -> crate::TypeData; {builder_count}] = [{builder_table}];

{builders}

{local_types}
"#,
        typescript_tag = pin.tag(),
        builder_count = globals.len(),
        typescript_sha = pin.sha(),
        type_globals = render_name_index(type_globals),
        value_globals = render_name_index(value_globals),
    )
}

/// Renders a name index sorted for binary search.
fn render_name_index(mut entries: Vec<(&str, &str)>) -> String {
    entries.sort_unstable();
    entries
        .into_iter()
        .map(|(name, id)| format!("({name:?}, ids::{id}),\n"))
        .collect()
}

/// Name of the function that builds a global's type data.
fn builder_name(global: &LoweredGlobal) -> String {
    format!(
        "global_{}",
        global
            .id_constant()
            .trim_end_matches("_ID_GLOBAL_TYPE_ID")
            .to_ascii_lowercase_cow()
    )
}

pub(super) fn render_local_types(globals: &[LoweredGlobal]) -> String {
    let mut arrays = String::new();
    let mut arms = String::new();
    for global in globals
        .iter()
        .filter(|global| !global.local_types().is_empty())
    {
        let id = global.id_constant();
        let name = format!("{}_LOCAL_TYPES", id.trim_end_matches("_ID_GLOBAL_TYPE_ID"));
        let count = global.local_types().len();
        let types = global
            .local_types()
            .iter()
            .map(render_type_data)
            .collect::<Vec<_>>()
            .join(",\n");
        arrays.push_str(&format!(
            "static {name}: std::sync::LazyLock<[crate::TypeData; {count}]> = std::sync::LazyLock::new(|| [{types}]);\n"
        ));
        arms.push_str(&format!("ids::{id} => &*{name},\n"));
    }
    format!(
        "{arrays}
        /// Supporting types in dependency order, indexed relative to their owning global.
        pub(crate) fn generated_local_types(owner: crate::globals::GlobalTypeId) -> &'static [crate::TypeData] {{
            match owner {{ {arms} _ => &[] }}
        }}"
    )
}

/// Dispatches lowered data to its Rust expression renderer.
fn render_type_data(data: &LoweredTypeData) -> String {
    match data {
        LoweredTypeData::AnyKeyword => "crate::TypeData::AnyKeyword".to_string(),
        LoweredTypeData::BigInt => "crate::TypeData::BigInt".to_string(),
        LoweredTypeData::BigIntLiteral(value) => format!(
            "crate::TypeData::Literal(Box::new(crate::Literal::BigInt(biome_rowan::Text::new_static({}))))",
            rust_string_literal(value.text()),
        ),
        LoweredTypeData::Boolean => "crate::TypeData::Boolean".to_string(),
        LoweredTypeData::BooleanLiteral(value) => {
            format!("crate::TypeData::Literal(Box::new(crate::Literal::Boolean({value}.into())))")
        }
        LoweredTypeData::NeverKeyword => "crate::TypeData::NeverKeyword".to_string(),
        LoweredTypeData::Null => "crate::TypeData::Null".to_string(),
        LoweredTypeData::Object(members) => format!(
            "crate::TypeData::from(crate::Object {{ prototype: None, members: Box::new([{}]), has_unknown_members: false }})",
            render_members(members),
        ),
        LoweredTypeData::ObjectKeyword => "crate::TypeData::ObjectKeyword".to_string(),
        LoweredTypeData::NumberLiteral(value) => format!(
            "crate::TypeData::Literal(Box::new(crate::Literal::Number(crate::literal::NumberLiteral::new(biome_rowan::Text::new_static({})))))",
            rust_string_literal(value.text()),
        ),
        LoweredTypeData::Class(class) => render_class(class),
        LoweredTypeData::Constructor(constructor) => render_constructor(constructor),
        LoweredTypeData::Function(function) => render_function(function),
        LoweredTypeData::Interface(interface) => render_interface(interface),
        LoweredTypeData::StringLiteral(value) => format!(
            "crate::TypeData::Literal(Box::new(crate::Literal::String(biome_rowan::Text::new_static({}).into())))",
            rust_string_literal(value.text()),
        ),
        LoweredTypeData::Union(types) => format!(
            "crate::TypeData::Union(Box::new(crate::Union({})))",
            render_type_references(types),
        ),
        LoweredTypeData::Tuple(elements) => format!(
            "crate::TypeData::from(crate::Tuple {{ elements: Box::new([{}]), is_inferred_array: false }})",
            elements
                .iter()
                .map(|element| format!(
                    "crate::TupleElementType {{ ty: {}, name: {}, is_optional: {}, is_rest: {} }}",
                    render_type_reference(element.ty()),
                    element.name().map_or_else(
                        || "None".to_string(),
                        |name| format!(
                            "Some(biome_rowan::Text::new_static({}))",
                            rust_string_literal(name)
                        )
                    ),
                    element.is_optional(),
                    element.is_rest(),
                ))
                .collect::<Vec<_>>()
                .join(","),
        ),
        LoweredTypeData::Intersection(types) => format!(
            "crate::TypeData::Intersection(Box::new(crate::Intersection({})))",
            render_type_references(types),
        ),
        LoweredTypeData::Reference(reference) => format!(
            "crate::TypeData::Reference({})",
            render_type_reference(reference),
        ),
        LoweredTypeData::Symbol => "crate::TypeData::Symbol".to_string(),
        LoweredTypeData::Readonly(ty) => format!(
            "crate::TypeData::TypeOperator(Box::new(crate::TypeOperatorType {{ operator: crate::TypeOperator::Readonly, ty: {} }}))",
            render_type_reference(ty),
        ),
        LoweredTypeData::Keyof(ty) => format!(
            "crate::TypeData::TypeOperator(Box::new(crate::TypeOperatorType {{ operator: crate::TypeOperator::Keyof, ty: {} }}))",
            render_type_reference(ty),
        ),
        LoweredTypeData::IndexedAccess { object, index } => format!(
            "crate::TypeData::IndexedAccess(Box::new(crate::IndexedAccessType {{ object: {}, index: {} }}))",
            render_type_reference(object),
            render_type_reference(index),
        ),
        LoweredTypeData::Undefined => "crate::TypeData::Undefined".to_string(),
        LoweredTypeData::InstanceOf {
            ty,
            type_parameters,
        } => format!(
            "crate::TypeData::instance_of(crate::TypeInstance {{ ty: {}, type_parameters: {} }})",
            render_type_reference(ty),
            render_type_references(type_parameters),
        ),
        LoweredTypeData::GenericParameter {
            is_const,
            name,
            constraint,
            default,
        } => format!(
            "crate::TypeData::from(crate::GenericTypeParameter {{ is_const: {is_const}, name: biome_rowan::Text::new_static({}), constraint: {}, default: {} }})",
            rust_string_literal(name.text()),
            constraint.as_ref().map_or_else(
                || "crate::TypeReference::unknown()".to_string(),
                render_type_reference
            ),
            default.as_ref().map_or_else(
                || "crate::TypeReference::unknown()".to_string(),
                render_type_reference
            ),
        ),
        LoweredTypeData::ThisKeyword => "crate::TypeData::ThisKeyword".to_string(),
        LoweredTypeData::UnknownKeyword => "crate::TypeData::UnknownKeyword".to_string(),
    }
}

/// Builds a `TypeData::Class` expression.
fn render_class(class: &LoweredClass) -> String {
    format!(
        "crate::TypeData::Class(Box::new(crate::Class {{
            name: Some(biome_rowan::Text::new_static({name})),
            type_parameters: {type_parameters},
            extends: {extends},
            implements: {implements},
            members: Box::new([{members}]),
        }}))",
        name = rust_string_literal(class.name()),
        type_parameters = render_type_references(class.type_parameters()),
        extends = class.extends().map_or_else(
            || "None".to_string(),
            |extends| format!("Some({})", render_type_reference(extends))
        ),
        implements = render_type_references(class.implements()),
        members = render_members(class.members()),
    )
}

/// Builds a boxed type-reference expression list.
fn render_type_references(references: &[LoweredTypeReference]) -> String {
    if references.is_empty() {
        return "Box::default()".to_string();
    }

    let mut rendered = String::from("Box::new([");
    for reference in references {
        rendered.push_str(&render_type_reference(reference));
        rendered.push(',');
    }
    rendered.push_str("])");
    rendered
}

/// Builds a `TypeData::Interface` expression.
fn render_interface(interface: &LoweredInterface) -> String {
    format!(
        "crate::TypeData::Interface(Box::new(crate::Interface {{
            name: biome_rowan::Text::new_static({name}),
            type_parameters: {type_parameters},
            extends: {extends},
            members: Box::new([{members}]),
        }}))",
        name = rust_string_literal(interface.name()),
        type_parameters = render_type_references(interface.type_parameters()),
        extends = render_type_references(interface.extends()),
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
            type_parameters: {type_parameters},
            parameters: Box::new([{parameters}]),
            return_type: {return_type},
        }}))",
        type_parameters = render_type_references(constructor.type_parameters()),
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
            type_parameters: {type_parameters},
            name: {name},
            parameters: Box::new([{parameters}]),
            return_type: crate::ReturnType::Type({return_type}),
        }}))",
        is_async = function.is_async(),
        type_parameters = render_type_references(function.type_parameters()),
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
        LoweredMemberKind::Named { optional: false } => {
            format!(
                "crate::TypeMemberKind::Named(biome_rowan::Text::new_static({}))",
                rust_string_literal(member.name()),
            )
        }
        LoweredMemberKind::Named { optional: true } => {
            format!(
                "crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static({}))",
                rust_string_literal(member.name()),
            )
        }
        LoweredMemberKind::NamedStatic => {
            format!(
                "crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static({}))",
                rust_string_literal(member.name()),
            )
        }
        LoweredMemberKind::IndexSignature { key_reference } => format!(
            "crate::TypeMemberKind::IndexSignature({})",
            render_type_reference(key_reference),
        ),
        LoweredMemberKind::Constructor => "crate::TypeMemberKind::Constructor".to_string(),
        LoweredMemberKind::CallSignature => "crate::TypeMemberKind::CallSignature".to_string(),
        LoweredMemberKind::ComputedStatic { key_reference } => {
            format!(
                "crate::TypeMemberKind::ComputedStatic({})",
                render_type_reference(key_reference)
            )
        }
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
        rendered.push_str(", accessibility: None },");
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

/// Preserves named bindings and uses pattern parameters for synthetic callback slots.
fn render_function_parameter(parameter: &LoweredFunctionParameter) -> String {
    match parameter.binding() {
        LoweredFunctionParameterBinding::Named(name) => {
            format!(
                "crate::FunctionParameter::Named(crate::NamedFunctionParameter {{
                    name: biome_rowan::Text::new_static({name}),
                    ty: {type_reference},
                    is_optional: {is_optional},
                    is_rest: {is_rest},
                }})",
                name = rust_string_literal(name.text()),
                type_reference = render_type_reference(parameter.type_reference()),
                is_optional = parameter.is_optional(),
                is_rest = parameter.is_rest(),
            )
        }
        LoweredFunctionParameterBinding::Pattern => {
            format!(
                "crate::FunctionParameter::Pattern(crate::PatternFunctionParameter {{
                    bindings: Box::default(),
                    ty: {type_reference},
                    is_optional: {is_optional},
                    is_rest: {is_rest},
                }})",
                type_reference = render_type_reference(parameter.type_reference()),
                is_optional = parameter.is_optional(),
                is_rest = parameter.is_rest(),
            )
        }
    }
}

/// Builds a generated `TypeReference` expression.
fn render_type_reference(reference: &LoweredTypeReference) -> String {
    match reference {
        LoweredTypeReference::Local(index) => {
            format!("crate::RawTypeId::Local(crate::TypeId::new({index})).into()")
        }
        LoweredTypeReference::Predefined(id) => {
            format!("crate::globals::{id}.into()")
        }
        LoweredTypeReference::Global(id) => {
            format!("crate::globals::{id}.into()")
        }
    }
}

/// Quotes a string for generated Rust source.
fn rust_string_literal(value: &str) -> String {
    format!("{value:?}")
}
