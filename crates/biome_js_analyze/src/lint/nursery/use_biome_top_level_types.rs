use crate::{
    frameworks::biome_plugin::{KindCategory, resolve_constant},
    services::semantic::Semantic,
};
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::{
    fmt::{Display, Formatter},
    markup,
};
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsPropertyModifier, JsCallArguments, JsCallExpression,
    JsConstructorClassMember, JsGetterClassMember, JsGetterObjectMember, JsLanguage,
    JsMethodClassMember, JsMethodObjectMember, JsPropertyClassMember, JsSetterClassMember,
    JsSetterObjectMember, JsStaticMemberExpression, JsSyntaxKind,
};
use biome_rowan::{AstNode, AstNodeList, SyntaxKindSet};
use biome_rule_options::use_biome_top_level_types::UseBiomeTopLevelTypesOptions;

declare_lint_rule! {
    /// Require kind sets in Biome JS plugins to be created at the top level of the module.
    ///
    /// In a [Biome JS plugin](https://biomejs.dev/linter/plugins/), `nodeKinds()` and
    /// `tokenKinds()` from `@biomejs/runtime/plugin` create kind sets, and the `union()` method
    /// of a kind set creates a new set from two sets. Creating a set takes time.
    ///
    /// Code inside a function, such as the `run()` function of a rule, can run many times, for
    /// example once for every matching node in every file. Creating the same set again on each
    /// call repeats that work. Create each set once, in a constant at the top level of the
    /// plugin, and reuse it.
    ///
    /// This rule reports calls to `nodeKinds()`, `tokenKinds()`, and `union()` inside functions,
    /// and in class fields that aren't `static`, because those fields are set again for every new
    /// instance of the class. The rule only checks `nodeKinds()` and `tokenKinds()` when they're
    /// imported from `@biomejs/runtime/plugin`, and `union()` when it's called on a kind set that
    /// the rule can see being created.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { defineRule, nodeKinds } from "@biomejs/runtime/plugin";
    ///
    /// export const rule = defineRule({
    ///     run(node) {
    ///         const CALLS = nodeKinds("JS_CALL_EXPRESSION", "JS_NEW_EXPRESSION");
    ///         return CALLS.has(node.parent);
    ///     },
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { nodeKinds } from "@biomejs/runtime/plugin";
    ///
    /// const CALLS = nodeKinds("JS_CALL_EXPRESSION", "JS_NEW_EXPRESSION");
    /// const IMPORTS = nodeKinds("JS_IMPORT_CALL_EXPRESSION");
    ///
    /// function isCallOrImport(node) {
    ///     return CALLS.union(IMPORTS).has(node);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { defineRule, nodeKinds } from "@biomejs/runtime/plugin";
    ///
    /// const CALLS = nodeKinds("JS_CALL_EXPRESSION", "JS_NEW_EXPRESSION");
    /// const CALLS_AND_IMPORTS = CALLS.union(nodeKinds("JS_IMPORT_CALL_EXPRESSION"));
    ///
    /// export const rule = defineRule({
    ///     run(node) {
    ///         return CALLS_AND_IMPORTS.has(node.parent);
    ///     },
    /// });
    /// ```
    ///
    /// ## See Also
    ///
    /// - To use kind sets instead of comparing kind names one by one, see [`useBiomeKindSets`](https://biomejs.dev/linter/rules/use-biome-kind-sets/).
    ///
    pub UseBiomeTopLevelTypes {
        version: "next",
        name: "useBiomeTopLevelTypes",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::BiomePlugin],
    }
}

/// How a call creates a kind set.
#[derive(Clone, Copy)]
pub enum KindSetCreation {
    /// A call to `nodeKinds()` or `tokenKinds()`.
    Constructor(KindCategory),
    /// A call to the `union()` method of a kind set.
    Union,
}

impl Rule for UseBiomeTopLevelTypes {
    type Query = Semantic<JsCallExpression>;
    type State = KindSetCreation;
    type Signals = Option<Self::State>;
    type Options = UseBiomeTopLevelTypesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();
        let creation = kind_set_creation(call, model)?;
        // A set that only builds part of a union, such as `nodeKinds("A")` in
        // `CALLS.union(nodeKinds("A"))`, is reported as part of that union.
        if is_union_operand(call, model) || !runs_repeatedly(call) {
            return None;
        }
        Some(creation)
    }

    fn diagnostic(ctx: &RuleContext<Self>, creation: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>{creation}"()"</Emphasis>" call creates a new kind set every time it runs."
                },
            )
            .note(markup! {
                "This code can run many times, for example once for every node that the "<Emphasis>"run()"</Emphasis>" function of a rule checks, or once for every new instance of a class, so the same set is created again each time."
            })
            .note(markup! {
                "Create the set once in a top-level constant, and use that constant here."
            }),
        )
    }
}

