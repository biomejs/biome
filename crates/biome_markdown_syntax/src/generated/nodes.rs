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
            hash_token: self.hash_token(),
            markdown_h2: self.markdown_h2(),
            hash_token: self.hash_token(),
            markdown_paragraph: self.markdown_paragraph(),
        }
    }
    pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn markdown_h2(&self) -> SyntaxResult<MarkdownH2> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn hash_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn markdown_paragraph(&self) -> Option<MarkdownParagraph> {
        support::node(&self.syntax, 3usize)
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
    pub hash_token: SyntaxResult<SyntaxToken>,
    pub markdown_h2: SyntaxResult<MarkdownH2>,
    pub hash_token: Option<SyntaxToken>,
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
            hash_token: self.hash_token(),
            markdown_h3: self.markdown_h3(),
            hash_token: self.hash_token(),
            markdown_paragraph: self.markdown_paragraph(),
        }
    }
    pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn markdown_h3(&self) -> SyntaxResult<MarkdownH3> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn hash_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn markdown_paragraph(&self) -> Option<MarkdownParagraph> {
        support::node(&self.syntax, 3usize)
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
    pub hash_token: SyntaxResult<SyntaxToken>,
    pub markdown_h3: SyntaxResult<MarkdownH3>,
    pub hash_token: Option<SyntaxToken>,
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
            hash_token: self.hash_token(),
            markdown_h4: self.markdown_h4(),
            hash_token: self.hash_token(),
            markdown_paragraph: self.markdown_paragraph(),
        }
    }
    pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn markdown_h4(&self) -> SyntaxResult<MarkdownH4> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn hash_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn markdown_paragraph(&self) -> Option<MarkdownParagraph> {
        support::node(&self.syntax, 3usize)
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
    pub hash_token: SyntaxResult<SyntaxToken>,
    pub markdown_h4: SyntaxResult<MarkdownH4>,
    pub hash_token: Option<SyntaxToken>,
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
            hash_token: self.hash_token(),
            markdown_h5: self.markdown_h5(),
            hash_token: self.hash_token(),
            markdown_paragraph: self.markdown_paragraph(),
        }
    }
    pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn markdown_h5(&self) -> SyntaxResult<MarkdownH5> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn hash_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn markdown_paragraph(&self) -> Option<MarkdownParagraph> {
        support::node(&self.syntax, 3usize)
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
    pub hash_token: SyntaxResult<SyntaxToken>,
    pub markdown_h5: SyntaxResult<MarkdownH5>,
    pub hash_token: Option<SyntaxToken>,
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
            hash_token: self.hash_token(),
            markdown_h6: self.markdown_h6(),
            hash_token: self.hash_token(),
            markdown_paragraph: self.markdown_paragraph(),
        }
    }
    pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn markdown_h6(&self) -> SyntaxResult<MarkdownH6> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn hash_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn markdown_paragraph(&self) -> Option<MarkdownParagraph> {
        support::node(&self.syntax, 3usize)
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
    pub hash_token: SyntaxResult<SyntaxToken>,
    pub markdown_h6: SyntaxResult<MarkdownH6>,
    pub hash_token: Option<SyntaxToken>,
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
            hash_token: self.hash_token(),
            markdown_h1: self.markdown_h1(),
            hash_token: self.hash_token(),
        }
    }
    pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn markdown_h1(&self) -> SyntaxResult<MarkdownH1> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn hash_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
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
    pub hash_token: SyntaxResult<SyntaxToken>,
    pub markdown_h1: SyntaxResult<MarkdownH1>,
    pub hash_token: Option<SyntaxToken>,
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
            markdown_string: self.markdown_string(),
        }
    }
    pub fn markdown_string(&self) -> SyntaxResult<MarkdownString> {
        support::required_node(&self.syntax, 0usize)
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
    pub markdown_string: SyntaxResult<MarkdownString>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MarkdownString {
    pub(crate) syntax: SyntaxNode,
}
impl MarkdownString {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MarkdownStringFields {
        MarkdownStringFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MarkdownString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MarkdownStringFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMarkdownBlock {
    MarkdownHeader(MarkdownHeader),
    MarkdownList(MarkdownList),
    MarkdownParagraph(MarkdownParagraph),
}
impl AnyMarkdownBlock {
    pub fn as_markdown_header(&self) -> Option<&MarkdownHeader> {
        match &self {
            AnyMarkdownBlock::MarkdownHeader(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_list(&self) -> Option<&MarkdownList> {
        match &self {
            AnyMarkdownBlock::MarkdownList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_markdown_paragraph(&self) -> Option<&MarkdownParagraph> {
        match &self {
            AnyMarkdownBlock::MarkdownParagraph(item) => Some(item),
            _ => None,
        }
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
            .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
            .field(
                "markdown_h2",
                &support::DebugSyntaxResult(self.markdown_h2()),
            )
            .field(
                "hash_token",
                &support::DebugOptionalElement(self.hash_token()),
            )
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
            .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
            .field(
                "markdown_h3",
                &support::DebugSyntaxResult(self.markdown_h3()),
            )
            .field(
                "hash_token",
                &support::DebugOptionalElement(self.hash_token()),
            )
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
            .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
            .field(
                "markdown_h4",
                &support::DebugSyntaxResult(self.markdown_h4()),
            )
            .field(
                "hash_token",
                &support::DebugOptionalElement(self.hash_token()),
            )
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
            .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
            .field(
                "markdown_h5",
                &support::DebugSyntaxResult(self.markdown_h5()),
            )
            .field(
                "hash_token",
                &support::DebugOptionalElement(self.hash_token()),
            )
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
            .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
            .field(
                "markdown_h6",
                &support::DebugSyntaxResult(self.markdown_h6()),
            )
            .field(
                "hash_token",
                &support::DebugOptionalElement(self.hash_token()),
            )
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
            .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
            .field(
                "markdown_h1",
                &support::DebugSyntaxResult(self.markdown_h1()),
            )
            .field(
                "hash_token",
                &support::DebugOptionalElement(self.hash_token()),
            )
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
                "markdown_string",
                &support::DebugSyntaxResult(self.markdown_string()),
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
impl AstNode for MarkdownString {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_STRING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_STRING
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MarkdownString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownString")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<MarkdownString> for SyntaxNode {
    fn from(n: MarkdownString) -> SyntaxNode {
        n.syntax
    }
}
impl From<MarkdownString> for SyntaxElement {
    fn from(n: MarkdownString) -> SyntaxElement {
        n.syntax.into()
    }
}
impl From<MarkdownHeader> for AnyMarkdownBlock {
    fn from(node: MarkdownHeader) -> AnyMarkdownBlock {
        AnyMarkdownBlock::MarkdownHeader(node)
    }
}
impl From<MarkdownList> for AnyMarkdownBlock {
    fn from(node: MarkdownList) -> AnyMarkdownBlock {
        AnyMarkdownBlock::MarkdownList(node)
    }
}
impl From<MarkdownParagraph> for AnyMarkdownBlock {
    fn from(node: MarkdownParagraph) -> AnyMarkdownBlock {
        AnyMarkdownBlock::MarkdownParagraph(node)
    }
}
impl AstNode for AnyMarkdownBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MarkdownHeader::KIND_SET
        .union(MarkdownList::KIND_SET)
        .union(MarkdownParagraph::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, MARKDOWN_HEADER | MARKDOWN_LIST | MARKDOWN_PARAGRAPH)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MARKDOWN_HEADER => AnyMarkdownBlock::MarkdownHeader(MarkdownHeader { syntax }),
            MARKDOWN_LIST => AnyMarkdownBlock::MarkdownList(MarkdownList { syntax }),
            MARKDOWN_PARAGRAPH => AnyMarkdownBlock::MarkdownParagraph(MarkdownParagraph { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyMarkdownBlock::MarkdownHeader(it) => &it.syntax,
            AnyMarkdownBlock::MarkdownList(it) => &it.syntax,
            AnyMarkdownBlock::MarkdownParagraph(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyMarkdownBlock::MarkdownHeader(it) => it.syntax,
            AnyMarkdownBlock::MarkdownList(it) => it.syntax,
            AnyMarkdownBlock::MarkdownParagraph(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMarkdownBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyMarkdownBlock::MarkdownHeader(it) => std::fmt::Debug::fmt(it, f),
            AnyMarkdownBlock::MarkdownList(it) => std::fmt::Debug::fmt(it, f),
            AnyMarkdownBlock::MarkdownParagraph(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMarkdownBlock> for SyntaxNode {
    fn from(n: AnyMarkdownBlock) -> SyntaxNode {
        match n {
            AnyMarkdownBlock::MarkdownHeader(it) => it.into(),
            AnyMarkdownBlock::MarkdownList(it) => it.into(),
            AnyMarkdownBlock::MarkdownParagraph(it) => it.into(),
        }
    }
}
impl From<AnyMarkdownBlock> for SyntaxElement {
    fn from(n: AnyMarkdownBlock) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyMarkdownBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownDocument {
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
impl std::fmt::Display for MarkdownHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownParagraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MarkdownString {
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
pub struct MarkdownList {
    syntax_list: SyntaxList,
}
impl MarkdownList {
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
impl AstNode for MarkdownList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MARKDOWN_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MARKDOWN_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<MarkdownList> {
        if Self::can_cast(syntax.kind()) {
            Some(MarkdownList {
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
impl Serialize for MarkdownList {
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
impl AstNodeList for MarkdownList {
    type Language = Language;
    type Node = MarkdownString;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MarkdownList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MarkdownList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MarkdownList {
    type Item = MarkdownString;
    type IntoIter = AstNodeListIterator<Language, MarkdownString>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MarkdownList {
    type Item = MarkdownString;
    type IntoIter = AstNodeListIterator<Language, MarkdownString>;
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
