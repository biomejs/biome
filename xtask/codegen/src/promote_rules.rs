use crate::move_rule::{
    RuleMove, apply_rule_moves, plan_rule_moves, update_rule_categories, validate_rule_categories,
};
use anyhow::{Context, Result, bail, ensure};
use biome_deserialize::{
    Deserializable, DeserializableTypes, DeserializableValue, DeserializationContext,
    DeserializationVisitor, MapMembers, Text, TextRange, json::deserialize_from_json_str,
};
use biome_deserialize_macros::Deserializable;
use biome_diagnostics::print_diagnostic_to_string;
use biome_string_case::Case;
use regex::{Captures, Regex};
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::Path,
};

const RULE_OPTIONS_PATH: &str = "crates/biome_rule_options/src";
const RULE_RENAMING_PATH: &str = "crates/biome_migrate/src/analyzers/rule_mover.rs";

#[derive(Clone, Copy, Debug, Default, Deserializable, Eq, PartialEq)]
enum RuleGroup {
    A11y,
    Complexity,
    #[default]
    Correctness,
    Performance,
    Security,
    Style,
    Suspicious,
}

impl RuleGroup {
    const fn as_str(self) -> &'static str {
        match self {
            Self::A11y => "a11y",
            Self::Complexity => "complexity",
            Self::Correctness => "correctness",
            Self::Performance => "performance",
            Self::Security => "security",
            Self::Style => "style",
            Self::Suspicious => "suspicious",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserializable, Eq, PartialEq)]
enum RuleSeverity {
    Info,
    Warn,
    Error,
}

impl RuleSeverity {
    const fn rust_expression(self) -> &'static str {
        match self {
            Self::Info => "Severity::Information",
            Self::Warn => "Severity::Warning",
            Self::Error => "Severity::Error",
        }
    }
}

#[derive(Clone, Debug, Default, Deserializable, Eq, PartialEq)]
#[deserializable(unknown_fields = "deny")]
struct Promotion {
    #[deserializable(required)]
    group: RuleGroup,
    recommended: Option<bool>,
    severity: Option<RuleSeverity>,
    new_name: Option<String>,
}

impl Promotion {
    fn group(&self) -> RuleGroup {
        self.group
    }
}

type PromotionManifest = BTreeMap<String, Promotion>;

#[derive(Debug)]
struct PlannedPromotion {
    current_name: String,
    new_name: String,
    promotion: Promotion,
    moves: Vec<RuleMove>,
}

pub fn promote_rules(manifest_path: &Path) -> Result<()> {
    let root = env::current_dir().context("failed to determine the current directory")?;
    promote_rules_at(&root, manifest_path)
}

fn promote_rules_at(root: &Path, manifest_path: &Path) -> Result<()> {
    let manifest = fs::read_to_string(manifest_path)
        .with_context(|| format!("failed to read {}", manifest_path.display()))?;
    let manifest = deserialize_json(&manifest, manifest_path)?;
    let plan = build_plan(root, manifest)?;

    apply_plan(root, &plan)
}

fn deserialize_json<T: Deserializable>(source: &str, path: &Path) -> Result<T> {
    let path = path.display().to_string();
    let (deserialized, diagnostics) =
        deserialize_from_json_str(source, Default::default(), &path).consume();
    if !diagnostics.is_empty() {
        let diagnostics = diagnostics
            .iter()
            .map(print_diagnostic_to_string)
            .collect::<Vec<_>>()
            .join("\n");
        bail!("failed to parse {path}:\n{diagnostics}");
    }
    deserialized.with_context(|| format!("failed to deserialize {path}"))
}

