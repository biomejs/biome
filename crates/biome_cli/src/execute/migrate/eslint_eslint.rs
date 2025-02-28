use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableTypes, DeserializableValue,
    DeserializationContext, DeserializationDiagnostic, DeserializationVisitor, Merge,
};
use biome_deserialize_macros::Deserializable;
use biome_rowan::TextRange;
use rustc_hash::FxHashMap;
use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::ops::DerefMut;
use std::vec;
use std::{any::TypeId, marker::PhantomData, ops::Deref};

use super::{eslint_jsxa11y, eslint_typescript, eslint_unicorn, ignorefile};

/// This modules includes implementations for deserializing an eslint configuration.
///
/// The defined types follow the ESLint configuration schema described at
/// <https://github.com/eslint/eslint/blob/ce838adc3b673e52a151f36da0eedf5876977514/lib/shared/types.js>.
///
/// See [super::eslint_to_biome] for converting an ESLint config to a Biome config.

#[derive(Debug)]
pub(crate) enum AnyConfigData {
    Flat(FlatConfigData),
    Legacy(LegacyConfigData),
}
impl From<FlatConfigData> for AnyConfigData {
    fn from(value: FlatConfigData) -> Self {
        AnyConfigData::Flat(value)
    }
}
impl From<LegacyConfigData> for AnyConfigData {
    fn from(value: LegacyConfigData) -> Self {
        AnyConfigData::Legacy(value)
    }
}

#[derive(Debug, Default, Deserializable)]
pub(crate) struct FlatConfigData(pub(crate) Vec<FlatConfigObject>);

#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct FlatConfigObject {
    pub(crate) files: Vec<Box<str>>,
    /// The glob patterns that ignore to lint.
    pub(crate) ignores: Vec<Box<str>>,
    // using `Option` is important to distinguish a global ignores from a config objerct
    pub(crate) language_options: Option<FlatLanguageOptions>,
    // using `Option` is important to distinguish a global ignores from a config objerct
    pub(crate) rules: Option<Rules>,
}
impl FlatConfigObject {
    /// Rteurns `true` if this config contains only `ignores`.
    ///
    /// See https://eslint.org/docs/latest/use/configure/configuration-files-new#globally-ignoring-files-with-ignores
    pub(crate) fn is_global_ignores(&self) -> bool {
        !self.ignores.is_empty()
            && self.files.is_empty()
            && self.language_options.is_none()
            && self.rules.is_none()
    }

    /// Rteurns `true` if this config doesn't specify `files` or `ignores`.
    pub(crate) fn is_global_config(&self) -> bool {
        self.ignores.is_empty() && self.files.is_empty()
    }
}
impl Merge for FlatConfigObject {
    fn merge_with(&mut self, other: Self) {
        self.files.extend(other.files);
        self.ignores.extend(other.ignores);
        self.language_options.merge_with(other.language_options);
        self.rules.merge_with(other.rules);
    }
}

#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct FlatLanguageOptions {
    pub(crate) globals: Globals,
}
impl Merge for FlatLanguageOptions {
    fn merge_with(&mut self, other: Self) {
        self.globals.merge_with(other.globals);
    }
}

#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct EslintPackageJson {
    pub(crate) eslint_config: Option<LegacyConfigData>,
    pub(crate) eslint_ignore: Vec<IgnorePattern>,
}

#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct LegacyConfigData {
    pub(crate) extends: ShorthandVec<String>,
    pub(crate) globals: Globals,
    /// The glob patterns that ignore to lint.
    pub(crate) ignore_patterns: ShorthandVec<IgnorePattern>,
    /// The parser options.
    pub(crate) rules: Rules,
    pub(crate) overrides: Vec<OverrideConfigData>,
}
impl Merge for LegacyConfigData {
    fn merge_with(&mut self, mut other: Self) {
        self.extends.merge_with(other.extends);
        self.globals.merge_with(other.globals);
        self.ignore_patterns.merge_with(other.ignore_patterns);
        self.rules.merge_with(other.rules);
        self.overrides.append(&mut other.overrides);
    }
}

