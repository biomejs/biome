use crate::services::vue::VueSemantic;
use biome_analyze::context::RuleContext;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::HtmlRoot;
use biome_rule_options::no_vue_undeclared_slots::NoVueUndeclaredSlotsOptions;
use biome_vue_semantic::{BlockKind, RefSite, ReferenceId, Resolution};

declare_lint_rule! {
    /// Disallow `<slot>` elements whose slot isn't declared in the component's types.
    ///
    /// In a Vue component written in TypeScript, the slots the component renders can be
    /// declared with types. The types describe which slots a parent component can fill, and
    /// the data each slot passes to that content. Editors and type checkers use them to check
    /// the content parent components pass in. A slot that is rendered but not declared isn't
    /// checked at all.
    ///
    /// Slots are declared in one of two ways:
    ///
    /// - with `defineSlots()` in `<script setup>`;
    /// - with the `slots` option of the component, written as `Object as SlotsType<...>`.
    ///
    /// A `<slot>` element without a `name` attribute renders the slot named `default`.
    ///
    /// The rule only checks components that have a `<script lang="ts">` or
    /// `<script lang="tsx">` block, because slots can only be declared with types.
    /// It doesn't report a slot when its name is computed with an expression, or when the
    /// declared slots come from a type that Biome can't read, such as an imported type or an
    /// interface that extends another one.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// defineSlots<{
    ///     default(props: { msg: string }): any;
    /// }>();
    /// </script>
    ///
    /// <template>
    ///     <slot name="footer" />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script lang="ts">
    /// import { defineComponent, type SlotsType } from "vue";
    ///
    /// export default defineComponent({
    ///     slots: Object as SlotsType<{
    ///         header: { title: string };
    ///     }>,
    /// });
    /// </script>
    ///
    /// <template>
    ///     <slot />
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue,ignore
    /// <script setup lang="ts">
    /// defineSlots<{
    ///     default(props: { msg: string }): any;
    ///     footer(props: {}): any;
    /// }>();
    /// </script>
    ///
    /// <template>
    ///     <slot msg="hello" />
    ///     <slot name="footer" />
    /// </template>
    /// ```
    pub NoVueUndeclaredSlots {
        version: "2.6.5",
        name: "noVueUndeclaredSlots",
        language: "html",
        sources: &[RuleSource::EslintVueJs("require-explicit-slots").inspired()],
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
    }
}

impl Rule for NoVueUndeclaredSlots {
    type Query = VueSemantic<HtmlRoot>;
    /// The reference that the `<slot>` element makes to its slot name.
    type State = ReferenceId;
    type Signals = Box<[Self::State]>;
    type Options = NoVueUndeclaredSlotsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let is_typescript = model.blocks().any(|block| {
            block.kind() == BlockKind::Script && matches!(block.lang(), Some("ts" | "tsx"))
        });
        if !is_typescript {
            return Box::default();
        }
        model
            .references()
            .filter(|reference| {
                matches!(reference.site(), RefSite::SlotName(_))
                    && reference.resolution() == Resolution::Unresolved
            })
            .map(|reference| reference.id())
            .collect()
    }

    fn diagnostic(ctx: &RuleContext<Self>, id: &Self::State) -> Option<RuleDiagnostic> {
        let reference = ctx.model().reference(*id);
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                reference.range(),
                markup! {
                    "The slot "<Emphasis>{reference.name()}</Emphasis>" isn't declared."
                },
            )
            .note(markup! {
                "Without a declaration, the content that parent components pass to this slot isn't type checked."
            })
            .note(markup! {
                "Declare the slot with "<Emphasis>"defineSlots()"</Emphasis>" in "<Emphasis>"<script setup>"</Emphasis>", or with the "<Emphasis>"slots"</Emphasis>" option of the component."
            }),
        )
    }
}
