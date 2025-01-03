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
    CssDeclarationOrRuleBlock, CssDeclarationWithSemicolon, CssIdentifier, CssLanguage,
};
use biome_rowan::{AstNode, BatchMutationExt, TokenText};
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
    /// 1. Nested rules
    /// 1. Nested media queries & at-rules
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
    items: Vec<AnyCssDeclarationOrRule>,
    btree: SortableRuleOrDeclarationTree,
    is_unsafe_to_sort: bool,
}

impl Rule for UseSortedProperties {
    type Query = Ast<CssDeclarationOrRuleBlock>;
    type State = UseSortedPropertiesState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let query_result = ctx.query();

        let items = query_result
            .items()
            .into_iter()
            .collect::<Vec<AnyCssDeclarationOrRule>>();
        let is_unsafe_to_sort =
            contains_shorthand_after_longhand(&items) || contains_unknown_property(&items);
        let btree = SortableRuleOrDeclarationTree::new(&items);

        Some(UseSortedPropertiesState {
            block: query_result.clone(),
            items,
            btree,
            is_unsafe_to_sort,
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        if state.is_unsafe_to_sort || state.btree.is_sorted() {
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
        if state.is_unsafe_to_sort {
            return None;
        }

        let mut mutation = ctx.root().begin();

        for (desired_position, sort_info) in state.btree.0.iter().enumerate() {
            if sort_info.original_position != desired_position {
                mutation.replace_node_discard_trivia(
                    state.items.get(sort_info.original_position)?.clone(),
                    state.items.get(desired_position)?.clone(),
                );
            }
        }

        return Some(RuleAction::<CssLanguage>::new(
            ActionCategory::QuickFix(Cow::Borrowed("")),
            ctx.metadata().applicability(),
            markup! { "Sort these properties" }.to_owned(),
            mutation,
        ));
    }
}

pub struct SortableRuleOrDeclarationTree(pub BTreeSet<SortInfo>);

impl SortableRuleOrDeclarationTree {
    pub fn new(items: &[AnyCssDeclarationOrRule]) -> Self {
        SortableRuleOrDeclarationTree(
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
}

fn css_identifier_to_prop_text(ident: &CssIdentifier) -> Option<TokenText> {
    let tok = ident.value_token().ok()?;
    Some(tok.token_text_trimmed())
}

fn css_declaration_to_prop_text(
    decl_with_semicolon: &CssDeclarationWithSemicolon,
) -> Option<TokenText> {
    let prop_name = decl_with_semicolon
        .declaration()
        .ok()?
        .property()
        .ok()?
        .as_css_generic_property()?
        .name()
        .ok();
    if let Some(AnyCssDeclarationName::CssIdentifier(ident)) = prop_name {
        return Some(ident.value_token().ok()?.token_text_trimmed());
    }
    None
}

/// Returns a declaration's vendor prefix in lowercase
fn prop_text_to_prefix(tok_text: &TokenText) -> Option<&'static str> {
    let prop_lowercase = tok_text.text().to_lowercase_cow();
    let prefix = vender_prefix(&prop_lowercase);
    if prefix == "" {
        return None;
    }
    return Some(prefix);
}

/// Returns a declaration's property name without vendor prefix in lowercase.
fn prop_text_to_unprefixed(tok_text: &TokenText) -> Cow<'_, str> {
    let prop_lowercase = tok_text.text().to_lowercase_cow();
    let prefix = vender_prefix(&prop_lowercase);
    if prefix == "" {
        return prop_lowercase;
    }
    let unprefixed = match &prop_lowercase {
        Cow::Borrowed(s) => s
            .strip_prefix(prefix)
            .and_then(|stripped| Some(Cow::Borrowed(stripped))),
        Cow::Owned(s) => s
            .strip_prefix(prefix)
            .and_then(|stripped| Some(Cow::Owned(stripped.to_owned()))),
    };
    return unprefixed.unwrap_or(prop_lowercase);
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum NodeKindOrder {
    CustomProperty,
    ComposesProperty,
    Declaration,
    UnknownDeclaration,
    NestedRuleOrAtRule,
    UnknownKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
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

                if let Some(_) = &prop {
                } else {
                    return SortInfo::unknown();
                }

                match prop {
                    Some(AnyCssProperty::CssComposesProperty(_)) => {
                        SortInfo::from_kind(NodeKindOrder::ComposesProperty)
                    }
                    Some(AnyCssProperty::CssGenericProperty(prop)) => match prop.name().ok() {
                        Some(name) => match name {
                            AnyCssDeclarationName::CssDashedIdentifier(_) => {
                                SortInfo::from_kind(NodeKindOrder::CustomProperty)
                            }
                            AnyCssDeclarationName::CssIdentifier(ident) => {
                                if let Some(prop_text) = css_identifier_to_prop_text(&ident) {
                                    let prefix = prop_text_to_prefix(&prop_text);
                                    let unprefixed = prop_text_to_unprefixed(&prop_text);

                                    let vendor_prefix_idx: u32 = match prefix {
                                        Some(prefix) => VENDOR_PREFIXES
                                            .iter()
                                            .position(|vp| vp == &prefix)
                                            .map_or(u32::MAX, |pos| pos as u32),
                                        None => u32::MAX,
                                    };

                                    if let Some(idx) = PROPERTY_ORDER_MAP.get(unprefixed.as_ref()) {
                                        SortInfo::from_declaration(*idx, vendor_prefix_idx)
                                    } else {
                                        SortInfo::from_kind(NodeKindOrder::UnknownDeclaration)
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
                let prefix = prop_text_to_prefix(&prop_text);
                let unprefixed = prop_text_to_unprefixed(&prop_text);

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

    // let mut seen_shorthand_properties = HashSet::<String>::with_capacity(nodes.len());

    // // (iterating backwards means we can store a smaller list of seen shorthands instead of seen longhands)
    // for node in nodes.iter().rev() {
    //     if let AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(decl_with_semicolon) = node {
    //         if let Some(prop_text) = &css_declaration_to_prop_text(decl_with_semicolon) {
    //             let prefix = prop_text_to_prefix(&prop_text);
    //             let unprefixed = prop_text_to_unprefixed(&prop_text);

    //             let potential_shorthands = [
    //                 LONGHAND_SUB_PROPERTIES_MAP.get(unprefixed),
    //                 RESET_TO_INITIAL_PROPERTIES_MAP.get(unprefixed),
    //             ];
    //             for shorthand in potential_shorthands.into_iter().flatten() {
    //                 let key = prefix.to_owned() + shorthand;
    //                 if seen_shorthand_properties.contains(&key) {
    //                     return true;
    //                 }
    //             }

    //             get_longhand_sub_properties

    //             if property_may_override_others(unprefixed) {
    //                 let key = prefix.to_owned() + unprefixed;
    //                 seen_shorthand_properties.insert(key);
    //             }
    //         }
    //     }
    // }

    // false
}

/// Check for properties that don't have a defined order. We don't sort anything in that case.
fn contains_unknown_property(nodes: &[AnyCssDeclarationOrRule]) -> bool {
    for node in nodes.iter() {
        if let AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(decl_with_semicolon) = node {
            if let Some(prop_text) = &css_declaration_to_prop_text(decl_with_semicolon) {
                let unprefixed = prop_text_to_unprefixed(&prop_text);
                if !PROPERTY_ORDER_MAP.contains_key(unprefixed.as_ref()) {
                    return true;
                }
            }
        }
    }

    false
}
