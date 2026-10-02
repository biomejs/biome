use crate::ast::JsAstNode;
use crate::token::JsAstToken;
use biome_js_syntax::{JsLanguage, JsSyntaxElement, JsSyntaxNode};
use biome_rowan::BatchMutation;
use boa_engine::class::{Class, ClassBuilder};
use boa_engine::{
    Context, Finalize, JsData, JsNativeError, JsResult, JsValue, NativeFunction, Trace, js_string,
};

#[derive(Debug, JsData)]
pub(crate) struct JsMutation {
    batch: Option<BatchMutation<JsLanguage>>,
}

impl Finalize for JsMutation {}

// SAFETY: The batch contains only Rowan data and no Boa-managed values.
unsafe impl Trace for JsMutation {
    boa_engine::gc::empty_trace!();
}

#[derive(Clone, Copy)]
enum ElementCategory {
    Node,
    Token,
    Either,
}

impl JsMutation {
    pub(crate) fn register(context: &mut Context) -> JsResult<()> {
        context.register_global_class::<Self>()?;
        context
            .global_object()
            .delete_property_or_throw(js_string!(Self::NAME), context)?;
        Ok(())
    }

    /// Consumes a batch targeting the source currently being analyzed.
    pub(crate) fn take(
        value: &JsValue,
        source: &JsSyntaxNode,
    ) -> JsResult<BatchMutation<JsLanguage>> {
        let object = value
            .as_object()
            .ok_or_else(|| JsNativeError::typ().with_message("The fix's mutation property is not a mutation object. Set it to the value returned by createMutation(node)."))?;
        let mut mutation = object
            .downcast_mut::<Self>()
            .ok_or_else(|| JsNativeError::typ().with_message("The fix's mutation property is not a mutation object. Set it to the value returned by createMutation(node)."))?;
        if mutation
            .batch
            .as_ref()
            .is_some_and(|batch| batch.root() != source)
        {
            return Err(JsNativeError::typ()
                .with_message("This mutation does not edit the source being analyzed. Create the mutation from a node in that source before passing it to registerDiagnostic().")
                .into());
        }
        mutation.batch.take().ok_or_else(|| {
            JsNativeError::typ()
                .with_message("This mutation has already been passed to registerDiagnostic(). Create a new mutation for each diagnostic, and finish adding edits before registering it.")
                .into()
        })
    }

    fn change(
        this: &JsValue,
        args: &[JsValue],
        category: ElementCategory,
        replace: bool,
    ) -> JsResult<JsValue> {
        let method = match (category, replace) {
            (ElementCategory::Node, true) => "replaceNode",
            (ElementCategory::Token, true) => "replaceToken",
            (ElementCategory::Either, true) => "replaceElement",
            (ElementCategory::Node, false) => "removeNode",
            (ElementCategory::Token, false) => "removeToken",
            (ElementCategory::Either, false) => "removeElement",
        };
        let element = match category {
            ElementCategory::Node => "node",
            ElementCategory::Token => "token",
            ElementCategory::Either => "node or token",
        };
        let (target, replacement) = match (replace, args) {
            (true, [target, replacement]) => (
                element_from_value(target, category, method, "first")?,
                Some(element_from_value(replacement, category, method, "second")?),
            ),
            (false, [target]) => (element_from_value(target, category, method, "first")?, None),
            _ => {
                return Err(JsNativeError::typ()
                    .with_message(if replace {
                        format!("{method}() requires exactly two arguments. Pass the {element} to replace first, then its replacement.")
                    } else {
                        format!("{method}() requires exactly one argument. Pass the {element} to remove.")
                    })
                    .into());
            }
        };
        if let Some(replacement) = &replacement
            && matches!(&target, JsSyntaxElement::Node(_))
                != matches!(replacement, JsSyntaxElement::Node(_))
        {
            return Err(JsNativeError::typ()
                .with_message("replaceElement() cannot replace a node with a token or a token with a node. Pass two nodes or two tokens, or use replaceElement().")
                .into());
        }

        // All argument downcasts finish before borrowing the receiver mutably:
        // JavaScript can pass the receiver itself as an argument.
        let object = this.as_object().ok_or_else(|| {
            JsNativeError::typ().with_message(format!("{method}() was called without a mutation object. Call it on the object returned by createMutation(node)."))
        })?;
        let mut mutation = object.downcast_mut::<Self>().ok_or_else(|| {
            JsNativeError::typ().with_message(format!("{method}() was called without a mutation object. Call it on the object returned by createMutation(node)."))
        })?;
        let batch = mutation.batch.as_mut().ok_or_else(|| {
            JsNativeError::typ().with_message("This mutation has already been passed to registerDiagnostic(). Create a new mutation for each diagnostic, and finish adding edits before registering it.")
        })?;
        match (target, replacement) {
            (JsSyntaxElement::Token(target), Some(JsSyntaxElement::Token(replacement))) => {
                batch.replace_token(target, replacement);
            }
            (target, Some(replacement)) => batch.replace_element(target, replacement),
            (JsSyntaxElement::Token(target), None) => batch.remove_token(target),
            (target, None) => batch.remove_element(target),
        }
        Ok(JsValue::undefined())
    }
}

