use crate::{
    keywords::{WEBKIT_SCROLLBAR_PSEUDO_CLASSES, WEBKIT_SCROLLBAR_PSEUDO_ELEMENTS},
    utils::{is_custom_selector, is_known_pseudo_class, is_page_pseudo_class, vendor_prefixed},
};
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    CssBogusPseudoClass, CssPageSelectorPseudo, CssPseudoClassFunctionCompoundSelector,
    CssPseudoClassFunctionCompoundSelectorList, CssPseudoClassFunctionIdentifier,
    CssPseudoClassFunctionNth, CssPseudoClassFunctionRelativeSelectorList,
    CssPseudoClassFunctionSelector, CssPseudoClassFunctionSelectorList,
    CssPseudoClassFunctionValueList, CssPseudoClassIdentifier, CssPseudoElementSelector,
    CssSyntaxToken,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange, declare_node_union};
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

impl AnyPseudoLike {
    fn name_range(&self) -> Option<TextRange> {
        Some(match self {
            Self::CssBogusPseudoClass(_) => return None,
            Self::CssPseudoClassFunctionCompoundSelector(selector) => {
                let name = selector.name().ok()?;
                name.text_trimmed_range()
            }
            Self::CssPseudoClassFunctionCompoundSelectorList(selector_list) => {
                let name = selector_list.name().ok()?;
                name.text_trimmed_range()
            }
            Self::CssPseudoClassFunctionIdentifier(ident) => {
                let name = ident.name_token().ok()?;
                name.text_trimmed_range()
            }
            Self::CssPseudoClassFunctionNth(func_nth) => {
                let name = func_nth.name().ok()?;
                name.text_trimmed_range()
            }
            Self::CssPseudoClassFunctionRelativeSelectorList(selector_list) => {
                let name = selector_list.name_token().ok()?;
                name.text_trimmed_range()
            }
            Self::CssPseudoClassFunctionSelector(selector) => {
                let name = selector.name().ok()?;
                name.text_trimmed_range()
            }
            Self::CssPseudoClassFunctionSelectorList(selector_list) => {
                let name = selector_list.name().ok()?;
                name.text_trimmed_range()
            }
            Self::CssPseudoClassFunctionValueList(func_value_list) => {
                let name = func_value_list.name_token().ok()?;
                name.text_trimmed_range()
            }
            Self::CssPseudoClassIdentifier(ident) => {
                let name = ident.name().ok()?;
                name.range()
            }
            Self::CssPageSelectorPseudo(page_pseudo) => {
                let name = page_pseudo.selector().ok()?;
                name.text_trimmed_range()
            }
        })
    }

    fn name(&self) -> Option<CssSyntaxToken> {
        Some(match self {
            Self::CssBogusPseudoClass(_) => return None,
            Self::CssPseudoClassFunctionCompoundSelector(selector) => selector.name().ok()?,
            Self::CssPseudoClassFunctionCompoundSelectorList(selector_list) => {
                selector_list.name().ok()?
            }
            Self::CssPseudoClassFunctionIdentifier(ident) => ident.name_token().ok()?,
            Self::CssPseudoClassFunctionNth(func_nth) => func_nth.name().ok()?,
            Self::CssPseudoClassFunctionRelativeSelectorList(selector_list) => {
                selector_list.name_token().ok()?
            }
            Self::CssPseudoClassFunctionSelector(selector) => selector.name().ok()?,
            Self::CssPseudoClassFunctionSelectorList(selector_list) => selector_list.name().ok()?,
            Self::CssPseudoClassFunctionValueList(func_value_list) => {
                func_value_list.name_token().ok()?
            }
            Self::CssPseudoClassIdentifier(ident) => {
                let name = ident.name().ok()?;
                name.value_token().ok()?
            }
            Self::CssPageSelectorPseudo(page_pseudo) => page_pseudo.selector().ok()?,
        })
    }
}

fn is_webkit_pseudo_class(node: &AnyPseudoLike) -> bool {
    let mut prev_element = node.syntax().parent().and_then(|p| p.prev_sibling());
    while let Some(prev) = &prev_element {
        let maybe_selector = CssPseudoElementSelector::cast_ref(prev);
        if let Some(selector) = maybe_selector.as_ref() {
            return WEBKIT_SCROLLBAR_PSEUDO_ELEMENTS
                .contains(&selector.to_trimmed_text().trim_matches(':'));
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
    class_name: Box<str>,
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
        let is_css_modules = ctx.is_css_modules();
        let span = pseudo_class.name_range()?;
        let name = pseudo_class.name()?;

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

        let lower_name = name.text_trimmed().to_ascii_lowercase_cow();
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

        let is_valid_global = lower_name == "global" && is_css_modules;

        if is_valid_class || is_valid_global {
            None
        } else {
            Some(NoUnknownPseudoClassSelectorState {
                class_name: name.text_trimmed().into(),
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
