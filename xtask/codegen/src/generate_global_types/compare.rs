//! Structural checks for lowered generated globals.

use anyhow::{Result, bail};

use super::lower::{
    LoweredClass, LoweredConstructor, LoweredFunction, LoweredFunctionParameter,
    LoweredGlobalTypes, LoweredInterface, LoweredMemberKind, LoweredTypeData, LoweredTypeReference,
};

/// Number of `Error` class members expected in generated output.
const ERROR_MEMBER_COUNT: usize = 6;
/// Expected number of lowered generated globals.
const GENERATED_GLOBAL_COUNT: usize = 7;

/// Expected shape of one lowered disposable pair (interface + dispose helper), checked by
/// [`assert_disposable_shape`] against the generated model.
struct DisposableShape<'a> {
    interface_name: &'a str,
    interface_id_constant: &'a str,
    member_name: &'a str,
    symbol_id: &'static str,
    helper_type_id: &'static str,
    helper_name: &'a str,
    helper_id_constant: &'a str,
    helper_is_async: bool,
    return_type_id: &'static str,
}

/// Validates lowered generated globals before they are emitted.
pub fn compare_lowered_globals(lowered: &LoweredGlobalTypes) -> Result<()> {
    if lowered.globals().len() != GENERATED_GLOBAL_COUNT {
        bail!(
            "generated globals contain {} entries, expected {}",
            lowered.globals().len(),
            GENERATED_GLOBAL_COUNT
        );
    }

    let Some(error) = lowered.global("Error") else {
        bail!("generated globals are missing the Error global");
    };
    if error.id_constant() != "ERROR_ID_GLOBAL_TYPE_ID" {
        bail!(
            "generated Error global targets {}, expected ERROR_ID_GLOBAL_TYPE_ID",
            error.id_constant()
        );
    }

    let LoweredTypeData::Class(class) = error.data() else {
        bail!("generated Error global is not a class");
    };
    if class.name() != "Error" {
        bail!(
            "generated Error class has name {}, expected Error",
            class.name()
        );
    }

    assert_error_named_string_member(lowered, "name", false)?;
    assert_error_named_string_member(lowered, "message", false)?;
    assert_error_stack_member(lowered)?;
    assert_error_constructor_member(lowered)?;
    assert_error_call_member(lowered)?;
    assert_error_prototype_member(lowered)?;

    if class.members().len() != ERROR_MEMBER_COUNT {
        bail!(
            "generated Error global has {} members, expected {}",
            class.members().len(),
            ERROR_MEMBER_COUNT
        );
    }

    let constructor = generated_constructor(lowered)?;
    assert_error_constructor_shape(constructor)?;

    let call = generated_function(lowered, "Error.call", "ERROR_CALL_ID_GLOBAL_TYPE_ID")?;
    assert_error_call_shape(call)?;

    assert_disposable_shape(
        lowered,
        DisposableShape {
            interface_name: "Disposable",
            interface_id_constant: "DISPOSABLE_ID_GLOBAL_TYPE_ID",
            member_name: "[Symbol.dispose]",
            symbol_id: "GLOBAL_SYMBOL_DISPOSE_ID",
            helper_type_id: "GLOBAL_DISPOSABLE_DISPOSE_ID",
            helper_name: "Disposable[Symbol.dispose]",
            helper_id_constant: "DISPOSABLE_DISPOSE_ID_GLOBAL_TYPE_ID",
            helper_is_async: false,
            return_type_id: "GLOBAL_VOID_ID",
        },
    )?;
    assert_disposable_shape(
        lowered,
        DisposableShape {
            interface_name: "AsyncDisposable",
            interface_id_constant: "ASYNC_DISPOSABLE_ID_GLOBAL_TYPE_ID",
            member_name: "[Symbol.asyncDispose]",
            symbol_id: "GLOBAL_SYMBOL_ASYNC_DISPOSE_ID",
            helper_type_id: "GLOBAL_ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID",
            helper_name: "AsyncDisposable[Symbol.asyncDispose]",
            helper_id_constant: "ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID_GLOBAL_TYPE_ID",
            helper_is_async: true,
            return_type_id: "GLOBAL_INSTANCEOF_PROMISE_ID",
        },
    )?;

    Ok(())
}

/// Returns the generated `Error` class.
fn generated_error_class(lowered: &LoweredGlobalTypes) -> Result<&LoweredClass> {
    let Some(error) = lowered.global("Error") else {
        bail!("generated globals are missing the Error global");
    };
    let LoweredTypeData::Class(class) = error.data() else {
        bail!("generated Error global is not a class");
    };
    Ok(class)
}

