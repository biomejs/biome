use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, JsStaticMemberExpression, global_identifier};
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::no_undeclared_env_vars::NoUndeclaredEnvVarsOptions;

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
    /// When `MY_VAR` is not declared in `turbo.json` or the allowed list:
    ///
    /// ```js,ignore
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
                "The environment variable "<Emphasis>{env_var}</Emphasis>" is not listed as a dependency in turbo.json. Add this environment variable a task's 'env'/'passThroughEnv', or to 'globalEnv', 'globalPassThroughEnv', in your turbo.json(c) configuration to ensure correct caching behavior in Turborepo."
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

/// Default allowed environment variables that are commonly used.
/// Sorted alphabetically for binary search.
const DEFAULT_ALLOWED_ENV_VARS: &[&str] = &[
    "CI", "HOME", "NODE_ENV", "PATH", "PWD", "SHELL", "TZ", "USER",
];

/// Default allowed prefixes for framework and CI provider specific env vars.
/// These are checked using `starts_with` for better performance than regex.
const DEFAULT_ALLOWED_PREFIXES: &[&str] = &[
    // Frameworks
    "NEXT_PUBLIC_",
    "VITE_",
    "REACT_APP_",
    "VUE_APP_",
    "NUXT_",
    "GATSBY_",
    "EXPO_PUBLIC_",
    // CI Providers
    "VERCEL_",
];

/// Checks if an environment variable is allowed based on default values and patterns
fn is_env_var_allowed_by_defaults(env_var: &str) -> bool {
    // Check against default allowed env vars (sorted, so binary search is used)
    if DEFAULT_ALLOWED_ENV_VARS.binary_search(&env_var).is_ok() {
        return true;
    }

    // Check for exact match "VERCEL" (CI provider)
    if env_var == "VERCEL" {
        return true;
    }

    // Check against default allowed prefixes
    for prefix in DEFAULT_ALLOWED_PREFIXES {
        if env_var.starts_with(prefix) {
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
