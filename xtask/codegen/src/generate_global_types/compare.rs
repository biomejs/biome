//! Structural checks for the lowered `Promise` projection.

use anyhow::{Result, bail};

use super::lower::{
    LoweredClass, LoweredFunction, LoweredFunctionParameterBinding, LoweredGlobalTypes,
    LoweredMemberKind, LoweredTypeData, LoweredTypeReference,
};

const PROMISE_MEMBER_COUNT: usize = 11;
const PROMISE_TYPE_PARAMETERS: &[LoweredTypeReference] =
    &[LoweredTypeReference::Predefined("GLOBAL_T_ID")];

#[derive(Clone, Copy)]
struct PromiseMethodShape {
    member_name: &'static str,
    member_type_id: &'static str,
    helper_name: &'static str,
    helper_id_constant: &'static str,
    is_static: bool,
}

const PROMISE_METHOD_SHAPES: &[PromiseMethodShape] = &[
    PromiseMethodShape {
        member_name: "catch",
        member_type_id: "GLOBAL_PROMISE_CATCH_ID",
        helper_name: "Promise.prototype.catch",
        helper_id_constant: "PROMISE_CATCH_ID_GLOBAL_TYPE_ID",
        is_static: false,
    },
    PromiseMethodShape {
        member_name: "finally",
        member_type_id: "GLOBAL_PROMISE_FINALLY_ID",
        helper_name: "Promise.prototype.finally",
        helper_id_constant: "PROMISE_FINALLY_ID_GLOBAL_TYPE_ID",
        is_static: false,
    },
    PromiseMethodShape {
        member_name: "then",
        member_type_id: "GLOBAL_PROMISE_THEN_ID",
        helper_name: "Promise.prototype.then",
        helper_id_constant: "PROMISE_THEN_ID_GLOBAL_TYPE_ID",
        is_static: false,
    },
    PromiseMethodShape {
        member_name: "all",
        member_type_id: "GLOBAL_PROMISE_ALL_ID",
        helper_name: "Promise.all",
        helper_id_constant: "PROMISE_ALL_ID_GLOBAL_TYPE_ID",
        is_static: true,
    },
    PromiseMethodShape {
        member_name: "allSettled",
        member_type_id: "GLOBAL_PROMISE_ALL_SETTLED_ID",
        helper_name: "Promise.allSettled",
        helper_id_constant: "PROMISE_ALL_SETTLED_ID_GLOBAL_TYPE_ID",
        is_static: true,
    },
    PromiseMethodShape {
        member_name: "any",
        member_type_id: "GLOBAL_PROMISE_ANY_ID",
        helper_name: "Promise.any",
        helper_id_constant: "PROMISE_ANY_ID_GLOBAL_TYPE_ID",
        is_static: true,
    },
    PromiseMethodShape {
        member_name: "race",
        member_type_id: "GLOBAL_PROMISE_RACE_ID",
        helper_name: "Promise.race",
        helper_id_constant: "PROMISE_RACE_ID_GLOBAL_TYPE_ID",
        is_static: true,
    },
    PromiseMethodShape {
        member_name: "reject",
        member_type_id: "GLOBAL_PROMISE_REJECT_ID",
        helper_name: "Promise.reject",
        helper_id_constant: "PROMISE_REJECT_ID_GLOBAL_TYPE_ID",
        is_static: true,
    },
    PromiseMethodShape {
        member_name: "resolve",
        member_type_id: "GLOBAL_PROMISE_RESOLVE_ID",
        helper_name: "Promise.resolve",
        helper_id_constant: "PROMISE_RESOLVE_ID_GLOBAL_TYPE_ID",
        is_static: true,
    },
    PromiseMethodShape {
        member_name: "try",
        member_type_id: "GLOBAL_PROMISE_TRY_ID",
        helper_name: "Promise.try",
        helper_id_constant: "PROMISE_TRY_ID_GLOBAL_TYPE_ID",
        is_static: true,
    },
];

