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
#[cfg(feature = "serde")]
impl Serialize for HtmlContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
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
            html: self.html(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn directive(&self) -> Option<HtmlDirective> {
        support::node(&self.syntax, 1usize)
    }
    pub fn html(&self) -> Option<AnyHtmlElement> {
        support::node(&self.syntax, 2usize)
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
    pub directive: Option<HtmlDirective>,
    pub html: Option<AnyHtmlElement>,
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
            v_dash_token: self.v_dash_token(),
            name: self.name(),
            argument: self.argument(),
            modifier: self.modifier(),
            value: self.value(),
        }
    }
    pub fn v_dash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<HtmlName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn argument(&self) -> SyntaxResult<VueDirectiveArgument> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn modifier(&self) -> VueDirectiveModifierList {
        support::list(&self.syntax, 3usize)
    }
    pub fn value(&self) -> SyntaxResult<VueDirectiveValue> {
        support::required_node(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for VueDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct VueDirectiveFields {
    pub v_dash_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<HtmlName>,
    pub argument: SyntaxResult<VueDirectiveArgument>,
    pub modifier: VueDirectiveModifierList,
    pub value: SyntaxResult<VueDirectiveValue>,
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
            any_vue_directive_argument: self.any_vue_directive_argument(),
        }
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn any_vue_directive_argument(&self) -> SyntaxResult<AnyVueDirectiveArgument> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for VueDirectiveArgument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct VueDirectiveArgumentFields {
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub any_vue_directive_argument: SyntaxResult<AnyVueDirectiveArgument>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VueDirectiveArgumentDynamic {
    pub(crate) syntax: SyntaxNode,
}
impl VueDirectiveArgumentDynamic {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> VueDirectiveArgumentDynamicFields {
        VueDirectiveArgumentDynamicFields {
            l_brack_token: self.l_brack_token(),
            argument: self.argument(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn argument(&self) -> SyntaxResult<HtmlName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for VueDirectiveArgumentDynamic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct VueDirectiveArgumentDynamicFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub argument: SyntaxResult<HtmlName>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VueDirectiveArgumentStatic {
    pub(crate) syntax: SyntaxNode,
}
impl VueDirectiveArgumentStatic {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> VueDirectiveArgumentStaticFields {
        VueDirectiveArgumentStaticFields {
            argument: self.argument(),
        }
    }
    pub fn argument(&self) -> SyntaxResult<HtmlName> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for VueDirectiveArgumentStatic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct VueDirectiveArgumentStaticFields {
    pub argument: SyntaxResult<HtmlName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VueDirectiveModifier {
    pub(crate) syntax: SyntaxNode,
}
impl VueDirectiveModifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> VueDirectiveModifierFields {
        VueDirectiveModifierFields {
            dot_token: self.dot_token(),
            modifier: self.modifier(),
        }
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn modifier(&self) -> SyntaxResult<HtmlName> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for VueDirectiveModifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct VueDirectiveModifierFields {
    pub dot_token: SyntaxResult<SyntaxToken>,
    pub modifier: SyntaxResult<HtmlName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VueDirectiveValue {
    pub(crate) syntax: SyntaxNode,
}
impl VueDirectiveValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> VueDirectiveValueFields {
        VueDirectiveValueFields {
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
impl Serialize for VueDirectiveValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct VueDirectiveValueFields {
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<HtmlString>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VueTemplateInterpolation {
    pub(crate) syntax: SyntaxNode,
}
impl VueTemplateInterpolation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> VueTemplateInterpolationFields {
        VueTemplateInterpolationFields {
            l_double_curly_token: self.l_double_curly_token(),
            value: self.value(),
            r_double_curly_token: self.r_double_curly_token(),
        }
    }
    pub fn l_double_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<HtmlContent> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_double_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for VueTemplateInterpolation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct VueTemplateInterpolationFields {
    pub l_double_curly_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<HtmlContent>,
    pub r_double_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VueVBindShorthand {
    pub(crate) syntax: SyntaxNode,
}
impl VueVBindShorthand {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> VueVBindShorthandFields {
        VueVBindShorthandFields {
            colon_token: self.colon_token(),
            argument: self.argument(),
            modifier: self.modifier(),
            value: self.value(),
        }
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn argument(&self) -> SyntaxResult<VueDirectiveArgument> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn modifier(&self) -> VueDirectiveModifierList {
        support::list(&self.syntax, 2usize)
    }
    pub fn value(&self) -> SyntaxResult<VueDirectiveValue> {
        support::required_node(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for VueVBindShorthand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct VueVBindShorthandFields {
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub argument: SyntaxResult<VueDirectiveArgument>,
    pub modifier: VueDirectiveModifierList,
    pub value: SyntaxResult<VueDirectiveValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VueVOnShorthand {
    pub(crate) syntax: SyntaxNode,
}
impl VueVOnShorthand {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> VueVOnShorthandFields {
        VueVOnShorthandFields {
            at_token: self.at_token(),
            argument: self.argument(),
            modifier: self.modifier(),
            value: self.value(),
        }
    }
    pub fn at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn argument(&self) -> SyntaxResult<VueDirectiveArgument> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn modifier(&self) -> VueDirectiveModifierList {
        support::list(&self.syntax, 2usize)
    }
    pub fn value(&self) -> SyntaxResult<VueDirectiveValue> {
        support::required_node(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for VueVOnShorthand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct VueVOnShorthandFields {
    pub at_token: SyntaxResult<SyntaxToken>,
    pub argument: SyntaxResult<VueDirectiveArgument>,
    pub modifier: VueDirectiveModifierList,
    pub value: SyntaxResult<VueDirectiveValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyHtmlAttribute {
    AnyVueDirective(AnyVueDirective),
    HtmlAttribute(HtmlAttribute),
    HtmlBogusAttribute(HtmlBogusAttribute),
}
impl AnyHtmlAttribute {
    pub fn as_any_vue_directive(&self) -> Option<&AnyVueDirective> {
        match &self {
            AnyHtmlAttribute::AnyVueDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_attribute(&self) -> Option<&HtmlAttribute> {
        match &self {
            AnyHtmlAttribute::HtmlAttribute(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_bogus_attribute(&self) -> Option<&HtmlBogusAttribute> {
        match &self {
            AnyHtmlAttribute::HtmlBogusAttribute(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyHtmlElement {
    HtmlBogusElement(HtmlBogusElement),
    HtmlContent(HtmlContent),
    HtmlElement(HtmlElement),
    HtmlSelfClosingElement(HtmlSelfClosingElement),
    VueTemplateInterpolation(VueTemplateInterpolation),
}
impl AnyHtmlElement {
    pub fn as_html_bogus_element(&self) -> Option<&HtmlBogusElement> {
        match &self {
            AnyHtmlElement::HtmlBogusElement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_html_content(&self) -> Option<&HtmlContent> {
        match &self {
            AnyHtmlElement::HtmlContent(item) => Some(item),
            _ => None,
        }
    }
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
    pub fn as_vue_template_interpolation(&self) -> Option<&VueTemplateInterpolation> {
        match &self {
            AnyHtmlElement::VueTemplateInterpolation(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyVueDirective {
    VueDirective(VueDirective),
    VueVBindShorthand(VueVBindShorthand),
    VueVOnShorthand(VueVOnShorthand),
}
impl AnyVueDirective {
    pub fn as_vue_directive(&self) -> Option<&VueDirective> {
        match &self {
            AnyVueDirective::VueDirective(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_vue_v_bind_shorthand(&self) -> Option<&VueVBindShorthand> {
        match &self {
            AnyVueDirective::VueVBindShorthand(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_vue_v_on_shorthand(&self) -> Option<&VueVOnShorthand> {
        match &self {
            AnyVueDirective::VueVOnShorthand(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyVueDirectiveArgument {
    VueDirectiveArgumentDynamic(VueDirectiveArgumentDynamic),
    VueDirectiveArgumentStatic(VueDirectiveArgumentStatic),
}
impl AnyVueDirectiveArgument {
    pub fn as_vue_directive_argument_dynamic(&self) -> Option<&VueDirectiveArgumentDynamic> {
        match &self {
            AnyVueDirectiveArgument::VueDirectiveArgumentDynamic(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_vue_directive_argument_static(&self) -> Option<&VueDirectiveArgumentStatic> {
        match &self {
            AnyVueDirectiveArgument::VueDirectiveArgumentStatic(item) => Some(item),
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
        f.debug_struct("HtmlContent")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<HtmlContent> for SyntaxNode {
    fn from(n: HtmlContent) -> SyntaxNode {
        n.syntax
    }
}
impl From<HtmlContent> for SyntaxElement {
    fn from(n: HtmlContent) -> SyntaxElement {
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
            .field(
                "directive",
                &support::DebugOptionalElement(self.directive()),
            )
            .field("html", &support::DebugOptionalElement(self.html()))
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
        f.debug_struct("VueDirective")
            .field(
                "v_dash_token",
                &support::DebugSyntaxResult(self.v_dash_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("argument", &support::DebugSyntaxResult(self.argument()))
            .field("modifier", &self.modifier())
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<VueDirective> for SyntaxNode {
    fn from(n: VueDirective) -> SyntaxNode {
        n.syntax
    }
}
impl From<VueDirective> for SyntaxElement {
    fn from(n: VueDirective) -> SyntaxElement {
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
        f.debug_struct("VueDirectiveArgument")
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field(
                "any_vue_directive_argument",
                &support::DebugSyntaxResult(self.any_vue_directive_argument()),
            )
            .finish()
    }
}
impl From<VueDirectiveArgument> for SyntaxNode {
    fn from(n: VueDirectiveArgument) -> SyntaxNode {
        n.syntax
    }
}
impl From<VueDirectiveArgument> for SyntaxElement {
    fn from(n: VueDirectiveArgument) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for VueDirectiveArgumentDynamic {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_DIRECTIVE_ARGUMENT_DYNAMIC as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_DIRECTIVE_ARGUMENT_DYNAMIC
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for VueDirectiveArgumentDynamic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VueDirectiveArgumentDynamic")
            .field(
                "l_brack_token",
                &support::DebugSyntaxResult(self.l_brack_token()),
            )
            .field("argument", &support::DebugSyntaxResult(self.argument()))
            .field(
                "r_brack_token",
                &support::DebugSyntaxResult(self.r_brack_token()),
            )
            .finish()
    }
}
impl From<VueDirectiveArgumentDynamic> for SyntaxNode {
    fn from(n: VueDirectiveArgumentDynamic) -> SyntaxNode {
        n.syntax
    }
}
impl From<VueDirectiveArgumentDynamic> for SyntaxElement {
    fn from(n: VueDirectiveArgumentDynamic) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for VueDirectiveArgumentStatic {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_DIRECTIVE_ARGUMENT_STATIC as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_DIRECTIVE_ARGUMENT_STATIC
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for VueDirectiveArgumentStatic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VueDirectiveArgumentStatic")
            .field("argument", &support::DebugSyntaxResult(self.argument()))
            .finish()
    }
}
impl From<VueDirectiveArgumentStatic> for SyntaxNode {
    fn from(n: VueDirectiveArgumentStatic) -> SyntaxNode {
        n.syntax
    }
}
impl From<VueDirectiveArgumentStatic> for SyntaxElement {
    fn from(n: VueDirectiveArgumentStatic) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for VueDirectiveModifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_DIRECTIVE_MODIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_DIRECTIVE_MODIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for VueDirectiveModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VueDirectiveModifier")
            .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
            .field("modifier", &support::DebugSyntaxResult(self.modifier()))
            .finish()
    }
}
impl From<VueDirectiveModifier> for SyntaxNode {
    fn from(n: VueDirectiveModifier) -> SyntaxNode {
        n.syntax
    }
}
impl From<VueDirectiveModifier> for SyntaxElement {
    fn from(n: VueDirectiveModifier) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for VueDirectiveValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_DIRECTIVE_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_DIRECTIVE_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for VueDirectiveValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VueDirectiveValue")
            .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<VueDirectiveValue> for SyntaxNode {
    fn from(n: VueDirectiveValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<VueDirectiveValue> for SyntaxElement {
    fn from(n: VueDirectiveValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for VueTemplateInterpolation {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_TEMPLATE_INTERPOLATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_TEMPLATE_INTERPOLATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for VueTemplateInterpolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VueTemplateInterpolation")
            .field(
                "l_double_curly_token",
                &support::DebugSyntaxResult(self.l_double_curly_token()),
            )
            .field("value", &support::DebugSyntaxResult(self.value()))
            .field(
                "r_double_curly_token",
                &support::DebugSyntaxResult(self.r_double_curly_token()),
            )
            .finish()
    }
}
impl From<VueTemplateInterpolation> for SyntaxNode {
    fn from(n: VueTemplateInterpolation) -> SyntaxNode {
        n.syntax
    }
}
impl From<VueTemplateInterpolation> for SyntaxElement {
    fn from(n: VueTemplateInterpolation) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for VueVBindShorthand {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_V_BIND_SHORTHAND as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_V_BIND_SHORTHAND
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for VueVBindShorthand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VueVBindShorthand")
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("argument", &support::DebugSyntaxResult(self.argument()))
            .field("modifier", &self.modifier())
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<VueVBindShorthand> for SyntaxNode {
    fn from(n: VueVBindShorthand) -> SyntaxNode {
        n.syntax
    }
}
impl From<VueVBindShorthand> for SyntaxElement {
    fn from(n: VueVBindShorthand) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for VueVOnShorthand {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_V_ON_SHORTHAND as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_V_ON_SHORTHAND
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for VueVOnShorthand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VueVOnShorthand")
            .field("at_token", &support::DebugSyntaxResult(self.at_token()))
            .field("argument", &support::DebugSyntaxResult(self.argument()))
            .field("modifier", &self.modifier())
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<VueVOnShorthand> for SyntaxNode {
    fn from(n: VueVOnShorthand) -> SyntaxNode {
        n.syntax
    }
}
impl From<VueVOnShorthand> for SyntaxElement {
    fn from(n: VueVOnShorthand) -> SyntaxElement {
        n.syntax.into()
    }
}
impl From<HtmlAttribute> for AnyHtmlAttribute {
    fn from(node: HtmlAttribute) -> AnyHtmlAttribute {
        AnyHtmlAttribute::HtmlAttribute(node)
    }
}
impl From<HtmlBogusAttribute> for AnyHtmlAttribute {
    fn from(node: HtmlBogusAttribute) -> AnyHtmlAttribute {
        AnyHtmlAttribute::HtmlBogusAttribute(node)
    }
}
impl AstNode for AnyHtmlAttribute {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyVueDirective::KIND_SET
        .union(HtmlAttribute::KIND_SET)
        .union(HtmlBogusAttribute::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            HTML_ATTRIBUTE | HTML_BOGUS_ATTRIBUTE => true,
            k if AnyVueDirective::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            HTML_ATTRIBUTE => AnyHtmlAttribute::HtmlAttribute(HtmlAttribute { syntax }),
            HTML_BOGUS_ATTRIBUTE => {
                AnyHtmlAttribute::HtmlBogusAttribute(HtmlBogusAttribute { syntax })
            }
            _ => {
                if let Some(any_vue_directive) = AnyVueDirective::cast(syntax) {
                    return Some(AnyHtmlAttribute::AnyVueDirective(any_vue_directive));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyHtmlAttribute::HtmlAttribute(it) => &it.syntax,
            AnyHtmlAttribute::HtmlBogusAttribute(it) => &it.syntax,
            AnyHtmlAttribute::AnyVueDirective(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyHtmlAttribute::HtmlAttribute(it) => it.syntax,
            AnyHtmlAttribute::HtmlBogusAttribute(it) => it.syntax,
            AnyHtmlAttribute::AnyVueDirective(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyHtmlAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyHtmlAttribute::AnyVueDirective(it) => std::fmt::Debug::fmt(it, f),
            AnyHtmlAttribute::HtmlAttribute(it) => std::fmt::Debug::fmt(it, f),
            AnyHtmlAttribute::HtmlBogusAttribute(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyHtmlAttribute> for SyntaxNode {
    fn from(n: AnyHtmlAttribute) -> SyntaxNode {
        match n {
            AnyHtmlAttribute::AnyVueDirective(it) => it.into(),
            AnyHtmlAttribute::HtmlAttribute(it) => it.into(),
            AnyHtmlAttribute::HtmlBogusAttribute(it) => it.into(),
        }
    }
}
impl From<AnyHtmlAttribute> for SyntaxElement {
    fn from(n: AnyHtmlAttribute) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<HtmlBogusElement> for AnyHtmlElement {
    fn from(node: HtmlBogusElement) -> AnyHtmlElement {
        AnyHtmlElement::HtmlBogusElement(node)
    }
}
impl From<HtmlContent> for AnyHtmlElement {
    fn from(node: HtmlContent) -> AnyHtmlElement {
        AnyHtmlElement::HtmlContent(node)
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
impl From<VueTemplateInterpolation> for AnyHtmlElement {
    fn from(node: VueTemplateInterpolation) -> AnyHtmlElement {
        AnyHtmlElement::VueTemplateInterpolation(node)
    }
}
impl AstNode for AnyHtmlElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = HtmlBogusElement::KIND_SET
        .union(HtmlContent::KIND_SET)
        .union(HtmlElement::KIND_SET)
        .union(HtmlSelfClosingElement::KIND_SET)
        .union(VueTemplateInterpolation::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            HTML_BOGUS_ELEMENT
                | HTML_CONTENT
                | HTML_ELEMENT
                | HTML_SELF_CLOSING_ELEMENT
                | VUE_TEMPLATE_INTERPOLATION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            HTML_BOGUS_ELEMENT => AnyHtmlElement::HtmlBogusElement(HtmlBogusElement { syntax }),
            HTML_CONTENT => AnyHtmlElement::HtmlContent(HtmlContent { syntax }),
            HTML_ELEMENT => AnyHtmlElement::HtmlElement(HtmlElement { syntax }),
            HTML_SELF_CLOSING_ELEMENT => {
                AnyHtmlElement::HtmlSelfClosingElement(HtmlSelfClosingElement { syntax })
            }
            VUE_TEMPLATE_INTERPOLATION => {
                AnyHtmlElement::VueTemplateInterpolation(VueTemplateInterpolation { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyHtmlElement::HtmlBogusElement(it) => &it.syntax,
            AnyHtmlElement::HtmlContent(it) => &it.syntax,
            AnyHtmlElement::HtmlElement(it) => &it.syntax,
            AnyHtmlElement::HtmlSelfClosingElement(it) => &it.syntax,
            AnyHtmlElement::VueTemplateInterpolation(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyHtmlElement::HtmlBogusElement(it) => it.syntax,
            AnyHtmlElement::HtmlContent(it) => it.syntax,
            AnyHtmlElement::HtmlElement(it) => it.syntax,
            AnyHtmlElement::HtmlSelfClosingElement(it) => it.syntax,
            AnyHtmlElement::VueTemplateInterpolation(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyHtmlElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyHtmlElement::HtmlBogusElement(it) => std::fmt::Debug::fmt(it, f),
            AnyHtmlElement::HtmlContent(it) => std::fmt::Debug::fmt(it, f),
            AnyHtmlElement::HtmlElement(it) => std::fmt::Debug::fmt(it, f),
            AnyHtmlElement::HtmlSelfClosingElement(it) => std::fmt::Debug::fmt(it, f),
            AnyHtmlElement::VueTemplateInterpolation(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyHtmlElement> for SyntaxNode {
    fn from(n: AnyHtmlElement) -> SyntaxNode {
        match n {
            AnyHtmlElement::HtmlBogusElement(it) => it.into(),
            AnyHtmlElement::HtmlContent(it) => it.into(),
            AnyHtmlElement::HtmlElement(it) => it.into(),
            AnyHtmlElement::HtmlSelfClosingElement(it) => it.into(),
            AnyHtmlElement::VueTemplateInterpolation(it) => it.into(),
        }
    }
}
impl From<AnyHtmlElement> for SyntaxElement {
    fn from(n: AnyHtmlElement) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<VueDirective> for AnyVueDirective {
    fn from(node: VueDirective) -> AnyVueDirective {
        AnyVueDirective::VueDirective(node)
    }
}
impl From<VueVBindShorthand> for AnyVueDirective {
    fn from(node: VueVBindShorthand) -> AnyVueDirective {
        AnyVueDirective::VueVBindShorthand(node)
    }
}
impl From<VueVOnShorthand> for AnyVueDirective {
    fn from(node: VueVOnShorthand) -> AnyVueDirective {
        AnyVueDirective::VueVOnShorthand(node)
    }
}
impl AstNode for AnyVueDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = VueDirective::KIND_SET
        .union(VueVBindShorthand::KIND_SET)
        .union(VueVOnShorthand::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            VUE_DIRECTIVE | VUE_V_BIND_SHORTHAND | VUE_V_ON_SHORTHAND
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            VUE_DIRECTIVE => AnyVueDirective::VueDirective(VueDirective { syntax }),
            VUE_V_BIND_SHORTHAND => {
                AnyVueDirective::VueVBindShorthand(VueVBindShorthand { syntax })
            }
            VUE_V_ON_SHORTHAND => AnyVueDirective::VueVOnShorthand(VueVOnShorthand { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyVueDirective::VueDirective(it) => &it.syntax,
            AnyVueDirective::VueVBindShorthand(it) => &it.syntax,
            AnyVueDirective::VueVOnShorthand(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyVueDirective::VueDirective(it) => it.syntax,
            AnyVueDirective::VueVBindShorthand(it) => it.syntax,
            AnyVueDirective::VueVOnShorthand(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyVueDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyVueDirective::VueDirective(it) => std::fmt::Debug::fmt(it, f),
            AnyVueDirective::VueVBindShorthand(it) => std::fmt::Debug::fmt(it, f),
            AnyVueDirective::VueVOnShorthand(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyVueDirective> for SyntaxNode {
    fn from(n: AnyVueDirective) -> SyntaxNode {
        match n {
            AnyVueDirective::VueDirective(it) => it.into(),
            AnyVueDirective::VueVBindShorthand(it) => it.into(),
            AnyVueDirective::VueVOnShorthand(it) => it.into(),
        }
    }
}
impl From<AnyVueDirective> for SyntaxElement {
    fn from(n: AnyVueDirective) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<VueDirectiveArgumentDynamic> for AnyVueDirectiveArgument {
    fn from(node: VueDirectiveArgumentDynamic) -> AnyVueDirectiveArgument {
        AnyVueDirectiveArgument::VueDirectiveArgumentDynamic(node)
    }
}
impl From<VueDirectiveArgumentStatic> for AnyVueDirectiveArgument {
    fn from(node: VueDirectiveArgumentStatic) -> AnyVueDirectiveArgument {
        AnyVueDirectiveArgument::VueDirectiveArgumentStatic(node)
    }
}
impl AstNode for AnyVueDirectiveArgument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        VueDirectiveArgumentDynamic::KIND_SET.union(VueDirectiveArgumentStatic::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            VUE_DIRECTIVE_ARGUMENT_DYNAMIC | VUE_DIRECTIVE_ARGUMENT_STATIC
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            VUE_DIRECTIVE_ARGUMENT_DYNAMIC => {
                AnyVueDirectiveArgument::VueDirectiveArgumentDynamic(VueDirectiveArgumentDynamic {
                    syntax,
                })
            }
            VUE_DIRECTIVE_ARGUMENT_STATIC => {
                AnyVueDirectiveArgument::VueDirectiveArgumentStatic(VueDirectiveArgumentStatic {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyVueDirectiveArgument::VueDirectiveArgumentDynamic(it) => &it.syntax,
            AnyVueDirectiveArgument::VueDirectiveArgumentStatic(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyVueDirectiveArgument::VueDirectiveArgumentDynamic(it) => it.syntax,
            AnyVueDirectiveArgument::VueDirectiveArgumentStatic(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyVueDirectiveArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyVueDirectiveArgument::VueDirectiveArgumentDynamic(it) => std::fmt::Debug::fmt(it, f),
            AnyVueDirectiveArgument::VueDirectiveArgumentStatic(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyVueDirectiveArgument> for SyntaxNode {
    fn from(n: AnyVueDirectiveArgument) -> SyntaxNode {
        match n {
            AnyVueDirectiveArgument::VueDirectiveArgumentDynamic(it) => it.into(),
            AnyVueDirectiveArgument::VueDirectiveArgumentStatic(it) => it.into(),
        }
    }
}
impl From<AnyVueDirectiveArgument> for SyntaxElement {
    fn from(n: AnyVueDirectiveArgument) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyHtmlAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyHtmlElement {
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
impl std::fmt::Display for VueDirectiveArgumentDynamic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VueDirectiveArgumentStatic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VueDirectiveModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VueDirectiveValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VueTemplateInterpolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VueVBindShorthand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VueVOnShorthand {
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
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
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
    fn from(n: HtmlBogusAttribute) -> SyntaxNode {
        n.syntax
    }
}
impl From<HtmlBogusAttribute> for SyntaxElement {
    fn from(n: HtmlBogusAttribute) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
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
    fn from(n: HtmlBogusElement) -> SyntaxNode {
        n.syntax
    }
}
impl From<HtmlBogusElement> for SyntaxElement {
    fn from(n: HtmlBogusElement) -> SyntaxElement {
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
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct VueDirectiveModifierList {
    syntax_list: SyntaxList,
}
impl VueDirectiveModifierList {
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
impl AstNode for VueDirectiveModifierList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VUE_DIRECTIVE_MODIFIER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VUE_DIRECTIVE_MODIFIER_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<VueDirectiveModifierList> {
        if Self::can_cast(syntax.kind()) {
            Some(VueDirectiveModifierList {
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
impl Serialize for VueDirectiveModifierList {
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
impl AstNodeList for VueDirectiveModifierList {
    type Language = Language;
    type Node = VueDirectiveModifier;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for VueDirectiveModifierList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("VueDirectiveModifierList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &VueDirectiveModifierList {
    type Item = VueDirectiveModifier;
    type IntoIter = AstNodeListIterator<Language, VueDirectiveModifier>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for VueDirectiveModifierList {
    type Item = VueDirectiveModifier;
    type IntoIter = AstNodeListIterator<Language, VueDirectiveModifier>;
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
