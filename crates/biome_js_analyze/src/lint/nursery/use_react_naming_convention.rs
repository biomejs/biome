use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsMemberExpression, JsAssignmentExpression,
    JsCallExpression, JsInitializerClause, JsSyntaxToken, JsVariableDeclarator,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_react_naming_convention::UseReactNamingConventionOptions;
use biome_string_case::Case;

use crate::react::{ReactLibrary, is_react_call_api};
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Enforces naming conventions for React `createContext`, `useId`, and `useRef`.
    ///
    /// React relies on naming conventions to make the intent of a value obvious at a glance.
    /// This rule checks the identifier a call is assigned to and enforces the convention that
    /// matches the called API:
    ///
    /// - A value assigned from `createContext` must be a valid component name (PascalCase) with
    ///   the suffix `Context`, for example `ThemeContext`.
    /// - A value assigned from `useId` must be named `id` or end with `Id`, for example `myId`.
    /// - A value assigned from `useRef` must be named `ref` or end with `Ref`, for example `myRef`.
    ///
    /// The convention is only enforced when the result is stored in an identifier. A result that
    /// is immediately dereferenced, such as `useRef(null).current`, is not checked.
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
                Case::identify(name, false) == Case::Pascal && name.ends_with("Context")
            }
            Self::Id => name == "id" || name.ends_with("Id"),
            Self::Ref => name == "ref" || name.ends_with("Ref"),
        }
    }
}

pub struct ReactNamingConventionState {
    convention: ReactNamingConvention,
    range: TextRange,
}

impl Rule for UseReactNamingConvention {
    type Query = Semantic<JsCallExpression>;
    type State = ReactNamingConventionState;
    type Signals = Option<Self::State>;
    type Options = UseReactNamingConventionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();
        let callee = call.callee().ok()?.omit_parentheses();

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

        // The result is immediately dereferenced (e.g. `useRef(null).current`)
        // rather than stored, so there is no name to check.
        let parent = call.syntax().parent()?;
        if AnyJsMemberExpression::can_cast(parent.kind()) {
            return None;
        }

        let name = resolve_assignment_target_name(call)?;
        if convention.is_satisfied_by(name.text_trimmed()) {
            return None;
        }

        Some(ReactNamingConventionState {
            convention,
            range: name.text_trimmed_range(),
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (message, reason, action) = match state.convention {
            ReactNamingConvention::Context => (
                markup! {
                    "This React context is not following the context convention."
                }
                .to_owned(),
                markup! {
                    "React contexts have to follow a convention to make them more recognizable across the codebase."
                }
                .to_owned(),
                markup! {
                    "Rename it to a PascalCase name ending with "<Emphasis>"Context"</Emphasis>", such as "<Emphasis>"ThemeContext"</Emphasis>"."
                }
                .to_owned(),
            ),
            ReactNamingConvention::Id => (
                markup! {
                    "This React id is not following the id convention."
                }
                .to_owned(),
                markup! {
                    "React ids have to follow a convention to make them more recognizable across the codebase."
                }
                .to_owned(),
                markup! {
                    "Rename the value to "<Emphasis>"id"</Emphasis>" or a name ending with "<Emphasis>"Id"</Emphasis>", such as "<Emphasis>"myId"</Emphasis>"."
                }
                .to_owned(),
            ),
            ReactNamingConvention::Ref => (
                markup! {
                    "This React ref is not following the ref convention."
                }
                .to_owned(),
                markup! {
                    "React refs have to follow a convention to make them more recognizable across the codebase."
                }
                .to_owned(),
                markup! {
                    "Rename the value to "<Emphasis>"ref"</Emphasis>" or a name ending with "<Emphasis>"Ref"</Emphasis>", such as "<Emphasis>"myRef"</Emphasis>"."
                }
                .to_owned(),
            ),
        };
        Some(
            RuleDiagnostic::new(rule_category!(), state.range, message)
                .note(reason)
                .note(action),
        )
    }
}

/// Returns the identifier token the call result is assigned to.
///
/// It resolves both variable declarators (`const myRef = useRef()`) and assignment
/// targets (`refs.myRef = useRef()`), and returns `None` when the result is not stored
/// in a plain identifier.
fn resolve_assignment_target_name(call: &JsCallExpression) -> Option<JsSyntaxToken> {
    let parent = call.syntax().parent()?;

    if let Some(initializer) = JsInitializerClause::cast_ref(&parent) {
        return initializer
            .parent::<JsVariableDeclarator>()?
            .id()
            .ok()?
            .as_any_js_binding()?
            .as_js_identifier_binding()?
            .name_token()
            .ok();
    }

    if let Some(assignment) = JsAssignmentExpression::cast_ref(&parent) {
        return match assignment.left().ok()? {
            AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsIdentifierAssignment(
                identifier,
            )) => identifier.name_token().ok(),
            AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsStaticMemberAssignment(
                member,
            )) => member.member().ok()?.as_js_name()?.value_token().ok(),
            _ => None,
        };
    }

    None
}
