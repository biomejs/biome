use crate::{
    services::{control_flow::AnyJsControlFlowRoot, semantic::Semantic},
    JsRuleAction,
};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;

use biome_js_factory::make;
use biome_js_semantic::{ReferencesExtensions, Scope, SemanticModel, SemanticScopeExtensions};
use biome_js_syntax::*;
use biome_rowan::{declare_node_union, AstNode, BatchMutationExt};

declare_lint_rule! {
    /// Require `const` declarations for variables that are only assigned once.
    ///
    /// Variables that are initialized and never reassigned and
    /// variables that are only assigned once can be declared as `const`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let a = 3;
    /// console.log(a);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // `a` is redefined (not reassigned) on each loop step.
    /// for (let a of [1, 2, 3]) {
    ///     console.log(a);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // `a` is redefined (not reassigned) on each loop step.
    /// for (let a in [1, 2, 3]) {
    ///     console.log(a);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let a;
    /// a = 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let a = 3;
    /// {
    ///     let a = 4;
    ///     a = 2;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let a = 2;
    /// a = 3;
    /// console.log(a);
    /// ```
    ///
    /// ```js
    /// let a = 1, b = 2;
    /// b = 3;
    /// ```
    ///
    /// ```js
    /// let a;
    /// a; // the variable is read before its assignement
    /// a = 0;
    /// ```
    pub UseConst {
        version: "1.0.0",
        name: "useConst",
        language: "js",
        sources: &[RuleSource::Eslint("prefer-const")],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseConst {
    type Query = Semantic<AnyJsVariableDeclaration>;
    type State = ConstBindings;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let declaration = ctx.query();
        let model = ctx.model();

        // Not a let declaration or inside a for-loop init
        if !declaration.is_let() || declaration.parent::<JsForStatement>().is_some() {
            return None;
        }

        ConstBindings::new(declaration, model)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let declaration = ctx.query();
        let kind = declaration.kind_token().ok()?;
        let title_end = if state.can_be_const.len() == 1 {
            "a variable that is only assigned once."
        } else {
            "some variables that are only assigned once."
        };
        let mut diag = RuleDiagnostic::new(
            rule_category!(),
            kind.text_trimmed_range(),
            markup! {
                "This "<Emphasis>"let"</Emphasis>" declares "{title_end}
            },
        );

        for binding in state.can_be_const.iter() {
            let binding_name = binding.name_token().ok()?;
            if let Some(write) = binding.all_writes(ctx.model()).next() {
                diag = diag.detail(
                    write.syntax().text_trimmed_range(),
                    markup! {
                        "'"{ binding_name.text_trimmed() }"' is only assigned here."
                    },
                );
            } else {
                diag = diag.detail(
                    binding_name.text_trimmed_range(),
                    markup! {
                        "'"{ binding_name.text_trimmed() }"' is never reassigned."
                    },
                );
            }
        }

        Some(diag)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let declaration = ctx.query();
        if state.can_fix {
            let mut batch = ctx.root().begin();
            batch.replace_token(
                declaration.kind_token().ok()?,
                make::token(JsSyntaxKind::CONST_KW),
            );
            Some(JsRuleAction::new(
                ctx.metadata().action_category(ctx.category(), ctx.group()),
                ctx.metadata().applicability(),
                markup! { "Use "<Emphasis>"const"</Emphasis>" instead." }.to_owned(),
                batch,
            ))
        } else {
            None
        }
    }
}

pub struct ConstBindings {
    pub can_be_const: Vec<JsIdentifierBinding>,
    pub can_fix: bool,
}

enum ConstCheckResult {
    Fix,
    Report,
}

impl ConstBindings {
    pub fn new(declaration: &AnyJsVariableDeclaration, model: &SemanticModel) -> Option<Self> {
        let mut state = Self {
            can_be_const: Vec::new(),
            can_fix: true,
        };
        let in_for_in_or_of_loop = matches!(
            declaration,
            AnyJsVariableDeclaration::JsForVariableDeclaration(..)
        );
        let mut bindings = 0;
        for_each_binding_of(declaration, |binding, declarator| {
            bindings += 1;

            let has_initializer = declarator.initializer().is_some();
            let fix =
                check_binding_can_be_const(&binding, in_for_in_or_of_loop, has_initializer, model);
            match fix {
                Some(ConstCheckResult::Fix) => state.can_be_const.push(binding),
                Some(ConstCheckResult::Report) => {
                    state.can_be_const.push(binding);
                    state.can_fix = false;
                }
                None => state.can_fix = false,
            }
        });

        // Only flag if all bindings can be const
        (state.can_be_const.len() == bindings).then_some(state)
    }
}

/// Check if a binding can be const
fn check_binding_can_be_const(
    binding: &JsIdentifierBinding,
    in_for_in_or_of_loop: bool,
    has_initializer: bool,
    model: &SemanticModel,
) -> Option<ConstCheckResult> {
    let mut writes = binding.all_writes(model);

    // In a for-in or for-of loop or if it has an initializer
    if in_for_in_or_of_loop || has_initializer {
        return writes.next().is_none().then_some(ConstCheckResult::Fix);
    }

    let binding_scope = binding.scope(model);
    let write = writes.next()?;
    // If teher are multiple assignement or the write is not in the same scope
    if writes.next().is_some() || write.scope() != binding_scope {
        return None;
    }
    let host = write
        .syntax()
        .ancestors()
        .find_map(DestructuringHost::cast)?;
    if host.has_member_expr_assignment() || host.has_outer_variables(&write.scope()) {
        return None;
    }

    let mut refs = binding.all_references(model);
    // If a read precedes the write, don't report it.
    // Ignore reads that are in an inner control flow root.
    // For example, this ignores reads inside a function:
    // ```js
    // let v;
    // function f() { v; }
    // ```
    let next_ref = refs.find(|x| {
        x.is_write()
            || !x
                .scope()
                .ancestors()
                .take_while(|scope| scope != &binding_scope)
                .any(|scope| AnyJsControlFlowRoot::can_cast(scope.syntax().kind()))
    });
    if matches!(next_ref, Some(next_ref) if next_ref.is_read()) {
        return None;
    }

    host.can_become_variable_declaration()?
        .then_some(ConstCheckResult::Report)
}

fn for_each_declarator_of(decl: &AnyJsVariableDeclaration, f: impl FnMut(JsVariableDeclarator)) {
    match decl {
        AnyJsVariableDeclaration::JsVariableDeclaration(x) => x
            .declarators()
            .into_iter()
            .filter_map(Result::ok)
            .for_each(f),
        AnyJsVariableDeclaration::JsForVariableDeclaration(x) => {
            x.declarator().into_iter().for_each(f)
        }
    }
}

fn for_each_binding_of(
    decl: &AnyJsVariableDeclaration,
    mut f: impl FnMut(JsIdentifierBinding, &JsVariableDeclarator),
) {
    for_each_declarator_of(decl, |declarator| {
        if let Ok(pattern) = declarator.id() {
            with_binding_pat_identifiers(pattern, &mut |binding| {
                f(binding, &declarator);
                false
            });
        }
    });
}

/// Visit [JsIdentifierBinding] in the given [JsAnyBindingPattern].
///
/// Traversal stops if the given function returns true.
pub(crate) fn with_binding_pat_identifiers(
    pat: AnyJsBindingPattern,
    f: &mut impl FnMut(JsIdentifierBinding) -> bool,
) -> bool {
    match pat {
        AnyJsBindingPattern::AnyJsBinding(id) => with_binding_identifier(id, f),
        AnyJsBindingPattern::JsArrayBindingPattern(p) => with_array_binding_pat_identifiers(&p, f),
        AnyJsBindingPattern::JsObjectBindingPattern(p) => {
            with_object_binding_pat_identifiers(&p, f)
        }
    }
}

fn with_object_binding_pat_identifiers(
    pat: &JsObjectBindingPattern,
    f: &mut impl FnMut(JsIdentifierBinding) -> bool,
) -> bool {
    pat.properties()
        .into_iter()
        .filter_map(Result::ok)
        .any(|it| {
            use AnyJsObjectBindingPatternMember as P;
            match it {
                P::JsObjectBindingPatternProperty(p) => p
                    .pattern()
                    .is_ok_and(|it| with_binding_pat_identifiers(it, f)),
                P::JsObjectBindingPatternRest(p) => {
                    p.binding().is_ok_and(|it| with_binding_identifier(it, f))
                }
                P::JsObjectBindingPatternShorthandProperty(p) => p
                    .identifier()
                    .is_ok_and(|it| with_binding_identifier(it, f)),
                P::JsBogusBinding(_) | P::JsMetavariable(_) => false,
            }
        })
}

fn with_array_binding_pat_identifiers(
    pat: &JsArrayBindingPattern,
    f: &mut impl FnMut(JsIdentifierBinding) -> bool,
) -> bool {
    pat.elements().into_iter().filter_map(Result::ok).any(|it| {
        use AnyJsArrayBindingPatternElement as P;
        match it {
            P::JsArrayBindingPatternRestElement(p) => p
                .pattern()
                .is_ok_and(|it| with_binding_pat_identifiers(it, f)),
            P::JsArrayHole(_) => false,
            P::JsArrayBindingPatternElement(p) => p
                .pattern()
                .is_ok_and(|it| with_binding_pat_identifiers(it, f)),
        }
    })
}

fn with_binding_identifier(
    binding: AnyJsBinding,
    f: &mut impl FnMut(JsIdentifierBinding) -> bool,
) -> bool {
    match binding {
        AnyJsBinding::JsIdentifierBinding(id) => f(id),
        AnyJsBinding::JsBogusBinding(_) | AnyJsBinding::JsMetavariable(_) => false,
    }
}

declare_node_union! {
    pub DestructuringHost = JsVariableDeclarator | JsAssignmentExpression
}

impl DestructuringHost {
    fn can_become_variable_declaration(&self) -> Option<bool> {
        match self {
            Self::JsVariableDeclarator(_) => Some(true),
            Self::JsAssignmentExpression(e) => {
                let mut parent = e.syntax().parent()?;
                while parent.kind() == JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION {
                    parent = parent.parent()?;
                }

                if parent.kind() == JsSyntaxKind::JS_EXPRESSION_STATEMENT {
                    parent = parent.parent()?;
                    Some(
                        parent.kind() == JsSyntaxKind::JS_STATEMENT_LIST
                            || parent.kind() == JsSyntaxKind::JS_MODULE_ITEM_LIST,
                    )
                } else {
                    // example: while(a = b) {}
                    None
                }
            }
        }
    }

    fn has_member_expr_assignment(&self) -> bool {
        match self {
            Self::JsAssignmentExpression(it) => it.left().is_ok_and(has_member_expr_in_assign_pat),
            _ => false,
        }
    }

    fn has_outer_variables(&self, scope: &Scope) -> bool {
        match self {
            Self::JsVariableDeclarator(it) => it
                .id()
                .is_ok_and(|pat| has_outer_variables_in_binding_pat(pat, scope)),
            Self::JsAssignmentExpression(it) => it
                .left()
                .is_ok_and(|pat| has_outer_variables_in_assign_pat(&pat, scope)),
        }
    }
}

fn has_outer_variables_in_binding_pat(pat: AnyJsBindingPattern, scope: &Scope) -> bool {
    with_binding_pat_identifiers(pat, &mut |it| is_outer_variable_in_binding(&it, scope))
}

fn is_outer_variable_in_binding(binding: &JsIdentifierBinding, scope: &Scope) -> bool {
    binding
        .name_token()
        .is_ok_and(|name| is_binding_in_outer_scopes(scope, &name))
}

fn has_member_expr_in_assign_pat(pat: AnyJsAssignmentPattern) -> bool {
    use AnyJsAssignmentPattern as P;
    match pat {
        P::AnyJsAssignment(p) => is_member_expr_assignment(p),
        P::JsArrayAssignmentPattern(p) => has_member_expr_in_array_pat(&p),
        P::JsObjectAssignmentPattern(p) => has_member_expr_in_object_assign_pat(&p),
    }
}

fn has_member_expr_in_object_assign_pat(pat: &JsObjectAssignmentPattern) -> bool {
    pat.properties()
        .into_iter()
        .filter_map(Result::ok)
        .any(|it| {
            use AnyJsObjectAssignmentPatternMember as P;
            match it {
                P::JsObjectAssignmentPatternProperty(p) => {
                    p.pattern().is_ok_and(has_member_expr_in_assign_pat)
                }
                P::JsObjectAssignmentPatternRest(p) => {
                    p.target().is_ok_and(is_member_expr_assignment)
                }
                P::JsObjectAssignmentPatternShorthandProperty(_) | P::JsBogusAssignment(_) => false,
            }
        })
}

fn has_member_expr_in_array_pat(pat: &JsArrayAssignmentPattern) -> bool {
    pat.elements()
        .into_iter()
        .filter_map(Result::ok)
        .any(|it| it.pattern().is_some_and(has_member_expr_in_assign_pat))
}

fn is_member_expr_assignment(mut assignment: AnyJsAssignment) -> bool {
    use AnyJsAssignment::*;
    while let JsParenthesizedAssignment(p) = assignment {
        if let Ok(p) = p.assignment() {
            assignment = p
        } else {
            return false;
        }
    }
    matches!(
        assignment,
        JsComputedMemberAssignment(_) | JsStaticMemberAssignment(_)
    )
}

fn has_outer_variables_in_assign_pat(pat: &AnyJsAssignmentPattern, scope: &Scope) -> bool {
    use AnyJsAssignmentPattern as P;
    match pat {
        P::AnyJsAssignment(p) => is_outer_variable_in_assignment(p, scope),
        P::JsArrayAssignmentPattern(p) => has_outer_variables_in_object_assign_pat(p, scope),
        P::JsObjectAssignmentPattern(p) => has_outer_variables_in_array_assign_pat(p, scope),
    }
}

fn has_outer_variables_in_array_assign_pat(pat: &JsObjectAssignmentPattern, scope: &Scope) -> bool {
    pat.properties()
        .into_iter()
        .filter_map(Result::ok)
        .any(|it| {
            use AnyJsObjectAssignmentPatternMember as P;
            match it {
                P::JsObjectAssignmentPatternProperty(p) => p
                    .pattern()
                    .is_ok_and(|it| has_outer_variables_in_assign_pat(&it, scope)),
                P::JsObjectAssignmentPatternRest(p) => p
                    .target()
                    .is_ok_and(|it| is_outer_variable_in_assignment(&it, scope)),
                P::JsObjectAssignmentPatternShorthandProperty(p) => p
                    .identifier()
                    .is_ok_and(|it| is_outer_ident_in_assignment(&it, scope)),
                P::JsBogusAssignment(_) => false,
            }
        })
}

fn has_outer_variables_in_object_assign_pat(pat: &JsArrayAssignmentPattern, scope: &Scope) -> bool {
    pat.elements().into_iter().filter_map(Result::ok).any(|it| {
        it.pattern()
            .is_some_and(|p| has_outer_variables_in_assign_pat(&p, scope))
    })
}

fn is_outer_variable_in_assignment(e: &AnyJsAssignment, scope: &Scope) -> bool {
    match e {
        AnyJsAssignment::JsIdentifierAssignment(it) => is_outer_ident_in_assignment(it, scope),
        _ => false,
    }
}

fn is_outer_ident_in_assignment(assignment: &JsIdentifierAssignment, scope: &Scope) -> bool {
    assignment
        .name_token()
        .is_ok_and(|name| is_binding_in_outer_scopes(scope, &name))
}

fn is_binding_in_outer_scopes(scope: &Scope, name: &JsSyntaxToken) -> bool {
    let text = name.text_trimmed();
    scope
        .ancestors()
        .skip(1) // Skip current scope
        .any(|scope| scope.get_binding(text).is_some())
}
