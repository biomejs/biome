use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_js_factory::make::{
    js_directive, js_directive_list, jsx_string_literal, jsx_string_literal_single_quotes,
};
use biome_js_syntax::{JsFileSource, JsScript};
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
        let file_source = ctx.source_type::<JsFileSource>();

        if node.directives().is_empty() && file_source.is_script() {
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
        let value = if ctx.as_preferred_quote().is_double() {
            jsx_string_literal("use strict")
        } else {
            jsx_string_literal_single_quotes("use strict")
        };
        let directives = js_directive_list(vec![js_directive(value).build()]);
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
