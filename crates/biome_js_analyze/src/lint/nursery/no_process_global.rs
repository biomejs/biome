use crate::services::semantic::Semantic;
use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::JsImport;
use biome_js_syntax::JsLanguage;
use biome_js_syntax::JsModuleItemList;
use biome_js_syntax::JsSyntaxKind;
use biome_js_syntax::T;
use biome_js_syntax::{global_identifier, AnyJsExpression};
use biome_rowan::AstNode;
use biome_rowan::BatchMutationExt;
use biome_rowan::SyntaxElement;
use biome_rowan::SyntaxNode;
use biome_rowan::TriviaPieceKind;

declare_lint_rule! {
    /// Disallow the use of `process` global.
    ///
    /// Node.js and Deno expose `process` global but they are hard to statically analyze by tools,
    /// so code should not assume they are available. Instead, `import process from "node:process"`.
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = process.env.FOO;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import process from "node:process";
    ///
    /// const foo = process.env.FOO;
    /// ```
    ///
    /// The rule is not able to detect cases where the global object is aliased:
    ///
    /// ```js
    /// const foo = globalThis;
    /// const bar = foo.process;
    /// ```
    ///
    pub NoProcessGlobal {
        version: "next",
        name: "noProcessGlobal",
        language: "js",
        sources: &[RuleSource::DenoLint("no-process-global")],
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoProcessGlobal {
    type Query = Semantic<AnyJsExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let (reference, name) = global_identifier(node)?;
        if name.text() != "process" {
            return None;
        }
        model.binding(&reference).is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Usage of the `process` global is discouraged."
                },
            ).note(markup! {
                "`process` global is hard for tools to statically analyze, so code should not assume they are available."
            })
            .note(markup! {
                "Add `import process from \"node:process\";` to this file's imports."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        if let Some(top_level_statement) = ctx
            .query()
            .syntax()
            .ancestors()
            .find(is_top_level_statement)
        {
            // insert new import at:
            // 1. after the most recent import statement. Or, if no import exist
            // 2. at the beginning of the file
            let mut most_recent_import = top_level_statement.prev_sibling_or_token();
            loop {
                match &most_recent_import {
                    Some(node) => {
                        if matches!(node.kind(), JsSyntaxKind::JS_IMPORT) {
                            break;
                        }

                        most_recent_import = node.prev_sibling_or_token();
                    }
                    _ => break,
                }
            }

            let Some(module_item_list) = top_level_statement
                .parent()
                .and_then(JsModuleItemList::cast)
            else {
                return None;
            };
            let module_item_list = module_item_list.into_syntax();
            let mut slot = 0;
            let new_items: [SyntaxNode<JsLanguage>; 2];
            let new_process_import = create_porcess_import();

            // WIP: need to handle trivias properly, e.g. append below any head comments for case 2
            if let Some(Some(import)) = most_recent_import.map(|node| node.into_node()) {
                let Some(import_slot) = module_item_list
                    .slots()
                    .position(|slot| slot.into_node().as_ref() == Some(&import))
                else {
                    return None;
                };
                slot = import_slot;
                new_items = [import, new_process_import.into()];
            } else {
                let Some(first_child) = module_item_list.first_child() else {
                    return None;
                };
                new_items = [new_process_import.into(), first_child];
            }

            let new_module_item_list = module_item_list.clone().splice_slots(
                slot..(slot + 1),
                new_items
                    .into_iter()
                    .map(|item| Some(SyntaxElement::Node(item))),
            );
            mutation.replace_element(module_item_list.into(), new_module_item_list.into());
            return Some(JsRuleAction::new(
                ctx.metadata().action_category(ctx.category(), ctx.group()),
                ctx.metadata().applicability(),
                markup! { "Add `import process from \"node:process\";` to this file's imports." }
                    .to_owned(),
                mutation,
            ));
        }

        None
    }
}

fn is_top_level_statement(node: &SyntaxNode<JsLanguage>) -> bool {
    match node.parent() {
        Some(g) => JsModuleItemList::cast(g).is_some(),
        _ => false,
    }
}

fn create_porcess_import() -> JsImport {
    let whitespace = [(TriviaPieceKind::Whitespace, " ")];
    let new_line = [(TriviaPieceKind::Newline, "\n")];
    let source = make::js_module_source(make::js_string_literal("node::process"));
    let binding = make::js_identifier_binding(
        make::ident("process").with_trailing_trivia(whitespace.clone()),
    );
    let specifier = make::js_default_import_specifier(binding.into());
    let clause = make::js_import_default_clause(
        specifier,
        make::token(T![from]).with_trailing_trivia(whitespace.clone()),
        source.into(),
    )
    .build();
    make::js_import(
        make::token(T![import])
            .with_trailing_trivia(whitespace.clone())
            .with_leading_trivia(new_line.clone()),
        clause.into(),
    )
    .with_semicolon_token(make::token(T![;]).with_trailing_trivia(new_line.clone()))
    .build()
}
