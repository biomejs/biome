use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsExpression, AnyJsMemberExpression,
    JsAssignmentExpression, JsAssignmentOperator, JsCallExpression, JsInitializerClause,
    JsPropertyClassMember, JsSyntaxToken, JsVariableDeclarator,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_react_naming_convention::UseReactNamingConventionOptions;
use biome_string_case::Case;

use crate::react::{ReactLibrary, is_react_call_api};
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Enforces naming conventions for React `createContext`, `useId`, and `useRef`.
    ///
    /// This rules checks the variable a React API hook is assigned to
    /// and enforces a name convention to make the intent of a value obvious at a glance:
    ///
    /// - A value assigned from `createContext` must be a valid component name (PascalCase) with
    ///   the suffix `Context`, for example `ThemeContext`.
    /// - A value assigned from `useId` must be named `id` or a valid camelCase name ending with
    ///   `Id`, for example `myId`.
    /// - A value assigned from `useRef` must be named `ref` or a valid camelCase name ending with
    ///   `Ref`, for example `myRef`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// import { createContext } from "react";
    /// const theme = createContext("");
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// import { useId } from "react";
    /// const randomString = useId();
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// import { useRef } from "react";
    /// const node = useRef(null);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// import { createContext } from "react";
    /// const ThemeContext = createContext("");
    /// ```
    ///
    /// ```jsx
    /// import { useId } from "react";
    /// const myId = useId();
    /// ```
    ///
    /// ```jsx
    /// import { useRef } from "react";
    /// const myRef = useRef(null);
    /// ```
    ///
    pub UseReactNamingConvention {
        version: "next",
        name: "useReactNamingConvention",
        language: "jsx",
        recommended: false,
        severity: Severity::Information,
        domains: &[RuleDomain::React],
        sources: &[
            RuleSource::EslintReactXyz("naming-convention-context-name").same(),
            RuleSource::EslintReactNamingConvention("context-name").same(),
            RuleSource::EslintReactXyz("naming-convention-id-name").same(),
            RuleSource::EslintReactNamingConvention("id-name").same(),
            RuleSource::EslintReactXyz("naming-convention-ref-name").same(),
            RuleSource::EslintReactNamingConvention("ref-name").same(),
        ],
    }
}

impl Rule for UseReactNamingConvention {
    type Query = Semantic<JsCallExpression>;
    type State = ReactNamingConventionState;
    type Signals = Option<Self::State>;
    type Options = UseReactNamingConventionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();
        let callee = call.callee().ok()?;

        let convention = if is_react_call_api(&callee, model, ReactLibrary::React, "createContext")
        {
            ReactNamingConvention::Context
        } else if is_react_call_api(&callee, model, ReactLibrary::React, "useId") {
            ReactNamingConvention::Id
        } else if is_react_call_api(&callee, model, ReactLibrary::React, "useRef") {
            ReactNamingConvention::Ref
        } else {
            return None;
        };

        let expression = AnyJsExpression::from(call.clone()).outer_expression()?;
        let parent = expression.syntax().parent()?;

        if AnyJsMemberExpression::can_cast(parent.kind()) {
            return None;
        }

        let name = if let Some(initializer) = JsInitializerClause::cast_ref(&parent) {
            resolve_initializer_target_name(&initializer)
        } else if let Some(assignment) = JsAssignmentExpression::cast_ref(&parent) {
            if assignment.operator().ok() != Some(JsAssignmentOperator::Assign) {
                return None;
            }
            resolve_assignment_target_name(&assignment)
        } else {
            None
        }?;
        if convention.is_satisfied_by(name.text_trimmed()) {
            return None;
        }

        Some(ReactNamingConventionState {
            convention,
            range: name.text_trimmed_range(),
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state.convention {
            ReactNamingConvention::Context => Some(RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "This React context is not following the context naming convention."
                },
            )
            .note(markup! {
                "React contexts have to follow a naming convention to make them more recognizable across the codebase."
            })
            .note(markup! {
                "Rename it to a PascalCase name ending with "<Emphasis>"Context"</Emphasis>", such as "<Emphasis>"ThemeContext"</Emphasis>"."
            })),
            ReactNamingConvention::Id => Some(RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "This React id is not following the id naming convention."
                },
            )
            .note(markup! {
                "React ids have to follow a naming convention to make them more recognizable across the codebase."
            })
            .note(markup! {
                "Rename the value to "<Emphasis>"id"</Emphasis>" or a name ending with "<Emphasis>"Id"</Emphasis>", such as "<Emphasis>"myId"</Emphasis>"."
            })),
            ReactNamingConvention::Ref => Some(RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "This React ref is not following the ref naming convention."
                },
            )
            .note(markup! {
                "React refs have to follow a naming convention to make them more recognizable across the codebase."
            })
            .note(markup! {
                "Rename the value to "<Emphasis>"ref"</Emphasis>" or a name ending with "<Emphasis>"Ref"</Emphasis>", such as "<Emphasis>"myRef"</Emphasis>"."
            })),
        }
    }
}

pub struct ReactNamingConventionState {
    convention: ReactNamingConvention,
    range: TextRange,
}

/// The React API whose naming convention is being enforced.
#[derive(Clone, Copy)]
pub enum ReactNamingConvention {
    Context,
    Id,
    Ref,
}

impl ReactNamingConvention {
    fn is_satisfied_by(self, name: &str) -> bool {
        match self {
            Self::Context => {
                Case::identify(name, true) == Case::Pascal && name.ends_with("Context")
            }
            Self::Id => {
                name == "id" || (Case::identify(name, true) == Case::Camel && name.ends_with("Id"))
            }
            Self::Ref => {
                name == "ref"
                    || (Case::identify(name, true) == Case::Camel && name.ends_with("Ref"))
            }
        }
    }
}

/// Returns the identifier a variable declarator or class property binds the call result to.
///
/// It resolves variable declarators (`const myRef = useRef()`) and class properties
/// (`class Foo { myRef = useRef() }`), and returns `None` when the result is not stored
/// in a plain identifier.
fn resolve_initializer_target_name(initializer: &JsInitializerClause) -> Option<JsSyntaxToken> {
    let declaration = initializer.syntax().parent()?;

    if let Some(declarator) = JsVariableDeclarator::cast_ref(&declaration) {
        declarator
            .id()
            .ok()?
            .as_any_js_binding()?
            .as_js_identifier_binding()?
            .name_token()
            .ok()
    } else if let Some(property) = JsPropertyClassMember::cast_ref(&declaration) {
        property
            .name()
            .ok()?
            .as_js_literal_member_name()?
            .value()
            .ok()
    } else {
        None
    }
}

/// Returns the identifier an assignment expression targets (`refs.myRef = useRef()`).
///
/// It returns `None` when the result is not stored in a plain identifier.
fn resolve_assignment_target_name(assignment: &JsAssignmentExpression) -> Option<JsSyntaxToken> {
    match assignment.left().ok()? {
        AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsIdentifierAssignment(
            identifier,
        )) => identifier.name_token().ok(),
        AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsStaticMemberAssignment(
            member,
        )) => member.member().ok()?.as_js_name()?.value_token().ok(),
        _ => None,
    }
}
