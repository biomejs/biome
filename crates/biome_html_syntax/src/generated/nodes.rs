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
            content_token: self.content_token(),
            r_fence_token: self.r_fence_token(),
        }
    }
    pub fn l_fence_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn content_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
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
    pub content_token: Option<SyntaxToken>,
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
    pub fn value(&self) -> SyntaxResult<HtmlString> {
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
    pub value: SyntaxResult<HtmlString>,
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
            l_double_curly_token: self.l_double_curly_token(),
            expression_token: self.expression_token(),
            r_double_curly_token: self.r_double_curly_token(),
        }
    }
    pub fn l_double_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn r_double_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
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
    pub l_double_curly_token: SyntaxResult<SyntaxToken>,
    pub expression_token: SyntaxResult<SyntaxToken>,
    pub r_double_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SvelteTextExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SvelteTextExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SvelteTextExpressionFields {
        SvelteTextExpressionFields {
            l_curly_token: self.l_curly_token(),
            expression_token: self.expression_token(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SvelteTextExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SvelteTextExpressionFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub expression_token: SyntaxResult<SyntaxToken>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
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
    HtmlAttribute(HtmlAttribute),
    HtmlBogusAttribute(HtmlBogusAttribute),
}
impl AnyHtmlAttribute {
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
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyHtmlContent {
    AnyHtmlTextExpression(AnyHtmlTextExpression),
    HtmlContent(HtmlContent),
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
    HtmlTextExpression(HtmlTextExpression),
    SvelteTextExpression(SvelteTextExpression),
}
impl AnyHtmlTextExpression {
    pub fn as_html_text_expression(&self) -> Option<&HtmlTextExpression> {
        match &self {
            Self::HtmlTextExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_svelte_text_expression(&self) -> Option<&SvelteTextExpression> {
        match &self {
            Self::SvelteTextExpression(item) => Some(item),
            _ => None,
        }
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
                .field(
                    "content_token",
                    &support::DebugOptionalElement(self.content_token()),
                )
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
                    "l_double_curly_token",
                    &support::DebugSyntaxResult(self.l_double_curly_token()),
                )
                .field(
                    "expression_token",
                    &support::DebugSyntaxResult(self.expression_token()),
                )
                .field(
                    "r_double_curly_token",
                    &support::DebugSyntaxResult(self.r_double_curly_token()),
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
impl AstNode for SvelteTextExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SVELTE_TEXT_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SVELTE_TEXT_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SvelteTextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SvelteTextExpression")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field(
                    "expression_token",
                    &support::DebugSyntaxResult(self.expression_token()),
                )
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("SvelteTextExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SvelteTextExpression> for SyntaxNode {
    fn from(n: SvelteTextExpression) -> Self {
        n.syntax
    }
}
impl From<SvelteTextExpression> for SyntaxElement {
    fn from(n: SvelteTextExpression) -> Self {
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
            Self::AstroBogusFrontmatter(it) => &it.syntax,
            Self::AstroFrontmatterElement(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::AstroBogusFrontmatter(it) => it.syntax,
            Self::AstroFrontmatterElement(it) => it.syntax,
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
            AnyAstroFrontmatterElement::AstroBogusFrontmatter(it) => it.into(),
            AnyAstroFrontmatterElement::AstroFrontmatterElement(it) => it.into(),
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
impl AstNode for AnyHtmlAttribute {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        HtmlAttribute::KIND_SET.union(HtmlBogusAttribute::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, HTML_ATTRIBUTE | HTML_BOGUS_ATTRIBUTE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            HTML_ATTRIBUTE => Self::HtmlAttribute(HtmlAttribute { syntax }),
            HTML_BOGUS_ATTRIBUTE => Self::HtmlBogusAttribute(HtmlBogusAttribute { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::HtmlAttribute(it) => &it.syntax,
            Self::HtmlBogusAttribute(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::HtmlAttribute(it) => it.syntax,
            Self::HtmlBogusAttribute(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyHtmlAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HtmlAttribute(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlBogusAttribute(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyHtmlAttribute> for SyntaxNode {
    fn from(n: AnyHtmlAttribute) -> Self {
        match n {
            AnyHtmlAttribute::HtmlAttribute(it) => it.into(),
            AnyHtmlAttribute::HtmlBogusAttribute(it) => it.into(),
        }
    }
}
impl From<AnyHtmlAttribute> for SyntaxElement {
    fn from(n: AnyHtmlAttribute) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<HtmlContent> for AnyHtmlContent {
    fn from(node: HtmlContent) -> Self {
        Self::HtmlContent(node)
    }
}
impl AstNode for AnyHtmlContent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyHtmlTextExpression::KIND_SET.union(HtmlContent::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            HTML_CONTENT => true,
            k if AnyHtmlTextExpression::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            HTML_CONTENT => Self::HtmlContent(HtmlContent { syntax }),
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
            Self::HtmlContent(it) => &it.syntax,
            Self::AnyHtmlTextExpression(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::HtmlContent(it) => it.syntax,
            Self::AnyHtmlTextExpression(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyHtmlContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyHtmlTextExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::HtmlContent(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyHtmlContent> for SyntaxNode {
    fn from(n: AnyHtmlContent) -> Self {
        match n {
            AnyHtmlContent::AnyHtmlTextExpression(it) => it.into(),
            AnyHtmlContent::HtmlContent(it) => it.into(),
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
            Self::HtmlBogusElement(it) => &it.syntax,
            Self::HtmlCdataSection(it) => &it.syntax,
            Self::HtmlElement(it) => &it.syntax,
            Self::HtmlSelfClosingElement(it) => &it.syntax,
            Self::AnyHtmlContent(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::HtmlBogusElement(it) => it.syntax,
            Self::HtmlCdataSection(it) => it.syntax,
            Self::HtmlElement(it) => it.syntax,
            Self::HtmlSelfClosingElement(it) => it.syntax,
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
            AnyHtmlElement::AnyHtmlContent(it) => it.into(),
            AnyHtmlElement::HtmlBogusElement(it) => it.into(),
            AnyHtmlElement::HtmlCdataSection(it) => it.into(),
            AnyHtmlElement::HtmlElement(it) => it.into(),
            AnyHtmlElement::HtmlSelfClosingElement(it) => it.into(),
        }
    }
}
impl From<AnyHtmlElement> for SyntaxElement {
    fn from(n: AnyHtmlElement) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<HtmlTextExpression> for AnyHtmlTextExpression {
    fn from(node: HtmlTextExpression) -> Self {
        Self::HtmlTextExpression(node)
    }
}
impl From<SvelteTextExpression> for AnyHtmlTextExpression {
    fn from(node: SvelteTextExpression) -> Self {
        Self::SvelteTextExpression(node)
    }
}
impl AstNode for AnyHtmlTextExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        HtmlTextExpression::KIND_SET.union(SvelteTextExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, HTML_TEXT_EXPRESSION | SVELTE_TEXT_EXPRESSION)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            HTML_TEXT_EXPRESSION => Self::HtmlTextExpression(HtmlTextExpression { syntax }),
            SVELTE_TEXT_EXPRESSION => Self::SvelteTextExpression(SvelteTextExpression { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::HtmlTextExpression(it) => &it.syntax,
            Self::SvelteTextExpression(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::HtmlTextExpression(it) => it.syntax,
            Self::SvelteTextExpression(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyHtmlTextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HtmlTextExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SvelteTextExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyHtmlTextExpression> for SyntaxNode {
    fn from(n: AnyHtmlTextExpression) -> Self {
        match n {
            AnyHtmlTextExpression::HtmlTextExpression(it) => it.into(),
            AnyHtmlTextExpression::SvelteTextExpression(it) => it.into(),
        }
    }
}
impl From<AnyHtmlTextExpression> for SyntaxElement {
    fn from(n: AnyHtmlTextExpression) -> Self {
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
impl std::fmt::Display for HtmlElement {
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
impl std::fmt::Display for SvelteTextExpression {
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
biome_rowan::declare_node_union! { pub AnyHtmlBogusNode = AstroBogusFrontmatter | HtmlBogus | HtmlBogusAttribute | HtmlBogusElement }
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
