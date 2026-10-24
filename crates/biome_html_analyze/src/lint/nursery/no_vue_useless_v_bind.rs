use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_factory::make;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyHtmlAttributeInitializer, AnyVueDirective, HtmlAttributeInitializerClause,
    HtmlSyntaxKind, HtmlSyntaxToken, T, VueDirective, VueVBindShorthandDirective,
};
use biome_rowan::{AstNode, BatchMutationExt, declare_node_union};
use biome_rule_options::no_vue_useless_v_bind::NoVueUselessVBindOptions;
use biome_vue_semantic::{DirectiveKind, Element, RootShape, Snippet};

use crate::{HtmlRuleAction, VueSemantic};

declare_lint_rule! {
    /// Disallow `v-bind` directives whose value is a fixed string.
    ///
    /// A `v-bind` directive, written as `v-bind:name` or `:name`, sets an attribute to the result of
    /// a JavaScript expression. When that expression is a fixed string, such as `'foo'`, the
    /// directive always produces the same value. A plain attribute like `id="foo"` does the same
    /// thing and is easier to read.
    ///
    /// The rule doesn't report directives with modifiers, such as `:id.prop="'foo'"`, because
    /// modifiers change how Vue applies the value. It also skips elements inside a `v-pre`
    /// element, where Vue leaves the template as written.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div :id="'foo'" />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-bind:title="`Hello`" />
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <div id="foo" />
    ///   <div :id="foo" />
    ///   <div :id="'foo' + bar" />
    ///   <div :id="`foo-${bar}`" />
    ///   <div :id.prop="'foo'" />
    /// </template>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `ignoreComments`
    ///
    /// Default: `false`
    ///
    /// When `true`, the rule doesn't report a directive whose value contains a comment.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignoreComments": true
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///   <div :id="'foo'" />
    /// </template>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <div :id="'foo' /* used by the tests */" />
    /// </template>
    /// ```
    ///
    /// ### `ignoreStringEscapes`
    ///
    /// Default: `false`
    ///
    /// When `true`, the rule doesn't report a directive whose string contains an escape sequence
    /// for a special character, such as `\n` for a line break or `\x41` for a character code. A
    /// plain attribute can't contain these escape sequences.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignoreStringEscapes": true
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///   <div :title="'It\'s here'" />
    /// </template>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <div :title="'Line 1\nLine 2'" />
    /// </template>
    /// ```
    ///
    pub NoVueUselessVBind {
        version: "next",
        name: "noVueUselessVBind",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-useless-v-bind").inspired()],
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyVueBindDirective = VueDirective | VueVBindShorthandDirective
}

impl Rule for NoVueUselessVBind {
    type Query = VueSemantic<AnyVueBindDirective>;
    /// The snippet holding the value of the reported directive.
    type State = Snippet;
    type Signals = Option<Self::State>;
    type Options = NoVueUselessVBindOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attr = ctx.model().attr_at(ctx.query().range())?;
        if attr.directive_kind()? != DirectiveKind::Bind
            || attr.static_arg().is_none()
            || attr.modifiers().next().is_some()
        {
            return None;
        }
        let snippet = attr.value_snippet()?;
        if !matches!(snippet.root(), RootShape::String(_)) {
            return None;
        }
        // Vue doesn't compile anything inside a `v-pre` element, so there the
        // directive is a plain attribute named `:id` or `v-bind:id`.
        if std::iter::successors(Some(attr.element()), Element::parent)
            .any(|element| element.directive(DirectiveKind::Pre).is_some())
        {
            return None;
        }

        let options = ctx.options();
        if options.ignore_comments() && snippet.has_comments() {
            return None;
        }
        if options.ignore_string_escapes() && snippet.string_value().is_none() {
            return None;
        }
        Some(snippet)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _snippet: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>"v-bind"</Emphasis>" directive binds a fixed string."
                },
            )
            .note(markup! {
                "A plain attribute gives the same result without "<Emphasis>"v-bind"</Emphasis>", and is easier to read."
            })
            .note(markup! {
                "Remove "<Emphasis>"v-bind"</Emphasis>" and write the string as the attribute value."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, snippet: &Self::State) -> Option<HtmlRuleAction> {
        if snippet.has_comments() {
            return None;
        }
        let value = snippet.string_value()?;
        let attr = snippet.attr()?;
        let name = attr.static_arg()?;
        // Writing a second attribute with the same name would make the
        // element invalid.
        if attr
            .element()
            .attrs()
            .any(|other| other.plain_name() == Some(name))
        {
            return None;
        }

        let node = ctx.query();
        let AnyHtmlAttributeInitializer::HtmlString(string) = node.initializer()?.value().ok()?
        else {
            return None;
        };
        let value_token = if string.value_token().ok()?.text_trimmed().starts_with('\'') {
            make::html_string_literal_single_quotes(&value.replace('\'', "&apos;"))
        } else {
            make::html_string_literal(&value.replace('"', "&quot;"))
        };
        let name_token = HtmlSyntaxToken::new_detached(HtmlSyntaxKind::HTML_LITERAL, name, [], []);
        let attribute = make::html_attribute(make::html_attribute_name(name_token))
            .with_initializer(make::html_attribute_initializer_clause(
                make::token(T![=]),
                AnyHtmlAttributeInitializer::HtmlString(make::html_string(value_token)),
            ))
            .build();

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyHtmlAttribute::AnyVueDirective(node.clone().into()),
            AnyHtmlAttribute::HtmlAttribute(attribute),
        );
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace the directive with a plain attribute." }.to_owned(),
            mutation,
        ))
    }
}

impl AnyVueBindDirective {
    fn initializer(&self) -> Option<HtmlAttributeInitializerClause> {
        match self {
            Self::VueDirective(directive) => directive.initializer(),
            Self::VueVBindShorthandDirective(directive) => directive.initializer(),
        }
    }
}

impl From<AnyVueBindDirective> for AnyVueDirective {
    fn from(directive: AnyVueBindDirective) -> Self {
        match directive {
            AnyVueBindDirective::VueDirective(directive) => Self::VueDirective(directive),
            AnyVueBindDirective::VueVBindShorthandDirective(directive) => {
                Self::VueVBindShorthandDirective(directive)
            }
        }
    }
}
