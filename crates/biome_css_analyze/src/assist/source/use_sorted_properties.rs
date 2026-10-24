use crate::{
    CssRuleAction,
    order::PROPERTY_ORDER_MAP,
    utils::{get_longhand_sub_properties, get_reset_to_initial_properties, vender_prefix},
};
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_source_rule,
};
use biome_console::markup;
use biome_css_factory::make::{
    css_declaration_list, css_declaration_or_at_rule_list, css_declaration_or_rule_list,
};
use biome_css_syntax::{
    AnyCssDeclarationName, AnyCssDeclarationOrRule, AnyCssProperty, AnyCssRule,
    CssDeclarationBlock, CssDeclarationOrAtRuleBlock, CssDeclarationOrRuleBlock,
    CssDeclarationWithSemicolon, CssKeyframesItem, CssLanguage, CssSyntaxKind, CssSyntaxToken,
    TextRange, keywords::VENDOR_PREFIXES,
};
use biome_rowan::{AstNode, BatchMutationExt, TokenText, declare_node_union};
use biome_rule_options::use_sorted_properties::UseSortedPropertiesOptions;
use biome_string_case::StrOnlyExtension;
use std::{borrow::Cow, cmp::Ordering, collections::HashMap};

declare_source_rule! {
    /// Sort CSS properties into a consistent order.
    ///
    /// The action uses a predefined order that groups related properties by purpose.
    /// Custom properties come first within each group. Nested rules, at-rules, Sass variable
    /// declarations, and other non-property items stay in place and separate the groups.
    ///
    /// Properties inside CSS and SCSS keyframe steps use the same order.
    ///
    /// The main groups are:
    ///
    /// 1. Custom properties.
    /// 2. Layout properties, including `display`, `visibility`, flexbox, and grid.
    /// 3. Margin and padding properties.
    /// 4. Typography properties, including `font` and `color`.
    /// 5. Interaction properties, including `pointer-events`.
    /// 6. Background and border properties.
    /// 7. Transition and animation properties.
    ///
    /// Biome skips blocks containing unknown properties because their correct position is not
    /// known. If sorting could change the result of a shorthand and longhand combination, Biome
    /// reports the block but does not offer the action.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diff
    /// p {
    ///   transition: opacity 1s ease;
    ///   border: 1px solid black;
    ///   pointer-events: none;
    ///   color: black;
    ///   margin: 8px;
    ///   display: block;
    ///   --custom: 100;
    /// }
    /// ```
    ///
    /// ```css,expect_diff
    /// p {
    ///   span { color: blue; }
    ///   color: red;
    ///   display: block;
    /// }
    /// ```
    ///
    /// ```css,expect_diff
    /// @keyframes fade {
    ///   from {
    ///     opacity: 0;
    ///     display: block;
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// p {
    ///   --custom: 100;
    ///   display: block;
    ///   margin: 8px;
    ///   color: black;
    ///   pointer-events: none;
    ///   border: 1px solid black;
    ///   transition: opacity 1s ease;
    /// }
    /// ```
    ///
    /// ```css
    /// p {
    ///   span { color: blue; }
    ///   display: block;
    ///   color: red;
    /// }
    /// ```
    ///
    pub UseSortedProperties {
        version: "2.0.0",
        name: "useSortedProperties",
        language: "css",
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

declare_node_union! {
    pub AnySortableBlockLike = CssDeclarationOrRuleBlock | CssDeclarationBlock | CssDeclarationOrAtRuleBlock
}

pub struct UseSortedPropertiesState {
    /// The containing node
    block: AnySortableBlockLike,
    /// The items to be sorted in their original order
    original_properties: Vec<AnyCssDeclarationOrRule>,
    /// The items in their sorted order, or [None] if sorting would be unsafe or if the rule contains unknown items.
    sorted_properties: Option<RecessOrderProperties>,
}

impl Rule for UseSortedProperties {
    type Query = Ast<AnySortableBlockLike>;
    type State = UseSortedPropertiesState;
    type Signals = Option<Self::State>;
    type Options = UseSortedPropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        // Declaration and declaration-or-at-rule blocks also occur in descriptor at-rules
        // such as @font-face; only sort them when they belong to a keyframe step.
        if !matches!(node, AnySortableBlockLike::CssDeclarationOrRuleBlock(_)) {
            CssKeyframesItem::cast(node.syntax().parent()?)?;
        }

        let original_properties = match node {
            AnySortableBlockLike::CssDeclarationOrRuleBlock(block) => {
                block.items().into_iter().collect::<Vec<_>>()
            }
            AnySortableBlockLike::CssDeclarationBlock(block) => block
                .declarations()
                .into_iter()
                .map(|declaration| AnyCssDeclarationOrRule::cast(declaration.into_syntax()))
                .collect::<Option<Vec<_>>>()?,
            AnySortableBlockLike::CssDeclarationOrAtRuleBlock(block) => block
                .items()
                .into_iter()
                .map(|declaration| AnyCssDeclarationOrRule::cast(declaration.into_syntax()))
                .collect::<Option<Vec<_>>>()?,
        };

        if contains_property_conflicts(&original_properties) {
            // This would be unsafe to sort
            return Some(UseSortedPropertiesState {
                block: node.clone(),
                original_properties,
                sorted_properties: None,
            });
        }

        if contains_unknown_property(&original_properties) {
            return None;
        }

        let sorted_properties = Some(RecessOrderProperties::new(&original_properties));
        let state = UseSortedPropertiesState {
            block: node.clone(),
            original_properties,
            sorted_properties,
        };

        let Some(sorted_properties) = &state.sorted_properties else {
            return None;
        };
        if sorted_properties.is_sorted(&state.original_properties) {
            return None;
        };

        Some(state)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "The properties are not sorted."
            },
        ))
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        Some(ctx.query().syntax().first_token()?.text_range())
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<CssRuleAction> {
        let Some(sorted_properties) = &state.sorted_properties else {
            return None;
        };
        if sorted_properties.is_sorted(&state.original_properties) {
            return None;
        }

        let mut mutation = ctx.root().begin();
        match &state.block {
            AnySortableBlockLike::CssDeclarationOrRuleBlock(block) => {
                mutation.replace_node_discard_trivia(
                    block.items(),
                    css_declaration_or_rule_list(sorted_properties.sorted_items()?),
                );
            }
            AnySortableBlockLike::CssDeclarationBlock(block) => {
                mutation.replace_node_discard_trivia(
                    block.declarations(),
                    css_declaration_list(sorted_properties.sorted_items()?),
                );
            }
            AnySortableBlockLike::CssDeclarationOrAtRuleBlock(block) => {
                mutation.replace_node_discard_trivia(
                    block.items(),
                    css_declaration_or_at_rule_list(sorted_properties.sorted_items()?),
                );
            }
        }

        Some(CssRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Sort these properties" }.to_owned(),
            mutation,
        ))
    }
}

