use crate::services::vue::VueComponent;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsLiteralExpression, JsClassDeclaration,
    JsFunctionBody, JsFunctionDeclaration, JsLanguage, JsMethodObjectMember, JsReturnStatement,
    JsSyntaxNode,
};
use biome_rowan::{AstNode, NodeOrToken, SyntaxKindSet, TextRange, WalkEvent, declare_node_union};
use biome_rule_options::use_vue_valid_default_prop::UseVueValidDefaultPropOptions;
use biome_vue_semantic::{DefaultSource, Prop, PropType, TypeSet};

declare_lint_rule! {
    /// Require the default value of a Vue prop to match the type of the prop.
    ///
    /// When a prop declares its type, such as `type: Number`, its default value should have
    /// that type too. Otherwise, the component receives a value it doesn't expect whenever the
    /// parent doesn't pass the prop.
    ///
    /// Default values that are objects or arrays must be returned from a function, such as
    /// `() => []`. Vue calls this function for each instance of the component, so every
    /// instance gets its own copy. An object or array written directly would be shared by all
    /// instances, and changing it in one instance would change it in all of them.
    ///
    /// This rule checks default values given in the `props` option, in `defineProps()`, in
    /// `withDefaults()`, in a destructured `defineProps()`, and in `defineModel()`.
    ///
    /// A default value written in a destructuring pattern, such as
    /// `const { items = [] } = defineProps()`, is used as it is. Vue doesn't call it like a
    /// function, so it can be an object or an array, but it can't be a function that creates
    /// the value.
    ///
    /// The rule only checks props whose types are all built-in types such as `String`,
    /// `Number`, `Boolean`, `Array`, `Object`, or `Function`, and only default values whose
    /// type is clear from the code, such as numbers, strings, arrays, and objects.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   props: {
    ///     count: {
    ///       type: Number,
    ///       default: "zero",
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   props: {
    ///     items: {
    ///       type: Array,
    ///       default: [],
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// withDefaults(defineProps<{ label?: string }>(), {
    ///   label: () => 42,
    /// });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// const { tags = () => [] } = defineProps({ tags: Array });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// defineModel({ type: Object, default: { size: "small" } });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup>
    /// const { tags = [] } = defineProps({
    ///   count: { type: Number, default: 0 },
    ///   items: { type: Array, default: () => [] },
    ///   options: {
    ///     type: Object,
    ///     default() {
    ///       return { size: "small" };
    ///     },
    ///   },
    ///   format: { type: Function, default: (value) => String(value) },
    ///   tags: Array,
    /// });
    /// defineModel({ type: Object, default: () => ({ size: "small" }) });
    /// </script>
    /// ```
    ///
    pub UseVueValidDefaultProp {
        version: "next",
        name: "useVueValidDefaultProp",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("require-valid-default-prop").same()],
    }
}

impl Rule for UseVueValidDefaultProp {
    type Query = VueComponent;
    type State = InvalidDefault;
    type Signals = Box<[Self::State]>;
    type Options = UseVueValidDefaultPropOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let component = ctx.query();
        let root = ctx.root();
        let model = ctx.model();
        let mut signals = Vec::new();
        for prop in component.props().chain(component.models()) {
            let types = prop.types();
            // A prop that accepts a type the rule can't name may accept any value.
            if types.is_empty() || types.contains(PropType::Unknown) {
                continue;
            }
            for default in prop.defaults() {
                let Some(value) = AnyDefaultValue::from_range(root.syntax(), default.range())
                else {
                    continue;
                };
                for (range, problem) in check_default(&value, types, default.source(), model) {
                    signals.push(InvalidDefault {
                        prop: prop.clone(),
                        range,
                        source: default.source(),
                        problem,
                    });
                }
            }
        }
        signals.into_boxed_slice()
    }

    fn text_range(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<TextRange> {
        Some(state.range)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = state.prop.name();
        let types = state.prop.types();
        let accepted = AcceptedTypes(types);
        let diagnostic = match state.problem {
            Problem::WrongType => {
                let diagnostic = RuleDiagnostic::new(
                    rule_category!(),
                    state.range,
                    markup! {
                        "The default value of the prop "<Emphasis>{name}</Emphasis>" doesn't match the type of the prop."
                    },
                )
                .note(markup! {
                    "The prop accepts "<Emphasis>{accepted}</Emphasis>"."
                });
                if state.source != DefaultSource::Destructure
                    && types.intersects(PropType::Object | PropType::Array)
                {
                    diagnostic.note(markup! {
                        "A default object or array must be returned from a function, such as "<Emphasis>"() => []"</Emphasis>"."
                    })
                } else {
                    diagnostic
                }
            }
            Problem::WrongReturnType => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "The function returns a default value for the prop "<Emphasis>{name}</Emphasis>" that doesn't match the type of the prop."
                },
            )
            .note(markup! {
                "The prop accepts "<Emphasis>{accepted}</Emphasis>"."
            }),
            Problem::MissingFactory => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "The default value of the prop "<Emphasis>{name}</Emphasis>" isn't returned from a function."
                },
            )
            .note(markup! {
                "An object or array written directly as a default value is shared by every instance of the component. Changing it in one instance changes it in all of them."
            })
            .note(markup! {
                "Return the value from a function instead, such as "<Emphasis>"() => []"</Emphasis>" or "<Emphasis>"() => ({})"</Emphasis>"."
            }),
            Problem::UncalledFunction => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "The default value of the prop "<Emphasis>{name}</Emphasis>" is a function, but the prop doesn't accept functions."
                },
            )
            .note(markup! {
                "A default value in a destructuring pattern is used as it is. Vue doesn't call it to create the value."
            })
            .note(markup! {
                "Write a value of type "<Emphasis>{accepted}</Emphasis>" directly instead."
            }),
        };
        Some(diagnostic)
    }
}

