use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, utils::fix_separators,
};
use biome_console::markup;
use biome_diagnostics::category;
use biome_json_factory::make;
use biome_json_syntax::{
    AnyJsonValue, JsonMember, JsonMemberList, JsonRoot, JsonSyntaxKind, JsonSyntaxToken, T,
};
use biome_rowan::{
    AstNode, AstNodeExt, AstSeparatedList, BatchMutationExt, TokenText, TriviaPieceKind,
};

use crate::{MigrationAction, declare_migration};

declare_migration! {
    pub(crate) TailwindOptions {
        version: "next",
        name: "tailwindOptions",
        fix_kind: FixKind::Safe,
    }
}

impl Rule for TailwindOptions {
    // The query is the root, rather than the rule, so that this migration runs
    // before `ruleMover` renames `useSortedClasses`.
    type Query = Ast<JsonRoot>;
    type State = MovedOptions;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.query()
            .syntax()
            .descendants()
            .filter_map(JsonMember::cast)
            .filter(|rule| {
                rule.name()
                    .ok()
                    .and_then(|name| name.inner_string_text())
                    .is_some_and(|name| name.text() == "useSortedClasses")
                    && is_lint_rule(rule)
            })
            .filter_map(|rule| {
                let mut tailwind_options = TailwindOptionValues::default();
                let rule_value = take_tailwind_options(&rule.value().ok()?, &mut tailwind_options)?;
                Some(MovedOptions {
                    rule,
                    rule_value,
                    tailwind_options,
                })
            })
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                category!("migrate"),
                state.rule.name().ok()?.range(),
                markup! {
                    "The "<Emphasis>"attributes"</Emphasis>" and "<Emphasis>"functions"</Emphasis>" options of this rule moved to the top-level "<Emphasis>"tailwind"</Emphasis>" configuration."
                }
                .to_owned(),
            )
            .deprecated(),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<MigrationAction> {
        let rule = &state.rule;
        let root_list = ctx
            .root()
            .value()
            .ok()?
            .as_json_object_value()?
            .json_member_list();
        // Adding `tailwind` changes the root object, which contains the rule,
        // so both changes are made in a single replacement of the root.
        let new_root_list = root_list.clone().replace_node(
            rule.clone(),
            rule.clone().with_value(state.rule_value.clone()),
        )?;
        let new_root_list = with_tailwind_options(&new_root_list, &state.tailwind_options)?;
        let mut mutation = ctx.root().begin();
        mutation.replace_node(root_list, new_root_list);
        Some(MigrationAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Move the options to the "<Emphasis>"tailwind"</Emphasis>" configuration."
            },
            mutation,
        ))
    }
}

pub(crate) struct MovedOptions {
    /// The `useSortedClasses` member.
    rule: JsonMember,
    /// The configuration of the rule without the moved options.
    rule_value: AnyJsonValue,
    tailwind_options: TailwindOptionValues,
}

/// Whether `member` configures a lint rule: it sits in a group of `rules`,
/// at the top level or in an override.
fn is_lint_rule(member: &JsonMember) -> bool {
    member
        .syntax()
        .ancestors()
        .skip(1)
        .filter(|ancestor| ancestor.kind() == JsonSyntaxKind::JSON_MEMBER)
        .nth(1)
        .and_then(JsonMember::cast)
        .and_then(|rules| rules.name().ok()?.inner_string_text())
        .is_some_and(|name| name.text() == "rules")
}

/// The options of `useSortedClasses` that moved to the top-level `tailwind`
/// configuration, as `tailwind` values.
#[derive(Default)]
pub(crate) struct TailwindOptionValues {
    /// The value of `tailwind.attributes`. `useSortedClasses` always checked
    /// `class` and `className`, while `tailwind.attributes` replaces them, so
    /// they're included.
    attributes: Vec<String>,
    /// The value of `tailwind.mergeFunctions`.
    merge_functions: Vec<String>,
    /// The value of `tailwind.variantFunctions`.
    variant_functions: Vec<String>,
}

impl TailwindOptionValues {
    /// Records the `functions` option of `useSortedClasses`. `tailwind`
    /// recognizes a function by the name its callee starts with, so `tw.*`
    /// becomes `tw`. `cva` and `tv` describe styles with an object, which
    /// `tailwind.variantFunctions` is for.
    fn add_function(&mut self, function: &str) {
        let name = function.split('.').next().unwrap_or(function);
        let functions = if matches!(name, "cva" | "tv") {
            &mut self.variant_functions
        } else {
            &mut self.merge_functions
        };
        if !functions.iter().any(|existing| existing == name) {
            functions.push(name.to_string());
        }
    }
}