#[derive(Debug, Default)]
pub(crate) struct IgnorePattern(pub(crate) Box<str>);
impl Deref for IgnorePattern {
    type Target = Box<str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<str> for IgnorePattern {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
impl biome_deserialize::Deserializable for IgnorePattern {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let s = biome_deserialize::Text::deserialize(ctx, value, name)?;
        Some(Self(ignorefile::convert_pattern(s.text()).into_boxed_str()))
    }
}

//? ESLint plugins export metadata in their main export.
/// This includes presets in the `configs` field.
#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct PluginExport {
    pub(crate) configs: FxHashMap<String, LegacyConfigData>,
}

#[derive(Debug, Default, Deserializable)]
pub(crate) struct Globals(pub(crate) FxHashMap<Box<str>, GlobalConf>);
impl Globals {
    pub(crate) fn enabled(self) -> impl Iterator<Item = Box<str>> {
        self.0.into_iter().filter_map(|(global_name, global_conf)| {
            global_conf.is_enabled().then_some(global_name)
        })
    }
}
impl Deref for Globals {
    type Target = FxHashMap<Box<str>, GlobalConf>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Merge for Globals {
    fn merge_with(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

#[derive(Debug)]
pub(crate) enum GlobalConf {
    Flag(bool),
    Qualifier(GlobalConfQualifier),
}
impl GlobalConf {
    pub(crate) fn is_enabled(&self) -> bool {
        match self {
            GlobalConf::Flag(result) => *result,
            GlobalConf::Qualifier(qualifier) => !matches!(qualifier, GlobalConfQualifier::Off),
        }
    }
}
impl Deserializable for GlobalConf {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            Deserializable::deserialize(ctx, value, name).map(Self::Qualifier)
        } else {
            Deserializable::deserialize(ctx, value, name).map(Self::Flag)
        }
    }
}

#[derive(Debug, Deserializable)]
pub(crate) enum GlobalConfQualifier {
    Off,
    Readable,
    Readonly,
    Writable,
    Writeable,
}

#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct OverrideConfigData {
    pub(crate) extends: ShorthandVec<Box<str>>,
    pub(crate) globals: Globals,
    /// The glob patterns for excluded files.
    pub(crate) excluded_files: ShorthandVec<Box<str>>,
    /// The glob patterns for target files.
    pub(crate) files: ShorthandVec<Box<str>>,
    pub(crate) rules: Rules,
}

#[derive(Debug, Default)]
pub(crate) struct ShorthandVec<T>(Vec<T>);
impl<T> Merge for ShorthandVec<T> {
    fn merge_with(&mut self, mut other: Self) {
        self.0.append(&mut other.0);
    }
}
impl<T> From<T> for ShorthandVec<T> {
    fn from(value: T) -> Self {
        Self(vec![value])
    }
}
impl<T> Deref for ShorthandVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for ShorthandVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T> IntoIterator for ShorthandVec<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<T: Deserializable> Deserializable for ShorthandVec<T> {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        Some(ShorthandVec(
            if value.visitable_type()? == DeserializableType::Array {
                Deserializable::deserialize(ctx, value, name)?
            } else {
                Vec::from_iter([Deserializable::deserialize(ctx, value, name)?])
            },
        ))
    }
}

