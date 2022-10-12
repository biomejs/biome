//! Generated file, do not edit by hand, see `xtask/codegen`

use super::TemplateRegistry;
use crate::convert::{FromV8, ToV8};
use rome_rowan::{AstNode as _, AstNodeList as _, AstSeparatedList as _};
pub(super) fn register_interfaces(
    scope: &mut v8::HandleScope<'_, ()>,
    global: v8::Local<'_, v8::ObjectTemplate>,
    registry: &mut TemplateRegistry,
) {
    registry
        .build_enum::<rome_json_syntax::JsonSyntaxKind>(scope, global, "JsonSyntaxKind")
        .variant("EOF", rome_json_syntax::JsonSyntaxKind::EOF)
        .variant("COLON", rome_json_syntax::JsonSyntaxKind::COLON)
        .variant("COMMA", rome_json_syntax::JsonSyntaxKind::COMMA)
        .variant("L_PAREN", rome_json_syntax::JsonSyntaxKind::L_PAREN)
        .variant("R_PAREN", rome_json_syntax::JsonSyntaxKind::R_PAREN)
        .variant("L_CURLY", rome_json_syntax::JsonSyntaxKind::L_CURLY)
        .variant("R_CURLY", rome_json_syntax::JsonSyntaxKind::R_CURLY)
        .variant("L_BRACK", rome_json_syntax::JsonSyntaxKind::L_BRACK)
        .variant("R_BRACK", rome_json_syntax::JsonSyntaxKind::R_BRACK)
        .variant("NULL_KW", rome_json_syntax::JsonSyntaxKind::NULL_KW)
        .variant("TRUE_KW", rome_json_syntax::JsonSyntaxKind::TRUE_KW)
        .variant("FALSE_KW", rome_json_syntax::JsonSyntaxKind::FALSE_KW)
        .variant(
            "JSON_STRING_LITERAL",
            rome_json_syntax::JsonSyntaxKind::JSON_STRING_LITERAL,
        )
        .variant(
            "JSON_NUMBER_LITERAL",
            rome_json_syntax::JsonSyntaxKind::JSON_NUMBER_LITERAL,
        )
        .variant("ERROR_TOKEN", rome_json_syntax::JsonSyntaxKind::ERROR_TOKEN)
        .variant("NEWLINE", rome_json_syntax::JsonSyntaxKind::NEWLINE)
        .variant("WHITESPACE", rome_json_syntax::JsonSyntaxKind::WHITESPACE)
        .variant("JSON_ROOT", rome_json_syntax::JsonSyntaxKind::JSON_ROOT)
        .variant("JSON_VALUE", rome_json_syntax::JsonSyntaxKind::JSON_VALUE)
        .variant("JSON_NUMBER", rome_json_syntax::JsonSyntaxKind::JSON_NUMBER)
        .variant("JSON_STRING", rome_json_syntax::JsonSyntaxKind::JSON_STRING)
        .variant(
            "JSON_BOOLEAN",
            rome_json_syntax::JsonSyntaxKind::JSON_BOOLEAN,
        )
        .variant("JSON_NULL", rome_json_syntax::JsonSyntaxKind::JSON_NULL)
        .variant("JSON_ARRAY", rome_json_syntax::JsonSyntaxKind::JSON_ARRAY)
        .variant("JSON_OBJECT", rome_json_syntax::JsonSyntaxKind::JSON_OBJECT)
        .variant(
            "JSON_MEMBER_LIST",
            rome_json_syntax::JsonSyntaxKind::JSON_MEMBER_LIST,
        )
        .variant("JSON_MEMBER", rome_json_syntax::JsonSyntaxKind::JSON_MEMBER)
        .variant(
            "JSON_ARRAY_ELEMENT_LIST",
            rome_json_syntax::JsonSyntaxKind::JSON_ARRAY_ELEMENT_LIST,
        )
        .variant(
            "JSON_UNKNOWN",
            rome_json_syntax::JsonSyntaxKind::JSON_UNKNOWN,
        )
        .finish(scope);
    registry
        .build_class::<rome_json_syntax::JsonArray>(scope, global, "JsonArray")
        .extends::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>(scope)
        .method(scope, "l_brack_token", JsonArray_l_brack_token)
        .method(scope, "elements", JsonArray_elements)
        .method(scope, "r_brack_token", JsonArray_r_brack_token)
        .finish(scope);
    registry
        .build_class::<rome_json_syntax::JsonBoolean>(scope, global, "JsonBoolean")
        .extends::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>(scope)
        .method(scope, "true_token", JsonBoolean_true_token)
        .method(scope, "false_token", JsonBoolean_false_token)
        .finish(scope);
    registry
        .build_class::<rome_json_syntax::JsonMember>(scope, global, "JsonMember")
        .extends::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>(scope)
        .method(scope, "key", JsonMember_key)
        .method(scope, "colon_token", JsonMember_colon_token)
        .method(scope, "value", JsonMember_value)
        .finish(scope);
    registry
        .build_class::<rome_json_syntax::JsonNull>(scope, global, "JsonNull")
        .extends::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>(scope)
        .method(scope, "null_token", JsonNull_null_token)
        .finish(scope);
    registry
        .build_class::<rome_json_syntax::JsonNumber>(scope, global, "JsonNumber")
        .extends::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>(scope)
        .method(
            scope,
            "json_number_literal_token",
            JsonNumber_json_number_literal_token,
        )
        .finish(scope);
    registry
        .build_class::<rome_json_syntax::JsonObject>(scope, global, "JsonObject")
        .extends::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>(scope)
        .method(scope, "l_curly_token", JsonObject_l_curly_token)
        .method(scope, "json_member_list", JsonObject_json_member_list)
        .method(scope, "r_curly_token", JsonObject_r_curly_token)
        .finish(scope);
    registry
        .build_class::<rome_json_syntax::JsonRoot>(scope, global, "JsonRoot")
        .extends::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>(scope)
        .method(scope, "json_value", JsonRoot_json_value)
        .finish(scope);
    registry
        .build_class::<rome_json_syntax::JsonString>(scope, global, "JsonString")
        .extends::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>(scope)
        .method(
            scope,
            "json_string_literal_token",
            JsonString_json_string_literal_token,
        )
        .finish(scope);
    registry
        .build_class::<rome_json_syntax::JsonUnknown>(scope, global, "JsonUnknown")
        .finish(scope);
    registry
        .build_class::<rome_json_syntax::JsonArrayElementList>(
            scope,
            global,
            "JsonArrayElementList",
        )
        .method(scope, "iter", JsonArrayElementList_iter)
        .finish(scope);
    registry
        .build_class::<rome_json_syntax::JsonMemberList>(scope, global, "JsonMemberList")
        .method(scope, "iter", JsonMemberList_iter)
        .finish(scope);
    registry
        .build_interface::<rome_rowan::AstSeparatedListNodesIterator<
            rome_json_syntax::JsonLanguage,
            rome_json_syntax::JsonMember,
        >>(scope)
        .iterable(scope, AstSeparatedListNodesIterator_next)
        .finish(scope);
    registry
        .build_interface::<rome_rowan::AstSeparatedListNodesIterator<
            rome_json_syntax::JsonLanguage,
            rome_json_syntax::JsonValue,
        >>(scope)
        .iterable(scope, AstSeparatedListNodesIterator_next)
        .finish(scope);
}
#[allow(non_snake_case)]
fn AstSeparatedListNodesIterator_next<'s, T: ToV8<'s>>(
    item: rome_rowan::SyntaxResult<T>,
    scope: &mut v8::HandleScope<'s>,
) -> anyhow::Result<v8::Local<'s, v8::Value>> {
    ToV8::to_v8(item?, scope)
}
impl<'s> ToV8<'s> for rome_json_syntax::JsonArray {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            rome_json_syntax::JsonArray,
            rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn JsonArray_l_brack_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = rome_json_syntax::JsonArray::cast_ref(&*this).unwrap();
    let result = this.l_brack_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn JsonArray_elements<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = rome_json_syntax::JsonArray::cast_ref(&*this).unwrap();
    let result = this.elements();
    let result = ToV8::to_v8(result, scope).unwrap();
    res.set(result);
}
#[allow(non_snake_case)]
fn JsonArray_r_brack_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = rome_json_syntax::JsonArray::cast_ref(&*this).unwrap();
    let result = this.r_brack_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for rome_json_syntax::JsonBoolean {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            rome_json_syntax::JsonBoolean,
            rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn JsonBoolean_true_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = rome_json_syntax::JsonBoolean::cast_ref(&*this).unwrap();
    let result = this.true_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn JsonBoolean_false_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = rome_json_syntax::JsonBoolean::cast_ref(&*this).unwrap();
    let result = this.false_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for rome_json_syntax::JsonMember {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            rome_json_syntax::JsonMember,
            rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn JsonMember_key<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = rome_json_syntax::JsonMember::cast_ref(&*this).unwrap();
    let result = this.key();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn JsonMember_colon_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = rome_json_syntax::JsonMember::cast_ref(&*this).unwrap();
    let result = this.colon_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn JsonMember_value<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = rome_json_syntax::JsonMember::cast_ref(&*this).unwrap();
    let result = this.value();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for rome_json_syntax::JsonNull {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            rome_json_syntax::JsonNull,
            rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn JsonNull_null_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = rome_json_syntax::JsonNull::cast_ref(&*this).unwrap();
    let result = this.null_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for rome_json_syntax::JsonNumber {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            rome_json_syntax::JsonNumber,
            rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn JsonNumber_json_number_literal_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = rome_json_syntax::JsonNumber::cast_ref(&*this).unwrap();
    let result = this.json_number_literal_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for rome_json_syntax::JsonObject {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            rome_json_syntax::JsonObject,
            rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn JsonObject_l_curly_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = rome_json_syntax::JsonObject::cast_ref(&*this).unwrap();
    let result = this.l_curly_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn JsonObject_json_member_list<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = rome_json_syntax::JsonObject::cast_ref(&*this).unwrap();
    let result = this.json_member_list();
    let result = ToV8::to_v8(result, scope).unwrap();
    res.set(result);
}
#[allow(non_snake_case)]
fn JsonObject_r_curly_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = rome_json_syntax::JsonObject::cast_ref(&*this).unwrap();
    let result = this.r_curly_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for rome_json_syntax::JsonRoot {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            rome_json_syntax::JsonRoot,
            rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn JsonRoot_json_value<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = rome_json_syntax::JsonRoot::cast_ref(&*this).unwrap();
    let result = this.json_value();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for rome_json_syntax::JsonString {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            rome_json_syntax::JsonString,
            rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn JsonString_json_string_literal_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<rome_rowan::SyntaxNode<rome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = rome_json_syntax::JsonString::cast_ref(&*this).unwrap();
    let result = this.json_string_literal_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for rome_json_syntax::JsonValue {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        match self {
            Self::JsonArray(node) => ToV8::to_v8(node, scope),
            Self::JsonBoolean(node) => ToV8::to_v8(node, scope),
            Self::JsonNull(node) => ToV8::to_v8(node, scope),
            Self::JsonNumber(node) => ToV8::to_v8(node, scope),
            Self::JsonObject(node) => ToV8::to_v8(node, scope),
            Self::JsonString(node) => ToV8::to_v8(node, scope),
            Self::JsonUnknown(node) => ToV8::to_v8(node, scope),
        }
    }
}
crate::convert::impl_convert_native!(rome_json_syntax::JsonUnknown);
crate::convert::impl_convert_native!(rome_json_syntax::JsonArrayElementList);
#[allow(non_snake_case)]
fn JsonArrayElementList_iter<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this =
        std::cell::Ref::<rome_json_syntax::JsonArrayElementList>::from_v8(scope, this).unwrap();
    let iter = this.iter();
    let iter = ToV8::to_v8(iter, scope).unwrap();
    res.set(iter);
}
crate::convert::impl_convert_native!(rome_json_syntax::JsonMemberList);
#[allow(non_snake_case)]
fn JsonMemberList_iter<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<rome_json_syntax::JsonMemberList>::from_v8(scope, this).unwrap();
    let iter = this.iter();
    let iter = ToV8::to_v8(iter, scope).unwrap();
    res.set(iter);
}
crate :: convert :: impl_convert_native ! (rome_rowan :: AstSeparatedListNodesIterator < rome_json_syntax :: JsonLanguage , rome_json_syntax :: JsonMember >);
crate :: convert :: impl_convert_native ! (rome_rowan :: AstSeparatedListNodesIterator < rome_json_syntax :: JsonLanguage , rome_json_syntax :: JsonValue >);