pub struct InvalidDefault {
    prop: Prop,
    /// The range of the default value, or of a value its function returns.
    range: TextRange,
    source: DefaultSource,
    problem: Problem,
}

#[derive(Clone, Copy)]
pub enum Problem {
    /// The default value has a type the prop doesn't accept.
    WrongType,
    /// The function given as the default value returns a value of a type the prop doesn't
    /// accept.
    WrongReturnType,
    /// An object or array is given directly where Vue expects a function that returns it.
    MissingFactory,
    /// A function is given as a destructuring default to a prop that doesn't accept functions.
    /// Vue only calls the functions given to `default` and `withDefaults()`.
    UncalledFunction,
}

declare_node_union! {
    /// The syntax a default value can be written with: an expression, or a method such as
    /// `default() { return []; }`.
    pub AnyDefaultValue = AnyJsExpression | JsMethodObjectMember
}

impl AnyDefaultValue {
    /// Returns the default value whose range is exactly `range`.
    fn from_range(root: &JsSyntaxNode, range: TextRange) -> Option<Self> {
        if !root.text_range_with_trivia().contains_range(range) {
            return None;
        }
        let start = match root.covering_element(range) {
            NodeOrToken::Node(node) => node,
            NodeOrToken::Token(token) => token.parent()?,
        };
        start
            .ancestors()
            .skip_while(|node| node.text_trimmed_range() != range)
            .take_while(|node| node.text_trimmed_range() == range)
            .find_map(Self::cast)
    }
}

/// Returns the range and the problem of each part of a default value that doesn't match
/// `types`.
fn check_default(
    value: &AnyDefaultValue,
    types: TypeSet,
    source: DefaultSource,
    model: &SemanticModel,
) -> Vec<(TextRange, Problem)> {
    let (body, is_plain_function) = match value {
        AnyDefaultValue::JsMethodObjectMember(method) => (
            method.body().ok().map(AnyJsFunctionBody::JsFunctionBody),
            method.async_token().is_none() && method.star_token().is_none(),
        ),
        AnyDefaultValue::AnyJsExpression(expression) => match expression.inner_expression() {
            Some(AnyJsExpression::JsFunctionExpression(function)) => (
                function.body().ok().map(AnyJsFunctionBody::JsFunctionBody),
                function.async_token().is_none() && function.star_token().is_none(),
            ),
            Some(AnyJsExpression::JsArrowFunctionExpression(arrow)) => {
                (arrow.body().ok(), arrow.async_token().is_none())
            }
            Some(inner) => {
                let Some(ty) = value_type(&inner, model) else {
                    return Vec::new();
                };
                let problem = if !types.contains(ty) {
                    Problem::WrongType
                } else if source != DefaultSource::Destructure
                    && (ty == PropType::Object || ty == PropType::Array)
                {
                    Problem::MissingFactory
                } else {
                    return Vec::new();
                };
                return vec![(expression.range(), problem)];
            }
            None => return Vec::new(),
        },
    };

    if types.contains(PropType::Function) {
        return Vec::new();
    }
    if source == DefaultSource::Destructure {
        return vec![(value.range(), Problem::UncalledFunction)];
    }
    // The value an async function or a generator returns is not the default value.
    if !is_plain_function {
        return Vec::new();
    }
    let returned: Vec<_> = match body {
        Some(AnyJsFunctionBody::AnyJsExpression(expression)) => vec![expression],
        Some(AnyJsFunctionBody::JsFunctionBody(body)) => returned_values(&body),
        None => Vec::new(),
    };
    returned
        .into_iter()
        .filter(|expression| {
            value_type(expression, model).is_some_and(|ty| !types.contains(ty))
        })
        .map(|expression| (expression.range(), Problem::WrongReturnType))
        .collect()
}