fn with_terminated_line_comment(
    item: AnyCssDeclarationOrRule,
    next_item: Option<&AnyCssDeclarationOrRule>,
) -> Option<AnyCssDeclarationOrRule> {
    let last_token = item.syntax().last_token()?;
    if !last_token
        .trailing_trivia()
        .pieces()
        .any(|piece| piece.text().starts_with("//"))
    {
        return Some(item);
    }

    let next_token = match next_item {
        Some(next) => next.syntax().first_token()?,
        None => item.syntax().parent()?.last_token()?.next_token()?,
    };
    if next_token
        .leading_trivia()
        .pieces()
        .find(|piece| !piece.kind().is_whitespace())
        .is_some_and(|piece| piece.kind().is_newline())
    {
        return Some(item);
    }

    // The parser attaches the terminating newline to the following token.
    // Preserve it when sorting would otherwise extend the comment over code.
    let newline = last_token
        .next_token()?
        .leading_trivia()
        .pieces()
        .find(|piece| piece.kind().is_newline())?;
    item.append_trivia_pieces([newline])
}

fn with_semicolon(item: AnyCssDeclarationOrRule) -> Option<AnyCssDeclarationOrRule> {
    match item {
        AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(declaration)
            if declaration.semicolon_token().is_none() =>
        {
            let trailing_trivia = declaration.syntax().last_token()?.trailing_trivia();
            let semicolon = CssSyntaxToken::new_detached(CssSyntaxKind::SEMICOLON, ";", [], [])
                .with_trailing_trivia_pieces(trailing_trivia.pieces());
            Some(
                declaration
                    .with_trailing_trivia_pieces([])?
                    .with_semicolon_token(Some(semicolon))
                    .into(),
            )
        }
        item => Some(item),
    }
}

