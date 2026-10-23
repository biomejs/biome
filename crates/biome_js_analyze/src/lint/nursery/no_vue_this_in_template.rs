use crate::JsRuleAction;
use crate::lint::complexity::use_arrow_function::AnyThisScope;
use crate::services::vue::VueSemantic;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsExpression, AnyJsMemberExpression, JsSyntaxKind, JsThisExpression,
    assign_ext::AnyJsMemberAssignment, static_value::StaticValue,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, BatchMutationExt, declare_node_union};
use biome_rule_options::no_vue_this_in_template::NoVueThisInTemplateOptions;
use biome_unicode_table::is_js_ident;
use biome_vue_semantic::{SnippetHost, SymbolKind, is_template_global};

declare_lint_rule! {
    /// Disallow `this` in Vue templates.
    ///
    /// Vue templates can read the properties of the component directly. Writing `this.count`
    /// in a template does the same thing as writing `count`, so `this` only adds noise.
    ///
    /// The rule doesn't report `this.name` when writing `name` alone would mean something else,
    /// for example when a `v-for` or `v-slot` variable, or a function parameter, has the same name.
    ///
    /// :::note
    /// This rule reads Vue templates, which requires
    /// [`html.experimentalFullSupportEnabled`](https://biomejs.dev/reference/configuration/#htmlexperimentalfullsupportenabled)
    /// to be `true`. Without that setting, the rule reports nothing.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///   <a :href="this.url">{{ text }}</a>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///   <p>{{ this.text }}</p>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///   <button @click="this.count++">Add</button>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <a :href="url">{{ text }}</a>
    ///   <button @click="count++">Add</button>
    ///   <!-- `item` alone would be the `v-for` variable. -->
    ///   <p v-for="item in items">{{ this.item }}</p>
    /// </template>
    /// ```
    pub NoVueThisInTemplate {
        version: "next",
        name: "noVueThisInTemplate",
        language: "js",
        sources: &[RuleSource::EslintVueJs("this-in-template").inspired()],
        recommended: true,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Vue],
    }
}

declare_node_union! {
    pub AnyJsMember = AnyJsMemberExpression | AnyJsMemberAssignment
}

impl Rule for NoVueThisInTemplate {
    type Query = VueSemantic<AnyJsMember>;
    /// The name of the member read from `this`.
    type State = StaticValue;
    type Signals = Option<Self::State>;
    type Options = NoVueThisInTemplateOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let member = ctx.query();
        let snippet = ctx.snippet()?;
        if !matches!(
            snippet.host(),
            SnippetHost::Interpolation(_)
                | SnippetHost::DirectiveValue(_)
                | SnippetHost::VForIterable(_)
        ) {
            return None;
        }

        let object = member.object().ok()?.omit_parentheses();
        let this = JsThisExpression::cast_ref(object.syntax())?;
        // A function written in the template has its own `this`, which isn't the component.
        let this_scope = this
            .syntax()
            .ancestors()
            .skip(1)
            .find(|node| AnyThisScope::can_cast(node.kind()));
        if this_scope.is_some_and(|scope| scope.parent().is_some()) {
            return None;
        }

        let name = member.member_name()?;
        let text = name.text();
        if !is_js_ident(text) || is_reserved(text) || is_template_global(text) {
            return None;
        }
        // In an event handler, `$event` alone is the event, not a property of the component.
        if text == "$event" && ctx.source_type::<JsFileSource>().is_vue_event_handler() {
            return None;
        }
        // A variable of the template expression itself, such as a function parameter.
        // `SemanticModel::scope` can't be used: when an arrow function makes up the whole
        // expression, its scope has the same range as the scope of the expression, and the
        // lookup returns the latter.
        let range = member.range();
        let is_shadowed_in_expression = ctx
            .model()
            .global_scope()
            .descendents()
            .filter(|scope| scope.range().contains_range(range))
            .any(|scope| scope.get_binding(text).is_some());
        if is_shadowed_in_expression {
            return None;
        }
        let is_shadowed_in_template = snippet.lookup(text).is_some_and(|symbol| {
            matches!(symbol.kind(), SymbolKind::VForAlias | SymbolKind::SlotParam)
        });
        if is_shadowed_in_template {
            return None;
        }

        Some(name)
    }

    fn diagnostic(ctx: &RuleContext<Self>, name: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Unnecessary "<Emphasis>"this"</Emphasis>" in a Vue template."
                },
            )
            .note(markup! {
                "Templates can read the properties of the component directly, so "<Emphasis>"this."{name.text()}</Emphasis>" is the same as "<Emphasis>{name.text()}</Emphasis>"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, name: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        match ctx.query() {
            AnyJsMember::AnyJsMemberExpression(member) => {
                let identifier = make::js_identifier_expression(make::js_reference_identifier(
                    make::ident(name.text()),
                ));
                mutation.replace_node_transfer_trivia(
                    AnyJsExpression::from(member.clone()),
                    identifier.into(),
                )?;
            }
            AnyJsMember::AnyJsMemberAssignment(member) => {
                let identifier = make::js_identifier_assignment(make::ident(name.text()));
                mutation.replace_node_transfer_trivia(
                    AnyJsAssignment::cast_ref(member.syntax())?,
                    identifier.into(),
                )?;
            }
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove "<Emphasis>"this"</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

impl AnyJsMember {
    fn object(&self) -> biome_rowan::SyntaxResult<AnyJsExpression> {
        match self {
            Self::AnyJsMemberExpression(member) => member.object(),
            Self::AnyJsMemberAssignment(member) => member.object(),
        }
    }

    /// Returns the name of the member, when it is written as a name or a string literal.
    fn member_name(&self) -> Option<StaticValue> {
        match self {
            Self::AnyJsMemberExpression(member) => member.member_name(),
            Self::AnyJsMemberAssignment(AnyJsMemberAssignment::JsStaticMemberAssignment(
                member,
            )) => Some(StaticValue::String(
                member.member().ok()?.as_js_name()?.value_token().ok()?,
            )),
            Self::AnyJsMemberAssignment(AnyJsMemberAssignment::JsComputedMemberAssignment(
                member,
            )) => match member.member().ok()?.omit_parentheses().as_static_value()? {
                value @ StaticValue::String(_) => Some(value),
                _ => None,
            },
        }
    }
}

/// Returns `true` when `name` can't be used as a variable name in a template expression.
fn is_reserved(name: &str) -> bool {
    JsSyntaxKind::from_keyword(name).is_some_and(JsSyntaxKind::is_non_contextual_keyword)
        || RESERVED_NAMES.contains(&name)
}

/// Names that aren't keywords, but that can't be read from the component without `this`.
const RESERVED_NAMES: &[&str] = &["arguments", "await", "eval"];
