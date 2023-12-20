use crate::semantic_services::Semantic;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsCallArgument, AnyJsClassMemberName,
    AnyJsExpression, AnyJsObjectMemberName, AnyJsTemplateElement, JsSpread,
};
use biome_js_syntax::{
    JsAssignmentExpression, JsCallExpression, JsComputedMemberName, JsGetterClassMember,
    JsGetterObjectMember, JsMethodClassMember, JsMethodObjectMember, JsPropertyClassMember,
    JsPropertyObjectMember, JsSetterClassMember,
};
use biome_rowan::{declare_node_union, AstNode, AstSeparatedList};

declare_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding ESLint rule (if any):
    ///
    /// Source: https://eslint.org/docs/latest/rules/rule-name
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// var a = 1;
    /// ```
    ///
    pub(crate) NoThenProperty {
        version: "next",
        name: "noThenProperty",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) NoThenPropertyQuery = JsComputedMemberName | JsMethodObjectMember | JsPropertyObjectMember | JsGetterObjectMember | JsPropertyClassMember | JsMethodClassMember | JsGetterClassMember | JsSetterClassMember | JsAssignmentExpression | JsCallExpression
}

pub enum NoThenPropertyState {
    Object,
    Export,
    Class,
}

impl NoThenPropertyState {
    fn diagnostic_message(&self) -> &str {
        match self {
            NoThenPropertyState::Object => "Do not add `then` to an object.",
            NoThenPropertyState::Export => "Do not export `then`.",
            NoThenPropertyState::Class => "Do not add `then` to a class.",
        }
    }
}

