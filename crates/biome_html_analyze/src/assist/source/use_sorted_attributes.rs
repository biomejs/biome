use crate::HtmlRuleAction;
use biome_analyze::shared::sort_attributes::{AttributeGroup, SortableAttribute};
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_diagnostics::{Applicability, category};
use biome_html_syntax::{
    AnyAstroDirective, AnyHtmlAttribute, AnySvelteBindingProperty, AnySvelteDirective,
    AnyVueDirective, AnyVueDirectiveArgument, AstroDirectiveValue, HtmlAttributeList, HtmlLanguage,
    HtmlOpeningElement, HtmlSelfClosingElement, SvelteDirectiveValue,
};
use biome_rowan::{AstNode, AstNodeExt, BatchMutationExt, SyntaxToken};
use biome_rule_options::use_sorted_attributes::{SortOrder, UseSortedAttributesOptions};
use std::{borrow::Cow, cmp::Ordering, iter::zip};

declare_source_rule! {
    /// Enforce attribute sorting in HTML elements.
    ///
    /// This rule checks if HTML attributes, along with Astro, Svelte, and Vue directives,
    /// are sorted in a consistent way.
    /// The sort order is:
    /// - Regular HTML attributes, sorted alphabetically according to the `sortOrder` option
    /// - Astro directives, sorted alphabetically according to `sortOrder`
    /// - Svelte directives, sorted according to eslint-plugin-svelte's [`sort-attributes` rule](https://sveltejs.github.io/eslint-plugin-svelte/rules/sort-attributes/)
    /// - Vue directives, sorted according to the [Vue.js Style Guide](https://eslint.vuejs.org/rules/attributes-order)
    ///
    /// If two attributes belong to the same category, they will be sorted alphabetically
    /// according to `sortOrder`.
    ///
    /// This rule will not consider spread props or the [Vue `v-bind="object"` syntax](https://vuejs.org/guide/essentials/template-syntax.html#dynamically-binding-multiple-attributes)
    /// as sortable.
    /// Instead, it will sort each group of consecutive sortable attributes within the element,
    /// leaving any spread props or `v-bind="object"` attributes in place.
    /// This prevents breaking the override of certain props using spread
    /// props or `v-bind="object"`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <input type="text" id="name" name="name" />
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <textarea id="mytextarea" name="textarea" rows="5" cols="20" data-1="" data-11="" data-12="" data-2="">Hello, world!</textarea>
    /// ```
    ///
    /// ```astro,expect_diagnostic
    ///   <svg slot="fallback" class="generic-avatar" transition:name="avatar">...</svg>
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    ///   	<input type="range" bind:value={b} min="0" max="10" />
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    ///   	<div bind:value2={a} bind:value1={a} {...props} style:color="red">...</div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    ///   	<input @input="onInput" :value="text" placeholder="Type here">
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <input id="name" name="name" type="text" />
    /// ```
    ///
    /// ```html
    /// <textarea cols="20" data-1="" data-2="" data-11="" data-12="" id="mytextarea" name="textarea" rows="5">Hello, world!</textarea>
    /// ```
    ///
    /// ```astro
    ///   <svg class="generic-avatar" slot="fallback" transition:name="avatar">...</svg>
    /// ```
    ///
    /// ```svelte
    ///   	<input max="10" min="0" type="range" bind:value={b} />
    /// ```
    ///
    /// ```svelte
    ///   	<div bind:value1={a} bind:value2={a} {...props} style:color="red">...</div>
    /// ```
    ///
    /// ```vue
    ///   	<input placeholder="Type here" :value="text" @input="onInput" >
    /// ```
    ///
    /// ## Options
    ///
    /// The following options are available
    ///
    /// ### `sortOrder`
    /// The sort ordering to enforce.
    /// Values:
    ///
    /// - `"[natural](https://en.wikipedia.org/wiki/Natural_sort_order)"`
    /// - `"[lexicographic](https://en.wikipedia.org/wiki/Lexicographic_order)"`
    ///
    /// Default: `"natural"`
    ///
    /// #### Examples for `"sortOrder": "lexicographic"`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "sortOrder": "lexicographic"
    ///     }
    /// }
    /// ```
    /// ```html,use_options,expect_diagnostic
    /// <textarea id="mytextarea" name="textarea" rows="5" cols="20" data-1="" data-2="" data-11="" data-12="">Hello, world!</textarea>
    /// ```
    ///
    pub UseSortedAttributes {
        version: "next",
        name: "useSortedAttributes",
        language: "html",
        recommended: false,
        sources: &[RuleSource::HtmlEslint("sort-attrs").inspired(), RuleSource::EslintVueJs("attributes-order").inspired(), RuleSource::EslintSvelte("sort-attributes").inspired(), RuleSource::EslintAstro("sort-attributes").inspired()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseSortedAttributes {
    type Query = Ast<HtmlAttributeList>;
    type State = AttributeGroup<SortableHtmlAttribute>;
    type Signals = Box<[Self::State]>;
    type Options = UseSortedAttributesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attrs = ctx.query();
        let options = ctx.options();

        let mut current_attr_group = AttributeGroup::default();
        let mut attr_groups = Vec::new();
        let sort_by = options.sort_order.unwrap_or_default();

        let comparator = get_comparator(sort_by);

        // Convert to boolean-based comparator for is_sorted_by
        let boolean_comparator = |a: &SortableHtmlAttribute, b: &SortableHtmlAttribute| {
            comparator(a, b) != Ordering::Greater
        };

        let reset_attr_group = |mut current_group: AttributeGroup<SortableHtmlAttribute>,
                                groups: &mut Vec<_>| {
            if !current_group.is_empty() && !current_group.is_sorted(boolean_comparator) {
                groups.push(current_group);
                AttributeGroup::default()
            } else {
                // Reuse the same buffer
                current_group.clear();
                current_group
            }
        };

        for attr in attrs {
            match attr {
                AnyHtmlAttribute::HtmlSpreadAttribute(_) => {
                    current_attr_group = reset_attr_group(current_attr_group, &mut attr_groups);
                }
                attr => {
                    if is_v_bind_object(&attr) {
                        current_attr_group = reset_attr_group(current_attr_group, &mut attr_groups);
                    } else {
                        current_attr_group.attrs.push(SortableHtmlAttribute(attr));
                    }
                }
            }
        }
        if !current_attr_group.is_empty() && !current_attr_group.is_sorted(boolean_comparator) {
            attr_groups.push(current_attr_group);
        }
        attr_groups.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("assist/source/useSortedAttributes"),
            Self::text_range(ctx, state)?,
            markup! {
                "The attributes are not sorted."
            },
        ))
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        ctx.query().syntax().ancestors().skip(1).find_map(|node| {
            HtmlOpeningElement::cast_ref(&node)
                .map(|element| element.range())
                .or_else(|| HtmlSelfClosingElement::cast_ref(&node).map(|element| element.range()))
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let mut mutation = ctx.root().begin();
        let options = ctx.options();
        let sort_by = options.sort_order.unwrap_or_default();

        let comparator = get_comparator(sort_by);

        for (SortableHtmlAttribute(attr), SortableHtmlAttribute(sorted_attr)) in
            zip(state.attrs.iter(), state.get_sorted_attributes(comparator)?)
        {
            mutation.replace_node_discard_trivia(attr.clone(), sorted_attr);
        }

        Some(RuleAction::new(
            rule_action_category!(),
            Applicability::Always,
            markup! { "Sort the HTML attributes." },
            mutation,
        ))
    }
}

fn is_v_bind_object(attr: &AnyHtmlAttribute) -> bool {
    if let AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueDirective(dir)) = attr {
        if let Ok(attr_name) = dir.name_token().as_ref().map(|token| token.text_trimmed()) {
            attr_name == "v-bind" && dir.arg().is_none()
        } else {
            false
        }
    } else {
        false
    }
}

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord)]
enum SortCategory {
    HtmlAttribute,
    AstroClassDirective,
    AstroClientDirective,
    AstroDefineDirective,
    AstroIsDirective,
    AstroServerDirective,
    AstroSetDirective,
    SvelteBindThisDirective,
    SvelteStyleDirective,
    SvelteClassDirective,
    SvelteBindDirective,
    SvelteUseDirective,
    SvelteTransitionDirective,
    SvelteInDirective,
    SvelteOutDirective,
    SvelteAnimateDirective,
    SvelteAttachAttribute,

