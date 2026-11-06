//! Subtree traversals that plugins filter with a node kind set: `node.descendants(kinds)`
//! and `node.walk(kinds, enter, exit)`.
//!
//! Plugins usually look for a few node kinds in a large tree. Walking the tree
//! in JavaScript with `children()` wraps every node in a JavaScript object and
//! calls back into the plugin for each one. These traversals read the tree in
//! Rust, and wrap and pass to JavaScript only the nodes whose kind is in the
//! set.
//!
//! Both traversals visit the descendants of the receiver in source order and
//! never visit the receiver itself, like `ancestors()` excludes it.

use crate::ast::JsAstNode;
use crate::kind_set::{JsNodeKindSet, JsTokenKindSet};
use biome_js_syntax::{JsLanguage, JsSyntaxNode};
use biome_rowan::{SyntaxKindSet, WalkEvent};
use boa_engine::object::builtins::{JsArray, JsFunction};
use boa_engine::{Context, JsNativeError, JsResult, JsValue};

impl JsAstNode {
    /// Implements `node.descendants(kinds)`.
    ///
    /// Returns a fresh array of the descendants whose kind is in `kinds`, in
    /// source order.
    pub(crate) fn descendants(
        this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let Some(Self { node }) = Self::from_value(this) else {
            return Err(JsNativeError::typ()
                .with_message("descendants() was called on a value that is not a node. Call node.descendants(kinds) on the node passed to run() or a node obtained by traversing it.")
                .into());
        };
        let [kinds] = args else {
            return Err(JsNativeError::typ()
                .with_message("node.descendants() received the wrong number of arguments. Pass exactly one set created with nodeKinds(), using node.descendants(kinds).")
                .into());
        };
        let kinds = traversal_kinds(kinds, "descendants")?;

        let mut events = node.preorder_of_kinds(kinds);
        if kinds.matches(node.kind()) {
            // Skips the event entering the receiver.
            events.next();
        }
        let descendants = events
            .filter_map(|event| match event {
                WalkEvent::Enter(descendant) => Some(Self::from_node(descendant, context)),
                WalkEvent::Leave(_) => None,
            })
            .collect::<Vec<_>>();

        Ok(JsArray::from_iter(descendants, context).into())
    }

    /// Implements `node.walk(kinds, enter, exit)`.
    ///
    /// Calls `enter` with each descendant whose kind is in `kinds`, in source
    /// order, and `exit` with the same object once the descendants of that
    /// node have been visited. When `enter` returns `false`, the descendants of
    /// that node are skipped and `exit` is not called for it. An exception
    /// thrown by a callback stops the walk and propagates to the caller.
    pub(crate) fn walk(
        this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let Some(Self { node }) = Self::from_value(this) else {
            return Err(JsNativeError::typ()
                .with_message("walk() was called on a value that is not a node. Call node.walk(kinds, enter, exit) on the node passed to run() or a node obtained by traversing it.")
                .into());
        };
        let (kinds, enter, exit) = match args {
            [kinds, enter] => (kinds, enter, None),
            [kinds, enter, exit] => (kinds, enter, Some(exit).filter(|exit| !exit.is_undefined())),
            _ => {
                return Err(JsNativeError::typ()
                    .with_message("node.walk() received the wrong number of arguments. Pass a set created with nodeKinds(), an enter function, and optionally an exit function, using node.walk(kinds, enter, exit).")
                    .into());
            }
        };
        let kinds = traversal_kinds(kinds, "walk")?;
        let enter = enter.as_function().ok_or_else(|| {
            JsNativeError::typ().with_message("The enter argument to node.walk() is not a function. Pass a function that receives each matching node.")
        })?;
        let exit = exit
            .map(|exit| {
                exit.as_function().ok_or_else(|| {
                    JsNativeError::typ().with_message("The exit argument to node.walk() is not a function. Pass a function that receives each matching node after its descendants, or omit the argument.")
                })
            })
            .transpose()?;

        walk_descendants(&node, kinds, &enter, exit.as_ref(), context)?;
        Ok(JsValue::undefined())
    }
}

fn walk_descendants(
    node: &JsSyntaxNode,
    kinds: SyntaxKindSet<JsLanguage>,
    enter: &JsFunction,
    exit: Option<&JsFunction>,
    context: &mut Context,
) -> JsResult<()> {
    // Passing `exit` the object `enter` received lets plugins key state on node
    // identity, such as a `WeakSet` of the elements that pushed onto a stack.
    let mut entered = Vec::new();
    let mut events = node.preorder_of_kinds(kinds);
    if kinds.matches(node.kind()) {
        // Skips the event entering the receiver. The event leaving it comes last.
        events.next();
    }
    while let Some(event) = events.next() {
        match event {
            WalkEvent::Enter(descendant) => {
                let value = JsAstNode::from_node(descendant, context);
                let result =
                    enter.call(&JsValue::undefined(), std::slice::from_ref(&value), context)?;
                if result.as_boolean() == Some(false) {
                    events.skip_subtree();
                    // The next event leaves the skipped node.
                    events.next();
                } else if exit.is_some() {
                    entered.push(value);
                }
            }
            WalkEvent::Leave(_) => {
                // `entered` is empty when the receiver itself is left.
                if let Some(exit) = exit
                    && let Some(value) = entered.pop()
                {
                    exit.call(&JsValue::undefined(), &[value], context)?;
                }
            }
        }
    }
    Ok(())
}

fn traversal_kinds(value: &JsValue, method: &str) -> JsResult<SyntaxKindSet<JsLanguage>> {
    if let Some(kinds) = JsNodeKindSet::<JsLanguage>::from_value(value) {
        return Ok(kinds);
    }
    let message = if JsTokenKindSet::<JsLanguage>::from_value(value).is_some() {
        format!(
            "node.{method}() received a set of token kinds, but it only visits nodes. Create a set of node kinds with nodeKinds(...kinds) instead."
        )
    } else {
        format!(
            "The kinds argument to node.{method}() is not a node kind set. Create one with nodeKinds(), such as nodeKinds(\"JS_CALL_EXPRESSION\"), and pass it instead of a kind string or array."
        )
    };
    Err(JsNativeError::typ().with_message(message).into())
}
