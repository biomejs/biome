use biome_js_syntax::{AnyJsModuleItem, AnyJsStatement};
use biome_js_type_info::Type;

use crate::utils::parse_ts;

#[allow(dead_code)]
mod utils;

#[test]
fn quick_test() {
    const CODE: &str = r#"(void 0) && 1"#;

    let root = parse_ts(CODE);
    let expr = root
        .as_js_module()
        .unwrap()
        .items()
        .into_iter()
        .filter_map(|item| match item {
            AnyJsModuleItem::AnyJsStatement(stmt) => Some(stmt),
            _ => None,
        })
        .find_map(|stmt| match stmt {
            AnyJsStatement::JsExpressionStatement(expr) => Some(expr),
            _ => None,
        })
        .unwrap()
        .expression()
        .unwrap();

    let ty = Type::from_any_js_expression(&expr);
    eprintln!("{:?}", ty);
}
