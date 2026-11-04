use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssPseudoClass, AnyCssPseudoElement, CssIdentifier, CssPseudoClassFunctionRelativeSelectorList,
    CssPseudoClassFunctionSelectorList, CssPseudoClassSelector, CssPseudoElementSelector,
    CssSyntaxToken,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, SyntaxResult, TextRange, declare_node_union};
use biome_rule_options::no_pseudo_element_in_functional_pseudo_class::NoPseudoElementInFunctionalPseudoClassOptions;

declare_lint_rule! {
    /// Disallow pseudo-elements inside `:is()`, `:where()`, `:not()` and `:has()`.
    ///
    /// These pseudo-classes only accept selectors that match elements. A pseudo-element such as
    /// `::before` isn't allowed inside them, so browsers drop the selector that contains it:
    /// `:is()` and `:where()` ignore that argument, while `:not()` and `:has()` make the whole
    /// selector invalid. Either way, the styles never apply to the pseudo-element.
    ///
    /// The rule also checks the older `:matches()`, and reports the single-colon forms `:before`,
    /// `:after`, `:first-line` and `:first-letter`, because browsers treat them as pseudo-elements
    /// too.
    ///
    /// Other functional pseudo-classes and pseudo-elements, such as `:host()` and `::slotted()`, and
    /// pseudo-classes whose name uses SCSS interpolation, aren't checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// .card:is(.featured::before) {
    ///   content: "";
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// li:not(::marker) {
    ///   color: red;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// .card:is(.featured, .pinned)::before {
    ///   content: "";
    /// }
    ///
    /// li:not(.done)::marker {
    ///   color: red;
    /// }
    /// ```
    ///
    pub NoPseudoElementInFunctionalPseudoClass {
        version: "2.6.5",
        name: "noPseudoElementInFunctionalPseudoClass",
        language: "css",
        recommended: false,
        severity: Severity::Error,
    }
}

impl Rule for NoPseudoElementInFunctionalPseudoClass {
    type Query = Ast<AnyPseudoElementLike>;
    type State = (TextRange, CssSyntaxToken);
    type Signals = Option<Self::State>;
    type Options = NoPseudoElementInFunctionalPseudoClassOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if let AnyPseudoElementLike::CssPseudoClassSelector(selector) = node
            && !is_legacy_pseudo_element(selector)
        {
            return None;
        }
        for ancestor in node.syntax().ancestors().skip(1) {
            if let Some(pseudo_class) = AnyFunctionalPseudoClass::cast_ref(&ancestor) {
                return Some((node.range(), pseudo_class.name().ok()?.value_token().ok()?));
            }
            // Other functional pseudo-classes and pseudo-elements, such as `:host()` and
            // `::slotted()`, aren't checked, even when they are inside `:is()`.
            if AnyCssPseudoClass::can_cast(ancestor.kind())
                || AnyCssPseudoElement::can_cast(ancestor.kind())
            {
                return None;
            }
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.0,
                markup! {
                    "This pseudo-element can't be used inside :"{state.1.text_trimmed()}"()."
                },
            )
            .note(markup! {
                "Pseudo-elements aren't valid inside :is(), :where(), :not() or :has(). Browsers drop the invalid selector, so these styles never apply to the pseudo-element."
            })
            .note(markup! {
                "To style the pseudo-element, put it at the end of the selector, outside the pseudo-class, for example "<Emphasis>".card:is(.featured, .pinned)::before"</Emphasis>"."
            }),
        )
    }
}

declare_node_union! {
    pub AnyPseudoElementLike = CssPseudoElementSelector | CssPseudoClassSelector
}

declare_node_union! {
    pub AnyFunctionalPseudoClass = CssPseudoClassFunctionSelectorList | CssPseudoClassFunctionRelativeSelectorList
}

impl AnyFunctionalPseudoClass {
    fn name(&self) -> SyntaxResult<CssIdentifier> {
        match self {
            Self::CssPseudoClassFunctionSelectorList(node) => node.name(),
            Self::CssPseudoClassFunctionRelativeSelectorList(node) => node.name(),
        }
    }
}

/// Pseudo-elements from CSS 2 that browsers still accept with a single colon.
const LEGACY_PSEUDO_ELEMENTS: [&str; 4] = ["after", "before", "first-letter", "first-line"];

fn is_legacy_pseudo_element(selector: &CssPseudoClassSelector) -> bool {
    let Ok(AnyCssPseudoClass::CssPseudoClassIdentifier(class)) = selector.class() else {
        return false;
    };
    class
        .name()
        .ok()
        .and_then(|name| name.as_css_identifier()?.value_token().ok())
        .is_some_and(|token| {
            LEGACY_PSEUDO_ELEMENTS
                .iter()
                .any(|legacy| token.text_trimmed().eq_ignore_ascii_case(legacy))
        })
}