/// Returns the generated constructor helper for `new Error(...)`.
fn generated_constructor(lowered: &LoweredGlobalTypes) -> Result<&LoweredConstructor> {
    let Some(constructor) = lowered.global("Error.constructor") else {
        bail!("generated globals are missing the Error constructor helper");
    };
    if constructor.id_constant() != "ERROR_CONSTRUCTOR_ID_GLOBAL_TYPE_ID" {
        bail!(
            "generated Error constructor targets {}, expected ERROR_CONSTRUCTOR_ID_GLOBAL_TYPE_ID",
            constructor.id_constant()
        );
    }
    let LoweredTypeData::Constructor(constructor) = constructor.data() else {
        bail!("generated Error constructor helper is not constructor data");
    };
    Ok(constructor)
}

/// Returns the lowered interface named `name`, checking it targets `id_constant`.
fn generated_interface<'a>(
    lowered: &'a LoweredGlobalTypes,
    name: &str,
    id_constant: &str,
) -> Result<&'a LoweredInterface> {
    let Some(global) = lowered.global(name) else {
        bail!("generated globals are missing {name}");
    };
    if global.id_constant() != id_constant {
        bail!(
            "generated {name} targets {}, expected {id_constant}",
            global.id_constant()
        );
    }
    let LoweredTypeData::Interface(interface) = global.data() else {
        bail!("generated {name} global is not an interface");
    };
    if interface.name() != name {
        bail!(
            "generated {name} interface has unexpected name {}",
            interface.name()
        );
    }
    Ok(interface)
}

/// Returns the lowered function named `name`, checking it targets `id_constant`.
fn generated_function<'a>(
    lowered: &'a LoweredGlobalTypes,
    name: &str,
    id_constant: &str,
) -> Result<&'a LoweredFunction> {
    let Some(global) = lowered.global(name) else {
        bail!("generated globals are missing {name}");
    };
    if global.id_constant() != id_constant {
        bail!(
            "generated {name} targets {}, expected {id_constant}",
            global.id_constant()
        );
    }
    let LoweredTypeData::Function(function) = global.data() else {
        bail!("generated {name} helper is not function data");
    };
    Ok(function)
}

/// Validates that the generated interface and dispose helper match the expected [`DisposableShape`]:
/// the single computed member keyed by the well-known symbol, and the helper's async flag and return.
fn assert_disposable_shape(lowered: &LoweredGlobalTypes, shape: DisposableShape) -> Result<()> {
    let interface =
        generated_interface(lowered, shape.interface_name, shape.interface_id_constant)?;
    let [member] = interface.members() else {
        bail!(
            "generated {} has {} members, expected one",
            shape.interface_name,
            interface.members().len()
        );
    };
    if member.name() != shape.member_name {
        bail!(
            "generated {} member has unexpected name {}",
            shape.interface_name,
            member.name()
        );
    }
    if member.kind()
        != &(LoweredMemberKind::ComputedValue {
            key_reference: LoweredTypeReference::Predefined(shape.symbol_id),
        })
    {
        bail!(
            "generated {} member has unexpected kind",
            shape.interface_name
        );
    }
    if member.type_reference() != &LoweredTypeReference::Predefined(shape.helper_type_id) {
        bail!(
            "generated {} member has unexpected type",
            shape.interface_name
        );
    }

    let helper = generated_function(lowered, shape.helper_name, shape.helper_id_constant)?;
    if helper.is_async() != shape.helper_is_async {
        bail!(
            "generated {} helper has unexpected async flag",
            shape.helper_name
        );
    }
    if helper.name().is_some() {
        bail!("generated {} helper should be anonymous", shape.helper_name);
    }
    if !helper.parameters().is_empty() {
        bail!(
            "generated {} helper should not have parameters",
            shape.helper_name
        );
    }
    if helper.return_type() != &LoweredTypeReference::Predefined(shape.return_type_id) {
        bail!(
            "generated {} helper has unexpected return type",
            shape.helper_name
        );
    }

    Ok(())
}

/// Validates a required `Error` string field such as `name` or `message`.
fn assert_error_named_string_member(
    lowered: &LoweredGlobalTypes,
    name: &str,
    optional: bool,
) -> Result<()> {
    let class = generated_error_class(lowered)?;
    let Some(member) = class.member(name) else {
        bail!("missing required Error member {name}");
    };
    if member.kind()
        != &(LoweredMemberKind::Named {
            optional,
            readonly: false,
        })
    {
        bail!(
            "generated Error member {} has unexpected kind",
            member.name()
        );
    }
    if member.type_reference() != &LoweredTypeReference::Predefined("GLOBAL_STRING_ID") {
        bail!(
            "generated Error member {} has unexpected type",
            member.name()
        );
    }
    Ok(())
}

