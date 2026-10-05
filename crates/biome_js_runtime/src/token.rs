use crate::ast::JsAstNode;
use biome_js_syntax::{JsSyntaxKind, JsSyntaxToken};
use biome_text_size::TextSize;
use boa_engine::class::{Class, ClassBuilder};
use boa_engine::object::JsObject;
use boa_engine::property::Attribute;
use boa_engine::{
    Context, Finalize, JsData, JsNativeError, JsResult, JsString, JsValue, NativeFunction, Trace,
    js_string,
};

#[derive(Clone, Debug, JsData)]
pub(crate) struct JsAstToken {
    pub(crate) token: JsSyntaxToken,
}

impl Finalize for JsAstToken {}

// SAFETY: Rowan tokens contain no Boa-managed values.
unsafe impl Trace for JsAstToken {
    boa_engine::gc::empty_trace!();
}

impl JsAstToken {
    pub(crate) fn register(context: &mut Context) -> JsResult<()> {
        context.register_global_class::<Self>()?;
        context
            .global_object()
            .delete_property_or_throw(js_string!(Self::NAME), context)?;
        Ok(())
    }

    /// Wraps a token. Call `register` first to attach the token class prototype.
    pub(crate) fn from_token(token: JsSyntaxToken, context: &mut Context) -> JsValue {
        let prototype = context
            .get_global_class::<Self>()
            .map(|class| class.prototype());
        JsObject::from_proto_and_data(prototype, Self { token }).into()
    }

    pub(crate) fn from_value(value: &JsValue) -> Option<Self> {
        let object = value.as_object()?;
        let token = object.downcast_ref::<Self>()?;
        Some(token.clone())
    }

    fn get_kind(this: &JsValue, _args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let token = Self::from_value(this).ok_or_else(|| {
            JsNativeError::typ().with_message("Cannot read token.kind because this value is not a token. Use a token returned by node.token(field), node.childrenWithTokens(), or factory.token().")
        })?;
        Ok(JsString::from(format!("{:?}", token.token.kind())).into())
    }

    fn get_text(this: &JsValue, _args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let token = Self::from_value(this).ok_or_else(|| {
            JsNativeError::typ().with_message("Cannot read token.text because this value is not a token. Use a token returned by node.token(field), node.childrenWithTokens(), or factory.token().")
        })?;
        Ok(JsString::from(token.token.text_trimmed()).into())
    }

    fn get_parent(this: &JsValue, _args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let token = Self::from_value(this).ok_or_else(|| {
            JsNativeError::typ().with_message("Cannot read token.parent because this value is not a token. Use a token returned by node.token(field), node.childrenWithTokens(), or factory.token().")
        })?;
        Ok(token
            .token
            .parent()
            .map_or_else(JsValue::undefined, |parent| {
                JsAstNode::from_node(parent, context)
            }))
    }
}

impl Class for JsAstToken {
    const NAME: &'static str = "__JsAstToken";

    fn init(class: &mut ClassBuilder<'_>) -> JsResult<()> {
        let kind =
            NativeFunction::from_fn_ptr(Self::get_kind).to_js_function(class.context().realm());
        let text =
            NativeFunction::from_fn_ptr(Self::get_text).to_js_function(class.context().realm());
        let parent =
            NativeFunction::from_fn_ptr(Self::get_parent).to_js_function(class.context().realm());
        class
            .accessor(js_string!("kind"), Some(kind), None, Attribute::ENUMERABLE)
            .accessor(js_string!("text"), Some(text), None, Attribute::ENUMERABLE)
            .accessor(js_string!("parent"), Some(parent), None, Attribute::empty());
        Ok(())
    }

    fn data_constructor(
        _new_target: &JsValue,
        _args: &[JsValue],
        _context: &mut Context,
    ) -> JsResult<Self> {
        Err(JsNativeError::typ()
            .with_message("Tokens do not have a public constructor. Create a token with factory.token(kind, text), or obtain one from node.token(field) or node.childrenWithTokens().")
            .into())
    }
}

pub(crate) fn factory_token(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
) -> JsResult<JsValue> {
    let (kind, text) = match args {
        [kind] => (kind, None),
        [kind, text] => (kind, Some(primitive_string(text, "text")?)),
        _ => {
            return Err(JsNativeError::typ()
                .with_message("factory.token() received the wrong number of arguments. Pass a kind string as the first argument and, when needed, a text string as the second argument, with no additional arguments.")
                .into());
        }
    };
    let name = primitive_string(kind, "kind")?;
    let kind = JsAstNode::token_kind_from_name(&name).ok_or_else(|| {
        JsNativeError::typ().with_message(format!("factory.token() does not support the kind {name:?}. Use a name from the JsTokenKind definition, such as \"IDENT\" or \"LET_KW\"."))
    })?;
    let token = create_token(kind, text.as_deref())?;
    Ok(JsAstToken::from_token(token, context))
}

fn primitive_string(value: &JsValue, argument: &str) -> JsResult<String> {
    value
        .as_string()
        .ok_or_else(|| {
            JsNativeError::typ().with_message(format!("The {argument} argument to factory.token() is not a string. Pass a string value, not a String object."))
        })?
        .to_std_string()
        .map_err(|_| {
            JsNativeError::typ()
                .with_message(format!("The {argument} argument to factory.token() contains an incomplete Unicode character. Check the string's \\u escapes and supply complete Unicode characters."))
                .into()
        })
}

