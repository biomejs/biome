use std::{
    borrow::Cow,
    collections::{BTreeSet, HashSet},
};

use biome_analyze::{
    context::RuleContext, declare_lint_rule, ActionCategory, Ast, FixKind, Rule, RuleAction,
    RuleDiagnostic,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssDeclarationName, AnyCssDeclarationOrRule, AnyCssProperty, AnyCssRule,
    CssDeclarationOrRuleBlock, CssDeclarationOrRuleList, CssDeclarationWithSemicolon,
    CssIdentifier, CssLanguage, CssSyntaxKind,
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
    /// Enforce ordering of CSS properties.
    ///
    /// This rule checks if the properties and nested rules are in a consistent order.
    ///
    /// The expected ordering is roughly:
    /// 1. Custom properties
    /// 1. Layout properties (display, flex, grid)
    /// 1. Margin & padding properties
    /// 1. Typography properties (font, color)
    /// 1. Interaction properties (pointer-events, visibility)
    /// 1. Background & border properties
    /// 1. Transition & animation properties
    /// 1. Nested rules, media queries & other at-rules
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// p { color: black; display: block; }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// p { display: block; color: black; }
    /// ```
    ///
    pub UseSortedProperties {
        version: "next",
        name: "useSortedProperties",
        language: "css",
        recommended: true,
        fix_kind: FixKind::Safe,
    }
}

pub struct UseSortedPropertiesState {
    block: CssDeclarationOrRuleBlock,
    original_properties: Vec<AnyCssDeclarationOrRule>,
    sorted_properties: RecessOrderProperties,
    is_unsafe_to_sort: bool,
}

impl Rule for UseSortedProperties {
    type Query = Ast<CssDeclarationOrRuleBlock>;
    type State = UseSortedPropertiesState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let query_result = ctx.query();

        let original_properties = query_result
            .items()
            .into_iter()
            .collect::<Vec<AnyCssDeclarationOrRule>>();
        let is_unsafe_to_sort = contains_shorthand_after_longhand(&original_properties)
            || contains_unknown_property(&original_properties);
        let sorted_properties = RecessOrderProperties::new(&original_properties);

