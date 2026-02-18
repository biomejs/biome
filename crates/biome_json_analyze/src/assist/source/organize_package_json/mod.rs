use crate::JsonRuleAction;
use crate::utils::is_package_json;
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, context::RuleContext, declare_source_rule,
};
use biome_console::markup;
use biome_diagnostics::category;
use biome_json_factory::make;
use biome_json_syntax::{JsonMember, JsonMemberList, JsonObjectValue, JsonRoot, T};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use std::collections::HashMap;

mod field_order;
mod sorters;

use field_order::{FieldTransformer, get_field_index, get_field_transformer};

declare_source_rule! {
    /// Organize package.json fields according to established conventions.
    ///
    /// Sorts fields following the same conventions as the popular
    /// [sort-package-json](https://github.com/keithamus/sort-package-json) tool.
    ///
    /// ## Sorting Rules
    ///
    /// `package.json` fields are sorted by the order they are listed below. The default key sort order is alphabetical.
    ///
    /// _Note: when a specific key order is used, any other keys will be sorted in the end of the object_
    ///
    /// | Key                   | Rules                                                                          |
    /// | --------------------- | ------------------------------------------------------------------------------ |
    /// | \$schema              |                                                                                |
    /// | name                  |                                                                                |
    /// | displayName           |                                                                                |
    /// | version               |                                                                                |
    /// | private               |                                                                                |
    /// | description           |                                                                                |
    /// | categories            | Unique items                                                                   |
    /// | keywords              | Unique items                                                                   |
    /// | homepage              |                                                                                |
    /// | bugs                  | Key order: `url`, `email`                                                      |
    /// | repository            | Key order: `type`, `url`                                                       |
    /// | funding               | Key order: `type`, `url`                                                       |
    /// | license               | Key order: `type`, `url`                                                       |
    /// | qna                   |                                                                                |
    /// | author                | Key order: `name`, `email`, `url`                                              |
    /// | maintainers           | Key order (per item): `name`, `email`, `url`                                   |
    /// | contributors          | Key order (per item): `name`, `email`, `url`                                   |
    /// | publisher             |                                                                                |
    /// | sideEffects           |                                                                                |
    /// | type                  |                                                                                |
    /// | imports               |                                                                                |
    /// | exports               |                                                                                |
    /// | main                  |                                                                                |
    /// | svelte                |                                                                                |
    /// | umd:main              |                                                                                |
    /// | jsdelivr              |                                                                                |
    /// | unpkg                 |                                                                                |
    /// | module                |                                                                                |
    /// | source                |                                                                                |
    /// | jsnext:main           |                                                                                |
    /// | browser               |                                                                                |
    /// | react-native          |                                                                                |
    /// | types                 |                                                                                |
    /// | typesVersions         |                                                                                |
    /// | typings               |                                                                                |
    /// | style                 |                                                                                |
    /// | example               |                                                                                |
    /// | examplestyle          |                                                                                |
    /// | assets                |                                                                                |
    /// | bin                   | Key sort                                                                       |
    /// | man                   |                                                                                |
    /// | directories           | Key order: `lib`, `bin`, `man`, `doc`, `example`, `test`                       |
    /// | files                 | Unique items                                                                   |
    /// | workspaces            | Key order (when object): `packages`, `catalog`                                 |
    /// | binary,               | Key order: `module_name`, `module_path`, `remote_path`, `package_name`, `host` |
    /// | scripts               | [Script sort](#scripts)                                                        |
    /// | betterScripts         | [Script sort](#scripts)                                                        |
    /// | contributes           | Key sort                                                                       |
    /// | activationEvents      | Unique items                                                                   |
    /// | husky                 | Sorts the `hooks` field using [git hook sort](#git-hooks)                      |
    /// | simple-git-hooks      | Key sort using [git hook sort](#git-hooks)                                     |
    /// | pre-commit            |                                                                                |
    /// | commitlint            | Key sort                                                                       |
    /// | lint-staged           |                                                                                |
    /// | nano-staged           |                                                                                |
    /// | config                | Key sort                                                                       |
    /// | nodemonConfig         | Key sort                                                                       |
    /// | browserify            | Key sort                                                                       |
    /// | babel                 | Key sort                                                                       |
    /// | browserslist          |                                                                                |
    /// | xo                    | Key sort                                                                       |
    /// | prettier              | [Prettier sort](#prettier)                                                     |
    /// | eslintConfig          | [ESLint sort](#eslint)                                                         |
    /// | eslintIgnore          |                                                                                |
    /// | npmpackagejsonlint    | Key sort (also recognizes: npmPackageJsonLintConfig, npmpkgjsonlint)           |
    /// | release               | Key sort                                                                       |
    /// | remarkConfig          | Key sort                                                                       |
    /// | stylelint             |                                                                                |
    /// | ava                   | Key sort                                                                       |
    /// | jest                  | Key sort                                                                       |
    /// | mocha                 | Key sort                                                                       |
    /// | nyc                   | Key sort                                                                       |
    /// | tap                   | Key sort                                                                       |
    /// | oclif                 | Key sort (deep)                                                                |
    /// | resolutions           | Key sort                                                                       |
    /// | dependencies          | Key sort                                                                       |
    /// | devDependencies       | Key sort                                                                       |
    /// | dependenciesMeta      | Key sort (deep)                                                                |
    /// | peerDependencies      | Key sort                                                                       |
    /// | peerDependenciesMeta  | Key sort (deep)                                                                |
    /// | optionalDependencies  | Key sort                                                                       |
    /// | bundledDependencies   | Sort unique items                                                              |
    /// | bundleDependencies    | Sort unique items                                                              |
    /// | extensionPack         | Sort unique items                                                              |
    /// | extensionDependencies | Sort unique items                                                              |
    /// | flat                  |                                                                                |
    /// | packageManager        |                                                                                |
    /// | engines               | Key sort                                                                       |
    /// | engineStrict          | Key sort                                                                       |
    /// | devEngines            | Key order (packageManager): `name`, `version`, `onFail`                        |
    /// | volta                 | Key order: `node`, `npm`, `yarn`                                               |
    /// | languageName          |                                                                                |
    /// | os                    |                                                                                |
    /// | cpu                   |                                                                                |
    /// | preferGlobal          | Key sort                                                                       |
    /// | publishConfig         | Key sort                                                                       |
    /// | icon                  |                                                                                |
    /// | badges                | Key order (per item): `description`, `url`, `href`                             |
    /// | galleryBanner         | Key sort                                                                       |
    /// | preview               |                                                                                |
    /// | markdown              |                                                                                |
    /// | pnpm                  | Base property order: `neverBuiltDependencies`, `onlyBuiltDependencies`, `onlyBuiltDependenciesFile`, `overrides`, `packageExtensions`, `patchedDependencies`, `peerDependencyRules`, `allowedDeprecatedVersions`, `allowNonAppliedPatches`, `auditConfig`, `ignoredOptionalDependencies`, `updateConfig`. Other properties sorted alphabetically. Deep sorting applied. |
    ///
    /// ### Special Rules
    ///
    /// #### ESLint
    ///
    /// Fields are sorted by the order they are listed below:
    ///
    /// | Key                           | Rules                                                               |
    /// | ----------------------------- | ------------------------------------------------------------------- |
    /// | env                           |                                                                     |
    /// | parser                        |                                                                     |
    /// | parserOptions                 |                                                                     |
    /// | settings                      |                                                                     |
    /// | plugins                       |                                                                     |
    /// | extends                       |                                                                     |
    /// | rules                         | Group built-in rules first, then plugin rules. Each group is sorted |
    /// | overrides                     | Key order (per item): `files`, `excludedFiles`                      |
    /// | globals                       |                                                                     |
    /// | processor                     |                                                                     |
    /// | noInlineConfig                |                                                                     |
    /// | reportUnusedDisableDirectives |                                                                     |
    ///
    /// #### Git Hooks
    ///
    /// Item order:
    ///
    /// - `applypatch-msg`
    /// - `pre-applypatch`
    /// - `post-applypatch`
    /// - `pre-commit`
    /// - `pre-merge-commit`
    /// - `prepare-commit-msg`
    /// - `commit-msg`
    /// - `post-commit`
    /// - `pre-rebase`
    /// - `post-checkout`
    /// - `post-merge`
    /// - `pre-push`
    /// - `pre-receive`
    /// - `update`
    /// - `proc-receive`
    /// - `post-receive`
    /// - `post-update`
    /// - `reference-transaction`
    /// - `push-to-checkout`
    /// - `pre-auto-gc`
    /// - `post-rewrite`
    /// - `sendemail-validate`
    /// - `fsmonitor-watchman`
    /// - `p4-changelist`
    /// - `p4-prepare-changelist`
    /// - `p4-post-changelist`
    /// - `p4-pre-submit`
    /// - `post-index-change`
    ///
    /// #### Prettier
    ///
    /// Keys are sorted alphabetically except for `overrides`, which is placed last. Keys are also sorted in `overrides` and `overrides.options` items.
    ///
    /// #### Scripts
    ///
    /// Keys are sorted alphabetically except for [pre/post scripts](https://docs.npmjs.com/cli/v6/using-npm/scripts#pre--post-scripts). Those are placed before and after their corresponding base npm script.
    ///
    /// An example - notice how `preinstall` and `postinstall` are placed before and after `install`:
    ///
    /// ```json
    /// {
    ///   "scripts": {
    ///     "build": "",
    ///     "preinstall": "",
    ///     "install": "",
    ///     "postinstall": "",
    ///     "lint": ""
    ///   }
    /// }
    /// ```
    ///
    /// Scripts for which the pre/post order is applied:
    ///
    /// - install
    /// - pack
    /// - prepare
    /// - publish
    /// - restart
    /// - shrinkwrap
    /// - start
    /// - stop
    /// - test
    /// - uninstall
    /// - version
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,expect_diagnostic,file=package.json
    /// {
    ///   "dependencies": {
    ///     "lodash": "^4.0.0"
    ///   },
    ///   "name": "my-package",
    ///   "version": "1.0.0"
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json,file=package.json
    /// {
    ///   "name": "my-package",
    ///   "version": "1.0.0",
    ///   "dependencies": {
    ///     "lodash": "^4.0.0"
    ///   }
    /// }
    /// ```
    pub OrganizePackageJson {
        version: "next",
        name: "organizePackageJson",
        language: "json",
        fix_kind: FixKind::Safe,
    }
}