/// Returns the type of the value `expression` evaluates to, when it can be read from the code.
fn value_type(expression: &AnyJsExpression, model: &SemanticModel) -> Option<PropType> {
    match expression.inner_expression()? {
        AnyJsExpression::AnyJsLiteralExpression(literal) => match literal {
            AnyJsLiteralExpression::JsBigintLiteralExpression(_) => Some(PropType::BigInt),
            AnyJsLiteralExpression::JsBooleanLiteralExpression(_) => Some(PropType::Boolean),
            AnyJsLiteralExpression::JsNullLiteralExpression(_) => None,
            AnyJsLiteralExpression::JsNumberLiteralExpression(_) => Some(PropType::Number),
            AnyJsLiteralExpression::JsRegexLiteralExpression(_) => Some(PropType::Object),
            AnyJsLiteralExpression::JsStringLiteralExpression(_) => Some(PropType::String),
        },
        AnyJsExpression::JsTemplateExpression(template) if template.tag().is_none() => {
            Some(PropType::String)
        }
        AnyJsExpression::JsArrayExpression(_) => Some(PropType::Array),
        AnyJsExpression::JsObjectExpression(_) => Some(PropType::Object),
        AnyJsExpression::JsFunctionExpression(_) | AnyJsExpression::JsArrowFunctionExpression(_) => {
            Some(PropType::Function)
        }
        // `Number("1")` and `Symbol()` create a value of the type they are named after, unless
        // the name refers to a variable of the file.
        AnyJsExpression::JsCallExpression(call) => {
            let callee = call.callee().ok()?.omit_parentheses();
            let reference = callee.as_js_identifier_expression()?.name().ok()?;
            if model.binding(&reference).is_some() {
                return None;
            }
            let name = reference.value_token().ok()?;
            let index = CONSTRUCTOR_TYPES
                .binary_search_by_key(&name.text_trimmed(), |(name, _)| name)
                .ok()?;
            Some(CONSTRUCTOR_TYPES[index].1)
        }
        _ => None,
    }
}

/// Returns the values of the `return` statements of `body`, without the ones of nested
/// functions and classes.
fn returned_values(body: &JsFunctionBody) -> Vec<AnyJsExpression> {
    let mut values = Vec::new();
    let mut preorder = body.syntax().preorder_of_kinds(RETURN_SEARCH_KINDS);
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if let Some(statement) = JsReturnStatement::cast_ref(&node) {
            values.extend(statement.argument());
        }
        preorder.skip_subtree();
    }
    values
}

/// Lists the types a prop accepts, such as `String or Number`.
struct AcceptedTypes(TypeSet);

impl biome_console::fmt::Display for AcceptedTypes {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        for (index, ty) in self.0.iter().enumerate() {
            if index > 0 {
                fmt.write_str(" or ")?;
            }
            fmt.write_str(match ty {
                PropType::Boolean => "Boolean",
                PropType::String => "String",
                PropType::Number => "Number",
                PropType::Array => "Array",
                PropType::Object => "Object",
                PropType::Function => "Function",
                PropType::Symbol => "Symbol",
                PropType::Date => "Date",
                PropType::BigInt => "BigInt",
                PropType::Unknown => "unknown",
            })?;
        }
        Ok(())
    }
}

/// The nodes [`returned_values`] stops at: `return` statements, and the nodes whose
/// `return` statements belong to another function. Every expression is included, because a
/// `return` statement can only appear inside one when the expression is a function or a
/// class.
const RETURN_SEARCH_KINDS: SyntaxKindSet<JsLanguage> = JsReturnStatement::KIND_SET
    .union(AnyJsExpression::KIND_SET)
    .union(JsFunctionDeclaration::KIND_SET)
    .union(JsClassDeclaration::KIND_SET);

/// The global functions that return a value of the type they are named after when called,
/// sorted by name.
const CONSTRUCTOR_TYPES: &[(&str, PropType)] = &[
    ("Array", PropType::Array),
    ("BigInt", PropType::BigInt),
    ("Boolean", PropType::Boolean),
    ("Function", PropType::Function),
    ("Number", PropType::Number),
    ("Object", PropType::Object),
    ("String", PropType::String),
    ("Symbol", PropType::Symbol),
];

#[cfg(test)]
mod tests {
    use super::CONSTRUCTOR_TYPES;

    #[test]
    fn constructor_types_are_sorted() {
        for pair in CONSTRUCTOR_TYPES.windows(2) {
            assert!(
                pair[0].0 < pair[1].0,
                "{} must come after {}",
                pair[0].0,
                pair[1].0
            );
        }
    }
}
