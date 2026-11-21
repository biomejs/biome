use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsNamedImportSpecifier, JsCallExpression,
    JsObjectExpression, JsStaticMemberExpression, binding_ext::AnyJsBindingDeclaration,
    inner_string_text, static_value::StaticValue,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::no_convex_duplicate_indexes::NoConvexDuplicateIndexesOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow Convex database indexes whose fields are the first fields of another index on the same table.
    ///
    /// A query can use a Convex index to filter on any number of its first fields, so an index on
    /// `["author", "channel"]` already answers every query that an index on `["author"]` can. Every
    /// index slows down writes to the table and takes up storage, so the shorter index only adds
    /// cost. Indexes with identical fields are reported too.
    ///
    /// Staged indexes (`staged: true`) are ignored: staging the longer index while the shorter one
    /// is still in use is how you replace one with the other.
    ///
    /// Convex appends `_creationTime` to every index. If you need results sorted by the shorter
    /// index's fields followed by `_creationTime`, keep both and suppress this rule on the shorter one.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { defineTable } from "convex/server";
    /// import { v } from "convex/values";
    ///
    /// defineTable({ author: v.string(), channel: v.string() })
    ///     .index("by_author", ["author"])
    ///     .index("by_author_and_channel", ["author", "channel"]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { defineTable } from "convex/server";
    /// import { v } from "convex/values";
    ///
    /// defineTable({ author: v.string() })
    ///     .index("by_author", ["author"])
    ///     .index("by_writer", ["author"]);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { defineTable } from "convex/server";
    /// import { v } from "convex/values";
    ///
    /// // Neither index starts with all the fields of the other one.
    /// defineTable({ author: v.string(), channel: v.string() })
    ///     .index("by_channel", ["channel"])
    ///     .index("by_author_and_channel", ["author", "channel"]);
    ///
    /// // The longer index is staged, so the shorter one is still needed until it's ready.
    /// defineTable({ author: v.string(), channel: v.string() })
    ///     .index("by_author", ["author"])
    ///     .index("by_author_and_channel", { fields: ["author", "channel"], staged: true });
    ///
    /// // To query by `author` alone, use the longer index and filter only on its first field.
    /// await ctx.db
    ///     .query("messages")
    ///     .withIndex("by_author_and_channel", (q) => q.eq("author", author))
    ///     .collect();
    /// ```
    ///
    pub NoConvexDuplicateIndexes {
        version: "next",
        name: "noConvexDuplicateIndexes",
        language: "js",
        sources: &[RuleSource::EslintConvex("no-duplicate-indexes").same()],
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Convex],
    }
}

impl Rule for NoConvexDuplicateIndexes {
    type Query = Semantic<JsCallExpression>;
    type State = RedundantIndex;
    type Signals = Box<[Self::State]>;
    type Options = NoConvexDuplicateIndexesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        if !is_define_table_call(call, ctx.model()) {
            return Box::default();
        }

        let indexes = chained_indexes(call);
        indexes
            .iter()
            .enumerate()
            .filter_map(|(i, redundant)| {
                // Each redundant index is reported once, against the first index that covers it.
                // When two indexes have the same fields, only the later one is reported.
                let (_, covering) = indexes.iter().enumerate().find(|(j, other)| {
                    *j != i
                        && is_prefix(&redundant.fields, &other.fields)
                        && (other.fields.len() > redundant.fields.len() || *j < i)
                })?;
                Some(RedundantIndex {
                    redundant: redundant.definition.clone(),
                    covering: covering.definition.clone(),
                    next_field: covering.fields.get(redundant.fields.len()).cloned(),
                })
            })
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = state.redundant.name.text();
        let covering = state.covering.name.text();
        let range = state.redundant.range()?;
        let covering_range = state.covering.range()?;
        Some(match &state.next_field {
            Some(next_field) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The index "<Emphasis>{name}</Emphasis>" is redundant because the index "<Emphasis>{covering}</Emphasis>" starts with the same fields."
                },
            )
            .detail(
                covering_range,
                markup! {
                    "The index "<Emphasis>{covering}</Emphasis>" is defined here."
                },
            )
            .note(markup! {
                "Every index slows down inserts and updates and takes up storage. Queries that use "<Emphasis>{name}</Emphasis>" find the same documents with "<Emphasis>{covering}</Emphasis>", so delete "<Emphasis>{name}</Emphasis>" and use "<Emphasis>{covering}</Emphasis>" instead."
            })
            .note(markup! {
                "Keep both indexes only if you need the documents in a different order: "<Emphasis>{name}</Emphasis>" sorts matching documents by "<Emphasis>"_creationTime"</Emphasis>", while "<Emphasis>{covering}</Emphasis>" sorts them by "<Emphasis>{next_field.text()}</Emphasis>"."
            }),
            None => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The index "<Emphasis>{name}</Emphasis>" has the same fields as the index "<Emphasis>{covering}</Emphasis>"."
                },
            )
            .detail(
                covering_range,
                markup! {
                    "The index "<Emphasis>{covering}</Emphasis>" is defined here."
                },
            )
            .note(markup! {
                "Both indexes find the same documents in the same order, but each one slows down inserts and updates and takes up storage. Delete "<Emphasis>{name}</Emphasis>" and use "<Emphasis>{covering}</Emphasis>" instead."
            }),
        })
    }
}

