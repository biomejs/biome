use biome_js_parser::{JsParserOptions, parse};
use biome_languages::JsFileSource;
use biome_react_compiler::{ConvertInput, convert_file};
use react_compiler_ast::expressions::Expression;
use react_compiler_ast::jsx::JSXAttributeValue;
use react_compiler_ast::patterns::{ObjectPatternProperty, PatternLike};
use react_compiler_ast::statements::Statement;

#[test]
fn converts_directive_and_function() {
    let source = r#""use memo";
function Component(props) {
    return "hello";
}
"#;
    let source_type = JsFileSource::jsx();
    let parsed = parse(source, source_type, JsParserOptions::default());

    let file = convert_file(ConvertInput {
        root: &parsed.tree(),
        source,
        source_type,
    })
    .expect("expected conversion to succeed");

    assert_eq!(file.program.directives.len(), 1);
    assert_eq!(file.program.directives[0].value.value, "use memo");

    let Statement::FunctionDeclaration(function) = &file.program.body[0] else {
        panic!("expected function declaration");
    };
    assert_eq!(function.id.as_ref().unwrap().name, "Component");
    assert_eq!(function.id.as_ref().unwrap().base.node_id, Some(22));
    assert_eq!(function.params.len(), 1);
    assert_eq!(function.base.start, Some(12));
    assert_eq!(function.base.node_id, Some(13));
}

#[test]
fn converts_variable_member_and_jsx() {
    let source = r#"function Component(props) {
    const value = props.value;
    return <div id="x">{value}</div>;
}
"#;
    let source_type = JsFileSource::jsx();
    let parsed = parse(source, source_type, JsParserOptions::default());

    let file = convert_file(ConvertInput {
        root: &parsed.tree(),
        source,
        source_type,
    })
    .expect("expected conversion to succeed");

    let Statement::FunctionDeclaration(function) = &file.program.body[0] else {
        panic!("expected function declaration");
    };

    let Statement::VariableDeclaration(declaration) = &function.body.body[0] else {
        panic!("expected variable declaration");
    };
    assert_eq!(declaration.declarations.len(), 1);
    assert!(declaration.declarations[0].init.is_some());

    let Statement::ReturnStatement(return_statement) = &function.body.body[1] else {
        panic!("expected return statement");
    };
    assert!(return_statement.argument.is_some());
}

#[test]
fn converts_imports_arrays_objects_arrows_and_operators() {
    let source = r#"import React, { useMemo } from "react";
function Component(props) {
    const values = [1, 2, 3];
    const view = values.map((value) => ({ value: value + props.offset }));
    return props.enabled && <div>{view.length > 0 ? view.length : 0}</div>;
}
"#;
    let source_type = JsFileSource::jsx();
    let parsed = parse(source, source_type, JsParserOptions::default());

    let file = convert_file(ConvertInput {
        root: &parsed.tree(),
        source,
        source_type,
    })
    .expect("expected conversion to succeed");

    assert!(matches!(
        file.program.body[0],
        Statement::ImportDeclaration(_)
    ));
    let Statement::FunctionDeclaration(function) = &file.program.body[1] else {
        panic!("expected function declaration");
    };
    assert_eq!(function.body.body.len(), 3);
}

#[test]
fn converts_named_function_export() {
    let source = r#"export function Component() {
    return <div />;
}
"#;
    let source_type = JsFileSource::jsx();
    let parsed = parse(source, source_type, JsParserOptions::default());

    let file = convert_file(ConvertInput {
        root: &parsed.tree(),
        source,
        source_type,
    })
    .expect("expected conversion to succeed");

    assert!(matches!(
        file.program.body[0],
        Statement::ExportNamedDeclaration(_)
    ));
}

#[test]
fn converts_spreads_function_expressions_new_await_and_updates() {
    let source = r#"async function Component(props) {
    const handler = function handleClick(event) {
        props.count++;
        props.value = event.value;
    };
    const next = new Widget(...props.items);
    const value = await props.load({ ...props.options });
    return <Widget {...props} value={value}>{...props.children}</Widget>;
}
"#;
    let source_type = JsFileSource::jsx();
    let parsed = parse(source, source_type, JsParserOptions::default());

    let file = convert_file(ConvertInput {
        root: &parsed.tree(),
        source,
        source_type,
    })
    .expect("expected conversion to succeed");

    let Statement::FunctionDeclaration(function) = &file.program.body[0] else {
        panic!("expected function declaration");
    };
    assert_eq!(function.body.body.len(), 4);
}

