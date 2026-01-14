use crate::react::{ReactLibrary, is_react_call_api};
use biome_js_semantic::{Capture, Closure, ClosureExtensions, SemanticModel};
use biome_js_syntax::JsSyntaxToken;
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, JsArrowFunctionExpression, JsCallExpression,
    JsFunctionExpression, TextRange, static_value::StaticValue,
};
use biome_rowan::AstNode;
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};

use biome_analyze::QueryMatch;
use biome_rule_options::use_exhaustive_dependencies::StableHookResult;

/// Return result of [react_hook_with_dependency].
#[derive(Debug)]
pub struct ReactCallWithDependencyResult {
    pub(crate) function_name_range: TextRange,
    pub(crate) closure_node: Option<AnyJsExpression>,
    pub(crate) dependencies_node: Option<AnyJsExpression>,
}

pub enum AnyJsFunctionExpression {
    JsArrowFunctionExpression(JsArrowFunctionExpression),
    JsFunctionExpression(JsFunctionExpression),
}

impl AnyJsFunctionExpression {
    pub fn closure(&self, model: &SemanticModel) -> Closure {
        match self {
            Self::JsArrowFunctionExpression(arrow_function) => arrow_function.closure(model),
            Self::JsFunctionExpression(function) => function.closure(model),
        }
    }
}

impl TryFrom<AnyJsExpression> for AnyJsFunctionExpression {
    type Error = ();

    fn try_from(expression: AnyJsExpression) -> Result<Self, Self::Error> {
        match expression {
            AnyJsExpression::JsArrowFunctionExpression(arrow_function) => {
                Ok(Self::JsArrowFunctionExpression(arrow_function))
            }
            AnyJsExpression::JsFunctionExpression(function) => {
                Ok(Self::JsFunctionExpression(function))
            }
            _ => Err(()),
        }
    }
}

/// Returns all captures in the given closure and nested closures which are defined within the
/// range of the specified closure.
pub fn all_captures_in_closure(closure: &Closure) -> impl Iterator<Item = Capture> + use<> {
    let range = closure.closure_range();
    closure
        .descendents()
        .flat_map(|closure| closure.all_captures())
        .filter(move |capture| {
            !range.contains(capture.declaration_range().start())
                && range.contains(capture.node().text_range().start())
        })
}