impl Class for JsMutation {
    const NAME: &'static str = "__JsMutation";

    fn init(class: &mut ClassBuilder<'_>) -> JsResult<()> {
        class
            .method(
                js_string!("replaceNode"),
                2,
                NativeFunction::from_fn_ptr(|this, args, _| {
                    Self::change(this, args, ElementCategory::Node, true)
                }),
            )
            .method(
                js_string!("replaceToken"),
                2,
                NativeFunction::from_fn_ptr(|this, args, _| {
                    Self::change(this, args, ElementCategory::Token, true)
                }),
            )
            .method(
                js_string!("replaceElement"),
                2,
                NativeFunction::from_fn_ptr(|this, args, _| {
                    Self::change(this, args, ElementCategory::Either, true)
                }),
            )
            .method(
                js_string!("removeNode"),
                1,
                NativeFunction::from_fn_ptr(|this, args, _| {
                    Self::change(this, args, ElementCategory::Node, false)
                }),
            )
            .method(
                js_string!("removeToken"),
                1,
                NativeFunction::from_fn_ptr(|this, args, _| {
                    Self::change(this, args, ElementCategory::Token, false)
                }),
            )
            .method(
                js_string!("removeElement"),
                1,
                NativeFunction::from_fn_ptr(|this, args, _| {
                    Self::change(this, args, ElementCategory::Either, false)
                }),
            );
        Ok(())
    }

    fn data_constructor(
        _new_target: &JsValue,
        _args: &[JsValue],
        _context: &mut Context,
    ) -> JsResult<Self> {
        Err(JsNativeError::typ()
            .with_message(
                "Mutations cannot be constructed with new. Call createMutation(node) instead.",
            )
            .into())
    }
}

pub(crate) fn create_mutation(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
) -> JsResult<JsValue> {
    let [anchor] = args else {
        return Err(JsNativeError::typ()
            .with_message("createMutation() requires exactly one node. Pass the node supplied to run() or a node reached through its fields or traversal methods.")
            .into());
    };
    let anchor = JsAstNode::from_value(anchor).ok_or_else(|| {
        JsNativeError::typ().with_message("createMutation() received a value that is not a Biome node. Pass the node supplied to run() or one of its child nodes, not a token, string, or plain object.")
    })?;
    let root = anchor
        .node
        .ancestors()
        .last()
        .unwrap_or_else(|| anchor.node.clone());
    Ok(JsMutation::from_data(
        JsMutation {
            batch: Some(BatchMutation::new(root)),
        },
        context,
    )?
    .into())
}

fn element_from_value(
    value: &JsValue,
    category: ElementCategory,
    method: &str,
    argument: &str,
) -> JsResult<JsSyntaxElement> {
    if matches!(category, ElementCategory::Node | ElementCategory::Either)
        && let Some(node) = JsAstNode::from_value(value)
    {
        return Ok(node.node.into());
    }
    if matches!(category, ElementCategory::Token | ElementCategory::Either)
        && let Some(token) = JsAstToken::from_value(value)
    {
        return Ok(token.token.into());
    }
    let requirement = match (category, argument == "first") {
        (ElementCategory::Node, true) => {
            "a Biome node from the source passed to createMutation(). Read the node through its fields or traversal methods; do not pass a token, string, or plain object"
        }
        (ElementCategory::Node, false) => {
            "a Biome node. Use a node from run(), its fields, or a node field-update method; do not pass a token, string, or plain object"
        }
        (ElementCategory::Token, true) => {
            "a Biome token from the source passed to createMutation(). Use node.token(field) or node.childrenWithTokens() to obtain it; do not pass the string returned by a token field"
        }
        (ElementCategory::Token, false) => {
            "a Biome token. Use node.token(field), node.childrenWithTokens(), or factory.token(); do not pass the string returned by a token field"
        }
        (ElementCategory::Either, true) => {
            "a Biome node or token from the source passed to createMutation(). Use node.childrenWithTokens() to get editable elements; do not pass a string or plain object"
        }
        (ElementCategory::Either, false) => {
            "a Biome node or token. Use node traversal, node field-update methods, or factory.token() to get a replacement; do not pass a string or plain object"
        }
    };
    Err(JsNativeError::typ()
        .with_message(format!(
            "The {argument} argument to {method}() must be {requirement}."
        ))
        .into())
}