    VueDefinition,
    VueListRendering,
    VueConditional,
    VueRenderModifier,
    VueUnique,
    VueSlot,
    VueTwoWayBinding,
    VueCustomDirective,
    // `v-bind`, etc.
    VueOtherAttribute,
    VueEvent,
    VueContent,

    Unknown,
}

#[derive(PartialEq, Eq, Clone)]
pub struct SortableHtmlAttribute(AnyHtmlAttribute);

impl SortableHtmlAttribute {
    fn category(&self) -> SortCategory {
        match &self.0 {
            AnyHtmlAttribute::HtmlAttribute(attr) => {
                if let Ok(attr_name) = attr
                    .name()
                    .and_then(|name| name.value_token())
                    .as_ref()
                    .map(|token| token.text_trimmed())
                {
                    match attr_name {
                        // Vue ref attribute
                        "ref" => SortCategory::VueUnique,
                        _ => SortCategory::HtmlAttribute,
                    }
                } else {
                    SortCategory::HtmlAttribute
                }
            }
            AnyHtmlAttribute::HtmlAttributeSingleTextExpression(_) => SortCategory::HtmlAttribute,
            AnyHtmlAttribute::AnyAstroDirective(AnyAstroDirective::AstroClassDirective(_)) => {
                SortCategory::AstroClassDirective
            }
            AnyHtmlAttribute::AnyAstroDirective(AnyAstroDirective::AstroClientDirective(_)) => {
                SortCategory::AstroClientDirective
            }
            AnyHtmlAttribute::AnyAstroDirective(AnyAstroDirective::AstroDefineDirective(_)) => {
                SortCategory::AstroDefineDirective
            }
            AnyHtmlAttribute::AnyAstroDirective(AnyAstroDirective::AstroIsDirective(_)) => {
                SortCategory::AstroIsDirective
            }
            AnyHtmlAttribute::AnyAstroDirective(AnyAstroDirective::AstroServerDirective(_)) => {
                SortCategory::AstroServerDirective
            }
            AnyHtmlAttribute::AnyAstroDirective(AnyAstroDirective::AstroSetDirective(_)) => {
                SortCategory::AstroSetDirective
            }
            AnyHtmlAttribute::AnySvelteDirective(AnySvelteDirective::SvelteStyleDirective(_)) => {
                SortCategory::SvelteStyleDirective
            }
            AnyHtmlAttribute::AnySvelteDirective(AnySvelteDirective::SvelteClassDirective(_)) => {
                SortCategory::SvelteClassDirective
            }
            AnyHtmlAttribute::AnySvelteDirective(AnySvelteDirective::SvelteBindDirective(dir)) => {
                if let Some(token) = dir
                    .value()
                    .ok()
                    .and_then(|value| svelte_directive_value_token(&value))
                {
                    match token.text_trimmed() {
                        "this" => SortCategory::SvelteBindThisDirective,
                        _ => SortCategory::SvelteBindDirective,
                    }
                } else {
                    SortCategory::SvelteBindDirective
                }
            }
            AnyHtmlAttribute::AnySvelteDirective(AnySvelteDirective::SvelteUseDirective(_)) => {
                SortCategory::SvelteUseDirective
            }
            AnyHtmlAttribute::AnySvelteDirective(
                AnySvelteDirective::SvelteTransitionDirective(_),
            ) => SortCategory::SvelteTransitionDirective,
            AnyHtmlAttribute::AnySvelteDirective(AnySvelteDirective::SvelteInDirective(_)) => {
                SortCategory::SvelteInDirective
            }
            AnyHtmlAttribute::AnySvelteDirective(AnySvelteDirective::SvelteOutDirective(_)) => {
                SortCategory::SvelteOutDirective
            }
            AnyHtmlAttribute::AnySvelteDirective(AnySvelteDirective::SvelteAnimateDirective(_)) => {
                SortCategory::SvelteAnimateDirective
            }
            AnyHtmlAttribute::SvelteAttachAttribute(_) => SortCategory::SvelteAttachAttribute,
            AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueBogusDirective(_)) => {
                SortCategory::Unknown
            }
            AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueDirective(dir)) => {
                if let Ok(attr_name) = dir.name_token().as_ref().map(|token| token.text_trimmed()) {
                    match attr_name {
                        "v-for" => SortCategory::VueListRendering,
                        "v-if" | "v-else-if" | "v-else" | "v-show" | "v-cloak" => {
                            SortCategory::VueConditional
                        }
                        "v-once" | "v-pre" => SortCategory::VueRenderModifier,
                        "v-slot" => SortCategory::VueSlot,
                        "v-model" => SortCategory::VueTwoWayBinding,
                        "v-on" => SortCategory::VueEvent,
                        "v-text" | "v-html" => SortCategory::VueContent,
                        "v-bind" => SortCategory::VueOtherAttribute,
                        _ => SortCategory::VueCustomDirective,
                    }
                } else {
                    SortCategory::VueCustomDirective
                }
            }
            AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueVBindShorthandDirective(dir)) => {
                if let Ok(arg) = dir.arg().and_then(|arg| arg.arg()) {
                    match arg {
                        AnyVueDirectiveArgument::VueBogusDirectiveArgument(_) => {
                            SortCategory::Unknown
                        }
                        AnyVueDirectiveArgument::VueDynamicArgument(_) => {
                            SortCategory::VueOtherAttribute
                        }
                        AnyVueDirectiveArgument::VueStaticArgument(arg) => {
                            if let Ok(arg_name) =
                                arg.name_token().as_ref().map(|token| token.text_trimmed())
                            {
                                match arg_name {
                                    "is" => SortCategory::VueDefinition,
                                    "key" => SortCategory::VueUnique,
                                    _ => SortCategory::VueOtherAttribute,
                                }
                            } else {
                                SortCategory::VueCustomDirective
                            }
                        }
                    }
                } else {
                    SortCategory::VueCustomDirective
                }
            }
            AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueVOnShorthandDirective(_)) => {
                SortCategory::VueEvent
            }
            AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueVSlotShorthandDirective(_)) => {
                SortCategory::VueSlot
            }
            _ => SortCategory::Unknown,
        }
    }
}