/// Model the possible shapes of an ESLint's rule configuration
#[derive(Debug, Clone)]
pub(crate) enum RuleConf<T = (), U = ()> {
    // `{ rule: <severity> }` and `{ rule: [<severity>] }`
    Severity(Severity),
    // `{ rule: <severity> }` and `{ rule: [<severity>, <option1>] }`
    Option(Severity, T),
    // `{ rule: <severity> }` and `{ rule: [<severity>, <option1>, <option2>] }`
    Options(Severity, T, U),
    // `{ rule: <severity> }` and `{ rule: [<severity>, <option1.1>, <option1.2>, ...] }`
    Spread(Severity, Vec<T>),
}
impl<T, U> RuleConf<T, U> {
    pub(crate) fn severity(&self) -> Severity {
        match self {
            Self::Severity(severity) => *severity,
            Self::Option(severity, _) => *severity,
            Self::Options(severity, _, _) => *severity,
            Self::Spread(severity, _) => *severity,
        }
    }
}
impl<T> RuleConf<T, ()> {
    pub(crate) fn into_vec(self) -> Vec<T> {
        match self {
            RuleConf::Severity(_) => vec![],
            RuleConf::Option(_, value) | RuleConf::Options(_, value, _) => vec![value],
            RuleConf::Spread(_, result) => result,
        }
    }
}
impl<T: Default, U: Default> RuleConf<T, U> {
    pub(crate) fn option_or_default(self) -> T {
        match self {
            RuleConf::Severity(_) | RuleConf::Options(_, _, _) | RuleConf::Spread(_, _) => {
                T::default()
            }
            RuleConf::Option(_, option) => option,
        }
    }
}
impl<T: Deserializable + 'static, U: Deserializable + 'static> Deserializable for RuleConf<T, U> {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor<T, U>(PhantomData<(T, U)>);
        impl<T: Deserializable + 'static, U: Deserializable + 'static> DeserializationVisitor
            for Visitor<T, U>
        {
            type Output = RuleConf<T, U>;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::ARRAY;
            fn visit_array(
                self,
                ctx: &mut impl DeserializationContext,
                values: impl Iterator<Item = Option<impl DeserializableValue>>,
                range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                let mut values = values.flatten();
                let Some(first_value) = values.next() else {
                    ctx.report(
                        DeserializationDiagnostic::new("A severity is expected.").with_range(range),
                    );
                    return None;
                };
                let severity = Deserializable::deserialize(ctx, &first_value, "")?;
                if TypeId::of::<T>() == TypeId::of::<()>() {
                    return Some(RuleConf::Severity(severity));
                }
                let Some(second_value) = values.next() else {
                    return Some(RuleConf::Severity(severity));
                };
                let Some(option) = T::deserialize(ctx, &second_value, "") else {
                    // Recover by ignoring the failed deserialization
                    return Some(RuleConf::Severity(severity));
                };
                let Some(third_value) = values.next() else {
                    return Some(RuleConf::Option(severity, option));
                };
                if TypeId::of::<U>() != TypeId::of::<()>() {
                    if let Some(option2) = U::deserialize(ctx, &third_value, "") {
                        return Some(RuleConf::Options(severity, option, option2));
                    } else {
                        // Recover by ignoring the failed deserialization
                        return Some(RuleConf::Option(severity, option));
                    }
                }
                let Some(option2) = T::deserialize(ctx, &third_value, "") else {
                    // Recover by ignoring the failed deserialization
                    return Some(RuleConf::Option(severity, option));
                };
                let mut spread = Vec::new();
                spread.push(option);
                spread.push(option2);
                spread.extend(values.filter_map(|val| T::deserialize(ctx, &val, "")));
                Some(RuleConf::Spread(severity, spread))
            }
        }
        if matches!(
            value.visitable_type()?,
            DeserializableType::Number | DeserializableType::Str
        ) {
            Deserializable::deserialize(ctx, value, name).map(RuleConf::Severity)
        } else {
            value.deserialize(ctx, Visitor(PhantomData), name)
        }
    }
}

#[derive(Clone, Copy, Debug, Deserializable)]
#[deserializable(try_from = "NumberOrString")]
pub(crate) enum Severity {
    Off,
    Warn,
    Error,
}
impl TryFrom<NumberOrString> for Severity {
    type Error = &'static str;

