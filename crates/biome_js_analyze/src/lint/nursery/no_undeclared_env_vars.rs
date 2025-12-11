use std::sync::LazyLock;

use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, JsStaticMemberExpression, global_identifier};
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::no_undeclared_env_vars::NoUndeclaredEnvVarsOptions;
use regex::Regex;

use crate::services::turborepo::Turborepo;

declare_lint_rule! {
    /// Disallow the use of undeclared environment variables.
    ///
    /// In Turborepo projects, environment variables used in tasks must be declared
    /// in the `turbo.json(c)` configuration file to ensure proper caching behavior.
    /// Using undeclared environment variables can lead to incorrect cache hits
    /// and unpredictable build behavior.
    ///
    /// This rule checks for `process.env.VAR_NAME` and `import.meta.env.VAR_NAME`
    /// accesses and validates them against:
    /// 1. Environment variables declared in `turbo.json(c)` (`globalEnv`, `globalPassThroughEnv`, task-level `env`, and task-level `passThroughEnv`)
    /// 2. Environment variables specified in the rule's `allowedEnvVars` option
    /// 3. Default allowed variables (common system vars and framework-specific patterns)
    ///
    /// ## Default Allowed Variables
    ///
    /// The following environment variables are always allowed without explicit declaration:
    ///
    /// **System variables:**
    /// - `CI`, `HOME`, `PATH`, `PWD`, `SHELL`, `TZ`, `USER`
    ///
    /// **Node.js:**
    /// - `NODE_ENV`
    ///
    /// **Framework and provider-specific patterns (all variables matching these prefixes):**
    /// - `NEXT_PUBLIC_*` (Next.js)
    /// - `VITE_*` (Vite)
    /// - `REACT_APP_*` (Create React App)
    /// - `VUE_APP_*` (Vue CLI)
    /// - `NUXT_*` (Nuxt)
    /// - `GATSBY_*` (Gatsby)
    /// - `EXPO_PUBLIC_*` (Expo)
    /// - `VERCEL`, `VERCEL_*` (Vercel)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// // When MY_VAR is not declared in turbo.json or allowed list
    /// const value = process.env.MY_VAR;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // NODE_ENV is always allowed
    /// const value = process.env.NODE_ENV;
    /// ```
    ///
    /// ## Options
    ///
    /// Use the options to specify additional environment variables that are not declared in `globalEnv`,
    /// `globalPassThroughEnv`, or task-level `env`/`passThroughEnv` in `turbo.json`.
    /// Supports regular expression patterns (anchors `^` and `$` are implicit).
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allowedEnvVars": ["MY_APP_.*", "ACME_TOKEN"]
    ///     }
    /// }
    /// ```
    ///
    pub NoUndeclaredEnvVars {
        version: "next",
        name: "noUndeclaredEnvVars",
        language: "js",
        sources: &[RuleSource::EslintTurbo("no-undeclared-env-vars").same()],
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Turborepo],
    }
}

/// State that holds the environment variable name being accessed
pub struct EnvVarAccess {
    /// The name of the environment variable
    env_var_name: TokenText,
}

impl Rule for NoUndeclaredEnvVars {
    type Query = Turborepo<JsStaticMemberExpression>;
    type State = EnvVarAccess;
    type Signals = Option<Self::State>;
    type Options = NoUndeclaredEnvVarsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        // Skip the rule if no turbo.json was found - this rule only applies to Turborepo projects
        if !ctx.has_turborepo_config() {
            return None;
        }

        let static_member_expr = ctx.query();
        let options = ctx.options();
        let model = ctx.model();

        // Check if this is either process.env.SOMETHING or import.meta.env.SOMETHING
        let object = static_member_expr.object().ok()?;
        let parent_member = object.as_js_static_member_expression()?;
        let env_member = parent_member.member().ok()?;

        // Must be accessing ".env"
        if env_member.as_js_name()?.to_trimmed_text().text() != "env" {
            return None;
        }

        let parent_object = parent_member.object().ok()?;

        let is_process_env = is_global_process_object(&parent_object, model);
        let is_import_meta_env = is_import_meta_object(&parent_object);