/// Validates the optional `Error#stack` member.
fn assert_error_stack_member(lowered: &LoweredGlobalTypes) -> Result<()> {
    let class = generated_error_class(lowered)?;
    let Some(member) = class.member("stack") else {
        bail!("missing required Error member stack");
    };
    if member.kind()
        != &(LoweredMemberKind::Named {
            optional: true,
            readonly: false,
        })
    {
        bail!("generated Error member stack has unexpected kind");
    }
    if member.type_reference() != &LoweredTypeReference::Predefined("GLOBAL_STRING_ID") {
        bail!("generated Error member stack has unexpected type");
    }
    Ok(())
}

/// Validates `ErrorConstructor.prototype`.
fn assert_error_prototype_member(lowered: &LoweredGlobalTypes) -> Result<()> {
    let class = generated_error_class(lowered)?;
    let Some(member) = class.member("prototype") else {
        bail!("missing required Error prototype member");
    };
    if member.kind() != &(LoweredMemberKind::NamedStatic { readonly: true }) {
        bail!("generated Error prototype member has unexpected kind");
    }
    if member.type_reference() != &LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_ERROR_ID") {
        bail!("generated Error prototype member has unexpected type");
    }
    Ok(())
}

/// Validates the `new Error(...)` construct signature member.
fn assert_error_constructor_member(lowered: &LoweredGlobalTypes) -> Result<()> {
    let class = generated_error_class(lowered)?;
    let Some(member) = class.member("constructor") else {
        bail!("missing required Error constructor member");
    };
    if member.kind() != &LoweredMemberKind::Constructor {
        bail!("generated Error constructor member has unexpected kind");
    }
    if member.type_reference() != &LoweredTypeReference::Predefined("GLOBAL_ERROR_CONSTRUCTOR_ID") {
        bail!("generated Error constructor member has unexpected type");
    }
    Ok(())
}

/// Validates the `Error(...)` call signature member.
fn assert_error_call_member(lowered: &LoweredGlobalTypes) -> Result<()> {
    let class = generated_error_class(lowered)?;
    let Some(member) = class.member("call") else {
        bail!("missing required Error call signature member");
    };
    if member.kind() != &LoweredMemberKind::CallSignature {
        bail!("generated Error call signature member has unexpected kind");
    }
    if member.type_reference() != &LoweredTypeReference::Predefined("GLOBAL_ERROR_CALL_ID") {
        bail!("generated Error call signature member has unexpected type");
    }
    Ok(())
}

/// Validates the generated constructor helper shape.
fn assert_error_constructor_shape(constructor: &LoweredConstructor) -> Result<()> {
    assert_single_optional_message_parameter(
        constructor.parameters(),
        "generated Error constructor",
    )?;
    if constructor.return_type() != Some(&LoweredTypeReference::Predefined("GLOBAL_ERROR_ID")) {
        bail!("generated Error constructor has unexpected return type");
    }
    Ok(())
}

/// Validates the generated call helper shape.
fn assert_error_call_shape(call: &LoweredFunction) -> Result<()> {
    if call.is_async() {
        bail!("generated Error call helper should not be async");
    }
    if call.name() != Some("Error") {
        bail!("generated Error call helper has unexpected function name");
    }
    assert_single_optional_message_parameter(call.parameters(), "generated Error call")?;
    if call.return_type() != &LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_ERROR_ID") {
        bail!("generated Error call has unexpected return type");
    }
    Ok(())
}

/// Validates the shared optional `message?: string` parameter.
fn assert_single_optional_message_parameter(
    parameters: &[LoweredFunctionParameter],
    owner: &str,
) -> Result<()> {
    let [parameter] = parameters else {
        bail!(
            "{owner} has {} parameters, expected one message parameter",
            parameters.len()
        );
    };
    if parameter.name() != "message" {
        bail!("{owner} has unexpected parameter name {}", parameter.name());
    }
    if parameter.type_reference() != &LoweredTypeReference::Predefined("GLOBAL_STRING_ID") {
        bail!("{owner} message parameter has unexpected type");
    }
    if !parameter.is_optional() {
        bail!("{owner} message parameter must be optional");
    }
    if parameter.is_rest() {
        bail!("{owner} message parameter must not be rest");
    }
    Ok(())
}
