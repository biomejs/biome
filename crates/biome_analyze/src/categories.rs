use enumflags2::{BitFlags, bitflags};
use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum RuleCategory {
    /// This rule checks the syntax according to the language specification
    /// and emits error diagnostics accordingly
    Syntax,
    /// This rule performs static analysis of the source code to detect
    /// invalid or error-prone patterns, and emits diagnostics along with
    /// proposed fixes
    Lint,
    /// This rule detects refactoring opportunities and emits code action
    /// signals
    Action,
    /// This rule detects transformations that should be applied to the code
    Transformation,
}

impl RuleCategory {
    /// Returns a `str` that should be used for suppression comments
    pub const fn as_suppression_category(&self) -> &'static str {
        match self {
            Self::Syntax => "syntax",
            Self::Lint => "lint",
            Self::Action => "assist",
            Self::Transformation => "transformation",
        }
    }
}

impl FromStr for RuleCategory {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "syntax" => Ok(Self::Syntax),
            "lint" => Ok(Self::Lint),
            "action" => Ok(Self::Action),
            "transformation" => Ok(Self::Transformation),
            _ => Err("Invalid rule category"),
        }
    }
}

impl Display for RuleCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Syntax => write!(f, "Syntax"),
            Self::Lint => write!(f, "Lint"),
            Self::Action => write!(f, "Action"),
            Self::Transformation => write!(f, "Transformation"),
        }
    }
}

/// Actions that suppress rules should start with this string
pub const SUPPRESSION_INLINE_ACTION_CATEGORY: &str = "quickfix.suppressRule.inline.biome";
pub const SUPPRESSION_TOP_LEVEL_ACTION_CATEGORY: &str = "quickfix.suppressRule.topLevel.biome";