pub struct RedundantIndex {
    /// The index that the rule reports.
    redundant: IndexDefinition,
    /// An index that answers every query that [Self::redundant] answers.
    covering: IndexDefinition,
    /// The first field of [Self::covering] that [Self::redundant] doesn't have,
    /// or `None` when both indexes have the same fields.
    next_field: Option<StaticValue>,
}

#[derive(Clone)]
pub struct IndexDefinition {
    /// The `.index(...)` call that defines the index.
    call: JsCallExpression,
    /// The name of the index, the first argument of the call.
    name: StaticValue,
}

impl IndexDefinition {
    /// Returns the range from `index` to the end of the call in `.index(...)`.
    fn range(&self) -> Option<TextRange> {
        let callee = self.call.callee().ok()?;
        let member = callee.as_js_static_member_expression()?.member().ok()?;
        Some(TextRange::new(
            member.range().start(),
            self.call.range().end(),
        ))
    }
}

/// An index whose name and fields are known without running the code.
struct ParsedIndex {
    definition: IndexDefinition,
    /// The fields of the index, in order. Every field is a string.
    fields: Vec<StaticValue>,
}

/// Returns `true` if `call` calls `defineTable` imported by name, possibly renamed, or as a member
/// of a namespace import. The module it's imported from isn't checked, so a module that
/// re-exports `defineTable` from `convex/server` is also supported.
fn is_define_table_call(call: &JsCallExpression, model: &SemanticModel) -> bool {
    let Ok(callee) = call.callee() else {
        return false;
    };
    match callee.omit_parentheses() {
        AnyJsExpression::JsIdentifierExpression(identifier) => identifier
            .name()
            .ok()
            .and_then(|name| model.binding(&name))
            .and_then(|binding| {
                let specifier: AnyJsNamedImportSpecifier = match binding.tree().declaration()? {
                    AnyJsBindingDeclaration::JsNamedImportSpecifier(specifier) => specifier.into(),
                    AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(specifier) => {
                        specifier.into()
                    }
                    _ => return None,
                };
                specifier.imported_name()
            })
            .is_some_and(|name| inner_string_text(&name) == "defineTable"),
        AnyJsExpression::JsStaticMemberExpression(member) => {
            member
                .member()
                .ok()
                .and_then(|member| member.as_js_name()?.value_token().ok())
                .is_some_and(|name| name.text_trimmed() == "defineTable")
                && member
                    .object()
                    .ok()
                    .and_then(|object| object.omit_parentheses().as_js_reference_identifier())
                    .and_then(|object| model.binding(&object))
                    .is_some_and(|binding| {
                        matches!(
                            binding.tree().declaration(),
                            Some(AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_))
                        )
                    })
        }
        _ => false,
    }
}

