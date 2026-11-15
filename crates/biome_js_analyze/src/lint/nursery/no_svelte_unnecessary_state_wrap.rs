use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsReferenceIdentifier};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TextSize, TokenText};
use biome_rule_options::no_svelte_unnecessary_state_wrap::NoSvelteUnnecessaryStateWrapOptions;
use biome_svelte_semantic::{Imported, Rune, RuneCall};

use crate::JsRuleAction;
use crate::services::svelte::{SvelteSemantic, SvelteSemanticServices};

/// Classes from `svelte/reactivity` that are already reactive without `$state`.
const REACTIVE_CLASSES: &[&str] = &[
    "SvelteSet",
    "SvelteMap",
    "SvelteURL",
    "SvelteURLSearchParams",
    "SvelteDate",
    "MediaQuery",
];

const SVELTE_REACTIVITY_MODULE: &str = "svelte/reactivity";

declare_lint_rule! {
    /// Disallow unnecessary `$state` wrapping of reactive classes.
    ///
    /// Several classes exported from `svelte/reactivity` — such as `SvelteMap`, `SvelteSet`, and
    /// `SvelteDate` — are already deeply reactive without the `$state` rune. Wrapping them in
    /// `$state(...)` is redundant and may mislead readers into thinking the reactivity comes from
    /// the rune rather than the class itself.
    ///
    /// Use the `additionalReactiveClasses` option to extend this list with custom reactive classes
    /// from your own codebase.
    ///
    /// Use `allowReassign: true` if you need to reassign the variable itself after declaration,
    /// which requires `$state` to track the reference change.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// import { SvelteMap } from "svelte/reactivity";
    /// const map = $state(new SvelteMap());
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// <script>
    /// import { SvelteMap } from "svelte/reactivity";
    /// const map = new SvelteMap();
    /// </script>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `allowReassign`
    ///
    /// When `true`, suppresses the autofix for variables that are reassigned after declaration.
    /// Because reassigning a `$state`-wrapped value changes the binding itself, removing `$state`
    /// would break reactivity for those reassignments. The diagnostic still fires — only the
    /// unsafe autofix is withheld.
    ///
    /// Default: `false`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "allowReassign": true
    ///   }
    /// }
    /// ```
    /// ```svelte,expect_diagnostic,use_options
    /// <script>
    /// import { SvelteMap } from "svelte/reactivity";
    /// const map = $state(new SvelteMap());
    /// </script>
    /// ```
    ///
    /// ```svelte,expect_diagnostic,use_options
    /// <script>
    /// import { SvelteMap } from "svelte/reactivity";
    /// let map = $state(new SvelteMap());
    /// map = new SvelteMap();
    /// </script>
    /// ```
    ///
    /// ### `additionalReactiveClasses`
    ///
    /// An array of additional class names to treat as already reactive (beyond the built-in
    /// `svelte/reactivity` classes). Use this to extend the rule with custom reactive classes
    /// from your own codebase.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "additionalReactiveClasses": ["MyReactiveStore"]
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```svelte,expect_diagnostic,use_options
    /// <script>
    /// const store = $state(new MyReactiveStore());
    /// </script>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```svelte,use_options
    /// <script>
    /// const store = new MyReactiveStore();
    /// </script>
    /// ```
    ///
    pub NoSvelteUnnecessaryStateWrap {
        version: "2.5.2",
        name: "noSvelteUnnecessaryStateWrap",
        language: "js",
        domains: &[RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("no-unnecessary-state-wrap").same()],
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoSvelteUnnecessaryStateWrap {
    type Query = SvelteSemantic<JsCallExpression>;
    type State = TokenText;
    type Signals = Option<Self::State>;
    type Options = NoSvelteUnnecessaryStateWrapOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let svelte = ctx.svelte_model();
        let offset = snippet_offset(ctx);

        // The model records the calls of runes, which nothing in the file can shadow, and
        // `state_call` keeps those of `$state`.
        let rune_call = state_call(ctx, call)?;
        if rune_call.args() != 1 {
            return None;
        }
        let argument = call
            .arguments()
            .ok()?
            .args()
            .first()?
            .ok()?
            .as_any_js_expression()?
            .clone();

        // Argument must be `new X(...)` or `X(...)`, where `X` may be read from a namespace, as
        // in `new reactivity.SvelteMap()`.
        let class_callee = match argument.omit_parentheses() {
            AnyJsExpression::JsNewExpression(new_expr) => new_expr.callee().ok()?,
            AnyJsExpression::JsCallExpression(inner_call) => inner_call.callee().ok()?,
            _ => return None,
        };
        let (base, members) = match class_callee.clone().omit_parentheses() {
            AnyJsExpression::JsIdentifierExpression(identifier) => (identifier.name().ok()?, 0),
            AnyJsExpression::JsStaticMemberExpression(member) => {
                let object = member.object().ok()?.omit_parentheses();
                (object.as_js_identifier_expression()?.name().ok()?, 1)
            }
            _ => return None,
        };

        let name = class_name(&base, members, &class_callee)?;

        // A named import must be the callee itself, so that `SvelteDate.now()` isn't taken for
        // the class. A namespace import must be read for exactly one member.
        let reference = svelte.reference_at(base.range() + offset)?;
        let expected_members = match reference.symbol().as_ref().and_then(|symbol| symbol.imported()) {
            Some(Imported::Named(_)) => Some(0),
            Some(Imported::Namespace) => Some(1),
            _ => None,
        };
        if expected_members == Some(members)
            && reference.path().count() == members
            && let Some((SVELTE_REACTIVITY_MODULE, imported)) = reference.imported_member()
            && REACTIVE_CLASSES.contains(&imported)
        {
            return Some(name);
        }

        // Additional classes are matched by name, wherever they are declared.
        ctx.options()
            .additional_reactive_classes()
            .iter()
            .any(|class| class.as_ref() == name.text())
            .then_some(name)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let call = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                call.range(),
                markup! {
                    <Emphasis>{state.text()}</Emphasis>" is already reactive, wrapping it in "<Emphasis>"$state()"</Emphasis>" is unnecessary."
                },
            )
            .note(markup! {
                "Classes from "<Emphasis>"svelte/reactivity"</Emphasis>" track their own mutations without needing a "<Emphasis>"$state"</Emphasis>" wrapper."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();

        // When allowReassign is enabled, withhold the fix for variables that are reassigned after
        // declaration, in the script or in the template, as by `bind:this`. Removing `$state`
        // would break reactivity for those reference changes.
        if ctx.options().allow_reassign()
            && state_call(ctx, call)?
                .symbols()
                .any(|symbol| symbol.is_reassigned())
        {
            return None;
        }

        let arg = call.arguments().ok()?.args().first()?.ok()?;
        let inner_expr = arg.as_any_js_expression()?.clone();

        let mut mutation = ctx.root().begin();
        mutation.replace_node(AnyJsExpression::JsCallExpression(call.clone()), inner_expr);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the unnecessary "<Emphasis>"$state()"</Emphasis>" wrapper." }
                .to_owned(),
            mutation,
        ))
    }
}

/// The position in the host document of the start of the analyzed tree.
fn snippet_offset(services: &SvelteSemanticServices) -> TextSize {
    services
        .snippet()
        .map_or_else(TextSize::default, |snippet| snippet.offset())
}

/// Returns the model's record of `call` when it calls the `$state` rune.
fn state_call(services: &SvelteSemanticServices, call: &JsCallExpression) -> Option<RuneCall> {
    services
        .svelte_model()
        .rune_call_at(call.range() + snippet_offset(services))
        .filter(|rune_call| rune_call.rune() == Rune::State)
}

/// Returns the name of the class a callee names, as written: `X` for `X`, and
/// `X` for `namespace.X`.
fn class_name(
    base: &JsReferenceIdentifier,
    members: usize,
    callee: &AnyJsExpression,
) -> Option<TokenText> {
    if members == 0 {
        return Some(base.value_token().ok()?.token_text_trimmed());
    }
    let member = callee.clone().omit_parentheses();
    let member = member.as_js_static_member_expression()?.member().ok()?;
    Some(member.value_token().ok()?.token_text_trimmed())
}
