use std::sync::Arc;

use camino::Utf8PathBuf;

use crate::react::components::{AnyPotentialReactComponentDeclaration, ReactComponentInfo};
use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, AnyJsTemplateElement, AnyJsxAttributeValue,
    JsxAttribute,
};
use biome_package::PackageJson;
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::no_react_string_refs::NoReactStringRefsOptions;

declare_lint_rule! {
    /// Disallow string refs in React components.
    ///
    /// String refs are a legacy React feature. Modern React code should use callback refs,
    /// `createRef()`, or `useRef()` instead.
    ///
    /// Biome also flags template literal refs, even though upstream only does so through an option.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// function Hello() {
    ///   return <div ref="hello">Hello</div>;
    /// }
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// function Hello({ id }) {
    ///   return <div ref={`hello-${id}`}>Hello</div>;
    /// }
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// class Hello extends React.Component {
    ///   componentDidMount() {
    ///     this.refs.hello.focus();
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// function Hello() {
    ///   const helloRef = useRef(null);
    ///   return <div ref={helloRef}>Hello</div>;
    /// }
    /// ```
    ///
    pub NoReactStringRefs {
        version: "2.4.14",
        name: "noReactStringRefs",
        language: "js",
        sources: &[RuleSource::EslintReact("no-string-refs").same()],
        domains: &[RuleDomain::React],
        recommended: true,
        severity: Severity::Warning,
    }
}

declare_node_union! {
    pub AnyNoReactStringRefsQuery = JsxAttribute | AnyJsMemberExpression
}

pub enum NoReactStringRefsState {
    RefAttribute(TextRange),
    ThisRefs(TextRange),
}

impl Rule for NoReactStringRefs {
    type Query = Semantic<AnyNoReactStringRefsQuery>;
    type State = NoReactStringRefsState;
    type Signals = Option<Self::State>;
    type Options = NoReactStringRefsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyNoReactStringRefsQuery::JsxAttribute(attribute) => {
                let name = attribute.name().ok()?;
                let name = name.as_jsx_name()?;
                if name.value_token().ok()?.text_trimmed() != "ref" {
                    return None;
                }

                let value = attribute.initializer()?.value().ok()?;
                is_string_ref_value(&value)
                    .then_some(NoReactStringRefsState::RefAttribute(value.range()))
            }
            AnyNoReactStringRefsQuery::AnyJsMemberExpression(member_expression) => {
                if is_react_18_3_or_higher(ctx) {
                    return None;
                }

                let refs_range = this_refs_range(member_expression)?;

                member_expression
                    .syntax()
                    .ancestors()
                    .filter_map(AnyPotentialReactComponentDeclaration::cast)
                    .find_map(|declaration| {
                        ReactComponentInfo::from_declaration(declaration.syntax())
                    })
                    .map(|_| NoReactStringRefsState::ThisRefs(refs_range))
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (range, message, note, help) = match state {
            NoReactStringRefsState::RefAttribute(range) => (
                *range,
                markup! { "String refs are deprecated." },
                markup! {
                    "String refs are a legacy React feature that can make ref usage harder to follow."
                },
                markup! {
                    "Use a callback ref or an object ref created with "<Emphasis>"createRef()"</Emphasis>" or "<Emphasis>"useRef()"</Emphasis>" instead."
                },
            ),
            NoReactStringRefsState::ThisRefs(range) => (
                *range,
                markup! { "Using "<Emphasis>"this.refs"</Emphasis>" is deprecated." },
                markup! {
                    "Accessing refs through "<Emphasis>"this.refs"</Emphasis>" relies on React's legacy string ref behavior."
                },
                markup! {
                    "Store the ref on an instance field with a callback ref, or switch to "<Emphasis>"createRef()"</Emphasis>" or "<Emphasis>"useRef()"</Emphasis>"."
                },
            ),
        };

        Some(
            RuleDiagnostic::new(rule_category!(), range, message)
                .note(note)
                .note(help),
        )
    }
}

/// Check if the attribute value is a string ref, which can be either a string literal or a template literal with only string chunks.
fn is_string_ref_value(value: &AnyJsxAttributeValue) -> bool {
    match value {
        AnyJsxAttributeValue::AnyJsxTag(_) => false,
        AnyJsxAttributeValue::JsxString(_) => true,
        AnyJsxAttributeValue::JsxExpressionAttributeValue(expression) => {
            match expression.expression().ok() {
                Some(AnyJsExpression::AnyJsLiteralExpression(literal)) => {
                    literal.as_js_string_literal_expression().is_some()
                }
                Some(AnyJsExpression::JsTemplateExpression(template)) => {
                    template.elements().into_iter().any(|element| {
                        matches!(
                            element,
                            AnyJsTemplateElement::JsTemplateChunkElement(_)
                                | AnyJsTemplateElement::JsTemplateElement(_)
                        )
                    })
                }
                _ => false,
            }
        }
    }
}

fn is_react_18_3_or_higher(ctx: &RuleContext<NoReactStringRefs>) -> bool {
    ctx.get_service::<Option<(Utf8PathBuf, Arc<PackageJson>)>>()
        .and_then(|manifest| {
            manifest
                .as_ref()
                .map(|(_, package_json)| package_json.matches_dependency("react", ">=18.3.0"))
        })
        == Some(true)
}

/// Returns `Some` if the member expression is `this.refs`, otherwise `None`.
fn this_refs_range(member_expression: &AnyJsMemberExpression) -> Option<TextRange> {
    if member_expression
        .object()
        .ok()?
        .as_js_this_expression()
        .is_some()
        && member_expression.member_name()?.text() == "refs"
    {
        return Some(member_expression.range());
    }

    None
}
