use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Phases, QueryMatch, Queryable, Rule,
    RuleDiagnostic, RuleSource, Visitor,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{jsx_ext::AnyJsxElement, JsLanguage, JsSyntaxKind, JsxAttribute};
use biome_rowan::{AstNode, BatchMutationExt, WalkEvent};
use biome_string_case::StrOnlyExtension;

declare_lint_rule! {
    /// Enforce that autoFocus prop is not used on elements.
    ///
    /// Autofocusing elements can cause usability issues for sighted and non-sighted users, alike.
    /// But the autofocus attribute should be added to the element the user is expected to
    /// interact with immediately upon opening a modal dialog or popover.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <input autoFocus />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <input autoFocus="true" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <input autoFocus={"false"} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <input autoFocus={undefined} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <input />
    ///```
    ///
    /// ```jsx
    /// <div />
    ///```
    ///
    /// ```jsx
    /// <button />
    ///```
    ///
    /// ```jsx
    /// // `autoFocus` prop in user created component is valid
    /// <MyComponent autoFocus={true} />
    ///```
    ///
    /// ```jsx
    /// // `autoFocus` prop in element has `popover` attribute is valid
    /// <div popover><input autoFocus /></div>
    /// ```
    ///
    /// ```jsx
    /// // `autoFocus` prop in `dialog` is valid
    /// <dialog><input autoFocus /></dialog>
    /// ```
    ///
    /// ## Resources
    ///
    /// - [WHATWG HTML Standard, The autofocus attribute](https://html.spec.whatwg.org/multipage/interaction.html#attr-fe-autofocus)
    /// - [The accessibility of HTML 5 autofocus](https://brucelawson.co.uk/2009/the-accessibility-of-html-5-autofocus/)
    /// - [MDN Web Docs, HTMLElement: autofocus property](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/autofocus)
    ///
    pub NoAutofocus {
        version: "1.0.0",
        name: "noAutofocus",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("no-autofocus")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

fn find_kept_autofocus_mark(element: &AnyJsxElement) -> bool {
    // the check for no_autofocus can be ignored
    // 1. inside the `dialog` element
    // 2. inside the element with the popover attribute

    let is_dialog_element = match element.name_value_token() {
        Ok(syntax_token) => {
            let tag_name = String::from(syntax_token.text_trimmed());
            tag_name.to_lowercase_cow() == "dialog"
        }
        Err(_) => false,
    };

    let has_popover_attr = element.has_truthy_attribute("popover");

    is_dialog_element || has_popover_attr
}

#[derive(Default)]
struct ValidAutofocusVisitor {
    stack: Vec<(AnyJsxElement, bool)>,
}

impl Visitor for ValidAutofocusVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &biome_rowan::WalkEvent<biome_rowan::SyntaxNode<Self::Language>>,
        mut ctx: biome_analyze::VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(node) => {
                let kind = node.kind();

                match kind {
                    JsSyntaxKind::JSX_OPENING_ELEMENT => {
                        let element = AnyJsxElement::unwrap_cast(node.clone());

                        let is_hold = match self.stack.last() {
                            None => false,
                            Some((_, value)) => *value,
                        };

                        if is_hold {
                            self.stack.push((element.clone(), true));
                        } else {
                            let next_hold = find_kept_autofocus_mark(&element);
                            self.stack.push((element.clone(), next_hold));
                        }

                        ctx.match_query(ValidAutofocus(element));
                    }
                    JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT => {
                        let element = AnyJsxElement::unwrap_cast(node.clone());

                        let is_hold = match self.stack.last() {
                            None => false,
                            Some((_, value)) => *value,
                        };

                        if !is_hold {
                            ctx.match_query(ValidAutofocus(element));
                        }
                    }
                    JsSyntaxKind::JSX_CLOSING_ELEMENT => {
                        self.stack.pop();
                    }
                    _ => {}
                }
            }
            WalkEvent::Leave(_) => {}
        };
    }
}

pub struct ValidAutofocus(AnyJsxElement);

impl QueryMatch for ValidAutofocus {
    fn text_range(&self) -> biome_rowan::TextRange {
        self.0.range()
    }
}

impl Queryable for ValidAutofocus {
    type Input = Self;

    type Output = AnyJsxElement;

    type Language = JsLanguage;

    type Services = ();

    fn build_visitor(
        analyzer: &mut impl biome_analyze::AddVisitor<Self::Language>,
        _: &<Self::Language as biome_rowan::Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, ValidAutofocusVisitor::default);
    }

    fn unwrap_match(_: &biome_analyze::ServiceBag, query: &Self::Input) -> Self::Output {
        query.0.clone()
    }
}

impl Rule for NoAutofocus {
    type Query = ValidAutofocus;
    type State = JsxAttribute;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.is_custom_component() {
            return None;
        }
        node.find_attribute_by_name("autoFocus")
    }

    fn diagnostic(_ctx: &RuleContext<Self>, attr: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            attr.syntax().text_trimmed_range(),
            markup! {
                "Avoid the "<Emphasis>"autoFocus"</Emphasis>" attribute."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, attr: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        if attr.syntax().has_trailing_comments() {
            let prev_token = attr.syntax().first_token()?.prev_token()?;
            let new_token =
                prev_token.append_trivia_pieces(attr.syntax().last_trailing_trivia()?.pieces());
            mutation.replace_token_discard_trivia(prev_token, new_token);
        }
        mutation.remove_node(attr.clone());
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"autoFocus"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}
