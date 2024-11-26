use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use biome_analyze::{Ast, RuleSource};
use biome_console::{markup, MarkupBuf};
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsAssignment, AnyJsAssignmentPattern, AnyJsCallArgument,
    AnyJsDeclarationClause, AnyJsExportClause, AnyJsExportNamedSpecifier, AnyJsExpression,
    AnyJsObjectMemberName, AnyJsTemplateElement, ClassMemberName, JsMethodObjectMember,
};
use biome_js_syntax::{
    AnyJsClassMember, AnyJsObjectMember, JsAssignmentExpression, JsCallExpression,
    JsComputedMemberName, JsExport,
};
use biome_rowan::{declare_node_union, AstNode, AstSeparatedList, TextRange};

declare_lint_rule! {
    /// Disallow `then` property.
    ///
    /// When combining objects with a `then` method (thenable objects) with await expressions or dynamic imports, caution is necessary.
    /// These syntaxes interpret the object's then method as intended for the resolution or rejection of a promise, which can lead to unexpected behavior or errors.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// export {then};
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = {
    ///     then() {}
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = {
    ///     get then() {}
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = {
    ///    get then() {}
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo.then = function () {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///     then() {}
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///     static then() {}
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// export {then as success};
    /// ```
    ///
    /// ```js
    /// const foo = {
    ///     success() {}
    /// };
    /// ```
    ///
    /// ```js
    /// class Foo {
    ///     success() {}
    /// }
    /// ```
    ///
    /// ```js
    /// const foo = bar.then;
    /// ```
    ///
    pub NoThenProperty {
        version: "1.5.0",
        name: "noThenProperty",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-thenable")],
        recommended: true,
        severity: Severity::Error,
    }
}

declare_node_union! {
    pub NoThenPropertyQuery =
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
    fn diagnostic_message(&self) -> MarkupBuf {
        match self.message {
            NoThenPropertyMessage::Object => {
                markup! { "Do not add "<Emphasis>"then"</Emphasis>" to an object." }.to_owned()
            }
            NoThenPropertyMessage::Export => {
                markup! { "Do not export "<Emphasis>"then"</Emphasis>"."}.to_owned()
            }
            NoThenPropertyMessage::Class => {
                markup! {"Do not add "<Emphasis>"then"</Emphasis>" to a class." }.to_owned()
            }
        }
    }
}
impl Rule for NoThenProperty {
    type Query = Ast<NoThenPropertyQuery>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        match binding {
            NoThenPropertyQuery::AnyJsObjectMember(node) => process_js_object_member(node),
            NoThenPropertyQuery::AnyJsClassMember(node) => process_js_class_member(node),
            NoThenPropertyQuery::JsComputedMemberName(node) => {
                process_js_computed_member_name(node)
            }
            NoThenPropertyQuery::JsAssignmentExpression(node) => process_js_assignment_expr(node),
            NoThenPropertyQuery::JsCallExpression(node) => process_js_call_expr(node),
            NoThenPropertyQuery::JsExport(node) => process_js_export_named_clause(node),
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.range,
            state.diagnostic_message(),
        ))
    }
}

fn process_js_object_member(node: &AnyJsObjectMember) -> Option<RuleState> {
    match node {
        AnyJsObjectMember::JsGetterObjectMember(node) => {
            if node.name().ok()?.name()? == "then" {
                return Some(RuleState {
                    range: node.name().ok()?.range(),
                    message: NoThenPropertyMessage::Object,
                });
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
        AnyJsObjectMember::JsMethodObjectMember(node) => {
            return process_js_method_object_member(node)
        }
        _ => return None,
    };
    None
}

fn process_js_method_object_member(node: &JsMethodObjectMember) -> Option<RuleState> {
    let member_name = node.name().ok()?;
    match member_name {
        AnyJsObjectMemberName::JsComputedMemberName(expr) => match expr.expression().ok()? {
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
                    if let AnyJsTemplateElement::JsTemplateChunkElement(chunk) = l {
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
        },
        AnyJsObjectMemberName::JsLiteralMemberName(literal) => {
            if literal.name().ok()? == "then" {
                return Some(RuleState {
                    range: node.name().ok()?.range(),
                    message: NoThenPropertyMessage::Object,
                });
            }
        }
        _ => return None,
    }
    None
}

fn process_js_class_member(node: &AnyJsClassMember) -> Option<RuleState> {
    let any_class_member_name = node.name().ok()??;
    if let Some(ClassMemberName::Public(name)) = any_class_member_name.name() {
        if name == "then" {
            return Some(RuleState {
                range: any_class_member_name.range(),
                message: NoThenPropertyMessage::Class,
            });
        }
    }
    None
}

fn process_js_computed_member_name(node: &JsComputedMemberName) -> Option<RuleState> {
    match node.expression().ok()? {
        AnyJsExpression::AnyJsLiteralExpression(expr) => {
            if expr.value_token().ok()?.text() == "\"then\"" {
                return Some(RuleState {
                    range: expr.range(),
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
    }
    None
}

fn process_js_assignment_expr(node: &JsAssignmentExpression) -> Option<RuleState> {
    match node.left().ok()? {
        AnyJsAssignmentPattern::AnyJsAssignment(assignment) => match assignment {
            AnyJsAssignment::JsComputedMemberAssignment(c) => {
                if c.member().ok()?.to_trimmed_string() == "\"then\""
                    || c.member().ok()?.to_trimmed_string() == "`then`"
                {
                    return Some(RuleState {
                        range: node.left().ok()?.range(),
                        message: NoThenPropertyMessage::Object,
                    });
                }
            }
            AnyJsAssignment::JsStaticMemberAssignment(m) => {
                if m.member().ok()?.to_trimmed_string() == "then" {
                    return Some(RuleState {
                        range: node.left().ok()?.range(),
                        message: NoThenPropertyMessage::Object,
                    });
                }
            }
            _ => return None,
        },
        _ => return None,
    }
    None
}

fn process_js_call_expr(node: &JsCallExpression) -> Option<RuleState> {
    if node.is_optional_chain() {
        return None;
    }
    match node.callee().ok()? {
        AnyJsExpression::JsStaticMemberExpression(m) => {
            if m.is_optional_chain() {
                return None;
            }

            let callee = m.object().ok()?.to_trimmed_string();
            let member = m.member().ok()?.to_trimmed_string();

            let args = node.arguments().ok()?.args();
            let first = args.iter().next()?.ok()?;

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
                                AnyJsArrayElement::AnyJsExpression(
                                    AnyJsExpression::JsArrayExpression(arg),
                                ) => {
                                    let key = arg.elements().first()?.ok()?;
                                    if key.to_trimmed_string() == "\"then\""
                                        || key.to_trimmed_string() == "`then`"
                                    {
                                        return Some(RuleState {
                                            range: key.range(),
                                            message: NoThenPropertyMessage::Object,
                                        });
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
            if (callee == "Object" || callee == "Reflect") && member == "defineProperty" {
                if args.len() < 3 {
                    return None;
                }
                if matches!(first, AnyJsCallArgument::JsSpread(_)) {
                    return None;
                }
                let second = args.iter().nth(1)?.ok()?;
                if second.to_trimmed_string() == "\"then\""
                    || second.to_trimmed_string() == "`then`"
                {
                    return Some(RuleState {
                        range: second.range(),
                        message: NoThenPropertyMessage::Object,
                    });
                }
            }
        }
        _ => return None,
    }
    None
}

fn process_js_export_named_clause(node: &JsExport) -> Option<RuleState> {
    match node.export_clause().ok()? {
        AnyJsExportClause::JsExportNamedClause(node) => {
            let specifiers = node.specifiers();
            for specifier in specifiers.iter() {
                match specifier.ok()? {
                    AnyJsExportNamedSpecifier::JsExportNamedShorthandSpecifier(name) => {
                        if name.name().ok()?.name().ok()? == "then" {
                            return Some(RuleState {
                                range: name.name().ok()?.range(),
                                message: NoThenPropertyMessage::Export,
                            });
                        }
                    }
                    AnyJsExportNamedSpecifier::JsExportNamedSpecifier(name) => {
                        if name.exported_name().ok()?.to_trimmed_string() == "then" {
                            return Some(RuleState {
                                range: name.exported_name().ok()?.range(),
                                message: NoThenPropertyMessage::Export,
                            });
                        }
                    }
                }
            }
        }
        AnyJsExportClause::AnyJsDeclarationClause(
            AnyJsDeclarationClause::JsVariableDeclarationClause(node),
        ) => {
            let decls = node.declaration().ok()?;
            for d in decls.declarators().iter() {
                let id = d.ok()?.id().ok()?;
                if id.to_trimmed_string() == "then" {
                    return Some(RuleState {
                        range: id.range(),
                        message: NoThenPropertyMessage::Object,
                    });
                }
            }
        }
        _ => return None,
    }
    None
}
