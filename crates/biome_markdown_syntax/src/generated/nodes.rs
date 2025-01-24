//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    macros::map_syntax_node,
    MarkdownLanguage as Language, MarkdownSyntaxElement as SyntaxElement,
    MarkdownSyntaxElementChildren as SyntaxElementChildren,
    MarkdownSyntaxKind::{self as SyntaxKind, *},
    MarkdownSyntaxList as SyntaxList, MarkdownSyntaxNode as SyntaxNode,
    MarkdownSyntaxToken as SyntaxToken,
};
use biome_rowan::{
    support, AstNode, AstNodeList, AstNodeListIterator, AstNodeSlotMap, AstSeparatedList,
    AstSeparatedListNodesIterator, RawSyntaxKind, SyntaxKindSet, SyntaxResult,
};
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Formatter};
#[doc = r" Sentinel value indicating a missing element in a dynamic node, where"]
#[doc = r" the slots are not statically known."]
pub(crate) const SLOT_MAP_EMPTY_VALUE: u8 = u8::MAX;
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
            md_textual: self.md_textual(),
        }
    }
    pub fn md_textual(&self) -> SyntaxResult<MdTextual> {
        support::required_node(&self.syntax, 0usize)
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
    pub md_textual: SyntaxResult<MdTextual>,
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
            md_paragraph: self.md_paragraph(),
            after: self.after(),
        }
    }
    pub fn before(&self) -> MdHashList {
        support::list(&self.syntax, 0usize)
    }
    pub fn md_paragraph(&self) -> Option<MdParagraph> {
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
    pub md_paragraph: Option<MdParagraph>,
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
            md_textual: self.md_textual(),
        }
    }
    pub fn md_textual(&self) -> SyntaxResult<MdTextual> {
        support::required_node(&self.syntax, 0usize)
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
    pub md_textual: SyntaxResult<MdTextual>,
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
            md_textual: self.md_textual(),
        }
    }
    pub fn md_textual(&self) -> SyntaxResult<MdTextual> {
        support::required_node(&self.syntax, 0usize)
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
    pub md_textual: SyntaxResult<MdTextual>,
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
            md_textual: self.md_textual(),
        }
    }
    pub fn md_textual(&self) -> SyntaxResult<MdTextual> {
        support::required_node(&self.syntax, 0usize)
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
    pub md_textual: SyntaxResult<MdTextual>,
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
            alt: self.alt(),
            src: self.src(),
            title: self.title(),
        }
    }
    pub fn alt(&self) -> SyntaxResult<MdTextual> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn src(&self) -> SyntaxResult<MdTextual> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn title(&self) -> Option<MdTextual> {
        support::node(&self.syntax, 2usize)
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
    pub alt: SyntaxResult<MdTextual>,
    pub src: SyntaxResult<MdTextual>,
    pub title: Option<MdTextual>,
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
    pub label: SyntaxResult<MdTextual>,
    pub url: SyntaxResult<MdTextual>,
    pub title: Option<MdTextual>,
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
            md_paragraph_item_list: self.md_paragraph_item_list(),
        }
    }
    pub fn md_paragraph_item_list(&self) -> MdParagraphItemList {
        support::list(&self.syntax, 0usize)
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
    pub md_paragraph_item_list: MdParagraphItemList,
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
            AnyCodeBlock::MdFencedCodeBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_indent_code_block(&self) -> Option<&MdIndentCodeBlock> {
        match &self {
            AnyCodeBlock::MdIndentCodeBlock(item) => Some(item),
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
            AnyContainerBlock::MdBulletListItem(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_order_list_item(&self) -> Option<&MdOrderListItem> {
        match &self {
            AnyContainerBlock::MdOrderListItem(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_quote(&self) -> Option<&MdQuote> {
        match &self {
            AnyContainerBlock::MdQuote(item) => Some(item),
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
            AnyLeafBlock::AnyCodeBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_header(&self) -> Option<&MdHeader> {
        match &self {
            AnyLeafBlock::MdHeader(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_html_block(&self) -> Option<&MdHtmlBlock> {
        match &self {
            AnyLeafBlock::MdHtmlBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_link_block(&self) -> Option<&MdLinkBlock> {
        match &self {
            AnyLeafBlock::MdLinkBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_paragraph(&self) -> Option<&MdParagraph> {
        match &self {
            AnyLeafBlock::MdParagraph(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_setext_header(&self) -> Option<&MdSetextHeader> {
        match &self {
            AnyLeafBlock::MdSetextHeader(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_thematic_break_block(&self) -> Option<&MdThematicBreakBlock> {
        match &self {
            AnyLeafBlock::MdThematicBreakBlock(item) => Some(item),
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
            AnyMdBlock::AnyContainerBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_any_leaf_block(&self) -> Option<&AnyLeafBlock> {
        match &self {
            AnyMdBlock::AnyLeafBlock(item) => Some(item),
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
    MdInlineLink(MdInlineLink),
    MdSoftBreak(MdSoftBreak),
    MdTextual(MdTextual),
}
impl AnyMdInline {
    pub fn as_md_hard_line(&self) -> Option<&MdHardLine> {
        match &self {
            AnyMdInline::MdHardLine(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_html_block(&self) -> Option<&MdHtmlBlock> {
        match &self {
            AnyMdInline::MdHtmlBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_inline_code(&self) -> Option<&MdInlineCode> {
        match &self {
            AnyMdInline::MdInlineCode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_inline_emphasis(&self) -> Option<&MdInlineEmphasis> {
        match &self {
            AnyMdInline::MdInlineEmphasis(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_inline_image(&self) -> Option<&MdInlineImage> {
        match &self {
            AnyMdInline::MdInlineImage(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_inline_link(&self) -> Option<&MdInlineLink> {
        match &self {
            AnyMdInline::MdInlineLink(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_soft_break(&self) -> Option<&MdSoftBreak> {
        match &self {
            AnyMdInline::MdSoftBreak(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_textual(&self) -> Option<&MdTextual> {
        match &self {
            AnyMdInline::MdTextual(item) => Some(item),
            _ => None,
        }
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
    fn from(n: MdBulletListItem) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdBulletListItem> for SyntaxElement {
    fn from(n: MdBulletListItem) -> SyntaxElement {
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
    fn from(n: MdDocument) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdDocument> for SyntaxElement {
    fn from(n: MdDocument) -> SyntaxElement {
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
                .field("md_textual", &support::DebugSyntaxResult(self.md_textual()))
                .finish()
        } else {
            f.debug_struct("MdFencedCodeBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdFencedCodeBlock> for SyntaxNode {
    fn from(n: MdFencedCodeBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdFencedCodeBlock> for SyntaxElement {
    fn from(n: MdFencedCodeBlock) -> SyntaxElement {
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
    fn from(n: MdHardLine) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdHardLine> for SyntaxElement {
    fn from(n: MdHardLine) -> SyntaxElement {
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
    fn from(n: MdHash) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdHash> for SyntaxElement {
    fn from(n: MdHash) -> SyntaxElement {
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
                .field(
                    "md_paragraph",
                    &support::DebugOptionalElement(self.md_paragraph()),
                )
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
    fn from(n: MdHeader) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdHeader> for SyntaxElement {
    fn from(n: MdHeader) -> SyntaxElement {
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
    fn from(n: MdHtmlBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdHtmlBlock> for SyntaxElement {
    fn from(n: MdHtmlBlock) -> SyntaxElement {
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
    fn from(n: MdIndent) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdIndent> for SyntaxElement {
    fn from(n: MdIndent) -> SyntaxElement {
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
                .field("md_textual", &support::DebugSyntaxResult(self.md_textual()))
                .finish()
        } else {
            f.debug_struct("MdIndentCodeBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdIndentCodeBlock> for SyntaxNode {
    fn from(n: MdIndentCodeBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdIndentCodeBlock> for SyntaxElement {
    fn from(n: MdIndentCodeBlock) -> SyntaxElement {
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
                .field("md_textual", &support::DebugSyntaxResult(self.md_textual()))
                .finish()
        } else {
            f.debug_struct("MdInlineCode").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdInlineCode> for SyntaxNode {
    fn from(n: MdInlineCode) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdInlineCode> for SyntaxElement {
    fn from(n: MdInlineCode) -> SyntaxElement {
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
                .field("md_textual", &support::DebugSyntaxResult(self.md_textual()))
                .finish()
        } else {
            f.debug_struct("MdInlineEmphasis").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdInlineEmphasis> for SyntaxNode {
    fn from(n: MdInlineEmphasis) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdInlineEmphasis> for SyntaxElement {
    fn from(n: MdInlineEmphasis) -> SyntaxElement {
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
                .field("alt", &support::DebugSyntaxResult(self.alt()))
                .field("src", &support::DebugSyntaxResult(self.src()))
                .field("title", &support::DebugOptionalElement(self.title()))
                .finish()
        } else {
            f.debug_struct("MdInlineImage").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdInlineImage> for SyntaxNode {
    fn from(n: MdInlineImage) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdInlineImage> for SyntaxElement {
    fn from(n: MdInlineImage) -> SyntaxElement {
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
                .field("label", &support::DebugSyntaxResult(self.label()))
                .field("url", &support::DebugSyntaxResult(self.url()))
                .field("title", &support::DebugOptionalElement(self.title()))
                .finish()
        } else {
            f.debug_struct("MdInlineLink").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdInlineLink> for SyntaxNode {
    fn from(n: MdInlineLink) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdInlineLink> for SyntaxElement {
    fn from(n: MdInlineLink) -> SyntaxElement {
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
    fn from(n: MdLinkBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdLinkBlock> for SyntaxElement {
    fn from(n: MdLinkBlock) -> SyntaxElement {
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
    fn from(n: MdOrderListItem) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdOrderListItem> for SyntaxElement {
    fn from(n: MdOrderListItem) -> SyntaxElement {
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
                .field("md_paragraph_item_list", &self.md_paragraph_item_list())
                .finish()
        } else {
            f.debug_struct("MdParagraph").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MdParagraph> for SyntaxNode {
    fn from(n: MdParagraph) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdParagraph> for SyntaxElement {
    fn from(n: MdParagraph) -> SyntaxElement {
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
    fn from(n: MdQuote) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdQuote> for SyntaxElement {
    fn from(n: MdQuote) -> SyntaxElement {
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
    fn from(n: MdSetextHeader) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdSetextHeader> for SyntaxElement {
    fn from(n: MdSetextHeader) -> SyntaxElement {
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
    fn from(n: MdSoftBreak) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdSoftBreak> for SyntaxElement {
    fn from(n: MdSoftBreak) -> SyntaxElement {
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
    fn from(n: MdTextual) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdTextual> for SyntaxElement {
    fn from(n: MdTextual) -> SyntaxElement {
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
    fn from(n: MdThematicBreakBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdThematicBreakBlock> for SyntaxElement {
    fn from(n: MdThematicBreakBlock) -> SyntaxElement {
        n.syntax.into()
    }
}
impl From<MdFencedCodeBlock> for AnyCodeBlock {
    fn from(node: MdFencedCodeBlock) -> AnyCodeBlock {
        AnyCodeBlock::MdFencedCodeBlock(node)
    }
}
impl From<MdIndentCodeBlock> for AnyCodeBlock {
    fn from(node: MdIndentCodeBlock) -> AnyCodeBlock {
        AnyCodeBlock::MdIndentCodeBlock(node)
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
            MD_FENCED_CODE_BLOCK => AnyCodeBlock::MdFencedCodeBlock(MdFencedCodeBlock { syntax }),
            MD_INDENT_CODE_BLOCK => AnyCodeBlock::MdIndentCodeBlock(MdIndentCodeBlock { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCodeBlock::MdFencedCodeBlock(it) => &it.syntax,
            AnyCodeBlock::MdIndentCodeBlock(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCodeBlock::MdFencedCodeBlock(it) => it.syntax,
            AnyCodeBlock::MdIndentCodeBlock(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCodeBlock::MdFencedCodeBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyCodeBlock::MdIndentCodeBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCodeBlock> for SyntaxNode {
    fn from(n: AnyCodeBlock) -> SyntaxNode {
        match n {
            AnyCodeBlock::MdFencedCodeBlock(it) => it.into(),
            AnyCodeBlock::MdIndentCodeBlock(it) => it.into(),
        }
    }
}
impl From<AnyCodeBlock> for SyntaxElement {
    fn from(n: AnyCodeBlock) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MdBulletListItem> for AnyContainerBlock {
    fn from(node: MdBulletListItem) -> AnyContainerBlock {
        AnyContainerBlock::MdBulletListItem(node)
    }
}
impl From<MdOrderListItem> for AnyContainerBlock {
    fn from(node: MdOrderListItem) -> AnyContainerBlock {
        AnyContainerBlock::MdOrderListItem(node)
    }
}
impl From<MdQuote> for AnyContainerBlock {
    fn from(node: MdQuote) -> AnyContainerBlock {
        AnyContainerBlock::MdQuote(node)
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
            MD_BULLET_LIST_ITEM => AnyContainerBlock::MdBulletListItem(MdBulletListItem { syntax }),
            MD_ORDER_LIST_ITEM => AnyContainerBlock::MdOrderListItem(MdOrderListItem { syntax }),
            MD_QUOTE => AnyContainerBlock::MdQuote(MdQuote { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyContainerBlock::MdBulletListItem(it) => &it.syntax,
            AnyContainerBlock::MdOrderListItem(it) => &it.syntax,
            AnyContainerBlock::MdQuote(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyContainerBlock::MdBulletListItem(it) => it.syntax,
            AnyContainerBlock::MdOrderListItem(it) => it.syntax,
            AnyContainerBlock::MdQuote(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyContainerBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyContainerBlock::MdBulletListItem(it) => std::fmt::Debug::fmt(it, f),
            AnyContainerBlock::MdOrderListItem(it) => std::fmt::Debug::fmt(it, f),
            AnyContainerBlock::MdQuote(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyContainerBlock> for SyntaxNode {
    fn from(n: AnyContainerBlock) -> SyntaxNode {
        match n {
            AnyContainerBlock::MdBulletListItem(it) => it.into(),
            AnyContainerBlock::MdOrderListItem(it) => it.into(),
            AnyContainerBlock::MdQuote(it) => it.into(),
        }
    }
}
impl From<AnyContainerBlock> for SyntaxElement {
    fn from(n: AnyContainerBlock) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MdHeader> for AnyLeafBlock {
    fn from(node: MdHeader) -> AnyLeafBlock {
        AnyLeafBlock::MdHeader(node)
    }
}
impl From<MdHtmlBlock> for AnyLeafBlock {
    fn from(node: MdHtmlBlock) -> AnyLeafBlock {
        AnyLeafBlock::MdHtmlBlock(node)
    }
}
impl From<MdLinkBlock> for AnyLeafBlock {
    fn from(node: MdLinkBlock) -> AnyLeafBlock {
        AnyLeafBlock::MdLinkBlock(node)
    }
}
impl From<MdParagraph> for AnyLeafBlock {
    fn from(node: MdParagraph) -> AnyLeafBlock {
        AnyLeafBlock::MdParagraph(node)
    }
}
impl From<MdSetextHeader> for AnyLeafBlock {
    fn from(node: MdSetextHeader) -> AnyLeafBlock {
        AnyLeafBlock::MdSetextHeader(node)
    }
}
impl From<MdThematicBreakBlock> for AnyLeafBlock {
    fn from(node: MdThematicBreakBlock) -> AnyLeafBlock {
        AnyLeafBlock::MdThematicBreakBlock(node)
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
            MD_HEADER => AnyLeafBlock::MdHeader(MdHeader { syntax }),
            MD_HTML_BLOCK => AnyLeafBlock::MdHtmlBlock(MdHtmlBlock { syntax }),
            MD_LINK_BLOCK => AnyLeafBlock::MdLinkBlock(MdLinkBlock { syntax }),
            MD_PARAGRAPH => AnyLeafBlock::MdParagraph(MdParagraph { syntax }),
            MD_SETEXT_HEADER => AnyLeafBlock::MdSetextHeader(MdSetextHeader { syntax }),
            MD_THEMATIC_BREAK_BLOCK => {
                AnyLeafBlock::MdThematicBreakBlock(MdThematicBreakBlock { syntax })
            }
            _ => {
                if let Some(any_code_block) = AnyCodeBlock::cast(syntax) {
                    return Some(AnyLeafBlock::AnyCodeBlock(any_code_block));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyLeafBlock::MdHeader(it) => &it.syntax,
            AnyLeafBlock::MdHtmlBlock(it) => &it.syntax,
            AnyLeafBlock::MdLinkBlock(it) => &it.syntax,
            AnyLeafBlock::MdParagraph(it) => &it.syntax,
            AnyLeafBlock::MdSetextHeader(it) => &it.syntax,
            AnyLeafBlock::MdThematicBreakBlock(it) => &it.syntax,
            AnyLeafBlock::AnyCodeBlock(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyLeafBlock::MdHeader(it) => it.syntax,
            AnyLeafBlock::MdHtmlBlock(it) => it.syntax,
            AnyLeafBlock::MdLinkBlock(it) => it.syntax,
            AnyLeafBlock::MdParagraph(it) => it.syntax,
            AnyLeafBlock::MdSetextHeader(it) => it.syntax,
            AnyLeafBlock::MdThematicBreakBlock(it) => it.syntax,
            AnyLeafBlock::AnyCodeBlock(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyLeafBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyLeafBlock::AnyCodeBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::MdHeader(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::MdHtmlBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::MdLinkBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::MdParagraph(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::MdSetextHeader(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::MdThematicBreakBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyLeafBlock> for SyntaxNode {
    fn from(n: AnyLeafBlock) -> SyntaxNode {
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
    fn from(n: AnyLeafBlock) -> SyntaxElement {
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
                return Some(AnyMdBlock::AnyContainerBlock(any_container_block));
            }
            Err(syntax) => syntax,
        };
        if let Some(any_leaf_block) = AnyLeafBlock::cast(syntax) {
            return Some(AnyMdBlock::AnyLeafBlock(any_leaf_block));
        }
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyMdBlock::AnyContainerBlock(it) => it.syntax(),
            AnyMdBlock::AnyLeafBlock(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyMdBlock::AnyContainerBlock(it) => it.into_syntax(),
            AnyMdBlock::AnyLeafBlock(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyMdBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyMdBlock::AnyContainerBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyMdBlock::AnyLeafBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMdBlock> for SyntaxNode {
    fn from(n: AnyMdBlock) -> SyntaxNode {
        match n {
            AnyMdBlock::AnyContainerBlock(it) => it.into(),
            AnyMdBlock::AnyLeafBlock(it) => it.into(),
        }
    }
}
impl From<AnyMdBlock> for SyntaxElement {
    fn from(n: AnyMdBlock) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MdHardLine> for AnyMdInline {
    fn from(node: MdHardLine) -> AnyMdInline {
        AnyMdInline::MdHardLine(node)
    }
}
impl From<MdHtmlBlock> for AnyMdInline {
    fn from(node: MdHtmlBlock) -> AnyMdInline {
        AnyMdInline::MdHtmlBlock(node)
    }
}
impl From<MdInlineCode> for AnyMdInline {
    fn from(node: MdInlineCode) -> AnyMdInline {
        AnyMdInline::MdInlineCode(node)
    }
}
impl From<MdInlineEmphasis> for AnyMdInline {
    fn from(node: MdInlineEmphasis) -> AnyMdInline {
        AnyMdInline::MdInlineEmphasis(node)
    }
}
impl From<MdInlineImage> for AnyMdInline {
    fn from(node: MdInlineImage) -> AnyMdInline {
        AnyMdInline::MdInlineImage(node)
    }
}
impl From<MdInlineLink> for AnyMdInline {
    fn from(node: MdInlineLink) -> AnyMdInline {
        AnyMdInline::MdInlineLink(node)
    }
}
impl From<MdSoftBreak> for AnyMdInline {
    fn from(node: MdSoftBreak) -> AnyMdInline {
        AnyMdInline::MdSoftBreak(node)
    }
}
impl From<MdTextual> for AnyMdInline {
    fn from(node: MdTextual) -> AnyMdInline {
        AnyMdInline::MdTextual(node)
    }
}
impl AstNode for AnyMdInline {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MdHardLine::KIND_SET
        .union(MdHtmlBlock::KIND_SET)
        .union(MdInlineCode::KIND_SET)
        .union(MdInlineEmphasis::KIND_SET)
        .union(MdInlineImage::KIND_SET)
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
                | MD_INLINE_LINK
                | MD_SOFT_BREAK
                | MD_TEXTUAL
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MD_HARD_LINE => AnyMdInline::MdHardLine(MdHardLine { syntax }),
            MD_HTML_BLOCK => AnyMdInline::MdHtmlBlock(MdHtmlBlock { syntax }),
            MD_INLINE_CODE => AnyMdInline::MdInlineCode(MdInlineCode { syntax }),
            MD_INLINE_EMPHASIS => AnyMdInline::MdInlineEmphasis(MdInlineEmphasis { syntax }),
            MD_INLINE_IMAGE => AnyMdInline::MdInlineImage(MdInlineImage { syntax }),
            MD_INLINE_LINK => AnyMdInline::MdInlineLink(MdInlineLink { syntax }),
            MD_SOFT_BREAK => AnyMdInline::MdSoftBreak(MdSoftBreak { syntax }),
            MD_TEXTUAL => AnyMdInline::MdTextual(MdTextual { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyMdInline::MdHardLine(it) => &it.syntax,
            AnyMdInline::MdHtmlBlock(it) => &it.syntax,
            AnyMdInline::MdInlineCode(it) => &it.syntax,
            AnyMdInline::MdInlineEmphasis(it) => &it.syntax,
            AnyMdInline::MdInlineImage(it) => &it.syntax,
            AnyMdInline::MdInlineLink(it) => &it.syntax,
            AnyMdInline::MdSoftBreak(it) => &it.syntax,
            AnyMdInline::MdTextual(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyMdInline::MdHardLine(it) => it.syntax,
            AnyMdInline::MdHtmlBlock(it) => it.syntax,
            AnyMdInline::MdInlineCode(it) => it.syntax,
            AnyMdInline::MdInlineEmphasis(it) => it.syntax,
            AnyMdInline::MdInlineImage(it) => it.syntax,
            AnyMdInline::MdInlineLink(it) => it.syntax,
            AnyMdInline::MdSoftBreak(it) => it.syntax,
            AnyMdInline::MdTextual(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMdInline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyMdInline::MdHardLine(it) => std::fmt::Debug::fmt(it, f),
            AnyMdInline::MdHtmlBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyMdInline::MdInlineCode(it) => std::fmt::Debug::fmt(it, f),
            AnyMdInline::MdInlineEmphasis(it) => std::fmt::Debug::fmt(it, f),
            AnyMdInline::MdInlineImage(it) => std::fmt::Debug::fmt(it, f),
            AnyMdInline::MdInlineLink(it) => std::fmt::Debug::fmt(it, f),
            AnyMdInline::MdSoftBreak(it) => std::fmt::Debug::fmt(it, f),
            AnyMdInline::MdTextual(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMdInline> for SyntaxNode {
    fn from(n: AnyMdInline) -> SyntaxNode {
        match n {
            AnyMdInline::MdHardLine(it) => it.into(),
            AnyMdInline::MdHtmlBlock(it) => it.into(),
            AnyMdInline::MdInlineCode(it) => it.into(),
            AnyMdInline::MdInlineEmphasis(it) => it.into(),
            AnyMdInline::MdInlineImage(it) => it.into(),
            AnyMdInline::MdInlineLink(it) => it.into(),
            AnyMdInline::MdSoftBreak(it) => it.into(),
            AnyMdInline::MdTextual(it) => it.into(),
        }
    }
}
impl From<AnyMdInline> for SyntaxElement {
    fn from(n: AnyMdInline) -> SyntaxElement {
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
    fn from(n: MdBogus) -> SyntaxNode {
        n.syntax
    }
}
impl From<MdBogus> for SyntaxElement {
    fn from(n: MdBogus) -> SyntaxElement {
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
    fn cast(syntax: SyntaxNode) -> Option<MdBlockList> {
        if Self::can_cast(syntax.kind()) {
            Some(MdBlockList {
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
    fn cast(syntax: SyntaxNode) -> Option<MdBulletList> {
        if Self::can_cast(syntax.kind()) {
            Some(MdBulletList {
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
    type Node = AnyCodeBlock;
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
    type Item = AnyCodeBlock;
    type IntoIter = AstNodeListIterator<Language, AnyCodeBlock>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MdBulletList {
    type Item = AnyCodeBlock;
    type IntoIter = AstNodeListIterator<Language, AnyCodeBlock>;
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
    fn cast(syntax: SyntaxNode) -> Option<MdHashList> {
        if Self::can_cast(syntax.kind()) {
            Some(MdHashList {
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
    fn cast(syntax: SyntaxNode) -> Option<MdOrderList> {
        if Self::can_cast(syntax.kind()) {
            Some(MdOrderList {
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
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MdParagraphItemList {
    syntax_list: SyntaxList,
}
impl MdParagraphItemList {
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
impl AstNode for MdParagraphItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_PARAGRAPH_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MD_PARAGRAPH_ITEM_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<MdParagraphItemList> {
        if Self::can_cast(syntax.kind()) {
            Some(MdParagraphItemList {
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
impl Serialize for MdParagraphItemList {
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
impl AstNodeList for MdParagraphItemList {
    type Language = Language;
    type Node = AnyMdInline;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MdParagraphItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MdParagraphItemList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MdParagraphItemList {
    type Item = AnyMdInline;
    type IntoIter = AstNodeListIterator<Language, AnyMdInline>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MdParagraphItemList {
    type Item = AnyMdInline;
    type IntoIter = AstNodeListIterator<Language, AnyMdInline>;
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
