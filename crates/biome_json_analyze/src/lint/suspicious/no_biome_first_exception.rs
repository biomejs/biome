use crate::{ConfigSource, JsonRuleAction};
use biome_analyze::{FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_json_factory::make::{
    json_array_element_list, json_string_literal, json_string_value, token,
};
use biome_json_syntax::{AnyJsonValue, JsonArrayElementList, JsonMember, JsonRoot, T};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TextRange};
use biome_rule_options::no_biome_first_exception::NoBiomeFirstExceptionOptions;

declare_lint_rule! {
    /// Prevents the misuse of glob patterns inside the `files.includes` field.
    ///
    /// ## Leading of negated patterns
    /// If the first pattern of `files.includes` starts with the leading `!`, Biome won't have any file to crawl. Generally,
    /// it is a good practice to declare the files/folders to include first, and then the files/folder to ignore.
    ///
    /// Check the [official documentation](https://biomejs.dev/guides/configure-biome/#exclude-files-via-configuration) for more examples.
    ///
    /// ### Examples
    ///
    /// #### Invalid
    ///
    /// ```json,ignore
    /// {
    ///     "files": {
    ///         "includes": ["!dist"]
    ///     }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```json,ignore
    /// {
    ///     "files": {
    ///         "includes": ["src/**", "!dist"]
    ///     }
    /// }
    /// ```
    ///
    /// ## Leading with catch-all `**`
    ///
    /// If the user configuration file extends from other sources (other configuration files or libraries), and those files contain the catch-all glob `**` in `files.includes`,
    /// the rule will trigger a violation if also the user configuration file has a `**`.
    ///
    /// #### Invalid
    ///
    /// ```jsonc,ignore
    /// // biome.json
    /// {
    ///     "extends": ["./base.json"],
    ///     "files": {
    ///         "includes": ["**", "!**/test"]
    ///     }
    /// }
    /// ```
    ///
    /// ```jsonc,ignore
    /// // base.json
    /// {
    ///     "files": {
    ///         "includes": ["**", "!**/dist"]
    ///     }
    /// }
    /// ```
    ///
    pub NoBiomeFirstException {
        version: "2.2.0",
        name: "noBiomeFirstException",
        language: "json",
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoBiomeFirstException {
    type Query = ConfigSource<JsonRoot>;
    type State = ViolationKind;
    type Signals = Option<Self::State>;
    type Options = NoBiomeFirstExceptionOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let root = ctx.query();
        let file_path = ctx.file_path();

        // we check if and extended package starts with `**`
        #[cfg(feature = "configuration")]
        let extends_starts_with_catch_all = ctx.extends().is_some_and(|mut extends| {
            extends.any(|c| {
                c.files
                    .as_ref()
                    .and_then(|files| files.includes.as_deref())
                    .is_some_and(|globs| globs.first().is_some_and(|glob| glob.as_str() == "**"))
            })
        });
        #[cfg(not(feature = "configuration"))]
        let extends_starts_with_catch_all = false;
        // we use ends_with so it works only during testing
        if !file_path
            .file_name()
            .is_some_and(|file_name| file_name.ends_with("biome.json"))
            && !file_path
                .file_name()
                .is_some_and(|file_name| file_name.ends_with("biome.jsonc"))
        {
            return None;
        }

        let value = root.value().ok()?;
        let value = value.as_json_object_value()?;

        let includes = value
            .find_member("files")
            .and_then(|files| files.value().ok())
            .and_then(|value| value.as_json_object_value().cloned())?
            .find_member("includes")?;
        let extends = value.find_member("extends");

        let extends_root = extends.is_some_and(|extends| extends.value().ok().is_some());

        let includes_first_value = includes
            .value()
            .ok()?
            .as_json_array_value()?
            .elements()
            .iter()
            .flatten()
            .next()
            .and_then(|element| element.as_json_string_value().cloned());

        if extends_root && let Some(includes_first_value) = includes_first_value {
            return if includes_first_value.inner_string_text().ok()?.text() == "**"
                && extends_starts_with_catch_all
            {
                let includes_value = includes.value().ok()?;
                let includes_value = includes_value.as_json_array_value()?;
                let includes_value = includes_value.elements();
                Some(ViolationKind::ExtendedStar(
                    includes_first_value.range(),
                    includes_value,
                ))
            } else {
                None
            };
        }

        let value = includes.value().ok()?;
        let value = value.as_json_array_value()?;
        if let Some(element) = value.elements().first() {
            let element = element.ok()?;
            let string_value = element.as_json_string_value()?;

            if string_value
                .inner_string_text()
                .ok()?
                .text()
                .starts_with('!')
            {
                return Some(ViolationKind::NoStar(string_value.range(), includes));
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            ViolationKind::ExtendedStar(range, _) => {
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        range,
                        markup! {
                    "Biome detected that at least one of your extended packages starts with "<Emphasis>"**"</Emphasis>"."
                },
                    )
                        .note(markup! {
                    "When an extended package uses a catch-all, adding an additional catch-all could lead to possible issues."
            }),
                )
            }
            ViolationKind::NoStar(range, _) => {
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        range,
                        markup! {
                    "Incorrect usage of the exception detected."
                },
                    )
                        .note(markup! {
                    "Having a pattern that starts with `!` as first item will cause Biome to match no files."
            }),
                )
            }
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsonRuleAction> {
        let mut mutation = ctx.root().begin();

        match state {
            ViolationKind::ExtendedStar(_, includes_value) => {
                let mut iter = includes_value.iter();
                // remove the first value
                iter.next();
                let new_list = iter.flatten().collect::<Vec<_>>();
                let mut separator_list = vec![];
                if new_list.len() > 1 {
                    for _ in 0..new_list.len() - 1 {
                        separator_list.push(token(T![,]))
                    }
                }

                mutation.replace_node(
                    includes_value.clone(),
                    json_array_element_list(new_list, separator_list),
                );
                Some(JsonRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! {
                        "Remove "<Emphasis>"**"</Emphasis>" from your list."
                    },
                    mutation,
                ))
            }
            ViolationKind::NoStar(_, includes) => {
                let old_list = includes.value().ok()?.as_json_array_value()?.elements();
                let list = old_list.iter().flatten().collect::<Vec<_>>();
                let mut new_list = vec![AnyJsonValue::JsonStringValue(json_string_value(
                    json_string_literal("**"),
                ))];

                new_list.extend(list);
                let mut separators = vec![];

                for _ in 0..new_list.len() - 1 {
                    separators.push(token(T![,]))
                }

                let new_list = json_array_element_list(new_list, separators);

                mutation.replace_node(old_list, new_list);

                Some(JsonRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! {
                        "Add the pattern "<Emphasis>"**"</Emphasis>" at the beginning of the list."
                    },
                    mutation,
                ))
            }
        }
    }
}

pub enum ViolationKind {
    ExtendedStar(TextRange, JsonArrayElementList),
    NoStar(TextRange, JsonMember),
}
