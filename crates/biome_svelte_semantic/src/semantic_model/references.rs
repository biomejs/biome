//! The reference pass: finds every use of a name and resolves it to the
//! declaration it refers to.
//!
//! Names cross language boundaries in a Svelte component. An identifier in a
//! template expression can be a variable of `{#each}` declared in HTML, a
//! binding of the instance script, or a binding of `<script module>`. The
//! instance script can call a `{#snippet}` declared at the root of the
//! template. This pass walks one scope chain that spans all of them.
//!
//! It also records what each use does with the value, and follows the
//! indirections Svelte defines: `$count` reads the store `count`, and
//! assigning to a property of an item of `{#each}` changes the list.

use super::builder::ModelBuilder;
use super::layout::Layout;
use super::model::*;
use super::script::{Declared, Tree};
use super::snippets::root_expression;
use biome_js_syntax::{
    AnyJsExpression, AnyJsIdentifierReference, AnyJsLiteralExpression, JsCallArgumentList,
    JsCallArguments, JsCallExpression, JsComputedMemberAssignment, JsComputedMemberExpression,
    JsIdentifierAssignment, JsReferenceIdentifier, JsStaticMemberAssignment,
    JsStaticMemberExpression, JsSyntaxKind, JsSyntaxNode, JsUnaryExpression, JsUnaryOperator,
};
use biome_rowan::{AstNode, AstSeparatedList, TokenText};
use rustc_hash::FxHashSet;

/// Methods that change an array, a map, a set or a date in place.
const MUTATING_METHODS: &[&str] = &[
    "add",
    "append",
    "clear",
    "copyWithin",
    "delete",
    "fill",
    "pop",
    "push",
    "reverse",
    "set",
    "shift",
    "sort",
    "splice",
    "unshift",
];

/// What a use does with a value, read from the syntax around it.
struct Usage {
    path: Vec<TokenText>,
    path_open: bool,
    access: Access,
    call_args: Option<u8>,
    /// The outermost expression of the whole member chain.
    end: JsSyntaxNode,
}

/// Follows the member chain that starts at `start` and classifies what
/// happens to the value at its end.
///
/// The path stops at the first member whose name the model cannot read, as in
/// `list[index]`. The members after it are still followed, so that
/// `list[index].done = true` is a write to `list`.
fn usage_of(start: &JsSyntaxNode) -> Usage {
    let mut current = start.clone();
    let mut path = Vec::new();
    let mut path_open = false;
    // The name of the member `current` reads, when it is static.
    let mut member_name = None;
    let mut written = matches!(current.kind(), JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT);

    while let Some(parent) = current.parent() {
        match parent.kind() {
            JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
            | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
            | JsSyntaxKind::TS_AS_EXPRESSION
            | JsSyntaxKind::TS_SATISFIES_EXPRESSION => current = parent,
            JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
                let member = JsStaticMemberExpression::unwrap_cast(parent.clone());
                if member.object().ok().map(AstNode::into_syntax) != Some(current.clone()) {
                    break;
                }
                let Some(name) = member
                    .member()
                    .ok()
                    .and_then(|name| name.value_token().ok())
                else {
                    break;
                };
                member_name = Some(name.token_text_trimmed());
                if !path_open {
                    path.extend(member_name.clone());
                }
                current = parent;
            }
            JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => {
                let member = JsStaticMemberAssignment::unwrap_cast(parent.clone());
                if member.object().ok().map(AstNode::into_syntax) != Some(current.clone()) {
                    break;
                }
                if !path_open
                    && let Some(name) = member
                        .member()
                        .ok()
                        .and_then(|name| name.value_token().ok())
                {
                    path.push(name.token_text_trimmed());
                }
                written = true;
                current = parent;
                break;
            }
            JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                let member = JsComputedMemberExpression::unwrap_cast(parent.clone());
                if member.object().ok().map(AstNode::into_syntax) != Some(current.clone()) {
                    break;
                }
                member_name = member.member().ok().and_then(static_key);
                match &member_name {
                    Some(name) if !path_open => path.push(name.clone()),
                    Some(_) => {}
                    None => path_open = true,
                }
                current = parent;
            }
            JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                let member = JsComputedMemberAssignment::unwrap_cast(parent.clone());
                if member.object().ok().map(AstNode::into_syntax) != Some(current.clone()) {
                    break;
                }
                match member.member().ok().and_then(static_key) {
                    Some(name) if !path_open => path.push(name),
                    Some(_) => {}
                    None => path_open = true,
                }
                written = true;
                current = parent;
                break;
            }
            _ => break,
        }
    }

    let mut usage = Usage {
        path,
        path_open,
        access: Access::Read,
        call_args: None,
        end: current.clone(),
    };
    if written {
        usage.access = Access::Write;
        return usage;
    }
    let Some(parent) = current.parent() else {
        return usage;
    };
    match parent.kind() {
        JsSyntaxKind::JS_CALL_EXPRESSION => {
            let call = JsCallExpression::unwrap_cast(parent);
            if call.callee().ok().map(AstNode::into_syntax) == Some(current) {
                if member_name
                    .as_ref()
                    .is_some_and(|name| MUTATING_METHODS.contains(&name.text()))
                {
                    usage.access = Access::Mutate;
                    // The method is not part of the path.
                    if !usage.path_open {
                        usage.path.pop();
                    }
                } else {
                    usage.access = Access::Call;
                    usage.call_args = Some(
                        call.arguments()
                            .map_or(0, |arguments| arguments.args().len() as u8),
                    );
                }
            } else {
                usage.access = Access::Escape;
            }
        }
        JsSyntaxKind::JS_UNARY_EXPRESSION => {
            if JsUnaryExpression::unwrap_cast(parent)
                .operator()
                .is_ok_and(|operator| operator == JsUnaryOperator::Delete)
            {
                usage.access = Access::Write;
            }
        }
        JsSyntaxKind::JS_CALL_ARGUMENT_LIST if is_object_assign_target(&current) => {
            usage.access = Access::Mutate;
        }
        JsSyntaxKind::JS_CALL_ARGUMENT_LIST
        | JsSyntaxKind::JS_FOR_IN_STATEMENT
        | JsSyntaxKind::JS_SPREAD
        | JsSyntaxKind::JS_RETURN_STATEMENT
        | JsSyntaxKind::JS_INITIALIZER_CLAUSE
        | JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER
        | JsSyntaxKind::JS_SHORTHAND_PROPERTY_OBJECT_MEMBER
        | JsSyntaxKind::JS_ARRAY_ELEMENT_LIST
        | JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => usage.access = Access::Escape,
        _ => {}
    }
    usage
}

