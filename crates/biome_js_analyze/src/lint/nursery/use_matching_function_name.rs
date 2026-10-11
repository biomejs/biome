use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::fmt::{self, Display};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsExpression, ClassMemberName,
    JsAssignmentExpression, JsFunctionExpression, JsInitializerClause, JsPropertyClassMember,
    JsPropertyObjectMember, JsSyntaxKind, JsVariableDeclarator, T, inner_string_text,
    static_value::StaticValue, unescape_js_string,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, Text, TextRange, TokenText};
use biome_rule_options::use_matching_function_name::UseMatchingFunctionNameOptions;
use biome_unicode_table::is_js_ident;

declare_lint_rule! {
    /// Require function names to match the name of the variable or property they are assigned to.
    ///
    /// In `const foo = function bar() {}`, the function has two names: `bar`, its own name, and
    /// `foo`, the variable that stores it. The code calls the function as `foo`,
    /// while stack traces and debugging tools show `bar`.
    /// When the two names differ, it's harder to tell which function an error comes from.
    ///
    /// The rule checks functions written with the `function` keyword and given their own name
    /// when they are:
    ///
    /// - used to initialize a variable;
    /// - assigned to a variable or property with `=`, `&&=`, `||=`, or `??=`;
    /// - used as the value of an object property or class property.
    ///
    /// The rule ignores anonymous functions, arrow functions, private class properties,
    /// properties whose name isn't known before the code runs, such as `obj[key]`,
    /// and assignments to `module.exports`.
    /// It also ignores names that a function can't have, such as `"my-key"` or `class`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = function bar() {};
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// obj.foo = function bar() {};
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const obj = { foo: function bar() {} };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///     foo = function bar() {};
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = function foo() {};
    /// const bar = function () {};
    /// obj.foo = function foo() {};
    /// obj[key] = function bar() {};
    /// obj["my-key"] = function bar() {};
    /// module.exports = function foo() {};
    /// const obj = { foo: function foo() {}, bar: () => {} };
    /// class Foo {
    ///     foo = function foo() {};
    ///     #bar = function baz() {};
    /// }
    /// ```
    ///
    pub UseMatchingFunctionName {
        version: "next",
        name: "useMatchingFunctionName",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("func-name-matching").same()],
    }
}

impl Rule for UseMatchingFunctionName {
    type Query = Ast<JsFunctionExpression>;
    type State = NameMismatch;
    type Signals = Option<Self::State>;
    type Options = UseMatchingFunctionNameOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let function = ctx.query();
        let function_name = function_name(function)?;
        let expression = AnyJsExpression::from(function.clone()).outer_expression()?;
        let parent = expression.syntax().parent()?;
        let (target, name, start) = match parent.kind() {
            JsSyntaxKind::JS_INITIALIZER_CLAUSE => {
                let holder = JsInitializerClause::unwrap_cast(parent).syntax().parent()?;
                match holder.kind() {
                    JsSyntaxKind::JS_VARIABLE_DECLARATOR => {
                        let declarator = JsVariableDeclarator::unwrap_cast(holder);
                        let name = declarator
                            .id()
                            .ok()?
                            .as_any_js_binding()?
                            .as_js_identifier_binding()?
                            .name_token()
                            .ok()?
                            .token_text_trimmed();
                        (NameTarget::Variable, name, declarator.range().start())
                    }
                    JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER => {
                        let member = JsPropertyClassMember::unwrap_cast(holder);
                        let ClassMemberName::Public(name) = member.name().ok()?.name()? else {
                            return None;
                        };
                        (NameTarget::Property, name, member.range().start())
                    }
                    _ => return None,
                }
            }
            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                let assignment = JsAssignmentExpression::unwrap_cast(parent);
                if !STORING_ASSIGNMENT_OPERATORS.contains(assignment.operator_token().ok()?.kind())
                    || assignment.right().ok()?.syntax() != expression.syntax()
                {
                    return None;
                }
                let AnyJsAssignmentPattern::AnyJsAssignment(left) = assignment.left().ok()? else {
                    return None;
                };
                let (target, name) = assignment_target_name(&left)?;
                (target, name, assignment.range().start())
            }
            JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER => {
                let member = JsPropertyObjectMember::unwrap_cast(parent);
                if member.value().ok()?.syntax() != expression.syntax() {
                    return None;
                }
                let name = member.name().ok()?.name()?;
                (NameTarget::Property, name, member.range().start())
            }
            _ => return None,
        };
        let unescaped_name = unescape_js_string(name.clone());
        if unescaped_name == function_name || !can_be_function_name(&unescaped_name) {
            return None;
        }
        Some(NameMismatch {
            target,
            name,
            range: TextRange::new(start, function.id()?.range().end()),
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let function_name = function_name(ctx.query())?;
        let name = unescape_js_string(state.name.clone());
        let target = state.target;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "The function name "<Emphasis>{function_name.text()}</Emphasis>" doesn't match the "{target}" name "<Emphasis>{name.text()}</Emphasis>"."
                },
            )
            .note(markup! {
                "The code calls this function through the "{target}" name, but stack traces and debugging tools show the function name. Different names make it harder to tell which function an error comes from."
            })
            .note(markup! {
                "Rename the function or the "{target}" so that both names are the same."
            }),
        )
    }
}

