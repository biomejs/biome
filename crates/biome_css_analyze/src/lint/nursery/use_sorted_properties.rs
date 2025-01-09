use std::{
    borrow::Cow,
    collections::{BTreeSet, HashSet},
};

use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssDeclarationName, AnyCssDeclarationOrRule, AnyCssProperty, AnyCssRule,
    CssDeclarationOrRuleBlock, CssDeclarationOrRuleList, CssDeclarationWithSemicolon,
    CssSyntaxKind,
};
use biome_rowan::{AstNode, BatchMutationExt, NodeOrToken, SyntaxNode, TokenText};
use biome_string_case::StrOnlyExtension;

use crate::{
    keywords::VENDOR_PREFIXES,
    order::PROPERTY_ORDER_MAP,
    utils::{get_longhand_sub_properties, get_reset_to_initial_properties, vender_prefix},
    CssRuleAction,
};

declare_lint_rule! {
    /// Enforce ordering of CSS properties and nested rules.
    ///
    /// This rule ensures the contents of a CSS rule are ordered consistantly.
    ///
    /// Properties are ordered semantically, with more important properties near the top and
    /// similar properties grouped together. Nested rules and at-rules are placed after properties.
    ///
    /// The ordering is roughly:
    /// 1. Custom properties
    /// 1. Layout properties (display, flex, grid)
    /// 1. Margin and padding properties
    /// 1. Typography properties (font, color)
    /// 1. Interaction properties (pointer-events, visibility)
    /// 1. Background and border properties
    /// 1. Transition and animation properties
    /// 1. Nested rules, media queries and other at-rules
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   transition: opactity 1s ease;
    ///   border: 1px solid black;
    ///   pointer-events: none;
    ///   color: black;
    ///   margin: 8px;
    ///   display: block;
    ///   --custom: 100;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   span { color: blue; }
    ///   color: red;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// p {
    ///   --custom:·100;
    ///   display:·block;
    ///   margin:·8px;
    ///   color: black;
    ///   pointer-events:·none;
    ///   border:·1px·solid·black;
    ///   transition:·opactity·1s·ease;
    /// }
    /// ```
    ///
    /// ```css
    /// p {
    ///   color: red;
    ///   span { color: blue; }
    /// }
    /// ```
    ///
    pub UseSortedProperties {
        version: "next",
        name: "useSortedProperties",
        language: "css",
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

pub struct UseSortedPropertiesState {
    /// The containing node
    block: CssDeclarationOrRuleBlock,
    /// The items to be sorted in their original order
    original_properties: Vec<AnyCssDeclarationOrRule>,
    /// The items in their sorted order, or [None] if sorting would be unsafe or if the rule contains unknown items.
    sorted_properties: Option<RecessOrderProperties>,
}

impl Rule for UseSortedProperties {
    type Query = Ast<CssDeclarationOrRuleBlock>;
    type State = UseSortedPropertiesState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        let original_properties = node
            .items()
            .into_iter()
            .collect::<Vec<AnyCssDeclarationOrRule>>();

        if contains_shorthand_after_longhand(&original_properties)
            || contains_unknown_property(&original_properties)
        {
            // This would be unsafe to sort
            return Some(UseSortedPropertiesState {
                block: node.clone(),
                original_properties,
                sorted_properties: None,
            });
        }

        let sorted_properties = Some(RecessOrderProperties::new(&original_properties));
        Some(UseSortedPropertiesState {
            block: node.clone(),
            original_properties,
            sorted_properties,
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let Some(sorted_properties) = &state.sorted_properties else {
            return None;
        };
        if sorted_properties.is_sorted() {
            return None;
        }

        return Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.block.range(),
                markup! {
                    "Properties can be sorted."
                },
            )
            .note(markup! {
                "Consistently ordering CSS properties can improve readability."
            }),
        );
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<CssRuleAction> {
        let Some(sorted_properties) = &state.sorted_properties else {
            return None;
        };
        if sorted_properties.is_sorted() {
            return None;
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            state.block.items(),
            sorted_properties.as_sorted(&state.original_properties)?,
        );

        return Some(CssRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Sort these properties" }.to_owned(),
            mutation,
        ));
    }
}

/// "Recess order" is a semantic ordering that mimics the behavior of [stylelint-config-recess-order](https://github.com/stormwarning/stylelint-config-recess-order).
///
/// Which in turn mimics the behavior of twitter's [RECESS](https://github.com/twitter-archive/recess/blob/29bccc870b7b4ccaa0a138e504caf608a6606b59/lib/lint/strict-property-order.js).
pub struct RecessOrderProperties(pub BTreeSet<SortInfo>);

impl RecessOrderProperties {
    pub fn new(items: &[AnyCssDeclarationOrRule]) -> Self {
        RecessOrderProperties(
            items
                .iter()
                .enumerate()
                .map(|(position, node)| {
                    let mut sort_info = SortInfo::from(node);
                    sort_info.original_position = position;
                    sort_info
                })
                .collect::<BTreeSet<SortInfo>>(),
        )
    }

    pub fn is_sorted(&self) -> bool {
        // The list is sorted if the original_position field equals the actual position for every item
        self.0
            .iter()
            .enumerate()
            .all(|(position, item)| position == item.original_position)
    }

    pub fn as_sorted(
        &self,
        original_items: &[AnyCssDeclarationOrRule],
    ) -> Option<CssDeclarationOrRuleList> {
        CssDeclarationOrRuleList::cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_DECLARATION_OR_RULE_LIST,
            self.0.iter().map(|sort_info| {
                original_items
                    .get(sort_info.original_position)
                    .map(|node| NodeOrToken::Node(node.clone().into_syntax()))
            }),
        ))
    }
}