/// The category of a code action, this type maps directly to the
/// [CodeActionKind] type in the Language Server Protocol specification
///
/// [CodeActionKind]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#codeActionKind
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ActionCategory {
    /// Base kind for quickfix actions: 'quickfix'.
    ///
    /// This action provides a fix to the diagnostic emitted by the same signal
    QuickFix(Cow<'static, str>),
    /// Base kind for refactoring actions: 'refactor'.
    ///
    /// This action provides an optional refactor opportunity
    Refactor(RefactorKind),
    /// Base kind for source actions: `source`.
    ///
    /// Source code actions apply to the entire file.
    Source(SourceActionKind),
    /// This action is using a base kind not covered by any of the previous
    /// variants
    Other(OtherActionCategory),
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum OtherActionCategory {
    /// Base kind for inline suppressions actions: `quickfix.suppressRule.inline.biome`
    InlineSuppression,
    /// Base kind for inline suppressions actions: `quickfix.suppressRule.topLevel.biome`
    ToplevelSuppression,
    /// Generic action that can't be mapped
    Generic(Cow<'static, str>),
}

impl ActionCategory {
    /// Returns true if this category matches the provided filter
    ///
    /// ## Examples
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use biome_analyze::{ActionCategory, RefactorKind, OtherActionCategory};
    ///
    /// assert!(ActionCategory::QuickFix(Cow::from("quickfix")).matches("quickfix"));
    ///
    /// assert!(ActionCategory::Refactor(RefactorKind::None).matches("refactor"));
    /// assert!(!ActionCategory::Refactor(RefactorKind::None).matches("refactor.extract"));
    ///
    /// assert!(ActionCategory::Refactor(RefactorKind::Extract).matches("refactor"));
    /// assert!(ActionCategory::Refactor(RefactorKind::Extract).matches("refactor.extract"));
    ///
    /// assert!(ActionCategory::Other(OtherActionCategory::InlineSuppression).matches("quickfix.suppressRule.inline.biome"));
    /// assert!(ActionCategory::Other(OtherActionCategory::ToplevelSuppression).matches("quickfix.suppressRule.topLevel.biome"));
    /// ```
    pub fn matches(&self, filter: &str) -> bool {
        self.to_str().starts_with(filter)
    }

    /// Returns the representation of this [ActionCategory] as a `CodeActionKind` string
    pub fn to_str(&self) -> Cow<'static, str> {
        match self {
            Self::QuickFix(tag) => {
                if tag.is_empty() {
                    Cow::Borrowed("quickfix.biome")
                } else {
                    Cow::Owned(format!("quickfix.biome.{tag}"))
                }
            }

            Self::Refactor(RefactorKind::None) => Cow::Borrowed("refactor.biome"),
            Self::Refactor(RefactorKind::Extract) => Cow::Borrowed("refactor.extract.biome"),
            Self::Refactor(RefactorKind::Inline) => Cow::Borrowed("refactor.inline.biome"),
            Self::Refactor(RefactorKind::Rewrite) => Cow::Borrowed("refactor.rewrite.biome"),
            Self::Refactor(RefactorKind::Other(tag)) => Cow::Owned(format!("refactor.{tag}.biome")),

            Self::Source(SourceActionKind::None) => Cow::Borrowed("source.biome"),
            Self::Source(SourceActionKind::FixAll) => Cow::Borrowed("source.fixAll.biome"),
            Self::Source(SourceActionKind::OrganizeImports) => {
                Cow::Borrowed("source.organizeImports.biome")
            }
            Self::Source(SourceActionKind::Other(tag)) => Cow::Owned(format!("source.biome.{tag}")),

            Self::Other(other_action) => match other_action {
                OtherActionCategory::InlineSuppression => {
                    Cow::Borrowed("quickfix.suppressRule.inline.biome")
                }
                OtherActionCategory::ToplevelSuppression => {
                    Cow::Borrowed("quickfix.suppressRule.topLevel.biome")
                }
                OtherActionCategory::Generic(tag) => Cow::Owned(format!("{tag}.biome")),
            },
        }
    }
}

/// The sub-category of a refactor code action.
///
/// [Check the LSP spec](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#codeActionKind) for more information:
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum RefactorKind {
    /// This action describes a refactor with no particular sub-category
    None,
    /// Base kind for refactoring extraction actions: 'refactor.extract'.
    ///
    /// Example extract actions:
    /// - Extract method
    /// - Extract function
    /// - Extract variable
    /// - Extract interface from class
    Extract,
    /// Base kind for refactoring inline actions: 'refactor.inline'.
    ///
    /// Example inline actions:
    /// - Inline function
    /// - Inline variable
    /// - Inline constant
    /// - ...
    Inline,
    /// Base kind for refactoring rewrite actions: 'refactor.rewrite'.
    ///
    /// Example rewrite actions:
    /// - Convert JavaScript function to class
    /// - Add or remove parameter
    /// - Encapsulate field
    /// - Make method static
    /// - Move method to base class
    /// - ...
    Rewrite,
    /// This action is using a refactor kind not covered by any of the previous
    /// variants
    Other(Cow<'static, str>),
}

/// The sub-category of a source code action
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum SourceActionKind {
    /// This action describes a source action with no particular sub-category
    None,
    // Base kind for a 'fix all' source action: `source.fixAll`.
    //
    // 'Fix all' actions automatically fix errors that have a clear fix that
    // do not require user input. They should not suppress errors or perform
    // unsafe fixes such as generating new types or classes.
    FixAll,
    /// Base kind for an organize imports source action: `source.organizeImports`.
    OrganizeImports,
    /// This action is using a source action kind not covered by any of the
    /// previous variants
    Other(Cow<'static, str>),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[bitflags]
#[repr(u8)]
pub(crate) enum Categories {
    Syntax = 1 << RuleCategory::Syntax as u8,
    Lint = 1 << RuleCategory::Lint as u8,
    Assist = 1 << RuleCategory::Action as u8,
    Transformation = 1 << RuleCategory::Transformation as u8,
}

#[derive(Copy, Clone)]
/// The categories supported by the analyzer.
///
/// The default implementation of this type returns an instance with all the categories.
///
/// Use [RuleCategoriesBuilder] to generate the categories you want to query.
pub struct RuleCategories(BitFlags<Categories>);

impl Display for RuleCategories {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Debug for RuleCategories {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            write!(f, "No categories")
        } else {
            let mut list = f.debug_list();
            if self.0.contains(Categories::Syntax) {
                list.entry(&RuleCategory::Syntax);
            }
            if self.0.contains(Categories::Lint) {
                list.entry(&RuleCategory::Lint);
            }
            if self.0.contains(Categories::Assist) {
                list.entry(&RuleCategory::Action);
            }
            list.finish()
        }
    }
}

impl RuleCategories {
    pub fn empty() -> Self {
        let empty: BitFlags<Categories> = BitFlags::empty();
        Self(empty)
    }

    pub fn all() -> Self {
        let empty: BitFlags<Categories> = BitFlags::all();
        Self(empty)
    }

    pub fn insert(&mut self, other: impl Into<Self>) {
        self.0.insert(other.into().0);
    }

    pub fn remove(&mut self, other: impl Into<Self>) {
        self.0.remove(other.into().0);
    }

    /// Checks whether the current categories contain a specific [RuleCategories]
    pub fn contains(&self, other: impl Into<Self>) -> bool {
        self.0.contains(other.into().0)
    }

    /// Checks if `category` matches the current categories
    pub fn matches(&self, category: &str) -> bool {
        if let Ok(category) = Self::from_str(category) {
            self.contains(category)
        } else {
            false
        }
    }
}

impl Default for RuleCategories {
    fn default() -> Self {
        Self::all()
    }
}

impl RuleCategories {
    pub fn is_syntax(&self) -> bool {
        self.0.contains(Categories::Syntax)
    }

    pub fn is_lint(&self) -> bool {
        self.0.contains(Categories::Lint)
    }

    pub fn is_assist(&self) -> bool {
        self.0.contains(Categories::Assist)
    }
}

impl From<RuleCategory> for RuleCategories {
    fn from(input: RuleCategory) -> Self {
        match input {
            RuleCategory::Syntax => Self(BitFlags::from_flag(Categories::Syntax)),
            RuleCategory::Lint => Self(BitFlags::from_flag(Categories::Lint)),
            RuleCategory::Action => Self(BitFlags::from_flag(Categories::Assist)),
            RuleCategory::Transformation => Self(BitFlags::from_flag(Categories::Transformation)),
        }
    }
}

impl From<&RuleCategory> for RuleCategories {
    fn from(input: &RuleCategory) -> Self {
        match input {
            RuleCategory::Syntax => Self(BitFlags::from_flag(Categories::Syntax)),
            RuleCategory::Lint => Self(BitFlags::from_flag(Categories::Lint)),
            RuleCategory::Action => Self(BitFlags::from_flag(Categories::Assist)),
            RuleCategory::Transformation => Self(BitFlags::from_flag(Categories::Transformation)),
        }
    }
}

impl FromStr for RuleCategories {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "syntax" => Ok(Self(BitFlags::from_flag(Categories::Syntax))),
            "lint" => Ok(Self(BitFlags::from_flag(Categories::Lint))),
            "action" => Ok(Self(BitFlags::from_flag(Categories::Assist))),
            "transformation" => Ok(Self(BitFlags::from_flag(Categories::Transformation))),
            _ => Err("Invalid rule category"),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for RuleCategories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut flags = Vec::new();

        if self.0.contains(Categories::Syntax) {
            flags.push(RuleCategory::Syntax);
        }

        if self.0.contains(Categories::Lint) {
            flags.push(RuleCategory::Lint);
        }

        if self.0.contains(Categories::Assist) {
            flags.push(RuleCategory::Action);
        }

        if self.0.contains(Categories::Transformation) {
            flags.push(RuleCategory::Transformation);
        }

        serializer.collect_seq(flags)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RuleCategories {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, SeqAccess};
        use std::fmt::{self, Formatter};

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = RuleCategories;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                write!(formatter, "RuleCategories")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut result = RuleCategories::empty();

                while let Some(item) = seq.next_element::<RuleCategory>()? {
                    result.0 |= RuleCategories::from(item).0;
                }

                Ok(result)
            }
        }

        deserializer.deserialize_seq(Visitor)
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for RuleCategories {
    fn schema_name() -> String {
        String::from("RuleCategories")
    }

    fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        <Vec<RuleCategory>>::json_schema(generator)
    }
}

#[derive(Debug, Default)]
/// A convenient type create a [RuleCategories] type
///
/// ```
/// use biome_analyze::{RuleCategoriesBuilder, RuleCategory};
/// let mut categories = RuleCategoriesBuilder::default().with_syntax().with_lint().build();
///
/// assert!(categories.contains(RuleCategory::Lint));
/// assert!(categories.contains(RuleCategory::Syntax));
/// assert!(!categories.contains(RuleCategory::Action));
/// assert!(!categories.contains(RuleCategory::Transformation));
/// ```
pub struct RuleCategoriesBuilder {
    flags: BitFlags<Categories>,
}

impl RuleCategoriesBuilder {
    pub fn with_syntax(mut self) -> Self {
        self.flags.insert(Categories::Syntax);
        self
    }

    pub fn with_lint(mut self) -> Self {
        self.flags.insert(Categories::Lint);
        self
    }

    pub fn with_assist(mut self) -> Self {
        self.flags.insert(Categories::Assist);
        self
    }

    pub fn with_transformation(mut self) -> Self {
        self.flags.insert(Categories::Transformation);
        self
    }

    pub fn build(self) -> RuleCategories {
        RuleCategories(self.flags)
    }
}
