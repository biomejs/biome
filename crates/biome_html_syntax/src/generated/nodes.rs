//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    HtmlLanguage as Language, HtmlSyntaxElement as SyntaxElement,
    HtmlSyntaxElementChildren as SyntaxElementChildren,
    HtmlSyntaxKind::{self as SyntaxKind, *},
    HtmlSyntaxList as SyntaxList, HtmlSyntaxNode as SyntaxNode, HtmlSyntaxToken as SyntaxToken,
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
pub struct AstroClassDirective {
    pub(crate) syntax: SyntaxNode,
}
impl AstroClassDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> AstroClassDirectiveFields {
        AstroClassDirectiveFields {
            class_token: self.class_token(),
            value: self.value(),
        }
    }
    pub fn class_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<AstroDirectiveValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for AstroClassDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct AstroClassDirectiveFields {
    pub class_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AstroDirectiveValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AstroClientDirective {
    pub(crate) syntax: SyntaxNode,
}
impl AstroClientDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> AstroClientDirectiveFields {
        AstroClientDirectiveFields {
            client_token: self.client_token(),
            value: self.value(),
        }
    }
    pub fn client_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<AstroDirectiveValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for AstroClientDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct AstroClientDirectiveFields {
    pub client_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AstroDirectiveValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AstroDefineDirective {
    pub(crate) syntax: SyntaxNode,
}
impl AstroDefineDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> AstroDefineDirectiveFields {
        AstroDefineDirectiveFields {
            define_token: self.define_token(),
            value: self.value(),
        }
    }
    pub fn define_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<AstroDirectiveValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for AstroDefineDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct AstroDefineDirectiveFields {
    pub define_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AstroDirectiveValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AstroDirectiveValue {
    pub(crate) syntax: SyntaxNode,
}
impl AstroDirectiveValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> AstroDirectiveValueFields {
        AstroDirectiveValueFields {
            colon_token_token: self.colon_token_token(),
            name: self.name(),
            initializer: self.initializer(),
        }
    }
    pub fn colon_token_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<HtmlAttributeName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn initializer(&self) -> Option<HtmlAttributeInitializerClause> {
        support::node(&self.syntax, 2usize)
    }
}
impl Serialize for AstroDirectiveValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct AstroDirectiveValueFields {
    pub colon_token_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<HtmlAttributeName>,
    pub initializer: Option<HtmlAttributeInitializerClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AstroEmbeddedContent {
    pub(crate) syntax: SyntaxNode,
}
impl AstroEmbeddedContent {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> AstroEmbeddedContentFields {
        AstroEmbeddedContentFields {
            content_token: self.content_token(),
        }
    }
    pub fn content_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
}
impl Serialize for AstroEmbeddedContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct AstroEmbeddedContentFields {
    pub content_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AstroFrontmatterElement {
    pub(crate) syntax: SyntaxNode,
}
impl AstroFrontmatterElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> AstroFrontmatterElementFields {
        AstroFrontmatterElementFields {
            l_fence_token: self.l_fence_token(),
            content: self.content(),
            r_fence_token: self.r_fence_token(),
        }
    }
    pub fn l_fence_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn content(&self) -> SyntaxResult<AstroEmbeddedContent> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_fence_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for AstroFrontmatterElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct AstroFrontmatterElementFields {
    pub l_fence_token: SyntaxResult<SyntaxToken>,
    pub content: SyntaxResult<AstroEmbeddedContent>,
    pub r_fence_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AstroIsDirective {
    pub(crate) syntax: SyntaxNode,
}
impl AstroIsDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> AstroIsDirectiveFields {
        AstroIsDirectiveFields {
            is_token: self.is_token(),
            value: self.value(),
        }
    }
    pub fn is_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<AstroDirectiveValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for AstroIsDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct AstroIsDirectiveFields {
    pub is_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AstroDirectiveValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AstroServerDirective {
    pub(crate) syntax: SyntaxNode,
}
impl AstroServerDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> AstroServerDirectiveFields {
        AstroServerDirectiveFields {
            server_token: self.server_token(),
            value: self.value(),
        }
    }
    pub fn server_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<AstroDirectiveValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for AstroServerDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct AstroServerDirectiveFields {
    pub server_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AstroDirectiveValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AstroSetDirective {
    pub(crate) syntax: SyntaxNode,
}
impl AstroSetDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> AstroSetDirectiveFields {
        AstroSetDirectiveFields {
            set_token: self.set_token(),
            value: self.value(),
        }
    }
    pub fn set_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<AstroDirectiveValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for AstroSetDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct AstroSetDirectiveFields {
    pub set_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AstroDirectiveValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlAttribute {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlAttribute {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlAttributeFields {
        HtmlAttributeFields {
            name: self.name(),
            initializer: self.initializer(),
        }
    }
    pub fn name(&self) -> SyntaxResult<HtmlAttributeName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn initializer(&self) -> Option<HtmlAttributeInitializerClause> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for HtmlAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlAttributeFields {
    pub name: SyntaxResult<HtmlAttributeName>,
    pub initializer: Option<HtmlAttributeInitializerClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlAttributeDoubleTextExpression {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlAttributeDoubleTextExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlAttributeDoubleTextExpressionFields {
        HtmlAttributeDoubleTextExpressionFields {
            l_double_curly_token: self.l_double_curly_token(),
            expression: self.expression(),
            r_double_curly_token: self.r_double_curly_token(),
        }
    }
    pub fn l_double_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_double_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for HtmlAttributeDoubleTextExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlAttributeDoubleTextExpressionFields {
    pub l_double_curly_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<HtmlTextExpression>,
    pub r_double_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlAttributeInitializerClause {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlAttributeInitializerClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlAttributeInitializerClauseFields {
        HtmlAttributeInitializerClauseFields {
            eq_token: self.eq_token(),
            value: self.value(),
        }
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyHtmlAttributeInitializer> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for HtmlAttributeInitializerClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlAttributeInitializerClauseFields {
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyHtmlAttributeInitializer>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlAttributeName {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlAttributeName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlAttributeNameFields {
        HtmlAttributeNameFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for HtmlAttributeName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlAttributeNameFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlAttributeSingleTextExpression {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlAttributeSingleTextExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlAttributeSingleTextExpressionFields {
        HtmlAttributeSingleTextExpressionFields {
            l_curly_token: self.l_curly_token(),
            expression: self.expression(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for HtmlAttributeSingleTextExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlAttributeSingleTextExpressionFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<HtmlTextExpression>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlCdataSection {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlCdataSection {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlCdataSectionFields {
        HtmlCdataSectionFields {
            cdata_start_token: self.cdata_start_token(),
            content_token: self.content_token(),
            cdata_end_token: self.cdata_end_token(),
        }
    }
    pub fn cdata_start_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn content_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn cdata_end_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for HtmlCdataSection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlCdataSectionFields {
    pub cdata_start_token: SyntaxResult<SyntaxToken>,
    pub content_token: SyntaxResult<SyntaxToken>,
    pub cdata_end_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlClosingElement {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlClosingElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlClosingElementFields {
        HtmlClosingElementFields {
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
    pub fn name(&self) -> SyntaxResult<AnyHtmlTagName> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for HtmlClosingElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlClosingElementFields {
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub slash_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<AnyHtmlTagName>,
    pub r_angle_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlComponentName {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlComponentName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlComponentNameFields {
        HtmlComponentNameFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for HtmlComponentName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlComponentNameFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlContent {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlContent {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlContentFields {
        HtmlContentFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for HtmlContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlContentFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlDirective {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlDirectiveFields {
        HtmlDirectiveFields {
            l_angle_token: self.l_angle_token(),
            excl_token: self.excl_token(),
            doctype_token: self.doctype_token(),
            html_token: self.html_token(),
            quirk_token: self.quirk_token(),
            public_id_token: self.public_id_token(),
            system_id_token: self.system_id_token(),
            r_angle_token: self.r_angle_token(),
        }
    }
    pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn doctype_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn html_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 3usize)
    }
    pub fn quirk_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 4usize)
    }
    pub fn public_id_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 5usize)
    }
    pub fn system_id_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 6usize)
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 7usize)
    }
}
impl Serialize for HtmlDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlDirectiveFields {
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub excl_token: SyntaxResult<SyntaxToken>,
    pub doctype_token: SyntaxResult<SyntaxToken>,
    pub html_token: Option<SyntaxToken>,
    pub quirk_token: Option<SyntaxToken>,
    pub public_id_token: Option<SyntaxToken>,
    pub system_id_token: Option<SyntaxToken>,
    pub r_angle_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlDoubleTextExpression {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlDoubleTextExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlDoubleTextExpressionFields {
        HtmlDoubleTextExpressionFields {
            l_double_curly_token: self.l_double_curly_token(),
            expression: self.expression(),
            r_double_curly_token: self.r_double_curly_token(),
        }
    }
    pub fn l_double_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_double_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for HtmlDoubleTextExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlDoubleTextExpressionFields {
    pub l_double_curly_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<HtmlTextExpression>,
    pub r_double_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlElement {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlElementFields {
        HtmlElementFields {
            opening_element: self.opening_element(),
            children: self.children(),
            closing_element: self.closing_element(),
        }
    }
    pub fn opening_element(&self) -> SyntaxResult<HtmlOpeningElement> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn children(&self) -> HtmlElementList {
        support::list(&self.syntax, 1usize)
    }
    pub fn closing_element(&self) -> SyntaxResult<HtmlClosingElement> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for HtmlElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlElementFields {
    pub opening_element: SyntaxResult<HtmlOpeningElement>,
    pub children: HtmlElementList,
    pub closing_element: SyntaxResult<HtmlClosingElement>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlEmbeddedContent {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlEmbeddedContent {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlEmbeddedContentFields {
        HtmlEmbeddedContentFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for HtmlEmbeddedContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlEmbeddedContentFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlMemberName {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlMemberName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlMemberNameFields {
        HtmlMemberNameFields {
            object: self.object(),
            dot_token: self.dot_token(),
            member: self.member(),
        }
    }
    pub fn object(&self) -> SyntaxResult<AnyHtmlComponentObjectName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn member(&self) -> SyntaxResult<HtmlTagName> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for HtmlMemberName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlMemberNameFields {
    pub object: SyntaxResult<AnyHtmlComponentObjectName>,
    pub dot_token: SyntaxResult<SyntaxToken>,
    pub member: SyntaxResult<HtmlTagName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlOpeningElement {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlOpeningElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlOpeningElementFields {
        HtmlOpeningElementFields {
            l_angle_token: self.l_angle_token(),
            name: self.name(),
            attributes: self.attributes(),
            r_angle_token: self.r_angle_token(),
        }
    }
    pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<AnyHtmlTagName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn attributes(&self) -> HtmlAttributeList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for HtmlOpeningElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlOpeningElementFields {
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<AnyHtmlTagName>,
    pub attributes: HtmlAttributeList,
    pub r_angle_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlRoot {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlRootFields {
        HtmlRootFields {
            bom_token: self.bom_token(),
            frontmatter: self.frontmatter(),
            directive: self.directive(),
            html: self.html(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn frontmatter(&self) -> Option<AnyAstroFrontmatterElement> {
        support::node(&self.syntax, 1usize)
    }
    pub fn directive(&self) -> Option<HtmlDirective> {
        support::node(&self.syntax, 2usize)
    }
    pub fn html(&self) -> HtmlElementList {
        support::list(&self.syntax, 3usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl Serialize for HtmlRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlRootFields {
    pub bom_token: Option<SyntaxToken>,
    pub frontmatter: Option<AnyAstroFrontmatterElement>,
    pub directive: Option<HtmlDirective>,
    pub html: HtmlElementList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlSelfClosingElement {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlSelfClosingElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlSelfClosingElementFields {
        HtmlSelfClosingElementFields {
            l_angle_token: self.l_angle_token(),
            name: self.name(),
            attributes: self.attributes(),
            slash_token: self.slash_token(),
            r_angle_token: self.r_angle_token(),
        }
    }
    pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<AnyHtmlTagName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn attributes(&self) -> HtmlAttributeList {
        support::list(&self.syntax, 2usize)
    }
    pub fn slash_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 3usize)
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl Serialize for HtmlSelfClosingElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlSelfClosingElementFields {
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<AnyHtmlTagName>,
    pub attributes: HtmlAttributeList,
    pub slash_token: Option<SyntaxToken>,
    pub r_angle_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlSingleTextExpression {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlSingleTextExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlSingleTextExpressionFields {
        HtmlSingleTextExpressionFields {
            l_curly_token: self.l_curly_token(),
            expression: self.expression(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for HtmlSingleTextExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlSingleTextExpressionFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<HtmlTextExpression>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlSpreadAttribute {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlSpreadAttribute {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlSpreadAttributeFields {
        HtmlSpreadAttributeFields {
            l_curly_token: self.l_curly_token(),
            dotdotdot_token: self.dotdotdot_token(),
            argument: self.argument(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn argument(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for HtmlSpreadAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlSpreadAttributeFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub dotdotdot_token: SyntaxResult<SyntaxToken>,
    pub argument: SyntaxResult<HtmlTextExpression>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlString {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlString {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlStringFields {
        HtmlStringFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for HtmlString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlStringFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlTagName {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlTagName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlTagNameFields {
        HtmlTagNameFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for HtmlTagName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlTagNameFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlTextExpression {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlTextExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlTextExpressionFields {
        HtmlTextExpressionFields {
            html_literal_token: self.html_literal_token(),
        }
    }
    pub fn html_literal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for HtmlTextExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct HtmlTextExpressionFields {
    pub html_literal_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteAnimateDirective {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteAnimateDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteAnimateDirectiveFields {
        SvelteAnimateDirectiveFields {
            animate_token: self.animate_token(),
            value: self.value(),
        }
    }
    pub fn animate_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<SvelteDirectiveValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SvelteAnimateDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteAnimateDirectiveFields {
    pub animate_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<SvelteDirectiveValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteAttachAttribute {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteAttachAttribute {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteAttachAttributeFields {
        SvelteAttachAttributeFields {
            sv_curly_at_token: self.sv_curly_at_token(),
            attach_token: self.attach_token(),
            expression: self.expression(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn sv_curly_at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn attach_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn expression(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for SvelteAttachAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteAttachAttributeFields {
    pub sv_curly_at_token: SyntaxResult<SyntaxToken>,
    pub attach_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<HtmlTextExpression>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteAwaitBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteAwaitBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteAwaitBlockFields {
        SvelteAwaitBlockFields {
            opening_block: self.opening_block(),
            clauses: self.clauses(),
            closing_block: self.closing_block(),
        }
    }
    pub fn opening_block(&self) -> SyntaxResult<SvelteAwaitOpeningBlock> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn clauses(&self) -> SvelteAwaitClausesList {
        support::list(&self.syntax, 1usize)
    }
    pub fn closing_block(&self) -> SyntaxResult<SvelteAwaitClosingBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for SvelteAwaitBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteAwaitBlockFields {
    pub opening_block: SyntaxResult<SvelteAwaitOpeningBlock>,
    pub clauses: SvelteAwaitClausesList,
    pub closing_block: SyntaxResult<SvelteAwaitClosingBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteAwaitCatchBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteAwaitCatchBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteAwaitCatchBlockFields {
        SvelteAwaitCatchBlockFields {
            sv_curly_colon_token: self.sv_curly_colon_token(),
            catch_token: self.catch_token(),
            name: self.name(),
            r_curly_token: self.r_curly_token(),
            children: self.children(),
        }
    }
    pub fn sv_curly_colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn catch_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn children(&self) -> HtmlElementList {
        support::list(&self.syntax, 4usize)
    }
}
impl Serialize for SvelteAwaitCatchBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteAwaitCatchBlockFields {
    pub sv_curly_colon_token: SyntaxResult<SyntaxToken>,
    pub catch_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<HtmlTextExpression>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
    pub children: HtmlElementList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteAwaitCatchClause {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteAwaitCatchClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteAwaitCatchClauseFields {
        SvelteAwaitCatchClauseFields {
            catch_token: self.catch_token(),
            name: self.name(),
        }
    }
    pub fn catch_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SvelteAwaitCatchClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteAwaitCatchClauseFields {
    pub catch_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<HtmlTextExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteAwaitClosingBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteAwaitClosingBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteAwaitClosingBlockFields {
        SvelteAwaitClosingBlockFields {
            sv_curly_slash_token: self.sv_curly_slash_token(),
            await_token: self.await_token(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn sv_curly_slash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn await_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SvelteAwaitClosingBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteAwaitClosingBlockFields {
    pub sv_curly_slash_token: SyntaxResult<SyntaxToken>,
    pub await_token: SyntaxResult<SyntaxToken>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteAwaitOpeningBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteAwaitOpeningBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteAwaitOpeningBlockFields {
        SvelteAwaitOpeningBlockFields {
            sv_curly_hash_token: self.sv_curly_hash_token(),
            await_token: self.await_token(),
            expression: self.expression(),
            then_clause: self.then_clause(),
            catch_clause: self.catch_clause(),
            r_curly_token: self.r_curly_token(),
            children: self.children(),
        }
    }
    pub fn sv_curly_hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn await_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn expression(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn then_clause(&self) -> Option<SvelteAwaitThenClause> {
        support::node(&self.syntax, 3usize)
    }
    pub fn catch_clause(&self) -> Option<SvelteAwaitCatchClause> {
        support::node(&self.syntax, 4usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn children(&self) -> HtmlElementList {
        support::list(&self.syntax, 6usize)
    }
}
impl Serialize for SvelteAwaitOpeningBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteAwaitOpeningBlockFields {
    pub sv_curly_hash_token: SyntaxResult<SyntaxToken>,
    pub await_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<HtmlTextExpression>,
    pub then_clause: Option<SvelteAwaitThenClause>,
    pub catch_clause: Option<SvelteAwaitCatchClause>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
    pub children: HtmlElementList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteAwaitThenBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteAwaitThenBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteAwaitThenBlockFields {
        SvelteAwaitThenBlockFields {
            sv_curly_colon_token: self.sv_curly_colon_token(),
            then_token: self.then_token(),
            name: self.name(),
            r_curly_token: self.r_curly_token(),
            children: self.children(),
        }
    }
    pub fn sv_curly_colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn then_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn children(&self) -> HtmlElementList {
        support::list(&self.syntax, 4usize)
    }
}
impl Serialize for SvelteAwaitThenBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteAwaitThenBlockFields {
    pub sv_curly_colon_token: SyntaxResult<SyntaxToken>,
    pub then_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<HtmlTextExpression>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
    pub children: HtmlElementList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteAwaitThenClause {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteAwaitThenClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteAwaitThenClauseFields {
        SvelteAwaitThenClauseFields {
            then_token: self.then_token(),
            name: self.name(),
        }
    }
    pub fn then_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SvelteAwaitThenClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteAwaitThenClauseFields {
    pub then_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<HtmlTextExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteBindDirective {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteBindDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteBindDirectiveFields {
        SvelteBindDirectiveFields {
            bind_token: self.bind_token(),
            value: self.value(),
        }
    }
    pub fn bind_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<SvelteDirectiveValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SvelteBindDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteBindDirectiveFields {
    pub bind_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<SvelteDirectiveValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteClassDirective {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteClassDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteClassDirectiveFields {
        SvelteClassDirectiveFields {
            class_token: self.class_token(),
            value: self.value(),
        }
    }
    pub fn class_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<SvelteDirectiveValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SvelteClassDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteClassDirectiveFields {
    pub class_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<SvelteDirectiveValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteConstBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteConstBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteConstBlockFields {
        SvelteConstBlockFields {
            sv_curly_at_token: self.sv_curly_at_token(),
            const_token: self.const_token(),
            expression: self.expression(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn sv_curly_at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn const_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn expression(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for SvelteConstBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteConstBlockFields {
    pub sv_curly_at_token: SyntaxResult<SyntaxToken>,
    pub const_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<HtmlTextExpression>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteCurlyDestructuredName {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteCurlyDestructuredName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteCurlyDestructuredNameFields {
        SvelteCurlyDestructuredNameFields {
            l_curly_token: self.l_curly_token(),
            names: self.names(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn names(&self) -> SvelteBindingAssignmentBindingList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SvelteCurlyDestructuredName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteCurlyDestructuredNameFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub names: SvelteBindingAssignmentBindingList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteDebugBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteDebugBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteDebugBlockFields {
        SvelteDebugBlockFields {
            sv_curly_at_token: self.sv_curly_at_token(),
            debug_token: self.debug_token(),
            bindings: self.bindings(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn sv_curly_at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn debug_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn bindings(&self) -> SvelteBindingList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for SvelteDebugBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteDebugBlockFields {
    pub sv_curly_at_token: SyntaxResult<SyntaxToken>,
    pub debug_token: SyntaxResult<SyntaxToken>,
    pub bindings: SvelteBindingList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteDirectiveModifier {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteDirectiveModifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteDirectiveModifierFields {
        SvelteDirectiveModifierFields {
            bitwise_or_token: self.bitwise_or_token(),
            name: self.name(),
        }
    }
    pub fn bitwise_or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<SvelteName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SvelteDirectiveModifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteDirectiveModifierFields {
    pub bitwise_or_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<SvelteName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteDirectiveValue {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteDirectiveValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteDirectiveValueFields {
        SvelteDirectiveValueFields {
            colon_token: self.colon_token(),
            property: self.property(),
            modifiers: self.modifiers(),
            initializer: self.initializer(),
        }
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn property(&self) -> SyntaxResult<AnySvelteBindingProperty> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn modifiers(&self) -> SvelteDirectiveModifierList {
        support::list(&self.syntax, 2usize)
    }
    pub fn initializer(&self) -> Option<HtmlAttributeInitializerClause> {
        support::node(&self.syntax, 3usize)
    }
}
impl Serialize for SvelteDirectiveValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteDirectiveValueFields {
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub property: SyntaxResult<AnySvelteBindingProperty>,
    pub modifiers: SvelteDirectiveModifierList,
    pub initializer: Option<HtmlAttributeInitializerClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteEachAsKeyedItem {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteEachAsKeyedItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteEachAsKeyedItemFields {
        SvelteEachAsKeyedItemFields {
            as_token: self.as_token(),
            name: self.name(),
            index: self.index(),
            key: self.key(),
        }
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<AnySvelteEachName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn index(&self) -> Option<SvelteEachIndex> {
        support::node(&self.syntax, 2usize)
    }
    pub fn key(&self) -> Option<SvelteEachKey> {
        support::node(&self.syntax, 3usize)
    }
}
impl Serialize for SvelteEachAsKeyedItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteEachAsKeyedItemFields {
    pub as_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<AnySvelteEachName>,
    pub index: Option<SvelteEachIndex>,
    pub key: Option<SvelteEachKey>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteEachBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteEachBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteEachBlockFields {
        SvelteEachBlockFields {
            opening_block: self.opening_block(),
            children: self.children(),
            else_clause: self.else_clause(),
            closing_block: self.closing_block(),
        }
    }
    pub fn opening_block(&self) -> SyntaxResult<SvelteEachOpeningBlock> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn children(&self) -> HtmlElementList {
        support::list(&self.syntax, 1usize)
    }
    pub fn else_clause(&self) -> Option<SvelteElseClause> {
        support::node(&self.syntax, 2usize)
    }
    pub fn closing_block(&self) -> SyntaxResult<SvelteEachClosingBlock> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl Serialize for SvelteEachBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteEachBlockFields {
    pub opening_block: SyntaxResult<SvelteEachOpeningBlock>,
    pub children: HtmlElementList,
    pub else_clause: Option<SvelteElseClause>,
    pub closing_block: SyntaxResult<SvelteEachClosingBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteEachClosingBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteEachClosingBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteEachClosingBlockFields {
        SvelteEachClosingBlockFields {
            sv_curly_slash_token: self.sv_curly_slash_token(),
            each_token: self.each_token(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn sv_curly_slash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn each_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SvelteEachClosingBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteEachClosingBlockFields {
    pub sv_curly_slash_token: SyntaxResult<SyntaxToken>,
    pub each_token: SyntaxResult<SyntaxToken>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteEachIndex {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteEachIndex {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteEachIndexFields {
        SvelteEachIndexFields {
            comma_token: self.comma_token(),
            value: self.value(),
        }
    }
    pub fn comma_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<SvelteName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SvelteEachIndex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteEachIndexFields {
    pub comma_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<SvelteName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteEachKey {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteEachKey {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteEachKeyFields {
        SvelteEachKeyFields {
            l_paren_token: self.l_paren_token(),
            expression: self.expression(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SvelteEachKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteEachKeyFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<HtmlTextExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteEachKeyedItem {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteEachKeyedItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteEachKeyedItemFields {
        SvelteEachKeyedItemFields {
            index: self.index(),
        }
    }
    pub fn index(&self) -> Option<SvelteEachIndex> {
        support::node(&self.syntax, 0usize)
    }
}
impl Serialize for SvelteEachKeyedItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteEachKeyedItemFields {
    pub index: Option<SvelteEachIndex>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteEachOpeningBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteEachOpeningBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteEachOpeningBlockFields {
        SvelteEachOpeningBlockFields {
            sv_curly_hash_token: self.sv_curly_hash_token(),
            each_token: self.each_token(),
            list: self.list(),
            item: self.item(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn sv_curly_hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn each_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn list(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn item(&self) -> Option<AnySvelteBlockItem> {
        support::node(&self.syntax, 3usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl Serialize for SvelteEachOpeningBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteEachOpeningBlockFields {
    pub sv_curly_hash_token: SyntaxResult<SyntaxToken>,
    pub each_token: SyntaxResult<SyntaxToken>,
    pub list: SyntaxResult<HtmlTextExpression>,
    pub item: Option<AnySvelteBlockItem>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteElseClause {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteElseClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteElseClauseFields {
        SvelteElseClauseFields {
            sv_curly_colon_token: self.sv_curly_colon_token(),
            else_token: self.else_token(),
            r_curly_token: self.r_curly_token(),
            children: self.children(),
        }
    }
    pub fn sv_curly_colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn else_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn children(&self) -> HtmlElementList {
        support::list(&self.syntax, 3usize)
    }
}
impl Serialize for SvelteElseClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteElseClauseFields {
    pub sv_curly_colon_token: SyntaxResult<SyntaxToken>,
    pub else_token: SyntaxResult<SyntaxToken>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
    pub children: HtmlElementList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteElseIfClause {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteElseIfClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteElseIfClauseFields {
        SvelteElseIfClauseFields {
            sv_curly_colon_token: self.sv_curly_colon_token(),
            else_token: self.else_token(),
            if_token: self.if_token(),
            expression: self.expression(),
            r_curly_token: self.r_curly_token(),
            children: self.children(),
        }
    }
    pub fn sv_curly_colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn else_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn if_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn expression(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn children(&self) -> HtmlElementList {
        support::list(&self.syntax, 5usize)
    }
}
impl Serialize for SvelteElseIfClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteElseIfClauseFields {
    pub sv_curly_colon_token: SyntaxResult<SyntaxToken>,
    pub else_token: SyntaxResult<SyntaxToken>,
    pub if_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<HtmlTextExpression>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
    pub children: HtmlElementList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteHtmlBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteHtmlBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteHtmlBlockFields {
        SvelteHtmlBlockFields {
            sv_curly_at_token: self.sv_curly_at_token(),
            html_token: self.html_token(),
            expression: self.expression(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn sv_curly_at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn html_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn expression(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for SvelteHtmlBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteHtmlBlockFields {
    pub sv_curly_at_token: SyntaxResult<SyntaxToken>,
    pub html_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<HtmlTextExpression>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteIfBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteIfBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteIfBlockFields {
        SvelteIfBlockFields {
            opening_block: self.opening_block(),
            else_if_clauses: self.else_if_clauses(),
            else_clause: self.else_clause(),
            closing_block: self.closing_block(),
        }
    }
    pub fn opening_block(&self) -> SyntaxResult<SvelteIfOpeningBlock> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn else_if_clauses(&self) -> SvelteElseIfClauseList {
        support::list(&self.syntax, 1usize)
    }
    pub fn else_clause(&self) -> Option<SvelteElseClause> {
        support::node(&self.syntax, 2usize)
    }
    pub fn closing_block(&self) -> SyntaxResult<SvelteIfClosingBlock> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl Serialize for SvelteIfBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteIfBlockFields {
    pub opening_block: SyntaxResult<SvelteIfOpeningBlock>,
    pub else_if_clauses: SvelteElseIfClauseList,
    pub else_clause: Option<SvelteElseClause>,
    pub closing_block: SyntaxResult<SvelteIfClosingBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteIfClosingBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteIfClosingBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteIfClosingBlockFields {
        SvelteIfClosingBlockFields {
            sv_curly_slash_token: self.sv_curly_slash_token(),
            if_token: self.if_token(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn sv_curly_slash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn if_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SvelteIfClosingBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteIfClosingBlockFields {
    pub sv_curly_slash_token: SyntaxResult<SyntaxToken>,
    pub if_token: SyntaxResult<SyntaxToken>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteIfOpeningBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteIfOpeningBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteIfOpeningBlockFields {
        SvelteIfOpeningBlockFields {
            sv_curly_hash_token: self.sv_curly_hash_token(),
            if_token: self.if_token(),
            expression: self.expression(),
            r_curly_token: self.r_curly_token(),
            children: self.children(),
        }
    }
    pub fn sv_curly_hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn if_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn expression(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn children(&self) -> HtmlElementList {
        support::list(&self.syntax, 4usize)
    }
}
impl Serialize for SvelteIfOpeningBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteIfOpeningBlockFields {
    pub sv_curly_hash_token: SyntaxResult<SyntaxToken>,
    pub if_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<HtmlTextExpression>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
    pub children: HtmlElementList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteInDirective {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteInDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteInDirectiveFields {
        SvelteInDirectiveFields {
            in_token: self.in_token(),
            value: self.value(),
        }
    }
    pub fn in_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<SvelteDirectiveValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SvelteInDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteInDirectiveFields {
    pub in_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<SvelteDirectiveValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteKeyBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteKeyBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteKeyBlockFields {
        SvelteKeyBlockFields {
            opening_block: self.opening_block(),
            children: self.children(),
            closing_block: self.closing_block(),
        }
    }
    pub fn opening_block(&self) -> SyntaxResult<SvelteKeyOpeningBlock> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn children(&self) -> HtmlElementList {
        support::list(&self.syntax, 1usize)
    }
    pub fn closing_block(&self) -> SyntaxResult<SvelteKeyClosingBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for SvelteKeyBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteKeyBlockFields {
    pub opening_block: SyntaxResult<SvelteKeyOpeningBlock>,
    pub children: HtmlElementList,
    pub closing_block: SyntaxResult<SvelteKeyClosingBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteKeyClosingBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteKeyClosingBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteKeyClosingBlockFields {
        SvelteKeyClosingBlockFields {
            sv_curly_slash_token: self.sv_curly_slash_token(),
            key_token: self.key_token(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn sv_curly_slash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn key_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SvelteKeyClosingBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteKeyClosingBlockFields {
    pub sv_curly_slash_token: SyntaxResult<SyntaxToken>,
    pub key_token: SyntaxResult<SyntaxToken>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteKeyOpeningBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteKeyOpeningBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteKeyOpeningBlockFields {
        SvelteKeyOpeningBlockFields {
            sv_curly_hash_token: self.sv_curly_hash_token(),
            key_token: self.key_token(),
            expression: self.expression(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn sv_curly_hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn key_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn expression(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for SvelteKeyOpeningBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteKeyOpeningBlockFields {
    pub sv_curly_hash_token: SyntaxResult<SyntaxToken>,
    pub key_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<HtmlTextExpression>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteLiteralFields {
        SvelteLiteralFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SvelteLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteLiteralFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteName {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteNameFields {
        SvelteNameFields {
            ident_token: self.ident_token(),
        }
    }
    pub fn ident_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SvelteName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteNameFields {
    pub ident_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteOutDirective {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteOutDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteOutDirectiveFields {
        SvelteOutDirectiveFields {
            out_token: self.out_token(),
            value: self.value(),
        }
    }
    pub fn out_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<SvelteDirectiveValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SvelteOutDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteOutDirectiveFields {
    pub out_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<SvelteDirectiveValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteRenderBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteRenderBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteRenderBlockFields {
        SvelteRenderBlockFields {
            sv_curly_at_token: self.sv_curly_at_token(),
            render_token: self.render_token(),
            expression: self.expression(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn sv_curly_at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn render_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn expression(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for SvelteRenderBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteRenderBlockFields {
    pub sv_curly_at_token: SyntaxResult<SyntaxToken>,
    pub render_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<HtmlTextExpression>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteRestBinding {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteRestBinding {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteRestBindingFields {
        SvelteRestBindingFields {
            dotdotdot_token: self.dotdotdot_token(),
            name: self.name(),
        }
    }
    pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<SvelteName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SvelteRestBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteRestBindingFields {
    pub dotdotdot_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<SvelteName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteSnippetBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteSnippetBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteSnippetBlockFields {
        SvelteSnippetBlockFields {
            opening_block: self.opening_block(),
            closing_block: self.closing_block(),
        }
    }
    pub fn opening_block(&self) -> SyntaxResult<SvelteSnippetOpeningBlock> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn closing_block(&self) -> SyntaxResult<SvelteSnippetClosingBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SvelteSnippetBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteSnippetBlockFields {
    pub opening_block: SyntaxResult<SvelteSnippetOpeningBlock>,
    pub closing_block: SyntaxResult<SvelteSnippetClosingBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteSnippetClosingBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteSnippetClosingBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteSnippetClosingBlockFields {
        SvelteSnippetClosingBlockFields {
            sv_curly_slash_token: self.sv_curly_slash_token(),
            snippet_token: self.snippet_token(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn sv_curly_slash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn snippet_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SvelteSnippetClosingBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteSnippetClosingBlockFields {
    pub sv_curly_slash_token: SyntaxResult<SyntaxToken>,
    pub snippet_token: SyntaxResult<SyntaxToken>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteSnippetOpeningBlock {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteSnippetOpeningBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteSnippetOpeningBlockFields {
        SvelteSnippetOpeningBlockFields {
            sv_curly_hash_token: self.sv_curly_hash_token(),
            snippet_token: self.snippet_token(),
            expression: self.expression(),
            r_curly_token: self.r_curly_token(),
            children: self.children(),
        }
    }
    pub fn sv_curly_hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn snippet_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn expression(&self) -> SyntaxResult<HtmlTextExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn children(&self) -> HtmlElementList {
        support::list(&self.syntax, 4usize)
    }
}
impl Serialize for SvelteSnippetOpeningBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteSnippetOpeningBlockFields {
    pub sv_curly_hash_token: SyntaxResult<SyntaxToken>,
    pub snippet_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<HtmlTextExpression>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
    pub children: HtmlElementList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteSquareDestructuredName {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteSquareDestructuredName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteSquareDestructuredNameFields {
        SvelteSquareDestructuredNameFields {
            l_brack_token: self.l_brack_token(),
            names: self.names(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn names(&self) -> SvelteBindingAssignmentBindingList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SvelteSquareDestructuredName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteSquareDestructuredNameFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub names: SvelteBindingAssignmentBindingList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteStyleDirective {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteStyleDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteStyleDirectiveFields {
        SvelteStyleDirectiveFields {
            style_token: self.style_token(),
            value: self.value(),
        }
    }
    pub fn style_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<SvelteDirectiveValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SvelteStyleDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteStyleDirectiveFields {
    pub style_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<SvelteDirectiveValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteTransitionDirective {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteTransitionDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteTransitionDirectiveFields {
        SvelteTransitionDirectiveFields {
            transition_token: self.transition_token(),
            value: self.value(),
        }
    }
    pub fn transition_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<SvelteDirectiveValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SvelteTransitionDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteTransitionDirectiveFields {
    pub transition_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<SvelteDirectiveValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteUseDirective {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteUseDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteUseDirectiveFields {
        SvelteUseDirectiveFields {
            use_token: self.use_token(),
            value: self.value(),
        }
    }
    pub fn use_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<SvelteDirectiveValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SvelteUseDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteUseDirectiveFields {
    pub use_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<SvelteDirectiveValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VueDirective {
    pub(crate) syntax: SyntaxNode,
}
impl VueDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> VueDirectiveFields {
        VueDirectiveFields {
            name_token: self.name_token(),
            arg: self.arg(),
            modifiers: self.modifiers(),
            initializer: self.initializer(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn arg(&self) -> Option<VueDirectiveArgument> {
        support::node(&self.syntax, 1usize)
    }
    pub fn modifiers(&self) -> VueModifierList {
        support::list(&self.syntax, 2usize)
    }
    pub fn initializer(&self) -> Option<HtmlAttributeInitializerClause> {
        support::node(&self.syntax, 3usize)
    }
}
impl Serialize for VueDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct VueDirectiveFields {
    pub name_token: SyntaxResult<SyntaxToken>,
    pub arg: Option<VueDirectiveArgument>,
    pub modifiers: VueModifierList,
    pub initializer: Option<HtmlAttributeInitializerClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VueDirectiveArgument {
    pub(crate) syntax: SyntaxNode,
}
impl VueDirectiveArgument {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> VueDirectiveArgumentFields {
        VueDirectiveArgumentFields {
            colon_token: self.colon_token(),
            arg: self.arg(),
        }
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn arg(&self) -> SyntaxResult<AnyVueDirectiveArgument> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for VueDirectiveArgument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct VueDirectiveArgumentFields {
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub arg: SyntaxResult<AnyVueDirectiveArgument>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VueDynamicArgument {
    pub(crate) syntax: SyntaxNode,
}
impl VueDynamicArgument {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> VueDynamicArgumentFields {
        VueDynamicArgumentFields {
            l_brack_token: self.l_brack_token(),
            name_token: self.name_token(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for VueDynamicArgument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct VueDynamicArgumentFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub name_token: SyntaxResult<SyntaxToken>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VueModifier {
    pub(crate) syntax: SyntaxNode,
}
impl VueModifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> VueModifierFields {
        VueModifierFields {
            dot_token: self.dot_token(),
            modifier_token: self.modifier_token(),
        }
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn modifier_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for VueModifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct VueModifierFields {
    pub dot_token: SyntaxResult<SyntaxToken>,
    pub modifier_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VueStaticArgument {
    pub(crate) syntax: SyntaxNode,
}
impl VueStaticArgument {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> VueStaticArgumentFields {
        VueStaticArgumentFields {
            name_token: self.name_token(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for VueStaticArgument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct VueStaticArgumentFields {
    pub name_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VueVBindShorthandDirective {
    pub(crate) syntax: SyntaxNode,
}
impl VueVBindShorthandDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> VueVBindShorthandDirectiveFields {
        VueVBindShorthandDirectiveFields {
            arg: self.arg(),
            modifiers: self.modifiers(),
            initializer: self.initializer(),
        }
    }
    pub fn arg(&self) -> SyntaxResult<VueDirectiveArgument> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn modifiers(&self) -> VueModifierList {
        support::list(&self.syntax, 1usize)
    }
    pub fn initializer(&self) -> Option<HtmlAttributeInitializerClause> {
        support::node(&self.syntax, 2usize)
    }
}
impl Serialize for VueVBindShorthandDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct VueVBindShorthandDirectiveFields {
    pub arg: SyntaxResult<VueDirectiveArgument>,
    pub modifiers: VueModifierList,
    pub initializer: Option<HtmlAttributeInitializerClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VueVOnShorthandDirective {
    pub(crate) syntax: SyntaxNode,
}
impl VueVOnShorthandDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> VueVOnShorthandDirectiveFields {
        VueVOnShorthandDirectiveFields {
            at_token: self.at_token(),
            arg: self.arg(),
            modifiers: self.modifiers(),
            initializer: self.initializer(),
        }
    }
    pub fn at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn arg(&self) -> SyntaxResult<AnyVueDirectiveArgument> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn modifiers(&self) -> VueModifierList {
        support::list(&self.syntax, 2usize)
    }
    pub fn initializer(&self) -> Option<HtmlAttributeInitializerClause> {
        support::node(&self.syntax, 3usize)
    }
}
impl Serialize for VueVOnShorthandDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct VueVOnShorthandDirectiveFields {
    pub at_token: SyntaxResult<SyntaxToken>,
    pub arg: SyntaxResult<AnyVueDirectiveArgument>,
    pub modifiers: VueModifierList,
    pub initializer: Option<HtmlAttributeInitializerClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VueVSlotShorthandDirective {
    pub(crate) syntax: SyntaxNode,
}
impl VueVSlotShorthandDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> VueVSlotShorthandDirectiveFields {
        VueVSlotShorthandDirectiveFields {
            hash_token: self.hash_token(),
            arg: self.arg(),
            modifiers: self.modifiers(),
            initializer: self.initializer(),
        }
    }
    pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn arg(&self) -> SyntaxResult<AnyVueDirectiveArgument> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn modifiers(&self) -> VueModifierList {
        support::list(&self.syntax, 2usize)
    }
    pub fn initializer(&self) -> Option<HtmlAttributeInitializerClause> {
        support::node(&self.syntax, 3usize)
    }
}
impl Serialize for VueVSlotShorthandDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct VueVSlotShorthandDirectiveFields {
    pub hash_token: SyntaxResult<SyntaxToken>,
    pub arg: SyntaxResult<AnyVueDirectiveArgument>,
    pub modifiers: VueModifierList,
    pub initializer: Option<HtmlAttributeInitializerClause>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyAstroDirective {
    AstroClassDirective(AstroClassDirective),
    AstroClientDirective(AstroClientDirective),
    AstroDefineDirective(AstroDefineDirective),
    AstroIsDirective(AstroIsDirective),
    AstroServerDirective(AstroServerDirective),
    AstroSetDirective(AstroSetDirective),
}
impl AnyAstroDirective {
    pub fn as_astro_class_directive(&self) -> Option<&AstroClassDirective> {
        match &self {
            Self::AstroClassDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_astro_client_directive(&self) -> Option<&AstroClientDirective> {
        match &self {
            Self::AstroClientDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_astro_define_directive(&self) -> Option<&AstroDefineDirective> {
        match &self {
            Self::AstroDefineDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_astro_is_directive(&self) -> Option<&AstroIsDirective> {
        match &self {
            Self::AstroIsDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_astro_server_directive(&self) -> Option<&AstroServerDirective> {
        match &self {
            Self::AstroServerDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_astro_set_directive(&self) -> Option<&AstroSetDirective> {
        match &self {
            Self::AstroSetDirective(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyAstroFrontmatterElement {
    AstroBogusFrontmatter(AstroBogusFrontmatter),
    AstroFrontmatterElement(AstroFrontmatterElement),
}
impl AnyAstroFrontmatterElement {
    pub fn as_astro_bogus_frontmatter(&self) -> Option<&AstroBogusFrontmatter> {
        match &self {
            Self::AstroBogusFrontmatter(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_astro_frontmatter_element(&self) -> Option<&AstroFrontmatterElement> {
        match &self {
            Self::AstroFrontmatterElement(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyHtmlAttribute {
    AnyAstroDirective(AnyAstroDirective),
    AnySvelteDirective(AnySvelteDirective),
    AnyVueDirective(AnyVueDirective),
    HtmlAttribute(HtmlAttribute),
    HtmlAttributeDoubleTextExpression(HtmlAttributeDoubleTextExpression),
    HtmlAttributeSingleTextExpression(HtmlAttributeSingleTextExpression),
    HtmlBogusAttribute(HtmlBogusAttribute),
    HtmlSpreadAttribute(HtmlSpreadAttribute),
    SvelteAttachAttribute(SvelteAttachAttribute),
}
impl AnyHtmlAttribute {
    pub fn as_any_astro_directive(&self) -> Option<&AnyAstroDirective> {
        match &self {
            Self::AnyAstroDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_any_svelte_directive(&self) -> Option<&AnySvelteDirective> {
        match &self {
            Self::AnySvelteDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_any_vue_directive(&self) -> Option<&AnyVueDirective> {
        match &self {
            Self::AnyVueDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_attribute(&self) -> Option<&HtmlAttribute> {
        match &self {
            Self::HtmlAttribute(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_attribute_double_text_expression(
        &self,
    ) -> Option<&HtmlAttributeDoubleTextExpression> {
        match &self {
            Self::HtmlAttributeDoubleTextExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_attribute_single_text_expression(
        &self,
    ) -> Option<&HtmlAttributeSingleTextExpression> {
        match &self {
            Self::HtmlAttributeSingleTextExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_bogus_attribute(&self) -> Option<&HtmlBogusAttribute> {
        match &self {
            Self::HtmlBogusAttribute(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_spread_attribute(&self) -> Option<&HtmlSpreadAttribute> {
        match &self {
            Self::HtmlSpreadAttribute(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_attach_attribute(&self) -> Option<&SvelteAttachAttribute> {
        match &self {
            Self::SvelteAttachAttribute(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyHtmlAttributeInitializer {
    HtmlAttributeSingleTextExpression(HtmlAttributeSingleTextExpression),
    HtmlString(HtmlString),
}
impl AnyHtmlAttributeInitializer {
    pub fn as_html_attribute_single_text_expression(
        &self,
    ) -> Option<&HtmlAttributeSingleTextExpression> {
        match &self {
            Self::HtmlAttributeSingleTextExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_string(&self) -> Option<&HtmlString> {
        match &self {
            Self::HtmlString(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyHtmlComponentObjectName {
    HtmlComponentName(HtmlComponentName),
    HtmlMemberName(HtmlMemberName),
    HtmlTagName(HtmlTagName),
}
impl AnyHtmlComponentObjectName {
    pub fn as_html_component_name(&self) -> Option<&HtmlComponentName> {
        match &self {
            Self::HtmlComponentName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_member_name(&self) -> Option<&HtmlMemberName> {
        match &self {
            Self::HtmlMemberName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_tag_name(&self) -> Option<&HtmlTagName> {
        match &self {
            Self::HtmlTagName(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyHtmlContent {
    AnyHtmlTextExpression(AnyHtmlTextExpression),
    HtmlContent(HtmlContent),
    HtmlEmbeddedContent(HtmlEmbeddedContent),
}
impl AnyHtmlContent {
    pub fn as_any_html_text_expression(&self) -> Option<&AnyHtmlTextExpression> {
        match &self {
            Self::AnyHtmlTextExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_content(&self) -> Option<&HtmlContent> {
        match &self {
            Self::HtmlContent(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_embedded_content(&self) -> Option<&HtmlEmbeddedContent> {
        match &self {
            Self::HtmlEmbeddedContent(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyHtmlElement {
    AnyHtmlContent(AnyHtmlContent),
    HtmlBogusElement(HtmlBogusElement),
    HtmlCdataSection(HtmlCdataSection),
    HtmlElement(HtmlElement),
    HtmlSelfClosingElement(HtmlSelfClosingElement),
}
impl AnyHtmlElement {
    pub fn as_any_html_content(&self) -> Option<&AnyHtmlContent> {
        match &self {
            Self::AnyHtmlContent(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_bogus_element(&self) -> Option<&HtmlBogusElement> {
        match &self {
            Self::HtmlBogusElement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_cdata_section(&self) -> Option<&HtmlCdataSection> {
        match &self {
            Self::HtmlCdataSection(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_element(&self) -> Option<&HtmlElement> {
        match &self {
            Self::HtmlElement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_self_closing_element(&self) -> Option<&HtmlSelfClosingElement> {
        match &self {
            Self::HtmlSelfClosingElement(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyHtmlTagName {
    HtmlComponentName(HtmlComponentName),
    HtmlMemberName(HtmlMemberName),
    HtmlTagName(HtmlTagName),
}
impl AnyHtmlTagName {
    pub fn as_html_component_name(&self) -> Option<&HtmlComponentName> {
        match &self {
            Self::HtmlComponentName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_member_name(&self) -> Option<&HtmlMemberName> {
        match &self {
            Self::HtmlMemberName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_tag_name(&self) -> Option<&HtmlTagName> {
        match &self {
            Self::HtmlTagName(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyHtmlTextExpression {
    AnySvelteBlock(AnySvelteBlock),
    HtmlBogusTextExpression(HtmlBogusTextExpression),
    HtmlDoubleTextExpression(HtmlDoubleTextExpression),
    HtmlSingleTextExpression(HtmlSingleTextExpression),
}
impl AnyHtmlTextExpression {
    pub fn as_any_svelte_block(&self) -> Option<&AnySvelteBlock> {
        match &self {
            Self::AnySvelteBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_bogus_text_expression(&self) -> Option<&HtmlBogusTextExpression> {
        match &self {
            Self::HtmlBogusTextExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_double_text_expression(&self) -> Option<&HtmlDoubleTextExpression> {
        match &self {
            Self::HtmlDoubleTextExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_single_text_expression(&self) -> Option<&HtmlSingleTextExpression> {
        match &self {
            Self::HtmlSingleTextExpression(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySvelteAwaitClauses {
    SvelteAwaitCatchBlock(SvelteAwaitCatchBlock),
    SvelteAwaitThenBlock(SvelteAwaitThenBlock),
    SvelteBogusBlock(SvelteBogusBlock),
}
impl AnySvelteAwaitClauses {
    pub fn as_svelte_await_catch_block(&self) -> Option<&SvelteAwaitCatchBlock> {
        match &self {
            Self::SvelteAwaitCatchBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_await_then_block(&self) -> Option<&SvelteAwaitThenBlock> {
        match &self {
            Self::SvelteAwaitThenBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_bogus_block(&self) -> Option<&SvelteBogusBlock> {
        match &self {
            Self::SvelteBogusBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySvelteBindingAssignmentBinding {
    SvelteName(SvelteName),
    SvelteRestBinding(SvelteRestBinding),
}
impl AnySvelteBindingAssignmentBinding {
    pub fn as_svelte_name(&self) -> Option<&SvelteName> {
        match &self {
            Self::SvelteName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_rest_binding(&self) -> Option<&SvelteRestBinding> {
        match &self {
            Self::SvelteRestBinding(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySvelteBindingProperty {
    SvelteLiteral(SvelteLiteral),
    SvelteName(SvelteName),
}
impl AnySvelteBindingProperty {
    pub fn as_svelte_literal(&self) -> Option<&SvelteLiteral> {
        match &self {
            Self::SvelteLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_name(&self) -> Option<&SvelteName> {
        match &self {
            Self::SvelteName(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySvelteBlock {
    SvelteAwaitBlock(SvelteAwaitBlock),
    SvelteBogusBlock(SvelteBogusBlock),
    SvelteConstBlock(SvelteConstBlock),
    SvelteDebugBlock(SvelteDebugBlock),
    SvelteEachBlock(SvelteEachBlock),
    SvelteHtmlBlock(SvelteHtmlBlock),
    SvelteIfBlock(SvelteIfBlock),
    SvelteKeyBlock(SvelteKeyBlock),
    SvelteRenderBlock(SvelteRenderBlock),
    SvelteSnippetBlock(SvelteSnippetBlock),
}
impl AnySvelteBlock {
    pub fn as_svelte_await_block(&self) -> Option<&SvelteAwaitBlock> {
        match &self {
            Self::SvelteAwaitBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_bogus_block(&self) -> Option<&SvelteBogusBlock> {
        match &self {
            Self::SvelteBogusBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_const_block(&self) -> Option<&SvelteConstBlock> {
        match &self {
            Self::SvelteConstBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_debug_block(&self) -> Option<&SvelteDebugBlock> {
        match &self {
            Self::SvelteDebugBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_each_block(&self) -> Option<&SvelteEachBlock> {
        match &self {
            Self::SvelteEachBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_html_block(&self) -> Option<&SvelteHtmlBlock> {
        match &self {
            Self::SvelteHtmlBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_if_block(&self) -> Option<&SvelteIfBlock> {
        match &self {
            Self::SvelteIfBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_key_block(&self) -> Option<&SvelteKeyBlock> {
        match &self {
            Self::SvelteKeyBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_render_block(&self) -> Option<&SvelteRenderBlock> {
        match &self {
            Self::SvelteRenderBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_snippet_block(&self) -> Option<&SvelteSnippetBlock> {
        match &self {
            Self::SvelteSnippetBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySvelteBlockItem {
    SvelteEachAsKeyedItem(SvelteEachAsKeyedItem),
    SvelteEachKeyedItem(SvelteEachKeyedItem),
}
impl AnySvelteBlockItem {
    pub fn as_svelte_each_as_keyed_item(&self) -> Option<&SvelteEachAsKeyedItem> {
        match &self {
            Self::SvelteEachAsKeyedItem(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_each_keyed_item(&self) -> Option<&SvelteEachKeyedItem> {
        match &self {
            Self::SvelteEachKeyedItem(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySvelteDestructuredName {
    SvelteCurlyDestructuredName(SvelteCurlyDestructuredName),
    SvelteSquareDestructuredName(SvelteSquareDestructuredName),
}
impl AnySvelteDestructuredName {
    pub fn as_svelte_curly_destructured_name(&self) -> Option<&SvelteCurlyDestructuredName> {
        match &self {
            Self::SvelteCurlyDestructuredName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_square_destructured_name(&self) -> Option<&SvelteSquareDestructuredName> {
        match &self {
            Self::SvelteSquareDestructuredName(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySvelteDirective {
    SvelteAnimateDirective(SvelteAnimateDirective),
    SvelteBindDirective(SvelteBindDirective),
    SvelteClassDirective(SvelteClassDirective),
    SvelteInDirective(SvelteInDirective),
    SvelteOutDirective(SvelteOutDirective),
    SvelteStyleDirective(SvelteStyleDirective),
    SvelteTransitionDirective(SvelteTransitionDirective),
    SvelteUseDirective(SvelteUseDirective),
}
impl AnySvelteDirective {
    pub fn as_svelte_animate_directive(&self) -> Option<&SvelteAnimateDirective> {
        match &self {
            Self::SvelteAnimateDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_bind_directive(&self) -> Option<&SvelteBindDirective> {
        match &self {
            Self::SvelteBindDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_class_directive(&self) -> Option<&SvelteClassDirective> {
        match &self {
            Self::SvelteClassDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_in_directive(&self) -> Option<&SvelteInDirective> {
        match &self {
            Self::SvelteInDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_out_directive(&self) -> Option<&SvelteOutDirective> {
        match &self {
            Self::SvelteOutDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_style_directive(&self) -> Option<&SvelteStyleDirective> {
        match &self {
            Self::SvelteStyleDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_transition_directive(&self) -> Option<&SvelteTransitionDirective> {
        match &self {
            Self::SvelteTransitionDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_use_directive(&self) -> Option<&SvelteUseDirective> {
        match &self {
            Self::SvelteUseDirective(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySvelteEachName {
    AnySvelteDestructuredName(AnySvelteDestructuredName),
    HtmlTextExpression(HtmlTextExpression),
    SvelteName(SvelteName),
}
impl AnySvelteEachName {
    pub fn as_any_svelte_destructured_name(&self) -> Option<&AnySvelteDestructuredName> {
        match &self {
            Self::AnySvelteDestructuredName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_text_expression(&self) -> Option<&HtmlTextExpression> {
        match &self {
            Self::HtmlTextExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_name(&self) -> Option<&SvelteName> {
        match &self {
            Self::SvelteName(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyVueDirective {
    VueBogusDirective(VueBogusDirective),
    VueDirective(VueDirective),
    VueVBindShorthandDirective(VueVBindShorthandDirective),
    VueVOnShorthandDirective(VueVOnShorthandDirective),
    VueVSlotShorthandDirective(VueVSlotShorthandDirective),
}
impl AnyVueDirective {
    pub fn as_vue_bogus_directive(&self) -> Option<&VueBogusDirective> {
        match &self {
            Self::VueBogusDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_vue_directive(&self) -> Option<&VueDirective> {
        match &self {
            Self::VueDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_vue_v_bind_shorthand_directive(&self) -> Option<&VueVBindShorthandDirective> {
        match &self {
            Self::VueVBindShorthandDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_vue_v_on_shorthand_directive(&self) -> Option<&VueVOnShorthandDirective> {
        match &self {
            Self::VueVOnShorthandDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_vue_v_slot_shorthand_directive(&self) -> Option<&VueVSlotShorthandDirective> {
        match &self {
            Self::VueVSlotShorthandDirective(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyVueDirectiveArgument {
    VueBogusDirectiveArgument(VueBogusDirectiveArgument),
    VueDynamicArgument(VueDynamicArgument),
    VueStaticArgument(VueStaticArgument),
}
impl AnyVueDirectiveArgument {
    pub fn as_vue_bogus_directive_argument(&self) -> Option<&VueBogusDirectiveArgument> {
        match &self {
            Self::VueBogusDirectiveArgument(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_vue_dynamic_argument(&self) -> Option<&VueDynamicArgument> {
        match &self {
            Self::VueDynamicArgument(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_vue_static_argument(&self) -> Option<&VueStaticArgument> {
        match &self {
            Self::VueStaticArgument(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for AstroClassDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ASTRO_CLASS_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ASTRO_CLASS_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for AstroClassDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("AstroClassDirective")
                .field(
                    "class_token",
                    &support::DebugSyntaxResult(self.class_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("AstroClassDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<AstroClassDirective> for SyntaxNode {
    fn from(n: AstroClassDirective) -> Self {
        n.syntax
    }
}
impl From<AstroClassDirective> for SyntaxElement {
    fn from(n: AstroClassDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for AstroClientDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ASTRO_CLIENT_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ASTRO_CLIENT_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for AstroClientDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("AstroClientDirective")
                .field(
                    "client_token",
                    &support::DebugSyntaxResult(self.client_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("AstroClientDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<AstroClientDirective> for SyntaxNode {
    fn from(n: AstroClientDirective) -> Self {
        n.syntax
    }
}
impl From<AstroClientDirective> for SyntaxElement {
    fn from(n: AstroClientDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for AstroDefineDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ASTRO_DEFINE_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ASTRO_DEFINE_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for AstroDefineDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("AstroDefineDirective")
                .field(
                    "define_token",
                    &support::DebugSyntaxResult(self.define_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("AstroDefineDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<AstroDefineDirective> for SyntaxNode {
    fn from(n: AstroDefineDirective) -> Self {
        n.syntax
    }
}
impl From<AstroDefineDirective> for SyntaxElement {
    fn from(n: AstroDefineDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for AstroDirectiveValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ASTRO_DIRECTIVE_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ASTRO_DIRECTIVE_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for AstroDirectiveValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("AstroDirectiveValue")
                .field(
                    "colon_token_token",
                    &support::DebugSyntaxResult(self.colon_token_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "initializer",
                    &support::DebugOptionalElement(self.initializer()),
                )
                .finish()
        } else {
            f.debug_struct("AstroDirectiveValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<AstroDirectiveValue> for SyntaxNode {
    fn from(n: AstroDirectiveValue) -> Self {
        n.syntax
    }
}
impl From<AstroDirectiveValue> for SyntaxElement {
    fn from(n: AstroDirectiveValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for AstroEmbeddedContent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ASTRO_EMBEDDED_CONTENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ASTRO_EMBEDDED_CONTENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for AstroEmbeddedContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("AstroEmbeddedContent")
                .field(
                    "content_token",
                    &support::DebugOptionalElement(self.content_token()),
                )
                .finish()
        } else {
            f.debug_struct("AstroEmbeddedContent").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<AstroEmbeddedContent> for SyntaxNode {
    fn from(n: AstroEmbeddedContent) -> Self {
        n.syntax
    }
}
impl From<AstroEmbeddedContent> for SyntaxElement {
    fn from(n: AstroEmbeddedContent) -> Self {
        n.syntax.into()
    }
}
impl AstNode for AstroFrontmatterElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ASTRO_FRONTMATTER_ELEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ASTRO_FRONTMATTER_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for AstroFrontmatterElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("AstroFrontmatterElement")
                .field(
                    "l_fence_token",
                    &support::DebugSyntaxResult(self.l_fence_token()),
                )
                .field("content", &support::DebugSyntaxResult(self.content()))
                .field(
                    "r_fence_token",
                    &support::DebugSyntaxResult(self.r_fence_token()),
                )
                .finish()
        } else {
            f.debug_struct("AstroFrontmatterElement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<AstroFrontmatterElement> for SyntaxNode {
    fn from(n: AstroFrontmatterElement) -> Self {
        n.syntax
    }
}
impl From<AstroFrontmatterElement> for SyntaxElement {
    fn from(n: AstroFrontmatterElement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for AstroIsDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ASTRO_IS_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ASTRO_IS_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for AstroIsDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("AstroIsDirective")
                .field("is_token", &support::DebugSyntaxResult(self.is_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("AstroIsDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<AstroIsDirective> for SyntaxNode {
    fn from(n: AstroIsDirective) -> Self {
        n.syntax
    }
}
impl From<AstroIsDirective> for SyntaxElement {
    fn from(n: AstroIsDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for AstroServerDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ASTRO_SERVER_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ASTRO_SERVER_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for AstroServerDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("AstroServerDirective")
                .field(
                    "server_token",
                    &support::DebugSyntaxResult(self.server_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("AstroServerDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<AstroServerDirective> for SyntaxNode {
    fn from(n: AstroServerDirective) -> Self {
        n.syntax
    }
}
impl From<AstroServerDirective> for SyntaxElement {
    fn from(n: AstroServerDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for AstroSetDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ASTRO_SET_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ASTRO_SET_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for AstroSetDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("AstroSetDirective")
                .field("set_token", &support::DebugSyntaxResult(self.set_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("AstroSetDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<AstroSetDirective> for SyntaxNode {
    fn from(n: AstroSetDirective) -> Self {
        n.syntax
    }
}
impl From<AstroSetDirective> for SyntaxElement {
    fn from(n: AstroSetDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlAttribute {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_ATTRIBUTE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_ATTRIBUTE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlAttribute")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "initializer",
                    &support::DebugOptionalElement(self.initializer()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlAttribute").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlAttribute> for SyntaxNode {
    fn from(n: HtmlAttribute) -> Self {
        n.syntax
    }
}
impl From<HtmlAttribute> for SyntaxElement {
    fn from(n: HtmlAttribute) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlAttributeDoubleTextExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_ATTRIBUTE_DOUBLE_TEXT_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_ATTRIBUTE_DOUBLE_TEXT_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlAttributeDoubleTextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlAttributeDoubleTextExpression")
                .field(
                    "l_double_curly_token",
                    &support::DebugSyntaxResult(self.l_double_curly_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "r_double_curly_token",
                    &support::DebugSyntaxResult(self.r_double_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlAttributeDoubleTextExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlAttributeDoubleTextExpression> for SyntaxNode {
    fn from(n: HtmlAttributeDoubleTextExpression) -> Self {
        n.syntax
    }
}
impl From<HtmlAttributeDoubleTextExpression> for SyntaxElement {
    fn from(n: HtmlAttributeDoubleTextExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlAttributeInitializerClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_ATTRIBUTE_INITIALIZER_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_ATTRIBUTE_INITIALIZER_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlAttributeInitializerClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlAttributeInitializerClause")
                .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("HtmlAttributeInitializerClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlAttributeInitializerClause> for SyntaxNode {
    fn from(n: HtmlAttributeInitializerClause) -> Self {
        n.syntax
    }
}
impl From<HtmlAttributeInitializerClause> for SyntaxElement {
    fn from(n: HtmlAttributeInitializerClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlAttributeName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_ATTRIBUTE_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_ATTRIBUTE_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlAttributeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlAttributeName")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlAttributeName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlAttributeName> for SyntaxNode {
    fn from(n: HtmlAttributeName) -> Self {
        n.syntax
    }
}
impl From<HtmlAttributeName> for SyntaxElement {
    fn from(n: HtmlAttributeName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlAttributeSingleTextExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_ATTRIBUTE_SINGLE_TEXT_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_ATTRIBUTE_SINGLE_TEXT_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlAttributeSingleTextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlAttributeSingleTextExpression")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlAttributeSingleTextExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlAttributeSingleTextExpression> for SyntaxNode {
    fn from(n: HtmlAttributeSingleTextExpression) -> Self {
        n.syntax
    }
}
impl From<HtmlAttributeSingleTextExpression> for SyntaxElement {
    fn from(n: HtmlAttributeSingleTextExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlCdataSection {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_CDATA_SECTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_CDATA_SECTION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlCdataSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlCdataSection")
                .field(
                    "cdata_start_token",
                    &support::DebugSyntaxResult(self.cdata_start_token()),
                )
                .field(
                    "content_token",
                    &support::DebugSyntaxResult(self.content_token()),
                )
                .field(
                    "cdata_end_token",
                    &support::DebugSyntaxResult(self.cdata_end_token()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlCdataSection").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlCdataSection> for SyntaxNode {
    fn from(n: HtmlCdataSection) -> Self {
        n.syntax
    }
}
impl From<HtmlCdataSection> for SyntaxElement {
    fn from(n: HtmlCdataSection) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlClosingElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_CLOSING_ELEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_CLOSING_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlClosingElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlClosingElement")
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
            f.debug_struct("HtmlClosingElement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlClosingElement> for SyntaxNode {
    fn from(n: HtmlClosingElement) -> Self {
        n.syntax
    }
}
impl From<HtmlClosingElement> for SyntaxElement {
    fn from(n: HtmlClosingElement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlComponentName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_COMPONENT_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_COMPONENT_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlComponentName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlComponentName")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlComponentName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlComponentName> for SyntaxNode {
    fn from(n: HtmlComponentName) -> Self {
        n.syntax
    }
}
impl From<HtmlComponentName> for SyntaxElement {
    fn from(n: HtmlComponentName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlContent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_CONTENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_CONTENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlContent")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlContent").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlContent> for SyntaxNode {
    fn from(n: HtmlContent) -> Self {
        n.syntax
    }
}
impl From<HtmlContent> for SyntaxElement {
    fn from(n: HtmlContent) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlDirective")
                .field(
                    "l_angle_token",
                    &support::DebugSyntaxResult(self.l_angle_token()),
                )
                .field("excl_token", &support::DebugSyntaxResult(self.excl_token()))
                .field(
                    "doctype_token",
                    &support::DebugSyntaxResult(self.doctype_token()),
                )
                .field(
                    "html_token",
                    &support::DebugOptionalElement(self.html_token()),
                )
                .field(
                    "quirk_token",
                    &support::DebugOptionalElement(self.quirk_token()),
                )
                .field(
                    "public_id_token",
                    &support::DebugOptionalElement(self.public_id_token()),
                )
                .field(
                    "system_id_token",
                    &support::DebugOptionalElement(self.system_id_token()),
                )
                .field(
                    "r_angle_token",
                    &support::DebugSyntaxResult(self.r_angle_token()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlDirective> for SyntaxNode {
    fn from(n: HtmlDirective) -> Self {
        n.syntax
    }
}
impl From<HtmlDirective> for SyntaxElement {
    fn from(n: HtmlDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlDoubleTextExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_DOUBLE_TEXT_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_DOUBLE_TEXT_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlDoubleTextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlDoubleTextExpression")
                .field(
                    "l_double_curly_token",
                    &support::DebugSyntaxResult(self.l_double_curly_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "r_double_curly_token",
                    &support::DebugSyntaxResult(self.r_double_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlDoubleTextExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlDoubleTextExpression> for SyntaxNode {
    fn from(n: HtmlDoubleTextExpression) -> Self {
        n.syntax
    }
}
impl From<HtmlDoubleTextExpression> for SyntaxElement {
    fn from(n: HtmlDoubleTextExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_ELEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlElement")
                .field(
                    "opening_element",
                    &support::DebugSyntaxResult(self.opening_element()),
                )
                .field("children", &self.children())
                .field(
                    "closing_element",
                    &support::DebugSyntaxResult(self.closing_element()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlElement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlElement> for SyntaxNode {
    fn from(n: HtmlElement) -> Self {
        n.syntax
    }
}
impl From<HtmlElement> for SyntaxElement {
    fn from(n: HtmlElement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlEmbeddedContent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_EMBEDDED_CONTENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_EMBEDDED_CONTENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlEmbeddedContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlEmbeddedContent")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlEmbeddedContent").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlEmbeddedContent> for SyntaxNode {
    fn from(n: HtmlEmbeddedContent) -> Self {
        n.syntax
    }
}
impl From<HtmlEmbeddedContent> for SyntaxElement {
    fn from(n: HtmlEmbeddedContent) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlMemberName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_MEMBER_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_MEMBER_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlMemberName")
                .field("object", &support::DebugSyntaxResult(self.object()))
                .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
                .field("member", &support::DebugSyntaxResult(self.member()))
                .finish()
        } else {
            f.debug_struct("HtmlMemberName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlMemberName> for SyntaxNode {
    fn from(n: HtmlMemberName) -> Self {
        n.syntax
    }
}
impl From<HtmlMemberName> for SyntaxElement {
    fn from(n: HtmlMemberName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlOpeningElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_OPENING_ELEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_OPENING_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlOpeningElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlOpeningElement")
                .field(
                    "l_angle_token",
                    &support::DebugSyntaxResult(self.l_angle_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("attributes", &self.attributes())
                .field(
                    "r_angle_token",
                    &support::DebugSyntaxResult(self.r_angle_token()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlOpeningElement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlOpeningElement> for SyntaxNode {
    fn from(n: HtmlOpeningElement) -> Self {
        n.syntax
    }
}
impl From<HtmlOpeningElement> for SyntaxElement {
    fn from(n: HtmlOpeningElement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_ROOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlRoot")
                .field(
                    "bom_token",
                    &support::DebugOptionalElement(self.bom_token()),
                )
                .field(
                    "frontmatter",
                    &support::DebugOptionalElement(self.frontmatter()),
                )
                .field(
                    "directive",
                    &support::DebugOptionalElement(self.directive()),
                )
                .field("html", &self.html())
                .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
                .finish()
        } else {
            f.debug_struct("HtmlRoot").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlRoot> for SyntaxNode {
    fn from(n: HtmlRoot) -> Self {
        n.syntax
    }
}
impl From<HtmlRoot> for SyntaxElement {
    fn from(n: HtmlRoot) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlSelfClosingElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_SELF_CLOSING_ELEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_SELF_CLOSING_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlSelfClosingElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlSelfClosingElement")
                .field(
                    "l_angle_token",
                    &support::DebugSyntaxResult(self.l_angle_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("attributes", &self.attributes())
                .field(
                    "slash_token",
                    &support::DebugOptionalElement(self.slash_token()),
                )
                .field(
                    "r_angle_token",
                    &support::DebugSyntaxResult(self.r_angle_token()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlSelfClosingElement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlSelfClosingElement> for SyntaxNode {
    fn from(n: HtmlSelfClosingElement) -> Self {
        n.syntax
    }
}
impl From<HtmlSelfClosingElement> for SyntaxElement {
    fn from(n: HtmlSelfClosingElement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlSingleTextExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_SINGLE_TEXT_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_SINGLE_TEXT_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlSingleTextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlSingleTextExpression")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlSingleTextExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlSingleTextExpression> for SyntaxNode {
    fn from(n: HtmlSingleTextExpression) -> Self {
        n.syntax
    }
}
impl From<HtmlSingleTextExpression> for SyntaxElement {
    fn from(n: HtmlSingleTextExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlSpreadAttribute {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_SPREAD_ATTRIBUTE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_SPREAD_ATTRIBUTE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlSpreadAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlSpreadAttribute")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field(
                    "dotdotdot_token",
                    &support::DebugSyntaxResult(self.dotdotdot_token()),
                )
                .field("argument", &support::DebugSyntaxResult(self.argument()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlSpreadAttribute").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlSpreadAttribute> for SyntaxNode {
    fn from(n: HtmlSpreadAttribute) -> Self {
        n.syntax
    }
}
impl From<HtmlSpreadAttribute> for SyntaxElement {
    fn from(n: HtmlSpreadAttribute) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlString {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_STRING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_STRING
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlString")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlString").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlString> for SyntaxNode {
    fn from(n: HtmlString) -> Self {
        n.syntax
    }
}
impl From<HtmlString> for SyntaxElement {
    fn from(n: HtmlString) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlTagName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_TAG_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_TAG_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlTagName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlTagName")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlTagName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlTagName> for SyntaxNode {
    fn from(n: HtmlTagName) -> Self {
        n.syntax
    }
}
impl From<HtmlTagName> for SyntaxElement {
    fn from(n: HtmlTagName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for HtmlTextExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_TEXT_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_TEXT_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlTextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("HtmlTextExpression")
                .field(
                    "html_literal_token",
                    &support::DebugSyntaxResult(self.html_literal_token()),
                )
                .finish()
        } else {
            f.debug_struct("HtmlTextExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<HtmlTextExpression> for SyntaxNode {
    fn from(n: HtmlTextExpression) -> Self {
        n.syntax
    }
}
impl From<HtmlTextExpression> for SyntaxElement {
    fn from(n: HtmlTextExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteAnimateDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_ANIMATE_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_ANIMATE_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteAnimateDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteAnimateDirective")
                .field(
                    "animate_token",
                    &support::DebugSyntaxResult(self.animate_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("SvelteAnimateDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteAnimateDirective> for SyntaxNode {
    fn from(n: SvelteAnimateDirective) -> Self {
        n.syntax
    }
}
impl From<SvelteAnimateDirective> for SyntaxElement {
    fn from(n: SvelteAnimateDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteAttachAttribute {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_ATTACH_ATTRIBUTE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_ATTACH_ATTRIBUTE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteAttachAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteAttachAttribute")
                .field(
                    "sv_curly_at_token",
                    &support::DebugSyntaxResult(self.sv_curly_at_token()),
                )
                .field(
                    "attach_token",
                    &support::DebugSyntaxResult(self.attach_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteAttachAttribute").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteAttachAttribute> for SyntaxNode {
    fn from(n: SvelteAttachAttribute) -> Self {
        n.syntax
    }
}
impl From<SvelteAttachAttribute> for SyntaxElement {
    fn from(n: SvelteAttachAttribute) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteAwaitBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_AWAIT_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_AWAIT_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteAwaitBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteAwaitBlock")
                .field(
                    "opening_block",
                    &support::DebugSyntaxResult(self.opening_block()),
                )
                .field("clauses", &self.clauses())
                .field(
                    "closing_block",
                    &support::DebugSyntaxResult(self.closing_block()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteAwaitBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteAwaitBlock> for SyntaxNode {
    fn from(n: SvelteAwaitBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteAwaitBlock> for SyntaxElement {
    fn from(n: SvelteAwaitBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteAwaitCatchBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_AWAIT_CATCH_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_AWAIT_CATCH_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteAwaitCatchBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteAwaitCatchBlock")
                .field(
                    "sv_curly_colon_token",
                    &support::DebugSyntaxResult(self.sv_curly_colon_token()),
                )
                .field(
                    "catch_token",
                    &support::DebugSyntaxResult(self.catch_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .field("children", &self.children())
                .finish()
        } else {
            f.debug_struct("SvelteAwaitCatchBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteAwaitCatchBlock> for SyntaxNode {
    fn from(n: SvelteAwaitCatchBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteAwaitCatchBlock> for SyntaxElement {
    fn from(n: SvelteAwaitCatchBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteAwaitCatchClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_AWAIT_CATCH_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_AWAIT_CATCH_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteAwaitCatchClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteAwaitCatchClause")
                .field(
                    "catch_token",
                    &support::DebugSyntaxResult(self.catch_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("SvelteAwaitCatchClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteAwaitCatchClause> for SyntaxNode {
    fn from(n: SvelteAwaitCatchClause) -> Self {
        n.syntax
    }
}
impl From<SvelteAwaitCatchClause> for SyntaxElement {
    fn from(n: SvelteAwaitCatchClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteAwaitClosingBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_AWAIT_CLOSING_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_AWAIT_CLOSING_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteAwaitClosingBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteAwaitClosingBlock")
                .field(
                    "sv_curly_slash_token",
                    &support::DebugSyntaxResult(self.sv_curly_slash_token()),
                )
                .field(
                    "await_token",
                    &support::DebugSyntaxResult(self.await_token()),
                )
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteAwaitClosingBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteAwaitClosingBlock> for SyntaxNode {
    fn from(n: SvelteAwaitClosingBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteAwaitClosingBlock> for SyntaxElement {
    fn from(n: SvelteAwaitClosingBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteAwaitOpeningBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_AWAIT_OPENING_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_AWAIT_OPENING_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteAwaitOpeningBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteAwaitOpeningBlock")
                .field(
                    "sv_curly_hash_token",
                    &support::DebugSyntaxResult(self.sv_curly_hash_token()),
                )
                .field(
                    "await_token",
                    &support::DebugSyntaxResult(self.await_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "then_clause",
                    &support::DebugOptionalElement(self.then_clause()),
                )
                .field(
                    "catch_clause",
                    &support::DebugOptionalElement(self.catch_clause()),
                )
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .field("children", &self.children())
                .finish()
        } else {
            f.debug_struct("SvelteAwaitOpeningBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteAwaitOpeningBlock> for SyntaxNode {
    fn from(n: SvelteAwaitOpeningBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteAwaitOpeningBlock> for SyntaxElement {
    fn from(n: SvelteAwaitOpeningBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteAwaitThenBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_AWAIT_THEN_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_AWAIT_THEN_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteAwaitThenBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteAwaitThenBlock")
                .field(
                    "sv_curly_colon_token",
                    &support::DebugSyntaxResult(self.sv_curly_colon_token()),
                )
                .field("then_token", &support::DebugSyntaxResult(self.then_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .field("children", &self.children())
                .finish()
        } else {
            f.debug_struct("SvelteAwaitThenBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteAwaitThenBlock> for SyntaxNode {
    fn from(n: SvelteAwaitThenBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteAwaitThenBlock> for SyntaxElement {
    fn from(n: SvelteAwaitThenBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteAwaitThenClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_AWAIT_THEN_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_AWAIT_THEN_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteAwaitThenClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteAwaitThenClause")
                .field("then_token", &support::DebugSyntaxResult(self.then_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("SvelteAwaitThenClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteAwaitThenClause> for SyntaxNode {
    fn from(n: SvelteAwaitThenClause) -> Self {
        n.syntax
    }
}
impl From<SvelteAwaitThenClause> for SyntaxElement {
    fn from(n: SvelteAwaitThenClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteBindDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_BIND_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_BIND_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteBindDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteBindDirective")
                .field("bind_token", &support::DebugSyntaxResult(self.bind_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("SvelteBindDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteBindDirective> for SyntaxNode {
    fn from(n: SvelteBindDirective) -> Self {
        n.syntax
    }
}
impl From<SvelteBindDirective> for SyntaxElement {
    fn from(n: SvelteBindDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteClassDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_CLASS_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_CLASS_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteClassDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteClassDirective")
                .field(
                    "class_token",
                    &support::DebugSyntaxResult(self.class_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("SvelteClassDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteClassDirective> for SyntaxNode {
    fn from(n: SvelteClassDirective) -> Self {
        n.syntax
    }
}
impl From<SvelteClassDirective> for SyntaxElement {
    fn from(n: SvelteClassDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteConstBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_CONST_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_CONST_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteConstBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteConstBlock")
                .field(
                    "sv_curly_at_token",
                    &support::DebugSyntaxResult(self.sv_curly_at_token()),
                )
                .field(
                    "const_token",
                    &support::DebugSyntaxResult(self.const_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteConstBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteConstBlock> for SyntaxNode {
    fn from(n: SvelteConstBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteConstBlock> for SyntaxElement {
    fn from(n: SvelteConstBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteCurlyDestructuredName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_CURLY_DESTRUCTURED_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_CURLY_DESTRUCTURED_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteCurlyDestructuredName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteCurlyDestructuredName")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("names", &self.names())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteCurlyDestructuredName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteCurlyDestructuredName> for SyntaxNode {
    fn from(n: SvelteCurlyDestructuredName) -> Self {
        n.syntax
    }
}
impl From<SvelteCurlyDestructuredName> for SyntaxElement {
    fn from(n: SvelteCurlyDestructuredName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteDebugBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_DEBUG_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_DEBUG_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteDebugBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteDebugBlock")
                .field(
                    "sv_curly_at_token",
                    &support::DebugSyntaxResult(self.sv_curly_at_token()),
                )
                .field(
                    "debug_token",
                    &support::DebugSyntaxResult(self.debug_token()),
                )
                .field("bindings", &self.bindings())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteDebugBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteDebugBlock> for SyntaxNode {
    fn from(n: SvelteDebugBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteDebugBlock> for SyntaxElement {
    fn from(n: SvelteDebugBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteDirectiveModifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_DIRECTIVE_MODIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_DIRECTIVE_MODIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteDirectiveModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteDirectiveModifier")
                .field(
                    "bitwise_or_token",
                    &support::DebugSyntaxResult(self.bitwise_or_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("SvelteDirectiveModifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteDirectiveModifier> for SyntaxNode {
    fn from(n: SvelteDirectiveModifier) -> Self {
        n.syntax
    }
}
impl From<SvelteDirectiveModifier> for SyntaxElement {
    fn from(n: SvelteDirectiveModifier) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteDirectiveValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_DIRECTIVE_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_DIRECTIVE_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteDirectiveValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteDirectiveValue")
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("property", &support::DebugSyntaxResult(self.property()))
                .field("modifiers", &self.modifiers())
                .field(
                    "initializer",
                    &support::DebugOptionalElement(self.initializer()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteDirectiveValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteDirectiveValue> for SyntaxNode {
    fn from(n: SvelteDirectiveValue) -> Self {
        n.syntax
    }
}
impl From<SvelteDirectiveValue> for SyntaxElement {
    fn from(n: SvelteDirectiveValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteEachAsKeyedItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_EACH_AS_KEYED_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_EACH_AS_KEYED_ITEM
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteEachAsKeyedItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteEachAsKeyedItem")
                .field("as_token", &support::DebugSyntaxResult(self.as_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("index", &support::DebugOptionalElement(self.index()))
                .field("key", &support::DebugOptionalElement(self.key()))
                .finish()
        } else {
            f.debug_struct("SvelteEachAsKeyedItem").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteEachAsKeyedItem> for SyntaxNode {
    fn from(n: SvelteEachAsKeyedItem) -> Self {
        n.syntax
    }
}
impl From<SvelteEachAsKeyedItem> for SyntaxElement {
    fn from(n: SvelteEachAsKeyedItem) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteEachBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_EACH_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_EACH_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteEachBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteEachBlock")
                .field(
                    "opening_block",
                    &support::DebugSyntaxResult(self.opening_block()),
                )
                .field("children", &self.children())
                .field(
                    "else_clause",
                    &support::DebugOptionalElement(self.else_clause()),
                )
                .field(
                    "closing_block",
                    &support::DebugSyntaxResult(self.closing_block()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteEachBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteEachBlock> for SyntaxNode {
    fn from(n: SvelteEachBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteEachBlock> for SyntaxElement {
    fn from(n: SvelteEachBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteEachClosingBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_EACH_CLOSING_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_EACH_CLOSING_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteEachClosingBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteEachClosingBlock")
                .field(
                    "sv_curly_slash_token",
                    &support::DebugSyntaxResult(self.sv_curly_slash_token()),
                )
                .field("each_token", &support::DebugSyntaxResult(self.each_token()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteEachClosingBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteEachClosingBlock> for SyntaxNode {
    fn from(n: SvelteEachClosingBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteEachClosingBlock> for SyntaxElement {
    fn from(n: SvelteEachClosingBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteEachIndex {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_EACH_INDEX as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_EACH_INDEX
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteEachIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteEachIndex")
                .field(
                    "comma_token",
                    &support::DebugSyntaxResult(self.comma_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("SvelteEachIndex").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteEachIndex> for SyntaxNode {
    fn from(n: SvelteEachIndex) -> Self {
        n.syntax
    }
}
impl From<SvelteEachIndex> for SyntaxElement {
    fn from(n: SvelteEachIndex) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteEachKey {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_EACH_KEY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_EACH_KEY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteEachKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteEachKey")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteEachKey").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteEachKey> for SyntaxNode {
    fn from(n: SvelteEachKey) -> Self {
        n.syntax
    }
}
impl From<SvelteEachKey> for SyntaxElement {
    fn from(n: SvelteEachKey) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteEachKeyedItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_EACH_KEYED_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_EACH_KEYED_ITEM
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteEachKeyedItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteEachKeyedItem")
                .field("index", &support::DebugOptionalElement(self.index()))
                .finish()
        } else {
            f.debug_struct("SvelteEachKeyedItem").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteEachKeyedItem> for SyntaxNode {
    fn from(n: SvelteEachKeyedItem) -> Self {
        n.syntax
    }
}
impl From<SvelteEachKeyedItem> for SyntaxElement {
    fn from(n: SvelteEachKeyedItem) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteEachOpeningBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_EACH_OPENING_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_EACH_OPENING_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteEachOpeningBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteEachOpeningBlock")
                .field(
                    "sv_curly_hash_token",
                    &support::DebugSyntaxResult(self.sv_curly_hash_token()),
                )
                .field("each_token", &support::DebugSyntaxResult(self.each_token()))
                .field("list", &support::DebugSyntaxResult(self.list()))
                .field("item", &support::DebugOptionalElement(self.item()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteEachOpeningBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteEachOpeningBlock> for SyntaxNode {
    fn from(n: SvelteEachOpeningBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteEachOpeningBlock> for SyntaxElement {
    fn from(n: SvelteEachOpeningBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteElseClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_ELSE_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_ELSE_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteElseClause")
                .field(
                    "sv_curly_colon_token",
                    &support::DebugSyntaxResult(self.sv_curly_colon_token()),
                )
                .field("else_token", &support::DebugSyntaxResult(self.else_token()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .field("children", &self.children())
                .finish()
        } else {
            f.debug_struct("SvelteElseClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteElseClause> for SyntaxNode {
    fn from(n: SvelteElseClause) -> Self {
        n.syntax
    }
}
impl From<SvelteElseClause> for SyntaxElement {
    fn from(n: SvelteElseClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteElseIfClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_ELSE_IF_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_ELSE_IF_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteElseIfClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteElseIfClause")
                .field(
                    "sv_curly_colon_token",
                    &support::DebugSyntaxResult(self.sv_curly_colon_token()),
                )
                .field("else_token", &support::DebugSyntaxResult(self.else_token()))
                .field("if_token", &support::DebugSyntaxResult(self.if_token()))
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .field("children", &self.children())
                .finish()
        } else {
            f.debug_struct("SvelteElseIfClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteElseIfClause> for SyntaxNode {
    fn from(n: SvelteElseIfClause) -> Self {
        n.syntax
    }
}
impl From<SvelteElseIfClause> for SyntaxElement {
    fn from(n: SvelteElseIfClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteHtmlBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_HTML_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_HTML_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteHtmlBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteHtmlBlock")
                .field(
                    "sv_curly_at_token",
                    &support::DebugSyntaxResult(self.sv_curly_at_token()),
                )
                .field("html_token", &support::DebugSyntaxResult(self.html_token()))
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteHtmlBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteHtmlBlock> for SyntaxNode {
    fn from(n: SvelteHtmlBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteHtmlBlock> for SyntaxElement {
    fn from(n: SvelteHtmlBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteIfBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_IF_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_IF_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteIfBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteIfBlock")
                .field(
                    "opening_block",
                    &support::DebugSyntaxResult(self.opening_block()),
                )
                .field("else_if_clauses", &self.else_if_clauses())
                .field(
                    "else_clause",
                    &support::DebugOptionalElement(self.else_clause()),
                )
                .field(
                    "closing_block",
                    &support::DebugSyntaxResult(self.closing_block()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteIfBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteIfBlock> for SyntaxNode {
    fn from(n: SvelteIfBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteIfBlock> for SyntaxElement {
    fn from(n: SvelteIfBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteIfClosingBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_IF_CLOSING_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_IF_CLOSING_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteIfClosingBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteIfClosingBlock")
                .field(
                    "sv_curly_slash_token",
                    &support::DebugSyntaxResult(self.sv_curly_slash_token()),
                )
                .field("if_token", &support::DebugSyntaxResult(self.if_token()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteIfClosingBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteIfClosingBlock> for SyntaxNode {
    fn from(n: SvelteIfClosingBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteIfClosingBlock> for SyntaxElement {
    fn from(n: SvelteIfClosingBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteIfOpeningBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_IF_OPENING_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_IF_OPENING_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteIfOpeningBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteIfOpeningBlock")
                .field(
                    "sv_curly_hash_token",
                    &support::DebugSyntaxResult(self.sv_curly_hash_token()),
                )
                .field("if_token", &support::DebugSyntaxResult(self.if_token()))
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .field("children", &self.children())
                .finish()
        } else {
            f.debug_struct("SvelteIfOpeningBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteIfOpeningBlock> for SyntaxNode {
    fn from(n: SvelteIfOpeningBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteIfOpeningBlock> for SyntaxElement {
    fn from(n: SvelteIfOpeningBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteInDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_IN_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_IN_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteInDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteInDirective")
                .field("in_token", &support::DebugSyntaxResult(self.in_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("SvelteInDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteInDirective> for SyntaxNode {
    fn from(n: SvelteInDirective) -> Self {
        n.syntax
    }
}
impl From<SvelteInDirective> for SyntaxElement {
    fn from(n: SvelteInDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteKeyBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_KEY_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_KEY_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteKeyBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteKeyBlock")
                .field(
                    "opening_block",
                    &support::DebugSyntaxResult(self.opening_block()),
                )
                .field("children", &self.children())
                .field(
                    "closing_block",
                    &support::DebugSyntaxResult(self.closing_block()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteKeyBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteKeyBlock> for SyntaxNode {
    fn from(n: SvelteKeyBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteKeyBlock> for SyntaxElement {
    fn from(n: SvelteKeyBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteKeyClosingBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_KEY_CLOSING_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_KEY_CLOSING_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteKeyClosingBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteKeyClosingBlock")
                .field(
                    "sv_curly_slash_token",
                    &support::DebugSyntaxResult(self.sv_curly_slash_token()),
                )
                .field("key_token", &support::DebugSyntaxResult(self.key_token()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteKeyClosingBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteKeyClosingBlock> for SyntaxNode {
    fn from(n: SvelteKeyClosingBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteKeyClosingBlock> for SyntaxElement {
    fn from(n: SvelteKeyClosingBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteKeyOpeningBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_KEY_OPENING_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_KEY_OPENING_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteKeyOpeningBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteKeyOpeningBlock")
                .field(
                    "sv_curly_hash_token",
                    &support::DebugSyntaxResult(self.sv_curly_hash_token()),
                )
                .field("key_token", &support::DebugSyntaxResult(self.key_token()))
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteKeyOpeningBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteKeyOpeningBlock> for SyntaxNode {
    fn from(n: SvelteKeyOpeningBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteKeyOpeningBlock> for SyntaxElement {
    fn from(n: SvelteKeyOpeningBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_LITERAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteLiteral")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteLiteral").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteLiteral> for SyntaxNode {
    fn from(n: SvelteLiteral) -> Self {
        n.syntax
    }
}
impl From<SvelteLiteral> for SyntaxElement {
    fn from(n: SvelteLiteral) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteName")
                .field(
                    "ident_token",
                    &support::DebugSyntaxResult(self.ident_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteName> for SyntaxNode {
    fn from(n: SvelteName) -> Self {
        n.syntax
    }
}
impl From<SvelteName> for SyntaxElement {
    fn from(n: SvelteName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteOutDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_OUT_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_OUT_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteOutDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteOutDirective")
                .field("out_token", &support::DebugSyntaxResult(self.out_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("SvelteOutDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteOutDirective> for SyntaxNode {
    fn from(n: SvelteOutDirective) -> Self {
        n.syntax
    }
}
impl From<SvelteOutDirective> for SyntaxElement {
    fn from(n: SvelteOutDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteRenderBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_RENDER_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_RENDER_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteRenderBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteRenderBlock")
                .field(
                    "sv_curly_at_token",
                    &support::DebugSyntaxResult(self.sv_curly_at_token()),
                )
                .field(
                    "render_token",
                    &support::DebugSyntaxResult(self.render_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteRenderBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteRenderBlock> for SyntaxNode {
    fn from(n: SvelteRenderBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteRenderBlock> for SyntaxElement {
    fn from(n: SvelteRenderBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteRestBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_REST_BINDING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_REST_BINDING
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteRestBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteRestBinding")
                .field(
                    "dotdotdot_token",
                    &support::DebugSyntaxResult(self.dotdotdot_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("SvelteRestBinding").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteRestBinding> for SyntaxNode {
    fn from(n: SvelteRestBinding) -> Self {
        n.syntax
    }
}
impl From<SvelteRestBinding> for SyntaxElement {
    fn from(n: SvelteRestBinding) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteSnippetBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_SNIPPET_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_SNIPPET_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteSnippetBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteSnippetBlock")
                .field(
                    "opening_block",
                    &support::DebugSyntaxResult(self.opening_block()),
                )
                .field(
                    "closing_block",
                    &support::DebugSyntaxResult(self.closing_block()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteSnippetBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteSnippetBlock> for SyntaxNode {
    fn from(n: SvelteSnippetBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteSnippetBlock> for SyntaxElement {
    fn from(n: SvelteSnippetBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteSnippetClosingBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_SNIPPET_CLOSING_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_SNIPPET_CLOSING_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteSnippetClosingBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteSnippetClosingBlock")
                .field(
                    "sv_curly_slash_token",
                    &support::DebugSyntaxResult(self.sv_curly_slash_token()),
                )
                .field(
                    "snippet_token",
                    &support::DebugSyntaxResult(self.snippet_token()),
                )
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteSnippetClosingBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteSnippetClosingBlock> for SyntaxNode {
    fn from(n: SvelteSnippetClosingBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteSnippetClosingBlock> for SyntaxElement {
    fn from(n: SvelteSnippetClosingBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteSnippetOpeningBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_SNIPPET_OPENING_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_SNIPPET_OPENING_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteSnippetOpeningBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteSnippetOpeningBlock")
                .field(
                    "sv_curly_hash_token",
                    &support::DebugSyntaxResult(self.sv_curly_hash_token()),
                )
                .field(
                    "snippet_token",
                    &support::DebugSyntaxResult(self.snippet_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .field("children", &self.children())
                .finish()
        } else {
            f.debug_struct("SvelteSnippetOpeningBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteSnippetOpeningBlock> for SyntaxNode {
    fn from(n: SvelteSnippetOpeningBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteSnippetOpeningBlock> for SyntaxElement {
    fn from(n: SvelteSnippetOpeningBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteSquareDestructuredName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_SQUARE_DESTRUCTURED_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_SQUARE_DESTRUCTURED_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteSquareDestructuredName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteSquareDestructuredName")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("names", &self.names())
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteSquareDestructuredName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteSquareDestructuredName> for SyntaxNode {
    fn from(n: SvelteSquareDestructuredName) -> Self {
        n.syntax
    }
}
impl From<SvelteSquareDestructuredName> for SyntaxElement {
    fn from(n: SvelteSquareDestructuredName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteStyleDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_STYLE_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_STYLE_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteStyleDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteStyleDirective")
                .field(
                    "style_token",
                    &support::DebugSyntaxResult(self.style_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("SvelteStyleDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteStyleDirective> for SyntaxNode {
    fn from(n: SvelteStyleDirective) -> Self {
        n.syntax
    }
}
impl From<SvelteStyleDirective> for SyntaxElement {
    fn from(n: SvelteStyleDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteTransitionDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_TRANSITION_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_TRANSITION_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteTransitionDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteTransitionDirective")
                .field(
                    "transition_token",
                    &support::DebugSyntaxResult(self.transition_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("SvelteTransitionDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteTransitionDirective> for SyntaxNode {
    fn from(n: SvelteTransitionDirective) -> Self {
        n.syntax
    }
}
impl From<SvelteTransitionDirective> for SyntaxElement {
    fn from(n: SvelteTransitionDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SvelteUseDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_USE_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_USE_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteUseDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteUseDirective")
                .field("use_token", &support::DebugSyntaxResult(self.use_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("SvelteUseDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteUseDirective> for SyntaxNode {
    fn from(n: SvelteUseDirective) -> Self {
        n.syntax
    }
}
impl From<SvelteUseDirective> for SyntaxElement {
    fn from(n: SvelteUseDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for VueDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for VueDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("VueDirective")
                .field("name_token", &support::DebugSyntaxResult(self.name_token()))
                .field("arg", &support::DebugOptionalElement(self.arg()))
                .field("modifiers", &self.modifiers())
                .field(
                    "initializer",
                    &support::DebugOptionalElement(self.initializer()),
                )
                .finish()
        } else {
            f.debug_struct("VueDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<VueDirective> for SyntaxNode {
    fn from(n: VueDirective) -> Self {
        n.syntax
    }
}
impl From<VueDirective> for SyntaxElement {
    fn from(n: VueDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for VueDirectiveArgument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_DIRECTIVE_ARGUMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_DIRECTIVE_ARGUMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for VueDirectiveArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("VueDirectiveArgument")
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("arg", &support::DebugSyntaxResult(self.arg()))
                .finish()
        } else {
            f.debug_struct("VueDirectiveArgument").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<VueDirectiveArgument> for SyntaxNode {
    fn from(n: VueDirectiveArgument) -> Self {
        n.syntax
    }
}
impl From<VueDirectiveArgument> for SyntaxElement {
    fn from(n: VueDirectiveArgument) -> Self {
        n.syntax.into()
    }
}
impl AstNode for VueDynamicArgument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_DYNAMIC_ARGUMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_DYNAMIC_ARGUMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for VueDynamicArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("VueDynamicArgument")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("name_token", &support::DebugSyntaxResult(self.name_token()))
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("VueDynamicArgument").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<VueDynamicArgument> for SyntaxNode {
    fn from(n: VueDynamicArgument) -> Self {
        n.syntax
    }
}
impl From<VueDynamicArgument> for SyntaxElement {
    fn from(n: VueDynamicArgument) -> Self {
        n.syntax.into()
    }
}
impl AstNode for VueModifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_MODIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_MODIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for VueModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("VueModifier")
                .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
                .field(
                    "modifier_token",
                    &support::DebugSyntaxResult(self.modifier_token()),
                )
                .finish()
        } else {
            f.debug_struct("VueModifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<VueModifier> for SyntaxNode {
    fn from(n: VueModifier) -> Self {
        n.syntax
    }
}
impl From<VueModifier> for SyntaxElement {
    fn from(n: VueModifier) -> Self {
        n.syntax.into()
    }
}
impl AstNode for VueStaticArgument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_STATIC_ARGUMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_STATIC_ARGUMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for VueStaticArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("VueStaticArgument")
                .field("name_token", &support::DebugSyntaxResult(self.name_token()))
                .finish()
        } else {
            f.debug_struct("VueStaticArgument").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<VueStaticArgument> for SyntaxNode {
    fn from(n: VueStaticArgument) -> Self {
        n.syntax
    }
}
impl From<VueStaticArgument> for SyntaxElement {
    fn from(n: VueStaticArgument) -> Self {
        n.syntax.into()
    }
}
impl AstNode for VueVBindShorthandDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_V_BIND_SHORTHAND_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_V_BIND_SHORTHAND_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for VueVBindShorthandDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("VueVBindShorthandDirective")
                .field("arg", &support::DebugSyntaxResult(self.arg()))
                .field("modifiers", &self.modifiers())
                .field(
                    "initializer",
                    &support::DebugOptionalElement(self.initializer()),
                )
                .finish()
        } else {
            f.debug_struct("VueVBindShorthandDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<VueVBindShorthandDirective> for SyntaxNode {
    fn from(n: VueVBindShorthandDirective) -> Self {
        n.syntax
    }
}
impl From<VueVBindShorthandDirective> for SyntaxElement {
    fn from(n: VueVBindShorthandDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for VueVOnShorthandDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_V_ON_SHORTHAND_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_V_ON_SHORTHAND_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for VueVOnShorthandDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("VueVOnShorthandDirective")
                .field("at_token", &support::DebugSyntaxResult(self.at_token()))
                .field("arg", &support::DebugSyntaxResult(self.arg()))
                .field("modifiers", &self.modifiers())
                .field(
                    "initializer",
                    &support::DebugOptionalElement(self.initializer()),
                )
                .finish()
        } else {
            f.debug_struct("VueVOnShorthandDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<VueVOnShorthandDirective> for SyntaxNode {
    fn from(n: VueVOnShorthandDirective) -> Self {
        n.syntax
    }
}
impl From<VueVOnShorthandDirective> for SyntaxElement {
    fn from(n: VueVOnShorthandDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for VueVSlotShorthandDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_V_SLOT_SHORTHAND_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_V_SLOT_SHORTHAND_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for VueVSlotShorthandDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("VueVSlotShorthandDirective")
                .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
                .field("arg", &support::DebugSyntaxResult(self.arg()))
                .field("modifiers", &self.modifiers())
                .field(
                    "initializer",
                    &support::DebugOptionalElement(self.initializer()),
                )
                .finish()
        } else {
            f.debug_struct("VueVSlotShorthandDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<VueVSlotShorthandDirective> for SyntaxNode {
    fn from(n: VueVSlotShorthandDirective) -> Self {
        n.syntax
    }
}
impl From<VueVSlotShorthandDirective> for SyntaxElement {
    fn from(n: VueVSlotShorthandDirective) -> Self {
        n.syntax.into()
    }
}
impl From<AstroClassDirective> for AnyAstroDirective {
    fn from(node: AstroClassDirective) -> Self {
        Self::AstroClassDirective(node)
    }
}
impl From<AstroClientDirective> for AnyAstroDirective {
    fn from(node: AstroClientDirective) -> Self {
        Self::AstroClientDirective(node)
    }
}
impl From<AstroDefineDirective> for AnyAstroDirective {
    fn from(node: AstroDefineDirective) -> Self {
        Self::AstroDefineDirective(node)
    }
}
impl From<AstroIsDirective> for AnyAstroDirective {
    fn from(node: AstroIsDirective) -> Self {
        Self::AstroIsDirective(node)
    }
}
impl From<AstroServerDirective> for AnyAstroDirective {
    fn from(node: AstroServerDirective) -> Self {
        Self::AstroServerDirective(node)
    }
}
impl From<AstroSetDirective> for AnyAstroDirective {
    fn from(node: AstroSetDirective) -> Self {
        Self::AstroSetDirective(node)
    }
}
impl AstNode for AnyAstroDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AstroClassDirective::KIND_SET
        .union(AstroClientDirective::KIND_SET)
        .union(AstroDefineDirective::KIND_SET)
        .union(AstroIsDirective::KIND_SET)
        .union(AstroServerDirective::KIND_SET)
        .union(AstroSetDirective::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            ASTRO_CLASS_DIRECTIVE
                | ASTRO_CLIENT_DIRECTIVE
                | ASTRO_DEFINE_DIRECTIVE
                | ASTRO_IS_DIRECTIVE
                | ASTRO_SERVER_DIRECTIVE
                | ASTRO_SET_DIRECTIVE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            ASTRO_CLASS_DIRECTIVE => Self::AstroClassDirective(AstroClassDirective { syntax }),
            ASTRO_CLIENT_DIRECTIVE => Self::AstroClientDirective(AstroClientDirective { syntax }),
            ASTRO_DEFINE_DIRECTIVE => Self::AstroDefineDirective(AstroDefineDirective { syntax }),
            ASTRO_IS_DIRECTIVE => Self::AstroIsDirective(AstroIsDirective { syntax }),
            ASTRO_SERVER_DIRECTIVE => Self::AstroServerDirective(AstroServerDirective { syntax }),
            ASTRO_SET_DIRECTIVE => Self::AstroSetDirective(AstroSetDirective { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::AstroClassDirective(it) => it.syntax(),
            Self::AstroClientDirective(it) => it.syntax(),
            Self::AstroDefineDirective(it) => it.syntax(),
            Self::AstroIsDirective(it) => it.syntax(),
            Self::AstroServerDirective(it) => it.syntax(),
            Self::AstroSetDirective(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::AstroClassDirective(it) => it.into_syntax(),
            Self::AstroClientDirective(it) => it.into_syntax(),
            Self::AstroDefineDirective(it) => it.into_syntax(),
            Self::AstroIsDirective(it) => it.into_syntax(),
            Self::AstroServerDirective(it) => it.into_syntax(),
            Self::AstroSetDirective(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyAstroDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AstroClassDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::AstroClientDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::AstroDefineDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::AstroIsDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::AstroServerDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::AstroSetDirective(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyAstroDirective> for SyntaxNode {
    fn from(n: AnyAstroDirective) -> Self {
        match n {
            AnyAstroDirective::AstroClassDirective(it) => it.into_syntax(),
            AnyAstroDirective::AstroClientDirective(it) => it.into_syntax(),
            AnyAstroDirective::AstroDefineDirective(it) => it.into_syntax(),
            AnyAstroDirective::AstroIsDirective(it) => it.into_syntax(),
            AnyAstroDirective::AstroServerDirective(it) => it.into_syntax(),
            AnyAstroDirective::AstroSetDirective(it) => it.into_syntax(),
        }
    }
}
impl From<AnyAstroDirective> for SyntaxElement {
    fn from(n: AnyAstroDirective) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<AstroBogusFrontmatter> for AnyAstroFrontmatterElement {
    fn from(node: AstroBogusFrontmatter) -> Self {
        Self::AstroBogusFrontmatter(node)
    }
}
impl From<AstroFrontmatterElement> for AnyAstroFrontmatterElement {
    fn from(node: AstroFrontmatterElement) -> Self {
        Self::AstroFrontmatterElement(node)
    }
}
impl AstNode for AnyAstroFrontmatterElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AstroBogusFrontmatter::KIND_SET.union(AstroFrontmatterElement::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, ASTRO_BOGUS_FRONTMATTER | ASTRO_FRONTMATTER_ELEMENT)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            ASTRO_BOGUS_FRONTMATTER => {
                Self::AstroBogusFrontmatter(AstroBogusFrontmatter { syntax })
            }
            ASTRO_FRONTMATTER_ELEMENT => {
                Self::AstroFrontmatterElement(AstroFrontmatterElement { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::AstroBogusFrontmatter(it) => it.syntax(),
            Self::AstroFrontmatterElement(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::AstroBogusFrontmatter(it) => it.into_syntax(),
            Self::AstroFrontmatterElement(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyAstroFrontmatterElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AstroBogusFrontmatter(it) => std::fmt::Debug::fmt(it, f),
            Self::AstroFrontmatterElement(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyAstroFrontmatterElement> for SyntaxNode {
    fn from(n: AnyAstroFrontmatterElement) -> Self {
        match n {
            AnyAstroFrontmatterElement::AstroBogusFrontmatter(it) => it.into_syntax(),
            AnyAstroFrontmatterElement::AstroFrontmatterElement(it) => it.into_syntax(),
        }
    }
}
impl From<AnyAstroFrontmatterElement> for SyntaxElement {
    fn from(n: AnyAstroFrontmatterElement) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<HtmlAttribute> for AnyHtmlAttribute {
    fn from(node: HtmlAttribute) -> Self {
        Self::HtmlAttribute(node)
    }
}
impl From<HtmlAttributeDoubleTextExpression> for AnyHtmlAttribute {
    fn from(node: HtmlAttributeDoubleTextExpression) -> Self {
        Self::HtmlAttributeDoubleTextExpression(node)
    }
}
impl From<HtmlAttributeSingleTextExpression> for AnyHtmlAttribute {
    fn from(node: HtmlAttributeSingleTextExpression) -> Self {
        Self::HtmlAttributeSingleTextExpression(node)
    }
}
impl From<HtmlBogusAttribute> for AnyHtmlAttribute {
    fn from(node: HtmlBogusAttribute) -> Self {
        Self::HtmlBogusAttribute(node)
    }
}
impl From<HtmlSpreadAttribute> for AnyHtmlAttribute {
    fn from(node: HtmlSpreadAttribute) -> Self {
        Self::HtmlSpreadAttribute(node)
    }
}
impl From<SvelteAttachAttribute> for AnyHtmlAttribute {
    fn from(node: SvelteAttachAttribute) -> Self {
        Self::SvelteAttachAttribute(node)
    }
}
impl AstNode for AnyHtmlAttribute {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyAstroDirective::KIND_SET
        .union(AnySvelteDirective::KIND_SET)
        .union(AnyVueDirective::KIND_SET)
        .union(HtmlAttribute::KIND_SET)
        .union(HtmlAttributeDoubleTextExpression::KIND_SET)
        .union(HtmlAttributeSingleTextExpression::KIND_SET)
        .union(HtmlBogusAttribute::KIND_SET)
        .union(HtmlSpreadAttribute::KIND_SET)
        .union(SvelteAttachAttribute::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            HTML_ATTRIBUTE
            | HTML_ATTRIBUTE_DOUBLE_TEXT_EXPRESSION
            | HTML_ATTRIBUTE_SINGLE_TEXT_EXPRESSION
            | HTML_BOGUS_ATTRIBUTE
            | HTML_SPREAD_ATTRIBUTE
            | SVELTE_ATTACH_ATTRIBUTE => true,
            k if AnyAstroDirective::can_cast(k) => true,
            k if AnySvelteDirective::can_cast(k) => true,
            k if AnyVueDirective::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            HTML_ATTRIBUTE => Self::HtmlAttribute(HtmlAttribute { syntax }),
            HTML_ATTRIBUTE_DOUBLE_TEXT_EXPRESSION => {
                Self::HtmlAttributeDoubleTextExpression(HtmlAttributeDoubleTextExpression {
                    syntax,
                })
            }
            HTML_ATTRIBUTE_SINGLE_TEXT_EXPRESSION => {
                Self::HtmlAttributeSingleTextExpression(HtmlAttributeSingleTextExpression {
                    syntax,
                })
            }
            HTML_BOGUS_ATTRIBUTE => Self::HtmlBogusAttribute(HtmlBogusAttribute { syntax }),
            HTML_SPREAD_ATTRIBUTE => Self::HtmlSpreadAttribute(HtmlSpreadAttribute { syntax }),
            SVELTE_ATTACH_ATTRIBUTE => {
                Self::SvelteAttachAttribute(SvelteAttachAttribute { syntax })
            }
            _ => {
                let syntax = match AnyAstroDirective::try_cast(syntax) {
                    Ok(any_astro_directive) => {
                        return Some(Self::AnyAstroDirective(any_astro_directive));
                    }
                    Err(syntax) => syntax,
                };
                let syntax = match AnySvelteDirective::try_cast(syntax) {
                    Ok(any_svelte_directive) => {
                        return Some(Self::AnySvelteDirective(any_svelte_directive));
                    }
                    Err(syntax) => syntax,
                };
                if let Some(any_vue_directive) = AnyVueDirective::cast(syntax) {
                    return Some(Self::AnyVueDirective(any_vue_directive));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::HtmlAttribute(it) => it.syntax(),
            Self::HtmlAttributeDoubleTextExpression(it) => it.syntax(),
            Self::HtmlAttributeSingleTextExpression(it) => it.syntax(),
            Self::HtmlBogusAttribute(it) => it.syntax(),
            Self::HtmlSpreadAttribute(it) => it.syntax(),
            Self::SvelteAttachAttribute(it) => it.syntax(),
            Self::AnyAstroDirective(it) => it.syntax(),
            Self::AnySvelteDirective(it) => it.syntax(),
            Self::AnyVueDirective(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::HtmlAttribute(it) => it.into_syntax(),
            Self::HtmlAttributeDoubleTextExpression(it) => it.into_syntax(),
            Self::HtmlAttributeSingleTextExpression(it) => it.into_syntax(),
            Self::HtmlBogusAttribute(it) => it.into_syntax(),
            Self::HtmlSpreadAttribute(it) => it.into_syntax(),
            Self::SvelteAttachAttribute(it) => it.into_syntax(),
            Self::AnyAstroDirective(it) => it.into_syntax(),
            Self::AnySvelteDirective(it) => it.into_syntax(),
            Self::AnyVueDirective(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyHtmlAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyAstroDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::AnySvelteDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::AnyVueDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlAttribute(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlAttributeDoubleTextExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlAttributeSingleTextExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlBogusAttribute(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlSpreadAttribute(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteAttachAttribute(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyHtmlAttribute> for SyntaxNode {
    fn from(n: AnyHtmlAttribute) -> Self {
        match n {
            AnyHtmlAttribute::AnyAstroDirective(it) => it.into_syntax(),
            AnyHtmlAttribute::AnySvelteDirective(it) => it.into_syntax(),
            AnyHtmlAttribute::AnyVueDirective(it) => it.into_syntax(),
            AnyHtmlAttribute::HtmlAttribute(it) => it.into_syntax(),
            AnyHtmlAttribute::HtmlAttributeDoubleTextExpression(it) => it.into_syntax(),
            AnyHtmlAttribute::HtmlAttributeSingleTextExpression(it) => it.into_syntax(),
            AnyHtmlAttribute::HtmlBogusAttribute(it) => it.into_syntax(),
            AnyHtmlAttribute::HtmlSpreadAttribute(it) => it.into_syntax(),
            AnyHtmlAttribute::SvelteAttachAttribute(it) => it.into_syntax(),
        }
    }
}
impl From<AnyHtmlAttribute> for SyntaxElement {
    fn from(n: AnyHtmlAttribute) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<HtmlAttributeSingleTextExpression> for AnyHtmlAttributeInitializer {
    fn from(node: HtmlAttributeSingleTextExpression) -> Self {
        Self::HtmlAttributeSingleTextExpression(node)
    }
}
impl From<HtmlString> for AnyHtmlAttributeInitializer {
    fn from(node: HtmlString) -> Self {
        Self::HtmlString(node)
    }
}
impl AstNode for AnyHtmlAttributeInitializer {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        HtmlAttributeSingleTextExpression::KIND_SET.union(HtmlString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, HTML_ATTRIBUTE_SINGLE_TEXT_EXPRESSION | HTML_STRING)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            HTML_ATTRIBUTE_SINGLE_TEXT_EXPRESSION => {
                Self::HtmlAttributeSingleTextExpression(HtmlAttributeSingleTextExpression {
                    syntax,
                })
            }
            HTML_STRING => Self::HtmlString(HtmlString { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::HtmlAttributeSingleTextExpression(it) => it.syntax(),
            Self::HtmlString(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::HtmlAttributeSingleTextExpression(it) => it.into_syntax(),
            Self::HtmlString(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyHtmlAttributeInitializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HtmlAttributeSingleTextExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyHtmlAttributeInitializer> for SyntaxNode {
    fn from(n: AnyHtmlAttributeInitializer) -> Self {
        match n {
            AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(it) => it.into_syntax(),
            AnyHtmlAttributeInitializer::HtmlString(it) => it.into_syntax(),
        }
    }
}
impl From<AnyHtmlAttributeInitializer> for SyntaxElement {
    fn from(n: AnyHtmlAttributeInitializer) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<HtmlComponentName> for AnyHtmlComponentObjectName {
    fn from(node: HtmlComponentName) -> Self {
        Self::HtmlComponentName(node)
    }
}
impl From<HtmlMemberName> for AnyHtmlComponentObjectName {
    fn from(node: HtmlMemberName) -> Self {
        Self::HtmlMemberName(node)
    }
}
impl From<HtmlTagName> for AnyHtmlComponentObjectName {
    fn from(node: HtmlTagName) -> Self {
        Self::HtmlTagName(node)
    }
}
impl AstNode for AnyHtmlComponentObjectName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = HtmlComponentName::KIND_SET
        .union(HtmlMemberName::KIND_SET)
        .union(HtmlTagName::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, HTML_COMPONENT_NAME | HTML_MEMBER_NAME | HTML_TAG_NAME)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            HTML_COMPONENT_NAME => Self::HtmlComponentName(HtmlComponentName { syntax }),
            HTML_MEMBER_NAME => Self::HtmlMemberName(HtmlMemberName { syntax }),
            HTML_TAG_NAME => Self::HtmlTagName(HtmlTagName { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::HtmlComponentName(it) => it.syntax(),
            Self::HtmlMemberName(it) => it.syntax(),
            Self::HtmlTagName(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::HtmlComponentName(it) => it.into_syntax(),
            Self::HtmlMemberName(it) => it.into_syntax(),
            Self::HtmlTagName(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyHtmlComponentObjectName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HtmlComponentName(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlMemberName(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlTagName(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyHtmlComponentObjectName> for SyntaxNode {
    fn from(n: AnyHtmlComponentObjectName) -> Self {
        match n {
            AnyHtmlComponentObjectName::HtmlComponentName(it) => it.into_syntax(),
            AnyHtmlComponentObjectName::HtmlMemberName(it) => it.into_syntax(),
            AnyHtmlComponentObjectName::HtmlTagName(it) => it.into_syntax(),
        }
    }
}
impl From<AnyHtmlComponentObjectName> for SyntaxElement {
    fn from(n: AnyHtmlComponentObjectName) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<HtmlContent> for AnyHtmlContent {
    fn from(node: HtmlContent) -> Self {
        Self::HtmlContent(node)
    }
}
impl From<HtmlEmbeddedContent> for AnyHtmlContent {
    fn from(node: HtmlEmbeddedContent) -> Self {
        Self::HtmlEmbeddedContent(node)
    }
}
impl AstNode for AnyHtmlContent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyHtmlTextExpression::KIND_SET
        .union(HtmlContent::KIND_SET)
        .union(HtmlEmbeddedContent::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            HTML_CONTENT | HTML_EMBEDDED_CONTENT => true,
            k if AnyHtmlTextExpression::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            HTML_CONTENT => Self::HtmlContent(HtmlContent { syntax }),
            HTML_EMBEDDED_CONTENT => Self::HtmlEmbeddedContent(HtmlEmbeddedContent { syntax }),
            _ => {
                if let Some(any_html_text_expression) = AnyHtmlTextExpression::cast(syntax) {
                    return Some(Self::AnyHtmlTextExpression(any_html_text_expression));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::HtmlContent(it) => it.syntax(),
            Self::HtmlEmbeddedContent(it) => it.syntax(),
            Self::AnyHtmlTextExpression(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::HtmlContent(it) => it.into_syntax(),
            Self::HtmlEmbeddedContent(it) => it.into_syntax(),
            Self::AnyHtmlTextExpression(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyHtmlContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyHtmlTextExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlContent(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlEmbeddedContent(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyHtmlContent> for SyntaxNode {
    fn from(n: AnyHtmlContent) -> Self {
        match n {
            AnyHtmlContent::AnyHtmlTextExpression(it) => it.into_syntax(),
            AnyHtmlContent::HtmlContent(it) => it.into_syntax(),
            AnyHtmlContent::HtmlEmbeddedContent(it) => it.into_syntax(),
        }
    }
}
impl From<AnyHtmlContent> for SyntaxElement {
    fn from(n: AnyHtmlContent) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<HtmlBogusElement> for AnyHtmlElement {
    fn from(node: HtmlBogusElement) -> Self {
        Self::HtmlBogusElement(node)
    }
}
impl From<HtmlCdataSection> for AnyHtmlElement {
    fn from(node: HtmlCdataSection) -> Self {
        Self::HtmlCdataSection(node)
    }
}
impl From<HtmlElement> for AnyHtmlElement {
    fn from(node: HtmlElement) -> Self {
        Self::HtmlElement(node)
    }
}
impl From<HtmlSelfClosingElement> for AnyHtmlElement {
    fn from(node: HtmlSelfClosingElement) -> Self {
        Self::HtmlSelfClosingElement(node)
    }
}
impl AstNode for AnyHtmlElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyHtmlContent::KIND_SET
        .union(HtmlBogusElement::KIND_SET)
        .union(HtmlCdataSection::KIND_SET)
        .union(HtmlElement::KIND_SET)
        .union(HtmlSelfClosingElement::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            HTML_BOGUS_ELEMENT | HTML_CDATA_SECTION | HTML_ELEMENT | HTML_SELF_CLOSING_ELEMENT => {
                true
            }
            k if AnyHtmlContent::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            HTML_BOGUS_ELEMENT => Self::HtmlBogusElement(HtmlBogusElement { syntax }),
            HTML_CDATA_SECTION => Self::HtmlCdataSection(HtmlCdataSection { syntax }),
            HTML_ELEMENT => Self::HtmlElement(HtmlElement { syntax }),
            HTML_SELF_CLOSING_ELEMENT => {
                Self::HtmlSelfClosingElement(HtmlSelfClosingElement { syntax })
            }
            _ => {
                if let Some(any_html_content) = AnyHtmlContent::cast(syntax) {
                    return Some(Self::AnyHtmlContent(any_html_content));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::HtmlBogusElement(it) => it.syntax(),
            Self::HtmlCdataSection(it) => it.syntax(),
            Self::HtmlElement(it) => it.syntax(),
            Self::HtmlSelfClosingElement(it) => it.syntax(),
            Self::AnyHtmlContent(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::HtmlBogusElement(it) => it.into_syntax(),
            Self::HtmlCdataSection(it) => it.into_syntax(),
            Self::HtmlElement(it) => it.into_syntax(),
            Self::HtmlSelfClosingElement(it) => it.into_syntax(),
            Self::AnyHtmlContent(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyHtmlElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyHtmlContent(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlBogusElement(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlCdataSection(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlElement(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlSelfClosingElement(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyHtmlElement> for SyntaxNode {
    fn from(n: AnyHtmlElement) -> Self {
        match n {
            AnyHtmlElement::AnyHtmlContent(it) => it.into_syntax(),
            AnyHtmlElement::HtmlBogusElement(it) => it.into_syntax(),
            AnyHtmlElement::HtmlCdataSection(it) => it.into_syntax(),
            AnyHtmlElement::HtmlElement(it) => it.into_syntax(),
            AnyHtmlElement::HtmlSelfClosingElement(it) => it.into_syntax(),
        }
    }
}
impl From<AnyHtmlElement> for SyntaxElement {
    fn from(n: AnyHtmlElement) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<HtmlComponentName> for AnyHtmlTagName {
    fn from(node: HtmlComponentName) -> Self {
        Self::HtmlComponentName(node)
    }
}
impl From<HtmlMemberName> for AnyHtmlTagName {
    fn from(node: HtmlMemberName) -> Self {
        Self::HtmlMemberName(node)
    }
}
impl From<HtmlTagName> for AnyHtmlTagName {
    fn from(node: HtmlTagName) -> Self {
        Self::HtmlTagName(node)
    }
}
impl AstNode for AnyHtmlTagName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = HtmlComponentName::KIND_SET
        .union(HtmlMemberName::KIND_SET)
        .union(HtmlTagName::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, HTML_COMPONENT_NAME | HTML_MEMBER_NAME | HTML_TAG_NAME)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            HTML_COMPONENT_NAME => Self::HtmlComponentName(HtmlComponentName { syntax }),
            HTML_MEMBER_NAME => Self::HtmlMemberName(HtmlMemberName { syntax }),
            HTML_TAG_NAME => Self::HtmlTagName(HtmlTagName { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::HtmlComponentName(it) => it.syntax(),
            Self::HtmlMemberName(it) => it.syntax(),
            Self::HtmlTagName(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::HtmlComponentName(it) => it.into_syntax(),
            Self::HtmlMemberName(it) => it.into_syntax(),
            Self::HtmlTagName(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyHtmlTagName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HtmlComponentName(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlMemberName(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlTagName(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyHtmlTagName> for SyntaxNode {
    fn from(n: AnyHtmlTagName) -> Self {
        match n {
            AnyHtmlTagName::HtmlComponentName(it) => it.into_syntax(),
            AnyHtmlTagName::HtmlMemberName(it) => it.into_syntax(),
            AnyHtmlTagName::HtmlTagName(it) => it.into_syntax(),
        }
    }
}
impl From<AnyHtmlTagName> for SyntaxElement {
    fn from(n: AnyHtmlTagName) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<HtmlBogusTextExpression> for AnyHtmlTextExpression {
    fn from(node: HtmlBogusTextExpression) -> Self {
        Self::HtmlBogusTextExpression(node)
    }
}
impl From<HtmlDoubleTextExpression> for AnyHtmlTextExpression {
    fn from(node: HtmlDoubleTextExpression) -> Self {
        Self::HtmlDoubleTextExpression(node)
    }
}
impl From<HtmlSingleTextExpression> for AnyHtmlTextExpression {
    fn from(node: HtmlSingleTextExpression) -> Self {
        Self::HtmlSingleTextExpression(node)
    }
}
impl AstNode for AnyHtmlTextExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnySvelteBlock::KIND_SET
        .union(HtmlBogusTextExpression::KIND_SET)
        .union(HtmlDoubleTextExpression::KIND_SET)
        .union(HtmlSingleTextExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            HTML_BOGUS_TEXT_EXPRESSION
            | HTML_DOUBLE_TEXT_EXPRESSION
            | HTML_SINGLE_TEXT_EXPRESSION => true,
            k if AnySvelteBlock::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            HTML_BOGUS_TEXT_EXPRESSION => {
                Self::HtmlBogusTextExpression(HtmlBogusTextExpression { syntax })
            }
            HTML_DOUBLE_TEXT_EXPRESSION => {
                Self::HtmlDoubleTextExpression(HtmlDoubleTextExpression { syntax })
            }
            HTML_SINGLE_TEXT_EXPRESSION => {
                Self::HtmlSingleTextExpression(HtmlSingleTextExpression { syntax })
            }
            _ => {
                if let Some(any_svelte_block) = AnySvelteBlock::cast(syntax) {
                    return Some(Self::AnySvelteBlock(any_svelte_block));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::HtmlBogusTextExpression(it) => it.syntax(),
            Self::HtmlDoubleTextExpression(it) => it.syntax(),
            Self::HtmlSingleTextExpression(it) => it.syntax(),
            Self::AnySvelteBlock(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::HtmlBogusTextExpression(it) => it.into_syntax(),
            Self::HtmlDoubleTextExpression(it) => it.into_syntax(),
            Self::HtmlSingleTextExpression(it) => it.into_syntax(),
            Self::AnySvelteBlock(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyHtmlTextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnySvelteBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlBogusTextExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlDoubleTextExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlSingleTextExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyHtmlTextExpression> for SyntaxNode {
    fn from(n: AnyHtmlTextExpression) -> Self {
        match n {
            AnyHtmlTextExpression::AnySvelteBlock(it) => it.into_syntax(),
            AnyHtmlTextExpression::HtmlBogusTextExpression(it) => it.into_syntax(),
            AnyHtmlTextExpression::HtmlDoubleTextExpression(it) => it.into_syntax(),
            AnyHtmlTextExpression::HtmlSingleTextExpression(it) => it.into_syntax(),
        }
    }
}
impl From<AnyHtmlTextExpression> for SyntaxElement {
    fn from(n: AnyHtmlTextExpression) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SvelteAwaitCatchBlock> for AnySvelteAwaitClauses {
    fn from(node: SvelteAwaitCatchBlock) -> Self {
        Self::SvelteAwaitCatchBlock(node)
    }
}
impl From<SvelteAwaitThenBlock> for AnySvelteAwaitClauses {
    fn from(node: SvelteAwaitThenBlock) -> Self {
        Self::SvelteAwaitThenBlock(node)
    }
}
impl From<SvelteBogusBlock> for AnySvelteAwaitClauses {
    fn from(node: SvelteBogusBlock) -> Self {
        Self::SvelteBogusBlock(node)
    }
}
impl AstNode for AnySvelteAwaitClauses {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SvelteAwaitCatchBlock::KIND_SET
        .union(SvelteAwaitThenBlock::KIND_SET)
        .union(SvelteBogusBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SVELTE_AWAIT_CATCH_BLOCK | SVELTE_AWAIT_THEN_BLOCK | SVELTE_BOGUS_BLOCK
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SVELTE_AWAIT_CATCH_BLOCK => {
                Self::SvelteAwaitCatchBlock(SvelteAwaitCatchBlock { syntax })
            }
            SVELTE_AWAIT_THEN_BLOCK => Self::SvelteAwaitThenBlock(SvelteAwaitThenBlock { syntax }),
            SVELTE_BOGUS_BLOCK => Self::SvelteBogusBlock(SvelteBogusBlock { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SvelteAwaitCatchBlock(it) => it.syntax(),
            Self::SvelteAwaitThenBlock(it) => it.syntax(),
            Self::SvelteBogusBlock(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SvelteAwaitCatchBlock(it) => it.into_syntax(),
            Self::SvelteAwaitThenBlock(it) => it.into_syntax(),
            Self::SvelteBogusBlock(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnySvelteAwaitClauses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SvelteAwaitCatchBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteAwaitThenBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteBogusBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySvelteAwaitClauses> for SyntaxNode {
    fn from(n: AnySvelteAwaitClauses) -> Self {
        match n {
            AnySvelteAwaitClauses::SvelteAwaitCatchBlock(it) => it.into_syntax(),
            AnySvelteAwaitClauses::SvelteAwaitThenBlock(it) => it.into_syntax(),
            AnySvelteAwaitClauses::SvelteBogusBlock(it) => it.into_syntax(),
        }
    }
}
impl From<AnySvelteAwaitClauses> for SyntaxElement {
    fn from(n: AnySvelteAwaitClauses) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SvelteName> for AnySvelteBindingAssignmentBinding {
    fn from(node: SvelteName) -> Self {
        Self::SvelteName(node)
    }
}
impl From<SvelteRestBinding> for AnySvelteBindingAssignmentBinding {
    fn from(node: SvelteRestBinding) -> Self {
        Self::SvelteRestBinding(node)
    }
}
impl AstNode for AnySvelteBindingAssignmentBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SvelteName::KIND_SET.union(SvelteRestBinding::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SVELTE_NAME | SVELTE_REST_BINDING)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SVELTE_NAME => Self::SvelteName(SvelteName { syntax }),
            SVELTE_REST_BINDING => Self::SvelteRestBinding(SvelteRestBinding { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SvelteName(it) => it.syntax(),
            Self::SvelteRestBinding(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SvelteName(it) => it.into_syntax(),
            Self::SvelteRestBinding(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnySvelteBindingAssignmentBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SvelteName(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteRestBinding(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySvelteBindingAssignmentBinding> for SyntaxNode {
    fn from(n: AnySvelteBindingAssignmentBinding) -> Self {
        match n {
            AnySvelteBindingAssignmentBinding::SvelteName(it) => it.into_syntax(),
            AnySvelteBindingAssignmentBinding::SvelteRestBinding(it) => it.into_syntax(),
        }
    }
}
impl From<AnySvelteBindingAssignmentBinding> for SyntaxElement {
    fn from(n: AnySvelteBindingAssignmentBinding) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SvelteLiteral> for AnySvelteBindingProperty {
    fn from(node: SvelteLiteral) -> Self {
        Self::SvelteLiteral(node)
    }
}
impl From<SvelteName> for AnySvelteBindingProperty {
    fn from(node: SvelteName) -> Self {
        Self::SvelteName(node)
    }
}
impl AstNode for AnySvelteBindingProperty {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SvelteLiteral::KIND_SET.union(SvelteName::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SVELTE_LITERAL | SVELTE_NAME)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SVELTE_LITERAL => Self::SvelteLiteral(SvelteLiteral { syntax }),
            SVELTE_NAME => Self::SvelteName(SvelteName { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SvelteLiteral(it) => it.syntax(),
            Self::SvelteName(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SvelteLiteral(it) => it.into_syntax(),
            Self::SvelteName(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnySvelteBindingProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SvelteLiteral(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteName(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySvelteBindingProperty> for SyntaxNode {
    fn from(n: AnySvelteBindingProperty) -> Self {
        match n {
            AnySvelteBindingProperty::SvelteLiteral(it) => it.into_syntax(),
            AnySvelteBindingProperty::SvelteName(it) => it.into_syntax(),
        }
    }
}
impl From<AnySvelteBindingProperty> for SyntaxElement {
    fn from(n: AnySvelteBindingProperty) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SvelteAwaitBlock> for AnySvelteBlock {
    fn from(node: SvelteAwaitBlock) -> Self {
        Self::SvelteAwaitBlock(node)
    }
}
impl From<SvelteBogusBlock> for AnySvelteBlock {
    fn from(node: SvelteBogusBlock) -> Self {
        Self::SvelteBogusBlock(node)
    }
}
impl From<SvelteConstBlock> for AnySvelteBlock {
    fn from(node: SvelteConstBlock) -> Self {
        Self::SvelteConstBlock(node)
    }
}
impl From<SvelteDebugBlock> for AnySvelteBlock {
    fn from(node: SvelteDebugBlock) -> Self {
        Self::SvelteDebugBlock(node)
    }
}
impl From<SvelteEachBlock> for AnySvelteBlock {
    fn from(node: SvelteEachBlock) -> Self {
        Self::SvelteEachBlock(node)
    }
}
impl From<SvelteHtmlBlock> for AnySvelteBlock {
    fn from(node: SvelteHtmlBlock) -> Self {
        Self::SvelteHtmlBlock(node)
    }
}
impl From<SvelteIfBlock> for AnySvelteBlock {
    fn from(node: SvelteIfBlock) -> Self {
        Self::SvelteIfBlock(node)
    }
}
impl From<SvelteKeyBlock> for AnySvelteBlock {
    fn from(node: SvelteKeyBlock) -> Self {
        Self::SvelteKeyBlock(node)
    }
}
impl From<SvelteRenderBlock> for AnySvelteBlock {
    fn from(node: SvelteRenderBlock) -> Self {
        Self::SvelteRenderBlock(node)
    }
}
impl From<SvelteSnippetBlock> for AnySvelteBlock {
    fn from(node: SvelteSnippetBlock) -> Self {
        Self::SvelteSnippetBlock(node)
    }
}
impl AstNode for AnySvelteBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SvelteAwaitBlock::KIND_SET
        .union(SvelteBogusBlock::KIND_SET)
        .union(SvelteConstBlock::KIND_SET)
        .union(SvelteDebugBlock::KIND_SET)
        .union(SvelteEachBlock::KIND_SET)
        .union(SvelteHtmlBlock::KIND_SET)
        .union(SvelteIfBlock::KIND_SET)
        .union(SvelteKeyBlock::KIND_SET)
        .union(SvelteRenderBlock::KIND_SET)
        .union(SvelteSnippetBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SVELTE_AWAIT_BLOCK
                | SVELTE_BOGUS_BLOCK
                | SVELTE_CONST_BLOCK
                | SVELTE_DEBUG_BLOCK
                | SVELTE_EACH_BLOCK
                | SVELTE_HTML_BLOCK
                | SVELTE_IF_BLOCK
                | SVELTE_KEY_BLOCK
                | SVELTE_RENDER_BLOCK
                | SVELTE_SNIPPET_BLOCK
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SVELTE_AWAIT_BLOCK => Self::SvelteAwaitBlock(SvelteAwaitBlock { syntax }),
            SVELTE_BOGUS_BLOCK => Self::SvelteBogusBlock(SvelteBogusBlock { syntax }),
            SVELTE_CONST_BLOCK => Self::SvelteConstBlock(SvelteConstBlock { syntax }),
            SVELTE_DEBUG_BLOCK => Self::SvelteDebugBlock(SvelteDebugBlock { syntax }),
            SVELTE_EACH_BLOCK => Self::SvelteEachBlock(SvelteEachBlock { syntax }),
            SVELTE_HTML_BLOCK => Self::SvelteHtmlBlock(SvelteHtmlBlock { syntax }),
            SVELTE_IF_BLOCK => Self::SvelteIfBlock(SvelteIfBlock { syntax }),
            SVELTE_KEY_BLOCK => Self::SvelteKeyBlock(SvelteKeyBlock { syntax }),
            SVELTE_RENDER_BLOCK => Self::SvelteRenderBlock(SvelteRenderBlock { syntax }),
            SVELTE_SNIPPET_BLOCK => Self::SvelteSnippetBlock(SvelteSnippetBlock { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SvelteAwaitBlock(it) => it.syntax(),
            Self::SvelteBogusBlock(it) => it.syntax(),
            Self::SvelteConstBlock(it) => it.syntax(),
            Self::SvelteDebugBlock(it) => it.syntax(),
            Self::SvelteEachBlock(it) => it.syntax(),
            Self::SvelteHtmlBlock(it) => it.syntax(),
            Self::SvelteIfBlock(it) => it.syntax(),
            Self::SvelteKeyBlock(it) => it.syntax(),
            Self::SvelteRenderBlock(it) => it.syntax(),
            Self::SvelteSnippetBlock(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SvelteAwaitBlock(it) => it.into_syntax(),
            Self::SvelteBogusBlock(it) => it.into_syntax(),
            Self::SvelteConstBlock(it) => it.into_syntax(),
            Self::SvelteDebugBlock(it) => it.into_syntax(),
            Self::SvelteEachBlock(it) => it.into_syntax(),
            Self::SvelteHtmlBlock(it) => it.into_syntax(),
            Self::SvelteIfBlock(it) => it.into_syntax(),
            Self::SvelteKeyBlock(it) => it.into_syntax(),
            Self::SvelteRenderBlock(it) => it.into_syntax(),
            Self::SvelteSnippetBlock(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnySvelteBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SvelteAwaitBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteBogusBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteConstBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteDebugBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteEachBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteHtmlBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteIfBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteKeyBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteRenderBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteSnippetBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySvelteBlock> for SyntaxNode {
    fn from(n: AnySvelteBlock) -> Self {
        match n {
            AnySvelteBlock::SvelteAwaitBlock(it) => it.into_syntax(),
            AnySvelteBlock::SvelteBogusBlock(it) => it.into_syntax(),
            AnySvelteBlock::SvelteConstBlock(it) => it.into_syntax(),
            AnySvelteBlock::SvelteDebugBlock(it) => it.into_syntax(),
            AnySvelteBlock::SvelteEachBlock(it) => it.into_syntax(),
            AnySvelteBlock::SvelteHtmlBlock(it) => it.into_syntax(),
            AnySvelteBlock::SvelteIfBlock(it) => it.into_syntax(),
            AnySvelteBlock::SvelteKeyBlock(it) => it.into_syntax(),
            AnySvelteBlock::SvelteRenderBlock(it) => it.into_syntax(),
            AnySvelteBlock::SvelteSnippetBlock(it) => it.into_syntax(),
        }
    }
}
impl From<AnySvelteBlock> for SyntaxElement {
    fn from(n: AnySvelteBlock) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SvelteEachAsKeyedItem> for AnySvelteBlockItem {
    fn from(node: SvelteEachAsKeyedItem) -> Self {
        Self::SvelteEachAsKeyedItem(node)
    }
}
impl From<SvelteEachKeyedItem> for AnySvelteBlockItem {
    fn from(node: SvelteEachKeyedItem) -> Self {
        Self::SvelteEachKeyedItem(node)
    }
}
impl AstNode for AnySvelteBlockItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SvelteEachAsKeyedItem::KIND_SET.union(SvelteEachKeyedItem::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SVELTE_EACH_AS_KEYED_ITEM | SVELTE_EACH_KEYED_ITEM)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SVELTE_EACH_AS_KEYED_ITEM => {
                Self::SvelteEachAsKeyedItem(SvelteEachAsKeyedItem { syntax })
            }
            SVELTE_EACH_KEYED_ITEM => Self::SvelteEachKeyedItem(SvelteEachKeyedItem { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SvelteEachAsKeyedItem(it) => it.syntax(),
            Self::SvelteEachKeyedItem(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SvelteEachAsKeyedItem(it) => it.into_syntax(),
            Self::SvelteEachKeyedItem(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnySvelteBlockItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SvelteEachAsKeyedItem(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteEachKeyedItem(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySvelteBlockItem> for SyntaxNode {
    fn from(n: AnySvelteBlockItem) -> Self {
        match n {
            AnySvelteBlockItem::SvelteEachAsKeyedItem(it) => it.into_syntax(),
            AnySvelteBlockItem::SvelteEachKeyedItem(it) => it.into_syntax(),
        }
    }
}
impl From<AnySvelteBlockItem> for SyntaxElement {
    fn from(n: AnySvelteBlockItem) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SvelteCurlyDestructuredName> for AnySvelteDestructuredName {
    fn from(node: SvelteCurlyDestructuredName) -> Self {
        Self::SvelteCurlyDestructuredName(node)
    }
}
impl From<SvelteSquareDestructuredName> for AnySvelteDestructuredName {
    fn from(node: SvelteSquareDestructuredName) -> Self {
        Self::SvelteSquareDestructuredName(node)
    }
}
impl AstNode for AnySvelteDestructuredName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SvelteCurlyDestructuredName::KIND_SET.union(SvelteSquareDestructuredName::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SVELTE_CURLY_DESTRUCTURED_NAME | SVELTE_SQUARE_DESTRUCTURED_NAME
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SVELTE_CURLY_DESTRUCTURED_NAME => {
                Self::SvelteCurlyDestructuredName(SvelteCurlyDestructuredName { syntax })
            }
            SVELTE_SQUARE_DESTRUCTURED_NAME => {
                Self::SvelteSquareDestructuredName(SvelteSquareDestructuredName { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SvelteCurlyDestructuredName(it) => it.syntax(),
            Self::SvelteSquareDestructuredName(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SvelteCurlyDestructuredName(it) => it.into_syntax(),
            Self::SvelteSquareDestructuredName(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnySvelteDestructuredName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SvelteCurlyDestructuredName(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteSquareDestructuredName(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySvelteDestructuredName> for SyntaxNode {
    fn from(n: AnySvelteDestructuredName) -> Self {
        match n {
            AnySvelteDestructuredName::SvelteCurlyDestructuredName(it) => it.into_syntax(),
            AnySvelteDestructuredName::SvelteSquareDestructuredName(it) => it.into_syntax(),
        }
    }
}
impl From<AnySvelteDestructuredName> for SyntaxElement {
    fn from(n: AnySvelteDestructuredName) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SvelteAnimateDirective> for AnySvelteDirective {
    fn from(node: SvelteAnimateDirective) -> Self {
        Self::SvelteAnimateDirective(node)
    }
}
impl From<SvelteBindDirective> for AnySvelteDirective {
    fn from(node: SvelteBindDirective) -> Self {
        Self::SvelteBindDirective(node)
    }
}
impl From<SvelteClassDirective> for AnySvelteDirective {
    fn from(node: SvelteClassDirective) -> Self {
        Self::SvelteClassDirective(node)
    }
}
impl From<SvelteInDirective> for AnySvelteDirective {
    fn from(node: SvelteInDirective) -> Self {
        Self::SvelteInDirective(node)
    }
}
impl From<SvelteOutDirective> for AnySvelteDirective {
    fn from(node: SvelteOutDirective) -> Self {
        Self::SvelteOutDirective(node)
    }
}
impl From<SvelteStyleDirective> for AnySvelteDirective {
    fn from(node: SvelteStyleDirective) -> Self {
        Self::SvelteStyleDirective(node)
    }
}
impl From<SvelteTransitionDirective> for AnySvelteDirective {
    fn from(node: SvelteTransitionDirective) -> Self {
        Self::SvelteTransitionDirective(node)
    }
}
impl From<SvelteUseDirective> for AnySvelteDirective {
    fn from(node: SvelteUseDirective) -> Self {
        Self::SvelteUseDirective(node)
    }
}
impl AstNode for AnySvelteDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SvelteAnimateDirective::KIND_SET
        .union(SvelteBindDirective::KIND_SET)
        .union(SvelteClassDirective::KIND_SET)
        .union(SvelteInDirective::KIND_SET)
        .union(SvelteOutDirective::KIND_SET)
        .union(SvelteStyleDirective::KIND_SET)
        .union(SvelteTransitionDirective::KIND_SET)
        .union(SvelteUseDirective::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SVELTE_ANIMATE_DIRECTIVE
                | SVELTE_BIND_DIRECTIVE
                | SVELTE_CLASS_DIRECTIVE
                | SVELTE_IN_DIRECTIVE
                | SVELTE_OUT_DIRECTIVE
                | SVELTE_STYLE_DIRECTIVE
                | SVELTE_TRANSITION_DIRECTIVE
                | SVELTE_USE_DIRECTIVE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SVELTE_ANIMATE_DIRECTIVE => {
                Self::SvelteAnimateDirective(SvelteAnimateDirective { syntax })
            }
            SVELTE_BIND_DIRECTIVE => Self::SvelteBindDirective(SvelteBindDirective { syntax }),
            SVELTE_CLASS_DIRECTIVE => Self::SvelteClassDirective(SvelteClassDirective { syntax }),
            SVELTE_IN_DIRECTIVE => Self::SvelteInDirective(SvelteInDirective { syntax }),
            SVELTE_OUT_DIRECTIVE => Self::SvelteOutDirective(SvelteOutDirective { syntax }),
            SVELTE_STYLE_DIRECTIVE => Self::SvelteStyleDirective(SvelteStyleDirective { syntax }),
            SVELTE_TRANSITION_DIRECTIVE => {
                Self::SvelteTransitionDirective(SvelteTransitionDirective { syntax })
            }
            SVELTE_USE_DIRECTIVE => Self::SvelteUseDirective(SvelteUseDirective { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SvelteAnimateDirective(it) => it.syntax(),
            Self::SvelteBindDirective(it) => it.syntax(),
            Self::SvelteClassDirective(it) => it.syntax(),
            Self::SvelteInDirective(it) => it.syntax(),
            Self::SvelteOutDirective(it) => it.syntax(),
            Self::SvelteStyleDirective(it) => it.syntax(),
            Self::SvelteTransitionDirective(it) => it.syntax(),
            Self::SvelteUseDirective(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SvelteAnimateDirective(it) => it.into_syntax(),
            Self::SvelteBindDirective(it) => it.into_syntax(),
            Self::SvelteClassDirective(it) => it.into_syntax(),
            Self::SvelteInDirective(it) => it.into_syntax(),
            Self::SvelteOutDirective(it) => it.into_syntax(),
            Self::SvelteStyleDirective(it) => it.into_syntax(),
            Self::SvelteTransitionDirective(it) => it.into_syntax(),
            Self::SvelteUseDirective(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnySvelteDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SvelteAnimateDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteBindDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteClassDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteInDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteOutDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteStyleDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteTransitionDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteUseDirective(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySvelteDirective> for SyntaxNode {
    fn from(n: AnySvelteDirective) -> Self {
        match n {
            AnySvelteDirective::SvelteAnimateDirective(it) => it.into_syntax(),
            AnySvelteDirective::SvelteBindDirective(it) => it.into_syntax(),
            AnySvelteDirective::SvelteClassDirective(it) => it.into_syntax(),
            AnySvelteDirective::SvelteInDirective(it) => it.into_syntax(),
            AnySvelteDirective::SvelteOutDirective(it) => it.into_syntax(),
            AnySvelteDirective::SvelteStyleDirective(it) => it.into_syntax(),
            AnySvelteDirective::SvelteTransitionDirective(it) => it.into_syntax(),
            AnySvelteDirective::SvelteUseDirective(it) => it.into_syntax(),
        }
    }
}
impl From<AnySvelteDirective> for SyntaxElement {
    fn from(n: AnySvelteDirective) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<HtmlTextExpression> for AnySvelteEachName {
    fn from(node: HtmlTextExpression) -> Self {
        Self::HtmlTextExpression(node)
    }
}
impl From<SvelteName> for AnySvelteEachName {
    fn from(node: SvelteName) -> Self {
        Self::SvelteName(node)
    }
}
impl AstNode for AnySvelteEachName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnySvelteDestructuredName::KIND_SET
        .union(HtmlTextExpression::KIND_SET)
        .union(SvelteName::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            HTML_TEXT_EXPRESSION | SVELTE_NAME => true,
            k if AnySvelteDestructuredName::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            HTML_TEXT_EXPRESSION => Self::HtmlTextExpression(HtmlTextExpression { syntax }),
            SVELTE_NAME => Self::SvelteName(SvelteName { syntax }),
            _ => {
                if let Some(any_svelte_destructured_name) = AnySvelteDestructuredName::cast(syntax)
                {
                    return Some(Self::AnySvelteDestructuredName(
                        any_svelte_destructured_name,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::HtmlTextExpression(it) => it.syntax(),
            Self::SvelteName(it) => it.syntax(),
            Self::AnySvelteDestructuredName(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::HtmlTextExpression(it) => it.into_syntax(),
            Self::SvelteName(it) => it.into_syntax(),
            Self::AnySvelteDestructuredName(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnySvelteEachName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnySvelteDestructuredName(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlTextExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteName(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySvelteEachName> for SyntaxNode {
    fn from(n: AnySvelteEachName) -> Self {
        match n {
            AnySvelteEachName::AnySvelteDestructuredName(it) => it.into_syntax(),
            AnySvelteEachName::HtmlTextExpression(it) => it.into_syntax(),
            AnySvelteEachName::SvelteName(it) => it.into_syntax(),
        }
    }
}
impl From<AnySvelteEachName> for SyntaxElement {
    fn from(n: AnySvelteEachName) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<VueBogusDirective> for AnyVueDirective {
    fn from(node: VueBogusDirective) -> Self {
        Self::VueBogusDirective(node)
    }
}
impl From<VueDirective> for AnyVueDirective {
    fn from(node: VueDirective) -> Self {
        Self::VueDirective(node)
    }
}
impl From<VueVBindShorthandDirective> for AnyVueDirective {
    fn from(node: VueVBindShorthandDirective) -> Self {
        Self::VueVBindShorthandDirective(node)
    }
}
impl From<VueVOnShorthandDirective> for AnyVueDirective {
    fn from(node: VueVOnShorthandDirective) -> Self {
        Self::VueVOnShorthandDirective(node)
    }
}
impl From<VueVSlotShorthandDirective> for AnyVueDirective {
    fn from(node: VueVSlotShorthandDirective) -> Self {
        Self::VueVSlotShorthandDirective(node)
    }
}
impl AstNode for AnyVueDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = VueBogusDirective::KIND_SET
        .union(VueDirective::KIND_SET)
        .union(VueVBindShorthandDirective::KIND_SET)
        .union(VueVOnShorthandDirective::KIND_SET)
        .union(VueVSlotShorthandDirective::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            VUE_BOGUS_DIRECTIVE
                | VUE_DIRECTIVE
                | VUE_V_BIND_SHORTHAND_DIRECTIVE
                | VUE_V_ON_SHORTHAND_DIRECTIVE
                | VUE_V_SLOT_SHORTHAND_DIRECTIVE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            VUE_BOGUS_DIRECTIVE => Self::VueBogusDirective(VueBogusDirective { syntax }),
            VUE_DIRECTIVE => Self::VueDirective(VueDirective { syntax }),
            VUE_V_BIND_SHORTHAND_DIRECTIVE => {
                Self::VueVBindShorthandDirective(VueVBindShorthandDirective { syntax })
            }
            VUE_V_ON_SHORTHAND_DIRECTIVE => {
                Self::VueVOnShorthandDirective(VueVOnShorthandDirective { syntax })
            }
            VUE_V_SLOT_SHORTHAND_DIRECTIVE => {
                Self::VueVSlotShorthandDirective(VueVSlotShorthandDirective { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::VueBogusDirective(it) => it.syntax(),
            Self::VueDirective(it) => it.syntax(),
            Self::VueVBindShorthandDirective(it) => it.syntax(),
            Self::VueVOnShorthandDirective(it) => it.syntax(),
            Self::VueVSlotShorthandDirective(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::VueBogusDirective(it) => it.into_syntax(),
            Self::VueDirective(it) => it.into_syntax(),
            Self::VueVBindShorthandDirective(it) => it.into_syntax(),
            Self::VueVOnShorthandDirective(it) => it.into_syntax(),
            Self::VueVSlotShorthandDirective(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyVueDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VueBogusDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::VueDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::VueVBindShorthandDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::VueVOnShorthandDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::VueVSlotShorthandDirective(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyVueDirective> for SyntaxNode {
    fn from(n: AnyVueDirective) -> Self {
        match n {
            AnyVueDirective::VueBogusDirective(it) => it.into_syntax(),
            AnyVueDirective::VueDirective(it) => it.into_syntax(),
            AnyVueDirective::VueVBindShorthandDirective(it) => it.into_syntax(),
            AnyVueDirective::VueVOnShorthandDirective(it) => it.into_syntax(),
            AnyVueDirective::VueVSlotShorthandDirective(it) => it.into_syntax(),
        }
    }
}
impl From<AnyVueDirective> for SyntaxElement {
    fn from(n: AnyVueDirective) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<VueBogusDirectiveArgument> for AnyVueDirectiveArgument {
    fn from(node: VueBogusDirectiveArgument) -> Self {
        Self::VueBogusDirectiveArgument(node)
    }
}
impl From<VueDynamicArgument> for AnyVueDirectiveArgument {
    fn from(node: VueDynamicArgument) -> Self {
        Self::VueDynamicArgument(node)
    }
}
impl From<VueStaticArgument> for AnyVueDirectiveArgument {
    fn from(node: VueStaticArgument) -> Self {
        Self::VueStaticArgument(node)
    }
}
impl AstNode for AnyVueDirectiveArgument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = VueBogusDirectiveArgument::KIND_SET
        .union(VueDynamicArgument::KIND_SET)
        .union(VueStaticArgument::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            VUE_BOGUS_DIRECTIVE_ARGUMENT | VUE_DYNAMIC_ARGUMENT | VUE_STATIC_ARGUMENT
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            VUE_BOGUS_DIRECTIVE_ARGUMENT => {
                Self::VueBogusDirectiveArgument(VueBogusDirectiveArgument { syntax })
            }
            VUE_DYNAMIC_ARGUMENT => Self::VueDynamicArgument(VueDynamicArgument { syntax }),
            VUE_STATIC_ARGUMENT => Self::VueStaticArgument(VueStaticArgument { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::VueBogusDirectiveArgument(it) => it.syntax(),
            Self::VueDynamicArgument(it) => it.syntax(),
            Self::VueStaticArgument(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::VueBogusDirectiveArgument(it) => it.into_syntax(),
            Self::VueDynamicArgument(it) => it.into_syntax(),
            Self::VueStaticArgument(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyVueDirectiveArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VueBogusDirectiveArgument(it) => std::fmt::Debug::fmt(it, f),
            Self::VueDynamicArgument(it) => std::fmt::Debug::fmt(it, f),
            Self::VueStaticArgument(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyVueDirectiveArgument> for SyntaxNode {
    fn from(n: AnyVueDirectiveArgument) -> Self {
        match n {
            AnyVueDirectiveArgument::VueBogusDirectiveArgument(it) => it.into_syntax(),
            AnyVueDirectiveArgument::VueDynamicArgument(it) => it.into_syntax(),
            AnyVueDirectiveArgument::VueStaticArgument(it) => it.into_syntax(),
        }
    }
}
impl From<AnyVueDirectiveArgument> for SyntaxElement {
    fn from(n: AnyVueDirectiveArgument) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyAstroDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyAstroFrontmatterElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyHtmlAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyHtmlAttributeInitializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyHtmlComponentObjectName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyHtmlContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyHtmlElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyHtmlTagName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyHtmlTextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySvelteAwaitClauses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySvelteBindingAssignmentBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySvelteBindingProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySvelteBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySvelteBlockItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySvelteDestructuredName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySvelteDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySvelteEachName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyVueDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyVueDirectiveArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AstroClassDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AstroClientDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AstroDefineDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AstroDirectiveValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AstroEmbeddedContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AstroFrontmatterElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AstroIsDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AstroServerDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AstroSetDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlAttributeDoubleTextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlAttributeInitializerClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlAttributeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlAttributeSingleTextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlCdataSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlClosingElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlComponentName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlDoubleTextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlEmbeddedContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlOpeningElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlSelfClosingElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlSingleTextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlSpreadAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlTagName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HtmlTextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteAnimateDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteAttachAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteAwaitBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteAwaitCatchBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteAwaitCatchClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteAwaitClosingBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteAwaitOpeningBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteAwaitThenBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteAwaitThenClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteBindDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteClassDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteConstBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteCurlyDestructuredName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteDebugBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteDirectiveModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteDirectiveValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteEachAsKeyedItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteEachBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteEachClosingBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteEachIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteEachKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteEachKeyedItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteEachOpeningBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteElseIfClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteHtmlBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteIfBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteIfClosingBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteIfOpeningBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteInDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteKeyBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteKeyClosingBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteKeyOpeningBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteOutDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteRenderBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteRestBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteSnippetBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteSnippetClosingBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteSnippetOpeningBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteSquareDestructuredName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteStyleDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteTransitionDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteUseDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VueDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VueDirectiveArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VueDynamicArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VueModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VueStaticArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VueVBindShorthandDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VueVOnShorthandDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VueVSlotShorthandDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct AstroBogusFrontmatter {
    syntax: SyntaxNode,
}
impl AstroBogusFrontmatter {
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
impl AstNode for AstroBogusFrontmatter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ASTRO_BOGUS_FRONTMATTER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ASTRO_BOGUS_FRONTMATTER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for AstroBogusFrontmatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AstroBogusFrontmatter")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<AstroBogusFrontmatter> for SyntaxNode {
    fn from(n: AstroBogusFrontmatter) -> Self {
        n.syntax
    }
}
impl From<AstroBogusFrontmatter> for SyntaxElement {
    fn from(n: AstroBogusFrontmatter) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct HtmlBogus {
    syntax: SyntaxNode,
}
impl HtmlBogus {
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
impl AstNode for HtmlBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_BOGUS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HtmlBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<HtmlBogus> for SyntaxNode {
    fn from(n: HtmlBogus) -> Self {
        n.syntax
    }
}
impl From<HtmlBogus> for SyntaxElement {
    fn from(n: HtmlBogus) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct HtmlBogusAttribute {
    syntax: SyntaxNode,
}
impl HtmlBogusAttribute {
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
impl AstNode for HtmlBogusAttribute {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_BOGUS_ATTRIBUTE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_BOGUS_ATTRIBUTE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlBogusAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HtmlBogusAttribute")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<HtmlBogusAttribute> for SyntaxNode {
    fn from(n: HtmlBogusAttribute) -> Self {
        n.syntax
    }
}
impl From<HtmlBogusAttribute> for SyntaxElement {
    fn from(n: HtmlBogusAttribute) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct HtmlBogusElement {
    syntax: SyntaxNode,
}
impl HtmlBogusElement {
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
impl AstNode for HtmlBogusElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_BOGUS_ELEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_BOGUS_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlBogusElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HtmlBogusElement")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<HtmlBogusElement> for SyntaxNode {
    fn from(n: HtmlBogusElement) -> Self {
        n.syntax
    }
}
impl From<HtmlBogusElement> for SyntaxElement {
    fn from(n: HtmlBogusElement) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct HtmlBogusTextExpression {
    syntax: SyntaxNode,
}
impl HtmlBogusTextExpression {
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
impl AstNode for HtmlBogusTextExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_BOGUS_TEXT_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_BOGUS_TEXT_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlBogusTextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HtmlBogusTextExpression")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<HtmlBogusTextExpression> for SyntaxNode {
    fn from(n: HtmlBogusTextExpression) -> Self {
        n.syntax
    }
}
impl From<HtmlBogusTextExpression> for SyntaxElement {
    fn from(n: HtmlBogusTextExpression) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SvelteBogusBlock {
    syntax: SyntaxNode,
}
impl SvelteBogusBlock {
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
impl AstNode for SvelteBogusBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_BOGUS_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_BOGUS_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteBogusBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SvelteBogusBlock")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<SvelteBogusBlock> for SyntaxNode {
    fn from(n: SvelteBogusBlock) -> Self {
        n.syntax
    }
}
impl From<SvelteBogusBlock> for SyntaxElement {
    fn from(n: SvelteBogusBlock) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct VueBogusDirective {
    syntax: SyntaxNode,
}
impl VueBogusDirective {
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
impl AstNode for VueBogusDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_BOGUS_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_BOGUS_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for VueBogusDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VueBogusDirective")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<VueBogusDirective> for SyntaxNode {
    fn from(n: VueBogusDirective) -> Self {
        n.syntax
    }
}
impl From<VueBogusDirective> for SyntaxElement {
    fn from(n: VueBogusDirective) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct VueBogusDirectiveArgument {
    syntax: SyntaxNode,
}
impl VueBogusDirectiveArgument {
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
impl AstNode for VueBogusDirectiveArgument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_BOGUS_DIRECTIVE_ARGUMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_BOGUS_DIRECTIVE_ARGUMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for VueBogusDirectiveArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VueBogusDirectiveArgument")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<VueBogusDirectiveArgument> for SyntaxNode {
    fn from(n: VueBogusDirectiveArgument) -> Self {
        n.syntax
    }
}
impl From<VueBogusDirectiveArgument> for SyntaxElement {
    fn from(n: VueBogusDirectiveArgument) -> Self {
        n.syntax.into()
    }
}
biome_rowan::declare_node_union! { pub AnyHtmlBogusNode = AstroBogusFrontmatter | HtmlBogus | HtmlBogusAttribute | HtmlBogusElement | HtmlBogusTextExpression | SvelteBogusBlock | VueBogusDirective | VueBogusDirectiveArgument }
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct HtmlAttributeList {
    syntax_list: SyntaxList,
}
impl HtmlAttributeList {
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
impl AstNode for HtmlAttributeList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_ATTRIBUTE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_ATTRIBUTE_LIST
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
impl Serialize for HtmlAttributeList {
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
impl AstNodeList for HtmlAttributeList {
    type Language = Language;
    type Node = AnyHtmlAttribute;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for HtmlAttributeList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("HtmlAttributeList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &HtmlAttributeList {
    type Item = AnyHtmlAttribute;
    type IntoIter = AstNodeListIterator<Language, AnyHtmlAttribute>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for HtmlAttributeList {
    type Item = AnyHtmlAttribute;
    type IntoIter = AstNodeListIterator<Language, AnyHtmlAttribute>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct HtmlElementList {
    syntax_list: SyntaxList,
}
impl HtmlElementList {
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
impl AstNode for HtmlElementList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_ELEMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_ELEMENT_LIST
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
impl Serialize for HtmlElementList {
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
impl AstNodeList for HtmlElementList {
    type Language = Language;
    type Node = AnyHtmlElement;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for HtmlElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("HtmlElementList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &HtmlElementList {
    type Item = AnyHtmlElement;
    type IntoIter = AstNodeListIterator<Language, AnyHtmlElement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for HtmlElementList {
    type Item = AnyHtmlElement;
    type IntoIter = AstNodeListIterator<Language, AnyHtmlElement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SvelteAwaitClausesList {
    syntax_list: SyntaxList,
}
impl SvelteAwaitClausesList {
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
impl AstNode for SvelteAwaitClausesList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_AWAIT_CLAUSES_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_AWAIT_CLAUSES_LIST
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
impl Serialize for SvelteAwaitClausesList {
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
impl AstNodeList for SvelteAwaitClausesList {
    type Language = Language;
    type Node = AnySvelteAwaitClauses;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SvelteAwaitClausesList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SvelteAwaitClausesList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &SvelteAwaitClausesList {
    type Item = AnySvelteAwaitClauses;
    type IntoIter = AstNodeListIterator<Language, AnySvelteAwaitClauses>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for SvelteAwaitClausesList {
    type Item = AnySvelteAwaitClauses;
    type IntoIter = AstNodeListIterator<Language, AnySvelteAwaitClauses>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SvelteBindingAssignmentBindingList {
    syntax_list: SyntaxList,
}
impl SvelteBindingAssignmentBindingList {
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
impl AstNode for SvelteBindingAssignmentBindingList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_BINDING_ASSIGNMENT_BINDING_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_BINDING_ASSIGNMENT_BINDING_LIST
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
impl Serialize for SvelteBindingAssignmentBindingList {
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
impl AstSeparatedList for SvelteBindingAssignmentBindingList {
    type Language = Language;
    type Node = AnySvelteBindingAssignmentBinding;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SvelteBindingAssignmentBindingList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SvelteBindingAssignmentBindingList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SvelteBindingAssignmentBindingList {
    type Item = SyntaxResult<AnySvelteBindingAssignmentBinding>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnySvelteBindingAssignmentBinding>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SvelteBindingAssignmentBindingList {
    type Item = SyntaxResult<AnySvelteBindingAssignmentBinding>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnySvelteBindingAssignmentBinding>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SvelteBindingList {
    syntax_list: SyntaxList,
}
impl SvelteBindingList {
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
impl AstNode for SvelteBindingList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_BINDING_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_BINDING_LIST
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
impl Serialize for SvelteBindingList {
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
impl AstSeparatedList for SvelteBindingList {
    type Language = Language;
    type Node = SvelteName;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SvelteBindingList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SvelteBindingList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SvelteBindingList {
    type Item = SyntaxResult<SvelteName>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SvelteName>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SvelteBindingList {
    type Item = SyntaxResult<SvelteName>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SvelteName>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SvelteDirectiveModifierList {
    syntax_list: SyntaxList,
}
impl SvelteDirectiveModifierList {
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
impl AstNode for SvelteDirectiveModifierList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_DIRECTIVE_MODIFIER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_DIRECTIVE_MODIFIER_LIST
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
impl Serialize for SvelteDirectiveModifierList {
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
impl AstNodeList for SvelteDirectiveModifierList {
    type Language = Language;
    type Node = SvelteDirectiveModifier;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SvelteDirectiveModifierList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SvelteDirectiveModifierList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &SvelteDirectiveModifierList {
    type Item = SvelteDirectiveModifier;
    type IntoIter = AstNodeListIterator<Language, SvelteDirectiveModifier>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for SvelteDirectiveModifierList {
    type Item = SvelteDirectiveModifier;
    type IntoIter = AstNodeListIterator<Language, SvelteDirectiveModifier>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SvelteElseIfClauseList {
    syntax_list: SyntaxList,
}
impl SvelteElseIfClauseList {
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
impl AstNode for SvelteElseIfClauseList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_ELSE_IF_CLAUSE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_ELSE_IF_CLAUSE_LIST
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
impl Serialize for SvelteElseIfClauseList {
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
impl AstNodeList for SvelteElseIfClauseList {
    type Language = Language;
    type Node = SvelteElseIfClause;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SvelteElseIfClauseList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SvelteElseIfClauseList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &SvelteElseIfClauseList {
    type Item = SvelteElseIfClause;
    type IntoIter = AstNodeListIterator<Language, SvelteElseIfClause>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for SvelteElseIfClauseList {
    type Item = SvelteElseIfClause;
    type IntoIter = AstNodeListIterator<Language, SvelteElseIfClause>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct VueModifierList {
    syntax_list: SyntaxList,
}
impl VueModifierList {
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
impl AstNode for VueModifierList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_MODIFIER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_MODIFIER_LIST
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
impl Serialize for VueModifierList {
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
impl AstNodeList for VueModifierList {
    type Language = Language;
    type Node = VueModifier;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for VueModifierList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("VueModifierList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &VueModifierList {
    type Item = VueModifier;
    type IntoIter = AstNodeListIterator<Language, VueModifier>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for VueModifierList {
    type Item = VueModifier;
    type IntoIter = AstNodeListIterator<Language, VueModifier>;
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