/// Returns `true` when `node` is the first argument of `Object.assign()`, the
/// object that the call writes to.
///
/// `Object` is matched by name: the model doesn't check that it is the global.
fn is_object_assign_target(node: &JsSyntaxNode) -> bool {
    let Some(list) = node.parent().and_then(JsCallArgumentList::cast) else {
        return false;
    };
    let is_first = list
        .first()
        .and_then(|argument| argument.ok())
        .is_some_and(|argument| argument.syntax() == node);
    if !is_first {
        return false;
    }
    let Some(member) = list
        .parent::<JsCallArguments>()
        .and_then(|arguments| arguments.parent::<JsCallExpression>())
        .and_then(|call| call.callee().ok())
        .and_then(|callee| {
            callee
                .omit_parentheses()
                .as_js_static_member_expression()
                .cloned()
        })
    else {
        return false;
    };
    member
        .member()
        .ok()
        .and_then(|name| name.value_token().ok())
        .is_some_and(|name| name.text_trimmed() == "assign")
        && member
            .object()
            .ok()
            .and_then(|object| object.omit_parentheses().as_js_reference_identifier())
            .is_some_and(|object| object.has_name("Object"))
}

/// Returns the name a computed member key spells out, as in `list["push"]`.
fn static_key(key: AnyJsExpression) -> Option<TokenText> {
    match key.omit_parentheses() {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(literal),
        ) => literal.inner_string_text().ok(),
        _ => None,
    }
}

/// Returns the node a member chain starts at for a reference node: the
/// identifier expression that wraps a reference, or the node itself.
fn chain_start(node: &JsSyntaxNode) -> JsSyntaxNode {
    match node.parent() {
        Some(parent) if parent.kind() == JsSyntaxKind::JS_IDENTIFIER_EXPRESSION => parent,
        _ => node.clone(),
    }
}

