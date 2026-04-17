use biome_css_syntax::{
    AnyCssRoot, CssComplexSelector, CssComposesPropertyValue, CssCompoundSelector,
    CssContainerAtRule, CssDashedIdentifier, CssDeclaration, CssGenericComponentValueList,
    CssIdentifier, CssMediaAtRule, CssNestedQualifiedRule, CssQualifiedRule,
    CssStartingStyleAtRule, CssSupportsAtRule, CssSyntaxKind, CssSyntaxToken, ScssExpression,
};
use biome_rowan::{
    AstNode, AstNodeList, AstPtr, Direction, SendNode, SyntaxKind, SyntaxResult, TextRange,
    TextSize, TokenText, declare_node_union,
};
use biome_string_case::StrOnlyExtension;
use rustc_hash::FxHashMap;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::sync::Arc;

/// The façade for all semantic information of a CSS document.
///
/// This struct provides access to the root, rules, and individual nodes of the CSS document.
/// It holds a reference-counted pointer to the internal `SemanticModelData`.
#[derive(Clone, Debug)]
pub struct SemanticModel {
    pub(crate) data: Arc<SemanticModelData>,
    root: SendNode,
}

impl SemanticModel {
    pub(crate) fn new(data: SemanticModelData, root: SendNode) -> Self {
        Self {
            data: Arc::new(data),
            root,
        }
    }

    pub fn root(&self) -> AnyCssRoot {
        self.root.to_language_root::<AnyCssRoot>()
    }

    /// Returns a slice of all top-level rules in the CSS document.
    pub fn rules(&self) -> &[Rule] {
        &self.data.rules
    }

    pub fn global_custom_variables(&self) -> &FxHashMap<String, CssGlobalCustomVariable> {
        &self.data.global_custom_variables
    }

    pub fn get_rule_by_id(&self, id: &RuleId) -> Option<&Rule> {
        self.data.rules_by_id.get(id)
    }

    /// Returns the rule that contains the given range.
    pub fn get_rule_by_range(&self, target_range: TextRange) -> Option<&Rule> {
        // Generally, this function narrows down the search before finding the most specific rule for better performance.
        // But when the target range starts from 0, the BTreeMap's range method may not work as expected due to
        // the comparison semantics of TextRange.

        // Handle the edge case where the target range starts from 0.
        if target_range.start() == TextSize::from(0) {
            self.data
                .range_to_rule
                .iter()
                .rev()
                .find(|&(&range, _)| range.contains_range(target_range))
                .map(|(_, rule)| rule)
        } else {
            self.data
                .range_to_rule
                .range(..=target_range)
                .rev()
                .find(|&(&range, _)| range.contains_range(target_range))
                .map(|(_, rule)| rule)
        }
    }

    /// Returns an iterator over the specificity of all rules in source order.
    pub fn specificity_of_rules(&self) -> impl Iterator<Item = Specificity> + '_ {
        self.data
            .range_to_rule
            .values()
            .flat_map(|rule| rule.selectors())
            .map(|selector| selector.specificity())
    }

    pub fn is_media_rule(&self, rule: &Rule) -> bool {
        matches!(rule.node(&self.root()), AnyRuleStart::CssMediaAtRule(_))
    }
}

/// Contains the internal data of a `SemanticModel`.
///
/// This struct holds the root of the CSS document, a mapping of nodes by their range,
/// and a list of all rules in the document.
#[derive(Debug)]
pub(crate) struct SemanticModelData {
    /// List of all top-level rules in the CSS document
    pub(crate) rules: Vec<Rule>,
    /// Map of CSS variables declared in the `:root` selector or using the @property rule.
    pub(crate) global_custom_variables: FxHashMap<String, CssGlobalCustomVariable>,
    /// Map of all the rules by their id
    pub(crate) rules_by_id: FxHashMap<RuleId, Rule>,
    /// Map of the range of each rule to the rule itself
    pub(crate) range_to_rule: BTreeMap<TextRange, Rule>,
}

