use biome_css_syntax::{
    AnyCssRoot, CssLanguage, CssMediaAtRule, CssScopeAtRule, CssSupportsAtRule, CssSyntaxKind,
    CssSyntaxToken, ScssAtRootAtRule, T,
};
use biome_rowan::{AstNode, AstPtr, SyntaxKindSet, TextRange, TokenText};
use rustc_hash::FxHashMap;
use std::{collections::BTreeMap, sync::LazyLock};

use super::model::{
    CssGlobalCustomVariableData, CssModelDeclarationData, CssPropertyAtRuleData, ResolvedSelector,
    RuleData, RuleId, SelectorData, SemanticModel, SemanticModelData, Specificity, selector_tokens,
};
use crate::events::SemanticEvent;
use crate::model::{AnyCssSelectorLike, AnyRuleStart};

static EXPLICIT_COMBINATOR_KINDS: LazyLock<SyntaxKindSet<CssLanguage>> = LazyLock::new(|| {
    SyntaxKindSet::of(T![>])
        .union(SyntaxKindSet::of(T![+]))
        .union(SyntaxKindSet::of(T![~]))
        .union(SyntaxKindSet::of(T![||]))
});
const NON_SELECTOR_RULE_KINDS: SyntaxKindSet<CssLanguage> = CssMediaAtRule::KIND_SET
    .union(CssScopeAtRule::KIND_SET)
    .union(CssSupportsAtRule::KIND_SET);

pub struct SemanticModelBuilder {
    root: AnyCssRoot,
    /// All rules, indexed by RuleId
    all_rules: Vec<RuleData>,
    /// IDs of top-level rules only
    top_level_rule_ids: Vec<RuleId>,
    root_declarations: Vec<CssModelDeclarationData>,
    global_custom_variables: FxHashMap<TokenText, CssGlobalCustomVariableData>,
    at_property_rules: Vec<CssPropertyAtRuleData>,
    at_property_by_range: FxHashMap<TextRange, usize>,
    last_at_property_by_name: FxHashMap<TokenText, usize>,
    /// Stack of rule IDs to keep track of the current rule hierarchy
    current_rule_stack: Vec<RuleId>,
    /// Map from text range to RuleId
    range_to_rule_id: BTreeMap<TextRange, RuleId>,
    /// Indicates if the current node is within a `:root` selector
    is_in_root_selector: bool,
}

impl SemanticModelBuilder {
    pub fn new(root: AnyCssRoot) -> Self {
        Self {
            root,
            all_rules: Vec::new(),
            top_level_rule_ids: Vec::new(),
            root_declarations: Vec::new(),
            current_rule_stack: Vec::new(),
            global_custom_variables: FxHashMap::default(),
            at_property_rules: Vec::new(),
            at_property_by_range: FxHashMap::default(),
            last_at_property_by_name: FxHashMap::default(),
            range_to_rule_id: BTreeMap::default(),
            is_in_root_selector: false,
        }
    }

    fn get_last_parent_selector_rule(&self) -> Option<&RuleData> {
        let mut iterator = self.current_rule_stack.iter().rev();
        let mut current_parent_id = iterator
            .next()
            .and_then(|rule_id| self.all_rules.get(rule_id.index()))
            .and_then(|rule| rule.parent_id);

        loop {
            if let Some(parent_id) = &current_parent_id {
                let rule = self.all_rules.get(parent_id.index())?;
                let typed_node = rule.node.to_node(self.root.syntax());
                let is_at_root_without_selector =
                    if let AnyRuleStart::ScssAtRootAtRule(at_root) = &typed_node {
                        at_root.selector().is_none()
                    } else {
                        false
                    };
                if NON_SELECTOR_RULE_KINDS.matches(typed_node.syntax().kind())
                    || is_at_root_without_selector
                {
                    current_parent_id = iterator
                        .next()
                        .and_then(|rule_id| self.all_rules.get(rule_id.index()))
                        .and_then(|rule| rule.parent_id);
                } else {
                    return Some(rule);
                }
            } else {
                return None;
            }
        }
    }

    fn selector_detaches_from_parent(&self, node: &AnyCssSelectorLike) -> bool {
        if selector_tokens(node)
            .iter()
            .any(|token| token.kind() == CssSyntaxKind::AMP)
        {
            return false;
        }
        let Some(parent_rule) = self.get_last_parent_selector_rule() else {
            return false;
        };
        let parent_range = parent_rule.range(&self.root);

        node.syntax()
            .ancestors()
            .take_while(|ancestor| ancestor.text_trimmed_range() != parent_range)
            .filter_map(ScssAtRootAtRule::cast)
            .any(|at_root| at_root_excludes_style_rules(&at_root))
    }