fn build_plan(root: &Path, manifest: PromotionManifest) -> Result<Vec<PlannedPromotion>> {
    ensure!(!manifest.is_empty(), "the promotion manifest is empty");

    let mut target_names = BTreeSet::new();
    let mut plan = Vec::with_capacity(manifest.len());

    for (current_name, promotion) in manifest {
        validate_rule_name(&current_name)?;
        let new_name = promotion
            .new_name
            .clone()
            .unwrap_or_else(|| current_name.clone());
        validate_rule_name(&new_name)?;
        ensure!(
            target_names.insert(new_name.clone()),
            "multiple rules are promoted as {new_name}"
        );

        let moves = plan_rule_moves(root, &current_name, promotion.group().as_str(), &new_name)?;
        ensure!(
            !moves.is_empty(),
            "could not find the nursery rule {current_name}"
        );
        for rule_move in &moves {
            ensure!(
                rule_move.category == "lint" && rule_move.old_group == "nursery",
                "{} is not a nursery lint rule",
                rule_move.source.display()
            );
            ensure!(
                !rule_move.is_noop(),
                "{current_name} is already in the {} group",
                promotion.group().as_str()
            );

            let source = fs::read_to_string(&rule_move.source)
                .with_context(|| format!("failed to read {}", rule_move.source.display()))?;
            ensure!(
                source.contains(&format!("name: \"{current_name}\"")),
                "{} does not declare the rule {current_name}",
                rule_move.source.display()
            );
            update_rule_source(source, &current_name, &new_name, &promotion)
                .with_context(|| format!("failed to update {}", rule_move.source.display()))?;
            validate_fixture_options(
                &rule_move.tests,
                &current_name,
                &new_name,
                promotion.group(),
            )?;
        }
        validate_rule_categories(root, moves.iter())?;

        if current_name != new_name {
            validate_options_rename(root, &current_name, &new_name)?;
        }

        plan.push(PlannedPromotion {
            current_name,
            new_name,
            promotion,
            moves,
        });
    }

    Ok(plan)
}

fn validate_rule_name(name: &str) -> Result<()> {
    let mut chars = name.chars();
    ensure!(
        chars.next().is_some_and(|first| first.is_ascii_lowercase())
            && chars.all(|character| character.is_ascii_alphanumeric()),
        "{name} is not an ASCII lower-camel-case rule name"
    );
    ensure!(
        Case::Camel.convert(name) == name,
        "{name} is not a lower-camel-case rule name"
    );
    Ok(())
}

fn validate_options_rename(root: &Path, current_name: &str, new_name: &str) -> Result<()> {
    let options_path = root.join(RULE_OPTIONS_PATH);
    let current_name_snake = Case::Snake.convert(current_name);
    let current_path = options_path.join(format!("{current_name_snake}.rs"));
    ensure!(
        current_path.is_file(),
        "the rule options file {} does not exist",
        current_path.display()
    );
    let new_path = options_path.join(format!("{}.rs", Case::Snake.convert(new_name)));
    ensure!(
        !new_path.exists(),
        "the rule options destination {} already exists",
        new_path.display()
    );

    let options_lib_path = options_path.join("lib.rs");
    let options_lib = fs::read_to_string(&options_lib_path)
        .with_context(|| format!("failed to read {}", options_lib_path.display()))?;
    ensure!(
        options_lib
            .matches(&format!("pub mod {current_name_snake};"))
            .count()
            == 1,
        "expected one module declaration for {current_name_snake}"
    );

    let rule_renaming_path = root.join(RULE_RENAMING_PATH);
    let rule_renamings = fs::read_to_string(&rule_renaming_path)
        .with_context(|| format!("failed to read {}", rule_renaming_path.display()))?;
    rule_renaming_entries_range(&rule_renamings)?;
    ensure!(
        !rule_renamings.contains(&format!("(\"{current_name}\",")),
        "RULE_RENAMING already contains {current_name}"
    );
    Ok(())
}

fn apply_plan(root: &Path, plan: &[PlannedPromotion]) -> Result<()> {
    for planned in plan {
        println!(
            "Promoting lint/nursery/{} to lint/{}/{}...",
            planned.current_name,
            planned.promotion.group().as_str(),
            planned.new_name
        );
        for rule_move in &planned.moves {
            let source = fs::read_to_string(&rule_move.source)
                .with_context(|| format!("failed to read {}", rule_move.source.display()))?;
            let source = update_rule_source(
                source,
                &planned.current_name,
                &planned.new_name,
                &planned.promotion,
            )?;
            fs::write(&rule_move.source, source)
                .with_context(|| format!("failed to write {}", rule_move.source.display()))?;
            update_fixtures(
                &rule_move.tests,
                &planned.current_name,
                &planned.new_name,
                planned.promotion.group(),
            )?;
        }

        if planned.current_name != planned.new_name {
            rename_rule_options(root, &planned.current_name, &planned.new_name)?;
        }
        apply_rule_moves(&planned.moves)?;
    }

    update_rule_categories(root, plan.iter().flat_map(|planned| planned.moves.iter()))?;
    update_rule_renamings(root, plan)?;
    Ok(())
}