/// Represents a CSS rule set, including its selectors, declarations, and nested rules.
///
/// ┌─ Rule Set ──────────────────────────┐
/// │                                     │
/// │  p {                ← Selector      │
/// │    color: red;      ← Declaration   │
/// │     │       │                       │
/// │     │       └─ Value                │
/// │     └─ Property                     |
/// |                                     |
/// │    .child {         ← children      │
/// │      color: blue;                   |
/// |    }                                |
/// │  }                                  │
/// └─────────────────────────────────────┘
///
#[derive(Debug, Clone)]
pub struct Rule {
    pub(crate) id: RuleId,
    pub(crate) node: AstPtr<AnyRuleStart>,
    /// The selectors associated with this rule.
    pub(crate) selectors: Vec<Selector>,
    /// The declarations within this rule.
    pub(crate) declarations: Vec<CssModelDeclaration>,
    /// The id of the parent rule
    pub(crate) parent_id: Option<RuleId>,
    /// The ids of the child rules
    pub(crate) child_ids: Vec<RuleId>,
    /// Specificity context of this rule
    /// See https://drafts.csswg.org/selectors-4/#specificity-rules
    pub(crate) specificity: Specificity,
}

impl Rule {
    pub fn id(&self) -> RuleId {
        self.id
    }

    pub fn node(&self, css_root: &AnyCssRoot) -> AnyRuleStart {
        self.node.to_node(css_root.syntax())
    }

    pub fn range(&self, css_root: &AnyCssRoot) -> TextRange {
        self.node
            .to_node(css_root.syntax())
            .syntax()
            .text_trimmed_range()
    }

    pub fn selectors(&self) -> &[Selector] {
        &self.selectors
    }

    pub fn declarations(&self) -> &[CssModelDeclaration] {
        &self.declarations
    }

    pub fn parent_id(&self) -> Option<&RuleId> {
        self.parent_id.as_ref()
    }

    pub fn child_ids(&self) -> &[RuleId] {
        &self.child_ids
    }

    pub fn specificity(&self) -> Specificity {
        self.specificity
    }
}

declare_node_union! {
    pub AnyRuleStart = CssQualifiedRule | CssNestedQualifiedRule | CssContainerAtRule | CssMediaAtRule | CssStartingStyleAtRule | CssSupportsAtRule
}

impl AnyRuleStart {
    pub fn text_trimmed_range(&self) -> TextRange {
        match self {
            Self::CssQualifiedRule(node) => node.syntax().text_trimmed_range(),
            Self::CssNestedQualifiedRule(node) => node.syntax().text_trimmed_range(),
            Self::CssContainerAtRule(node) => node.syntax().text_trimmed_range(),
            Self::CssMediaAtRule(node) => node.syntax().text_trimmed_range(),
            Self::CssStartingStyleAtRule(node) => node.syntax().text_trimmed_range(),
            Self::CssSupportsAtRule(node) => node.syntax().text_trimmed_range(),
        }
    }
}

declare_node_union! {
    pub AnyCssSelectorLike = CssCompoundSelector | CssComplexSelector
}

impl AnyCssSelectorLike {
    pub fn has_nesting_selectors(&self) -> bool {
        match self {
            Self::CssCompoundSelector(node) => !node.nesting_selectors().is_empty(),
            Self::CssComplexSelector(node) => node.nesting_level() > 0,
        }
    }

    pub fn nesting_level(&self) -> usize {
        match self {
            Self::CssCompoundSelector(node) => node.nesting_selectors().len(),
            Self::CssComplexSelector(node) => node.nesting_level(),
        }
    }
}

/// A resolved CSS selector represented as an ordered sequence of `(kind, text)` pairs.
///
/// Rather than building a string, the resolved selector stores the sequence of
/// tokens that make up the selector after nesting/`&` resolution. This avoids
/// string allocation and trivia contamination: only non-trivia tokens are stored,
/// so embedded whitespace or comments in the source do not affect equality.
///
/// The kind is stored alongside each token text so that the `Display` impl can
/// reconstruct the canonical whitespace representation:
/// - `CSS_SPACE_LITERAL` tokens are emitted as-is (they are already a space).
/// - Explicit combinator tokens (`>`, `+`, `~`, `||`) are surrounded by spaces.
/// - All other tokens are emitted without surrounding spaces.
///
/// For a top-level selector like `.foo > .bar` the token pairs are
/// `[(DOT, ".foo"), (CSS_SPACE_LITERAL_OR_GT, ">"), (DOT, ".bar")]`.
///
/// Two `ResolvedSelector`s are equal when all their `(kind, text)` pairs match,
/// enabling duplicate-selector detection without any string allocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSelector(pub(crate) Vec<(CssSyntaxKind, TokenText)>);

