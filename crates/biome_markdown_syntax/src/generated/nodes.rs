//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
use crate::{
    macros::map_syntax_node,
    MarkdownLanguage as Language, MarkdownSyntaxElement as SyntaxElement,
    MarkdownSyntaxElementChildren as SyntaxElementChildren,
    MarkdownSyntaxKind::{self as SyntaxKind, *},
    MarkdownSyntaxList as SyntaxList, MarkdownSyntaxNode as SyntaxNode,
    MarkdownSyntaxToken as SyntaxToken,
};
use biome_rowan::{support, AstNode, RawSyntaxKind, SyntaxKindSet, SyntaxResult};
#[allow(unused)]
use biome_rowan::{
    AstNodeList, AstNodeListIterator, AstNodeSlotMap, AstSeparatedList,
    AstSeparatedListNodesIterator,
};
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Formatter};
#[doc = r" Sentinel value indicating a missing element in a dynamic node, where"]
#[doc = r" the slots are not statically known."]
#[allow(dead_code)]
pub(crate) const SLOT_MAP_EMPTY_VALUE: u8 = u8::MAX;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownBreakBlock {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownBreakBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownBreakBlockFields {
        MarkdownBreakBlockFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownBreakBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownBreakBlockFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownBulletListItem {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownBulletListItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownBulletListItemFields {
        MarkdownBulletListItemFields {
            markdown_bullet_list: self.markdown_bullet_list(),
        }
    }
    pub fn markdown_bullet_list(&self) -> MarkdownBulletList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownBulletListItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownBulletListItemFields {
    pub markdown_bullet_list: MarkdownBulletList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownDocument {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownDocument {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownDocumentFields {
        MarkdownDocumentFields {
            bom_token: self.bom_token(),
            value: self.value(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> MarkdownBlockList {
        support::list(&self.syntax, 1usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MarkdownDocument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownDocumentFields {
    pub bom_token: Option<SyntaxToken>,
    pub value: MarkdownBlockList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownFencedCodeBlock {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownFencedCodeBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownFencedCodeBlockFields {
        MarkdownFencedCodeBlockFields {
            markdown_textual: self.markdown_textual(),
        }
    }
    pub fn markdown_textual(&self) -> SyntaxResult<MarkdownTextual> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownFencedCodeBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownFencedCodeBlockFields {
    pub markdown_textual: SyntaxResult<MarkdownTextual>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownHardLine {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownHardLine {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownHardLineFields {
        MarkdownHardLineFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownHardLine {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownHardLineFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownHash {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownHash {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownHashFields {
        MarkdownHashFields {
            hash_token: self.hash_token(),
        }
    }
    pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownHashFields {
    pub hash_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownHeader {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownHeader {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownHeaderFields {
        MarkdownHeaderFields {
            before: self.before(),
            markdown_paragraph: self.markdown_paragraph(),
            after: self.after(),
        }
    }
    pub fn before(&self) -> MarkdownHashList {
        support::list(&self.syntax, 0usize)
    }
    pub fn markdown_paragraph(&self) -> Option<MarkdownParagraph> {
        support::node(&self.syntax, 1usize)
    }
    pub fn after(&self) -> MarkdownHashList {
        support::list(&self.syntax, 2usize)
    }
}
impl Serialize for MarkdownHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownHeaderFields {
    pub before: MarkdownHashList,
    pub markdown_paragraph: Option<MarkdownParagraph>,
    pub after: MarkdownHashList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownHtmlBlock {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownHtmlBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownHtmlBlockFields {
        MarkdownHtmlBlockFields {
            markdown_textual: self.markdown_textual(),
        }
    }
    pub fn markdown_textual(&self) -> SyntaxResult<MarkdownTextual> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownHtmlBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownHtmlBlockFields {
    pub markdown_textual: SyntaxResult<MarkdownTextual>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownIndent {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownIndent {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownIndentFields {
        MarkdownIndentFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownIndent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownIndentFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownIndentCodeBlock {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownIndentCodeBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownIndentCodeBlockFields {
        MarkdownIndentCodeBlockFields {
            markdown_textual: self.markdown_textual(),
        }
    }
    pub fn markdown_textual(&self) -> SyntaxResult<MarkdownTextual> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownIndentCodeBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownIndentCodeBlockFields {
    pub markdown_textual: SyntaxResult<MarkdownTextual>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownInlineCode {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownInlineCode {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownInlineCodeFields {
        MarkdownInlineCodeFields {
            markdown_textual: self.markdown_textual(),
        }
    }
    pub fn markdown_textual(&self) -> SyntaxResult<MarkdownTextual> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownInlineCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownInlineCodeFields {
    pub markdown_textual: SyntaxResult<MarkdownTextual>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownInlineEmphasis {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownInlineEmphasis {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownInlineEmphasisFields {
        MarkdownInlineEmphasisFields {
            markdown_textual: self.markdown_textual(),
        }
    }
    pub fn markdown_textual(&self) -> SyntaxResult<MarkdownTextual> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownInlineEmphasis {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownInlineEmphasisFields {
    pub markdown_textual: SyntaxResult<MarkdownTextual>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownInlineImage {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownInlineImage {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownInlineImageFields {
        MarkdownInlineImageFields {
            alt: self.alt(),
            src: self.src(),
            title: self.title(),
        }
    }
    pub fn alt(&self) -> SyntaxResult<MarkdownTextual> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn src(&self) -> SyntaxResult<MarkdownTextual> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn title(&self) -> Option<MarkdownTextual> {
        support::node(&self.syntax, 2usize)
    }
}
impl Serialize for MarkdownInlineImage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownInlineImageFields {
    pub alt: SyntaxResult<MarkdownTextual>,
    pub src: SyntaxResult<MarkdownTextual>,
    pub title: Option<MarkdownTextual>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownInlineLink {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownInlineLink {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownInlineLinkFields {
        MarkdownInlineLinkFields {
            label: self.label(),
            url: self.url(),
            title: self.title(),
        }
    }
    pub fn label(&self) -> SyntaxResult<MarkdownTextual> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn url(&self) -> SyntaxResult<MarkdownTextual> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn title(&self) -> Option<MarkdownTextual> {
        support::node(&self.syntax, 2usize)
    }
}
impl Serialize for MarkdownInlineLink {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownInlineLinkFields {
    pub label: SyntaxResult<MarkdownTextual>,
    pub url: SyntaxResult<MarkdownTextual>,
    pub title: Option<MarkdownTextual>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownLinkBlock {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownLinkBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownLinkBlockFields {
        MarkdownLinkBlockFields {
            label: self.label(),
            url: self.url(),
            title: self.title(),
        }
    }
    pub fn label(&self) -> SyntaxResult<MarkdownTextual> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn url(&self) -> SyntaxResult<MarkdownTextual> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn title(&self) -> Option<MarkdownTextual> {
        support::node(&self.syntax, 2usize)
    }
}
impl Serialize for MarkdownLinkBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownLinkBlockFields {
    pub label: SyntaxResult<MarkdownTextual>,
    pub url: SyntaxResult<MarkdownTextual>,
    pub title: Option<MarkdownTextual>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownOrderListItem {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownOrderListItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownOrderListItemFields {
        MarkdownOrderListItemFields {
            markdown_bullet_list: self.markdown_bullet_list(),
        }
    }
    pub fn markdown_bullet_list(&self) -> MarkdownBulletList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownOrderListItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownOrderListItemFields {
    pub markdown_bullet_list: MarkdownBulletList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownParagraph {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownParagraph {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownParagraphFields {
        MarkdownParagraphFields {
            markdown_paragraph_item_list: self.markdown_paragraph_item_list(),
        }
    }
    pub fn markdown_paragraph_item_list(&self) -> MarkdownParagraphItemList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownParagraph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownParagraphFields {
    pub markdown_paragraph_item_list: MarkdownParagraphItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownQuote {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownQuote {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownQuoteFields {
        MarkdownQuoteFields {
            any_markdown_block: self.any_markdown_block(),
        }
    }
    pub fn any_markdown_block(&self) -> SyntaxResult<AnyMarkdownBlock> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownQuote {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownQuoteFields {
    pub any_markdown_block: SyntaxResult<AnyMarkdownBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownSetextHeader {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownSetextHeader {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownSetextHeaderFields {
        MarkdownSetextHeaderFields {
            markdown_paragraph: self.markdown_paragraph(),
        }
    }
    pub fn markdown_paragraph(&self) -> SyntaxResult<MarkdownParagraph> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownSetextHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownSetextHeaderFields {
    pub markdown_paragraph: SyntaxResult<MarkdownParagraph>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownSoftBreak {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownSoftBreak {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownSoftBreakFields {
        MarkdownSoftBreakFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownSoftBreak {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownSoftBreakFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownTextual {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownTextual {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownTextualFields {
        MarkdownTextualFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownTextual {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownTextualFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCodeBlock {
    MarkdownFencedCodeBlock(MarkdownFencedCodeBlock),
    MarkdownIndentCodeBlock(MarkdownIndentCodeBlock),
}
impl AnyCodeBlock {
    pub fn as_markdown_fenced_code_block(&self) -> Option<&MarkdownFencedCodeBlock> {
        match &self {
            AnyCodeBlock::MarkdownFencedCodeBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_indent_code_block(&self) -> Option<&MarkdownIndentCodeBlock> {
        match &self {
            AnyCodeBlock::MarkdownIndentCodeBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyContainerBlock {
    MarkdownBulletListItem(MarkdownBulletListItem),
    MarkdownOrderListItem(MarkdownOrderListItem),
    MarkdownQuote(MarkdownQuote),
}
impl AnyContainerBlock {
    pub fn as_markdown_bullet_list_item(&self) -> Option<&MarkdownBulletListItem> {
        match &self {
            AnyContainerBlock::MarkdownBulletListItem(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_order_list_item(&self) -> Option<&MarkdownOrderListItem> {
        match &self {
            AnyContainerBlock::MarkdownOrderListItem(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_quote(&self) -> Option<&MarkdownQuote> {
        match &self {
            AnyContainerBlock::MarkdownQuote(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyLeafBlock {
    AnyCodeBlock(AnyCodeBlock),
    MarkdownBreakBlock(MarkdownBreakBlock),
    MarkdownHeader(MarkdownHeader),
    MarkdownHtmlBlock(MarkdownHtmlBlock),
    MarkdownLinkBlock(MarkdownLinkBlock),
    MarkdownParagraph(MarkdownParagraph),
    MarkdownSetextHeader(MarkdownSetextHeader),
}
impl AnyLeafBlock {
    pub fn as_any_code_block(&self) -> Option<&AnyCodeBlock> {
        match &self {
            AnyLeafBlock::AnyCodeBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_break_block(&self) -> Option<&MarkdownBreakBlock> {
        match &self {
            AnyLeafBlock::MarkdownBreakBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_header(&self) -> Option<&MarkdownHeader> {
        match &self {
            AnyLeafBlock::MarkdownHeader(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_html_block(&self) -> Option<&MarkdownHtmlBlock> {
        match &self {
            AnyLeafBlock::MarkdownHtmlBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_link_block(&self) -> Option<&MarkdownLinkBlock> {
        match &self {
            AnyLeafBlock::MarkdownLinkBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_paragraph(&self) -> Option<&MarkdownParagraph> {
        match &self {
            AnyLeafBlock::MarkdownParagraph(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_setext_header(&self) -> Option<&MarkdownSetextHeader> {
        match &self {
            AnyLeafBlock::MarkdownSetextHeader(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMarkdownBlock {
    AnyContainerBlock(AnyContainerBlock),
    AnyLeafBlock(AnyLeafBlock),
}
impl AnyMarkdownBlock {
    pub fn as_any_container_block(&self) -> Option<&AnyContainerBlock> {
        match &self {
            AnyMarkdownBlock::AnyContainerBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_any_leaf_block(&self) -> Option<&AnyLeafBlock> {
        match &self {
            AnyMarkdownBlock::AnyLeafBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMarkdownInline {
    MarkdownHardLine(MarkdownHardLine),
    MarkdownHtmlBlock(MarkdownHtmlBlock),
    MarkdownInlineCode(MarkdownInlineCode),
    MarkdownInlineEmphasis(MarkdownInlineEmphasis),
    MarkdownInlineImage(MarkdownInlineImage),
    MarkdownInlineLink(MarkdownInlineLink),
    MarkdownSoftBreak(MarkdownSoftBreak),
    MarkdownTextual(MarkdownTextual),
}
impl AnyMarkdownInline {
    pub fn as_markdown_hard_line(&self) -> Option<&MarkdownHardLine> {
        match &self {
            AnyMarkdownInline::MarkdownHardLine(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_html_block(&self) -> Option<&MarkdownHtmlBlock> {
        match &self {
            AnyMarkdownInline::MarkdownHtmlBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_inline_code(&self) -> Option<&MarkdownInlineCode> {
        match &self {
            AnyMarkdownInline::MarkdownInlineCode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_inline_emphasis(&self) -> Option<&MarkdownInlineEmphasis> {
        match &self {
            AnyMarkdownInline::MarkdownInlineEmphasis(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_inline_image(&self) -> Option<&MarkdownInlineImage> {
        match &self {
            AnyMarkdownInline::MarkdownInlineImage(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_inline_link(&self) -> Option<&MarkdownInlineLink> {
        match &self {
            AnyMarkdownInline::MarkdownInlineLink(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_soft_break(&self) -> Option<&MarkdownSoftBreak> {
        match &self {
            AnyMarkdownInline::MarkdownSoftBreak(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_textual(&self) -> Option<&MarkdownTextual> {
        match &self {
            AnyMarkdownInline::MarkdownTextual(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for MarkdownBreakBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_BREAK_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_BREAK_BLOCK
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
impl std::fmt::Debug for MarkdownBreakBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownBreakBlock")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<MarkdownBreakBlock> for SyntaxNode {
    fn from(n: MarkdownBreakBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownBreakBlock> for SyntaxElement {
    fn from(n: MarkdownBreakBlock) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownBulletListItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_BULLET_LIST_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_BULLET_LIST_ITEM
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
impl std::fmt::Debug for MarkdownBulletListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownBulletListItem")
            .field("markdown_bullet_list", &self.markdown_bullet_list())
            .finish()
    }
}
impl From<MarkdownBulletListItem> for SyntaxNode {
    fn from(n: MarkdownBulletListItem) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownBulletListItem> for SyntaxElement {
    fn from(n: MarkdownBulletListItem) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownDocument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_DOCUMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_DOCUMENT
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
impl std::fmt::Debug for MarkdownDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownDocument")
            .field(
                "bom_token",
                &support::DebugOptionalElement(self.bom_token()),
            )
            .field("value", &self.value())
            .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
            .finish()
    }
}
impl From<MarkdownDocument> for SyntaxNode {
    fn from(n: MarkdownDocument) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownDocument> for SyntaxElement {
    fn from(n: MarkdownDocument) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownFencedCodeBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_FENCED_CODE_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_FENCED_CODE_BLOCK
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
impl std::fmt::Debug for MarkdownFencedCodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownFencedCodeBlock")
            .field(
                "markdown_textual",
                &support::DebugSyntaxResult(self.markdown_textual()),
            )
            .finish()
    }
}
impl From<MarkdownFencedCodeBlock> for SyntaxNode {
    fn from(n: MarkdownFencedCodeBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownFencedCodeBlock> for SyntaxElement {
    fn from(n: MarkdownFencedCodeBlock) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownHardLine {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_HARD_LINE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_HARD_LINE
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
impl std::fmt::Debug for MarkdownHardLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownHardLine")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<MarkdownHardLine> for SyntaxNode {
    fn from(n: MarkdownHardLine) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownHardLine> for SyntaxElement {
    fn from(n: MarkdownHardLine) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownHash {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_HASH as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_HASH
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
impl std::fmt::Debug for MarkdownHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownHash")
            .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
            .finish()
    }
}
impl From<MarkdownHash> for SyntaxNode {
    fn from(n: MarkdownHash) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownHash> for SyntaxElement {
    fn from(n: MarkdownHash) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownHeader {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_HEADER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_HEADER
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
impl std::fmt::Debug for MarkdownHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownHeader")
            .field("before", &self.before())
            .field(
                "markdown_paragraph",
                &support::DebugOptionalElement(self.markdown_paragraph()),
            )
            .field("after", &self.after())
            .finish()
    }
}
impl From<MarkdownHeader> for SyntaxNode {
    fn from(n: MarkdownHeader) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownHeader> for SyntaxElement {
    fn from(n: MarkdownHeader) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownHtmlBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_HTML_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_HTML_BLOCK
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
impl std::fmt::Debug for MarkdownHtmlBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownHtmlBlock")
            .field(
                "markdown_textual",
                &support::DebugSyntaxResult(self.markdown_textual()),
            )
            .finish()
    }
}
impl From<MarkdownHtmlBlock> for SyntaxNode {
    fn from(n: MarkdownHtmlBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownHtmlBlock> for SyntaxElement {
    fn from(n: MarkdownHtmlBlock) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownIndent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_INDENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_INDENT
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
impl std::fmt::Debug for MarkdownIndent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownIndent")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<MarkdownIndent> for SyntaxNode {
    fn from(n: MarkdownIndent) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownIndent> for SyntaxElement {
    fn from(n: MarkdownIndent) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownIndentCodeBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_INDENT_CODE_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_INDENT_CODE_BLOCK
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
impl std::fmt::Debug for MarkdownIndentCodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownIndentCodeBlock")
            .field(
                "markdown_textual",
                &support::DebugSyntaxResult(self.markdown_textual()),
            )
            .finish()
    }
}
impl From<MarkdownIndentCodeBlock> for SyntaxNode {
    fn from(n: MarkdownIndentCodeBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownIndentCodeBlock> for SyntaxElement {
    fn from(n: MarkdownIndentCodeBlock) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownInlineCode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_INLINE_CODE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_INLINE_CODE
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
impl std::fmt::Debug for MarkdownInlineCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownInlineCode")
            .field(
                "markdown_textual",
                &support::DebugSyntaxResult(self.markdown_textual()),
            )
            .finish()
    }
}
impl From<MarkdownInlineCode> for SyntaxNode {
    fn from(n: MarkdownInlineCode) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownInlineCode> for SyntaxElement {
    fn from(n: MarkdownInlineCode) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownInlineEmphasis {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_INLINE_EMPHASIS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_INLINE_EMPHASIS
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
impl std::fmt::Debug for MarkdownInlineEmphasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownInlineEmphasis")
            .field(
                "markdown_textual",
                &support::DebugSyntaxResult(self.markdown_textual()),
            )
            .finish()
    }
}
impl From<MarkdownInlineEmphasis> for SyntaxNode {
    fn from(n: MarkdownInlineEmphasis) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownInlineEmphasis> for SyntaxElement {
    fn from(n: MarkdownInlineEmphasis) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownInlineImage {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_INLINE_IMAGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_INLINE_IMAGE
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
impl std::fmt::Debug for MarkdownInlineImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownInlineImage")
            .field("alt", &support::DebugSyntaxResult(self.alt()))
            .field("src", &support::DebugSyntaxResult(self.src()))
            .field("title", &support::DebugOptionalElement(self.title()))
            .finish()
    }
}
impl From<MarkdownInlineImage> for SyntaxNode {
    fn from(n: MarkdownInlineImage) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownInlineImage> for SyntaxElement {
    fn from(n: MarkdownInlineImage) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownInlineLink {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_INLINE_LINK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_INLINE_LINK
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
impl std::fmt::Debug for MarkdownInlineLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownInlineLink")
            .field("label", &support::DebugSyntaxResult(self.label()))
            .field("url", &support::DebugSyntaxResult(self.url()))
            .field("title", &support::DebugOptionalElement(self.title()))
            .finish()
    }
}
impl From<MarkdownInlineLink> for SyntaxNode {
    fn from(n: MarkdownInlineLink) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownInlineLink> for SyntaxElement {
    fn from(n: MarkdownInlineLink) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownLinkBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_LINK_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_LINK_BLOCK
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
impl std::fmt::Debug for MarkdownLinkBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownLinkBlock")
            .field("label", &support::DebugSyntaxResult(self.label()))
            .field("url", &support::DebugSyntaxResult(self.url()))
            .field("title", &support::DebugOptionalElement(self.title()))
            .finish()
    }
}
impl From<MarkdownLinkBlock> for SyntaxNode {
    fn from(n: MarkdownLinkBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownLinkBlock> for SyntaxElement {
    fn from(n: MarkdownLinkBlock) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownOrderListItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_ORDER_LIST_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_ORDER_LIST_ITEM
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
impl std::fmt::Debug for MarkdownOrderListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownOrderListItem")
            .field("markdown_bullet_list", &self.markdown_bullet_list())
            .finish()
    }
}
impl From<MarkdownOrderListItem> for SyntaxNode {
    fn from(n: MarkdownOrderListItem) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownOrderListItem> for SyntaxElement {
    fn from(n: MarkdownOrderListItem) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownParagraph {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_PARAGRAPH as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_PARAGRAPH
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
impl std::fmt::Debug for MarkdownParagraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownParagraph")
            .field(
                "markdown_paragraph_item_list",
                &self.markdown_paragraph_item_list(),
            )
            .finish()
    }
}
impl From<MarkdownParagraph> for SyntaxNode {
    fn from(n: MarkdownParagraph) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownParagraph> for SyntaxElement {
    fn from(n: MarkdownParagraph) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownQuote {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_QUOTE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_QUOTE
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
impl std::fmt::Debug for MarkdownQuote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownQuote")
            .field(
                "any_markdown_block",
                &support::DebugSyntaxResult(self.any_markdown_block()),
            )
            .finish()
    }
}
impl From<MarkdownQuote> for SyntaxNode {
    fn from(n: MarkdownQuote) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownQuote> for SyntaxElement {
    fn from(n: MarkdownQuote) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownSetextHeader {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_SETEXT_HEADER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_SETEXT_HEADER
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
impl std::fmt::Debug for MarkdownSetextHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownSetextHeader")
            .field(
                "markdown_paragraph",
                &support::DebugSyntaxResult(self.markdown_paragraph()),
            )
            .finish()
    }
}
impl From<MarkdownSetextHeader> for SyntaxNode {
    fn from(n: MarkdownSetextHeader) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownSetextHeader> for SyntaxElement {
    fn from(n: MarkdownSetextHeader) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownSoftBreak {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_SOFT_BREAK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_SOFT_BREAK
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
impl std::fmt::Debug for MarkdownSoftBreak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownSoftBreak")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<MarkdownSoftBreak> for SyntaxNode {
    fn from(n: MarkdownSoftBreak) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownSoftBreak> for SyntaxElement {
    fn from(n: MarkdownSoftBreak) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownTextual {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_TEXTUAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_TEXTUAL
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
impl std::fmt::Debug for MarkdownTextual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownTextual")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<MarkdownTextual> for SyntaxNode {
    fn from(n: MarkdownTextual) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownTextual> for SyntaxElement {
    fn from(n: MarkdownTextual) -> SyntaxElement {
        n.syntax.into()
    }
}
impl From<MarkdownFencedCodeBlock> for AnyCodeBlock {
    fn from(node: MarkdownFencedCodeBlock) -> AnyCodeBlock {
        AnyCodeBlock::MarkdownFencedCodeBlock(node)
    }
}
impl From<MarkdownIndentCodeBlock> for AnyCodeBlock {
    fn from(node: MarkdownIndentCodeBlock) -> AnyCodeBlock {
        AnyCodeBlock::MarkdownIndentCodeBlock(node)
    }
}
impl AstNode for AnyCodeBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        MarkdownFencedCodeBlock::KIND_SET.union(MarkdownIndentCodeBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            MARKDOWN_FENCED_CODE_BLOCK | MARKDOWN_INDENT_CODE_BLOCK
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MARKDOWN_FENCED_CODE_BLOCK => {
                AnyCodeBlock::MarkdownFencedCodeBlock(MarkdownFencedCodeBlock { syntax })
            }
            MARKDOWN_INDENT_CODE_BLOCK => {
                AnyCodeBlock::MarkdownIndentCodeBlock(MarkdownIndentCodeBlock { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCodeBlock::MarkdownFencedCodeBlock(it) => &it.syntax,
            AnyCodeBlock::MarkdownIndentCodeBlock(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCodeBlock::MarkdownFencedCodeBlock(it) => it.syntax,
            AnyCodeBlock::MarkdownIndentCodeBlock(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCodeBlock::MarkdownFencedCodeBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyCodeBlock::MarkdownIndentCodeBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCodeBlock> for SyntaxNode {
    fn from(n: AnyCodeBlock) -> SyntaxNode {
        match n {
            AnyCodeBlock::MarkdownFencedCodeBlock(it) => it.into(),
            AnyCodeBlock::MarkdownIndentCodeBlock(it) => it.into(),
        }
    }
}
impl From<AnyCodeBlock> for SyntaxElement {
    fn from(n: AnyCodeBlock) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MarkdownBulletListItem> for AnyContainerBlock {
    fn from(node: MarkdownBulletListItem) -> AnyContainerBlock {
        AnyContainerBlock::MarkdownBulletListItem(node)
    }
}
impl From<MarkdownOrderListItem> for AnyContainerBlock {
    fn from(node: MarkdownOrderListItem) -> AnyContainerBlock {
        AnyContainerBlock::MarkdownOrderListItem(node)
    }
}
impl From<MarkdownQuote> for AnyContainerBlock {
    fn from(node: MarkdownQuote) -> AnyContainerBlock {
        AnyContainerBlock::MarkdownQuote(node)
    }
}
impl AstNode for AnyContainerBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MarkdownBulletListItem::KIND_SET
        .union(MarkdownOrderListItem::KIND_SET)
        .union(MarkdownQuote::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            MARKDOWN_BULLET_LIST_ITEM | MARKDOWN_ORDER_LIST_ITEM | MARKDOWN_QUOTE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MARKDOWN_BULLET_LIST_ITEM => {
                AnyContainerBlock::MarkdownBulletListItem(MarkdownBulletListItem { syntax })
            }
            MARKDOWN_ORDER_LIST_ITEM => {
                AnyContainerBlock::MarkdownOrderListItem(MarkdownOrderListItem { syntax })
            }
            MARKDOWN_QUOTE => AnyContainerBlock::MarkdownQuote(MarkdownQuote { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyContainerBlock::MarkdownBulletListItem(it) => &it.syntax,
            AnyContainerBlock::MarkdownOrderListItem(it) => &it.syntax,
            AnyContainerBlock::MarkdownQuote(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyContainerBlock::MarkdownBulletListItem(it) => it.syntax,
            AnyContainerBlock::MarkdownOrderListItem(it) => it.syntax,
            AnyContainerBlock::MarkdownQuote(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyContainerBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyContainerBlock::MarkdownBulletListItem(it) => std::fmt::Debug::fmt(it, f),
            AnyContainerBlock::MarkdownOrderListItem(it) => std::fmt::Debug::fmt(it, f),
            AnyContainerBlock::MarkdownQuote(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyContainerBlock> for SyntaxNode {
    fn from(n: AnyContainerBlock) -> SyntaxNode {
        match n {
            AnyContainerBlock::MarkdownBulletListItem(it) => it.into(),
            AnyContainerBlock::MarkdownOrderListItem(it) => it.into(),
            AnyContainerBlock::MarkdownQuote(it) => it.into(),
        }
    }
}
impl From<AnyContainerBlock> for SyntaxElement {
    fn from(n: AnyContainerBlock) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MarkdownBreakBlock> for AnyLeafBlock {
    fn from(node: MarkdownBreakBlock) -> AnyLeafBlock {
        AnyLeafBlock::MarkdownBreakBlock(node)
    }
}
impl From<MarkdownHeader> for AnyLeafBlock {
    fn from(node: MarkdownHeader) -> AnyLeafBlock {
        AnyLeafBlock::MarkdownHeader(node)
    }
}
impl From<MarkdownHtmlBlock> for AnyLeafBlock {
    fn from(node: MarkdownHtmlBlock) -> AnyLeafBlock {
        AnyLeafBlock::MarkdownHtmlBlock(node)
    }
}
impl From<MarkdownLinkBlock> for AnyLeafBlock {
    fn from(node: MarkdownLinkBlock) -> AnyLeafBlock {
        AnyLeafBlock::MarkdownLinkBlock(node)
    }
}
impl From<MarkdownParagraph> for AnyLeafBlock {
    fn from(node: MarkdownParagraph) -> AnyLeafBlock {
        AnyLeafBlock::MarkdownParagraph(node)
    }
}
impl From<MarkdownSetextHeader> for AnyLeafBlock {
    fn from(node: MarkdownSetextHeader) -> AnyLeafBlock {
        AnyLeafBlock::MarkdownSetextHeader(node)
    }
}
impl AstNode for AnyLeafBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCodeBlock::KIND_SET
        .union(MarkdownBreakBlock::KIND_SET)
        .union(MarkdownHeader::KIND_SET)
        .union(MarkdownHtmlBlock::KIND_SET)
        .union(MarkdownLinkBlock::KIND_SET)
        .union(MarkdownParagraph::KIND_SET)
        .union(MarkdownSetextHeader::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MARKDOWN_BREAK_BLOCK
            | MARKDOWN_HEADER
            | MARKDOWN_HTML_BLOCK
            | MARKDOWN_LINK_BLOCK
            | MARKDOWN_PARAGRAPH
            | MARKDOWN_SETEXT_HEADER => true,
            k if AnyCodeBlock::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MARKDOWN_BREAK_BLOCK => AnyLeafBlock::MarkdownBreakBlock(MarkdownBreakBlock { syntax }),
            MARKDOWN_HEADER => AnyLeafBlock::MarkdownHeader(MarkdownHeader { syntax }),
            MARKDOWN_HTML_BLOCK => AnyLeafBlock::MarkdownHtmlBlock(MarkdownHtmlBlock { syntax }),
            MARKDOWN_LINK_BLOCK => AnyLeafBlock::MarkdownLinkBlock(MarkdownLinkBlock { syntax }),
            MARKDOWN_PARAGRAPH => AnyLeafBlock::MarkdownParagraph(MarkdownParagraph { syntax }),
            MARKDOWN_SETEXT_HEADER => {
                AnyLeafBlock::MarkdownSetextHeader(MarkdownSetextHeader { syntax })
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
            AnyLeafBlock::MarkdownBreakBlock(it) => &it.syntax,
            AnyLeafBlock::MarkdownHeader(it) => &it.syntax,
            AnyLeafBlock::MarkdownHtmlBlock(it) => &it.syntax,
            AnyLeafBlock::MarkdownLinkBlock(it) => &it.syntax,
            AnyLeafBlock::MarkdownParagraph(it) => &it.syntax,
            AnyLeafBlock::MarkdownSetextHeader(it) => &it.syntax,
            AnyLeafBlock::AnyCodeBlock(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyLeafBlock::MarkdownBreakBlock(it) => it.syntax,
            AnyLeafBlock::MarkdownHeader(it) => it.syntax,
            AnyLeafBlock::MarkdownHtmlBlock(it) => it.syntax,
            AnyLeafBlock::MarkdownLinkBlock(it) => it.syntax,
            AnyLeafBlock::MarkdownParagraph(it) => it.syntax,
            AnyLeafBlock::MarkdownSetextHeader(it) => it.syntax,
            AnyLeafBlock::AnyCodeBlock(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyLeafBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyLeafBlock::AnyCodeBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::MarkdownBreakBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::MarkdownHeader(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::MarkdownHtmlBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::MarkdownLinkBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::MarkdownParagraph(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::MarkdownSetextHeader(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyLeafBlock> for SyntaxNode {
    fn from(n: AnyLeafBlock) -> SyntaxNode {
        match n {
            AnyLeafBlock::AnyCodeBlock(it) => it.into(),
            AnyLeafBlock::MarkdownBreakBlock(it) => it.into(),
            AnyLeafBlock::MarkdownHeader(it) => it.into(),
            AnyLeafBlock::MarkdownHtmlBlock(it) => it.into(),
            AnyLeafBlock::MarkdownLinkBlock(it) => it.into(),
            AnyLeafBlock::MarkdownParagraph(it) => it.into(),
            AnyLeafBlock::MarkdownSetextHeader(it) => it.into(),
        }
    }
}
impl From<AnyLeafBlock> for SyntaxElement {
    fn from(n: AnyLeafBlock) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl AstNode for AnyMarkdownBlock {
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
                return Some(AnyMarkdownBlock::AnyContainerBlock(any_container_block));
            }
            Err(syntax) => syntax,
        };
        if let Some(any_leaf_block) = AnyLeafBlock::cast(syntax) {
            return Some(AnyMarkdownBlock::AnyLeafBlock(any_leaf_block));
        }
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyMarkdownBlock::AnyContainerBlock(it) => it.syntax(),
            AnyMarkdownBlock::AnyLeafBlock(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyMarkdownBlock::AnyContainerBlock(it) => it.into_syntax(),
            AnyMarkdownBlock::AnyLeafBlock(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyMarkdownBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyMarkdownBlock::AnyContainerBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyMarkdownBlock::AnyLeafBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMarkdownBlock> for SyntaxNode {
    fn from(n: AnyMarkdownBlock) -> SyntaxNode {
        match n {
            AnyMarkdownBlock::AnyContainerBlock(it) => it.into(),
            AnyMarkdownBlock::AnyLeafBlock(it) => it.into(),
        }
    }
}
impl From<AnyMarkdownBlock> for SyntaxElement {
    fn from(n: AnyMarkdownBlock) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MarkdownHardLine> for AnyMarkdownInline {
    fn from(node: MarkdownHardLine) -> AnyMarkdownInline {
        AnyMarkdownInline::MarkdownHardLine(node)
    }
}
impl From<MarkdownHtmlBlock> for AnyMarkdownInline {
    fn from(node: MarkdownHtmlBlock) -> AnyMarkdownInline {
        AnyMarkdownInline::MarkdownHtmlBlock(node)
    }
}
impl From<MarkdownInlineCode> for AnyMarkdownInline {
    fn from(node: MarkdownInlineCode) -> AnyMarkdownInline {
        AnyMarkdownInline::MarkdownInlineCode(node)
    }
}
impl From<MarkdownInlineEmphasis> for AnyMarkdownInline {
    fn from(node: MarkdownInlineEmphasis) -> AnyMarkdownInline {
        AnyMarkdownInline::MarkdownInlineEmphasis(node)
    }
}
impl From<MarkdownInlineImage> for AnyMarkdownInline {
    fn from(node: MarkdownInlineImage) -> AnyMarkdownInline {
        AnyMarkdownInline::MarkdownInlineImage(node)
    }
}
impl From<MarkdownInlineLink> for AnyMarkdownInline {
    fn from(node: MarkdownInlineLink) -> AnyMarkdownInline {
        AnyMarkdownInline::MarkdownInlineLink(node)
    }
}
impl From<MarkdownSoftBreak> for AnyMarkdownInline {
    fn from(node: MarkdownSoftBreak) -> AnyMarkdownInline {
        AnyMarkdownInline::MarkdownSoftBreak(node)
    }
}
impl From<MarkdownTextual> for AnyMarkdownInline {
    fn from(node: MarkdownTextual) -> AnyMarkdownInline {
        AnyMarkdownInline::MarkdownTextual(node)
    }
}
impl AstNode for AnyMarkdownInline {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MarkdownHardLine::KIND_SET
        .union(MarkdownHtmlBlock::KIND_SET)
        .union(MarkdownInlineCode::KIND_SET)
        .union(MarkdownInlineEmphasis::KIND_SET)
        .union(MarkdownInlineImage::KIND_SET)
        .union(MarkdownInlineLink::KIND_SET)
        .union(MarkdownSoftBreak::KIND_SET)
        .union(MarkdownTextual::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            MARKDOWN_HARD_LINE
                | MARKDOWN_HTML_BLOCK
                | MARKDOWN_INLINE_CODE
                | MARKDOWN_INLINE_EMPHASIS
                | MARKDOWN_INLINE_IMAGE
                | MARKDOWN_INLINE_LINK
                | MARKDOWN_SOFT_BREAK
                | MARKDOWN_TEXTUAL
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MARKDOWN_HARD_LINE => AnyMarkdownInline::MarkdownHardLine(MarkdownHardLine { syntax }),
            MARKDOWN_HTML_BLOCK => {
                AnyMarkdownInline::MarkdownHtmlBlock(MarkdownHtmlBlock { syntax })
            }
            MARKDOWN_INLINE_CODE => {
                AnyMarkdownInline::MarkdownInlineCode(MarkdownInlineCode { syntax })
            }
            MARKDOWN_INLINE_EMPHASIS => {
                AnyMarkdownInline::MarkdownInlineEmphasis(MarkdownInlineEmphasis { syntax })
            }
            MARKDOWN_INLINE_IMAGE => {
                AnyMarkdownInline::MarkdownInlineImage(MarkdownInlineImage { syntax })
            }
            MARKDOWN_INLINE_LINK => {
                AnyMarkdownInline::MarkdownInlineLink(MarkdownInlineLink { syntax })
            }
            MARKDOWN_SOFT_BREAK => {
                AnyMarkdownInline::MarkdownSoftBreak(MarkdownSoftBreak { syntax })
            }
            MARKDOWN_TEXTUAL => AnyMarkdownInline::MarkdownTextual(MarkdownTextual { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyMarkdownInline::MarkdownHardLine(it) => &it.syntax,
            AnyMarkdownInline::MarkdownHtmlBlock(it) => &it.syntax,
            AnyMarkdownInline::MarkdownInlineCode(it) => &it.syntax,
            AnyMarkdownInline::MarkdownInlineEmphasis(it) => &it.syntax,
            AnyMarkdownInline::MarkdownInlineImage(it) => &it.syntax,
            AnyMarkdownInline::MarkdownInlineLink(it) => &it.syntax,
            AnyMarkdownInline::MarkdownSoftBreak(it) => &it.syntax,
            AnyMarkdownInline::MarkdownTextual(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyMarkdownInline::MarkdownHardLine(it) => it.syntax,
            AnyMarkdownInline::MarkdownHtmlBlock(it) => it.syntax,
            AnyMarkdownInline::MarkdownInlineCode(it) => it.syntax,
            AnyMarkdownInline::MarkdownInlineEmphasis(it) => it.syntax,
            AnyMarkdownInline::MarkdownInlineImage(it) => it.syntax,
            AnyMarkdownInline::MarkdownInlineLink(it) => it.syntax,
            AnyMarkdownInline::MarkdownSoftBreak(it) => it.syntax,
            AnyMarkdownInline::MarkdownTextual(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMarkdownInline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyMarkdownInline::MarkdownHardLine(it) => std::fmt::Debug::fmt(it, f),
            AnyMarkdownInline::MarkdownHtmlBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyMarkdownInline::MarkdownInlineCode(it) => std::fmt::Debug::fmt(it, f),
            AnyMarkdownInline::MarkdownInlineEmphasis(it) => std::fmt::Debug::fmt(it, f),
            AnyMarkdownInline::MarkdownInlineImage(it) => std::fmt::Debug::fmt(it, f),
            AnyMarkdownInline::MarkdownInlineLink(it) => std::fmt::Debug::fmt(it, f),
            AnyMarkdownInline::MarkdownSoftBreak(it) => std::fmt::Debug::fmt(it, f),
            AnyMarkdownInline::MarkdownTextual(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMarkdownInline> for SyntaxNode {
    fn from(n: AnyMarkdownInline) -> SyntaxNode {
        match n {
            AnyMarkdownInline::MarkdownHardLine(it) => it.into(),
            AnyMarkdownInline::MarkdownHtmlBlock(it) => it.into(),
            AnyMarkdownInline::MarkdownInlineCode(it) => it.into(),
            AnyMarkdownInline::MarkdownInlineEmphasis(it) => it.into(),
            AnyMarkdownInline::MarkdownInlineImage(it) => it.into(),
            AnyMarkdownInline::MarkdownInlineLink(it) => it.into(),
            AnyMarkdownInline::MarkdownSoftBreak(it) => it.into(),
            AnyMarkdownInline::MarkdownTextual(it) => it.into(),
        }
    }
}
impl From<AnyMarkdownInline> for SyntaxElement {
    fn from(n: AnyMarkdownInline) -> SyntaxElement {
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
impl std::fmt::Display for AnyMarkdownBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMarkdownInline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownBreakBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownBulletListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownFencedCodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownHardLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownHtmlBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownIndent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownIndentCodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownInlineCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownInlineEmphasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownInlineImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownInlineLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownLinkBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownOrderListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownParagraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownQuote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownSetextHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownSoftBreak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownTextual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MarkdownBogus {
    syntax: SyntaxNode,
}
impl MarkdownBogus {
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
impl AstNode for MarkdownBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_BOGUS
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
impl std::fmt::Debug for MarkdownBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<MarkdownBogus> for SyntaxNode {
    fn from(n: MarkdownBogus) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownBogus> for SyntaxElement {
    fn from(n: MarkdownBogus) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MarkdownBlockList {
    syntax_list: SyntaxList,
}
impl MarkdownBlockList {
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
impl AstNode for MarkdownBlockList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_BLOCK_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_BLOCK_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<MarkdownBlockList> {
        if Self::can_cast(syntax.kind()) {
            Some(MarkdownBlockList {
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
impl Serialize for MarkdownBlockList {
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
impl AstNodeList for MarkdownBlockList {
    type Language = Language;
    type Node = AnyMarkdownBlock;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MarkdownBlockList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MarkdownBlockList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MarkdownBlockList {
    type Item = AnyMarkdownBlock;
    type IntoIter = AstNodeListIterator<Language, AnyMarkdownBlock>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MarkdownBlockList {
    type Item = AnyMarkdownBlock;
    type IntoIter = AstNodeListIterator<Language, AnyMarkdownBlock>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MarkdownBulletList {
    syntax_list: SyntaxList,
}
impl MarkdownBulletList {
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
impl AstNode for MarkdownBulletList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_BULLET_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_BULLET_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<MarkdownBulletList> {
        if Self::can_cast(syntax.kind()) {
            Some(MarkdownBulletList {
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
impl Serialize for MarkdownBulletList {
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
impl AstNodeList for MarkdownBulletList {
    type Language = Language;
    type Node = AnyCodeBlock;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MarkdownBulletList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MarkdownBulletList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MarkdownBulletList {
    type Item = AnyCodeBlock;
    type IntoIter = AstNodeListIterator<Language, AnyCodeBlock>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MarkdownBulletList {
    type Item = AnyCodeBlock;
    type IntoIter = AstNodeListIterator<Language, AnyCodeBlock>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MarkdownHashList {
    syntax_list: SyntaxList,
}
impl MarkdownHashList {
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
impl AstNode for MarkdownHashList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_HASH_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_HASH_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<MarkdownHashList> {
        if Self::can_cast(syntax.kind()) {
            Some(MarkdownHashList {
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
impl Serialize for MarkdownHashList {
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
impl AstNodeList for MarkdownHashList {
    type Language = Language;
    type Node = MarkdownHash;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MarkdownHashList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MarkdownHashList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MarkdownHashList {
    type Item = MarkdownHash;
    type IntoIter = AstNodeListIterator<Language, MarkdownHash>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MarkdownHashList {
    type Item = MarkdownHash;
    type IntoIter = AstNodeListIterator<Language, MarkdownHash>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MarkdownOrderList {
    syntax_list: SyntaxList,
}
impl MarkdownOrderList {
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
impl AstNode for MarkdownOrderList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_ORDER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_ORDER_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<MarkdownOrderList> {
        if Self::can_cast(syntax.kind()) {
            Some(MarkdownOrderList {
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
impl Serialize for MarkdownOrderList {
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
impl AstNodeList for MarkdownOrderList {
    type Language = Language;
    type Node = AnyCodeBlock;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MarkdownOrderList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MarkdownOrderList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MarkdownOrderList {
    type Item = AnyCodeBlock;
    type IntoIter = AstNodeListIterator<Language, AnyCodeBlock>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MarkdownOrderList {
    type Item = AnyCodeBlock;
    type IntoIter = AstNodeListIterator<Language, AnyCodeBlock>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MarkdownParagraphItemList {
    syntax_list: SyntaxList,
}
impl MarkdownParagraphItemList {
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
impl AstNode for MarkdownParagraphItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_PARAGRAPH_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_PARAGRAPH_ITEM_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<MarkdownParagraphItemList> {
        if Self::can_cast(syntax.kind()) {
            Some(MarkdownParagraphItemList {
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
impl Serialize for MarkdownParagraphItemList {
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
impl AstNodeList for MarkdownParagraphItemList {
    type Language = Language;
    type Node = AnyMarkdownInline;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MarkdownParagraphItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MarkdownParagraphItemList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MarkdownParagraphItemList {
    type Item = AnyMarkdownInline;
    type IntoIter = AstNodeListIterator<Language, AnyMarkdownInline>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MarkdownParagraphItemList {
    type Item = AnyMarkdownInline;
    type IntoIter = AstNodeListIterator<Language, AnyMarkdownInline>;
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
