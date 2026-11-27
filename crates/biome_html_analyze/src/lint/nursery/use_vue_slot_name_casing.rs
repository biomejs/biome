use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{
    HtmlAttribute, HtmlAttributeList, T, element_ext::AnyHtmlTagElement,
};
use biome_languages::HtmlFileSource;
use biome_rowan::AstNode;
use biome_rule_options::use_vue_slot_name_casing::{SlotNameCasing, UseVueSlotNameCasingOptions};
use biome_string_case::Case;

declare_lint_rule! {
    /// Enforce a consistent casing for the names of Vue slots.
    ///
    /// A component declares its slots with `<slot name="...">` elements, and the parent fills
    /// them with `v-slot:name` or `#name`. Vue doesn't convert slot names between camelCase and
    /// kebab-case, so the parent must spell the name exactly as the component does. Using one
    /// casing for every slot name makes the names easier to remember and to search for.
    ///
    /// Only `<slot>` elements with a static `name` attribute are checked. Dynamic names, such as
    /// `:name="slotName"`, are ignored.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <slot name="foo-bar" />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <slot name="foo_bar" />
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <slot name="foo" />
    ///   <slot name="fooBar" />
    ///   <slot :name="slotName" />
    /// </template>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `casing`
    ///
    /// Default: `"camelCase"`
    ///
    /// The casing that slot names must follow:
    ///
    /// - `"camelCase"`: slot names must be in camelCase, such as `fooBar`.
    /// - `"kebab-case"`: slot names must be in kebab-case, such as `foo-bar`.
    /// - `"singleword"`: slot names must only contain lowercase letters from `a` to `z`, such as `foo`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "casing": "kebab-case"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///   <slot name="fooBar" />
    /// </template>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <slot name="foo" />
    ///   <slot name="foo-bar" />
    /// </template>
    /// ```
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "casing": "singleword"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///   <slot name="fooBar" />
    /// </template>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <slot name="foo" />
    /// </template>
    /// ```
    ///
    pub UseVueSlotNameCasing {
        version: "2.6.5",
        name: "useVueSlotNameCasing",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("slot-name-casing").same()],
    }
}

impl Rule for UseVueSlotNameCasing {
    type Query = Ast<HtmlAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseVueSlotNameCasingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attribute = ctx.query();
        if attribute.name().ok()?.value_token().ok()?.text_trimmed() != "name"
            || !ctx.source_type::<HtmlFileSource>().is_vue()
        {
            return None;
        }

        let element = attribute
            .parent::<HtmlAttributeList>()?
            .parent::<AnyHtmlTagElement>()?;
        if element.tag_name_kind()? != T![slot] {
            return None;
        }

        let name = attribute.html_string()?.inner_string_text().ok()?;
        if name.is_empty() {
            return None;
        }

        (!follows_casing(name.text(), ctx.options().casing())).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let value = ctx.query().html_string()?;
        let name = value.inner_string_text().ok()?;
        let casing = ctx.options().casing();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                value.range(),
                markup! {
                    "The slot name "<Emphasis>{name.text()}</Emphasis>" isn't "{CasingName(casing)}"."
                },
            )
            .note(markup! {
                "Vue doesn't convert slot names between casings, so components that fill this slot must use the exact same name. Using one casing for every slot name keeps them consistent."
            })
            .note(markup! {
                "Rename the slot so that it is "{CasingName(casing)}", and update the components that fill it."
            }),
        )
    }
}

/// Returns whether the slot `name` follows `casing`.
///
/// For camelCase and kebab-case, names made of characters without a lowercase or uppercase
/// form, such as digits or Japanese text, are allowed because they can't be written in another
/// casing.
fn follows_casing(name: &str, casing: SlotNameCasing) -> bool {
    let allowed_cases = match casing {
        SlotNameCasing::Camel => Case::Camel | Case::Uni,
        SlotNameCasing::Kebab => Case::Kebab | Case::Uni,
        SlotNameCasing::SingleWord => return name.bytes().all(|byte| byte.is_ascii_lowercase()),
    };
    allowed_cases.contains(Case::identify(name, false))
}

/// Writes the name of a [`SlotNameCasing`] as it appears in diagnostics.
struct CasingName(SlotNameCasing);

impl biome_console::fmt::Display for CasingName {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self.0 {
            SlotNameCasing::Camel => "in camelCase",
            SlotNameCasing::Kebab => "in kebab-case",
            SlotNameCasing::SingleWord => "a single word made only of the lowercase letters a to z",
        })
    }
}