impl ResolvedSelector {
    /// Returns an iterator over the `(kind, text)` pairs that make up this selector.
    pub fn tokens(&self) -> impl Iterator<Item = &(CssSyntaxKind, TokenText)> {
        self.0.iter()
    }

    /// Returns `true` if `kind` represents a combinator token (explicit or implicit).
    ///
    /// Combinator tokens separate compound selectors. `CSS_SPACE_LITERAL` is the
    /// implicit descendant combinator; the others are explicit.
    pub fn is_combinator(kind: CssSyntaxKind) -> bool {
        matches!(
            kind,
            CssSyntaxKind::CSS_SPACE_LITERAL
                | CssSyntaxKind::R_ANGLE
                | CssSyntaxKind::PLUS
                | CssSyntaxKind::TILDE
                | CssSyntaxKind::PIPE2
        )
    }

    /// Returns `true` if `kind` represents an explicit CSS combinator
    /// that should be surrounded by spaces when displayed.
    fn is_explicit_combinator(kind: CssSyntaxKind) -> bool {
        matches!(
            kind,
            CssSyntaxKind::R_ANGLE
                | CssSyntaxKind::PLUS
                | CssSyntaxKind::TILDE
                | CssSyntaxKind::PIPE2
        )
    }

    /// Returns `true` if `kind` starts a new simple selector within a compound.
    ///
    /// This is used to split compound selectors into their constituent simple
    /// selectors for normalization.
    fn starts_simple_selector(kind: CssSyntaxKind) -> bool {
        matches!(
            kind,
            // Class selector: `.foo`
            CssSyntaxKind::DOT
            // ID selector: `#foo`
            | CssSyntaxKind::HASH
            // Attribute selector: `[attr]`
            | CssSyntaxKind::L_BRACK
            // Pseudo-class: `:hover`, pseudo-element: `::before`
            | CssSyntaxKind::COLON
            | CssSyntaxKind::COLON2
        )
    }

    /// Produces a normalized string representation of this selector for use in
    /// duplicate-selector comparison.
    ///
    /// Normalization follows the same rules as stylelint's `normalizeSelector`:
    ///
    /// 1. **Whitespace**: already stripped (trivia excluded from token list).
    /// 2. **Type selector case**: HTML tag names are case-insensitive, so bare
    ///    type selectors (`div`, `SPAN`, `P`) are lowercased.
    /// 3. **Compound selector ordering**: within a compound selector (tokens between
    ///    two combinators), the individual simple selectors (`.a`, `#id`, `[attr]`,
    ///    `:hover`) are sorted alphabetically so that `.a.b` and `.b.a` are
    ///    considered equal.
    ///
    /// The combinator representation is canonical: space-literal becomes `" "`,
    /// explicit combinators keep their text with surrounding spaces.
    pub fn normalize(&self) -> String {
        // Step 1: split token list at combinators into (combinator_str, compound_tokens) pairs.
        // The first pair has an empty combinator.
        let mut compounds: Vec<(String, Vec<(CssSyntaxKind, &TokenText)>)> = Vec::new();
        let mut current_combinator = String::new();
        let mut current_tokens: Vec<(CssSyntaxKind, &TokenText)> = Vec::new();

        for (kind, text) in &self.0 {
            if Self::is_combinator(*kind) {
                compounds.push((current_combinator, current_tokens));
                // Represent all combinators canonically with surrounding spaces
                current_combinator = if *kind == CssSyntaxKind::CSS_SPACE_LITERAL {
                    " ".to_string()
                } else {
                    format!(" {text} ")
                };
                current_tokens = Vec::new();
            } else {
                current_tokens.push((*kind, text));
            }
        }
        // Push the last compound (or the only one for simple selectors)
        compounds.push((current_combinator, current_tokens));

        // Step 2: for each compound, split into simple-selector chunks and sort them.
        let mut result = String::new();

        for (combinator, tokens) in &compounds {
            result.push_str(combinator);

            // Split the compound into simple-selector chunks.
            // A new chunk starts at each token that "opens" a new simple selector
            // (DOT, HASH, L_BRACK, COLON, COLON2), or at the very first token
            // (which may be a type selector: IDENT, STAR, or even AMP for nesting).
            let mut chunks: Vec<String> = Vec::new();
            let mut chunk = String::new();

            for (i, (kind, text)) in tokens.iter().enumerate() {
                let starts_new = i == 0 || Self::starts_simple_selector(*kind);
                if starts_new && i != 0 {
                    if !chunk.is_empty() {
                        chunks.push(chunk);
                    }
                    chunk = String::new();
                }

                // Lowercase bare type selectors (IDENT at position 0 of the compound,
                // not preceded by DOT/HASH/COLON — i.e., the first token is the type).
                // In practice: if this is the first token of the compound and it is
                // an IDENT (not DOT/HASH/COLON/COLON2), treat it as a type selector.
                let token_text =
                    if i == 0 && matches!(*kind, CssSyntaxKind::IDENT | CssSyntaxKind::STAR) {
                        text.to_lowercase_cow()
                    } else {
                        text.to_string().into()
                    };

                chunk.push_str(&token_text);
            }
            if !chunk.is_empty() {
                chunks.push(chunk);
            }

            // Sort simple-selector chunks within the compound for canonical order.
            // This makes `.b.a` == `.a.b`.
            //
            // The first chunk may be a type selector (IDENT) or universal selector
            // (*), which by CSS grammar must always come first in a compound.  Only
            // sort the non-type chunks (index 1 onward) so that `a:hover` and
            // `a.foo` are never reordered to `:hovera` or `.fooa`.
            let (type_prefix, rest) = if chunks
                .first()
                .is_some_and(|c| !c.starts_with(['.', '#', '[', ':', '&']))
            {
                chunks.split_at_mut(1)
            } else {
                chunks.split_at_mut(0)
            };
            rest.sort();
            result.push_str(&type_prefix.join(""));
            result.push_str(&rest.join(""));
        }

        result
    }
}

