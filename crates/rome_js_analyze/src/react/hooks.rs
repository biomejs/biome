use crate::react::{is_react_call_api, ReactLibrary};
use std::collections::{HashMap, HashSet};

use biome_js_semantic::{Capture, Closure, ClosureExtensions, SemanticModel};
use biome_js_syntax::{
    binding_ext::AnyJsIdentifierBinding, static_value::StaticValue, AnyJsExpression,
    AnyJsMemberExpression, JsArrayBindingPattern, JsArrayBindingPatternElementList,
    JsArrowFunctionExpression, JsCallExpression, JsFunctionExpression, JsVariableDeclarator,
    TextRange,
};
use biome_rowan::AstNode;
use serde::{Deserialize, Serialize};

/// Return result of [react_hook_with_dependency].
pub(crate) struct ReactCallWithDependencyResult {
    pub(crate) function_name_range: TextRange,
    pub(crate) closure_node: Option<AnyJsExpression>,
    pub(crate) dependencies_node: Option<AnyJsExpression>,
}

pub enum AnyJsFunctionExpression {
    JsArrowFunctionExpression(JsArrowFunctionExpression),
    JsFunctionExpression(JsFunctionExpression),
}

impl AnyJsFunctionExpression {
    fn closure(&self, model: &SemanticModel) -> Closure {
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

impl ReactCallWithDependencyResult {
    /// Returns all [Reference] captured by the closure argument of the React hook.
    /// See [react_hook_with_dependency].
    pub fn all_captures(&self, model: &SemanticModel) -> impl Iterator<Item = Capture> {
        self.closure_node
            .as_ref()
            .and_then(|node| AnyJsFunctionExpression::try_from(node.clone()).ok())
            .map(|function_expression| {
                let closure = function_expression.closure(model);
                let range = *closure.closure_range();
                closure
                    .descendents()
                    .flat_map(|closure| closure.all_captures())
                    .filter(move |capture| capture.declaration_range().intersect(range).is_none())
            })
            .into_iter()
            .flatten()
    }

    /// Returns all dependencies of a React hook.
    /// See [react_hook_with_dependency]
    pub fn all_dependencies(&self) -> impl Iterator<Item = AnyJsExpression> {
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
    pub closure_index: Option<usize>,
    pub dependencies_index: Option<usize>,
}

impl From<(usize, usize)> for ReactHookConfiguration {
    fn from((closure, dependencies): (usize, usize)) -> Self {
        Self {
            closure_index: Some(closure),
            dependencies_index: Some(dependencies),
        }
    }
}

pub(crate) fn react_hook_configuration<'a>(
    call: &JsCallExpression,
    hooks: &'a HashMap<String, ReactHookConfiguration>,
) -> Option<&'a ReactHookConfiguration> {
    let name = call
        .callee()
        .ok()?
        .as_js_identifier_expression()?
        .name()
        .ok()?
        .value_token()
        .ok()?;
    let name = name.text_trimmed();

    hooks.get(name)
}

const HOOKS_WITH_DEPS_API: [&str; 6] = [
    "useEffect",
    "useLayoutEffect",
    "useInsertionEffect",
    "useCallback",
    "useMemo",
    "useImperativeHandle",
];

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
    hooks: &HashMap<String, ReactHookConfiguration>,
    model: &SemanticModel,
) -> Option<ReactCallWithDependencyResult> {
    let expression = call.callee().ok()?;
    let name = if let Some(identifier) = expression.as_js_reference_identifier() {
        Some(StaticValue::String(identifier.value_token().ok()?))
    } else if let Some(member_expr) = AnyJsMemberExpression::cast_ref(expression.syntax()) {
        Some(member_expr.member_name()?)
    } else {
        None
    }?;
    let function_name_range = name.range();
    let name = name.text();

    // check if the hooks api is imported from the react library
    if HOOKS_WITH_DEPS_API.contains(&name)
        && !is_react_call_api(expression, model, ReactLibrary::React, name)
    {
        return None;
    }

    let hook = hooks.get(name)?;
    let closure_index = hook.closure_index?;
    let dependencies_index = hook.dependencies_index?;

    let mut indices = [closure_index, dependencies_index];
    indices.sort();
    let [closure_node, dependencies_node] = call.get_arguments_by_index(indices);

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
    hook_name: String,
    /// Index of the position of the stable return, [None] if
    /// none returns are stable
    index: Option<usize>,
}

impl StableReactHookConfiguration {
    pub fn new(hook_name: &str, index: Option<usize>) -> Self {
        Self {
            hook_name: hook_name.into(),
            index,
        }
    }
}

/// Checks if the binding is bound to a stable React hook
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
pub fn is_binding_react_stable(
    binding: &AnyJsIdentifierBinding,
    stable_config: &HashSet<StableReactHookConfiguration>,
) -> bool {
    fn array_binding_declarator_index(
        binding: &AnyJsIdentifierBinding,
    ) -> Option<(JsVariableDeclarator, Option<usize>)> {
        let index = binding.syntax().index() / 2;
        let declarator = binding
            .parent::<JsArrayBindingPatternElementList>()?
            .parent::<JsArrayBindingPattern>()?
            .parent::<JsVariableDeclarator>()?;
        Some((declarator, Some(index)))
    }

    fn assignment_declarator(
        binding: &AnyJsIdentifierBinding,
    ) -> Option<(JsVariableDeclarator, Option<usize>)> {
        let declarator = binding.parent::<JsVariableDeclarator>()?;
        Some((declarator, None))
    }

    array_binding_declarator_index(binding)
        .or_else(|| assignment_declarator(binding))
        .and_then(|(declarator, index)| {
            let hook_name = declarator
                .initializer()?
                .expression()
                .ok()?
                .as_js_call_expression()?
                .callee()
                .ok()?
                .as_js_identifier_expression()?
                .name()
                .ok()?
                .value_token()
                .ok()?
                .token_text();

            let stable = StableReactHookConfiguration {
                hook_name: hook_name.to_string(),
                index,
            };

            Some(stable_config.contains(&stable))
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod test {
    use super::*;
    use biome_js_parser::JsParserOptions;
    use biome_js_syntax::JsFileSource;

    #[test]
    pub fn ok_react_stable_captures() {
        let r = biome_js_parser::parse(
            "const ref = useRef();",
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let node = r
            .syntax()
            .descendants()
            .filter(|x| x.text_trimmed() == "ref")
            .last()
            .unwrap();
        let set_name = AnyJsIdentifierBinding::cast(node).unwrap();

        let config = HashSet::from_iter([
            StableReactHookConfiguration::new("useRef", None),
            StableReactHookConfiguration::new("useState", Some(1)),
        ]);

        assert!(is_binding_react_stable(&set_name, &config));
    }
}
