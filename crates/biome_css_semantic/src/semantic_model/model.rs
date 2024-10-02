use std::{collections::BTreeMap, rc::Rc};

use biome_css_syntax::{AnyCssSelector, CssRoot};
use biome_rowan::{TextRange, TextSize};
use rustc_hash::FxHashMap;

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

    pub fn get_rule_by_id(&self, id: RuleId) -> Option<&Rule> {
        self.data.rules_by_id.get(&id)
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
                .find(|(&range, _)| range.contains_range(target_range))
                .map(|(_, rule)| rule)
        } else {
            self.data
                .range_to_rule
                .range(..=target_range)
                .rev()
                .find(|(&range, _)| range.contains_range(target_range))
                .map(|(_, rule)| rule)
        }
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
    pub id: RuleId,
    /// The selectors associated with this rule.
    pub selectors: Vec<Selector>,
    /// The declarations within this rule.
    pub declarations: Vec<CssDeclaration>,
    /// The id of the parent rule
    pub parent_id: Option<RuleId>,
    /// The ids of the child rules
    pub child_ids: Vec<RuleId>,
    /// The text range of this rule in the source document.
    pub range: TextRange,
    /// Specificity context of this rule  
    /// See https://drafts.csswg.org/selectors-4/#specificity-rules
    pub specificity: Specificity,
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
    /// The name of the selector.
    pub name: String,
    /// The text range of the selector in the source document.
    pub range: TextRange,
    pub original: AnyCssSelector,
    /// The specificity of the selector.
    pub specificity: Specificity,
}

/// Represents the specificity of a CSS selector.
///
/// This specificity is represented as a tuple of three `u32` values,
/// corresponding to (ID selectors, class selectors, type selectors).
/// More details https://developer.mozilla.org/en-US/docs/Web/CSS/Specificity
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
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
    type Output = Specificity;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::AddAssign for Specificity {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = rhs.0;
        self.1 = rhs.1;
        self.2 = rhs.2;
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
pub struct CssDeclaration {
    pub property: CssProperty,
    pub value: CssValue,
    pub range: TextRange,
}

#[derive(Debug, Clone, Default)]
pub struct CssProperty {
    pub name: String,
    pub range: TextRange,
}

#[derive(Debug, Clone, Default)]
pub struct CssValue {
    pub text: String,
    pub range: TextRange,
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
    Root(CssDeclaration),
    AtProperty {
        property: CssProperty,
        syntax: Option<String>,
        inherits: Option<bool>,
        initial_value: Option<CssValue>,
        range: TextRange,
    },
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuleId(u32);

impl RuleId {
    pub fn new(index: usize) -> Self {
        // SAFETY: We didn't handle files execedding `u32::MAX` bytes.
        // Thus, it isn't possible to execedd `u32::MAX` bindings.
        Self(index as u32)
    }

    pub fn index(self) -> usize {
        self.0 as usize
    }
}
