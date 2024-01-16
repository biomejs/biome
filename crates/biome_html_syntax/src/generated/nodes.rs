//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
use crate::{
    macros::map_syntax_node,
    HtmlLanguage as Language, HtmlSyntaxElement as SyntaxElement,
    HtmlSyntaxElementChildren as SyntaxElementChildren,
    HtmlSyntaxKind::{self as SyntaxKind, *},
    HtmlSyntaxList as SyntaxList, HtmlSyntaxNode as SyntaxNode, HtmlSyntaxToken as SyntaxToken,
};
use biome_rowan::{support, AstNode, RawSyntaxKind, SyntaxKindSet, SyntaxResult};
#[allow(unused)]
use biome_rowan::{
    AstNodeList, AstNodeListIterator, AstNodeSlotMap, AstSeparatedList,
    AstSeparatedListNodesIterator,
};
#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Formatter};
#[doc = r" Sentinel value indicating a missing element in a dynamic node, where"]
#[doc = r" the slots are not statically known."]
#[allow(dead_code)]
pub(crate) const SLOT_MAP_EMPTY_VALUE: u8 = u8::MAX;
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
    pub fn name(&self) -> SyntaxResult<HtmlName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn initializer(&self) -> Option<HtmlAttributeInitializerClause> {
        support::node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for HtmlAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct HtmlAttributeFields {
    pub name: SyntaxResult<HtmlName>,
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
#[cfg(feature = "serde")]
impl Serialize for HtmlAttributeInitializerClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct HtmlAttributeInitializerClauseFields {
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<HtmlString>,
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
    pub fn name(&self) -> SyntaxResult<HtmlName> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for HtmlClosingElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct HtmlClosingElementFields {
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub slash_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<HtmlName>,
    pub r_angle_token: SyntaxResult<SyntaxToken>,
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
            content: self.content(),
            r_angle_token: self.r_angle_token(),
        }
    }
    pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn content(&self) -> SyntaxResult<HtmlString> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for HtmlDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct HtmlDirectiveFields {
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub excl_token: SyntaxResult<SyntaxToken>,
    pub content: SyntaxResult<HtmlString>,
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
#[cfg(feature = "serde")]
impl Serialize for HtmlElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct HtmlElementFields {
    pub opening_element: SyntaxResult<HtmlOpeningElement>,
    pub children: HtmlElementList,
    pub closing_element: SyntaxResult<HtmlClosingElement>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HtmlName {
    pub(crate) syntax: SyntaxNode,
}
impl HtmlName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> HtmlNameFields {
        HtmlNameFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for HtmlName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct HtmlNameFields {
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
    pub fn name(&self) -> SyntaxResult<HtmlName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn attributes(&self) -> HtmlAttributeList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for HtmlOpeningElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct HtmlOpeningElementFields {
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<HtmlName>,
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
            directive: self.directive(),
            tags: self.tags(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn directive(&self) -> SyntaxResult<HtmlDirective> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn tags(&self) -> HtmlElementList {
        support::list(&self.syntax, 2usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for HtmlRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct HtmlRootFields {
    pub bom_token: Option<SyntaxToken>,
    pub directive: SyntaxResult<HtmlDirective>,
    pub tags: HtmlElementList,
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
    pub fn name(&self) -> SyntaxResult<HtmlName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn attributes(&self) -> HtmlAttributeList {
        support::list(&self.syntax, 2usize)
    }
    pub fn slash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for HtmlSelfClosingElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct HtmlSelfClosingElementFields {
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<HtmlName>,
    pub attributes: HtmlAttributeList,
    pub slash_token: SyntaxResult<SyntaxToken>,
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
#[cfg(feature = "serde")]
impl Serialize for HtmlString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct HtmlStringFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyHtmlElement {
    HtmlElement(HtmlElement),
    HtmlSelfClosingElement(HtmlSelfClosingElement),
}
impl AnyHtmlElement {
    pub fn as_html_element(&self) -> Option<&HtmlElement> {
        match &self {
            AnyHtmlElement::HtmlElement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_self_closing_element(&self) -> Option<&HtmlSelfClosingElement> {
        match &self {
            AnyHtmlElement::HtmlSelfClosingElement(item) => Some(item),
            _ => None,
        }
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
        f.debug_struct("HtmlAttribute")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "initializer",
                &support::DebugOptionalElement(self.initializer()),
            )
            .finish()
    }
}
impl From<HtmlAttribute> for SyntaxNode {
    fn from(n: HtmlAttribute) -> SyntaxNode {
        n.syntax
    }
}
impl From<HtmlAttribute> for SyntaxElement {
    fn from(n: HtmlAttribute) -> SyntaxElement {
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
        f.debug_struct("HtmlAttributeInitializerClause")
            .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<HtmlAttributeInitializerClause> for SyntaxNode {
    fn from(n: HtmlAttributeInitializerClause) -> SyntaxNode {
        n.syntax
    }
}
impl From<HtmlAttributeInitializerClause> for SyntaxElement {
    fn from(n: HtmlAttributeInitializerClause) -> SyntaxElement {
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
    }
}
impl From<HtmlClosingElement> for SyntaxNode {
    fn from(n: HtmlClosingElement) -> SyntaxNode {
        n.syntax
    }
}
impl From<HtmlClosingElement> for SyntaxElement {
    fn from(n: HtmlClosingElement) -> SyntaxElement {
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
        f.debug_struct("HtmlDirective")
            .field(
                "l_angle_token",
                &support::DebugSyntaxResult(self.l_angle_token()),
            )
            .field("excl_token", &support::DebugSyntaxResult(self.excl_token()))
            .field("content", &support::DebugSyntaxResult(self.content()))
            .field(
                "r_angle_token",
                &support::DebugSyntaxResult(self.r_angle_token()),
            )
            .finish()
    }
}
impl From<HtmlDirective> for SyntaxNode {
    fn from(n: HtmlDirective) -> SyntaxNode {
        n.syntax
    }
}
impl From<HtmlDirective> for SyntaxElement {
    fn from(n: HtmlDirective) -> SyntaxElement {
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
    }
}
impl From<HtmlElement> for SyntaxNode {
    fn from(n: HtmlElement) -> SyntaxNode {
        n.syntax
    }
}
impl From<HtmlElement> for SyntaxElement {
    fn from(n: HtmlElement) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for HtmlName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(HTML_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HTML_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for HtmlName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HtmlName")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<HtmlName> for SyntaxNode {
    fn from(n: HtmlName) -> SyntaxNode {
        n.syntax
    }
}
impl From<HtmlName> for SyntaxElement {
    fn from(n: HtmlName) -> SyntaxElement {
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
    }
}
impl From<HtmlOpeningElement> for SyntaxNode {
    fn from(n: HtmlOpeningElement) -> SyntaxNode {
        n.syntax
    }
}
impl From<HtmlOpeningElement> for SyntaxElement {
    fn from(n: HtmlOpeningElement) -> SyntaxElement {
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
        f.debug_struct("HtmlRoot")
            .field(
                "bom_token",
                &support::DebugOptionalElement(self.bom_token()),
            )
            .field("directive", &support::DebugSyntaxResult(self.directive()))
            .field("tags", &self.tags())
            .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
            .finish()
    }
}
impl From<HtmlRoot> for SyntaxNode {
    fn from(n: HtmlRoot) -> SyntaxNode {
        n.syntax
    }
}
impl From<HtmlRoot> for SyntaxElement {
    fn from(n: HtmlRoot) -> SyntaxElement {
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
        f.debug_struct("HtmlSelfClosingElement")
            .field(
                "l_angle_token",
                &support::DebugSyntaxResult(self.l_angle_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("attributes", &self.attributes())
            .field(
                "slash_token",
                &support::DebugSyntaxResult(self.slash_token()),
            )
            .field(
                "r_angle_token",
                &support::DebugSyntaxResult(self.r_angle_token()),
            )
            .finish()
    }
}
impl From<HtmlSelfClosingElement> for SyntaxNode {
    fn from(n: HtmlSelfClosingElement) -> SyntaxNode {
        n.syntax
    }
}
impl From<HtmlSelfClosingElement> for SyntaxElement {
    fn from(n: HtmlSelfClosingElement) -> SyntaxElement {
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
        f.debug_struct("HtmlString")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<HtmlString> for SyntaxNode {
    fn from(n: HtmlString) -> SyntaxNode {
        n.syntax
    }
}
impl From<HtmlString> for SyntaxElement {
    fn from(n: HtmlString) -> SyntaxElement {
        n.syntax.into()
    }
}
impl From<HtmlElement> for AnyHtmlElement {
    fn from(node: HtmlElement) -> AnyHtmlElement {
        AnyHtmlElement::HtmlElement(node)
    }
}
impl From<HtmlSelfClosingElement> for AnyHtmlElement {
    fn from(node: HtmlSelfClosingElement) -> AnyHtmlElement {
        AnyHtmlElement::HtmlSelfClosingElement(node)
    }
}
impl AstNode for AnyHtmlElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        HtmlElement::KIND_SET.union(HtmlSelfClosingElement::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, HTML_ELEMENT | HTML_SELF_CLOSING_ELEMENT)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            HTML_ELEMENT => AnyHtmlElement::HtmlElement(HtmlElement { syntax }),
            HTML_SELF_CLOSING_ELEMENT => {
                AnyHtmlElement::HtmlSelfClosingElement(HtmlSelfClosingElement { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyHtmlElement::HtmlElement(it) => &it.syntax,
            AnyHtmlElement::HtmlSelfClosingElement(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyHtmlElement::HtmlElement(it) => it.syntax,
            AnyHtmlElement::HtmlSelfClosingElement(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyHtmlElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyHtmlElement::HtmlElement(it) => std::fmt::Debug::fmt(it, f),
            AnyHtmlElement::HtmlSelfClosingElement(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyHtmlElement> for SyntaxNode {
    fn from(n: AnyHtmlElement) -> SyntaxNode {
        match n {
            AnyHtmlElement::HtmlElement(it) => it.into(),
            AnyHtmlElement::HtmlSelfClosingElement(it) => it.into(),
        }
    }
}
impl From<AnyHtmlElement> for SyntaxElement {
    fn from(n: AnyHtmlElement) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyHtmlElement {
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
impl std::fmt::Display for HtmlClosingElement {
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
impl std::fmt::Display for HtmlName {
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
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
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
    fn from(n: HtmlBogus) -> SyntaxNode {
        n.syntax
    }
}
impl From<HtmlBogus> for SyntaxElement {
    fn from(n: HtmlBogus) -> SyntaxElement {
        n.syntax.into()
    }
}
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
    fn cast(syntax: SyntaxNode) -> Option<HtmlAttributeList> {
        if Self::can_cast(syntax.kind()) {
            Some(HtmlAttributeList {
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
#[cfg(feature = "serde")]
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
    type Node = HtmlAttribute;
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
    type Item = HtmlAttribute;
    type IntoIter = AstNodeListIterator<Language, HtmlAttribute>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for HtmlAttributeList {
    type Item = HtmlAttribute;
    type IntoIter = AstNodeListIterator<Language, HtmlAttribute>;
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
    fn cast(syntax: SyntaxNode) -> Option<HtmlElementList> {
        if Self::can_cast(syntax.kind()) {
            Some(HtmlElementList {
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
#[cfg(feature = "serde")]
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
