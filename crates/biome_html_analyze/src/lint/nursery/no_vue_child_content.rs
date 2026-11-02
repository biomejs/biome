use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_factory::make;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyVueDirective, HtmlElement, HtmlSyntaxToken, VueDirective,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TextRange};
use biome_rule_options::no_vue_child_content::NoVueChildContentOptions;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Disallow content inside elements whose content is replaced by `v-html` or `v-text`.
    ///
    /// The `v-html` and `v-text` directives replace everything between the opening and closing
    /// tags of their element. Vue discards anything written there, including text, other
    /// elements, and comments, so it never appears on the page.
    ///
    /// Either remove the content, or remove the directive and keep the content.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-html="html">Loading...</div>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <p v-text="message"><span>{{ fallback }}</span></p>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-html="html"><!-- The content goes here --></div>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <div v-html="html"></div>
    ///   <p v-text="message" />
    ///   <div v-html="html">
    ///   </div>
    ///   <div>Loading...</div>
    /// </template>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `additionalDirectives`
    ///
    /// Other directives that replace the content of their element, such as the `v-t` directive
    /// of Vue I18n. Write the names without the `v-` prefix. `v-html` and `v-text` are always
    /// checked.
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "additionalDirectives": ["t"]
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///   <p v-t="'greeting'">Hello</p>
    /// </template>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <p v-t="'greeting'"></p>
    /// </template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue `v-html` directive](https://vuejs.org/api/built-in-directives.html#v-html)
    /// - [Vue `v-text` directive](https://vuejs.org/api/built-in-directives.html#v-text)
    pub NoVueChildContent {
        version: "next",
        name: "noVueChildContent",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-child-content").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoVueChildContent {
    type Query = Ast<HtmlElement>;
    /// The first directive of the element that replaces its content.
    type State = VueDirective;
    type Signals = Option<Self::State>;
    type Options = NoVueChildContentOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let opening = element.opening_element().ok()?;
        let additional_directives = ctx.options().additional_directives();
        let directive = opening.attributes().iter().find_map(|attribute| {
            let AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueDirective(directive)) =
                attribute
            else {
                return None;
            };
            replaces_content(&directive, additional_directives).then_some(directive)
        })?;

        let has_content = !element.children().is_empty()
            || has_comments(&opening.r_angle_token().ok()?, &closing_l_angle(element)?);
        has_content.then_some(directive)
    }

    fn diagnostic(ctx: &RuleContext<Self>, directive: &Self::State) -> Option<RuleDiagnostic> {
        let element = ctx.query();
        let name = directive.name_token().ok()?;
        let name = name.text_trimmed();
        let content_range = TextRange::new(
            element
                .opening_element()
                .ok()?
                .r_angle_token()
                .ok()?
                .text_trimmed_range()
                .end(),
            closing_l_angle(element)?.text_trimmed_range().start(),
        );
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                content_range,
                markup! {
                    "The content of this element is replaced by the "<Emphasis>{name}</Emphasis>" directive."
                },
            )
            .detail(
                directive.range(),
                markup! { "This directive replaces the content of the element." },
            )
            .note(markup! {
                "Vue discards this content, so it never appears on the page."
            })
            .note(markup! {
                "Remove the content, or remove "<Emphasis>{name}</Emphasis>" and keep the content."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _directive: &Self::State) -> Option<HtmlRuleAction> {
        let element = ctx.query();
        let r_angle = element.opening_element().ok()?.r_angle_token().ok()?;
        let l_angle = closing_l_angle(element)?;

        let mut mutation = ctx.root().begin();
        mutation.replace_node(element.children(), make::html_element_list([]));
        // Whitespace and comments next to the tags are trivia of these tokens.
        mutation.replace_token_discard_trivia(r_angle.clone(), r_angle.with_trailing_trivia([]));
        mutation.replace_token_discard_trivia(l_angle.clone(), l_angle.with_leading_trivia([]));
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the content of the element." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns `true` if Vue replaces the content of the element of `directive`.
///
/// `additional_directives` are names without the `v-` prefix.
fn replaces_content(directive: &VueDirective, additional_directives: &[Box<str>]) -> bool {
    if directive.is_html() || directive.is_text() {
        return true;
    }
    directive.name_token().is_ok_and(|name| {
        name.text_trimmed()
            .strip_prefix("v-")
            .is_some_and(|name| additional_directives.iter().any(|d| d.as_ref() == name))
    })
}

/// Returns the `<` token of the closing tag of `element`.
fn closing_l_angle(element: &HtmlElement) -> Option<HtmlSyntaxToken> {
    element.closing_element().ok()?.l_angle_token().ok()
}

/// Returns `true` if the trivia after the `>` of the opening tag or before the
/// `<` of the closing tag contains a comment.
///
/// Comments are trivia, so an element whose only content is a comment has no
/// children, and the comment is attached to one of these two tokens.
fn has_comments(r_angle: &HtmlSyntaxToken, l_angle: &HtmlSyntaxToken) -> bool {
    r_angle
        .trailing_trivia()
        .pieces()
        .chain(l_angle.leading_trivia().pieces())
        .any(|piece| piece.is_comments())
}
