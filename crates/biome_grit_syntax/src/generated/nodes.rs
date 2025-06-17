//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    GritLanguage as Language, GritSyntaxElement as SyntaxElement,
    GritSyntaxElementChildren as SyntaxElementChildren,
    GritSyntaxKind::{self as SyntaxKind, *},
    GritSyntaxList as SyntaxList, GritSyntaxNode as SyntaxNode, GritSyntaxToken as SyntaxToken,
    macros::map_syntax_node,
};
use biome_rowan::{
    AstNode, AstNodeList, AstNodeListIterator, AstNodeSlotMap, AstSeparatedList,
    AstSeparatedListNodesIterator, RawSyntaxKind, SyntaxKindSet, SyntaxResult, support,
};
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Formatter};
#[doc = r" Sentinel value indicating a missing element in a dynamic node, where"]
#[doc = r" the slots are not statically known."]
pub(crate) const SLOT_MAP_EMPTY_VALUE: u8 = u8::MAX;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritAddOperation {
    pub(crate) syntax: SyntaxNode,
}
impl GritAddOperation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritAddOperationFields {
        GritAddOperationFields {
            left: self.left(),
            plus_token: self.plus_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn plus_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritAddOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritAddOperationFields {
    pub left: SyntaxResult<AnyGritPattern>,
    pub plus_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritAnnotation {
    pub(crate) syntax: SyntaxNode,
}
impl GritAnnotation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritAnnotationFields {
        GritAnnotationFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritAnnotation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritAnnotationFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritAssignmentAsPattern {
    pub(crate) syntax: SyntaxNode,
}
impl GritAssignmentAsPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritAssignmentAsPatternFields {
        GritAssignmentAsPatternFields {
            container: self.container(),
            eq_token: self.eq_token(),
            pattern: self.pattern(),
        }
    }
    pub fn container(&self) -> SyntaxResult<AnyGritContainer> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritAssignmentAsPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritAssignmentAsPatternFields {
    pub container: SyntaxResult<AnyGritContainer>,
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritBacktickSnippetLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl GritBacktickSnippetLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritBacktickSnippetLiteralFields {
        GritBacktickSnippetLiteralFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritBacktickSnippetLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritBacktickSnippetLiteralFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritBooleanLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl GritBooleanLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritBooleanLiteralFields {
        GritBooleanLiteralFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritBooleanLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritBooleanLiteralFields {
    pub value: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritBracketedPattern {
    pub(crate) syntax: SyntaxNode,
}
impl GritBracketedPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritBracketedPatternFields {
        GritBracketedPatternFields {
            l_paren_token: self.l_paren_token(),
            pattern: self.pattern(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GritBracketedPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritBracketedPatternFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritPattern>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritBracketedPredicate {
    pub(crate) syntax: SyntaxNode,
}
impl GritBracketedPredicate {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritBracketedPredicateFields {
        GritBracketedPredicateFields {
            l_paren_token: self.l_paren_token(),
            predicate: self.predicate(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn predicate(&self) -> SyntaxResult<AnyGritPredicate> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GritBracketedPredicate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritBracketedPredicateFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub predicate: SyntaxResult<AnyGritPredicate>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritBubble {
    pub(crate) syntax: SyntaxNode,
}
impl GritBubble {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritBubbleFields {
        GritBubbleFields {
            bubble_token: self.bubble_token(),
            scope: self.scope(),
            pattern: self.pattern(),
        }
    }
    pub fn bubble_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn scope(&self) -> Option<GritBubbleScope> {
        support::node(&self.syntax, 1usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritMaybeCurlyPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritBubble {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritBubbleFields {
    pub bubble_token: SyntaxResult<SyntaxToken>,
    pub scope: Option<GritBubbleScope>,
    pub pattern: SyntaxResult<AnyGritMaybeCurlyPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritBubbleScope {
    pub(crate) syntax: SyntaxNode,
}
impl GritBubbleScope {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritBubbleScopeFields {
        GritBubbleScopeFields {
            l_paren_token: self.l_paren_token(),
            variables: self.variables(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn variables(&self) -> GritVariableList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GritBubbleScope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritBubbleScopeFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub variables: GritVariableList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritCodeSnippet {
    pub(crate) syntax: SyntaxNode,
}
impl GritCodeSnippet {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritCodeSnippetFields {
        GritCodeSnippetFields {
            source: self.source(),
        }
    }
    pub fn source(&self) -> SyntaxResult<AnyGritCodeSnippetSource> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for GritCodeSnippet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritCodeSnippetFields {
    pub source: SyntaxResult<AnyGritCodeSnippetSource>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritCurlyPattern {
    pub(crate) syntax: SyntaxNode,
}
impl GritCurlyPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritCurlyPatternFields {
        GritCurlyPatternFields {
            l_curly_token: self.l_curly_token(),
            pattern: self.pattern(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GritCurlyPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritCurlyPatternFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritPattern>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritDivOperation {
    pub(crate) syntax: SyntaxNode,
}
impl GritDivOperation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritDivOperationFields {
        GritDivOperationFields {
            left: self.left(),
            slash_token: self.slash_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn slash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritDivOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritDivOperationFields {
    pub left: SyntaxResult<AnyGritPattern>,
    pub slash_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritDot {
    pub(crate) syntax: SyntaxNode,
}
impl GritDot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritDotFields {
        GritDotFields {
            dot_token: self.dot_token(),
        }
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritDot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritDotFields {
    pub dot_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritDotdotdot {
    pub(crate) syntax: SyntaxNode,
}
impl GritDotdotdot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritDotdotdotFields {
        GritDotdotdotFields {
            dotdotdot_token: self.dotdotdot_token(),
            pattern: self.pattern(),
        }
    }
    pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> Option<AnyGritMaybeCurlyPattern> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for GritDotdotdot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritDotdotdotFields {
    pub dotdotdot_token: SyntaxResult<SyntaxToken>,
    pub pattern: Option<AnyGritMaybeCurlyPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritDoubleLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl GritDoubleLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritDoubleLiteralFields {
        GritDoubleLiteralFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritDoubleLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritDoubleLiteralFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritEngineName {
    pub(crate) syntax: SyntaxNode,
}
impl GritEngineName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritEngineNameFields {
        GritEngineNameFields {
            engine_kind: self.engine_kind(),
        }
    }
    pub fn engine_kind(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritEngineName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritEngineNameFields {
    pub engine_kind: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritEvery {
    pub(crate) syntax: SyntaxNode,
}
impl GritEvery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritEveryFields {
        GritEveryFields {
            every_token: self.every_token(),
            pattern: self.pattern(),
        }
    }
    pub fn every_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritMaybeCurlyPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for GritEvery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritEveryFields {
    pub every_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritMaybeCurlyPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritFiles {
    pub(crate) syntax: SyntaxNode,
}
impl GritFiles {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritFilesFields {
        GritFilesFields {
            multifile_token: self.multifile_token(),
            l_curly_token: self.l_curly_token(),
            files: self.files(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn multifile_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn files(&self) -> GritPatternList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for GritFiles {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritFilesFields {
    pub multifile_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub files: GritPatternList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritFunctionDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GritFunctionDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritFunctionDefinitionFields {
        GritFunctionDefinitionFields {
            function_token: self.function_token(),
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            args: self.args(),
            r_paren_token: self.r_paren_token(),
            body: self.body(),
        }
    }
    pub fn function_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<GritName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn args(&self) -> GritVariableList {
        support::list(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn body(&self) -> SyntaxResult<GritPredicateCurly> {
        support::required_node(&self.syntax, 5usize)
    }
}
impl Serialize for GritFunctionDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritFunctionDefinitionFields {
    pub function_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GritName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub args: GritVariableList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<GritPredicateCurly>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritIntLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl GritIntLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritIntLiteralFields {
        GritIntLiteralFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritIntLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritIntLiteralFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritJavascriptBodyWrapper {
    pub(crate) syntax: SyntaxNode,
}
impl GritJavascriptBodyWrapper {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritJavascriptBodyWrapperFields {
        GritJavascriptBodyWrapperFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritJavascriptBodyWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritJavascriptBodyWrapperFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritJavascriptFunctionDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GritJavascriptFunctionDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritJavascriptFunctionDefinitionFields {
        GritJavascriptFunctionDefinitionFields {
            function_token: self.function_token(),
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            args: self.args(),
            r_paren_token: self.r_paren_token(),
            js_token: self.js_token(),
            grit_javascript_body_wrapper: self.grit_javascript_body_wrapper(),
        }
    }
    pub fn function_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<GritName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn args(&self) -> GritVariableList {
        support::list(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn js_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn grit_javascript_body_wrapper(&self) -> SyntaxResult<GritJavascriptBodyWrapper> {
        support::required_node(&self.syntax, 6usize)
    }
}
impl Serialize for GritJavascriptFunctionDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritJavascriptFunctionDefinitionFields {
    pub function_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GritName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub args: GritVariableList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub js_token: SyntaxResult<SyntaxToken>,
    pub grit_javascript_body_wrapper: SyntaxResult<GritJavascriptBodyWrapper>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritLanguageDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl GritLanguageDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritLanguageDeclarationFields {
        GritLanguageDeclarationFields {
            language_token: self.language_token(),
            name: self.name(),
            flavor: self.flavor(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn language_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<AnyGritLanguageName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn flavor(&self) -> Option<GritLanguageFlavor> {
        support::node(&self.syntax, 2usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 3usize)
    }
}
impl Serialize for GritLanguageDeclaration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritLanguageDeclarationFields {
    pub language_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<AnyGritLanguageName>,
    pub flavor: Option<GritLanguageFlavor>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritLanguageFlavor {
    pub(crate) syntax: SyntaxNode,
}
impl GritLanguageFlavor {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritLanguageFlavorFields {
        GritLanguageFlavorFields {
            l_paren_token: self.l_paren_token(),
            flavors: self.flavors(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn flavors(&self) -> GritLanguageFlavorList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GritLanguageFlavor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritLanguageFlavorFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub flavors: GritLanguageFlavorList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritLanguageFlavorKind {
    pub(crate) syntax: SyntaxNode,
}
impl GritLanguageFlavorKind {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritLanguageFlavorKindFields {
        GritLanguageFlavorKindFields {
            flavor_kind: self.flavor_kind(),
        }
    }
    pub fn flavor_kind(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritLanguageFlavorKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritLanguageFlavorKindFields {
    pub flavor_kind: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritLanguageName {
    pub(crate) syntax: SyntaxNode,
}
impl GritLanguageName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritLanguageNameFields {
        GritLanguageNameFields {
            language_kind: self.language_kind(),
        }
    }
    pub fn language_kind(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritLanguageName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritLanguageNameFields {
    pub language_kind: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritLanguageSpecificSnippet {
    pub(crate) syntax: SyntaxNode,
}
impl GritLanguageSpecificSnippet {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritLanguageSpecificSnippetFields {
        GritLanguageSpecificSnippetFields {
            language: self.language(),
            snippet_token: self.snippet_token(),
        }
    }
    pub fn language(&self) -> SyntaxResult<AnyGritLanguageName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn snippet_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for GritLanguageSpecificSnippet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritLanguageSpecificSnippetFields {
    pub language: SyntaxResult<AnyGritLanguageName>,
    pub snippet_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritLike {
    pub(crate) syntax: SyntaxNode,
}
impl GritLike {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritLikeFields {
        GritLikeFields {
            like_token: self.like_token(),
            threshold: self.threshold(),
            l_curly_token: self.l_curly_token(),
            example: self.example(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn like_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn threshold(&self) -> Option<GritLikeThreshold> {
        support::node(&self.syntax, 1usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn example(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl Serialize for GritLike {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritLikeFields {
    pub like_token: SyntaxResult<SyntaxToken>,
    pub threshold: Option<GritLikeThreshold>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub example: SyntaxResult<AnyGritPattern>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritLikeThreshold {
    pub(crate) syntax: SyntaxNode,
}
impl GritLikeThreshold {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritLikeThresholdFields {
        GritLikeThresholdFields {
            l_paren_token: self.l_paren_token(),
            threshold: self.threshold(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn threshold(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GritLikeThreshold {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritLikeThresholdFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub threshold: SyntaxResult<AnyGritPattern>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritList {
    pub(crate) syntax: SyntaxNode,
}
impl GritList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritListFields {
        GritListFields {
            name: self.name(),
            l_brack_token: self.l_brack_token(),
            patterns: self.patterns(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn name(&self) -> Option<GritName> {
        support::node(&self.syntax, 0usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn patterns(&self) -> GritListPatternList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for GritList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritListFields {
    pub name: Option<GritName>,
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub patterns: GritListPatternList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritListAccessor {
    pub(crate) syntax: SyntaxNode,
}
impl GritListAccessor {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritListAccessorFields {
        GritListAccessorFields {
            list: self.list(),
            l_brack_token: self.l_brack_token(),
            index: self.index(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn list(&self) -> SyntaxResult<AnyGritListAccessorSubject> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn index(&self) -> SyntaxResult<AnyGritListIndex> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for GritListAccessor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritListAccessorFields {
    pub list: SyntaxResult<AnyGritListAccessorSubject>,
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub index: SyntaxResult<AnyGritListIndex>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritMap {
    pub(crate) syntax: SyntaxNode,
}
impl GritMap {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritMapFields {
        GritMapFields {
            l_curly_token: self.l_curly_token(),
            elements: self.elements(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn elements(&self) -> GritMapElementList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GritMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritMapFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub elements: GritMapElementList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritMapAccessor {
    pub(crate) syntax: SyntaxNode,
}
impl GritMapAccessor {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritMapAccessorFields {
        GritMapAccessorFields {
            map: self.map(),
            dot_token: self.dot_token(),
            key: self.key(),
        }
    }
    pub fn map(&self) -> SyntaxResult<AnyGritMapAccessorSubject> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn key(&self) -> SyntaxResult<AnyGritMapKey> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritMapAccessor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritMapAccessorFields {
    pub map: SyntaxResult<AnyGritMapAccessorSubject>,
    pub dot_token: SyntaxResult<SyntaxToken>,
    pub key: SyntaxResult<AnyGritMapKey>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritMapElement {
    pub(crate) syntax: SyntaxNode,
}
impl GritMapElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritMapElementFields {
        GritMapElementFields {
            key: self.key(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn key(&self) -> SyntaxResult<GritName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritMapElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritMapElementFields {
    pub key: SyntaxResult<GritName>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritModOperation {
    pub(crate) syntax: SyntaxNode,
}
impl GritModOperation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritModOperationFields {
        GritModOperationFields {
            left: self.left(),
            remainder_token: self.remainder_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn remainder_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritModOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritModOperationFields {
    pub left: SyntaxResult<AnyGritPattern>,
    pub remainder_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritMulOperation {
    pub(crate) syntax: SyntaxNode,
}
impl GritMulOperation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritMulOperationFields {
        GritMulOperationFields {
            left: self.left(),
            star_token: self.star_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritMulOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritMulOperationFields {
    pub left: SyntaxResult<AnyGritPattern>,
    pub star_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritName {
    pub(crate) syntax: SyntaxNode,
}
impl GritName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritNameFields {
        GritNameFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritNameFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritNamedArg {
    pub(crate) syntax: SyntaxNode,
}
impl GritNamedArg {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritNamedArgFields {
        GritNamedArgFields {
            name: self.name(),
            eq_token: self.eq_token(),
            pattern: self.pattern(),
        }
    }
    pub fn name(&self) -> SyntaxResult<GritName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritNamedArg {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritNamedArgFields {
    pub name: SyntaxResult<GritName>,
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritNegativeIntLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl GritNegativeIntLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritNegativeIntLiteralFields {
        GritNegativeIntLiteralFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritNegativeIntLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritNegativeIntLiteralFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritNodeLike {
    pub(crate) syntax: SyntaxNode,
}
impl GritNodeLike {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritNodeLikeFields {
        GritNodeLikeFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            named_args: self.named_args(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<GritName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn named_args(&self) -> GritNamedArgList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for GritNodeLike {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritNodeLikeFields {
    pub name: SyntaxResult<GritName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub named_args: GritNamedArgList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritNot {
    pub(crate) syntax: SyntaxNode,
}
impl GritNot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritNotFields {
        GritNotFields {
            token: self.token(),
        }
    }
    pub fn token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritNot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritNotFields {
    pub token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternAccumulate {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternAccumulate {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternAccumulateFields {
        GritPatternAccumulateFields {
            left: self.left(),
            add_assign_token: self.add_assign_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn add_assign_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritPatternAccumulate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternAccumulateFields {
    pub left: SyntaxResult<AnyGritPattern>,
    pub add_assign_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternAfter {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternAfter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternAfterFields {
        GritPatternAfterFields {
            after_token: self.after_token(),
            pattern: self.pattern(),
        }
    }
    pub fn after_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for GritPatternAfter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternAfterFields {
    pub after_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternAnd {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternAnd {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternAndFields {
        GritPatternAndFields {
            and_token: self.and_token(),
            l_curly_token: self.l_curly_token(),
            patterns: self.patterns(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn and_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn patterns(&self) -> GritPatternList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for GritPatternAnd {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternAndFields {
    pub and_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub patterns: GritPatternList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternAny {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternAny {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternAnyFields {
        GritPatternAnyFields {
            any_token: self.any_token(),
            l_curly_token: self.l_curly_token(),
            patterns: self.patterns(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn any_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn patterns(&self) -> GritPatternList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for GritPatternAny {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternAnyFields {
    pub any_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub patterns: GritPatternList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternAs {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternAs {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternAsFields {
        GritPatternAsFields {
            pattern: self.pattern(),
            as_token: self.as_token(),
            variable: self.variable(),
        }
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn variable(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritPatternAs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternAsFields {
    pub pattern: SyntaxResult<AnyGritPattern>,
    pub as_token: SyntaxResult<SyntaxToken>,
    pub variable: SyntaxResult<GritVariable>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternBefore {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternBefore {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternBeforeFields {
        GritPatternBeforeFields {
            before_token: self.before_token(),
            pattern: self.pattern(),
        }
    }
    pub fn before_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for GritPatternBefore {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternBeforeFields {
    pub before_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternContains {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternContains {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternContainsFields {
        GritPatternContainsFields {
            contains_token: self.contains_token(),
            contains: self.contains(),
            until_clause: self.until_clause(),
        }
    }
    pub fn contains_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn contains(&self) -> SyntaxResult<AnyGritMaybeCurlyPattern> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn until_clause(&self) -> Option<GritPatternUntilClause> {
        support::node(&self.syntax, 2usize)
    }
}
impl Serialize for GritPatternContains {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternContainsFields {
    pub contains_token: SyntaxResult<SyntaxToken>,
    pub contains: SyntaxResult<AnyGritMaybeCurlyPattern>,
    pub until_clause: Option<GritPatternUntilClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternDefinitionFields {
        GritPatternDefinitionFields {
            visibility_token: self.visibility_token(),
            pattern_token: self.pattern_token(),
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            args: self.args(),
            r_paren_token: self.r_paren_token(),
            language: self.language(),
            body: self.body(),
        }
    }
    pub fn visibility_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn pattern_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<GritName> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn args(&self) -> GritVariableList {
        support::list(&self.syntax, 4usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn language(&self) -> Option<GritLanguageDeclaration> {
        support::node(&self.syntax, 6usize)
    }
    pub fn body(&self) -> SyntaxResult<GritPatternDefinitionBody> {
        support::required_node(&self.syntax, 7usize)
    }
}
impl Serialize for GritPatternDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternDefinitionFields {
    pub visibility_token: Option<SyntaxToken>,
    pub pattern_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GritName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub args: GritVariableList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub language: Option<GritLanguageDeclaration>,
    pub body: SyntaxResult<GritPatternDefinitionBody>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternDefinitionBody {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternDefinitionBody {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternDefinitionBodyFields {
        GritPatternDefinitionBodyFields {
            l_curly_token: self.l_curly_token(),
            patterns: self.patterns(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn patterns(&self) -> GritPatternList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GritPatternDefinitionBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternDefinitionBodyFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub patterns: GritPatternList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternElseClause {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternElseClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternElseClauseFields {
        GritPatternElseClauseFields {
            else_token: self.else_token(),
            else_pattern: self.else_pattern(),
        }
    }
    pub fn else_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn else_pattern(&self) -> SyntaxResult<AnyGritMaybeCurlyPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for GritPatternElseClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternElseClauseFields {
    pub else_token: SyntaxResult<SyntaxToken>,
    pub else_pattern: SyntaxResult<AnyGritMaybeCurlyPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternIfElse {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternIfElse {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternIfElseFields {
        GritPatternIfElseFields {
            if_token: self.if_token(),
            l_paren_token: self.l_paren_token(),
            if_predicate: self.if_predicate(),
            r_paren_token: self.r_paren_token(),
            then_pattern: self.then_pattern(),
            else_clause: self.else_clause(),
        }
    }
    pub fn if_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn if_predicate(&self) -> SyntaxResult<AnyGritPredicate> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn then_pattern(&self) -> SyntaxResult<AnyGritMaybeCurlyPattern> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn else_clause(&self) -> Option<GritPatternElseClause> {
        support::node(&self.syntax, 5usize)
    }
}
impl Serialize for GritPatternIfElse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternIfElseFields {
    pub if_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub if_predicate: SyntaxResult<AnyGritPredicate>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub then_pattern: SyntaxResult<AnyGritMaybeCurlyPattern>,
    pub else_clause: Option<GritPatternElseClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternIncludes {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternIncludes {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternIncludesFields {
        GritPatternIncludesFields {
            includes_token: self.includes_token(),
            includes: self.includes(),
        }
    }
    pub fn includes_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn includes(&self) -> SyntaxResult<AnyGritMaybeCurlyPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for GritPatternIncludes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternIncludesFields {
    pub includes_token: SyntaxResult<SyntaxToken>,
    pub includes: SyntaxResult<AnyGritMaybeCurlyPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternLimit {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternLimit {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternLimitFields {
        GritPatternLimitFields {
            pattern: self.pattern(),
            limit_token: self.limit_token(),
            limit: self.limit(),
        }
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn limit_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn limit(&self) -> SyntaxResult<GritIntLiteral> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritPatternLimit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternLimitFields {
    pub pattern: SyntaxResult<AnyGritPattern>,
    pub limit_token: SyntaxResult<SyntaxToken>,
    pub limit: SyntaxResult<GritIntLiteral>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternMaybe {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternMaybe {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternMaybeFields {
        GritPatternMaybeFields {
            maybe_token: self.maybe_token(),
            pattern: self.pattern(),
        }
    }
    pub fn maybe_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritMaybeCurlyPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for GritPatternMaybe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternMaybeFields {
    pub maybe_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritMaybeCurlyPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternNot {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternNot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternNotFields {
        GritPatternNotFields {
            not: self.not(),
            pattern: self.pattern(),
        }
    }
    pub fn not(&self) -> SyntaxResult<GritNot> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for GritPatternNot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternNotFields {
    pub not: SyntaxResult<GritNot>,
    pub pattern: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternOr {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternOr {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternOrFields {
        GritPatternOrFields {
            or_token: self.or_token(),
            l_curly_token: self.l_curly_token(),
            patterns: self.patterns(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn patterns(&self) -> GritPatternList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for GritPatternOr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternOrFields {
    pub or_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub patterns: GritPatternList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternOrElse {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternOrElse {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternOrElseFields {
        GritPatternOrElseFields {
            orelse_token: self.orelse_token(),
            l_curly_token: self.l_curly_token(),
            patterns: self.patterns(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn orelse_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn patterns(&self) -> GritPatternList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for GritPatternOrElse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternOrElseFields {
    pub orelse_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub patterns: GritPatternList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternUntilClause {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternUntilClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternUntilClauseFields {
        GritPatternUntilClauseFields {
            until_token: self.until_token(),
            until: self.until(),
        }
    }
    pub fn until_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn until(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for GritPatternUntilClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternUntilClauseFields {
    pub until_token: SyntaxResult<SyntaxToken>,
    pub until: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternWhere {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternWhere {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternWhereFields {
        GritPatternWhereFields {
            pattern: self.pattern(),
            where_token: self.where_token(),
            side_condition: self.side_condition(),
        }
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn where_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn side_condition(&self) -> SyntaxResult<AnyGritPredicate> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritPatternWhere {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPatternWhereFields {
    pub pattern: SyntaxResult<AnyGritPattern>,
    pub where_token: SyntaxResult<SyntaxToken>,
    pub side_condition: SyntaxResult<AnyGritPredicate>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateAccumulate {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateAccumulate {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateAccumulateFields {
        GritPredicateAccumulateFields {
            left: self.left(),
            add_assign_token: self.add_assign_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn add_assign_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritPredicateAccumulate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateAccumulateFields {
    pub left: SyntaxResult<GritVariable>,
    pub add_assign_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateAnd {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateAnd {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateAndFields {
        GritPredicateAndFields {
            and_token: self.and_token(),
            l_curly_token: self.l_curly_token(),
            predicates: self.predicates(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn and_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn predicates(&self) -> GritPredicateList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for GritPredicateAnd {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateAndFields {
    pub and_token: Option<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub predicates: GritPredicateList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateAny {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateAny {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateAnyFields {
        GritPredicateAnyFields {
            any_token: self.any_token(),
            l_curly_token: self.l_curly_token(),
            predicates: self.predicates(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn any_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn predicates(&self) -> GritPredicateList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for GritPredicateAny {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateAnyFields {
    pub any_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub predicates: GritPredicateList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateAssignment {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateAssignment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateAssignmentFields {
        GritPredicateAssignmentFields {
            container: self.container(),
            eq_token: self.eq_token(),
            pattern: self.pattern(),
        }
    }
    pub fn container(&self) -> SyntaxResult<AnyGritContainer> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritPredicateAssignment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateAssignmentFields {
    pub container: SyntaxResult<AnyGritContainer>,
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateCall {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateCall {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateCallFields {
        GritPredicateCallFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            named_args: self.named_args(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<GritName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn named_args(&self) -> GritNamedArgList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for GritPredicateCall {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateCallFields {
    pub name: SyntaxResult<GritName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub named_args: GritNamedArgList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateCurly {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateCurly {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateCurlyFields {
        GritPredicateCurlyFields {
            l_curly_token: self.l_curly_token(),
            predicates: self.predicates(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn predicates(&self) -> GritPredicateList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GritPredicateCurly {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateCurlyFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub predicates: GritPredicateList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateDefinitionFields {
        GritPredicateDefinitionFields {
            predicate_token: self.predicate_token(),
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            args: self.args(),
            r_paren_token: self.r_paren_token(),
            body: self.body(),
        }
    }
    pub fn predicate_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<GritName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn args(&self) -> GritVariableList {
        support::list(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn body(&self) -> SyntaxResult<GritPredicateCurly> {
        support::required_node(&self.syntax, 5usize)
    }
}
impl Serialize for GritPredicateDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateDefinitionFields {
    pub predicate_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GritName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub args: GritVariableList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<GritPredicateCurly>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateElseClause {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateElseClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateElseClauseFields {
        GritPredicateElseClauseFields {
            else_token: self.else_token(),
            else_predicate: self.else_predicate(),
        }
    }
    pub fn else_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn else_predicate(&self) -> SyntaxResult<AnyGritPredicate> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for GritPredicateElseClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateElseClauseFields {
    pub else_token: SyntaxResult<SyntaxToken>,
    pub else_predicate: SyntaxResult<AnyGritPredicate>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateEqual {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateEqual {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateEqualFields {
        GritPredicateEqualFields {
            left: self.left(),
            equality_token: self.equality_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn equality_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritPredicateEqual {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateEqualFields {
    pub left: SyntaxResult<GritVariable>,
    pub equality_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateGreater {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateGreater {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateGreaterFields {
        GritPredicateGreaterFields {
            left: self.left(),
            r_angle_token: self.r_angle_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritPredicateGreater {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateGreaterFields {
    pub left: SyntaxResult<GritVariable>,
    pub r_angle_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateGreaterEqual {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateGreaterEqual {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateGreaterEqualFields {
        GritPredicateGreaterEqualFields {
            left: self.left(),
            greater_than_equal_token: self.greater_than_equal_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn greater_than_equal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritPredicateGreaterEqual {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateGreaterEqualFields {
    pub left: SyntaxResult<GritVariable>,
    pub greater_than_equal_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateIfElse {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateIfElse {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateIfElseFields {
        GritPredicateIfElseFields {
            if_token: self.if_token(),
            l_paren_token: self.l_paren_token(),
            if_predicate: self.if_predicate(),
            r_paren_token: self.r_paren_token(),
            then_predicate: self.then_predicate(),
            else_clause: self.else_clause(),
        }
    }
    pub fn if_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn if_predicate(&self) -> SyntaxResult<AnyGritPredicate> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn then_predicate(&self) -> SyntaxResult<AnyGritPredicate> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn else_clause(&self) -> Option<GritPredicateElseClause> {
        support::node(&self.syntax, 5usize)
    }
}
impl Serialize for GritPredicateIfElse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateIfElseFields {
    pub if_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub if_predicate: SyntaxResult<AnyGritPredicate>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub then_predicate: SyntaxResult<AnyGritPredicate>,
    pub else_clause: Option<GritPredicateElseClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateLess {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateLess {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateLessFields {
        GritPredicateLessFields {
            left: self.left(),
            l_angle_token: self.l_angle_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritPredicateLess {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateLessFields {
    pub left: SyntaxResult<GritVariable>,
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateLessEqual {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateLessEqual {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateLessEqualFields {
        GritPredicateLessEqualFields {
            left: self.left(),
            less_than_equal_token: self.less_than_equal_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn less_than_equal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritPredicateLessEqual {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateLessEqualFields {
    pub left: SyntaxResult<GritVariable>,
    pub less_than_equal_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateMatch {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateMatch {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateMatchFields {
        GritPredicateMatchFields {
            left: self.left(),
            match_token: self.match_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyGritPredicateMatchSubject> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn match_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritPredicateMatch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateMatchFields {
    pub left: SyntaxResult<AnyGritPredicateMatchSubject>,
    pub match_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateMaybe {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateMaybe {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateMaybeFields {
        GritPredicateMaybeFields {
            maybe_token: self.maybe_token(),
            predicate: self.predicate(),
        }
    }
    pub fn maybe_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn predicate(&self) -> SyntaxResult<AnyGritPredicate> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for GritPredicateMaybe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateMaybeFields {
    pub maybe_token: SyntaxResult<SyntaxToken>,
    pub predicate: SyntaxResult<AnyGritPredicate>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateNot {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateNot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateNotFields {
        GritPredicateNotFields {
            not: self.not(),
            predicate: self.predicate(),
        }
    }
    pub fn not(&self) -> SyntaxResult<GritNot> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn predicate(&self) -> SyntaxResult<AnyGritPredicate> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for GritPredicateNot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateNotFields {
    pub not: SyntaxResult<GritNot>,
    pub predicate: SyntaxResult<AnyGritPredicate>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateNotEqual {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateNotEqual {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateNotEqualFields {
        GritPredicateNotEqualFields {
            left: self.left(),
            inequality_token: self.inequality_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn inequality_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritPredicateNotEqual {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateNotEqualFields {
    pub left: SyntaxResult<GritVariable>,
    pub inequality_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateOr {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateOr {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateOrFields {
        GritPredicateOrFields {
            or_token: self.or_token(),
            l_curly_token: self.l_curly_token(),
            predicates: self.predicates(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn predicates(&self) -> GritPredicateList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for GritPredicateOr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateOrFields {
    pub or_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub predicates: GritPredicateList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateReturn {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateReturn {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateReturnFields {
        GritPredicateReturnFields {
            return_token: self.return_token(),
            pattern: self.pattern(),
        }
    }
    pub fn return_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for GritPredicateReturn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateReturnFields {
    pub return_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateRewrite {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateRewrite {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateRewriteFields {
        GritPredicateRewriteFields {
            left: self.left(),
            annotation: self.annotation(),
            fat_arrow_token: self.fat_arrow_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn annotation(&self) -> Option<GritAnnotation> {
        support::node(&self.syntax, 1usize)
    }
    pub fn fat_arrow_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl Serialize for GritPredicateRewrite {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritPredicateRewriteFields {
    pub left: SyntaxResult<GritVariable>,
    pub annotation: Option<GritAnnotation>,
    pub fat_arrow_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritRawBacktickSnippetLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl GritRawBacktickSnippetLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritRawBacktickSnippetLiteralFields {
        GritRawBacktickSnippetLiteralFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritRawBacktickSnippetLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritRawBacktickSnippetLiteralFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritRegexLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl GritRegexLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritRegexLiteralFields {
        GritRegexLiteralFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritRegexLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritRegexLiteralFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritRegexPattern {
    pub(crate) syntax: SyntaxNode,
}
impl GritRegexPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritRegexPatternFields {
        GritRegexPatternFields {
            regex: self.regex(),
            variables: self.variables(),
        }
    }
    pub fn regex(&self) -> SyntaxResult<AnyGritRegex> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn variables(&self) -> Option<GritRegexPatternVariables> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for GritRegexPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritRegexPatternFields {
    pub regex: SyntaxResult<AnyGritRegex>,
    pub variables: Option<GritRegexPatternVariables>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritRegexPatternVariables {
    pub(crate) syntax: SyntaxNode,
}
impl GritRegexPatternVariables {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritRegexPatternVariablesFields {
        GritRegexPatternVariablesFields {
            l_paren_token: self.l_paren_token(),
            args: self.args(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn args(&self) -> GritVariableList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GritRegexPatternVariables {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritRegexPatternVariablesFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub args: GritVariableList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritRewrite {
    pub(crate) syntax: SyntaxNode,
}
impl GritRewrite {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritRewriteFields {
        GritRewriteFields {
            left: self.left(),
            annotation: self.annotation(),
            fat_arrow_token: self.fat_arrow_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn annotation(&self) -> Option<GritAnnotation> {
        support::node(&self.syntax, 1usize)
    }
    pub fn fat_arrow_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl Serialize for GritRewrite {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritRewriteFields {
    pub left: SyntaxResult<AnyGritPattern>,
    pub annotation: Option<GritAnnotation>,
    pub fat_arrow_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritRoot {
    pub(crate) syntax: SyntaxNode,
}
impl GritRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritRootFields {
        GritRootFields {
            bom_token: self.bom_token(),
            version: self.version(),
            language: self.language(),
            definitions: self.definitions(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn version(&self) -> Option<AnyGritVersion> {
        support::node(&self.syntax, 1usize)
    }
    pub fn language(&self) -> Option<AnyGritLanguageDeclaration> {
        support::node(&self.syntax, 2usize)
    }
    pub fn definitions(&self) -> GritDefinitionList {
        support::list(&self.syntax, 3usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl Serialize for GritRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritRootFields {
    pub bom_token: Option<SyntaxToken>,
    pub version: Option<AnyGritVersion>,
    pub language: Option<AnyGritLanguageDeclaration>,
    pub definitions: GritDefinitionList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritSequential {
    pub(crate) syntax: SyntaxNode,
}
impl GritSequential {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritSequentialFields {
        GritSequentialFields {
            sequential_token: self.sequential_token(),
            l_curly_token: self.l_curly_token(),
            sequential: self.sequential(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn sequential_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn sequential(&self) -> GritPatternList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for GritSequential {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritSequentialFields {
    pub sequential_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub sequential: GritPatternList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritSnippetRegexLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl GritSnippetRegexLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritSnippetRegexLiteralFields {
        GritSnippetRegexLiteralFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritSnippetRegexLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritSnippetRegexLiteralFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritSome {
    pub(crate) syntax: SyntaxNode,
}
impl GritSome {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritSomeFields {
        GritSomeFields {
            some_token: self.some_token(),
            pattern: self.pattern(),
        }
    }
    pub fn some_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritMaybeCurlyPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for GritSome {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritSomeFields {
    pub some_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritMaybeCurlyPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritStringLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl GritStringLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritStringLiteralFields {
        GritStringLiteralFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritStringLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritStringLiteralFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritSubOperation {
    pub(crate) syntax: SyntaxNode,
}
impl GritSubOperation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritSubOperationFields {
        GritSubOperationFields {
            left: self.left(),
            minus_token: self.minus_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn minus_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GritSubOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritSubOperationFields {
    pub left: SyntaxResult<AnyGritPattern>,
    pub minus_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritUndefinedLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl GritUndefinedLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritUndefinedLiteralFields {
        GritUndefinedLiteralFields {
            token_token: self.token_token(),
        }
    }
    pub fn token_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritUndefinedLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritUndefinedLiteralFields {
    pub token_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritUnderscore {
    pub(crate) syntax: SyntaxNode,
}
impl GritUnderscore {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritUnderscoreFields {
        GritUnderscoreFields {
            token_token: self.token_token(),
        }
    }
    pub fn token_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritUnderscore {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritUnderscoreFields {
    pub token_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritVariable {
    pub(crate) syntax: SyntaxNode,
}
impl GritVariable {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritVariableFields {
        GritVariableFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GritVariable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritVariableFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritVersion {
    pub(crate) syntax: SyntaxNode,
}
impl GritVersion {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritVersionFields {
        GritVersionFields {
            engine_token: self.engine_token(),
            engine_name: self.engine_name(),
            l_paren_token: self.l_paren_token(),
            version: self.version(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn engine_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn engine_name(&self) -> SyntaxResult<GritEngineName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn version(&self) -> SyntaxResult<GritDoubleLiteral> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl Serialize for GritVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritVersionFields {
    pub engine_token: SyntaxResult<SyntaxToken>,
    pub engine_name: SyntaxResult<GritEngineName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub version: SyntaxResult<GritDoubleLiteral>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritWithin {
    pub(crate) syntax: SyntaxNode,
}
impl GritWithin {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritWithinFields {
        GritWithinFields {
            within_token: self.within_token(),
            pattern: self.pattern(),
            until_clause: self.until_clause(),
        }
    }
    pub fn within_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritMaybeCurlyPattern> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn until_clause(&self) -> Option<GritPatternUntilClause> {
        support::node(&self.syntax, 2usize)
    }
}
impl Serialize for GritWithin {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GritWithinFields {
    pub within_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritMaybeCurlyPattern>,
    pub until_clause: Option<GritPatternUntilClause>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritCodeSnippetSource {
    GritBacktickSnippetLiteral(GritBacktickSnippetLiteral),
    GritLanguageSpecificSnippet(GritLanguageSpecificSnippet),
    GritRawBacktickSnippetLiteral(GritRawBacktickSnippetLiteral),
}
impl AnyGritCodeSnippetSource {
    pub fn as_grit_backtick_snippet_literal(&self) -> Option<&GritBacktickSnippetLiteral> {
        match &self {
            Self::GritBacktickSnippetLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_language_specific_snippet(&self) -> Option<&GritLanguageSpecificSnippet> {
        match &self {
            Self::GritLanguageSpecificSnippet(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_raw_backtick_snippet_literal(&self) -> Option<&GritRawBacktickSnippetLiteral> {
        match &self {
            Self::GritRawBacktickSnippetLiteral(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritContainer {
    GritBogusContainer(GritBogusContainer),
    GritListAccessor(GritListAccessor),
    GritMapAccessor(GritMapAccessor),
    GritVariable(GritVariable),
}
impl AnyGritContainer {
    pub fn as_grit_bogus_container(&self) -> Option<&GritBogusContainer> {
        match &self {
            Self::GritBogusContainer(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_list_accessor(&self) -> Option<&GritListAccessor> {
        match &self {
            Self::GritListAccessor(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_map_accessor(&self) -> Option<&GritMapAccessor> {
        match &self {
            Self::GritMapAccessor(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_variable(&self) -> Option<&GritVariable> {
        match &self {
            Self::GritVariable(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritDefinition {
    AnyGritPattern(AnyGritPattern),
    GritBogusDefinition(GritBogusDefinition),
    GritFunctionDefinition(GritFunctionDefinition),
    GritJavascriptFunctionDefinition(GritJavascriptFunctionDefinition),
    GritPatternDefinition(GritPatternDefinition),
    GritPredicateDefinition(GritPredicateDefinition),
}
impl AnyGritDefinition {
    pub fn as_any_grit_pattern(&self) -> Option<&AnyGritPattern> {
        match &self {
            Self::AnyGritPattern(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_bogus_definition(&self) -> Option<&GritBogusDefinition> {
        match &self {
            Self::GritBogusDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_function_definition(&self) -> Option<&GritFunctionDefinition> {
        match &self {
            Self::GritFunctionDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_javascript_function_definition(
        &self,
    ) -> Option<&GritJavascriptFunctionDefinition> {
        match &self {
            Self::GritJavascriptFunctionDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_pattern_definition(&self) -> Option<&GritPatternDefinition> {
        match &self {
            Self::GritPatternDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_definition(&self) -> Option<&GritPredicateDefinition> {
        match &self {
            Self::GritPredicateDefinition(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritLanguageDeclaration {
    GritBogusLanguageDeclaration(GritBogusLanguageDeclaration),
    GritLanguageDeclaration(GritLanguageDeclaration),
}
impl AnyGritLanguageDeclaration {
    pub fn as_grit_bogus_language_declaration(&self) -> Option<&GritBogusLanguageDeclaration> {
        match &self {
            Self::GritBogusLanguageDeclaration(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_language_declaration(&self) -> Option<&GritLanguageDeclaration> {
        match &self {
            Self::GritLanguageDeclaration(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritLanguageFlavorKind {
    GritBogusLanguageFlavorKind(GritBogusLanguageFlavorKind),
    GritLanguageFlavorKind(GritLanguageFlavorKind),
}
impl AnyGritLanguageFlavorKind {
    pub fn as_grit_bogus_language_flavor_kind(&self) -> Option<&GritBogusLanguageFlavorKind> {
        match &self {
            Self::GritBogusLanguageFlavorKind(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_language_flavor_kind(&self) -> Option<&GritLanguageFlavorKind> {
        match &self {
            Self::GritLanguageFlavorKind(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritLanguageName {
    GritBogusLanguageName(GritBogusLanguageName),
    GritLanguageName(GritLanguageName),
}
impl AnyGritLanguageName {
    pub fn as_grit_bogus_language_name(&self) -> Option<&GritBogusLanguageName> {
        match &self {
            Self::GritBogusLanguageName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_language_name(&self) -> Option<&GritLanguageName> {
        match &self {
            Self::GritLanguageName(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritListAccessorSubject {
    AnyGritContainer(AnyGritContainer),
    GritList(GritList),
}
impl AnyGritListAccessorSubject {
    pub fn as_any_grit_container(&self) -> Option<&AnyGritContainer> {
        match &self {
            Self::AnyGritContainer(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_list(&self) -> Option<&GritList> {
        match &self {
            Self::GritList(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritListIndex {
    AnyGritContainer(AnyGritContainer),
    GritIntLiteral(GritIntLiteral),
    GritNegativeIntLiteral(GritNegativeIntLiteral),
}
impl AnyGritListIndex {
    pub fn as_any_grit_container(&self) -> Option<&AnyGritContainer> {
        match &self {
            Self::AnyGritContainer(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_int_literal(&self) -> Option<&GritIntLiteral> {
        match &self {
            Self::GritIntLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_negative_int_literal(&self) -> Option<&GritNegativeIntLiteral> {
        match &self {
            Self::GritNegativeIntLiteral(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritListPattern {
    AnyGritPattern(AnyGritPattern),
    GritDotdotdot(GritDotdotdot),
}
impl AnyGritListPattern {
    pub fn as_any_grit_pattern(&self) -> Option<&AnyGritPattern> {
        match &self {
            Self::AnyGritPattern(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_dotdotdot(&self) -> Option<&GritDotdotdot> {
        match &self {
            Self::GritDotdotdot(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritLiteral {
    GritBogusLiteral(GritBogusLiteral),
    GritBooleanLiteral(GritBooleanLiteral),
    GritCodeSnippet(GritCodeSnippet),
    GritDoubleLiteral(GritDoubleLiteral),
    GritIntLiteral(GritIntLiteral),
    GritList(GritList),
    GritMap(GritMap),
    GritStringLiteral(GritStringLiteral),
    GritUndefinedLiteral(GritUndefinedLiteral),
}
impl AnyGritLiteral {
    pub fn as_grit_bogus_literal(&self) -> Option<&GritBogusLiteral> {
        match &self {
            Self::GritBogusLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_boolean_literal(&self) -> Option<&GritBooleanLiteral> {
        match &self {
            Self::GritBooleanLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_code_snippet(&self) -> Option<&GritCodeSnippet> {
        match &self {
            Self::GritCodeSnippet(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_double_literal(&self) -> Option<&GritDoubleLiteral> {
        match &self {
            Self::GritDoubleLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_int_literal(&self) -> Option<&GritIntLiteral> {
        match &self {
            Self::GritIntLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_list(&self) -> Option<&GritList> {
        match &self {
            Self::GritList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_map(&self) -> Option<&GritMap> {
        match &self {
            Self::GritMap(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_string_literal(&self) -> Option<&GritStringLiteral> {
        match &self {
            Self::GritStringLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_undefined_literal(&self) -> Option<&GritUndefinedLiteral> {
        match &self {
            Self::GritUndefinedLiteral(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritMapAccessorSubject {
    AnyGritContainer(AnyGritContainer),
    GritMap(GritMap),
}
impl AnyGritMapAccessorSubject {
    pub fn as_any_grit_container(&self) -> Option<&AnyGritContainer> {
        match &self {
            Self::AnyGritContainer(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_map(&self) -> Option<&GritMap> {
        match &self {
            Self::GritMap(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritMapElement {
    GritBogusMapElement(GritBogusMapElement),
    GritMapElement(GritMapElement),
}
impl AnyGritMapElement {
    pub fn as_grit_bogus_map_element(&self) -> Option<&GritBogusMapElement> {
        match &self {
            Self::GritBogusMapElement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_map_element(&self) -> Option<&GritMapElement> {
        match &self {
            Self::GritMapElement(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritMapKey {
    GritName(GritName),
    GritVariable(GritVariable),
}
impl AnyGritMapKey {
    pub fn as_grit_name(&self) -> Option<&GritName> {
        match &self {
            Self::GritName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_variable(&self) -> Option<&GritVariable> {
        match &self {
            Self::GritVariable(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritMaybeCurlyPattern {
    AnyGritPattern(AnyGritPattern),
    GritCurlyPattern(GritCurlyPattern),
}
impl AnyGritMaybeCurlyPattern {
    pub fn as_any_grit_pattern(&self) -> Option<&AnyGritPattern> {
        match &self {
            Self::AnyGritPattern(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_curly_pattern(&self) -> Option<&GritCurlyPattern> {
        match &self {
            Self::GritCurlyPattern(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritMaybeNamedArg {
    AnyGritPattern(AnyGritPattern),
    GritBogusNamedArg(GritBogusNamedArg),
    GritNamedArg(GritNamedArg),
}
impl AnyGritMaybeNamedArg {
    pub fn as_any_grit_pattern(&self) -> Option<&AnyGritPattern> {
        match &self {
            Self::AnyGritPattern(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_bogus_named_arg(&self) -> Option<&GritBogusNamedArg> {
        match &self {
            Self::GritBogusNamedArg(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_named_arg(&self) -> Option<&GritNamedArg> {
        match &self {
            Self::GritNamedArg(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritPattern {
    AnyGritLiteral(AnyGritLiteral),
    GritAddOperation(GritAddOperation),
    GritAssignmentAsPattern(GritAssignmentAsPattern),
    GritBogusPattern(GritBogusPattern),
    GritBracketedPattern(GritBracketedPattern),
    GritBubble(GritBubble),
    GritDivOperation(GritDivOperation),
    GritDot(GritDot),
    GritEvery(GritEvery),
    GritFiles(GritFiles),
    GritLike(GritLike),
    GritListAccessor(GritListAccessor),
    GritMapAccessor(GritMapAccessor),
    GritModOperation(GritModOperation),
    GritMulOperation(GritMulOperation),
    GritNodeLike(GritNodeLike),
    GritPatternAccumulate(GritPatternAccumulate),
    GritPatternAfter(GritPatternAfter),
    GritPatternAnd(GritPatternAnd),
    GritPatternAny(GritPatternAny),
    GritPatternAs(GritPatternAs),
    GritPatternBefore(GritPatternBefore),
    GritPatternContains(GritPatternContains),
    GritPatternIfElse(GritPatternIfElse),
    GritPatternIncludes(GritPatternIncludes),
    GritPatternLimit(GritPatternLimit),
    GritPatternMaybe(GritPatternMaybe),
    GritPatternNot(GritPatternNot),
    GritPatternOr(GritPatternOr),
    GritPatternOrElse(GritPatternOrElse),
    GritPatternWhere(GritPatternWhere),
    GritRegexPattern(GritRegexPattern),
    GritRewrite(GritRewrite),
    GritSequential(GritSequential),
    GritSome(GritSome),
    GritSubOperation(GritSubOperation),
    GritUnderscore(GritUnderscore),
    GritVariable(GritVariable),
    GritWithin(GritWithin),
}
impl AnyGritPattern {
    pub fn as_any_grit_literal(&self) -> Option<&AnyGritLiteral> {
        match &self {
            Self::AnyGritLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_add_operation(&self) -> Option<&GritAddOperation> {
        match &self {
            Self::GritAddOperation(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_assignment_as_pattern(&self) -> Option<&GritAssignmentAsPattern> {
        match &self {
            Self::GritAssignmentAsPattern(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_bogus_pattern(&self) -> Option<&GritBogusPattern> {
        match &self {
            Self::GritBogusPattern(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_bracketed_pattern(&self) -> Option<&GritBracketedPattern> {
        match &self {
            Self::GritBracketedPattern(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_bubble(&self) -> Option<&GritBubble> {
        match &self {
            Self::GritBubble(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_div_operation(&self) -> Option<&GritDivOperation> {
        match &self {
            Self::GritDivOperation(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_dot(&self) -> Option<&GritDot> {
        match &self {
            Self::GritDot(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_every(&self) -> Option<&GritEvery> {
        match &self {
            Self::GritEvery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_files(&self) -> Option<&GritFiles> {
        match &self {
            Self::GritFiles(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_like(&self) -> Option<&GritLike> {
        match &self {
            Self::GritLike(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_list_accessor(&self) -> Option<&GritListAccessor> {
        match &self {
            Self::GritListAccessor(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_map_accessor(&self) -> Option<&GritMapAccessor> {
        match &self {
            Self::GritMapAccessor(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_mod_operation(&self) -> Option<&GritModOperation> {
        match &self {
            Self::GritModOperation(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_mul_operation(&self) -> Option<&GritMulOperation> {
        match &self {
            Self::GritMulOperation(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_node_like(&self) -> Option<&GritNodeLike> {
        match &self {
            Self::GritNodeLike(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_pattern_accumulate(&self) -> Option<&GritPatternAccumulate> {
        match &self {
            Self::GritPatternAccumulate(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_pattern_after(&self) -> Option<&GritPatternAfter> {
        match &self {
            Self::GritPatternAfter(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_pattern_and(&self) -> Option<&GritPatternAnd> {
        match &self {
            Self::GritPatternAnd(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_pattern_any(&self) -> Option<&GritPatternAny> {
        match &self {
            Self::GritPatternAny(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_pattern_as(&self) -> Option<&GritPatternAs> {
        match &self {
            Self::GritPatternAs(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_pattern_before(&self) -> Option<&GritPatternBefore> {
        match &self {
            Self::GritPatternBefore(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_pattern_contains(&self) -> Option<&GritPatternContains> {
        match &self {
            Self::GritPatternContains(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_pattern_if_else(&self) -> Option<&GritPatternIfElse> {
        match &self {
            Self::GritPatternIfElse(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_pattern_includes(&self) -> Option<&GritPatternIncludes> {
        match &self {
            Self::GritPatternIncludes(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_pattern_limit(&self) -> Option<&GritPatternLimit> {
        match &self {
            Self::GritPatternLimit(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_pattern_maybe(&self) -> Option<&GritPatternMaybe> {
        match &self {
            Self::GritPatternMaybe(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_pattern_not(&self) -> Option<&GritPatternNot> {
        match &self {
            Self::GritPatternNot(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_pattern_or(&self) -> Option<&GritPatternOr> {
        match &self {
            Self::GritPatternOr(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_pattern_or_else(&self) -> Option<&GritPatternOrElse> {
        match &self {
            Self::GritPatternOrElse(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_pattern_where(&self) -> Option<&GritPatternWhere> {
        match &self {
            Self::GritPatternWhere(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_regex_pattern(&self) -> Option<&GritRegexPattern> {
        match &self {
            Self::GritRegexPattern(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_rewrite(&self) -> Option<&GritRewrite> {
        match &self {
            Self::GritRewrite(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_sequential(&self) -> Option<&GritSequential> {
        match &self {
            Self::GritSequential(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_some(&self) -> Option<&GritSome> {
        match &self {
            Self::GritSome(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_sub_operation(&self) -> Option<&GritSubOperation> {
        match &self {
            Self::GritSubOperation(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_underscore(&self) -> Option<&GritUnderscore> {
        match &self {
            Self::GritUnderscore(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_variable(&self) -> Option<&GritVariable> {
        match &self {
            Self::GritVariable(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_within(&self) -> Option<&GritWithin> {
        match &self {
            Self::GritWithin(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritPredicate {
    GritBogusPredicate(GritBogusPredicate),
    GritBooleanLiteral(GritBooleanLiteral),
    GritBracketedPredicate(GritBracketedPredicate),
    GritPredicateAccumulate(GritPredicateAccumulate),
    GritPredicateAnd(GritPredicateAnd),
    GritPredicateAny(GritPredicateAny),
    GritPredicateAssignment(GritPredicateAssignment),
    GritPredicateCall(GritPredicateCall),
    GritPredicateEqual(GritPredicateEqual),
    GritPredicateGreater(GritPredicateGreater),
    GritPredicateGreaterEqual(GritPredicateGreaterEqual),
    GritPredicateIfElse(GritPredicateIfElse),
    GritPredicateLess(GritPredicateLess),
    GritPredicateLessEqual(GritPredicateLessEqual),
    GritPredicateMatch(GritPredicateMatch),
    GritPredicateMaybe(GritPredicateMaybe),
    GritPredicateNot(GritPredicateNot),
    GritPredicateNotEqual(GritPredicateNotEqual),
    GritPredicateOr(GritPredicateOr),
    GritPredicateReturn(GritPredicateReturn),
    GritPredicateRewrite(GritPredicateRewrite),
}
impl AnyGritPredicate {
    pub fn as_grit_bogus_predicate(&self) -> Option<&GritBogusPredicate> {
        match &self {
            Self::GritBogusPredicate(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_boolean_literal(&self) -> Option<&GritBooleanLiteral> {
        match &self {
            Self::GritBooleanLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_bracketed_predicate(&self) -> Option<&GritBracketedPredicate> {
        match &self {
            Self::GritBracketedPredicate(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_accumulate(&self) -> Option<&GritPredicateAccumulate> {
        match &self {
            Self::GritPredicateAccumulate(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_and(&self) -> Option<&GritPredicateAnd> {
        match &self {
            Self::GritPredicateAnd(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_any(&self) -> Option<&GritPredicateAny> {
        match &self {
            Self::GritPredicateAny(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_assignment(&self) -> Option<&GritPredicateAssignment> {
        match &self {
            Self::GritPredicateAssignment(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_call(&self) -> Option<&GritPredicateCall> {
        match &self {
            Self::GritPredicateCall(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_equal(&self) -> Option<&GritPredicateEqual> {
        match &self {
            Self::GritPredicateEqual(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_greater(&self) -> Option<&GritPredicateGreater> {
        match &self {
            Self::GritPredicateGreater(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_greater_equal(&self) -> Option<&GritPredicateGreaterEqual> {
        match &self {
            Self::GritPredicateGreaterEqual(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_if_else(&self) -> Option<&GritPredicateIfElse> {
        match &self {
            Self::GritPredicateIfElse(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_less(&self) -> Option<&GritPredicateLess> {
        match &self {
            Self::GritPredicateLess(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_less_equal(&self) -> Option<&GritPredicateLessEqual> {
        match &self {
            Self::GritPredicateLessEqual(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_match(&self) -> Option<&GritPredicateMatch> {
        match &self {
            Self::GritPredicateMatch(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_maybe(&self) -> Option<&GritPredicateMaybe> {
        match &self {
            Self::GritPredicateMaybe(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_not(&self) -> Option<&GritPredicateNot> {
        match &self {
            Self::GritPredicateNot(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_not_equal(&self) -> Option<&GritPredicateNotEqual> {
        match &self {
            Self::GritPredicateNotEqual(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_or(&self) -> Option<&GritPredicateOr> {
        match &self {
            Self::GritPredicateOr(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_return(&self) -> Option<&GritPredicateReturn> {
        match &self {
            Self::GritPredicateReturn(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_rewrite(&self) -> Option<&GritPredicateRewrite> {
        match &self {
            Self::GritPredicateRewrite(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritPredicateMatchSubject {
    AnyGritContainer(AnyGritContainer),
    AnyGritLiteral(AnyGritLiteral),
}
impl AnyGritPredicateMatchSubject {
    pub fn as_any_grit_container(&self) -> Option<&AnyGritContainer> {
        match &self {
            Self::AnyGritContainer(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_any_grit_literal(&self) -> Option<&AnyGritLiteral> {
        match &self {
            Self::AnyGritLiteral(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritRegex {
    GritRegexLiteral(GritRegexLiteral),
    GritSnippetRegexLiteral(GritSnippetRegexLiteral),
}
impl AnyGritRegex {
    pub fn as_grit_regex_literal(&self) -> Option<&GritRegexLiteral> {
        match &self {
            Self::GritRegexLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_snippet_regex_literal(&self) -> Option<&GritSnippetRegexLiteral> {
        match &self {
            Self::GritSnippetRegexLiteral(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGritVersion {
    GritBogusVersion(GritBogusVersion),
    GritVersion(GritVersion),
}
impl AnyGritVersion {
    pub fn as_grit_bogus_version(&self) -> Option<&GritBogusVersion> {
        match &self {
            Self::GritBogusVersion(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_version(&self) -> Option<&GritVersion> {
        match &self {
            Self::GritVersion(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for GritAddOperation {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_ADD_OPERATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_ADD_OPERATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritAddOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritAddOperation")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field("plus_token", &support::DebugSyntaxResult(self.plus_token()))
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("GritAddOperation").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritAddOperation> for SyntaxNode {
    fn from(n: GritAddOperation) -> Self {
        n.syntax
    }
}
impl From<GritAddOperation> for SyntaxElement {
    fn from(n: GritAddOperation) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritAnnotation {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_ANNOTATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_ANNOTATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritAnnotation")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritAnnotation").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritAnnotation> for SyntaxNode {
    fn from(n: GritAnnotation) -> Self {
        n.syntax
    }
}
impl From<GritAnnotation> for SyntaxElement {
    fn from(n: GritAnnotation) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritAssignmentAsPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_ASSIGNMENT_AS_PATTERN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_ASSIGNMENT_AS_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritAssignmentAsPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritAssignmentAsPattern")
                .field("container", &support::DebugSyntaxResult(self.container()))
                .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .finish()
        } else {
            f.debug_struct("GritAssignmentAsPattern").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritAssignmentAsPattern> for SyntaxNode {
    fn from(n: GritAssignmentAsPattern) -> Self {
        n.syntax
    }
}
impl From<GritAssignmentAsPattern> for SyntaxElement {
    fn from(n: GritAssignmentAsPattern) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritBacktickSnippetLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BACKTICK_SNIPPET_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BACKTICK_SNIPPET_LITERAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBacktickSnippetLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritBacktickSnippetLiteral")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritBacktickSnippetLiteral").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritBacktickSnippetLiteral> for SyntaxNode {
    fn from(n: GritBacktickSnippetLiteral) -> Self {
        n.syntax
    }
}
impl From<GritBacktickSnippetLiteral> for SyntaxElement {
    fn from(n: GritBacktickSnippetLiteral) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritBooleanLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOOLEAN_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOOLEAN_LITERAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBooleanLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritBooleanLiteral")
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("GritBooleanLiteral").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritBooleanLiteral> for SyntaxNode {
    fn from(n: GritBooleanLiteral) -> Self {
        n.syntax
    }
}
impl From<GritBooleanLiteral> for SyntaxElement {
    fn from(n: GritBooleanLiteral) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritBracketedPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BRACKETED_PATTERN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BRACKETED_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBracketedPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritBracketedPattern")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritBracketedPattern").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritBracketedPattern> for SyntaxNode {
    fn from(n: GritBracketedPattern) -> Self {
        n.syntax
    }
}
impl From<GritBracketedPattern> for SyntaxElement {
    fn from(n: GritBracketedPattern) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritBracketedPredicate {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BRACKETED_PREDICATE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BRACKETED_PREDICATE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBracketedPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritBracketedPredicate")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("predicate", &support::DebugSyntaxResult(self.predicate()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritBracketedPredicate").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritBracketedPredicate> for SyntaxNode {
    fn from(n: GritBracketedPredicate) -> Self {
        n.syntax
    }
}
impl From<GritBracketedPredicate> for SyntaxElement {
    fn from(n: GritBracketedPredicate) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritBubble {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BUBBLE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BUBBLE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBubble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritBubble")
                .field(
                    "bubble_token",
                    &support::DebugSyntaxResult(self.bubble_token()),
                )
                .field("scope", &support::DebugOptionalElement(self.scope()))
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .finish()
        } else {
            f.debug_struct("GritBubble").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritBubble> for SyntaxNode {
    fn from(n: GritBubble) -> Self {
        n.syntax
    }
}
impl From<GritBubble> for SyntaxElement {
    fn from(n: GritBubble) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritBubbleScope {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BUBBLE_SCOPE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BUBBLE_SCOPE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBubbleScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritBubbleScope")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("variables", &self.variables())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritBubbleScope").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritBubbleScope> for SyntaxNode {
    fn from(n: GritBubbleScope) -> Self {
        n.syntax
    }
}
impl From<GritBubbleScope> for SyntaxElement {
    fn from(n: GritBubbleScope) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritCodeSnippet {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_CODE_SNIPPET as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_CODE_SNIPPET
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritCodeSnippet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritCodeSnippet")
                .field("source", &support::DebugSyntaxResult(self.source()))
                .finish()
        } else {
            f.debug_struct("GritCodeSnippet").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritCodeSnippet> for SyntaxNode {
    fn from(n: GritCodeSnippet) -> Self {
        n.syntax
    }
}
impl From<GritCodeSnippet> for SyntaxElement {
    fn from(n: GritCodeSnippet) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritCurlyPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_CURLY_PATTERN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_CURLY_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritCurlyPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritCurlyPattern")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritCurlyPattern").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritCurlyPattern> for SyntaxNode {
    fn from(n: GritCurlyPattern) -> Self {
        n.syntax
    }
}
impl From<GritCurlyPattern> for SyntaxElement {
    fn from(n: GritCurlyPattern) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritDivOperation {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_DIV_OPERATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_DIV_OPERATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritDivOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritDivOperation")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "slash_token",
                    &support::DebugSyntaxResult(self.slash_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("GritDivOperation").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritDivOperation> for SyntaxNode {
    fn from(n: GritDivOperation) -> Self {
        n.syntax
    }
}
impl From<GritDivOperation> for SyntaxElement {
    fn from(n: GritDivOperation) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritDot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_DOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_DOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritDot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritDot")
                .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
                .finish()
        } else {
            f.debug_struct("GritDot").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritDot> for SyntaxNode {
    fn from(n: GritDot) -> Self {
        n.syntax
    }
}
impl From<GritDot> for SyntaxElement {
    fn from(n: GritDot) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritDotdotdot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_DOTDOTDOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_DOTDOTDOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritDotdotdot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritDotdotdot")
                .field(
                    "dotdotdot_token",
                    &support::DebugSyntaxResult(self.dotdotdot_token()),
                )
                .field("pattern", &support::DebugOptionalElement(self.pattern()))
                .finish()
        } else {
            f.debug_struct("GritDotdotdot").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritDotdotdot> for SyntaxNode {
    fn from(n: GritDotdotdot) -> Self {
        n.syntax
    }
}
impl From<GritDotdotdot> for SyntaxElement {
    fn from(n: GritDotdotdot) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritDoubleLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_DOUBLE_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_DOUBLE_LITERAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritDoubleLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritDoubleLiteral")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritDoubleLiteral").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritDoubleLiteral> for SyntaxNode {
    fn from(n: GritDoubleLiteral) -> Self {
        n.syntax
    }
}
impl From<GritDoubleLiteral> for SyntaxElement {
    fn from(n: GritDoubleLiteral) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritEngineName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_ENGINE_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_ENGINE_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritEngineName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritEngineName")
                .field(
                    "engine_kind",
                    &support::DebugSyntaxResult(self.engine_kind()),
                )
                .finish()
        } else {
            f.debug_struct("GritEngineName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritEngineName> for SyntaxNode {
    fn from(n: GritEngineName) -> Self {
        n.syntax
    }
}
impl From<GritEngineName> for SyntaxElement {
    fn from(n: GritEngineName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritEvery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_EVERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_EVERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritEvery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritEvery")
                .field(
                    "every_token",
                    &support::DebugSyntaxResult(self.every_token()),
                )
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .finish()
        } else {
            f.debug_struct("GritEvery").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritEvery> for SyntaxNode {
    fn from(n: GritEvery) -> Self {
        n.syntax
    }
}
impl From<GritEvery> for SyntaxElement {
    fn from(n: GritEvery) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritFiles {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_FILES as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_FILES
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritFiles")
                .field(
                    "multifile_token",
                    &support::DebugSyntaxResult(self.multifile_token()),
                )
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("files", &self.files())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritFiles").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritFiles> for SyntaxNode {
    fn from(n: GritFiles) -> Self {
        n.syntax
    }
}
impl From<GritFiles> for SyntaxElement {
    fn from(n: GritFiles) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritFunctionDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_FUNCTION_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_FUNCTION_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritFunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritFunctionDefinition")
                .field(
                    "function_token",
                    &support::DebugSyntaxResult(self.function_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("args", &self.args())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field("body", &support::DebugSyntaxResult(self.body()))
                .finish()
        } else {
            f.debug_struct("GritFunctionDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritFunctionDefinition> for SyntaxNode {
    fn from(n: GritFunctionDefinition) -> Self {
        n.syntax
    }
}
impl From<GritFunctionDefinition> for SyntaxElement {
    fn from(n: GritFunctionDefinition) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritIntLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_INT_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_INT_LITERAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritIntLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritIntLiteral")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritIntLiteral").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritIntLiteral> for SyntaxNode {
    fn from(n: GritIntLiteral) -> Self {
        n.syntax
    }
}
impl From<GritIntLiteral> for SyntaxElement {
    fn from(n: GritIntLiteral) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritJavascriptBodyWrapper {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_JAVASCRIPT_BODY_WRAPPER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_JAVASCRIPT_BODY_WRAPPER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritJavascriptBodyWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritJavascriptBodyWrapper")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritJavascriptBodyWrapper").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritJavascriptBodyWrapper> for SyntaxNode {
    fn from(n: GritJavascriptBodyWrapper) -> Self {
        n.syntax
    }
}
impl From<GritJavascriptBodyWrapper> for SyntaxElement {
    fn from(n: GritJavascriptBodyWrapper) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritJavascriptFunctionDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_JAVASCRIPT_FUNCTION_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_JAVASCRIPT_FUNCTION_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritJavascriptFunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritJavascriptFunctionDefinition")
                .field(
                    "function_token",
                    &support::DebugSyntaxResult(self.function_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("args", &self.args())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field("js_token", &support::DebugSyntaxResult(self.js_token()))
                .field(
                    "grit_javascript_body_wrapper",
                    &support::DebugSyntaxResult(self.grit_javascript_body_wrapper()),
                )
                .finish()
        } else {
            f.debug_struct("GritJavascriptFunctionDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritJavascriptFunctionDefinition> for SyntaxNode {
    fn from(n: GritJavascriptFunctionDefinition) -> Self {
        n.syntax
    }
}
impl From<GritJavascriptFunctionDefinition> for SyntaxElement {
    fn from(n: GritJavascriptFunctionDefinition) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritLanguageDeclaration {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LANGUAGE_DECLARATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LANGUAGE_DECLARATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritLanguageDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritLanguageDeclaration")
                .field(
                    "language_token",
                    &support::DebugSyntaxResult(self.language_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("flavor", &support::DebugOptionalElement(self.flavor()))
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritLanguageDeclaration").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritLanguageDeclaration> for SyntaxNode {
    fn from(n: GritLanguageDeclaration) -> Self {
        n.syntax
    }
}
impl From<GritLanguageDeclaration> for SyntaxElement {
    fn from(n: GritLanguageDeclaration) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritLanguageFlavor {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LANGUAGE_FLAVOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LANGUAGE_FLAVOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritLanguageFlavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritLanguageFlavor")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("flavors", &self.flavors())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritLanguageFlavor").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritLanguageFlavor> for SyntaxNode {
    fn from(n: GritLanguageFlavor) -> Self {
        n.syntax
    }
}
impl From<GritLanguageFlavor> for SyntaxElement {
    fn from(n: GritLanguageFlavor) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritLanguageFlavorKind {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LANGUAGE_FLAVOR_KIND as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LANGUAGE_FLAVOR_KIND
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritLanguageFlavorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritLanguageFlavorKind")
                .field(
                    "flavor_kind",
                    &support::DebugSyntaxResult(self.flavor_kind()),
                )
                .finish()
        } else {
            f.debug_struct("GritLanguageFlavorKind").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritLanguageFlavorKind> for SyntaxNode {
    fn from(n: GritLanguageFlavorKind) -> Self {
        n.syntax
    }
}
impl From<GritLanguageFlavorKind> for SyntaxElement {
    fn from(n: GritLanguageFlavorKind) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritLanguageName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LANGUAGE_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LANGUAGE_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritLanguageName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritLanguageName")
                .field(
                    "language_kind",
                    &support::DebugSyntaxResult(self.language_kind()),
                )
                .finish()
        } else {
            f.debug_struct("GritLanguageName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritLanguageName> for SyntaxNode {
    fn from(n: GritLanguageName) -> Self {
        n.syntax
    }
}
impl From<GritLanguageName> for SyntaxElement {
    fn from(n: GritLanguageName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritLanguageSpecificSnippet {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LANGUAGE_SPECIFIC_SNIPPET as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LANGUAGE_SPECIFIC_SNIPPET
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritLanguageSpecificSnippet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritLanguageSpecificSnippet")
                .field("language", &support::DebugSyntaxResult(self.language()))
                .field(
                    "snippet_token",
                    &support::DebugSyntaxResult(self.snippet_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritLanguageSpecificSnippet").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritLanguageSpecificSnippet> for SyntaxNode {
    fn from(n: GritLanguageSpecificSnippet) -> Self {
        n.syntax
    }
}
impl From<GritLanguageSpecificSnippet> for SyntaxElement {
    fn from(n: GritLanguageSpecificSnippet) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritLike {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LIKE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LIKE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritLike")
                .field("like_token", &support::DebugSyntaxResult(self.like_token()))
                .field(
                    "threshold",
                    &support::DebugOptionalElement(self.threshold()),
                )
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("example", &support::DebugSyntaxResult(self.example()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritLike").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritLike> for SyntaxNode {
    fn from(n: GritLike) -> Self {
        n.syntax
    }
}
impl From<GritLike> for SyntaxElement {
    fn from(n: GritLike) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritLikeThreshold {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LIKE_THRESHOLD as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LIKE_THRESHOLD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritLikeThreshold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritLikeThreshold")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("threshold", &support::DebugSyntaxResult(self.threshold()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritLikeThreshold").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritLikeThreshold> for SyntaxNode {
    fn from(n: GritLikeThreshold) -> Self {
        n.syntax
    }
}
impl From<GritLikeThreshold> for SyntaxElement {
    fn from(n: GritLikeThreshold) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritList")
                .field("name", &support::DebugOptionalElement(self.name()))
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("patterns", &self.patterns())
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritList").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritList> for SyntaxNode {
    fn from(n: GritList) -> Self {
        n.syntax
    }
}
impl From<GritList> for SyntaxElement {
    fn from(n: GritList) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritListAccessor {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LIST_ACCESSOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LIST_ACCESSOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritListAccessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritListAccessor")
                .field("list", &support::DebugSyntaxResult(self.list()))
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("index", &support::DebugSyntaxResult(self.index()))
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritListAccessor").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritListAccessor> for SyntaxNode {
    fn from(n: GritListAccessor) -> Self {
        n.syntax
    }
}
impl From<GritListAccessor> for SyntaxElement {
    fn from(n: GritListAccessor) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritMap {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_MAP as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_MAP
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritMap")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("elements", &self.elements())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritMap").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritMap> for SyntaxNode {
    fn from(n: GritMap) -> Self {
        n.syntax
    }
}
impl From<GritMap> for SyntaxElement {
    fn from(n: GritMap) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritMapAccessor {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_MAP_ACCESSOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_MAP_ACCESSOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritMapAccessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritMapAccessor")
                .field("map", &support::DebugSyntaxResult(self.map()))
                .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
                .field("key", &support::DebugSyntaxResult(self.key()))
                .finish()
        } else {
            f.debug_struct("GritMapAccessor").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritMapAccessor> for SyntaxNode {
    fn from(n: GritMapAccessor) -> Self {
        n.syntax
    }
}
impl From<GritMapAccessor> for SyntaxElement {
    fn from(n: GritMapAccessor) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritMapElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_MAP_ELEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_MAP_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritMapElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritMapElement")
                .field("key", &support::DebugSyntaxResult(self.key()))
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("GritMapElement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritMapElement> for SyntaxNode {
    fn from(n: GritMapElement) -> Self {
        n.syntax
    }
}
impl From<GritMapElement> for SyntaxElement {
    fn from(n: GritMapElement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritModOperation {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_MOD_OPERATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_MOD_OPERATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritModOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritModOperation")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "remainder_token",
                    &support::DebugSyntaxResult(self.remainder_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("GritModOperation").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritModOperation> for SyntaxNode {
    fn from(n: GritModOperation) -> Self {
        n.syntax
    }
}
impl From<GritModOperation> for SyntaxElement {
    fn from(n: GritModOperation) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritMulOperation {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_MUL_OPERATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_MUL_OPERATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritMulOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritMulOperation")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field("star_token", &support::DebugSyntaxResult(self.star_token()))
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("GritMulOperation").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritMulOperation> for SyntaxNode {
    fn from(n: GritMulOperation) -> Self {
        n.syntax
    }
}
impl From<GritMulOperation> for SyntaxElement {
    fn from(n: GritMulOperation) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritName")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritName> for SyntaxNode {
    fn from(n: GritName) -> Self {
        n.syntax
    }
}
impl From<GritName> for SyntaxElement {
    fn from(n: GritName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritNamedArg {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_NAMED_ARG as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_NAMED_ARG
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritNamedArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritNamedArg")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .finish()
        } else {
            f.debug_struct("GritNamedArg").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritNamedArg> for SyntaxNode {
    fn from(n: GritNamedArg) -> Self {
        n.syntax
    }
}
impl From<GritNamedArg> for SyntaxElement {
    fn from(n: GritNamedArg) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritNegativeIntLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_NEGATIVE_INT_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_NEGATIVE_INT_LITERAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritNegativeIntLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritNegativeIntLiteral")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritNegativeIntLiteral").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritNegativeIntLiteral> for SyntaxNode {
    fn from(n: GritNegativeIntLiteral) -> Self {
        n.syntax
    }
}
impl From<GritNegativeIntLiteral> for SyntaxElement {
    fn from(n: GritNegativeIntLiteral) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritNodeLike {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_NODE_LIKE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_NODE_LIKE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritNodeLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritNodeLike")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("named_args", &self.named_args())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritNodeLike").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritNodeLike> for SyntaxNode {
    fn from(n: GritNodeLike) -> Self {
        n.syntax
    }
}
impl From<GritNodeLike> for SyntaxElement {
    fn from(n: GritNodeLike) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritNot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_NOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_NOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritNot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritNot")
                .field("token", &support::DebugSyntaxResult(self.token()))
                .finish()
        } else {
            f.debug_struct("GritNot").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritNot> for SyntaxNode {
    fn from(n: GritNot) -> Self {
        n.syntax
    }
}
impl From<GritNot> for SyntaxElement {
    fn from(n: GritNot) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternAccumulate {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_ACCUMULATE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_ACCUMULATE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternAccumulate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternAccumulate")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "add_assign_token",
                    &support::DebugSyntaxResult(self.add_assign_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("GritPatternAccumulate").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternAccumulate> for SyntaxNode {
    fn from(n: GritPatternAccumulate) -> Self {
        n.syntax
    }
}
impl From<GritPatternAccumulate> for SyntaxElement {
    fn from(n: GritPatternAccumulate) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternAfter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_AFTER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_AFTER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternAfter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternAfter")
                .field(
                    "after_token",
                    &support::DebugSyntaxResult(self.after_token()),
                )
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .finish()
        } else {
            f.debug_struct("GritPatternAfter").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternAfter> for SyntaxNode {
    fn from(n: GritPatternAfter) -> Self {
        n.syntax
    }
}
impl From<GritPatternAfter> for SyntaxElement {
    fn from(n: GritPatternAfter) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternAnd {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_AND as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_AND
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternAnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternAnd")
                .field("and_token", &support::DebugSyntaxResult(self.and_token()))
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("patterns", &self.patterns())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritPatternAnd").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternAnd> for SyntaxNode {
    fn from(n: GritPatternAnd) -> Self {
        n.syntax
    }
}
impl From<GritPatternAnd> for SyntaxElement {
    fn from(n: GritPatternAnd) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternAny {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_ANY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_ANY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternAny {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternAny")
                .field("any_token", &support::DebugSyntaxResult(self.any_token()))
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("patterns", &self.patterns())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritPatternAny").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternAny> for SyntaxNode {
    fn from(n: GritPatternAny) -> Self {
        n.syntax
    }
}
impl From<GritPatternAny> for SyntaxElement {
    fn from(n: GritPatternAny) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternAs {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_AS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_AS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternAs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternAs")
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .field("as_token", &support::DebugSyntaxResult(self.as_token()))
                .field("variable", &support::DebugSyntaxResult(self.variable()))
                .finish()
        } else {
            f.debug_struct("GritPatternAs").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternAs> for SyntaxNode {
    fn from(n: GritPatternAs) -> Self {
        n.syntax
    }
}
impl From<GritPatternAs> for SyntaxElement {
    fn from(n: GritPatternAs) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternBefore {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_BEFORE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_BEFORE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternBefore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternBefore")
                .field(
                    "before_token",
                    &support::DebugSyntaxResult(self.before_token()),
                )
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .finish()
        } else {
            f.debug_struct("GritPatternBefore").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternBefore> for SyntaxNode {
    fn from(n: GritPatternBefore) -> Self {
        n.syntax
    }
}
impl From<GritPatternBefore> for SyntaxElement {
    fn from(n: GritPatternBefore) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternContains {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_CONTAINS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_CONTAINS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternContains {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternContains")
                .field(
                    "contains_token",
                    &support::DebugSyntaxResult(self.contains_token()),
                )
                .field("contains", &support::DebugSyntaxResult(self.contains()))
                .field(
                    "until_clause",
                    &support::DebugOptionalElement(self.until_clause()),
                )
                .finish()
        } else {
            f.debug_struct("GritPatternContains").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternContains> for SyntaxNode {
    fn from(n: GritPatternContains) -> Self {
        n.syntax
    }
}
impl From<GritPatternContains> for SyntaxElement {
    fn from(n: GritPatternContains) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternDefinition")
                .field(
                    "visibility_token",
                    &support::DebugOptionalElement(self.visibility_token()),
                )
                .field(
                    "pattern_token",
                    &support::DebugSyntaxResult(self.pattern_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("args", &self.args())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field("language", &support::DebugOptionalElement(self.language()))
                .field("body", &support::DebugSyntaxResult(self.body()))
                .finish()
        } else {
            f.debug_struct("GritPatternDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternDefinition> for SyntaxNode {
    fn from(n: GritPatternDefinition) -> Self {
        n.syntax
    }
}
impl From<GritPatternDefinition> for SyntaxElement {
    fn from(n: GritPatternDefinition) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternDefinitionBody {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_DEFINITION_BODY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_DEFINITION_BODY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternDefinitionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternDefinitionBody")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("patterns", &self.patterns())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritPatternDefinitionBody").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternDefinitionBody> for SyntaxNode {
    fn from(n: GritPatternDefinitionBody) -> Self {
        n.syntax
    }
}
impl From<GritPatternDefinitionBody> for SyntaxElement {
    fn from(n: GritPatternDefinitionBody) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternElseClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_ELSE_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_ELSE_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternElseClause")
                .field("else_token", &support::DebugSyntaxResult(self.else_token()))
                .field(
                    "else_pattern",
                    &support::DebugSyntaxResult(self.else_pattern()),
                )
                .finish()
        } else {
            f.debug_struct("GritPatternElseClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternElseClause> for SyntaxNode {
    fn from(n: GritPatternElseClause) -> Self {
        n.syntax
    }
}
impl From<GritPatternElseClause> for SyntaxElement {
    fn from(n: GritPatternElseClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternIfElse {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_IF_ELSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_IF_ELSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternIfElse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternIfElse")
                .field("if_token", &support::DebugSyntaxResult(self.if_token()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field(
                    "if_predicate",
                    &support::DebugSyntaxResult(self.if_predicate()),
                )
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field(
                    "then_pattern",
                    &support::DebugSyntaxResult(self.then_pattern()),
                )
                .field(
                    "else_clause",
                    &support::DebugOptionalElement(self.else_clause()),
                )
                .finish()
        } else {
            f.debug_struct("GritPatternIfElse").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternIfElse> for SyntaxNode {
    fn from(n: GritPatternIfElse) -> Self {
        n.syntax
    }
}
impl From<GritPatternIfElse> for SyntaxElement {
    fn from(n: GritPatternIfElse) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternIncludes {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_INCLUDES as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_INCLUDES
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternIncludes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternIncludes")
                .field(
                    "includes_token",
                    &support::DebugSyntaxResult(self.includes_token()),
                )
                .field("includes", &support::DebugSyntaxResult(self.includes()))
                .finish()
        } else {
            f.debug_struct("GritPatternIncludes").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternIncludes> for SyntaxNode {
    fn from(n: GritPatternIncludes) -> Self {
        n.syntax
    }
}
impl From<GritPatternIncludes> for SyntaxElement {
    fn from(n: GritPatternIncludes) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternLimit {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_LIMIT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_LIMIT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternLimit")
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .field(
                    "limit_token",
                    &support::DebugSyntaxResult(self.limit_token()),
                )
                .field("limit", &support::DebugSyntaxResult(self.limit()))
                .finish()
        } else {
            f.debug_struct("GritPatternLimit").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternLimit> for SyntaxNode {
    fn from(n: GritPatternLimit) -> Self {
        n.syntax
    }
}
impl From<GritPatternLimit> for SyntaxElement {
    fn from(n: GritPatternLimit) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternMaybe {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_MAYBE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_MAYBE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternMaybe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternMaybe")
                .field(
                    "maybe_token",
                    &support::DebugSyntaxResult(self.maybe_token()),
                )
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .finish()
        } else {
            f.debug_struct("GritPatternMaybe").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternMaybe> for SyntaxNode {
    fn from(n: GritPatternMaybe) -> Self {
        n.syntax
    }
}
impl From<GritPatternMaybe> for SyntaxElement {
    fn from(n: GritPatternMaybe) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternNot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_NOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_NOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternNot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternNot")
                .field("not", &support::DebugSyntaxResult(self.not()))
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .finish()
        } else {
            f.debug_struct("GritPatternNot").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternNot> for SyntaxNode {
    fn from(n: GritPatternNot) -> Self {
        n.syntax
    }
}
impl From<GritPatternNot> for SyntaxElement {
    fn from(n: GritPatternNot) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternOr {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_OR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_OR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternOr")
                .field("or_token", &support::DebugSyntaxResult(self.or_token()))
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("patterns", &self.patterns())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritPatternOr").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternOr> for SyntaxNode {
    fn from(n: GritPatternOr) -> Self {
        n.syntax
    }
}
impl From<GritPatternOr> for SyntaxElement {
    fn from(n: GritPatternOr) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternOrElse {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_OR_ELSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_OR_ELSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternOrElse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternOrElse")
                .field(
                    "orelse_token",
                    &support::DebugSyntaxResult(self.orelse_token()),
                )
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("patterns", &self.patterns())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritPatternOrElse").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternOrElse> for SyntaxNode {
    fn from(n: GritPatternOrElse) -> Self {
        n.syntax
    }
}
impl From<GritPatternOrElse> for SyntaxElement {
    fn from(n: GritPatternOrElse) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternUntilClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_UNTIL_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_UNTIL_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternUntilClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternUntilClause")
                .field(
                    "until_token",
                    &support::DebugSyntaxResult(self.until_token()),
                )
                .field("until", &support::DebugSyntaxResult(self.until()))
                .finish()
        } else {
            f.debug_struct("GritPatternUntilClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternUntilClause> for SyntaxNode {
    fn from(n: GritPatternUntilClause) -> Self {
        n.syntax
    }
}
impl From<GritPatternUntilClause> for SyntaxElement {
    fn from(n: GritPatternUntilClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPatternWhere {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_WHERE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_WHERE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternWhere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPatternWhere")
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .field(
                    "where_token",
                    &support::DebugSyntaxResult(self.where_token()),
                )
                .field(
                    "side_condition",
                    &support::DebugSyntaxResult(self.side_condition()),
                )
                .finish()
        } else {
            f.debug_struct("GritPatternWhere").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPatternWhere> for SyntaxNode {
    fn from(n: GritPatternWhere) -> Self {
        n.syntax
    }
}
impl From<GritPatternWhere> for SyntaxElement {
    fn from(n: GritPatternWhere) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateAccumulate {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_ACCUMULATE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_ACCUMULATE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateAccumulate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateAccumulate")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "add_assign_token",
                    &support::DebugSyntaxResult(self.add_assign_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("GritPredicateAccumulate").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateAccumulate> for SyntaxNode {
    fn from(n: GritPredicateAccumulate) -> Self {
        n.syntax
    }
}
impl From<GritPredicateAccumulate> for SyntaxElement {
    fn from(n: GritPredicateAccumulate) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateAnd {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_AND as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_AND
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateAnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateAnd")
                .field(
                    "and_token",
                    &support::DebugOptionalElement(self.and_token()),
                )
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("predicates", &self.predicates())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritPredicateAnd").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateAnd> for SyntaxNode {
    fn from(n: GritPredicateAnd) -> Self {
        n.syntax
    }
}
impl From<GritPredicateAnd> for SyntaxElement {
    fn from(n: GritPredicateAnd) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateAny {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_ANY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_ANY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateAny {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateAny")
                .field("any_token", &support::DebugSyntaxResult(self.any_token()))
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("predicates", &self.predicates())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritPredicateAny").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateAny> for SyntaxNode {
    fn from(n: GritPredicateAny) -> Self {
        n.syntax
    }
}
impl From<GritPredicateAny> for SyntaxElement {
    fn from(n: GritPredicateAny) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateAssignment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_ASSIGNMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_ASSIGNMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateAssignment")
                .field("container", &support::DebugSyntaxResult(self.container()))
                .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .finish()
        } else {
            f.debug_struct("GritPredicateAssignment").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateAssignment> for SyntaxNode {
    fn from(n: GritPredicateAssignment) -> Self {
        n.syntax
    }
}
impl From<GritPredicateAssignment> for SyntaxElement {
    fn from(n: GritPredicateAssignment) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateCall {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_CALL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_CALL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateCall")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("named_args", &self.named_args())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritPredicateCall").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateCall> for SyntaxNode {
    fn from(n: GritPredicateCall) -> Self {
        n.syntax
    }
}
impl From<GritPredicateCall> for SyntaxElement {
    fn from(n: GritPredicateCall) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateCurly {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_CURLY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_CURLY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateCurly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateCurly")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("predicates", &self.predicates())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritPredicateCurly").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateCurly> for SyntaxNode {
    fn from(n: GritPredicateCurly) -> Self {
        n.syntax
    }
}
impl From<GritPredicateCurly> for SyntaxElement {
    fn from(n: GritPredicateCurly) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateDefinition")
                .field(
                    "predicate_token",
                    &support::DebugSyntaxResult(self.predicate_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("args", &self.args())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field("body", &support::DebugSyntaxResult(self.body()))
                .finish()
        } else {
            f.debug_struct("GritPredicateDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateDefinition> for SyntaxNode {
    fn from(n: GritPredicateDefinition) -> Self {
        n.syntax
    }
}
impl From<GritPredicateDefinition> for SyntaxElement {
    fn from(n: GritPredicateDefinition) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateElseClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_ELSE_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_ELSE_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateElseClause")
                .field("else_token", &support::DebugSyntaxResult(self.else_token()))
                .field(
                    "else_predicate",
                    &support::DebugSyntaxResult(self.else_predicate()),
                )
                .finish()
        } else {
            f.debug_struct("GritPredicateElseClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateElseClause> for SyntaxNode {
    fn from(n: GritPredicateElseClause) -> Self {
        n.syntax
    }
}
impl From<GritPredicateElseClause> for SyntaxElement {
    fn from(n: GritPredicateElseClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateEqual {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_EQUAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_EQUAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateEqual")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "equality_token",
                    &support::DebugSyntaxResult(self.equality_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("GritPredicateEqual").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateEqual> for SyntaxNode {
    fn from(n: GritPredicateEqual) -> Self {
        n.syntax
    }
}
impl From<GritPredicateEqual> for SyntaxElement {
    fn from(n: GritPredicateEqual) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateGreater {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_GREATER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_GREATER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateGreater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateGreater")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "r_angle_token",
                    &support::DebugSyntaxResult(self.r_angle_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("GritPredicateGreater").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateGreater> for SyntaxNode {
    fn from(n: GritPredicateGreater) -> Self {
        n.syntax
    }
}
impl From<GritPredicateGreater> for SyntaxElement {
    fn from(n: GritPredicateGreater) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateGreaterEqual {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_GREATER_EQUAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_GREATER_EQUAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateGreaterEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateGreaterEqual")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "greater_than_equal_token",
                    &support::DebugSyntaxResult(self.greater_than_equal_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("GritPredicateGreaterEqual").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateGreaterEqual> for SyntaxNode {
    fn from(n: GritPredicateGreaterEqual) -> Self {
        n.syntax
    }
}
impl From<GritPredicateGreaterEqual> for SyntaxElement {
    fn from(n: GritPredicateGreaterEqual) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateIfElse {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_IF_ELSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_IF_ELSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateIfElse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateIfElse")
                .field("if_token", &support::DebugSyntaxResult(self.if_token()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field(
                    "if_predicate",
                    &support::DebugSyntaxResult(self.if_predicate()),
                )
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field(
                    "then_predicate",
                    &support::DebugSyntaxResult(self.then_predicate()),
                )
                .field(
                    "else_clause",
                    &support::DebugOptionalElement(self.else_clause()),
                )
                .finish()
        } else {
            f.debug_struct("GritPredicateIfElse").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateIfElse> for SyntaxNode {
    fn from(n: GritPredicateIfElse) -> Self {
        n.syntax
    }
}
impl From<GritPredicateIfElse> for SyntaxElement {
    fn from(n: GritPredicateIfElse) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateLess {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_LESS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_LESS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateLess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateLess")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "l_angle_token",
                    &support::DebugSyntaxResult(self.l_angle_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("GritPredicateLess").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateLess> for SyntaxNode {
    fn from(n: GritPredicateLess) -> Self {
        n.syntax
    }
}
impl From<GritPredicateLess> for SyntaxElement {
    fn from(n: GritPredicateLess) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateLessEqual {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_LESS_EQUAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_LESS_EQUAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateLessEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateLessEqual")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "less_than_equal_token",
                    &support::DebugSyntaxResult(self.less_than_equal_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("GritPredicateLessEqual").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateLessEqual> for SyntaxNode {
    fn from(n: GritPredicateLessEqual) -> Self {
        n.syntax
    }
}
impl From<GritPredicateLessEqual> for SyntaxElement {
    fn from(n: GritPredicateLessEqual) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateMatch {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_MATCH as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_MATCH
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateMatch")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "match_token",
                    &support::DebugSyntaxResult(self.match_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("GritPredicateMatch").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateMatch> for SyntaxNode {
    fn from(n: GritPredicateMatch) -> Self {
        n.syntax
    }
}
impl From<GritPredicateMatch> for SyntaxElement {
    fn from(n: GritPredicateMatch) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateMaybe {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_MAYBE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_MAYBE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateMaybe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateMaybe")
                .field(
                    "maybe_token",
                    &support::DebugSyntaxResult(self.maybe_token()),
                )
                .field("predicate", &support::DebugSyntaxResult(self.predicate()))
                .finish()
        } else {
            f.debug_struct("GritPredicateMaybe").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateMaybe> for SyntaxNode {
    fn from(n: GritPredicateMaybe) -> Self {
        n.syntax
    }
}
impl From<GritPredicateMaybe> for SyntaxElement {
    fn from(n: GritPredicateMaybe) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateNot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_NOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_NOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateNot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateNot")
                .field("not", &support::DebugSyntaxResult(self.not()))
                .field("predicate", &support::DebugSyntaxResult(self.predicate()))
                .finish()
        } else {
            f.debug_struct("GritPredicateNot").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateNot> for SyntaxNode {
    fn from(n: GritPredicateNot) -> Self {
        n.syntax
    }
}
impl From<GritPredicateNot> for SyntaxElement {
    fn from(n: GritPredicateNot) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateNotEqual {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_NOT_EQUAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_NOT_EQUAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateNotEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateNotEqual")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "inequality_token",
                    &support::DebugSyntaxResult(self.inequality_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("GritPredicateNotEqual").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateNotEqual> for SyntaxNode {
    fn from(n: GritPredicateNotEqual) -> Self {
        n.syntax
    }
}
impl From<GritPredicateNotEqual> for SyntaxElement {
    fn from(n: GritPredicateNotEqual) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateOr {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_OR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_OR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateOr")
                .field("or_token", &support::DebugSyntaxResult(self.or_token()))
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("predicates", &self.predicates())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritPredicateOr").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateOr> for SyntaxNode {
    fn from(n: GritPredicateOr) -> Self {
        n.syntax
    }
}
impl From<GritPredicateOr> for SyntaxElement {
    fn from(n: GritPredicateOr) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateReturn {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_RETURN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_RETURN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateReturn")
                .field(
                    "return_token",
                    &support::DebugSyntaxResult(self.return_token()),
                )
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .finish()
        } else {
            f.debug_struct("GritPredicateReturn").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateReturn> for SyntaxNode {
    fn from(n: GritPredicateReturn) -> Self {
        n.syntax
    }
}
impl From<GritPredicateReturn> for SyntaxElement {
    fn from(n: GritPredicateReturn) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateRewrite {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_REWRITE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_REWRITE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateRewrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritPredicateRewrite")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "annotation",
                    &support::DebugOptionalElement(self.annotation()),
                )
                .field(
                    "fat_arrow_token",
                    &support::DebugSyntaxResult(self.fat_arrow_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("GritPredicateRewrite").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritPredicateRewrite> for SyntaxNode {
    fn from(n: GritPredicateRewrite) -> Self {
        n.syntax
    }
}
impl From<GritPredicateRewrite> for SyntaxElement {
    fn from(n: GritPredicateRewrite) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritRawBacktickSnippetLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_RAW_BACKTICK_SNIPPET_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_RAW_BACKTICK_SNIPPET_LITERAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritRawBacktickSnippetLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritRawBacktickSnippetLiteral")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritRawBacktickSnippetLiteral").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritRawBacktickSnippetLiteral> for SyntaxNode {
    fn from(n: GritRawBacktickSnippetLiteral) -> Self {
        n.syntax
    }
}
impl From<GritRawBacktickSnippetLiteral> for SyntaxElement {
    fn from(n: GritRawBacktickSnippetLiteral) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritRegexLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_REGEX_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_REGEX_LITERAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritRegexLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritRegexLiteral")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritRegexLiteral").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritRegexLiteral> for SyntaxNode {
    fn from(n: GritRegexLiteral) -> Self {
        n.syntax
    }
}
impl From<GritRegexLiteral> for SyntaxElement {
    fn from(n: GritRegexLiteral) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritRegexPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_REGEX_PATTERN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_REGEX_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritRegexPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritRegexPattern")
                .field("regex", &support::DebugSyntaxResult(self.regex()))
                .field(
                    "variables",
                    &support::DebugOptionalElement(self.variables()),
                )
                .finish()
        } else {
            f.debug_struct("GritRegexPattern").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritRegexPattern> for SyntaxNode {
    fn from(n: GritRegexPattern) -> Self {
        n.syntax
    }
}
impl From<GritRegexPattern> for SyntaxElement {
    fn from(n: GritRegexPattern) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritRegexPatternVariables {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_REGEX_PATTERN_VARIABLES as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_REGEX_PATTERN_VARIABLES
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritRegexPatternVariables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritRegexPatternVariables")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("args", &self.args())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritRegexPatternVariables").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritRegexPatternVariables> for SyntaxNode {
    fn from(n: GritRegexPatternVariables) -> Self {
        n.syntax
    }
}
impl From<GritRegexPatternVariables> for SyntaxElement {
    fn from(n: GritRegexPatternVariables) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritRewrite {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_REWRITE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_REWRITE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritRewrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritRewrite")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "annotation",
                    &support::DebugOptionalElement(self.annotation()),
                )
                .field(
                    "fat_arrow_token",
                    &support::DebugSyntaxResult(self.fat_arrow_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("GritRewrite").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritRewrite> for SyntaxNode {
    fn from(n: GritRewrite) -> Self {
        n.syntax
    }
}
impl From<GritRewrite> for SyntaxElement {
    fn from(n: GritRewrite) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_ROOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritRoot")
                .field(
                    "bom_token",
                    &support::DebugOptionalElement(self.bom_token()),
                )
                .field("version", &support::DebugOptionalElement(self.version()))
                .field("language", &support::DebugOptionalElement(self.language()))
                .field("definitions", &self.definitions())
                .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
                .finish()
        } else {
            f.debug_struct("GritRoot").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritRoot> for SyntaxNode {
    fn from(n: GritRoot) -> Self {
        n.syntax
    }
}
impl From<GritRoot> for SyntaxElement {
    fn from(n: GritRoot) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritSequential {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_SEQUENTIAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_SEQUENTIAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritSequential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritSequential")
                .field(
                    "sequential_token",
                    &support::DebugSyntaxResult(self.sequential_token()),
                )
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("sequential", &self.sequential())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritSequential").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritSequential> for SyntaxNode {
    fn from(n: GritSequential) -> Self {
        n.syntax
    }
}
impl From<GritSequential> for SyntaxElement {
    fn from(n: GritSequential) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritSnippetRegexLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_SNIPPET_REGEX_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_SNIPPET_REGEX_LITERAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritSnippetRegexLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritSnippetRegexLiteral")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritSnippetRegexLiteral").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritSnippetRegexLiteral> for SyntaxNode {
    fn from(n: GritSnippetRegexLiteral) -> Self {
        n.syntax
    }
}
impl From<GritSnippetRegexLiteral> for SyntaxElement {
    fn from(n: GritSnippetRegexLiteral) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritSome {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_SOME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_SOME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritSome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritSome")
                .field("some_token", &support::DebugSyntaxResult(self.some_token()))
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .finish()
        } else {
            f.debug_struct("GritSome").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritSome> for SyntaxNode {
    fn from(n: GritSome) -> Self {
        n.syntax
    }
}
impl From<GritSome> for SyntaxElement {
    fn from(n: GritSome) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritStringLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_STRING_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_STRING_LITERAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritStringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritStringLiteral")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritStringLiteral").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritStringLiteral> for SyntaxNode {
    fn from(n: GritStringLiteral) -> Self {
        n.syntax
    }
}
impl From<GritStringLiteral> for SyntaxElement {
    fn from(n: GritStringLiteral) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritSubOperation {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_SUB_OPERATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_SUB_OPERATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritSubOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritSubOperation")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "minus_token",
                    &support::DebugSyntaxResult(self.minus_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("GritSubOperation").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritSubOperation> for SyntaxNode {
    fn from(n: GritSubOperation) -> Self {
        n.syntax
    }
}
impl From<GritSubOperation> for SyntaxElement {
    fn from(n: GritSubOperation) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritUndefinedLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_UNDEFINED_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_UNDEFINED_LITERAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritUndefinedLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritUndefinedLiteral")
                .field(
                    "token_token",
                    &support::DebugSyntaxResult(self.token_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritUndefinedLiteral").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritUndefinedLiteral> for SyntaxNode {
    fn from(n: GritUndefinedLiteral) -> Self {
        n.syntax
    }
}
impl From<GritUndefinedLiteral> for SyntaxElement {
    fn from(n: GritUndefinedLiteral) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritUnderscore {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_UNDERSCORE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_UNDERSCORE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritUnderscore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritUnderscore")
                .field(
                    "token_token",
                    &support::DebugSyntaxResult(self.token_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritUnderscore").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritUnderscore> for SyntaxNode {
    fn from(n: GritUnderscore) -> Self {
        n.syntax
    }
}
impl From<GritUnderscore> for SyntaxElement {
    fn from(n: GritUnderscore) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritVariable {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_VARIABLE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_VARIABLE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritVariable")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritVariable").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritVariable> for SyntaxNode {
    fn from(n: GritVariable) -> Self {
        n.syntax
    }
}
impl From<GritVariable> for SyntaxElement {
    fn from(n: GritVariable) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritVersion {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_VERSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_VERSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritVersion")
                .field(
                    "engine_token",
                    &support::DebugSyntaxResult(self.engine_token()),
                )
                .field(
                    "engine_name",
                    &support::DebugSyntaxResult(self.engine_name()),
                )
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("version", &support::DebugSyntaxResult(self.version()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("GritVersion").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritVersion> for SyntaxNode {
    fn from(n: GritVersion) -> Self {
        n.syntax
    }
}
impl From<GritVersion> for SyntaxElement {
    fn from(n: GritVersion) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GritWithin {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_WITHIN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_WITHIN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritWithin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GritWithin")
                .field(
                    "within_token",
                    &support::DebugSyntaxResult(self.within_token()),
                )
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .field(
                    "until_clause",
                    &support::DebugOptionalElement(self.until_clause()),
                )
                .finish()
        } else {
            f.debug_struct("GritWithin").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GritWithin> for SyntaxNode {
    fn from(n: GritWithin) -> Self {
        n.syntax
    }
}
impl From<GritWithin> for SyntaxElement {
    fn from(n: GritWithin) -> Self {
        n.syntax.into()
    }
}
impl From<GritBacktickSnippetLiteral> for AnyGritCodeSnippetSource {
    fn from(node: GritBacktickSnippetLiteral) -> Self {
        Self::GritBacktickSnippetLiteral(node)
    }
}
impl From<GritLanguageSpecificSnippet> for AnyGritCodeSnippetSource {
    fn from(node: GritLanguageSpecificSnippet) -> Self {
        Self::GritLanguageSpecificSnippet(node)
    }
}
impl From<GritRawBacktickSnippetLiteral> for AnyGritCodeSnippetSource {
    fn from(node: GritRawBacktickSnippetLiteral) -> Self {
        Self::GritRawBacktickSnippetLiteral(node)
    }
}
impl AstNode for AnyGritCodeSnippetSource {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GritBacktickSnippetLiteral::KIND_SET
        .union(GritLanguageSpecificSnippet::KIND_SET)
        .union(GritRawBacktickSnippetLiteral::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            GRIT_BACKTICK_SNIPPET_LITERAL
                | GRIT_LANGUAGE_SPECIFIC_SNIPPET
                | GRIT_RAW_BACKTICK_SNIPPET_LITERAL
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_BACKTICK_SNIPPET_LITERAL => {
                Self::GritBacktickSnippetLiteral(GritBacktickSnippetLiteral { syntax })
            }
            GRIT_LANGUAGE_SPECIFIC_SNIPPET => {
                Self::GritLanguageSpecificSnippet(GritLanguageSpecificSnippet { syntax })
            }
            GRIT_RAW_BACKTICK_SNIPPET_LITERAL => {
                Self::GritRawBacktickSnippetLiteral(GritRawBacktickSnippetLiteral { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritBacktickSnippetLiteral(it) => &it.syntax,
            Self::GritLanguageSpecificSnippet(it) => &it.syntax,
            Self::GritRawBacktickSnippetLiteral(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritBacktickSnippetLiteral(it) => it.syntax,
            Self::GritLanguageSpecificSnippet(it) => it.syntax,
            Self::GritRawBacktickSnippetLiteral(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGritCodeSnippetSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GritBacktickSnippetLiteral(it) => std::fmt::Debug::fmt(it, f),
            Self::GritLanguageSpecificSnippet(it) => std::fmt::Debug::fmt(it, f),
            Self::GritRawBacktickSnippetLiteral(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritCodeSnippetSource> for SyntaxNode {
    fn from(n: AnyGritCodeSnippetSource) -> Self {
        match n {
            AnyGritCodeSnippetSource::GritBacktickSnippetLiteral(it) => it.into(),
            AnyGritCodeSnippetSource::GritLanguageSpecificSnippet(it) => it.into(),
            AnyGritCodeSnippetSource::GritRawBacktickSnippetLiteral(it) => it.into(),
        }
    }
}
impl From<AnyGritCodeSnippetSource> for SyntaxElement {
    fn from(n: AnyGritCodeSnippetSource) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritBogusContainer> for AnyGritContainer {
    fn from(node: GritBogusContainer) -> Self {
        Self::GritBogusContainer(node)
    }
}
impl From<GritListAccessor> for AnyGritContainer {
    fn from(node: GritListAccessor) -> Self {
        Self::GritListAccessor(node)
    }
}
impl From<GritMapAccessor> for AnyGritContainer {
    fn from(node: GritMapAccessor) -> Self {
        Self::GritMapAccessor(node)
    }
}
impl From<GritVariable> for AnyGritContainer {
    fn from(node: GritVariable) -> Self {
        Self::GritVariable(node)
    }
}
impl AstNode for AnyGritContainer {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GritBogusContainer::KIND_SET
        .union(GritListAccessor::KIND_SET)
        .union(GritMapAccessor::KIND_SET)
        .union(GritVariable::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            GRIT_BOGUS_CONTAINER | GRIT_LIST_ACCESSOR | GRIT_MAP_ACCESSOR | GRIT_VARIABLE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_BOGUS_CONTAINER => Self::GritBogusContainer(GritBogusContainer { syntax }),
            GRIT_LIST_ACCESSOR => Self::GritListAccessor(GritListAccessor { syntax }),
            GRIT_MAP_ACCESSOR => Self::GritMapAccessor(GritMapAccessor { syntax }),
            GRIT_VARIABLE => Self::GritVariable(GritVariable { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritBogusContainer(it) => &it.syntax,
            Self::GritListAccessor(it) => &it.syntax,
            Self::GritMapAccessor(it) => &it.syntax,
            Self::GritVariable(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritBogusContainer(it) => it.syntax,
            Self::GritListAccessor(it) => it.syntax,
            Self::GritMapAccessor(it) => it.syntax,
            Self::GritVariable(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGritContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GritBogusContainer(it) => std::fmt::Debug::fmt(it, f),
            Self::GritListAccessor(it) => std::fmt::Debug::fmt(it, f),
            Self::GritMapAccessor(it) => std::fmt::Debug::fmt(it, f),
            Self::GritVariable(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritContainer> for SyntaxNode {
    fn from(n: AnyGritContainer) -> Self {
        match n {
            AnyGritContainer::GritBogusContainer(it) => it.into(),
            AnyGritContainer::GritListAccessor(it) => it.into(),
            AnyGritContainer::GritMapAccessor(it) => it.into(),
            AnyGritContainer::GritVariable(it) => it.into(),
        }
    }
}
impl From<AnyGritContainer> for SyntaxElement {
    fn from(n: AnyGritContainer) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritBogusDefinition> for AnyGritDefinition {
    fn from(node: GritBogusDefinition) -> Self {
        Self::GritBogusDefinition(node)
    }
}
impl From<GritFunctionDefinition> for AnyGritDefinition {
    fn from(node: GritFunctionDefinition) -> Self {
        Self::GritFunctionDefinition(node)
    }
}
impl From<GritJavascriptFunctionDefinition> for AnyGritDefinition {
    fn from(node: GritJavascriptFunctionDefinition) -> Self {
        Self::GritJavascriptFunctionDefinition(node)
    }
}
impl From<GritPatternDefinition> for AnyGritDefinition {
    fn from(node: GritPatternDefinition) -> Self {
        Self::GritPatternDefinition(node)
    }
}
impl From<GritPredicateDefinition> for AnyGritDefinition {
    fn from(node: GritPredicateDefinition) -> Self {
        Self::GritPredicateDefinition(node)
    }
}
impl AstNode for AnyGritDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyGritPattern::KIND_SET
        .union(GritBogusDefinition::KIND_SET)
        .union(GritFunctionDefinition::KIND_SET)
        .union(GritJavascriptFunctionDefinition::KIND_SET)
        .union(GritPatternDefinition::KIND_SET)
        .union(GritPredicateDefinition::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            GRIT_BOGUS_DEFINITION
            | GRIT_FUNCTION_DEFINITION
            | GRIT_JAVASCRIPT_FUNCTION_DEFINITION
            | GRIT_PATTERN_DEFINITION
            | GRIT_PREDICATE_DEFINITION => true,
            k if AnyGritPattern::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_BOGUS_DEFINITION => Self::GritBogusDefinition(GritBogusDefinition { syntax }),
            GRIT_FUNCTION_DEFINITION => {
                Self::GritFunctionDefinition(GritFunctionDefinition { syntax })
            }
            GRIT_JAVASCRIPT_FUNCTION_DEFINITION => {
                Self::GritJavascriptFunctionDefinition(GritJavascriptFunctionDefinition { syntax })
            }
            GRIT_PATTERN_DEFINITION => {
                Self::GritPatternDefinition(GritPatternDefinition { syntax })
            }
            GRIT_PREDICATE_DEFINITION => {
                Self::GritPredicateDefinition(GritPredicateDefinition { syntax })
            }
            _ => {
                if let Some(any_grit_pattern) = AnyGritPattern::cast(syntax) {
                    return Some(Self::AnyGritPattern(any_grit_pattern));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritBogusDefinition(it) => &it.syntax,
            Self::GritFunctionDefinition(it) => &it.syntax,
            Self::GritJavascriptFunctionDefinition(it) => &it.syntax,
            Self::GritPatternDefinition(it) => &it.syntax,
            Self::GritPredicateDefinition(it) => &it.syntax,
            Self::AnyGritPattern(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritBogusDefinition(it) => it.syntax,
            Self::GritFunctionDefinition(it) => it.syntax,
            Self::GritJavascriptFunctionDefinition(it) => it.syntax,
            Self::GritPatternDefinition(it) => it.syntax,
            Self::GritPredicateDefinition(it) => it.syntax,
            Self::AnyGritPattern(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyGritDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyGritPattern(it) => std::fmt::Debug::fmt(it, f),
            Self::GritBogusDefinition(it) => std::fmt::Debug::fmt(it, f),
            Self::GritFunctionDefinition(it) => std::fmt::Debug::fmt(it, f),
            Self::GritJavascriptFunctionDefinition(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPatternDefinition(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateDefinition(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritDefinition> for SyntaxNode {
    fn from(n: AnyGritDefinition) -> Self {
        match n {
            AnyGritDefinition::AnyGritPattern(it) => it.into(),
            AnyGritDefinition::GritBogusDefinition(it) => it.into(),
            AnyGritDefinition::GritFunctionDefinition(it) => it.into(),
            AnyGritDefinition::GritJavascriptFunctionDefinition(it) => it.into(),
            AnyGritDefinition::GritPatternDefinition(it) => it.into(),
            AnyGritDefinition::GritPredicateDefinition(it) => it.into(),
        }
    }
}
impl From<AnyGritDefinition> for SyntaxElement {
    fn from(n: AnyGritDefinition) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritBogusLanguageDeclaration> for AnyGritLanguageDeclaration {
    fn from(node: GritBogusLanguageDeclaration) -> Self {
        Self::GritBogusLanguageDeclaration(node)
    }
}
impl From<GritLanguageDeclaration> for AnyGritLanguageDeclaration {
    fn from(node: GritLanguageDeclaration) -> Self {
        Self::GritLanguageDeclaration(node)
    }
}
impl AstNode for AnyGritLanguageDeclaration {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        GritBogusLanguageDeclaration::KIND_SET.union(GritLanguageDeclaration::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            GRIT_BOGUS_LANGUAGE_DECLARATION | GRIT_LANGUAGE_DECLARATION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_BOGUS_LANGUAGE_DECLARATION => {
                Self::GritBogusLanguageDeclaration(GritBogusLanguageDeclaration { syntax })
            }
            GRIT_LANGUAGE_DECLARATION => {
                Self::GritLanguageDeclaration(GritLanguageDeclaration { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritBogusLanguageDeclaration(it) => &it.syntax,
            Self::GritLanguageDeclaration(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritBogusLanguageDeclaration(it) => it.syntax,
            Self::GritLanguageDeclaration(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGritLanguageDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GritBogusLanguageDeclaration(it) => std::fmt::Debug::fmt(it, f),
            Self::GritLanguageDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritLanguageDeclaration> for SyntaxNode {
    fn from(n: AnyGritLanguageDeclaration) -> Self {
        match n {
            AnyGritLanguageDeclaration::GritBogusLanguageDeclaration(it) => it.into(),
            AnyGritLanguageDeclaration::GritLanguageDeclaration(it) => it.into(),
        }
    }
}
impl From<AnyGritLanguageDeclaration> for SyntaxElement {
    fn from(n: AnyGritLanguageDeclaration) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritBogusLanguageFlavorKind> for AnyGritLanguageFlavorKind {
    fn from(node: GritBogusLanguageFlavorKind) -> Self {
        Self::GritBogusLanguageFlavorKind(node)
    }
}
impl From<GritLanguageFlavorKind> for AnyGritLanguageFlavorKind {
    fn from(node: GritLanguageFlavorKind) -> Self {
        Self::GritLanguageFlavorKind(node)
    }
}
impl AstNode for AnyGritLanguageFlavorKind {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        GritBogusLanguageFlavorKind::KIND_SET.union(GritLanguageFlavorKind::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            GRIT_BOGUS_LANGUAGE_FLAVOR_KIND | GRIT_LANGUAGE_FLAVOR_KIND
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_BOGUS_LANGUAGE_FLAVOR_KIND => {
                Self::GritBogusLanguageFlavorKind(GritBogusLanguageFlavorKind { syntax })
            }
            GRIT_LANGUAGE_FLAVOR_KIND => {
                Self::GritLanguageFlavorKind(GritLanguageFlavorKind { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritBogusLanguageFlavorKind(it) => &it.syntax,
            Self::GritLanguageFlavorKind(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritBogusLanguageFlavorKind(it) => it.syntax,
            Self::GritLanguageFlavorKind(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGritLanguageFlavorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GritBogusLanguageFlavorKind(it) => std::fmt::Debug::fmt(it, f),
            Self::GritLanguageFlavorKind(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritLanguageFlavorKind> for SyntaxNode {
    fn from(n: AnyGritLanguageFlavorKind) -> Self {
        match n {
            AnyGritLanguageFlavorKind::GritBogusLanguageFlavorKind(it) => it.into(),
            AnyGritLanguageFlavorKind::GritLanguageFlavorKind(it) => it.into(),
        }
    }
}
impl From<AnyGritLanguageFlavorKind> for SyntaxElement {
    fn from(n: AnyGritLanguageFlavorKind) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritBogusLanguageName> for AnyGritLanguageName {
    fn from(node: GritBogusLanguageName) -> Self {
        Self::GritBogusLanguageName(node)
    }
}
impl From<GritLanguageName> for AnyGritLanguageName {
    fn from(node: GritLanguageName) -> Self {
        Self::GritLanguageName(node)
    }
}
impl AstNode for AnyGritLanguageName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        GritBogusLanguageName::KIND_SET.union(GritLanguageName::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, GRIT_BOGUS_LANGUAGE_NAME | GRIT_LANGUAGE_NAME)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_BOGUS_LANGUAGE_NAME => {
                Self::GritBogusLanguageName(GritBogusLanguageName { syntax })
            }
            GRIT_LANGUAGE_NAME => Self::GritLanguageName(GritLanguageName { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritBogusLanguageName(it) => &it.syntax,
            Self::GritLanguageName(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritBogusLanguageName(it) => it.syntax,
            Self::GritLanguageName(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGritLanguageName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GritBogusLanguageName(it) => std::fmt::Debug::fmt(it, f),
            Self::GritLanguageName(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritLanguageName> for SyntaxNode {
    fn from(n: AnyGritLanguageName) -> Self {
        match n {
            AnyGritLanguageName::GritBogusLanguageName(it) => it.into(),
            AnyGritLanguageName::GritLanguageName(it) => it.into(),
        }
    }
}
impl From<AnyGritLanguageName> for SyntaxElement {
    fn from(n: AnyGritLanguageName) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritList> for AnyGritListAccessorSubject {
    fn from(node: GritList) -> Self {
        Self::GritList(node)
    }
}
impl AstNode for AnyGritListAccessorSubject {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyGritContainer::KIND_SET.union(GritList::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            GRIT_LIST => true,
            k if AnyGritContainer::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_LIST => Self::GritList(GritList { syntax }),
            _ => {
                if let Some(any_grit_container) = AnyGritContainer::cast(syntax) {
                    return Some(Self::AnyGritContainer(any_grit_container));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritList(it) => &it.syntax,
            Self::AnyGritContainer(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritList(it) => it.syntax,
            Self::AnyGritContainer(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyGritListAccessorSubject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyGritContainer(it) => std::fmt::Debug::fmt(it, f),
            Self::GritList(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritListAccessorSubject> for SyntaxNode {
    fn from(n: AnyGritListAccessorSubject) -> Self {
        match n {
            AnyGritListAccessorSubject::AnyGritContainer(it) => it.into(),
            AnyGritListAccessorSubject::GritList(it) => it.into(),
        }
    }
}
impl From<AnyGritListAccessorSubject> for SyntaxElement {
    fn from(n: AnyGritListAccessorSubject) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritIntLiteral> for AnyGritListIndex {
    fn from(node: GritIntLiteral) -> Self {
        Self::GritIntLiteral(node)
    }
}
impl From<GritNegativeIntLiteral> for AnyGritListIndex {
    fn from(node: GritNegativeIntLiteral) -> Self {
        Self::GritNegativeIntLiteral(node)
    }
}
impl AstNode for AnyGritListIndex {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyGritContainer::KIND_SET
        .union(GritIntLiteral::KIND_SET)
        .union(GritNegativeIntLiteral::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            GRIT_INT_LITERAL | GRIT_NEGATIVE_INT_LITERAL => true,
            k if AnyGritContainer::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_INT_LITERAL => Self::GritIntLiteral(GritIntLiteral { syntax }),
            GRIT_NEGATIVE_INT_LITERAL => {
                Self::GritNegativeIntLiteral(GritNegativeIntLiteral { syntax })
            }
            _ => {
                if let Some(any_grit_container) = AnyGritContainer::cast(syntax) {
                    return Some(Self::AnyGritContainer(any_grit_container));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritIntLiteral(it) => &it.syntax,
            Self::GritNegativeIntLiteral(it) => &it.syntax,
            Self::AnyGritContainer(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritIntLiteral(it) => it.syntax,
            Self::GritNegativeIntLiteral(it) => it.syntax,
            Self::AnyGritContainer(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyGritListIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyGritContainer(it) => std::fmt::Debug::fmt(it, f),
            Self::GritIntLiteral(it) => std::fmt::Debug::fmt(it, f),
            Self::GritNegativeIntLiteral(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritListIndex> for SyntaxNode {
    fn from(n: AnyGritListIndex) -> Self {
        match n {
            AnyGritListIndex::AnyGritContainer(it) => it.into(),
            AnyGritListIndex::GritIntLiteral(it) => it.into(),
            AnyGritListIndex::GritNegativeIntLiteral(it) => it.into(),
        }
    }
}
impl From<AnyGritListIndex> for SyntaxElement {
    fn from(n: AnyGritListIndex) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritDotdotdot> for AnyGritListPattern {
    fn from(node: GritDotdotdot) -> Self {
        Self::GritDotdotdot(node)
    }
}
impl AstNode for AnyGritListPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyGritPattern::KIND_SET.union(GritDotdotdot::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            GRIT_DOTDOTDOT => true,
            k if AnyGritPattern::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_DOTDOTDOT => Self::GritDotdotdot(GritDotdotdot { syntax }),
            _ => {
                if let Some(any_grit_pattern) = AnyGritPattern::cast(syntax) {
                    return Some(Self::AnyGritPattern(any_grit_pattern));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritDotdotdot(it) => &it.syntax,
            Self::AnyGritPattern(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritDotdotdot(it) => it.syntax,
            Self::AnyGritPattern(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyGritListPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyGritPattern(it) => std::fmt::Debug::fmt(it, f),
            Self::GritDotdotdot(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritListPattern> for SyntaxNode {
    fn from(n: AnyGritListPattern) -> Self {
        match n {
            AnyGritListPattern::AnyGritPattern(it) => it.into(),
            AnyGritListPattern::GritDotdotdot(it) => it.into(),
        }
    }
}
impl From<AnyGritListPattern> for SyntaxElement {
    fn from(n: AnyGritListPattern) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritBogusLiteral> for AnyGritLiteral {
    fn from(node: GritBogusLiteral) -> Self {
        Self::GritBogusLiteral(node)
    }
}
impl From<GritBooleanLiteral> for AnyGritLiteral {
    fn from(node: GritBooleanLiteral) -> Self {
        Self::GritBooleanLiteral(node)
    }
}
impl From<GritCodeSnippet> for AnyGritLiteral {
    fn from(node: GritCodeSnippet) -> Self {
        Self::GritCodeSnippet(node)
    }
}
impl From<GritDoubleLiteral> for AnyGritLiteral {
    fn from(node: GritDoubleLiteral) -> Self {
        Self::GritDoubleLiteral(node)
    }
}
impl From<GritIntLiteral> for AnyGritLiteral {
    fn from(node: GritIntLiteral) -> Self {
        Self::GritIntLiteral(node)
    }
}
impl From<GritList> for AnyGritLiteral {
    fn from(node: GritList) -> Self {
        Self::GritList(node)
    }
}
impl From<GritMap> for AnyGritLiteral {
    fn from(node: GritMap) -> Self {
        Self::GritMap(node)
    }
}
impl From<GritStringLiteral> for AnyGritLiteral {
    fn from(node: GritStringLiteral) -> Self {
        Self::GritStringLiteral(node)
    }
}
impl From<GritUndefinedLiteral> for AnyGritLiteral {
    fn from(node: GritUndefinedLiteral) -> Self {
        Self::GritUndefinedLiteral(node)
    }
}
impl AstNode for AnyGritLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GritBogusLiteral::KIND_SET
        .union(GritBooleanLiteral::KIND_SET)
        .union(GritCodeSnippet::KIND_SET)
        .union(GritDoubleLiteral::KIND_SET)
        .union(GritIntLiteral::KIND_SET)
        .union(GritList::KIND_SET)
        .union(GritMap::KIND_SET)
        .union(GritStringLiteral::KIND_SET)
        .union(GritUndefinedLiteral::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            GRIT_BOGUS_LITERAL
                | GRIT_BOOLEAN_LITERAL
                | GRIT_CODE_SNIPPET
                | GRIT_DOUBLE_LITERAL
                | GRIT_INT_LITERAL
                | GRIT_LIST
                | GRIT_MAP
                | GRIT_STRING_LITERAL
                | GRIT_UNDEFINED_LITERAL
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_BOGUS_LITERAL => Self::GritBogusLiteral(GritBogusLiteral { syntax }),
            GRIT_BOOLEAN_LITERAL => Self::GritBooleanLiteral(GritBooleanLiteral { syntax }),
            GRIT_CODE_SNIPPET => Self::GritCodeSnippet(GritCodeSnippet { syntax }),
            GRIT_DOUBLE_LITERAL => Self::GritDoubleLiteral(GritDoubleLiteral { syntax }),
            GRIT_INT_LITERAL => Self::GritIntLiteral(GritIntLiteral { syntax }),
            GRIT_LIST => Self::GritList(GritList { syntax }),
            GRIT_MAP => Self::GritMap(GritMap { syntax }),
            GRIT_STRING_LITERAL => Self::GritStringLiteral(GritStringLiteral { syntax }),
            GRIT_UNDEFINED_LITERAL => Self::GritUndefinedLiteral(GritUndefinedLiteral { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritBogusLiteral(it) => &it.syntax,
            Self::GritBooleanLiteral(it) => &it.syntax,
            Self::GritCodeSnippet(it) => &it.syntax,
            Self::GritDoubleLiteral(it) => &it.syntax,
            Self::GritIntLiteral(it) => &it.syntax,
            Self::GritList(it) => &it.syntax,
            Self::GritMap(it) => &it.syntax,
            Self::GritStringLiteral(it) => &it.syntax,
            Self::GritUndefinedLiteral(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritBogusLiteral(it) => it.syntax,
            Self::GritBooleanLiteral(it) => it.syntax,
            Self::GritCodeSnippet(it) => it.syntax,
            Self::GritDoubleLiteral(it) => it.syntax,
            Self::GritIntLiteral(it) => it.syntax,
            Self::GritList(it) => it.syntax,
            Self::GritMap(it) => it.syntax,
            Self::GritStringLiteral(it) => it.syntax,
            Self::GritUndefinedLiteral(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGritLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GritBogusLiteral(it) => std::fmt::Debug::fmt(it, f),
            Self::GritBooleanLiteral(it) => std::fmt::Debug::fmt(it, f),
            Self::GritCodeSnippet(it) => std::fmt::Debug::fmt(it, f),
            Self::GritDoubleLiteral(it) => std::fmt::Debug::fmt(it, f),
            Self::GritIntLiteral(it) => std::fmt::Debug::fmt(it, f),
            Self::GritList(it) => std::fmt::Debug::fmt(it, f),
            Self::GritMap(it) => std::fmt::Debug::fmt(it, f),
            Self::GritStringLiteral(it) => std::fmt::Debug::fmt(it, f),
            Self::GritUndefinedLiteral(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritLiteral> for SyntaxNode {
    fn from(n: AnyGritLiteral) -> Self {
        match n {
            AnyGritLiteral::GritBogusLiteral(it) => it.into(),
            AnyGritLiteral::GritBooleanLiteral(it) => it.into(),
            AnyGritLiteral::GritCodeSnippet(it) => it.into(),
            AnyGritLiteral::GritDoubleLiteral(it) => it.into(),
            AnyGritLiteral::GritIntLiteral(it) => it.into(),
            AnyGritLiteral::GritList(it) => it.into(),
            AnyGritLiteral::GritMap(it) => it.into(),
            AnyGritLiteral::GritStringLiteral(it) => it.into(),
            AnyGritLiteral::GritUndefinedLiteral(it) => it.into(),
        }
    }
}
impl From<AnyGritLiteral> for SyntaxElement {
    fn from(n: AnyGritLiteral) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritMap> for AnyGritMapAccessorSubject {
    fn from(node: GritMap) -> Self {
        Self::GritMap(node)
    }
}
impl AstNode for AnyGritMapAccessorSubject {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyGritContainer::KIND_SET.union(GritMap::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            GRIT_MAP => true,
            k if AnyGritContainer::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_MAP => Self::GritMap(GritMap { syntax }),
            _ => {
                if let Some(any_grit_container) = AnyGritContainer::cast(syntax) {
                    return Some(Self::AnyGritContainer(any_grit_container));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritMap(it) => &it.syntax,
            Self::AnyGritContainer(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritMap(it) => it.syntax,
            Self::AnyGritContainer(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyGritMapAccessorSubject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyGritContainer(it) => std::fmt::Debug::fmt(it, f),
            Self::GritMap(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritMapAccessorSubject> for SyntaxNode {
    fn from(n: AnyGritMapAccessorSubject) -> Self {
        match n {
            AnyGritMapAccessorSubject::AnyGritContainer(it) => it.into(),
            AnyGritMapAccessorSubject::GritMap(it) => it.into(),
        }
    }
}
impl From<AnyGritMapAccessorSubject> for SyntaxElement {
    fn from(n: AnyGritMapAccessorSubject) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritBogusMapElement> for AnyGritMapElement {
    fn from(node: GritBogusMapElement) -> Self {
        Self::GritBogusMapElement(node)
    }
}
impl From<GritMapElement> for AnyGritMapElement {
    fn from(node: GritMapElement) -> Self {
        Self::GritMapElement(node)
    }
}
impl AstNode for AnyGritMapElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        GritBogusMapElement::KIND_SET.union(GritMapElement::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, GRIT_BOGUS_MAP_ELEMENT | GRIT_MAP_ELEMENT)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_BOGUS_MAP_ELEMENT => Self::GritBogusMapElement(GritBogusMapElement { syntax }),
            GRIT_MAP_ELEMENT => Self::GritMapElement(GritMapElement { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritBogusMapElement(it) => &it.syntax,
            Self::GritMapElement(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritBogusMapElement(it) => it.syntax,
            Self::GritMapElement(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGritMapElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GritBogusMapElement(it) => std::fmt::Debug::fmt(it, f),
            Self::GritMapElement(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritMapElement> for SyntaxNode {
    fn from(n: AnyGritMapElement) -> Self {
        match n {
            AnyGritMapElement::GritBogusMapElement(it) => it.into(),
            AnyGritMapElement::GritMapElement(it) => it.into(),
        }
    }
}
impl From<AnyGritMapElement> for SyntaxElement {
    fn from(n: AnyGritMapElement) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritName> for AnyGritMapKey {
    fn from(node: GritName) -> Self {
        Self::GritName(node)
    }
}
impl From<GritVariable> for AnyGritMapKey {
    fn from(node: GritVariable) -> Self {
        Self::GritVariable(node)
    }
}
impl AstNode for AnyGritMapKey {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GritName::KIND_SET.union(GritVariable::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, GRIT_NAME | GRIT_VARIABLE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_NAME => Self::GritName(GritName { syntax }),
            GRIT_VARIABLE => Self::GritVariable(GritVariable { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritName(it) => &it.syntax,
            Self::GritVariable(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritName(it) => it.syntax,
            Self::GritVariable(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGritMapKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GritName(it) => std::fmt::Debug::fmt(it, f),
            Self::GritVariable(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritMapKey> for SyntaxNode {
    fn from(n: AnyGritMapKey) -> Self {
        match n {
            AnyGritMapKey::GritName(it) => it.into(),
            AnyGritMapKey::GritVariable(it) => it.into(),
        }
    }
}
impl From<AnyGritMapKey> for SyntaxElement {
    fn from(n: AnyGritMapKey) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritCurlyPattern> for AnyGritMaybeCurlyPattern {
    fn from(node: GritCurlyPattern) -> Self {
        Self::GritCurlyPattern(node)
    }
}
impl AstNode for AnyGritMaybeCurlyPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyGritPattern::KIND_SET.union(GritCurlyPattern::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            GRIT_CURLY_PATTERN => true,
            k if AnyGritPattern::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_CURLY_PATTERN => Self::GritCurlyPattern(GritCurlyPattern { syntax }),
            _ => {
                if let Some(any_grit_pattern) = AnyGritPattern::cast(syntax) {
                    return Some(Self::AnyGritPattern(any_grit_pattern));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritCurlyPattern(it) => &it.syntax,
            Self::AnyGritPattern(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritCurlyPattern(it) => it.syntax,
            Self::AnyGritPattern(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyGritMaybeCurlyPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyGritPattern(it) => std::fmt::Debug::fmt(it, f),
            Self::GritCurlyPattern(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritMaybeCurlyPattern> for SyntaxNode {
    fn from(n: AnyGritMaybeCurlyPattern) -> Self {
        match n {
            AnyGritMaybeCurlyPattern::AnyGritPattern(it) => it.into(),
            AnyGritMaybeCurlyPattern::GritCurlyPattern(it) => it.into(),
        }
    }
}
impl From<AnyGritMaybeCurlyPattern> for SyntaxElement {
    fn from(n: AnyGritMaybeCurlyPattern) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritBogusNamedArg> for AnyGritMaybeNamedArg {
    fn from(node: GritBogusNamedArg) -> Self {
        Self::GritBogusNamedArg(node)
    }
}
impl From<GritNamedArg> for AnyGritMaybeNamedArg {
    fn from(node: GritNamedArg) -> Self {
        Self::GritNamedArg(node)
    }
}
impl AstNode for AnyGritMaybeNamedArg {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyGritPattern::KIND_SET
        .union(GritBogusNamedArg::KIND_SET)
        .union(GritNamedArg::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            GRIT_BOGUS_NAMED_ARG | GRIT_NAMED_ARG => true,
            k if AnyGritPattern::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_BOGUS_NAMED_ARG => Self::GritBogusNamedArg(GritBogusNamedArg { syntax }),
            GRIT_NAMED_ARG => Self::GritNamedArg(GritNamedArg { syntax }),
            _ => {
                if let Some(any_grit_pattern) = AnyGritPattern::cast(syntax) {
                    return Some(Self::AnyGritPattern(any_grit_pattern));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritBogusNamedArg(it) => &it.syntax,
            Self::GritNamedArg(it) => &it.syntax,
            Self::AnyGritPattern(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritBogusNamedArg(it) => it.syntax,
            Self::GritNamedArg(it) => it.syntax,
            Self::AnyGritPattern(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyGritMaybeNamedArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyGritPattern(it) => std::fmt::Debug::fmt(it, f),
            Self::GritBogusNamedArg(it) => std::fmt::Debug::fmt(it, f),
            Self::GritNamedArg(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritMaybeNamedArg> for SyntaxNode {
    fn from(n: AnyGritMaybeNamedArg) -> Self {
        match n {
            AnyGritMaybeNamedArg::AnyGritPattern(it) => it.into(),
            AnyGritMaybeNamedArg::GritBogusNamedArg(it) => it.into(),
            AnyGritMaybeNamedArg::GritNamedArg(it) => it.into(),
        }
    }
}
impl From<AnyGritMaybeNamedArg> for SyntaxElement {
    fn from(n: AnyGritMaybeNamedArg) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritAddOperation> for AnyGritPattern {
    fn from(node: GritAddOperation) -> Self {
        Self::GritAddOperation(node)
    }
}
impl From<GritAssignmentAsPattern> for AnyGritPattern {
    fn from(node: GritAssignmentAsPattern) -> Self {
        Self::GritAssignmentAsPattern(node)
    }
}
impl From<GritBogusPattern> for AnyGritPattern {
    fn from(node: GritBogusPattern) -> Self {
        Self::GritBogusPattern(node)
    }
}
impl From<GritBracketedPattern> for AnyGritPattern {
    fn from(node: GritBracketedPattern) -> Self {
        Self::GritBracketedPattern(node)
    }
}
impl From<GritBubble> for AnyGritPattern {
    fn from(node: GritBubble) -> Self {
        Self::GritBubble(node)
    }
}
impl From<GritDivOperation> for AnyGritPattern {
    fn from(node: GritDivOperation) -> Self {
        Self::GritDivOperation(node)
    }
}
impl From<GritDot> for AnyGritPattern {
    fn from(node: GritDot) -> Self {
        Self::GritDot(node)
    }
}
impl From<GritEvery> for AnyGritPattern {
    fn from(node: GritEvery) -> Self {
        Self::GritEvery(node)
    }
}
impl From<GritFiles> for AnyGritPattern {
    fn from(node: GritFiles) -> Self {
        Self::GritFiles(node)
    }
}
impl From<GritLike> for AnyGritPattern {
    fn from(node: GritLike) -> Self {
        Self::GritLike(node)
    }
}
impl From<GritListAccessor> for AnyGritPattern {
    fn from(node: GritListAccessor) -> Self {
        Self::GritListAccessor(node)
    }
}
impl From<GritMapAccessor> for AnyGritPattern {
    fn from(node: GritMapAccessor) -> Self {
        Self::GritMapAccessor(node)
    }
}
impl From<GritModOperation> for AnyGritPattern {
    fn from(node: GritModOperation) -> Self {
        Self::GritModOperation(node)
    }
}
impl From<GritMulOperation> for AnyGritPattern {
    fn from(node: GritMulOperation) -> Self {
        Self::GritMulOperation(node)
    }
}
impl From<GritNodeLike> for AnyGritPattern {
    fn from(node: GritNodeLike) -> Self {
        Self::GritNodeLike(node)
    }
}
impl From<GritPatternAccumulate> for AnyGritPattern {
    fn from(node: GritPatternAccumulate) -> Self {
        Self::GritPatternAccumulate(node)
    }
}
impl From<GritPatternAfter> for AnyGritPattern {
    fn from(node: GritPatternAfter) -> Self {
        Self::GritPatternAfter(node)
    }
}
impl From<GritPatternAnd> for AnyGritPattern {
    fn from(node: GritPatternAnd) -> Self {
        Self::GritPatternAnd(node)
    }
}
impl From<GritPatternAny> for AnyGritPattern {
    fn from(node: GritPatternAny) -> Self {
        Self::GritPatternAny(node)
    }
}
impl From<GritPatternAs> for AnyGritPattern {
    fn from(node: GritPatternAs) -> Self {
        Self::GritPatternAs(node)
    }
}
impl From<GritPatternBefore> for AnyGritPattern {
    fn from(node: GritPatternBefore) -> Self {
        Self::GritPatternBefore(node)
    }
}
impl From<GritPatternContains> for AnyGritPattern {
    fn from(node: GritPatternContains) -> Self {
        Self::GritPatternContains(node)
    }
}
impl From<GritPatternIfElse> for AnyGritPattern {
    fn from(node: GritPatternIfElse) -> Self {
        Self::GritPatternIfElse(node)
    }
}
impl From<GritPatternIncludes> for AnyGritPattern {
    fn from(node: GritPatternIncludes) -> Self {
        Self::GritPatternIncludes(node)
    }
}
impl From<GritPatternLimit> for AnyGritPattern {
    fn from(node: GritPatternLimit) -> Self {
        Self::GritPatternLimit(node)
    }
}
impl From<GritPatternMaybe> for AnyGritPattern {
    fn from(node: GritPatternMaybe) -> Self {
        Self::GritPatternMaybe(node)
    }
}
impl From<GritPatternNot> for AnyGritPattern {
    fn from(node: GritPatternNot) -> Self {
        Self::GritPatternNot(node)
    }
}
impl From<GritPatternOr> for AnyGritPattern {
    fn from(node: GritPatternOr) -> Self {
        Self::GritPatternOr(node)
    }
}
impl From<GritPatternOrElse> for AnyGritPattern {
    fn from(node: GritPatternOrElse) -> Self {
        Self::GritPatternOrElse(node)
    }
}
impl From<GritPatternWhere> for AnyGritPattern {
    fn from(node: GritPatternWhere) -> Self {
        Self::GritPatternWhere(node)
    }
}
impl From<GritRegexPattern> for AnyGritPattern {
    fn from(node: GritRegexPattern) -> Self {
        Self::GritRegexPattern(node)
    }
}
impl From<GritRewrite> for AnyGritPattern {
    fn from(node: GritRewrite) -> Self {
        Self::GritRewrite(node)
    }
}
impl From<GritSequential> for AnyGritPattern {
    fn from(node: GritSequential) -> Self {
        Self::GritSequential(node)
    }
}
impl From<GritSome> for AnyGritPattern {
    fn from(node: GritSome) -> Self {
        Self::GritSome(node)
    }
}
impl From<GritSubOperation> for AnyGritPattern {
    fn from(node: GritSubOperation) -> Self {
        Self::GritSubOperation(node)
    }
}
impl From<GritUnderscore> for AnyGritPattern {
    fn from(node: GritUnderscore) -> Self {
        Self::GritUnderscore(node)
    }
}
impl From<GritVariable> for AnyGritPattern {
    fn from(node: GritVariable) -> Self {
        Self::GritVariable(node)
    }
}
impl From<GritWithin> for AnyGritPattern {
    fn from(node: GritWithin) -> Self {
        Self::GritWithin(node)
    }
}
impl AstNode for AnyGritPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyGritLiteral::KIND_SET
        .union(GritAddOperation::KIND_SET)
        .union(GritAssignmentAsPattern::KIND_SET)
        .union(GritBogusPattern::KIND_SET)
        .union(GritBracketedPattern::KIND_SET)
        .union(GritBubble::KIND_SET)
        .union(GritDivOperation::KIND_SET)
        .union(GritDot::KIND_SET)
        .union(GritEvery::KIND_SET)
        .union(GritFiles::KIND_SET)
        .union(GritLike::KIND_SET)
        .union(GritListAccessor::KIND_SET)
        .union(GritMapAccessor::KIND_SET)
        .union(GritModOperation::KIND_SET)
        .union(GritMulOperation::KIND_SET)
        .union(GritNodeLike::KIND_SET)
        .union(GritPatternAccumulate::KIND_SET)
        .union(GritPatternAfter::KIND_SET)
        .union(GritPatternAnd::KIND_SET)
        .union(GritPatternAny::KIND_SET)
        .union(GritPatternAs::KIND_SET)
        .union(GritPatternBefore::KIND_SET)
        .union(GritPatternContains::KIND_SET)
        .union(GritPatternIfElse::KIND_SET)
        .union(GritPatternIncludes::KIND_SET)
        .union(GritPatternLimit::KIND_SET)
        .union(GritPatternMaybe::KIND_SET)
        .union(GritPatternNot::KIND_SET)
        .union(GritPatternOr::KIND_SET)
        .union(GritPatternOrElse::KIND_SET)
        .union(GritPatternWhere::KIND_SET)
        .union(GritRegexPattern::KIND_SET)
        .union(GritRewrite::KIND_SET)
        .union(GritSequential::KIND_SET)
        .union(GritSome::KIND_SET)
        .union(GritSubOperation::KIND_SET)
        .union(GritUnderscore::KIND_SET)
        .union(GritVariable::KIND_SET)
        .union(GritWithin::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            GRIT_ADD_OPERATION
            | GRIT_ASSIGNMENT_AS_PATTERN
            | GRIT_BOGUS_PATTERN
            | GRIT_BRACKETED_PATTERN
            | GRIT_BUBBLE
            | GRIT_DIV_OPERATION
            | GRIT_DOT
            | GRIT_EVERY
            | GRIT_FILES
            | GRIT_LIKE
            | GRIT_LIST_ACCESSOR
            | GRIT_MAP_ACCESSOR
            | GRIT_MOD_OPERATION
            | GRIT_MUL_OPERATION
            | GRIT_NODE_LIKE
            | GRIT_PATTERN_ACCUMULATE
            | GRIT_PATTERN_AFTER
            | GRIT_PATTERN_AND
            | GRIT_PATTERN_ANY
            | GRIT_PATTERN_AS
            | GRIT_PATTERN_BEFORE
            | GRIT_PATTERN_CONTAINS
            | GRIT_PATTERN_IF_ELSE
            | GRIT_PATTERN_INCLUDES
            | GRIT_PATTERN_LIMIT
            | GRIT_PATTERN_MAYBE
            | GRIT_PATTERN_NOT
            | GRIT_PATTERN_OR
            | GRIT_PATTERN_OR_ELSE
            | GRIT_PATTERN_WHERE
            | GRIT_REGEX_PATTERN
            | GRIT_REWRITE
            | GRIT_SEQUENTIAL
            | GRIT_SOME
            | GRIT_SUB_OPERATION
            | GRIT_UNDERSCORE
            | GRIT_VARIABLE
            | GRIT_WITHIN => true,
            k if AnyGritLiteral::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_ADD_OPERATION => Self::GritAddOperation(GritAddOperation { syntax }),
            GRIT_ASSIGNMENT_AS_PATTERN => {
                Self::GritAssignmentAsPattern(GritAssignmentAsPattern { syntax })
            }
            GRIT_BOGUS_PATTERN => Self::GritBogusPattern(GritBogusPattern { syntax }),
            GRIT_BRACKETED_PATTERN => Self::GritBracketedPattern(GritBracketedPattern { syntax }),
            GRIT_BUBBLE => Self::GritBubble(GritBubble { syntax }),
            GRIT_DIV_OPERATION => Self::GritDivOperation(GritDivOperation { syntax }),
            GRIT_DOT => Self::GritDot(GritDot { syntax }),
            GRIT_EVERY => Self::GritEvery(GritEvery { syntax }),
            GRIT_FILES => Self::GritFiles(GritFiles { syntax }),
            GRIT_LIKE => Self::GritLike(GritLike { syntax }),
            GRIT_LIST_ACCESSOR => Self::GritListAccessor(GritListAccessor { syntax }),
            GRIT_MAP_ACCESSOR => Self::GritMapAccessor(GritMapAccessor { syntax }),
            GRIT_MOD_OPERATION => Self::GritModOperation(GritModOperation { syntax }),
            GRIT_MUL_OPERATION => Self::GritMulOperation(GritMulOperation { syntax }),
            GRIT_NODE_LIKE => Self::GritNodeLike(GritNodeLike { syntax }),
            GRIT_PATTERN_ACCUMULATE => {
                Self::GritPatternAccumulate(GritPatternAccumulate { syntax })
            }
            GRIT_PATTERN_AFTER => Self::GritPatternAfter(GritPatternAfter { syntax }),
            GRIT_PATTERN_AND => Self::GritPatternAnd(GritPatternAnd { syntax }),
            GRIT_PATTERN_ANY => Self::GritPatternAny(GritPatternAny { syntax }),
            GRIT_PATTERN_AS => Self::GritPatternAs(GritPatternAs { syntax }),
            GRIT_PATTERN_BEFORE => Self::GritPatternBefore(GritPatternBefore { syntax }),
            GRIT_PATTERN_CONTAINS => Self::GritPatternContains(GritPatternContains { syntax }),
            GRIT_PATTERN_IF_ELSE => Self::GritPatternIfElse(GritPatternIfElse { syntax }),
            GRIT_PATTERN_INCLUDES => Self::GritPatternIncludes(GritPatternIncludes { syntax }),
            GRIT_PATTERN_LIMIT => Self::GritPatternLimit(GritPatternLimit { syntax }),
            GRIT_PATTERN_MAYBE => Self::GritPatternMaybe(GritPatternMaybe { syntax }),
            GRIT_PATTERN_NOT => Self::GritPatternNot(GritPatternNot { syntax }),
            GRIT_PATTERN_OR => Self::GritPatternOr(GritPatternOr { syntax }),
            GRIT_PATTERN_OR_ELSE => Self::GritPatternOrElse(GritPatternOrElse { syntax }),
            GRIT_PATTERN_WHERE => Self::GritPatternWhere(GritPatternWhere { syntax }),
            GRIT_REGEX_PATTERN => Self::GritRegexPattern(GritRegexPattern { syntax }),
            GRIT_REWRITE => Self::GritRewrite(GritRewrite { syntax }),
            GRIT_SEQUENTIAL => Self::GritSequential(GritSequential { syntax }),
            GRIT_SOME => Self::GritSome(GritSome { syntax }),
            GRIT_SUB_OPERATION => Self::GritSubOperation(GritSubOperation { syntax }),
            GRIT_UNDERSCORE => Self::GritUnderscore(GritUnderscore { syntax }),
            GRIT_VARIABLE => Self::GritVariable(GritVariable { syntax }),
            GRIT_WITHIN => Self::GritWithin(GritWithin { syntax }),
            _ => {
                if let Some(any_grit_literal) = AnyGritLiteral::cast(syntax) {
                    return Some(Self::AnyGritLiteral(any_grit_literal));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritAddOperation(it) => &it.syntax,
            Self::GritAssignmentAsPattern(it) => &it.syntax,
            Self::GritBogusPattern(it) => &it.syntax,
            Self::GritBracketedPattern(it) => &it.syntax,
            Self::GritBubble(it) => &it.syntax,
            Self::GritDivOperation(it) => &it.syntax,
            Self::GritDot(it) => &it.syntax,
            Self::GritEvery(it) => &it.syntax,
            Self::GritFiles(it) => &it.syntax,
            Self::GritLike(it) => &it.syntax,
            Self::GritListAccessor(it) => &it.syntax,
            Self::GritMapAccessor(it) => &it.syntax,
            Self::GritModOperation(it) => &it.syntax,
            Self::GritMulOperation(it) => &it.syntax,
            Self::GritNodeLike(it) => &it.syntax,
            Self::GritPatternAccumulate(it) => &it.syntax,
            Self::GritPatternAfter(it) => &it.syntax,
            Self::GritPatternAnd(it) => &it.syntax,
            Self::GritPatternAny(it) => &it.syntax,
            Self::GritPatternAs(it) => &it.syntax,
            Self::GritPatternBefore(it) => &it.syntax,
            Self::GritPatternContains(it) => &it.syntax,
            Self::GritPatternIfElse(it) => &it.syntax,
            Self::GritPatternIncludes(it) => &it.syntax,
            Self::GritPatternLimit(it) => &it.syntax,
            Self::GritPatternMaybe(it) => &it.syntax,
            Self::GritPatternNot(it) => &it.syntax,
            Self::GritPatternOr(it) => &it.syntax,
            Self::GritPatternOrElse(it) => &it.syntax,
            Self::GritPatternWhere(it) => &it.syntax,
            Self::GritRegexPattern(it) => &it.syntax,
            Self::GritRewrite(it) => &it.syntax,
            Self::GritSequential(it) => &it.syntax,
            Self::GritSome(it) => &it.syntax,
            Self::GritSubOperation(it) => &it.syntax,
            Self::GritUnderscore(it) => &it.syntax,
            Self::GritVariable(it) => &it.syntax,
            Self::GritWithin(it) => &it.syntax,
            Self::AnyGritLiteral(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritAddOperation(it) => it.syntax,
            Self::GritAssignmentAsPattern(it) => it.syntax,
            Self::GritBogusPattern(it) => it.syntax,
            Self::GritBracketedPattern(it) => it.syntax,
            Self::GritBubble(it) => it.syntax,
            Self::GritDivOperation(it) => it.syntax,
            Self::GritDot(it) => it.syntax,
            Self::GritEvery(it) => it.syntax,
            Self::GritFiles(it) => it.syntax,
            Self::GritLike(it) => it.syntax,
            Self::GritListAccessor(it) => it.syntax,
            Self::GritMapAccessor(it) => it.syntax,
            Self::GritModOperation(it) => it.syntax,
            Self::GritMulOperation(it) => it.syntax,
            Self::GritNodeLike(it) => it.syntax,
            Self::GritPatternAccumulate(it) => it.syntax,
            Self::GritPatternAfter(it) => it.syntax,
            Self::GritPatternAnd(it) => it.syntax,
            Self::GritPatternAny(it) => it.syntax,
            Self::GritPatternAs(it) => it.syntax,
            Self::GritPatternBefore(it) => it.syntax,
            Self::GritPatternContains(it) => it.syntax,
            Self::GritPatternIfElse(it) => it.syntax,
            Self::GritPatternIncludes(it) => it.syntax,
            Self::GritPatternLimit(it) => it.syntax,
            Self::GritPatternMaybe(it) => it.syntax,
            Self::GritPatternNot(it) => it.syntax,
            Self::GritPatternOr(it) => it.syntax,
            Self::GritPatternOrElse(it) => it.syntax,
            Self::GritPatternWhere(it) => it.syntax,
            Self::GritRegexPattern(it) => it.syntax,
            Self::GritRewrite(it) => it.syntax,
            Self::GritSequential(it) => it.syntax,
            Self::GritSome(it) => it.syntax,
            Self::GritSubOperation(it) => it.syntax,
            Self::GritUnderscore(it) => it.syntax,
            Self::GritVariable(it) => it.syntax,
            Self::GritWithin(it) => it.syntax,
            Self::AnyGritLiteral(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyGritPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyGritLiteral(it) => std::fmt::Debug::fmt(it, f),
            Self::GritAddOperation(it) => std::fmt::Debug::fmt(it, f),
            Self::GritAssignmentAsPattern(it) => std::fmt::Debug::fmt(it, f),
            Self::GritBogusPattern(it) => std::fmt::Debug::fmt(it, f),
            Self::GritBracketedPattern(it) => std::fmt::Debug::fmt(it, f),
            Self::GritBubble(it) => std::fmt::Debug::fmt(it, f),
            Self::GritDivOperation(it) => std::fmt::Debug::fmt(it, f),
            Self::GritDot(it) => std::fmt::Debug::fmt(it, f),
            Self::GritEvery(it) => std::fmt::Debug::fmt(it, f),
            Self::GritFiles(it) => std::fmt::Debug::fmt(it, f),
            Self::GritLike(it) => std::fmt::Debug::fmt(it, f),
            Self::GritListAccessor(it) => std::fmt::Debug::fmt(it, f),
            Self::GritMapAccessor(it) => std::fmt::Debug::fmt(it, f),
            Self::GritModOperation(it) => std::fmt::Debug::fmt(it, f),
            Self::GritMulOperation(it) => std::fmt::Debug::fmt(it, f),
            Self::GritNodeLike(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPatternAccumulate(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPatternAfter(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPatternAnd(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPatternAny(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPatternAs(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPatternBefore(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPatternContains(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPatternIfElse(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPatternIncludes(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPatternLimit(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPatternMaybe(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPatternNot(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPatternOr(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPatternOrElse(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPatternWhere(it) => std::fmt::Debug::fmt(it, f),
            Self::GritRegexPattern(it) => std::fmt::Debug::fmt(it, f),
            Self::GritRewrite(it) => std::fmt::Debug::fmt(it, f),
            Self::GritSequential(it) => std::fmt::Debug::fmt(it, f),
            Self::GritSome(it) => std::fmt::Debug::fmt(it, f),
            Self::GritSubOperation(it) => std::fmt::Debug::fmt(it, f),
            Self::GritUnderscore(it) => std::fmt::Debug::fmt(it, f),
            Self::GritVariable(it) => std::fmt::Debug::fmt(it, f),
            Self::GritWithin(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritPattern> for SyntaxNode {
    fn from(n: AnyGritPattern) -> Self {
        match n {
            AnyGritPattern::AnyGritLiteral(it) => it.into(),
            AnyGritPattern::GritAddOperation(it) => it.into(),
            AnyGritPattern::GritAssignmentAsPattern(it) => it.into(),
            AnyGritPattern::GritBogusPattern(it) => it.into(),
            AnyGritPattern::GritBracketedPattern(it) => it.into(),
            AnyGritPattern::GritBubble(it) => it.into(),
            AnyGritPattern::GritDivOperation(it) => it.into(),
            AnyGritPattern::GritDot(it) => it.into(),
            AnyGritPattern::GritEvery(it) => it.into(),
            AnyGritPattern::GritFiles(it) => it.into(),
            AnyGritPattern::GritLike(it) => it.into(),
            AnyGritPattern::GritListAccessor(it) => it.into(),
            AnyGritPattern::GritMapAccessor(it) => it.into(),
            AnyGritPattern::GritModOperation(it) => it.into(),
            AnyGritPattern::GritMulOperation(it) => it.into(),
            AnyGritPattern::GritNodeLike(it) => it.into(),
            AnyGritPattern::GritPatternAccumulate(it) => it.into(),
            AnyGritPattern::GritPatternAfter(it) => it.into(),
            AnyGritPattern::GritPatternAnd(it) => it.into(),
            AnyGritPattern::GritPatternAny(it) => it.into(),
            AnyGritPattern::GritPatternAs(it) => it.into(),
            AnyGritPattern::GritPatternBefore(it) => it.into(),
            AnyGritPattern::GritPatternContains(it) => it.into(),
            AnyGritPattern::GritPatternIfElse(it) => it.into(),
            AnyGritPattern::GritPatternIncludes(it) => it.into(),
            AnyGritPattern::GritPatternLimit(it) => it.into(),
            AnyGritPattern::GritPatternMaybe(it) => it.into(),
            AnyGritPattern::GritPatternNot(it) => it.into(),
            AnyGritPattern::GritPatternOr(it) => it.into(),
            AnyGritPattern::GritPatternOrElse(it) => it.into(),
            AnyGritPattern::GritPatternWhere(it) => it.into(),
            AnyGritPattern::GritRegexPattern(it) => it.into(),
            AnyGritPattern::GritRewrite(it) => it.into(),
            AnyGritPattern::GritSequential(it) => it.into(),
            AnyGritPattern::GritSome(it) => it.into(),
            AnyGritPattern::GritSubOperation(it) => it.into(),
            AnyGritPattern::GritUnderscore(it) => it.into(),
            AnyGritPattern::GritVariable(it) => it.into(),
            AnyGritPattern::GritWithin(it) => it.into(),
        }
    }
}
impl From<AnyGritPattern> for SyntaxElement {
    fn from(n: AnyGritPattern) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritBogusPredicate> for AnyGritPredicate {
    fn from(node: GritBogusPredicate) -> Self {
        Self::GritBogusPredicate(node)
    }
}
impl From<GritBooleanLiteral> for AnyGritPredicate {
    fn from(node: GritBooleanLiteral) -> Self {
        Self::GritBooleanLiteral(node)
    }
}
impl From<GritBracketedPredicate> for AnyGritPredicate {
    fn from(node: GritBracketedPredicate) -> Self {
        Self::GritBracketedPredicate(node)
    }
}
impl From<GritPredicateAccumulate> for AnyGritPredicate {
    fn from(node: GritPredicateAccumulate) -> Self {
        Self::GritPredicateAccumulate(node)
    }
}
impl From<GritPredicateAnd> for AnyGritPredicate {
    fn from(node: GritPredicateAnd) -> Self {
        Self::GritPredicateAnd(node)
    }
}
impl From<GritPredicateAny> for AnyGritPredicate {
    fn from(node: GritPredicateAny) -> Self {
        Self::GritPredicateAny(node)
    }
}
impl From<GritPredicateAssignment> for AnyGritPredicate {
    fn from(node: GritPredicateAssignment) -> Self {
        Self::GritPredicateAssignment(node)
    }
}
impl From<GritPredicateCall> for AnyGritPredicate {
    fn from(node: GritPredicateCall) -> Self {
        Self::GritPredicateCall(node)
    }
}
impl From<GritPredicateEqual> for AnyGritPredicate {
    fn from(node: GritPredicateEqual) -> Self {
        Self::GritPredicateEqual(node)
    }
}
impl From<GritPredicateGreater> for AnyGritPredicate {
    fn from(node: GritPredicateGreater) -> Self {
        Self::GritPredicateGreater(node)
    }
}
impl From<GritPredicateGreaterEqual> for AnyGritPredicate {
    fn from(node: GritPredicateGreaterEqual) -> Self {
        Self::GritPredicateGreaterEqual(node)
    }
}
impl From<GritPredicateIfElse> for AnyGritPredicate {
    fn from(node: GritPredicateIfElse) -> Self {
        Self::GritPredicateIfElse(node)
    }
}
impl From<GritPredicateLess> for AnyGritPredicate {
    fn from(node: GritPredicateLess) -> Self {
        Self::GritPredicateLess(node)
    }
}
impl From<GritPredicateLessEqual> for AnyGritPredicate {
    fn from(node: GritPredicateLessEqual) -> Self {
        Self::GritPredicateLessEqual(node)
    }
}
impl From<GritPredicateMatch> for AnyGritPredicate {
    fn from(node: GritPredicateMatch) -> Self {
        Self::GritPredicateMatch(node)
    }
}
impl From<GritPredicateMaybe> for AnyGritPredicate {
    fn from(node: GritPredicateMaybe) -> Self {
        Self::GritPredicateMaybe(node)
    }
}
impl From<GritPredicateNot> for AnyGritPredicate {
    fn from(node: GritPredicateNot) -> Self {
        Self::GritPredicateNot(node)
    }
}
impl From<GritPredicateNotEqual> for AnyGritPredicate {
    fn from(node: GritPredicateNotEqual) -> Self {
        Self::GritPredicateNotEqual(node)
    }
}
impl From<GritPredicateOr> for AnyGritPredicate {
    fn from(node: GritPredicateOr) -> Self {
        Self::GritPredicateOr(node)
    }
}
impl From<GritPredicateReturn> for AnyGritPredicate {
    fn from(node: GritPredicateReturn) -> Self {
        Self::GritPredicateReturn(node)
    }
}
impl From<GritPredicateRewrite> for AnyGritPredicate {
    fn from(node: GritPredicateRewrite) -> Self {
        Self::GritPredicateRewrite(node)
    }
}
impl AstNode for AnyGritPredicate {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GritBogusPredicate::KIND_SET
        .union(GritBooleanLiteral::KIND_SET)
        .union(GritBracketedPredicate::KIND_SET)
        .union(GritPredicateAccumulate::KIND_SET)
        .union(GritPredicateAnd::KIND_SET)
        .union(GritPredicateAny::KIND_SET)
        .union(GritPredicateAssignment::KIND_SET)
        .union(GritPredicateCall::KIND_SET)
        .union(GritPredicateEqual::KIND_SET)
        .union(GritPredicateGreater::KIND_SET)
        .union(GritPredicateGreaterEqual::KIND_SET)
        .union(GritPredicateIfElse::KIND_SET)
        .union(GritPredicateLess::KIND_SET)
        .union(GritPredicateLessEqual::KIND_SET)
        .union(GritPredicateMatch::KIND_SET)
        .union(GritPredicateMaybe::KIND_SET)
        .union(GritPredicateNot::KIND_SET)
        .union(GritPredicateNotEqual::KIND_SET)
        .union(GritPredicateOr::KIND_SET)
        .union(GritPredicateReturn::KIND_SET)
        .union(GritPredicateRewrite::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            GRIT_BOGUS_PREDICATE
                | GRIT_BOOLEAN_LITERAL
                | GRIT_BRACKETED_PREDICATE
                | GRIT_PREDICATE_ACCUMULATE
                | GRIT_PREDICATE_AND
                | GRIT_PREDICATE_ANY
                | GRIT_PREDICATE_ASSIGNMENT
                | GRIT_PREDICATE_CALL
                | GRIT_PREDICATE_EQUAL
                | GRIT_PREDICATE_GREATER
                | GRIT_PREDICATE_GREATER_EQUAL
                | GRIT_PREDICATE_IF_ELSE
                | GRIT_PREDICATE_LESS
                | GRIT_PREDICATE_LESS_EQUAL
                | GRIT_PREDICATE_MATCH
                | GRIT_PREDICATE_MAYBE
                | GRIT_PREDICATE_NOT
                | GRIT_PREDICATE_NOT_EQUAL
                | GRIT_PREDICATE_OR
                | GRIT_PREDICATE_RETURN
                | GRIT_PREDICATE_REWRITE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_BOGUS_PREDICATE => Self::GritBogusPredicate(GritBogusPredicate { syntax }),
            GRIT_BOOLEAN_LITERAL => Self::GritBooleanLiteral(GritBooleanLiteral { syntax }),
            GRIT_BRACKETED_PREDICATE => {
                Self::GritBracketedPredicate(GritBracketedPredicate { syntax })
            }
            GRIT_PREDICATE_ACCUMULATE => {
                Self::GritPredicateAccumulate(GritPredicateAccumulate { syntax })
            }
            GRIT_PREDICATE_AND => Self::GritPredicateAnd(GritPredicateAnd { syntax }),
            GRIT_PREDICATE_ANY => Self::GritPredicateAny(GritPredicateAny { syntax }),
            GRIT_PREDICATE_ASSIGNMENT => {
                Self::GritPredicateAssignment(GritPredicateAssignment { syntax })
            }
            GRIT_PREDICATE_CALL => Self::GritPredicateCall(GritPredicateCall { syntax }),
            GRIT_PREDICATE_EQUAL => Self::GritPredicateEqual(GritPredicateEqual { syntax }),
            GRIT_PREDICATE_GREATER => Self::GritPredicateGreater(GritPredicateGreater { syntax }),
            GRIT_PREDICATE_GREATER_EQUAL => {
                Self::GritPredicateGreaterEqual(GritPredicateGreaterEqual { syntax })
            }
            GRIT_PREDICATE_IF_ELSE => Self::GritPredicateIfElse(GritPredicateIfElse { syntax }),
            GRIT_PREDICATE_LESS => Self::GritPredicateLess(GritPredicateLess { syntax }),
            GRIT_PREDICATE_LESS_EQUAL => {
                Self::GritPredicateLessEqual(GritPredicateLessEqual { syntax })
            }
            GRIT_PREDICATE_MATCH => Self::GritPredicateMatch(GritPredicateMatch { syntax }),
            GRIT_PREDICATE_MAYBE => Self::GritPredicateMaybe(GritPredicateMaybe { syntax }),
            GRIT_PREDICATE_NOT => Self::GritPredicateNot(GritPredicateNot { syntax }),
            GRIT_PREDICATE_NOT_EQUAL => {
                Self::GritPredicateNotEqual(GritPredicateNotEqual { syntax })
            }
            GRIT_PREDICATE_OR => Self::GritPredicateOr(GritPredicateOr { syntax }),
            GRIT_PREDICATE_RETURN => Self::GritPredicateReturn(GritPredicateReturn { syntax }),
            GRIT_PREDICATE_REWRITE => Self::GritPredicateRewrite(GritPredicateRewrite { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritBogusPredicate(it) => &it.syntax,
            Self::GritBooleanLiteral(it) => &it.syntax,
            Self::GritBracketedPredicate(it) => &it.syntax,
            Self::GritPredicateAccumulate(it) => &it.syntax,
            Self::GritPredicateAnd(it) => &it.syntax,
            Self::GritPredicateAny(it) => &it.syntax,
            Self::GritPredicateAssignment(it) => &it.syntax,
            Self::GritPredicateCall(it) => &it.syntax,
            Self::GritPredicateEqual(it) => &it.syntax,
            Self::GritPredicateGreater(it) => &it.syntax,
            Self::GritPredicateGreaterEqual(it) => &it.syntax,
            Self::GritPredicateIfElse(it) => &it.syntax,
            Self::GritPredicateLess(it) => &it.syntax,
            Self::GritPredicateLessEqual(it) => &it.syntax,
            Self::GritPredicateMatch(it) => &it.syntax,
            Self::GritPredicateMaybe(it) => &it.syntax,
            Self::GritPredicateNot(it) => &it.syntax,
            Self::GritPredicateNotEqual(it) => &it.syntax,
            Self::GritPredicateOr(it) => &it.syntax,
            Self::GritPredicateReturn(it) => &it.syntax,
            Self::GritPredicateRewrite(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritBogusPredicate(it) => it.syntax,
            Self::GritBooleanLiteral(it) => it.syntax,
            Self::GritBracketedPredicate(it) => it.syntax,
            Self::GritPredicateAccumulate(it) => it.syntax,
            Self::GritPredicateAnd(it) => it.syntax,
            Self::GritPredicateAny(it) => it.syntax,
            Self::GritPredicateAssignment(it) => it.syntax,
            Self::GritPredicateCall(it) => it.syntax,
            Self::GritPredicateEqual(it) => it.syntax,
            Self::GritPredicateGreater(it) => it.syntax,
            Self::GritPredicateGreaterEqual(it) => it.syntax,
            Self::GritPredicateIfElse(it) => it.syntax,
            Self::GritPredicateLess(it) => it.syntax,
            Self::GritPredicateLessEqual(it) => it.syntax,
            Self::GritPredicateMatch(it) => it.syntax,
            Self::GritPredicateMaybe(it) => it.syntax,
            Self::GritPredicateNot(it) => it.syntax,
            Self::GritPredicateNotEqual(it) => it.syntax,
            Self::GritPredicateOr(it) => it.syntax,
            Self::GritPredicateReturn(it) => it.syntax,
            Self::GritPredicateRewrite(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGritPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GritBogusPredicate(it) => std::fmt::Debug::fmt(it, f),
            Self::GritBooleanLiteral(it) => std::fmt::Debug::fmt(it, f),
            Self::GritBracketedPredicate(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateAccumulate(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateAnd(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateAny(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateAssignment(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateCall(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateEqual(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateGreater(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateGreaterEqual(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateIfElse(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateLess(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateLessEqual(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateMatch(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateMaybe(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateNot(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateNotEqual(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateOr(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateReturn(it) => std::fmt::Debug::fmt(it, f),
            Self::GritPredicateRewrite(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritPredicate> for SyntaxNode {
    fn from(n: AnyGritPredicate) -> Self {
        match n {
            AnyGritPredicate::GritBogusPredicate(it) => it.into(),
            AnyGritPredicate::GritBooleanLiteral(it) => it.into(),
            AnyGritPredicate::GritBracketedPredicate(it) => it.into(),
            AnyGritPredicate::GritPredicateAccumulate(it) => it.into(),
            AnyGritPredicate::GritPredicateAnd(it) => it.into(),
            AnyGritPredicate::GritPredicateAny(it) => it.into(),
            AnyGritPredicate::GritPredicateAssignment(it) => it.into(),
            AnyGritPredicate::GritPredicateCall(it) => it.into(),
            AnyGritPredicate::GritPredicateEqual(it) => it.into(),
            AnyGritPredicate::GritPredicateGreater(it) => it.into(),
            AnyGritPredicate::GritPredicateGreaterEqual(it) => it.into(),
            AnyGritPredicate::GritPredicateIfElse(it) => it.into(),
            AnyGritPredicate::GritPredicateLess(it) => it.into(),
            AnyGritPredicate::GritPredicateLessEqual(it) => it.into(),
            AnyGritPredicate::GritPredicateMatch(it) => it.into(),
            AnyGritPredicate::GritPredicateMaybe(it) => it.into(),
            AnyGritPredicate::GritPredicateNot(it) => it.into(),
            AnyGritPredicate::GritPredicateNotEqual(it) => it.into(),
            AnyGritPredicate::GritPredicateOr(it) => it.into(),
            AnyGritPredicate::GritPredicateReturn(it) => it.into(),
            AnyGritPredicate::GritPredicateRewrite(it) => it.into(),
        }
    }
}
impl From<AnyGritPredicate> for SyntaxElement {
    fn from(n: AnyGritPredicate) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl AstNode for AnyGritPredicateMatchSubject {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyGritContainer::KIND_SET.union(AnyGritLiteral::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            k if AnyGritContainer::can_cast(k) => true,
            k if AnyGritLiteral::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let syntax = match AnyGritContainer::try_cast(syntax) {
            Ok(any_grit_container) => {
                return Some(Self::AnyGritContainer(any_grit_container));
            }
            Err(syntax) => syntax,
        };
        if let Some(any_grit_literal) = AnyGritLiteral::cast(syntax) {
            return Some(Self::AnyGritLiteral(any_grit_literal));
        }
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::AnyGritContainer(it) => it.syntax(),
            Self::AnyGritLiteral(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::AnyGritContainer(it) => it.into_syntax(),
            Self::AnyGritLiteral(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyGritPredicateMatchSubject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyGritContainer(it) => std::fmt::Debug::fmt(it, f),
            Self::AnyGritLiteral(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritPredicateMatchSubject> for SyntaxNode {
    fn from(n: AnyGritPredicateMatchSubject) -> Self {
        match n {
            AnyGritPredicateMatchSubject::AnyGritContainer(it) => it.into(),
            AnyGritPredicateMatchSubject::AnyGritLiteral(it) => it.into(),
        }
    }
}
impl From<AnyGritPredicateMatchSubject> for SyntaxElement {
    fn from(n: AnyGritPredicateMatchSubject) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritRegexLiteral> for AnyGritRegex {
    fn from(node: GritRegexLiteral) -> Self {
        Self::GritRegexLiteral(node)
    }
}
impl From<GritSnippetRegexLiteral> for AnyGritRegex {
    fn from(node: GritSnippetRegexLiteral) -> Self {
        Self::GritSnippetRegexLiteral(node)
    }
}
impl AstNode for AnyGritRegex {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        GritRegexLiteral::KIND_SET.union(GritSnippetRegexLiteral::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, GRIT_REGEX_LITERAL | GRIT_SNIPPET_REGEX_LITERAL)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_REGEX_LITERAL => Self::GritRegexLiteral(GritRegexLiteral { syntax }),
            GRIT_SNIPPET_REGEX_LITERAL => {
                Self::GritSnippetRegexLiteral(GritSnippetRegexLiteral { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritRegexLiteral(it) => &it.syntax,
            Self::GritSnippetRegexLiteral(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritRegexLiteral(it) => it.syntax,
            Self::GritSnippetRegexLiteral(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGritRegex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GritRegexLiteral(it) => std::fmt::Debug::fmt(it, f),
            Self::GritSnippetRegexLiteral(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritRegex> for SyntaxNode {
    fn from(n: AnyGritRegex) -> Self {
        match n {
            AnyGritRegex::GritRegexLiteral(it) => it.into(),
            AnyGritRegex::GritSnippetRegexLiteral(it) => it.into(),
        }
    }
}
impl From<AnyGritRegex> for SyntaxElement {
    fn from(n: AnyGritRegex) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritBogusVersion> for AnyGritVersion {
    fn from(node: GritBogusVersion) -> Self {
        Self::GritBogusVersion(node)
    }
}
impl From<GritVersion> for AnyGritVersion {
    fn from(node: GritVersion) -> Self {
        Self::GritVersion(node)
    }
}
impl AstNode for AnyGritVersion {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        GritBogusVersion::KIND_SET.union(GritVersion::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, GRIT_BOGUS_VERSION | GRIT_VERSION)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_BOGUS_VERSION => Self::GritBogusVersion(GritBogusVersion { syntax }),
            GRIT_VERSION => Self::GritVersion(GritVersion { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GritBogusVersion(it) => &it.syntax,
            Self::GritVersion(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GritBogusVersion(it) => it.syntax,
            Self::GritVersion(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGritVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GritBogusVersion(it) => std::fmt::Debug::fmt(it, f),
            Self::GritVersion(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritVersion> for SyntaxNode {
    fn from(n: AnyGritVersion) -> Self {
        match n {
            AnyGritVersion::GritBogusVersion(it) => it.into(),
            AnyGritVersion::GritVersion(it) => it.into(),
        }
    }
}
impl From<AnyGritVersion> for SyntaxElement {
    fn from(n: AnyGritVersion) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyGritCodeSnippetSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritLanguageDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritLanguageFlavorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritLanguageName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritListAccessorSubject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritListIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritListPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritMapAccessorSubject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritMapElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritMapKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritMaybeCurlyPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritMaybeNamedArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritPredicateMatchSubject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritRegex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritAddOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritAssignmentAsPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritBacktickSnippetLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritBooleanLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritBracketedPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritBracketedPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritBubble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritBubbleScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritCodeSnippet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritCurlyPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritDivOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritDot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritDotdotdot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritDoubleLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritEngineName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritEvery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritFunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritIntLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritJavascriptBodyWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritJavascriptFunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritLanguageDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritLanguageFlavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritLanguageFlavorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritLanguageName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritLanguageSpecificSnippet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritLikeThreshold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritListAccessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritMapAccessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritMapElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritModOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritMulOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritNamedArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritNegativeIntLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritNodeLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritNot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternAccumulate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternAfter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternAnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternAny {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternAs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternBefore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternContains {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternDefinitionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternIfElse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternIncludes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternMaybe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternNot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternOrElse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternUntilClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternWhere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateAccumulate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateAnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateAny {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateCurly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateGreater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateGreaterEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateIfElse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateLess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateLessEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateMaybe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateNot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateNotEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateRewrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritRawBacktickSnippetLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritRegexLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritRegexPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritRegexPatternVariables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritRewrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritSequential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritSnippetRegexLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritSome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritStringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritSubOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritUndefinedLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritUnderscore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritWithin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GritBogus {
    syntax: SyntaxNode,
}
impl GritBogus {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogus> for SyntaxNode {
    fn from(n: GritBogus) -> Self {
        n.syntax
    }
}
impl From<GritBogus> for SyntaxElement {
    fn from(n: GritBogus) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GritBogusContainer {
    syntax: SyntaxNode,
}
impl GritBogusContainer {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogusContainer {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS_CONTAINER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS_CONTAINER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogusContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogusContainer")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogusContainer> for SyntaxNode {
    fn from(n: GritBogusContainer) -> Self {
        n.syntax
    }
}
impl From<GritBogusContainer> for SyntaxElement {
    fn from(n: GritBogusContainer) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GritBogusDefinition {
    syntax: SyntaxNode,
}
impl GritBogusDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogusDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogusDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogusDefinition")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogusDefinition> for SyntaxNode {
    fn from(n: GritBogusDefinition) -> Self {
        n.syntax
    }
}
impl From<GritBogusDefinition> for SyntaxElement {
    fn from(n: GritBogusDefinition) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GritBogusLanguageDeclaration {
    syntax: SyntaxNode,
}
impl GritBogusLanguageDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogusLanguageDeclaration {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS_LANGUAGE_DECLARATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS_LANGUAGE_DECLARATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogusLanguageDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogusLanguageDeclaration")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogusLanguageDeclaration> for SyntaxNode {
    fn from(n: GritBogusLanguageDeclaration) -> Self {
        n.syntax
    }
}
impl From<GritBogusLanguageDeclaration> for SyntaxElement {
    fn from(n: GritBogusLanguageDeclaration) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GritBogusLanguageFlavorKind {
    syntax: SyntaxNode,
}
impl GritBogusLanguageFlavorKind {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogusLanguageFlavorKind {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS_LANGUAGE_FLAVOR_KIND as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS_LANGUAGE_FLAVOR_KIND
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogusLanguageFlavorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogusLanguageFlavorKind")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogusLanguageFlavorKind> for SyntaxNode {
    fn from(n: GritBogusLanguageFlavorKind) -> Self {
        n.syntax
    }
}
impl From<GritBogusLanguageFlavorKind> for SyntaxElement {
    fn from(n: GritBogusLanguageFlavorKind) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GritBogusLanguageName {
    syntax: SyntaxNode,
}
impl GritBogusLanguageName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogusLanguageName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS_LANGUAGE_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS_LANGUAGE_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogusLanguageName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogusLanguageName")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogusLanguageName> for SyntaxNode {
    fn from(n: GritBogusLanguageName) -> Self {
        n.syntax
    }
}
impl From<GritBogusLanguageName> for SyntaxElement {
    fn from(n: GritBogusLanguageName) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GritBogusLiteral {
    syntax: SyntaxNode,
}
impl GritBogusLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogusLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS_LITERAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogusLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogusLiteral")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogusLiteral> for SyntaxNode {
    fn from(n: GritBogusLiteral) -> Self {
        n.syntax
    }
}
impl From<GritBogusLiteral> for SyntaxElement {
    fn from(n: GritBogusLiteral) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GritBogusMapElement {
    syntax: SyntaxNode,
}
impl GritBogusMapElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogusMapElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS_MAP_ELEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS_MAP_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogusMapElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogusMapElement")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogusMapElement> for SyntaxNode {
    fn from(n: GritBogusMapElement) -> Self {
        n.syntax
    }
}
impl From<GritBogusMapElement> for SyntaxElement {
    fn from(n: GritBogusMapElement) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GritBogusNamedArg {
    syntax: SyntaxNode,
}
impl GritBogusNamedArg {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogusNamedArg {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS_NAMED_ARG as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS_NAMED_ARG
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogusNamedArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogusNamedArg")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogusNamedArg> for SyntaxNode {
    fn from(n: GritBogusNamedArg) -> Self {
        n.syntax
    }
}
impl From<GritBogusNamedArg> for SyntaxElement {
    fn from(n: GritBogusNamedArg) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GritBogusPattern {
    syntax: SyntaxNode,
}
impl GritBogusPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogusPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS_PATTERN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogusPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogusPattern")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogusPattern> for SyntaxNode {
    fn from(n: GritBogusPattern) -> Self {
        n.syntax
    }
}
impl From<GritBogusPattern> for SyntaxElement {
    fn from(n: GritBogusPattern) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GritBogusPredicate {
    syntax: SyntaxNode,
}
impl GritBogusPredicate {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogusPredicate {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS_PREDICATE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS_PREDICATE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogusPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogusPredicate")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogusPredicate> for SyntaxNode {
    fn from(n: GritBogusPredicate) -> Self {
        n.syntax
    }
}
impl From<GritBogusPredicate> for SyntaxElement {
    fn from(n: GritBogusPredicate) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GritBogusVersion {
    syntax: SyntaxNode,
}
impl GritBogusVersion {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogusVersion {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS_VERSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS_VERSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogusVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogusVersion")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogusVersion> for SyntaxNode {
    fn from(n: GritBogusVersion) -> Self {
        n.syntax
    }
}
impl From<GritBogusVersion> for SyntaxElement {
    fn from(n: GritBogusVersion) -> Self {
        n.syntax.into()
    }
}
biome_rowan::declare_node_union! { pub AnyGritBogusNode = GritBogus | GritBogusContainer | GritBogusDefinition | GritBogusLanguageDeclaration | GritBogusLanguageFlavorKind | GritBogusLanguageName | GritBogusLiteral | GritBogusMapElement | GritBogusNamedArg | GritBogusPattern | GritBogusPredicate | GritBogusVersion }
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritDefinitionList {
    syntax_list: SyntaxList,
}
impl GritDefinitionList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritDefinitionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_DEFINITION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_DEFINITION_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for GritDefinitionList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritDefinitionList {
    type Language = Language;
    type Node = AnyGritDefinition;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritDefinitionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritDefinitionList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritDefinitionList {
    type Item = SyntaxResult<AnyGritDefinition>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritDefinitionList {
    type Item = SyntaxResult<AnyGritDefinition>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritLanguageFlavorList {
    syntax_list: SyntaxList,
}
impl GritLanguageFlavorList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritLanguageFlavorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LANGUAGE_FLAVOR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LANGUAGE_FLAVOR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for GritLanguageFlavorList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritLanguageFlavorList {
    type Language = Language;
    type Node = AnyGritLanguageFlavorKind;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritLanguageFlavorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritLanguageFlavorList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritLanguageFlavorList {
    type Item = SyntaxResult<AnyGritLanguageFlavorKind>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritLanguageFlavorKind>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritLanguageFlavorList {
    type Item = SyntaxResult<AnyGritLanguageFlavorKind>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritLanguageFlavorKind>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritListPatternList {
    syntax_list: SyntaxList,
}
impl GritListPatternList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritListPatternList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LIST_PATTERN_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LIST_PATTERN_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for GritListPatternList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritListPatternList {
    type Language = Language;
    type Node = AnyGritListPattern;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritListPatternList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritListPatternList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritListPatternList {
    type Item = SyntaxResult<AnyGritListPattern>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritListPattern>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritListPatternList {
    type Item = SyntaxResult<AnyGritListPattern>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritListPattern>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritMapElementList {
    syntax_list: SyntaxList,
}
impl GritMapElementList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritMapElementList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_MAP_ELEMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_MAP_ELEMENT_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for GritMapElementList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritMapElementList {
    type Language = Language;
    type Node = AnyGritMapElement;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritMapElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritMapElementList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritMapElementList {
    type Item = SyntaxResult<AnyGritMapElement>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritMapElement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritMapElementList {
    type Item = SyntaxResult<AnyGritMapElement>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritMapElement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritNamedArgList {
    syntax_list: SyntaxList,
}
impl GritNamedArgList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritNamedArgList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_NAMED_ARG_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_NAMED_ARG_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for GritNamedArgList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritNamedArgList {
    type Language = Language;
    type Node = AnyGritMaybeNamedArg;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritNamedArgList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritNamedArgList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritNamedArgList {
    type Item = SyntaxResult<AnyGritMaybeNamedArg>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritMaybeNamedArg>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritNamedArgList {
    type Item = SyntaxResult<AnyGritMaybeNamedArg>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritMaybeNamedArg>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritPatternList {
    syntax_list: SyntaxList,
}
impl GritPatternList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritPatternList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for GritPatternList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritPatternList {
    type Language = Language;
    type Node = AnyGritPattern;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritPatternList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritPatternList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritPatternList {
    type Item = SyntaxResult<AnyGritPattern>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritPattern>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritPatternList {
    type Item = SyntaxResult<AnyGritPattern>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritPattern>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritPredicateList {
    syntax_list: SyntaxList,
}
impl GritPredicateList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritPredicateList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for GritPredicateList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritPredicateList {
    type Language = Language;
    type Node = AnyGritPredicate;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritPredicateList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritPredicateList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritPredicateList {
    type Item = SyntaxResult<AnyGritPredicate>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritPredicate>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritPredicateList {
    type Item = SyntaxResult<AnyGritPredicate>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritPredicate>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritVariableList {
    syntax_list: SyntaxList,
}
impl GritVariableList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritVariableList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_VARIABLE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_VARIABLE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for GritVariableList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritVariableList {
    type Language = Language;
    type Node = GritVariable;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritVariableList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritVariableList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritVariableList {
    type Item = SyntaxResult<GritVariable>;
    type IntoIter = AstSeparatedListNodesIterator<Language, GritVariable>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritVariableList {
    type Item = SyntaxResult<GritVariable>;
    type IntoIter = AstSeparatedListNodesIterator<Language, GritVariable>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone)]
pub struct DebugSyntaxElementChildren(pub SyntaxElementChildren);
impl Debug for DebugSyntaxElementChildren {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.clone().0.map(DebugSyntaxElement))
            .finish()
    }
}
struct DebugSyntaxElement(SyntaxElement);
impl Debug for DebugSyntaxElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            SyntaxElement::Node(node) => {
                map_syntax_node ! (node . clone () , node => std :: fmt :: Debug :: fmt (& node , f))
            }
            SyntaxElement::Token(token) => Debug::fmt(token, f),
        }
    }
}
