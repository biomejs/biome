use super::scan::KEYWORDS;
use super::*;
use crate::lexer::{JsLexContext, JsReLexContext};
use biome_js_syntax::T;
use biome_parser::lexer::ReLexer;
use biome_rowan::SyntaxKind;
use std::path::Path;

const JS: BulkLexOptions = BulkLexOptions {
    jsx: false,
    typescript: false,
};
const TS: BulkLexOptions = BulkLexOptions {
    jsx: false,
    typescript: true,
};
const JSX: BulkLexOptions = BulkLexOptions {
    jsx: true,
    typescript: false,
};
const TSX: BulkLexOptions = BulkLexOptions {
    jsx: true,
    typescript: true,
};

/// Returns the bulk tokens of `source`, after checking them against [JsLexer].
///
/// Every token must be the one that `JsLexer` returns at its start, with the same diagnostics,
/// in each context it claims, and the tokens must cover the source without gaps. A regex must
/// be what `JsLexer` re-lexes the `/` into.
fn check_with(source: &str, options: BulkLexOptions) -> BulkTokens {
    let tokens = BulkTokens::new(&mut JsLexer::from_str(source), options);
    let portable = BulkTokens::new_portable(&mut JsLexer::from_str(source), options);
    assert!(
        tokens.starts == portable.starts && tokens.kinds == portable.kinds,
        "the portable passes disagree on {source:?}"
    );
    let mut reference = JsLexer::from_str(source);
    let len = source.len() as u32;

    let mut expected_start = 0;
    let all: Vec<_> = tokens.iter().collect();
    let (all, eof) = all.split_at(all.len() - 2);
    assert_eq!(
        eof,
        [
            (len, len, EOF as u16 | REGULAR),
            (len, len, EOF as u16 | REGULAR)
        ]
    );

    for &(start, end, raw) in all {
        let describe = || {
            format!(
                "token {:?} with flags {:#x} at {start}..{end} in {source:?} with {options:?}",
                kind(raw),
                raw & !KIND_MASK
            )
        };
        assert_eq!(start, expected_start, "gap before {}", describe());
        assert!(end > start, "empty {}", describe());
        expected_start = end;
        let stored: Vec<_> = tokens
            .diagnostics_at(TextSize::from(start))
            .iter()
            .map(|diagnostic| (format!("{:?}", diagnostic.diagnostic), diagnostic))
            .collect();
        assert_eq!(
            raw & DIAGNOSTICS != 0,
            !stored.is_empty(),
            "diagnostics flag of {}",
            describe()
        );

        let contexts = [
            (REGULAR, JsLexContext::Regular),
            (TEMPLATE, JsLexContext::TemplateElement { tagged: false }),
            (JSX_CHILD, JsLexContext::JsxChild { astro: false }),
            (
                JSX_ATTRIBUTE_VALUE,
                JsLexContext::JsxAttributeValue { astro: false },
            ),
        ];
        assert!(
            contexts.iter().any(|(flag, _)| raw & flag != 0),
            "no context for {}",
            describe()
        );
        for (flag, context) in contexts {
            if raw & flag == 0 {
                continue;
            }
            let lexed = reference.lex_at(start as usize, false, context);
            let mut lexed_end = u32::from(reference.current_range().end());
            let mut lexed_kind = lexed;
            if kind(raw) == JS_REGEX_LITERAL {
                assert!(
                    matches!(lexed, T![/] | T![/=]),
                    "regex isn't a slash for {}",
                    describe()
                );
                lexed_kind = reference.re_lex(JsReLexContext::Regex);
                lexed_end = u32::from(reference.current_range().end());
            }
            assert_eq!(lexed_kind, kind(raw), "{}", describe());
            assert_eq!(lexed_end, end, "{}", describe());
            assert_eq!(
                reference.has_unicode_escape(),
                raw & UNICODE_ESCAPE != 0,
                "{}",
                describe()
            );
            let reported: Vec<_> = reference
                .take_diagnostics_after(0)
                .iter()
                .map(|diagnostic| format!("{diagnostic:?}"))
                .collect();
            let expected: Vec<_> = stored.iter().map(|(text, _)| text.clone()).collect();
            assert_eq!(reported, expected, "diagnostics of {}", describe());
            if flag == TEMPLATE {
                reference.lex_at(
                    start as usize,
                    false,
                    JsLexContext::TemplateElement { tagged: true },
                );
                let reported: Vec<_> = reference
                    .take_diagnostics_after(0)
                    .iter()
                    .map(|diagnostic| format!("{diagnostic:?}"))
                    .collect();
                let expected: Vec<_> = stored
                    .iter()
                    .filter(|(_, diagnostic)| diagnostic.in_tagged_template)
                    .map(|(text, _)| text.clone())
                    .collect();
                assert_eq!(reported, expected, "tagged diagnostics of {}", describe());
            }
        }
    }
    assert_eq!(
        expected_start, len,
        "tokens end before the source in {source:?}"
    );
    tokens
}

