use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, options::PreferredQuote, ActionCategory, Ast, FixKind,
    Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{JsScript, JsSyntaxKind, JsSyntaxToken, T};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};

declare_lint_rule! {
    /// Enforce the use of the directive `"use strict"` in script files.
    ///
    /// The JavaScript [strict mode](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Strict_mode) prohibits some obsolete JavaScript syntaxes and makes some slight semantic chnmages to allow more optimizations by JavaScript engines.
    ///  EcmaScript modules are always in strict mode, while JavaScript scripts are by default in non-strict mode, also known as _sloppy mode_.
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
        if node
            .directives()
            .iter()
            .filter_map(|directive| directive.inner_string_text().ok())
            .all(|directive| directive.text() != "use strict")
        {
            Some(())
        } else {
            None
        }
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
                "Check the "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Strict_mode">"for more information regarding strict mode."</Hyperlink>
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
        let value = JsSyntaxToken::new_detached(JsSyntaxKind::JSX_STRING_LITERAL, value, [], []);
        let use_strict_diretcive = make::js_directive(value)
            .with_semicolon_token(make::token(T![;]))
            .build();
        let directives = make::js_directive_list(
            node.directives()
                .into_iter()
                .chain([use_strict_diretcive])
                .collect::<Vec<_>>(),
        );
        let new_node = node.clone().with_directives(directives);
        mutation.replace_node(node, new_node);
        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup!("Insert a top level"<Emphasis>"\"use strict\" "</Emphasis>".").to_owned(),
            mutation,
        ))
    }
}
