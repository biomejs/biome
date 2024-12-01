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

pub(crate) struct AsCompiler;

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
                format!("It is usually incorrect to redefine a variable \"{name}\" using `as`"),
                node.range(),
            ));
        }

        let pattern = PatternCompiler::from_node(&pattern, context)?;
        let variable = VariableCompiler::from_node(&variable, context);
        Ok(Where::new(
            Pattern::Variable(variable.clone()),
            Predicate::Match(Box::new(Match::new(
                Container::Variable(variable),
                Some(pattern),
            ))),
        ))
    }
}

fn pattern_repeated_variable(
    pattern: &AnyGritPattern,
    name: &str,
    lang: &impl Language,
) -> Result<bool, CompileError> {
    let node = GritNode::from(pattern.syntax());
    let cursor = traverse(node.walk(), Order::Pre);
    Ok(cursor
        .filter(|node| {
            node.kind() == GritSyntaxKind::GRIT_VARIABLE
                || node.kind() == GritSyntaxKind::GRIT_CODE_SNIPPET
        })
        .map(|node| {
            let text = node.text_trimmed();
            if node.kind() == GritSyntaxKind::GRIT_VARIABLE {
                Ok(text == name)
            } else {
                Ok(is_variables_in_snippet(name, &text.to_string(), lang))
            }
        })
        .collect::<Result<Vec<bool>, CompileError>>()?
        .into_iter()
        .any(|b| b))
}

fn is_variables_in_snippet(name: &str, snippet: &str, lang: &impl Language) -> bool {
    let variables = split_snippet(snippet, lang);
    variables.iter().any(|variable| variable.1 == name)
}
