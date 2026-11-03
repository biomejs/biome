//! Sets of node kinds and token kinds that plugins build with `nodeKinds()` and
//! `tokenKinds()`.
//!
//! A node kind set stores a [`SyntaxKindSet`] and a token kind set stores a
//! [`TokenSet`], so `set.has(element)` reads the element's kind in Rust instead
//! of creating a kind string for JavaScript to compare. The two are separate
//! classes: native functions that take a set downcast to the class they expect,
//! which rejects a set of the wrong category.
//!
//! Both classes are generic over the analyzed language. A language plugs in by
//! implementing [`KindSetLanguage`] for the names and plugin objects of its
//! syntax tree, and registering and exporting its own instantiations.

use crate::ast::JsAstNode;
use crate::token::JsAstToken;
use biome_js_syntax::{JsLanguage, JsSyntaxKind};
use biome_parser::TokenSet;
use biome_rowan::{Language, SyntaxKindSet};
use boa_engine::class::{Class, ClassBuilder};
use boa_engine::object::JsObject;
use boa_engine::{
    Context, Finalize, JsData, JsNativeError, JsResult, JsValue, NativeFunction, Trace, js_string,
};

/// The language-specific parts of the kind sets exposed to plugins.
pub(crate) trait KindSetLanguage: Language + 'static {
    /// The plugin API type that lists the node kind names, e.g. `JsNodeByKind`.
    const NODE_KIND_LIST: &'static str;
    /// A node kind name used in error messages, e.g. `JS_CALL_EXPRESSION`.
    const NODE_KIND_EXAMPLE: &'static str;
    /// A token kind name used in error messages, e.g. `IDENT`.
    const TOKEN_KIND_EXAMPLE: &'static str;

    /// Resolves a name listed in [`Self::NODE_KIND_LIST`].
    fn node_kind_from_name(name: &str) -> Option<Self::Kind>;

    /// Resolves every name that `token.kind` can return.
    ///
    /// Every resolved kind must fit in a [`TokenSet`].
    fn token_kind_from_name(name: &str) -> Option<Self::Kind>;

    /// Returns the kind of a plugin node object of this language.
    fn node_kind(object: &JsObject) -> Option<Self::Kind>;

    /// Returns the kind of a plugin token object of this language.
    fn token_kind(object: &JsObject) -> Option<Self::Kind>;
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Category {
    Node,
    Token,
}