fn css_declaration_to_prop_text(
    decl_with_semicolon: &CssDeclarationWithSemicolon,
) -> Option<TokenText> {
    decl_with_semicolon
        .declaration()
        .ok()?
        .property()
        .ok()?
        .as_css_generic_property()?
        .name()
        .ok()
        .and_then(|prop_name| {
            if let AnyCssDeclarationName::CssIdentifier(ident) = prop_name {
                Some(ident.value_token().ok()?.token_text_trimmed())
            } else {
                None
            }
        })
}

/// Returns a declaration's vendor prefix in lowercase
fn prop_text_to_prefix(tok_text: &TokenText) -> Option<&'static str> {
    let prop_lowercase = tok_text.text().to_lowercase_cow();
    let prefix = vender_prefix(&prop_lowercase);
    if prefix.is_empty() {
        return None;
    }
    Some(prefix)
}

/// Returns a declaration's property name without vendor prefix in lowercase.
fn prop_text_to_unprefixed(tok_text: &TokenText) -> Cow<'_, str> {
    let prop_lowercase = tok_text.text().to_lowercase_cow();
    let prefix = vender_prefix(&prop_lowercase);
    if prefix.is_empty() {
        return prop_lowercase;
    }
    let unprefixed = match &prop_lowercase {
        Cow::Borrowed(s) => s.strip_prefix(prefix).map(Cow::Borrowed),
        Cow::Owned(s) => s
            .strip_prefix(prefix)
            .map(|stripped| Cow::Owned(stripped.to_owned())),
    };
    unprefixed.unwrap_or(prop_lowercase)
}

/// Defines the ordering of different kinds of css nodes, e.g. declaration nodes should be before at-rules.
// The order of these enum members controls the sort order of the css nodes.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum NodeKindOrder {
    /// e.g. --custom-property: red;
    CustomProperty,
    /// e.g. composes: .rule;
    ComposesProperty,
    /// e.g. color: red;
    Declaration,
    /// CSS declarations that aren't in [PROPERTY_ORDER_MAP], e.g. abc: red;
    UnknownDeclaration,
    /// Nested items, e.g. .nested {}
    NestedRuleOrAtRule,
    /// Everything else
    UnknownKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
/// Defines sort order using lexographical sorting of this struct's fields
pub struct SortInfo {
    /// Nodes are ordered by their node kind, in the order defined by [NodeKindOrder].
    kind: NodeKindOrder,
    /// Property nodes are sorted by the value stored in [PROPERTY_ORDER_MAP]. This value is the index at which the property appears in the [crate::order::PROPERTY_ORDER] array.
    property_index: usize,
    /// Vendor prefixes are sorted by the index at which they appear in [VENDOR_PREFIXES]. Non-prefixed properties go after prefixed properties.
    vendor_prefix_index: usize,
    /// The index of this node within its containing rule, to make the sort stable. Additionally used to retrieve the original node in [RecessOrderProperties::as_sorted].
    original_position: usize,
}

impl SortInfo {
    fn from_kind(kind: NodeKindOrder) -> Self {
        Self {
            kind,
            property_index: 0,
            vendor_prefix_index: 0,
            original_position: 0,
        }
    }
    fn from_declaration(property_index: usize, vendor_prefix_index: usize) -> Self {
        Self {
            kind: NodeKindOrder::Declaration,
            property_index,
            vendor_prefix_index,
            original_position: 0,
        }
    }
    fn unknown() -> Self {
        Self {
            kind: NodeKindOrder::UnknownKind,
            property_index: 0,
            vendor_prefix_index: 0,
            original_position: 0,
        }
    }
}

