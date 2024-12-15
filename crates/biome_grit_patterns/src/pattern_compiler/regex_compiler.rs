use super::{compilation_context::NodeCompilationContext, variable_compiler::VariableCompiler};
use crate::{
    diagnostics::CompilerDiagnostic, grit_context::GritQueryContext,
    pattern_compiler::snippet_compiler::parse_snippet_content, util::TextRangeGritExt,
    CompileError,
};
use biome_grit_syntax::{AnyGritRegex, GritRegexPattern};
use grit_pattern_matcher::pattern::{RegexLike, RegexPattern};
use grit_util::Language;

pub(crate) struct RegexCompiler;

impl RegexCompiler {
    pub(crate) fn from_node(
        node: &GritRegexPattern,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<RegexPattern<GritQueryContext>, CompileError> {
        if is_rhs {
            return Err(CompileError::InvalidRegexPosition);
        }

        let regex = match node.regex()? {
            AnyGritRegex::GritRegexLiteral(regex_node) => {
                let token = regex_node.value_token()?;
                let regex = token.text_trimmed();
                debug_assert!(regex.starts_with("r\"") && regex.ends_with('"'));
                RegexLike::Regex(regex[2..regex.len() - 1].to_string())
            }
            AnyGritRegex::GritSnippetRegexLiteral(regex_node) => {
                let token = regex_node.value_token()?;
                let regex = token.text_trimmed();
                let range = token.text_trimmed_range().to_byte_range();
                debug_assert!(regex.starts_with("r`") && regex.ends_with('`'));

                if !context
                    .compilation
                    .lang
                    .metavariable_regex()
                    .is_match(regex)
                {
                    let alternative = format!("r\"{}\"", &regex[2..regex.len() - 1]);
                    context.log(CompilerDiagnostic::new_warning(
                        format!("Unnecessary use of metavariable snippet syntax without metavariables. Replace {regex} with {alternative}"),
                        token.text_trimmed_range(),
                    ));
                }

                let pattern =
                    parse_snippet_content(&regex[2..regex.len() - 1], range, context, is_rhs)?;
                RegexLike::Pattern(Box::new(pattern))
            }
        };

        let variables: Vec<_> = node
            .variables()
            .map(|variables| variables.args())
            .map(|args| {
                args.into_iter()
                    .filter_map(Result::ok)
                    .map(|variable| VariableCompiler::from_node(&variable, context))
                    .collect()
            })
            .unwrap_or_default();

        Ok(RegexPattern::new(regex, variables))
    }
}