#[test]
fn converts_namespace_import_and_default_export_expression() {
    let source = r#"import * as React from "react";
export default React.memo(function Component() {
    return null;
});
"#;
    let source_type = JsFileSource::jsx();
    let parsed = parse(source, source_type, JsParserOptions::default());

    let file = convert_file(ConvertInput {
        root: &parsed.tree(),
        source,
        source_type,
    })
    .expect("expected conversion to succeed");

    assert!(matches!(
        file.program.body[0],
        Statement::ImportDeclaration(_)
    ));
    assert!(matches!(
        file.program.body[1],
        Statement::ExportDefaultDeclaration(_)
    ));
}

#[test]
fn converts_fragments_destructuring_templates_sequences_and_exports() {
    let source = r#"export const answer = 42;
export { answer as value };
export * from "./shared";

function Component(props) {
    const { title: heading = "fallback", ...rest } = props;
    const [first, , ...remaining] = props.items;
    const label = `hello ${heading}`;
    return (<><span>{(first, label)}</span><Widget {...rest} /></>);
}
"#;
    let source_type = JsFileSource::jsx();
    let parsed = parse(source, source_type, JsParserOptions::default());

    let file = convert_file(ConvertInput {
        root: &parsed.tree(),
        source,
        source_type,
    })
    .expect("expected conversion to succeed");

    assert!(matches!(
        file.program.body[0],
        Statement::ExportNamedDeclaration(_)
    ));
    assert!(matches!(
        file.program.body[1],
        Statement::ExportNamedDeclaration(_)
    ));
    assert!(matches!(
        file.program.body[2],
        Statement::ExportAllDeclaration(_)
    ));
    let Statement::FunctionDeclaration(function) = &file.program.body[3] else {
        panic!("expected function declaration");
    };
    assert_eq!(function.body.body.len(), 4);
}

#[test]
fn converts_meta_import_yield_object_methods_and_classes() {
    let source = r#"class View extends Base {}
const Generated = class extends Base {};
const object = {
    method(value) { return import.meta.url; },
    get current() { return import("./current"); },
    set current(value) { this.value = value; },
};
function* values(items) {
    yield* items;
}
"#;
    let source_type = JsFileSource::jsx();
    let parsed = parse(source, source_type, JsParserOptions::default());

    let file = convert_file(ConvertInput {
        root: &parsed.tree(),
        source,
        source_type,
    })
    .expect("expected conversion to succeed");

    assert!(matches!(
        file.program.body[0],
        Statement::ClassDeclaration(_)
    ));
    assert!(matches!(
        file.program.body[1],
        Statement::VariableDeclaration(_)
    ));
    assert!(matches!(
        file.program.body[2],
        Statement::VariableDeclaration(_)
    ));
    assert!(matches!(
        file.program.body[3],
        Statement::FunctionDeclaration(_)
    ));
}

#[test]
fn converts_class_exports() {
    let source = r#"export class NamedView extends Base {}
export default class DefaultView extends Base {}
"#;
    let source_type = JsFileSource::jsx();
    let parsed = parse(source, source_type, JsParserOptions::default());

    let file = convert_file(ConvertInput {
        root: &parsed.tree(),
        source,
        source_type,
    })
    .expect("expected conversion to succeed");

    assert!(matches!(
        file.program.body[0],
        Statement::ExportNamedDeclaration(_)
    ));
    assert!(matches!(
        file.program.body[1],
        Statement::ExportDefaultDeclaration(_)
    ));
}

#[test]
fn converts_control_flow_literals_and_ts_wrappers() {
    let source = r#"function Component(props, ...rest) {
    label: for (let index = 0; index < props.items.length; index++) {
        if (index in props.items) continue label;
        break;
    }
    for (const item of props.items) { props.visit(item); }
    while (props.ready) { debugger; }
    do { props.tick(); } while (props.pending);
    switch (props.kind) { case "a": throw /x+/g; default: props.done(1n); }
    try { props.run(); } catch (error) { props.fail(error); } finally { props.cleanup(); }
    const value = (props.value as number) satisfies number;
    const [first, ...remaining] = rest;
    ({ value: props.value = first } = props);
    return props?.view?.(value) instanceof View ? <View /> : null;
}
"#;
    let source_type = JsFileSource::tsx();
    let parsed = parse(source, source_type, JsParserOptions::default());

    let file = convert_file(ConvertInput {
        root: &parsed.tree(),
        source,
        source_type,
    })
    .expect("expected conversion to succeed");

    let Statement::FunctionDeclaration(function) = &file.program.body[0] else {
        panic!("expected function declaration");
    };
    assert_eq!(function.params.len(), 2);
    assert!(matches!(
        function.body.body[0],
        Statement::LabeledStatement(_)
    ));
    assert!(matches!(
        function.body.body[2],
        Statement::WhileStatement(_)
    ));
    assert!(matches!(
        function.body.body[4],
        Statement::SwitchStatement(_)
    ));
    assert!(matches!(function.body.body[5], Statement::TryStatement(_)));
}

