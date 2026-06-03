use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument, AnyJsExpression, JsCallExpression,
    JsFileSource, JsImport, JsVariableDeclarator,
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

pub struct RuleState {
    /// Name of the reactive class being unnecessarily wrapped.
    class_name: TokenText,
    /// The inner expression (the `new X()` / `X()` argument) to replace the call with.
    inner_expr: AnyJsExpression,
}

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
        let AnyJsExpression::JsIdentifierExpression(callee_ident) = call.callee().ok()? else {
            return None;
        };
        if callee_ident.name().ok()?.value_token().ok()?.text_trimmed() != "$state" {
            return None;
        }

        // Must be called with exactly one argument.
        let args = call.arguments().ok()?.args();
        if args.len() != 1 {
            return None;
        }
        let AnyJsCallArgument::AnyJsExpression(arg_expr) = args.first()?.ok()? else {
            return None;
        };

        // Argument must be `new X(...)` or `X(...)`.
        let class_callee = match &arg_expr {
            AnyJsExpression::JsNewExpression(new_expr) => new_expr.callee().ok()?,
            AnyJsExpression::JsCallExpression(inner_call) => inner_call.callee().ok()?,
            _ => return None,
        };
        let AnyJsExpression::JsIdentifierExpression(class_ident) = class_callee else {
            return None;
        };
        let class_ref = class_ident.name().ok()?;
        let class_name_text = class_ref.value_token().ok()?.token_text_trimmed();

        let options = ctx.options();

        let is_builtin = REACTIVE_CLASSES.contains(&class_name_text.text());
        let is_additional = options
            .additional_reactive_classes
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .any(|c| c.as_ref() == class_name_text.text());

        if !is_builtin && !is_additional {
            return None;
        }

        // For built-in classes, verify the class is imported from `svelte/reactivity`.
        if is_builtin {
            let binding = model.binding(&class_ref)?;
            let imported_from = binding
                .syntax()
                .ancestors()
                .find_map(JsImport::cast)
                .and_then(|import| import.source_text().ok());
            match imported_from {
                Some(source) if source.text() == SVELTE_REACTIVITY_MODULE => {}
                _ => return None,
            }
        }

        // If `allowReassign` is enabled, skip variables that are reassigned after declaration.
        if options.allow_reassign.unwrap_or(false)
            && let Some(declarator) = call
                .syntax()
                .ancestors()
                .find_map(JsVariableDeclarator::cast)
            && let Ok(AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
                id_binding,
            ))) = declarator.id()
            && id_binding.all_writes(model).next().is_some()
        {
            return None;
        }

        Some(RuleState {
            class_name: class_name_text,
            inner_expr: arg_expr,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let call = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                call.range(),
                markup! {
                    <Emphasis>{state.class_name.text()}</Emphasis>" is already reactive, wrapping it in "<Emphasis>"$state()"</Emphasis>" is unnecessary."
                },
            )
            .note(markup! {
                "Classes from "<Emphasis>"svelte/reactivity"</Emphasis>" track their own mutations without needing a "<Emphasis>"$state"</Emphasis>" wrapper."
            })
            .note(markup! {
                "Remove the "<Emphasis>"$state()"</Emphasis>" wrapper. If you need to reassign the variable itself, enable the "<Emphasis>"allowReassign"</Emphasis>" option."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsExpression::JsCallExpression(call.clone()),
            state.inner_expr.clone(),
        );
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the unnecessary "<Emphasis>"$state()"</Emphasis>" wrapper." }
                .to_owned(),
            mutation,
        ))
    }
}
