use crate::{JsExtent, Jsx};

/// Two HTML void elements, matched case-sensitively as Astro matches them.
fn is_void_element(name: &[u8]) -> bool {
    matches!(name, b"br" | b"input")
}

/// The offset of the closing `---` fence, or `None` when the scan reaches
/// the end of the source without finding one.
fn fence(source: &str) -> Option<usize> {
    let length = JsExtent::frontmatter_length(source.as_bytes());
    (length < source.len()).then_some(length)
}

/// The offset of the `}` closing a text expression, or `None` when the
/// expression is unterminated. `<br>` and `<input>` have no children.
fn expression(source: &str) -> Option<usize> {
    let length = JsExtent::expression_length(source.as_bytes(), Jsx::Enabled { is_void_element });
    (length < source.len()).then_some(length)
}

/// Like [`expression`], but every element needs `/>` or a closing tag.
fn strict_expression(source: &str) -> Option<usize> {
    let jsx = Jsx::Enabled {
        is_void_element: |_| false,
    };
    let length = JsExtent::expression_length(source.as_bytes(), jsx);
    (length < source.len()).then_some(length)
}

/// The offset of the `]` closing a Vue dynamic directive argument, or `None`
/// when the bracket is unbalanced.
fn argument(source: &str) -> Option<usize> {
    let length = JsExtent::argument_length(source.as_bytes());
    (length < source.len()).then_some(length)
}

#[test]
fn unterminated_quotes_hide_the_fence() {
    assert_eq!(fence("'\"`\n---\n"), None);
    assert_eq!(fence("`'\"\"'\n---\n"), None);
}

#[test]
fn balanced_quotes_leave_the_fence_visible() {
    assert_eq!(fence(" '\"``\"' \n---\n"), Some(9));
}

#[test]
fn issue_9108_double_quoted_with_apostrophe() {
    assert!(fence("\"test'\"\n---\n").is_some());
}

#[test]
fn issue_9108_single_quoted_with_double_quote() {
    assert!(fence("'it\"s'\n---\n").is_some());
}

#[test]
fn issue_9108_template_with_mixed_quotes() {
    assert!(fence("`it's a \"test\"`\n---\n").is_some());
}

#[test]
fn issue_9108_multiple_strings_with_mixed_quotes() {
    assert!(fence("const a = \"it's\"; const b = 'say \"hi\"';\n---\n").is_some());
}

#[test]
fn issue_8882_multiline_block_comment_with_apostrophe() {
    assert!(fence("/*\n * Doesn't that stink?\n */\n---\n").is_some());
}

#[test]
fn issue_8882_jsdoc_comment_then_code() {
    let source = "/**\n * In this comment, if you add any string opening or closing, such as an apostrophe, the file will show \n * a bunch of errors. Doesn't (remove the apostrophe in the previous word to fix) that stink? \n */\nimport type { HTMLAttributes } from \"astro/types\";\nconst { class: className } = Astro.props;\n---\n";
    assert!(fence(source).is_some());
}

#[test]
fn issue_8882_multiline_block_comment_quotes_every_line() {
    let source = "/* line one 'unclosed\n   line two `unclosed\n   line three \"unclosed */\nconst x = 1;\n---\n";
    assert!(fence(source).is_some());
}

#[test]
fn fence_inside_single_line_comment_is_not_a_fence() {
    assert_eq!(fence("// ---"), None);
}

#[test]
fn fence_inside_block_comment_is_not_a_fence() {
    assert_eq!(fence("/*\n---\n"), None);
}

#[test]
fn a_line_comment_ends_at_its_newline() {
    assert!(fence("// ---\nconst f = \"something\"\n---\n").is_some());
}

#[test]
fn escaped_double_quote_inside_double_string() {
    assert!(fence("\"\\\"\"\n---\n").is_some());
}

#[test]
fn escaped_single_quote_inside_single_string() {
    assert!(fence("'\\''\n---\n").is_some());
}

#[test]
fn escaped_backtick_inside_template() {
    assert!(fence("`\\``\n---\n").is_some());
}

#[test]
fn double_backslash_then_quote() {
    assert!(fence("\"\\\\\"\n---\n").is_some());
}

#[test]
fn issue_9187_regex_with_single_quote() {
    assert!(fence("const test = /'/\n---\n").is_some());
}

#[test]
fn issue_9187_regex_with_double_quote() {
    assert!(fence("const test = /\"/\n---\n").is_some());
}

#[test]
fn issue_9187_regex_with_dashes() {
    assert!(fence("const test = /---/\n---\n").is_some());
}

#[test]
fn issue_9187_regex_with_escape_and_quantifier() {
    assert!(fence("const test = /\\d{4}/\n---\n").is_some());
}

#[test]
fn regex_character_class_may_contain_a_slash() {
    assert!(fence("const a = /[/]'/;\n---\n").is_some());
}