/// Moves the `attributes` and `functions` options out of the configuration
/// `value` of the rule into `tailwind_options`. Returns the
/// configuration without them, or `None` if it has neither.
fn take_tailwind_options(
    value: &AnyJsonValue,
    tailwind_options: &mut TailwindOptionValues,
) -> Option<AnyJsonValue> {
    let rule = value.as_json_object_value()?;
    let options_member = rule.find_member("options")?;
    let options = options_member.value().ok()?;
    let options = options.as_json_object_value()?;
    let mut moved = false;
    for member in options.json_member_list().iter().flatten() {
        let Some(name) = member.name().ok().and_then(|name| name.inner_string_text()) else {
            continue;
        };
        let strings = || {
            member
                .value()
                .into_iter()
                .flat_map(|value| string_values(&value))
        };
        match name.text() {
            "attributes" => {
                moved = true;
                for attribute in ["class", "className"]
                    .into_iter()
                    .map(str::to_string)
                    .chain(strings().map(|attribute| attribute.to_string()))
                {
                    if !tailwind_options.attributes.contains(&attribute) {
                        tailwind_options.attributes.push(attribute);
                    }
                }
            }
            "functions" => {
                moved = true;
                for function in strings() {
                    tailwind_options.add_function(function.text());
                }
            }
            _ => {}
        }
    }
    if !moved {
        return None;
    }
    let is_moved = |member: &JsonMember| {
        member
            .name()
            .ok()
            .and_then(|name| name.inner_string_text())
            .is_some_and(|name| matches!(name.text(), "attributes" | "functions"))
    };
    let remaining_options =
        rebuild_members(options.json_member_list(), |member| !is_moved(member), [])?;
    let rule_members = if remaining_options.is_empty() {
        rebuild_members(
            rule.json_member_list(),
            |member| member.range() != options_member.range(),
            [],
        )?
    } else {
        let new_options = options_member.clone().with_value(
            options
                .clone()
                .with_json_member_list(remaining_options)
                .into(),
        );
        rule.json_member_list()
            .replace_node(options_member, new_options)?
    };
    Some(rule.clone().with_json_member_list(rule_members).into())
}

/// Adds `tailwind_options` to the root `members`. Older configurations can't
/// have a `tailwind` member, so an existing one comes from moving the options
/// of another `useSortedClasses`, such as one in an override, and the values
/// are combined.
fn with_tailwind_options(
    members: &JsonMemberList,
    tailwind_options: &TailwindOptionValues,
) -> Option<JsonMemberList> {
    let existing = members.iter().flatten().find(|member| {
        member
            .name()
            .ok()
            .and_then(|name| name.inner_string_text())
            .is_some_and(|name| name.text() == "tailwind")
    });
    let existing_object = existing
        .as_ref()
        .and_then(|member| member.value().ok()?.as_json_object_value().cloned());
    let tailwind_members: Vec<_> = [
        ("attributes", &tailwind_options.attributes),
        ("mergeFunctions", &tailwind_options.merge_functions),
        ("variantFunctions", &tailwind_options.variant_functions),
    ]
    .into_iter()
    .filter_map(|(name, values)| {
        let mut combined: Vec<String> = existing_object
            .as_ref()
            .and_then(|object| object.find_member(name)?.value().ok())
            .map(|value| {
                string_values(&value)
                    .map(|value| value.to_string())
                    .collect()
            })
            .unwrap_or_default();
        for value in values {
            if !combined.contains(value) {
                combined.push(value.clone());
            }
        }
        (!combined.is_empty()).then(|| string_array_member(name, &combined))
    })
    .collect();
    let separators = (1..tailwind_members.len()).map(|_| comma());
    // The new member goes on its own line, like the first member of the root.
    let leading_member = existing.clone().or_else(|| members.iter().flatten().next());
    let mut name = make::json_string_literal("tailwind");
    if let Some(first_token) = leading_member.and_then(|member| member.syntax().first_token()) {
        name = name.with_leading_trivia_pieces(first_token.leading_trivia().pieces());
    }
    let tailwind = make::json_member(
        make::json_member_name(name).into(),
        make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        make::json_object_value(
            make::token(T!['{']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            make::json_member_list(tailwind_members, separators),
            make::token(T!['}']).with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]),
        )
        .into(),
    );
    match existing {
        Some(existing) => members.clone().replace_node(existing, tailwind),
        None => rebuild_members(members.clone(), |_| true, [tailwind]),
    }
}

/// Returns the strings of `value` if it's an array, such as `["clsx", "cn"]`.
fn string_values(value: &AnyJsonValue) -> impl Iterator<Item = TokenText> + use<> {
    value
        .as_json_array_value()
        .cloned()
        .into_iter()
        .flat_map(|array| array.elements().iter().flatten().collect::<Vec<_>>())
        .filter_map(|element| element.as_json_string_value()?.inner_string_text().ok())
}

/// Rebuilds `members` with the members `keep` accepts, followed by `added`.
fn rebuild_members(
    members: JsonMemberList,
    keep: impl Fn(&JsonMember) -> bool,
    added: impl IntoIterator<Item = JsonMember>,
) -> Option<JsonMemberList> {
    let needs_last_separator = members.trailing_separator().is_some();
    let mut elements = Vec::with_capacity(members.len());
    for element in members.elements() {
        let node = element.node.ok()?;
        if keep(&node) {
            elements.push((node, element.trailing_separator.ok()?));
        }
    }
    elements.extend(added.into_iter().map(|member| (member, None)));
    fix_separators(
        elements
            .iter_mut()
            .map(|(node, separator)| (node, separator)),
        needs_last_separator,
        || make::token(T![,]),
    );
    let separators: Vec<_> = elements
        .iter_mut()
        .filter_map(|(_, separator)| separator.take())
        .collect();
    Some(make::json_member_list(
        elements.into_iter().map(|(node, _)| node),
        separators,
    ))
}

/// Creates a member such as `"attributes": ["class", "className"]`.
fn string_array_member(name: &str, values: &[String]) -> JsonMember {
    let elements: Vec<_> = values
        .iter()
        .map(|value| {
            AnyJsonValue::JsonStringValue(make::json_string_value(make::json_string_literal(value)))
        })
        .collect();
    let separators = (1..elements.len()).map(|_| comma());
    make::json_member(
        make::json_member_name(make::json_string_literal(name)).into(),
        make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        make::json_array_value(
            make::token(T!['[']),
            make::json_array_element_list(elements, separators),
            make::token(T![']']),
        )
        .into(),
    )
}

fn comma() -> JsonSyntaxToken {
    make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])
}