impl Rule for OrganizePackageJson {
    type Query = Ast<JsonRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let path = ctx.file_path();
        if !is_package_json(path) {
            return None;
        }

        let root = ctx.query();
        let value = root.value().ok()?;
        let object = value.as_json_object_value()?;
        let members = object.json_member_list();

        if is_organized(&members, object) {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let root = ctx.query();
        let value = root.value().ok()?;
        let object = value.as_json_object_value()?;

        Some(RuleDiagnostic::new(
            category!("assist/source/organizePackageJson"),
            object.range(),
            markup! {
                "package.json fields can be organized."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsonRuleAction> {
        let root = ctx.query();
        let value = root.value().ok()?;
        let object = value.as_json_object_value()?;
        let members = object.json_member_list();

        let organized_members = organize_members(&members, object)?;

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(members, organized_members);

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Organize package.json fields."
            },
            mutation,
        ))
    }
}

/// Extract field names from JsonMemberList
fn extract_field_names(members: &JsonMemberList) -> Vec<String> {
    members
        .iter()
        .filter_map(|member| {
            member
                .ok()?
                .name()
                .ok()?
                .inner_string_text()
                .ok()
                .map(|text| text.text().to_string())
        })
        .collect()
}

fn is_organized(members: &JsonMemberList, object: &JsonObjectValue) -> bool {
    let field_names = extract_field_names(members);

    if field_names.is_empty() {
        return true;
    }

    let sorted_names = get_sorted_field_order(&field_names);
    if field_names != sorted_names {
        return false;
    }

    for member in members.iter().filter_map(|m| m.ok()) {
        if let Ok(name) = member.name().and_then(|n| n.inner_string_text()) {
            let field_name = name.text();
            let transformer = get_field_transformer(field_name);

            if transformer != FieldTransformer::None
                && needs_transformation(&member, transformer, object)
            {
                return false;
            }
        }
    }

    true
}

fn needs_transformation(
    member: &JsonMember,
    transformer: FieldTransformer,
    root_object: &JsonObjectValue,
) -> bool {
    let Ok(value) = member.value() else {
        return false;
    };

    sorters::try_transform_field(&value, transformer, root_object).is_some()
}

fn organize_members(
    members: &JsonMemberList,
    root_object: &JsonObjectValue,
) -> Option<JsonMemberList> {
    let member_map: HashMap<String, JsonMember> = members
        .iter()
        .filter_map(|member| {
            let member = member.ok()?;
            let name = member
                .name()
                .ok()?
                .inner_string_text()
                .ok()?
                .text()
                .to_string();
            Some((name, member))
        })
        .collect();

    if member_map.is_empty() {
        return None;
    }

    let field_names = extract_field_names(members);
    let sorted_names = get_sorted_field_order(&field_names);

    let mut elements = Vec::new();
    let mut separators = Vec::new();

    for (i, field_name) in sorted_names.iter().enumerate() {
        if let Some(member) = member_map.get(field_name) {
            let transformed_member = apply_field_transformer(member, field_name, root_object);
            elements.push(transformed_member);

            if i < sorted_names.len() - 1 {
                separators.push(make::token(T![,]));
            }
        }
    }

    Some(make::json_member_list(elements, separators))
}

fn apply_field_transformer(
    member: &JsonMember,
    field_name: &str,
    root_object: &JsonObjectValue,
) -> JsonMember {
    let transformer = get_field_transformer(field_name);

    let Ok(value) = member.value() else {
        return member.clone();
    };

    let transformed_value = sorters::try_transform_field(&value, transformer, root_object)
        .unwrap_or_else(|| value.clone());

    member.clone().with_value(transformed_value)
}

fn get_sorted_field_order(field_names: &[String]) -> Vec<String> {
    let mut known_fields = Vec::new();
    let mut unknown_fields = Vec::new();
    let mut private_fields = Vec::new();

    for name in field_names {
        if name.starts_with('_') {
            private_fields.push(name.clone());
        } else if get_field_index(name).is_some() {
            known_fields.push(name.clone());
        } else {
            unknown_fields.push(name.clone());
        }
    }

    known_fields.sort_by_key(|name| get_field_index(name).unwrap_or(usize::MAX));
    unknown_fields.sort();
    private_fields.sort();

    let mut result = Vec::new();
    result.extend(known_fields);
    result.extend(unknown_fields);
    result.extend(private_fields);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sorted_field_order() {
        let fields = vec![
            "dependencies".to_string(),
            "name".to_string(),
            "version".to_string(),
            "unknown".to_string(),
            "_private".to_string(),
        ];

        let sorted = get_sorted_field_order(&fields);

        assert_eq!(
            sorted,
            vec!["name", "version", "dependencies", "unknown", "_private"]
        );
    }

    #[test]
    fn test_get_sorted_field_order_private_last() {
        let fields = vec![
            "_z".to_string(),
            "name".to_string(),
            "_a".to_string(),
            "version".to_string(),
        ];

        let sorted = get_sorted_field_order(&fields);

        assert_eq!(sorted, vec!["name", "version", "_a", "_z"]);
    }

    #[test]
    fn test_get_sorted_field_order_unknown_alphabetical() {
        let fields = vec![
            "zebra".to_string(),
            "name".to_string(),
            "apple".to_string(),
            "version".to_string(),
        ];

        let sorted = get_sorted_field_order(&fields);

        assert_eq!(sorted, vec!["name", "version", "apple", "zebra"]);
    }
}
