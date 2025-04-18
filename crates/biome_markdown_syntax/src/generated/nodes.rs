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
            space_token: self.space_token(),
            content: self.content(),
        }
    }
    pub fn bullet(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn space_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn content(&self) -> MdInlineItemList {
        support::list(&self.syntax, 2usize)
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
    pub space_token: SyntaxResult<SyntaxToken>,
    pub content: MdInlineItemList,
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
            l_fence_token: self.l_fence_token(),
            code_list: self.code_list(),
            l_hard_line: self.l_hard_line(),
            content: self.content(),
            r_hard_line: self.r_hard_line(),
            r_fence_token: self.r_fence_token(),
        }
    }
    pub fn l_fence_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn code_list(&self) -> MdCodeNameList {
        support::list(&self.syntax, 1usize)
    }
    pub fn l_hard_line(&self) -> SyntaxResult<MdHardLine> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn content(&self) -> SyntaxResult<MdTextual> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_hard_line(&self) -> SyntaxResult<MdHardLine> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn r_fence_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
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
    pub l_fence_token: SyntaxResult<SyntaxToken>,
    pub code_list: MdCodeNameList,
    pub l_hard_line: SyntaxResult<MdHardLine>,
    pub content: SyntaxResult<MdTextual>,
    pub r_hard_line: SyntaxResult<MdHardLine>,
    pub r_fence_token: SyntaxResult<SyntaxToken>,
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
            md_textual: self.md_textual(),
        }
    }
    pub fn md_textual(&self) -> SyntaxResult<MdTextual> {
        support::required_node(&self.syntax, 0usize)
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
    pub md_textual: SyntaxResult<MdTextual>,
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
            lines: self.lines(),
        }
    }
    pub fn lines(&self) -> MdIndentedCodeLineList {
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
    pub lines: MdIndentedCodeLineList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdIndentedCodeLine {
    pub(crate) syntax: SyntaxNode,
}
impl MdIndentedCodeLine {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdIndentedCodeLineFields {
        MdIndentedCodeLineFields {
            indentation: self.indentation(),
            content: self.content(),
        }
    }
    pub fn indentation(&self) -> SyntaxResult<MdIndent> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn content(&self) -> SyntaxResult<MdTextual> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for MdIndentedCodeLine {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdIndentedCodeLineFields {
    pub indentation: SyntaxResult<MdIndent>,
    pub content: SyntaxResult<MdTextual>,
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
            l_brack_token: self.l_brack_token(),
            excl_token: self.excl_token(),
            alt: self.alt(),
            source: self.source(),
            r_brack_token: self.r_brack_token(),
            link: self.link(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn alt(&self) -> SyntaxResult<MdInlineImageAlt> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn source(&self) -> SyntaxResult<MdInlineImageSource> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn link(&self) -> Option<MdInlineImageLink> {
        support::node(&self.syntax, 5usize)
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
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub excl_token: SyntaxResult<SyntaxToken>,
    pub alt: SyntaxResult<MdInlineImageAlt>,
    pub source: SyntaxResult<MdInlineImageSource>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
    pub link: Option<MdInlineImageLink>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdInlineImageAlt {
    pub(crate) syntax: SyntaxNode,
}
impl MdInlineImageAlt {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdInlineImageAltFields {
        MdInlineImageAltFields {
            l_brack_token: self.l_brack_token(),
            content: self.content(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn content(&self) -> MdInlineItemList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MdInlineImageAlt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdInlineImageAltFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub content: MdInlineItemList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdInlineImageLink {
    pub(crate) syntax: SyntaxNode,
}
impl MdInlineImageLink {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdInlineImageLinkFields {
        MdInlineImageLinkFields {
            l_paren_token: self.l_paren_token(),
            content: self.content(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn content(&self) -> MdInlineItemList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MdInlineImageLink {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdInlineImageLinkFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub content: MdInlineItemList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdInlineImageSource {
    pub(crate) syntax: SyntaxNode,
}
impl MdInlineImageSource {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdInlineImageSourceFields {
        MdInlineImageSourceFields {
            l_paren_token: self.l_paren_token(),
            content: self.content(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn content(&self) -> MdInlineItemList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MdInlineImageSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdInlineImageSourceFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub content: MdInlineItemList,
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
            source: self.source(),
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
    pub fn source(&self) -> MdInlineItemList {
        support::list(&self.syntax, 4usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
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
    pub source: MdInlineItemList,
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
pub struct MdOrderListItem {
    pub(crate) syntax: SyntaxNode,
}
impl MdOrderListItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MdOrderListItemFields {
        MdOrderListItemFields {
            md_bullet_list: self.md_bullet_list(),
        }
    }
    pub fn md_bullet_list(&self) -> MdBulletList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for MdOrderListItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MdOrderListItemFields {
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
    pub fn hard_line(&self) -> SyntaxResult<MdHardLine> {
        support::required_node(&self.syntax, 1usize)
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
    pub hard_line: SyntaxResult<MdHardLine>,
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
            any_md_block: self.any_md_block(),
        }
    }
    pub fn any_md_block(&self) -> SyntaxResult<AnyMdBlock> {
        support::required_node(&self.syntax, 0usize)
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
    pub any_md_block: SyntaxResult<AnyMdBlock>,
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
            md_paragraph: self.md_paragraph(),
        }
    }
    pub fn md_paragraph(&self) -> SyntaxResult<MdParagraph> {
        support::required_node(&self.syntax, 0usize)
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
    pub md_paragraph: SyntaxResult<MdParagraph>,
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
pub enum AnyCodeBlock {
    MdFencedCodeBlock(MdFencedCodeBlock),
    MdIndentCodeBlock(MdIndentCodeBlock),
}
impl AnyCodeBlock {
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
pub enum AnyContainerBlock {
    MdBulletListItem(MdBulletListItem),
    MdOrderListItem(MdOrderListItem),
    MdQuote(MdQuote),
}
impl AnyContainerBlock {
    pub fn as_md_bullet_list_item(&self) -> Option<&MdBulletListItem> {
        match &self {
            Self::MdBulletListItem(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_order_list_item(&self) -> Option<&MdOrderListItem> {
        match &self {
            Self::MdOrderListItem(item) => Some(item),
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
pub enum AnyLeafBlock {
    AnyCodeBlock(AnyCodeBlock),
    MdHeader(MdHeader),
    MdHtmlBlock(MdHtmlBlock),
    MdLinkBlock(MdLinkBlock),
    MdParagraph(MdParagraph),
    MdSetextHeader(MdSetextHeader),
    MdThematicBreakBlock(MdThematicBreakBlock),
}
impl AnyLeafBlock {
    pub fn as_any_code_block(&self) -> Option<&AnyCodeBlock> {
        match &self {
            Self::AnyCodeBlock(item) => Some(item),
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
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMdBlock {
    AnyContainerBlock(AnyContainerBlock),
    AnyLeafBlock(AnyLeafBlock),
}
impl AnyMdBlock {
    pub fn as_any_container_block(&self) -> Option<&AnyContainerBlock> {
        match &self {
            Self::AnyContainerBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_any_leaf_block(&self) -> Option<&AnyLeafBlock> {
        match &self {
            Self::AnyLeafBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMdInline {
    MdHardLine(MdHardLine),
    MdHtmlBlock(MdHtmlBlock),
    MdInlineCode(MdInlineCode),
    MdInlineEmphasis(MdInlineEmphasis),
    MdInlineImage(MdInlineImage),
    MdInlineItalic(MdInlineItalic),
    MdInlineLink(MdInlineLink),
    MdSoftBreak(MdSoftBreak),
    MdTextual(MdTextual),
}
impl AnyMdInline {
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
                .field(
                    "space_token",
                    &support::DebugSyntaxResult(self.space_token()),
                )
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
                .field(
                    "l_fence_token",
                    &support::DebugSyntaxResult(self.l_fence_token()),
                )
                .field("code_list", &self.code_list())
                .field(
                    "l_hard_line",
                    &support::DebugSyntaxResult(self.l_hard_line()),
                )
                .field("content", &support::DebugSyntaxResult(self.content()))
                .field(
                    "r_hard_line",
                    &support::DebugSyntaxResult(self.r_hard_line()),
                )
                .field(
                    "r_fence_token",
                    &support::DebugSyntaxResult(self.r_fence_token()),
                )
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
                .field("md_textual", &support::DebugSyntaxResult(self.md_textual()))
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
                .field("lines", &self.lines())
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
impl AstNode for MdIndentedCodeLine {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_INDENTED_CODE_LINE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_INDENTED_CODE_LINE
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
impl std::fmt::Debug for MdIndentedCodeLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdIndentedCodeLine")
                .field(
                    "indentation",
                    &support::DebugSyntaxResult(self.indentation()),
                )
                .field("content", &support::DebugSyntaxResult(self.content()))
                .finish()
        } else {
            f.debug_struct("MdIndentedCodeLine").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdIndentedCodeLine> for SyntaxNode {
    fn from(n: MdIndentedCodeLine) -> Self {
        n.syntax
    }
}
impl From<MdIndentedCodeLine> for SyntaxElement {
    fn from(n: MdIndentedCodeLine) -> Self {
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
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("excl_token", &support::DebugSyntaxResult(self.excl_token()))
                .field("alt", &support::DebugSyntaxResult(self.alt()))
                .field("source", &support::DebugSyntaxResult(self.source()))
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .field("link", &support::DebugOptionalElement(self.link()))
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
impl AstNode for MdInlineImageAlt {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_INLINE_IMAGE_ALT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_INLINE_IMAGE_ALT
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
impl std::fmt::Debug for MdInlineImageAlt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdInlineImageAlt")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("content", &self.content())
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdInlineImageAlt").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdInlineImageAlt> for SyntaxNode {
    fn from(n: MdInlineImageAlt) -> Self {
        n.syntax
    }
}
impl From<MdInlineImageAlt> for SyntaxElement {
    fn from(n: MdInlineImageAlt) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdInlineImageLink {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_INLINE_IMAGE_LINK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_INLINE_IMAGE_LINK
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
impl std::fmt::Debug for MdInlineImageLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdInlineImageLink")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("content", &self.content())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdInlineImageLink").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdInlineImageLink> for SyntaxNode {
    fn from(n: MdInlineImageLink) -> Self {
        n.syntax
    }
}
impl From<MdInlineImageLink> for SyntaxElement {
    fn from(n: MdInlineImageLink) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MdInlineImageSource {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_INLINE_IMAGE_SOURCE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_INLINE_IMAGE_SOURCE
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
impl std::fmt::Debug for MdInlineImageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdInlineImageSource")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("content", &self.content())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("MdInlineImageSource").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdInlineImageSource> for SyntaxNode {
    fn from(n: MdInlineImageSource) -> Self {
        n.syntax
    }
}
impl From<MdInlineImageSource> for SyntaxElement {
    fn from(n: MdInlineImageSource) -> Self {
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
                .field("source", &self.source())
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
impl AstNode for MdOrderListItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_ORDER_LIST_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_ORDER_LIST_ITEM
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
impl std::fmt::Debug for MdOrderListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MdOrderListItem")
                .field("md_bullet_list", &self.md_bullet_list())
                .finish()
        } else {
            f.debug_struct("MdOrderListItem").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdOrderListItem> for SyntaxNode {
    fn from(n: MdOrderListItem) -> Self {
        n.syntax
    }
}
impl From<MdOrderListItem> for SyntaxElement {
    fn from(n: MdOrderListItem) -> Self {
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
                .field("hard_line", &support::DebugSyntaxResult(self.hard_line()))
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
                .field(
                    "any_md_block",
                    &support::DebugSyntaxResult(self.any_md_block()),
                )
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
                .field(
                    "md_paragraph",
                    &support::DebugSyntaxResult(self.md_paragraph()),
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
impl From<MdFencedCodeBlock> for AnyCodeBlock {
    fn from(node: MdFencedCodeBlock) -> Self {
        Self::MdFencedCodeBlock(node)
    }
}
impl From<MdIndentCodeBlock> for AnyCodeBlock {
    fn from(node: MdIndentCodeBlock) -> Self {
        Self::MdIndentCodeBlock(node)
    }
}
impl AstNode for AnyCodeBlock {
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
            Self::MdFencedCodeBlock(it) => &it.syntax,
            Self::MdIndentCodeBlock(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MdFencedCodeBlock(it) => it.syntax,
            Self::MdIndentCodeBlock(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MdFencedCodeBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::MdIndentCodeBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCodeBlock> for SyntaxNode {
    fn from(n: AnyCodeBlock) -> Self {
        match n {
            AnyCodeBlock::MdFencedCodeBlock(it) => it.into(),
            AnyCodeBlock::MdIndentCodeBlock(it) => it.into(),
        }
    }
}
impl From<AnyCodeBlock> for SyntaxElement {
    fn from(n: AnyCodeBlock) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MdBulletListItem> for AnyContainerBlock {
    fn from(node: MdBulletListItem) -> Self {
        Self::MdBulletListItem(node)
    }
}
impl From<MdOrderListItem> for AnyContainerBlock {
    fn from(node: MdOrderListItem) -> Self {
        Self::MdOrderListItem(node)
    }
}
impl From<MdQuote> for AnyContainerBlock {
    fn from(node: MdQuote) -> Self {
        Self::MdQuote(node)
    }
}
impl AstNode for AnyContainerBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MdBulletListItem::KIND_SET
        .union(MdOrderListItem::KIND_SET)
        .union(MdQuote::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, MD_BULLET_LIST_ITEM | MD_ORDER_LIST_ITEM | MD_QUOTE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MD_BULLET_LIST_ITEM => Self::MdBulletListItem(MdBulletListItem { syntax }),
            MD_ORDER_LIST_ITEM => Self::MdOrderListItem(MdOrderListItem { syntax }),
            MD_QUOTE => Self::MdQuote(MdQuote { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MdBulletListItem(it) => &it.syntax,
            Self::MdOrderListItem(it) => &it.syntax,
            Self::MdQuote(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MdBulletListItem(it) => it.syntax,
            Self::MdOrderListItem(it) => it.syntax,
            Self::MdQuote(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyContainerBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MdBulletListItem(it) => std::fmt::Debug::fmt(it, f),
            Self::MdOrderListItem(it) => std::fmt::Debug::fmt(it, f),
            Self::MdQuote(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyContainerBlock> for SyntaxNode {
    fn from(n: AnyContainerBlock) -> Self {
        match n {
            AnyContainerBlock::MdBulletListItem(it) => it.into(),
            AnyContainerBlock::MdOrderListItem(it) => it.into(),
            AnyContainerBlock::MdQuote(it) => it.into(),
        }
    }
}
impl From<AnyContainerBlock> for SyntaxElement {
    fn from(n: AnyContainerBlock) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MdHeader> for AnyLeafBlock {
    fn from(node: MdHeader) -> Self {
        Self::MdHeader(node)
    }
}
impl From<MdHtmlBlock> for AnyLeafBlock {
    fn from(node: MdHtmlBlock) -> Self {
        Self::MdHtmlBlock(node)
    }
}
impl From<MdLinkBlock> for AnyLeafBlock {
    fn from(node: MdLinkBlock) -> Self {
        Self::MdLinkBlock(node)
    }
}
impl From<MdParagraph> for AnyLeafBlock {
    fn from(node: MdParagraph) -> Self {
        Self::MdParagraph(node)
    }
}
impl From<MdSetextHeader> for AnyLeafBlock {
    fn from(node: MdSetextHeader) -> Self {
        Self::MdSetextHeader(node)
    }
}
impl From<MdThematicBreakBlock> for AnyLeafBlock {
    fn from(node: MdThematicBreakBlock) -> Self {
        Self::MdThematicBreakBlock(node)
    }
}
impl AstNode for AnyLeafBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCodeBlock::KIND_SET
        .union(MdHeader::KIND_SET)
        .union(MdHtmlBlock::KIND_SET)
        .union(MdLinkBlock::KIND_SET)
        .union(MdParagraph::KIND_SET)
        .union(MdSetextHeader::KIND_SET)
        .union(MdThematicBreakBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MD_HEADER
            | MD_HTML_BLOCK
            | MD_LINK_BLOCK
            | MD_PARAGRAPH
            | MD_SETEXT_HEADER
            | MD_THEMATIC_BREAK_BLOCK => true,
            k if AnyCodeBlock::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MD_HEADER => Self::MdHeader(MdHeader { syntax }),
            MD_HTML_BLOCK => Self::MdHtmlBlock(MdHtmlBlock { syntax }),
            MD_LINK_BLOCK => Self::MdLinkBlock(MdLinkBlock { syntax }),
            MD_PARAGRAPH => Self::MdParagraph(MdParagraph { syntax }),
            MD_SETEXT_HEADER => Self::MdSetextHeader(MdSetextHeader { syntax }),
            MD_THEMATIC_BREAK_BLOCK => Self::MdThematicBreakBlock(MdThematicBreakBlock { syntax }),
            _ => {
                if let Some(any_code_block) = AnyCodeBlock::cast(syntax) {
                    return Some(Self::AnyCodeBlock(any_code_block));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MdHeader(it) => &it.syntax,
            Self::MdHtmlBlock(it) => &it.syntax,
            Self::MdLinkBlock(it) => &it.syntax,
            Self::MdParagraph(it) => &it.syntax,
            Self::MdSetextHeader(it) => &it.syntax,
            Self::MdThematicBreakBlock(it) => &it.syntax,
            Self::AnyCodeBlock(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MdHeader(it) => it.syntax,
            Self::MdHtmlBlock(it) => it.syntax,
            Self::MdLinkBlock(it) => it.syntax,
            Self::MdParagraph(it) => it.syntax,
            Self::MdSetextHeader(it) => it.syntax,
            Self::MdThematicBreakBlock(it) => it.syntax,
            Self::AnyCodeBlock(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyLeafBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCodeBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::MdHeader(it) => std::fmt::Debug::fmt(it, f),
            Self::MdHtmlBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::MdLinkBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::MdParagraph(it) => std::fmt::Debug::fmt(it, f),
            Self::MdSetextHeader(it) => std::fmt::Debug::fmt(it, f),
            Self::MdThematicBreakBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyLeafBlock> for SyntaxNode {
    fn from(n: AnyLeafBlock) -> Self {
        match n {
            AnyLeafBlock::AnyCodeBlock(it) => it.into(),
            AnyLeafBlock::MdHeader(it) => it.into(),
            AnyLeafBlock::MdHtmlBlock(it) => it.into(),
            AnyLeafBlock::MdLinkBlock(it) => it.into(),
            AnyLeafBlock::MdParagraph(it) => it.into(),
            AnyLeafBlock::MdSetextHeader(it) => it.into(),
            AnyLeafBlock::MdThematicBreakBlock(it) => it.into(),
        }
    }
}
impl From<AnyLeafBlock> for SyntaxElement {
    fn from(n: AnyLeafBlock) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl AstNode for AnyMdBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyContainerBlock::KIND_SET.union(AnyLeafBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            k if AnyContainerBlock::can_cast(k) => true,
            k if AnyLeafBlock::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let syntax = match AnyContainerBlock::try_cast(syntax) {
            Ok(any_container_block) => {
                return Some(Self::AnyContainerBlock(any_container_block));
            }
            Err(syntax) => syntax,
        };
        if let Some(any_leaf_block) = AnyLeafBlock::cast(syntax) {
            return Some(Self::AnyLeafBlock(any_leaf_block));
        }
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::AnyContainerBlock(it) => it.syntax(),
            Self::AnyLeafBlock(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::AnyContainerBlock(it) => it.into_syntax(),
            Self::AnyLeafBlock(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyMdBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyContainerBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::AnyLeafBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMdBlock> for SyntaxNode {
    fn from(n: AnyMdBlock) -> Self {
        match n {
            AnyMdBlock::AnyContainerBlock(it) => it.into(),
            AnyMdBlock::AnyLeafBlock(it) => it.into(),
        }
    }
}
impl From<AnyMdBlock> for SyntaxElement {
    fn from(n: AnyMdBlock) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
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
    const KIND_SET: SyntaxKindSet<Language> = MdHardLine::KIND_SET
        .union(MdHtmlBlock::KIND_SET)
        .union(MdInlineCode::KIND_SET)
        .union(MdInlineEmphasis::KIND_SET)
        .union(MdInlineImage::KIND_SET)
        .union(MdInlineItalic::KIND_SET)
        .union(MdInlineLink::KIND_SET)
        .union(MdSoftBreak::KIND_SET)
        .union(MdTextual::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            MD_HARD_LINE
                | MD_HTML_BLOCK
                | MD_INLINE_CODE
                | MD_INLINE_EMPHASIS
                | MD_INLINE_IMAGE
                | MD_INLINE_ITALIC
                | MD_INLINE_LINK
                | MD_SOFT_BREAK
                | MD_TEXTUAL
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MD_HARD_LINE => Self::MdHardLine(MdHardLine { syntax }),
            MD_HTML_BLOCK => Self::MdHtmlBlock(MdHtmlBlock { syntax }),
            MD_INLINE_CODE => Self::MdInlineCode(MdInlineCode { syntax }),
            MD_INLINE_EMPHASIS => Self::MdInlineEmphasis(MdInlineEmphasis { syntax }),
            MD_INLINE_IMAGE => Self::MdInlineImage(MdInlineImage { syntax }),
            MD_INLINE_ITALIC => Self::MdInlineItalic(MdInlineItalic { syntax }),
            MD_INLINE_LINK => Self::MdInlineLink(MdInlineLink { syntax }),
            MD_SOFT_BREAK => Self::MdSoftBreak(MdSoftBreak { syntax }),
            MD_TEXTUAL => Self::MdTextual(MdTextual { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MdHardLine(it) => &it.syntax,
            Self::MdHtmlBlock(it) => &it.syntax,
            Self::MdInlineCode(it) => &it.syntax,
            Self::MdInlineEmphasis(it) => &it.syntax,
            Self::MdInlineImage(it) => &it.syntax,
            Self::MdInlineItalic(it) => &it.syntax,
            Self::MdInlineLink(it) => &it.syntax,
            Self::MdSoftBreak(it) => &it.syntax,
            Self::MdTextual(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MdHardLine(it) => it.syntax,
            Self::MdHtmlBlock(it) => it.syntax,
            Self::MdInlineCode(it) => it.syntax,
            Self::MdInlineEmphasis(it) => it.syntax,
            Self::MdInlineImage(it) => it.syntax,
            Self::MdInlineItalic(it) => it.syntax,
            Self::MdInlineLink(it) => it.syntax,
            Self::MdSoftBreak(it) => it.syntax,
            Self::MdTextual(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMdInline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MdHardLine(it) => std::fmt::Debug::fmt(it, f),
            Self::MdHtmlBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::MdInlineCode(it) => std::fmt::Debug::fmt(it, f),
            Self::MdInlineEmphasis(it) => std::fmt::Debug::fmt(it, f),
            Self::MdInlineImage(it) => std::fmt::Debug::fmt(it, f),
            Self::MdInlineItalic(it) => std::fmt::Debug::fmt(it, f),
            Self::MdInlineLink(it) => std::fmt::Debug::fmt(it, f),
            Self::MdSoftBreak(it) => std::fmt::Debug::fmt(it, f),
            Self::MdTextual(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMdInline> for SyntaxNode {
    fn from(n: AnyMdInline) -> Self {
        match n {
            AnyMdInline::MdHardLine(it) => it.into(),
            AnyMdInline::MdHtmlBlock(it) => it.into(),
            AnyMdInline::MdInlineCode(it) => it.into(),
            AnyMdInline::MdInlineEmphasis(it) => it.into(),
            AnyMdInline::MdInlineImage(it) => it.into(),
            AnyMdInline::MdInlineItalic(it) => it.into(),
            AnyMdInline::MdInlineLink(it) => it.into(),
            AnyMdInline::MdSoftBreak(it) => it.into(),
            AnyMdInline::MdTextual(it) => it.into(),
        }
    }
}
impl From<AnyMdInline> for SyntaxElement {
    fn from(n: AnyMdInline) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyCodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyContainerBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyLeafBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMdBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMdInline {
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
impl std::fmt::Display for MdIndentedCodeLine {
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
impl std::fmt::Display for MdInlineImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdInlineImageAlt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdInlineImageLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdInlineImageSource {
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
impl std::fmt::Display for MdOrderListItem {
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
impl AstSeparatedList for MdCodeNameList {
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
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for MdCodeNameList {
    type Item = SyntaxResult<MdTextual>;
    type IntoIter = AstSeparatedListNodesIterator<Language, MdTextual>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &MdCodeNameList {
    type Item = SyntaxResult<MdTextual>;
    type IntoIter = AstSeparatedListNodesIterator<Language, MdTextual>;
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
pub struct MdIndentedCodeLineList {
    syntax_list: SyntaxList,
}
impl MdIndentedCodeLineList {
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
impl AstNode for MdIndentedCodeLineList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_INDENTED_CODE_LINE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_INDENTED_CODE_LINE_LIST
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
impl Serialize for MdIndentedCodeLineList {
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
impl AstNodeList for MdIndentedCodeLineList {
    type Language = Language;
    type Node = MdIndentedCodeLine;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MdIndentedCodeLineList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MdIndentedCodeLineList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MdIndentedCodeLineList {
    type Item = MdIndentedCodeLine;
    type IntoIter = AstNodeListIterator<Language, MdIndentedCodeLine>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MdIndentedCodeLineList {
    type Item = MdIndentedCodeLine;
    type IntoIter = AstNodeListIterator<Language, MdIndentedCodeLine>;
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
pub struct MdOrderList {
    syntax_list: SyntaxList,
}
impl MdOrderList {
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
impl AstNode for MdOrderList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_ORDER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_ORDER_LIST
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
impl Serialize for MdOrderList {
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
impl AstNodeList for MdOrderList {
    type Language = Language;
    type Node = AnyCodeBlock;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MdOrderList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MdOrderList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MdOrderList {
    type Item = AnyCodeBlock;
    type IntoIter = AstNodeListIterator<Language, AnyCodeBlock>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MdOrderList {
    type Item = AnyCodeBlock;
    type IntoIter = AstNodeListIterator<Language, AnyCodeBlock>;
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