impl Category {
    /// The plugin API function that creates sets of this category.
    fn function(self) -> &'static str {
        match self {
            Self::Node => "nodeKinds",
            Self::Token => "tokenKinds",
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Node => "node",
            Self::Token => "token",
        }
    }

    fn other(self) -> Self {
        match self {
            Self::Node => Self::Token,
            Self::Token => Self::Node,
        }
    }

    fn kind_from_name<L: KindSetLanguage>(self, name: &str) -> Option<L::Kind> {
        match self {
            Self::Node => L::node_kind_from_name(name),
            Self::Token => L::token_kind_from_name(name),
        }
    }

    /// Resolves the arguments of `nodeKinds()` or `tokenKinds()`.
    fn kinds_from_args<L: KindSetLanguage>(
        self,
        args: &[JsValue],
    ) -> JsResult<impl Iterator<Item = L::Kind>> {
        args.iter()
            .map(|arg| {
                let name = arg
                    .as_string()
                    .ok_or_else(|| self.non_string_error::<L>())?
                    .to_std_string_lossy();
                self.kind_from_name::<L>(&name)
                    .ok_or_else(|| self.unknown_kind_error::<L>(&name))
            })
            .collect::<JsResult<Vec<_>>>()
            .map(Vec::into_iter)
    }

    fn non_string_error<L: KindSetLanguage>(self) -> boa_engine::JsError {
        let example = match self {
            Self::Node => L::NODE_KIND_EXAMPLE,
            Self::Token => L::TOKEN_KIND_EXAMPLE,
        };
        JsNativeError::typ()
            .with_message(format!(
                "{}() requires {kind} kind names as strings. Pass names such as {example:?}, not {kind} objects.",
                self.function(),
                kind = self.name(),
            ))
            .into()
    }

    fn unknown_kind_error<L: KindSetLanguage>(self, name: &str) -> boa_engine::JsError {
        let message = if self.other().kind_from_name::<L>(name).is_some() {
            format!(
                "{}() received the {actual} kind {name:?}, but it only accepts {expected} kinds. Create a set of {actual} kinds with {}({name:?}) instead.",
                self.function(),
                self.other().function(),
                actual = self.other().name(),
                expected = self.name(),
            )
        } else {
            match self {
                Self::Node => format!(
                    "nodeKinds() received the unknown node kind {name:?}. Pass a node kind listed in {}, such as {:?}.",
                    L::NODE_KIND_LIST,
                    L::NODE_KIND_EXAMPLE,
                ),
                Self::Token => format!(
                    "tokenKinds() received the unknown token kind {name:?}. Pass a kind that token.kind can return, such as {:?}.",
                    L::TOKEN_KIND_EXAMPLE,
                ),
            }
        };
        JsNativeError::typ().with_message(message).into()
    }

    fn receiver_error(self) -> boa_engine::JsError {
        JsNativeError::typ()
            .with_message(format!(
                "has() was called on a value that is not a {kind} kind set. Call set.has(element) on a set returned by {}().",
                self.function(),
                kind = self.name(),
            ))
            .into()
    }

    /// Returns the category and kind of the `set.has(element)` argument, or
    /// `None` for `undefined`.
    fn element_kind<L: KindSetLanguage>(
        self,
        args: &[JsValue],
    ) -> JsResult<Option<(Self, L::Kind)>> {
        let [element] = args else {
            return Err(JsNativeError::typ()
                .with_message(format!(
                    "set.has() received the wrong number of arguments. Pass exactly one node, token, or undefined to a set returned by {}().",
                    self.function()
                ))
                .into());
        };
        if element.is_undefined() {
            return Ok(None);
        }
        let kind = element.as_object().and_then(|object| {
            L::node_kind(&object)
                .map(|kind| (Self::Node, kind))
                .or_else(|| L::token_kind(&object).map(|kind| (Self::Token, kind)))
        });
        match kind {
            Some(kind) => Ok(Some(kind)),
            None => Err(JsNativeError::typ()
                .with_message(format!(
                    "set.has() requires a Biome node or token, but the argument is neither. Pass a node or token to the set returned by {}(), not a kind string or a plain object.",
                    self.function()
                ))
                .into()),
        }
    }
}

/// An immutable set of node kinds, created by `nodeKinds()`.
pub(crate) struct JsNodeKindSet<L: KindSetLanguage> {
    /// Boxed because Boa requires native data to be at most 8-byte aligned.
    kinds: Box<SyntaxKindSet<L>>,
}

/// An immutable set of token kinds, created by `tokenKinds()`.
pub(crate) struct JsTokenKindSet<L: KindSetLanguage> {
    /// Boxed because Boa requires native data to be at most 8-byte aligned.
    kinds: Box<TokenSet<L::Kind>>,
}

impl<L: KindSetLanguage> JsData for JsNodeKindSet<L> {}
impl<L: KindSetLanguage> JsData for JsTokenKindSet<L> {}
impl<L: KindSetLanguage> Finalize for JsNodeKindSet<L> {}
impl<L: KindSetLanguage> Finalize for JsTokenKindSet<L> {}

// SAFETY: The set contains only plain bits and no Boa-managed values.
unsafe impl<L: KindSetLanguage> Trace for JsNodeKindSet<L> {
    boa_engine::gc::empty_trace!();
}

// SAFETY: The set contains only plain bits and no Boa-managed values.
unsafe impl<L: KindSetLanguage> Trace for JsTokenKindSet<L> {
    boa_engine::gc::empty_trace!();
}

