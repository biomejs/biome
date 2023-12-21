use crate::semantic_services::Semantic;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsAssignment, AnyJsAssignmentPattern, AnyJsCallArgument,
    AnyJsClassMemberName, AnyJsDeclarationClause, AnyJsExportClause, AnyJsExportNamedSpecifier,
    AnyJsExpression, AnyJsObjectMemberName, AnyJsTemplateElement,
};
use biome_js_syntax::{
    AnyJsClassMember, AnyJsObjectMember, JsAssignmentExpression, JsCallExpression,
    JsComputedMemberName, JsExport,
};
use biome_rowan::{declare_node_union, AstNode, AstSeparatedList, TextRange};

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
    pub(crate) NoThenPropertyQuery =
        AnyJsObjectMember |
        JsComputedMemberName |
        AnyJsClassMember |
        JsAssignmentExpression |
        JsCallExpression  |
        JsExport
}

pub enum NoThenPropertyMessage {
    Object,
    Export,
    Class,
}

pub struct RuleState {
    range: TextRange,
    message: NoThenPropertyMessage,
}

impl RuleState {
    fn diagnostic_message(&self) -> &str {
        match self.message {
            NoThenPropertyMessage::Object => "Do not add `then` to an object.",
            NoThenPropertyMessage::Export => "Do not export `then`.",
            NoThenPropertyMessage::Class => "Do not add `then` to a class.",
        }
    }
}
impl Rule for NoThenProperty {
    type Query = Semantic<NoThenPropertyQuery>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        match binding {
            NoThenPropertyQuery::AnyJsObjectMember(node) => match node {
                AnyJsObjectMember::JsGetterObjectMember(node) => {
                    return Some(RuleState {
                        range: node.name().ok()?.range(),
                        message: NoThenPropertyMessage::Object,
                    });
                }
                AnyJsObjectMember::JsMethodObjectMember(node) => {
                    let member_name = node.name().ok()?;
                    match member_name {
                        AnyJsObjectMemberName::JsComputedMemberName(expr) => {
                            match expr.expression().ok()? {
                                AnyJsExpression::AnyJsLiteralExpression(lit) => {
                                    if lit.value_token().ok()?.text() == "then" {
                                        return Some(RuleState {
                                            range: node.name().ok()?.range(),
                                            message: NoThenPropertyMessage::Object,
                                        });
                                    }
                                }
                                AnyJsExpression::JsTemplateExpression(lit) => {
                                    for l in lit.elements() {
                                        if let AnyJsTemplateElement::JsTemplateChunkElement(chunk) =
                                            l
                                        {
                                            if chunk.template_chunk_token().ok()?.text() == "then" {
                                                return Some(RuleState {
                                                    range: node.name().ok()?.range(),
                                                    message: NoThenPropertyMessage::Object,
                                                });
                                            }
                                        }
                                    }
                                }
                                _ => return None,
                            }
                        }
                        AnyJsObjectMemberName::JsLiteralMemberName(literal) => {
                            if literal.name().ok()? == "then" {
                                return Some(RuleState {
                                    range: node.name().ok()?.range(),
                                    message: NoThenPropertyMessage::Object,
                                });
                            }
                        }
                    }
                }
                AnyJsObjectMember::JsPropertyObjectMember(node) => {
                    if node.name().ok()?.name()? == "then" {
                        return Some(RuleState {
                            range: node.name().ok()?.range(),
                            message: NoThenPropertyMessage::Object,
                        });
                    }
                }
                _ => return None,
            },
            NoThenPropertyQuery::AnyJsClassMember(node) => {
                if let Some(AnyJsClassMemberName::JsPrivateClassMemberName(_)) = node.name().ok()? {
                    return None;
                }
                match node {
                    AnyJsClassMember::JsGetterClassMember(node) => {
                        if node.name().ok()?.name()? == "then" {
                            return Some(RuleState {
                                range: node.name().ok()?.range(),
                                message: NoThenPropertyMessage::Class,
                            });
                        }
                    }
                    AnyJsClassMember::JsMethodClassMember(node) => {
                        if node.name().ok()?.name()? == "then" {
                            return Some(RuleState {
                                range: node.name().ok()?.range(),
                                message: NoThenPropertyMessage::Class,
                            });
                        }
                    }
                    AnyJsClassMember::JsPropertyClassMember(node) => {
                        if node.name().ok()?.name()? == "then" {
                            return Some(RuleState {
                                range: node.name().ok()?.range(),
                                message: NoThenPropertyMessage::Class,
                            });
                        }
                    }
                    AnyJsClassMember::JsSetterClassMember(node) => {
                        if node.name().ok()?.name()? == "then" {
                            return Some(RuleState {
                                range: node.name().ok()?.range(),
                                message: NoThenPropertyMessage::Class,
                            });
                        }
                    }
                    _ => return None,
                }
            }
            NoThenPropertyQuery::JsComputedMemberName(node) => match node.expression().ok()? {
                AnyJsExpression::AnyJsLiteralExpression(lit) => {
                    if lit.value_token().ok()?.text() == "\"then\"" {
                        return Some(RuleState {
                            range: lit.range(),
                            message: NoThenPropertyMessage::Object,
                        });
                    }
                }
                AnyJsExpression::JsTemplateExpression(lit) => {
                    for l in lit.elements() {
                        if let AnyJsTemplateElement::JsTemplateChunkElement(chunk) = l {
                            if chunk.template_chunk_token().ok()?.text() == "then" {
                                return Some(RuleState {
                                    range: chunk.range(),
                                    message: NoThenPropertyMessage::Object,
                                });
                            }
                        }
                    }
                }
                _ => return None,
            },
            NoThenPropertyQuery::JsAssignmentExpression(node) => match node.left().ok()? {
                AnyJsAssignmentPattern::AnyJsAssignment(assignment) => match assignment {
                    AnyJsAssignment::JsComputedMemberAssignment(c) => {
                        if c.member().ok()?.text() == "\"then\""
                            || c.member().ok()?.text() == "`then`"
                        {
                            return Some(RuleState {
                                range: node.left().ok()?.range(),
                                message: NoThenPropertyMessage::Object,
                            });
                        }
                    }
                    AnyJsAssignment::JsStaticMemberAssignment(m) => {
                        if m.member().ok()?.text() == "then" {
                            return Some(RuleState {
                                range: node.left().ok()?.range(),
                                message: NoThenPropertyMessage::Object,
                            });
                        }
                    }
                    _ => return None,
                },
                _ => return None,
            },
            NoThenPropertyQuery::JsCallExpression(node) => {
                if node.is_optional_chain() {
                    return None;
                }
                match node.callee().ok()? {
                    AnyJsExpression::JsStaticMemberExpression(m) => {
                        if m.is_optional_chain() {
                            return None;
                        }

                        let callee = m.object().ok()?.text();
                        let member = m.member().ok()?.text();

                        let args = node.arguments().ok()?.args();
                        let first = args.iter().nth(0)?.ok()?;

                        // Handle `Object.fromEntries()`
                        // ex)
                        //   Object.fromEntries([["then", 1]])
                        //   Object.fromEntries([['foo', 'foo'], ['then', 32],['bar', 'bar']]);
                        if callee == "Object" && member == "fromEntries" {
                            if args.len() != 1 {
                                return None;
                            }
                            if let AnyJsCallArgument::AnyJsExpression(expr) = &first {
                                if let AnyJsExpression::JsArrayExpression(array) = expr {
                                    for arr in array.elements().iter() {
                                        match arr.ok()? {
                                            AnyJsArrayElement::AnyJsExpression(expr) => {
                                                match expr {
                                                    AnyJsExpression::JsArrayExpression(arg) => {
                                                        let key = arg.elements().first()?.ok()?;
                                                        if key.text() == "\"then\""
                                                            || key.text() == "`then`"
                                                        {
                                                            return Some(RuleState {
                                                                range: key.range(),
                                                                message:
                                                                    NoThenPropertyMessage::Object,
                                                            });
                                                        }
                                                    }
                                                    _ => continue,
                                                }
                                            }
                                            _ => continue,
                                        }
                                    }
                                } else {
                                    return None;
                                }
                            }
                        }

                        // Handle `Object.defineProperty({}, "then", {})`
                        if (callee == "Object" || callee == "Reflect") && member == "defineProperty"
                        {
                            if args.len() < 3 {
                                return None;
                            }
                            if matches!(first, AnyJsCallArgument::JsSpread(_)) {
                                return None;
                            }
                            let second = args.iter().nth(1)?.ok()?;
                            if second.text() == "\"then\"" || second.text() == "`then`" {
                                return Some(RuleState {
                                    range: second.range(),
                                    message: NoThenPropertyMessage::Object,
                                });
                            }
                        }
                    }
                    _ => return None,
                }
            }
            NoThenPropertyQuery::JsExport(node) => {
                let export_clause = node.export_clause().ok()?;
                match export_clause {
                    AnyJsExportClause::JsExportNamedClause(node) => {
                        let specifiers = node.specifiers();
                        for specifier in specifiers.iter() {
                            match specifier.ok()? {
                                AnyJsExportNamedSpecifier::JsExportNamedShorthandSpecifier(
                                    name,
                                ) => {
                                    if name.name().ok()?.name().ok()? == "then" {
                                        return Some(RuleState {
                                            range: name.name().ok()?.range(),
                                            message: NoThenPropertyMessage::Export,
                                        });
                                    }
                                }
                                AnyJsExportNamedSpecifier::JsExportNamedSpecifier(name) => {
                                    if name.exported_name().ok()?.text() == "then" {
                                        return Some(RuleState {
                                            range: name.exported_name().ok()?.range(),
                                            message: NoThenPropertyMessage::Export,
                                        });
                                    }
                                }
                            }
                        }
                    }
                    AnyJsExportClause::AnyJsDeclarationClause(node) => match node {
                        AnyJsDeclarationClause::JsVariableDeclarationClause(node) => {
                            let decls = node.declaration().ok()?;
                            for d in decls.declarators().iter() {
                                let id = d.ok()?.id().ok()?;
                                if id.text() == "then" {
                                    return Some(RuleState {
                                        range: id.range(),
                                        message: NoThenPropertyMessage::Object,
                                    });
                                }
                            }
                        }
                        _ => return None,
                    },
                    _ => return None,
                }
            }
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.range,
            state.diagnostic_message(),
        ))
    }
}
