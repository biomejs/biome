use std::cell::Ref;

use rome_js_syntax::{JsLanguage, JsSyntaxNode, WalkEvent};
use rome_rowan::{
    syntax::{Descendants, DescendantsTokens, DescendantsWithTokens, Preorder},
    BatchMutation, SyntaxTriviaPiece,
};

type JsSyntaxTriviaPiece = SyntaxTriviaPiece<JsLanguage>;
type JsPreorder = Preorder<JsLanguage>;
type JsDescendants = Descendants<JsLanguage>;
type JsDescendantsTokens = DescendantsTokens<JsLanguage>;
type JsDescendantsWithTokens = DescendantsWithTokens<JsLanguage>;
type JsWalkEvent = WalkEvent<JsSyntaxNode>;
type JsBatchMutation = BatchMutation<JsLanguage>;

include!("rowan.idl.rs");

#[allow(non_snake_case)]
fn JsSyntaxNode_splice_slots<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut result: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = Ref::<rome_js_syntax::JsSyntaxNode>::from_v8(scope, this).unwrap();
    let this = this.clone();

    let range = args.get(0i32);
    let range: std::ops::Range<usize> =
        serde_v8::from_v8(scope, range).expect("failed to deserialize value from V8");

    let replace_with = args.get(1i32);
    let replace_with =
        v8::Local::<v8::Array>::try_from(replace_with).expect("invalid argument type");
    let length = replace_with.length();
    let replace_with = (0..length).map(|index| {
        let item = replace_with.get_index(scope, index)?;
        let item = rome_js_syntax::JsSyntaxElement::from_v8(scope, item).ok()?;
        Some(item)
    });

    let res = this.splice_slots(range, replace_with);
    let res = ToV8::to_v8(res, scope).unwrap();
    result.set(res);
}