    fn try_from(value: NumberOrString) -> Result<Self, &'static str> {
        match value {
            NumberOrString::Number(n) => match n {
                0 => Ok(Severity::Off),
                1 => Ok(Severity::Warn),
                2 => Ok(Severity::Error),
                _ => Err("Severity should be 0, 1 or 2."),
            },
            NumberOrString::String(s) => match s.as_ref() {
                "off" => Ok(Severity::Off),
                "warn" => Ok(Severity::Warn),
                "error" => Ok(Severity::Error),
                _ => Err("Severity should be 'off', 'warn' or 'error'."),
            },
        }
    }
}
impl From<Severity> for biome_configuration::RulePlainConfiguration {
    fn from(value: Severity) -> biome_configuration::RulePlainConfiguration {
        match value {
            Severity::Off => biome_configuration::RulePlainConfiguration::Off,
            Severity::Warn => biome_configuration::RulePlainConfiguration::Warn,
            Severity::Error => biome_configuration::RulePlainConfiguration::Error,
        }
    }
}
#[derive(Debug, Clone)]
enum NumberOrString {
    Number(u64),
    String(String),
}
impl Deserializable for NumberOrString {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        Some(if value.visitable_type()? == DeserializableType::Str {
            Self::String(Deserializable::deserialize(ctx, value, name)?)
        } else {
            Self::Number(Deserializable::deserialize(ctx, value, name)?)
        })
    }
}

#[derive(Debug, Default)]
pub(crate) struct Rules(pub(crate) rustc_hash::FxHashSet<Rule>);
impl Merge for Rules {
    fn merge_with(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}
impl Deref for Rules {
    type Target = rustc_hash::FxHashSet<Rule>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deserializable for Rules {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Rules;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;
            fn visit_map(
                self,
                ctx: &mut impl DeserializationContext,
                members: impl Iterator<
                    Item = Option<(
                        impl biome_deserialize::DeserializableValue,
                        impl biome_deserialize::DeserializableValue,
                    )>,
                >,
                _range: biome_rowan::TextRange,
                name: &str,
            ) -> Option<Self::Output> {
                use biome_deserialize::Text;
                let mut result = rustc_hash::FxHashSet::default();
                for (key, value) in members.flatten() {
                    let Some(rule_name) = Text::deserialize(ctx, &key, "") else {
                        continue;
                    };
                    match rule_name.text() {
                        // Eslint rules with options that we handle
                        "no-console" => {
                            if let Some(conf) = RuleConf::deserialize(ctx, &value, name) {
                                result.insert(Rule::NoConsole(conf));
                            }
                        }
                        "no-restricted-globals" => {
                            if let Some(conf) = RuleConf::deserialize(ctx, &value, name) {
                                result.insert(Rule::NoRestrictedGlobals(conf));
                            }
                        }
                        // Eslint plugin rules with options that we handle
                        "jsx-a11y/aria-role" => {
                            if let Some(conf) = RuleConf::deserialize(ctx, &value, name) {
                                result.insert(Rule::Jsxa11yArioaRoles(conf));
                            }
                        }
                        "@typescript-eslint/array-type" => {
                            if let Some(conf) = RuleConf::deserialize(ctx, &value, name) {
                                result.insert(Rule::TypeScriptArrayType(conf));
                            }
                        }
                        "@typescript-eslint/explicit-member-accessibility" => {
                            if let Some(conf) = RuleConf::deserialize(ctx, &value, name) {
                                result.insert(Rule::TypeScriptExplicitMemberAccessibility(conf));
                            }
                        }
                        "@typescript-eslint/naming-convention" => {
                            if let Some(conf) = RuleConf::deserialize(ctx, &value, name) {
                                result.insert(Rule::TypeScriptNamingConvention(conf));
                            }
                        }
                        "unicorn/filename-case" => {
                            if let Some(conf) = RuleConf::deserialize(ctx, &value, name) {
                                result.insert(Rule::UnicornFilenameCase(conf));
                            }
                        }
                        // Other rules
                        rule_name => {
                            if let Some(conf) = RuleConf::<()>::deserialize(ctx, &value, name) {
                                result.insert(Rule::Any(
                                    Cow::Owned(rule_name.to_string()),
                                    conf.severity(),
                                ));
                            }
                        }
                    }
                }
                Some(Rules(result))
            }
        }
        value.deserialize(ctx, Visitor, name)
    }
}