pub struct NameMismatch {
    target: NameTarget,
    /// The name of the variable or property that stores the function, as written in the source.
    name: TokenText,
    /// Range from the start of the declaration, assignment, or property to the end of the
    /// function's name.
    range: TextRange,
}

#[derive(Clone, Copy)]
pub enum NameTarget {
    Variable,
    Property,
}

impl Display for NameTarget {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Variable => "variable",
            Self::Property => "property",
        })
    }
}

/// Assignment operators after which the target holds the function.
const STORING_ASSIGNMENT_OPERATORS: TokenSet<JsSyntaxKind> =
    token_set![T![=], T![&&=], T![||=], T![??=]];

fn function_name(function: &JsFunctionExpression) -> Option<Text> {
    let name = function
        .id()?
        .as_js_identifier_binding()?
        .name_token()
        .ok()?
        .token_text_trimmed();
    Some(unescape_js_string(name))
}

/// Returns the name assigned to by `left`, or `None` if the name isn't known statically, is
/// private, or is `module.exports`.
fn assignment_target_name(left: &AnyJsAssignment) -> Option<(NameTarget, TokenText)> {
    let (object, name) = match left {
        AnyJsAssignment::JsIdentifierAssignment(identifier) => {
            let name = identifier.name_token().ok()?.token_text_trimmed();
            return Some((NameTarget::Variable, name));
        }
        AnyJsAssignment::JsStaticMemberAssignment(member) => {
            let name = member
                .member()
                .ok()?
                .as_js_name()?
                .value_token()
                .ok()?
                .token_text_trimmed();
            (member.object().ok()?, name)
        }
        AnyJsAssignment::JsComputedMemberAssignment(member) => {
            let StaticValue::String(token) =
                member.member().ok()?.omit_parentheses().as_static_value()?
            else {
                return None;
            };
            (member.object().ok()?, inner_string_text(&token))
        }
        AnyJsAssignment::JsParenthesizedAssignment(parenthesized) => {
            return assignment_target_name(&parenthesized.assignment().ok()?);
        }
        _ => return None,
    };
    if name == "exports"
        && object
            .as_js_identifier_expression()
            .and_then(|identifier| identifier.name().ok())
            .is_some_and(|reference| reference.has_name("module"))
    {
        return None;
    }
    Some((NameTarget::Property, name))
}

/// Returns `true` if a function expression can be named `name` in at least some code.
///
/// Words reserved only in strict mode, such as `let` and `static`, are accepted because
/// scripts can use them as function names.
fn can_be_function_name(name: &str) -> bool {
    is_js_ident(name)
        && !JsSyntaxKind::from_keyword(name).is_some_and(|keyword| {
            keyword.is_non_contextual_keyword() && !keyword.is_future_reserved_keyword()
        })
}
