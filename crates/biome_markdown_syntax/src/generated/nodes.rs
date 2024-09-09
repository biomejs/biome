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
    pub fn value(&self) -> SyntaxResult<AnyMarkdownBlock> {
        support::required_node(&self.syntax, 1usize)
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
    pub value: SyntaxResult<AnyMarkdownBlock>,
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
pub struct MarkdownH1 {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownH1 {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownH1Fields {
        MarkdownH1Fields {
            markdown_paragraph: self.markdown_paragraph(),
        }
    }
    pub fn markdown_paragraph(&self) -> Option<MarkdownParagraph> {
        support::node(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownH1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownH1Fields {
    pub markdown_paragraph: Option<MarkdownParagraph>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownH2 {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownH2 {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownH2Fields {
        MarkdownH2Fields {
            markdown_paragraph: self.markdown_paragraph(),
        }
    }
    pub fn markdown_paragraph(&self) -> Option<MarkdownParagraph> {
        support::node(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownH2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownH2Fields {
    pub markdown_paragraph: Option<MarkdownParagraph>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownH3 {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownH3 {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownH3Fields {
        MarkdownH3Fields {
            markdown_paragraph: self.markdown_paragraph(),
        }
    }
    pub fn markdown_paragraph(&self) -> Option<MarkdownParagraph> {
        support::node(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownH3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownH3Fields {
    pub markdown_paragraph: Option<MarkdownParagraph>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownH4 {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownH4 {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownH4Fields {
        MarkdownH4Fields {
            markdown_paragraph: self.markdown_paragraph(),
        }
    }
    pub fn markdown_paragraph(&self) -> Option<MarkdownParagraph> {
        support::node(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownH4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownH4Fields {
    pub markdown_paragraph: Option<MarkdownParagraph>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownH5 {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownH5 {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownH5Fields {
        MarkdownH5Fields {
            markdown_paragraph: self.markdown_paragraph(),
        }
    }
    pub fn markdown_paragraph(&self) -> Option<MarkdownParagraph> {
        support::node(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownH5 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownH5Fields {
    pub markdown_paragraph: Option<MarkdownParagraph>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownH6 {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownH6 {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownH6Fields {
        MarkdownH6Fields {
            markdown_paragraph: self.markdown_paragraph(),
        }
    }
    pub fn markdown_paragraph(&self) -> Option<MarkdownParagraph> {
        support::node(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownH6 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownH6Fields {
    pub markdown_paragraph: Option<MarkdownParagraph>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownHTMLBlock {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownHTMLBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownHTMLBlockFields {
        MarkdownHTMLBlockFields {
            markdown_textual: self.markdown_textual(),
        }
    }
    pub fn markdown_textual(&self) -> SyntaxResult<MarkdownTextual> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownHTMLBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownHTMLBlockFields {
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
pub struct MarkdownSetextH1 {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownSetextH1 {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownSetextH1Fields {
        MarkdownSetextH1Fields {
            markdown_paragraph: self.markdown_paragraph(),
        }
    }
    pub fn markdown_paragraph(&self) -> SyntaxResult<MarkdownParagraph> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownSetextH1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownSetextH1Fields {
    pub markdown_paragraph: SyntaxResult<MarkdownParagraph>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownSetextH2 {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownSetextH2 {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownSetextH2Fields {
        MarkdownSetextH2Fields {
            markdown_paragraph: self.markdown_paragraph(),
        }
    }
    pub fn markdown_paragraph(&self) -> SyntaxResult<MarkdownParagraph> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownSetextH2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownSetextH2Fields {
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
    AnyMakrdownSetextHeader(AnyMakrdownSetextHeader),
    AnyMarkdownHeader(AnyMarkdownHeader),
    MarkdownBreakBlock(MarkdownBreakBlock),
    MarkdownHTMLBlock(MarkdownHTMLBlock),
    MarkdownLinkBlock(MarkdownLinkBlock),
    MarkdownParagraph(MarkdownParagraph),
}
impl AnyLeafBlock {
    pub fn as_any_code_block(&self) -> Option<&AnyCodeBlock> {
        match &self {
            AnyLeafBlock::AnyCodeBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_any_makrdown_setext_header(&self) -> Option<&AnyMakrdownSetextHeader> {
        match &self {
            AnyLeafBlock::AnyMakrdownSetextHeader(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_any_markdown_header(&self) -> Option<&AnyMarkdownHeader> {
        match &self {
            AnyLeafBlock::AnyMarkdownHeader(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_break_block(&self) -> Option<&MarkdownBreakBlock> {
        match &self {
            AnyLeafBlock::MarkdownBreakBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_html_block(&self) -> Option<&MarkdownHTMLBlock> {
        match &self {
            AnyLeafBlock::MarkdownHTMLBlock(item) => Some(item),
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
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMakrdownSetextHeader {
    MarkdownSetextH1(MarkdownSetextH1),
    MarkdownSetextH2(MarkdownSetextH2),
}
impl AnyMakrdownSetextHeader {
    pub fn as_markdown_setext_h1(&self) -> Option<&MarkdownSetextH1> {
        match &self {
            AnyMakrdownSetextHeader::MarkdownSetextH1(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_setext_h2(&self) -> Option<&MarkdownSetextH2> {
        match &self {
            AnyMakrdownSetextHeader::MarkdownSetextH2(item) => Some(item),
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
pub enum AnyMarkdownHeader {
    MarkdownH1(MarkdownH1),
    MarkdownH2(MarkdownH2),
    MarkdownH3(MarkdownH3),
    MarkdownH4(MarkdownH4),
    MarkdownH5(MarkdownH5),
    MarkdownH6(MarkdownH6),
}
impl AnyMarkdownHeader {
    pub fn as_markdown_h1(&self) -> Option<&MarkdownH1> {
        match &self {
            AnyMarkdownHeader::MarkdownH1(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_h2(&self) -> Option<&MarkdownH2> {
        match &self {
            AnyMarkdownHeader::MarkdownH2(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_h3(&self) -> Option<&MarkdownH3> {
        match &self {
            AnyMarkdownHeader::MarkdownH3(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_h4(&self) -> Option<&MarkdownH4> {
        match &self {
            AnyMarkdownHeader::MarkdownH4(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_h5(&self) -> Option<&MarkdownH5> {
        match &self {
            AnyMarkdownHeader::MarkdownH5(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_h6(&self) -> Option<&MarkdownH6> {
        match &self {
            AnyMarkdownHeader::MarkdownH6(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMarkdownInline {
    MarkdownHTMLBlock(MarkdownHTMLBlock),
    MarkdownHardLine(MarkdownHardLine),
    MarkdownInlineCode(MarkdownInlineCode),
    MarkdownInlineEmphasis(MarkdownInlineEmphasis),
    MarkdownInlineImage(MarkdownInlineImage),
    MarkdownInlineLink(MarkdownInlineLink),
    MarkdownSoftBreak(MarkdownSoftBreak),
    MarkdownTextual(MarkdownTextual),
}
impl AnyMarkdownInline {
    pub fn as_markdown_html_block(&self) -> Option<&MarkdownHTMLBlock> {
        match &self {
            AnyMarkdownInline::MarkdownHTMLBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_hard_line(&self) -> Option<&MarkdownHardLine> {
        match &self {
            AnyMarkdownInline::MarkdownHardLine(item) => Some(item),
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
            .field("value", &support::DebugSyntaxResult(self.value()))
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
impl AstNode for MarkdownH1 {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_H1 as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_H1
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
impl std::fmt::Debug for MarkdownH1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownH1")
            .field(
                "markdown_paragraph",
                &support::DebugOptionalElement(self.markdown_paragraph()),
            )
            .finish()
    }
}
impl From<MarkdownH1> for SyntaxNode {
    fn from(n: MarkdownH1) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownH1> for SyntaxElement {
    fn from(n: MarkdownH1) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownH2 {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_H2 as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_H2
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
impl std::fmt::Debug for MarkdownH2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownH2")
            .field(
                "markdown_paragraph",
                &support::DebugOptionalElement(self.markdown_paragraph()),
            )
            .finish()
    }
}
impl From<MarkdownH2> for SyntaxNode {
    fn from(n: MarkdownH2) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownH2> for SyntaxElement {
    fn from(n: MarkdownH2) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownH3 {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_H3 as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_H3
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
impl std::fmt::Debug for MarkdownH3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownH3")
            .field(
                "markdown_paragraph",
                &support::DebugOptionalElement(self.markdown_paragraph()),
            )
            .finish()
    }
}
impl From<MarkdownH3> for SyntaxNode {
    fn from(n: MarkdownH3) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownH3> for SyntaxElement {
    fn from(n: MarkdownH3) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownH4 {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_H4 as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_H4
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
impl std::fmt::Debug for MarkdownH4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownH4")
            .field(
                "markdown_paragraph",
                &support::DebugOptionalElement(self.markdown_paragraph()),
            )
            .finish()
    }
}
impl From<MarkdownH4> for SyntaxNode {
    fn from(n: MarkdownH4) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownH4> for SyntaxElement {
    fn from(n: MarkdownH4) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownH5 {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_H5 as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_H5
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
impl std::fmt::Debug for MarkdownH5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownH5")
            .field(
                "markdown_paragraph",
                &support::DebugOptionalElement(self.markdown_paragraph()),
            )
            .finish()
    }
}
impl From<MarkdownH5> for SyntaxNode {
    fn from(n: MarkdownH5) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownH5> for SyntaxElement {
    fn from(n: MarkdownH5) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownH6 {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_H6 as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_H6
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
impl std::fmt::Debug for MarkdownH6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownH6")
            .field(
                "markdown_paragraph",
                &support::DebugOptionalElement(self.markdown_paragraph()),
            )
            .finish()
    }
}
impl From<MarkdownH6> for SyntaxNode {
    fn from(n: MarkdownH6) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownH6> for SyntaxElement {
    fn from(n: MarkdownH6) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownHTMLBlock {
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
impl std::fmt::Debug for MarkdownHTMLBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownHTMLBlock")
            .field(
                "markdown_textual",
                &support::DebugSyntaxResult(self.markdown_textual()),
            )
            .finish()
    }
}
impl From<MarkdownHTMLBlock> for SyntaxNode {
    fn from(n: MarkdownHTMLBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownHTMLBlock> for SyntaxElement {
    fn from(n: MarkdownHTMLBlock) -> SyntaxElement {
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
impl AstNode for MarkdownSetextH1 {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_SETEXT_H1 as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_SETEXT_H1
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
impl std::fmt::Debug for MarkdownSetextH1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownSetextH1")
            .field(
                "markdown_paragraph",
                &support::DebugSyntaxResult(self.markdown_paragraph()),
            )
            .finish()
    }
}
impl From<MarkdownSetextH1> for SyntaxNode {
    fn from(n: MarkdownSetextH1) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownSetextH1> for SyntaxElement {
    fn from(n: MarkdownSetextH1) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MarkdownSetextH2 {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_SETEXT_H2 as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_SETEXT_H2
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
impl std::fmt::Debug for MarkdownSetextH2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownSetextH2")
            .field(
                "markdown_paragraph",
                &support::DebugSyntaxResult(self.markdown_paragraph()),
            )
            .finish()
    }
}
impl From<MarkdownSetextH2> for SyntaxNode {
    fn from(n: MarkdownSetextH2) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownSetextH2> for SyntaxElement {
    fn from(n: MarkdownSetextH2) -> SyntaxElement {
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
impl From<MarkdownHTMLBlock> for AnyLeafBlock {
    fn from(node: MarkdownHTMLBlock) -> AnyLeafBlock {
        AnyLeafBlock::MarkdownHTMLBlock(node)
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
impl AstNode for AnyLeafBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCodeBlock::KIND_SET
        .union(AnyMakrdownSetextHeader::KIND_SET)
        .union(AnyMarkdownHeader::KIND_SET)
        .union(MarkdownBreakBlock::KIND_SET)
        .union(MarkdownHTMLBlock::KIND_SET)
        .union(MarkdownLinkBlock::KIND_SET)
        .union(MarkdownParagraph::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            MARKDOWN_BREAK_BLOCK | MARKDOWN_HTML_BLOCK | MARKDOWN_LINK_BLOCK
            | MARKDOWN_PARAGRAPH => true,
            k if AnyCodeBlock::can_cast(k) => true,
            k if AnyMakrdownSetextHeader::can_cast(k) => true,
            k if AnyMarkdownHeader::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MARKDOWN_BREAK_BLOCK => AnyLeafBlock::MarkdownBreakBlock(MarkdownBreakBlock { syntax }),
            MARKDOWN_HTML_BLOCK => AnyLeafBlock::MarkdownHTMLBlock(MarkdownHTMLBlock { syntax }),
            MARKDOWN_LINK_BLOCK => AnyLeafBlock::MarkdownLinkBlock(MarkdownLinkBlock { syntax }),
            MARKDOWN_PARAGRAPH => AnyLeafBlock::MarkdownParagraph(MarkdownParagraph { syntax }),
            _ => {
                let syntax = match AnyCodeBlock::try_cast(syntax) {
                    Ok(any_code_block) => {
                        return Some(AnyLeafBlock::AnyCodeBlock(any_code_block));
                    }
                    Err(syntax) => syntax,
                };
                let syntax = match AnyMakrdownSetextHeader::try_cast(syntax) {
                    Ok(any_makrdown_setext_header) => {
                        return Some(AnyLeafBlock::AnyMakrdownSetextHeader(
                            any_makrdown_setext_header,
                        ));
                    }
                    Err(syntax) => syntax,
                };
                if let Some(any_markdown_header) = AnyMarkdownHeader::cast(syntax) {
                    return Some(AnyLeafBlock::AnyMarkdownHeader(any_markdown_header));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyLeafBlock::MarkdownBreakBlock(it) => &it.syntax,
            AnyLeafBlock::MarkdownHTMLBlock(it) => &it.syntax,
            AnyLeafBlock::MarkdownLinkBlock(it) => &it.syntax,
            AnyLeafBlock::MarkdownParagraph(it) => &it.syntax,
            AnyLeafBlock::AnyCodeBlock(it) => it.syntax(),
            AnyLeafBlock::AnyMakrdownSetextHeader(it) => it.syntax(),
            AnyLeafBlock::AnyMarkdownHeader(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyLeafBlock::MarkdownBreakBlock(it) => it.syntax,
            AnyLeafBlock::MarkdownHTMLBlock(it) => it.syntax,
            AnyLeafBlock::MarkdownLinkBlock(it) => it.syntax,
            AnyLeafBlock::MarkdownParagraph(it) => it.syntax,
            AnyLeafBlock::AnyCodeBlock(it) => it.into_syntax(),
            AnyLeafBlock::AnyMakrdownSetextHeader(it) => it.into_syntax(),
            AnyLeafBlock::AnyMarkdownHeader(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyLeafBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyLeafBlock::AnyCodeBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::AnyMakrdownSetextHeader(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::AnyMarkdownHeader(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::MarkdownBreakBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::MarkdownHTMLBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::MarkdownLinkBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyLeafBlock::MarkdownParagraph(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyLeafBlock> for SyntaxNode {
    fn from(n: AnyLeafBlock) -> SyntaxNode {
        match n {
            AnyLeafBlock::AnyCodeBlock(it) => it.into(),
            AnyLeafBlock::AnyMakrdownSetextHeader(it) => it.into(),
            AnyLeafBlock::AnyMarkdownHeader(it) => it.into(),
            AnyLeafBlock::MarkdownBreakBlock(it) => it.into(),
            AnyLeafBlock::MarkdownHTMLBlock(it) => it.into(),
            AnyLeafBlock::MarkdownLinkBlock(it) => it.into(),
            AnyLeafBlock::MarkdownParagraph(it) => it.into(),
        }
    }
}
impl From<AnyLeafBlock> for SyntaxElement {
    fn from(n: AnyLeafBlock) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MarkdownSetextH1> for AnyMakrdownSetextHeader {
    fn from(node: MarkdownSetextH1) -> AnyMakrdownSetextHeader {
        AnyMakrdownSetextHeader::MarkdownSetextH1(node)
    }
}
impl From<MarkdownSetextH2> for AnyMakrdownSetextHeader {
    fn from(node: MarkdownSetextH2) -> AnyMakrdownSetextHeader {
        AnyMakrdownSetextHeader::MarkdownSetextH2(node)
    }
}
impl AstNode for AnyMakrdownSetextHeader {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        MarkdownSetextH1::KIND_SET.union(MarkdownSetextH2::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, MARKDOWN_SETEXT_H1 | MARKDOWN_SETEXT_H2)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MARKDOWN_SETEXT_H1 => {
                AnyMakrdownSetextHeader::MarkdownSetextH1(MarkdownSetextH1 { syntax })
            }
            MARKDOWN_SETEXT_H2 => {
                AnyMakrdownSetextHeader::MarkdownSetextH2(MarkdownSetextH2 { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyMakrdownSetextHeader::MarkdownSetextH1(it) => &it.syntax,
            AnyMakrdownSetextHeader::MarkdownSetextH2(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyMakrdownSetextHeader::MarkdownSetextH1(it) => it.syntax,
            AnyMakrdownSetextHeader::MarkdownSetextH2(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMakrdownSetextHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyMakrdownSetextHeader::MarkdownSetextH1(it) => std::fmt::Debug::fmt(it, f),
            AnyMakrdownSetextHeader::MarkdownSetextH2(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMakrdownSetextHeader> for SyntaxNode {
    fn from(n: AnyMakrdownSetextHeader) -> SyntaxNode {
        match n {
            AnyMakrdownSetextHeader::MarkdownSetextH1(it) => it.into(),
            AnyMakrdownSetextHeader::MarkdownSetextH2(it) => it.into(),
        }
    }
}
impl From<AnyMakrdownSetextHeader> for SyntaxElement {
    fn from(n: AnyMakrdownSetextHeader) -> SyntaxElement {
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
impl From<MarkdownH1> for AnyMarkdownHeader {
    fn from(node: MarkdownH1) -> AnyMarkdownHeader {
        AnyMarkdownHeader::MarkdownH1(node)
    }
}
impl From<MarkdownH2> for AnyMarkdownHeader {
    fn from(node: MarkdownH2) -> AnyMarkdownHeader {
        AnyMarkdownHeader::MarkdownH2(node)
    }
}
impl From<MarkdownH3> for AnyMarkdownHeader {
    fn from(node: MarkdownH3) -> AnyMarkdownHeader {
        AnyMarkdownHeader::MarkdownH3(node)
    }
}
impl From<MarkdownH4> for AnyMarkdownHeader {
    fn from(node: MarkdownH4) -> AnyMarkdownHeader {
        AnyMarkdownHeader::MarkdownH4(node)
    }
}
impl From<MarkdownH5> for AnyMarkdownHeader {
    fn from(node: MarkdownH5) -> AnyMarkdownHeader {
        AnyMarkdownHeader::MarkdownH5(node)
    }
}
impl From<MarkdownH6> for AnyMarkdownHeader {
    fn from(node: MarkdownH6) -> AnyMarkdownHeader {
        AnyMarkdownHeader::MarkdownH6(node)
    }
}
impl AstNode for AnyMarkdownHeader {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MarkdownH1::KIND_SET
        .union(MarkdownH2::KIND_SET)
        .union(MarkdownH3::KIND_SET)
        .union(MarkdownH4::KIND_SET)
        .union(MarkdownH5::KIND_SET)
        .union(MarkdownH6::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            MARKDOWN_H1 | MARKDOWN_H2 | MARKDOWN_H3 | MARKDOWN_H4 | MARKDOWN_H5 | MARKDOWN_H6
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MARKDOWN_H1 => AnyMarkdownHeader::MarkdownH1(MarkdownH1 { syntax }),
            MARKDOWN_H2 => AnyMarkdownHeader::MarkdownH2(MarkdownH2 { syntax }),
            MARKDOWN_H3 => AnyMarkdownHeader::MarkdownH3(MarkdownH3 { syntax }),
            MARKDOWN_H4 => AnyMarkdownHeader::MarkdownH4(MarkdownH4 { syntax }),
            MARKDOWN_H5 => AnyMarkdownHeader::MarkdownH5(MarkdownH5 { syntax }),
            MARKDOWN_H6 => AnyMarkdownHeader::MarkdownH6(MarkdownH6 { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyMarkdownHeader::MarkdownH1(it) => &it.syntax,
            AnyMarkdownHeader::MarkdownH2(it) => &it.syntax,
            AnyMarkdownHeader::MarkdownH3(it) => &it.syntax,
            AnyMarkdownHeader::MarkdownH4(it) => &it.syntax,
            AnyMarkdownHeader::MarkdownH5(it) => &it.syntax,
            AnyMarkdownHeader::MarkdownH6(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyMarkdownHeader::MarkdownH1(it) => it.syntax,
            AnyMarkdownHeader::MarkdownH2(it) => it.syntax,
            AnyMarkdownHeader::MarkdownH3(it) => it.syntax,
            AnyMarkdownHeader::MarkdownH4(it) => it.syntax,
            AnyMarkdownHeader::MarkdownH5(it) => it.syntax,
            AnyMarkdownHeader::MarkdownH6(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMarkdownHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyMarkdownHeader::MarkdownH1(it) => std::fmt::Debug::fmt(it, f),
            AnyMarkdownHeader::MarkdownH2(it) => std::fmt::Debug::fmt(it, f),
            AnyMarkdownHeader::MarkdownH3(it) => std::fmt::Debug::fmt(it, f),
            AnyMarkdownHeader::MarkdownH4(it) => std::fmt::Debug::fmt(it, f),
            AnyMarkdownHeader::MarkdownH5(it) => std::fmt::Debug::fmt(it, f),
            AnyMarkdownHeader::MarkdownH6(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMarkdownHeader> for SyntaxNode {
    fn from(n: AnyMarkdownHeader) -> SyntaxNode {
        match n {
            AnyMarkdownHeader::MarkdownH1(it) => it.into(),
            AnyMarkdownHeader::MarkdownH2(it) => it.into(),
            AnyMarkdownHeader::MarkdownH3(it) => it.into(),
            AnyMarkdownHeader::MarkdownH4(it) => it.into(),
            AnyMarkdownHeader::MarkdownH5(it) => it.into(),
            AnyMarkdownHeader::MarkdownH6(it) => it.into(),
        }
    }
}
impl From<AnyMarkdownHeader> for SyntaxElement {
    fn from(n: AnyMarkdownHeader) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MarkdownHTMLBlock> for AnyMarkdownInline {
    fn from(node: MarkdownHTMLBlock) -> AnyMarkdownInline {
        AnyMarkdownInline::MarkdownHTMLBlock(node)
    }
}
impl From<MarkdownHardLine> for AnyMarkdownInline {
    fn from(node: MarkdownHardLine) -> AnyMarkdownInline {
        AnyMarkdownInline::MarkdownHardLine(node)
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
    const KIND_SET: SyntaxKindSet<Language> = MarkdownHTMLBlock::KIND_SET
        .union(MarkdownHardLine::KIND_SET)
        .union(MarkdownInlineCode::KIND_SET)
        .union(MarkdownInlineEmphasis::KIND_SET)
        .union(MarkdownInlineImage::KIND_SET)
        .union(MarkdownInlineLink::KIND_SET)
        .union(MarkdownSoftBreak::KIND_SET)
        .union(MarkdownTextual::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            MARKDOWN_HTML_BLOCK
                | MARKDOWN_HARD_LINE
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
            MARKDOWN_HTML_BLOCK => {
                AnyMarkdownInline::MarkdownHTMLBlock(MarkdownHTMLBlock { syntax })
            }
            MARKDOWN_HARD_LINE => AnyMarkdownInline::MarkdownHardLine(MarkdownHardLine { syntax }),
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
            AnyMarkdownInline::MarkdownHTMLBlock(it) => &it.syntax,
            AnyMarkdownInline::MarkdownHardLine(it) => &it.syntax,
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
            AnyMarkdownInline::MarkdownHTMLBlock(it) => it.syntax,
            AnyMarkdownInline::MarkdownHardLine(it) => it.syntax,
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
            AnyMarkdownInline::MarkdownHTMLBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyMarkdownInline::MarkdownHardLine(it) => std::fmt::Debug::fmt(it, f),
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
            AnyMarkdownInline::MarkdownHTMLBlock(it) => it.into(),
            AnyMarkdownInline::MarkdownHardLine(it) => it.into(),
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
impl std::fmt::Display for AnyMakrdownSetextHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMarkdownBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMarkdownHeader {
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
impl std::fmt::Display for MarkdownH1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownH2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownH3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownH4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownH5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownH6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownHTMLBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownHardLine {
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
impl std::fmt::Display for MarkdownSetextH1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownSetextH2 {
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