/// Validates the lowered `Promise` projection before it is emitted.
pub fn compare_lowered_globals(lowered: &LoweredGlobalTypes) -> Result<()> {
    assert_promise_shape(lowered)
}

/// Rejects lowered Promise data that would change the predefined resolver projection.
fn assert_promise_shape(lowered: &LoweredGlobalTypes) -> Result<()> {
    let Some(promise) = lowered.global("Promise") else {
        bail!("generated globals are missing the Promise global");
    };
    if promise.id_constant() != "PROMISE_ID_GLOBAL_TYPE_ID" {
        bail!(
            "generated Promise global targets {}, expected PROMISE_ID_GLOBAL_TYPE_ID",
            promise.id_constant()
        );
    }
    let LoweredTypeData::Class(class) = promise.data() else {
        bail!("generated Promise global is not a class");
    };
    if class.name() != "Promise" {
        bail!(
            "generated Promise class has name {}, expected Promise",
            class.name()
        );
    }
    if class.type_parameters() != PROMISE_TYPE_PARAMETERS {
        bail!("generated Promise global has unexpected type parameters");
    }
    if class.members().len() != PROMISE_MEMBER_COUNT {
        bail!(
            "generated Promise global has {} members, expected {PROMISE_MEMBER_COUNT}",
            class.members().len()
        );
    }

    let Some(constructor_member) = class.member("constructor") else {
        bail!("generated Promise global is missing its constructor");
    };
    if constructor_member.kind() != &LoweredMemberKind::Constructor
        || constructor_member.type_reference()
            != &LoweredTypeReference::Predefined("GLOBAL_PROMISE_CONSTRUCT_ID")
    {
        bail!("generated Promise constructor member has unexpected shape");
    }

    let constructor = generated_function(
        lowered,
        "Promise.constructor",
        "PROMISE_CONSTRUCT_ID_GLOBAL_TYPE_ID",
    )?;
    if constructor.is_async()
        || !constructor.type_parameters().is_empty()
        || constructor.name() != Some("Promise.constructor")
        || constructor.return_type() != &LoweredTypeReference::Predefined("GLOBAL_VOID_ID")
    {
        bail!("generated Promise constructor helper has unexpected shape");
    }
    let [parameter] = constructor.parameters() else {
        bail!("generated Promise constructor helper must have one parameter");
    };
    if parameter.binding() != &LoweredFunctionParameterBinding::Pattern
        || parameter.type_reference()
            != &LoweredTypeReference::Predefined("GLOBAL_VOID_CALLBACK_ID")
        || parameter.is_optional()
        || parameter.is_rest()
    {
        bail!("generated Promise constructor helper has an unexpected parameter");
    }

    for shape in PROMISE_METHOD_SHAPES {
        assert_promise_method(class, lowered, *shape)?;
    }

    Ok(())
}

/// Checks one Promise member and the synthetic callable referenced by that member.
fn assert_promise_method(
    class: &LoweredClass,
    lowered: &LoweredGlobalTypes,
    shape: PromiseMethodShape,
) -> Result<()> {
    let Some(member) = class.member(shape.member_name) else {
        bail!("generated Promise global is missing {}", shape.member_name);
    };
    let expected_kind = if shape.is_static {
        LoweredMemberKind::NamedStatic
    } else {
        LoweredMemberKind::Named { optional: false }
    };
    if member.kind() != &expected_kind
        || member.type_reference() != &LoweredTypeReference::Predefined(shape.member_type_id)
    {
        bail!(
            "generated Promise.{} member has unexpected shape",
            shape.member_name
        );
    }

    let helper = generated_function(lowered, shape.helper_name, shape.helper_id_constant)?;
    if helper.is_async()
        || !helper.type_parameters().is_empty()
        || helper.name() != Some(shape.helper_name)
        || !helper.parameters().is_empty()
        || helper.return_type() != &LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_PROMISE_ID")
    {
        bail!(
            "generated {} helper has unexpected shape",
            shape.helper_name
        );
    }
    Ok(())
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
