use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{Binding, Reference, SemanticModel};
use biome_js_syntax::{
    AnyJsExpression, AnyTsType, JsArrowFunctionExpression, JsBinaryExpression, JsCallExpression,
    JsClassExpression, JsComputedMemberExpression, JsConditionalExpression, JsDoWhileStatement,
    JsForStatement, JsFormalParameter, JsFunctionBody, JsFunctionDeclaration,
    JsFunctionExportDefaultDeclaration, JsFunctionExpression, JsIdentifierAssignment,
    JsInExpression, JsInstanceofExpression, JsLanguage, JsNewExpression, JsParameters,
    JsReferenceIdentifier, JsStaticMemberExpression, JsSyntaxNode, JsTemplateExpression,
    JsWhileStatement, JsYieldExpression, TextRange, binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, SyntaxKindSet, WalkEvent, declare_node_union};
use biome_rule_options::no_unmodified_loop_condition::NoUnmodifiedLoopConditionOptions;
use rustc_hash::{FxHashMap, FxHashSet};

declare_lint_rule! {
    /// Disallow loop conditions whose variables are never modified in the loop.
    ///
    /// A variable in a loop condition usually changes during the loop. If it does not,
    /// the loop may never terminate or may not run as intended.
    ///
    /// Binary and conditional expressions are checked as a group. The condition is
    /// considered modified when any variable in the group changes in the loop.
    /// References inside dynamic expressions, such as function calls and property accesses,
    /// are ignored because their values may change without a local assignment. A binary or
    /// conditional expression containing a dynamic expression is ignored as a group.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let node = getNode();
    /// while (node) {
    ///     process(node);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (let index = 0; index < 5;) {
    ///     process(index);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let node = getNode();
    /// while (node) {
    ///     process(node);
    ///     node = node.parent;
    /// }
    /// ```
    ///
    /// ```js
    /// for (let index = 0; index < items.length; index++) {
    ///     process(items[index]);
    /// }
    /// ```
    ///
    /// ```js
    /// while (object.ready) {
    ///     process(object);
    /// }
    /// ```
    ///
    pub NoUnmodifiedLoopCondition {
        version: "next",
        name: "noUnmodifiedLoopCondition",
        language: "js",
        sources: &[RuleSource::Eslint("no-unmodified-loop-condition").same()],
        recommended: false,
        severity: Severity::Warning,
    }
}

declare_node_union! {
    pub AnyLoopStatement = JsWhileStatement | JsDoWhileStatement | JsForStatement
}

impl Rule for NoUnmodifiedLoopCondition {
    type Query = Semantic<AnyLoopStatement>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoUnmodifiedLoopConditionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let loop_statement = ctx.query();
        let Some(test) = loop_statement.test() else {
            return Box::new([]);
        };
        let model = ctx.model();
        let mut references = condition_references(&test, model);

        // Check each binding once when a condition uses it more than once:
        //
        // ```js
        // while (start < end && start !== 0) {
        //     start++;
        // }
        // ```
        let mut modified_bindings = FxHashMap::default();

        for reference in &mut references {
            let binding_range = reference.binding.syntax().text_trimmed_range();
            reference.modified = *modified_bindings.entry(binding_range).or_insert_with(|| {
                has_initialized_var_in_loop(loop_statement, &reference.binding)
                    || reference
                        .binding
                        .all_writes()
                        .any(|write| has_write_in_loop(loop_statement, &write, model))
            });
        }

        // Binary and conditional expressions are groups. Changing one variable
        // keeps the whole group valid:
        //
        // ```js
        // while (left < right) {
        //     left++;
        // }
        // ```
        let modified_groups = references
            .iter()
            .filter(|reference| reference.modified)
            .filter_map(|reference| reference.group)
            .collect::<FxHashSet<_>>();