/// Whether `node` is written inside a function of its tree. A class field is
/// counted as a function, because its initializer runs when the class is
/// instantiated.
fn is_in_function(node: &JsSyntaxNode) -> bool {
    node.ancestors().skip(1).any(|ancestor| {
        matches!(
            ancestor.kind(),
            JsSyntaxKind::JS_FUNCTION_EXPRESSION
                | JsSyntaxKind::JS_FUNCTION_DECLARATION
                | JsSyntaxKind::JS_FUNCTION_EXPORT_DEFAULT_DECLARATION
                | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                | JsSyntaxKind::JS_METHOD_CLASS_MEMBER
                | JsSyntaxKind::JS_METHOD_OBJECT_MEMBER
                | JsSyntaxKind::JS_GETTER_CLASS_MEMBER
                | JsSyntaxKind::JS_GETTER_OBJECT_MEMBER
                | JsSyntaxKind::JS_SETTER_CLASS_MEMBER
                | JsSyntaxKind::JS_SETTER_OBJECT_MEMBER
                | JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER
                | JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER
        )
    })
}

fn is_type_reference(node: &JsSyntaxNode) -> bool {
    AnyJsIdentifierReference::cast_ref(node).is_some_and(|reference| reference.is_only_type())
}

/// Returns the free identifiers of a tree: references that nothing in the
/// tree itself declares.
fn free_references(tree: Tree) -> Vec<JsSyntaxNode> {
    tree.root
        .syntax()
        .descendants()
        .filter(|node| {
            if let Some(reference) = JsReferenceIdentifier::cast_ref(node) {
                tree.js.binding(&reference).is_none()
            } else if let Some(assignment) = JsIdentifierAssignment::cast_ref(node) {
                tree.js.binding(&assignment).is_none()
            } else {
                false
            }
        })
        .collect()
}

/// Whether `name` reads a store: `$count` reads the store `count`. Returns the
/// name of the store.
fn store_name(name: &str) -> Option<&str> {
    let store = name.strip_prefix('$')?;
    (!store.is_empty() && !store.starts_with('$')).then_some(store)
}

/// Whether the value of the snippet `snippet` is assigned to by Svelte: the
/// value of `bind:`.
fn is_bind_value(builder: &ModelBuilder, snippet: SnippetId) -> bool {
    match builder.data.snippets[snippet.index()].host {
        SnippetHost::AttrValue(attr) => matches!(
            builder.data.attrs[attr.index()].name,
            AttrName::Directive {
                kind: DirectiveKind::Bind,
                ..
            }
        ),
        _ => false,
    }
}

/// Records the uses of every symbol a JavaScript tree declares, as the
/// semantic model of the tree finds them.
pub(crate) fn collect_declared_references(
    builder: &mut ModelBuilder,
    trees: &[Tree],
    declared: &[Declared],
) {
    for declared in declared {
        let tree = trees[declared.tree];
        let Some(binding) = tree
            .js
            .as_binding_by_range(declared.binding.text_trimmed_range())
        else {
            continue;
        };
        let symbol_name = builder.data.symbols[declared.symbol.index()].name.clone();
        // Svelte rejects a store subscription in `<script module>` and in a
        // module, where `$store` is then an undeclared name.
        let can_subscribe = builder.data.file_kind == FileKind::Component
            && tree.snippet.is_some_and(|id| {
                !matches!(builder.data.snippets[id.index()].host, SnippetHost::Script(script)
                    if builder.data.scripts[script.index()].kind == ScriptKind::Module)
            });
        for reference in binding.all_references() {
            let node = reference.syntax();
            let Some(token) = node.first_token() else {
                continue;
            };
            let name = token.token_text_trimmed();
            let usage = usage_of(&chain_start(&node));
            let in_snippet_block = tree
                .snippet
                .is_some_and(|id| builder.data.snippets[id.index()].in_snippet_block);
            let store_subscription = name.text() != symbol_name.text();
            let resolution = if store_subscription && !can_subscribe {
                Resolution::Global
            } else {
                Resolution::Symbol(declared.symbol)
            };
            builder.add_reference(ReferenceData {
                range: tree.host(token.text_trimmed_range()),
                store_subscription: store_subscription && can_subscribe,
                name,
                snippet: tree.snippet,
                site: RefSite::Identifier,
                scope: None,
                via: None,
                path: usage.path.into_boxed_slice(),
                path_open: usage.path_open,
                access: usage.access,
                call_args: usage.call_args,
                resolution,
                in_function: is_in_function(&node),
                in_snippet_block,
                type_only: is_type_reference(&node),
            });
        }
    }
}