    pub fn build(self) -> SemanticModel {
        let data = SemanticModelData {
            root: self.root.syntax().as_send().expect("To be a root node"),
            all_rules: self.all_rules,
            top_level_rule_ids: self.top_level_rule_ids,
            root_declarations: self.root_declarations,
            global_custom_variables: self.global_custom_variables,
            at_property_rules: self.at_property_rules,
            at_property_by_range: self.at_property_by_range,
            last_at_property_by_name: self.last_at_property_by_name,
            range_to_rule_id: self.range_to_rule_id,
        };
        SemanticModel::new(data)
    }

    #[inline]
    pub fn push_event(&mut self, event: SemanticEvent) {
        match event {
            SemanticEvent::RuleStart(node) => {
                let new_rule_id = RuleId::new(self.all_rules.len());

                let parent_id = self.current_rule_stack.last().copied();

                let new_rule = RuleData {
                    id: new_rule_id,
                    node: AstPtr::new(&node),
                    selectors: Vec::new(),
                    declarations: Vec::new(),
                    parent_id,
                    child_ids: Vec::new(),
                    specificity: Specificity::default(),
                };

                if let Some(&parent_id) = self.current_rule_stack.last() {
                    self.all_rules[parent_id.index()]
                        .child_ids
                        .push(new_rule_id);
                }

                self.all_rules.push(new_rule);
                self.current_rule_stack.push(new_rule_id);
            }
            SemanticEvent::RuleEnd => {
                if let Some(completed_rule_id) = self.current_rule_stack.pop() {
                    let range = self.all_rules[completed_rule_id.index()].range(&self.root);
                    self.range_to_rule_id.insert(range, completed_rule_id);

                    if self.current_rule_stack.is_empty() {
                        self.top_level_rule_ids.push(completed_rule_id);
                    }
                }
            }
            SemanticEvent::SelectorDeclaration { node, specificity } => {
                if let Some(&current_rule_id) = self.current_rule_stack.last() {
                    let current_tokens = selector_tokens(&node);
                    let nesting_selector_count = current_tokens
                        .iter()
                        .filter(|token| token.kind() == CssSyntaxKind::AMP)
                        .count();
                    let detaches_from_parent = self.selector_detaches_from_parent(&node);
                    let parent_rule = if detaches_from_parent {
                        None
                    } else {
                        self.get_last_parent_selector_rule()
                    };
                    let parent_specificity = parent_rule
                        .map(|rule| {
                            rule.selectors
                                .iter()
                                .map(|selector| selector.specificity)
                                .max()
                                .unwrap_or_default()
                        })
                        .unwrap_or_default();
                    let parent_specificity = (0..nesting_selector_count.max(1))
                        .fold(Specificity::default(), |specificity, _| {
                            specificity + parent_specificity
                        });
                    let resolved_selectors: Vec<ResolvedSelector> =
                        if let Some(parent_rule) = parent_rule {
                            resolve_selector(&current_tokens, &parent_rule.selectors)
                        } else {
                            vec![ResolvedSelector(
                                current_tokens
                                    .iter()
                                    .map(|t| (t.kind(), t.token_text_trimmed()))
                                    .collect(),
                            )]
                        };

                    let current_rule = &mut self.all_rules[current_rule_id.index()];
                    let combined = parent_specificity + specificity;

                    for resolved in resolved_selectors {
                        current_rule.selectors.push(SelectorData {
                            node: AstPtr::new(&node),
                            resolved,
                            specificity: combined,
                        });
                    }

                    if combined > current_rule.specificity {
                        current_rule.specificity = combined;
                    }
                }
            }
            SemanticEvent::PropertyDeclaration {
                node,
                property,
                value,
            } => {
                let is_global_var =
                    self.is_in_root_selector && property.syntax().text_trimmed().starts_with("--");

                if let Ok(property_name) = property.value() {
                    let declaration = CssModelDeclarationData {
                        declaration: AstPtr::new(&node),
                        property: AstPtr::new(&property),
                        value,
                        property_name: property_name.clone(),
                    };
                    if is_global_var {
                        let variable = self
                            .global_custom_variables
                            .entry(property_name.clone())
                            .or_default();
                        variable.root = Some(declaration.clone());
                    }
                    if let Some(&current_rule_id) = self.current_rule_stack.last() {
                        self.all_rules[current_rule_id.index()]
                            .declarations
                            .push(declaration);
                    } else if self.root.as_css_declaration_snippet_root().is_some() {
                        self.root_declarations.push(declaration);
                    }
                }
            }
            SemanticEvent::RootSelectorStart => {
                self.is_in_root_selector = true;
            }
            SemanticEvent::RootSelectorEnd => {
                self.is_in_root_selector = false;
            }
            SemanticEvent::AtProperty {
                property,
                initial_value,
                syntax,
                inherits,
                inherits_requires_evaluation,
                range,
            } => {
                if let Ok(property_name) = property.value_token() {
                    let property_name = property_name.token_text_trimmed();
                    self.global_custom_variables
                        .entry(property_name.clone())
                        .or_default();
                    let index = self.at_property_rules.len();
                    let rule = CssPropertyAtRuleData {
                        name: property_name.clone(),
                        property: AstPtr::new(&property),
                        initial_value,
                        syntax,
                        inherits,
                        inherits_requires_evaluation,
                        range,
                    };
                    let is_registration_candidate = rule.is_registration_candidate(&self.root);
                    self.at_property_rules.push(rule);
                    self.at_property_by_range.insert(range, index);
                    if is_registration_candidate {
                        self.last_at_property_by_name.insert(property_name, index);
                    }
                }
            }
        }
    }
}

