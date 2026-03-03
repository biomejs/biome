//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    MarkdownLanguage as Language, MarkdownSyntaxElement as SyntaxElement,
    MarkdownSyntaxElementChildren as SyntaxElementChildren,
    MarkdownSyntaxKind::{self as SyntaxKind, *},
    MarkdownSyntaxList as SyntaxList, MarkdownSyntaxNode as SyntaxNode,
    MarkdownSyntaxToken as SyntaxToken,
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
pub struct MdAutolink {
    pub(crate) syntax: SyntaxNode,
}
impl MdAutolink {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdAutolinkFields {
        MdAutolinkFields {
            l_angle_token: self.l_angle_token(),
            value: self.value(),
            r_angle_token: self.r_angle_token(),
        }
    }
    pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> MdInlineItemList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MdAutolink {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdAutolinkFields {
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub value: MdInlineItemList,
    pub r_angle_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdBullet {
    pub(crate) syntax: SyntaxNode,
}
impl MdBullet {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdBulletFields {
        MdBulletFields {
            bullet: self.bullet(),
            content: self.content(),
        }
    }
    pub fn bullet(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn content(&self) -> MdBlockList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for MdBullet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdBulletFields {
    pub bullet: SyntaxResult<SyntaxToken>,
    pub content: MdBlockList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdBulletListItem {
    pub(crate) syntax: SyntaxNode,
}
impl MdBulletListItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdBulletListItemFields {
        MdBulletListItemFields {
            md_bullet_list: self.md_bullet_list(),
        }
    }
    pub fn md_bullet_list(&self) -> MdBulletList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for MdBulletListItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdBulletListItemFields {
    pub md_bullet_list: MdBulletList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdDocument {
    pub(crate) syntax: SyntaxNode,
}
impl MdDocument {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdDocumentFields {
        MdDocumentFields {
            bom_token: self.bom_token(),
            value: self.value(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> MdBlockList {
        support::list(&self.syntax, 1usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MdDocument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdDocumentFields {
    pub bom_token: Option<SyntaxToken>,
    pub value: MdBlockList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdEntityReference {
    pub(crate) syntax: SyntaxNode,
}
impl MdEntityReference {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdEntityReferenceFields {
        MdEntityReferenceFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MdEntityReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdEntityReferenceFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdFencedCodeBlock {
    pub(crate) syntax: SyntaxNode,
}
impl MdFencedCodeBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdFencedCodeBlockFields {
        MdFencedCodeBlockFields {
            l_fence: self.l_fence(),
            code_list: self.code_list(),
            content: self.content(),
            r_fence: self.r_fence(),
        }
    }
    pub fn l_fence(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn code_list(&self) -> MdCodeNameList {
        support::list(&self.syntax, 1usize)
    }
    pub fn content(&self) -> MdInlineItemList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_fence(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for MdFencedCodeBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdFencedCodeBlockFields {
    pub l_fence: SyntaxResult<SyntaxToken>,
    pub code_list: MdCodeNameList,
    pub content: MdInlineItemList,
    pub r_fence: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdHardLine {
    pub(crate) syntax: SyntaxNode,
}
impl MdHardLine {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdHardLineFields {
        MdHardLineFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MdHardLine {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdHardLineFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdHash {
    pub(crate) syntax: SyntaxNode,
}
impl MdHash {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdHashFields {
        MdHashFields {
            hash_token: self.hash_token(),
        }
    }
    pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MdHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdHashFields {
    pub hash_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdHeader {
    pub(crate) syntax: SyntaxNode,
}
impl MdHeader {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdHeaderFields {
        MdHeaderFields {
            before: self.before(),
            content: self.content(),
            after: self.after(),
        }
    }
    pub fn before(&self) -> MdHashList {
        support::list(&self.syntax, 0usize)
    }
    pub fn content(&self) -> Option<MdParagraph> {
        support::node(&self.syntax, 1usize)
    }
    pub fn after(&self) -> MdHashList {
        support::list(&self.syntax, 2usize)
    }
}
impl Serialize for MdHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdHeaderFields {
    pub before: MdHashList,
    pub content: Option<MdParagraph>,
    pub after: MdHashList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdHtmlBlock {
    pub(crate) syntax: SyntaxNode,
}
impl MdHtmlBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdHtmlBlockFields {
        MdHtmlBlockFields {
            content: self.content(),
        }
    }
    pub fn content(&self) -> MdInlineItemList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for MdHtmlBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdHtmlBlockFields {
    pub content: MdInlineItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdIndent {
    pub(crate) syntax: SyntaxNode,
}
impl MdIndent {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdIndentFields {
        MdIndentFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MdIndent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdIndentFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdIndentCodeBlock {
    pub(crate) syntax: SyntaxNode,
}
impl MdIndentCodeBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdIndentCodeBlockFields {
        MdIndentCodeBlockFields {
            content: self.content(),
        }
    }
    pub fn content(&self) -> MdInlineItemList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for MdIndentCodeBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdIndentCodeBlockFields {
    pub content: MdInlineItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdInlineCode {
    pub(crate) syntax: SyntaxNode,
}
impl MdInlineCode {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdInlineCodeFields {
        MdInlineCodeFields {
            l_tick_token: self.l_tick_token(),
            content: self.content(),
            r_tick_token: self.r_tick_token(),
        }
    }
    pub fn l_tick_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn content(&self) -> MdInlineItemList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_tick_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MdInlineCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdInlineCodeFields {
    pub l_tick_token: SyntaxResult<SyntaxToken>,
    pub content: MdInlineItemList,
    pub r_tick_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdInlineEmphasis {
    pub(crate) syntax: SyntaxNode,
}
impl MdInlineEmphasis {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdInlineEmphasisFields {
        MdInlineEmphasisFields {
            l_fence: self.l_fence(),
            content: self.content(),
            r_fence: self.r_fence(),
        }
    }
    pub fn l_fence(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn content(&self) -> MdInlineItemList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_fence(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MdInlineEmphasis {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdInlineEmphasisFields {
    pub l_fence: SyntaxResult<SyntaxToken>,
    pub content: MdInlineItemList,
    pub r_fence: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdInlineHtml {
    pub(crate) syntax: SyntaxNode,
}
impl MdInlineHtml {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdInlineHtmlFields {
        MdInlineHtmlFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> MdInlineItemList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for MdInlineHtml {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdInlineHtmlFields {
    pub value: MdInlineItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdInlineImage {
    pub(crate) syntax: SyntaxNode,
}
impl MdInlineImage {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdInlineImageFields {
        MdInlineImageFields {
            excl_token: self.excl_token(),
            l_brack_token: self.l_brack_token(),
            alt: self.alt(),
            r_brack_token: self.r_brack_token(),
            l_paren_token: self.l_paren_token(),
            destination: self.destination(),
            title: self.title(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn alt(&self) -> MdInlineItemList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn destination(&self) -> MdInlineItemList {
        support::list(&self.syntax, 5usize)
    }
    pub fn title(&self) -> Option<MdLinkTitle> {
        support::node(&self.syntax, 6usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 7usize)
    }
}
impl Serialize for MdInlineImage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdInlineImageFields {
    pub excl_token: SyntaxResult<SyntaxToken>,
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub alt: MdInlineItemList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub destination: MdInlineItemList,
    pub title: Option<MdLinkTitle>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdInlineItalic {
    pub(crate) syntax: SyntaxNode,
}
impl MdInlineItalic {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdInlineItalicFields {
        MdInlineItalicFields {
            l_fence: self.l_fence(),
            content: self.content(),
            r_fence: self.r_fence(),
        }
    }
    pub fn l_fence(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn content(&self) -> MdInlineItemList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_fence(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MdInlineItalic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdInlineItalicFields {
    pub l_fence: SyntaxResult<SyntaxToken>,
    pub content: MdInlineItemList,
    pub r_fence: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdInlineLink {
    pub(crate) syntax: SyntaxNode,
}
impl MdInlineLink {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdInlineLinkFields {
        MdInlineLinkFields {
            l_brack_token: self.l_brack_token(),
            text: self.text(),
            r_brack_token: self.r_brack_token(),
            l_paren_token: self.l_paren_token(),
            destination: self.destination(),
            title: self.title(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn text(&self) -> MdInlineItemList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn destination(&self) -> MdInlineItemList {
        support::list(&self.syntax, 4usize)
    }
    pub fn title(&self) -> Option<MdLinkTitle> {
        support::node(&self.syntax, 5usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 6usize)
    }
}
impl Serialize for MdInlineLink {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdInlineLinkFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub text: MdInlineItemList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub destination: MdInlineItemList,
    pub title: Option<MdLinkTitle>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdLinkBlock {
    pub(crate) syntax: SyntaxNode,
}
impl MdLinkBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdLinkBlockFields {
        MdLinkBlockFields {
            label: self.label(),
            url: self.url(),
            title: self.title(),
        }
    }
    pub fn label(&self) -> SyntaxResult<MdTextual> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn url(&self) -> SyntaxResult<MdTextual> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn title(&self) -> Option<MdTextual> {
        support::node(&self.syntax, 2usize)
    }
}
impl Serialize for MdLinkBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdLinkBlockFields {
    pub label: SyntaxResult<MdTextual>,
    pub url: SyntaxResult<MdTextual>,
    pub title: Option<MdTextual>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdLinkDestination {
    pub(crate) syntax: SyntaxNode,
}
impl MdLinkDestination {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdLinkDestinationFields {
        MdLinkDestinationFields {
            content: self.content(),
        }
    }
    pub fn content(&self) -> MdInlineItemList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for MdLinkDestination {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdLinkDestinationFields {
    pub content: MdInlineItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdLinkLabel {
    pub(crate) syntax: SyntaxNode,
}
impl MdLinkLabel {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdLinkLabelFields {
        MdLinkLabelFields {
            content: self.content(),
        }
    }
    pub fn content(&self) -> MdInlineItemList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for MdLinkLabel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdLinkLabelFields {
    pub content: MdInlineItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdLinkReferenceDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl MdLinkReferenceDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdLinkReferenceDefinitionFields {
        MdLinkReferenceDefinitionFields {
            l_brack_token: self.l_brack_token(),
            label: self.label(),
            r_brack_token: self.r_brack_token(),
            colon_token: self.colon_token(),
            destination: self.destination(),
            title: self.title(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn label(&self) -> SyntaxResult<MdLinkLabel> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn destination(&self) -> SyntaxResult<MdLinkDestination> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn title(&self) -> Option<MdLinkTitle> {
        support::node(&self.syntax, 5usize)
    }
}
impl Serialize for MdLinkReferenceDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdLinkReferenceDefinitionFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub label: SyntaxResult<MdLinkLabel>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub destination: SyntaxResult<MdLinkDestination>,
    pub title: Option<MdLinkTitle>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdLinkTitle {
    pub(crate) syntax: SyntaxNode,
}
impl MdLinkTitle {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdLinkTitleFields {
        MdLinkTitleFields {
            content: self.content(),
        }
    }
    pub fn content(&self) -> MdInlineItemList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for MdLinkTitle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdLinkTitleFields {
    pub content: MdInlineItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdNewline {
    pub(crate) syntax: SyntaxNode,
}
impl MdNewline {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdNewlineFields {
        MdNewlineFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MdNewline {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdNewlineFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdOrderedListItem {
    pub(crate) syntax: SyntaxNode,
}
impl MdOrderedListItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdOrderedListItemFields {
        MdOrderedListItemFields {
            md_bullet_list: self.md_bullet_list(),
        }
    }
    pub fn md_bullet_list(&self) -> MdBulletList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for MdOrderedListItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdOrderedListItemFields {
    pub md_bullet_list: MdBulletList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdParagraph {
    pub(crate) syntax: SyntaxNode,
}
impl MdParagraph {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdParagraphFields {
        MdParagraphFields {
            list: self.list(),
            hard_line: self.hard_line(),
        }
    }
    pub fn list(&self) -> MdInlineItemList {
        support::list(&self.syntax, 0usize)
    }
    pub fn hard_line(&self) -> Option<MdHardLine> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for MdParagraph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdParagraphFields {
    pub list: MdInlineItemList,
    pub hard_line: Option<MdHardLine>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdQuote {
    pub(crate) syntax: SyntaxNode,
}
impl MdQuote {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdQuoteFields {
        MdQuoteFields {
            prefix: self.prefix(),
            content: self.content(),
        }
    }
    pub fn prefix(&self) -> SyntaxResult<MdQuotePrefix> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn content(&self) -> MdBlockList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for MdQuote {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdQuoteFields {
    pub prefix: SyntaxResult<MdQuotePrefix>,
    pub content: MdBlockList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdQuoteIndent {
    pub(crate) syntax: SyntaxNode,
}
impl MdQuoteIndent {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdQuoteIndentFields {
        MdQuoteIndentFields {
            md_quote_pre_marker_indent_token: self.md_quote_pre_marker_indent_token(),
        }
    }
    pub fn md_quote_pre_marker_indent_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MdQuoteIndent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdQuoteIndentFields {
    pub md_quote_pre_marker_indent_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdQuotePrefix {
    pub(crate) syntax: SyntaxNode,
}
impl MdQuotePrefix {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdQuotePrefixFields {
        MdQuotePrefixFields {
            pre_marker_indent: self.pre_marker_indent(),
            marker_token: self.marker_token(),
            post_marker_space_token: self.post_marker_space_token(),
        }
    }
    pub fn pre_marker_indent(&self) -> MdQuoteIndentList {
        support::list(&self.syntax, 0usize)
    }
    pub fn marker_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn post_marker_space_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
}
impl Serialize for MdQuotePrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdQuotePrefixFields {
    pub pre_marker_indent: MdQuoteIndentList,
    pub marker_token: SyntaxResult<SyntaxToken>,
    pub post_marker_space_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdReferenceImage {
    pub(crate) syntax: SyntaxNode,
}
impl MdReferenceImage {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdReferenceImageFields {
        MdReferenceImageFields {
            excl_token: self.excl_token(),
            l_brack_token: self.l_brack_token(),
            alt: self.alt(),
            r_brack_token: self.r_brack_token(),
            label: self.label(),
        }
    }
    pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn alt(&self) -> MdInlineItemList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn label(&self) -> Option<MdReferenceLinkLabel> {
        support::node(&self.syntax, 4usize)
    }
}
impl Serialize for MdReferenceImage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdReferenceImageFields {
    pub excl_token: SyntaxResult<SyntaxToken>,
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub alt: MdInlineItemList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
    pub label: Option<MdReferenceLinkLabel>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdReferenceLink {
    pub(crate) syntax: SyntaxNode,
}
impl MdReferenceLink {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdReferenceLinkFields {
        MdReferenceLinkFields {
            l_brack_token: self.l_brack_token(),
            text: self.text(),
            r_brack_token: self.r_brack_token(),
            label: self.label(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn text(&self) -> MdInlineItemList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn label(&self) -> Option<MdReferenceLinkLabel> {
        support::node(&self.syntax, 3usize)
    }
}
impl Serialize for MdReferenceLink {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdReferenceLinkFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub text: MdInlineItemList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
    pub label: Option<MdReferenceLinkLabel>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdReferenceLinkLabel {
    pub(crate) syntax: SyntaxNode,
}
impl MdReferenceLinkLabel {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdReferenceLinkLabelFields {
        MdReferenceLinkLabelFields {
            l_brack_token: self.l_brack_token(),
            label: self.label(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn label(&self) -> MdInlineItemList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MdReferenceLinkLabel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdReferenceLinkLabelFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub label: MdInlineItemList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdSetextHeader {
    pub(crate) syntax: SyntaxNode,
}
impl MdSetextHeader {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdSetextHeaderFields {
        MdSetextHeaderFields {
            content: self.content(),
            underline_token: self.underline_token(),
        }
    }
    pub fn content(&self) -> MdInlineItemList {
        support::list(&self.syntax, 0usize)
    }
    pub fn underline_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for MdSetextHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdSetextHeaderFields {
    pub content: MdInlineItemList,
    pub underline_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdSoftBreak {
    pub(crate) syntax: SyntaxNode,
}
impl MdSoftBreak {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdSoftBreakFields {
        MdSoftBreakFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MdSoftBreak {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdSoftBreakFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdTextual {
    pub(crate) syntax: SyntaxNode,
}
impl MdTextual {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdTextualFields {
        MdTextualFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MdTextual {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdTextualFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdThematicBreakBlock {
    pub(crate) syntax: SyntaxNode,
}
impl MdThematicBreakBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdThematicBreakBlockFields {
        MdThematicBreakBlockFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MdThematicBreakBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdThematicBreakBlockFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMdBlock {
    AnyMdContainerBlock(AnyMdContainerBlock),
    AnyMdLeafBlock(AnyMdLeafBlock),
    MdQuotePrefix(MdQuotePrefix),
}
impl AnyMdBlock {
    pub fn as_any_md_container_block(&self) -> Option<&AnyMdContainerBlock> {
        match &self {
            Self::AnyMdContainerBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_any_md_leaf_block(&self) -> Option<&AnyMdLeafBlock> {
        match &self {
            Self::AnyMdLeafBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_quote_prefix(&self) -> Option<&MdQuotePrefix> {
        match &self {
            Self::MdQuotePrefix(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMdCodeBlock {
    MdFencedCodeBlock(MdFencedCodeBlock),
    MdIndentCodeBlock(MdIndentCodeBlock),
}
impl AnyMdCodeBlock {
    pub fn as_md_fenced_code_block(&self) -> Option<&MdFencedCodeBlock> {
        match &self {
            Self::MdFencedCodeBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_indent_code_block(&self) -> Option<&MdIndentCodeBlock> {
        match &self {
            Self::MdIndentCodeBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMdContainerBlock {
    MdBulletListItem(MdBulletListItem),
    MdOrderedListItem(MdOrderedListItem),
    MdQuote(MdQuote),
}
impl AnyMdContainerBlock {
    pub fn as_md_bullet_list_item(&self) -> Option<&MdBulletListItem> {
        match &self {
            Self::MdBulletListItem(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_ordered_list_item(&self) -> Option<&MdOrderedListItem> {
        match &self {
            Self::MdOrderedListItem(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_quote(&self) -> Option<&MdQuote> {
        match &self {
            Self::MdQuote(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMdInline {
    MdAutolink(MdAutolink),
    MdEntityReference(MdEntityReference),
    MdHardLine(MdHardLine),
    MdHtmlBlock(MdHtmlBlock),
    MdInlineCode(MdInlineCode),
    MdInlineEmphasis(MdInlineEmphasis),
    MdInlineHtml(MdInlineHtml),
    MdInlineImage(MdInlineImage),
    MdInlineItalic(MdInlineItalic),
    MdInlineLink(MdInlineLink),
    MdQuotePrefix(MdQuotePrefix),
    MdReferenceImage(MdReferenceImage),
    MdReferenceLink(MdReferenceLink),
    MdSoftBreak(MdSoftBreak),
    MdTextual(MdTextual),
}
impl AnyMdInline {
    pub fn as_md_autolink(&self) -> Option<&MdAutolink> {
        match &self {
            Self::MdAutolink(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_entity_reference(&self) -> Option<&MdEntityReference> {
        match &self {
            Self::MdEntityReference(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_hard_line(&self) -> Option<&MdHardLine> {
        match &self {
            Self::MdHardLine(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_html_block(&self) -> Option<&MdHtmlBlock> {
        match &self {
            Self::MdHtmlBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_inline_code(&self) -> Option<&MdInlineCode> {
        match &self {
            Self::MdInlineCode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_inline_emphasis(&self) -> Option<&MdInlineEmphasis> {
        match &self {
            Self::MdInlineEmphasis(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_inline_html(&self) -> Option<&MdInlineHtml> {
        match &self {
            Self::MdInlineHtml(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_inline_image(&self) -> Option<&MdInlineImage> {
        match &self {
            Self::MdInlineImage(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_inline_italic(&self) -> Option<&MdInlineItalic> {
        match &self {
            Self::MdInlineItalic(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_inline_link(&self) -> Option<&MdInlineLink> {
        match &self {
            Self::MdInlineLink(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_quote_prefix(&self) -> Option<&MdQuotePrefix> {
        match &self {
            Self::MdQuotePrefix(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_reference_image(&self) -> Option<&MdReferenceImage> {
        match &self {
            Self::MdReferenceImage(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_reference_link(&self) -> Option<&MdReferenceLink> {
        match &self {
            Self::MdReferenceLink(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_soft_break(&self) -> Option<&MdSoftBreak> {
        match &self {
            Self::MdSoftBreak(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_textual(&self) -> Option<&MdTextual> {
        match &self {
            Self::MdTextual(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMdLeafBlock {
    AnyMdCodeBlock(AnyMdCodeBlock),
    MdHeader(MdHeader),
    MdHtmlBlock(MdHtmlBlock),
    MdLinkBlock(MdLinkBlock),
    MdLinkReferenceDefinition(MdLinkReferenceDefinition),
    MdNewline(MdNewline),
    MdParagraph(MdParagraph),
    MdSetextHeader(MdSetextHeader),
    MdThematicBreakBlock(MdThematicBreakBlock),
}
impl AnyMdLeafBlock {
    pub fn as_any_md_code_block(&self) -> Option<&AnyMdCodeBlock> {
        match &self {
            Self::AnyMdCodeBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_header(&self) -> Option<&MdHeader> {
        match &self {
            Self::MdHeader(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_html_block(&self) -> Option<&MdHtmlBlock> {
        match &self {
            Self::MdHtmlBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_link_block(&self) -> Option<&MdLinkBlock> {
        match &self {
            Self::MdLinkBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_link_reference_definition(&self) -> Option<&MdLinkReferenceDefinition> {
        match &self {
            Self::MdLinkReferenceDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_newline(&self) -> Option<&MdNewline> {
        match &self {
            Self::MdNewline(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_paragraph(&self) -> Option<&MdParagraph> {
        match &self {
            Self::MdParagraph(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_setext_header(&self) -> Option<&MdSetextHeader> {
        match &self {
            Self::MdSetextHeader(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_thematic_break_block(&self) -> Option<&MdThematicBreakBlock> {
        match &self {
            Self::MdThematicBreakBlock(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for MdAutolink {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_AUTOLINK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_AUTOLINK
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
impl std::fmt::Debug for MdAutolink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdAutolink")
                .field(
                    "l_angle_token",
                    &support::DebugSyntaxResult(self.l_angle_token()),
                )
                .field("value", &self.value())
                .field(
                    "r_angle_token",
                    &support::DebugSyntaxResult(self.r_angle_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdAutolink").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdAutolink> for SyntaxNode {
    fn from(n: MdAutolink) -> Self {
        n.syntax
    }
}
impl From<MdAutolink> for SyntaxElement {
    fn from(n: MdAutolink) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdBullet {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_BULLET as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_BULLET
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
impl std::fmt::Debug for MdBullet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdBullet")
                .field("bullet", &support::DebugSyntaxResult(self.bullet()))
                .field("content", &self.content())
                .finish()
        } else {
            f.debug_struct("MdBullet").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdBullet> for SyntaxNode {
    fn from(n: MdBullet) -> Self {
        n.syntax
    }
}
impl From<MdBullet> for SyntaxElement {
    fn from(n: MdBullet) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdBulletListItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_BULLET_LIST_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_BULLET_LIST_ITEM
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
impl std::fmt::Debug for MdBulletListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdBulletListItem")
                .field("md_bullet_list", &self.md_bullet_list())
                .finish()
        } else {
            f.debug_struct("MdBulletListItem").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdBulletListItem> for SyntaxNode {
    fn from(n: MdBulletListItem) -> Self {
        n.syntax
    }
}
impl From<MdBulletListItem> for SyntaxElement {
    fn from(n: MdBulletListItem) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdDocument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_DOCUMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_DOCUMENT
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
impl std::fmt::Debug for MdDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdDocument")
                .field(
                    "bom_token",
                    &support::DebugOptionalElement(self.bom_token()),
                )
                .field("value", &self.value())
                .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
                .finish()
        } else {
            f.debug_struct("MdDocument").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdDocument> for SyntaxNode {
    fn from(n: MdDocument) -> Self {
        n.syntax
    }
}
impl From<MdDocument> for SyntaxElement {
    fn from(n: MdDocument) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdEntityReference {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_ENTITY_REFERENCE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_ENTITY_REFERENCE
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
impl std::fmt::Debug for MdEntityReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdEntityReference")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdEntityReference").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdEntityReference> for SyntaxNode {
    fn from(n: MdEntityReference) -> Self {
        n.syntax
    }
}
impl From<MdEntityReference> for SyntaxElement {
    fn from(n: MdEntityReference) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdFencedCodeBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_FENCED_CODE_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_FENCED_CODE_BLOCK
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
impl std::fmt::Debug for MdFencedCodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdFencedCodeBlock")
                .field("l_fence", &support::DebugSyntaxResult(self.l_fence()))
                .field("code_list", &self.code_list())
                .field("content", &self.content())
                .field("r_fence", &support::DebugSyntaxResult(self.r_fence()))
                .finish()
        } else {
            f.debug_struct("MdFencedCodeBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdFencedCodeBlock> for SyntaxNode {
    fn from(n: MdFencedCodeBlock) -> Self {
        n.syntax
    }
}
impl From<MdFencedCodeBlock> for SyntaxElement {
    fn from(n: MdFencedCodeBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdHardLine {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_HARD_LINE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_HARD_LINE
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
impl std::fmt::Debug for MdHardLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdHardLine")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdHardLine").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdHardLine> for SyntaxNode {
    fn from(n: MdHardLine) -> Self {
        n.syntax
    }
}
impl From<MdHardLine> for SyntaxElement {
    fn from(n: MdHardLine) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdHash {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_HASH as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_HASH
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
impl std::fmt::Debug for MdHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdHash")
                .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
                .finish()
        } else {
            f.debug_struct("MdHash").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdHash> for SyntaxNode {
    fn from(n: MdHash) -> Self {
        n.syntax
    }
}
impl From<MdHash> for SyntaxElement {
    fn from(n: MdHash) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdHeader {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_HEADER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_HEADER
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
impl std::fmt::Debug for MdHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdHeader")
                .field("before", &self.before())
                .field("content", &support::DebugOptionalElement(self.content()))
                .field("after", &self.after())
                .finish()
        } else {
            f.debug_struct("MdHeader").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdHeader> for SyntaxNode {
    fn from(n: MdHeader) -> Self {
        n.syntax
    }
}
impl From<MdHeader> for SyntaxElement {
    fn from(n: MdHeader) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdHtmlBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_HTML_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_HTML_BLOCK
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
impl std::fmt::Debug for MdHtmlBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdHtmlBlock")
                .field("content", &self.content())
                .finish()
        } else {
            f.debug_struct("MdHtmlBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdHtmlBlock> for SyntaxNode {
    fn from(n: MdHtmlBlock) -> Self {
        n.syntax
    }
}
impl From<MdHtmlBlock> for SyntaxElement {
    fn from(n: MdHtmlBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdIndent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_INDENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_INDENT
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
impl std::fmt::Debug for MdIndent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdIndent")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdIndent").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdIndent> for SyntaxNode {
    fn from(n: MdIndent) -> Self {
        n.syntax
    }
}
impl From<MdIndent> for SyntaxElement {
    fn from(n: MdIndent) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdIndentCodeBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_INDENT_CODE_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_INDENT_CODE_BLOCK
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
impl std::fmt::Debug for MdIndentCodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdIndentCodeBlock")
                .field("content", &self.content())
                .finish()
        } else {
            f.debug_struct("MdIndentCodeBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdIndentCodeBlock> for SyntaxNode {
    fn from(n: MdIndentCodeBlock) -> Self {
        n.syntax
    }
}
impl From<MdIndentCodeBlock> for SyntaxElement {
    fn from(n: MdIndentCodeBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdInlineCode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_INLINE_CODE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_INLINE_CODE
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
impl std::fmt::Debug for MdInlineCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdInlineCode")
                .field(
                    "l_tick_token",
                    &support::DebugSyntaxResult(self.l_tick_token()),
                )
                .field("content", &self.content())
                .field(
                    "r_tick_token",
                    &support::DebugSyntaxResult(self.r_tick_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdInlineCode").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdInlineCode> for SyntaxNode {
    fn from(n: MdInlineCode) -> Self {
        n.syntax
    }
}
impl From<MdInlineCode> for SyntaxElement {
    fn from(n: MdInlineCode) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdInlineEmphasis {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_INLINE_EMPHASIS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_INLINE_EMPHASIS
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
impl std::fmt::Debug for MdInlineEmphasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdInlineEmphasis")
                .field("l_fence", &support::DebugSyntaxResult(self.l_fence()))
                .field("content", &self.content())
                .field("r_fence", &support::DebugSyntaxResult(self.r_fence()))
                .finish()
        } else {
            f.debug_struct("MdInlineEmphasis").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdInlineEmphasis> for SyntaxNode {
    fn from(n: MdInlineEmphasis) -> Self {
        n.syntax
    }
}
impl From<MdInlineEmphasis> for SyntaxElement {
    fn from(n: MdInlineEmphasis) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdInlineHtml {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_INLINE_HTML as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_INLINE_HTML
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
impl std::fmt::Debug for MdInlineHtml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdInlineHtml")
                .field("value", &self.value())
                .finish()
        } else {
            f.debug_struct("MdInlineHtml").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdInlineHtml> for SyntaxNode {
    fn from(n: MdInlineHtml) -> Self {
        n.syntax
    }
}
impl From<MdInlineHtml> for SyntaxElement {
    fn from(n: MdInlineHtml) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdInlineImage {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_INLINE_IMAGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_INLINE_IMAGE
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
impl std::fmt::Debug for MdInlineImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdInlineImage")
                .field("excl_token", &support::DebugSyntaxResult(self.excl_token()))
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("alt", &self.alt())
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("destination", &self.destination())
                .field("title", &support::DebugOptionalElement(self.title()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdInlineImage").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdInlineImage> for SyntaxNode {
    fn from(n: MdInlineImage) -> Self {
        n.syntax
    }
}
impl From<MdInlineImage> for SyntaxElement {
    fn from(n: MdInlineImage) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdInlineItalic {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_INLINE_ITALIC as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_INLINE_ITALIC
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
impl std::fmt::Debug for MdInlineItalic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdInlineItalic")
                .field("l_fence", &support::DebugSyntaxResult(self.l_fence()))
                .field("content", &self.content())
                .field("r_fence", &support::DebugSyntaxResult(self.r_fence()))
                .finish()
        } else {
            f.debug_struct("MdInlineItalic").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdInlineItalic> for SyntaxNode {
    fn from(n: MdInlineItalic) -> Self {
        n.syntax
    }
}
impl From<MdInlineItalic> for SyntaxElement {
    fn from(n: MdInlineItalic) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdInlineLink {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_INLINE_LINK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_INLINE_LINK
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
impl std::fmt::Debug for MdInlineLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdInlineLink")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("text", &self.text())
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("destination", &self.destination())
                .field("title", &support::DebugOptionalElement(self.title()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdInlineLink").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdInlineLink> for SyntaxNode {
    fn from(n: MdInlineLink) -> Self {
        n.syntax
    }
}
impl From<MdInlineLink> for SyntaxElement {
    fn from(n: MdInlineLink) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdLinkBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_LINK_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_LINK_BLOCK
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
impl std::fmt::Debug for MdLinkBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdLinkBlock")
                .field("label", &support::DebugSyntaxResult(self.label()))
                .field("url", &support::DebugSyntaxResult(self.url()))
                .field("title", &support::DebugOptionalElement(self.title()))
                .finish()
        } else {
            f.debug_struct("MdLinkBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdLinkBlock> for SyntaxNode {
    fn from(n: MdLinkBlock) -> Self {
        n.syntax
    }
}
impl From<MdLinkBlock> for SyntaxElement {
    fn from(n: MdLinkBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdLinkDestination {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_LINK_DESTINATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_LINK_DESTINATION
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
impl std::fmt::Debug for MdLinkDestination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdLinkDestination")
                .field("content", &self.content())
                .finish()
        } else {
            f.debug_struct("MdLinkDestination").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdLinkDestination> for SyntaxNode {
    fn from(n: MdLinkDestination) -> Self {
        n.syntax
    }
}
impl From<MdLinkDestination> for SyntaxElement {
    fn from(n: MdLinkDestination) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdLinkLabel {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_LINK_LABEL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_LINK_LABEL
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
impl std::fmt::Debug for MdLinkLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdLinkLabel")
                .field("content", &self.content())
                .finish()
        } else {
            f.debug_struct("MdLinkLabel").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdLinkLabel> for SyntaxNode {
    fn from(n: MdLinkLabel) -> Self {
        n.syntax
    }
}
impl From<MdLinkLabel> for SyntaxElement {
    fn from(n: MdLinkLabel) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdLinkReferenceDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_LINK_REFERENCE_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_LINK_REFERENCE_DEFINITION
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
impl std::fmt::Debug for MdLinkReferenceDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdLinkReferenceDefinition")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("label", &support::DebugSyntaxResult(self.label()))
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field(
                    "destination",
                    &support::DebugSyntaxResult(self.destination()),
                )
                .field("title", &support::DebugOptionalElement(self.title()))
                .finish()
        } else {
            f.debug_struct("MdLinkReferenceDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdLinkReferenceDefinition> for SyntaxNode {
    fn from(n: MdLinkReferenceDefinition) -> Self {
        n.syntax
    }
}
impl From<MdLinkReferenceDefinition> for SyntaxElement {
    fn from(n: MdLinkReferenceDefinition) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdLinkTitle {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_LINK_TITLE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_LINK_TITLE
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
impl std::fmt::Debug for MdLinkTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdLinkTitle")
                .field("content", &self.content())
                .finish()
        } else {
            f.debug_struct("MdLinkTitle").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdLinkTitle> for SyntaxNode {
    fn from(n: MdLinkTitle) -> Self {
        n.syntax
    }
}
impl From<MdLinkTitle> for SyntaxElement {
    fn from(n: MdLinkTitle) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdNewline {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_NEWLINE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_NEWLINE
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
impl std::fmt::Debug for MdNewline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdNewline")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdNewline").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdNewline> for SyntaxNode {
    fn from(n: MdNewline) -> Self {
        n.syntax
    }
}
impl From<MdNewline> for SyntaxElement {
    fn from(n: MdNewline) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdOrderedListItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_ORDERED_LIST_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_ORDERED_LIST_ITEM
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
impl std::fmt::Debug for MdOrderedListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdOrderedListItem")
                .field("md_bullet_list", &self.md_bullet_list())
                .finish()
        } else {
            f.debug_struct("MdOrderedListItem").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdOrderedListItem> for SyntaxNode {
    fn from(n: MdOrderedListItem) -> Self {
        n.syntax
    }
}
impl From<MdOrderedListItem> for SyntaxElement {
    fn from(n: MdOrderedListItem) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdParagraph {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_PARAGRAPH as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_PARAGRAPH
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
impl std::fmt::Debug for MdParagraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdParagraph")
                .field("list", &self.list())
                .field(
                    "hard_line",
                    &support::DebugOptionalElement(self.hard_line()),
                )
                .finish()
        } else {
            f.debug_struct("MdParagraph").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdParagraph> for SyntaxNode {
    fn from(n: MdParagraph) -> Self {
        n.syntax
    }
}
impl From<MdParagraph> for SyntaxElement {
    fn from(n: MdParagraph) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdQuote {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_QUOTE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_QUOTE
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
impl std::fmt::Debug for MdQuote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdQuote")
                .field("prefix", &support::DebugSyntaxResult(self.prefix()))
                .field("content", &self.content())
                .finish()
        } else {
            f.debug_struct("MdQuote").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdQuote> for SyntaxNode {
    fn from(n: MdQuote) -> Self {
        n.syntax
    }
}
impl From<MdQuote> for SyntaxElement {
    fn from(n: MdQuote) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdQuoteIndent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_QUOTE_INDENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_QUOTE_INDENT
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
impl std::fmt::Debug for MdQuoteIndent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdQuoteIndent")
                .field(
                    "md_quote_pre_marker_indent_token",
                    &support::DebugSyntaxResult(self.md_quote_pre_marker_indent_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdQuoteIndent").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdQuoteIndent> for SyntaxNode {
    fn from(n: MdQuoteIndent) -> Self {
        n.syntax
    }
}
impl From<MdQuoteIndent> for SyntaxElement {
    fn from(n: MdQuoteIndent) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdQuotePrefix {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_QUOTE_PREFIX as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_QUOTE_PREFIX
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
impl std::fmt::Debug for MdQuotePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdQuotePrefix")
                .field("pre_marker_indent", &self.pre_marker_indent())
                .field(
                    "marker_token",
                    &support::DebugSyntaxResult(self.marker_token()),
                )
                .field(
                    "post_marker_space_token",
                    &support::DebugOptionalElement(self.post_marker_space_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdQuotePrefix").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdQuotePrefix> for SyntaxNode {
    fn from(n: MdQuotePrefix) -> Self {
        n.syntax
    }
}
impl From<MdQuotePrefix> for SyntaxElement {
    fn from(n: MdQuotePrefix) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdReferenceImage {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_REFERENCE_IMAGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_REFERENCE_IMAGE
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
impl std::fmt::Debug for MdReferenceImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdReferenceImage")
                .field("excl_token", &support::DebugSyntaxResult(self.excl_token()))
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("alt", &self.alt())
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .field("label", &support::DebugOptionalElement(self.label()))
                .finish()
        } else {
            f.debug_struct("MdReferenceImage").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdReferenceImage> for SyntaxNode {
    fn from(n: MdReferenceImage) -> Self {
        n.syntax
    }
}
impl From<MdReferenceImage> for SyntaxElement {
    fn from(n: MdReferenceImage) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdReferenceLink {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_REFERENCE_LINK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_REFERENCE_LINK
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
impl std::fmt::Debug for MdReferenceLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdReferenceLink")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("text", &self.text())
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .field("label", &support::DebugOptionalElement(self.label()))
                .finish()
        } else {
            f.debug_struct("MdReferenceLink").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdReferenceLink> for SyntaxNode {
    fn from(n: MdReferenceLink) -> Self {
        n.syntax
    }
}
impl From<MdReferenceLink> for SyntaxElement {
    fn from(n: MdReferenceLink) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdReferenceLinkLabel {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_REFERENCE_LINK_LABEL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_REFERENCE_LINK_LABEL
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
impl std::fmt::Debug for MdReferenceLinkLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdReferenceLinkLabel")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("label", &self.label())
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdReferenceLinkLabel").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdReferenceLinkLabel> for SyntaxNode {
    fn from(n: MdReferenceLinkLabel) -> Self {
        n.syntax
    }
}
impl From<MdReferenceLinkLabel> for SyntaxElement {
    fn from(n: MdReferenceLinkLabel) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdSetextHeader {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_SETEXT_HEADER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_SETEXT_HEADER
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
impl std::fmt::Debug for MdSetextHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdSetextHeader")
                .field("content", &self.content())
                .field(
                    "underline_token",
                    &support::DebugSyntaxResult(self.underline_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdSetextHeader").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdSetextHeader> for SyntaxNode {
    fn from(n: MdSetextHeader) -> Self {
        n.syntax
    }
}
impl From<MdSetextHeader> for SyntaxElement {
    fn from(n: MdSetextHeader) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdSoftBreak {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_SOFT_BREAK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_SOFT_BREAK
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
impl std::fmt::Debug for MdSoftBreak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdSoftBreak")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdSoftBreak").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdSoftBreak> for SyntaxNode {
    fn from(n: MdSoftBreak) -> Self {
        n.syntax
    }
}
impl From<MdSoftBreak> for SyntaxElement {
    fn from(n: MdSoftBreak) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdTextual {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_TEXTUAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_TEXTUAL
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
impl std::fmt::Debug for MdTextual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdTextual")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdTextual").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdTextual> for SyntaxNode {
    fn from(n: MdTextual) -> Self {
        n.syntax
    }
}
impl From<MdTextual> for SyntaxElement {
    fn from(n: MdTextual) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdThematicBreakBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_THEMATIC_BREAK_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_THEMATIC_BREAK_BLOCK
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
impl std::fmt::Debug for MdThematicBreakBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdThematicBreakBlock")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdThematicBreakBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdThematicBreakBlock> for SyntaxNode {
    fn from(n: MdThematicBreakBlock) -> Self {
        n.syntax
    }
}
impl From<MdThematicBreakBlock> for SyntaxElement {
    fn from(n: MdThematicBreakBlock) -> Self {
        n.syntax.into()
    }
}
impl From<MdQuotePrefix> for AnyMdBlock {
    fn from(node: MdQuotePrefix) -> Self {
        Self::MdQuotePrefix(node)
    }
}
impl AstNode for AnyMdBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyMdContainerBlock::KIND_SET
        .union(AnyMdLeafBlock::KIND_SET)
        .union(MdQuotePrefix::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MD_QUOTE_PREFIX => true,
            k if AnyMdContainerBlock::can_cast(k) => true,
            k if AnyMdLeafBlock::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MD_QUOTE_PREFIX => Self::MdQuotePrefix(MdQuotePrefix { syntax }),
            _ => {
                let syntax = match AnyMdContainerBlock::try_cast(syntax) {
                    Ok(any_md_container_block) => {
                        return Some(Self::AnyMdContainerBlock(any_md_container_block));
                    }
                    Err(syntax) => syntax,
                };
                if let Some(any_md_leaf_block) = AnyMdLeafBlock::cast(syntax) {
                    return Some(Self::AnyMdLeafBlock(any_md_leaf_block));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MdQuotePrefix(it) => it.syntax(),
            Self::AnyMdContainerBlock(it) => it.syntax(),
            Self::AnyMdLeafBlock(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MdQuotePrefix(it) => it.into_syntax(),
            Self::AnyMdContainerBlock(it) => it.into_syntax(),
            Self::AnyMdLeafBlock(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyMdBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyMdContainerBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::AnyMdLeafBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::MdQuotePrefix(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMdBlock> for SyntaxNode {
    fn from(n: AnyMdBlock) -> Self {
        match n {
            AnyMdBlock::AnyMdContainerBlock(it) => it.into_syntax(),
            AnyMdBlock::AnyMdLeafBlock(it) => it.into_syntax(),
            AnyMdBlock::MdQuotePrefix(it) => it.into_syntax(),
        }
    }
}
impl From<AnyMdBlock> for SyntaxElement {
    fn from(n: AnyMdBlock) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MdFencedCodeBlock> for AnyMdCodeBlock {
    fn from(node: MdFencedCodeBlock) -> Self {
        Self::MdFencedCodeBlock(node)
    }
}
impl From<MdIndentCodeBlock> for AnyMdCodeBlock {
    fn from(node: MdIndentCodeBlock) -> Self {
        Self::MdIndentCodeBlock(node)
    }
}
impl AstNode for AnyMdCodeBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        MdFencedCodeBlock::KIND_SET.union(MdIndentCodeBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, MD_FENCED_CODE_BLOCK | MD_INDENT_CODE_BLOCK)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MD_FENCED_CODE_BLOCK => Self::MdFencedCodeBlock(MdFencedCodeBlock { syntax }),
            MD_INDENT_CODE_BLOCK => Self::MdIndentCodeBlock(MdIndentCodeBlock { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MdFencedCodeBlock(it) => it.syntax(),
            Self::MdIndentCodeBlock(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MdFencedCodeBlock(it) => it.into_syntax(),
            Self::MdIndentCodeBlock(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyMdCodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MdFencedCodeBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::MdIndentCodeBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMdCodeBlock> for SyntaxNode {
    fn from(n: AnyMdCodeBlock) -> Self {
        match n {
            AnyMdCodeBlock::MdFencedCodeBlock(it) => it.into_syntax(),
            AnyMdCodeBlock::MdIndentCodeBlock(it) => it.into_syntax(),
        }
    }
}
impl From<AnyMdCodeBlock> for SyntaxElement {
    fn from(n: AnyMdCodeBlock) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MdBulletListItem> for AnyMdContainerBlock {
    fn from(node: MdBulletListItem) -> Self {
        Self::MdBulletListItem(node)
    }
}
impl From<MdOrderedListItem> for AnyMdContainerBlock {
    fn from(node: MdOrderedListItem) -> Self {
        Self::MdOrderedListItem(node)
    }
}
impl From<MdQuote> for AnyMdContainerBlock {
    fn from(node: MdQuote) -> Self {
        Self::MdQuote(node)
    }
}
impl AstNode for AnyMdContainerBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MdBulletListItem::KIND_SET
        .union(MdOrderedListItem::KIND_SET)
        .union(MdQuote::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, MD_BULLET_LIST_ITEM | MD_ORDERED_LIST_ITEM | MD_QUOTE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MD_BULLET_LIST_ITEM => Self::MdBulletListItem(MdBulletListItem { syntax }),
            MD_ORDERED_LIST_ITEM => Self::MdOrderedListItem(MdOrderedListItem { syntax }),
            MD_QUOTE => Self::MdQuote(MdQuote { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MdBulletListItem(it) => it.syntax(),
            Self::MdOrderedListItem(it) => it.syntax(),
            Self::MdQuote(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MdBulletListItem(it) => it.into_syntax(),
            Self::MdOrderedListItem(it) => it.into_syntax(),
            Self::MdQuote(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyMdContainerBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MdBulletListItem(it) => std::fmt::Debug::fmt(it, f),
            Self::MdOrderedListItem(it) => std::fmt::Debug::fmt(it, f),
            Self::MdQuote(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMdContainerBlock> for SyntaxNode {
    fn from(n: AnyMdContainerBlock) -> Self {
        match n {
            AnyMdContainerBlock::MdBulletListItem(it) => it.into_syntax(),
            AnyMdContainerBlock::MdOrderedListItem(it) => it.into_syntax(),
            AnyMdContainerBlock::MdQuote(it) => it.into_syntax(),
        }
    }
}
impl From<AnyMdContainerBlock> for SyntaxElement {
    fn from(n: AnyMdContainerBlock) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MdAutolink> for AnyMdInline {
    fn from(node: MdAutolink) -> Self {
        Self::MdAutolink(node)
    }
}
impl From<MdEntityReference> for AnyMdInline {
    fn from(node: MdEntityReference) -> Self {
        Self::MdEntityReference(node)
    }
}
impl From<MdHardLine> for AnyMdInline {
    fn from(node: MdHardLine) -> Self {
        Self::MdHardLine(node)
    }
}
impl From<MdHtmlBlock> for AnyMdInline {
    fn from(node: MdHtmlBlock) -> Self {
        Self::MdHtmlBlock(node)
    }
}
impl From<MdInlineCode> for AnyMdInline {
    fn from(node: MdInlineCode) -> Self {
        Self::MdInlineCode(node)
    }
}
impl From<MdInlineEmphasis> for AnyMdInline {
    fn from(node: MdInlineEmphasis) -> Self {
        Self::MdInlineEmphasis(node)
    }
}
impl From<MdInlineHtml> for AnyMdInline {
    fn from(node: MdInlineHtml) -> Self {
        Self::MdInlineHtml(node)
    }
}
impl From<MdInlineImage> for AnyMdInline {
    fn from(node: MdInlineImage) -> Self {
        Self::MdInlineImage(node)
    }
}
impl From<MdInlineItalic> for AnyMdInline {
    fn from(node: MdInlineItalic) -> Self {
        Self::MdInlineItalic(node)
    }
}
impl From<MdInlineLink> for AnyMdInline {
    fn from(node: MdInlineLink) -> Self {
        Self::MdInlineLink(node)
    }
}
impl From<MdQuotePrefix> for AnyMdInline {
    fn from(node: MdQuotePrefix) -> Self {
        Self::MdQuotePrefix(node)
    }
}
impl From<MdReferenceImage> for AnyMdInline {
    fn from(node: MdReferenceImage) -> Self {
        Self::MdReferenceImage(node)
    }
}
impl From<MdReferenceLink> for AnyMdInline {
    fn from(node: MdReferenceLink) -> Self {
        Self::MdReferenceLink(node)
    }
}
impl From<MdSoftBreak> for AnyMdInline {
    fn from(node: MdSoftBreak) -> Self {
        Self::MdSoftBreak(node)
    }
}
impl From<MdTextual> for AnyMdInline {
    fn from(node: MdTextual) -> Self {
        Self::MdTextual(node)
    }
}
impl AstNode for AnyMdInline {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MdAutolink::KIND_SET
        .union(MdEntityReference::KIND_SET)
        .union(MdHardLine::KIND_SET)
        .union(MdHtmlBlock::KIND_SET)
        .union(MdInlineCode::KIND_SET)
        .union(MdInlineEmphasis::KIND_SET)
        .union(MdInlineHtml::KIND_SET)
        .union(MdInlineImage::KIND_SET)
        .union(MdInlineItalic::KIND_SET)
        .union(MdInlineLink::KIND_SET)
        .union(MdQuotePrefix::KIND_SET)
        .union(MdReferenceImage::KIND_SET)
        .union(MdReferenceLink::KIND_SET)
        .union(MdSoftBreak::KIND_SET)
        .union(MdTextual::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            MD_AUTOLINK
                | MD_ENTITY_REFERENCE
                | MD_HARD_LINE
                | MD_HTML_BLOCK
                | MD_INLINE_CODE
                | MD_INLINE_EMPHASIS
                | MD_INLINE_HTML
                | MD_INLINE_IMAGE
                | MD_INLINE_ITALIC
                | MD_INLINE_LINK
                | MD_QUOTE_PREFIX
                | MD_REFERENCE_IMAGE
                | MD_REFERENCE_LINK
                | MD_SOFT_BREAK
                | MD_TEXTUAL
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MD_AUTOLINK => Self::MdAutolink(MdAutolink { syntax }),
            MD_ENTITY_REFERENCE => Self::MdEntityReference(MdEntityReference { syntax }),
            MD_HARD_LINE => Self::MdHardLine(MdHardLine { syntax }),
            MD_HTML_BLOCK => Self::MdHtmlBlock(MdHtmlBlock { syntax }),
            MD_INLINE_CODE => Self::MdInlineCode(MdInlineCode { syntax }),
            MD_INLINE_EMPHASIS => Self::MdInlineEmphasis(MdInlineEmphasis { syntax }),
            MD_INLINE_HTML => Self::MdInlineHtml(MdInlineHtml { syntax }),
            MD_INLINE_IMAGE => Self::MdInlineImage(MdInlineImage { syntax }),
            MD_INLINE_ITALIC => Self::MdInlineItalic(MdInlineItalic { syntax }),
            MD_INLINE_LINK => Self::MdInlineLink(MdInlineLink { syntax }),
            MD_QUOTE_PREFIX => Self::MdQuotePrefix(MdQuotePrefix { syntax }),
            MD_REFERENCE_IMAGE => Self::MdReferenceImage(MdReferenceImage { syntax }),
            MD_REFERENCE_LINK => Self::MdReferenceLink(MdReferenceLink { syntax }),
            MD_SOFT_BREAK => Self::MdSoftBreak(MdSoftBreak { syntax }),
            MD_TEXTUAL => Self::MdTextual(MdTextual { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MdAutolink(it) => it.syntax(),
            Self::MdEntityReference(it) => it.syntax(),
            Self::MdHardLine(it) => it.syntax(),
            Self::MdHtmlBlock(it) => it.syntax(),
            Self::MdInlineCode(it) => it.syntax(),
            Self::MdInlineEmphasis(it) => it.syntax(),
            Self::MdInlineHtml(it) => it.syntax(),
            Self::MdInlineImage(it) => it.syntax(),
            Self::MdInlineItalic(it) => it.syntax(),
            Self::MdInlineLink(it) => it.syntax(),
            Self::MdQuotePrefix(it) => it.syntax(),
            Self::MdReferenceImage(it) => it.syntax(),
            Self::MdReferenceLink(it) => it.syntax(),
            Self::MdSoftBreak(it) => it.syntax(),
            Self::MdTextual(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MdAutolink(it) => it.into_syntax(),
            Self::MdEntityReference(it) => it.into_syntax(),
            Self::MdHardLine(it) => it.into_syntax(),
            Self::MdHtmlBlock(it) => it.into_syntax(),
            Self::MdInlineCode(it) => it.into_syntax(),
            Self::MdInlineEmphasis(it) => it.into_syntax(),
            Self::MdInlineHtml(it) => it.into_syntax(),
            Self::MdInlineImage(it) => it.into_syntax(),
            Self::MdInlineItalic(it) => it.into_syntax(),
            Self::MdInlineLink(it) => it.into_syntax(),
            Self::MdQuotePrefix(it) => it.into_syntax(),
            Self::MdReferenceImage(it) => it.into_syntax(),
            Self::MdReferenceLink(it) => it.into_syntax(),
            Self::MdSoftBreak(it) => it.into_syntax(),
            Self::MdTextual(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyMdInline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MdAutolink(it) => std::fmt::Debug::fmt(it, f),
            Self::MdEntityReference(it) => std::fmt::Debug::fmt(it, f),
            Self::MdHardLine(it) => std::fmt::Debug::fmt(it, f),
            Self::MdHtmlBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::MdInlineCode(it) => std::fmt::Debug::fmt(it, f),
            Self::MdInlineEmphasis(it) => std::fmt::Debug::fmt(it, f),
            Self::MdInlineHtml(it) => std::fmt::Debug::fmt(it, f),
            Self::MdInlineImage(it) => std::fmt::Debug::fmt(it, f),
            Self::MdInlineItalic(it) => std::fmt::Debug::fmt(it, f),
            Self::MdInlineLink(it) => std::fmt::Debug::fmt(it, f),
            Self::MdQuotePrefix(it) => std::fmt::Debug::fmt(it, f),
            Self::MdReferenceImage(it) => std::fmt::Debug::fmt(it, f),
            Self::MdReferenceLink(it) => std::fmt::Debug::fmt(it, f),
            Self::MdSoftBreak(it) => std::fmt::Debug::fmt(it, f),
            Self::MdTextual(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMdInline> for SyntaxNode {
    fn from(n: AnyMdInline) -> Self {
        match n {
            AnyMdInline::MdAutolink(it) => it.into_syntax(),
            AnyMdInline::MdEntityReference(it) => it.into_syntax(),
            AnyMdInline::MdHardLine(it) => it.into_syntax(),
            AnyMdInline::MdHtmlBlock(it) => it.into_syntax(),
            AnyMdInline::MdInlineCode(it) => it.into_syntax(),
            AnyMdInline::MdInlineEmphasis(it) => it.into_syntax(),
            AnyMdInline::MdInlineHtml(it) => it.into_syntax(),
            AnyMdInline::MdInlineImage(it) => it.into_syntax(),
            AnyMdInline::MdInlineItalic(it) => it.into_syntax(),
            AnyMdInline::MdInlineLink(it) => it.into_syntax(),
            AnyMdInline::MdQuotePrefix(it) => it.into_syntax(),
            AnyMdInline::MdReferenceImage(it) => it.into_syntax(),
            AnyMdInline::MdReferenceLink(it) => it.into_syntax(),
            AnyMdInline::MdSoftBreak(it) => it.into_syntax(),
            AnyMdInline::MdTextual(it) => it.into_syntax(),
        }
    }
}
impl From<AnyMdInline> for SyntaxElement {
    fn from(n: AnyMdInline) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MdHeader> for AnyMdLeafBlock {
    fn from(node: MdHeader) -> Self {
        Self::MdHeader(node)
    }
}
impl From<MdHtmlBlock> for AnyMdLeafBlock {
    fn from(node: MdHtmlBlock) -> Self {
        Self::MdHtmlBlock(node)
    }
}
impl From<MdLinkBlock> for AnyMdLeafBlock {
    fn from(node: MdLinkBlock) -> Self {
        Self::MdLinkBlock(node)
    }
}
impl From<MdLinkReferenceDefinition> for AnyMdLeafBlock {
    fn from(node: MdLinkReferenceDefinition) -> Self {
        Self::MdLinkReferenceDefinition(node)
    }
}
impl From<MdNewline> for AnyMdLeafBlock {
    fn from(node: MdNewline) -> Self {
        Self::MdNewline(node)
    }
}
impl From<MdParagraph> for AnyMdLeafBlock {
    fn from(node: MdParagraph) -> Self {
        Self::MdParagraph(node)
    }
}
impl From<MdSetextHeader> for AnyMdLeafBlock {
    fn from(node: MdSetextHeader) -> Self {
        Self::MdSetextHeader(node)
    }
}
impl From<MdThematicBreakBlock> for AnyMdLeafBlock {
    fn from(node: MdThematicBreakBlock) -> Self {
        Self::MdThematicBreakBlock(node)
    }
}
impl AstNode for AnyMdLeafBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyMdCodeBlock::KIND_SET
        .union(MdHeader::KIND_SET)
        .union(MdHtmlBlock::KIND_SET)
        .union(MdLinkBlock::KIND_SET)
        .union(MdLinkReferenceDefinition::KIND_SET)
        .union(MdNewline::KIND_SET)
        .union(MdParagraph::KIND_SET)
        .union(MdSetextHeader::KIND_SET)
        .union(MdThematicBreakBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MD_HEADER
            | MD_HTML_BLOCK
            | MD_LINK_BLOCK
            | MD_LINK_REFERENCE_DEFINITION
            | MD_NEWLINE
            | MD_PARAGRAPH
            | MD_SETEXT_HEADER
            | MD_THEMATIC_BREAK_BLOCK => true,
            k if AnyMdCodeBlock::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MD_HEADER => Self::MdHeader(MdHeader { syntax }),
            MD_HTML_BLOCK => Self::MdHtmlBlock(MdHtmlBlock { syntax }),
            MD_LINK_BLOCK => Self::MdLinkBlock(MdLinkBlock { syntax }),
            MD_LINK_REFERENCE_DEFINITION => {
                Self::MdLinkReferenceDefinition(MdLinkReferenceDefinition { syntax })
            }
            MD_NEWLINE => Self::MdNewline(MdNewline { syntax }),
            MD_PARAGRAPH => Self::MdParagraph(MdParagraph { syntax }),
            MD_SETEXT_HEADER => Self::MdSetextHeader(MdSetextHeader { syntax }),
            MD_THEMATIC_BREAK_BLOCK => Self::MdThematicBreakBlock(MdThematicBreakBlock { syntax }),
            _ => {
                if let Some(any_md_code_block) = AnyMdCodeBlock::cast(syntax) {
                    return Some(Self::AnyMdCodeBlock(any_md_code_block));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MdHeader(it) => it.syntax(),
            Self::MdHtmlBlock(it) => it.syntax(),
            Self::MdLinkBlock(it) => it.syntax(),
            Self::MdLinkReferenceDefinition(it) => it.syntax(),
            Self::MdNewline(it) => it.syntax(),
            Self::MdParagraph(it) => it.syntax(),
            Self::MdSetextHeader(it) => it.syntax(),
            Self::MdThematicBreakBlock(it) => it.syntax(),
            Self::AnyMdCodeBlock(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MdHeader(it) => it.into_syntax(),
            Self::MdHtmlBlock(it) => it.into_syntax(),
            Self::MdLinkBlock(it) => it.into_syntax(),
            Self::MdLinkReferenceDefinition(it) => it.into_syntax(),
            Self::MdNewline(it) => it.into_syntax(),
            Self::MdParagraph(it) => it.into_syntax(),
            Self::MdSetextHeader(it) => it.into_syntax(),
            Self::MdThematicBreakBlock(it) => it.into_syntax(),
            Self::AnyMdCodeBlock(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyMdLeafBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyMdCodeBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::MdHeader(it) => std::fmt::Debug::fmt(it, f),
            Self::MdHtmlBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::MdLinkBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::MdLinkReferenceDefinition(it) => std::fmt::Debug::fmt(it, f),
            Self::MdNewline(it) => std::fmt::Debug::fmt(it, f),
            Self::MdParagraph(it) => std::fmt::Debug::fmt(it, f),
            Self::MdSetextHeader(it) => std::fmt::Debug::fmt(it, f),
            Self::MdThematicBreakBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMdLeafBlock> for SyntaxNode {
    fn from(n: AnyMdLeafBlock) -> Self {
        match n {
            AnyMdLeafBlock::AnyMdCodeBlock(it) => it.into_syntax(),
            AnyMdLeafBlock::MdHeader(it) => it.into_syntax(),
            AnyMdLeafBlock::MdHtmlBlock(it) => it.into_syntax(),
            AnyMdLeafBlock::MdLinkBlock(it) => it.into_syntax(),
            AnyMdLeafBlock::MdLinkReferenceDefinition(it) => it.into_syntax(),
            AnyMdLeafBlock::MdNewline(it) => it.into_syntax(),
            AnyMdLeafBlock::MdParagraph(it) => it.into_syntax(),
            AnyMdLeafBlock::MdSetextHeader(it) => it.into_syntax(),
            AnyMdLeafBlock::MdThematicBreakBlock(it) => it.into_syntax(),
        }
    }
}
impl From<AnyMdLeafBlock> for SyntaxElement {
    fn from(n: AnyMdLeafBlock) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyMdBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMdCodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMdContainerBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMdInline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMdLeafBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdAutolink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdBullet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdBulletListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdEntityReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdFencedCodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdHardLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdHtmlBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdIndent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdIndentCodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdInlineCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdInlineEmphasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdInlineHtml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdInlineImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdInlineItalic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdInlineLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdLinkBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdLinkDestination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdLinkLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdLinkReferenceDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdLinkTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdNewline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdOrderedListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdParagraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdQuote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdQuoteIndent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdQuotePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdReferenceImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdReferenceLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdReferenceLinkLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdSetextHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdSoftBreak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdTextual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdThematicBreakBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MdBogus {
    syntax: SyntaxNode,
}
impl MdBogus {
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
impl AstNode for MdBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_BOGUS
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
impl std::fmt::Debug for MdBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MdBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<MdBogus> for SyntaxNode {
    fn from(n: MdBogus) -> Self {
        n.syntax
    }
}
impl From<MdBogus> for SyntaxElement {
    fn from(n: MdBogus) -> Self {
        n.syntax.into()
    }
}
biome_rowan::declare_node_union! { pub AnyMdBogusNode = MdBogus }
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MdBlockList {
    syntax_list: SyntaxList,
}
impl MdBlockList {
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
impl AstNode for MdBlockList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_BLOCK_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_BLOCK_LIST
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
impl Serialize for MdBlockList {
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
impl AstNodeList for MdBlockList {
    type Language = Language;
    type Node = AnyMdBlock;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MdBlockList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MdBlockList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MdBlockList {
    type Item = AnyMdBlock;
    type IntoIter = AstNodeListIterator<Language, AnyMdBlock>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MdBlockList {
    type Item = AnyMdBlock;
    type IntoIter = AstNodeListIterator<Language, AnyMdBlock>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MdBulletList {
    syntax_list: SyntaxList,
}
impl MdBulletList {
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
impl AstNode for MdBulletList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_BULLET_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_BULLET_LIST
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
impl Serialize for MdBulletList {
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
impl AstNodeList for MdBulletList {
    type Language = Language;
    type Node = MdBullet;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MdBulletList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MdBulletList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MdBulletList {
    type Item = MdBullet;
    type IntoIter = AstNodeListIterator<Language, MdBullet>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MdBulletList {
    type Item = MdBullet;
    type IntoIter = AstNodeListIterator<Language, MdBullet>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MdCodeNameList {
    syntax_list: SyntaxList,
}
impl MdCodeNameList {
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
impl AstNode for MdCodeNameList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_CODE_NAME_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_CODE_NAME_LIST
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
impl Serialize for MdCodeNameList {
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
impl AstNodeList for MdCodeNameList {
    type Language = Language;
    type Node = MdTextual;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MdCodeNameList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MdCodeNameList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MdCodeNameList {
    type Item = MdTextual;
    type IntoIter = AstNodeListIterator<Language, MdTextual>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MdCodeNameList {
    type Item = MdTextual;
    type IntoIter = AstNodeListIterator<Language, MdTextual>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MdHashList {
    syntax_list: SyntaxList,
}
impl MdHashList {
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
impl AstNode for MdHashList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_HASH_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_HASH_LIST
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
impl Serialize for MdHashList {
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
impl AstNodeList for MdHashList {
    type Language = Language;
    type Node = MdHash;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MdHashList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MdHashList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MdHashList {
    type Item = MdHash;
    type IntoIter = AstNodeListIterator<Language, MdHash>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MdHashList {
    type Item = MdHash;
    type IntoIter = AstNodeListIterator<Language, MdHash>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MdInlineItemList {
    syntax_list: SyntaxList,
}
impl MdInlineItemList {
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
impl AstNode for MdInlineItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_INLINE_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_INLINE_ITEM_LIST
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
impl Serialize for MdInlineItemList {
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
impl AstNodeList for MdInlineItemList {
    type Language = Language;
    type Node = AnyMdInline;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MdInlineItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MdInlineItemList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MdInlineItemList {
    type Item = AnyMdInline;
    type IntoIter = AstNodeListIterator<Language, AnyMdInline>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MdInlineItemList {
    type Item = AnyMdInline;
    type IntoIter = AstNodeListIterator<Language, AnyMdInline>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MdQuoteIndentList {
    syntax_list: SyntaxList,
}
impl MdQuoteIndentList {
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
impl AstNode for MdQuoteIndentList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_QUOTE_INDENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_QUOTE_INDENT_LIST
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
impl Serialize for MdQuoteIndentList {
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
impl AstNodeList for MdQuoteIndentList {
    type Language = Language;
    type Node = MdQuoteIndent;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MdQuoteIndentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MdQuoteIndentList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MdQuoteIndentList {
    type Item = MdQuoteIndent;
    type IntoIter = AstNodeListIterator<Language, MdQuoteIndent>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MdQuoteIndentList {
    type Item = MdQuoteIndent;
    type IntoIter = AstNodeListIterator<Language, MdQuoteIndent>;
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