impl Rule for NoThenProperty {
    type Query = Semantic<NoThenPropertyQuery>;
    type State = NoThenPropertyState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        match binding {
            NoThenPropertyQuery::JsPropertyObjectMember(node) => {
                if node.name().ok()?.name()? == "then" {
                    return Some(NoThenPropertyState::Object);
                }
            }
            NoThenPropertyQuery::JsGetterObjectMember(node) => {
                if node.name().ok()?.name()? == "then" {
                    return Some(NoThenPropertyState::Object);
                }
            }
            NoThenPropertyQuery::JsPropertyClassMember(node) => {
                if let AnyJsClassMemberName::JsPrivateClassMemberName(_) = node.name().ok()? {
                    return None;
                }
                if node.name().ok()?.name()? == "then" {
                    return Some(NoThenPropertyState::Class);
                }
            }
            NoThenPropertyQuery::JsMethodClassMember(node) => {
                if let AnyJsClassMemberName::JsPrivateClassMemberName(_) = node.name().ok()? {
                    return None;
                }
                if node.name().ok()?.name()? == "then" {
                    return Some(NoThenPropertyState::Class);
                }
            }
            NoThenPropertyQuery::JsGetterClassMember(node) => {
                if let AnyJsClassMemberName::JsPrivateClassMemberName(_) = node.name().ok()? {
                    return None;
                }
                if node.name().ok()?.name()? == "then" {
                    return Some(NoThenPropertyState::Class);
                }
            }
            NoThenPropertyQuery::JsSetterClassMember(node) => {
                if let AnyJsClassMemberName::JsPrivateClassMemberName(_) = node.name().ok()? {
                    return None;
                }
                if node.name().ok()?.name()? == "then" {
                    return Some(NoThenPropertyState::Class);
                }
            }
            NoThenPropertyQuery::JsComputedMemberName(node) => match node.expression().ok()? {
                AnyJsExpression::AnyJsLiteralExpression(lit) => {
                    if lit.value_token().ok()?.text() == "\"then\"" {
                        return Some(NoThenPropertyState::Object);
                    }
                }
                AnyJsExpression::JsTemplateExpression(lit) => {
                    for l in lit.elements() {
                        if let AnyJsTemplateElement::JsTemplateChunkElement(chunk) = l {
                            if chunk.template_chunk_token().ok()?.text() == "then" {
                                return Some(NoThenPropertyState::Object);
                            }
                        }
                    }
                }
                _ => return None,
            },
            NoThenPropertyQuery::JsMethodObjectMember(node) => {
                let member_name = node.name().ok()?;
                match member_name {
                    AnyJsObjectMemberName::JsComputedMemberName(expr) => {
                        match expr.expression().ok()? {
                            AnyJsExpression::AnyJsLiteralExpression(lit) => {
                                if lit.value_token().ok()?.text() == "then" {
                                    return Some(NoThenPropertyState::Object);
                                }
                            }
                            AnyJsExpression::JsTemplateExpression(lit) => {
                                for l in lit.elements() {
                                    if let AnyJsTemplateElement::JsTemplateChunkElement(chunk) = l {
                                        if chunk.template_chunk_token().ok()?.text() == "then" {
                                            return Some(NoThenPropertyState::Object);
                                        }
                                    }
                                }
                            }
                            _ => return None,
                        }
                    }
                    AnyJsObjectMemberName::JsLiteralMemberName(literal) => {
                        if literal.name().ok()? == "then" {
                            return Some(NoThenPropertyState::Object);
                        }
                    }
                }
            }
            NoThenPropertyQuery::JsAssignmentExpression(node) => match node.left().ok()? {
                AnyJsAssignmentPattern::AnyJsAssignment(assignment) => match assignment {
                    AnyJsAssignment::JsComputedMemberAssignment(c) => {
                        if c.member().ok()?.text() == "\"then\""
                            || c.member().ok()?.text() == "`then`"
                        {
                            return Some(NoThenPropertyState::Object);
                        }
                    }
                    AnyJsAssignment::JsStaticMemberAssignment(m) => {
                        if m.member().ok()?.text() == "then" {
                            return Some(NoThenPropertyState::Object);
                        }
                    }
                    _ => return None,
                },
                _ => return None,
            },
            NoThenPropertyQuery::JsCallExpression(node) => match node.callee().ok()? {
                AnyJsExpression::JsStaticMemberExpression(m) => {
                    if m.object().ok()?.text() == "Object"
                        || m.object().ok()?.text() == "Reflect"
                            && m.member().ok()?.text() == "defineProperty"
                    {
                        if node.arguments().ok()?.args().len() < 3 {
                            return None;
                        }
                        let first = node.arguments().ok()?.args().iter().nth(0)?.ok()?;
                        if matches!(first, AnyJsCallArgument::JsSpread(_)) {
                            return None;
                        }
                        let args = node.arguments().ok()?.args().iter().nth(1)?.ok()?;
                        if args.text() == "\"then\"" || args.text() == "`then`" {
                            return Some(NoThenPropertyState::Object);
                        }
                    }
                }
                _ => return None,
            },
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let range = match node {
            NoThenPropertyQuery::JsPropertyObjectMember(node) => node.name().ok()?.range(),
            NoThenPropertyQuery::JsGetterObjectMember(node) => node.name().ok()?.range(),
            NoThenPropertyQuery::JsPropertyClassMember(node) => node.name().ok()?.range(),
            NoThenPropertyQuery::JsMethodClassMember(node) => node.name().ok()?.range(),
            NoThenPropertyQuery::JsGetterClassMember(node) => node.name().ok()?.range(),
            NoThenPropertyQuery::JsSetterClassMember(node) => node.name().ok()?.range(),
            NoThenPropertyQuery::JsComputedMemberName(node) => node.range(),
            NoThenPropertyQuery::JsMethodObjectMember(node) => node.range(),
            NoThenPropertyQuery::JsAssignmentExpression(node) => node.left().ok()?.range(),
            NoThenPropertyQuery::JsCallExpression(node) => {
                node.arguments().ok()?.args().iter().nth(1)?.ok()?.range()
            }
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            state.diagnostic_message(),
        ))
    }
}
