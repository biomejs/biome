use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsExpression, JsCallExpression, JsFileSource, JsImport,
    JsVariableDeclarator,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TokenText};
use biome_rule_options::no_svelte_unnecessary_state_wrap::NoSvelteUnnecessaryStateWrapOptions;

use crate::JsRuleAction;
use crate::services::semantic::Semantic;

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
    ///
    /// #### Invalid
    ///
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
        version: "next",
        name: "noSvelteUnnecessaryStateWrap",
        language: "js",
        domains: &[RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("no-unnecessary-state-wrap").same()],
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

/// The name of the reactive class being unnecessarily wrapped.
pub type RuleState = TokenText;

impl Rule for NoSvelteUnnecessaryStateWrap {
    type Query = Semantic<JsCallExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoSvelteUnnecessaryStateWrapOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();

        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_svelte()
        {
            return None;
        }

        // Callee must be the `$state` rune.
        let callee_ident = call.callee().ok()?.as_js_identifier_expression()?;
        if callee_ident.name().ok()?.value_token().ok()?.text_trimmed() != "$state" {
            return None;
        }

        // Must be called with exactly one argument.
        let args = call.arguments().ok()?.args();
        if args.len() != 1 {
            return None;
        }
        let arg_expr = args.first()?.ok()?.as_any_js_expression()?.clone();

        // Argument must be `new X(...)` or `X(...)`.
        let class_callee = match &arg_expr {
            AnyJsExpression::JsNewExpression(new_expr) => new_expr.callee().ok()?,
            AnyJsExpression::JsCallExpression(inner_call) => inner_call.callee().ok()?,
            _ => return None,
        };
        let class_ident = class_callee.as_js_identifier_expression()?;
        let class_ref = class_ident.name().ok()?;
        let class_name_text = class_ref.value_token().ok()?.token_text_trimmed();

        let is_builtin = REACTIVE_CLASSES.contains(&class_name_text.text());
        let is_additional = ctx
            .options()
            .additional_reactive_classes()
            .iter()
            .any(|c| c.as_ref() == class_name_text.text());

        if !is_builtin && !is_additional {
            return None;
        }

        // For built-in classes, verify the class is imported from `svelte/reactivity`.
        // We only check static ES module imports (`import { X } from "..."`), which is
        // the only realistic pattern in Svelte component files. Dynamic `import()` and
        // CommonJS `require()` are not handled.
        if is_builtin {
            let binding = model.binding(&class_ref)?;
            let imported_from = binding
                .syntax()
                .ancestors()
                .skip(1)
                .find_map(JsImport::cast)
                .and_then(|import| import.source_text().ok());
            match imported_from {
                Some(source) if source.text() == SVELTE_REACTIVITY_MODULE => {}
                _ => return None,
            }
        }

        Some(class_name_text)
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
        let model = ctx.model();

        // When allowReassign is enabled, withhold the fix for variables that are reassigned after
        // declaration — removing $state would break reactivity for those reference changes.
        if ctx.options().allow_reassign()
            && let Some(declarator) = call
                .syntax()
                .ancestors()
                .skip(1)
                .find_map(JsVariableDeclarator::cast)
            && let Ok(AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
                id_binding,
            ))) = declarator.id()
            && id_binding.all_writes(model).next().is_some()
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