fn at_root_excludes_style_rules(at_root: &ScssAtRootAtRule) -> bool {
    let Some(query) = at_root.query() else {
        return true;
    };
    let Ok(modifier) = query.modifier() else {
        return true;
    };
    let includes_rules = modifier.text_trimmed().eq_ignore_ascii_case("with");
    let names_rules = query.queries().into_iter().any(|query| {
        let query = query.to_trimmed_string();
        query.eq_ignore_ascii_case("all") || query.eq_ignore_ascii_case("rule")
    });
    names_rules != includes_rules
}

/// Synthetic space-literal `(kind, TokenText)` pair used as the implicit descendant
/// combinator when a nested selector contains no `&` reference.
fn space_combinator() -> (CssSyntaxKind, TokenText) {
    use biome_rowan::SyntaxKind;
    (
        CssSyntaxKind::CSS_SPACE_LITERAL,
        TokenText::new_raw(CssSyntaxKind::CSS_SPACE_LITERAL.to_raw(), " "),
    )
}

fn is_explicit_combinator(kind: CssSyntaxKind) -> bool {
    EXPLICIT_COMBINATOR_KINDS.matches(kind)
}

/// Resolves the `current` token sequence against each parent [`Selector`],
/// producing one [`ResolvedSelector`] per parent selector.
///
/// Resolution rules (per the CSS nesting spec):
/// - If any token in `current` is an `AMP` (`&`), every such occurrence is
///   replaced in-place by the full token sequence of the parent selector.
/// - If there is no `&`, the parent token sequence is prepended. A synthetic
///   descendant combinator is inserted unless either sequence already meets at
///   an explicit combinator.
///
/// Tokens are stored as `(CssSyntaxKind, TokenText)` pairs so that the
/// `Display` impl can reconstruct canonical whitespace around combinators.
fn resolve_selector(current: &[CssSyntaxToken], parents: &[SelectorData]) -> Vec<ResolvedSelector> {
    let has_amp = current.iter().any(|t| t.kind() == CssSyntaxKind::AMP);

    parents
        .iter()
        .map(|parent| {
            let parent_tokens = &parent.resolved.0;
            if has_amp {
                let amp_count = current
                    .iter()
                    .filter(|t| t.kind() == CssSyntaxKind::AMP)
                    .count();
                let non_amp_count = current.len() - amp_count;
                let capacity = amp_count * parent_tokens.len() + non_amp_count;

                let mut tokens = Vec::with_capacity(capacity);
                for t in current {
                    if t.kind() == CssSyntaxKind::AMP {
                        tokens.extend(parent_tokens.iter().cloned());
                    } else {
                        tokens.push((t.kind(), t.token_text_trimmed()));
                    }
                }
                ResolvedSelector(tokens)
            } else {
                let mut tokens = Vec::with_capacity(parent_tokens.len() + 1 + current.len());
                tokens.extend(parent_tokens.iter().cloned());
                let has_combinator_boundary = current
                    .first()
                    .is_some_and(|token| is_explicit_combinator(token.kind()))
                    || parent_tokens
                        .last()
                        .is_some_and(|(kind, _)| is_explicit_combinator(*kind));
                if !has_combinator_boundary {
                    tokens.push(space_combinator());
                }
                tokens.extend(current.iter().map(|t| (t.kind(), t.token_text_trimmed())));
                ResolvedSelector(tokens)
            }
        })
        .collect()
}