        Some(UseSortedPropertiesState {
            block: query_result.clone(),
            original_properties,
            sorted_properties,
            is_unsafe_to_sort,
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        if state.is_unsafe_to_sort || state.sorted_properties.is_sorted() {
            return None;
        }

        return Some(RuleDiagnostic::new(
            rule_category!(),
            state.block.range(),
            markup! {
                "Properties can be sorted."
            },
        ));
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<CssRuleAction> {
        if state.is_unsafe_to_sort || state.sorted_properties.is_sorted() {
            return None;
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(
            state.block.items(),
            state
                .sorted_properties
                .as_sorted(&state.original_properties)?,
        );

        return Some(RuleAction::<CssLanguage>::new(
            ActionCategory::QuickFix(Cow::Borrowed("")),
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

fn css_identifier_to_prop_text(ident: &CssIdentifier) -> Option<TokenText> {
    let tok = ident.value_token().ok()?;
    Some(tok.token_text_trimmed())
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
        .and_then(|prop_name| -> Option<TokenText> {
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum NodeKindOrder {
    CustomProperty,
    ComposesProperty,
    Declaration,
    UnknownDeclaration,
    NestedRuleOrAtRule,
    UnknownKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
/// Define sort order using lexographical sorting of this struct
pub struct SortInfo {
    // First, nodes are sorted by the kind of node (e.g. declarations before at-rules)
    kind: NodeKindOrder,
    // Property nodes are sorted by a predefined desired order
    property: u32,
    // Vendor prefixed properties should be before the non-prefixed version of the same property
    vendor_prefix: u32,
    original_position: usize,
}

impl SortInfo {
    fn from_kind(kind: NodeKindOrder) -> Self {
        Self {
            kind,
            property: 0,
            vendor_prefix: 0,
            original_position: 0,
        }
    }
    fn from_declaration(property: u32, vendor_prefix: u32) -> Self {
        Self {
            kind: NodeKindOrder::Declaration,
            property,
            vendor_prefix,
            original_position: 0,
        }
    }
    fn unknown() -> Self {
        Self {
            kind: NodeKindOrder::UnknownKind,
            property: 0,
            vendor_prefix: 0,
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

                match prop {
                    Some(AnyCssProperty::CssComposesProperty(_)) => {
                        SortInfo::from_kind(NodeKindOrder::ComposesProperty)
                    }
                    Some(AnyCssProperty::CssGenericProperty(prop)) => match prop.name().ok() {
                        Some(AnyCssDeclarationName::CssDashedIdentifier(_)) => {
                            SortInfo::from_kind(NodeKindOrder::CustomProperty)
                        }
                        Some(AnyCssDeclarationName::CssIdentifier(ident)) => {
                            if let Some(prop_text) = css_identifier_to_prop_text(&ident) {
                                let prefix = prop_text_to_prefix(&prop_text);
                                let unprefixed = prop_text_to_unprefixed(&prop_text);

                                let vendor_prefix_idx = match prefix {
                                    Some(prefix) => VENDOR_PREFIXES
                                        .iter()
                                        .position(|vp| vp == &prefix)
                                        .map_or(u32::MAX, |pos| pos as u32),
                                    None => u32::MAX,
                                };

                                if let Some(property_idx) =
                                    PROPERTY_ORDER_MAP.get(unprefixed.as_ref())
                                {
                                    SortInfo::from_declaration(*property_idx, vendor_prefix_idx)
                                } else {
                                    SortInfo::from_kind(NodeKindOrder::UnknownDeclaration)
                                }
                            } else {
                                SortInfo::unknown()
                            }
                        }
                        None => SortInfo::unknown(),
                    },
                    _ => SortInfo::unknown(),
                }
            }
        }
    }
}

/// Check if any shortand property (e.g. margin) appears after any of its longhand sub properties (e.g. margin-top).
/// Sorting such properties would be unsafe, so we need to bail out. The no_shorthand_property_overrides rule will catch that case instead.
fn contains_shorthand_after_longhand(nodes: &[AnyCssDeclarationOrRule]) -> bool {
    let mut disallowed_longhand_properties = HashSet::<(Option<&str>, &str)>::new();

    // This works backwards.
    // Starting from the bottom, when we see a shorthand property, record the set of longhand properties that are no longer allowed to appear above it.
    for node in nodes.iter().rev() {
        if let AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(decl_with_semicolon) = node {
            if let Some(prop_text) = &css_declaration_to_prop_text(decl_with_semicolon) {
                let prefix = prop_text_to_prefix(prop_text);
                let unprefixed = prop_text_to_unprefixed(prop_text);

                // Check for disallowed properties
                for disallowed_property in disallowed_longhand_properties.iter() {
                    if (prefix, unprefixed.as_ref()) == *disallowed_property {
                        return true;
                    }
                }

                // Disallow sub properties to appear above this property
                for longhand_child_property in [
                    get_longhand_sub_properties(&unprefixed),
                    get_reset_to_initial_properties(&unprefixed),
                ]
                .into_iter()
                .flatten()
                {
                    disallowed_longhand_properties.insert((prefix, longhand_child_property));
                }
            }
        }
    }

    false
}

/// Check for properties that don't have a defined order. We don't sort anything in that case.
fn contains_unknown_property(nodes: &[AnyCssDeclarationOrRule]) -> bool {
    for node in nodes.iter() {
        if let AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(decl_with_semicolon) = node {
            if let Some(prop_text) = &css_declaration_to_prop_text(decl_with_semicolon) {
                let unprefixed = prop_text_to_unprefixed(prop_text);
                if !PROPERTY_ORDER_MAP.contains_key(unprefixed.as_ref()) {
                    return true;
                }
            }
        }
    }

    false
}
