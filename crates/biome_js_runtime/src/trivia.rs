//! Trivia exposed to plugins: `token.leadingTrivia()`, `token.trailingTrivia()`,
//! `token.leadingComments()`, `token.trailingComments()`, `node.leadingTrivia()`,
//! and `node.trailingTrivia()`.
//!
//! Biome doesn't store whitespace, line breaks, and comments as syntax
//! elements. Each token owns the trivia before it (leading) and the trivia
//! after it up to the next line break (trailing). Plugins receive this trivia
//! as `JsTriviaPiece` objects, one per piece, that remember their owning token.
//!
//! Text the parser skipped while recovering from a syntax error is also stored
//! as trivia, but it isn't exposed to plugins.

use crate::ast::JsAstNode;
use crate::token::JsAstToken;
use biome_js_syntax::{JsLanguage, JsSyntaxToken};
use biome_rowan::{SyntaxTriviaPiece, TriviaPieceKind};
use boa_engine::class::{Class, ClassBuilder};
use boa_engine::object::JsObject;
use boa_engine::object::builtins::JsArray;
use boa_engine::property::Attribute;
use boa_engine::{
    Context, Finalize, JsData, JsNativeError, JsResult, JsString, JsValue, NativeFunction, Trace,
    js_string,
};

/// The trivia kinds exposed to plugins. Skipped text has no kind, so it can't
/// be wrapped.
#[derive(Clone, Copy, Debug)]
enum JsTriviaKind {
    Newline,
    Whitespace,
    SingleLineComment,
    MultiLineComment,
}

impl JsTriviaKind {
    fn from_piece_kind(kind: TriviaPieceKind) -> Option<Self> {
        match kind {
            TriviaPieceKind::Newline => Some(Self::Newline),
            TriviaPieceKind::Whitespace => Some(Self::Whitespace),
            TriviaPieceKind::SingleLineComment => Some(Self::SingleLineComment),
            TriviaPieceKind::MultiLineComment => Some(Self::MultiLineComment),
            TriviaPieceKind::Skipped => None,
        }
    }

    fn is_comment(self) -> bool {
        matches!(self, Self::SingleLineComment | Self::MultiLineComment)
    }

    fn name(self) -> JsString {
        match self {
            Self::Newline => js_string!("newline"),
            Self::Whitespace => js_string!("whitespace"),
            Self::SingleLineComment => js_string!("singleLineComment"),
            Self::MultiLineComment => js_string!("multiLineComment"),
        }
    }
}

#[derive(Clone, Debug, JsData)]
pub(crate) struct JsTriviaPiece {
    pub(crate) piece: SyntaxTriviaPiece<JsLanguage>,
    kind: JsTriviaKind,
}

impl Finalize for JsTriviaPiece {}

// SAFETY: Rowan trivia pieces contain no Boa-managed values.
unsafe impl Trace for JsTriviaPiece {
    boa_engine::gc::empty_trace!();
}

impl JsTriviaPiece {
    pub(crate) fn register(context: &mut Context) -> JsResult<()> {
        context.register_global_class::<Self>()?;
        context
            .global_object()
            .delete_property_or_throw(js_string!(Self::NAME), context)?;
        Ok(())
    }

    /// Wraps a trivia piece, or returns `None` for skipped text. Call
    /// `register` first to attach the class prototype.
    fn from_piece(piece: SyntaxTriviaPiece<JsLanguage>, context: &mut Context) -> Option<JsValue> {
        let kind = JsTriviaKind::from_piece_kind(piece.kind())?;
        let prototype = context
            .get_global_class::<Self>()
            .map(|class| class.prototype());
        Some(JsObject::from_proto_and_data(prototype, Self { piece, kind }).into())
    }

    pub(crate) fn from_value(value: &JsValue) -> Option<Self> {
        let object = value.as_object()?;
        let piece = object.downcast_ref::<Self>()?;
        Some(piece.clone())
    }

    fn get_kind(this: &JsValue, _args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let piece = Self::from_value(this).ok_or_else(|| {
            JsNativeError::typ().with_message("Cannot read piece.kind because this value is not a trivia piece. Use a piece returned by leadingTrivia(), trailingTrivia(), leadingComments(), or trailingComments().")
        })?;
        Ok(piece.kind.name().into())
    }

    fn get_text(this: &JsValue, _args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let piece = Self::from_value(this).ok_or_else(|| {
            JsNativeError::typ().with_message("Cannot read piece.text because this value is not a trivia piece. Use a piece returned by leadingTrivia(), trailingTrivia(), leadingComments(), or trailingComments().")
        })?;
        Ok(JsString::from(piece.piece.text()).into())
    }

    fn get_token(this: &JsValue, _args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let piece = Self::from_value(this).ok_or_else(|| {
            JsNativeError::typ().with_message("Cannot read piece.token because this value is not a trivia piece. Use a piece returned by leadingTrivia(), trailingTrivia(), leadingComments(), or trailingComments().")
        })?;
        Ok(JsAstToken::from_token(piece.piece.token(), context))
    }

