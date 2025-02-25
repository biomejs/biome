use crate::JsRuleAction;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
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
use serde::{Deserialize, Serialize};

declare_lint_rule! {
    /// Disallow `target="_blank"` attribute without `rel="noreferrer"`
    ///
    /// When creating anchor `a` element, there are times when its link has to be opened in a new browser tab
    /// via `target="_blank"` attribute. This attribute has to paired with `rel="noreferrer"` or you're incur
    /// in a security issue.
    ///
    /// Refer to [the noreferrer documentation](https://html.spec.whatwg.org/multipage/links.html#link-type-noreferrer)
    /// and the [the noopener documentation](https://html.spec.whatwg.org/multipage/links.html#link-type-noopener)
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
    /// <a href='http://external.link' target='_blank' rel='noopener'>child</a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a {...props} href='http://external.link' target='_blank' rel='noopener'>child</a>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <a href='http://external.link' rel='noreferrer' target='_blank'>child</a>
    /// ```
    ///
    /// ```jsx
    /// <a href='http://external.link' target='_blank' rel='noopener' {...props}>child</a>
    /// ```
    ///
    /// ## Options
    ///
    /// The option `allowDomains` allows specific domains to use `target="_blank"` without `rel="noreferrer"`.
    /// In the following configuration, it's allowed to use the domains `https://example.com` and `example.org`:
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
        sources: &[RuleSource::EslintReact("jsx-no-target-blank")],
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
    type Options = AllowDomainOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.name_value_token().ok()?.text_trimmed() != "a"
            || node.find_attribute_by_name("href").is_none()
        {
            return None;
        }

        let target_attribute = node.find_attribute_by_name("target")?;
        let rel_attribute = node.find_attribute_by_name("rel");

        if target_attribute.as_static_value()?.text() == "_blank" {
            if !ctx.options().allow_domains.is_empty() {
                let href_attribute = node.find_attribute_by_name("href")?;
                if let Some(href_value) = href_attribute.as_static_value() {
                    let href = href_value.text();
                    let allow_domains: Vec<&str> = ctx
                        .options()
                        .allow_domains
                        .iter()
                        .map(AsRef::as_ref)
                        .collect();
                    if is_allowed_domain(href, &allow_domains) {
                        return None;
                    }
                }
            }

            match rel_attribute {
                None => {
                    if !node.has_trailing_spread_prop(&target_attribute) {
                        return Some((target_attribute, None));
                    }
                }
                Some(rel_attribute) => {
                    if rel_attribute.initializer().is_none()
                        || (!rel_attribute
                            .as_static_value()?
                            .text()
                            .split_ascii_whitespace()
                            .any(|f| f == "noreferrer")
                            && !node.has_trailing_spread_prop(&target_attribute)
                            && !node.has_trailing_spread_prop(&rel_attribute))
                    {
                        return Some((target_attribute, Some(rel_attribute)));
                    }
                }
            }
        }

        None
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
                "noreferrer {}",
                prev_jsx_string.inner_string_text().ok()?.text()
            );
            mutation.replace_node(
                prev_jsx_string.clone(),
                jsx_string(jsx_string_literal(new_text.trim_end())),
            );

            (markup! {
                "Add the "<Emphasis>"\"noreferrer\""</Emphasis>" to the existing attribute."
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
                AnyJsxAttributeValue::JsxString(jsx_string(jsx_string_literal("noreferrer"))),
            ))
            .build();

            new_attribute_list.push(AnyJsxAttribute::JsxAttribute(new_attribute));

            mutation.replace_node(old_attribute_list, jsx_attribute_list(new_attribute_list));

            (markup! {
                "Add the "<Emphasis>"rel=\"noreferrer\""</Emphasis>" attribute."
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
        _ctx: &RuleContext<Self>,
        (target_attribute, _): &Self::State,
    ) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            target_attribute.syntax().text_trimmed_range(),
            markup! {
                "Avoid using "<Emphasis>"target=\"_blank\""</Emphasis>" without "<Emphasis>"rel=\"noreferrer\""</Emphasis>"."
            },
        ).note(
            markup!{
                "Opening external links in new tabs without rel=\"noreferrer\" is a security risk. See \
                "<Hyperlink href="https://html.spec.whatwg.org/multipage/links.html#link-type-noopener">"the explanation"</Hyperlink>" for more details."
            }
        ))
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct AllowDomainOptions {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// List of domains to allow `target="_blank"` without `rel="noreferrer"`
    pub allow_domains: Vec<String>,
}

fn is_allowed_domain(href: &str, allow_domains: &[&str]) -> bool {
    allow_domains
        .iter()
        .any(|allowed| href.starts_with(allowed) || href.contains(allowed))
}
