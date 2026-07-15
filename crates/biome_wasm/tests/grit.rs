#![cfg(target_arch = "wasm32")]

use biome_grit_patterns::testing::{compile_js_query, make_js_file};
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn empty_call_with_multiple_metavariables() {
    let query = compile_js_query(
        r#"`expect($arg).$method()` where {
            $arg <: 1,
            $method <: `toBeTruthy`,
        }"#,
    );
    let result = query
        .execute(make_js_file("expect(1).toBeTruthy()"))
        .expect("could not execute query");

    assert_eq!(result.effects.len(), 1);
}