        references
            .into_iter()
            .filter(|reference| {
                !reference.modified
                    && reference
                        .group
                        .is_none_or(|group| !modified_groups.contains(&group))
            })
            .map(|reference| reference.range)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This loop condition variable is not modified in the loop."
                },
            )
            .note(markup! {
                "An unchanged condition can make the loop run forever or prevent it from behaving as intended."
            })
            .note(markup! {
                "Update the variable during each iteration, or use a condition whose value can change."
            }),
        )
    }
}

const GROUP_EXPRESSION_KINDS: SyntaxKindSet<JsLanguage> = JsBinaryExpression::KIND_SET
    .union(JsInExpression::KIND_SET)
    .union(JsInstanceofExpression::KIND_SET)
    .union(JsConditionalExpression::KIND_SET);

const DYNAMIC_EXPRESSION_KINDS: SyntaxKindSet<JsLanguage> = JsCallExpression::KIND_SET
    .union(JsStaticMemberExpression::KIND_SET)
    .union(JsComputedMemberExpression::KIND_SET)
    .union(JsNewExpression::KIND_SET)
    .union(JsYieldExpression::KIND_SET);

const SKIPPED_SUBTREE_KINDS: SyntaxKindSet<JsLanguage> = JsFunctionExpression::KIND_SET
    .union(JsArrowFunctionExpression::KIND_SET)
    .union(JsClassExpression::KIND_SET)
    .union(JsFunctionBody::KIND_SET)
    .union(JsParameters::KIND_SET)
    .union(JsFormalParameter::KIND_SET)
    .union(AnyTsType::KIND_SET);

/// One variable use collected from the loop condition.
struct ConditionReference {
    /// Where the variable appears in the condition.
    range: TextRange,
    /// The variable this name resolves to.
    binding: Binding,
    /// The surrounding binary or conditional expression, if any.
    group: Option<TextRange>,
    /// Set when the loop can change the variable.
    modified: bool,
}

/// Bookkeeping for the outermost group being walked.
struct ActiveGroup {
    /// The key shared by every variable in the group.
    range: TextRange,
    /// The number of group expressions that have not been exited yet.
    depth: usize,
    /// Where this group's variables begin in `references`.
    references_start: usize,
    /// Set when a call or property access makes the group unsafe to check.
    is_dynamic: bool,
}

impl AnyLoopStatement {
    fn test(&self) -> Option<AnyJsExpression> {
        match self {
            Self::JsWhileStatement(statement) => statement.test().ok(),
            Self::JsDoWhileStatement(statement) => statement.test().ok(),
            Self::JsForStatement(statement) => statement.test(),
        }
    }

    fn contains_reference(&self, reference: &Reference) -> bool {
        self.contains_range(reference.syntax().text_trimmed_range())
    }

    fn contains_range(&self, range: TextRange) -> bool {
        if !self.syntax().text_trimmed_range().contains_range(range) {
            return false;
        }

        match self {
            // The initializer runs before the loop, not during an iteration:
            //
            // ```js
            // for (index = 0; index < 5;) {
            //     process(index);
            // }
            // ```
            Self::JsForStatement(statement) => statement.initializer().is_none_or(|initializer| {
                !initializer.syntax().text_trimmed_range().contains_range(range)
            }),
            Self::JsWhileStatement(_) | Self::JsDoWhileStatement(_) => true,
        }
    }
}

