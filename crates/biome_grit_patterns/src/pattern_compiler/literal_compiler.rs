use super::{
    compilation_context::NodeCompilationContext, list_compiler::ListCompiler,
    map_compiler::MapCompiler, snippet_compiler::parse_snippet_content,
};
use crate::{
    grit_context::GritQueryContext, util::TextRangeGritExt, CompileError, GritTargetLanguage,
};
use biome_grit_syntax::{AnyGritCodeSnippetSource, AnyGritLiteral, GritSyntaxKind};
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::{
    BooleanConstant, FloatConstant, IntConstant, Pattern, StringConstant,
};

pub(crate) struct LiteralCompiler;

impl LiteralCompiler {
    pub(crate) fn from_node_with_rhs(
        node: &AnyGritLiteral,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Pattern<GritQueryContext>, CompileError> {
        match node {
            AnyGritLiteral::GritBooleanLiteral(node) => Ok(Pattern::BooleanConstant(
                BooleanConstant::new(node.value()?.text_trimmed() == "true"),
            )),
            AnyGritLiteral::GritCodeSnippet(node) => match node.source()? {
                AnyGritCodeSnippetSource::GritBacktickSnippetLiteral(node) => {
                    let token = node.value_token()?;
                    let text = token.text_trimmed();
                    let range = node.syntax().text_trimmed_range().to_byte_range();
                    debug_assert!(text.len() >= 2, "Literals must have quotes");
                    parse_snippet_content(&text[1..text.len() - 1], range, context, is_rhs)
                }
                AnyGritCodeSnippetSource::GritLanguageSpecificSnippet(node) => {
                    let lang_node = node.language()?;
                    let lang_name = lang_node.to_trimmed_string();
                    if GritTargetLanguage::from_extension(&lang_name).is_none() {
                        return Err(CompileError::UnknownTargetLanguage(lang_name));
                    }

                    let snippet_token = node.snippet_token()?;
                    let source = snippet_token.text_trimmed();
                    let range = node.syntax().text_trimmed_range().to_byte_range();
                    debug_assert!(source.len() >= 2, "Literals must have quotes");
                    parse_snippet_content(&source[1..source.len() - 1], range, context, is_rhs)
                }
                AnyGritCodeSnippetSource::GritRawBacktickSnippetLiteral(node) => {
                    if !is_rhs {
                        return Err(CompileError::InvalidRawSnippetPosition);
                    }

                    let token = node.value_token()?;
                    let source = token.text_trimmed();
                    let range = token.text_trimmed_range().to_byte_range();
                    debug_assert!(source.starts_with("raw`") && source.ends_with('`'));
                    parse_snippet_content(&source[4..source.len() - 1], range, context, is_rhs)
                }
            },
            AnyGritLiteral::GritDoubleLiteral(node) => Ok(Pattern::FloatConstant(
                FloatConstant::new(node.value_token()?.text_trimmed().parse().map_err(|err| {
                    CompileError::LiteralOutOfRange(format!("Error parsing double: {err}"))
                })?),
            )),
            AnyGritLiteral::GritIntLiteral(node) => Ok(Pattern::IntConstant(IntConstant::new(
                node.value_token()?.text_trimmed().parse().map_err(|err| {
                    CompileError::LiteralOutOfRange(format!("Error parsing integer: {err}"))
                })?,
            ))),
            AnyGritLiteral::GritList(node) => Ok(Pattern::List(Box::new(
                ListCompiler::from_node_with_rhs(node, context, is_rhs)?,
            ))),
            AnyGritLiteral::GritMap(node) => Ok(Pattern::Map(Box::new(
                MapCompiler::from_node_with_rhs(node, context, is_rhs)?,
            ))),
            AnyGritLiteral::GritStringLiteral(node) => {
                let token = node.value_token()?;
                let text = token.text_trimmed();
                debug_assert!(text.len() >= 2, "Strings must have quotes");
                Ok(Pattern::StringConstant(StringConstant::new(unescape(
                    &text[1..text.len() - 1],
                ))))
            }
            AnyGritLiteral::GritUndefinedLiteral(_) => Ok(Pattern::Undefined),
            AnyGritLiteral::GritBogusLiteral(_) => Err(CompileError::UnexpectedKind(
                GritSyntaxKind::GRIT_BOGUS_LITERAL.into(),
            )),
        }
    }
}

fn unescape(string_literal: &str) -> String {
    let mut escaped = false;
    let mut value = String::with_capacity(string_literal.len());
    for c in string_literal.chars() {
        if escaped {
            match c {
                'n' => value.push('\n'),
                'r' => value.push('\r'),
                't' => value.push('\t'),
                '\\' => value.push('\\'),
                c => value.push(c),
            }
        } else if c == '\\' {
            escaped = true;
        } else {
            value.push(c);
        }
    }

    value
}
