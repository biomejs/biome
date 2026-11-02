mod quick_test;
mod spec_test;

mod formatter {
    mod markdown_module {
        tests_macros::gen_tests! {
            "tests/specs/markdown/**/*.md",
            crate::spec_test::run,
            ""
        }
    }
}

#[test]
fn prose_always_reflows_soft_break_in_emphasis() {
    use biome_markdown_formatter::{
        MdFormatLanguage,
        context::{MdFormatOptions, ProseWrap},
    };
    use biome_markdown_parser::{MarkdownParserOptions, parse_markdown};

    for (source, expected) in [
        (
            "Before **alpha\nbeta** after.\n",
            "Before **alpha beta** after.\n",
        ),
        (
            "Before ***alpha\nbeta*** after.\n",
            "Before **_alpha beta_** after.\n",
        ),
        (
            "Before **alpha *beta\ngamma* delta** after.\n",
            "Before **alpha _beta gamma_ delta** after.\n",
        ),
        (
            "Before **alpha _beta\ngamma__ delta** after.\n",
            "Before **alpha _beta gamma__ delta** after.\n",
        ),
        (
            "Before **alpha [beta\ngamma](/target) delta** after.\n",
            "Before **alpha [beta gamma](/target) delta** after.\n",
        ),
        (">> **alpha\n> beta**\n", "> > **alpha beta**\n"),
        (
            "> **alpha [beta\n> gamma](/target) delta**\n",
            "> **alpha [beta gamma](/target) delta**\n",
        ),
        (
            "Before ___alpha\nbeta__ after.\n",
            "Before **\\_alpha beta** after.\n",
        ),
        (
            "Before __alpha\nbeta___ after.\n",
            "Before **alpha beta\\_** after.\n",
        ),
        (
            "> Before **alpha\n> beta** after.\n",
            "> Before **alpha beta** after.\n",
        ),
        (
            "> Before **alpha\nbeta** after.\n",
            "> Before **alpha beta** after.\n",
        ),
        (
            "# **This heading contains enough words that wrapping it would split the heading into separate blocks**\n",
            "# **This heading contains enough words that wrapping it would split the heading into separate blocks**\n",
        ),
    ] {
        let parsed = parse_markdown(source, MarkdownParserOptions::default());
        let formatted = biome_formatter::format_node(
            &parsed.syntax(),
            MdFormatLanguage::new(MdFormatOptions::default().with_prose_wrap(ProseWrap::Always)),
            false,
        )
        .unwrap()
        .print()
        .unwrap();

        assert_eq!(formatted.as_code(), expected, "{source}");
    }
}
