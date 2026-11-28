use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsComputedMember, AnyJsExpression, AnyJsMemberExpression,
    JsAssignmentExpression, JsAssignmentOperator, JsExpressionStatement, JsUnaryExpression,
    JsUnaryOperator, T, binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, BatchMutationExt, Direction};
use biome_rule_options::no_collection_bracket_access::NoCollectionBracketAccessOptions;

use crate::{
    JsRuleAction,
    ast_utils::starts_unterminated_statement,
    services::semantic::Semantic,
    utils::builtin_collection::{CollectionKind, collection_kind, is_global_reference},
};

declare_lint_rule! {
    /// Disallow using square brackets to read, write, or delete the entries of a `Map`, `Set`, `WeakMap`, or `WeakSet`.
    ///
    /// A `Map`, `Set`, `WeakMap`, or `WeakSet` doesn't store its entries as properties.
    /// Square brackets, as in `map["key"]`, read or change a property of the collection object,
    /// so they don't find, add, or remove any entry. This is almost always a mistake.
    ///
    /// Use the methods of the collection instead:
    ///
    /// - to read: `get()` and `has()` for a `Map`, `has()` for a `Set`;
    /// - to write: `set()` for a `Map`, `add()` for a `Set`;
    /// - to delete: `delete()`.
    ///
    /// The rule only reports values that it knows are one of these collections: values
    /// created with `new Map()`, `new Set()`, `new WeakMap()`, or `new WeakSet()`, `const`
    /// variables that hold such a value, and variables or parameters whose type is one of
    /// these collections.
    ///
    /// Square brackets that access a real property or method of the collection, such as
    /// `map["size"]` or `map[Symbol.iterator]`, aren't reported.
    ///
    /// The fix replaces `map[key] = value;` with `map.set(key, value);`, and `delete map[key];`
    /// with `map.delete(key);`. It's unsafe, because the fixed code changes the entries of the
    /// collection instead of its properties.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const map = new Map();
    /// map["foo"] = "bar";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const map = new Map();
    /// const value = map["foo"];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const map = new Map();
    /// delete map["foo"];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const set = new Set();
    /// if (set["foo"]) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const map = new Map();
    /// map.set("foo", "bar");
    /// const value = map.get("foo");
    /// map.delete("foo");
    /// const count = map["size"];
    ///
    /// const set = new Set();
    /// if (set.has("foo")) {}
    /// ```
    ///
    pub NoCollectionBracketAccess {
        version: "next",
        name: "noCollectionBracketAccess",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-collection-bracket-access").same()],
        recommended: false,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

/// A bracket access on a collection, and what it does.
pub struct CollectionBracketAccess {
    kind: CollectionKind,
    access: AccessKind,
}

impl Rule for NoCollectionBracketAccess {
    type Query = Semantic<AnyJsComputedMember>;
    type State = CollectionBracketAccess;
    type Signals = Option<Self::State>;
    type Options = NoCollectionBracketAccessOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let kind = collection_kind(&node.object().ok()?, model)?;
        if is_collection_member(kind, node.member().ok()?, model) {
            return None;
        }
        let access = match node {
            AnyJsComputedMember::JsComputedMemberAssignment(_) => AccessKind::Write,
            AnyJsComputedMember::JsComputedMemberExpression(member) => {
                if deleting_expression(&member.clone().into()).is_some() {
                    AccessKind::Delete
                } else {
                    AccessKind::Read
                }
            }
        };
        Some(CollectionBracketAccess { kind, access })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let kind = state.kind;
        let is_map = matches!(kind, CollectionKind::Map | CollectionKind::WeakMap);
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "{state.access}" a property of the "<Emphasis>{kind}</Emphasis>", not one of its entries."
                },
            )
            .note(markup! {
                "A "<Emphasis>{kind}</Emphasis>" doesn't store its entries as properties, so square brackets don't find or change them."
            })
            .note(match (state.access, is_map) {
                (AccessKind::Read, true) => markup! {
                    "Use "<Emphasis>"get()"</Emphasis>" to read an entry, or "<Emphasis>"has()"</Emphasis>" to check whether it exists."
                },
                (AccessKind::Read, false) => markup! {
                    "Use "<Emphasis>"has()"</Emphasis>" to check whether it contains a value."
                },
                (AccessKind::Write, true) => markup! {
                    "Use "<Emphasis>"set()"</Emphasis>" to add or change an entry."
                },
                (AccessKind::Write, false) => markup! {
                    "Use "<Emphasis>"add()"</Emphasis>" to add a value."
                },
                (AccessKind::Delete, _) => markup! {
                    "Use "<Emphasis>"delete()"</Emphasis>" to remove an entry."
                },
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let (replaced, method, value) = match state.access {
            AccessKind::Read => return None,
            AccessKind::Write => {
                if !matches!(state.kind, CollectionKind::Map | CollectionKind::WeakMap) {
                    return None;
                }
                let assignment = node.parent::<JsAssignmentExpression>()?;
                if assignment.operator().ok()? != JsAssignmentOperator::Assign {
                    return None;
                }
                let value = assignment.right().ok()?;
                (AnyJsExpression::from(assignment), "set", Some(value))
            }
            AccessKind::Delete => {
                let AnyJsComputedMember::JsComputedMemberExpression(member) = node else {
                    return None;
                };
                if member.is_optional_chain() {
                    return None;
                }
                // `delete (map[key])` isn't fixed, so the fix doesn't have to decide where the
                // comments inside the parentheses go.
                let unary = member.parent::<JsUnaryExpression>()?;
                (AnyJsExpression::from(unary), "delete", None)
            }
        };
        if replaced.parent::<JsExpressionStatement>().is_none() || has_inner_comments(&replaced) {
            return None;
        }

        let object = node.object().ok()?;
        if !is_simple_object(&object) {
            return None;
        }
        // The fixed statement starts with the object instead of `delete`. A statement that
        // starts with `(` may continue the previous one when it doesn't end with `;`.
        if object
            .syntax()
            .first_token()
            .is_some_and(|token| token.kind() == T!['('])
            && starts_unterminated_statement(&replaced)
        {
            return None;
        }

        let key = node.member().ok()?.trim_comments_and_trivia()?;
        // A sequence such as `a, b` must keep its parentheses, or the call would receive
        // each of its parts as a separate argument.
        let key = if key.as_js_sequence_expression().is_some() {
            make::parenthesized(key).into()
        } else {
            key
        };
        let mut arguments = vec![AnyJsCallArgument::AnyJsExpression(key)];
        let mut separators = Vec::new();
        if let Some(value) = value {
            arguments.push(AnyJsCallArgument::AnyJsExpression(value.trim_comments_and_trivia()?));
            separators.push(make::token_with_trailing_space(T![,]));
        }
        let call = make::js_call_expression(
            make::js_static_member_expression(
                object.trim_comments_and_trivia()?,
                make::token(T![.]),
                make::js_name(make::ident(method)).into(),
            )
            .into(),
            make::js_call_arguments(
                make::token(T!['(']),
                make::js_call_argument_list(arguments, separators),
                make::token(T![')']),
            ),
        )
        .build();

        let mut mutation = ctx.root().begin();
        mutation.replace_node_transfer_trivia(replaced, call.into())?;
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>{method}"()"</Emphasis>" instead." },
            mutation,
        ))
    }
}