#[test]
fn converts_react_compiler_edge_cases() {
    let source = r#"// 😀
function Component({ title = "fallback" }) {
    const hex = 0x10;
    const label = `a\n${title}\u{21}`;
    return <Wrapper icon=<Icon /> fragment=<></> />;
}
"#;
    let source_type = JsFileSource::jsx();
    let parsed = parse(source, source_type, JsParserOptions::default());

    let file = convert_file(ConvertInput {
        root: &parsed.tree(),
        source,
        source_type,
    })
    .expect("expected conversion to succeed");

    let Statement::FunctionDeclaration(function) = &file.program.body[0] else {
        panic!("expected function declaration");
    };
    assert_eq!(function.base.start, Some(8));
    assert_eq!(function.base.loc.as_ref().unwrap().start.index, Some(6));

    let PatternLike::ObjectPattern(params) = &function.params[0] else {
        panic!("expected object pattern parameter");
    };
    let ObjectPatternProperty::ObjectProperty(property) = &params.properties[0] else {
        panic!("expected object pattern property");
    };
    let PatternLike::AssignmentPattern(default) = property.value.as_ref() else {
        panic!("expected assignment pattern");
    };
    assert_eq!(default.base.start, property.key.base().start);

    let Statement::VariableDeclaration(hex) = &function.body.body[0] else {
        panic!("expected hex variable declaration");
    };
    let Expression::NumericLiteral(number) = hex.declarations[0].init.as_deref().unwrap() else {
        panic!("expected numeric literal");
    };
    assert_eq!(number.value, 16.0);

    let Statement::VariableDeclaration(label) = &function.body.body[1] else {
        panic!("expected label variable declaration");
    };
    let Expression::TemplateLiteral(template) = label.declarations[0].init.as_deref().unwrap()
    else {
        panic!("expected template literal");
    };
    assert_eq!(template.quasis[0].value.raw, r#"a\n"#);
    assert_eq!(template.quasis[0].value.cooked.as_deref(), Some("a\n"));
    assert_eq!(template.quasis[1].value.raw, r#"\u{21}"#);
    assert_eq!(template.quasis[1].value.cooked.as_deref(), Some("!"));

    let Statement::ReturnStatement(return_statement) = &function.body.body[2] else {
        panic!("expected return statement");
    };
    let Expression::JSXElement(element) = return_statement.argument.as_deref().unwrap() else {
        panic!("expected jsx element");
    };
    let JSXAttributeValue::JSXElement(_) = element.opening_element.attributes[0]
        .as_attribute()
        .value
        .as_ref()
        .unwrap()
    else {
        panic!("expected jsx element attribute value");
    };
    let JSXAttributeValue::JSXFragment(_) = element.opening_element.attributes[1]
        .as_attribute()
        .value
        .as_ref()
        .unwrap()
    else {
        panic!("expected jsx fragment attribute value");
    };
}

trait TestObjectPatternPropExt {
    fn base(&self) -> &react_compiler_ast::common::BaseNode;
}

impl TestObjectPatternPropExt for Expression {
    fn base(&self) -> &react_compiler_ast::common::BaseNode {
        match self {
            Expression::Identifier(identifier) => &identifier.base,
            _ => panic!("expected identifier key"),
        }
    }
}

trait TestJsxAttributeExt {
    fn as_attribute(&self) -> &react_compiler_ast::jsx::JSXAttribute;
}

impl TestJsxAttributeExt for react_compiler_ast::jsx::JSXAttributeItem {
    fn as_attribute(&self) -> &react_compiler_ast::jsx::JSXAttribute {
        match self {
            react_compiler_ast::jsx::JSXAttributeItem::JSXAttribute(attribute) => attribute,
            _ => panic!("expected jsx attribute"),
        }
    }
}
