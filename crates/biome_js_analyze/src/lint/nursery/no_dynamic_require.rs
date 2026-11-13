use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, JsCallArguments, JsCallExpression, JsImportCallExpression,
};
use biome_rowan::{AstNode, SyntaxResult, declare_node_union};
use biome_rule_options::no_dynamic_require::NoDynamicRequireOptions;

declare_lint_rule! {
    /// Disallow `require()` calls whose module name isn't a fixed string.
    ///
    /// `require()` accepts any value as the module name, including names built while the
    /// program runs, such as `require("./locales/" + language)`. When the name is built this
    /// way, editors, bundlers, and other tools can't tell which file gets loaded. They can't
    /// check that the file exists, include it in a bundle, or find every place that uses it.
    ///
    /// This rule reports calls to `require()` whose first argument is anything other than a plain
    /// value written in the code, such as a string or a template literal without `${}`
    /// placeholders. Calls without arguments aren't reported. Calls to a `require` function
    /// declared in your own code aren't reported either.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// require(name);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// require("../" + name);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// require(`../${name}`);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// require(getName());
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// require("../name");
    /// require(`../name`);
    /// ```
    ///
    /// ## Options
    ///
    /// ### `checkDynamicImports`
    ///
    /// Default: `false`
    ///
    /// When `true`, the rule also reports `import()` calls whose module name isn't a fixed string.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "checkDynamicImports": true
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// import(`./locales/${language}.js`);
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// import("./locales/en.js");
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow `require()` entirely, see [`noCommonJs`](https://biomejs.dev/linter/rules/no-common-js/).
    ///
    pub NoDynamicRequire {
        version: "next",
        name: "noDynamicRequire",
        language: "js",
        sources: &[RuleSource::EslintImport("no-dynamic-require").same()],
        recommended: false,
        severity: Severity::Warning,
    }
}

declare_node_union! {
    pub AnyJsModuleLoadingCall = JsCallExpression | JsImportCallExpression
}

impl Rule for NoDynamicRequire {
    type Query = Semantic<AnyJsModuleLoadingCall>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoDynamicRequireOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        match node {
            AnyJsModuleLoadingCall::JsCallExpression(call) => {
                let callee = call.callee().ok()?;
                let reference = callee
                    .omit_parentheses()
                    .as_js_identifier_expression()?
                    .name()
                    .ok()?;
                if reference.value_token().ok()?.text_trimmed() != "require"
                    // A local binding named `require` isn't the module loader.
                    || ctx.model().binding(&reference).is_some()
                {
                    return None;
                }
            }
            AnyJsModuleLoadingCall::JsImportCallExpression(_) => {
                if !ctx.options().check_dynamic_imports() {
                    return None;
                }
            }
        }
        let argument = node.module_name_argument()?;
        (!is_static_module_name(&argument)).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let function_name = match node {
            AnyJsModuleLoadingCall::JsCallExpression(_) => "require()",
            AnyJsModuleLoadingCall::JsImportCallExpression(_) => "import()",
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.module_name_argument()?.range(),
                markup! {
                    "The module name passed to "<Emphasis>{function_name}</Emphasis>" isn't a fixed string."
                },
            )
            .note(markup! {
                "When the module name is built while the program runs, tools can't tell which file gets loaded, so they can't check that it exists or find where it's used."
            })
            .note(markup! {
                "Write the module name as a string, such as "<Emphasis>"\"./module\""</Emphasis>"."
            }),
        )
    }
}

impl AnyJsModuleLoadingCall {
    fn arguments(&self) -> SyntaxResult<JsCallArguments> {
        match self {
            Self::JsCallExpression(call) => call.arguments(),
            Self::JsImportCallExpression(call) => call.arguments(),
        }
    }

    /// Returns the first argument of the call, which holds the module name.
    fn module_name_argument(&self) -> Option<AnyJsCallArgument> {
        let [argument] = self.arguments().ok()?.get_arguments_by_index([0]);
        argument
    }
}

/// Returns `true` if the argument is a literal, or a template literal without
/// a tag and without `${}` placeholders.
fn is_static_module_name(argument: &AnyJsCallArgument) -> bool {
    match argument.as_any_js_expression().cloned().map(AnyJsExpression::omit_parentheses) {
        Some(AnyJsExpression::AnyJsLiteralExpression(_)) => true,
        Some(AnyJsExpression::JsTemplateExpression(template)) => template.is_constant(),
        _ => false,
    }
}
