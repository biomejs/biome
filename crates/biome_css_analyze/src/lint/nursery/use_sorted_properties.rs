use std::{cmp::Ordering, collections::HashSet};

use biome_analyze::{
    context::RuleContext, declare_lint_rule, ActionCategory, Ast, FixKind, Rule, RuleAction,
    RuleDiagnostic,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssDeclarationName, AnyCssDeclarationOrRule, AnyCssProperty, AnyCssRule,
    CssDeclarationOrRuleBlock, CssDeclarationWithSemicolon, CssIdentifier, CssLanguage,
};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::{
    keywords::{LONGHAND_SUB_PROPERTIES_MAP, RESET_TO_INITIAL_PROPERTIES_MAP, VENDOR_PREFIXES},
    order::PROPERTY_ORDER_MAP,
    utils::{property_may_override_others, vender_prefix},
    CssRuleAction,
};

declare_lint_rule! {
    /// Enforce ordering of CSS properties.
    ///
    /// This rule checks if the properties and nested rules are in a consistent order.
    ///
    /// The expected ordering is roughly:
    ///  - Custom properties
    ///  - Layout properties (display, flex, grid)
    ///  - Margin & padding properties
    ///  - Typography properties (font, color)
    ///  - Interaction properties (pointer-events, visibility)
    ///  - Background & border properties
    ///  - Transition & animation properties
    ///  - Nested rules
    ///  - Nested media queries & at-rules
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
    can_be_sorted: bool,
    first_out_of_order_pair: Option<(NodeWithPosition, NodeWithPosition)>,
}

impl Rule for UseSortedProperties {
    type Query = Ast<CssDeclarationOrRuleBlock>;
    type State = UseSortedPropertiesState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let query_result = ctx.query();
        let items = to_block_items_with_position(query_result);

        let first_out_of_order_pair = get_first_out_of_order_pair(&items);
        first_out_of_order_pair.as_ref()?;

