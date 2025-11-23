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
    pub fn name(&self) -> SyntaxResult<HtmlTagName> {
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
    pub name: SyntaxResult<HtmlTagName>,
    pub r_angle_token: SyntaxResult<SyntaxToken>,
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
    pub fn name(&self) -> SyntaxResult<HtmlTagName> {
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
    pub name: SyntaxResult<HtmlTagName>,
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
    pub fn name(&self) -> SyntaxResult<HtmlTagName> {
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
    pub name: SyntaxResult<HtmlTagName>,
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
            svelte_ident_token: self.svelte_ident_token(),
        }
    }
    pub fn svelte_ident_token(&self) -> SyntaxResult<SyntaxToken> {
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
    pub svelte_ident_token: SyntaxResult<SyntaxToken>,
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
    AnyVueDirective(AnyVueDirective),
    HtmlAttribute(HtmlAttribute),
    HtmlBogusAttribute(HtmlBogusAttribute),
    HtmlDoubleTextExpression(HtmlDoubleTextExpression),
    HtmlSingleTextExpression(HtmlSingleTextExpression),
    SvelteAttachAttribute(SvelteAttachAttribute),
}
impl AnyHtmlAttribute {
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
    pub fn as_html_bogus_attribute(&self) -> Option<&HtmlBogusAttribute> {
        match &self {
            Self::HtmlBogusAttribute(item) => Some(item),
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
    pub fn as_svelte_attach_attribute(&self) -> Option<&SvelteAttachAttribute> {
        match &self {
            Self::SvelteAttachAttribute(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyHtmlAttributeInitializer {
    HtmlSingleTextExpression(HtmlSingleTextExpression),
    HtmlString(HtmlString),
}
impl AnyHtmlAttributeInitializer {
    pub fn as_html_single_text_expression(&self) -> Option<&HtmlSingleTextExpression> {
        match &self {
            Self::HtmlSingleTextExpression(item) => Some(item),
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
pub enum AnySvelteBlock {
    SvelteBogusBlock(SvelteBogusBlock),
    SvelteConstBlock(SvelteConstBlock),
    SvelteDebugBlock(SvelteDebugBlock),
    SvelteHtmlBlock(SvelteHtmlBlock),
    SvelteIfBlock(SvelteIfBlock),
    SvelteKeyBlock(SvelteKeyBlock),
    SvelteRenderBlock(SvelteRenderBlock),
}
impl AnySvelteBlock {
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
                    "svelte_ident_token",
                    &support::DebugSyntaxResult(self.svelte_ident_token()),
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
impl From<HtmlBogusAttribute> for AnyHtmlAttribute {
    fn from(node: HtmlBogusAttribute) -> Self {
        Self::HtmlBogusAttribute(node)
    }
}
impl From<HtmlDoubleTextExpression> for AnyHtmlAttribute {
    fn from(node: HtmlDoubleTextExpression) -> Self {
        Self::HtmlDoubleTextExpression(node)
    }
}
impl From<HtmlSingleTextExpression> for AnyHtmlAttribute {
    fn from(node: HtmlSingleTextExpression) -> Self {
        Self::HtmlSingleTextExpression(node)
    }
}
impl From<SvelteAttachAttribute> for AnyHtmlAttribute {
    fn from(node: SvelteAttachAttribute) -> Self {
        Self::SvelteAttachAttribute(node)
    }
}
impl AstNode for AnyHtmlAttribute {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyVueDirective::KIND_SET
        .union(HtmlAttribute::KIND_SET)
        .union(HtmlBogusAttribute::KIND_SET)
        .union(HtmlDoubleTextExpression::KIND_SET)
        .union(HtmlSingleTextExpression::KIND_SET)
        .union(SvelteAttachAttribute::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            HTML_ATTRIBUTE
            | HTML_BOGUS_ATTRIBUTE
            | HTML_DOUBLE_TEXT_EXPRESSION
            | HTML_SINGLE_TEXT_EXPRESSION
            | SVELTE_ATTACH_ATTRIBUTE => true,
            k if AnyVueDirective::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            HTML_ATTRIBUTE => Self::HtmlAttribute(HtmlAttribute { syntax }),
            HTML_BOGUS_ATTRIBUTE => Self::HtmlBogusAttribute(HtmlBogusAttribute { syntax }),
            HTML_DOUBLE_TEXT_EXPRESSION => {
                Self::HtmlDoubleTextExpression(HtmlDoubleTextExpression { syntax })
            }
            HTML_SINGLE_TEXT_EXPRESSION => {
                Self::HtmlSingleTextExpression(HtmlSingleTextExpression { syntax })
            }
            SVELTE_ATTACH_ATTRIBUTE => {
                Self::SvelteAttachAttribute(SvelteAttachAttribute { syntax })
            }
            _ => {
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
            Self::HtmlBogusAttribute(it) => it.syntax(),
            Self::HtmlDoubleTextExpression(it) => it.syntax(),
            Self::HtmlSingleTextExpression(it) => it.syntax(),
            Self::SvelteAttachAttribute(it) => it.syntax(),
            Self::AnyVueDirective(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::HtmlAttribute(it) => it.into_syntax(),
            Self::HtmlBogusAttribute(it) => it.into_syntax(),
            Self::HtmlDoubleTextExpression(it) => it.into_syntax(),
            Self::HtmlSingleTextExpression(it) => it.into_syntax(),
            Self::SvelteAttachAttribute(it) => it.into_syntax(),
            Self::AnyVueDirective(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyHtmlAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyVueDirective(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlAttribute(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlBogusAttribute(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlDoubleTextExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlSingleTextExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteAttachAttribute(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyHtmlAttribute> for SyntaxNode {
    fn from(n: AnyHtmlAttribute) -> Self {
        match n {
            AnyHtmlAttribute::AnyVueDirective(it) => it.into_syntax(),
            AnyHtmlAttribute::HtmlAttribute(it) => it.into_syntax(),
            AnyHtmlAttribute::HtmlBogusAttribute(it) => it.into_syntax(),
            AnyHtmlAttribute::HtmlDoubleTextExpression(it) => it.into_syntax(),
            AnyHtmlAttribute::HtmlSingleTextExpression(it) => it.into_syntax(),
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
impl From<HtmlSingleTextExpression> for AnyHtmlAttributeInitializer {
    fn from(node: HtmlSingleTextExpression) -> Self {
        Self::HtmlSingleTextExpression(node)
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
        HtmlSingleTextExpression::KIND_SET.union(HtmlString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, HTML_SINGLE_TEXT_EXPRESSION | HTML_STRING)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            HTML_SINGLE_TEXT_EXPRESSION => {
                Self::HtmlSingleTextExpression(HtmlSingleTextExpression { syntax })
            }
            HTML_STRING => Self::HtmlString(HtmlString { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::HtmlSingleTextExpression(it) => it.syntax(),
            Self::HtmlString(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::HtmlSingleTextExpression(it) => it.into_syntax(),
            Self::HtmlString(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyHtmlAttributeInitializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HtmlSingleTextExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyHtmlAttributeInitializer> for SyntaxNode {
    fn from(n: AnyHtmlAttributeInitializer) -> Self {
        match n {
            AnyHtmlAttributeInitializer::HtmlSingleTextExpression(it) => it.into_syntax(),
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
impl AstNode for AnySvelteBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SvelteBogusBlock::KIND_SET
        .union(SvelteConstBlock::KIND_SET)
        .union(SvelteDebugBlock::KIND_SET)
        .union(SvelteHtmlBlock::KIND_SET)
        .union(SvelteIfBlock::KIND_SET)
        .union(SvelteKeyBlock::KIND_SET)
        .union(SvelteRenderBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SVELTE_BOGUS_BLOCK
                | SVELTE_CONST_BLOCK
                | SVELTE_DEBUG_BLOCK
                | SVELTE_HTML_BLOCK
                | SVELTE_IF_BLOCK
                | SVELTE_KEY_BLOCK
                | SVELTE_RENDER_BLOCK
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SVELTE_BOGUS_BLOCK => Self::SvelteBogusBlock(SvelteBogusBlock { syntax }),
            SVELTE_CONST_BLOCK => Self::SvelteConstBlock(SvelteConstBlock { syntax }),
            SVELTE_DEBUG_BLOCK => Self::SvelteDebugBlock(SvelteDebugBlock { syntax }),
            SVELTE_HTML_BLOCK => Self::SvelteHtmlBlock(SvelteHtmlBlock { syntax }),
            SVELTE_IF_BLOCK => Self::SvelteIfBlock(SvelteIfBlock { syntax }),
            SVELTE_KEY_BLOCK => Self::SvelteKeyBlock(SvelteKeyBlock { syntax }),
            SVELTE_RENDER_BLOCK => Self::SvelteRenderBlock(SvelteRenderBlock { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SvelteBogusBlock(it) => it.syntax(),
            Self::SvelteConstBlock(it) => it.syntax(),
            Self::SvelteDebugBlock(it) => it.syntax(),
            Self::SvelteHtmlBlock(it) => it.syntax(),
            Self::SvelteIfBlock(it) => it.syntax(),
            Self::SvelteKeyBlock(it) => it.syntax(),
            Self::SvelteRenderBlock(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SvelteBogusBlock(it) => it.into_syntax(),
            Self::SvelteConstBlock(it) => it.into_syntax(),
            Self::SvelteDebugBlock(it) => it.into_syntax(),
            Self::SvelteHtmlBlock(it) => it.into_syntax(),
            Self::SvelteIfBlock(it) => it.into_syntax(),
            Self::SvelteKeyBlock(it) => it.into_syntax(),
            Self::SvelteRenderBlock(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnySvelteBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SvelteBogusBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteConstBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteDebugBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteHtmlBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteIfBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteKeyBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteRenderBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySvelteBlock> for SyntaxNode {
    fn from(n: AnySvelteBlock) -> Self {
        match n {
            AnySvelteBlock::SvelteBogusBlock(it) => it.into_syntax(),
            AnySvelteBlock::SvelteConstBlock(it) => it.into_syntax(),
            AnySvelteBlock::SvelteDebugBlock(it) => it.into_syntax(),
            AnySvelteBlock::SvelteHtmlBlock(it) => it.into_syntax(),
            AnySvelteBlock::SvelteIfBlock(it) => it.into_syntax(),
            AnySvelteBlock::SvelteKeyBlock(it) => it.into_syntax(),
            AnySvelteBlock::SvelteRenderBlock(it) => it.into_syntax(),
        }
    }
}
impl From<AnySvelteBlock> for SyntaxElement {
    fn from(n: AnySvelteBlock) -> Self {
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
impl std::fmt::Display for AnyHtmlTextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySvelteBlock {
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
impl std::fmt::Display for HtmlAttribute {
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
impl std::fmt::Display for SvelteAttachAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteConstBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteDebugBlock {
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
impl std::fmt::Display for SvelteName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SvelteRenderBlock {
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