impl<L: KindSetLanguage> JsNodeKindSet<L> {
    pub(crate) fn register(context: &mut Context) -> JsResult<()> {
        context.register_global_class::<Self>()?;
        context
            .global_object()
            .delete_property_or_throw(js_string!(Self::NAME), context)?;
        Ok(())
    }

    /// Returns the kinds of a set created by `nodeKinds()`.
    pub(crate) fn from_value(value: &JsValue) -> Option<SyntaxKindSet<L>> {
        let object = value.as_object()?;
        let set = object.downcast_ref::<Self>()?;
        Some(*set.kinds)
    }

    /// Implements `nodeKinds(...kinds)`.
    pub(crate) fn create(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let kinds = Category::Node
            .kinds_from_args::<L>(args)?
            .fold(SyntaxKindSet::EMPTY, |kinds, kind| {
                kinds.union(SyntaxKindSet::of(kind))
            });
        let prototype = context
            .get_global_class::<Self>()
            .map(|class| class.prototype());
        let set = Self {
            kinds: Box::new(kinds),
        };
        Ok(JsObject::from_proto_and_data(prototype, set).into())
    }

    /// Implements `set.has(element)`.
    ///
    /// Returns `false` for `undefined` and for tokens. Throws for values that
    /// are not Biome nodes or tokens.
    fn has(this: &JsValue, args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let kinds = Self::from_value(this).ok_or_else(|| Category::Node.receiver_error())?;
        let has = match Category::Node.element_kind::<L>(args)? {
            Some((Category::Node, kind)) => kinds.matches(kind),
            _ => false,
        };
        Ok(has.into())
    }
}

impl<L: KindSetLanguage> JsTokenKindSet<L> {
    pub(crate) fn register(context: &mut Context) -> JsResult<()> {
        context.register_global_class::<Self>()?;
        context
            .global_object()
            .delete_property_or_throw(js_string!(Self::NAME), context)?;
        Ok(())
    }

    /// Returns the kinds of a set created by `tokenKinds()`.
    pub(crate) fn from_value(value: &JsValue) -> Option<TokenSet<L::Kind>> {
        let object = value.as_object()?;
        let set = object.downcast_ref::<Self>()?;
        Some(*set.kinds)
    }

    /// Implements `tokenKinds(...kinds)`.
    pub(crate) fn create(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let kinds = Category::Token
            .kinds_from_args::<L>(args)?
            .fold(TokenSet::EMPTY, |kinds, kind| {
                kinds.union(TokenSet::singleton(kind))
            });
        let prototype = context
            .get_global_class::<Self>()
            .map(|class| class.prototype());
        let set = Self {
            kinds: Box::new(kinds),
        };
        Ok(JsObject::from_proto_and_data(prototype, set).into())
    }

    /// Implements `set.has(element)`.
    ///
    /// Returns `false` for `undefined` and for nodes. Throws for values that
    /// are not Biome nodes or tokens.
    fn has(this: &JsValue, args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let kinds = Self::from_value(this).ok_or_else(|| Category::Token.receiver_error())?;
        let has = match Category::Token.element_kind::<L>(args)? {
            Some((Category::Token, kind)) => kinds.contains(kind),
            _ => false,
        };
        Ok(has.into())
    }
}

impl<L: KindSetLanguage> Class for JsNodeKindSet<L> {
    const NAME: &'static str = "__JsNodeKindSet";

    fn init(class: &mut ClassBuilder<'_>) -> JsResult<()> {
        class.method(js_string!("has"), 1, NativeFunction::from_fn_ptr(Self::has));
        Ok(())
    }

    fn data_constructor(
        _new_target: &JsValue,
        _args: &[JsValue],
        _context: &mut Context,
    ) -> JsResult<Self> {
        Err(JsNativeError::typ()
            .with_message("Node kind sets do not have a public constructor. Create one with nodeKinds(...kinds).")
            .into())
    }
}

impl<L: KindSetLanguage> Class for JsTokenKindSet<L> {
    const NAME: &'static str = "__JsTokenKindSet";