fn update_rule_source(
    mut source: String,
    current_name: &str,
    new_name: &str,
    promotion: &Promotion,
) -> Result<String> {
    if current_name != new_name {
        source = rename_rule_text(source, current_name, new_name)?;
    }

    let old_category = format!("lint/nursery/{current_name}");
    let new_category = format!("lint/{}/{new_name}", promotion.group().as_str());
    source = source.replace(&old_category, &new_category);

    if let Some(recommended) = promotion.recommended {
        set_metadata_field(
            &mut source,
            "recommended",
            if recommended { "true" } else { "false" },
            "language",
        )?;
    }
    if let Some(severity) = promotion.severity {
        let has_severity_import = source.lines().any(|line| {
            line.trim_start().starts_with("use biome_diagnostics") && line.contains("Severity")
        });
        set_metadata_field(
            &mut source,
            "severity",
            severity.rust_expression(),
            "recommended",
        )?;
        if !has_severity_import {
            source.insert_str(0, "use biome_diagnostics::Severity;\n");
        }
    }

    Ok(source)
}

fn rename_rule_text(mut text: String, current_name: &str, new_name: &str) -> Result<String> {
    let identifier_replacements = [
        (
            Case::Pascal.convert(current_name),
            Case::Pascal.convert(new_name),
        ),
        (
            Case::Snake.convert(current_name),
            Case::Snake.convert(new_name),
        ),
    ];
    for (current, new) in identifier_replacements {
        let pattern = Regex::new(&format!(
            r"\b{}(?P<suffix>[A-Za-z0-9_]*)\b",
            regex::escape(&current)
        ))?;
        text = pattern
            .replace_all(&text, |captures: &Captures<'_>| {
                format!("{new}{}", &captures["suffix"])
            })
            .into_owned();
    }

    let exact_replacements = [
        (format!("\"{current_name}\""), format!("\"{new_name}\"")),
        (
            format!("/rules/{}", Case::Kebab.convert(current_name)),
            format!("/rules/{}", Case::Kebab.convert(new_name)),
        ),
    ];
    for (current, new) in exact_replacements {
        text = Regex::new(&regex::escape(&current))?
            .replace_all(&text, new)
            .into_owned();
    }
    Ok(text)
}

fn set_metadata_field(
    source: &mut String,
    field: &str,
    value: &str,
    insert_after: &str,
) -> Result<()> {
    let mut lines: Vec<String> = source.split_inclusive('\n').map(str::to_string).collect();
    if source.is_empty() || !source.ends_with('\n') {
        let consumed = lines.iter().map(String::len).sum::<usize>();
        if consumed < source.len() {
            lines.push(source[consumed..].to_string());
        }
    }

    let field_prefix = format!("{field}:");
    let field_positions: Vec<_> = lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| {
            line.trim_start()
                .starts_with(&field_prefix)
                .then_some(index)
        })
        .collect();
    ensure!(
        field_positions.len() <= 1,
        "found multiple {field} fields in a rule declaration"
    );

    if let Some(index) = field_positions.first().copied() {
        let indent = &lines[index][..lines[index].len() - lines[index].trim_start().len()];
        let newline = if lines[index].ends_with('\n') {
            "\n"
        } else {
            ""
        };
        lines[index] = format!("{indent}{field}: {value},{newline}");
    } else {
        let insert_after_prefix = format!("{insert_after}:");
        let insert_positions: Vec<_> = lines
            .iter()
            .enumerate()
            .filter_map(|(index, line)| {
                line.trim_start()
                    .starts_with(&insert_after_prefix)
                    .then_some(index)
            })
            .collect();
        ensure!(
            insert_positions.len() == 1,
            "expected one {insert_after} field in a rule declaration"
        );
        let index = insert_positions[0];
        let indent = &lines[index][..lines[index].len() - lines[index].trim_start().len()];
        lines.insert(index + 1, format!("{indent}{field}: {value},\n"));
    }

    *source = lines.concat();
    Ok(())
}