impl ReactCallWithDependencyResult {
    /// Returns all dependencies of a React hook.
    /// See [react_hook_with_dependency]
    pub fn all_dependencies(&self) -> impl Iterator<Item = AnyJsExpression> + use<> {
        self.dependencies_node
            .as_ref()
            .and_then(|x| Some(x.as_js_array_expression()?.elements().into_iter()))
            .into_iter()
            .flatten()
            .filter_map(|x| x.ok()?.as_any_js_expression().cloned())
    }
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ReactHookConfiguration {
    pub closure_index: u8,
    pub dependencies_index: u8,

    /// `true` if it's a built-in React hook. For built-in hooks, we verify
    /// whether they are imported from the React library. For user-defined
    /// hooks, we don't.
    pub builtin: bool,
}

impl From<(u8, u8, bool)> for ReactHookConfiguration {
    fn from((closure_index, dependencies_index, builtin): (u8, u8, bool)) -> Self {
        Self {
            closure_index,
            dependencies_index,
            builtin,
        }
    }
}

fn get_untrimmed_callee_name(call: &JsCallExpression) -> Option<JsSyntaxToken> {
    let callee = call.callee().ok()?;

    if let Some(identifier) = callee.as_js_identifier_expression() {
        return identifier.name().ok()?.value_token().ok();
    }

    if let Some(member_expression) = callee.as_js_static_member_expression() {
        return member_expression.member().ok()?.value_token().ok();
    }

    None
}

/// Checks whether the given function name belongs to a React hook, based on the
/// official convention for React hook naming: Hook names must start with `use`
/// followed by a capital letter.
///
/// Source: https://react.dev/learn/reusing-logic-with-custom-hooks#hook-names-always-start-with-use
pub(crate) fn is_react_hook_name(name: &str) -> bool {
    name.starts_with("use") && name.chars().nth(3).is_some_and(char::is_uppercase)
}

/// Checks whether the given call expression calls a React hook, based on the
/// official convention for React hook naming: Hook names must start with `use`
/// followed by a capital letter.
///
/// See [is_react_hook_name()].
pub(crate) fn is_react_hook_call(call: &JsCallExpression) -> bool {
    let Some(name) = get_untrimmed_callee_name(call) else {
        return false;
    };

    // HACK: Jest/Vitest have some functions that start with `use` and are not hooks
    if let Some(expr) = call
        .callee()
        .ok()
        .and_then(|callee| callee.as_js_static_member_expression().cloned())
        .and_then(|member| member.object().ok())
        .and_then(|object| object.as_js_identifier_expression().cloned())
        .and_then(|ident| ident.name().ok())
        .and_then(|name| name.value_token().ok())
    {
        let expr_trimmed = expr.text_trimmed();
        if expr_trimmed == "jest" || expr_trimmed == "vi" {
            return false;
        }
    }

    is_react_hook_name(name.text_trimmed())
}

/// Returns the [TextRange] of the hook name; the node of the
/// expression of the argument that correspond to the closure of
/// the hook; and the node of the dependency list of the hook.
///
/// Example:
/// ```js
/// useEffect(() => {}, []);
///                     ^^ <- dependencies_node
///           ^^^^^^^^ <- closure_node
/// ^^^^^^^^^ <- function_name_range
/// ```
///
/// This function will use the parameter "hooks" with the configuration
/// of all function that are considered hooks. See [ReactHookConfiguration].
pub(crate) fn react_hook_with_dependency(
    call: &JsCallExpression,
    hooks: &FxHashMap<Box<str>, ReactHookConfiguration>,
    model: &SemanticModel,
) -> Option<ReactCallWithDependencyResult> {
    let expression = call.callee().ok()?.omit_parentheses();
    let name = if let Some(identifier) = expression.as_js_reference_identifier() {
        Some(StaticValue::String(identifier.value_token().ok()?))
    } else if let Some(member_expr) = AnyJsMemberExpression::cast_ref(expression.syntax()) {
        Some(member_expr.member_name()?)
    } else {
        None
    }?;
    let function_name_range = name.range();
    let name = name.text();

    let hook = hooks.get(name)?;

    // check if the hooks api is imported from the react library
    if hook.builtin && !is_react_call_api(&expression, model, ReactLibrary::React, name) {
        return None;
    }

    let closure_index = hook.closure_index as usize;
    let dependencies_index = hook.dependencies_index as usize;

    // Sort the indices to avoid triggering a panic, but reverse them
    // afterwards to avoid breaking results
    let mut indices = [closure_index, dependencies_index];
    indices.sort_unstable();
    let args = call.arguments().ok()?.get_arguments_by_index(indices);
    let [mut closure_node, mut dependencies_node] = args;
    if closure_index > dependencies_index {
        (closure_node, dependencies_node) = (dependencies_node, closure_node);
    }

    Some(ReactCallWithDependencyResult {
        function_name_range,
        closure_node: closure_node.and_then(|x| x.as_any_js_expression().cloned()),
        dependencies_node: dependencies_node.and_then(|x| x.as_any_js_expression().cloned()),
    })
}

/// Specifies which, if any, of the returns of a React hook are stable.
/// See [is_binding_react_stable].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StableReactHookConfiguration {
    /// Name of the React hook
    pub(crate) hook_name: Box<str>,

    /// The kind of (stable) result returned by the hook.
    pub(crate) result: StableHookResult,

    /// `true` if it's a built-in React hook. For built-in hooks, we verify
    /// whether they are imported from the React library. For user-defined
    /// hooks, we don't.
    pub(crate) builtin: bool,
}

impl StableReactHookConfiguration {
    pub fn new(hook_name: &str, result: StableHookResult, builtin: bool) -> Self {
        Self {
            hook_name: hook_name.into(),
            result,
            builtin,
        }
    }
}

/// Represents a potentially stable React hook result member.
#[derive(Clone)]
pub enum ReactHookResultMember {
    Key(String),
    Index(u8),
}