impl std::fmt::Display for ResolvedSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (kind, text) in &self.0 {
            if Self::is_explicit_combinator(*kind) {
                write!(f, " {text} ")?;
            } else {
                write!(f, "{text}")?;
            }
        }
        Ok(())
    }
}

/// Represents a CSS selector.
/// /// ```css
/// span {
/// ^^^^
///   color: red;
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Selector {
    pub(crate) node: AstPtr<AnyCssSelectorLike>,
    /// The resolved selector, accounting for nesting and `&` references.
    /// For top-level selectors this is the token sequence from the source.
    /// For nested selectors each `&` is replaced by the parent token sequence,
    /// and a space-literal token is inserted when there is no `&`.
    pub(crate) resolved: ResolvedSelector,
    /// The specificity of the selector.
    pub(crate) specificity: Specificity,
}

impl Selector {
    pub fn node(&self, root: &AnyCssRoot) -> AnyCssSelectorLike {
        self.node.to_node(root.syntax())
    }

    /// Returns the resolved selector, accounting for CSS nesting and `&` references.
    pub fn resolved(&self) -> &ResolvedSelector {
        &self.resolved
    }

    pub fn range(&self, root: &AnyCssRoot) -> TextRange {
        self.node
            .to_node(root.syntax())
            .syntax()
            .text_trimmed_range()
    }

    pub fn specificity(&self) -> Specificity {
        self.specificity
    }
}

/// Collects the non-trivia tokens of a selector node as [`CssSyntaxToken`]s.
///
/// Trivia tokens (whitespace, comments, newlines) are skipped because the
/// meaningful content of a selector is carried entirely by non-trivia tokens
/// and the explicit/implicit combinator tokens (including `CSS_SPACE_LITERAL`).
///
/// The returned tokens retain their [`CssSyntaxKind`] so that callers can
/// detect `AMP` (`&`) tokens during resolution without re-parsing text.
pub fn selector_tokens(node: &AnyCssSelectorLike) -> Vec<CssSyntaxToken> {
    node.syntax()
        .descendants_tokens(Direction::Next)
        .filter(|tok: &CssSyntaxToken| !tok.kind().is_trivia())
        .collect()
}

