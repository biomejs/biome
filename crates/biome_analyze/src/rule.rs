use crate::categories::{ActionCategory, RuleCategory};
use crate::context::RuleContext;
use crate::registry::{RegistryVisitor, RuleLanguage, RuleSuppressions};
use crate::{
    Phase, Phases, Queryable, SourceActionKind, SuppressionAction, SuppressionCommentEmitterPayload,
};
use biome_console::fmt::{Display, Formatter};
use biome_console::{markup, MarkupBuf, Padding};
use biome_diagnostics::advice::CodeSuggestionAdvice;
use biome_diagnostics::location::AsSpan;
use biome_diagnostics::{
    Advices, Category, Diagnostic, DiagnosticTags, Location, LogCategory, MessageAndDescription,
    Visit,
};
use biome_diagnostics::{Applicability, Severity};
use biome_rowan::{AstNode, BatchMutation, BatchMutationExt, Language, TextRange};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
/// Static metadata containing information about a rule
pub struct RuleMetadata {
    /// It marks if a rule is deprecated, and if so a reason has to be provided.
    pub deprecated: Option<&'static str>,
    /// The version when the rule was implemented
    pub version: &'static str,
    /// The name of this rule, displayed in the diagnostics it emits
    pub name: &'static str,
    /// The content of the documentation comments for this rule
    pub docs: &'static str,
    /// The language that the rule applies to.
    pub language: &'static str,
    /// Whether a rule is recommended or not
    pub recommended: bool,
    /// The kind of fix
    pub fix_kind: FixKind,
    /// The source URL of the rule
    pub sources: &'static [RuleSource],
    /// The source kind of the rule
    pub source_kind: Option<RuleSourceKind>,
    /// The default severity of the rule
    pub severity: Severity,
    /// Domains applied by this rule
    pub domains: &'static [RuleDomain],
}

impl biome_console::fmt::Display for RuleMetadata {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        fmt.write_markup(markup! {
            <Emphasis>"Summary"</Emphasis>
        })?;
        fmt.write_str("\n")?;
        fmt.write_str("\n")?;

        fmt.write_markup(markup! {
            "- Name: "<Emphasis>{self.name}</Emphasis>
        })?;
        fmt.write_str("\n")?;
        match self.fix_kind {
            FixKind::None => {
                fmt.write_markup(markup! {
                    "- No fix available."
                })?;
            }
            kind => {
                fmt.write_markup(markup! {
                    "- Fix: "<Emphasis>{kind}</Emphasis>
                })?;
            }
        }
        fmt.write_str("\n")?;

        fmt.write_markup(markup! {
            "- Default severity: "<Emphasis>{self.severity}</Emphasis>
        })?;
        fmt.write_str("\n")?;

        fmt.write_markup(markup! {
            "- Available from version: "<Emphasis>{self.version}</Emphasis>
        })?;
        fmt.write_str("\n")?;

        if self.domains.is_empty() && self.recommended {
            fmt.write_markup(markup! {
                "- This rule is not recommended"
            })?;
        }

        let domains = DisplayDomains(self.domains, self.recommended);

        fmt.write_str("\n")?;

        fmt.write_markup(markup!({ domains }))?;

        fmt.write_str("\n")?;

        fmt.write_markup(markup! {
            <Emphasis>"Description"</Emphasis>
        })?;
        fmt.write_str("\n")?;
        fmt.write_str("\n")?;

        for line in self.docs.lines() {
            if let Some((_, remainder)) = line.split_once("## ") {
                fmt.write_markup(markup! {
                    <Emphasis>{remainder.trim_start()}</Emphasis>
                })?;
            } else if let Some((_, remainder)) = line.split_once("### ") {
                fmt.write_markup(markup! {
                    <Emphasis>{remainder.trim_start()}</Emphasis>
                })?;
            } else {
                fmt.write_str(line)?;
            }

            fmt.write_str("\n")?;
        }

        Ok(())
    }
}

struct DisplayDomains(&'static [RuleDomain], bool);

impl Display for DisplayDomains {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        let domains = self.0;
        let recommended = self.1;

        if domains.is_empty() {
            return Ok(());
        }

        fmt.write_markup(markup!(
            <Emphasis>"Domains"</Emphasis>
        ))?;
        fmt.write_str("\n")?;
        fmt.write_str("\n")?;

        for domain in domains {
            let dependencies = domain.manifest_dependencies();

            fmt.write_markup(markup! {
                "- Name: "<Emphasis>{domain}</Emphasis>
            })?;
            fmt.write_str("\n")?;

            if recommended {
                fmt.write_markup(markup! {
                    "- The rule is recommended for this domain"
                })?;
                fmt.write_str("\n")?;
            }

            if !dependencies.is_empty() {
                fmt.write_markup(markup! {
                    "- The rule is enabled when one of these dependencies are detected:"
                })?;
                fmt.write_str("\n")?;
                let padding = Padding::new(2);
                for (index, (dep, range)) in dependencies.iter().enumerate() {
                    fmt.write_markup(
                        markup! { {padding}"- "<Emphasis>{dep}"@"{range}</Emphasis> },
                    )?;
                    if index + 1 < dependencies.len() {
                        fmt.write_str("\n")?;
                    }
                }
                fmt.write_str("\n")?;
            }

            let globals = domain.globals();

            if !globals.is_empty() {
                fmt.write_markup(markup! {
                    "- The rule adds the following globals: "
                })?;
                fmt.write_str("\n")?;

                let padding = Padding::new(2);
                for (index, global) in globals.iter().enumerate() {
                    fmt.write_markup(markup! { {padding}"- "<Emphasis>{global}</Emphasis> })?;
                    if index + 1 < globals.len() {
                        fmt.write_str("\n")?;
                    }
                }
                fmt.write_str("\n")?;
            }
            fmt.write_str("\n")?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(
        biome_deserialize_macros::Deserializable,
        serde::Deserialize,
        serde::Serialize
    )
)]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
/// Used to identify the kind of code action emitted by a rule
pub enum FixKind {
    /// The rule doesn't emit code actions.
    #[default]
    None,
    /// The rule emits a code action that is safe to apply. Usually these fixes don't change the semantic of the program.
    Safe,
    /// The rule emits a code action that is _unsafe_ to apply. Usually these fixes remove comments, or change
    /// the semantic of the program.
    Unsafe,
}

impl Display for FixKind {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        match self {
            FixKind::None => fmt.write_markup(markup!("none")),
            FixKind::Safe => fmt.write_markup(markup!(<Success>"safe"</Success>)),
            FixKind::Unsafe => fmt.write_markup(markup!(<Warn>"unsafe"</Warn>)),
        }
    }
}

