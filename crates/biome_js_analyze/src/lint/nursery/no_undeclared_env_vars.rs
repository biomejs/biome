use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, JsCallExpression, JsComputedMemberExpression,
    JsStaticMemberExpression, global_identifier, inner_string_text,
};
use biome_rowan::{AstNode, AstSeparatedList, TokenText};
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
    /// This rule checks for environment variable accesses in the following patterns:
    /// - `process.env.VAR_NAME` and `process.env["VAR_NAME"]`
    /// - `import.meta.env.VAR_NAME` and `import.meta.env["VAR_NAME"]`
    /// - `Bun.env.VAR_NAME` and `Bun.env["VAR_NAME"]`
    /// - `Deno.env.get("VAR_NAME")`
    ///
    /// It validates them against:
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
        version: "2.3.10",
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
    type Query = Turborepo<AnyJsMemberExpression>;
    type State = EnvVarAccess;
    type Signals = Option<Self::State>;
    type Options = NoUndeclaredEnvVarsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        // Skip the rule if no turbo.json was found - this rule only applies to Turborepo projects
        if !ctx.has_turborepo_config() {
            return None;
        }

        let member_expr = ctx.query();
        let options = ctx.options();
        let model = ctx.model();

        // Check if this is a Deno.env.get() call
        if let Some(deno_result) = match_deno_env_get(member_expr, model) {
            return match deno_result {
                Some(env_var_name) => check_env_var(ctx, env_var_name, options),
                // Matched Deno.env.get(...) but couldn't statically resolve the key (e.g. Deno.env.get(key))
                // so skip it.
                None => None,
            };
        }

        // Get the env var name and the parent object based on the expression type
        let (env_var_name, parent_object) = match member_expr {
            AnyJsMemberExpression::JsStaticMemberExpression(static_expr) => {
                extract_from_static_member(static_expr)?
            }
            AnyJsMemberExpression::JsComputedMemberExpression(computed_expr) => {
                extract_from_computed_member(computed_expr)?
            }
        };

        // Check if the parent object is a valid env access (process.env, import.meta.env, or Bun.env)
        if !is_global_env_object(&parent_object, model) && !is_import_meta_object(&parent_object) {
            return None;
        }

        check_env_var(ctx, env_var_name, options)
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

/// Extracts the env var name and parent object from a static member expression (e.g., `process.env.MY_VAR`)
fn extract_from_static_member(
    static_expr: &JsStaticMemberExpression,
) -> Option<(TokenText, AnyJsExpression)> {
    // Check if this is process.env.SOMETHING, import.meta.env.SOMETHING, or Bun.env.SOMETHING
    let object = static_expr.object().ok()?;
    let parent_member = object.as_js_static_member_expression()?;
    let env_member = parent_member.member().ok()?;

    // Must be accessing ".env"
    if env_member.as_js_name()?.to_trimmed_text().text() != "env" {
        return None;
    }

    let parent_object = parent_member.object().ok()?;

    // Get the env var name being accessed (e.g., NODE_ENV from process.env.NODE_ENV)
    let member = static_expr.member().ok()?;
    let env_var_name = member.as_js_name()?.value_token().ok()?;
    let env_var_text = env_var_name.token_text_trimmed();

    Some((env_var_text, parent_object))
}

/// Extracts the env var name and parent object from a computed member expression (e.g., `process.env["MY_VAR"]`)
/// Only handles string literal keys; dynamic keys like `process.env[key]` are skipped
fn extract_from_computed_member(
    computed_expr: &JsComputedMemberExpression,
) -> Option<(TokenText, AnyJsExpression)> {
    // Check if this is process.env["SOMETHING"], import.meta.env["SOMETHING"], or Bun.env["SOMETHING"]
    let object = computed_expr.object().ok()?;
    let parent_member = object.as_js_static_member_expression()?;
    let env_member = parent_member.member().ok()?;

    // Must be accessing ".env"
    if env_member.as_js_name()?.to_trimmed_text().text() != "env" {
        return None;
    }

    let parent_object = parent_member.object().ok()?;

    // Get the env var name from the computed member
    // Only process string literals, skip dynamic accesses like process.env[key]
    let member = computed_expr.member().ok()?;
    let member = member.omit_parentheses();

    // Check if it's a string literal
    let string_literal = member
        .as_any_js_literal_expression()?
        .as_js_string_literal_expression()?;
    let string_token = string_literal.value_token().ok()?;
    let env_var_name = inner_string_text(&string_token);

    Some((env_var_name, parent_object))
}

/// Checks if the object is a global env object (`process` or `Bun`, not shadowed by a local binding)
fn is_global_env_object(expr: &AnyJsExpression, model: &biome_js_semantic::SemanticModel) -> bool {
    let Some((reference, name)) = global_identifier(expr) else {
        return false;
    };

    // Check that it's named "process" or "Bun" and not bound to a local variable
    let name_text = name.text();
    (name_text == "process" || name_text == "Bun") && model.binding(&reference).is_none()
}

fn match_deno_env_get(
    member_expr: &AnyJsMemberExpression,
    model: &biome_js_semantic::SemanticModel,
) -> Option<Option<TokenText>> {
    let AnyJsMemberExpression::JsStaticMemberExpression(static_expr) = member_expr else {
        return None;
    };

    let get_member = static_expr.member().ok()?;
    if get_member.as_js_name()?.to_trimmed_text().text() != "get" {
        return None;
    }

    let object = static_expr.object().ok()?.omit_parentheses();
    let env_member = object.as_js_static_member_expression()?;
    if env_member
        .member()
        .ok()?
        .as_js_name()?
        .to_trimmed_text()
        .text()
        != "env"
    {
        return None;
    }

    let deno_object = env_member.object().ok()?.omit_parentheses();
    let Some((reference, name)) = global_identifier(&deno_object) else {
        return Some(None);
    };
    if name.text() != "Deno" || model.binding(&reference).is_some() {
        return Some(None);
    }

    let Some(parent) = member_expr.syntax().parent() else {
        return Some(None);
    };
    let Some(call) = JsCallExpression::cast(parent) else {
        return Some(None);
    };
    if call.is_optional() {
        return Some(None);
    }

    let first_arg = call.arguments().ok()?.args().first()?.ok()?;
    let first_arg = first_arg.as_any_js_expression()?.clone().omit_parentheses();
    let string_literal = first_arg
        .as_any_js_literal_expression()?
        .as_js_string_literal_expression()?;
    let string_token = string_literal.value_token().ok()?;

    Some(Some(inner_string_text(&string_token)))
}

fn check_env_var(
    ctx: &RuleContext<NoUndeclaredEnvVars>,
    env_var_name: TokenText,
    options: &NoUndeclaredEnvVarsOptions,
) -> Option<EnvVarAccess> {
    let env_var = env_var_name.text();
    if is_env_var_allowed_by_defaults(env_var) || is_env_var_allowed_by_options(env_var, options) {
        return None;
    }

    if ctx.is_env_var_declared(env_var) {
        return None;
    }

    Some(EnvVarAccess { env_var_name })
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