/// "Recess order" is a semantic ordering that mimics the behavior of [stylelint-config-recess-order](https://github.com/stormwarning/stylelint-config-recess-order).
///
/// Which in turn mimics the behavior of twitter's [RECESS](https://github.com/twitter-archive/recess/blob/29bccc870b7b4ccaa0a138e504caf608a6606b59/lib/lint/strict-property-order.js).
pub struct RecessOrderProperties(pub Vec<RecessOrderMember>);

impl RecessOrderProperties {
    pub fn new(items: &[AnyCssDeclarationOrRule]) -> Self {
        let mut members: Vec<_> = items
            .iter()
            .map(|item| RecessOrderMember(item.clone()))
            .collect();
        for group in members.chunk_by_mut(|a, b| {
            a.0.as_css_declaration_with_semicolon().is_some()
                && b.0.as_css_declaration_with_semicolon().is_some()
        }) {
            group.sort_unstable();
        }
        Self(members)
    }

    /// Checks if self's order matches the provided list's order.
    pub fn is_sorted(&self, original_items: &[AnyCssDeclarationOrRule]) -> bool {
        self.0
            .iter()
            .zip(original_items.iter())
            .all(|(a, b)| a.0 == *b)
    }

    fn sorted_items<N>(&self) -> Option<Vec<N>>
    where
        N: AstNode<Language = CssLanguage>,
    {
        let mut members = self.0.iter().peekable();
        let mut items = Vec::with_capacity(self.0.len());
        while let Some(member) = members.next() {
            let item =
                with_terminated_line_comment(member.0.clone(), members.peek().map(|next| &next.0))?;
            items.push(N::cast(with_semicolon(item)?.into_syntax())?);
        }
        Some(items)
    }
}

/// Defines the ordering of different kinds of css nodes, e.g. declaration nodes should be before at-rules.
// The order of these enum members controls the sort order of the css nodes.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeKindOrder {
    /// e.g. --custom-property: red;
    CustomProperty,
    /// e.g. composes: .rule;
    ComposesProperty,
    /// e.g. color: red;
    Declaration,
    /// Nested items, e.g. .nested {}
    NestedRuleOrAtRule,
    /// Everything else
    UnknownKind,
}

#[derive(PartialEq, Eq)]
pub struct RecessOrderMember(AnyCssDeclarationOrRule);

