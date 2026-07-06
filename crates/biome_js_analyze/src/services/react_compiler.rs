use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryMatch, Queryable, RuleKey, RuleMetadata,
    ServiceBag, ServicesDiagnostic, Visitor, VisitorContext, VisitorFinishContext,
};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsSyntaxNode, TextRange, WalkEvent};
use biome_react_compiler::{
    CompileInput, CompileOutput, ReactCompilerError, compile_program, default_lint_options,
};
use biome_rowan::AstNode;
use biome_rule_options::use_react_compiler::UseReactCompilerOptions;
use std::sync::Arc;

use super::semantic::SemanticModelBuilderVisitor;
use crate::lint::nursery::use_react_compiler::UseReactCompiler;

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

impl Queryable for ReactCompilerServices {
    type Input = ReactCompilerEvent;
    type Output = ReactCompilerMatch;

    type Language = JsLanguage;
    type Services = Self;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, root: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor::new(root));
        analyzer.add_visitor(Phases::Syntax, || ReactCompilerVisitor::new(root.clone()));
        analyzer.add_visitor(Phases::Semantic, ReactCompilerQueryVisitor::default);
    }

    fn unwrap_match(services: &ServiceBag, event: &ReactCompilerEvent) -> Self::Output {
        let result = services
            .get_service::<ReactCompilerResult>()
            .expect("ReactCompilerResult service is not registered")
            .clone();
        ReactCompilerMatch {
            result,
            range: event.text_range(),
        }
    }
}

#[derive(Clone)]
pub struct ReactCompilerMatch {
    pub result: ReactCompilerResult,
    pub range: TextRange,
}

pub struct ReactCompilerEvent(TextRange);

impl QueryMatch for ReactCompilerEvent {
    fn text_range(&self) -> TextRange {
        self.0
    }
}

struct ReactCompilerVisitor {
    root: AnyJsRoot,
    /// Options of the `useReactCompiler` rule, captured from the analyzer
    /// options on the first walk event: compilation happens in [`Self::finish`],
    /// which has no access to [`biome_analyze::AnalyzerOptions`].
    options: Option<UseReactCompilerOptions>,
}

impl ReactCompilerVisitor {
    fn new(root: AnyJsRoot) -> Self {
        Self {
            root,
            options: None,
        }
    }
}

impl Visitor for ReactCompilerVisitor {
    type Language = JsLanguage;

    fn visit(&mut self, _: &WalkEvent<JsSyntaxNode>, ctx: VisitorContext<JsLanguage>) {
        if self.options.is_none() {
            self.options = Some(
                ctx.options
                    .rule_options::<UseReactCompiler>()
                    .unwrap_or_default(),
            );
        }
    }

    fn finish(self: Box<Self>, ctx: VisitorFinishContext<JsLanguage>) {
        if ctx.services.get_service::<ReactCompilerResult>().is_none() {
            let result = match ctx.services.get_service::<SemanticModel>() {
                Some(model) => {
                    let source_type = ctx
                        .services
                        .get_service::<biome_js_syntax::JsFileSource>()
                        .copied()
                        .unwrap_or_default();
                    let source = self.root.syntax().text_with_trivia().to_string();

                    let mut options = default_lint_options(&source);
                    options.compilation_mode = self
                        .options
                        .unwrap_or_default()
                        .compilation_mode()
                        .as_compiler_mode()
                        .to_string();

                    compile_program(CompileInput {
                        root: &self.root,
                        model,
                        source: &source,
                        source_type,
                        options,
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
}

#[derive(Default)]
struct ReactCompilerQueryVisitor;

impl Visitor for ReactCompilerQueryVisitor {
    type Language = JsLanguage;

    fn visit(&mut self, event: &WalkEvent<JsSyntaxNode>, mut ctx: VisitorContext<JsLanguage>) {
        let root = match event {
            WalkEvent::Enter(node) => {
                if node.parent().is_some() {
                    return;
                }

                node
            }
            WalkEvent::Leave(_) => return,
        };

        ctx.match_query(ReactCompilerEvent(root.text_range_with_trivia()));
    }
}
