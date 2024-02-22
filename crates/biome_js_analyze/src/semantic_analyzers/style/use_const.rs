use crate::{semantic_services::Semantic, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;

use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_semantic::{ReferencesExtensions, Scope, SemanticModel, SemanticScopeExtensions};
use biome_js_syntax::*;
use biome_rowan::{declare_node_union, AstNode, BatchMutationExt};

declare_rule! {
    /// Require `const` declarations for variables that are never reassigned after declared.
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
    pub UseConst {
        version: "1.0.0",
        name: "useConst",
        source: RuleSource::Eslint("prefer-const"),
        recommended: true,
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
            "a variable which is never re-assigned."
        } else {
            "some variables which are never re-assigned."
        };
        let mut diag = RuleDiagnostic::new(
            rule_category!(),
            kind.text_trimmed_range(),
            markup! {
                "This "<Emphasis>"let"</Emphasis>" declares "{title_end}
            },
        );

        for binding in state.can_be_const.iter() {
            let binding = binding.name_token().ok()?;
            diag = diag.detail(
                binding.text_trimmed_range(),
                markup! {
                    "'"{ binding.text_trimmed() }"' is never re-assigned."
                },
            );
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
            Some(JsRuleAction {
                category: ActionCategory::QuickFix,
                applicability: Applicability::Always,
                message: markup! { "Use "<Emphasis>"const"</Emphasis>" instead." }.to_owned(),
                mutation: batch,
            })
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

    // If no initializer and one assignment in same scope
    let write = match (writes.next(), writes.next()) {
        (Some(v), None) if v.scope() == binding.scope(model) => v,
        _ => return None,
    };

    let host = write
        .syntax()
        .ancestors()
        .find_map(DestructuringHost::cast)?;
    if host.has_member_expr_assignment() || host.has_outer_variables(&write.scope()) {
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
                    .map_or(false, |it| with_binding_pat_identifiers(it, f)),
                P::JsObjectBindingPatternRest(p) => p
                    .binding()
                    .map_or(false, |it| with_binding_identifier(it, f)),
                P::JsObjectBindingPatternShorthandProperty(p) => p
                    .identifier()
                    .map_or(false, |it| with_binding_identifier(it, f)),
                P::JsBogusBinding(_) => false,
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
                .map_or(false, |it| with_binding_pat_identifiers(it, f)),
            P::JsArrayHole(_) => false,
            P::JsArrayBindingPatternElement(p) => p
                .pattern()
                .map_or(false, |it| with_binding_pat_identifiers(it, f)),
        }
    })
}

fn with_binding_identifier(
    binding: AnyJsBinding,
    f: &mut impl FnMut(JsIdentifierBinding) -> bool,
) -> bool {
    match binding {
        AnyJsBinding::JsIdentifierBinding(id) => f(id),
        AnyJsBinding::JsBogusBinding(_) => false,
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
            Self::JsAssignmentExpression(it) => {
                it.left().map_or(false, has_member_expr_in_assign_pat)
            }
            _ => false,
        }
    }

    fn has_outer_variables(&self, scope: &Scope) -> bool {
        match self {
            Self::JsVariableDeclarator(it) => it
                .id()
                .map_or(false, |pat| has_outer_variables_in_binding_pat(pat, scope)),
            Self::JsAssignmentExpression(it) => it
                .left()
                .map_or(false, |pat| has_outer_variables_in_assign_pat(&pat, scope)),
        }
    }
}

fn has_outer_variables_in_binding_pat(pat: AnyJsBindingPattern, scope: &Scope) -> bool {
    with_binding_pat_identifiers(pat, &mut |it| is_outer_variable_in_binding(&it, scope))
}

fn is_outer_variable_in_binding(binding: &JsIdentifierBinding, scope: &Scope) -> bool {
    binding
        .name_token()
        .map_or(false, |name| is_binding_in_outer_scopes(scope, &name))
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
                    p.pattern().map_or(false, has_member_expr_in_assign_pat)
                }
                P::JsObjectAssignmentPatternRest(p) => {
                    p.target().map_or(false, is_member_expr_assignment)
                }
                P::JsObjectAssignmentPatternShorthandProperty(_) | P::JsBogusAssignment(_) => false,
            }
        })
}

fn has_member_expr_in_array_pat(pat: &JsArrayAssignmentPattern) -> bool {
    pat.elements()
        .into_iter()
        .filter_map(Result::ok)
        .any(|it| it.pattern().map_or(false, has_member_expr_in_assign_pat))
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
                    .map_or(false, |it| has_outer_variables_in_assign_pat(&it, scope)),
                P::JsObjectAssignmentPatternRest(p) => p
                    .target()
                    .map_or(false, |it| is_outer_variable_in_assignment(&it, scope)),
                P::JsObjectAssignmentPatternShorthandProperty(p) => p
                    .identifier()
                    .map_or(false, |it| is_outer_ident_in_assignment(&it, scope)),
                P::JsBogusAssignment(_) => false,
            }
        })
}

fn has_outer_variables_in_object_assign_pat(pat: &JsArrayAssignmentPattern, scope: &Scope) -> bool {
    pat.elements().into_iter().filter_map(Result::ok).any(|it| {
        it.pattern()
            .map_or(false, |p| has_outer_variables_in_assign_pat(&p, scope))
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
        .map_or(false, |name| is_binding_in_outer_scopes(scope, &name))
}

fn is_binding_in_outer_scopes(scope: &Scope, name: &JsSyntaxToken) -> bool {
    let text = name.text_trimmed();
    scope
        .ancestors()
        .skip(1) // Skip current scope
        .any(|scope| scope.get_binding(text).is_some())
}
