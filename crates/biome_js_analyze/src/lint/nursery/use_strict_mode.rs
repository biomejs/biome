use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, options::PreferredQuote, Ast, FixKind, Rule,
    RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make::{js_directive, js_directive_list, token};
use biome_js_syntax::{JsScript, JsSyntaxKind, JsSyntaxToken, T};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TriviaPieceKind};

declare_lint_rule! {
    /// Enforce the use of the directive `"use strict"` in script files.
    ///
    /// The JavaScript [strict mode](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Strict_mode) prohibits some obsolete JavaScript syntaxes and makes some slight semantic changes to allow more optimizations by JavaScript engines.
    /// EcmaScript modules are always in strict mode, while JavaScript scripts are by default in non-strict mode, also known as _sloppy mode_.
    /// A developer can add the `"use strict"` directive at the start of a script file to enable the strict mode in that file.
    ///
    /// Biome considers a CommonJS (`.cjs`) file as a script file.
    /// By default, Biome recognizes a JavaScript file (`.js`) as a module file, except if `"type": "commonjs"` is specified in `package.json`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```cjs,expect_diagnostic
    /// var a = 1;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```cjs
    /// "use strict";
    ///
    /// var a = 1;
    /// ```
    ///
    pub UseStrictMode {
        version: "1.8.0",
        name: "useStrictMode",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseStrictMode {
    type Query = Ast<JsScript>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.directives().is_empty() && node.statements().is_empty() {
            return None;
        }

        if node.directives().iter().any(|directive| {
            directive
                .inner_string_text()
                .is_ok_and(|text| text.text() == "use strict")
        }) {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected absence of the directive "<Emphasis>"\"use strict\"."</Emphasis>
                },
            )
            .note(markup! {
                "Strict mode allows to opt-in some optimisations of the runtime engines, and it eliminates some JavaScript silent errors by changing them to throw errors."
            })
            .note(markup!{
                "Check the "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Strict_mode">"MDN web docs"</Hyperlink>" for more information regarding strict mode."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query().clone();
        let mut mutation = ctx.root().begin();
        let value = match ctx.as_preferred_quote() {
            PreferredQuote::Double => "\"use strict\"",
            PreferredQuote::Single => "'use strict'",
        };

        // check if the first statement has a newline as the first trivia.
        let is_statement_first_trivia_newline = node
            .statements()
            .syntax()
            .first_leading_trivia()
            .is_some_and(|f| {
                f.pieces()
                    .next()
                    .is_some_and(|piece| piece.kind() == TriviaPieceKind::Newline)
            });

        let mut leading_trivia = Vec::new();
        let mut trailing_trivia = Vec::new();

        // if the script has directives or an interpreter directive, add a newline before the "use strict" directive.
        if !node.directives().is_empty() || node.interpreter_token().is_some() {
            leading_trivia.push((TriviaPieceKind::Newline, "\n"));
        }

        // if the script has statements and the first statement does not have a newline before it, add a newline behind the "use strict" directive.
        if !node.statements().is_empty() && !is_statement_first_trivia_newline {
            trailing_trivia.push((TriviaPieceKind::Newline, "\n"));
        }

        let mut strict_directive_token =
            JsSyntaxToken::new_detached(JsSyntaxKind::JSX_STRING_LITERAL, value, [], []);

        if !leading_trivia.is_empty() {
            strict_directive_token = strict_directive_token.with_leading_trivia(leading_trivia);
        }

        let mut strict_directive = js_directive(strict_directive_token);

        if !trailing_trivia.is_empty() {
            strict_directive = strict_directive
                .with_semicolon_token(token(T![;]).with_trailing_trivia(trailing_trivia))
        } else {
            strict_directive = strict_directive.with_semicolon_token(token(T![;]));
        };

        let directives = js_directive_list(
            node.directives()
                .into_iter()
                .chain([strict_directive.build()])
                .collect::<Vec<_>>(),
        );

        let new_node = node.clone().with_directives(directives);

        // use replace_element_discard_trivia to prevent duplication of the leading comment when it is the first element in the node's leading trivia.
        mutation.replace_element_discard_trivia(node.into(), new_node.into());
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup!("Insert a top level "<Emphasis>"\"use strict\""</Emphasis>".").to_owned(),
            mutation,
        ))
    }
}