    fn init(class: &mut ClassBuilder<'_>) -> JsResult<()> {
        class.method(js_string!("has"), 1, NativeFunction::from_fn_ptr(Self::has));
        Ok(())
    }

    fn data_constructor(
        _new_target: &JsValue,
        _args: &[JsValue],
        _context: &mut Context,
    ) -> JsResult<Self> {
        Err(JsNativeError::typ()
            .with_message("Token kind sets do not have a public constructor. Create one with tokenKinds(...kinds).")
            .into())
    }
}

impl KindSetLanguage for JsLanguage {
    const NODE_KIND_LIST: &'static str = "JsNodeByKind";
    const NODE_KIND_EXAMPLE: &'static str = "JS_CALL_EXPRESSION";
    const TOKEN_KIND_EXAMPLE: &'static str = "IDENT";

    fn node_kind_from_name(name: &str) -> Option<JsSyntaxKind> {
        JsAstNode::syntax_kind_from_ast_name(name)
    }

    /// Adds the source-only kinds that `factory.token()` cannot construct.
    fn token_kind_from_name(name: &str) -> Option<JsSyntaxKind> {
        JsAstNode::token_kind_from_name(name).or(match name {
            "ERROR_TOKEN" => Some(JsSyntaxKind::ERROR_TOKEN),
            "NEWLINE" => Some(JsSyntaxKind::NEWLINE),
            "WHITESPACE" => Some(JsSyntaxKind::WHITESPACE),
            "COMMENT" => Some(JsSyntaxKind::COMMENT),
            "MULTILINE_COMMENT" => Some(JsSyntaxKind::MULTILINE_COMMENT),
            "EOF" => Some(JsSyntaxKind::EOF),
            "UNICODE_BOM" => Some(JsSyntaxKind::UNICODE_BOM),
            _ => None,
        })
    }

    fn node_kind(object: &JsObject) -> Option<JsSyntaxKind> {
        object
            .downcast_ref::<JsAstNode>()
            .map(|node| node.node.kind())
    }

    fn token_kind(object: &JsObject) -> Option<JsSyntaxKind> {
        object
            .downcast_ref::<JsAstToken>()
            .map(|token| token.token.kind())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_rowan::SyntaxKind;

    /// Every token kind that can appear in a parsed tree resolves by name and
    /// fits in a [`TokenSet`].
    #[test]
    fn token_kind_names_cover_every_token_kind() {
        let missing = (0..JsSyntaxKind::__LAST as u16)
            .map(JsSyntaxKind::from)
            .filter(|kind| {
                // `import.meta` uses META, GRIT_METAVARIABLE belongs to Grit patterns,
                // and TS_EXTENDS is a node kind without a node type.
                !matches!(
                    kind,
                    JsSyntaxKind::TOMBSTONE
                        | JsSyntaxKind::META_KW
                        | JsSyntaxKind::GRIT_METAVARIABLE
                        | JsSyntaxKind::TS_EXTENDS
                )
            })
            .map(|kind| (kind, format!("{kind:?}")))
            .filter(|(kind, name)| {
                !kind.is_list() && JsLanguage::node_kind_from_name(name).is_none()
            })
            .filter(|(kind, name)| {
                JsLanguage::token_kind_from_name(name) != Some(*kind)
                    || !TokenSet::singleton(*kind).contains(*kind)
            })
            .map(|(_, name)| name)
            .collect::<Vec<_>>();
        assert!(
            missing.is_empty(),
            "tokenKinds() does not accept {missing:?}"
        );
    }

    #[test]
    fn node_and_token_kind_names_are_disjoint() {
        for raw in 0..JsSyntaxKind::__LAST as u16 {
            let name = format!("{:?}", JsSyntaxKind::from(raw));
            assert!(
                JsLanguage::node_kind_from_name(&name).is_none()
                    || JsLanguage::token_kind_from_name(&name).is_none(),
                "{name} is both a node kind and a token kind"
            );
        }
    }
}
