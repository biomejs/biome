use crate::JsonRuleAction;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_json_factory::make::{
    json_boolean_value, json_member, json_member_list, json_member_name, json_string_literal,
    json_string_value, token,
};
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonObjectValue, T, inner_string_text};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TextRange, TriviaPieceKind};
use biome_rule_options::no_quickfix_biome::NoQuickfixBiomeOptions;

declare_lint_rule! {
    /// Disallow the use if `quickfix.biome` inside editor settings file.
    ///
    /// The code action `quickfix.biome` can be harmful because it instructs the editors
    /// to apply the code fix of lint rules and code actions atomically. If multiple rules or
    /// actions apply a code fix to the same code span, the editor will emit invalid code.
    ///
    /// The rule targets specifically VSCode settings and Zed settings. Specifically, paths that end with:
    /// - `.vscode/settings.json`
    /// - `Code/User/settings.json`
    /// - `.zed/settings.json`
    /// - `zed/settings.json`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,ignore
    /// {
    ///     "quickfix.biome": "explicit"
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json,ignore
    /// {
    ///     "source.fixAll.biome": "explicit"
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// The following options are available
    ///
    /// ### `additionalPaths`
    ///
    /// It's possible to specify a list of JSON paths, if your editor uses a JSON file setting that isn't supported natively by the rule.
    ///
    /// If your editor uses, for example, a file called `.myEditor/file.json`, you can add `".myEditor/file.json"` to the list.
    /// **The rule checks if the file ends with the given paths**.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "additionalPaths": [".myEditor/file.json"]
    ///     }
    /// }
    /// ```
    ///
    pub NoQuickfixBiome {
        version: "2.1.3",
        name: "noQuickfixBiome",
        language: "json",
        recommended: true,
        fix_kind: FixKind::Safe,
    }
}

const DEFAULT_PATHS: &[&str] = &[
    ".vscode/settings.json",
    "Code/User/settings.json",
    ".zed/settings.json",
    "zed/settings.json",
];

impl Rule for NoQuickfixBiome {
    type Query = Ast<JsonMember>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoQuickfixBiomeOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let path = ctx.file_path();
        let options = ctx.options();
        for default_path in DEFAULT_PATHS {
            if path.ends_with(default_path) {
                let name = node.name().ok()?;
                let value = name.value_token().ok()?;
                if inner_string_text(&value) == "quickfix.biome" {
                    return Some(name.range());
                }
            }
        }

        for default_path in options.additional_paths.iter() {
            if path.ends_with(default_path) {
                let name = node.name().ok()?;
                let value = name.value_token().ok()?;
                if inner_string_text(&value) == "quickfix.biome" {
                    return Some(name.range());
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "The use of "<Emphasis>"quickfix.biome"</Emphasis>" is deprecated."
                },
            )
            .note(markup! {
                    "The code action "<Emphasis>"quickfix.biome"</Emphasis>" applies the code fix of rules and actions without being aware of each other. This might cause the emission of malformed code, especially if the code fixes are applied to the same lines of code."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsonRuleAction> {
        let quick_fix_node = ctx.query();
        let path = ctx.file_path();
        let mut mutation = ctx.root().begin();
        let parent = quick_fix_node
            .syntax()
            .ancestors()
            .find_map(JsonObjectValue::cast)?;

        let parent_list = parent.json_member_list();
        let has_fix_all = parent_list.iter().flatten().any(|member| {
            member
                .name()
                .map(|name| {
                    name.value_token()
                        .map(|token| inner_string_text(&token) == "source.fixAll.biome")
                        .unwrap_or(false)
                })
                .unwrap_or(false)
        });

        let new_list = parent_list
            .iter()
            .flatten()
            .filter(|node| node != quick_fix_node)
            .collect::<Vec<_>>();
        if has_fix_all {
            let mut separators = vec![];

            for _ in 0..(new_list.len() - 1) {
                separators.push(token(T![,]));
            }

            let new_list = json_member_list(new_list, separators);
            mutation.replace_node(parent_list, new_list);
            Some(JsonRuleAction::new(
                ctx.metadata().action_category(ctx.category(), ctx.group()),
                ctx.metadata().applicability(),
                markup! {
                    "Remove the code action."
                },
                mutation,
            ))
        } else {
            let mut new_list = vec![];
            new_list.push(json_member(
                json_member_name(json_string_literal("source.fixAll.biome")),
                token(T![:]).with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
                if path.as_str().contains("zed") || path.as_str().contains(".zed") {
                    AnyJsonValue::JsonBooleanValue(json_boolean_value(token(T![true])))
                } else {
                    AnyJsonValue::JsonStringValue(json_string_value(json_string_literal(
                        "explicit",
                    )))
                },
            ));
            let mut separators = vec![];

            for _ in 0..(new_list.len() - 1) {
                separators.push(token(T![,]));
            }

            let new_list = json_member_list(new_list, separators);
            mutation.replace_node(parent_list, new_list);
            Some(JsonRuleAction::new(
                ctx.metadata().action_category(ctx.category(), ctx.group()),
                ctx.metadata().applicability(),
                markup! {
                    "Remove the code action."
                },
                mutation,
            ))
        }
    }
}
