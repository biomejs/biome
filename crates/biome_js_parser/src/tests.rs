use crate::test_utils::has_bogus_nodes_or_empty_slots;
use crate::{parse, parse_module, test_utils::assert_errors_are_absent, JsParserOptions, Parse};
use biome_console::fmt::{Formatter, Termcolor};
use biome_console::markup;
use biome_diagnostics::DiagnosticExt;
use biome_diagnostics::PrintDiagnostic;
use biome_js_syntax::{AnyJsRoot, JsFileSource, JsSyntaxKind};
use biome_js_syntax::{JsCallArguments, JsLogicalExpression, JsSyntaxToken};
use biome_rowan::{AstNode, Direction, TextSize};
use expect_test::expect_file;
use std::fmt::Write;
use std::panic::catch_unwind;
use std::path::{Path, PathBuf};

#[test]
fn parser_smoke_test() {
    let src = r#"
import "x" with { type: "json" }
"#;

    let module = parse(src, JsFileSource::tsx(), JsParserOptions::default());
    assert_errors_are_absent(&module, Path::new("parser_smoke_test"));
}

#[test]
fn parser_missing_smoke_test() {
    let src = r#"
        console.log("Hello world";
    "#;

    let module = parse_module(src, JsParserOptions::default());

    let arg_list = module
        .syntax()
        .descendants()
        .find_map(JsCallArguments::cast)
        .unwrap();

    let opening = arg_list.syntax().element_in_slot(0);
    let list = arg_list.syntax().element_in_slot(1);
    let closing = arg_list.syntax().element_in_slot(2);

    assert_eq!(opening.map(|o| o.to_string()), Some(String::from("(")));
    assert_eq!(
        list.map(|l| l.kind()),
        Some(JsSyntaxKind::JS_CALL_ARGUMENT_LIST)
    );
    assert_eq!(closing, None);
}

fn try_parse(path: &str, text: &str, options: JsParserOptions) -> Parse<AnyJsRoot> {
    let res = catch_unwind(|| {
        let path = Path::new(path);
        // Files containing a // SCRIPT comment are parsed as script and not as module
        // This is needed to test features that are restricted in strict mode.
        let source_type = if text.contains("// SCRIPT") {
            JsFileSource::js_script()
        } else {
            path.try_into().unwrap()
        };

        let parse = parse(text, source_type, options);

        assert_eq!(
            parse.syntax().to_string(),
            text,
            "Original source and re-printed tree differ\nParsed Tree: {:#?}",
            parse.syntax(),
        );

        parse
    });
    assert!(res.is_ok(), "Trying to parse `{}` panicked", path);
    res.unwrap()
}

fn try_parse_with_printed_ast(
    path: &str,
    text: &str,
    options: JsParserOptions,
) -> (Parse<AnyJsRoot>, String) {
    catch_unwind(|| {
        let parse = try_parse(path, text, options.clone());
        let formatted = format!("{:#?}", &parse.tree());
        (parse, formatted)
    })
    .unwrap_or_else(|err| {
        // Re-parsing the source here seems silly. But the problem is, that `SyntaxNode`s aren't
        // unwind safe. That's why the same `ParseResult` can't be reused here.
        // This should be fine because this code is only executed for local tests. No checked-in
        // test should ever hit this line.
        let re_parsed = try_parse(path, text, options);
        panic!(
            "Printing the AST for `{}` panicked. That means it is malformed. Err: {:?}\n{:#?}",
            path,
            err,
            re_parsed.syntax()
        );
    })
}

#[cfg(test)]
fn run_and_expect_no_errors(path: &str, _: &str, _: &str, _: &str) {
    let path = PathBuf::from(path);
    let text = std::fs::read_to_string(&path).unwrap();

    let options_path = path.with_extension("options.json");
    let options: JsParserOptions = std::fs::read_to_string(options_path)
        .ok()
        .and_then(|options| serde_json::from_str(&options).ok())
        .unwrap_or_default();

    let (parse, ast) = try_parse_with_printed_ast(path.to_str().unwrap(), &text, options);
    assert_errors_are_absent(&parse, &path);
    let actual = format!("{}\n\n{:#?}", ast, parse.syntax());

    let path = path.with_extension("rast");
    expect_file![path].assert_eq(&actual)
}

#[cfg(test)]
fn run_and_expect_errors(path: &str, _: &str, _: &str, _: &str) {
    let path = PathBuf::from(path);
    let text = std::fs::read_to_string(&path).unwrap();

    let options_path = path.with_extension("options.json");
    let options: JsParserOptions = std::fs::read_to_string(options_path)
        .ok()
        .and_then(|options| serde_json::from_str(&options).ok())
        .unwrap_or_default();

    let (parse, ast) = try_parse_with_printed_ast(path.to_str().unwrap(), &text, options);
    assert_errors_are_present(&parse, &path);
    let mut actual = format!("{}\n\n{:#?}", ast, parse.syntax());
    for diag in parse.diagnostics() {
        let mut write = biome_diagnostics::termcolor::Buffer::no_color();
        let error = diag
            .clone()
            .with_file_path(path.file_name().unwrap().to_string_lossy().to_string())
            .with_file_source_code(text.to_string());
        Formatter::new(&mut Termcolor(&mut write))
            .write_markup(markup! {
                {PrintDiagnostic::verbose(&error)}
            })
            .expect("failed to emit diagnostic");
        write!(
            actual,
            "--\n{}",
            std::str::from_utf8(write.as_slice()).expect("non utf8 in error buffer")
        )
        .unwrap();
    }
    write!(actual, "--\n{}", text).unwrap();

    let path = path.with_extension("rast");
    expect_file![path].assert_eq(&actual)
}

mod parser {
    mod ok {
        tests_macros::gen_tests! {"test_data/inline/ok/**/*.{js,ts,jsx,tsx}", crate::tests::run_and_expect_no_errors, ""}
    }
    mod err {
        tests_macros::gen_tests! {"test_data/inline/err/**/*.{js,ts,jsx,tsx}", crate::tests::run_and_expect_errors, ""}
    }
}

fn assert_errors_are_present(program: &Parse<AnyJsRoot>, path: &Path) {
    assert!(
        !program.diagnostics().is_empty(),
        "There should be errors in the file {:?}\nSyntax Tree: {:#?}",
        path.display(),
        program.syntax()
    );
}

#[test]
pub fn test_trivia_attached_to_tokens() {
    let text = "/**/let a = 1; // nice variable \n /*hey*/ let \t b = 2; // another nice variable";
    let m = parse_module(text, JsParserOptions::default());
    let mut tokens = m.syntax().descendants_tokens(Direction::Next);

    let is_let = |x: &JsSyntaxToken| x.text_trimmed() == "let";
    let first_let = tokens.find(is_let).unwrap();

    // first let leading trivia asserts
    let pieces: Vec<_> = first_let.leading_trivia().pieces().collect();
    assert!(matches!(pieces.first().map(|x| x.text()), Some("/**/")));
    assert!(pieces.get(1).is_none());

    // first let trailing trivia asserts
    let pieces: Vec<_> = first_let.trailing_trivia().pieces().collect();
    assert!(matches!(pieces.first().map(|x| x.text()), Some(" ")));
    assert!(pieces.get(1).is_none());

    // second let leading trivia asserts
    let second_let = tokens.find(is_let).unwrap();
    let pieces: Vec<_> = second_let.leading_trivia().pieces().collect();
    assert_eq!(4, pieces.len());
    assert!(matches!(pieces.first().map(|x| x.text()), Some("\n")));
    assert!(matches!(pieces.get(1).map(|x| x.text()), Some(" ")));
    assert!(matches!(pieces.get(2).map(|x| x.text()), Some("/*hey*/")));
    assert!(matches!(pieces.get(3).map(|x| x.text()), Some(" ")));

    // second let trailing trivia asserts
    let pieces: Vec<_> = second_let.trailing_trivia().pieces().collect();
    assert_eq!(1, pieces.len());
    assert!(matches!(pieces.first().map(|x| x.text()), Some(" \t ")));
}

#[test]
pub fn jsroot_display_text_and_trimmed() {
    let code = " let a = 1; \n ";
    let root = parse_module(code, JsParserOptions::default());
    let syntax = root.syntax();

    assert_eq!(format!("{}", syntax), code);

    let syntax_text = syntax.text();
    assert_eq!(format!("{}", syntax_text), code);

    let syntax_text = syntax.text_trimmed();
    assert_eq!(format!("{}", syntax_text), code.trim());
}

#[test]
pub fn jsroot_ranges() {
    //               0123456789A
    let code = " let a = 1;";
    let root = parse_module(code, JsParserOptions::default());
    let syntax = root.syntax();

    let first_let = syntax.first_token().unwrap();
    let range = first_let.text_range();
    assert_eq!(0usize, usize::from(range.start()));
    assert_eq!(5usize, usize::from(range.end()));

    let range = first_let.text_trimmed_range();
    assert_eq!(1usize, usize::from(range.start()));
    assert_eq!(4usize, usize::from(range.end()));

    let eq = syntax
        .descendants_tokens(Direction::Next)
        .find(|x| x.text_trimmed() == "=")
        .unwrap();
    let range = eq.text_range();
    assert_eq!(7usize, usize::from(range.start()));
    assert_eq!(9usize, usize::from(range.end()));

    let range = eq.text_trimmed_range();
    assert_eq!(7usize, usize::from(range.start()));
    assert_eq!(8usize, usize::from(range.end()));
}

#[test]
pub fn node_range_must_be_correct() {
    //               0123456789A123456789B123456789
    let text = " function foo() { let a = 1; }";
    let root = parse_module(text, JsParserOptions::default());

    let var_decl = root
        .syntax()
        .descendants()
        .find(|x| x.kind() == JsSyntaxKind::JS_VARIABLE_STATEMENT)
        .unwrap();

    let range = var_decl.text_range();
    assert_eq!(18usize, usize::from(range.start()));
    assert_eq!(29usize, usize::from(range.end()));

    let range = var_decl.text_trimmed_range();
    assert_eq!(18usize, usize::from(range.start()));
    assert_eq!(28usize, usize::from(range.end()));
}

#[test]
pub fn last_trivia_must_be_appended_to_eof() {
    //               0123456789A123456789B123456789CC
    let text = " function foo() { let a = 1; }\n";
    let root = parse_module(text, JsParserOptions::default());
    let syntax = root.syntax();

    let range = syntax.text_range();
    let start = range.start();
    let end = range.end();

    assert_eq!(TextSize::from(0), start);
    assert_eq!(TextSize::from(31), end);
}

#[test]
pub fn just_trivia_must_be_appended_to_eof() {
    //               0123456789A123456789B123456789C123
    let text = "// just trivia... nothing else....";
    let root = parse_module(text, JsParserOptions::default());
    let syntax = root.syntax();

    let range = syntax.text_range();
    let start = range.start();
    let end = range.end();

    assert_eq!(TextSize::from(0), start);
    assert_eq!(TextSize::from(34), end);
}

#[test]
pub fn node_contains_comments() {
    let text = "true && true // comment";
    let root = parse_module(text, JsParserOptions::default());
    let syntax = root.syntax();

    assert!(syntax.has_comments_descendants());
}

#[test]
fn parser_regexp_after_operator() {
    fn assert_no_errors(src: &str) {
        let module = parse(src, JsFileSource::js_script(), JsParserOptions::default());
        assert_errors_are_absent(&module, Path::new("parser_regexp_after_operator"));
    }
    assert_no_errors(r#"a=/a/"#);
    assert_no_errors(r#"a==/a/"#);
    assert_no_errors(r#"a===/a/"#);
    assert_no_errors(r#"a!=/a/"#);
    assert_no_errors(r#"a!==/a/"#);
}

#[test]
pub fn node_contains_trailing_comments() {
    let text = "true && (3 - 2 == 0) // comment";
    let root = parse_module(text, JsParserOptions::default());
    let syntax = root.syntax();
    let node = syntax
        .descendants()
        .find(|n| n.kind() == JsSyntaxKind::JS_LOGICAL_EXPRESSION)
        .unwrap();

    let logical_expression = JsLogicalExpression::cast(node).unwrap();
    let right = logical_expression.right().unwrap();

    assert!(right.syntax().has_trailing_comments());
    assert!(!right.syntax().has_leading_comments());
}

#[test]
pub fn node_contains_leading_comments() {
    let text = r"true &&
// comment
(3 - 2 == 0)";
    let root = parse_module(text, JsParserOptions::default());
    let syntax = root.syntax();
    let node = syntax
        .descendants()
        .find(|n| n.kind() == JsSyntaxKind::JS_LOGICAL_EXPRESSION)
        .unwrap();

    let logical_expression = JsLogicalExpression::cast(node).unwrap();
    let right = logical_expression.right().unwrap();

    assert!(right.syntax().has_leading_comments());
    assert!(!right.syntax().has_trailing_comments());
}

#[test]
pub fn node_has_comments() {
    let text = r"true &&
// comment
(3 - 2 == 0)";
    let root = parse_module(text, JsParserOptions::default());
    let syntax = root.syntax();
    let node = syntax
        .descendants()
        .find(|n| n.kind() == JsSyntaxKind::JS_LOGICAL_EXPRESSION)
        .unwrap();

    let logical_expression = JsLogicalExpression::cast(node).unwrap();
    let right = logical_expression.right().unwrap();

    assert!(right.syntax().has_comments_direct());
}

#[test]
fn diagnostics_print_correctly() {
    let text = r"const a";

    let root = parse_module(text, JsParserOptions::default());
    for diagnostic in root.diagnostics() {
        let mut write = biome_diagnostics::termcolor::Buffer::no_color();
        let error = diagnostic
            .clone()
            .with_file_path("example.js")
            .with_file_source_code(text.to_string());

        Formatter::new(&mut Termcolor(&mut write))
            .write_markup(markup! {
                {PrintDiagnostic::verbose(&error)}
            })
            .expect("failed to emit diagnostic");

        eprintln!(
            "{}",
            std::str::from_utf8(write.as_slice()).expect("non utf8 in error buffer")
        );
    }
}

#[ignore]
#[test]
pub fn quick_test() {
    let code = r#"
        type Equals = A extends (x: B extends C ? D : E) => 0 ? F : G;
    "#;
    let root = parse(
        code,
        JsFileSource::ts(),
        JsParserOptions::default().with_parse_class_parameter_decorators(),
    );
    let syntax = root.syntax();
    dbg!(&syntax, root.diagnostics(), root.has_errors());

    if has_bogus_nodes_or_empty_slots(&syntax) {
        panic!(
            "modified tree has bogus nodes or empty slots:\n{syntax:#?} \n\n {}",
            syntax
        )
    }
}
