use biome_css_syntax::{
    CssComplexSelector, CssComposesPropertyValue, CssCompoundSelector, CssContainerAtRule,
    CssDashedIdentifier, CssDeclaration, CssGenericComponentValueList, CssIdentifier,
    CssMediaAtRule, CssNestedQualifiedRule, CssQualifiedRule, CssRoot, CssStartingStyleAtRule,
    CssSupportsAtRule,
};
use biome_rowan::{
    AstNode, AstNodeList, SyntaxNodeText, SyntaxResult, TextRange, TextSize, TokenText,
    declare_node_union,
};
use rustc_hash::FxHashMap;
use std::hash::Hash;
use std::{collections::BTreeMap, rc::Rc};

/// The façade for all semantic information of a CSS document.
///
/// This struct provides access to the root, rules, and individual nodes of the CSS document.
/// It holds a reference-counted pointer to the internal `SemanticModelData`.
#[derive(Clone, Debug)]
pub struct SemanticModel {
    pub(crate) data: Rc<SemanticModelData>,
}

impl SemanticModel {
    pub(crate) fn new(data: SemanticModelData) -> Self {
        Self {
            data: Rc::new(data),
        }
    }

    pub fn root(&self) -> &CssRoot {
        &self.data.root
    }

    /// Returns a slice of all rules in the CSS document.
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
}

/// Contains the internal data of a `SemanticModel`.
///
/// This struct holds the root of the CSS document, a mapping of nodes by their range,
/// and a list of all rules in the document.
#[derive(Debug)]
pub(crate) struct SemanticModelData {
    pub(crate) root: CssRoot,
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
    pub(crate) node: AnyRuleStart,
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

    pub fn node(&self) -> &AnyRuleStart {
        &self.node
    }

    pub fn range(&self) -> TextRange {
        self.node.text_trimmed_range()
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

    pub const fn is_media_rule(&self) -> bool {
        matches!(self.node, AnyRuleStart::CssMediaAtRule(_))
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

/// Represents a CSS selector.
/// /// ```css
/// span {
/// ^^^^
///   color: red;
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Selector {
    pub(crate) node: AnyCssSelectorLike,
    /// The specificity of the selector.
    pub(crate) specificity: Specificity,
}

impl Selector {
    pub fn node(&self) -> &AnyCssSelectorLike {
        &self.node
    }

    pub fn text(&self) -> SyntaxNodeText {
        self.node.syntax().text_trimmed()
    }

    pub fn range(&self) -> TextRange {
        self.node.syntax().text_trimmed_range()
    }

    pub fn specificity(&self) -> Specificity {
        self.specificity
    }
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
    pub(crate) declaration: CssDeclaration,
    pub(crate) property: CssProperty,
    pub(crate) value: CssPropertyInitialValue,
}

impl CssModelDeclaration {
    pub fn declaration(&self) -> &CssDeclaration {
        &self.declaration
    }

    pub fn property(&self) -> &CssProperty {
        &self.property
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
    GenericComponent(CssGenericComponentValueList),
    Composes(CssComposesPropertyValue),
}

impl From<CssGenericComponentValueList> for CssPropertyInitialValue {
    fn from(value: CssGenericComponentValueList) -> Self {
        Self::GenericComponent(value)
    }
}

impl From<CssComposesPropertyValue> for CssPropertyInitialValue {
    fn from(value: CssComposesPropertyValue) -> Self {
        Self::Composes(value)
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
        property: CssProperty,
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