fn validate_fixture_options(
    path: &Path,
    current_name: &str,
    new_name: &str,
    group: RuleGroup,
) -> Result<()> {
    process_fixtures(path, current_name, new_name, group, false)
}

fn update_fixtures(
    path: &Path,
    current_name: &str,
    new_name: &str,
    group: RuleGroup,
) -> Result<()> {
    process_fixtures(path, current_name, new_name, group, true)
}

fn process_fixtures(
    path: &Path,
    current_name: &str,
    new_name: &str,
    group: RuleGroup,
    write: bool,
) -> Result<()> {
    for entry in fs::read_dir(path).with_context(|| format!("failed to read {}", path.display()))? {
        let entry =
            entry.with_context(|| format!("failed to read an entry in {}", path.display()))?;
        let entry_path = entry.path();
        if entry
            .file_type()
            .with_context(|| format!("failed to inspect {}", entry_path.display()))?
            .is_dir()
        {
            process_fixtures(&entry_path, current_name, new_name, group, write)?;
            continue;
        }

        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if file_name.ends_with(".snap") || (!write && !file_name.ends_with("options.json")) {
            continue;
        }

        let mut contents = fs::read_to_string(&entry_path)
            .with_context(|| format!("failed to read {}", entry_path.display()))?;
        if write {
            let old_category = format!("lint/nursery/{current_name}");
            let new_category = format!("lint/{}/{new_name}", group.as_str());
            contents = contents.replace(&old_category, &new_category);
        }
        if file_name.ends_with("options.json") {
            contents = update_fixture_options(contents, current_name, new_name, group, &entry_path)
                .with_context(|| format!("failed to update {}", entry_path.display()))?;
        }
        if write {
            fs::write(&entry_path, contents)
                .with_context(|| format!("failed to write {}", entry_path.display()))?;
        }
    }
    Ok(())
}

#[derive(Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
struct FixtureOptions {
    linter: Option<FixtureLinter>,
}

#[derive(Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
struct FixtureLinter {
    rules: Option<FixtureRuleGroups>,
}

#[derive(Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
struct FixtureRuleGroups {
    a11y: Option<FixtureRuleNames>,
    complexity: Option<FixtureRuleNames>,
    correctness: Option<FixtureRuleNames>,
    nursery: Option<FixtureRuleNames>,
    performance: Option<FixtureRuleNames>,
    security: Option<FixtureRuleNames>,
    style: Option<FixtureRuleNames>,
    suspicious: Option<FixtureRuleNames>,
}

impl FixtureRuleGroups {
    fn get(&self, group: RuleGroup) -> Option<&FixtureRuleNames> {
        match group {
            RuleGroup::A11y => self.a11y.as_ref(),
            RuleGroup::Complexity => self.complexity.as_ref(),
            RuleGroup::Correctness => self.correctness.as_ref(),
            RuleGroup::Performance => self.performance.as_ref(),
            RuleGroup::Security => self.security.as_ref(),
            RuleGroup::Style => self.style.as_ref(),
            RuleGroup::Suspicious => self.suspicious.as_ref(),
        }
    }
}

#[derive(Default)]
struct FixtureRuleNames(BTreeSet<String>);

impl Deserializable for FixtureRuleNames {
    fn deserialize(
        ctx: &mut dyn DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = FixtureRuleNames;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;

            fn visit_map(
                self,
                ctx: &mut dyn DeserializationContext,
                members: &mut MapMembers<'_>,
                _range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                let mut names = BTreeSet::new();
                for (key, _) in members.flatten() {
                    if let Some(key) = Text::deserialize(ctx, &key, "") {
                        names.insert(key.text().to_string());
                    }
                }
                Some(FixtureRuleNames(names))
            }
        }
        value.deserialize(ctx, Visitor, name)
    }
}

