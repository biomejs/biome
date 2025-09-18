use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsExpression, AnyJsFunction, AnyJsObjectMemberName,
    JsIdentifierBinding, JsObjectExpression, JsPropertyObjectMember,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};

declare_lint_rule! {
    /// Disallow usages that lose the reactivity of `props` passed to `setup` in Vue projects.
    ///
    /// Vue's Composition API requires that props passed to the `setup` function
    /// maintain reactivity. Destructuring props or using member expressions on props
    /// in the root scope of `setup` will cause the values to lose reactivity.
    ///
    /// This rule reports:
    /// - Direct destructuring of props in setup function parameters
    /// - Destructuring assignment of props in the root scope of setup (unless using `toRefs` or `toRef`)
    ///
    /// Note: destructuring is allowed inside nested functions, callbacks, and
    /// returned render functions where the reactive context is preserved.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// // Destructuring in setup parameters
    /// export default {
    ///   setup({ count }) {
    ///     // count is no longer reactive
    ///     return () => h('div', count)
    ///   }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // Destructuring in setup root scope
    /// export default {
    ///   setup(props) {
    ///     const { count } = props
    ///     // count is no longer reactive
    ///     return () => h('div', count)
    ///   }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // Also works with quoted property names
    /// export default {
    ///   "setup"(props) {
    ///     const { count } = props
    ///     // count is no longer reactive
    ///     return () => h('div', count)
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // Keep props reactive
    /// export default {
    ///   setup(props) {
    ///     watch(() => props.count, () => {
    ///       console.log(props.count)
    ///     })
    ///     return () => h('div', props.count)
    ///   }
    /// }
    /// ```
    ///
    /// ```js
    /// // Using toRefs maintains reactivity
    /// export default {
    ///   setup(props) {
    ///     const { count } = toRefs(props) // OK - count is a ref
    ///     watch(count, () => {
    ///       console.log(count.value)
    ///     })
    ///     return () => h('div', count.value)
    ///   }
    /// }
    /// ```
    ///
    /// ```js
    /// // Using toRef for individual properties
    /// export default {
    ///   setup(props) {
    ///     const count = toRef(props, 'count') // OK - count is a ref
    ///     return () => h('div', count.value)
    ///   }
    /// }
    /// ```
    ///
    /// ```js
    /// // Destructuring inside callbacks is OK
    /// export default {
    ///   setup(props) {
    ///     watch(() => props.count, () => {
    ///       const { count } = props // OK inside callback
    ///       console.log(count)
    ///     })
    ///     return () => {
    ///       const { count } = props // OK inside render function
    ///       return h('div', count)
    ///     }
    ///   }
    /// }
    /// ```
    ///
    pub NoVueSetupPropsReactivityLoss {
        version: "next",
        name: "noVueSetupPropsReactivityLoss",
        language: "js",
        domains: &[RuleDomain::Vue],
        recommended: false,
        sources: &[RuleSource::EslintVueJs("no-setup-props-reactivity-loss").inspired()],
    }
}

#[derive(Debug)]
pub enum Violation {
    ParameterDestructuring(TextRange),
    RootScopeDestructuring {
        destructuring_range: TextRange,
        props_param_range: TextRange,
    },
}

impl Rule for NoVueSetupPropsReactivityLoss {
    type Query = Semantic<AnyJsFunction>;
    type State = Violation;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let function = ctx.query();
        let model = ctx.model();

        if !is_vue_setup_function(function) {
            return vec![];
        }

        let mut violations = Vec::new();

        let first_param_binding = get_function_first_parameter(function);

        let Some(pattern) = first_param_binding else {
            return violations;
        };

        match pattern {
            AnyJsBindingPattern::JsObjectBindingPattern(obj_pattern) => {
                // Direct destructuring in parameters
                violations.push(Violation::ParameterDestructuring(obj_pattern.range()));
            }
            AnyJsBindingPattern::AnyJsBinding(binding) => {
                // Props is a regular parameter, check for destructuring in root scope
                if let Some(props_binding) = binding.as_js_identifier_binding() {
                    violations.extend(check_root_scope_destructuring(
                        function,
                        props_binding,
                        model,
                    ));
                }
            }
            _ => {}
        }

        violations
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state {
            Violation::ParameterDestructuring(range) => RuleDiagnostic::new(
                rule_category!(),
                *range,
                markup! {
                    "Destructuring props in the setup function parameters loses reactivity."
                },
            ),
            Violation::RootScopeDestructuring {
                destructuring_range,
                props_param_range,
            } => RuleDiagnostic::new(
                rule_category!(),
                *destructuring_range,
                markup! {
                    "Destructuring props in the root scope of setup loses reactivity."
                },
            )
            .detail(
                *props_param_range,
                markup! {
                    "The props parameter is defined here."
                },
            ),
        };