/// Writes the name of the function or method that creates the set.
impl Display for KindSetCreation {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        match self {
            Self::Constructor(category) => category.fmt(fmt),
            Self::Union => fmt.write_str("union"),
        }
    }
}

/// Returns how `call` creates a kind set, or `None` if it doesn't create one.
fn kind_set_creation(call: &JsCallExpression, model: &SemanticModel) -> Option<KindSetCreation> {
    let callee = call.callee().ok()?;
    if let Some(category) = KindCategory::from_set_constructor(&callee, model) {
        return Some(KindSetCreation::Constructor(category));
    }
    let AnyJsExpression::JsStaticMemberExpression(member) = callee.omit_parentheses() else {
        return None;
    };
    (is_union_member(&member) && is_kind_set(member.object().ok()?, model))
        .then_some(KindSetCreation::Union)
}

/// Returns whether `expression` is a kind set that the rule can see being created: a call to
/// `nodeKinds()`, `tokenKinds()`, or `union()` on another kind set, either directly or
/// through `const` variables.
fn is_kind_set(mut expression: AnyJsExpression, model: &SemanticModel) -> bool {
    // Each step follows one `union()` call to the set it's called on. The limit stops cycles
    // such as `const a = b.union(c), b = a.union(c);`, which parse without errors.
    for _ in 0..MAX_UNION_STEPS {
        let Some(AnyJsExpression::JsCallExpression(call)) = resolve_constant(expression, model)
        else {
            return false;
        };
        let Ok(callee) = call.callee() else {
            return false;
        };
        if KindCategory::from_set_constructor(&callee, model).is_some() {
            return true;
        }
        let AnyJsExpression::JsStaticMemberExpression(member) = callee.omit_parentheses() else {
            return false;
        };
        if !is_union_member(&member) {
            return false;
        }
        let Ok(object) = member.object() else {
            return false;
        };
        expression = object;
    }
    false
}

/// The number of `union()` calls that [`is_kind_set`] follows before giving up.
const MAX_UNION_STEPS: usize = 16;

/// Returns whether `call` is the set that a reported `union()` call is called on, or the set
/// passed to it.
fn is_union_operand(call: &JsCallExpression, model: &SemanticModel) -> bool {
    let Some(parent) = call
        .syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| ancestor.kind() != JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION)
    else {
        return false;
    };
    // `call` creates a kind set, so a `union()` call on it is reported.
    if let Some(member) = JsStaticMemberExpression::cast_ref(&parent) {
        return is_union_member(&member)
            && member.parent::<JsCallExpression>().is_some_and(|union| {
                union
                    .callee()
                    .is_ok_and(|callee| callee.syntax() == member.syntax())
            });
    }
    parent.kind() == JsSyntaxKind::JS_CALL_ARGUMENT_LIST
        && parent
            .parent()
            .and_then(JsCallArguments::cast)
            .and_then(|arguments| arguments.parent::<JsCallExpression>())
            .is_some_and(|union| {
                matches!(
                    kind_set_creation(&union, model),
                    Some(KindSetCreation::Union)
                )
            })
}

/// Returns whether `member` reads a property named `union`.
fn is_union_member(member: &JsStaticMemberExpression) -> bool {
    member
        .member()
        .ok()
        .and_then(|name| name.as_js_name()?.value_token().ok())
        .is_some_and(|name| name.text_trimmed() == "union")
}

/// Returns whether `call` can run more than once each time the module runs: inside a
/// function, or in a class field that isn't `static`.
fn runs_repeatedly(call: &JsCallExpression) -> bool {
    call.syntax().ancestors().skip(1).any(|ancestor| {
        if FUNCTION_KINDS.matches(ancestor.kind()) {
            return true;
        }
        JsPropertyClassMember::cast(ancestor).is_some_and(|property| {
            !property
                .modifiers()
                .iter()
                .any(|modifier| matches!(modifier, AnyJsPropertyModifier::JsStaticModifier(_)))
        })
    })
}

/// The nodes whose code runs each time they're called rather than once.
const FUNCTION_KINDS: SyntaxKindSet<JsLanguage> = AnyJsFunction::KIND_SET
    .union(JsMethodClassMember::KIND_SET)
    .union(JsMethodObjectMember::KIND_SET)
    .union(JsGetterClassMember::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterClassMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET)
    .union(JsConstructorClassMember::KIND_SET);