/// Represents the specificity of a CSS selector.
///
/// This specificity is represented as a tuple of three `u32` values,
/// corresponding to (ID selectors, class selectors, type selectors).
/// More details https://developer.mozilla.org/en-US/docs/Web/CSS/Specificity
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Copy)]
pub struct Specificity(pub u32, pub u32, pub u32);

/// In CSS, when selectors are combined (e.g., in a compound selector), their specificities are summed.
/// This implementation mirrors that behavior by adding the ID, class, and type selector counts separately.
///
/// Consider the following selector.
/// ```css
/// #id .class {}
/// ```
///
/// The specificity of each component is as follows:
/// - `#id` has a specificity of `Specificity(1, 0, 0)`
/// - `.class` has a specificity of `Specificity(0, 1, 0)`
///
/// Therefore, the combined selector `#id .class` has a specificity of:
/// - `Specificity(1 + 0, 0 + 1, 0 + 0) = Specificity(1, 1, 0)`
///
/// More details https://drafts.csswg.org/selectors/#example-d97bd125
impl std::ops::Add for Specificity {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::AddAssign for Specificity {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

/// Formats the `Specificity` instance to match the notation used in the official CSS specification.
///
/// More details https://www.w3.org/TR/selectors-4/#specificity-rules
impl std::fmt::Display for Specificity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

/// Represents a CSS declaration (property-value pair).
/// ```css
/// a {
///   color: red;
///   ^^^^^^^^^^^
/// }
/// ```
#[derive(Debug, Clone)]
pub struct CssModelDeclaration {
    pub(crate) declaration: AstPtr<CssDeclaration>,
    pub(crate) property: AstPtr<CssProperty>,
    pub(crate) value: CssPropertyInitialValue,
}

impl CssModelDeclaration {
    pub fn declaration(&self, root: &AnyCssRoot) -> CssDeclaration {
        self.declaration.to_node(root.syntax())
    }

    pub fn property(&self, root: &AnyCssRoot) -> CssProperty {
        self.property.to_node(root.syntax())
    }

    pub fn value(&self) -> &CssPropertyInitialValue {
        &self.value
    }
}

declare_node_union! {
    pub CssProperty = CssDashedIdentifier | CssIdentifier
}

impl CssProperty {
    pub fn value(&self) -> SyntaxResult<TokenText> {
        let token = match self {
            Self::CssDashedIdentifier(node) => node.value_token()?,
            Self::CssIdentifier(node) => node.value_token()?,
        };

        Ok(token.token_text_trimmed())
    }
}

#[derive(Debug, Clone)]
pub enum CssPropertyInitialValue {
    GenericComponent(AstPtr<CssGenericComponentValueList>),
    Composes(AstPtr<CssComposesPropertyValue>),
    ScssExpression(AstPtr<ScssExpression>),
}

impl From<CssGenericComponentValueList> for CssPropertyInitialValue {
    fn from(value: CssGenericComponentValueList) -> Self {
        Self::GenericComponent(AstPtr::new(&value))
    }
}

impl From<CssComposesPropertyValue> for CssPropertyInitialValue {
    fn from(value: CssComposesPropertyValue) -> Self {
        Self::Composes(AstPtr::new(&value))
    }
}

impl From<ScssExpression> for CssPropertyInitialValue {
    fn from(value: ScssExpression) -> Self {
        Self::ScssExpression(AstPtr::new(&value))
    }
}

/// Represents a CSS global custom variable declaration.
/// This can be declared in the `:root` selector or using the `@property` rule.
/// ```css
/// :root {
///   --custom-color: red;
/// }
///
/// @property --item-size {
///   syntax: "<percentage>";
///   inherits: true;
///   initial-value: 40%;
/// }
/// ```
#[derive(Debug, Clone)]
pub enum CssGlobalCustomVariable {
    Root(CssModelDeclaration),
    AtProperty {
        property: AstPtr<CssProperty>,
        syntax: Option<String>,
        inherits: Option<bool>,
        initial_value: Option<CssPropertyInitialValue>,
        range: TextRange,
    },
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuleId(u32);

impl RuleId {
    pub fn new(index: usize) -> Self {
        // SAFETY: We didn't handle files exceeding `u32::MAX` bytes.
        // Thus, it isn't possible to exceed `u32::MAX` bindings.
        Self(index as u32)
    }

    pub fn index(self) -> usize {
        self.0 as usize
    }
}