impl RecessOrderMember {
    /// Returns the kind of node for ordering purposes. The nodes are sorted in the order they're declared in [NodeKindOrder].
    pub fn kind(&self) -> NodeKindOrder {
        match &self.0 {
            AnyCssDeclarationOrRule::CssBogus(_) => NodeKindOrder::UnknownKind,
            AnyCssDeclarationOrRule::CssKeyframesItem(_) => NodeKindOrder::UnknownKind,
            AnyCssDeclarationOrRule::CssMetavariable(_) => NodeKindOrder::UnknownKind,
            AnyCssDeclarationOrRule::ScssVariableDeclaration(_) => NodeKindOrder::UnknownKind,
            AnyCssDeclarationOrRule::ScssNestingDeclaration(_) => NodeKindOrder::UnknownKind,
            AnyCssDeclarationOrRule::AnyCssRule(rule) => match rule {
                AnyCssRule::CssAtRule(_) => NodeKindOrder::NestedRuleOrAtRule,
                AnyCssRule::CssBogusRule(_) => NodeKindOrder::UnknownKind,
                AnyCssRule::CssNestedQualifiedRule(_) => NodeKindOrder::NestedRuleOrAtRule,
                AnyCssRule::CssQualifiedRule(_) => NodeKindOrder::UnknownKind,
            },
            AnyCssDeclarationOrRule::CssEmptyDeclaration(_) => NodeKindOrder::UnknownKind,
            AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(decl_with_semicolon) => {
                let Some(decl) = decl_with_semicolon.declaration().ok() else {
                    return NodeKindOrder::UnknownKind;
                };
                let Some(prop) = decl.property().ok() else {
                    return NodeKindOrder::UnknownKind;
                };
                match prop {
                    AnyCssProperty::CssBogusProperty(_) => NodeKindOrder::UnknownKind,
                    AnyCssProperty::CssComposesProperty(_) => NodeKindOrder::ComposesProperty,
                    AnyCssProperty::CssGenericProperty(prop) => {
                        let Some(prop) = prop.name().ok() else {
                            return NodeKindOrder::UnknownKind;
                        };
                        match prop {
                            AnyCssDeclarationName::AnyCssDashedIdentifier(_)
                            | AnyCssDeclarationName::TwValueThemeReference(_) => {
                                NodeKindOrder::CustomProperty
                            }
                            AnyCssDeclarationName::CssIdentifier(_) => NodeKindOrder::Declaration,
                            AnyCssDeclarationName::ScssInterpolatedIdentifier(name) => {
                                if name.syntax().text_trimmed().starts_with("--") {
                                    NodeKindOrder::CustomProperty
                                } else {
                                    NodeKindOrder::Declaration
                                }
                            }
                            AnyCssDeclarationName::CssMetavariable(_) => NodeKindOrder::UnknownKind,
                        }
                    }
                }
            }
        }
    }

    /// Returns the index of the property name in [crate::order::PROPERTY_ORDER].
    /// If none found, returns usize::MAX, causing unknown properties to be ordered after known properties.
    pub fn property_index(&self) -> usize {
        let Some(prop_text) = &self
            .0
            .as_css_declaration_with_semicolon()
            .and_then(css_declaration_to_prop_text)
        else {
            return usize::MAX;
        };

        let unprefixed = prop_text_to_unprefixed(prop_text);
        *PROPERTY_ORDER_MAP.get(unprefixed.as_ref()).unwrap_or(&0)
    }

    /// Returns the index of the property's vendor prefix in [VENDOR_PREFIXES].
    /// If no vendor prefix, returns usize::MAX, causing unprefixed properties to be ordered after prefixed properties.
    pub fn vendor_prefix_index(&self) -> usize {
        let Some(prop_text) = &self
            .0
            .as_css_declaration_with_semicolon()
            .and_then(css_declaration_to_prop_text)
        else {
            return usize::MAX;
        };

        if let Some(prefix) = prop_text_to_prefix(prop_text) {
            VENDOR_PREFIXES
                .iter()
                .position(|vp| vp == &prefix)
                .unwrap_or(usize::MAX)
        } else {
            usize::MAX
        }
    }
}

impl Ord for RecessOrderMember {
    /// First, sort by node kind. Use the order of declaration of the enum [NodeKindOrder].
    /// Then, sort by property name. Use the index at which the property appears in [crate::order::PROPERTY_ORDER].
    /// Then, sort by vendor prefix. Use the index at which the prefix appears in [VENDOR_PREFIXES]
    /// Then, sort by text range.
    fn cmp(&self, other: &Self) -> Ordering {
        let self_kind = self.kind();
        let other_kind = other.kind();
        if self_kind != other_kind {
            return self_kind.cmp(&other_kind);
        }

        let self_property_index = self.property_index();
        let other_property_index = other.property_index();
        if self_property_index != other_property_index {
            return self_property_index.cmp(&other_property_index);
        }

        let self_vendor_prefix = self.vendor_prefix_index();
        let other_vendor_prefix = other.vendor_prefix_index();
        if self_vendor_prefix != other_vendor_prefix {
            return self_vendor_prefix.cmp(&other_vendor_prefix);
        }

        self.0.range().cmp(&other.0.range())
    }
}

