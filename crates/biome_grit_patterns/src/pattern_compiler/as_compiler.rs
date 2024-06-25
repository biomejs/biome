use super::{
    compilation_context::NodeCompilationContext, snippet_compiler::split_snippet,
    variable_compiler::VariableCompiler, PatternCompiler,
};
use crate::{
    diagnostics::CompilerDiagnostic, grit_context::GritQueryContext, grit_node::GritNode,
    CompileError,
};
use biome_grit_syntax::{AnyGritPattern, GritPatternAs, GritSyntaxKind};
use biome_rowan::AstNode as _;
use grit_pattern_matcher::pattern::{Container, Match, Pattern, Predicate, Where};
use grit_util::{traverse, AstNode, Language, Order};

// TODO: `as` keyword
#[allow(dead_code)]
pub(crate) struct AsCompiler;

#[allow(dead_code)]
impl AsCompiler {
    pub(crate) fn from_node(
        node: &GritPatternAs,
        context: &mut NodeCompilationContext,
    ) -> Result<Where<GritQueryContext>, CompileError> {
        let pattern = node.pattern()?;
        let variable = node.variable()?;

        let name = variable.value_token()?;
        let name = name.text_trimmed();

        // this just searches the subtree for a variables that share the name.
        // could possible lead to some false positives, but more precise solutions
        // require much greater changes.
        if pattern_repeated_variable(&pattern, name, &context.compilation.lang)? {
            context.log(CompilerDiagnostic::new_warning(
                format!("It is usually incorrect to redefine a variable {name} using as"),
                node.range(),
            ));
        }

        let pattern = PatternCompiler::from_node(&pattern, context)?;
        let variable = VariableCompiler::from_node(&variable, context);
        Ok(Where::new(
            Pattern::Variable(variable),
            Predicate::Match(Box::new(Match::new(
                Container::Variable(variable),
                Some(pattern),
            ))),
        ))
    }
}

#[allow(dead_code)]
fn pattern_repeated_variable(
    pattern: &AnyGritPattern,
    name: &str,
    lang: &impl Language,
) -> Result<bool, CompileError> {
    let node = GritNode::from(pattern.syntax());
    let cursor = traverse(node.walk(), Order::Pre);
    Ok(cursor
        .filter(|n| {
            n.kind() == GritSyntaxKind::GRIT_VARIABLE
                || n.kind() == GritSyntaxKind::GRIT_CODE_SNIPPET
        })
        .map(|n| {
            let s = n.text_trimmed();
            if n.kind() == GritSyntaxKind::GRIT_VARIABLE {
                Ok(s == name)
            } else {
                Ok(is_variables_in_snippet(name, &s.to_string(), lang))
            }
        })
        .collect::<Result<Vec<bool>, CompileError>>()?
        .into_iter()
        .any(|b| b))
}

#[allow(dead_code)]
fn is_variables_in_snippet(name: &str, snippet: &str, lang: &impl Language) -> bool {
    let variables = split_snippet(snippet, lang);
    variables.iter().any(|v| v.1 == name)
}
