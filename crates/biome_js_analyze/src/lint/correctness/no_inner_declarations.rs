use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsDeclaration, JsFileSource, JsStatementList, JsSyntaxKind};
use biome_rowan::AstNode;

use crate::services::control_flow::AnyJsControlFlowRoot;

declare_lint_rule! {
    /// Disallow `function` and `var` declarations that are accessible outside their block.
    ///
    /// A `var` is accessible in the whole body of the nearest root (function, module, script, static block).
    /// To avoid confusion, they should be declared to the nearest root.
    ///
    /// Prior to ES2015, `function` declarations were only allowed in the nearest root,
    /// though parsers sometimes erroneously accept them elsewhere.
    /// In ES2015, inside an _ES module_, a `function` declaration is always block-scoped.
    ///
    /// Note that `const` and `let` declarations are block-scoped,
    /// and therefore they are not affected by this rule.
    /// Moreover, `function` declarations in nested blocks are allowed inside _ES modules_.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```cjs,expect_diagnostic
    /// if (test) {
    ///     function f() {}
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (test) {
    ///     var x = 1;
    /// }
    /// ```
    ///
    /// ```cjs,expect_diagnostic
    /// function f() {
    ///     if (test) {
    ///         function g() {}
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f() {
    ///     if (test) {
    ///         var x = 1;
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // inside a module, function declarations are block-scoped and thus allowed.
    /// if (test) {
    ///     function f() {}
    /// }
    /// export {}
    /// ```
    ///
    /// ```js
    /// function f() { }
    /// ```
    ///
    /// ```js
    /// function f() {
    ///     function g() {}
    /// }
    /// ```
    ///
    /// ```js
    /// function f() {
    ///     var x = 1;
    /// }
    /// ```
    ///
    /// ```js
    /// function f() {
    ///     if (test) {
    ///         const g = function() {};
    ///     }
    /// }
    /// ```
    ///
    pub NoInnerDeclarations {
        version: "1.0.0",
        name: "noInnerDeclarations",
        language: "js",
        sources: &[RuleSource::Eslint("no-inner-declarations")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoInnerDeclarations {
    type Query = Ast<AnyJsDeclaration>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let decl = ctx.query();
        let parent = match decl {
            AnyJsDeclaration::TsDeclareFunctionDeclaration(x) => {
                if ctx.source_type::<JsFileSource>().is_module() {
                    // In strict mode (implied by esm), function declarations are block-scoped.
                    return None;
                }
                // ignore TsDeclareStatement
                x.syntax().parent()?.parent()?
            }
            AnyJsDeclaration::JsFunctionDeclaration(x) => {
                if ctx.source_type::<JsFileSource>().is_module() {
                    // In strict mode (implied by esm), function declarations are block-scoped.
                    return None;
                }
                x.syntax().parent()?
            }
            AnyJsDeclaration::JsVariableDeclaration(x) => {
                if !x.is_var() {
                    return None;
                }
                let mut parent = x.syntax().parent()?;
                while matches!(
                    parent.kind(),
                    JsSyntaxKind::JS_VARIABLE_STATEMENT
                        | JsSyntaxKind::JS_VARIABLE_DECLARATION_CLAUSE
                        | JsSyntaxKind::TS_DECLARE_STATEMENT
                ) {
                    parent = parent.parent()?;
                }
                parent
            }
            _ => {
                return None;
            }
        };
        if matches!(
            parent.kind(),
            JsSyntaxKind::JS_EXPORT
                | JsSyntaxKind::TS_EXPORT_DECLARE_CLAUSE
                | JsSyntaxKind::JS_MODULE_ITEM_LIST
        ) {
            return None;
        }
        if JsStatementList::can_cast(parent.kind())
            && matches!(
                parent.parent()?.kind(),
                JsSyntaxKind::JS_FUNCTION_BODY
                    | JsSyntaxKind::JS_SCRIPT
                    | JsSyntaxKind::JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER
            )
        {
            return None;
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let decl = ctx.query();
        let decl_type = match decl {
            AnyJsDeclaration::JsFunctionDeclaration(_) => "function",
            _ => "var",
        };
        let nearest_root = decl
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(AnyJsControlFlowRoot::cast)?;
        let nearest_root_type = match nearest_root {
            AnyJsControlFlowRoot::JsModule(_) => "module",
            AnyJsControlFlowRoot::JsScript(_) => "script",
            AnyJsControlFlowRoot::JsStaticInitializationBlockClassMember(_) => "static block",
            _ => "enclosing function",
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            decl.range(),
            markup! {
                "This "<Emphasis>{decl_type}</Emphasis>" should be declared at the root of the "<Emphasis>{nearest_root_type}</Emphasis>"."
            },
        ).note(markup! {
            "The "<Emphasis>{decl_type}</Emphasis>" is accessible in the whole body of the "<Emphasis>{nearest_root_type}</Emphasis>".\nTo avoid confusion, it should be declared at the root of the "<Emphasis>{nearest_root_type}</Emphasis>"."
        }))
    }
}