/// Checks `source` with the options of JavaScript, TypeScript, JSX and TSX.
fn check(source: &str) {
    for options in [JS, TS, JSX, TSX] {
        check_with(source, options);
    }
}

/// Returns the kinds and texts of the tokens of `source` that aren't trivia.
fn tokens_of(source: &str, options: BulkLexOptions) -> Vec<(JsSyntaxKind, &str)> {
    check_with(source, options)
        .iter()
        .filter(|&(start, end, raw)| !kind(raw).is_trivia() && end > start)
        .map(|(start, end, raw)| (kind(raw), &source[start as usize..end as usize]))
        .collect()
}

/// Checks `snippet` at every offset within the first two 64-byte blocks, so that each of its
/// tokens crosses a block boundary somewhere.
fn check_at_block_boundaries(snippet: &str) {
    for padding in 0..=70 {
        check(&format!("{}{snippet}", " ".repeat(padding)));
        check(&format!("{}{snippet}x", "a".repeat(padding)));
        check(&format!("{}\n{snippet}\n", "/".repeat(padding.max(2))));
    }
}

#[test]
fn identifiers_and_keywords() {
    check_at_block_boundaries("let x = function () { return this.instanceof; }");
    check_at_block_boundaries("constructor implements interfaces satisfiesX _$abc $ _ a1 Z9");
    check_at_block_boundaries("abstract accessor asserts assert any async await boolean using");
    for (keyword, _) in KEYWORDS {
        let keyword = std::str::from_utf8(keyword).unwrap();
        check(&format!(
            "{keyword} {keyword}s x{keyword} {}",
            &keyword[1..]
        ));
    }
}

/// Every keyword the lexer knows, so that a keyword added to the lexer only can't go unnoticed.
#[test]
fn all_keyword_kinds() {
    for raw in BREAK_KW as u16..=USING_KW as u16 {
        let keyword = JsSyntaxKind::from(raw);
        if let Some(text) = keyword.to_string() {
            check(text);
            check(&format!("{text} {text}x x{text}"));
        }
    }
}

#[test]
fn whitespace_and_line_breaks() {
    check_at_block_boundaries("a \t\x0B\x0C b\r\n\n\r\r\nc\n\r");
    check_at_block_boundaries("\r");
    check_at_block_boundaries("\n");
}

#[test]
fn operators() {
    check_at_block_boundaries(
        "a!==b!=c!d===e==f=>g=h&&=i&&j&=k&l||=m||n|=o|p??=q??r?.s?.5:t?u**=v**w*=x*y",
    );
    check_at_block_boundaries("a+++b---c+=d-=e<<=f<<g<=h<i>>=j>>>k>=l%=m%n^=o^p~q@r#s...t..u.v");
    check_at_block_boundaries("(a)[b]{c};d,e:f");
    check_at_block_boundaries("a /= b / c");
}

#[test]
fn numbers() {
    check_at_block_boundaries(
        "0 1 12 1.5 .5 1. 1e5 1E+5 1e-5 1.5e5 .5e5 0x1F 0X1f 0b101 0B1 0o17 0O7 1n 0x1Fn 0b1n 0o7n 0n 0.5 0e5 0E-1",
    );
    check_at_block_boundaries(
        "1..toString 1.toString 0b12 0o78 0x1G 1_000 0_1 01 01.5 01n 08 1e 1e+ 0x",
    );
    check_at_block_boundaries("1_ 1__0 0x_1 1.5_0 1e1_0 5a 5$ 5_ 1.5n a.5 a?.5:b 1..2");
    check_at_block_boundaries(
        "1.e\\u0061 1.\\u0065 0x\\u0031 1é 1.é 1e\\u0031 1.5e\\u{31}x 'a' 1\\u0061'b'",
    );
}

