mod utils;

use biome_js_type_info::{Type, TypeInner};

use utils::assert_typed_bindings_snapshot;
use utils::get_variable_declaration;
use utils::{assert_type_snapshot, get_function_declaration, parse_ts};

#[test]
fn infer_type_of_promise_returning_function() {
    const CODE: &str = r#"function returnsPromise(): Promise<number> {
    return Promise.resolved(true);
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let ty = Type::from_js_function_declaration(&decl);
    assert_type_snapshot(CODE, ty, "infer_type_of_promise_returning_function");
}

#[test]
fn infer_type_of_async_function() {
    const CODE: &str = r#"async function returnsPromise(): Promise<string> {
	return "value";
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let ty = Type::from_js_function_declaration(&decl);
    assert_type_snapshot(CODE, ty, "infer_type_of_async_function");
}

#[test]
fn infer_type_of_array() {
    const CODE: &str = r#"const array: Array<string> = [];"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let bindings = Type::typed_bindings_from_js_variable_declaration(&decl);
    assert_typed_bindings_snapshot(CODE, &bindings, "infer_type_of_array");
}

#[test]
fn infer_type_of_destructured_array_element() {
    const CODE: &str = r#"const [a]: Array<string> = [];"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let bindings = Type::typed_bindings_from_js_variable_declaration(&decl);
    assert_typed_bindings_snapshot(CODE, &bindings, "infer_type_of_destructured_array_element");
}

#[test]
fn infer_type_of_function_with_destructured_arguments() {
    const CODE: &str = r#"function destruct({ a, b }: { a: number, b: string }, [first, ...rest]: Array<boolean>) {}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let ty = Type::from_js_function_declaration(&decl);
    assert_type_snapshot(
        CODE,
        ty,
        "infer_type_of_function_with_destructured_arguments",
    );
}

#[test]
fn infer_type_of_literal() {
    const CODE: &str = r#"const a = 123.45;"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let bindings = Type::typed_bindings_from_js_variable_declaration(&decl);
    assert_typed_bindings_snapshot(CODE, &bindings, "infer_type_of_literal");
}

#[test]
fn infer_type_of_binary_expression_eq() {
    const CODE: &str = r#"const a = 1 === 1"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let bindings = Type::typed_bindings_from_js_variable_declaration(&decl);
    assert_typed_bindings_snapshot(CODE, &bindings, "infer_type_of_binary_expression_eq");
}

#[test]
fn infer_type_of_binary_expression_ne() {
    const CODE: &str = r#"const a = 0 !== 1"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let bindings = Type::typed_bindings_from_js_variable_declaration(&decl);
    assert_typed_bindings_snapshot(CODE, &bindings, "infer_type_of_binary_expression_ne");
}

#[test]
#[cfg(target_pointer_width = "64")]
fn verify_type_sizes() {
    assert_eq!(
        std::mem::size_of::<Type>(),
        8,
        "`Type` should not be bigger than 8 bytes"
    );
    assert_eq!(
        std::mem::size_of::<TypeInner>(),
        16,
        "`TypeInner` should not be bigger than 16 bytes"
    );
}
