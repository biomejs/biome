use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsImportLike, AnyJsNamedImportSpecifier, JsImport,
    JsObjectBindingPatternShorthandProperty, JsStaticMemberExpression, JsVariableDeclarator,
    global_identifier,
};
use biome_rowan::AstNode;
use biome_rule_options::no_process_env::NoProcessEnvOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow the use of `process.env`.
    ///
    /// The `process.env` object in Node.js stores configuration settings. Using it directly throughout a project can cause problems:
    ///
    /// 1. It's harder to maintain
    /// 2. It can lead to conflicts in team development
    /// 3. It complicates deployment across multiple servers
    ///
    /// A better practice is to keep all settings in one configuration file and reference it throughout the project.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (process.env.NODE_ENV === 'development') {
    ///   // ...
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const config = require('./config');
    /// if (config.NODE_ENV === 'development') {
    ///   // ...
    /// }
    /// ```
    ///
    pub NoProcessEnv {
        version: "1.9.1",
        name: "noProcessEnv",
        language: "js",
        sources: &[RuleSource::EslintN("no-process-env").same()],
        recommended: false,
        severity: Severity::Information,
    }
}

impl Rule for NoProcessEnv {
    type Query = Semantic<JsStaticMemberExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoProcessEnvOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let object = node.object().ok()?;
        let member = node.member().ok()?.as_js_name()?.to_trimmed_text();

        let (reference, name) = global_identifier(&object.as_any_global_identifier_expression()?)?;

        if member.text() == "env" && name.text() == "process" {
            return match model.binding(&reference) {
                None => Some(()),
                Some(binding) => is_process_module_import(&binding).then_some(()),
            };
        }

        model
            .binding(&reference)
            .filter(is_env_from_process)
            .map(|_| ())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Don't use "<Emphasis>"process.env"</Emphasis>"."
                },
            )
            .note(markup! {
                "Use a centralized configuration file instead for better maintainability and deployment consistency."
            }),
        )
    }
}

fn is_process_module_import(binding: &biome_js_semantic::Binding) -> bool {
    const PROCESS_MODULE_NAMES: [&str; 2] = ["process", "node:process"];
    binding
        .syntax()
        .ancestors()
        .find_map(|ancestor| JsImport::cast(ancestor)?.source_text().ok())
        .is_some_and(|source| PROCESS_MODULE_NAMES.contains(&source.text()))
}

const PROCESS_MODULE_NAMES: [&str; 2] = ["process", "node:process"];

/// Returns `true` when `binding` is the `env` binding of the `process` (or
/// `node:process`) module, regardless of whether it is imported or required.
fn is_env_from_process(binding: &biome_js_semantic::Binding) -> bool {
    is_env_named_binding(binding) && is_from_process_module(binding)
}

/// Whether `binding` refers to the `env` export, e.g. `import { env }` (with an
/// optional alias) or `const { env } = ...`.
fn is_env_named_binding(binding: &biome_js_semantic::Binding) -> bool {
    binding.syntax().ancestors().skip(1).any(|n| {
        if let Some(specifier) = AnyJsNamedImportSpecifier::cast(n.clone()) {
            return specifier
                .imported_name()
                .is_some_and(|name| name.text_trimmed() == "env");
        }
        JsObjectBindingPatternShorthandProperty::cast(n)
            .and_then(|property| property.identifier().ok())
            .and_then(|id| id.as_js_identifier_binding()?.name_token().ok())
            .is_some_and(|name| name.text_trimmed() == "env")
    })
}

/// Whether `binding` originates from the `process`/`node:process` module,
/// either through an `import` statement, a `require(...)` call, or a dynamic
/// `import(...)` call.
fn is_from_process_module(binding: &biome_js_semantic::Binding) -> bool {
    binding.syntax().ancestors().skip(1).any(|n| {
        if let Some(import) = JsImport::cast(n.clone()) {
            return import
                .source_text()
                .ok()
                .is_some_and(|source| PROCESS_MODULE_NAMES.contains(&source.text()));
        }
        JsVariableDeclarator::cast(n)
            .and_then(|declarator| process_module_specifier(&declarator))
            .is_some_and(|source| PROCESS_MODULE_NAMES.contains(&source.text()))
    })
}

/// Returns the module specifier of a `require(...)` or dynamic `import(...)`
/// call used to initialize `declarator`, if any.
fn process_module_specifier(declarator: &JsVariableDeclarator) -> Option<biome_rowan::TokenText> {
    let expression = match declarator.initializer()?.expression().ok()? {
        AnyJsExpression::JsAwaitExpression(await_expression) => await_expression.argument().ok()?,
        expression => expression,
    };
    let import_like = match expression {
        AnyJsExpression::JsCallExpression(call) => AnyJsImportLike::JsCallExpression(call),
        AnyJsExpression::JsImportCallExpression(call) => {
            AnyJsImportLike::JsImportCallExpression(call)
        }
        _ => return None,
    };
    import_like.inner_string_text()
}
