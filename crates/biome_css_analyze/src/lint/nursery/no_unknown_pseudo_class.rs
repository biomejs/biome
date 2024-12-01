use crate::{
    keywords::{WEBKIT_SCROLLBAR_PSEUDO_CLASSES, WEBKIT_SCROLLBAR_PSEUDO_ELEMENTS},
    utils::{is_custom_selector, is_known_pseudo_class, is_page_pseudo_class, vendor_prefixed},
};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::{
    CssBogusPseudoClass, CssPageSelectorPseudo, CssPseudoClassFunctionCompoundSelector,
    CssPseudoClassFunctionCompoundSelectorList, CssPseudoClassFunctionIdentifier,
    CssPseudoClassFunctionNth, CssPseudoClassFunctionRelativeSelectorList,
    CssPseudoClassFunctionSelector, CssPseudoClassFunctionSelectorList,
    CssPseudoClassFunctionValueList, CssPseudoClassIdentifier, CssPseudoElementSelector,
};
use biome_diagnostics::Severity;
use biome_rowan::{declare_node_union, AstNode, TextRange};
use biome_string_case::StrLikeExtension;

declare_lint_rule! {
    /// Disallow unknown pseudo-class selectors.
    ///
    /// For details on known pseudo-class, see the [MDN web docs](https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-classes)
    ///
    /// This rule ignores vendor-prefixed pseudo-class selectors.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a:unknown {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a:UNKNOWN {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a:hoverr {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a:hover {}
    /// ```
    ///
    /// ```css
    /// a:focus {}
    /// ```
    ///
    /// ```css
    /// :not(p) {}
    /// ```
    ///
    /// ```css
    /// input:-moz-placeholder {}
    /// ```
    ///
    pub NoUnknownPseudoClass {
        version: "1.8.0",
        name: "noUnknownPseudoClass",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("selector-pseudo-class-no-unknown")],
    }
}
declare_node_union! {
  pub AnyPseudoLike =
      CssPseudoClassFunctionCompoundSelector
      | CssPseudoClassFunctionCompoundSelectorList
      | CssPseudoClassFunctionIdentifier
      | CssPseudoClassFunctionNth
      | CssPseudoClassFunctionRelativeSelectorList
      | CssPseudoClassFunctionSelector
      | CssPseudoClassFunctionSelectorList
      | CssPseudoClassFunctionValueList
      | CssPseudoClassIdentifier
      | CssBogusPseudoClass
      | CssPageSelectorPseudo
}

fn is_webkit_pseudo_class(node: &AnyPseudoLike) -> bool {
    let mut prev_element = node.syntax().parent().and_then(|p| p.prev_sibling());
    while let Some(prev) = &prev_element {
        let maybe_selector = CssPseudoElementSelector::cast_ref(prev);
        if let Some(selector) = maybe_selector.as_ref() {
            return WEBKIT_SCROLLBAR_PSEUDO_ELEMENTS
                .contains(&selector.to_trimmed_string().trim_matches(':'));
        };
        prev_element = prev.prev_sibling();
    }

    false
}

#[derive(Debug, Clone, Copy)]
enum PseudoClassType {
    PagePseudoClass,
    WebkitScrollbarPseudoClass,
    Other,
}

pub struct NoUnknownPseudoClassSelectorState {
    class_name: String,
    span: TextRange,
    class_type: PseudoClassType,
}

