use crate::JsonRuleAction;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_json_factory::make::{
    json_boolean_value, json_member, json_member_list, json_member_name, json_string_literal,
    json_string_value, token,
};
use biome_json_syntax::{
    AnyJsonMemberName, AnyJsonValue, JsonMember, JsonObjectValue, T, inner_string_text,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TextRange};
use biome_rule_options::no_quickfix_biome::NoQuickfixBiomeOptions;

declare_lint_rule! {
    /// Disallow `quickfix.biome` in supported editor settings.
    ///
    /// `quickfix.biome` asks the editor to apply independent Biome fixes together. When fixes edit
    /// the same source text, their changes can overlap and produce invalid code. Use
    /// `source.fixAll.biome` instead.
    ///
    /// The rule checks Visual Studio Code and Zed settings files whose paths end with:
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
    /// ### `additionalPaths`
    ///
    /// Adds settings-file path suffixes for other editors. For example, adding
    /// `".myEditor/file.json"` checks every file whose path ends with that value. Defaults to an
    /// empty list.
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
                let value = name.value_token()?;
                if inner_string_text(&value) == "quickfix.biome" {
                    return Some(name.range());
                }
            }
        }

        for default_path in options.additional_paths.iter() {
            if path.ends_with(default_path) {
                let name = node.name().ok()?;
                let value = name.value_token()?;
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
                .ok()
                .and_then(|name| name.value_token())
                .is_some_and(|token| inner_string_text(&token) == "source.fixAll.biome")
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
                AnyJsonMemberName::JsonMemberName(json_member_name(json_string_literal(
                    "source.fixAll.biome",
                ))),
                token(T![:]).with_trailing_space(),
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
