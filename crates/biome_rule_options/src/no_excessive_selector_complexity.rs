use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoExcessiveSelectorComplexityOptions {
    /// The maximum number of ID selectors, such as `#main`, allowed in a single selector.
    ///
    /// When this option isn't set, any number of ID selectors is allowed.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max_ids: Option<u16>,

    /// The maximum number of type selectors, such as `div` or `a`, allowed in a single selector.
    ///
    /// When this option isn't set, any number of type selectors is allowed.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max_types: Option<u16>,

    /// The maximum number of attribute selectors, such as `[type="text"]`, allowed in a single
    /// selector.
    ///
    /// When this option isn't set, any number of attribute selectors is allowed.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max_attributes: Option<u16>,

    /// The maximum number of pseudo-classes, such as `:hover`, allowed in a single selector.
    ///
    /// When this option isn't set, any number of pseudo-classes is allowed.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max_pseudo_classes: Option<u16>,

    /// The maximum number of universal selectors (`*`) allowed in a single selector.
    ///
    /// When this option isn't set, any number of universal selectors is allowed.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max_universals: Option<u16>,

    /// The maximum number of compound selectors allowed in a single selector.
    ///
    /// A compound selector is a group of selectors written together without spaces or
    /// combinators, such as `a.link:hover`.
    ///
    /// When this option isn't set, any number of compound selectors is allowed.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max_compounds: Option<u16>,

    /// The maximum number of combinators allowed in a single selector.
    ///
    /// A combinator joins two compound selectors: a space, `>`, `+`, `~`, or `||`.
    ///
    /// When this option isn't set, any number of combinators is allowed.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max_combinators: Option<u16>,

    /// Combinators that aren't allowed in selectors.
    ///
    /// Accepted values are `" "` (a space, which selects descendants), `">"`, `"+"`, `"~"`,
    /// and `"||"`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub disallow_combinators: Option<Box<[SelectorCombinator]>>,

    /// Pseudo-classes that aren't allowed in selectors, written without the leading colon,
    /// such as `"hover"`. Names are compared without regard to letter case.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub disallow_pseudo_classes: Option<Box<[Box<str>]>>,

    /// Pseudo-elements that aren't allowed in selectors, written without the leading colons,
    /// such as `"before"`. Names are compared without regard to letter case.
    ///
    /// The older single-colon forms `:before`, `:after`, `:first-line`, and `:first-letter`
    /// count as pseudo-elements.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub disallow_pseudo_elements: Option<Box<[Box<str>]>>,

    /// Attribute names that aren't allowed in attribute selectors, such as `"class"`.
    /// Names must match exactly, including letter case.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub disallow_attributes: Option<Box<[Box<str>]>>,

    /// Attribute selector operators that aren't allowed.
    ///
    /// Accepted values are `"="`, `"~="`, `"|="`, `"^="`, `"$="`, and `"*="`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub disallow_attribute_matchers: Option<Box<[AttributeMatcherOperator]>>,
}

impl NoExcessiveSelectorComplexityOptions {
    /// Returns [`Self::max_ids`] if it is set.
    pub fn max_ids(&self) -> Option<u16> {
        self.max_ids
    }

    /// Returns [`Self::max_types`] if it is set.
    pub fn max_types(&self) -> Option<u16> {
        self.max_types
    }

    /// Returns [`Self::max_attributes`] if it is set.
    pub fn max_attributes(&self) -> Option<u16> {
        self.max_attributes
    }

    /// Returns [`Self::max_pseudo_classes`] if it is set.
    pub fn max_pseudo_classes(&self) -> Option<u16> {
        self.max_pseudo_classes
    }

    /// Returns [`Self::max_universals`] if it is set.
    pub fn max_universals(&self) -> Option<u16> {
        self.max_universals
    }

    /// Returns [`Self::max_compounds`] if it is set.
    pub fn max_compounds(&self) -> Option<u16> {
        self.max_compounds
    }

    /// Returns [`Self::max_combinators`] if it is set.
    pub fn max_combinators(&self) -> Option<u16> {
        self.max_combinators
    }

    /// Returns [`Self::disallow_combinators`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn disallow_combinators(&self) -> &[SelectorCombinator] {
        self.disallow_combinators.as_deref().unwrap_or_default()
    }

    /// Returns [`Self::disallow_pseudo_classes`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn disallow_pseudo_classes(&self) -> &[Box<str>] {
        self.disallow_pseudo_classes.as_deref().unwrap_or_default()
    }

    /// Returns [`Self::disallow_pseudo_elements`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn disallow_pseudo_elements(&self) -> &[Box<str>] {
        self.disallow_pseudo_elements.as_deref().unwrap_or_default()
    }

    /// Returns [`Self::disallow_attributes`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn disallow_attributes(&self) -> &[Box<str>] {
        self.disallow_attributes.as_deref().unwrap_or_default()
    }

    /// Returns [`Self::disallow_attribute_matchers`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn disallow_attribute_matchers(&self) -> &[AttributeMatcherOperator] {
        self.disallow_attribute_matchers
            .as_deref()
            .unwrap_or_default()
    }
}

/// A symbol that joins two compound selectors.
#[derive(Clone, Copy, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum SelectorCombinator {
    /// A space, as in `nav a`. Selects elements inside another element.
    #[serde(rename = " ")]
    Descendant,

    /// `>`, as in `ul > li`. Selects direct children.
    #[serde(rename = ">")]
    Child,

    /// `+`, as in `h1 + p`. Selects the element right after another element.
    #[serde(rename = "+")]
    NextSibling,

    /// `~`, as in `h1 ~ p`. Selects any later element with the same parent.
    #[serde(rename = "~")]
    SubsequentSibling,

    /// `||`, as in `col || td`. Selects cells that belong to a table column.
    #[serde(rename = "||")]
    Column,
}

/// An operator that compares an attribute's value in an attribute selector.
#[derive(Clone, Copy, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum AttributeMatcherOperator {
    /// `=`: the value is exactly the given text.
    #[serde(rename = "=")]
    Exact,

    /// `~=`: the value is a space-separated list that contains the given word.
    #[serde(rename = "~=")]
    Includes,

    /// `|=`: the value is the given text, or starts with the given text followed by `-`.
    #[serde(rename = "|=")]
    DashMatch,

    /// `^=`: the value starts with the given text.
    #[serde(rename = "^=")]
    Prefix,

    /// `$=`: the value ends with the given text.
    #[serde(rename = "$=")]
    Suffix,

    /// `*=`: the value contains the given text.
    #[serde(rename = "*=")]
    Substring,
}
