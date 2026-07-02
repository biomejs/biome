//! Structural checks for lowered generated globals.

use anyhow::{Result, bail};

use super::lower::{
    LoweredClass, LoweredConstructor, LoweredFunction, LoweredFunctionParameter,
    LoweredGlobalTypes, LoweredMemberKind, LoweredTypeData, LoweredTypeReference,
};

/// Number of `Error` class members expected in generated output.
const ERROR_MEMBER_COUNT: usize = 6;
/// Number of generated globals expected for `Error`.
const ERROR_GLOBAL_COUNT: usize = 3;

/// Validates lowered generated globals before they are emitted.
pub fn compare_lowered_globals(lowered: &LoweredGlobalTypes) -> Result<()> {
    if lowered.globals().len() != ERROR_GLOBAL_COUNT {
        bail!(
            "generated Error global has {} globals, expected {}",
            lowered.globals().len(),
            ERROR_GLOBAL_COUNT
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

    let call = generated_call(lowered)?;
    assert_error_call_shape(call)?;

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

/// Returns the generated call helper for `Error(...)`.
fn generated_call(lowered: &LoweredGlobalTypes) -> Result<&LoweredFunction> {
    let Some(call) = lowered.global("Error.call") else {
        bail!("generated globals are missing the Error call helper");
    };
    if call.id_constant() != "ERROR_CALL_ID_GLOBAL_TYPE_ID" {
        bail!(
            "generated Error call targets {}, expected ERROR_CALL_ID_GLOBAL_TYPE_ID",
            call.id_constant()
        );
    }
    let LoweredTypeData::Function(call) = call.data() else {
        bail!("generated Error call helper is not function data");
    };
    Ok(call)
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
    if member.kind() != &(LoweredMemberKind::Named { optional }) {
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
    if member.kind() != &(LoweredMemberKind::Named { optional: true }) {
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
    if member.kind() != &LoweredMemberKind::NamedStatic {
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
