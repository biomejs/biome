use std::cell::Ref;

use anyhow::{bail, Context, Result};
use rome_js_syntax::{
    JsLanguage, JsSyntaxElement, JsSyntaxKind, JsSyntaxList, JsSyntaxNode, JsSyntaxToken,
    JsSyntaxTrivia, SyntaxNodeText, TextRange, TextSize, WalkEvent,
};
use rome_rowan::{
    syntax::{Descendants, DescendantsTokens, DescendantsWithTokens, Preorder},
    BatchMutation, Direction, RawSyntaxKind, SyntaxSlot, SyntaxTriviaPiece,
};

pub(crate) trait FromV8<'s>: Sized {
    fn from_v8(scope: &mut v8::HandleScope<'s>, value: v8::Local<'s, v8::Value>) -> Result<Self>;
}

pub(crate) trait ToV8<'s> {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> Result<v8::Local<'s, v8::Value>>;
}

macro_rules! impl_convert_serde {
    ($ty:ty) => {
        impl<'s> FromV8<'s> for $ty {
            fn from_v8(
                scope: &mut v8::HandleScope<'s>,
                value: v8::Local<'s, v8::Value>,
            ) -> Result<Self> {
                serde_v8::from_v8(scope, value)
                    .context("could not deserialize object from JS value")
            }
        }

        impl<'s> ToV8<'s> for $ty {
            fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> Result<v8::Local<'s, v8::Value>> {
                serde_v8::to_v8(scope, self).context("could not serialize object to JS value")
            }
        }
    };
}

impl_convert_serde!(bool);
impl_convert_serde!(i16);
impl_convert_serde!(i32);
impl_convert_serde!(i64);
impl_convert_serde!(isize);
impl_convert_serde!(u16);
impl_convert_serde!(u32);
impl_convert_serde!(u64);
impl_convert_serde!(usize);
impl_convert_serde!(f32);
impl_convert_serde!(f64);
impl_convert_serde!(String);

impl<'s> FromV8<'s> for &'s str {
    fn from_v8(scope: &mut v8::HandleScope<'s>, value: v8::Local<'s, v8::Value>) -> Result<Self> {
        serde_v8::from_v8(scope, value).context("could not deserialize object from JS value")
    }
}

impl<'s, 'str> ToV8<'s> for &'str str {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> Result<v8::Local<'s, v8::Value>> {
        serde_v8::to_v8(scope, self).context("could not serialize object to JS value")
    }
}

impl<'s, T: FromV8<'s>> FromV8<'s> for Option<T> {
    fn from_v8(scope: &mut v8::HandleScope<'s>, value: v8::Local<'s, v8::Value>) -> Result<Self> {
        if !value.is_null_or_undefined() {
            T::from_v8(scope, value).map(Some)
        } else {
            Ok(None)
        }
    }
}

impl<'s, T: ToV8<'s>> ToV8<'s> for Option<T> {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> Result<v8::Local<'s, v8::Value>> {
        if let Some(value) = self {
            value.to_v8(scope)
        } else {
            Ok(v8::undefined(scope).into())
        }
    }
}

impl_convert_serde!(TextRange);
impl_convert_serde!(TextSize);
impl_convert_serde!(RawSyntaxKind);
impl_convert_serde!(Direction);

macro_rules! impl_convert_native {
    ($ty:ty) => {
        impl<'s> FromV8<'s> for std::cell::Ref<'s, $ty> {
            fn from_v8(
                _scope: &mut v8::HandleScope<'s>,
                value: v8::Local<'s, v8::Value>,
            ) -> anyhow::Result<Self> {
                crate::registry::borrow_native(value)
            }
        }

        impl<'s> FromV8<'s> for std::cell::RefMut<'s, $ty> {
            fn from_v8(
                _scope: &mut v8::HandleScope<'s>,
                value: v8::Local<'s, v8::Value>,
            ) -> anyhow::Result<Self> {
                crate::registry::borrow_mut_native(value)
            }
        }

        impl<'s> ToV8<'s> for $ty {
            fn to_v8(
                self,
                scope: &mut v8::HandleScope<'s>,
            ) -> anyhow::Result<v8::Local<'s, v8::Value>> {
                crate::registry::instantiate(scope, self).map(Into::into)
            }
        }
    };
}

pub(crate) use impl_convert_native;

impl_convert_native!(SyntaxNodeText);
impl_convert_native!(SyntaxTriviaPiece<JsLanguage>);
impl_convert_native!(JsSyntaxTrivia);
impl_convert_native!(JsSyntaxList);
impl_convert_native!(JsSyntaxNode);
impl_convert_native!(JsSyntaxToken);
impl_convert_native!(SyntaxSlot<JsLanguage>);
impl_convert_native!(Preorder<JsLanguage>);
impl_convert_native!(Descendants<JsLanguage>);
impl_convert_native!(DescendantsTokens<JsLanguage>);
impl_convert_native!(DescendantsWithTokens<JsLanguage>);
impl_convert_native!(BatchMutation<JsLanguage>);

impl<'s> FromV8<'s> for JsSyntaxElement {
    fn from_v8(scope: &mut v8::HandleScope<'s>, value: v8::Local<'s, v8::Value>) -> Result<Self> {
        if let Ok(node) = Ref::<JsSyntaxNode>::from_v8(scope, value) {
            return Ok(JsSyntaxElement::Node(node.clone()));
        }

        if let Ok(node) = Ref::<JsSyntaxToken>::from_v8(scope, value) {
            return Ok(JsSyntaxElement::Token(node.clone()));
        }

        bail!("JS value is not a JsSyntaxNode or JsSyntaxToken")
    }
}

impl<'s> ToV8<'s> for JsSyntaxElement {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> Result<v8::Local<'s, v8::Value>> {
        match self {
            JsSyntaxElement::Node(node) => node.to_v8(scope),
            JsSyntaxElement::Token(token) => token.to_v8(scope),
        }
    }
}

impl<'s> ToV8<'s> for JsSyntaxKind {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> Result<v8::Local<'s, v8::Value>> {
        crate::registry::instantiate(scope, self).map(Into::into)
    }
}

impl<'s, T: ToV8<'s>> ToV8<'s> for WalkEvent<T> {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> Result<v8::Local<'s, v8::Value>> {
        let (kind, elem) = match self {
            WalkEvent::Enter(elem) => ("enter", elem),
            WalkEvent::Leave(elem) => ("leave", elem),
        };

        let result = v8::Object::new(scope);

        let key = v8::String::new(scope, "kind").context("failed to allocate string")?;
        let value = v8::String::new(scope, kind).context("failed to allocate string")?;
        result
            .set(scope, key.into(), value.into())
            .context("failed to set kind")?;

        let key = v8::String::new(scope, "elem").context("failed to allocate string")?;
        let value = elem.to_v8(scope)?;
        result
            .set(scope, key.into(), value)
            .context("failed to set elem")?;

        Ok(result.into())
    }
}