/// What a bracket access does with the property.
#[derive(Clone, Copy)]
enum AccessKind {
    Read,
    /// The property is assigned, updated, or used as the target of a destructuring
    /// assignment or a `for...in` or `for...of` loop.
    Write,
    /// The property is the operand of `delete`.
    Delete,
}

impl biome_console::fmt::Display for AccessKind {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Read => "reads",
            Self::Write => "changes",
            Self::Delete => "deletes",
        })
    }
}

/// Returns the `delete` expression that deletes `member`, ignoring the parentheses and
/// type assertions around `member`.
fn deleting_expression(member: &AnyJsExpression) -> Option<JsUnaryExpression> {
    let unary = member.outer_expression()?.parent::<JsUnaryExpression>()?;
    (unary.operator().ok()? == JsUnaryOperator::Delete).then_some(unary)
}

/// Returns `true` if every value that `key` may have is the name of a property or method of
/// a collection of the given kind.
///
/// The possible values of conditional expressions, logical expressions, and sequences
/// are all checked, and `const` variables are replaced with their initial value. Any
/// other value that isn't a string literal or a well-known `Symbol` such as
/// `Symbol.iterator` isn't a known property.
fn is_collection_member(kind: CollectionKind, key: AnyJsExpression, model: &SemanticModel) -> bool {
    let mut remaining_lookups = MAX_VARIABLE_LOOKUPS;
    let mut pending = vec![key];
    while let Some(key) = pending.pop() {
        let Some(key) = key.inner_expression() else {
            return false;
        };
        if let Some(value) = key.as_static_value() {
            let Some(name) = value.as_string_constant() else {
                return false;
            };
            if !is_member_name(kind, name) {
                return false;
            }
            continue;
        }
        match key {
            AnyJsExpression::JsConditionalExpression(conditional) => {
                let (Ok(consequent), Ok(alternate)) =
                    (conditional.consequent(), conditional.alternate())
                else {
                    return false;
                };
                pending.push(consequent);
                pending.push(alternate);
            }
            AnyJsExpression::JsLogicalExpression(logical) => {
                let (Ok(left), Ok(right)) = (logical.left(), logical.right()) else {
                    return false;
                };
                pending.push(left);
                pending.push(right);
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                let Some(lookups) = remaining_lookups.checked_sub(1) else {
                    return false;
                };
                remaining_lookups = lookups;
                let Some(initializer) = identifier
                    .name()
                    .ok()
                    .and_then(|name| model.binding(&name))
                    .and_then(|binding| binding.tree().declaration())
                    .and_then(|declaration| const_initializer(&declaration))
                else {
                    return false;
                };
                pending.push(initializer);
            }
            AnyJsExpression::JsStaticMemberExpression(_)
            | AnyJsExpression::JsComputedMemberExpression(_) => {
                if !is_member_symbol(kind, &key, model) {
                    return false;
                }
            }
            _ => return false,
        }
    }
    true
}

