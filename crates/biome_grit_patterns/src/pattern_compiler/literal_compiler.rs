use super::{compilation_context::CompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{AnyGritListPattern, AnyGritLiteral, GritSyntaxKind};
use grit_pattern_matcher::pattern::{
    BooleanConstant, FloatConstant, IntConstant, List, Pattern, StringConstant,
};

pub(crate) struct LiteralCompiler;

impl LiteralCompiler {
    pub(crate) fn from_node_with_rhs(
        node: &AnyGritLiteral,
        context: &mut CompilationContext,
        is_rhs: bool,
    ) -> Result<Pattern<GritQueryContext>, CompileError> {
        match node {
            AnyGritLiteral::GritBooleanLiteral(bool) => Ok(Pattern::BooleanConstant(
                BooleanConstant::new(bool.value()?.text_trimmed() == "true"),
            )),
            AnyGritLiteral::GritCodeSnippet(_) => todo!(),
            AnyGritLiteral::GritDoubleLiteral(double) => Ok(Pattern::FloatConstant(
                FloatConstant::new(double.value_token()?.text_trimmed().parse().map_err(
                    |err| CompileError::LiteralOutOfRange(format!("Error parsing double: {err}")),
                )?),
            )),
            AnyGritLiteral::GritIntLiteral(int) => Ok(Pattern::IntConstant(IntConstant::new(
                int.value_token()?.text_trimmed().parse().map_err(|err| {
                    CompileError::LiteralOutOfRange(format!("Error parsing integer: {err}"))
                })?,
            ))),
            AnyGritLiteral::GritList(list) => {
                let patterns = list
                    .patterns()
                    .into_iter()
                    .map(|pattern| match pattern {
                        Ok(pattern) => Ok(compile_list_pattern(&pattern, context, is_rhs)?),
                        Err(error) => Err(CompileError::from(error)),
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Pattern::List(Box::new(List::new(patterns))))
            }
            AnyGritLiteral::GritMap(_) => todo!(),
            AnyGritLiteral::GritStringLiteral(string) => {
                let token = string.value_token()?;
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

fn compile_list_pattern(
    node: &AnyGritListPattern,
    context: &mut CompilationContext,
    is_rhs: bool,
) -> Result<Pattern<GritQueryContext>, CompileError> {
    match node {
        AnyGritListPattern::AnyGritPattern(pattern) => {
            PatternCompiler::from_node_with_rhs(pattern, context, is_rhs)
        }
        AnyGritListPattern::GritDotdotdot(_) => Ok(Pattern::Dots),
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