        Some(UseSortedPropertiesState {
            block: query_result.clone(),
            can_be_sorted: !contains_shorthand_after_longhand(&items)
                && !contains_unknown_property(&items),
            first_out_of_order_pair,
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        if !state.can_be_sorted || state.first_out_of_order_pair.is_none() {
            return None;
        }

        if let Some((a, b)) = &state.first_out_of_order_pair {
            let a_description = node_short_description(a);
            let b_description = node_short_description(b);
            return Some(RuleDiagnostic::new(
                rule_category!(),
                state.block.range(),
                markup! {
                    "Properties should be sorted: "<Emphasis>{ a_description }</Emphasis>" should be before "<Emphasis>{ b_description }</Emphasis>"."
                },
            ));
        }

        None
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<CssRuleAction> {
        if !state.can_be_sorted || state.first_out_of_order_pair.is_none() {
            return None;
        }

        let mut mutation = ctx.root().begin();

        let original_items = to_block_items_with_position(&state.block);
        let mut sorted_items = original_items.clone();
        sorted_items.sort_by(|a, b| sort_info(&a.node).cmp(&sort_info(&b.node)));

        let pairs = original_items.iter().zip(sorted_items.iter());

        for (item_to_replace, replacement) in pairs {
            if item_to_replace.position != replacement.position {
                mutation.replace_node_discard_trivia(
                    item_to_replace.node.clone(),
                    replacement.node.clone(),
                );
            }
        }

        return Some(RuleAction::<CssLanguage>::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Sort these properties" }.to_owned(),
            mutation,
        ));
    }
}

#[derive(Clone)]
struct NodeWithPosition {
    position: usize,
    node: AnyCssDeclarationOrRule,
}

fn to_block_items_with_position(block: &CssDeclarationOrRuleBlock) -> Vec<NodeWithPosition> {
    block
        .items()
        .into_iter()
        .enumerate()
        .map(|(position, item)| NodeWithPosition {
            position,
            node: item,
        })
        .collect()
}

// takes the unsorted list and returns the first item that is out of order, and the item it should be placed before
// returns None if the list is sorted
fn get_first_out_of_order_pair(
    items: &[NodeWithPosition],
) -> Option<(NodeWithPosition, NodeWithPosition)> {
    if items.is_empty() {
        return None;
    }

    let mut first_out_of_order_node: Option<NodeWithPosition> = None;

    // find the first node that's 'less than' the previous node
    let mut prev_item_info = sort_info(&items[0].node);
    for item in items.iter() {
        let item_info = sort_info(&item.node);
        if item_info.cmp(&prev_item_info) == Ordering::Less {
            first_out_of_order_node = Some(item.clone());
            break;
        }
        prev_item_info = item_info;
    }

    // find where that node belongs
    if let Some(first_out_of_order_node) = first_out_of_order_node {
        let first_out_of_order_node_info = sort_info(&first_out_of_order_node.node);
        for item in items.iter() {
            let item_info = sort_info(&item.node);
            if item_info.cmp(&first_out_of_order_node_info) == Ordering::Greater {
                return Some((first_out_of_order_node.clone(), item.clone()));
            }
        }
    }

    None
}

// returns (vendor prefix, name without vendor prefix) in lowercase for a CssIdentifier
fn css_identifier_to_prop_name(ident: &CssIdentifier) -> Option<(String, String)> {
    let tok = ident.value_token().ok()?;
    let raw_prop_name = tok.token_text_trimmed().text().to_string();

    let prop_lowercase = raw_prop_name.to_lowercase();
    let prop_prefix = vender_prefix(&prop_lowercase).to_string();
    let unprefixed_prop = if let Some(unprefixed_slice) = prop_lowercase.strip_prefix(&prop_prefix)
    {
        unprefixed_slice.to_string()
    } else {
        prop_lowercase
    };

    Some((prop_prefix, unprefixed_prop))
}

// returns (vendor prefix, name without vendor prefix) in lowercase for a CssDeclarationWithSemicolon
fn css_declaration_to_prop_name(
    decl_with_semicolon: &CssDeclarationWithSemicolon,
) -> Option<(String, String)> {
    let prop_name = decl_with_semicolon
        .declaration()
        .ok()?
        .property()
        .ok()?
        .as_css_generic_property()?
        .name()
        .ok();
    if let Some(AnyCssDeclarationName::CssIdentifier(ident)) = prop_name {
        return css_identifier_to_prop_name(&ident);
    }
    None
}

const SORT_INFO_KIND_CUSTOM_PROPERTY: u32 = 1;
const SORT_INFO_KIND_COMPOSES_PROPERTY: u32 = 2;
const SORT_INFO_KIND_DECLARATION: u32 = 3;
const SORT_INFO_KIND_UNKNOWN_DECLARATION: u32 = 4;
const SORT_INFO_KIND_NESTED_RULE_OR_AT_RULE: u32 = 5;

// this struct can be directly sorted using the default cmp implementation
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct SortInfo {
    kind: u32,
    property: u32,
    vendor_prefix: u32,
}

impl SortInfo {
    fn from_kind(kind: u32) -> Self {
        Self {
            kind,
            property: 0,
            vendor_prefix: 0,
        }
    }
    fn from_declaration(property: u32, vendor_prefix: u32) -> Self {
        Self {
            kind: SORT_INFO_KIND_DECLARATION,
            property,
            vendor_prefix,
        }
    }
    fn unknown() -> Self {
        Self {
            kind: u32::MAX,
            property: 0,
            vendor_prefix: 0,
        }
    }
}

fn sort_info(item: &AnyCssDeclarationOrRule) -> SortInfo {
    match item {
        AnyCssDeclarationOrRule::CssBogus(_) => SortInfo::unknown(),
        AnyCssDeclarationOrRule::CssMetavariable(_) => SortInfo::unknown(),
        AnyCssDeclarationOrRule::AnyCssRule(rule) => match rule {
            AnyCssRule::CssAtRule(_) => SortInfo::from_kind(SORT_INFO_KIND_NESTED_RULE_OR_AT_RULE),
            AnyCssRule::CssBogusRule(_) => SortInfo::unknown(),
            AnyCssRule::CssNestedQualifiedRule(_) => {
                SortInfo::from_kind(SORT_INFO_KIND_NESTED_RULE_OR_AT_RULE)
            }
            AnyCssRule::CssQualifiedRule(_) => SortInfo::unknown(),
        },
        AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(decl_with_semicolon) => {
            let prop = decl_with_semicolon
                .declaration()
                .ok()
                .and_then(|decl| decl.property().ok());

            if let Some(prop) = prop {
                match prop {
                    AnyCssProperty::CssComposesProperty(_) => {
                        SortInfo::from_kind(SORT_INFO_KIND_COMPOSES_PROPERTY)
                    }
                    AnyCssProperty::CssGenericProperty(prop) => match prop.name().ok() {
                        Some(name) => match name {
                            AnyCssDeclarationName::CssDashedIdentifier(_) => {
                                SortInfo::from_kind(SORT_INFO_KIND_CUSTOM_PROPERTY)
                            }
                            AnyCssDeclarationName::CssIdentifier(ident) => {
                                if let Some(sanitized_prop) = css_identifier_to_prop_name(&ident) {
                                    let (vendor_prefix, plain_prop) = sanitized_prop;

                                    let vendor_prefix_idx: u32 = VENDOR_PREFIXES
                                        .iter()
                                        .position(|vp| vp == &vendor_prefix.as_str())
                                        .map_or(u32::MAX, |pos| pos as u32);

                                    if let Some(idx) = PROPERTY_ORDER_MAP.get(&plain_prop) {
                                        SortInfo::from_declaration(*idx, vendor_prefix_idx)
                                    } else {
                                        SortInfo::from_kind(SORT_INFO_KIND_UNKNOWN_DECLARATION)
                                    }
                                } else {
                                    SortInfo::unknown()
                                }
                            }
                        },
                        None => SortInfo::unknown(),
                    },
                    _ => SortInfo::unknown(),
                }
            } else {
                SortInfo::unknown()
            }
        }
    }
}

fn contains_shorthand_after_longhand(items: &[NodeWithPosition]) -> bool {
    let mut seen_shorthand_properties = HashSet::<String>::with_capacity(items.len());

    // (iterating backwards means we can store a smaller list of seen shorthands instead of seen longhands)
    for item in items.iter().rev() {
        let node = &item.node;
        if let AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(decl_with_semicolon) = node {
            if let Some(sanitized_prop) = &css_declaration_to_prop_name(decl_with_semicolon) {
                let (vendor_prefix, plain_prop) = sanitized_prop;

                // Check if longhand properties appear above shorthand properties
                // these would trigger another rule, no_shorthand_property_overrides, but this rule
                // would attempt to autofix it which would be unsafe, so we should suppress this rule here.
                let potential_shorthands = [
                    LONGHAND_SUB_PROPERTIES_MAP.get(plain_prop),
                    RESET_TO_INITIAL_PROPERTIES_MAP.get(plain_prop),
                ];
                for shorthand in potential_shorthands.into_iter().flatten() {
                    let key = vendor_prefix.to_owned() + shorthand;
                    if seen_shorthand_properties.contains(&key) {
                        return true;
                    }
                }

                if property_may_override_others(plain_prop) {
                    let key = vendor_prefix.to_owned() + plain_prop;
                    seen_shorthand_properties.insert(key);
                }
            }
        }
    }

    false
}

fn contains_unknown_property(items: &[NodeWithPosition]) -> bool {
    for item in items.iter() {
        let node = &item.node;
        if let AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(decl_with_semicolon) = node {
            if let Some(sanitized_prop) = &css_declaration_to_prop_name(decl_with_semicolon) {
                let (_, plain_prop) = sanitized_prop;
                if !PROPERTY_ORDER_MAP.contains_key(plain_prop) {
                    return true;
                }
            }
        }
    }

    false
}

fn node_short_description(node_with_position: &NodeWithPosition) -> String {
    let position = node_with_position.position + 1; // one-based

    let short_desciption = match &node_with_position.node {
        AnyCssDeclarationOrRule::AnyCssRule(rule) => match rule {
            AnyCssRule::CssAtRule(_) => Some(format!("the at-rule at position {}", position)),
            AnyCssRule::CssNestedQualifiedRule(_) => {
                Some(format!("the nested rule at position {}", position))
            }
            _ => None,
        },
        AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(decl_with_semicolon) => {
            decl_with_semicolon
                .declaration()
                .ok()
                .and_then(|decl| decl.property().ok())
                .and_then(|prop| match prop {
                    AnyCssProperty::CssComposesProperty(_) => Some("\"composes\"".to_string()),
                    AnyCssProperty::CssGenericProperty(prop) => match prop.name().ok() {
                        Some(name) => match name {
                            AnyCssDeclarationName::CssIdentifier(ident) => ident.value_token().ok(),
                            AnyCssDeclarationName::CssDashedIdentifier(ident) => {
                                ident.value_token().ok()
                            }
                        }
                        .map(|tok| "\"".to_owned() + tok.token_text_trimmed().text() + "\""),
                        None => None,
                    },
                    _ => None,
                })
        }
        _ => None,
    };

    short_desciption.unwrap_or_else(|| format!("the unknown item at position {}", position))
}
