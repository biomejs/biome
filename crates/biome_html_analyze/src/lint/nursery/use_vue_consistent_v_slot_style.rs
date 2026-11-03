use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_factory::make;
use biome_html_syntax::{
    AnyVueDirective, AnyVueDirectiveArgument, HtmlAttributeList, HtmlSyntaxKind, HtmlSyntaxToken,
    T, element_ext::AnyHtmlTagElement,
};
use biome_rowan::{AstNode, BatchMutationExt, Direction, TextRange};
use biome_rule_options::use_vue_consistent_v_slot_style::{
    UseVueConsistentVSlotStyleOptions, VueSlotStyle,
};

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Enforce a consistent style for `v-slot` in Vue templates.
    ///
    /// Vue lets you write the same slot in three ways:
    ///
    /// - the shorthand `#`, as in `#header`;
    /// - the longhand `v-slot:`, as in `v-slot:header`;
    /// - `v-slot` without a slot name, which only works for the default slot.
    ///
    /// Mixing these styles makes templates harder to read. This rule reports slots that
    /// don't use the configured style.
    ///
    /// By default, the rule expects:
    ///
    /// - `v-slot` for the default slot when it's placed directly on a component, as in
    ///   `<MyComponent v-slot="props">`;
    /// - `#default` for the default slot on a `<template>` element;
    /// - `#name` for named slots.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent #default="data">{{ data }}</MyComponent>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent>
    ///     <template v-slot>content</template>
    ///   </MyComponent>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent>
    ///     <template v-slot:header>content</template>
    ///   </MyComponent>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <MyComponent v-slot="data">{{ data }}</MyComponent>
    ///   <MyComponent>
    ///     <template #default>content</template>
    ///     <template #header>content</template>
    ///   </MyComponent>
    /// </template>
    /// ```
    ///
    /// ## Options
    ///
    /// The `atComponent` and `default` options accept `"shorthand"`, `"longhand"`, or
    /// `"v-slot"`. The `named` option accepts `"shorthand"` or `"longhand"`, because a named
    /// slot always needs its name.
    ///
    /// - `"shorthand"`: write `#`, as in `#default` or `#header`.
    /// - `"longhand"`: write `v-slot:`, as in `v-slot:default` or `v-slot:header`.
    /// - `"v-slot"`: write `v-slot` without a slot name.
    ///
    /// ### `atComponent`
    ///
    /// Default: `"v-slot"`
    ///
    /// The style for the default slot when `v-slot` is placed directly on a component, as in
    /// `<MyComponent v-slot="data">`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "atComponent": "shorthand"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///   <MyComponent v-slot="data">{{ data }}</MyComponent>
    /// </template>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <MyComponent #default="data">{{ data }}</MyComponent>
    /// </template>
    /// ```
    ///
    /// ### `default`
    ///
    /// Default: `"shorthand"`
    ///
    /// The style for the default slot on a `<template>` element, as in
    /// `<template #default>`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "default": "v-slot"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///   <MyComponent>
    ///     <template #default>content</template>
    ///   </MyComponent>
    /// </template>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <MyComponent>
    ///     <template v-slot>content</template>
    ///   </MyComponent>
    /// </template>
    /// ```
    ///
    /// ### `named`
    ///
    /// Default: `"shorthand"`
    ///
    /// The style for named slots, as in `<template #header>`. A slot with a dynamic name,
    /// such as `#[name]`, is a named slot.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "named": "longhand"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///   <MyComponent>
    ///     <template #header>content</template>
    ///   </MyComponent>
    /// </template>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <MyComponent>
    ///     <template v-slot:header>content</template>
    ///   </MyComponent>
    /// </template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue style guide: directive shorthands](https://vuejs.org/style-guide/rules-strongly-recommended.html#directive-shorthands)
    /// - [Vue slots](https://vuejs.org/guide/components/slots.html)
    pub UseVueConsistentVSlotStyle {
        version: "next",
        name: "useVueConsistentVSlotStyle",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("v-slot-style").same()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseVueConsistentVSlotStyle {
    type Query = Ast<AnyVueDirective>;
    /// The style the slot should be written in.
    type State = VueSlotStyle;
    type Signals = Option<Self::State>;
    type Options = UseVueConsistentVSlotStyleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let slot = SlotDirective::new(ctx.query())?;
        let options = ctx.options();
        let expected = if slot.is_default() {
            let element = ctx
                .query()
                .parent::<HtmlAttributeList>()?
                .parent::<AnyHtmlTagElement>()?;
            if element.name().ok()?.tag_name_kind() == Some(T![template]) {
                options.default_slot()
            } else {
                options.at_component()
            }
        } else {
            options.named().into()
        };
        (slot.style() != expected).then_some(expected)
    }

    fn diagnostic(ctx: &RuleContext<Self>, expected: &Self::State) -> Option<RuleDiagnostic> {
        let slot = SlotDirective::new(ctx.query())?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                slot.range,
                markup! {
                    "This slot should be written as "<Emphasis>{slot.written_as(*expected)}</Emphasis>"."
                },
            )
            .note(markup! {
                "This project writes every slot the same way, so that templates are easier to read."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, expected: &Self::State) -> Option<HtmlRuleAction> {
        let node = ctx.query();
        let slot = SlotDirective::new(node)?;
        let (modifiers, initializer) = match node {
            AnyVueDirective::VueDirective(directive) => {
                (directive.modifiers(), directive.initializer())
            }
            AnyVueDirective::VueVSlotShorthandDirective(shorthand) => {
                (shorthand.modifiers(), shorthand.initializer())
            }
            _ => return None,
        };
        let message = markup! {
            "Write it as "<Emphasis>{slot.written_as(*expected)}</Emphasis>"."
        }
        .to_owned();
        let argument = slot.argument.unwrap_or_else(|| {
            let name =
                HtmlSyntaxToken::new_detached(HtmlSyntaxKind::HTML_LITERAL, "default", [], []);
            make::vue_static_argument(name).into()
        });

        let replacement: AnyVueDirective = match expected {
            VueSlotStyle::Shorthand => {
                let builder =
                    make::vue_v_slot_shorthand_directive(make::token(T![#]), argument, modifiers);
                match initializer {
                    Some(initializer) => builder.with_initializer(initializer),
                    None => builder,
                }
                .build()
                .into()
            }
            VueSlotStyle::Longhand | VueSlotStyle::VSlot => {
                let mut builder = make::vue_directive(make::ident("v-slot"), modifiers);
                if *expected == VueSlotStyle::Longhand {
                    builder = builder.with_arg(
                        make::vue_directive_argument(make::token(T![:]))
                            .with_arg(argument)
                            .build(),
                    );
                }
                if let Some(initializer) = initializer {
                    builder = builder.with_initializer(initializer);
                }
                builder.build().into()
            }
        };

        let mut mutation = ctx.root().begin();
        mutation.replace_node(node.clone(), replacement);
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message,
            mutation,
        ))
    }
}

/// A `v-slot` directive in any of its forms.
struct SlotDirective {
    /// Whether the directive is written with `#`.
    is_shorthand: bool,
    /// The slot name, or `None` for `v-slot` without a name.
    argument: Option<AnyVueDirectiveArgument>,
    /// The range from the start of the directive to the end of its slot name. It leaves out
    /// modifiers and the value.
    range: TextRange,
}

impl SlotDirective {
    /// Returns `None` when `directive` isn't a `v-slot` directive, or when it has a `:` that
    /// isn't followed by a slot name.
    fn new(directive: &AnyVueDirective) -> Option<Self> {
        let (is_shorthand, argument, name_end) = match directive {
            AnyVueDirective::VueDirective(directive) => {
                if !directive.is_slot() {
                    return None;
                }
                match directive.arg() {
                    Some(argument) => {
                        let argument = argument.arg()?;
                        let end = argument.range().end();
                        (false, Some(argument), end)
                    }
                    None => (false, None, directive.name_token().ok()?.text_trimmed_range().end()),
                }
            }
            AnyVueDirective::VueVSlotShorthandDirective(shorthand) => {
                let argument = shorthand.arg().ok()?;
                let end = argument.range().end();
                (true, Some(argument), end)
            }
            _ => return None,
        };
        Some(Self {
            is_shorthand,
            argument,
            range: TextRange::new(directive.range().start(), name_end),
        })
    }

    fn style(&self) -> VueSlotStyle {
        if self.is_shorthand {
            VueSlotStyle::Shorthand
        } else if self.argument.is_some() {
            VueSlotStyle::Longhand
        } else {
            VueSlotStyle::VSlot
        }
    }

    /// Returns `true` for the default slot: a slot without a name, or one named `default`.
    /// A dynamic name, as in `#[name]`, is never treated as the default slot.
    fn is_default(&self) -> bool {
        match &self.argument {
            None => true,
            Some(AnyVueDirectiveArgument::VueStaticArgument(argument)) => argument
                .name_token()
                .is_ok_and(|name| name.text_trimmed() == "default"),
            Some(_) => false,
        }
    }

    /// Returns how this slot looks when written in `style`.
    fn written_as(&self, style: VueSlotStyle) -> SlotText<'_> {
        SlotText {
            style,
            argument: self.argument.as_ref(),
        }
    }
}

/// Writes a slot directive without its modifiers and value, such as `#header`,
/// `v-slot:header`, or `v-slot`. A slot without a name is written with the name `default`.
struct SlotText<'a> {
    style: VueSlotStyle,
    argument: Option<&'a AnyVueDirectiveArgument>,
}

impl biome_console::fmt::Display for SlotText<'_> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self.style {
            VueSlotStyle::Shorthand => "#",
            VueSlotStyle::Longhand => "v-slot:",
            VueSlotStyle::VSlot => return fmt.write_str("v-slot"),
        })?;
        match self.argument {
            Some(argument) => {
                for token in argument.syntax().descendants_tokens(Direction::Next) {
                    fmt.write_str(token.text_trimmed())?;
                }
                Ok(())
            }
            None => fmt.write_str("default"),
        }
    }
}