/// Returns the indexes defined by the `.index(...)` calls chained onto `define_table`, in source
/// order. Other calls in the chain, such as `.searchIndex(...)`, are followed but don't define
/// database indexes.
///
/// A table stored in a variable and indexed in a later statement isn't followed.
fn chained_indexes(define_table: &JsCallExpression) -> Vec<ParsedIndex> {
    let mut indexes = Vec::new();
    let mut current = AnyJsExpression::from(define_table.clone());
    while let Some((call, member)) = chained_call(&current) {
        if let Some(index) = parse_index_call(&call, &member) {
            indexes.push(index);
        }
        current = AnyJsExpression::from(call);
    }
    indexes
}

/// Returns the call `expression.method(...)` that continues a builder chain, along with the
/// `.method` member of its callee.
fn chained_call(
    expression: &AnyJsExpression,
) -> Option<(JsCallExpression, JsStaticMemberExpression)> {
    let outer = expression.outer_expression()?;
    let member = outer.parent::<JsStaticMemberExpression>()?;
    let call = member.parent::<JsCallExpression>()?;
    if call.callee().ok()?.syntax() != member.syntax() {
        return None;
    }
    Some((call, member))
}

/// Returns the index defined by `call` if `member` is `.index`, its name and fields are plain
/// strings, and it isn't staged.
fn parse_index_call(
    call: &JsCallExpression,
    member: &JsStaticMemberExpression,
) -> Option<ParsedIndex> {
    if member
        .member()
        .ok()?
        .as_js_name()?
        .value_token()
        .ok()?
        .text_trimmed()
        != "index"
    {
        return None;
    }

    let arguments = call.arguments().ok()?.args();
    let mut arguments = arguments.iter();
    let name = static_string(arguments.next()?.ok()?.as_any_js_expression()?)?;
    let mut fields = arguments
        .next()?
        .ok()?
        .as_any_js_expression()?
        .clone()
        .omit_type_wrappers()?;
    if let AnyJsExpression::JsObjectExpression(config) = &fields {
        fields = unstaged_config_fields(config)?;
    }
    let AnyJsExpression::JsArrayExpression(fields) = fields else {
        return None;
    };
    let fields = fields
        .elements()
        .iter()
        .map(|element| match element.ok()? {
            AnyJsArrayElement::AnyJsExpression(field) => static_string(&field),
            AnyJsArrayElement::JsArrayHole(_) | AnyJsArrayElement::JsSpread(_) => None,
        })
        .collect::<Option<Vec<_>>>()?;
    if fields.is_empty() {
        return None;
    }

    Some(ParsedIndex {
        definition: IndexDefinition {
            call: call.clone(),
            name,
        },
        fields,
    })
}

/// Returns the value of `fields` in an index config such as `{ fields: [...], staged: false }`.
///
/// Returns `None` if the index is staged, or might be: when `staged` is anything other than
/// `false`, or when a spread or a computed key could set `staged` or `fields`.
fn unstaged_config_fields(config: &JsObjectExpression) -> Option<AnyJsExpression> {
    let mut staged = None;
    let mut fields = None;
    // When a key appears more than once, the last one wins, as it does when the code runs.
    for member in config.members() {
        let member = member.ok()?;
        match member.name()?.text() {
            "staged" => staged = Some(member),
            "fields" => fields = Some(member),
            _ => {}
        }
    }

    if let Some(staged) = staged {
        let value = staged
            .as_js_property_object_member()?
            .value()
            .ok()?
            .omit_type_wrappers()?;
        if !matches!(value.as_static_value(), Some(StaticValue::Boolean(token)) if token.text_trimmed() == "false")
        {
            return None;
        }
    }

    fields?
        .as_js_property_object_member()?
        .value()
        .ok()?
        .omit_type_wrappers()
}

/// Returns the value of `expression` if it's a string literal or a template literal without
/// substitutions, ignoring parentheses and type assertions such as `as const`.
fn static_string(expression: &AnyJsExpression) -> Option<StaticValue> {
    let value = expression.clone().omit_type_wrappers()?.as_static_value()?;
    value.as_string_constant().is_some().then_some(value)
}

/// Returns `true` if `fields` are the first fields of `other`, in the same order.
fn is_prefix(fields: &[StaticValue], other: &[StaticValue]) -> bool {
    fields.len() <= other.len()
        && fields
            .iter()
            .zip(other)
            .all(|(field, other)| field.text() == other.text())
}
