use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, VisitableType,
};
use biome_js_factory::make::jsx_attribute_list;
use biome_rowan::BatchMutationExt;
use std::{cmp::Ordering, str::FromStr};

use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_diagnostics::Applicability;
use biome_js_syntax::{AnyJsxAttribute, JsxAttribute, JsxAttributeList};
use biome_rowan::{AstNode, TextRange};
use serde::{de::IntoDeserializer, Deserialize, Serialize};

use crate::JsRuleAction;

declare_rule! {
    /// Enforce props sorting in JSX elements.
    ///
    /// This rule checks if the JSX props are sorted in a consistent way.
    /// A spread prop resets the sorting order.
    /// The rule can be configured to sort props alphabetically, ignore case, and more.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// <Hello lastName="Smith" firstName="John" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// <Hello firstName="John" lastName="Smith" />;
    /// <Hello tel={5555555} {...this.props} firstName="John" lastName="Smith" />;
    /// ```
    ///
    /// ## Options
    ///
    /// ### `callbacksLast`
    ///
    /// When `true`, callback props are sorted last.
    ///
    /// #### Example
    ///
    /// ```js
    /// <Hello tel={5555555} onClick={this._handleClick} />;
    /// ```
    ///
    /// ### `shorthand`
    ///
    /// When `first`, shorthand props are sorted first.
    /// When `last`, shorthand props are sorted last, unless `callbacksLast` is `true`,
    /// in which case they are sorted before callbacks.
    /// Default is `ignore`.
    ///
    /// #### Example
    ///
    /// ```js
    /// // shorthand first
    /// <Hello active validate name="John" tel={5555555} />;
    /// // shorthand last
    /// <Hello name="John" tel={5555555} active validate />;
    /// ```
    ///
    /// ### `multiline`
    ///
    /// When `first`, multiline props are sorted first, unless `shorthand` is `first`,
    /// in which case they are sorted after shorthand props.
    /// When `last`, multiline props are sorted last, unless `shorthand` is `last` or `callbacksLast` is `true`,
    /// in which case they are sorted before shorthand props or callbacks.
    /// Default is `ignore`.
    ///
    /// #### Example
    ///
    /// ```js
    /// // multiline first
    /// <Hello
    ///   classes={{
    ///     greetings: classes.greetings,
    ///   }}
    ///   name="John"
    ///   tel={5555555}
    ///   active
    ///   validate
    /// />;
    /// // multiline last
    /// <Hello
    ///   name="John"
    ///   tel={5555555}
    ///   active
    ///   validate
    ///   classes={{
    ///     greetings: classes.greetings,
    ///   }}
    /// />;
    /// ```
    ///
    pub UseJsxSortProps {
        version: "next",
        name: "useJsxSortProps",
        sources: &[RuleSource::EslintReact("jsx-sort-props")],
        source_kind: RuleSourceKind::SameLogic,
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
    #[serde(default, skip_serializing_if = "is_default")]
    reserved_first: ReservedFirstBehavior,
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum ReservedFirstBehavior {
    Enabled(bool),
    ReservedProps(Vec<ReservedProps>),
}

impl Default for ReservedFirstBehavior {
    fn default() -> Self {
        ReservedFirstBehavior::Enabled(false)
    }
}

impl Deserializable for ReservedFirstBehavior {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        if value.visitable_type()? == VisitableType::ARRAY {
            Deserializable::deserialize(value, name, diagnostics).map(Self::ReservedProps)
        } else {
            Deserializable::deserialize(value, name, diagnostics).map(Self::Enabled)
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserializable, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum ReservedProps {
    Children,
    DangerouslySetInnerHTML,
    Key,
    Ref,
}

impl FromStr for ReservedProps {
    type Err = serde::de::value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as Deserialize>::deserialize(s.into_deserializer())
    }
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

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let props = ctx.query().clone();
        let options = ctx.options();
        let mut non_spread_props: Option<Vec<_>> = None;
        let mut new_props = Vec::new();
        for prop in props.clone() {
            match prop {
                AnyJsxAttribute::JsxAttribute(attr) => {
                    if let Some(non_spread_props) = &mut non_spread_props {
                        non_spread_props.push(attr);
                    } else {
                        non_spread_props = Some(vec![attr]);
                    }
                }
                AnyJsxAttribute::JsxSpreadAttribute(_) => {
                    if let Some(mut non_spread_props) = non_spread_props.take() {
                        non_spread_props.sort_by(compare_props(options));
                        new_props.extend(
                            non_spread_props
                                .into_iter()
                                .map(AnyJsxAttribute::JsxAttribute),
                        );
                    }
                    non_spread_props = None;
                    new_props.push(prop);
                }
            }
        }
        if let Some(mut non_spread_props) = non_spread_props {
            non_spread_props.sort_by(compare_props(options));
            new_props.extend(
                non_spread_props
                    .into_iter()
                    .map(AnyJsxAttribute::JsxAttribute),
            );
        }
        let mut mutation = ctx.root().begin();
        mutation.replace_node(props, jsx_attribute_list(new_props));

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! {
                "Sort the JSX props."
            }
            .to_owned(),
            mutation,
        })
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

        if options.reserved_first == ReservedFirstBehavior::Enabled(true) {
            if is_reserved(a, None) && !is_reserved(b, None) {
                return Ordering::Less;
            }
            if !is_reserved(a, None) && is_reserved(b, None) {
                return Ordering::Greater;
            }
        }

        if let ReservedFirstBehavior::ReservedProps(reserved) = &options.reserved_first {
            if is_reserved(a, Some(reserved)) && !is_reserved(b, Some(reserved)) {
                return Ordering::Less;
            }
            if !is_reserved(a, Some(reserved)) && is_reserved(b, Some(reserved)) {
                return Ordering::Greater;
            }
        }

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

fn is_reserved(prop: &JsxAttribute, reserved: Option<&[ReservedProps]>) -> bool {
    let Ok(prop_name) = prop.name() else {
        return false;
    };
    let prop_name = prop_name.text();
    let Ok(prop_name) = ReservedProps::from_str(&prop_name) else {
        return false;
    };
    reserved.map_or(true, |reserved| reserved.contains(&prop_name))
}

fn is_shorthand(prop: &JsxAttribute) -> bool {
    prop.initializer().is_none()
}

fn is_callback(prop: &JsxAttribute) -> bool {
    prop.name().is_ok_and(|name| name.text().starts_with("on"))
}

fn is_multiline(prop: &JsxAttribute) -> bool {
    prop.text().contains('\n')
}
