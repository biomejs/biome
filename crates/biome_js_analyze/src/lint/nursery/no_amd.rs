use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, JsCallExpression};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::no_amd::NoAmdOptions;

declare_lint_rule! {
    /// Disallow AMD `require()` and `define()` calls.
    ///
    /// AMD (Asynchronous Module Definition) is an older way to split code into modules. It needs a
    /// loader library, such as RequireJS, and declares the modules a file depends on as a list of
    /// names passed to `define()` or `require()`. Standard `import` and `export` statements work
    /// in all modern browsers and Node.js without an extra library, and editors and bundlers can
    /// understand them without running the code.
    ///
    /// This rule reports calls to `define()` and `require()` that receive exactly two arguments
    /// when the first one is an array, such as `define(["a"], function (a) {})`. It only reports
    /// calls at the top level of a file, outside of functions and blocks. Calls to a `define` or
    /// `require` function declared in your own code aren't reported, and neither are CommonJS
    /// calls such as `require("a")`.
    ///
    /// This rule is useful while moving a codebase from AMD to standard modules.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// define(["a", "b"], function (a, b) {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// require(["a", "b"], function (a, b) {});
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import a from "a";
    /// const b = require("b");
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow CommonJS `require()` and `module.exports` too, see [`noCommonJs`](https://biomejs.dev/linter/rules/no-common-js/).
    ///
    pub NoAmd {
        version: "next",
        name: "noAmd",
        language: "js",
        sources: &[RuleSource::EslintImport("no-amd").same()],
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for NoAmd {
    type Query = Semantic<JsCallExpression>;
    type State = AmdFunction;
    type Signals = Option<Self::State>;
    type Options = NoAmdOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let reference = call
            .callee()
            .ok()?
            .omit_parentheses()
            .as_js_identifier_expression()?
            .name()
            .ok()?;
        let function = match reference.value_token().ok()?.text_trimmed() {
            "define" => AmdFunction::Define,
            "require" => AmdFunction::Require,
            _ => return None,
        };
        let model = ctx.model();
        // A local binding named `define` or `require` isn't the AMD loader.
        if model.binding(&reference).is_some() || !model.scope(call.syntax()).is_global_scope() {
            return None;
        }
        let arguments = call.arguments().ok()?.args();
        if arguments.len() != 2 {
            return None;
        }
        let Some(Ok(AnyJsCallArgument::AnyJsExpression(dependencies))) = arguments.first() else {
            return None;
        };
        matches!(
            dependencies.omit_parentheses(),
            AnyJsExpression::JsArrayExpression(_)
        )
        .then_some(function)
    }

    fn diagnostic(ctx: &RuleContext<Self>, function: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().callee().ok()?.range(),
                markup! {
                    "This "<Emphasis>{function}</Emphasis>" call loads modules with AMD instead of "<Emphasis>"import"</Emphasis>" statements."
                },
            )
            .note(markup! {
                "AMD needs a loader library to run, while "<Emphasis>"import"</Emphasis>" and "<Emphasis>"export"</Emphasis>" statements work everywhere and can be understood by tools without running the code."
            })
            .note(markup! {
                "Load each module in the list with an "<Emphasis>"import"</Emphasis>" statement instead, such as "<Emphasis>"import a from \"a\""</Emphasis>"."
            }),
        )
    }
}

#[derive(Clone, Copy)]
pub enum AmdFunction {
    Define,
    Require,
}

impl biome_console::fmt::Display for AmdFunction {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Define => "define()",
            Self::Require => "require()",
        })
    }
}