#[derive(Debug, Default, Deserializable)]
pub struct NoConsoleOptions {
    /// Allowed calls on the console object.
    pub allow: Box<[Box<str>]>,
}
impl From<NoConsoleOptions> for biome_js_analyze::lint::suspicious::no_console::NoConsoleOptions {
    fn from(val: NoConsoleOptions) -> Self {
        biome_js_analyze::lint::suspicious::no_console::NoConsoleOptions { allow: val.allow }
    }
}

#[derive(Debug)]
pub(crate) enum NoRestrictedGlobal {
    Plain(String),
    WithMessage(GlobalWithMessage),
}
impl NoRestrictedGlobal {
    pub(crate) fn into_name(self) -> String {
        match self {
            NoRestrictedGlobal::Plain(name) => name,
            NoRestrictedGlobal::WithMessage(named) => named.name,
        }
    }
}
impl Deserializable for NoRestrictedGlobal {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            Deserializable::deserialize(ctx, value, name).map(NoRestrictedGlobal::Plain)
        } else {
            Deserializable::deserialize(ctx, value, name).map(NoRestrictedGlobal::WithMessage)
        }
    }
}
#[derive(Debug, Default, Deserializable)]
pub(crate) struct GlobalWithMessage {
    name: String,
    message: String,
}

#[derive(Debug)]
pub(crate) enum Rule {
    /// Any rule without its options.
    Any(Cow<'static, str>, Severity),
    // Eslint rules with its options
    // We use this to configure equivalent Bione's rules.
    NoConsole(RuleConf<Box<NoConsoleOptions>>),
    NoRestrictedGlobals(RuleConf<Box<NoRestrictedGlobal>>),
    // Eslint plugins
    Jsxa11yArioaRoles(RuleConf<Box<eslint_jsxa11y::AriaRoleOptions>>),
    TypeScriptArrayType(RuleConf<eslint_typescript::ArrayTypeOptions>),
    TypeScriptExplicitMemberAccessibility(
        RuleConf<eslint_typescript::ExplicitMemberAccessibilityOptions>,
    ),
    TypeScriptNamingConvention(RuleConf<Box<eslint_typescript::NamingConventionSelection>>),
    UnicornFilenameCase(RuleConf<eslint_unicorn::FilenameCaseOptions>),
    // If you add new variants, don't forget to update [Rules::deserialize].
}
impl Rule {
    pub(crate) fn name(&self) -> Cow<'static, str> {
        match self {
            Rule::Any(name, _) => name.clone(),
            Rule::NoConsole(_) => Cow::Borrowed("no-console"),
            Rule::NoRestrictedGlobals(_) => Cow::Borrowed("no-restricted-globals"),
            Rule::Jsxa11yArioaRoles(_) => Cow::Borrowed("jsx-a11y/aria-role"),
            Rule::TypeScriptArrayType(_) => Cow::Borrowed("@typescript-eslint/array-type"),
            Rule::TypeScriptExplicitMemberAccessibility(_) => {
                Cow::Borrowed("@typescript-eslint/explicit-member-accessibility")
            }
            Rule::TypeScriptNamingConvention(_) => {
                Cow::Borrowed("@typescript-eslint/naming-convention")
            }
            Rule::UnicornFilenameCase(_) => Cow::Borrowed("unicorn/filename-case"),
        }
    }
}
impl Eq for Rule {}
impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}