impl PartialOrd for RecessOrderMember {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Returns the css property name, if the node is a property.
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

/// Returns whether sorting would reverse declarations that can affect the same property.
/// For example, moving `margin` before an earlier `margin-top` would change which value wins.
/// Such conflicts must prevent the sorting action; `noShorthandPropertyOverrides` reports
/// shorthands that override earlier longhands, but does not cover every conflict checked here.
fn contains_property_conflicts(nodes: &[AnyCssDeclarationOrRule]) -> bool {
    let mut later_properties = HashMap::new();

    // Scan backwards so later declarations contribute their affected longhands and alias groups
    // before earlier declarations are checked. Keep the lowest sort index for each affected
    // property and vendor prefix: a larger index on an earlier declaration would reverse their
    // relative order when sorted, potentially changing the cascade.
    for node in nodes.iter().rev() {
        if node.as_css_declaration_with_semicolon().is_none() {
            later_properties.clear();
            continue;
        }

        let Some(prop_text) = &node
            .as_css_declaration_with_semicolon()
            .and_then(css_declaration_to_prop_text)
        else {
            continue;
        };

        let prefix = prop_text_to_prefix(prop_text);
        let unprefixed = prop_text_to_unprefixed(prop_text);
        let Some((&property, &index)) = PROPERTY_ORDER_MAP.get_key_value(unprefixed.as_ref()) else {
            continue;
        };

        if !matches!(property, "all" | "direction" | "unicode-bidi")
            && later_properties.contains_key(&(None, "all"))
        {
            return true;
        }

        for affected in std::iter::once(property)
            .chain(get_longhand_sub_properties(property).iter().copied())
            .chain(get_reset_to_initial_properties(property).iter().copied())
        {
            if later_properties
                .get(&(prefix, affected))
                .is_some_and(|later_index| index > *later_index)
            {
                return true;
            }

            if let Some((own_group, opposite_group)) = property_alias_groups(affected) {
                if later_properties
                    .get(&(prefix, opposite_group))
                    .is_some_and(|later_index| index > *later_index)
                {
                    return true;
                }
                later_properties
                    .entry((prefix, own_group))
                    .and_modify(|later_index| *later_index = index.min(*later_index))
                    .or_insert(index);
            }

            later_properties
                .entry((prefix, affected))
                .and_modify(|later_index| *later_index = index.min(*later_index))
                .or_insert(index);
        }
    }

    false
}

// Physical and logical declarations can overlap under a writing mode inherited from another rule.
fn property_alias_groups(property: &str) -> Option<(&'static str, &'static str)> {
    let (logical_group, physical_group) = if property.starts_with("corner-") {
        (":logical-corner-shape", ":physical-corner-shape")
    } else if property.starts_with("border-") {
        if property.ends_with("-color") {
            (":logical-border-color", ":physical-border-color")
        } else if property.ends_with("-style") {
            (":logical-border-style", ":physical-border-style")
        } else if property.ends_with("-width") {
            (":logical-border-width", ":physical-border-width")
        } else {
            return None;
        }
    } else {
        return None;
    };

    if property.contains("-block-")
        || property.contains("-inline-")
        || property.starts_with("corner-start-")
        || property.starts_with("corner-end-")
    {
        Some((logical_group, physical_group))
    } else if property.contains("-top-")
        || property.contains("-right-")
        || property.contains("-bottom-")
        || property.contains("-left-")
    {
        Some((physical_group, logical_group))
    } else {
        None
    }
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