/// Checks if the call expression is bound to a stable React hook
/// return value. Stable returns do not need to be specified
/// as dependencies.
///
/// Example:
/// ```js
/// let [name, setName] = useState("");
/// useEffect(() => {
///     // name is NOT stable, so it must be specified as dependency
///     console.log(name);
///     // setName IS stable, so it must not be specified as dependency
///     console.log(setName("a"));
/// }, [name]);
/// ```
pub fn is_react_hook_call_stable(
    call_expression: &JsCallExpression,
    member: Option<&ReactHookResultMember>,
    model: &SemanticModel,
    stable_config: &FxHashSet<StableReactHookConfiguration>,
) -> bool {
    let Ok(callee) = call_expression.callee() else {
        return false;
    };
    let Some(function_name) = callee.get_callee_member_name() else {
        return false;
    };
    let function_name = function_name.text_trimmed();
    stable_config.iter().any(|config| {
        if !config.builtin && config.hook_name.as_ref() != function_name {
            return false;
        }

        if config.builtin
            && !is_react_call_api(&callee, model, ReactLibrary::React, &config.hook_name)
        {
            return false;
        }

        match (&config.result, &member) {
            (StableHookResult::Identity, None) => true,
            (StableHookResult::Indices(indices), Some(ReactHookResultMember::Index(i))) => {
                indices.contains(i)
            }
            (StableHookResult::Keys(keys), Some(ReactHookResultMember::Key(k))) => keys.contains(k),
            _ => false,
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::react::hooks::is_react_hook_call;
    use biome_js_parser::JsParserOptions;
    use biome_js_semantic::{SemanticModelOptions, semantic_model};
    use biome_js_syntax::JsFileSource;

    #[test]
    fn test_is_react_hook_call() {
        {
            let r = biome_js_parser::parse(
                r#"useRef();"#,
                JsFileSource::js_module(),
                JsParserOptions::default(),
            );
            let node = r
                .syntax()
                .descendants()
                .filter(|x| x.text_trimmed() == "useRef()")
                .last()
                .unwrap();
            assert!(is_react_hook_call(&JsCallExpression::unwrap_cast(node)));
        }

        {
            let r = biome_js_parser::parse(
                r#"userCredentials();"#,
                JsFileSource::js_module(),
                JsParserOptions::default(),
            );
            let node = r
                .syntax()
                .descendants()
                .filter(|x| x.text_trimmed() == "userCredentials()")
                .last()
                .unwrap();
            let call = JsCallExpression::cast_ref(&node).unwrap();
            assert!(!is_react_hook_call(&call));
        }
    }

    mod is_react_hook_call_stable {
        use super::*;

        #[test]
        pub fn test_with_named_import() {
            let r = biome_js_parser::parse(
                r#"
                import { useRef } from "react";
                const ref = useRef();
            "#,
                JsFileSource::js_module(),
                JsParserOptions::default(),
            );

            let call_expression = r
                .syntax()
                .descendants()
                .find_map(JsCallExpression::cast)
                .unwrap();

            let config = FxHashSet::from_iter([
                StableReactHookConfiguration::new("useRef", StableHookResult::Identity, true),
                StableReactHookConfiguration::new(
                    "useState",
                    StableHookResult::Indices(vec![1]),
                    true,
                ),
            ]);

            assert!(is_react_hook_call_stable(
                &call_expression,
                None,
                &semantic_model(&r.ok().unwrap(), SemanticModelOptions::default()),
                &config
            ));
        }

        #[test]
        pub fn test_with_default_import() {
            let r = biome_js_parser::parse(
                r#"
                import * as React from "react";
                const ref = React.useRef();
            "#,
                JsFileSource::js_module(),
                JsParserOptions::default(),
            );
            let call_expression = r
                .syntax()
                .descendants()
                .find_map(JsCallExpression::cast)
                .unwrap();

            let config = FxHashSet::from_iter([
                StableReactHookConfiguration::new("useRef", StableHookResult::Identity, true),
                StableReactHookConfiguration::new(
                    "useState",
                    StableHookResult::Indices(vec![1]),
                    true,
                ),
            ]);

            assert!(is_react_hook_call_stable(
                &call_expression,
                None,
                &semantic_model(&r.ok().unwrap(), SemanticModelOptions::default()),
                &config
            ));
        }
    }
}