#[cfg(test)]
mod tests {
    use std::slice;

    use biome_js_parser::{JsParserOptions, parse_script};
    use biome_js_syntax::{JsSyntaxKind, JsSyntaxToken};
    use biome_rowan::Direction;

    use super::*;

    fn setup() -> JsResult<(Context, JsSyntaxNode, JsValue)> {
        let mut context = Context::default();
        JsAstNode::register(&mut context)?;
        JsAstToken::register(&mut context)?;
        JsMutation::register(&mut context)?;
        let root = parse_script("f(a,b);", JsParserOptions::default()).syntax();
        let value = JsAstNode::from_node(root.clone(), &mut context);
        Ok((context, root, value))
    }

    #[test]
    fn anchors_normalize_actual_roots_without_an_invocation() -> JsResult<()> {
        let (mut context, root, _) = setup()?;
        let call = root
            .descendants()
            .find(|node| node.kind() == JsSyntaxKind::JS_CALL_EXPRESSION)
            .unwrap();
        for tree in [root, call.detach()] {
            let anchor = tree.first_child().unwrap();
            let value = JsAstNode::from_node(anchor, &mut context);
            let mutation = create_mutation(&JsValue::undefined(), &[value], &mut context)?;
            let batch = JsMutation::take(&mutation, &tree)?;
            assert_eq!(batch.root(), &tree);
            assert!(batch.is_empty());
        }
        Ok(())
    }

    #[test]
    fn argument_and_comma_are_independent_targets() -> JsResult<()> {
        let (mut context, syntax, root) = setup()?;
        let argument = syntax
            .descendants()
            .find(|node| {
                node.kind() == JsSyntaxKind::JS_IDENTIFIER_EXPRESSION && node.text_trimmed() == "a"
            })
            .ok_or_else(|| JsNativeError::typ().with_message("Missing test argument"))?;
        let comma = syntax
            .descendants_tokens(Direction::Next)
            .find(|token| token.kind() == JsSyntaxKind::COMMA)
            .ok_or_else(|| JsNativeError::typ().with_message("Missing test comma"))?;
        let argument = JsAstNode::from_node(argument, &mut context);
        let comma = JsAstToken::from_token(comma, &mut context);
        let mutation =
            create_mutation(&JsValue::undefined(), slice::from_ref(&root), &mut context)?;
        assert!(
            JsMutation::change(&mutation, &[argument], ElementCategory::Node, false)?
                .is_undefined()
        );
        JsMutation::change(
            &mutation,
            slice::from_ref(&comma),
            ElementCategory::Token,
            false,
        )?;
        assert_eq!(
            JsMutation::take(&mutation, &syntax)?
                .commit()
                .text_with_trivia(),
            "f(b);"
        );
        assert!(JsMutation::take(&mutation, &syntax).is_err());
        assert!(JsMutation::change(&mutation, &[comma], ElementCategory::Token, false).is_err());
        Ok(())
    }

    #[test]
    fn mutation_arguments_and_submission_are_checked() -> JsResult<()> {
        let (mut context, syntax, root) = setup()?;
        let token = syntax
            .first_token()
            .ok_or_else(|| JsNativeError::typ().with_message("Missing test token"))?;
        let target = JsAstToken::from_token(token, &mut context);
        let replacement = JsAstToken::from_token(
            JsSyntaxToken::new_detached(JsSyntaxKind::IDENT, "g", [], []),
            &mut context,
        );
        let mutation =
            create_mutation(&JsValue::undefined(), slice::from_ref(&root), &mut context)?;
        for source in ["f(a,b);", "longer_call(with_more_arguments);"] {
            let foreign = parse_script(source, JsParserOptions::default()).syntax();
            assert!(JsMutation::take(&mutation, &foreign).is_err());
        }
        assert!(
            JsMutation::change(
                &mutation,
                &[target.clone(), root.clone()],
                ElementCategory::Either,
                true
            )
            .is_err()
        );
        assert!(
            JsMutation::change(
                &mutation,
                slice::from_ref(&mutation),
                ElementCategory::Either,
                false
            )
            .is_err()
        );
        assert!(
            JsMutation::change(
                &mutation,
                slice::from_ref(&target),
                ElementCategory::Node,
                false
            )
            .is_err()
        );
        JsMutation::change(
            &mutation,
            &[target.clone(), replacement],
            ElementCategory::Token,
            true,
        )?;
        assert_eq!(
            JsMutation::take(&mutation, &syntax)?
                .commit()
                .text_with_trivia(),
            "g(a,b);"
        );
        assert!(JsMutation::take(&JsValue::undefined(), &syntax).is_err());
        assert!(JsMutation::take(&root, &syntax).is_err());
        Ok(())
    }

