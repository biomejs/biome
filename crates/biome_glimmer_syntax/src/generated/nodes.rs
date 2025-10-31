//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    GlimmerLanguage as Language, GlimmerSyntaxElement as SyntaxElement,
    GlimmerSyntaxElementChildren as SyntaxElementChildren,
    GlimmerSyntaxKind::{self as SyntaxKind, *},
    GlimmerSyntaxList as SyntaxList, GlimmerSyntaxNode as SyntaxNode,
    GlimmerSyntaxToken as SyntaxToken,
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
pub struct GlimmerAtHead {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerAtHead {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerAtHeadFields {
        GlimmerAtHeadFields {
            at_token: self.at_token(),
            name_token: self.name_token(),
        }
    }
    pub fn at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for GlimmerAtHead {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerAtHeadFields {
    pub at_token: SyntaxResult<SyntaxToken>,
    pub name_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerAttributeNode {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerAttributeNode {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerAttributeNodeFields {
        GlimmerAttributeNodeFields {
            name_token: self.name_token(),
            eq_token: self.eq_token(),
            value: self.value(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<GlimmerAttributeValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GlimmerAttributeNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerAttributeNodeFields {
    pub name_token: SyntaxResult<SyntaxToken>,
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<GlimmerAttributeValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerBlock {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerBlockFields {
        GlimmerBlockFields {
            block_params: self.block_params(),
            statements: self.statements(),
        }
    }
    pub fn block_params(&self) -> Option<GlimmerBlockParams> {
        support::node(&self.syntax, 0usize)
    }
    pub fn statements(&self) -> GlimmerStatementList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for GlimmerBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerBlockFields {
    pub block_params: Option<GlimmerBlockParams>,
    pub statements: GlimmerStatementList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerBlockParams {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerBlockParams {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerBlockParamsFields {
        GlimmerBlockParamsFields {
            as_token: self.as_token(),
            bitwise_or_token: self.bitwise_or_token(),
            params: self.params(),
            bitwise_or_token: self.bitwise_or_token(),
        }
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn bitwise_or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn params(&self) -> GlimmerParamNameList {
        support::list(&self.syntax, 2usize)
    }
    pub fn bitwise_or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for GlimmerBlockParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerBlockParamsFields {
    pub as_token: SyntaxResult<SyntaxToken>,
    pub bitwise_or_token: SyntaxResult<SyntaxToken>,
    pub params: GlimmerParamNameList,
    pub bitwise_or_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerBlockStatement {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerBlockStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerBlockStatementFields {
        GlimmerBlockStatementFields {
            l_curly2_token: self.l_curly2_token(),
            hash_token: self.hash_token(),
            path: self.path(),
            params: self.params(),
            hash: self.hash(),
            r_curly2_token: self.r_curly2_token(),
            block: self.block(),
            else_block: self.else_block(),
            l_curly2_token: self.l_curly2_token(),
            slash_token: self.slash_token(),
            close_path: self.close_path(),
            r_curly2_token: self.r_curly2_token(),
        }
    }
    pub fn l_curly2_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn path(&self) -> SyntaxResult<GlimmerPathExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn params(&self) -> GlimmerParamsList {
        support::list(&self.syntax, 3usize)
    }
    pub fn hash(&self) -> SyntaxResult<GlimmerHash> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn r_curly2_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn block(&self) -> SyntaxResult<GlimmerBlock> {
        support::required_node(&self.syntax, 6usize)
    }
    pub fn else_block(&self) -> Option<GlimmerElseBlock> {
        support::node(&self.syntax, 7usize)
    }
    pub fn l_curly2_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 8usize)
    }
    pub fn slash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 9usize)
    }
    pub fn close_path(&self) -> SyntaxResult<GlimmerPathExpression> {
        support::required_node(&self.syntax, 10usize)
    }
    pub fn r_curly2_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 11usize)
    }
}
impl Serialize for GlimmerBlockStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerBlockStatementFields {
    pub l_curly2_token: SyntaxResult<SyntaxToken>,
    pub hash_token: SyntaxResult<SyntaxToken>,
    pub path: SyntaxResult<GlimmerPathExpression>,
    pub params: GlimmerParamsList,
    pub hash: SyntaxResult<GlimmerHash>,
    pub r_curly2_token: SyntaxResult<SyntaxToken>,
    pub block: SyntaxResult<GlimmerBlock>,
    pub else_block: Option<GlimmerElseBlock>,
    pub l_curly2_token: SyntaxResult<SyntaxToken>,
    pub slash_token: SyntaxResult<SyntaxToken>,
    pub close_path: SyntaxResult<GlimmerPathExpression>,
    pub r_curly2_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerBooleanLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerBooleanLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerBooleanLiteralFields {
        GlimmerBooleanLiteralFields {
            true_token: self.true_token(),
            false_token: self.false_token(),
        }
    }
    pub fn true_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn false_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for GlimmerBooleanLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerBooleanLiteralFields {
    pub true_token: SyntaxResult<SyntaxToken>,
    pub false_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerCommentStatement {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerCommentStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerCommentStatementFields {
        GlimmerCommentStatementFields {
            comment_token: self.comment_token(),
        }
    }
    pub fn comment_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GlimmerCommentStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerCommentStatementFields {
    pub comment_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerConcatStatement {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerConcatStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerConcatStatementFields {
        GlimmerConcatStatementFields {
            parts: self.parts(),
        }
    }
    pub fn parts(&self) -> GlimmerConcatPartList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for GlimmerConcatStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerConcatStatementFields {
    pub parts: GlimmerConcatPartList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerElementModifier {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerElementModifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerElementModifierFields {
        GlimmerElementModifierFields {
            l_curly2_token: self.l_curly2_token(),
            path: self.path(),
            params: self.params(),
            hash: self.hash(),
            r_curly2_token: self.r_curly2_token(),
        }
    }
    pub fn l_curly2_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn path(&self) -> SyntaxResult<GlimmerPathExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn params(&self) -> GlimmerParamsList {
        support::list(&self.syntax, 2usize)
    }
    pub fn hash(&self) -> SyntaxResult<GlimmerHash> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_curly2_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl Serialize for GlimmerElementModifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerElementModifierFields {
    pub l_curly2_token: SyntaxResult<SyntaxToken>,
    pub path: SyntaxResult<GlimmerPathExpression>,
    pub params: GlimmerParamsList,
    pub hash: SyntaxResult<GlimmerHash>,
    pub r_curly2_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerElementNode {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerElementNode {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerElementNodeFields {
        GlimmerElementNodeFields {
            opening: self.opening(),
            children: self.children(),
            closing: self.closing(),
        }
    }
    pub fn opening(&self) -> SyntaxResult<GlimmerStartTag> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn children(&self) -> GlimmerStatementList {
        support::list(&self.syntax, 1usize)
    }
    pub fn closing(&self) -> Option<GlimmerEndTag> {
        support::node(&self.syntax, 2usize)
    }
}
impl Serialize for GlimmerElementNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerElementNodeFields {
    pub opening: SyntaxResult<GlimmerStartTag>,
    pub children: GlimmerStatementList,
    pub closing: Option<GlimmerEndTag>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerElseBlock {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerElseBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerElseBlockFields {
        GlimmerElseBlockFields {
            l_curly2_token: self.l_curly2_token(),
            else_token: self.else_token(),
            if_token: self.if_token(),
            condition: self.condition(),
            r_curly2_token: self.r_curly2_token(),
            block: self.block(),
        }
    }
    pub fn l_curly2_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn else_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn if_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn condition(&self) -> Option<Expression> {
        support::node(&self.syntax, 3usize)
    }
    pub fn r_curly2_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn block(&self) -> SyntaxResult<GlimmerBlock> {
        support::required_node(&self.syntax, 5usize)
    }
}
impl Serialize for GlimmerElseBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerElseBlockFields {
    pub l_curly2_token: SyntaxResult<SyntaxToken>,
    pub else_token: SyntaxResult<SyntaxToken>,
    pub if_token: Option<SyntaxToken>,
    pub condition: Option<Expression>,
    pub r_curly2_token: SyntaxResult<SyntaxToken>,
    pub block: SyntaxResult<GlimmerBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerEndTag {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerEndTag {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerEndTagFields {
        GlimmerEndTagFields {
            l_angle_token: self.l_angle_token(),
            slash_token: self.slash_token(),
            name: self.name(),
            r_angle_token: self.r_angle_token(),
        }
    }
    pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn slash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<GlimmerPathExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for GlimmerEndTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerEndTagFields {
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub slash_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GlimmerPathExpression>,
    pub r_angle_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerHash {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerHash {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerHashFields {
        GlimmerHashFields {
            pairs: self.pairs(),
        }
    }
    pub fn pairs(&self) -> GlimmerHashPairList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for GlimmerHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerHashFields {
    pub pairs: GlimmerHashPairList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerHashPair {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerHashPair {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerHashPairFields {
        GlimmerHashPairFields {
            key_token: self.key_token(),
            eq_token: self.eq_token(),
            value: self.value(),
        }
    }
    pub fn key_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GlimmerHashPair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerHashPairFields {
    pub key_token: SyntaxResult<SyntaxToken>,
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<Expression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerMustacheCommentStatement {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerMustacheCommentStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerMustacheCommentStatementFields {
        GlimmerMustacheCommentStatementFields {
            mustache_comment_token: self.mustache_comment_token(),
        }
    }
    pub fn mustache_comment_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GlimmerMustacheCommentStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerMustacheCommentStatementFields {
    pub mustache_comment_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerMustacheStatement {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerMustacheStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerMustacheStatementFields {
        GlimmerMustacheStatementFields {
            l_curly2_token: self.l_curly2_token(),
            l_curly2_token: self.l_curly2_token(),
            expression: self.expression(),
            r_curly2_token: self.r_curly2_token(),
            r_curly2_token: self.r_curly2_token(),
        }
    }
    pub fn l_curly2_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly2_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_curly2_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn r_curly2_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 4usize)
    }
}
impl Serialize for GlimmerMustacheStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerMustacheStatementFields {
    pub l_curly2_token: SyntaxResult<SyntaxToken>,
    pub l_curly2_token: Option<SyntaxToken>,
    pub expression: SyntaxResult<Expression>,
    pub r_curly2_token: SyntaxResult<SyntaxToken>,
    pub r_curly2_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerNullLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerNullLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerNullLiteralFields {
        GlimmerNullLiteralFields {
            null_token: self.null_token(),
        }
    }
    pub fn null_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GlimmerNullLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerNullLiteralFields {
    pub null_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerNumberLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerNumberLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerNumberLiteralFields {
        GlimmerNumberLiteralFields {
            number_literal_token: self.number_literal_token(),
        }
    }
    pub fn number_literal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GlimmerNumberLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerNumberLiteralFields {
    pub number_literal_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerParamName {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerParamName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerParamNameFields {
        GlimmerParamNameFields {
            name_token: self.name_token(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GlimmerParamName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerParamNameFields {
    pub name_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerPathExpression {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerPathExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerPathExpressionFields {
        GlimmerPathExpressionFields {
            head: self.head(),
            tail: self.tail(),
        }
    }
    pub fn head(&self) -> SyntaxResult<PathHead> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn tail(&self) -> GlimmerPathSegmentList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for GlimmerPathExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerPathExpressionFields {
    pub head: SyntaxResult<PathHead>,
    pub tail: GlimmerPathSegmentList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerPathSegment {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerPathSegment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerPathSegmentFields {
        GlimmerPathSegmentFields {
            dot_token: self.dot_token(),
            segment_token: self.segment_token(),
        }
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn segment_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for GlimmerPathSegment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerPathSegmentFields {
    pub dot_token: SyntaxResult<SyntaxToken>,
    pub segment_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerRoot {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerRootFields {
        GlimmerRootFields {
            bom_token: self.bom_token(),
            statements: self.statements(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn statements(&self) -> GlimmerStatementList {
        support::list(&self.syntax, 1usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GlimmerRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerRootFields {
    pub bom_token: Option<SyntaxToken>,
    pub statements: GlimmerStatementList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerStartTag {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerStartTag {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerStartTagFields {
        GlimmerStartTagFields {
            l_angle_token: self.l_angle_token(),
            name: self.name(),
            attributes: self.attributes(),
            modifiers: self.modifiers(),
            block_params: self.block_params(),
            self_closing: self.self_closing(),
            r_angle_token: self.r_angle_token(),
        }
    }
    pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<GlimmerPathExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn attributes(&self) -> GlimmerAttributeList {
        support::list(&self.syntax, 2usize)
    }
    pub fn modifiers(&self) -> GlimmerElementModifierList {
        support::list(&self.syntax, 3usize)
    }
    pub fn block_params(&self) -> Option<GlimmerBlockParams> {
        support::node(&self.syntax, 4usize)
    }
    pub fn self_closing(&self) -> Option<SelfClosing> {
        support::node(&self.syntax, 5usize)
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 6usize)
    }
}
impl Serialize for GlimmerStartTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerStartTagFields {
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GlimmerPathExpression>,
    pub attributes: GlimmerAttributeList,
    pub modifiers: GlimmerElementModifierList,
    pub block_params: Option<GlimmerBlockParams>,
    pub self_closing: Option<SelfClosing>,
    pub r_angle_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerStringLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerStringLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerStringLiteralFields {
        GlimmerStringLiteralFields {
            string_literal_token: self.string_literal_token(),
        }
    }
    pub fn string_literal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GlimmerStringLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerStringLiteralFields {
    pub string_literal_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerSubExpression {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerSubExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerSubExpressionFields {
        GlimmerSubExpressionFields {
            l_paren_token: self.l_paren_token(),
            path: self.path(),
            params: self.params(),
            hash: self.hash(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn path(&self) -> SyntaxResult<GlimmerPathExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn params(&self) -> GlimmerParamsList {
        support::list(&self.syntax, 2usize)
    }
    pub fn hash(&self) -> SyntaxResult<GlimmerHash> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl Serialize for GlimmerSubExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerSubExpressionFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub path: SyntaxResult<GlimmerPathExpression>,
    pub params: GlimmerParamsList,
    pub hash: SyntaxResult<GlimmerHash>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerTextNode {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerTextNode {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerTextNodeFields {
        GlimmerTextNodeFields {
            text_token: self.text_token(),
        }
    }
    pub fn text_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GlimmerTextNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerTextNodeFields {
    pub text_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerThisHead {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerThisHead {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerThisHeadFields {
        GlimmerThisHeadFields {
            this_token: self.this_token(),
        }
    }
    pub fn this_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GlimmerThisHead {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerThisHeadFields {
    pub this_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerUndefinedLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerUndefinedLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerUndefinedLiteralFields {
        GlimmerUndefinedLiteralFields {
            undefined_token: self.undefined_token(),
        }
    }
    pub fn undefined_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GlimmerUndefinedLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerUndefinedLiteralFields {
    pub undefined_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GlimmerVarHead {
    pub(crate) syntax: SyntaxNode,
}
impl GlimmerVarHead {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GlimmerVarHeadFields {
        GlimmerVarHeadFields {
            name_token: self.name_token(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GlimmerVarHead {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GlimmerVarHeadFields {
    pub name_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SelfClosing {
    pub(crate) syntax: SyntaxNode,
}
impl SelfClosing {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SelfClosingFields {
        SelfClosingFields {
            slash_token: self.slash_token(),
        }
    }
    pub fn slash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SelfClosing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SelfClosingFields {
    pub slash_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyConcatPart {
    GlimmerMustacheStatement(GlimmerMustacheStatement),
    GlimmerTextNode(GlimmerTextNode),
}
impl AnyConcatPart {
    pub fn as_glimmer_mustache_statement(&self) -> Option<&GlimmerMustacheStatement> {
        match &self {
            Self::GlimmerMustacheStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_text_node(&self) -> Option<&GlimmerTextNode> {
        match &self {
            Self::GlimmerTextNode(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum Expression {
    GlimmerBogusExpression(GlimmerBogusExpression),
    GlimmerLiteral(GlimmerLiteral),
    GlimmerPathExpression(GlimmerPathExpression),
    GlimmerSubExpression(GlimmerSubExpression),
}
impl Expression {
    pub fn as_glimmer_bogus_expression(&self) -> Option<&GlimmerBogusExpression> {
        match &self {
            Self::GlimmerBogusExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_literal(&self) -> Option<&GlimmerLiteral> {
        match &self {
            Self::GlimmerLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_path_expression(&self) -> Option<&GlimmerPathExpression> {
        match &self {
            Self::GlimmerPathExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_sub_expression(&self) -> Option<&GlimmerSubExpression> {
        match &self {
            Self::GlimmerSubExpression(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum GlimmerAttributeValue {
    GlimmerConcatStatement(GlimmerConcatStatement),
    GlimmerMustacheStatement(GlimmerMustacheStatement),
    GlimmerTextNode(GlimmerTextNode),
}
impl GlimmerAttributeValue {
    pub fn as_glimmer_concat_statement(&self) -> Option<&GlimmerConcatStatement> {
        match &self {
            Self::GlimmerConcatStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_mustache_statement(&self) -> Option<&GlimmerMustacheStatement> {
        match &self {
            Self::GlimmerMustacheStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_text_node(&self) -> Option<&GlimmerTextNode> {
        match &self {
            Self::GlimmerTextNode(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum GlimmerLiteral {
    GlimmerBooleanLiteral(GlimmerBooleanLiteral),
    GlimmerNullLiteral(GlimmerNullLiteral),
    GlimmerNumberLiteral(GlimmerNumberLiteral),
    GlimmerStringLiteral(GlimmerStringLiteral),
    GlimmerUndefinedLiteral(GlimmerUndefinedLiteral),
}
impl GlimmerLiteral {
    pub fn as_glimmer_boolean_literal(&self) -> Option<&GlimmerBooleanLiteral> {
        match &self {
            Self::GlimmerBooleanLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_null_literal(&self) -> Option<&GlimmerNullLiteral> {
        match &self {
            Self::GlimmerNullLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_number_literal(&self) -> Option<&GlimmerNumberLiteral> {
        match &self {
            Self::GlimmerNumberLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_string_literal(&self) -> Option<&GlimmerStringLiteral> {
        match &self {
            Self::GlimmerStringLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_undefined_literal(&self) -> Option<&GlimmerUndefinedLiteral> {
        match &self {
            Self::GlimmerUndefinedLiteral(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum PathHead {
    GlimmerAtHead(GlimmerAtHead),
    GlimmerThisHead(GlimmerThisHead),
    GlimmerVarHead(GlimmerVarHead),
}
impl PathHead {
    pub fn as_glimmer_at_head(&self) -> Option<&GlimmerAtHead> {
        match &self {
            Self::GlimmerAtHead(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_this_head(&self) -> Option<&GlimmerThisHead> {
        match &self {
            Self::GlimmerThisHead(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_var_head(&self) -> Option<&GlimmerVarHead> {
        match &self {
            Self::GlimmerVarHead(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum Statement {
    GlimmerBlockStatement(GlimmerBlockStatement),
    GlimmerBogusStatement(GlimmerBogusStatement),
    GlimmerCommentStatement(GlimmerCommentStatement),
    GlimmerElementNode(GlimmerElementNode),
    GlimmerMustacheCommentStatement(GlimmerMustacheCommentStatement),
    GlimmerMustacheStatement(GlimmerMustacheStatement),
    GlimmerTextNode(GlimmerTextNode),
}
impl Statement {
    pub fn as_glimmer_block_statement(&self) -> Option<&GlimmerBlockStatement> {
        match &self {
            Self::GlimmerBlockStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_bogus_statement(&self) -> Option<&GlimmerBogusStatement> {
        match &self {
            Self::GlimmerBogusStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_comment_statement(&self) -> Option<&GlimmerCommentStatement> {
        match &self {
            Self::GlimmerCommentStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_element_node(&self) -> Option<&GlimmerElementNode> {
        match &self {
            Self::GlimmerElementNode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_mustache_comment_statement(
        &self,
    ) -> Option<&GlimmerMustacheCommentStatement> {
        match &self {
            Self::GlimmerMustacheCommentStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_mustache_statement(&self) -> Option<&GlimmerMustacheStatement> {
        match &self {
            Self::GlimmerMustacheStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_glimmer_text_node(&self) -> Option<&GlimmerTextNode> {
        match &self {
            Self::GlimmerTextNode(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for GlimmerAtHead {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_AT_HEAD as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_AT_HEAD
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
impl std::fmt::Debug for GlimmerAtHead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerAtHead")
                .field("at_token", &support::DebugSyntaxResult(self.at_token()))
                .field("name_token", &support::DebugSyntaxResult(self.name_token()))
                .finish()
        } else {
            f.debug_struct("GlimmerAtHead").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerAtHead> for SyntaxNode {
    fn from(n: GlimmerAtHead) -> Self {
        n.syntax
    }
}
impl From<GlimmerAtHead> for SyntaxElement {
    fn from(n: GlimmerAtHead) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerAttributeNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_ATTRIBUTE_NODE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_ATTRIBUTE_NODE
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
impl std::fmt::Debug for GlimmerAttributeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerAttributeNode")
                .field("name_token", &support::DebugSyntaxResult(self.name_token()))
                .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("GlimmerAttributeNode").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerAttributeNode> for SyntaxNode {
    fn from(n: GlimmerAttributeNode) -> Self {
        n.syntax
    }
}
impl From<GlimmerAttributeNode> for SyntaxElement {
    fn from(n: GlimmerAttributeNode) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_BLOCK
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
impl std::fmt::Debug for GlimmerBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerBlock")
                .field(
                    "block_params",
                    &support::DebugOptionalElement(self.block_params()),
                )
                .field("statements", &self.statements())
                .finish()
        } else {
            f.debug_struct("GlimmerBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerBlock> for SyntaxNode {
    fn from(n: GlimmerBlock) -> Self {
        n.syntax
    }
}
impl From<GlimmerBlock> for SyntaxElement {
    fn from(n: GlimmerBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerBlockParams {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_BLOCK_PARAMS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_BLOCK_PARAMS
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
impl std::fmt::Debug for GlimmerBlockParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerBlockParams")
                .field("as_token", &support::DebugSyntaxResult(self.as_token()))
                .field(
                    "bitwise_or_token",
                    &support::DebugSyntaxResult(self.bitwise_or_token()),
                )
                .field("params", &self.params())
                .field(
                    "bitwise_or_token",
                    &support::DebugSyntaxResult(self.bitwise_or_token()),
                )
                .finish()
        } else {
            f.debug_struct("GlimmerBlockParams").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerBlockParams> for SyntaxNode {
    fn from(n: GlimmerBlockParams) -> Self {
        n.syntax
    }
}
impl From<GlimmerBlockParams> for SyntaxElement {
    fn from(n: GlimmerBlockParams) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerBlockStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_BLOCK_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_BLOCK_STATEMENT
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
impl std::fmt::Debug for GlimmerBlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerBlockStatement")
                .field(
                    "l_curly2_token",
                    &support::DebugSyntaxResult(self.l_curly2_token()),
                )
                .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
                .field("path", &support::DebugSyntaxResult(self.path()))
                .field("params", &self.params())
                .field("hash", &support::DebugSyntaxResult(self.hash()))
                .field(
                    "r_curly2_token",
                    &support::DebugSyntaxResult(self.r_curly2_token()),
                )
                .field("block", &support::DebugSyntaxResult(self.block()))
                .field(
                    "else_block",
                    &support::DebugOptionalElement(self.else_block()),
                )
                .field(
                    "l_curly2_token",
                    &support::DebugSyntaxResult(self.l_curly2_token()),
                )
                .field(
                    "slash_token",
                    &support::DebugSyntaxResult(self.slash_token()),
                )
                .field("close_path", &support::DebugSyntaxResult(self.close_path()))
                .field(
                    "r_curly2_token",
                    &support::DebugSyntaxResult(self.r_curly2_token()),
                )
                .finish()
        } else {
            f.debug_struct("GlimmerBlockStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerBlockStatement> for SyntaxNode {
    fn from(n: GlimmerBlockStatement) -> Self {
        n.syntax
    }
}
impl From<GlimmerBlockStatement> for SyntaxElement {
    fn from(n: GlimmerBlockStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerBooleanLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_BOOLEAN_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_BOOLEAN_LITERAL
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
impl std::fmt::Debug for GlimmerBooleanLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerBooleanLiteral")
                .field("true_token", &support::DebugSyntaxResult(self.true_token()))
                .field(
                    "false_token",
                    &support::DebugSyntaxResult(self.false_token()),
                )
                .finish()
        } else {
            f.debug_struct("GlimmerBooleanLiteral").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerBooleanLiteral> for SyntaxNode {
    fn from(n: GlimmerBooleanLiteral) -> Self {
        n.syntax
    }
}
impl From<GlimmerBooleanLiteral> for SyntaxElement {
    fn from(n: GlimmerBooleanLiteral) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerCommentStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_COMMENT_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_COMMENT_STATEMENT
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
impl std::fmt::Debug for GlimmerCommentStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerCommentStatement")
                .field(
                    "comment_token",
                    &support::DebugSyntaxResult(self.comment_token()),
                )
                .finish()
        } else {
            f.debug_struct("GlimmerCommentStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerCommentStatement> for SyntaxNode {
    fn from(n: GlimmerCommentStatement) -> Self {
        n.syntax
    }
}
impl From<GlimmerCommentStatement> for SyntaxElement {
    fn from(n: GlimmerCommentStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerConcatStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_CONCAT_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_CONCAT_STATEMENT
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
impl std::fmt::Debug for GlimmerConcatStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerConcatStatement")
                .field("parts", &self.parts())
                .finish()
        } else {
            f.debug_struct("GlimmerConcatStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerConcatStatement> for SyntaxNode {
    fn from(n: GlimmerConcatStatement) -> Self {
        n.syntax
    }
}
impl From<GlimmerConcatStatement> for SyntaxElement {
    fn from(n: GlimmerConcatStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerElementModifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_ELEMENT_MODIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_ELEMENT_MODIFIER
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
impl std::fmt::Debug for GlimmerElementModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerElementModifier")
                .field(
                    "l_curly2_token",
                    &support::DebugSyntaxResult(self.l_curly2_token()),
                )
                .field("path", &support::DebugSyntaxResult(self.path()))
                .field("params", &self.params())
                .field("hash", &support::DebugSyntaxResult(self.hash()))
                .field(
                    "r_curly2_token",
                    &support::DebugSyntaxResult(self.r_curly2_token()),
                )
                .finish()
        } else {
            f.debug_struct("GlimmerElementModifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerElementModifier> for SyntaxNode {
    fn from(n: GlimmerElementModifier) -> Self {
        n.syntax
    }
}
impl From<GlimmerElementModifier> for SyntaxElement {
    fn from(n: GlimmerElementModifier) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerElementNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_ELEMENT_NODE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_ELEMENT_NODE
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
impl std::fmt::Debug for GlimmerElementNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerElementNode")
                .field("opening", &support::DebugSyntaxResult(self.opening()))
                .field("children", &self.children())
                .field("closing", &support::DebugOptionalElement(self.closing()))
                .finish()
        } else {
            f.debug_struct("GlimmerElementNode").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerElementNode> for SyntaxNode {
    fn from(n: GlimmerElementNode) -> Self {
        n.syntax
    }
}
impl From<GlimmerElementNode> for SyntaxElement {
    fn from(n: GlimmerElementNode) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerElseBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_ELSE_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_ELSE_BLOCK
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
impl std::fmt::Debug for GlimmerElseBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerElseBlock")
                .field(
                    "l_curly2_token",
                    &support::DebugSyntaxResult(self.l_curly2_token()),
                )
                .field("else_token", &support::DebugSyntaxResult(self.else_token()))
                .field("if_token", &support::DebugOptionalElement(self.if_token()))
                .field(
                    "condition",
                    &support::DebugOptionalElement(self.condition()),
                )
                .field(
                    "r_curly2_token",
                    &support::DebugSyntaxResult(self.r_curly2_token()),
                )
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("GlimmerElseBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerElseBlock> for SyntaxNode {
    fn from(n: GlimmerElseBlock) -> Self {
        n.syntax
    }
}
impl From<GlimmerElseBlock> for SyntaxElement {
    fn from(n: GlimmerElseBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerEndTag {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_END_TAG as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_END_TAG
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
impl std::fmt::Debug for GlimmerEndTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerEndTag")
                .field(
                    "l_angle_token",
                    &support::DebugSyntaxResult(self.l_angle_token()),
                )
                .field(
                    "slash_token",
                    &support::DebugSyntaxResult(self.slash_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "r_angle_token",
                    &support::DebugSyntaxResult(self.r_angle_token()),
                )
                .finish()
        } else {
            f.debug_struct("GlimmerEndTag").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerEndTag> for SyntaxNode {
    fn from(n: GlimmerEndTag) -> Self {
        n.syntax
    }
}
impl From<GlimmerEndTag> for SyntaxElement {
    fn from(n: GlimmerEndTag) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerHash {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_HASH as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_HASH
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
impl std::fmt::Debug for GlimmerHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerHash")
                .field("pairs", &self.pairs())
                .finish()
        } else {
            f.debug_struct("GlimmerHash").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerHash> for SyntaxNode {
    fn from(n: GlimmerHash) -> Self {
        n.syntax
    }
}
impl From<GlimmerHash> for SyntaxElement {
    fn from(n: GlimmerHash) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerHashPair {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_HASH_PAIR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_HASH_PAIR
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
impl std::fmt::Debug for GlimmerHashPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerHashPair")
                .field("key_token", &support::DebugSyntaxResult(self.key_token()))
                .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("GlimmerHashPair").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerHashPair> for SyntaxNode {
    fn from(n: GlimmerHashPair) -> Self {
        n.syntax
    }
}
impl From<GlimmerHashPair> for SyntaxElement {
    fn from(n: GlimmerHashPair) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerMustacheCommentStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_MUSTACHE_COMMENT_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_MUSTACHE_COMMENT_STATEMENT
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
impl std::fmt::Debug for GlimmerMustacheCommentStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerMustacheCommentStatement")
                .field(
                    "mustache_comment_token",
                    &support::DebugSyntaxResult(self.mustache_comment_token()),
                )
                .finish()
        } else {
            f.debug_struct("GlimmerMustacheCommentStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerMustacheCommentStatement> for SyntaxNode {
    fn from(n: GlimmerMustacheCommentStatement) -> Self {
        n.syntax
    }
}
impl From<GlimmerMustacheCommentStatement> for SyntaxElement {
    fn from(n: GlimmerMustacheCommentStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerMustacheStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_MUSTACHE_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_MUSTACHE_STATEMENT
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
impl std::fmt::Debug for GlimmerMustacheStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerMustacheStatement")
                .field(
                    "l_curly2_token",
                    &support::DebugSyntaxResult(self.l_curly2_token()),
                )
                .field(
                    "l_curly2_token",
                    &support::DebugOptionalElement(self.l_curly2_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "r_curly2_token",
                    &support::DebugSyntaxResult(self.r_curly2_token()),
                )
                .field(
                    "r_curly2_token",
                    &support::DebugOptionalElement(self.r_curly2_token()),
                )
                .finish()
        } else {
            f.debug_struct("GlimmerMustacheStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerMustacheStatement> for SyntaxNode {
    fn from(n: GlimmerMustacheStatement) -> Self {
        n.syntax
    }
}
impl From<GlimmerMustacheStatement> for SyntaxElement {
    fn from(n: GlimmerMustacheStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerNullLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_NULL_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_NULL_LITERAL
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
impl std::fmt::Debug for GlimmerNullLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerNullLiteral")
                .field("null_token", &support::DebugSyntaxResult(self.null_token()))
                .finish()
        } else {
            f.debug_struct("GlimmerNullLiteral").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerNullLiteral> for SyntaxNode {
    fn from(n: GlimmerNullLiteral) -> Self {
        n.syntax
    }
}
impl From<GlimmerNullLiteral> for SyntaxElement {
    fn from(n: GlimmerNullLiteral) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerNumberLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_NUMBER_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_NUMBER_LITERAL
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
impl std::fmt::Debug for GlimmerNumberLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerNumberLiteral")
                .field(
                    "number_literal_token",
                    &support::DebugSyntaxResult(self.number_literal_token()),
                )
                .finish()
        } else {
            f.debug_struct("GlimmerNumberLiteral").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerNumberLiteral> for SyntaxNode {
    fn from(n: GlimmerNumberLiteral) -> Self {
        n.syntax
    }
}
impl From<GlimmerNumberLiteral> for SyntaxElement {
    fn from(n: GlimmerNumberLiteral) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerParamName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_PARAM_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_PARAM_NAME
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
impl std::fmt::Debug for GlimmerParamName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerParamName")
                .field("name_token", &support::DebugSyntaxResult(self.name_token()))
                .finish()
        } else {
            f.debug_struct("GlimmerParamName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerParamName> for SyntaxNode {
    fn from(n: GlimmerParamName) -> Self {
        n.syntax
    }
}
impl From<GlimmerParamName> for SyntaxElement {
    fn from(n: GlimmerParamName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerPathExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_PATH_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_PATH_EXPRESSION
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
impl std::fmt::Debug for GlimmerPathExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerPathExpression")
                .field("head", &support::DebugSyntaxResult(self.head()))
                .field("tail", &self.tail())
                .finish()
        } else {
            f.debug_struct("GlimmerPathExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerPathExpression> for SyntaxNode {
    fn from(n: GlimmerPathExpression) -> Self {
        n.syntax
    }
}
impl From<GlimmerPathExpression> for SyntaxElement {
    fn from(n: GlimmerPathExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerPathSegment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_PATH_SEGMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_PATH_SEGMENT
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
impl std::fmt::Debug for GlimmerPathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerPathSegment")
                .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
                .field(
                    "segment_token",
                    &support::DebugSyntaxResult(self.segment_token()),
                )
                .finish()
        } else {
            f.debug_struct("GlimmerPathSegment").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerPathSegment> for SyntaxNode {
    fn from(n: GlimmerPathSegment) -> Self {
        n.syntax
    }
}
impl From<GlimmerPathSegment> for SyntaxElement {
    fn from(n: GlimmerPathSegment) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_ROOT
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
impl std::fmt::Debug for GlimmerRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerRoot")
                .field(
                    "bom_token",
                    &support::DebugOptionalElement(self.bom_token()),
                )
                .field("statements", &self.statements())
                .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
                .finish()
        } else {
            f.debug_struct("GlimmerRoot").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerRoot> for SyntaxNode {
    fn from(n: GlimmerRoot) -> Self {
        n.syntax
    }
}
impl From<GlimmerRoot> for SyntaxElement {
    fn from(n: GlimmerRoot) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerStartTag {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_START_TAG as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_START_TAG
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
impl std::fmt::Debug for GlimmerStartTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerStartTag")
                .field(
                    "l_angle_token",
                    &support::DebugSyntaxResult(self.l_angle_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("attributes", &self.attributes())
                .field("modifiers", &self.modifiers())
                .field(
                    "block_params",
                    &support::DebugOptionalElement(self.block_params()),
                )
                .field(
                    "self_closing",
                    &support::DebugOptionalElement(self.self_closing()),
                )
                .field(
                    "r_angle_token",
                    &support::DebugSyntaxResult(self.r_angle_token()),
                )
                .finish()
        } else {
            f.debug_struct("GlimmerStartTag").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerStartTag> for SyntaxNode {
    fn from(n: GlimmerStartTag) -> Self {
        n.syntax
    }
}
impl From<GlimmerStartTag> for SyntaxElement {
    fn from(n: GlimmerStartTag) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerStringLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_STRING_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_STRING_LITERAL
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
impl std::fmt::Debug for GlimmerStringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerStringLiteral")
                .field(
                    "string_literal_token",
                    &support::DebugSyntaxResult(self.string_literal_token()),
                )
                .finish()
        } else {
            f.debug_struct("GlimmerStringLiteral").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerStringLiteral> for SyntaxNode {
    fn from(n: GlimmerStringLiteral) -> Self {
        n.syntax
    }
}
impl From<GlimmerStringLiteral> for SyntaxElement {
    fn from(n: GlimmerStringLiteral) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerSubExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_SUB_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_SUB_EXPRESSION
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
impl std::fmt::Debug for GlimmerSubExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerSubExpression")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("path", &support::DebugSyntaxResult(self.path()))
                .field("params", &self.params())
                .field("hash", &support::DebugSyntaxResult(self.hash()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("GlimmerSubExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerSubExpression> for SyntaxNode {
    fn from(n: GlimmerSubExpression) -> Self {
        n.syntax
    }
}
impl From<GlimmerSubExpression> for SyntaxElement {
    fn from(n: GlimmerSubExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerTextNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_TEXT_NODE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_TEXT_NODE
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
impl std::fmt::Debug for GlimmerTextNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerTextNode")
                .field("text_token", &support::DebugSyntaxResult(self.text_token()))
                .finish()
        } else {
            f.debug_struct("GlimmerTextNode").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerTextNode> for SyntaxNode {
    fn from(n: GlimmerTextNode) -> Self {
        n.syntax
    }
}
impl From<GlimmerTextNode> for SyntaxElement {
    fn from(n: GlimmerTextNode) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerThisHead {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_THIS_HEAD as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_THIS_HEAD
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
impl std::fmt::Debug for GlimmerThisHead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerThisHead")
                .field("this_token", &support::DebugSyntaxResult(self.this_token()))
                .finish()
        } else {
            f.debug_struct("GlimmerThisHead").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerThisHead> for SyntaxNode {
    fn from(n: GlimmerThisHead) -> Self {
        n.syntax
    }
}
impl From<GlimmerThisHead> for SyntaxElement {
    fn from(n: GlimmerThisHead) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerUndefinedLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_UNDEFINED_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_UNDEFINED_LITERAL
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
impl std::fmt::Debug for GlimmerUndefinedLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerUndefinedLiteral")
                .field(
                    "undefined_token",
                    &support::DebugSyntaxResult(self.undefined_token()),
                )
                .finish()
        } else {
            f.debug_struct("GlimmerUndefinedLiteral").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerUndefinedLiteral> for SyntaxNode {
    fn from(n: GlimmerUndefinedLiteral) -> Self {
        n.syntax
    }
}
impl From<GlimmerUndefinedLiteral> for SyntaxElement {
    fn from(n: GlimmerUndefinedLiteral) -> Self {
        n.syntax.into()
    }
}
impl AstNode for GlimmerVarHead {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_VAR_HEAD as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_VAR_HEAD
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
impl std::fmt::Debug for GlimmerVarHead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GlimmerVarHead")
                .field("name_token", &support::DebugSyntaxResult(self.name_token()))
                .finish()
        } else {
            f.debug_struct("GlimmerVarHead").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GlimmerVarHead> for SyntaxNode {
    fn from(n: GlimmerVarHead) -> Self {
        n.syntax
    }
}
impl From<GlimmerVarHead> for SyntaxElement {
    fn from(n: GlimmerVarHead) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SelfClosing {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SELF_CLOSING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SELF_CLOSING
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
impl std::fmt::Debug for SelfClosing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SelfClosing")
                .field(
                    "slash_token",
                    &support::DebugSyntaxResult(self.slash_token()),
                )
                .finish()
        } else {
            f.debug_struct("SelfClosing").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SelfClosing> for SyntaxNode {
    fn from(n: SelfClosing) -> Self {
        n.syntax
    }
}
impl From<SelfClosing> for SyntaxElement {
    fn from(n: SelfClosing) -> Self {
        n.syntax.into()
    }
}
impl From<GlimmerMustacheStatement> for AnyConcatPart {
    fn from(node: GlimmerMustacheStatement) -> Self {
        Self::GlimmerMustacheStatement(node)
    }
}
impl From<GlimmerTextNode> for AnyConcatPart {
    fn from(node: GlimmerTextNode) -> Self {
        Self::GlimmerTextNode(node)
    }
}
impl AstNode for AnyConcatPart {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        GlimmerMustacheStatement::KIND_SET.union(GlimmerTextNode::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, GLIMMER_MUSTACHE_STATEMENT | GLIMMER_TEXT_NODE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GLIMMER_MUSTACHE_STATEMENT => {
                Self::GlimmerMustacheStatement(GlimmerMustacheStatement { syntax })
            }
            GLIMMER_TEXT_NODE => Self::GlimmerTextNode(GlimmerTextNode { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GlimmerMustacheStatement(it) => &it.syntax,
            Self::GlimmerTextNode(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GlimmerMustacheStatement(it) => it.syntax,
            Self::GlimmerTextNode(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyConcatPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GlimmerMustacheStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerTextNode(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyConcatPart> for SyntaxNode {
    fn from(n: AnyConcatPart) -> Self {
        match n {
            AnyConcatPart::GlimmerMustacheStatement(it) => it.into(),
            AnyConcatPart::GlimmerTextNode(it) => it.into(),
        }
    }
}
impl From<AnyConcatPart> for SyntaxElement {
    fn from(n: AnyConcatPart) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GlimmerBogusExpression> for Expression {
    fn from(node: GlimmerBogusExpression) -> Self {
        Self::GlimmerBogusExpression(node)
    }
}
impl From<GlimmerPathExpression> for Expression {
    fn from(node: GlimmerPathExpression) -> Self {
        Self::GlimmerPathExpression(node)
    }
}
impl From<GlimmerSubExpression> for Expression {
    fn from(node: GlimmerSubExpression) -> Self {
        Self::GlimmerSubExpression(node)
    }
}
impl AstNode for Expression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GlimmerBogusExpression::KIND_SET
        .union(GlimmerLiteral::KIND_SET)
        .union(GlimmerPathExpression::KIND_SET)
        .union(GlimmerSubExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            GLIMMER_BOGUS_EXPRESSION | GLIMMER_PATH_EXPRESSION | GLIMMER_SUB_EXPRESSION => true,
            k if GlimmerLiteral::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GLIMMER_BOGUS_EXPRESSION => {
                Self::GlimmerBogusExpression(GlimmerBogusExpression { syntax })
            }
            GLIMMER_PATH_EXPRESSION => {
                Self::GlimmerPathExpression(GlimmerPathExpression { syntax })
            }
            GLIMMER_SUB_EXPRESSION => Self::GlimmerSubExpression(GlimmerSubExpression { syntax }),
            _ => {
                if let Some(glimmer_literal) = GlimmerLiteral::cast(syntax) {
                    return Some(Self::GlimmerLiteral(glimmer_literal));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GlimmerBogusExpression(it) => &it.syntax,
            Self::GlimmerPathExpression(it) => &it.syntax,
            Self::GlimmerSubExpression(it) => &it.syntax,
            Self::GlimmerLiteral(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GlimmerBogusExpression(it) => it.syntax,
            Self::GlimmerPathExpression(it) => it.syntax,
            Self::GlimmerSubExpression(it) => it.syntax,
            Self::GlimmerLiteral(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GlimmerBogusExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerLiteral(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerPathExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerSubExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<Expression> for SyntaxNode {
    fn from(n: Expression) -> Self {
        match n {
            Expression::GlimmerBogusExpression(it) => it.into(),
            Expression::GlimmerLiteral(it) => it.into(),
            Expression::GlimmerPathExpression(it) => it.into(),
            Expression::GlimmerSubExpression(it) => it.into(),
        }
    }
}
impl From<Expression> for SyntaxElement {
    fn from(n: Expression) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GlimmerConcatStatement> for GlimmerAttributeValue {
    fn from(node: GlimmerConcatStatement) -> Self {
        Self::GlimmerConcatStatement(node)
    }
}
impl From<GlimmerMustacheStatement> for GlimmerAttributeValue {
    fn from(node: GlimmerMustacheStatement) -> Self {
        Self::GlimmerMustacheStatement(node)
    }
}
impl From<GlimmerTextNode> for GlimmerAttributeValue {
    fn from(node: GlimmerTextNode) -> Self {
        Self::GlimmerTextNode(node)
    }
}
impl AstNode for GlimmerAttributeValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GlimmerConcatStatement::KIND_SET
        .union(GlimmerMustacheStatement::KIND_SET)
        .union(GlimmerTextNode::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            GLIMMER_CONCAT_STATEMENT | GLIMMER_MUSTACHE_STATEMENT | GLIMMER_TEXT_NODE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GLIMMER_CONCAT_STATEMENT => {
                Self::GlimmerConcatStatement(GlimmerConcatStatement { syntax })
            }
            GLIMMER_MUSTACHE_STATEMENT => {
                Self::GlimmerMustacheStatement(GlimmerMustacheStatement { syntax })
            }
            GLIMMER_TEXT_NODE => Self::GlimmerTextNode(GlimmerTextNode { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GlimmerConcatStatement(it) => &it.syntax,
            Self::GlimmerMustacheStatement(it) => &it.syntax,
            Self::GlimmerTextNode(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GlimmerConcatStatement(it) => it.syntax,
            Self::GlimmerMustacheStatement(it) => it.syntax,
            Self::GlimmerTextNode(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for GlimmerAttributeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GlimmerConcatStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerMustacheStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerTextNode(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<GlimmerAttributeValue> for SyntaxNode {
    fn from(n: GlimmerAttributeValue) -> Self {
        match n {
            GlimmerAttributeValue::GlimmerConcatStatement(it) => it.into(),
            GlimmerAttributeValue::GlimmerMustacheStatement(it) => it.into(),
            GlimmerAttributeValue::GlimmerTextNode(it) => it.into(),
        }
    }
}
impl From<GlimmerAttributeValue> for SyntaxElement {
    fn from(n: GlimmerAttributeValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GlimmerBooleanLiteral> for GlimmerLiteral {
    fn from(node: GlimmerBooleanLiteral) -> Self {
        Self::GlimmerBooleanLiteral(node)
    }
}
impl From<GlimmerNullLiteral> for GlimmerLiteral {
    fn from(node: GlimmerNullLiteral) -> Self {
        Self::GlimmerNullLiteral(node)
    }
}
impl From<GlimmerNumberLiteral> for GlimmerLiteral {
    fn from(node: GlimmerNumberLiteral) -> Self {
        Self::GlimmerNumberLiteral(node)
    }
}
impl From<GlimmerStringLiteral> for GlimmerLiteral {
    fn from(node: GlimmerStringLiteral) -> Self {
        Self::GlimmerStringLiteral(node)
    }
}
impl From<GlimmerUndefinedLiteral> for GlimmerLiteral {
    fn from(node: GlimmerUndefinedLiteral) -> Self {
        Self::GlimmerUndefinedLiteral(node)
    }
}
impl AstNode for GlimmerLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GlimmerBooleanLiteral::KIND_SET
        .union(GlimmerNullLiteral::KIND_SET)
        .union(GlimmerNumberLiteral::KIND_SET)
        .union(GlimmerStringLiteral::KIND_SET)
        .union(GlimmerUndefinedLiteral::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            GLIMMER_BOOLEAN_LITERAL
                | GLIMMER_NULL_LITERAL
                | GLIMMER_NUMBER_LITERAL
                | GLIMMER_STRING_LITERAL
                | GLIMMER_UNDEFINED_LITERAL
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GLIMMER_BOOLEAN_LITERAL => {
                Self::GlimmerBooleanLiteral(GlimmerBooleanLiteral { syntax })
            }
            GLIMMER_NULL_LITERAL => Self::GlimmerNullLiteral(GlimmerNullLiteral { syntax }),
            GLIMMER_NUMBER_LITERAL => Self::GlimmerNumberLiteral(GlimmerNumberLiteral { syntax }),
            GLIMMER_STRING_LITERAL => Self::GlimmerStringLiteral(GlimmerStringLiteral { syntax }),
            GLIMMER_UNDEFINED_LITERAL => {
                Self::GlimmerUndefinedLiteral(GlimmerUndefinedLiteral { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GlimmerBooleanLiteral(it) => &it.syntax,
            Self::GlimmerNullLiteral(it) => &it.syntax,
            Self::GlimmerNumberLiteral(it) => &it.syntax,
            Self::GlimmerStringLiteral(it) => &it.syntax,
            Self::GlimmerUndefinedLiteral(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GlimmerBooleanLiteral(it) => it.syntax,
            Self::GlimmerNullLiteral(it) => it.syntax,
            Self::GlimmerNumberLiteral(it) => it.syntax,
            Self::GlimmerStringLiteral(it) => it.syntax,
            Self::GlimmerUndefinedLiteral(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for GlimmerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GlimmerBooleanLiteral(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerNullLiteral(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerNumberLiteral(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerStringLiteral(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerUndefinedLiteral(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<GlimmerLiteral> for SyntaxNode {
    fn from(n: GlimmerLiteral) -> Self {
        match n {
            GlimmerLiteral::GlimmerBooleanLiteral(it) => it.into(),
            GlimmerLiteral::GlimmerNullLiteral(it) => it.into(),
            GlimmerLiteral::GlimmerNumberLiteral(it) => it.into(),
            GlimmerLiteral::GlimmerStringLiteral(it) => it.into(),
            GlimmerLiteral::GlimmerUndefinedLiteral(it) => it.into(),
        }
    }
}
impl From<GlimmerLiteral> for SyntaxElement {
    fn from(n: GlimmerLiteral) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GlimmerAtHead> for PathHead {
    fn from(node: GlimmerAtHead) -> Self {
        Self::GlimmerAtHead(node)
    }
}
impl From<GlimmerThisHead> for PathHead {
    fn from(node: GlimmerThisHead) -> Self {
        Self::GlimmerThisHead(node)
    }
}
impl From<GlimmerVarHead> for PathHead {
    fn from(node: GlimmerVarHead) -> Self {
        Self::GlimmerVarHead(node)
    }
}
impl AstNode for PathHead {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GlimmerAtHead::KIND_SET
        .union(GlimmerThisHead::KIND_SET)
        .union(GlimmerVarHead::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, GLIMMER_AT_HEAD | GLIMMER_THIS_HEAD | GLIMMER_VAR_HEAD)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GLIMMER_AT_HEAD => Self::GlimmerAtHead(GlimmerAtHead { syntax }),
            GLIMMER_THIS_HEAD => Self::GlimmerThisHead(GlimmerThisHead { syntax }),
            GLIMMER_VAR_HEAD => Self::GlimmerVarHead(GlimmerVarHead { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GlimmerAtHead(it) => &it.syntax,
            Self::GlimmerThisHead(it) => &it.syntax,
            Self::GlimmerVarHead(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GlimmerAtHead(it) => it.syntax,
            Self::GlimmerThisHead(it) => it.syntax,
            Self::GlimmerVarHead(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for PathHead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GlimmerAtHead(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerThisHead(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerVarHead(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<PathHead> for SyntaxNode {
    fn from(n: PathHead) -> Self {
        match n {
            PathHead::GlimmerAtHead(it) => it.into(),
            PathHead::GlimmerThisHead(it) => it.into(),
            PathHead::GlimmerVarHead(it) => it.into(),
        }
    }
}
impl From<PathHead> for SyntaxElement {
    fn from(n: PathHead) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GlimmerBlockStatement> for Statement {
    fn from(node: GlimmerBlockStatement) -> Self {
        Self::GlimmerBlockStatement(node)
    }
}
impl From<GlimmerBogusStatement> for Statement {
    fn from(node: GlimmerBogusStatement) -> Self {
        Self::GlimmerBogusStatement(node)
    }
}
impl From<GlimmerCommentStatement> for Statement {
    fn from(node: GlimmerCommentStatement) -> Self {
        Self::GlimmerCommentStatement(node)
    }
}
impl From<GlimmerElementNode> for Statement {
    fn from(node: GlimmerElementNode) -> Self {
        Self::GlimmerElementNode(node)
    }
}
impl From<GlimmerMustacheCommentStatement> for Statement {
    fn from(node: GlimmerMustacheCommentStatement) -> Self {
        Self::GlimmerMustacheCommentStatement(node)
    }
}
impl From<GlimmerMustacheStatement> for Statement {
    fn from(node: GlimmerMustacheStatement) -> Self {
        Self::GlimmerMustacheStatement(node)
    }
}
impl From<GlimmerTextNode> for Statement {
    fn from(node: GlimmerTextNode) -> Self {
        Self::GlimmerTextNode(node)
    }
}
impl AstNode for Statement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GlimmerBlockStatement::KIND_SET
        .union(GlimmerBogusStatement::KIND_SET)
        .union(GlimmerCommentStatement::KIND_SET)
        .union(GlimmerElementNode::KIND_SET)
        .union(GlimmerMustacheCommentStatement::KIND_SET)
        .union(GlimmerMustacheStatement::KIND_SET)
        .union(GlimmerTextNode::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            GLIMMER_BLOCK_STATEMENT
                | GLIMMER_BOGUS_STATEMENT
                | GLIMMER_COMMENT_STATEMENT
                | GLIMMER_ELEMENT_NODE
                | GLIMMER_MUSTACHE_COMMENT_STATEMENT
                | GLIMMER_MUSTACHE_STATEMENT
                | GLIMMER_TEXT_NODE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GLIMMER_BLOCK_STATEMENT => {
                Self::GlimmerBlockStatement(GlimmerBlockStatement { syntax })
            }
            GLIMMER_BOGUS_STATEMENT => {
                Self::GlimmerBogusStatement(GlimmerBogusStatement { syntax })
            }
            GLIMMER_COMMENT_STATEMENT => {
                Self::GlimmerCommentStatement(GlimmerCommentStatement { syntax })
            }
            GLIMMER_ELEMENT_NODE => Self::GlimmerElementNode(GlimmerElementNode { syntax }),
            GLIMMER_MUSTACHE_COMMENT_STATEMENT => {
                Self::GlimmerMustacheCommentStatement(GlimmerMustacheCommentStatement { syntax })
            }
            GLIMMER_MUSTACHE_STATEMENT => {
                Self::GlimmerMustacheStatement(GlimmerMustacheStatement { syntax })
            }
            GLIMMER_TEXT_NODE => Self::GlimmerTextNode(GlimmerTextNode { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::GlimmerBlockStatement(it) => &it.syntax,
            Self::GlimmerBogusStatement(it) => &it.syntax,
            Self::GlimmerCommentStatement(it) => &it.syntax,
            Self::GlimmerElementNode(it) => &it.syntax,
            Self::GlimmerMustacheCommentStatement(it) => &it.syntax,
            Self::GlimmerMustacheStatement(it) => &it.syntax,
            Self::GlimmerTextNode(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::GlimmerBlockStatement(it) => it.syntax,
            Self::GlimmerBogusStatement(it) => it.syntax,
            Self::GlimmerCommentStatement(it) => it.syntax,
            Self::GlimmerElementNode(it) => it.syntax,
            Self::GlimmerMustacheCommentStatement(it) => it.syntax,
            Self::GlimmerMustacheStatement(it) => it.syntax,
            Self::GlimmerTextNode(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GlimmerBlockStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerBogusStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerCommentStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerElementNode(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerMustacheCommentStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerMustacheStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::GlimmerTextNode(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<Statement> for SyntaxNode {
    fn from(n: Statement) -> Self {
        match n {
            Statement::GlimmerBlockStatement(it) => it.into(),
            Statement::GlimmerBogusStatement(it) => it.into(),
            Statement::GlimmerCommentStatement(it) => it.into(),
            Statement::GlimmerElementNode(it) => it.into(),
            Statement::GlimmerMustacheCommentStatement(it) => it.into(),
            Statement::GlimmerMustacheStatement(it) => it.into(),
            Statement::GlimmerTextNode(it) => it.into(),
        }
    }
}
impl From<Statement> for SyntaxElement {
    fn from(n: Statement) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyConcatPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerAttributeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PathHead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerAtHead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerAttributeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerBlockParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerBlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerBooleanLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerCommentStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerConcatStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerElementModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerElementNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerElseBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerEndTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerHashPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerMustacheCommentStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerMustacheStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerNullLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerNumberLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerParamName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerPathExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerPathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerStartTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerStringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerSubExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerTextNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerThisHead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerUndefinedLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GlimmerVarHead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SelfClosing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GlimmerBogus {
    syntax: SyntaxNode,
}
impl GlimmerBogus {
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
impl AstNode for GlimmerBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_BOGUS
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
impl std::fmt::Debug for GlimmerBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlimmerBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GlimmerBogus> for SyntaxNode {
    fn from(n: GlimmerBogus) -> Self {
        n.syntax
    }
}
impl From<GlimmerBogus> for SyntaxElement {
    fn from(n: GlimmerBogus) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GlimmerBogusExpression {
    syntax: SyntaxNode,
}
impl GlimmerBogusExpression {
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
impl AstNode for GlimmerBogusExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_BOGUS_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_BOGUS_EXPRESSION
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
impl std::fmt::Debug for GlimmerBogusExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlimmerBogusExpression")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GlimmerBogusExpression> for SyntaxNode {
    fn from(n: GlimmerBogusExpression) -> Self {
        n.syntax
    }
}
impl From<GlimmerBogusExpression> for SyntaxElement {
    fn from(n: GlimmerBogusExpression) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GlimmerBogusStatement {
    syntax: SyntaxNode,
}
impl GlimmerBogusStatement {
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
impl AstNode for GlimmerBogusStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_BOGUS_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_BOGUS_STATEMENT
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
impl std::fmt::Debug for GlimmerBogusStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlimmerBogusStatement")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GlimmerBogusStatement> for SyntaxNode {
    fn from(n: GlimmerBogusStatement) -> Self {
        n.syntax
    }
}
impl From<GlimmerBogusStatement> for SyntaxElement {
    fn from(n: GlimmerBogusStatement) -> Self {
        n.syntax.into()
    }
}
biome_rowan::declare_node_union! { pub AnyGlimmerBogusNode = GlimmerBogus | GlimmerBogusExpression | GlimmerBogusStatement }
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GlimmerAttributeList {
    syntax_list: SyntaxList,
}
impl GlimmerAttributeList {
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
impl AstNode for GlimmerAttributeList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_ATTRIBUTE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_ATTRIBUTE_LIST
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
impl Serialize for GlimmerAttributeList {
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
impl AstNodeList for GlimmerAttributeList {
    type Language = Language;
    type Node = GlimmerAttributeNode;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GlimmerAttributeList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GlimmerAttributeList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GlimmerAttributeList {
    type Item = GlimmerAttributeNode;
    type IntoIter = AstNodeListIterator<Language, GlimmerAttributeNode>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GlimmerAttributeList {
    type Item = GlimmerAttributeNode;
    type IntoIter = AstNodeListIterator<Language, GlimmerAttributeNode>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GlimmerConcatPartList {
    syntax_list: SyntaxList,
}
impl GlimmerConcatPartList {
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
impl AstNode for GlimmerConcatPartList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_CONCAT_PART_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_CONCAT_PART_LIST
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
impl Serialize for GlimmerConcatPartList {
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
impl AstNodeList for GlimmerConcatPartList {
    type Language = Language;
    type Node = AnyConcatPart;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GlimmerConcatPartList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GlimmerConcatPartList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GlimmerConcatPartList {
    type Item = AnyConcatPart;
    type IntoIter = AstNodeListIterator<Language, AnyConcatPart>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GlimmerConcatPartList {
    type Item = AnyConcatPart;
    type IntoIter = AstNodeListIterator<Language, AnyConcatPart>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GlimmerElementModifierList {
    syntax_list: SyntaxList,
}
impl GlimmerElementModifierList {
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
impl AstNode for GlimmerElementModifierList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_ELEMENT_MODIFIER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_ELEMENT_MODIFIER_LIST
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
impl Serialize for GlimmerElementModifierList {
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
impl AstNodeList for GlimmerElementModifierList {
    type Language = Language;
    type Node = GlimmerElementModifier;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GlimmerElementModifierList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GlimmerElementModifierList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GlimmerElementModifierList {
    type Item = GlimmerElementModifier;
    type IntoIter = AstNodeListIterator<Language, GlimmerElementModifier>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GlimmerElementModifierList {
    type Item = GlimmerElementModifier;
    type IntoIter = AstNodeListIterator<Language, GlimmerElementModifier>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GlimmerHashPairList {
    syntax_list: SyntaxList,
}
impl GlimmerHashPairList {
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
impl AstNode for GlimmerHashPairList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_HASH_PAIR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_HASH_PAIR_LIST
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
impl Serialize for GlimmerHashPairList {
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
impl AstNodeList for GlimmerHashPairList {
    type Language = Language;
    type Node = GlimmerHashPair;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GlimmerHashPairList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GlimmerHashPairList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GlimmerHashPairList {
    type Item = GlimmerHashPair;
    type IntoIter = AstNodeListIterator<Language, GlimmerHashPair>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GlimmerHashPairList {
    type Item = GlimmerHashPair;
    type IntoIter = AstNodeListIterator<Language, GlimmerHashPair>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GlimmerParamNameList {
    syntax_list: SyntaxList,
}
impl GlimmerParamNameList {
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
impl AstNode for GlimmerParamNameList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_PARAM_NAME_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_PARAM_NAME_LIST
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
impl Serialize for GlimmerParamNameList {
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
impl AstNodeList for GlimmerParamNameList {
    type Language = Language;
    type Node = GlimmerParamName;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GlimmerParamNameList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GlimmerParamNameList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GlimmerParamNameList {
    type Item = GlimmerParamName;
    type IntoIter = AstNodeListIterator<Language, GlimmerParamName>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GlimmerParamNameList {
    type Item = GlimmerParamName;
    type IntoIter = AstNodeListIterator<Language, GlimmerParamName>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GlimmerParamsList {
    syntax_list: SyntaxList,
}
impl GlimmerParamsList {
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
impl AstNode for GlimmerParamsList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_PARAMS_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_PARAMS_LIST
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
impl Serialize for GlimmerParamsList {
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
impl AstNodeList for GlimmerParamsList {
    type Language = Language;
    type Node = Expression;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GlimmerParamsList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GlimmerParamsList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GlimmerParamsList {
    type Item = Expression;
    type IntoIter = AstNodeListIterator<Language, Expression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GlimmerParamsList {
    type Item = Expression;
    type IntoIter = AstNodeListIterator<Language, Expression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GlimmerPathSegmentList {
    syntax_list: SyntaxList,
}
impl GlimmerPathSegmentList {
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
impl AstNode for GlimmerPathSegmentList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_PATH_SEGMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_PATH_SEGMENT_LIST
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
impl Serialize for GlimmerPathSegmentList {
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
impl AstNodeList for GlimmerPathSegmentList {
    type Language = Language;
    type Node = GlimmerPathSegment;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GlimmerPathSegmentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GlimmerPathSegmentList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GlimmerPathSegmentList {
    type Item = GlimmerPathSegment;
    type IntoIter = AstNodeListIterator<Language, GlimmerPathSegment>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GlimmerPathSegmentList {
    type Item = GlimmerPathSegment;
    type IntoIter = AstNodeListIterator<Language, GlimmerPathSegment>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GlimmerStatementList {
    syntax_list: SyntaxList,
}
impl GlimmerStatementList {
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
impl AstNode for GlimmerStatementList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLIMMER_STATEMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GLIMMER_STATEMENT_LIST
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
impl Serialize for GlimmerStatementList {
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
impl AstNodeList for GlimmerStatementList {
    type Language = Language;
    type Node = Statement;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GlimmerStatementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GlimmerStatementList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GlimmerStatementList {
    type Item = Statement;
    type IntoIter = AstNodeListIterator<Language, Statement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GlimmerStatementList {
    type Item = Statement;
    type IntoIter = AstNodeListIterator<Language, Statement>;
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