    /// Implements `piece.isComment()`.
    fn is_comment(this: &JsValue, _args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let piece = Self::from_value(this).ok_or_else(|| {
            JsNativeError::typ().with_message("isComment() was called on a value that is not a trivia piece. Call piece.isComment() on a piece returned by leadingTrivia(), trailingTrivia(), leadingComments(), or trailingComments().")
        })?;
        Ok(piece.kind.is_comment().into())
    }

    /// Wraps `pieces` in a fresh array, omitting skipped text.
    fn wrap_pieces<I>(pieces: I, context: &mut Context) -> JsValue
    where
        I: IntoIterator<Item = SyntaxTriviaPiece<JsLanguage>>,
    {
        let pieces = pieces
            .into_iter()
            .filter_map(|piece| Self::from_piece(piece, context))
            .collect::<Vec<_>>();
        JsArray::from_iter(pieces, context).into()
    }
}

impl Class for JsTriviaPiece {
    const NAME: &'static str = "__JsTriviaPiece";

    fn init(class: &mut ClassBuilder<'_>) -> JsResult<()> {
        let kind =
            NativeFunction::from_fn_ptr(Self::get_kind).to_js_function(class.context().realm());
        let text =
            NativeFunction::from_fn_ptr(Self::get_text).to_js_function(class.context().realm());
        let token =
            NativeFunction::from_fn_ptr(Self::get_token).to_js_function(class.context().realm());
        class
            .accessor(js_string!("kind"), Some(kind), None, Attribute::ENUMERABLE)
            .accessor(js_string!("text"), Some(text), None, Attribute::ENUMERABLE)
            .accessor(js_string!("token"), Some(token), None, Attribute::empty())
            .method(
                js_string!("isComment"),
                0,
                NativeFunction::from_fn_ptr(Self::is_comment),
            );
        Ok(())
    }

    fn data_constructor(
        _new_target: &JsValue,
        _args: &[JsValue],
        _context: &mut Context,
    ) -> JsResult<Self> {
        Err(JsNativeError::typ()
            .with_message("Trivia pieces do not have a public constructor. Obtain them from leadingTrivia(), trailingTrivia(), leadingComments(), or trailingComments().")
            .into())
    }
}

impl JsAstToken {
    /// Implements `token.leadingTrivia()`.
    pub(crate) fn leading_trivia(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let token = Self::trivia_receiver(this, "leadingTrivia")?;
        Ok(JsTriviaPiece::wrap_pieces(
            token.leading_trivia().pieces(),
            context,
        ))
    }

    /// Implements `token.trailingTrivia()`.
    pub(crate) fn trailing_trivia(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let token = Self::trivia_receiver(this, "trailingTrivia")?;
        Ok(JsTriviaPiece::wrap_pieces(
            token.trailing_trivia().pieces(),
            context,
        ))
    }

    /// Implements `token.leadingComments()`: the comments in the leading
    /// trivia of the token.
    pub(crate) fn leading_comments(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let token = Self::trivia_receiver(this, "leadingComments")?;
        let comments = token
            .leading_trivia()
            .pieces()
            .filter(|piece| piece.is_comments());
        Ok(JsTriviaPiece::wrap_pieces(comments, context))
    }

    /// Implements `token.trailingComments()`: the comments in the trailing
    /// trivia of the token.
    pub(crate) fn trailing_comments(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let token = Self::trivia_receiver(this, "trailingComments")?;
        let comments = token
            .trailing_trivia()
            .pieces()
            .filter(|piece| piece.is_comments());
        Ok(JsTriviaPiece::wrap_pieces(comments, context))
    }

    /// Returns the token `method` was called on, or a `TypeError` naming
    /// `method` when the receiver is not a token.
    fn trivia_receiver(this: &JsValue, method: &str) -> JsResult<JsSyntaxToken> {
        let token = Self::from_value(this).ok_or_else(|| {
            JsNativeError::typ().with_message(format!("{method}() was called on a value that is not a token. Call token.{method}() on a token returned by node.token(field), node.childrenWithTokens(), or factory.token()."))
        })?;
        Ok(token.token)
    }
}

impl JsAstNode {
    /// Implements `node.leadingTrivia()`: the leading trivia of the first token
    /// of the node, or an empty array for a node without tokens.
    pub(crate) fn leading_trivia(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let Some(Self { node }) = Self::from_value(this) else {
            return Err(JsNativeError::typ()
                .with_message("leadingTrivia() was called on a value that is not a node. Call node.leadingTrivia() on the node passed to run() or a node obtained by traversing it.")
                .into());
        };
        let pieces = node
            .first_token()
            .into_iter()
            .flat_map(|token| token.leading_trivia().pieces());
        Ok(JsTriviaPiece::wrap_pieces(pieces, context))
    }

    /// Implements `node.trailingTrivia()`: the trailing trivia of the last
    /// token of the node, or an empty array for a node without tokens.
    pub(crate) fn trailing_trivia(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let Some(Self { node }) = Self::from_value(this) else {
            return Err(JsNativeError::typ()
                .with_message("trailingTrivia() was called on a value that is not a node. Call node.trailingTrivia() on the node passed to run() or a node obtained by traversing it.")
                .into());
        };
        let pieces = node
            .last_token()
            .into_iter()
            .flat_map(|token| token.trailing_trivia().pieces());
        Ok(JsTriviaPiece::wrap_pieces(pieces, context))
    }
}
