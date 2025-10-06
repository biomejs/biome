use crate::JsRuleAction;
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make::{
    jsx_attribute, jsx_attribute_initializer_clause, jsx_attribute_list, jsx_ident, jsx_name,
    jsx_string, jsx_string_literal, token,
};
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::{
    AnyJsxAttribute, AnyJsxAttributeName, AnyJsxAttributeValue, JsxAttribute, JsxAttributeList, T,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TriviaPieceKind};
use biome_rule_options::no_blank_target::NoBlankTargetOptions;

// Elements to check, in the form of (node name, attribute name) tuples.
//
// See: https://html.spec.whatwg.org/multipage/links.html#link-type-noopener
const CHECKED_ELEMENTS_WITH_ATTR: &[(&str, &str)] =
    &[("a", "href"), ("area", "href"), ("form", "action")];

declare_lint_rule! {
    /// Disallow `target="_blank"` attribute without `rel="noopener"`.
    ///
    /// When creating an anchor `a` element, there are times when its link has
    /// to be opened in a new browser tab via the `target="_blank"` attribute.
    /// This attribute has to be paired with `rel="noopener"` or you may run
    /// into security issues.
    ///
    /// See to the [`noopener` documentation](https://html.spec.whatwg.org/multipage/links.html#link-type-noopener).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <a href='http://external.link' target='_blank'>child</a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a href='http://external.link' target='_blank' rel='nofollow'>child</a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a {...props} href='http://external.link' target='_blank' rel='nofollow'>child</a>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <a href='http://external.link' rel='noopener' target='_blank'>child</a>
    /// ```
    ///
    /// ```jsx
    /// <a href='http://external.link' rel='noreferrer' target='_blank'>child</a>
    /// ```
    ///
    /// ```jsx
    /// // The rule accepts elements with spread props, because the required
    /// // attribute may be injected dynamically:
    /// <a href='http://external.link' target='_blank' {...props}>child</a>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `allowNoReferrer`
    ///
    /// By default, `noBlankTarget` accepts both `rel="noopener"` and
    /// `rel="noreferrer"` with links that have `target="_blank"`. This is
    /// because the latter _implies_ the former, so either one is sufficient to
    /// mitigate the security risk.
    ///
    /// However, allowing `rel="noreferrer"` may still be undesirable, because
    /// it can break tracking, which may be an undesirable side-effect. As such,
    /// you can set `allowNoReferrer: false` to _only_ accept `rel="noopener"`.
    ///
    /// See to the [`noreferrer` documentation](https://html.spec.whatwg.org/multipage/links.html#link-type-noreferrer).
    ///
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allowNoReferrer": false
    ///     }
    /// }
    /// ```
    ///
    /// ```jsx,use_options,expect_diagnostic
    /// <a href='http://external.link' rel='noreferrer' target='_blank'>child</a>
    /// ```
    ///
    /// Default: `true`
    ///
    /// ### `allowDomains`
    ///
    /// The option `allowDomains` allows specific domains to use
    /// `target="_blank"` without `rel="noopener"`. In the following
    /// configuration, it's allowed to use the domains `https://example.com` and
    /// `example.org`:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allowDomains": ["https://example.com", "example.org"]
    ///     }
    /// }
    /// ```
    ///
    /// ```jsx,use_options
    /// <>
    ///   <a target='_blank' testme href='https://example.com'></a>
    ///   <a target='_blank' href='example.org'></a>
    /// </>
    /// ```
    ///
    /// The diagnostic is applied to all domains not in the allow list:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allowDomains": ["https://example.com"]
    ///     }
    /// }
    /// ```
    ///
    /// ```jsx,expect_diagnostic,use_options
    /// <>
    ///   <a target='_blank' testme href='https://example.com'></a>
    ///   <a target='_blank' href='example.org'></a>
    /// </>
    /// ```
    /// Biome doesn't check if the list contains valid URLs.
    pub NoBlankTarget {
        version: "1.0.0",
        name: "noBlankTarget",
        language: "jsx",
        sources: &[RuleSource::EslintReact("jsx-no-target-blank").inspired()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoBlankTarget {
    type Query = Ast<AnyJsxElement>;
    /// Two attributes:
    /// 1. The attribute `target=`
    /// 2. The attribute `rel=`, if present
    type State = (JsxAttribute, Option<JsxAttribute>);
    type Signals = Option<Self::State>;
    type Options = NoBlankTargetOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let href = CHECKED_ELEMENTS_WITH_ATTR
            .iter()
            .find_map(|(node_name, attr_name)| {
                (node.name_value_token().ok()?.text_trimmed() == *node_name)
                    .then(|| node.find_attribute_by_name(attr_name))
                    .flatten()
            })?;
        let href = href.as_static_value()?;

        let target_attribute = node.find_attribute_by_name("target")?;
        if target_attribute.as_static_value()?.text() != "_blank" {
            return None;
        }

        if !ctx.options().allow_domains.is_empty() {
            let allow_domains: Vec<&str> = ctx
                .options()
                .allow_domains
                .iter()
                .map(AsRef::as_ref)
                .collect();
            if is_allowed_domain(href.text(), &allow_domains) {
                return None;
            }
        }

        let rel_attribute = node.find_attribute_by_name("rel");
        match rel_attribute {
            None => (!node.has_trailing_spread_prop(&target_attribute))
                .then_some((target_attribute, None)),
            Some(rel_attribute) => {
                if rel_attribute.initializer().is_none()
                    || (!rel_attribute
                        .as_static_value()?
                        .text()
                        .split_ascii_whitespace()
                        .any(|rel| {
                            rel == "noopener"
                                || ctx.options().allow_no_referrer && rel == "noreferrer"
                        })
                        && !node.has_trailing_spread_prop(&target_attribute)
                        && !node.has_trailing_spread_prop(&rel_attribute))
                {
                    Some((target_attribute, Some(rel_attribute)))
                } else {
                    None
                }
            }
        }
    }

    fn action(
        ctx: &RuleContext<Self>,
        (target_attribute, rel_attribute): &Self::State,
    ) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let message = if let Some(rel_attribute) = rel_attribute {
            let prev_jsx_attribute = rel_attribute.initializer()?.value().ok()?;
            let prev_jsx_string = prev_jsx_attribute.as_jsx_string()?;
            let new_text = format!(
                "noopener {}",
                prev_jsx_string.inner_string_text().ok()?.text()
            );
            mutation.replace_node(
                prev_jsx_string.clone(),
                jsx_string(jsx_string_literal(new_text.trim_end())),
            );

            (markup! {
                "Add the "<Emphasis>"\"noopener\""</Emphasis>" to the existing attribute."
            })
            .to_owned()
        } else {
            let old_attribute_list = target_attribute
                .syntax()
                .ancestors()
                .find_map(JsxAttributeList::cast)?;
            let mut new_attribute_list: Vec<_> = old_attribute_list.iter().collect();
            let new_attribute = jsx_attribute(AnyJsxAttributeName::JsxName(jsx_name(
                jsx_ident("rel").with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]),
            )))
            .with_initializer(jsx_attribute_initializer_clause(
                token(T![=]),
                AnyJsxAttributeValue::JsxString(jsx_string(jsx_string_literal("noopener"))),
            ))
            .build();

            new_attribute_list.push(AnyJsxAttribute::JsxAttribute(new_attribute));

            mutation.replace_node(old_attribute_list, jsx_attribute_list(new_attribute_list));

            (markup! {
                "Add the "<Emphasis>"rel=\"noopener\""</Emphasis>" attribute."
            })
            .to_owned()
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message,
            mutation,
        ))
    }

    fn diagnostic(
        ctx: &RuleContext<Self>,
        (target_attribute, _): &Self::State,
    ) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            target_attribute.syntax().text_trimmed_range(),
            if ctx.options().allow_no_referrer {
                markup! {
                    "Avoid using "<Emphasis>"target=\"_blank\""</Emphasis>
                    " without "<Emphasis>"rel=\"noopener\""</Emphasis>" or "
                    <Emphasis>"rel=\"noreferrer\""</Emphasis>"."
                }
            } else {
                markup! {
                    "Avoid using "<Emphasis>"target=\"_blank\""</Emphasis>
                    " without "<Emphasis>"rel=\"noopener\""</Emphasis>"."
                }
            }
        ).note(
            markup!{
                "Opening external links in new tabs without rel=\"noopener\" is a security risk. See \
                "<Hyperlink href="https://html.spec.whatwg.org/multipage/links.html#link-type-noopener">"the explanation"</Hyperlink>" for more details."
            }
        ))
    }
}

fn is_allowed_domain(href: &str, allow_domains: &[&str]) -> bool {
    allow_domains
        .iter()
        .any(|allowed| href.starts_with(allowed) || href.contains(allowed))
}