        if !is_process_env && !is_import_meta_env {
            return None;
        }

        // Get the env var name being accessed (e.g., NODE_ENV from process.env.NODE_ENV)
        let member = static_member_expr.member().ok()?;
        let env_var_name = member.as_js_name()?.value_token().ok()?;
        let env_var_text = env_var_name.token_text_trimmed();
        let env_var = env_var_text.text();

        // Check if this env var is allowed by default patterns or user options
        if is_env_var_allowed_by_defaults(env_var)
            || is_env_var_allowed_by_options(env_var, options)
        {
            return None;
        }

        // Check if declared in turbo.json
        if ctx.is_env_var_declared(env_var) {
            return None;
        }

        Some(EnvVarAccess {
            env_var_name: env_var_text,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let env_var = state.env_var_name.text();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "The environment variable "<Emphasis>{env_var}</Emphasis>" is not listed as a dependency in turbo.json. Add this environment variable to 'globalEnv', 'globalPassThroughEnv', or a task's 'env'/'passThroughEnv' array in your turbo.json(c) configuration to ensure correct caching behavior in Turborepo."
            },
        ))
    }
}

/// Checks if the object is the global `process` identifier (not shadowed by a local binding)
fn is_global_process_object(
    expr: &AnyJsExpression,
    model: &biome_js_semantic::SemanticModel,
) -> bool {
    let Some((reference, name)) = global_identifier(expr) else {
        return false;
    };

    // Check that it's named "process" and not bound to a local variable
    name.text() == "process" && model.binding(&reference).is_none()
}

/// Checks if the object is `import.meta`
fn is_import_meta_object(expr: &AnyJsExpression) -> bool {
    expr.as_js_import_meta_expression().is_some()
}

/// Default allowed environment variables that are commonly used
const DEFAULT_ALLOWED_ENV_VARS: &[&str] = &[
    "NODE_ENV", "CI", "TZ", "PATH", "HOME", "USER", "SHELL", "PWD",
];

/// Pre-compiled regex patterns for framework and CI provider specific env vars (e.g., NEXT_PUBLIC_*, VITE_*, etc.)
/// Using LazyLock ensures these are compiled only once and reused across all rule invocations.
static NEXT_PUBLIC_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^NEXT_PUBLIC_.*$").unwrap());
static VITE_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^VITE_.*$").unwrap());
static REACT_APP_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^REACT_APP_.*$").unwrap());
static VUE_APP_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^VUE_APP_.*$").unwrap());
static NUXT_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^NUXT_.*$").unwrap());
static GATSBY_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^GATSBY_.*$").unwrap());
static EXPO_PUBLIC_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^EXPO_PUBLIC_.*$").unwrap());
static VERCEL_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^VERCEL(_.*)?$").unwrap());

/// All default allowed patterns combined for iteration
static DEFAULT_ALLOWED_PATTERNS: &[&LazyLock<Regex>] = &[
    // Frameworks
    &NEXT_PUBLIC_PATTERN,
    &VITE_PATTERN,
    &REACT_APP_PATTERN,
    &VUE_APP_PATTERN,
    &NUXT_PATTERN,
    &GATSBY_PATTERN,
    &EXPO_PUBLIC_PATTERN,
    // CI Providers
    &VERCEL_PATTERN,
];

/// Checks if an environment variable is allowed based on default values and patterns
fn is_env_var_allowed_by_defaults(env_var: &str) -> bool {
    // Check against default allowed env vars
    if DEFAULT_ALLOWED_ENV_VARS.contains(&env_var) {
        return true;
    }

    // Check against pre-compiled default patterns
    for pattern in DEFAULT_ALLOWED_PATTERNS {
        if pattern.is_match(env_var) {
            return true;
        }
    }

    false
}

/// Checks if an environment variable is allowed based on user-specified options
fn is_env_var_allowed_by_options(env_var: &str, options: &NoUndeclaredEnvVarsOptions) -> bool {
    if let Some(allowed) = &options.allowed_env_vars {
        for pattern in allowed.iter() {
            if pattern.is_match(env_var) {
                return true;
            }
        }
    }

    false
}