/// Resolves the names each tree leaves free in the scope chain of the
/// component.
pub(crate) fn collect_free_references(builder: &mut ModelBuilder, trees: &[Tree]) {
    // The semantic model of a tree reports `$store = value` among the uses of
    // `store`, but doesn't bind it to `store`, so it is skipped here.
    let recorded: FxHashSet<_> = builder
        .data
        .references
        .iter()
        .map(|reference| reference.range)
        .collect();
    for tree in trees {
        let (scope, in_snippet_block, is_instance, bind_value) = match tree.snippet {
            Some(id) => {
                let row = &builder.data.snippets[id.index()];
                let is_instance = matches!(row.host, SnippetHost::Script(script)
                    if builder.data.scripts[script.index()].kind == ScriptKind::Instance);
                (
                    row.scope,
                    row.in_snippet_block,
                    is_instance,
                    is_bind_value(builder, id),
                )
            }
            None => (builder.data.module_scope, false, false, false),
        };
        let root_expression = root_expression(tree.root)
            .map(|expression| expression.omit_parentheses().into_syntax());
        for node in free_references(*tree) {
            let Some(token) = node.first_token() else {
                continue;
            };
            if recorded.contains(&tree.host(token.text_trimmed_range())) {
                continue;
            }
            let name = token.token_text_trimmed();
            let mut usage = usage_of(&chain_start(&node));
            if bind_value && Some(&usage.end) == root_expression.as_ref() {
                usage.access = Access::Write;
            }
            let (resolution, store_subscription) =
                resolve(builder, scope, name.text(), is_instance);
            builder.add_reference(ReferenceData {
                range: tree.host(token.text_trimmed_range()),
                name,
                snippet: tree.snippet,
                site: RefSite::Identifier,
                scope,
                via: None,
                path: usage.path.into_boxed_slice(),
                path_open: usage.path_open,
                access: usage.access,
                call_args: usage.call_args,
                resolution,
                store_subscription,
                in_function: is_in_function(&node),
                in_snippet_block,
                type_only: is_type_reference(&node),
            });
        }
    }
}

/// Resolves `name` written where `scope` is visible. Returns the resolution,
/// and whether the name reads a store.
///
/// `is_instance` is `true` in the instance script, which also sees the
/// snippets declared at the root of the template.
fn resolve(
    builder: &ModelBuilder,
    scope: Option<ScopeId>,
    name: &str,
    is_instance: bool,
) -> (Resolution, bool) {
    if Rune::is_base_name(name) {
        return (Resolution::Rune, false);
    }
    if let Some(scope) = scope {
        if let Some(symbol) = builder.lookup(scope, name) {
            return (Resolution::Symbol(symbol), false);
        }
        if is_instance
            && let Some(template) = builder.data.template_scope
            && let Some(symbol) = builder.data.scopes[template.index()]
                .symbols
                .iter()
                .copied()
                .find(|id| {
                    let symbol = &builder.data.symbols[id.index()];
                    symbol.kind == SymbolKind::Snippet && symbol.name.text() == name
                })
        {
            return (Resolution::Symbol(symbol), false);
        }
    }
    // A component subscribes to a store declared at the top level of one of
    // its scripts. Svelte rejects a subscription in `<script module>` and in
    // a module.
    if let Some(store) = store_name(name)
        && builder.data.file_kind == FileKind::Component
        && scope.is_some_and(|scope| Some(scope) != builder.data.module_scope)
        && let Some(symbol) = builder
            .data
            .instance_scope
            .and_then(|scope| builder.lookup(scope, store))
    {
        return (Resolution::Symbol(symbol), true);
    }
    (Resolution::Global, false)
}

/// Resolves the names the host document refers to outside of snippets:
/// component tags, and the names of directives.
pub(crate) fn collect_markup_references(builder: &mut ModelBuilder, layout: &Layout) {
    for reference in &layout.markup_refs {
        let (resolution, store_subscription) =
            resolve(builder, Some(reference.scope), reference.name.text(), false);
        let id = builder.add_reference(ReferenceData {
            range: reference.range,
            name: reference.name.clone(),
            snippet: None,
            site: reference.site,
            scope: Some(reference.scope),
            via: None,
            path: reference.path.clone(),
            path_open: false,
            access: reference.access,
            call_args: None,
            resolution,
            store_subscription,
            in_function: false,
            in_snippet_block: reference.in_snippet_block,
            type_only: false,
        });
        match reference.site {
            RefSite::TagName(element) => builder.data.elements[element.index()].tag_ref = Some(id),
            RefSite::DirectiveName(attr) => {
                if let AttrName::Directive { name_ref, .. } =
                    &mut builder.data.attrs[attr.index()].name
                {
                    *name_ref = Some(id);
                }
            }
            RefSite::DirectiveShorthand(attr) => {
                builder.data.attrs[attr.index()].implicit_ref = Some(id);
            }
            RefSite::Identifier | RefSite::EachItem => {}
        }
    }
}