#[test]
fn strings() {
    check_at_block_boundaries(r#"'a' "b" 'it''s' "q\"q" 'a\'b' "\\" '\n\t\r\b\v\f\0'"#);
    check_at_block_boundaries(
        r#"'\x41A\u{41}' "\u{10FFFF}" '\
continued' "\
" 'é' "𐍈""#,
    );
    check_at_block_boundaries(r#"'\x4' "\u004" '\u{' "\u{110000}" '\xZZ'"#);
    check_at_block_boundaries("'unterminated\n\"also\r\n'");
    check_at_block_boundaries("'\u{2028}' \"\u{2029}\"");
}

#[test]
fn unterminated_at_end_of_file() {
    for source in [
        "'abc",
        "\"abc\\",
        "`abc",
        "`abc${",
        "`abc${d",
        "`abc${d}",
        "/* abc",
        "/* abc *",
        "a = /abc",
        "a = /abc\\",
        "a = /[/",
        "// abc",
        "0x",
        "1e",
        "1_",
        "'\\",
        "\\u00",
        "#",
        "#!",
        "`\\",
        "`\\u",
        "a = /a/g\\u0067",
    ] {
        check(source);
        check(&format!("{}{source}", " ".repeat(60)));
    }
}

#[test]
fn comments() {
    check_at_block_boundaries("a // line\nb /* block */ c /* multi\nline */ d /**/ e /*/ x */");
    check_at_block_boundaries(
        "a // line\r\nb // \u{2028} c // \u{2029}\nd /* \u{2028} */ e /* \r */",
    );
    check_at_block_boundaries("a /* é 𐍈 */ b // é\n");
}

#[test]
fn non_ascii() {
    check_at_block_boundaries("café = naïve + ünïcödé; π * 2; 变量 = 1");
    check_at_block_boundaries("a\u{A0}b\u{2003}\u{3000}c\u{FEFF}d \u{A0} e");
    check_at_block_boundaries("a\u{2028}b\u{2029}c \u{2028} d");
    check_at_block_boundaries("a × b; ×; é×é; 1é; a\u{200D}b");
    check_at_block_boundaries("\\u0061bc = a\\u0062c + ab\\u{63} + \\u{1F600} + \\u0030 + a\\u");
    check_at_block_boundaries("\\u0069f (x) \\u{64}o;");
    check("\u{FEFF}#!/usr/bin/env node\nlet a");
    check("\u{FEFF}let a");
    check("#!/usr/bin/env node\nlet a");
    check("#!/usr/bin/env node");
    check("#foo");
    check("a #b \u{FEFF}c");
}

#[test]
fn control_chars() {
    check_at_block_boundaries("a \x00 b \x01 c \x7F d \x1F e");
    check_at_block_boundaries("'\x00' `\x01` /* \x02 */ // \x03\n");
}

#[test]
fn regex_and_division() {
    check_at_block_boundaries("a = /b/g; c = d / e / f; g = (h) / i; j = k[0] / l;");
    check_at_block_boundaries(
        "return /a/.test(b); typeof /a/; x = y++ / 2; x = y-- / 2; z = +/a/;",
    );
    check_at_block_boundaries("if (a) /b/.test(c); {} /d/; ({}) / e; f = `${g}` / h; i = 'j' / k;");
    check_at_block_boundaries(
        "a = /[/]/; b = /\\//; c = /a/gimsuyd; d = /a/gg; e = /a/uv; f = /a/x;",
    );
    check_at_block_boundaries("a = b\n/c/g; x = 1 / 2 / 3; y = a.return / 2; z = this / 2;");
    check_at_block_boundaries("a /= 2; b = /=/; c = /'/; d = /\"/; e = /`/; f = /*/; g = //;");
    check_at_block_boundaries("a = /b\nc/; d = /e\u{2028}f/; g = /é/; h = /\\é/; i = /a/é;");
}

#[test]
fn templates() {
    check_at_block_boundaries("a = `b`; c = ``; d = `e${f}g${h}i`; j = `${k}`; l = `${`${m}`}`;");
    check_at_block_boundaries(
        "a = `${ {b: 1} }`; c = `${ () => { return `d${e}` } }f`; g = `$ $$ {}`;",
    );
    check_at_block_boundaries(
        "a = `\\` \\${b} \\\\ \\u0041 \\x41 \\u{41} \\\r\n \\\n`; c = `é 𐍈 \u{2028}`;",
    );
    check_at_block_boundaries("a = `\\u{110000} \\x4 \\u004`; b = tag`\\unicode`; c = `\\`;");
    check_at_block_boundaries("a = `'\"/* // ${'`'} ${\"}\"} ${/}/} `;");
    check_at_block_boundaries("a = `\nmulti\r\nline\r${b}\n`;");
}

#[test]
fn jsx_and_types() {
    check_at_block_boundaries("<div className=\"a\" {...b}>text's {c} &amp; </div>");
    check_at_block_boundaries("<A<B>>(c) => <d-e f:g='h'/>; type T = Array<Array<number>>;");
    check_at_block_boundaries("a >>= b >>> c; d<<e; f >= g; h > > i; j <<= k;");
}

/// Checks sources glued together from fragments that the passes treat differently, chosen by
/// a fixed pseudo-random sequence.
#[test]
fn random_fragments() {
    const FRAGMENTS: &[&str] = &[
        " ",
        "\t",
        "\n",
        "\r\n",
        "\r",
        "\u{2028}",
        "\u{A0}",
        "a",
        "abc",
        "if",
        "return",
        "typeof",
        "instanceof",
        "constructor",
        "x1",
        "_$",
        "\\u0061",
        "\\u{62}",
        "é",
        "变",
        "×",
        "0",
        "1",
        "12",
        "0x1F",
        "0b1",
        "0o7",
        "1n",
        "1.5",
        ".5",
        "1e5",
        "1e",
        "1_0",
        "0.",
        "'s'",
        "\"d\"",
        "'\\''",
        "'\\x41'",
        "'\\u00'",
        "'",
        "\"",
        "`",
        "`t`",
        "`a${",
        "}",
        "${",
        "{",
        "(",
        ")",
        "[",
        "]",
        ";",
        ",",
        ".",
        "...",
        "=",
        "==",
        "===",
        "=>",
        "!",
        "!=",
        "+",
        "++",
        "-",
        "--",
        "*",
        "**",
        "/",
        "/=",
        "%",
        "<",
        "<<",
        ">",
        ">>",
        "&",
        "&&",
        "|",
        "||",
        "?",
        "??",
        "?.",
        ":",
        "~",
        "^",
        "@",
        "#",
        "#!",
        "//c",
        "/*c*/",
        "/*\n*/",
        "/*",
        "/a/g",
        "/[/]/",
        "\\",
        "\u{0}",
        "\u{1}",
        "\u{7F}",
        "<div>",
        "</div>",
        "<!--",
    ];
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for _ in 0..3000 {
        let count = 1 + next() % 40;
        let source: String = (0..count)
            .map(|_| FRAGMENTS[(next() % FRAGMENTS.len() as u64) as usize])
            .collect();
        check(&source);
        check(&format!("{}{source}", " ".repeat((next() % 64) as usize)));
    }
}

/// Checks every JavaScript and TypeScript file under `dir`, with the options of its extension.
/// Returns how many it checked.
fn check_files_in(dir: &Path) -> usize {
    let mut count = 0;
    let mut stack = vec![dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let options = match path.extension().and_then(|extension| extension.to_str()) {
                Some("js" | "mjs" | "cjs") => JS,
                Some("ts" | "mts" | "cts") => TS,
                Some("jsx") => JSX,
                Some("tsx") => TSX,
                _ => continue,
            };
            if let Ok(source) = std::fs::read_to_string(&path) {
                check_with(&source, options);
                count += 1;
            }
        }
    }
    count
}

#[test]
fn spec_fixtures() {
    let count = check_files_in(&Path::new(env!("CARGO_MANIFEST_DIR")).join("tests"));
    assert!(count > 700, "only found {count} fixtures");
}

/// Checks the files in the directories listed in `BIOME_BULK_LEX_DIRS`, separated by `:`, such
/// as the inputs that the `js_parser` benchmark downloads into the `target` directory.
#[test]
#[ignore = "needs BIOME_BULK_LEX_DIRS"]
fn files_in_directories() {
    let dirs = std::env::var("BIOME_BULK_LEX_DIRS").expect("BIOME_BULK_LEX_DIRS to be set");
    for dir in dirs.split(':') {
        let count = check_files_in(Path::new(dir));
        println!("checked {count} files in {dir}");
    }
}

#[test]
fn regex_decisions() {
    let regexes = |source: &str| -> Vec<String> {
        tokens_of(source, TS)
            .into_iter()
            .filter(|(kind, _)| *kind == JS_REGEX_LITERAL)
            .map(|(_, text)| text.to_string())
            .collect()
    };
    assert_eq!(regexes("a = /b/g; c = d / e / f;"), ["/b/g"]);
    assert_eq!(regexes("if (a) /b/.test(c); x = (a) / 2;"), ["/b/"]);
    assert_eq!(
        regexes("for (const a of b) /c/.test(a); while (x) /d/;"),
        ["/c/", "/d/"]
    );
    assert_eq!(regexes("x = {} / 2; function f() {} /g/.test(a);"), ["/g/"]);
    assert_eq!(
        regexes("x = function () {} / 2; y = class {} / 3;"),
        Vec::<String>::new()
    );
    assert_eq!(regexes("class A {} /c/; () => {} /d/;"), ["/c/", "/d/"]);
    assert_eq!(regexes("x = a++ / 2; y = ++/b/.c;"), ["/b/"]);
    assert_eq!(regexes("x = a! / 2; y = !/b/.test(c);"), ["/b/"]);
    assert_eq!(
        regexes("return /a/; typeof /b/; x = a.return / 2;"),
        ["/a/", "/b/"]
    );
    assert_eq!(regexes("x = `${/a/}` / 2; y = `b` / 3;"), ["/a/"]);
    assert_eq!(
        regexes("x = [1] / 2; y = 'a' / 3; z = 1 / 4;"),
        Vec::<String>::new()
    );
}

#[test]
fn jsx_structure() {
    let tokens = tokens_of("x = <div a=\"it's\" b={c}>don't {d} <e/> </div> / 2;", TSX);
    assert!(tokens.contains(&(JSX_STRING_LITERAL, "\"it's\"")));
    assert!(tokens.contains(&(JSX_TEXT_LITERAL, "don't ")));
    // After the element, `/` divides.
    assert!(tokens.contains(&(T![/], "/")));
    assert!(!tokens.iter().any(|(kind, _)| *kind == JS_REGEX_LITERAL));

    let tokens = tokens_of("<a>{`x${<b>'</b>}`}</a>", JSX);
    assert!(tokens.contains(&(JSX_TEXT_LITERAL, "'")));
    assert!(tokens.contains(&(TEMPLATE_CHUNK, "x")));

    // Type parameters of a TSX arrow function aren't an element.
    let tokens = tokens_of(
        "const f = <T,>(a: T) => a; const g = <T extends U>(a: T) => a;",
        TSX,
    );
    assert!(!tokens.iter().any(|(kind, _)| *kind == JSX_TEXT_LITERAL));
    // Nor are the type parameters of a function type or a call signature.
    let tokens = tokens_of(
        "const f: <T>(x: T) => T = (x) => x;\ntype G = { <T>(x: T): T };\nfoo();",
        TSX,
    );
    assert!(!tokens.iter().any(|(kind, _)| *kind == JSX_TEXT_LITERAL));
    // A comparison isn't an element either.
    let tokens = tokens_of("x = a < b; y = c <d> e;", JSX);
    assert!(!tokens.iter().any(|(kind, _)| *kind == JSX_TEXT_LITERAL));
}

#[test]
fn jsx() {
    check_at_block_boundaries("x = <div className=\"a\" {...b}>text's {c} &amp; <br/> </div>;");
    check_at_block_boundaries("<><a.b.c d-e:f='g' h=<i/> j={() => <k>{l}</k>}>m</a.b.c></>;");
    check_at_block_boundaries("<a>></a>; <b>}</b>; <c>{/* d */}</c>; <e>\n  multi\n  line\n</e>;");
    check_at_block_boundaries("<a\n  /* c */ b // d\n  ='e'\n/>; <f>{`${<g/>}`}</f>; <h>{/i/}</h>");
    check_at_block_boundaries("<Generic<T> a={1} />; <A<B<C>>></A>; <a b=\"unterminated />");
    check_at_block_boundaries("x = cond ? <a/> : <b>c</b>; return <d/>; f(<e/>, <f/>);");
    check_at_block_boundaries("<a><<b/><=c/></a>; <d></e>; <f g=></f>; <h");
}