impl TryFrom<FixKind> for Applicability {
    type Error = &'static str;
    fn try_from(value: FixKind) -> Result<Self, Self::Error> {
        match value {
            FixKind::None => Err("The fix kind is None"),
            FixKind::Safe => Ok(Applicability::Always),
            FixKind::Unsafe => Ok(Applicability::MaybeIncorrect),
        }
    }
}

#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum RuleSource {
    /// Rules from [Rust Clippy](https://rust-lang.github.io/rust-clippy/master/index.html)
    Clippy(&'static str),
    /// Rules from [Eslint](https://eslint.org/)
    Eslint(&'static str),
    /// Rules from [GraphQL-ESLint](https://github.com/dimaMachina/graphql-eslint)
    EslintGraphql(&'static str),
    /// Rules from [graphql-schema-linter](https://github.com/cjoudrey/graphql-schema-linter)
    EslintGraphqlSchemaLinter(&'static str),
    /// Rules from [Eslint Plugin Import](https://github.com/import-js/eslint-plugin-import)
    EslintImport(&'static str),
    /// Rules from [Eslint Plugin Import Access](https://github.com/uhyo/eslint-plugin-import-access)
    EslintImportAccess(&'static str),
    /// Rules from [Eslint Plugin Jest](https://github.com/jest-community/eslint-plugin-jest)
    EslintJest(&'static str),
    /// Rules from [Eslint Plugin JSX A11y](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y)
    EslintJsxA11y(&'static str),
    /// Rules from [Eslint Plugin React](https://github.com/jsx-eslint/eslint-plugin-react)
    EslintReact(&'static str),
    /// Rules from [Eslint Plugin React Hooks](https://github.com/facebook/react/blob/main/packages/eslint-plugin-react-hooks/README.md)
    EslintReactHooks(&'static str),
    /// Rules from [Eslint Plugin React Refresh](https://github.com/ArnaudBarre/eslint-plugin-react-refresh)
    EslintReactRefresh(&'static str),
    /// Rules from [Eslint Plugin Solid](https://github.com/solidjs-community/eslint-plugin-solid)
    EslintSolid(&'static str),
    /// Rules from [Eslint Plugin Sonar](https://github.com/SonarSource/eslint-plugin-sonarjs)
    EslintSonarJs(&'static str),
    /// Rules from [Eslint Plugin Stylistic](https://eslint.style)
    EslintStylistic(&'static str),
    /// Rules from [Eslint Plugin Typescript](https://typescript-eslint.io)
    EslintTypeScript(&'static str),
    /// Rules from [Eslint Plugin Unicorn](https://github.com/sindresorhus/eslint-plugin-unicorn)
    EslintUnicorn(&'static str),
    /// Rules from [Eslint Plugin Unused Imports](https://github.com/sweepline/eslint-plugin-unused-imports)
    EslintUnusedImports(&'static str),
    /// Rules from [Eslint Plugin Mysticatea](https://github.com/mysticatea/eslint-plugin)
    EslintMysticatea(&'static str),
    /// Rules from [Eslint Plugin Barrel Files](https://github.com/thepassle/eslint-plugin-barrel-files)
    EslintBarrelFiles(&'static str),
    /// Rules from [Eslint Plugin N](https://github.com/eslint-community/eslint-plugin-n)
    EslintN(&'static str),
    /// Rules from [Eslint Plugin Next](https://github.com/vercel/next.js/tree/canary/packages/eslint-plugin-next)
    EslintNext(&'static str),
    /// Rules from [Stylelint](https://github.com/stylelint/stylelint)
    Stylelint(&'static str),
    /// Rules from [Eslint Plugin No Secrets](https://github.com/nickdeis/eslint-plugin-no-secrets)
    EslintNoSecrets(&'static str),
    /// Rules from [deno lint](https://github.com/denoland/deno_lint)
    DenoLint(&'static str),
}

impl PartialEq for RuleSource {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl std::fmt::Display for RuleSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clippy(_) => write!(f, "Clippy"),
            Self::Eslint(_) => write!(f, "ESLint"),
            Self::EslintGraphql(_) => write!(f, "GraphQL-ESLint"),
            Self::EslintGraphqlSchemaLinter(_) => write!(f, "graphql-schema-linter"),
            Self::EslintImport(_) => write!(f, "eslint-plugin-import"),
            Self::EslintImportAccess(_) => write!(f, "eslint-plugin-import-access"),
            Self::EslintJest(_) => write!(f, "eslint-plugin-jest"),
            Self::EslintJsxA11y(_) => write!(f, "eslint-plugin-jsx-a11y"),
            Self::EslintReact(_) => write!(f, "eslint-plugin-react"),
            Self::EslintReactHooks(_) => write!(f, "eslint-plugin-react-hooks"),
            Self::EslintReactRefresh(_) => write!(f, "eslint-plugin-react-refresh"),
            Self::EslintSolid(_) => write!(f, "eslint-plugin-solid"),
            Self::EslintSonarJs(_) => write!(f, "eslint-plugin-sonarjs"),
            Self::EslintStylistic(_) => write!(f, "eslint-plugin-stylistic"),
            Self::EslintTypeScript(_) => write!(f, "typescript-eslint"),
            Self::EslintUnicorn(_) => write!(f, "eslint-plugin-unicorn"),
            Self::EslintUnusedImports(_) => write!(f, "eslint-plugin-unused-imports"),
            Self::EslintMysticatea(_) => write!(f, "@mysticatea/eslint-plugin"),
            Self::EslintBarrelFiles(_) => write!(f, "eslint-plugin-barrel-files"),
            Self::EslintN(_) => write!(f, "eslint-plugin-n"),
            Self::EslintNext(_) => write!(f, "@next/eslint-plugin-next"),
            Self::Stylelint(_) => write!(f, "Stylelint"),
            Self::EslintNoSecrets(_) => write!(f, "eslint-plugin-no-secrets"),
            Self::DenoLint(_) => write!(f, "deno-lint"),
        }
    }
}

impl PartialOrd for RuleSource {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RuleSource {
    fn cmp(&self, other: &Self) -> Ordering {
        if let (RuleSource::Eslint(self_rule), RuleSource::Eslint(other_rule)) = (self, other) {
            self_rule.cmp(other_rule)
        } else if self.is_eslint() {
            Ordering::Greater
        } else if other.is_eslint() {
            Ordering::Less
        } else {
            let self_rule = self.as_rule_name();
            let other_rule = other.as_rule_name();
            self_rule.cmp(other_rule)
        }
    }
}

impl RuleSource {
    pub fn as_rule_name(&self) -> &'static str {
        match self {
            Self::Clippy(rule_name)
            | Self::Eslint(rule_name)
            | Self::EslintGraphql(rule_name)
            | Self::EslintGraphqlSchemaLinter(rule_name)
            | Self::EslintImport(rule_name)
            | Self::EslintImportAccess(rule_name)
            | Self::EslintJest(rule_name)
            | Self::EslintJsxA11y(rule_name)
            | Self::EslintReact(rule_name)
            | Self::EslintReactHooks(rule_name)
            | Self::EslintReactRefresh(rule_name)
            | Self::EslintTypeScript(rule_name)
            | Self::EslintSolid(rule_name)
            | Self::EslintSonarJs(rule_name)
            | Self::EslintStylistic(rule_name)
            | Self::EslintUnicorn(rule_name)
            | Self::EslintUnusedImports(rule_name)
            | Self::EslintMysticatea(rule_name)
            | Self::EslintBarrelFiles(rule_name)
            | Self::EslintN(rule_name)
            | Self::EslintNext(rule_name)
            | Self::EslintNoSecrets(rule_name)
            | Self::Stylelint(rule_name)
            | Self::DenoLint(rule_name) => rule_name,
        }
    }

    pub fn to_namespaced_rule_name(&self) -> String {
        match self {
            Self::Clippy(rule_name) | Self::Eslint(rule_name) => (*rule_name).to_string(),
            Self::EslintGraphql(rule_name) => format!("graphql/{rule_name}"),
            Self::EslintGraphqlSchemaLinter(rule_name) => format!("graphql/{rule_name}"),
            Self::EslintImport(rule_name) => format!("import/{rule_name}"),
            Self::EslintImportAccess(rule_name) => format!("import-access/{rule_name}"),
            Self::EslintJest(rule_name) => format!("jest/{rule_name}"),
            Self::EslintJsxA11y(rule_name) => format!("jsx-a11y/{rule_name}"),
            Self::EslintReact(rule_name) => format!("react/{rule_name}"),
            Self::EslintReactHooks(rule_name) => format!("react-hooks/{rule_name}"),
            Self::EslintReactRefresh(rule_name) => format!("react-refresh/{rule_name}"),
            Self::EslintTypeScript(rule_name) => format!("@typescript-eslint/{rule_name}"),
            Self::EslintSolid(rule_name) => format!("solidjs/{rule_name}"),
            Self::EslintSonarJs(rule_name) => format!("sonarjs/{rule_name}"),
            Self::EslintStylistic(rule_name) => format!("@stylistic/{rule_name}"),
            Self::EslintUnicorn(rule_name) => format!("unicorn/{rule_name}"),
            Self::EslintUnusedImports(rule_name) => format!("unused-imports/{rule_name}"),
            Self::EslintMysticatea(rule_name) => format!("@mysticatea/{rule_name}"),
            Self::EslintBarrelFiles(rule_name) => format!("barrel-files/{rule_name}"),
            Self::EslintN(rule_name) => format!("n/{rule_name}"),
            Self::EslintNext(rule_name) => format!("@next/{rule_name}"),
            Self::Stylelint(rule_name) => format!("stylelint/{rule_name}"),
            Self::EslintNoSecrets(rule_name) => format!("no-secrets/{rule_name}"),
            Self::DenoLint(rule_name) => format!("deno-lint/{rule_name}"),
        }
    }

    pub fn to_rule_url(&self) -> String {
        match self {
            Self::Clippy(rule_name) => format!("https://rust-lang.github.io/rust-clippy/master/#{rule_name}"),
            Self::Eslint(rule_name) => format!("https://eslint.org/docs/latest/rules/{rule_name}"),
            Self::EslintGraphql(rule_name) => format!("https://the-guild.dev/graphql/eslint/rules/{rule_name}"),
            Self::EslintGraphqlSchemaLinter(rule_name) => format!("https://github.com/cjoudrey/graphql-schema-linter?tab=readme-ov-file#{rule_name}"),
            Self::EslintImport(rule_name) => format!("https://github.com/import-js/eslint-plugin-import/blob/main/docs/rules/{rule_name}.md"),
            Self::EslintImportAccess(_) => "https://github.com/uhyo/eslint-plugin-import-access".to_string(),
            Self::EslintJest(rule_name) => format!("https://github.com/jest-community/eslint-plugin-jest/blob/main/docs/rules/{rule_name}.md"),
            Self::EslintJsxA11y(rule_name) => format!("https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/{rule_name}.md"),
            Self::EslintReact(rule_name) => format!("https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/{rule_name}.md"),
            Self::EslintReactHooks(_) =>  "https://github.com/facebook/react/blob/main/packages/eslint-plugin-react-hooks/README.md".to_string(),
            Self::EslintReactRefresh(_) => "https://github.com/ArnaudBarre/eslint-plugin-react-refresh".to_string(),
            Self::EslintTypeScript(rule_name) => format!("https://typescript-eslint.io/rules/{rule_name}"),
            Self::EslintSolid(rule_name) => format!("https://github.com/solidjs-community/eslint-plugin-solid/blob/main/packages/eslint-plugin-solid/docs/{rule_name}.md"),
            Self::EslintSonarJs(rule_name) => format!("https://github.com/SonarSource/eslint-plugin-sonarjs/blob/HEAD/docs/rules/{rule_name}.md"),
            Self::EslintStylistic(rule_name) => format!("https://eslint.style/rules/default/{rule_name}"),
            Self::EslintUnicorn(rule_name) => format!("https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/{rule_name}.md"),
            Self::EslintUnusedImports(rule_name) => format!("https://github.com/sweepline/eslint-plugin-unused-imports/blob/master/docs/rules/{rule_name}.md"),
            Self::EslintMysticatea(rule_name) => format!("https://github.com/mysticatea/eslint-plugin/blob/master/docs/rules/{rule_name}.md"),
            Self::EslintBarrelFiles(rule_name) => format!("https://github.com/thepassle/eslint-plugin-barrel-files/blob/main/docs/rules/{rule_name}.md"),
            Self::EslintN(rule_name) => format!("https://github.com/eslint-community/eslint-plugin-n/blob/master/docs/rules/{rule_name}.md"),
            Self::EslintNext(rule_name) => format!("https://nextjs.org/docs/messages/{rule_name}"),
            Self::Stylelint(rule_name) => format!("https://github.com/stylelint/stylelint/blob/main/lib/rules/{rule_name}/README.md"),
            Self::EslintNoSecrets(_) => "https://github.com/nickdeis/eslint-plugin-no-secrets/blob/master/README.md".to_string(),
            Self::DenoLint(rule_name) => format!("https://lint.deno.land/rules/{rule_name}"),
        }
    }

    pub fn as_url_and_rule_name(&self) -> (String, &'static str) {
        (self.to_rule_url(), self.as_rule_name())
    }

    /// Original ESLint rule
    pub const fn is_eslint(&self) -> bool {
        matches!(self, Self::Eslint(_))
    }

    /// All ESLint plugins, exception for the TypeScript one
    pub const fn is_eslint_plugin(&self) -> bool {
        !matches!(self, Self::Clippy(_) | Self::Eslint(_) | Self::Stylelint(_))
    }

    pub const fn is_stylelint(&self) -> bool {
        matches!(self, Self::Stylelint(_))
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum RuleSourceKind {
    /// The rule implements the same logic of the source
    #[default]
    SameLogic,
    /// The rule deviate of the logic of the source
    Inspired,
}

impl RuleSourceKind {
    pub const fn is_inspired(&self) -> bool {
        matches!(self, Self::Inspired)
    }
}

/// Rule domains
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(
        serde::Deserialize,
        serde::Serialize,
        biome_deserialize_macros::Deserializable
    )
)]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum RuleDomain {
    /// React library rules
    React,
    /// Testing rules
    Test,
    /// Solid.js framework rules
    Solid,
    /// Next.js framework rules
    Next,
}

impl Display for RuleDomain {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        // use lower case naming, it needs to match the name of the configuration
        match self {
            RuleDomain::React => fmt.write_str("react"),
            RuleDomain::Test => fmt.write_str("test"),
            RuleDomain::Solid => fmt.write_str("solid"),
            RuleDomain::Next => fmt.write_str("next"),
        }
    }
}

impl RuleDomain {
    /// If the project has one of these dependencies, the domain will be automatically enabled, unless it's explicitly disabled by the configuration.
    ///
    /// If the array is empty, it means that the rules that belong to a certain domain won't enable themselves automatically.
    pub const fn manifest_dependencies(self) -> &'static [&'static (&'static str, &'static str)] {
        match self {
            RuleDomain::React => &[&("react", ">=16.0.0")],
            RuleDomain::Test => &[
                &("jest", ">=26.0.0"),
                &("mocha", ">=8.0.0"),
                &("ava", ">=2.0.0"),
                &("vitest", ">=1.0.0"),
            ],
            RuleDomain::Solid => &[&("solid", ">=1.0.0")],
            RuleDomain::Next => &[&("next", ">=14.0.0")],
        }
    }

    /// Global identifiers that should be added to the `globals` of the [crate::AnalyzerConfiguration] type
    pub const fn globals(self) -> &'static [&'static str] {
        match self {
            RuleDomain::React => &[],
            RuleDomain::Test => &[
                "after",
                "afterAll",
                "afterEach",
                "before",
                "beforeEach",
                "beforeAll",
                "describe",
                "it",
                "expect",
                "test",
            ],
            RuleDomain::Solid => &[],
            RuleDomain::Next => &[],
        }
    }
}

impl RuleMetadata {
    pub const fn new(
        version: &'static str,
        name: &'static str,
        docs: &'static str,
        language: &'static str,
    ) -> Self {
        Self {
            deprecated: None,
            version,
            name,
            docs,
            language,
            recommended: false,
            fix_kind: FixKind::None,
            sources: &[],
            source_kind: None,
            severity: Severity::Information,
            domains: &[],
        }
    }

    pub const fn recommended(mut self, recommended: bool) -> Self {
        self.recommended = recommended;
        self
    }

    pub const fn deprecated(mut self, deprecated: &'static str) -> Self {
        self.deprecated = Some(deprecated);
        self
    }

    pub const fn fix_kind(mut self, kind: FixKind) -> Self {
        self.fix_kind = kind;
        self
    }

    pub const fn sources(mut self, sources: &'static [RuleSource]) -> Self {
        self.sources = sources;
        //if self.source_kind.is_none() {
        //    self.source_kind = Some(RuleSourceKind::SameLogic);
        //}
        self
    }

    pub const fn source_kind(mut self, source_kind: RuleSourceKind) -> Self {
        self.source_kind = Some(source_kind);
        self
    }

    pub const fn language(mut self, language: &'static str) -> Self {
        self.language = language;
        self
    }

    pub const fn severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    pub const fn domains(mut self, domains: &'static [RuleDomain]) -> Self {
        self.domains = domains;
        self
    }

    pub fn applicability(&self) -> Applicability {
        self.fix_kind
            .try_into()
            .expect("Fix kind is not set in the rule metadata")
    }

    pub fn action_category(&self, category: RuleCategory, group: &'static str) -> ActionCategory {
        match category {
            RuleCategory::Lint => {
                ActionCategory::QuickFix(Cow::Owned(format!("{}.{}", group, self.name)))
            }
            RuleCategory::Action => {
                ActionCategory::Source(SourceActionKind::Other(Cow::Borrowed(self.name)))
            }
            RuleCategory::Syntax | RuleCategory::Transformation => unimplemented!(""),
        }
    }
}

pub trait RuleMeta {
    type Group: RuleGroup;
    const METADATA: RuleMetadata;
}

/// This macro is used to declare an analyzer rule type, and implement the
//  [RuleMeta] trait for it
///  # Example
///
/// The macro itself expect the following syntax:
///
/// ```rust,ignore
///use biome_analyze::declare_rule;
///
/// declare_lint_rule! {
///     /// Documentation
///     pub(crate) ExampleRule {
///         version: "1.0.0",
///         name: "ruleName",
///         recommended: false,
///     }
/// }
/// ```
///
/// Check [crate](module documentation) for a better
/// understanding of how the macro works
#[macro_export]
macro_rules! declare_lint_rule {
    ( $( #[doc = $doc:literal] )+ $vis:vis $id:ident {
        version: $version:literal,
        name: $name:tt,
        language: $language:literal,
        $( $key:ident: $value:expr, )*
    } ) => {

        biome_analyze::declare_rule!(
            $( #[doc = $doc] )*
            $vis $id {
                version: $version,
                name: $name,
                language: $language,
                $( $key: $value, )*
            }
        );

        // Declare a new `rule_category!` macro in the module context that
        // expands to the category of this rule
        // This is implemented by calling the `group_category!` macro from the
        // parent module (that should be declared by a call to `declare_group!`)
        // and providing it with the name of this rule as a string literal token
        #[expect(unused_macros)]
        macro_rules! rule_category {
            () => { super::group_category!( $name ) };
        }
    };
}

/// This macro is used to declare an analyzer rule type, and implement the
//  [RuleMeta] trait for it
///  # Example
///
/// The macro itself expect the following syntax:
///
/// ```rust,ignore
///use biome_analyze::declare_syntax_rule;
///
/// declare_syntax_rule! {
///     /// Documentation
///     pub(crate) ExampleRule {
///         version: "1.0.0",
///         name: "ruleName",
///         recommended: false,
///     }
/// }
/// ```
///
/// Check [crate](module documentation) for a better
/// understanding of how the macro works
#[macro_export]
macro_rules! declare_syntax_rule {
    ( $( #[doc = $doc:literal] )+ $vis:vis $id:ident {
        version: $version:literal,
        name: $name:tt,
        language: $language:literal,
        $( $key:ident: $value:expr, )*
    } ) => {

        biome_analyze::declare_rule!(
            $( #[doc = $doc] )*
            $vis $id {
                version: $version,
                name: $name,
                language: $language,
                severity: biome_diagnostics::Severity::Error,
                $( $key: $value, )*
            }
        );

        // Declare a new `rule_category!` macro in the module context that
        // expands to the category of this rule
        // This is implemented by calling the `group_category!` macro from the
        // parent module (that should be declared by a call to `declare_group!`)
        // and providing it with the name of this rule as a string literal token
        #[expect(unused_macros)]
        macro_rules! rule_category {
            () => { super::group_category!( $name ) };
        }
    };
}

#[macro_export]
macro_rules! declare_rule {
        ( $( #[doc = $doc:literal] )+ $vis:vis $id:ident {
        version: $version:literal,
        name: $name:tt,
        language: $language:literal,
        $( $key:ident: $value:expr, )*
    } ) => {
        $( #[doc = $doc] )*
        $vis enum $id {}

        impl $crate::RuleMeta for $id {
            type Group = super::Group;
            const METADATA: $crate::RuleMetadata =
                $crate::RuleMetadata::new($version, $name, concat!( $( $doc, "\n", )* ), $language) $( .$key($value) )*;
        }
    }
}

/// This macro is used to declare an analyzer rule type, and implement the
//  [RuleMeta] trait for it
///  # Example
///
/// The macro itself expect the following syntax:
///
/// ```rust,ignore
///use biome_analyze::declare_refactor_rule;
///
/// declare_refactor_rule! {
///     /// Documentation
///     pub(crate) ExampleRule {
///         version: "1.0.0",
///         name: "ruleName",
///         recommended: false,
///     }
/// }
/// ```
///
/// Check [crate](module documentation) for a better
/// understanding of how the macro works
#[macro_export]
macro_rules! declare_source_rule {
    ( $( #[doc = $doc:literal] )+ $vis:vis $id:ident {
        version: $version:literal,
        name: $name:tt,
        language: $language:literal,
        $( $key:ident: $value:expr, )*
    } ) => {
        biome_analyze::declare_rule!(
            $( #[doc = $doc] )*
            $vis $id {
                version: $version,
                name: $name,
                language: $language,
                fix_kind: biome_analyze::FixKind::None,
                $( $key: $value, )*
            }
        );

        /// This macro returns the corresponding [ActionCategory] to use inside the [RuleAction]
        #[expect(unused_macros)]
        macro_rules! rule_action_category {
            () => { biome_analyze::ActionCategory::Source(biome_analyze::SourceActionKind::Other(Cow::Borrowed($name)))  };
        }
    };
}

/// A rule group is a collection of rules under a given name, serving as a
/// "namespace" for lint rules and allowing the entire set of rules to be
/// disabled at once
pub trait RuleGroup {
    type Language: Language;
    type Category: GroupCategory;
    /// The name of this group, displayed in the diagnostics emitted by its rules
    const NAME: &'static str;
    /// Register all the rules belonging to this group into `registry`
    fn record_rules<V: RegistryVisitor<Self::Language> + ?Sized>(registry: &mut V);
}

/// This macro is used by the codegen script to declare an analyzer rule group,
/// and implement the [RuleGroup] trait for it
#[macro_export]
macro_rules! declare_lint_group {
    ( $vis:vis $id:ident { name: $name:tt, rules: [ $( $( $rule:ident )::* , )* ] } ) => {
        $vis enum $id {}

        impl $crate::RuleGroup for $id {
            type Language = <( $( $( $rule )::* , )* ) as $crate::GroupLanguage>::Language;
            type Category = super::Category;

            const NAME: &'static str = $name;

            fn record_rules<V: $crate::RegistryVisitor<Self::Language> + ?Sized>(registry: &mut V) {
                $( registry.record_rule::<$( $rule )::*>(); )*
            }
        }

        pub(self) use $id as Group;

        // Declare a `group_category!` macro in the context of this module (and
        // all its children). This macro takes the name of a rule as a string
        // literal token and expands to the category of the lint rule with this
        // name within this group.
        // This is implemented by calling the `category_concat!` macro with the
        // "lint" prefix, the name of this group, and the rule name argument
        #[expect(unused_macros)]
        macro_rules! group_category {
            ( $rule_name:tt ) => { $crate::category_concat!( "lint", $name, $rule_name ) };
        }

        // Re-export the macro for child modules, so `declare_rule!` can access
        // the category of its parent group by using the `super` module
        pub(self) use group_category;
    };
}

/// This macro is used by the codegen script to declare an analyzer rule group,
/// and implement the [RuleGroup] trait for it
#[macro_export]
macro_rules! declare_assist_group {
    ( $vis:vis $id:ident { name: $name:tt, rules: [ $( $( $rule:ident )::* , )* ] } ) => {
        $vis enum $id {}

        impl $crate::RuleGroup for $id {
            type Language = <( $( $( $rule )::* , )* ) as $crate::GroupLanguage>::Language;
            type Category = super::Category;

            const NAME: &'static str = $name;

            fn record_rules<V: $crate::RegistryVisitor<Self::Language> + ?Sized>(registry: &mut V) {
                $( registry.record_rule::<$( $rule )::*>(); )*
            }
        }

        pub(self) use $id as Group;

        // Declare a `group_category!` macro in the context of this module (and
        // all its children). This macro takes the name of a rule as a string
        // literal token and expands to the category of the lint rule with this
        // name within this group.
        // This is implemented by calling the `category_concat!` macro with the
        // "lint" prefix, the name of this group, and the rule name argument
        #[expect(unused_macros)]
        macro_rules! group_category {
            ( $rule_name:tt ) => { $crate::category_concat!( "assist", $name, $rule_name ) };
        }

        // Re-export the macro for child modules, so `declare_rule!` can access
        // the category of its parent group by using the `super` module
        pub(self) use group_category;
    };
}

/// This macro is used by the codegen script to declare an analyzer rule group,
/// and implement the [RuleGroup] trait for it
#[macro_export]
macro_rules! declare_syntax_group {
    ( $vis:vis $id:ident { name: $name:tt, rules: [ $( $( $rule:ident )::* , )* ] } ) => {
        $vis enum $id {}

        impl $crate::RuleGroup for $id {
            type Language = <( $( $( $rule )::* , )* ) as $crate::GroupLanguage>::Language;
            type Category = super::Category;

            const NAME: &'static str = $name;

            fn record_rules<V: $crate::RegistryVisitor<Self::Language> + ?Sized>(registry: &mut V) {
                $( registry.record_rule::<$( $rule )::*>(); )*
            }
        }

        pub(self) use $id as Group;

        // Declare a `group_category!` macro in the context of this module (and
        // all its children). This macro takes the name of a rule as a string
        // literal token and expands to the category of the lint rule with this
        // name within this group.
        // This is implemented by calling the `category_concat!` macro with the
        // "lint" prefix, the name of this group, and the rule name argument
        #[expect(unused_macros)]
        macro_rules! group_category {
            ( $rule_name:tt ) => { $crate::category_concat!( "syntax", $name, $rule_name ) };
        }

        // Re-export the macro for child modules, so `declare_rule!` can access
        // the category of its parent group by using the `super` module
        pub(self) use group_category;
    };
}

/// A group category is a collection of rule groups under a given category ID,
/// serving as a broad classification on the kind of diagnostic or code action
/// these rule emit, and allowing whole categories of rules to be disabled at
/// once depending on the kind of analysis being performed
pub trait GroupCategory {
    type Language: Language;
    /// The category ID used for all groups and rule belonging to this category
    const CATEGORY: RuleCategory;
    /// Register all the groups belonging to this category into `registry`
    fn record_groups<V: RegistryVisitor<Self::Language> + ?Sized>(registry: &mut V);
}

#[macro_export]
macro_rules! declare_category {
    ( $vis:vis $id:ident { kind: $kind:ident, groups: [ $( $( $group:ident )::* , )* ] } ) => {
        $vis enum $id {}

        impl $crate::GroupCategory for $id {
            type Language = <( $( $( $group )::* , )* ) as $crate::CategoryLanguage>::Language;

            const CATEGORY: $crate::RuleCategory = $crate::RuleCategory::$kind;

            fn record_groups<V: $crate::RegistryVisitor<Self::Language> + ?Sized>(registry: &mut V) {
                $( registry.record_group::<$( $group )::*>(); )*
            }
        }

        pub(self) use $id as Category;
    };
}

/// This trait is implemented for tuples of [Rule] types of size 1 to 29 if the
/// query type of all the rules in the tuple share the same associated
/// [Language] (which is then aliased as the `Language` associated type on
/// [GroupLanguage] itself). It is used to ensure all the rules in a given
/// group are all querying the same underlying language
pub trait GroupLanguage {
    type Language: Language;
}

/// This trait is implemented for tuples of [Rule] types of size 1 to 29 if the
/// language of all the groups in the tuple share the same associated
/// [Language] (which is then aliased as the `Language` associated type on
/// [CategoryLanguage] itself). It is used to ensure all the groups in a given
/// category are all querying the same underlying language
pub trait CategoryLanguage {
    type Language: Language;
}

/// Helper macro for implementing [GroupLanguage] on a large number of tuple types at once
macro_rules! impl_group_language {
    ( $head:ident $( , $rest:ident )* ) => {
        impl<$head $( , $rest )*> GroupLanguage for ($head, $( $rest ),*)
        where
            $head: Rule $( , $rest: Rule, <$rest as Rule>::Query: Queryable<Language = RuleLanguage<$head>> )*
        {
            type Language = RuleLanguage<$head>;
        }

        impl<$head $( , $rest )*> CategoryLanguage for ($head, $( $rest ),*)
        where
            $head: RuleGroup $( , $rest: RuleGroup<Language = <$head as RuleGroup>::Language> )*
        {
            type Language = <$head as RuleGroup>::Language;
        }

        impl_group_language!( $( $rest ),* );
    };

    () => {};
}

impl_group_language!(
    T00, T01, T02, T03, T04, T05, T06, T07, T08, T09, T10, T11, T12, T13, T14, T15, T16, T17, T18,
    T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37,
    T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56,
    T57, T58, T59, T60, T61, T62, T63, T64, T65, T66, T67, T68, T69, T70, T71, T72, T73, T74, T75,
    T76, T77, T78, T79, T80, T81, T82, T83, T84, T85, T86, T87, T88, T89
);

/// Trait implemented by all analysis rules: declares interest to a certain AstNode type,
/// and a callback function to be executed on all nodes matching the query to possibly
/// raise an analysis event
pub trait Rule: RuleMeta + Sized {
    /// The type of AstNode this rule is interested in
    type Query: Queryable;
    /// A generic type that will be kept in memory between a call to `run` and
    /// subsequent executions of `diagnostic` or `action`, allows the rule to
    /// hold some temporary state between the moment a signal is raised and
    /// when a diagnostic or action needs to be built
    type State;
    /// An iterator type returned by `run` to yield zero or more signals to the
    /// analyzer
    type Signals: IntoIterator<Item = Self::State>;
    /// The options that belong to a rule
    type Options: Default + Clone + Debug;

    fn phase() -> Phases {
        <<<Self as Rule>::Query as Queryable>::Services as Phase>::phase()
    }

    /// This function is called once for each node matching `Query` in the tree
    /// being analyzed. If it returns `Some` the state object will be wrapped
    /// in a generic `AnalyzerSignal`, and the consumer of the analyzer may call
    /// `diagnostic` or `action` on it
    fn run(ctx: &RuleContext<Self>) -> Self::Signals;

    /// Returns the instances associated with the given signal.
    ///
    /// This allows suppression of specific instances of a given rule, without
    /// suppressing other instances of the same rule.
    ///
    /// ## Example
    ///
    /// Consider the situation where the following two variables are unused:
    ///
    /// ```js
    /// let a, b;
    /// ```
    ///
    /// The rule `noUnusedVariables` will report a diagnostic about it, which we
    /// can suppress with `// biome-ignore lint/correctness/noUnusedVariables`.
    ///
    /// But what if we wanted to suppress the rule for `a`, but not for `b`?
    ///
    /// We would need to recognize there are actually two separate instances
    /// that the rule is reporting on, identified as "a" and "b". This allows
    /// the user to suppress a specific instance using
    /// `// biome-ignore lint/correctness/noUnusedVariables(a)`.
    ///
    /// *Note: For `noUnusedVariables` the above may not seem very useful (and
    /// indeed it's not implemented), but for rules such as
    /// `useExhaustiveDependencies` this is actually desirable.*
    fn instances_for_signal(_signal: &Self::State) -> Box<[Box<str>]> {
        Vec::new().into_boxed_slice()
    }

    /// Used by the analyzer to associate a range of source text to a signal in
    /// order to support suppression comments.
    ///
    /// If this function returns [None], the range of the query node will be used instead
    ///
    /// The default implementation returns the range of `Self::diagnostic`, and
    /// should return the correct value for most rules however you may want to
    /// override this if generating a diagnostic for this rule requires heavy
    /// processing and the range could be determined through a faster path
    fn text_range(ctx: &RuleContext<Self>, state: &Self::State) -> Option<TextRange> {
        Self::diagnostic(ctx, state).and_then(|diag| diag.span())
    }

    /// Allows the rule to suppress a set of syntax nodes to prevent them from
    /// matching the `Query`. This is useful for rules that implement a code
    /// action that recursively modifies multiple nodes at once, this hook
    /// allows these rules to avoid matching on those nodes again.
    ///
    /// # Example
    ///
    /// ```ignore
    /// impl Rule for SimplifyExpression {
    ///     type Query = BinaryExpression;
    ///
    ///     fn run(ctx: &RuleContext<Self>) -> Self::Signals {
    ///         // Recursively check this expression and its children for simplification
    ///         // opportunities
    ///         check_can_simplify(ctx.query())
    ///     }
    ///
    ///     fn suppressed_nodes(
    ///         _ctx: &RuleContext<Self>,
    ///         state: &Self::State,
    ///         suppressions: &mut RuleSuppressions<RuleLanguage<Self>>
    ///     ) {
    ///         // Prevent this rule from matching again on nodes that were already checked by
    ///         // `check_can_simplify`
    ///         for node in &state.nodes {
    ///             suppressions.suppress_node(node.clone());
    ///         }
    ///     }
    /// }
    /// ```
    fn suppressed_nodes(
        ctx: &RuleContext<Self>,
        state: &Self::State,
        suppressions: &mut RuleSuppressions<RuleLanguage<Self>>,
    ) {
        let (..) = (ctx, state, suppressions);
    }

    /// Called by the consumer of the analyzer to try to generate a diagnostic
    /// from a signal raised by `run`
    ///
    /// The default implementation returns None
    fn diagnostic(_ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        None
    }

    /// Called by the consumer of the analyzer to try to generate a code action
    /// from a signal raised by `run`
    ///
    /// The default implementation returns None
    fn action(
        ctx: &RuleContext<Self>,
        state: &Self::State,
    ) -> Option<RuleAction<RuleLanguage<Self>>> {
        let (..) = (ctx, state);
        None
    }

    fn top_level_suppression(
        ctx: &RuleContext<Self>,
        suppression_action: &dyn SuppressionAction<Language = RuleLanguage<Self>>,
    ) -> Option<SuppressAction<RuleLanguage<Self>>>
    where
        Self: 'static,
    {
        if <Self::Group as RuleGroup>::Category::CATEGORY == RuleCategory::Lint {
            let rule_category = format!(
                "lint/{}/{}",
                <Self::Group as RuleGroup>::NAME,
                Self::METADATA.name
            );
            let suppression_text = format!("biome-ignore-all {rule_category}");
            let root = ctx.root();

            if let Some(first_token) = root.syntax().first_token() {
                let mut mutation = root.begin();
                suppression_action.apply_top_level_suppression(
                    &mut mutation,
                    first_token,
                    suppression_text.as_str(),
                );
                return Some(SuppressAction {
                    mutation,
                    message: markup! { "Suppress rule " {rule_category} " for the whole file."}
                        .to_owned(),
                });
            }
        }
        None
    }

    /// Create a code action that allows to suppress the rule. The function
    /// returns the node to which the suppression comment is applied.
    fn inline_suppression(
        ctx: &RuleContext<Self>,
        text_range: &TextRange,
        suppression_action: &dyn SuppressionAction<Language = RuleLanguage<Self>>,
        suppression_reason: Option<&str>,
    ) -> Option<SuppressAction<RuleLanguage<Self>>>
    where
        Self: 'static,
    {
        // if the rule belongs to `Lint`, we auto generate an action to suppress the rule
        if <Self::Group as RuleGroup>::Category::CATEGORY == RuleCategory::Lint {
            let rule_category = format!(
                "lint/{}/{}",
                <Self::Group as RuleGroup>::NAME,
                Self::METADATA.name
            );
            let suppression_text = format!("biome-ignore {rule_category}");
            let root = ctx.root();
            let token = root.syntax().token_at_offset(text_range.start());
            let mut mutation = root.begin();
            suppression_action.inline_suppression(SuppressionCommentEmitterPayload {
                suppression_text: suppression_text.as_str(),
                mutation: &mut mutation,
                token_offset: token,
                diagnostic_text_range: text_range,
                suppression_reason: suppression_reason.unwrap_or("<explanation>"),
            });

            Some(SuppressAction {
                mutation,
                message: markup! { "Suppress rule " {rule_category} }.to_owned(),
            })
        } else {
            None
        }
    }

    /// Returns a mutation to apply to the code
    fn transform(
        _ctx: &RuleContext<Self>,
        _state: &Self::State,
    ) -> Option<BatchMutation<RuleLanguage<Self>>> {
        None
    }
}

/// Diagnostic object returned by a single analysis rule
#[derive(Clone, Debug, Diagnostic)]
pub struct RuleDiagnostic {
    #[category]
    pub(crate) category: &'static Category,
    #[location(span)]
    pub(crate) span: Option<TextRange>,
    #[message]
    #[description]
    pub(crate) message: MessageAndDescription,
    #[tags]
    pub(crate) tags: DiagnosticTags,
    #[advice]
    pub(crate) rule_advice: RuleAdvice,
    #[severity]
    pub(crate) severity: Severity,
}

#[derive(Clone, Debug, Default)]
/// It contains possible advices to show when printing a diagnostic that belong to the rule
pub struct RuleAdvice {
    pub(crate) details: Vec<Detail>,
    pub(crate) notes: Vec<(LogCategory, MarkupBuf)>,
    pub(crate) suggestion_list: Option<SuggestionList>,
    pub(crate) code_suggestion_list: Vec<CodeSuggestionAdvice<MarkupBuf>>,
}

#[derive(Clone, Debug, Default)]
pub struct SuggestionList {
    pub(crate) message: MarkupBuf,
    pub(crate) list: Vec<MarkupBuf>,
}

impl Advices for RuleAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        for detail in &self.details {
            visitor.record_log(
                detail.log_category,
                &markup! { {detail.message} }.to_owned(),
            )?;
            visitor.record_frame(Location::builder().span(&detail.range).build())?;
        }
        // we then print notes
        for (log_category, note) in &self.notes {
            visitor.record_log(*log_category, &markup! { {note} }.to_owned())?;
        }

        if let Some(suggestion_list) = &self.suggestion_list {
            visitor.record_log(
                LogCategory::Info,
                &markup! { {suggestion_list.message} }.to_owned(),
            )?;
            let list: Vec<_> = suggestion_list
                .list
                .iter()
                .map(|suggestion| suggestion as &dyn Display)
                .collect();
            visitor.record_list(&list)?;
        }

        // finally, we print possible code suggestions on how to fix the issue
        for suggestion in &self.code_suggestion_list {
            suggestion.record(visitor)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Detail {
    pub log_category: LogCategory,
    pub message: MarkupBuf,
    pub range: Option<TextRange>,
}

impl RuleDiagnostic {
    /// Creates a new [`RuleDiagnostic`] with a severity and title that will be
    /// used in a builder-like way to modify labels.
    pub fn new(category: &'static Category, span: impl AsSpan, title: impl Display) -> Self {
        let message = markup!({ title }).to_owned();
        Self {
            category,
            span: span.as_span(),
            message: MessageAndDescription::from(message),
            tags: DiagnosticTags::empty(),
            rule_advice: RuleAdvice::default(),
            severity: Severity::default(),
        }
    }

    /// Set an explicit plain-text summary for this diagnostic.
    pub fn description(mut self, summary: impl Into<String>) -> Self {
        self.message.set_description(summary.into());
        self
    }

    /// Marks this diagnostic as deprecated code, which will
    /// be displayed in the language server.
    ///
    /// This does not have any influence on the diagnostic rendering.
    pub fn deprecated(mut self) -> Self {
        self.tags |= DiagnosticTags::DEPRECATED_CODE;
        self
    }

    /// Marks this diagnostic as unnecessary code, which will
    /// be displayed in the language server.
    ///
    /// This does not have any influence on the diagnostic rendering.
    pub fn unnecessary(mut self) -> Self {
        self.tags |= DiagnosticTags::UNNECESSARY_CODE;
        self
    }

    /// Marks this diagnostic as verbose.
    ///
    /// The diagnostic will only be shown when using the `--verbose` argument.
    pub fn verbose(mut self) -> Self {
        self.tags |= DiagnosticTags::VERBOSE;
        self
    }

    /// Attaches a label to this [`RuleDiagnostic`].
    ///
    /// The given span has to be in the file that was provided while creating this [`RuleDiagnostic`].
    pub fn label(mut self, span: impl AsSpan, msg: impl Display) -> Self {
        self.rule_advice.details.push(Detail {
            log_category: LogCategory::Info,
            message: markup!({ msg }).to_owned(),
            range: span.as_span(),
        });
        self
    }

    /// Attaches a detailed message to this [`RuleDiagnostic`].
    pub fn detail(self, span: impl AsSpan, msg: impl Display) -> Self {
        self.label(span, msg)
    }

    /// Adds a footer to this [`RuleDiagnostic`], which will be displayed under the actual error.
    fn footer(mut self, log_category: LogCategory, msg: impl Display) -> Self {
        self.rule_advice
            .notes
            .push((log_category, markup!({ msg }).to_owned()));
        self
    }

    /// Adds a footer to this [`RuleDiagnostic`], with the `Info` log category.
    pub fn note(self, msg: impl Display) -> Self {
        self.footer(LogCategory::Info, msg)
    }

    /// It creates a new footer note which contains a message and a list of possible suggestions.
    /// Useful when there's need to suggest a list of things inside a diagnostic.
    pub fn footer_list(
        mut self,
        message: impl Display,
        list: impl IntoIterator<Item = impl Display>,
    ) -> Self {
        self.rule_advice.suggestion_list = Some(SuggestionList {
            message: markup! { {message} }.to_owned(),
            list: list
                .into_iter()
                .map(|msg| markup! {{msg}}.to_owned())
                .collect(),
        });
        self
    }

    /// Adds a footer to this [`RuleDiagnostic`], with the `Warn` severity.
    pub fn warning(self, msg: impl Display) -> Self {
        self.footer(LogCategory::Warn, msg)
    }

    pub(crate) fn span(&self) -> Option<TextRange> {
        self.span
    }

    pub fn advices(&self) -> &RuleAdvice {
        &self.rule_advice
    }
}

/// Code Action object returned by a single analysis rule
pub struct RuleAction<L: Language> {
    pub category: ActionCategory,
    applicability: Applicability,
    pub message: MarkupBuf,
    pub mutation: BatchMutation<L>,
}

impl<L: Language> RuleAction<L> {
    pub fn new(
        category: ActionCategory,
        applicability: impl Into<Applicability>,
        message: impl Display,
        mutation: BatchMutation<L>,
    ) -> Self {
        Self {
            category,
            applicability: applicability.into(),
            message: markup! {{message}}.to_owned(),
            mutation,
        }
    }

    pub fn applicability(&self) -> Applicability {
        self.applicability
    }
}

/// An action meant to suppress a lint rule
#[derive(Clone)]
pub struct SuppressAction<L: Language> {
    pub message: MarkupBuf,
    pub mutation: BatchMutation<L>,
}