/// Collects bound identifiers from a loop condition, excluding dynamic and non-runtime
/// subtrees and tracking their binary or conditional expression groups.
fn condition_references(
    test: &AnyJsExpression,
    model: &SemanticModel,
) -> Vec<ConditionReference> {
    let mut references = Vec::new();
    let mut active_group: Option<ActiveGroup> = None;
    let mut preorder = test.syntax().preorder();

    while let Some(event) = preorder.next() {
        match event {
            WalkEvent::Enter(node) => {
                if is_group_expression(&node) {
                    if let Some(group) = &mut active_group {
                        group.depth = group.depth.saturating_add(1);
                    } else {
                        active_group = Some(ActiveGroup {
                            range: node.text_trimmed_range(),
                            depth: 1,
                            references_start: references.len(),
                            is_dynamic: false,
                        });
                    }
                }

                if is_dynamic_expression(&node) {
                    if let Some(group) = &mut active_group {
                        // A call may change without a local write, so drop the whole group:
                        //
                        // ```js
                        // while (index < getLimit()) {
                        //     process(index);
                        // }
                        // ```
                        group.is_dynamic = true;
                    }
                    preorder.skip_subtree();
                    continue;
                }
                if is_skipped_subtree(&node) {
                    preorder.skip_subtree();
                    continue;
                }

                let (range, binding) = if let Some(identifier) =
                    JsReferenceIdentifier::cast_ref(&node)
                {
                    let Some(binding) = model.binding(&identifier) else {
                        continue;
                    };
                    (identifier.range(), binding)
                } else if let Some(identifier) = JsIdentifierAssignment::cast_ref(&node) {
                    let Some(binding) = model.binding(&identifier) else {
                        continue;
                    };
                    (identifier.range(), binding)
                } else {
                    continue;
                };

                references.push(ConditionReference {
                    range,
                    binding,
                    group: active_group.as_ref().map(|group| group.range),
                    modified: false,
                });
            }
            WalkEvent::Leave(node) => {
                if is_group_expression(&node) {
                    let Some(mut group) = active_group.take() else {
                        continue;
                    };
                    group.depth = group.depth.saturating_sub(1);
                    if group.depth == 0 {
                        if group.is_dynamic {
                            // Remove variables collected before the dynamic expression
                            // was found in this group.
                            references.truncate(group.references_start);
                        }
                    } else {
                        active_group = Some(group);
                    }
                }
            }
        }
    }

    references
}

fn is_group_expression(node: &JsSyntaxNode) -> bool {
    GROUP_EXPRESSION_KINDS.matches(node.kind())
}

fn is_dynamic_expression(node: &JsSyntaxNode) -> bool {
    DYNAMIC_EXPRESSION_KINDS.matches(node.kind())
        || JsTemplateExpression::cast_ref(node)
            .is_some_and(|template| template.tag().is_some())
}

fn is_skipped_subtree(node: &JsSyntaxNode) -> bool {
    SKIPPED_SUBTREE_KINDS.matches(node.kind())
}

fn has_initialized_var_in_loop(
    loop_statement: &AnyLoopStatement,
    binding: &Binding,
) -> bool {
    let Some(declaration) = binding.tree().declaration() else {
        return false;
    };
    let declaration = declaration
        .parent_binding_pattern_declaration()
        .unwrap_or(declaration);
    let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = declaration else {
        return false;
    };

    declarator.initializer().is_some()
        && declarator.declaration().is_some_and(|declaration| declaration.is_var())
        && loop_statement.contains_range(declarator.range())
}

fn has_write_in_loop(
    loop_statement: &AnyLoopStatement,
    write: &Reference,
    model: &SemanticModel,
) -> bool {
    if loop_statement.contains_reference(write) {
        return true;
    }

    // The write may live outside the loop in a named function used by the loop:
    //
    // ```js
    // function finish() {
    //     running = false;
    // }
    // while (running) {
    //     finish();
    // }
    // ```
    let Some(identifier) = write.syntax().ancestors().skip(1).find_map(|node| {
        if let Some(function) = JsFunctionDeclaration::cast_ref(&node) {
            return function
                .id()
                .ok()
                .and_then(|binding| binding.as_js_identifier_binding().cloned());
        }
        JsFunctionExportDefaultDeclaration::cast_ref(&node)
            .and_then(|function| function.id())
            .and_then(|binding| binding.as_js_identifier_binding().cloned())
    })
    else {
        return false;
    };

    model
        .as_binding(&identifier)
        .all_references()
        .filter(|reference| {
            // A type reference does not run the function:
            //
            // ```js
            // type Finish = typeof finish;
            // ```
            !reference
                .syntax()
                .ancestors()
                .skip(1)
                .any(|node| AnyTsType::can_cast(node.kind()))
        })
        .any(|reference| loop_statement.contains_reference(&reference))
}
