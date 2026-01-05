use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyVueDirective, HtmlAttributeList, VueDirectiveArgument};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::use_vue_v_for_key::UseVueVForKeyOptions;

declare_lint_rule! {
    /// Enforce that elements using `v-for` also specify a unique `key`.
    ///
    /// When rendering lists with `v-for`, Vue relies on a `key` to track elements efficiently.
    /// The `key` can be provided via longhand `v-bind:key` or shorthand `:key`. If you need to
    /// animate the entrance/exit of an item in a list, the key should be a unique identifier for
    /// each item in the list, and not the index of the item.
    ///
    /// For more information, see the Vue documentation on [list rendering](https://vuejs.org/guide/essentials/list#maintaining-state-with-key).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <li v-for="item in items">{{ item }}</li>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <li v-for="item in items" :key="item.id">{{ item }}</li>
    /// ```
    ///
    /// ```vue
    /// <li v-for="item in items" v-bind:key="item.id">{{ item }}</li>
    /// ```
    ///
    pub UseVueVForKey {
        version: "2.3.11",
        name: "useVueVForKey",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("require-v-for-key").same()],
    }
}

impl Rule for UseVueVForKey {
    type Query = Ast<HtmlAttributeList>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseVueVForKeyOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let attrs = ctx.query();
        let mut has_v_for = None;
        let mut has_key = false;
        for attr in attrs.iter() {
            if let Some(dir_any) = attr.as_any_vue_directive() {
                match dir_any {
                    AnyVueDirective::VueDirective(dir) => {
                        if dir.name_token().is_ok_and(|t| t.text_trimmed() == "v-for") {
                            has_v_for = Some(dir.range());
                        }
                        if dir.name_token().is_ok_and(|t| t.text_trimmed() == "v-bind")
                            && let Some(arg) = dir.arg()
                            && is_v_bind_key_argument(&arg)
                        {
                            has_key = true;
                        }
                    }
                    AnyVueDirective::VueVBindShorthandDirective(sh) => {
                        if let Ok(arg) = sh.arg()
                            && is_v_bind_key_argument(&arg)
                        {
                            has_key = true;
                        }
                    }
                    _ => {}
                }
                if has_v_for.is_some() && has_key {
                    // early exit if both found
                    return None;
                }
            }
        }
        if let (Some(v_for_range), false) = (has_v_for, has_key) {
            return Some(v_for_range);
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "This element is using "<Emphasis>"v-for"</Emphasis>", but the "<Emphasis>"key"</Emphasis>" attribute is missing."
                },
            )
            .note(markup! {
                "Using a unique key with "<Emphasis>"v-for"</Emphasis>" helps Vue optimize rendering and track elements efficiently. Failing to provide a key can result in unexpected behavior during updates. "<Hyperlink href="https://vuejs.org/guide/essentials/list#maintaining-state-with-key">"See the Vue docs for more info"</Hyperlink>"."
            })
            .note(markup! {
                "Provide the key using "<Emphasis>":key=\"value\""</Emphasis>", and have the value be a unique value from the items you are iterating over. For example: `<li v-for=\"item in items\" :key=\"item.id\">{{ item }}</li>`"
            }),
        )
    }
}

fn is_v_bind_key_argument(arg: &VueDirectiveArgument) -> bool {
    let Ok(arg) = arg.arg() else {
        return false;
    };
    let Some(static_arg) = arg.as_vue_static_argument() else {
        return false;
    };
    static_arg
        .name_token()
        .is_ok_and(|name| name.text() == "key")
}