impl From<&AnyCssDeclarationOrRule> for SortInfo {
    fn from(node: &AnyCssDeclarationOrRule) -> SortInfo {
        match node {
            AnyCssDeclarationOrRule::CssEmptyDeclaration(_) => SortInfo::unknown(),
            AnyCssDeclarationOrRule::CssBogus(_) => SortInfo::unknown(),
            AnyCssDeclarationOrRule::CssMetavariable(_) => SortInfo::unknown(),
            AnyCssDeclarationOrRule::AnyCssRule(rule) => match rule {
                AnyCssRule::CssAtRule(_) => SortInfo::from_kind(NodeKindOrder::NestedRuleOrAtRule),
                AnyCssRule::CssBogusRule(_) => SortInfo::unknown(),
                AnyCssRule::CssNestedQualifiedRule(_) => {
                    SortInfo::from_kind(NodeKindOrder::NestedRuleOrAtRule)
                }
                AnyCssRule::CssQualifiedRule(_) => SortInfo::unknown(),
            },
            AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(decl_with_semicolon) => {
                let prop = decl_with_semicolon
                    .declaration()
                    .ok()
                    .and_then(|decl| decl.property().ok());

                let Some(prop) = prop else {
                    return SortInfo::unknown();
                };

                // composes property is a unique node type
                if let AnyCssProperty::CssComposesProperty(_) = prop {
                    return SortInfo::from_kind(NodeKindOrder::ComposesProperty);
                }

                let AnyCssProperty::CssGenericProperty(prop) = prop else {
                    return SortInfo::unknown();
                };
                let Some(prop) = prop.name().ok() else {
                    return SortInfo::unknown();
                };

                match prop {
                    AnyCssDeclarationName::CssDashedIdentifier(_) => {
                        SortInfo::from_kind(NodeKindOrder::CustomProperty)
                    }
                    AnyCssDeclarationName::CssIdentifier(ident) => {
                        let tok = ident.value_token().ok();
                        let Some(tok) = tok else {
                            return SortInfo::unknown();
                        };
                        let prop_text = tok.token_text_trimmed();

                        let prefix = prop_text_to_prefix(&prop_text);
                        let unprefixed = prop_text_to_unprefixed(&prop_text);

                        let vendor_prefix_index = match prefix {
                            Some(prefix) => VENDOR_PREFIXES
                                .iter()
                                .position(|vp| vp == &prefix)
                                .unwrap_or(usize::MAX),
                            None => usize::MAX,
                        };

                        if let Some(property_index) = PROPERTY_ORDER_MAP.get(unprefixed.as_ref()) {
                            SortInfo::from_declaration(*property_index, vendor_prefix_index)
                        } else {
                            SortInfo::from_kind(NodeKindOrder::UnknownDeclaration)
                        }
                    }
                }
            }
        }
    }
}

/// Check if any shortand property (e.g. margin) appears after any of its longhand sub properties (e.g. margin-top).
/// Sorting such properties would be unsafe, so we need to bail out. The noShorthandPropertyOverrides rule will catch that case instead.
fn contains_shorthand_after_longhand(nodes: &[AnyCssDeclarationOrRule]) -> bool {
    let mut disallowed_longhand_properties = HashSet::<(Option<&str>, &str)>::new();

    // We have a mapping of shorthand to longhand sub properties, and to make use of that we need to iterate backwards.
    // Starting from the bottom, when we see a shorthand property, record the set of longhand properties that are no longer allowed to appear above it.
    for node in nodes.iter().rev() {
        let Some(prop_text) = &node
            .as_css_declaration_with_semicolon()
            .and_then(css_declaration_to_prop_text)
        else {
            continue;
        };

        let prefix = prop_text_to_prefix(prop_text);
        let unprefixed = prop_text_to_unprefixed(prop_text);

        // Check for disallowed properties
        for disallowed_property in disallowed_longhand_properties.iter() {
            if (prefix, unprefixed.as_ref()) == *disallowed_property {
                return true;
            }
        }

        // Disallow sub properties to appear above this property
        for longhand_child_property in get_longhand_sub_properties(&unprefixed) {
            disallowed_longhand_properties.insert((prefix, longhand_child_property));
        }
        for longhand_child_property in get_reset_to_initial_properties(&unprefixed) {
            disallowed_longhand_properties.insert((prefix, longhand_child_property));
        }
    }

    false
}

/// Check for properties that don't appear in [PROPERTY_ORDER_MAP]. They are likely a mistake.
/// The noUnknownProperty rule will catch it instead.
fn contains_unknown_property(nodes: &[AnyCssDeclarationOrRule]) -> bool {
    for node in nodes.iter() {
        let Some(prop_text) = &node
            .as_css_declaration_with_semicolon()
            .and_then(css_declaration_to_prop_text)
        else {
            continue;
        };

        let unprefixed = prop_text_to_unprefixed(prop_text);
        if !PROPERTY_ORDER_MAP.contains_key(unprefixed.as_ref()) {
            return true;
        }
    }

    false
}
