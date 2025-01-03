use biome_syntax_codegen::{generate_grammar, GrammarOptions, LanguageSrc};
use std::env::current_dir;

struct Source;

impl LanguageSrc for Source {
    fn punct(&self) -> &[(&str, &str)] {
        &[
            ("?", "QUESTION"),
            ("*", "ASTERISK"),
            (":", "COLON"),
            ("(", "L_BRACKET"),
            (")", "R_BRACKET"),
            ("|", "OR"),
            ("||", "DOUBLE_OR"),
            ("&&", "DOUBLE_AND"),
        ]
    }

    fn language_prefix(&self) -> &str {
        "mini"
    }

    fn keywords(&self) -> &[&str] {
        &["complex", "simple"]
    }

    fn literals(&self) -> &[&str] {
        &["MINI_STRING_LITERAL"]
    }

    fn tokens(&self) -> &[&str] {
        &["NEWLINE", "WHITESPACE", "IDENT", "COMMENT", "COMMA"]
    }

    fn nodes(&self) -> &[&str] {
        &[
            "MINI_ROOT",
            "MINI_GRAMMAR",
            "MINI_NODE_LIST",
            "MINI_SIMPLE_NODE",
            "MINI_COMPLEX_NODE",
            "MINI_BOGUS",
            "MINI_NODE_LIST",
        ]
    }

    fn prefixes(&self) -> &[&str] {
        &["mini_", "min_"]
    }

    fn to_method_name<'a>(&self, token_name: &'a str) -> &'a str {
        match token_name {
            "')'" => "l_bracket",
            "'('" => "r_bracket",
            _ => token_name,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let current_dir = current_dir()?.join("examples");
    let options = GrammarOptions {
        syntax_crate_name: "crate".to_string(),
        check_unions: false,
        ungrammar_file_path: current_dir.join("mini.ungram"),
        factory_dir_path: current_dir.join("./factory"),
        syntax_dir_path: current_dir.join("./syntax"),
    };

    generate_grammar(Source, options)?;

    Ok(())
}