        Some(
            diagnostic
                .note(markup! {
                    "In Vue's Composition API, props must be accessed as properties to maintain reactivity."
                })
                .note(markup! {
                    "Use 'props.propertyName' or 'toRefs(props)' to maintain reactivity."
                }),
        )
    }
}

fn is_vue_setup_function(function: &AnyJsFunction) -> bool {
    let Some(property) = function
        .syntax()
        .parent()
        .and_then(JsPropertyObjectMember::cast)
    else {
        return false;
    };

    let is_setup = property
        .name()
        .ok()
        .as_ref()
        .and_then(get_object_member_name_text)
        .is_some_and(|name| name == "setup");

    if !is_setup {
        return false;
    }

    property
        .syntax()
        .ancestors()
        .find_map(JsObjectExpression::cast)
        .is_some_and(|object_expr| is_default_export(&object_expr).is_some())
}

fn is_default_export(object_expr: &JsObjectExpression) -> Option<()> {
    object_expr.syntax().parent().and_then(|parent| {
        parent
            .parent()
            .filter(|grandparent| {
                grandparent.kind()
                    == biome_js_syntax::JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE
            })
            .map(|_| ())
    })
}

fn get_function_first_parameter(func: &AnyJsFunction) -> Option<AnyJsBindingPattern> {
    match func {
        AnyJsFunction::JsFunctionDeclaration(decl) => {
            let params = decl.parameters().ok()?;
            let param = params.items().iter().next()?.ok()?;
            let formal_param = param.as_any_js_formal_parameter()?;
            formal_param.as_js_formal_parameter()?.binding().ok()
        }
        AnyJsFunction::JsFunctionExpression(expr) => {
            let params = expr.parameters().ok()?;
            let param = params.items().iter().next()?.ok()?;
            let formal_param = param.as_any_js_formal_parameter()?;
            formal_param.as_js_formal_parameter()?.binding().ok()
        }
        AnyJsFunction::JsArrowFunctionExpression(arrow) => {
            let params = arrow.parameters().ok()?;
            match params {
                // 圆括号参数：(a, b) => {}
                biome_js_syntax::AnyJsArrowFunctionParameters::JsParameters(js_params) => {
                    let param = js_params.items().iter().next()?.ok()?;
                    let formal_param = param.as_any_js_formal_parameter()?;
                    formal_param.as_js_formal_parameter()?.binding().ok()
                }
                // 单个参数：a => {}
                biome_js_syntax::AnyJsArrowFunctionParameters::AnyJsBinding(binding) => {
                    Some(AnyJsBindingPattern::AnyJsBinding(binding))
                }
            }
        }
        _ => None,
    }
}

fn check_root_scope_destructuring(
    setup_fn: &AnyJsFunction,
    props_binding: &JsIdentifierBinding,
    model: &SemanticModel,
) -> Vec<Violation> {
    let mut violations = Vec::new();

    // 1. 获取 props 的绑定信息
    let props_semantic_binding = model.as_binding(props_binding);

    // 2. 遍历所有对 props 的读引用
    for reference in props_semantic_binding.all_reads() {
        // 获取引用的语法节点
        if let Some(reference_node) = reference.syntax().parent()
            && let Some(identifier) = biome_js_syntax::JsReferenceIdentifier::cast(reference_node)
        {
            // 3. 检查引用是否在 setup 函数的根作用域内
            if !is_reference_in_root_scope_of_function(&identifier, setup_fn) {
                continue; // 跳过不在根作用域的引用
            }

            // 4. 检查这个引用是否是解构表达式的一部分
            if let Some(destructuring_info) = is_reference_in_destructuring(&identifier) {
                // 检查是否是通过 toRefs/toRef 等响应式 API 进行的解构
                if is_safe_reactive_destructuring(&destructuring_info) {
                    continue; // 安全的解构，跳过
                }

                violations.push(Violation::RootScopeDestructuring {
                    destructuring_range: destructuring_info.destructuring_range,
                    props_param_range: props_binding.range(),
                });
            }
        }
    }

    violations
}

/// Get the text content from an object member name
/// 从对象成员名称中提取文本内容，支持各种属性名格式
fn get_object_member_name_text(name: &AnyJsObjectMemberName) -> Option<String> {
    match name {
        AnyJsObjectMemberName::JsLiteralMemberName(literal_name) => {
            let value_token = literal_name.value().ok()?;
            let text = value_token.text_trimmed();

            // 处理字符串字面量，去掉引号
            if (text.starts_with('"') && text.ends_with('"') && text.len() >= 2)
                || (text.starts_with('\'') && text.ends_with('\'') && text.len() >= 2)
            {
                Some(text[1..text.len() - 1].to_string())
            } else {
                // 标识符或数字字面量
                Some(text.to_string())
            }
        }

        AnyJsObjectMemberName::JsComputedMemberName(computed_name) => {
            let expression = computed_name.expression().ok()?;

            // 处理计算属性中的字符串字面量：{ ["setup"]: ... }
            if let Some(string_literal) = expression
                .as_any_js_literal_expression()
                .and_then(|literal| literal.as_js_string_literal_expression())
            {
                let value_token = string_literal.value_token().ok()?;
                let text = value_token.text_trimmed();

                if text.len() >= 2
                    && ((text.starts_with('"') && text.ends_with('"'))
                        || (text.starts_with('\'') && text.ends_with('\'')))
                {
                    return Some(text[1..text.len() - 1].to_string());
                }
                Some(text.to_string())
            } else {
                // 动态计算属性无法静态确定
                None
            }
        }

        AnyJsObjectMemberName::JsMetavariable(_) => None,
    }
}