    #[test]
    fn repeated_and_overlapping_edits_match_native_batches() -> JsResult<()> {
        let (mut context, syntax, root) = setup()?;
        let call = syntax
            .descendants()
            .find(|node| node.kind() == JsSyntaxKind::JS_CALL_EXPRESSION)
            .unwrap();
        let node = call.first_child().unwrap();
        let token = node.first_token().unwrap();
        let foreign = parse_script("g(c,d);", JsParserOptions::default()).syntax();
        let mut replacements = foreign
            .descendants()
            .filter(|node| node.kind() == JsSyntaxKind::JS_IDENTIFIER_EXPRESSION);
        let replacement = replacements.next().unwrap();
        let detached = replacements.next().unwrap().detach();
        let replace_node: (JsSyntaxElement, Option<JsSyntaxElement>) =
            (node.clone().into(), Some(replacement.clone().into()));
        let remove_node = (node.clone().into(), None);
        let replace_token = (
            token.clone().into(),
            Some(replacement.first_token().unwrap().into()),
        );
        let remove_token = (token.clone().into(), None);
        let replace_call = (call.clone().into(), Some(call.into()));
        let replace_root = (syntax.clone().into(), Some(syntax.clone().into()));
        let cases = [
            (
                "repeated_node_replacement",
                vec![
                    replace_node.clone(),
                    (node.into(), Some(detached.clone().into())),
                ],
            ),
            (
                "repeated_node_removal",
                vec![remove_node.clone(), remove_node.clone()],
            ),
            (
                "repeated_token_replacement",
                vec![
                    replace_token.clone(),
                    (token.into(), Some(detached.first_token().unwrap().into())),
                ],
            ),
            (
                "repeated_token_removal",
                vec![remove_token.clone(), remove_token],
            ),
            (
                "replace_then_remove",
                vec![replace_node.clone(), remove_node.clone()],
            ),
            (
                "remove_then_replace",
                vec![remove_node.clone(), replace_node],
            ),
            (
                "ancestor_then_descendant",
                vec![replace_call.clone(), remove_node.clone()],
            ),
            ("descendant_then_ancestor", vec![remove_node, replace_call]),
            ("root_then_descendant", vec![replace_root, replace_token]),
        ];
        for (case, changes) in cases {
            let mutation =
                create_mutation(&JsValue::undefined(), slice::from_ref(&root), &mut context)?;
            let mut native = BatchMutation::new(syntax.clone());
            for (target, replacement) in changes {
                let mut args = Vec::new();
                for element in std::iter::once(&target).chain(replacement.iter()) {
                    args.push(match element {
                        JsSyntaxElement::Node(node) => {
                            JsAstNode::from_node(node.clone(), &mut context)
                        }
                        JsSyntaxElement::Token(token) => {
                            JsAstToken::from_token(token.clone(), &mut context)
                        }
                    });
                }
                JsMutation::change(
                    &mutation,
                    &args,
                    ElementCategory::Either,
                    replacement.is_some(),
                )?;
                match replacement {
                    Some(replacement) => native.replace_element(target, replacement),
                    None => native.remove_element(target),
                }
            }
            let (actual_tree, actual_edit) =
                JsMutation::take(&mutation, &syntax)?.commit_with_text_range_and_edit(true);
            let (expected_tree, expected_edit) = native.commit_with_text_range_and_edit(true);
            assert_eq!(
                actual_tree.text_with_trivia(),
                expected_tree.text_with_trivia(),
                "{case}"
            );
            assert_eq!(
                actual_edit.map(|(range, edit)| (range, edit.new_string("f(a,b);"))),
                expected_edit.map(|(range, edit)| (range, edit.new_string("f(a,b);"))),
                "{case}"
            );
        }
        Ok(())
    }
}