impl Rule for NoUnknownPseudoClass {
    type Query = Ast<AnyPseudoLike>;
    type State = NoUnknownPseudoClassSelectorState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let pseudo_class = ctx.query();
        let (name, span) = match pseudo_class {
            AnyPseudoLike::CssBogusPseudoClass(class) => {
                Some((class.to_trimmed_string(), class.range()))
            }
            AnyPseudoLike::CssPseudoClassFunctionCompoundSelector(selector) => {
                let name = selector.name().ok()?;
                Some((name.text().to_string(), name.text_range()))
            }
            AnyPseudoLike::CssPseudoClassFunctionCompoundSelectorList(selector_list) => {
                let name = selector_list.name().ok()?;
                Some((name.text().to_string(), name.text_range()))
            }
            AnyPseudoLike::CssPseudoClassFunctionIdentifier(ident) => {
                let name = ident.name_token().ok()?;
                Some((name.text().to_string(), name.text_range()))
            }
            AnyPseudoLike::CssPseudoClassFunctionNth(func_nth) => {
                let name = func_nth.name().ok()?;
                Some((name.text().to_string(), name.text_range()))
            }
            AnyPseudoLike::CssPseudoClassFunctionRelativeSelectorList(selector_list) => {
                let name = selector_list.name_token().ok()?;
                Some((name.token_text_trimmed().to_string(), name.text_range()))
            }
            AnyPseudoLike::CssPseudoClassFunctionSelector(selector) => {
                let name = selector.name().ok()?;
                Some((name.text().to_string(), name.text_range()))
            }
            AnyPseudoLike::CssPseudoClassFunctionSelectorList(selector_list) => {
                let name = selector_list.name().ok()?;
                Some((name.text().to_string(), name.text_range()))
            }
            AnyPseudoLike::CssPseudoClassFunctionValueList(func_value_list) => {
                let name = func_value_list.name_token().ok()?;
                Some((name.text().to_string(), name.text_range()))
            }
            AnyPseudoLike::CssPseudoClassIdentifier(ident) => {
                let name = ident.name().ok()?;
                Some((name.to_trimmed_string().to_string(), name.range()))
            }
            AnyPseudoLike::CssPageSelectorPseudo(page_pseudo) => {
                let name = page_pseudo.selector().ok()?;
                Some((name.token_text_trimmed().to_string(), name.text_range()))
            }
        }?;

        let pseudo_type = match &pseudo_class {
            AnyPseudoLike::CssPageSelectorPseudo(_) => PseudoClassType::PagePseudoClass,
            _ => {
                if is_webkit_pseudo_class(pseudo_class) {
                    PseudoClassType::WebkitScrollbarPseudoClass
                } else {
                    PseudoClassType::Other
                }
            }
        };

        let lower_name = name.to_ascii_lowercase_cow();
        let lower_name = lower_name.as_ref();

        let is_valid_class = match pseudo_type {
            PseudoClassType::PagePseudoClass => is_page_pseudo_class(lower_name),
            PseudoClassType::WebkitScrollbarPseudoClass => {
                WEBKIT_SCROLLBAR_PSEUDO_CLASSES.contains(&lower_name)
                    || is_known_pseudo_class(lower_name)
            }
            PseudoClassType::Other => {
                is_custom_selector(lower_name)
                    || vendor_prefixed(lower_name)
                    || is_known_pseudo_class(lower_name)
            }
        };

        if is_valid_class {
            None
        } else {
            Some(NoUnknownPseudoClassSelectorState {
                class_name: name,
                span,
                class_type: pseudo_type,
            })
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let Self::State {
            class_name,
            span,
            class_type,
        } = state;
        let mut diag = RuleDiagnostic::new(
            rule_category!(),
            span,
            markup! {
                "Unexpected unknown pseudo-class "<Emphasis>{ class_name }</Emphasis>" "
            },
        );
        match class_type {
            PseudoClassType::PagePseudoClass => {
                diag = diag.note(markup! {
                    "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/@page">"MDN web docs"</Hyperlink>" for more details."
                });
            }
            PseudoClassType::WebkitScrollbarPseudoClass => {
                diag = diag.note(markup! {
                    "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/::-webkit-scrollbar">"MDN web docs"</Hyperlink>" for more details."
                });
            }
            PseudoClassType::Other => {
                diag = diag.note(markup! {
                    "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-classes">"MDN web docs"</Hyperlink>" for more details."
            });
            }
        };
        Some(diag)
    }
}
