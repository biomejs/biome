use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleKey, RuleMetadata,
    ServiceBag, ServicesDiagnostic, SyntaxVisitor, Visitor, VisitorContext, VisitorFinishContext,
};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsSyntaxNode, WalkEvent};
use biome_react_compiler::{
    CompileInput, CompileOutput, ReactCompilerError, compile_program, default_lint_options,
};
use biome_rowan::AstNode;
use std::sync::Arc;

use super::semantic::SemanticModelBuilderVisitor;

pub type ReactCompilerResult = Arc<Result<CompileOutput, ReactCompilerError>>;

pub struct ReactCompilerServices {
    result: ReactCompilerResult,
}

impl ReactCompilerServices {
    pub fn result(&self) -> &Result<CompileOutput, ReactCompilerError> {
        &self.result
    }
}

impl FromServices for ReactCompilerServices {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let result = services
            .get_service::<ReactCompilerResult>()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["ReactCompilerResult"]))?
            .clone();

        Ok(Self { result })
    }
}

impl Phase for ReactCompilerServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

#[derive(Clone)]
pub struct ReactCompiler<N>(pub N);

impl<N> Queryable for ReactCompiler<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Input = JsSyntaxNode;
    type Output = N;

    type Language = JsLanguage;
    type Services = ReactCompilerServices;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, root: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor::new(root));
        analyzer.add_visitor(Phases::Semantic, || ReactCompilerVisitor::new(root.clone()));
        analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}

struct ReactCompilerVisitor {
    root: AnyJsRoot,
}

impl ReactCompilerVisitor {
    fn new(root: AnyJsRoot) -> Self {
        Self { root }
    }
}

impl Visitor for ReactCompilerVisitor {
    type Language = JsLanguage;

    fn visit(&mut self, _: &WalkEvent<JsSyntaxNode>, _: VisitorContext<JsLanguage>) {}

    fn finish(self: Box<Self>, ctx: VisitorFinishContext<JsLanguage>) {
        if ctx.services.get_service::<ReactCompilerResult>().is_some() {
            return;
        }

        let result = match ctx.services.get_service::<SemanticModel>() {
            Some(model) => {
                let source_type = ctx
                    .services
                    .get_service::<biome_js_syntax::JsFileSource>()
                    .copied()
                    .unwrap_or_default();
                let source = self.root.syntax().text_with_trivia().to_string();

                compile_program(CompileInput {
                    root: &self.root,
                    model,
                    source: &source,
                    source_type,
                    options: default_lint_options(&source),
                })
            }
            None => Err(ReactCompilerError::CompilerOutput(
                "SemanticModel service is not registered".to_string(),
            )),
        };

        ctx.services
            .insert_service(Arc::new(result) as ReactCompilerResult);
    }
}
