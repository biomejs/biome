use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsMemberExpression, JsUnaryExpression,
    JsUnaryOperator, global_identifier,
};
use biome_rowan::AstNode;
use biome_rule_options::no_dynamic_delete::NoDynamicDeleteOptions;

use crate::services::semantic::Semantic;
use crate::shared::node_process::is_process_module_import;

declare_lint_rule! {
    /// Disallow using the `delete` operator with a property name that is computed at runtime.
    ///
    /// The `delete` operator removes a property from an object. When the property name
    /// is written inside square brackets and comes from a variable, a function call, or
    /// another expression, the code is using a plain object to store a changing set of keys.
    /// It is hard to tell which property gets removed just by reading the code.
    ///
    /// A [`Map`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map)
    /// or a [`Set`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set)
    /// is built for this use. Both have a `delete()` method for removing entries.
    ///
    /// This rule allows square brackets that contain a string literal, a number literal,
    /// or a negative number literal, because the property name is fixed in the code.
    ///
    /// In Node.js, `process.env` holds the environment variables of the running program,
    /// and it can't be replaced with a `Map`. The rule still reports `delete process.env[name]`,
    /// and suggests deleting each environment variable by its name instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const name = "name";
    /// delete container[name];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// delete container[name.toUpperCase()];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const variable = "API_KEY";
    /// delete process.env[variable];
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// delete container.aaa;
    /// delete container[7];
    /// delete container[-1];
    /// delete container["aaa"];
    /// delete container["Infinity"];
    ///
    /// const map = new Map();
    /// map.delete(name);
    ///
    /// delete process.env.API_KEY;
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow the `delete` operator on fixed property names, see [`noDelete`](https://biomejs.dev/linter/rules/no-delete/).
    ///
    pub NoDynamicDelete {
        version: "2.6.5",
        name: "noDynamicDelete",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("no-dynamic-delete").same()],
        recommended: false,
    }
}

/// A `delete` whose property name is computed at runtime.
pub struct DynamicDelete {
    /// The expression inside the square brackets.
    key: AnyJsExpression,
    target: DeleteTarget,
}

/// The kind of object that `delete` removes a property from.
#[derive(Clone, Copy)]
pub enum DeleteTarget {
    Object,
    /// `process.env` from Node.js, which holds environment variables.
    ProcessEnv,
}

impl Rule for NoDynamicDelete {
    type Query = Semantic<JsUnaryExpression>;
    type State = DynamicDelete;
    type Signals = Option<Self::State>;
    type Options = NoDynamicDeleteOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.operator().ok()? != JsUnaryOperator::Delete {
            return None;
        }
        let argument = node.argument().ok()?.omit_parentheses();
        let member_expression = argument.as_js_computed_member_expression()?;
        let key = member_expression.member().ok()?.omit_parentheses();
        if is_fixed_key(&key) {
            return None;
        }
        let object = member_expression.object().ok()?;
        let target = if is_process_env(ctx.model(), object).unwrap_or_default() {
            DeleteTarget::ProcessEnv
        } else {
            DeleteTarget::Object
        };
        Some(DynamicDelete { key, target })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = state.key.range();
        let diagnostic = match state.target {
            DeleteTarget::Object => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Avoid using "<Emphasis>"delete"</Emphasis>" with a property name that is computed at runtime."
                },
            )
            .note(markup! {
                "Removing properties by a changing name makes it hard to tell which property gets removed."
            })
            .note(markup! {
                "Use a "<Emphasis>"Map"</Emphasis>" or a "<Emphasis>"Set"</Emphasis>" and call its "<Emphasis>"delete()"</Emphasis>" method instead."
            }),
            DeleteTarget::ProcessEnv => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Avoid using "<Emphasis>"delete"</Emphasis>" with an environment variable name that is computed at runtime."
                },
            )
            .note(markup! {
                "Removing environment variables by a changing name makes it hard to tell which variables get removed."
            })
            .note(markup! {
                "Delete each environment variable by its name instead, for example "<Emphasis>"delete process.env.API_KEY"</Emphasis>"."
            }),
        };
        Some(diagnostic)
    }
}

/// Returns whether `member` is a string literal, a number literal, or a number
/// literal preceded by `-`.
fn is_fixed_key(member: &AnyJsExpression) -> bool {
    match member {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(_)
            | AnyJsLiteralExpression::JsNumberLiteralExpression(_),
        ) => true,
        AnyJsExpression::JsUnaryExpression(unary) => {
            unary.operator() == Ok(JsUnaryOperator::Minus)
                && unary.argument().is_ok_and(|argument| {
                    matches!(
                        argument.omit_parentheses(),
                        AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                        )
                    )
                })
        }
        _ => false,
    }
}

/// Returns whether `object` is `process.env` or `process["env"]`, where `process`
/// is the Node.js global or is imported from the `process` or `node:process` module.
fn is_process_env(model: &SemanticModel, object: AnyJsExpression) -> Option<bool> {
    let object = AnyJsMemberExpression::cast(object.omit_parentheses().into_syntax())?;
    if object.member_name()?.text() != "env" {
        return Some(false);
    }
    let process = object.object().ok()?.omit_parentheses();
    let (reference, name) = global_identifier(&process.as_any_global_identifier_expression()?)?;
    if name.text() != "process" {
        return Some(false);
    }
    Some(match model.binding(&reference) {
        None => true,
        Some(binding) => is_process_module_import(&binding),
    })
}
