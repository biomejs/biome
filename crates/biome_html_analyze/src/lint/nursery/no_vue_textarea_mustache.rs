use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{HtmlElement, T};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, AstNodeList, TextRange, TextSize};
use biome_rule_options::no_vue_textarea_mustache::NoVueTextareaMustacheOptions;

declare_lint_rule! {
    /// Disallow `{{ }}` text interpolation inside a Vue `<textarea>`.
    ///
    /// In most elements, Vue replaces `{{ expression }}` with the value of the expression.
    /// Vue doesn't support this inside a `<textarea>`. To show a value in a textarea and keep
    /// it up to date, bind it with `v-model` instead.
    ///
    /// Vue doesn't process `{{ }}` inside an element that has `v-pre`, or inside its children,
    /// so the rule ignores those textareas.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <textarea>{{ message }}</textarea>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <textarea v-model="message"></textarea>
    ///   <textarea v-pre>{{ message }}</textarea>
    /// </template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue guide: multiline text in forms](https://vuejs.org/guide/essentials/forms.html#multiline-text)
    pub NoVueTextareaMustache {
        version: "next",
        name: "noVueTextareaMustache",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-textarea-mustache").same()],
    }
}

impl Rule for NoVueTextareaMustache {
    type Query = Ast<HtmlElement>;
    /// The range of each `{{ }}` inside the textarea, delimiters included.
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoVueTextareaMustacheOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_vue() {
            return Box::default();
        }

        let textarea = ctx.query();
        let is_textarea = textarea
            .opening_element()
            .ok()
            .and_then(|opening| opening.name().ok()?.tag_name_kind())
            == Some(T![textarea]);
        if !is_textarea || is_in_v_pre(textarea) {
            return Box::default();
        }

        // The parser keeps the content of a `<textarea>` as a single piece of
        // raw text, so the `{{ }}` delimiters are found in that text.
        let Some(content) = textarea
            .children()
            .iter()
            .find_map(|child| child.as_any_html_content()?.as_html_embedded_content().cloned())
        else {
            return Box::default();
        };
        let Ok(token) = content.value_token() else {
            return Box::default();
        };
        let start = token.text_trimmed_range().start();
        mustache_ranges(token.text_trimmed())
            .map(|range| range + start)
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    <Emphasis>"{{ }}"</Emphasis>" can't be used inside a "<Emphasis>"<textarea>"</Emphasis>"."
                },
            )
            .note(markup! {
                "Vue doesn't support text interpolation inside a textarea."
            })
            .note(markup! {
                "Bind the value of the textarea with "<Emphasis>"v-model"</Emphasis>" instead."
            }),
        )
    }
}

/// Returns `true` if `element` or one of the elements around it has `v-pre`,
/// which tells Vue to leave its content as written.
fn is_in_v_pre(element: &HtmlElement) -> bool {
    element
        .syntax()
        .ancestors()
        .filter_map(HtmlElement::cast)
        .filter_map(|element| element.opening_element().ok())
        .any(|opening| {
            opening.attributes().iter().any(|attribute| {
                attribute
                    .as_any_vue_directive()
                    .and_then(|directive| directive.as_vue_directive())
                    .is_some_and(|directive| directive.is_pre())
            })
        })
}

/// Returns the range of each `{{ }}` in `text`, relative to the start of `text`.
///
/// A `{{` without a matching `}}` after it isn't an interpolation, so it is
/// skipped.
fn mustache_ranges(text: &str) -> impl Iterator<Item = TextRange> + '_ {
    let mut offset = 0;
    std::iter::from_fn(move || {
        let open = offset + text[offset..].find("{{")?;
        let close = open + 2 + text[open + 2..].find("}}")? + 2;
        offset = close;
        Some(TextRange::new(
            TextSize::from(open as u32),
            TextSize::from(close as u32),
        ))
    })
}
