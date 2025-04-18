use crate::utils::{assert_type_snapshot, get_function_declaration, parse_ts};
use biome_js_type_info::{Type, TypeInner};

mod utils;

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
