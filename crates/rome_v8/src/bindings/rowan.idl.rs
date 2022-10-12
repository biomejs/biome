use crate::convert::{FromV8, ToV8};
use super::TemplateRegistry;
pub(super) fn register_interfaces(
    scope: &mut v8::HandleScope<'_, ()>,
    global: v8::Local<'_, v8::ObjectTemplate>,
    registry: &mut TemplateRegistry,
) {
    registry
        .build_class_with_constructor::<
            JsBatchMutation,
        >(scope, global, "JsBatchMutation", JsBatchMutation_constructor)
        .method(scope, "replace_element", JsBatchMutation_replace_element)
        .method(
            scope,
            "replace_element_discard_trivia",
            JsBatchMutation_replace_element_discard_trivia,
        )
        .method(scope, "remove_element", JsBatchMutation_remove_element)
        .finish(scope);
    registry
        .build_interface::<JsDescendants>(scope)
        .iterable(scope, ToV8::to_v8)
        .finish(scope);
    registry
        .build_interface::<JsDescendantsTokens>(scope)
        .iterable(scope, ToV8::to_v8)
        .finish(scope);
    registry
        .build_interface::<JsDescendantsWithTokens>(scope)
        .iterable(scope, ToV8::to_v8)
        .finish(scope);
    registry
        .build_interface::<JsPreorder>(scope)
        .iterable(scope, ToV8::to_v8)
        .finish(scope);
    registry
        .build_class::<rome_js_syntax::JsSyntaxList>(scope, global, "JsSyntaxList")
        .method(scope, "len", JsSyntaxList_len)
        .method(scope, "is_empty", JsSyntaxList_is_empty)
        .method(scope, "first", JsSyntaxList_first)
        .method(scope, "last", JsSyntaxList_last)
        .method(scope, "node", JsSyntaxList_node)
        .finish(scope);
    registry
        .build_class::<rome_js_syntax::JsSyntaxNode>(scope, global, "JsSyntaxNode")
        .method(scope, "kind", JsSyntaxNode_kind)
        .method(scope, "text", JsSyntaxNode_text)
        .method(scope, "text_trimmed", JsSyntaxNode_text_trimmed)
        .method(scope, "text_range", JsSyntaxNode_text_range)
        .method(scope, "text_trimmed_range", JsSyntaxNode_text_trimmed_range)
        .method(scope, "first_leading_trivia", JsSyntaxNode_first_leading_trivia)
        .method(scope, "last_trailing_trivia", JsSyntaxNode_last_trailing_trivia)
        .method(scope, "parent", JsSyntaxNode_parent)
        .method(scope, "grand_parent", JsSyntaxNode_grand_parent)
        .method(scope, "index", JsSyntaxNode_index)
        .method(scope, "first_child", JsSyntaxNode_first_child)
        .method(scope, "last_child", JsSyntaxNode_last_child)
        .method(scope, "first_child_or_token", JsSyntaxNode_first_child_or_token)
        .method(scope, "last_child_or_token", JsSyntaxNode_last_child_or_token)
        .method(scope, "next_sibling", JsSyntaxNode_next_sibling)
        .method(scope, "prev_sibling", JsSyntaxNode_prev_sibling)
        .method(scope, "next_sibling_or_token", JsSyntaxNode_next_sibling_or_token)
        .method(scope, "prev_sibling_or_token", JsSyntaxNode_prev_sibling_or_token)
        .method(scope, "first_token", JsSyntaxNode_first_token)
        .method(scope, "last_token", JsSyntaxNode_last_token)
        .method(scope, "descendants", JsSyntaxNode_descendants)
        .method(scope, "descendants_tokens", JsSyntaxNode_descendants_tokens)
        .method(scope, "descendants_with_tokens", JsSyntaxNode_descendants_with_tokens)
        .method(scope, "preorder", JsSyntaxNode_preorder)
        .method(scope, "covering_element", JsSyntaxNode_covering_element)
        .method(scope, "child_or_token_at_range", JsSyntaxNode_child_or_token_at_range)
        .method(scope, "clone_subtree", JsSyntaxNode_clone_subtree)
        .method(scope, "detach", JsSyntaxNode_detach)
        .method(scope, "splice_slots", JsSyntaxNode_splice_slots)
        .method(scope, "replace_child", JsSyntaxNode_replace_child)
        .method(scope, "into_list", JsSyntaxNode_into_list)
        .method(scope, "has_comments_descendants", JsSyntaxNode_has_comments_descendants)
        .method(scope, "has_comments_direct", JsSyntaxNode_has_comments_direct)
        .method(
            scope,
            "first_or_last_token_have_comments",
            JsSyntaxNode_first_or_last_token_have_comments,
        )
        .method(scope, "has_trailing_comments", JsSyntaxNode_has_trailing_comments)
        .method(scope, "last_token_has_comments", JsSyntaxNode_last_token_has_comments)
        .method(scope, "first_token_has_comments", JsSyntaxNode_first_token_has_comments)
        .method(scope, "has_leading_comments", JsSyntaxNode_has_leading_comments)
        .finish(scope);
    registry
        .build_class::<rome_js_syntax::JsSyntaxToken>(scope, global, "JsSyntaxToken")
        .method(scope, "kind", JsSyntaxToken_kind)
        .method(scope, "text_range", JsSyntaxToken_text_range)
        .method(scope, "text_trimmed_range", JsSyntaxToken_text_trimmed_range)
        .method(scope, "text", JsSyntaxToken_text)
        .method(scope, "text_trimmed", JsSyntaxToken_text_trimmed)
        .method(scope, "parent", JsSyntaxToken_parent)
        .method(scope, "next_sibling_or_token", JsSyntaxToken_next_sibling_or_token)
        .method(scope, "prev_sibling_or_token", JsSyntaxToken_prev_sibling_or_token)
        .method(scope, "next_token", JsSyntaxToken_next_token)
        .method(scope, "prev_token", JsSyntaxToken_prev_token)
        .method(scope, "detach", JsSyntaxToken_detach)
        .method(scope, "leading_trivia", JsSyntaxToken_leading_trivia)
        .method(scope, "trailing_trivia", JsSyntaxToken_trailing_trivia)
        .method(scope, "has_trailing_comments", JsSyntaxToken_has_trailing_comments)
        .method(scope, "has_leading_comments", JsSyntaxToken_has_leading_comments)
        .method(
            scope,
            "has_leading_non_whitespace_trivia",
            JsSyntaxToken_has_leading_non_whitespace_trivia,
        )
        .finish(scope);
    registry
        .build_interface::<rome_js_syntax::JsSyntaxTrivia>(scope)
        .method(scope, "last", JsSyntaxTrivia_last)
        .method(scope, "first", JsSyntaxTrivia_first)
        .method(scope, "text", JsSyntaxTrivia_text)
        .method(scope, "text_range", JsSyntaxTrivia_text_range)
        .method(scope, "is_empty", JsSyntaxTrivia_is_empty)
        .method(scope, "has_skipped", JsSyntaxTrivia_has_skipped)
        .finish(scope);
    registry
        .build_interface::<JsSyntaxTriviaPiece>(scope)
        .method(scope, "text", JsSyntaxTriviaPiece_text)
        .method(scope, "text_len", JsSyntaxTriviaPiece_text_len)
        .method(scope, "text_range", JsSyntaxTriviaPiece_text_range)
        .method(scope, "is_newline", JsSyntaxTriviaPiece_is_newline)
        .method(scope, "is_whitespace", JsSyntaxTriviaPiece_is_whitespace)
        .method(scope, "is_comments", JsSyntaxTriviaPiece_is_comments)
        .method(scope, "is_skipped", JsSyntaxTriviaPiece_is_skipped)
        .method(scope, "token", JsSyntaxTriviaPiece_token)
        .finish(scope);
    registry.build_interface::<JsWalkEvent>(scope).finish(scope);
    registry
        .build_interface::<rome_rowan::SyntaxNodeText>(scope)
        .method(scope, "len", SyntaxNodeText_len)
        .method(scope, "is_empty", SyntaxNodeText_is_empty)
        .method(scope, "to_string", SyntaxNodeText_to_string)
        .finish(scope);
    registry.build_namespace(scope, global, "make").method(scope, "ident", make_ident);
}
#[allow(non_snake_case)]
fn JsBatchMutation_constructor<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    _res: v8::ReturnValue,
) {
    let root = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, args.get(0i32))
        .expect("could not load native object from JS value")
        .clone();
    let res = JsBatchMutation::new(root);
    crate::registry::store_native(scope, args.this(), res);
}
#[allow(non_snake_case)]
fn JsBatchMutation_replace_element<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    _result: v8::ReturnValue,
) {
    let this = args.this().into();
    let mut this = <std::cell::RefMut<JsBatchMutation> as FromV8>::from_v8(scope, this)
        .unwrap();
    let prev_element = <rome_js_syntax::JsSyntaxElement as FromV8>::from_v8(
            scope,
            args.get(0i32),
        )
        .expect("failed to deserialize argument from V8 value");
    let next_element = <rome_js_syntax::JsSyntaxElement as FromV8>::from_v8(
            scope,
            args.get(1i32),
        )
        .expect("failed to deserialize argument from V8 value");
    this.replace_element(prev_element, next_element);
}
#[allow(non_snake_case)]
fn JsBatchMutation_replace_element_discard_trivia<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    _result: v8::ReturnValue,
) {
    let this = args.this().into();
    let mut this = <std::cell::RefMut<JsBatchMutation> as FromV8>::from_v8(scope, this)
        .unwrap();
    let prev_element = <rome_js_syntax::JsSyntaxElement as FromV8>::from_v8(
            scope,
            args.get(0i32),
        )
        .expect("failed to deserialize argument from V8 value");
    let next_element = <rome_js_syntax::JsSyntaxElement as FromV8>::from_v8(
            scope,
            args.get(1i32),
        )
        .expect("failed to deserialize argument from V8 value");
    this.replace_element_discard_trivia(prev_element, next_element);
}
#[allow(non_snake_case)]
fn JsBatchMutation_remove_element<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    _result: v8::ReturnValue,
) {
    let this = args.this().into();
    let mut this = <std::cell::RefMut<JsBatchMutation> as FromV8>::from_v8(scope, this)
        .unwrap();
    let prev_element = <rome_js_syntax::JsSyntaxElement as FromV8>::from_v8(
            scope,
            args.get(0i32),
        )
        .expect("failed to deserialize argument from V8 value");
    this.remove_element(prev_element);
}
#[allow(non_snake_case)]
fn JsSyntaxList_len<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxList,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.len();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxList_is_empty<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxList,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.is_empty();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxList_first<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxList,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    if let Some(res) = this.first() {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxList_last<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxList,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    if let Some(res) = this.last() {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxList_node<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxList,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.node();
    let res = res.clone();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_kind<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.kind();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_text<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.text();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_text_trimmed<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.text_trimmed();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_text_range<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.text_range();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_text_trimmed_range<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.text_trimmed_range();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_first_leading_trivia<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    if let Some(res) = this.first_leading_trivia() {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxNode_last_trailing_trivia<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    if let Some(res) = this.last_trailing_trivia() {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxNode_parent<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    if let Some(res) = this.parent() {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxNode_grand_parent<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    if let Some(res) = this.grand_parent() {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxNode_index<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.index();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_first_child<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    if let Some(res) = this.first_child() {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxNode_last_child<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    if let Some(res) = this.last_child() {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxNode_first_child_or_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.first_child_or_token();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_last_child_or_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.last_child_or_token();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_next_sibling<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    if let Some(res) = this.next_sibling() {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxNode_prev_sibling<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    if let Some(res) = this.prev_sibling() {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxNode_next_sibling_or_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.next_sibling_or_token();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_prev_sibling_or_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.prev_sibling_or_token();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_first_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    if let Some(res) = this.first_token() {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxNode_last_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    if let Some(res) = this.last_token() {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxNode_descendants<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.descendants();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_descendants_tokens<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let direction = if let Some(direction)
        = <Option<rome_rowan::Direction> as FromV8>::from_v8(scope, args.get(0i32))
            .expect("failed to deserialize argument from V8 value")
    {
        direction
    } else {
        rome_rowan::Direction::Next
    };
    let res = this.descendants_tokens(direction);
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_descendants_with_tokens<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let direction = if let Some(direction)
        = <Option<rome_rowan::Direction> as FromV8>::from_v8(scope, args.get(0i32))
            .expect("failed to deserialize argument from V8 value")
    {
        direction
    } else {
        rome_rowan::Direction::Next
    };
    let res = this.descendants_with_tokens(direction);
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_preorder<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.preorder();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_covering_element<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let range = <rome_rowan::TextRange as FromV8>::from_v8(scope, args.get(0i32))
        .expect("failed to deserialize argument from V8 value");
    let res = this.covering_element(range);
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_child_or_token_at_range<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let range = <rome_rowan::TextRange as FromV8>::from_v8(scope, args.get(0i32))
        .expect("failed to deserialize argument from V8 value");
    let res = this.child_or_token_at_range(range);
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_clone_subtree<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.clone_subtree();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_detach<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let this = this.clone();
    let res = this.detach();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_replace_child<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let this = this.clone();
    let prev_element = <rome_js_syntax::JsSyntaxElement as FromV8>::from_v8(
            scope,
            args.get(0i32),
        )
        .expect("failed to deserialize argument from V8 value");
    let next_element = <rome_js_syntax::JsSyntaxElement as FromV8>::from_v8(
            scope,
            args.get(1i32),
        )
        .expect("failed to deserialize argument from V8 value");
    if let Some(res) = this.replace_child(prev_element, next_element) {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxNode_into_list<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let this = this.clone();
    let res = this.into_list();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_has_comments_descendants<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.has_comments_descendants();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_has_comments_direct<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.has_comments_direct();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_first_or_last_token_have_comments<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.first_or_last_token_have_comments();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_has_trailing_comments<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.has_trailing_comments();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_last_token_has_comments<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.last_token_has_comments();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_first_token_has_comments<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.first_token_has_comments();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxNode_has_leading_comments<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxNode,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.has_leading_comments();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxToken_kind<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxToken,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.kind();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxToken_text_range<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxToken,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.text_range();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxToken_text_trimmed_range<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxToken,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.text_trimmed_range();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxToken_text<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxToken,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.text();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxToken_text_trimmed<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxToken,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.text_trimmed();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxToken_parent<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxToken,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    if let Some(res) = this.parent() {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxToken_next_sibling_or_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxToken,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.next_sibling_or_token();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxToken_prev_sibling_or_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxToken,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.prev_sibling_or_token();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxToken_next_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxToken,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    if let Some(res) = this.next_token() {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxToken_prev_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxToken,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    if let Some(res) = this.prev_token() {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxToken_detach<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxToken,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let this = this.clone();
    let res = this.detach();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxToken_leading_trivia<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxToken,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.leading_trivia();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxToken_trailing_trivia<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxToken,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.trailing_trivia();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxToken_has_trailing_comments<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxToken,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.has_trailing_comments();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxToken_has_leading_comments<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxToken,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.has_leading_comments();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxToken_has_leading_non_whitespace_trivia<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxToken,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.has_leading_non_whitespace_trivia();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxTrivia_last<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxTrivia,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    if let Some(res) = this.last() {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxTrivia_first<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxTrivia,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    if let Some(res) = this.first() {
        let res = ToV8::to_v8(res, scope)
            .expect("failed to serialize result to JS value");
        result.set(res);
    }
}
#[allow(non_snake_case)]
fn JsSyntaxTrivia_text<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxTrivia,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.text();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxTrivia_text_range<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxTrivia,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.text_range();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxTrivia_is_empty<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxTrivia,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.is_empty();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxTrivia_has_skipped<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_js_syntax::JsSyntaxTrivia,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.has_skipped();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxTriviaPiece_text<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<JsSyntaxTriviaPiece> as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.text();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxTriviaPiece_text_len<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<JsSyntaxTriviaPiece> as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.text_len();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxTriviaPiece_text_range<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<JsSyntaxTriviaPiece> as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.text_range();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxTriviaPiece_is_newline<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<JsSyntaxTriviaPiece> as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.is_newline();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxTriviaPiece_is_whitespace<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<JsSyntaxTriviaPiece> as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.is_whitespace();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxTriviaPiece_is_comments<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<JsSyntaxTriviaPiece> as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.is_comments();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxTriviaPiece_is_skipped<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<JsSyntaxTriviaPiece> as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.is_skipped();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn JsSyntaxTriviaPiece_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<JsSyntaxTriviaPiece> as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.token();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn SyntaxNodeText_len<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_rowan::SyntaxNodeText,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.len();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn SyntaxNodeText_is_empty<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_rowan::SyntaxNodeText,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.is_empty();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn SyntaxNodeText_to_string<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = <std::cell::Ref<
        rome_rowan::SyntaxNodeText,
    > as FromV8>::from_v8(scope, this)
        .unwrap();
    let res = this.to_string();
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
#[allow(non_snake_case)]
fn make_ident<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let text = <String as FromV8>::from_v8(scope, args.get(0i32))
        .expect("failed to deserialize argument from V8 value");
    let res = rome_js_factory::make::ident(&text);
    let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
    result.set(res);
}
