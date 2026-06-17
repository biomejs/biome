use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsObjectBindingPatternMember, JsObjectBindingPattern,
};
use biome_rowan::AstNode;
use biome_rowan::TextRange;
use biome_rule_options::no_react_object_type_as_default_prop::NoReactObjectTypeAsDefaultPropOptions;
use crate::react::components::{AnyPotentialReactComponentDeclaration, ReactComponentInfo};

declare_lint_rule! {
    /// Disallow array, object, and function values as default props.
    ///
    /// In React, a default prop value like `{ items = [] }` is created every
    /// time the component renders. Arrays, objects, and functions are new values
    /// each time, even when they look the same. React then thinks the prop changed,
    /// so it may re-render the component more than needed, or re-run hooks like
    /// `useEffect` that depends on the prop.
    ///
    /// Numbers, strings, and other primitives are fine, because they stay the same
    /// between renders.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function Component({ items = [] }) {
    ///     return items;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const Component = ({ config = {} }) => config;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const EMPTY_ITEMS = [];
    ///
    /// function Component({ items = EMPTY_ITEMS }) {
    ///     return items;
    /// }
    /// ```
    ///
    /// ```js
    /// function Component({ count = 0, label = "default" }) {
    ///     return count;
    /// }
    /// ```
    ///
    pub NoReactObjectTypeAsDefaultProp {
        version: "next",
        name: "noReactObjectTypeAsDefaultProp",
        language: "js",
        sources: &[RuleSource::EslintReact("no-object-type-as-default-prop").same()],
        recommended: true,
        domains: &[RuleDomain::React],
        severity: Severity::Error,
    }
}

impl Rule for NoReactObjectTypeAsDefaultProp {
    type Query = Ast<AnyPotentialReactComponentDeclaration>;
    type State = ForbiddenDefault;
    type Signals = Vec<Self::State>;
    type Options = NoReactObjectTypeAsDefaultPropOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let Some(component) = ReactComponentInfo::from_declaration(node.syntax()) else {
            return vec![];
        };
        let Some(object_pattern) = component.props_object_pattern() else {
            return vec![];
        };
        collect_forbidden_defaults(&object_pattern)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let kind = &state.kind;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "A new "{kind}" is created here on every render."
                },
            )
            .note(markup! {
                "React sees this as a different value each render, so the component may re-render more than needed."
            })
            .note(markup! {
                "Move this value to a constant outside the component and use that as the default."
            }),
        )
    }
}

enum ForbiddenDefaultKind {
    ObjectLiteral,
    ArrayLiteral,
    ArrowFunction,
    FunctionExpression,
    ClassExpression,
    NewExpression,
    JsxElement,
    RegexLiteral,
    Symbol,
}

impl biome_console::fmt::Display for ForbiddenDefaultKind {
    fn fmt(&self, f: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        let repr = match self {
            Self::ObjectLiteral => "an object literal",
            Self::ArrayLiteral => "an array literal",
            Self::ArrowFunction => "an arrow function",
            Self::FunctionExpression => "a function expression",
            Self::ClassExpression => "a class expression",
            Self::NewExpression => "a newly constructed object",
            Self::JsxElement => "a JSX element",
            Self::RegexLiteral => "a regular expression literal",
            Self::Symbol => "a Symbol",
        };
        write!(f, "{repr}")
    }
}

pub struct ForbiddenDefault {
    range: TextRange,
    kind: ForbiddenDefaultKind,
}

fn forbidden_default_kind(expression: &AnyJsExpression) -> Option<ForbiddenDefaultKind> {
    let kind = match expression {
        AnyJsExpression::JsObjectExpression(_) => ForbiddenDefaultKind::ObjectLiteral,
        AnyJsExpression::JsArrayExpression(_) => ForbiddenDefaultKind::ArrayLiteral,
        AnyJsExpression::JsArrowFunctionExpression(_) => ForbiddenDefaultKind::ArrowFunction,
        AnyJsExpression::JsFunctionExpression(_) => ForbiddenDefaultKind::FunctionExpression,
        AnyJsExpression::JsClassExpression(_) => ForbiddenDefaultKind::ClassExpression,
        AnyJsExpression::JsNewExpression(_) => ForbiddenDefaultKind::NewExpression,
        AnyJsExpression::JsxTagExpression(_) => ForbiddenDefaultKind::JsxElement,
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsRegexLiteralExpression(_),
        ) => ForbiddenDefaultKind::RegexLiteral,
        _ => {
            let is_symbol_call = expression
                .as_js_call_expression()
                .and_then(|call| call.callee().ok())
                .and_then(|callee| callee.as_js_reference_identifier())
                .is_some_and(|ident| ident.has_name("Symbol"));
            if is_symbol_call {
                ForbiddenDefaultKind::Symbol
            } else {
                return None;
            }
        }
    };
    Some(kind)
}

fn collect_forbidden_defaults(object_pattern: &JsObjectBindingPattern) -> Vec<ForbiddenDefault> {
    object_pattern
        .properties()
        .into_iter()
        .filter_map(|property| {
            let AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(shorthand) =
                property.ok()?
            else {
                return None;
            };
            let default_value = shorthand.init()?.expression().ok()?;
            let kind = forbidden_default_kind(&default_value)?;
            Some(ForbiddenDefault {
                range: default_value.range(),
                kind,
            })
        })
        .collect()
}
