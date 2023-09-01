use crate::semantic_services::SemanticServices;
use rome_analyze::declare_rule;
use rome_analyze::{context::RuleContext, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::Scope;
use rome_js_syntax::binding_ext::AnyJsBindingDeclaration;
use rome_js_syntax::{
    AnyTsType, TextRange, TsIndexSignatureParameter, TsIndexSignatureTypeMember, TsTypeMemberList,
};
use rome_rowan::AstNode;
use std::collections::HashMap;

declare_rule! {
    /// Disallow variable, function, class, and type redeclarations in the same scope.
    ///
    /// Source: https://typescript-eslint.io/rules/no-redeclare
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 3;
    /// var a = 10;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let a = 3;
    /// let a = 10;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f() {}
    /// function f() {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class C {
    ///     static {
    ///         var c = 3;
    ///         var c = 10;
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type Person = { name: string; }
    /// class Person { name: string; }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var a = 3;
    /// a = 10;
    /// ```
    ///
    /// ```ts
    /// class Foo {
    ///     bar(a: A);
    ///     bar(a: A, b: B);
    ///     bar(a: A, b: B) {}
    /// }
    /// ```
    pub(crate) NoRedeclare {
        version: "1.0.0",
        name: "noRedeclare",
        recommended: true,
    }
}

#[derive(Debug)]
pub(crate) struct Redeclaration {
    name: String,
    declaration: TextRange,
    redeclaration: TextRange,
}

impl Rule for NoRedeclare {
    type Query = SemanticServices;
    type State = Redeclaration;
    type Signals = Vec<Redeclaration>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut redeclarations = Vec::default();
        for scope in ctx.query().scopes() {
            check_redeclarations_in_single_scope(&scope, &mut redeclarations);
        }
        redeclarations
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let Redeclaration {
            name,
            declaration,
            redeclaration,
        } = state;
        let diag = RuleDiagnostic::new(
            rule_category!(),
            redeclaration,
            markup! {
               "Shouldn't redeclare '"{ name }"'. Consider to delete it or rename it."
            },
        )
        .detail(
            declaration,
            markup! {
               "'"{ name }"' is defined here:"
            },
        );
        Some(diag)
    }
}

fn check_redeclarations_in_single_scope(scope: &Scope, redeclarations: &mut Vec<Redeclaration>) {
    let mut declarations = HashMap::<String, (TextRange, AnyJsBindingDeclaration)>::default();
    for binding in scope.bindings() {
        let id_binding = binding.tree();

        // We consider only binding of a declaration
        // This allows to skip function parameters, methods, ...
        if let Some(decl) = id_binding.declaration() {
            let name = id_binding.text();
            if let Some((first_text_range, first_decl)) = declarations.get(&name) {
                // Do not report:
                // - mergeable declarations.
                //   e.g. a `function` and a `namespace`
                // - when both are parameter-like.
                //   A parameter can override a previous parameter.
                // - when index signature parameters have the different type annotation or are not in the same type member

                if !(first_decl.is_mergeable(&decl)
                    || first_decl.is_parameter_like() && decl.is_parameter_like())
                {
                    match (first_decl, &decl) {
                        (
                            AnyJsBindingDeclaration::TsIndexSignatureParameter(first),
                            AnyJsBindingDeclaration::TsIndexSignatureParameter(second),
                        ) => {
                            if are_index_signature_params_same_type_and_member(first, second) {
                                redeclarations.push(Redeclaration {
                                    name,
                                    declaration: *first_text_range,
                                    redeclaration: id_binding.syntax().text_trimmed_range(),
                                })
                            }
                        }
                        _ => redeclarations.push(Redeclaration {
                            name,
                            declaration: *first_text_range,
                            redeclaration: id_binding.syntax().text_trimmed_range(),
                        }),
                    }
                }
            } else {
                declarations.insert(name, (id_binding.syntax().text_trimmed_range(), decl));
            }
        }
    }
}

/// Checks if the both `TsIndexSignatureParameter` have the same type annotation and are in the same type member
fn are_index_signature_params_same_type_and_member(
    first: &TsIndexSignatureParameter,
    second: &TsIndexSignatureParameter,
) -> bool {
    let are_same_index_signature_type_annotations =
        are_same_index_signature_type_annotations(first, second);
    let (Some(first), Some(second)) = (
        first.parent::<TsIndexSignatureTypeMember>(),
        second.parent::<TsIndexSignatureTypeMember>(),
    ) else {
        return false;
    };
    are_same_index_signature_type_annotations.unwrap_or(false)
        && are_same_type_members(&first, &second).unwrap_or(false)
}

/// Checks if the both `TsIndexSignatureParameter` have the same type annotation
fn are_same_index_signature_type_annotations(
    first: &TsIndexSignatureParameter,
    second: &TsIndexSignatureParameter,
) -> Option<bool> {
    let first_ts_type = first.type_annotation().ok()?.ty().ok()?;
    let second_ts_type = second.type_annotation().ok()?.ty().ok()?;
    match (first_ts_type, second_ts_type) {
        (AnyTsType::TsStringType(_), AnyTsType::TsStringType(_)) => Some(true),
        (AnyTsType::TsNumberType(_), AnyTsType::TsNumberType(_)) => Some(true),
        (AnyTsType::TsSymbolType(_), AnyTsType::TsSymbolType(_)) => Some(true),
        _ => None,
    }
}

fn are_same_type_members(
    first: &TsIndexSignatureTypeMember,
    second: &TsIndexSignatureTypeMember,
) -> Option<bool> {
    let first_parent = first.parent::<TsTypeMemberList>()?;
    let second_parent = second.parent::<TsTypeMemberList>()?;
    Some(first_parent == second_parent)
}