#[test]
fn nested_template_interpolation_closes_the_fence() {
    assert!(fence("const a = `x${`/y`}`;\n---\n").is_some());
}

#[test]
fn nested_template_interpolation_ends_the_expression() {
    let source = "`/blog${n === 0 ? '' : `/${n + 1}`}`}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

#[test]
fn apostrophe_in_jsx_text_ends_the_expression() {
    let source = "items.map((i) => <li>it's {i}</li>)}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

#[test]
fn a_self_closing_element_ends_the_expression() {
    let source = "<Icon />}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

#[test]
fn a_void_element_has_no_children() {
    let source = "cond && <span>It's<br>ok</span>}";
    assert_eq!(expression(source), Some(source.len() - 1));
    let source = "open && <label><input type=\"text\">don't</label>}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

#[test]
fn the_void_element_check_receives_the_name_as_written() {
    let source = "x && <BR>it's</BR>}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

/// Without void elements, `<br>` keeps its children open until `</span>`,
/// so the apostrophe in `It's` is read as a string that never closes.
#[test]
fn without_void_elements_an_unclosed_element_leaves_the_expression_unterminated() {
    assert_eq!(strict_expression("cond && <span>It's<br>ok</span>}"), None);
}

#[test]
fn without_void_elements_a_self_closing_element_has_no_children() {
    let source = "cond && <span>It's<br />ok</span>}";
    assert_eq!(strict_expression(source), Some(source.len() - 1));
}

#[test]
fn a_slash_in_an_unquoted_value_does_not_close_the_tag() {
    let source = "x && <div a=4/>it's</div>}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

#[test]
fn a_slash_after_an_unquoted_value_closes_the_tag() {
    let source = "x && <div a=4 />}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

#[test]
fn a_template_attribute_value_may_contain_an_angle_bracket() {
    let source = "x && <C a=`b > c` />}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

#[test]
fn type_arguments_do_not_end_a_tag() {
    let source = "x && <List<string>>it's</List>}";
    assert_eq!(expression(source), Some(source.len() - 1));
    let source = "x && <Table<Map<string, number>>>it's</Table>}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

#[test]
fn a_function_type_argument_does_not_end_the_list() {
    let source = "x && <C<(x: A) => B>>don't</C>}";
    assert_eq!(expression(source), Some(source.len() - 1));
    let source = "x && <C<() => B> />}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

#[test]
fn a_fragment_ends_the_expression() {
    let source = "<><b>a</b></>}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

#[test]
fn returned_jsx_ends_the_expression() {
    let source = "() => { return <div>a</div>; }}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

#[test]
fn a_comparison_does_not_start_jsx() {
    let source = "a < c}";
    assert_eq!(expression(source), Some(source.len() - 1));
    let source = "a<c}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

#[test]
fn a_generic_call_does_not_start_jsx() {
    let source = "new Map<string, number>()}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

#[test]
fn a_type_parameter_list_does_not_start_jsx() {
    let source = "<T,>(x: T) => x}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

/// A `<` that opens neither JSX nor a recognisably non-JSX construct
/// swallows the rest of the source; the scan retries without JSX rather
/// than letting the expression run away.
#[test]
fn a_runaway_element_falls_back_to_scanning_without_jsx() {
    let source = "await <T extends U>(v)}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

#[test]
fn a_trailing_escape_does_not_scan_past_the_input() {
    let source = "`a\\";
    assert_eq!(expression(source), None);
    assert_eq!(
        JsExtent::expression_length(source.as_bytes(), Jsx::Enabled { is_void_element }),
        source.len()
    );
}

/// Astro's frontmatter is TypeScript, where `<` opens a type assertion.
#[test]
fn a_type_assertion_in_frontmatter_closes_the_fence() {
    assert_eq!(fence("const a = <string>x;\n---\n"), Some(21));
}

#[test]
fn a_regex_starting_with_a_closing_angle_bracket_is_a_regex() {
    assert!(fence("const a = s.replace(/>/g, '&gt;');\n---\n<p>{a}</p>\n").is_some());
    assert!(fence("const isTag = (s) => />$/.test(s);\n---\n").is_some());
    assert!(fence("const r = [/>/, /</];\n---\n").is_some());
    let source = "s.replace(/>/g, '')}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

#[test]
fn a_regex_after_an_expression_keyword_is_a_regex() {
    assert!(fence("function hasTag(s) {\n  return />'/.test(s);\n}\n---\n").is_some());
    assert!(fence("function hasQuote(s) {\n  return /'/.test(s);\n}\n---\n").is_some());
    let source = "typeof />'/}";
    assert_eq!(expression(source), Some(source.len() - 1));
}

#[test]
fn a_slash_after_a_property_or_other_identifier_is_a_division() {
    assert!(fence("const apy = pool.yield / 100;\n---\n<p>{apy}</p>\n").is_some());
    assert!(fence("const a = this.#return / 2, b = \"/'\";\n---\n").is_some());
    assert!(fence("const a = begin / 2, b = \"/'\";\n---\n").is_some());
    assert!(fence("const a = pool. yield / 2, b = \"/'\";\n---\n").is_some());
    assert!(fence("const a = pool.\n  yield / 2, b = \"/'\";\n---\n").is_some());
}

#[test]
fn a_slash_after_a_contextual_keyword_used_as_a_name_is_a_division() {
    assert!(fence("const of = 8, n = of / 2, s = \"/'\";\n---\n").is_some());
}

#[test]
fn a_self_closing_tag_in_frontmatter_still_closes() {
    assert!(fence("const el = <Foo bar={1} />;\n---\n").is_some());
    assert!(fence("const el = <Foo bar=\"x\" />;\n---\n").is_some());
}

#[test]
fn a_closing_tag_in_frontmatter_does_not_open_a_regex() {
    assert!(fence("const items = xs.map((x) => <li>{x}</li>);\n---\n").is_some());
}

#[test]
fn a_nested_bracket_does_not_end_an_argument() {
    let source = "items[0]]";
    assert_eq!(argument(source), Some(source.len() - 1));
}

#[test]
fn a_bracket_in_a_string_does_not_end_an_argument() {
    let source = "'a]b']";
    assert_eq!(argument(source), Some(source.len() - 1));
}

/// The offset where the content of an MDX ESM block ends.
fn esm(source: &str) -> usize {
    JsExtent::esm_length(source.as_bytes())
}

#[test]
fn a_blank_line_ends_an_esm_block() {
    assert_eq!(esm("import a from \"a\"\n\nb"), 17);
}

#[test]
fn a_line_of_spaces_ends_an_esm_block() {
    assert_eq!(esm("import a from \"a\"\n \t\nb"), 17);
}

#[test]
fn an_esm_block_without_a_blank_line_runs_to_the_end() {
    let source = "import a from \"a\"\nexport const b = 1";
    assert_eq!(esm(source), source.len());
}

#[test]
fn a_blank_line_inside_braces_does_not_end_an_esm_block() {
    let source = "export {\n\n  a\n\n} from \"b\"\n\nc";
    assert_eq!(esm(source), source.len() - 3);
}

#[test]
fn a_blank_line_inside_a_template_does_not_end_an_esm_block() {
    let source = "export const a = `x\n\ny`\n\nb";
    assert_eq!(esm(source), source.len() - 3);
}

#[test]
fn a_blank_line_inside_a_block_comment_does_not_end_an_esm_block() {
    let source = "import a from \"a\" /*\n\n*/\n\nb";
    assert_eq!(esm(source), source.len() - 3);
}

#[test]
fn a_blank_line_after_a_line_comment_ends_an_esm_block() {
    assert_eq!(esm("import a from \"a\" // a\n\nb"), 22);
}

#[test]
fn a_blank_line_inside_jsx_does_not_end_an_esm_block() {
    let source = "export const a = <div>\n\n</div>\n\nb";
    assert_eq!(esm(source), source.len() - 3);
}

#[test]
fn a_blank_line_after_crlf_ends_an_esm_block() {
    assert_eq!(esm("import a\r\n\r\nb"), 8);
}

/// The length of the JSX tag at the start of `source`.
fn tag(source: &str) -> Option<usize> {
    JsExtent::tag_length(source.as_bytes())
}

#[test]
fn a_tag_ends_at_its_closing_angle_bracket() {
    assert_eq!(tag("<Note>text</Note>"), Some(6));
    assert_eq!(tag("</Note> text"), Some(7));
    assert_eq!(tag("<Note /> text"), Some(8));
}

#[test]
fn a_fragment_tag_ends_at_its_angle_bracket() {
    assert_eq!(tag("<>a"), Some(2));
    assert_eq!(tag("</>a"), Some(3));
}

#[test]
fn an_angle_bracket_in_an_attribute_does_not_end_a_tag() {
    let source = "<Chart title=\"a > b\" data={x > 1 ? \">\" : '>'} /> after";
    assert_eq!(tag(source), Some(source.len() - 6));
}

#[test]
fn an_element_in_an_attribute_expression_does_not_end_a_tag() {
    let source = "<Card icon={<Icon name=\"x\" />}>children";
    assert_eq!(tag(source), Some(source.len() - 8));
}

#[test]
fn a_tag_may_span_lines() {
    let source = "<Card\n  title=\"a\"\n>";
    assert_eq!(tag(source), Some(source.len()));
}

#[test]
fn an_unclosed_tag_has_no_length() {
    assert_eq!(tag("<Note title=\"a\""), None);
    assert_eq!(tag("<Note data={x"), None);
}

#[test]
fn a_comma_does_not_end_the_measured_tag() {
    assert_eq!(tag("<a, b> c"), Some(6));
    assert_eq!(tag("<Hello, world"), None);
}
