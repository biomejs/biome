use std::cmp::Ordering;

use biome_analyze::{context::RuleContext, declare_rule, Ast, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{AnyJsxAttribute, JsxAttribute, JsxAttributeList};
use biome_rowan::{AstNode, TextRange};
use serde::{Deserialize, Serialize};

declare_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding ESLint rule (if any):
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // var a = 1;
    /// ```
    ///
    pub UseJsxSortProps {
        version: "next",
        name: "useJsxSortProps",
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

#[derive(Clone, Debug, Default, Deserializable, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UseJsxSortPropsOptions {
    #[serde(default, skip_serializing_if = "is_default")]
    callbacks_last: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    shorthand: ShorthandBehavior,
    #[serde(default, skip_serializing_if = "is_default")]
    multiline: MultilineBehavior,
    #[serde(default, skip_serializing_if = "is_default")]
    ignore_case: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    no_sort_alphabetically: bool,
    // TODO: add reserved_first and locale options
}

#[derive(Clone, Debug, Default, Deserializable, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum MultilineBehavior {
    #[default]
    Ignore,
    First,
    Last,
}

#[derive(Clone, Debug, Default, Deserializable, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum ShorthandBehavior {
    #[default]
    Ignore,
    First,
    Last,
}

fn is_default<T: Default + Eq>(value: &T) -> bool {
    value == &T::default()
}

impl Rule for UseJsxSortProps {
    type Query = Ast<JsxAttributeList>;
    type State = TextRange;
    type Signals = Vec<Self::State>;
    type Options = UseJsxSortPropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let props = ctx.query();
        let options = ctx.options();
        let mut diagnostics = vec![];
        let mut non_spread_props: Option<Vec<_>> = None;
        for prop in props {
            match prop {
                AnyJsxAttribute::JsxAttribute(attr) => {
                    if let Some(non_spread_props) = &mut non_spread_props {
                        non_spread_props.push(attr);
                    } else {
                        non_spread_props = Some(vec![attr]);
                    }
                }
                AnyJsxAttribute::JsxSpreadAttribute(_) => {
                    if let Some(non_spread_props) = non_spread_props.take() {
                        diagnostics.extend(lint_non_spread_props(&non_spread_props, options));
                    }
                    non_spread_props = None;
                }
            }
        }
        if let Some(non_spread_props) = non_spread_props {
            diagnostics.extend(lint_non_spread_props(&non_spread_props, options));
        }
        diagnostics
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *state,
                markup! {
                    "These JSX props are not sorted."
                },
            )
            .note(markup! {
                "Use quick fix to sort them."
            }),
        )
    }
}

fn lint_non_spread_props(
    props: &[JsxAttribute],
    options: &UseJsxSortPropsOptions,
) -> Option<TextRange> {
    let mut sorted_props = props.to_vec();
    sorted_props.sort_by(compare_props(options));
    for (i, prop) in props.iter().enumerate() {
        if prop.name().ok()?.text() != sorted_props[i].name().ok()?.text() {
            return Some(TextRange::new(
                props.first()?.range().start(),
                props.last()?.range().end(),
            ));
        }
    }
    None
}

fn compare_props(
    options: &UseJsxSortPropsOptions,
) -> impl FnMut(&JsxAttribute, &JsxAttribute) -> Ordering + '_ {
    |a: &JsxAttribute, b: &JsxAttribute| -> Ordering {
        let (Ok(a_name), Ok(b_name)) = (a.name(), b.name()) else {
            return Ordering::Equal;
        };
        let (a_name, b_name) = (a_name.text(), b_name.text());

        if options.callbacks_last {
            if is_callback(a) && !is_callback(b) {
                return Ordering::Greater;
            }
            if !is_callback(a) && is_callback(b) {
                return Ordering::Less;
            }
        }

        if options.shorthand == ShorthandBehavior::First {
            if is_shorthand(a) && !is_shorthand(b) {
                return Ordering::Less;
            }
            if !is_shorthand(a) && is_shorthand(b) {
                return Ordering::Greater;
            }
        }

        if options.shorthand == ShorthandBehavior::Last {
            if is_shorthand(a) && !is_shorthand(b) {
                return Ordering::Greater;
            }
            if !is_shorthand(a) && is_shorthand(b) {
                return Ordering::Less;
            }
        }

        if options.multiline == MultilineBehavior::First {
            if is_multiline(a) && !is_multiline(b) {
                return Ordering::Less;
            }
            if !is_multiline(a) && is_multiline(b) {
                return Ordering::Greater;
            }
        }

        if options.multiline == MultilineBehavior::Last {
            if is_multiline(a) && !is_multiline(b) {
                return Ordering::Greater;
            }
            if !is_multiline(a) && is_multiline(b) {
                return Ordering::Less;
            }
        }

        if options.no_sort_alphabetically {
            return Ordering::Equal;
        }

        if options.ignore_case {
            return a_name.to_lowercase().cmp(&b_name.to_lowercase());
        }
        a_name.cmp(&b_name)
    }
}

fn is_shorthand(prop: &JsxAttribute) -> bool {
    prop.initializer().is_some()
}

fn is_callback(prop: &JsxAttribute) -> bool {
    prop.name().is_ok_and(|name| name.text().starts_with("on"))
}

fn is_multiline(prop: &JsxAttribute) -> bool {
    prop.text().contains('\n')
}