/// Returns `true` if `name` is a property or method of a collection of the given kind,
/// including the ones that every object inherits, such as `toString`.
fn is_member_name(kind: CollectionKind, name: &str) -> bool {
    let members = match kind {
        CollectionKind::Map => MAP_MEMBERS,
        CollectionKind::Set => SET_MEMBERS,
        CollectionKind::WeakMap => WEAK_MAP_MEMBERS,
        CollectionKind::WeakSet => WEAK_SET_MEMBERS,
    };
    members.binary_search(&name).is_ok() || OBJECT_MEMBERS.binary_search(&name).is_ok()
}

/// Returns `true` if `key` reads a well-known `Symbol`, such as `Symbol.iterator`, that
/// names a property or method of a collection of the given kind.
fn is_member_symbol(kind: CollectionKind, key: &AnyJsExpression, model: &SemanticModel) -> bool {
    let Some(member) = AnyJsMemberExpression::cast_ref(key.syntax()) else {
        return false;
    };
    if member.is_optional_chain() {
        return false;
    }
    let is_global_symbol = member
        .object()
        .ok()
        .and_then(|object| object.omit_parentheses().as_js_identifier_expression()?.name().ok())
        .is_some_and(|name| {
            name.value_token()
                .is_ok_and(|token| token.text_trimmed() == "Symbol")
                && is_global_reference(&name, model)
        });
    if !is_global_symbol {
        return false;
    }
    match member.member_name() {
        Some(name) => match name.text() {
            "toStringTag" => true,
            "iterator" => matches!(kind, CollectionKind::Map | CollectionKind::Set),
            _ => false,
        },
        None => false,
    }
}

/// Returns the value that a `const` variable is initialized with.
fn const_initializer(declaration: &AnyJsBindingDeclaration) -> Option<AnyJsExpression> {
    let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = declaration else {
        return None;
    };
    if !declarator.declaration()?.is_const() {
        return None;
    }
    declarator.initializer()?.expression().ok()
}

/// Returns `true` if `object` is a variable, possibly in parentheses, which the fix can
/// call a method on without adding parentheses.
fn is_simple_object(object: &AnyJsExpression) -> bool {
    matches!(
        object.clone().omit_parentheses(),
        AnyJsExpression::JsIdentifierExpression(_)
    )
}

/// Returns `true` if `node` has comments that the fix would lose or move, because they
/// are inside `node` rather than before or after it.
fn has_inner_comments(node: &AnyJsExpression) -> bool {
    let syntax = node.syntax();
    let first = syntax.first_token();
    let last = syntax.last_token();
    syntax.descendants_tokens(Direction::Next).any(|token| {
        (token.has_leading_comments() && first.as_ref() != Some(&token))
            || (token.has_trailing_comments() && last.as_ref() != Some(&token))
    })
}

/// The maximum number of variables that are looked up to find the values of a key.
const MAX_VARIABLE_LOOKUPS: u8 = 16;

/// The properties and methods of `Map.prototype`, sorted.
const MAP_MEMBERS: &[&str] = &[
    "clear",
    "delete",
    "entries",
    "forEach",
    "get",
    "getOrInsert",
    "getOrInsertComputed",
    "has",
    "keys",
    "set",
    "size",
    "values",
];

/// The properties and methods of `Set.prototype`, sorted.
const SET_MEMBERS: &[&str] = &[
    "add",
    "clear",
    "delete",
    "difference",
    "entries",
    "forEach",
    "has",
    "intersection",
    "isDisjointFrom",
    "isSubsetOf",
    "isSupersetOf",
    "keys",
    "size",
    "symmetricDifference",
    "union",
    "values",
];

/// The methods of `WeakMap.prototype`, sorted.
const WEAK_MAP_MEMBERS: &[&str] = &[
    "delete",
    "get",
    "getOrInsert",
    "getOrInsertComputed",
    "has",
    "set",
];

/// The methods of `WeakSet.prototype`, sorted.
const WEAK_SET_MEMBERS: &[&str] = &["add", "delete", "has"];

/// The properties and methods of `Object.prototype`, which every collection inherits, sorted.
const OBJECT_MEMBERS: &[&str] = &[
    "__defineGetter__",
    "__defineSetter__",
    "__lookupGetter__",
    "__lookupSetter__",
    "__proto__",
    "constructor",
    "hasOwnProperty",
    "isPrototypeOf",
    "propertyIsEnumerable",
    "toLocaleString",
    "toString",
    "valueOf",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn member_lists_sorted() {
        for members in [
            MAP_MEMBERS,
            SET_MEMBERS,
            WEAK_MAP_MEMBERS,
            WEAK_SET_MEMBERS,
            OBJECT_MEMBERS,
        ] {
            for pair in members.windows(2) {
                assert!(pair[0] < pair[1], "{} must come after {}", pair[0], pair[1]);
            }
        }
    }
}