/// Links the variables of each `{#each}` to the variable holding its list,
/// and records each change made through those variables as a change of the
/// list.
pub(crate) fn collect_each_mutations(builder: &mut ModelBuilder, layout: &Layout, trees: &[Tree]) {
    for (block, items) in &layout.each_items {
        let Some(list) = builder.data.blocks[block.index()].expression else {
            continue;
        };
        let Some(tree) = trees.iter().find(|tree| tree.snippet == Some(list)) else {
            continue;
        };
        let Some(base) =
            root_expression(tree.root).and_then(|expression| base_identifier(&expression))
        else {
            continue;
        };
        let range = tree.host(base.text_trimmed_range());
        let Some(source) = builder
            .data
            .references
            .iter()
            .find(|reference| reference.range == range)
            .and_then(|reference| match reference.resolution {
                Resolution::Symbol(symbol) => Some(symbol),
                _ => None,
            })
        else {
            continue;
        };
        for item in items {
            builder.symbol_mut(*item).each_source = Some(source);
        }
    }

    let mut projected = Vec::new();
    for reference in &builder.data.references {
        let Resolution::Symbol(symbol) = reference.resolution else {
            continue;
        };
        if !matches!(reference.access, Access::Write | Access::Mutate) {
            continue;
        }
        // A change through the item of a nested `{#each}` also changes the
        // lists of the enclosing blocks.
        let mut via = symbol;
        while let Some(source) = builder.data.symbols[via.index()].each_source {
            projected.push(ReferenceData {
                name: builder.data.symbols[source.index()].name.clone(),
                site: RefSite::EachItem,
                via: Some(via),
                path: Box::default(),
                path_open: true,
                access: Access::Mutate,
                call_args: None,
                resolution: Resolution::Symbol(source),
                store_subscription: false,
                ..reference.clone()
            });
            via = source;
            if projected.len() > builder.data.symbols.len() * 4 {
                break;
            }
        }
    }
    for reference in projected {
        builder.add_reference(reference);
    }
}

/// Returns the identifier a member chain starts with: `items` for
/// `data.items`.
fn base_identifier(expression: &AnyJsExpression) -> Option<JsSyntaxNode> {
    let mut current = expression.clone().omit_parentheses();
    loop {
        current = match current {
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                return Some(identifier.name().ok()?.into_syntax());
            }
            AnyJsExpression::JsStaticMemberExpression(member) => {
                member.object().ok()?.omit_parentheses()
            }
            AnyJsExpression::JsComputedMemberExpression(member) => {
                member.object().ok()?.omit_parentheses()
            }
            AnyJsExpression::TsNonNullAssertionExpression(assertion) => {
                assertion.expression().ok()?.omit_parentheses()
            }
            _ => return None,
        };
    }
}

/// Decides whether the component is in runes mode.
///
/// Svelte uses runes mode when `<svelte:options runes>` says so, and
/// otherwise when the component refers to a rune or uses `await` at the top
/// level of its instance script or of its template.
pub(crate) fn decide_runes_mode(builder: &mut ModelBuilder, trees: &[Tree]) {
    let has_rune = builder
        .data
        .references
        .iter()
        .any(|reference| reference.resolution == Resolution::Rune);
    let has_top_level_await = trees.iter().any(|tree| {
        let in_instance_or_template =
            tree.snippet
                .is_some_and(|id| match builder.data.snippets[id.index()].host {
                    SnippetHost::Script(script) => {
                        builder.data.scripts[script.index()].kind == ScriptKind::Instance
                    }
                    SnippetHost::Generics(_) => false,
                    _ => true,
                });
        in_instance_or_template
            && tree.root.syntax().descendants().any(|node| {
                node.kind() == JsSyntaxKind::JS_AWAIT_EXPRESSION && !is_in_function(&node)
            })
    });
    builder.data.runes = builder
        .data
        .runes_option
        .unwrap_or(has_rune || has_top_level_await);
}