fn is_reference_in_root_scope_of_function(
    reference: &biome_js_syntax::JsReferenceIdentifier,
    function: &AnyJsFunction,
) -> bool {
    let reference_syntax = reference.syntax();
    let function_syntax = function.syntax();

    // 获取引用到函数之间的所有祖先节点
    let mut current = reference_syntax.parent();

    while let Some(node) = current {
        // 如果到达了目标函数，说明引用在该函数内
        if node == *function_syntax {
            return true;
        }

        // 如果遇到了其他函数节点，说明引用不在根作用域
        if is_function_like_node(&node) && node != *function_syntax {
            return false;
        }

        current = node.parent();
    }

    false
}

/// 检查节点是否是函数类节点
fn is_function_like_node(node: &biome_rowan::SyntaxNode<biome_js_syntax::JsLanguage>) -> bool {
    use biome_js_syntax::JsSyntaxKind;
    matches!(
        node.kind(),
        JsSyntaxKind::JS_FUNCTION_DECLARATION
            | JsSyntaxKind::JS_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_METHOD_CLASS_MEMBER
            | JsSyntaxKind::JS_METHOD_OBJECT_MEMBER
            | JsSyntaxKind::JS_GETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_GETTER_OBJECT_MEMBER
            | JsSyntaxKind::JS_SETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_SETTER_OBJECT_MEMBER
    )
}

/// 解构信息结构
#[derive(Debug)]
struct DestructuringInfo {
    /// 解构模式的范围
    destructuring_range: TextRange,
    /// 解构的初始化表达式（可能是 props, toRefs(props) 等）
    initializer: Option<AnyJsExpression>,
}

/// 检查引用是否是解构表达式的一部分
fn is_reference_in_destructuring(
    reference: &biome_js_syntax::JsReferenceIdentifier,
) -> Option<DestructuringInfo> {
    let reference_syntax = reference.syntax();

    // 向上遍历祖先节点，寻找解构模式
    let mut current = reference_syntax.parent();

    while let Some(node) = current {
        // 检查是否是变量声明器（const { a } = props 的情况）
        if let Some(declarator) = biome_js_syntax::JsVariableDeclarator::cast(node.clone())
            && let Ok(id) = declarator.id()
            && let Some(obj_pattern) = id.as_js_object_binding_pattern()
        {
            // 检查初始化器
            let initializer = declarator
                .initializer()
                .and_then(|init| init.expression().ok());

            return Some(DestructuringInfo {
                destructuring_range: obj_pattern.range(),
                initializer,
            });
        }

        // 检查是否是赋值表达式的解构（{ a } = props 的情况）
        if let Some(assignment) = biome_js_syntax::JsAssignmentExpression::cast(node.clone())
            && let Ok(left) = assignment.left()
            && let Some(obj_pattern) = left.as_js_object_assignment_pattern()
        {
            let initializer = assignment.right().ok();
            return Some(DestructuringInfo {
                destructuring_range: obj_pattern.range(),
                initializer,
            });
        }

        current = node.parent();
    }

    None
}

/// 检查是否是安全的响应式解构
/// 例如：const { count } = toRefs(props) 或 const { count } = toRef(props, 'count')
fn is_safe_reactive_destructuring(destructuring_info: &DestructuringInfo) -> bool {
    if let Some(initializer) = &destructuring_info.initializer {
        // 检查初始化器是否是 toRefs/toRef 等响应式 API 调用
        is_reactive_api_call(initializer)
    } else {
        false
    }
}

/// 检查表达式是否是响应式 API 调用
fn is_reactive_api_call(expr: &AnyJsExpression) -> bool {
    if let Some(call_expr) = expr.as_js_call_expression()
        && let Ok(callee) = call_expr.callee()
        && let Some(ident_expr) = callee.as_js_identifier_expression()
        && let Ok(name) = ident_expr.name()
        && let Ok(token) = name.value_token()
    {
        let text = token.text_trimmed();
        return matches!(text, "toRefs" | "toRef" | "reactive" | "ref");
    }
    false
}