fn update_fixture_options(
    mut contents: String,
    current_name: &str,
    new_name: &str,
    group: RuleGroup,
    path: &Path,
) -> Result<String> {
    let options: FixtureOptions = deserialize_json(&contents, path)?;
    let Some(rules) = options.linter.and_then(|linter| linter.rules) else {
        return Ok(contents);
    };
    let Some(nursery) = rules.nursery.as_ref() else {
        return Ok(contents);
    };
    if !nursery.0.contains(current_name) {
        return Ok(contents);
    }

    ensure!(
        nursery.0.len() == 1,
        "the nursery fixture group contains rules other than {current_name}"
    );
    ensure!(
        rules.get(group).is_none(),
        "the fixture already contains the {} group",
        group.as_str()
    );

    contents = replace_json_key(contents, "nursery", group.as_str())?;
    if current_name != new_name {
        contents = replace_json_key(contents, current_name, new_name)?;
    }
    Ok(contents)
}

fn replace_json_key(contents: String, current: &str, new: &str) -> Result<String> {
    let pattern = Regex::new(&format!(
        r#""{}"(?P<separator>\s*:)"#,
        regex::escape(current)
    ))?;
    ensure!(
        pattern.find_iter(&contents).count() == 1,
        "expected one {current} key"
    );
    Ok(pattern
        .replace(&contents, |captures: &Captures<'_>| {
            format!(r#""{new}"{}"#, &captures["separator"])
        })
        .into_owned())
}

fn rename_rule_options(root: &Path, current_name: &str, new_name: &str) -> Result<()> {
    let options_path = root.join(RULE_OPTIONS_PATH);
    let current_name_snake = Case::Snake.convert(current_name);
    let new_name_snake = Case::Snake.convert(new_name);
    let current_path = options_path.join(format!("{current_name_snake}.rs"));
    let new_path = options_path.join(format!("{new_name_snake}.rs"));
    let contents = fs::read_to_string(&current_path)
        .with_context(|| format!("failed to read {}", current_path.display()))?;
    fs::write(
        &current_path,
        rename_rule_text(contents, current_name, new_name)?,
    )
    .with_context(|| format!("failed to write {}", current_path.display()))?;
    fs::rename(&current_path, &new_path).with_context(|| {
        format!(
            "failed to move {} to {}",
            current_path.display(),
            new_path.display()
        )
    })?;

    let lib_path = options_path.join("lib.rs");
    let mut lib = fs::read_to_string(&lib_path)
        .with_context(|| format!("failed to read {}", lib_path.display()))?;
    let old_module = format!("pub mod {current_name_snake};");
    ensure!(
        lib.matches(&old_module).count() == 1,
        "expected one module declaration for {current_name_snake}"
    );
    lib = lib.replace(&old_module, &format!("pub mod {new_name_snake};"));
    fs::write(&lib_path, lib).with_context(|| format!("failed to write {}", lib_path.display()))?;
    Ok(())
}

fn update_rule_renamings(root: &Path, plan: &[PlannedPromotion]) -> Result<()> {
    let renamings: Vec<_> = plan
        .iter()
        .filter(|planned| planned.current_name != planned.new_name)
        .collect();
    if renamings.is_empty() {
        return Ok(());
    }

    let path = root.join(RULE_RENAMING_PATH);
    let mut contents =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let (entries_start, entries_end) = rule_renaming_entries_range(&contents)?;
    let mut entries = contents[entries_start..entries_end].trim().to_string();

    for planned in renamings {
        let entry = format!(
            "    (\"{}\", RuleName::{})",
            planned.current_name,
            Case::Pascal.convert(&planned.new_name)
        );
        ensure!(
            !entries.contains(&format!("(\"{}\",", planned.current_name)),
            "RULE_RENAMING already contains {}",
            planned.current_name
        );
        if !entries.trim().is_empty() {
            entries.push_str(",\n");
        }
        entries.push_str(&entry);
    }

    contents.replace_range(entries_start..entries_end, &format!("\n{entries}"));
    fs::write(&path, contents).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

fn rule_renaming_entries_range(contents: &str) -> Result<(usize, usize)> {
    let list_start = contents
        .find("const RULE_RENAMING:")
        .context("failed to find RULE_RENAMING")?;
    let entries_start = contents[list_start..]
        .find("&[")
        .map(|offset| list_start + offset + 2)
        .context("failed to find the start of RULE_RENAMING")?;
    let entries_end = contents[entries_start..]
        .find("\n];")
        .map(|offset| entries_start + offset)
        .context("failed to find the end of RULE_RENAMING")?;
    Ok((entries_start, entries_end))
}