fn svelte_directive_value_token(
    directive: &SvelteDirectiveValue,
) -> Option<SyntaxToken<HtmlLanguage>> {
    match &directive.property().ok()? {
        AnySvelteBindingProperty::SvelteLiteral(l) => l.value_token().ok(),
        AnySvelteBindingProperty::SvelteName(n) => n.ident_token().ok(),
    }
}

fn vue_directive_arg_token(arg: &AnyVueDirectiveArgument) -> Option<SyntaxToken<HtmlLanguage>> {
    match arg {
        AnyVueDirectiveArgument::VueBogusDirectiveArgument(_) => None,
        AnyVueDirectiveArgument::VueDynamicArgument(_) => None,
        AnyVueDirectiveArgument::VueStaticArgument(arg) => arg.name_token().ok(),
    }
}

fn astro_directive_value_token(
    directive: &AstroDirectiveValue,
) -> Option<SyntaxToken<HtmlLanguage>> {
    directive.name().ok()?.value_token().ok()
}

impl SortableAttribute for SortableHtmlAttribute {
    type Language = HtmlLanguage;

    /// Returns the value of the attribute to be compared against another attribute from the same category.
    fn name(&self) -> Option<SyntaxToken<Self::Language>> {
        match &self.0 {
            AnyHtmlAttribute::HtmlAttribute(attr) => attr.name().ok()?.value_token().ok(),
            AnyHtmlAttribute::HtmlAttributeSingleTextExpression(attr) => {
                attr.expression().ok()?.html_literal_token().ok()
            }
            AnyHtmlAttribute::AnyAstroDirective(AnyAstroDirective::AstroClassDirective(dir)) => {
                astro_directive_value_token(&dir.value().ok()?)
            }
            AnyHtmlAttribute::AnyAstroDirective(AnyAstroDirective::AstroClientDirective(dir)) => {
                astro_directive_value_token(&dir.value().ok()?)
            }
            AnyHtmlAttribute::AnyAstroDirective(AnyAstroDirective::AstroDefineDirective(dir)) => {
                astro_directive_value_token(&dir.value().ok()?)
            }
            AnyHtmlAttribute::AnyAstroDirective(AnyAstroDirective::AstroIsDirective(dir)) => {
                astro_directive_value_token(&dir.value().ok()?)
            }
            AnyHtmlAttribute::AnyAstroDirective(AnyAstroDirective::AstroServerDirective(dir)) => {
                astro_directive_value_token(&dir.value().ok()?)
            }
            AnyHtmlAttribute::AnyAstroDirective(AnyAstroDirective::AstroSetDirective(dir)) => {
                astro_directive_value_token(&dir.value().ok()?)
            }
            AnyHtmlAttribute::AnySvelteDirective(AnySvelteDirective::SvelteStyleDirective(dir)) => {
                svelte_directive_value_token(&dir.value().ok()?)
            }
            AnyHtmlAttribute::AnySvelteDirective(AnySvelteDirective::SvelteClassDirective(dir)) => {
                svelte_directive_value_token(&dir.value().ok()?)
            }
            AnyHtmlAttribute::AnySvelteDirective(AnySvelteDirective::SvelteBindDirective(dir)) => {
                svelte_directive_value_token(&dir.value().ok()?)
            }
            AnyHtmlAttribute::AnySvelteDirective(AnySvelteDirective::SvelteUseDirective(dir)) => {
                svelte_directive_value_token(&dir.value().ok()?)
            }
            AnyHtmlAttribute::AnySvelteDirective(
                AnySvelteDirective::SvelteTransitionDirective(dir),
            ) => svelte_directive_value_token(&dir.value().ok()?),
            AnyHtmlAttribute::AnySvelteDirective(AnySvelteDirective::SvelteInDirective(dir)) => {
                svelte_directive_value_token(&dir.value().ok()?)
            }
            AnyHtmlAttribute::AnySvelteDirective(AnySvelteDirective::SvelteOutDirective(dir)) => {
                svelte_directive_value_token(&dir.value().ok()?)
            }
            AnyHtmlAttribute::AnySvelteDirective(AnySvelteDirective::SvelteAnimateDirective(
                dir,
            )) => svelte_directive_value_token(&dir.value().ok()?),
            AnyHtmlAttribute::SvelteAttachAttribute(_) => None,
            AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueDirective(dir)) => {
                match dir.name_token().ok()?.text_trimmed() {
                    "v-on" | "v-bind" | "v-slot" => dir
                        .arg()?
                        .arg()
                        .ok()
                        .and_then(|arg| vue_directive_arg_token(&arg)),
                    _ => dir.name_token().ok(),
                }
            }
            AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueVBindShorthandDirective(dir)) => {
                vue_directive_arg_token(&dir.arg().ok()?.arg().ok()?)
            }
            AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueVSlotShorthandDirective(dir)) => {
                vue_directive_arg_token(&dir.arg().ok()?)
            }
            AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueVOnShorthandDirective(dir)) => {
                vue_directive_arg_token(&dir.arg().ok()?)
            }
            _ => None,
        }
    }

    fn node(&self) -> &impl AstNode<Language = Self::Language> {
        &self.0
    }

    fn replace_token(
        self,
        prev_token: SyntaxToken<Self::Language>,
        next_token: SyntaxToken<Self::Language>,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self(
            self.0
                .replace_token_discard_trivia(prev_token, next_token)?,
        ))
    }
}

fn compare_html_attributes(
    a: &SortableHtmlAttribute,
    b: &SortableHtmlAttribute,
    comparator: fn(&SortableHtmlAttribute, &SortableHtmlAttribute) -> Ordering,
) -> Ordering {
    // Sort by category first
    if a.category() != b.category() {
        return a.category().cmp(&b.category());
    }

    // If category is the same, sort according to comparator
    comparator(a, b)
}

fn ascii_nat_cmp(a: &SortableHtmlAttribute, b: &SortableHtmlAttribute) -> Ordering {
    compare_html_attributes(a, b, SortableHtmlAttribute::ascii_nat_cmp)
}

fn lexicographic_cmp(a: &SortableHtmlAttribute, b: &SortableHtmlAttribute) -> Ordering {
    compare_html_attributes(a, b, SortableHtmlAttribute::lexicographic_cmp)
}

fn get_comparator(
    sort_order: SortOrder,
) -> fn(&SortableHtmlAttribute, &SortableHtmlAttribute) -> Ordering {
    match sort_order {
        SortOrder::Natural => ascii_nat_cmp,
        SortOrder::Lexicographic => lexicographic_cmp,
    }
}
