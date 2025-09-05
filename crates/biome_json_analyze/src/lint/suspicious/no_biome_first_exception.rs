use crate::JsonRuleAction;
use crate::utils::matches_parent_object;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_json_factory::make::{
    json_array_element_list, json_string_literal, json_string_value, token,
};
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonStringValue, T};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::no_biome_first_exception::NoBiomeFirstExceptionOptions;

declare_lint_rule! {
    /// Prevents the use of the `!` pattern in the first position of `files.includes` in the configuration file.
    ///
    /// If the first pattern of `files.includes` starts with the leading `!`, Biome won't have any file to crawl. Generally,
    /// it is a good practice to declare the files/folders to include first, and then the files/folder to ignore.
    ///
    /// Check the [official documentation](https://biomejs.dev/guides/configure-biome/#exclude-files-via-configuration) for more examples.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,ignore
    /// {
    ///     "files": {
    ///         "includes": ["!dist"]
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json,ignore
    /// {
    ///     "files": {
    ///         "includes": ["src/**", "!dist"]
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
    type Query = Ast<JsonMember>;
    type State = JsonStringValue;
    type Signals = Option<Self::State>;
    type Options = NoBiomeFirstExceptionOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let file_path = ctx.file_path();
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

        let name = node.name().ok()?;

        if name.inner_string_text().ok()?.text() != "includes" {
            return None;
        }

        if !matches_parent_object(node, "files") {
            return None;
        }

        let root = ctx.root();

        let extends_root = root
            .value()
            .ok()
            .and_then(|value| value.as_json_object_value().cloned())
            .and_then(|object| object.find_member("extends"))
            .is_some_and(|extends| extends.value().ok().is_some());

        if extends_root {
            return None;
        }

        let value = node.value().ok()?;
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
                return Some(string_value.clone());
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "Incorrect usage of the exception detected."
                },
            )
            .note(markup! {
                    "Having a pattern that starts with `!` as first item will cause Biome to match no files."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsonRuleAction> {
        let mut mutation = ctx.root().begin();
        let old_list = ctx.query().value().ok()?.as_json_array_value()?.elements();
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
                "Add the patter "<Emphasis>"**"</Emphasis>" at the beginning of the list."
            },
            mutation,
        ))
    }
}