/// Creates a detached token with no attached trivia.
/// Fixed-spelling tokens may omit `text`; identifiers and literals must supply it.
/// Supplied identifier and literal text is preserved without parsing or lexical validation.
fn create_token(kind: JsSyntaxKind, text: Option<&str>) -> JsResult<JsSyntaxToken> {
    // These fixed spellings are absent from the kind display table.
    let spelling = match kind {
        JsSyntaxKind::HASH => Some("#"),
        JsSyntaxKind::DOLLAR_CURLY => Some("${"),
        JsSyntaxKind::META => Some("meta"),
        JsSyntaxKind::TARGET => Some("target"),
        kind if kind.is_punct() || kind.is_keyword() => kind.to_string(),
        _ => None,
    };
    let text = if let Some(spelling) = spelling {
        if let Some(text) = text
            && text != spelling
        {
            return Err(JsNativeError::typ()
                .with_message(format!(
                    "factory.token() requires the spelling {spelling:?} for kind {kind:?}, but the text argument is {text:?}. Omit the text argument or change it to {spelling:?}."
                ))
                .into());
        }
        spelling
    } else {
        text.ok_or_else(|| {
            JsNativeError::typ().with_message(format!("factory.token() cannot create kind {kind:?} without text. Pass the token text as a second string argument."))
        })?
    };
    TextSize::try_from(text.len()).map_err(|_| {
        JsNativeError::typ().with_message("The text argument to factory.token() is too long to store in a token. Pass a shorter string as the second argument.")
    })?;

    Ok(JsSyntaxToken::new_detached(kind, text, [], []))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_construction_preserves_supplied_text() -> JsResult<()> {
        for (kind, text) in [
            (JsSyntaxKind::IDENT, "name"),
            (JsSyntaxKind::IDENT, "meta"),
            (JsSyntaxKind::IDENT, "async"),
            (JsSyntaxKind::IDENT, "default"),
            (JsSyntaxKind::IDENT, "let"),
            (JsSyntaxKind::IDENT, "l\\u0065t"),
            (JsSyntaxKind::IDENT, "\\u0061"),
            (JsSyntaxKind::JS_NUMBER_LITERAL, "1_000"),
            (JsSyntaxKind::JS_BIGINT_LITERAL, "123n"),
            (JsSyntaxKind::JS_STRING_LITERAL, "\"value\""),
            (JsSyntaxKind::JS_REGEX_LITERAL, "/a+/gi"),
            (JsSyntaxKind::JSX_IDENT, "custom-element"),
            (JsSyntaxKind::JSX_IDENT, "class"),
            (JsSyntaxKind::JSX_STRING_LITERAL, "\"value\""),
            (JsSyntaxKind::JSX_TEXT_LITERAL, "some text"),
            (JsSyntaxKind::IDENT, ""),
            (JsSyntaxKind::IDENT, "a b"),
            (JsSyntaxKind::IDENT, " a"),
            (JsSyntaxKind::IDENT, "a/* comment */"),
            (JsSyntaxKind::JS_NUMBER_LITERAL, "1e"),
            (JsSyntaxKind::JS_STRING_LITERAL, "\"unterminated"),
            (JsSyntaxKind::JSX_TEXT_LITERAL, "a{b}"),
        ] {
            let token = create_token(kind, Some(text))?;
            assert_eq!(token.kind(), kind);
            assert_eq!(token.text(), text);
            assert!(token.parent().is_none());
        }
        for (kind, text) in [
            (JsSyntaxKind::COMMA, ","),
            (JsSyntaxKind::SHR, ">>"),
            (JsSyntaxKind::HASH, "#"),
            (JsSyntaxKind::DOLLAR_CURLY, "${"),
            (JsSyntaxKind::META, "meta"),
            (JsSyntaxKind::TARGET, "target"),
        ] {
            assert_eq!(create_token(kind, None)?.text(), text);
            assert!(create_token(kind, Some("wrong")).is_err());
        }
        assert!(create_token(JsSyntaxKind::IDENT, None).is_err());
        Ok(())
    }

    #[test]
    fn factory_requires_primitive_strings() -> JsResult<()> {
        let mut context = Context::default();
        JsAstToken::register(&mut context)?;
        for args in [
            vec![],
            vec![JsValue::from(1)],
            vec![js_string!("IDENT").into(), JsValue::undefined()],
            vec![js_string!("IDENT").into(), JsValue::from(1)],
            vec![
                js_string!("IDENT").into(),
                JsString::from(&[0xd800u16][..]).into(),
            ],
            vec![js_string!("EOF").into()],
            vec![js_string!("JS_MODULE").into()],
        ] {
            assert!(factory_token(&JsValue::undefined(), &args, &mut context).is_err());
        }
        let value = factory_token(
            &JsValue::undefined(),
            &[js_string!("COMMA").into()],
            &mut context,
        )?;
        assert!(JsAstToken::from_value(&value).is_some());
        assert_eq!(
            JsAstToken::get_text(&value, &[], &mut context)?.as_string(),
            Some(js_string!(","))
        );
        Ok(())
    }
}
