//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    macros::map_syntax_node,
    GraphqlLanguage as Language, GraphqlSyntaxElement as SyntaxElement,
    GraphqlSyntaxElementChildren as SyntaxElementChildren,
    GraphqlSyntaxKind::{self as SyntaxKind, *},
    GraphqlSyntaxList as SyntaxList, GraphqlSyntaxNode as SyntaxNode,
    GraphqlSyntaxToken as SyntaxToken,
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
pub struct GraphqlAlias {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlAlias {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlAliasFields {
        GraphqlAliasFields {
            value: self.value(),
            colon_token: self.colon_token(),
        }
    }
    pub fn value(&self) -> SyntaxResult<GraphqlLiteralName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for GraphqlAlias {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlAliasFields {
    pub value: SyntaxResult<GraphqlLiteralName>,
    pub colon_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlArgument {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlArgument {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlArgumentFields {
        GraphqlArgumentFields {
            name: self.name(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn name(&self) -> SyntaxResult<GraphqlLiteralName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyGraphqlValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlArgument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlArgumentFields {
    pub name: SyntaxResult<GraphqlLiteralName>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyGraphqlValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlArguments {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlArguments {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlArgumentsFields {
        GraphqlArgumentsFields {
            l_paren_token: self.l_paren_token(),
            arguments: self.arguments(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn arguments(&self) -> GraphqlArgumentList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlArguments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlArgumentsFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub arguments: GraphqlArgumentList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlArgumentsDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlArgumentsDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlArgumentsDefinitionFields {
        GraphqlArgumentsDefinitionFields {
            l_paren_token: self.l_paren_token(),
            arguments: self.arguments(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn arguments(&self) -> GraphqlArgumentDefinitionList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlArgumentsDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlArgumentsDefinitionFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub arguments: GraphqlArgumentDefinitionList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlBooleanValue {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlBooleanValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlBooleanValueFields {
        GraphqlBooleanValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GraphqlBooleanValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlBooleanValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlDefaultValue {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlDefaultValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlDefaultValueFields {
        GraphqlDefaultValueFields {
            eq_token: self.eq_token(),
            value: self.value(),
        }
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyGraphqlValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for GraphqlDefaultValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlDefaultValueFields {
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyGraphqlValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlDescription {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlDescription {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlDescriptionFields {
        GraphqlDescriptionFields {
            graphql_string_value: self.graphql_string_value(),
        }
    }
    pub fn graphql_string_value(&self) -> SyntaxResult<GraphqlStringValue> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for GraphqlDescription {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlDescriptionFields {
    pub graphql_string_value: SyntaxResult<GraphqlStringValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlDirective {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlDirectiveFields {
        GraphqlDirectiveFields {
            at_token: self.at_token(),
            name: self.name(),
            arguments: self.arguments(),
        }
    }
    pub fn at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlNameReference> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn arguments(&self) -> Option<GraphqlArguments> {
        support::node(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlDirectiveFields {
    pub at_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlNameReference>,
    pub arguments: Option<GraphqlArguments>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlDirectiveDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlDirectiveDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlDirectiveDefinitionFields {
        GraphqlDirectiveDefinitionFields {
            description: self.description(),
            directive_token: self.directive_token(),
            at_token: self.at_token(),
            name: self.name(),
            arguments: self.arguments(),
            repeatable_token: self.repeatable_token(),
            on_token: self.on_token(),
            bitwise_or_token: self.bitwise_or_token(),
            locations: self.locations(),
        }
    }
    pub fn description(&self) -> Option<GraphqlDescription> {
        support::node(&self.syntax, 0usize)
    }
    pub fn directive_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlNameBinding> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn arguments(&self) -> Option<GraphqlArgumentsDefinition> {
        support::node(&self.syntax, 4usize)
    }
    pub fn repeatable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 5usize)
    }
    pub fn on_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 6usize)
    }
    pub fn bitwise_or_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 7usize)
    }
    pub fn locations(&self) -> GraphqlDirectiveLocationList {
        support::list(&self.syntax, 8usize)
    }
}
impl Serialize for GraphqlDirectiveDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlDirectiveDefinitionFields {
    pub description: Option<GraphqlDescription>,
    pub directive_token: SyntaxResult<SyntaxToken>,
    pub at_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlNameBinding>,
    pub arguments: Option<GraphqlArgumentsDefinition>,
    pub repeatable_token: Option<SyntaxToken>,
    pub on_token: SyntaxResult<SyntaxToken>,
    pub bitwise_or_token: Option<SyntaxToken>,
    pub locations: GraphqlDirectiveLocationList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlDirectiveLocation {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlDirectiveLocation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlDirectiveLocationFields {
        GraphqlDirectiveLocationFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GraphqlDirectiveLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlDirectiveLocationFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlEnumTypeDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlEnumTypeDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlEnumTypeDefinitionFields {
        GraphqlEnumTypeDefinitionFields {
            description: self.description(),
            enum_token: self.enum_token(),
            name: self.name(),
            directives: self.directives(),
            enum_values: self.enum_values(),
        }
    }
    pub fn description(&self) -> Option<GraphqlDescription> {
        support::node(&self.syntax, 0usize)
    }
    pub fn enum_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlNameBinding> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 3usize)
    }
    pub fn enum_values(&self) -> Option<GraphqlEnumValuesDefinition> {
        support::node(&self.syntax, 4usize)
    }
}
impl Serialize for GraphqlEnumTypeDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlEnumTypeDefinitionFields {
    pub description: Option<GraphqlDescription>,
    pub enum_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlNameBinding>,
    pub directives: GraphqlDirectiveList,
    pub enum_values: Option<GraphqlEnumValuesDefinition>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlEnumTypeExtension {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlEnumTypeExtension {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlEnumTypeExtensionFields {
        GraphqlEnumTypeExtensionFields {
            extend_token: self.extend_token(),
            enum_token: self.enum_token(),
            name: self.name(),
            directives: self.directives(),
            enum_values: self.enum_values(),
        }
    }
    pub fn extend_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn enum_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlNameReference> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 3usize)
    }
    pub fn enum_values(&self) -> Option<GraphqlEnumValuesDefinition> {
        support::node(&self.syntax, 4usize)
    }
}
impl Serialize for GraphqlEnumTypeExtension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlEnumTypeExtensionFields {
    pub extend_token: SyntaxResult<SyntaxToken>,
    pub enum_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlNameReference>,
    pub directives: GraphqlDirectiveList,
    pub enum_values: Option<GraphqlEnumValuesDefinition>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlEnumValue {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlEnumValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlEnumValueFields {
        GraphqlEnumValueFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<GraphqlLiteralName> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for GraphqlEnumValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlEnumValueFields {
    pub value: SyntaxResult<GraphqlLiteralName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlEnumValueDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlEnumValueDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlEnumValueDefinitionFields {
        GraphqlEnumValueDefinitionFields {
            description: self.description(),
            value: self.value(),
            directives: self.directives(),
        }
    }
    pub fn description(&self) -> Option<GraphqlDescription> {
        support::node(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<GraphqlLiteralName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlEnumValueDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlEnumValueDefinitionFields {
    pub description: Option<GraphqlDescription>,
    pub value: SyntaxResult<GraphqlLiteralName>,
    pub directives: GraphqlDirectiveList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlEnumValuesDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlEnumValuesDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlEnumValuesDefinitionFields {
        GraphqlEnumValuesDefinitionFields {
            l_curly_token: self.l_curly_token(),
            values: self.values(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn values(&self) -> GraphqlEnumValueList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlEnumValuesDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlEnumValuesDefinitionFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub values: GraphqlEnumValueList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlField {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlField {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlFieldFields {
        GraphqlFieldFields {
            alias: self.alias(),
            name: self.name(),
            arguments: self.arguments(),
            directives: self.directives(),
            selection_set: self.selection_set(),
        }
    }
    pub fn alias(&self) -> Option<GraphqlAlias> {
        support::node(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlLiteralName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn arguments(&self) -> Option<GraphqlArguments> {
        support::node(&self.syntax, 2usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 3usize)
    }
    pub fn selection_set(&self) -> Option<GraphqlSelectionSet> {
        support::node(&self.syntax, 4usize)
    }
}
impl Serialize for GraphqlField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlFieldFields {
    pub alias: Option<GraphqlAlias>,
    pub name: SyntaxResult<GraphqlLiteralName>,
    pub arguments: Option<GraphqlArguments>,
    pub directives: GraphqlDirectiveList,
    pub selection_set: Option<GraphqlSelectionSet>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlFieldDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlFieldDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlFieldDefinitionFields {
        GraphqlFieldDefinitionFields {
            description: self.description(),
            name: self.name(),
            arguments: self.arguments(),
            colon_token: self.colon_token(),
            ty: self.ty(),
            directives: self.directives(),
        }
    }
    pub fn description(&self) -> Option<GraphqlDescription> {
        support::node(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlLiteralName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn arguments(&self) -> Option<GraphqlArgumentsDefinition> {
        support::node(&self.syntax, 2usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn ty(&self) -> SyntaxResult<AnyGraphqlType> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 5usize)
    }
}
impl Serialize for GraphqlFieldDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlFieldDefinitionFields {
    pub description: Option<GraphqlDescription>,
    pub name: SyntaxResult<GraphqlLiteralName>,
    pub arguments: Option<GraphqlArgumentsDefinition>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub ty: SyntaxResult<AnyGraphqlType>,
    pub directives: GraphqlDirectiveList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlFieldsDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlFieldsDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlFieldsDefinitionFields {
        GraphqlFieldsDefinitionFields {
            l_curly_token: self.l_curly_token(),
            fields: self.fields(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn fields(&self) -> GraphqlFieldDefinitionList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlFieldsDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlFieldsDefinitionFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub fields: GraphqlFieldDefinitionList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlFloatValue {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlFloatValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlFloatValueFields {
        GraphqlFloatValueFields {
            graphql_float_literal_token: self.graphql_float_literal_token(),
        }
    }
    pub fn graphql_float_literal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GraphqlFloatValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlFloatValueFields {
    pub graphql_float_literal_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlFragmentDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlFragmentDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlFragmentDefinitionFields {
        GraphqlFragmentDefinitionFields {
            fragment_token: self.fragment_token(),
            name: self.name(),
            type_condition: self.type_condition(),
            directives: self.directives(),
            selection_set: self.selection_set(),
        }
    }
    pub fn fragment_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlNameBinding> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn type_condition(&self) -> SyntaxResult<GraphqlTypeCondition> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 3usize)
    }
    pub fn selection_set(&self) -> SyntaxResult<GraphqlSelectionSet> {
        support::required_node(&self.syntax, 4usize)
    }
}
impl Serialize for GraphqlFragmentDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlFragmentDefinitionFields {
    pub fragment_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlNameBinding>,
    pub type_condition: SyntaxResult<GraphqlTypeCondition>,
    pub directives: GraphqlDirectiveList,
    pub selection_set: SyntaxResult<GraphqlSelectionSet>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlFragmentSpread {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlFragmentSpread {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlFragmentSpreadFields {
        GraphqlFragmentSpreadFields {
            dotdotdot_token: self.dotdotdot_token(),
            name: self.name(),
            directives: self.directives(),
        }
    }
    pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlNameReference> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlFragmentSpread {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlFragmentSpreadFields {
    pub dotdotdot_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlNameReference>,
    pub directives: GraphqlDirectiveList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlImplementsInterfaces {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlImplementsInterfaces {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlImplementsInterfacesFields {
        GraphqlImplementsInterfacesFields {
            implements_token: self.implements_token(),
            amp_token: self.amp_token(),
            interfaces: self.interfaces(),
        }
    }
    pub fn implements_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn amp_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn interfaces(&self) -> GraphqlImplementsInterfaceList {
        support::list(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlImplementsInterfaces {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlImplementsInterfacesFields {
    pub implements_token: SyntaxResult<SyntaxToken>,
    pub amp_token: Option<SyntaxToken>,
    pub interfaces: GraphqlImplementsInterfaceList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlInlineFragment {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlInlineFragment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlInlineFragmentFields {
        GraphqlInlineFragmentFields {
            dotdotdot_token: self.dotdotdot_token(),
            type_condition: self.type_condition(),
            directives: self.directives(),
            selection_set: self.selection_set(),
        }
    }
    pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn type_condition(&self) -> Option<GraphqlTypeCondition> {
        support::node(&self.syntax, 1usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 2usize)
    }
    pub fn selection_set(&self) -> SyntaxResult<GraphqlSelectionSet> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl Serialize for GraphqlInlineFragment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlInlineFragmentFields {
    pub dotdotdot_token: SyntaxResult<SyntaxToken>,
    pub type_condition: Option<GraphqlTypeCondition>,
    pub directives: GraphqlDirectiveList,
    pub selection_set: SyntaxResult<GraphqlSelectionSet>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlInputFieldsDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlInputFieldsDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlInputFieldsDefinitionFields {
        GraphqlInputFieldsDefinitionFields {
            l_curly_token: self.l_curly_token(),
            fields: self.fields(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn fields(&self) -> GraphqlInputFieldList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlInputFieldsDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlInputFieldsDefinitionFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub fields: GraphqlInputFieldList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlInputObjectTypeDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlInputObjectTypeDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlInputObjectTypeDefinitionFields {
        GraphqlInputObjectTypeDefinitionFields {
            description: self.description(),
            input_token: self.input_token(),
            name: self.name(),
            directives: self.directives(),
            input_fields: self.input_fields(),
        }
    }
    pub fn description(&self) -> Option<GraphqlDescription> {
        support::node(&self.syntax, 0usize)
    }
    pub fn input_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlNameBinding> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 3usize)
    }
    pub fn input_fields(&self) -> Option<GraphqlInputFieldsDefinition> {
        support::node(&self.syntax, 4usize)
    }
}
impl Serialize for GraphqlInputObjectTypeDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlInputObjectTypeDefinitionFields {
    pub description: Option<GraphqlDescription>,
    pub input_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlNameBinding>,
    pub directives: GraphqlDirectiveList,
    pub input_fields: Option<GraphqlInputFieldsDefinition>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlInputObjectTypeExtension {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlInputObjectTypeExtension {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlInputObjectTypeExtensionFields {
        GraphqlInputObjectTypeExtensionFields {
            extend_token: self.extend_token(),
            input_token: self.input_token(),
            name: self.name(),
            directives: self.directives(),
            input_fields: self.input_fields(),
        }
    }
    pub fn extend_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn input_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlNameReference> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 3usize)
    }
    pub fn input_fields(&self) -> Option<GraphqlInputFieldsDefinition> {
        support::node(&self.syntax, 4usize)
    }
}
impl Serialize for GraphqlInputObjectTypeExtension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlInputObjectTypeExtensionFields {
    pub extend_token: SyntaxResult<SyntaxToken>,
    pub input_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlNameReference>,
    pub directives: GraphqlDirectiveList,
    pub input_fields: Option<GraphqlInputFieldsDefinition>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlInputValueDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlInputValueDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlInputValueDefinitionFields {
        GraphqlInputValueDefinitionFields {
            description: self.description(),
            name: self.name(),
            colon_token: self.colon_token(),
            ty: self.ty(),
            default: self.default(),
            directives: self.directives(),
        }
    }
    pub fn description(&self) -> Option<GraphqlDescription> {
        support::node(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlLiteralName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn ty(&self) -> SyntaxResult<AnyGraphqlType> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn default(&self) -> Option<GraphqlDefaultValue> {
        support::node(&self.syntax, 4usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 5usize)
    }
}
impl Serialize for GraphqlInputValueDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlInputValueDefinitionFields {
    pub description: Option<GraphqlDescription>,
    pub name: SyntaxResult<GraphqlLiteralName>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub ty: SyntaxResult<AnyGraphqlType>,
    pub default: Option<GraphqlDefaultValue>,
    pub directives: GraphqlDirectiveList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlIntValue {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlIntValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlIntValueFields {
        GraphqlIntValueFields {
            graphql_int_literal_token: self.graphql_int_literal_token(),
        }
    }
    pub fn graphql_int_literal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GraphqlIntValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlIntValueFields {
    pub graphql_int_literal_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlInterfaceTypeDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlInterfaceTypeDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlInterfaceTypeDefinitionFields {
        GraphqlInterfaceTypeDefinitionFields {
            description: self.description(),
            interface_token: self.interface_token(),
            name: self.name(),
            implements: self.implements(),
            directives: self.directives(),
            fields: self.fields(),
        }
    }
    pub fn description(&self) -> Option<GraphqlDescription> {
        support::node(&self.syntax, 0usize)
    }
    pub fn interface_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlNameBinding> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn implements(&self) -> Option<GraphqlImplementsInterfaces> {
        support::node(&self.syntax, 3usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 4usize)
    }
    pub fn fields(&self) -> Option<GraphqlFieldsDefinition> {
        support::node(&self.syntax, 5usize)
    }
}
impl Serialize for GraphqlInterfaceTypeDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlInterfaceTypeDefinitionFields {
    pub description: Option<GraphqlDescription>,
    pub interface_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlNameBinding>,
    pub implements: Option<GraphqlImplementsInterfaces>,
    pub directives: GraphqlDirectiveList,
    pub fields: Option<GraphqlFieldsDefinition>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlInterfaceTypeExtension {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlInterfaceTypeExtension {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlInterfaceTypeExtensionFields {
        GraphqlInterfaceTypeExtensionFields {
            extend_token: self.extend_token(),
            interface_token: self.interface_token(),
            name: self.name(),
            implements: self.implements(),
            directives: self.directives(),
            fields: self.fields(),
        }
    }
    pub fn extend_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn interface_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlNameReference> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn implements(&self) -> Option<GraphqlImplementsInterfaces> {
        support::node(&self.syntax, 3usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 4usize)
    }
    pub fn fields(&self) -> Option<GraphqlFieldsDefinition> {
        support::node(&self.syntax, 5usize)
    }
}
impl Serialize for GraphqlInterfaceTypeExtension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlInterfaceTypeExtensionFields {
    pub extend_token: SyntaxResult<SyntaxToken>,
    pub interface_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlNameReference>,
    pub implements: Option<GraphqlImplementsInterfaces>,
    pub directives: GraphqlDirectiveList,
    pub fields: Option<GraphqlFieldsDefinition>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlListType {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlListType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlListTypeFields {
        GraphqlListTypeFields {
            l_brack_token: self.l_brack_token(),
            element: self.element(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn element(&self) -> SyntaxResult<AnyGraphqlType> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlListType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlListTypeFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub element: SyntaxResult<AnyGraphqlType>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlListValue {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlListValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlListValueFields {
        GraphqlListValueFields {
            l_brack_token: self.l_brack_token(),
            elements: self.elements(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn elements(&self) -> GraphqlListValueElementList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlListValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlListValueFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub elements: GraphqlListValueElementList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlLiteralName {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlLiteralName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlLiteralNameFields {
        GraphqlLiteralNameFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GraphqlLiteralName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlLiteralNameFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlNameBinding {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlNameBinding {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlNameBindingFields {
        GraphqlNameBindingFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GraphqlNameBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlNameBindingFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlNameReference {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlNameReference {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlNameReferenceFields {
        GraphqlNameReferenceFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GraphqlNameReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlNameReferenceFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlNonNullType {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlNonNullType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlNonNullTypeFields {
        GraphqlNonNullTypeFields {
            base: self.base(),
            excl_token: self.excl_token(),
        }
    }
    pub fn base(&self) -> SyntaxResult<AnyGraphqlPrimitiveType> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for GraphqlNonNullType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlNonNullTypeFields {
    pub base: SyntaxResult<AnyGraphqlPrimitiveType>,
    pub excl_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlNullValue {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlNullValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlNullValueFields {
        GraphqlNullValueFields {
            null_token: self.null_token(),
        }
    }
    pub fn null_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GraphqlNullValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlNullValueFields {
    pub null_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlObjectField {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlObjectField {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlObjectFieldFields {
        GraphqlObjectFieldFields {
            name: self.name(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn name(&self) -> SyntaxResult<GraphqlLiteralName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyGraphqlValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlObjectField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlObjectFieldFields {
    pub name: SyntaxResult<GraphqlLiteralName>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyGraphqlValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlObjectTypeDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlObjectTypeDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlObjectTypeDefinitionFields {
        GraphqlObjectTypeDefinitionFields {
            description: self.description(),
            type_token: self.type_token(),
            name: self.name(),
            implements: self.implements(),
            directives: self.directives(),
            fields: self.fields(),
        }
    }
    pub fn description(&self) -> Option<GraphqlDescription> {
        support::node(&self.syntax, 0usize)
    }
    pub fn type_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlNameBinding> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn implements(&self) -> Option<GraphqlImplementsInterfaces> {
        support::node(&self.syntax, 3usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 4usize)
    }
    pub fn fields(&self) -> Option<GraphqlFieldsDefinition> {
        support::node(&self.syntax, 5usize)
    }
}
impl Serialize for GraphqlObjectTypeDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlObjectTypeDefinitionFields {
    pub description: Option<GraphqlDescription>,
    pub type_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlNameBinding>,
    pub implements: Option<GraphqlImplementsInterfaces>,
    pub directives: GraphqlDirectiveList,
    pub fields: Option<GraphqlFieldsDefinition>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlObjectTypeExtension {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlObjectTypeExtension {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlObjectTypeExtensionFields {
        GraphqlObjectTypeExtensionFields {
            extend_token: self.extend_token(),
            type_token: self.type_token(),
            name: self.name(),
            implements: self.implements(),
            directives: self.directives(),
            fields: self.fields(),
        }
    }
    pub fn extend_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn type_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlNameReference> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn implements(&self) -> Option<GraphqlImplementsInterfaces> {
        support::node(&self.syntax, 3usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 4usize)
    }
    pub fn fields(&self) -> Option<GraphqlFieldsDefinition> {
        support::node(&self.syntax, 5usize)
    }
}
impl Serialize for GraphqlObjectTypeExtension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlObjectTypeExtensionFields {
    pub extend_token: SyntaxResult<SyntaxToken>,
    pub type_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlNameReference>,
    pub implements: Option<GraphqlImplementsInterfaces>,
    pub directives: GraphqlDirectiveList,
    pub fields: Option<GraphqlFieldsDefinition>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlObjectValue {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlObjectValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlObjectValueFields {
        GraphqlObjectValueFields {
            l_curly_token: self.l_curly_token(),
            members: self.members(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn members(&self) -> GraphqlObjectValueMemberList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlObjectValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlObjectValueFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub members: GraphqlObjectValueMemberList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlOperationDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlOperationDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlOperationDefinitionFields {
        GraphqlOperationDefinitionFields {
            ty: self.ty(),
            name: self.name(),
            variables: self.variables(),
            directives: self.directives(),
            selection_set: self.selection_set(),
        }
    }
    pub fn ty(&self) -> SyntaxResult<GraphqlOperationType> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn name(&self) -> Option<GraphqlNameBinding> {
        support::node(&self.syntax, 1usize)
    }
    pub fn variables(&self) -> Option<GraphqlVariableDefinitions> {
        support::node(&self.syntax, 2usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 3usize)
    }
    pub fn selection_set(&self) -> SyntaxResult<GraphqlSelectionSet> {
        support::required_node(&self.syntax, 4usize)
    }
}
impl Serialize for GraphqlOperationDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlOperationDefinitionFields {
    pub ty: SyntaxResult<GraphqlOperationType>,
    pub name: Option<GraphqlNameBinding>,
    pub variables: Option<GraphqlVariableDefinitions>,
    pub directives: GraphqlDirectiveList,
    pub selection_set: SyntaxResult<GraphqlSelectionSet>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlOperationType {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlOperationType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlOperationTypeFields {
        GraphqlOperationTypeFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GraphqlOperationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlOperationTypeFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlRoot {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlRootFields {
        GraphqlRootFields {
            bom_token: self.bom_token(),
            definitions: self.definitions(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn definitions(&self) -> GraphqlDefinitionList {
        support::list(&self.syntax, 1usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlRootFields {
    pub bom_token: Option<SyntaxToken>,
    pub definitions: GraphqlDefinitionList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlRootOperationTypeDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlRootOperationTypeDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlRootOperationTypeDefinitionFields {
        GraphqlRootOperationTypeDefinitionFields {
            operation_type: self.operation_type(),
            colon_token: self.colon_token(),
            named_type: self.named_type(),
        }
    }
    pub fn operation_type(&self) -> SyntaxResult<GraphqlOperationType> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn named_type(&self) -> SyntaxResult<GraphqlNameReference> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlRootOperationTypeDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlRootOperationTypeDefinitionFields {
    pub operation_type: SyntaxResult<GraphqlOperationType>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub named_type: SyntaxResult<GraphqlNameReference>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlRootOperationTypes {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlRootOperationTypes {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlRootOperationTypesFields {
        GraphqlRootOperationTypesFields {
            l_curly_token: self.l_curly_token(),
            root_operation_type: self.root_operation_type(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn root_operation_type(&self) -> GraphqlRootOperationTypeDefinitionList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlRootOperationTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlRootOperationTypesFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub root_operation_type: GraphqlRootOperationTypeDefinitionList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlScalarTypeDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlScalarTypeDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlScalarTypeDefinitionFields {
        GraphqlScalarTypeDefinitionFields {
            description: self.description(),
            scalar_token: self.scalar_token(),
            name: self.name(),
            directives: self.directives(),
        }
    }
    pub fn description(&self) -> Option<GraphqlDescription> {
        support::node(&self.syntax, 0usize)
    }
    pub fn scalar_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlNameBinding> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 3usize)
    }
}
impl Serialize for GraphqlScalarTypeDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlScalarTypeDefinitionFields {
    pub description: Option<GraphqlDescription>,
    pub scalar_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlNameBinding>,
    pub directives: GraphqlDirectiveList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlScalarTypeExtension {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlScalarTypeExtension {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlScalarTypeExtensionFields {
        GraphqlScalarTypeExtensionFields {
            extend_token: self.extend_token(),
            scalar_token: self.scalar_token(),
            name: self.name(),
            directives: self.directives(),
        }
    }
    pub fn extend_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn scalar_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlNameReference> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 3usize)
    }
}
impl Serialize for GraphqlScalarTypeExtension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlScalarTypeExtensionFields {
    pub extend_token: SyntaxResult<SyntaxToken>,
    pub scalar_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlNameReference>,
    pub directives: GraphqlDirectiveList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlSchemaDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlSchemaDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlSchemaDefinitionFields {
        GraphqlSchemaDefinitionFields {
            description: self.description(),
            schema_token: self.schema_token(),
            directives: self.directives(),
            root_operation_types: self.root_operation_types(),
        }
    }
    pub fn description(&self) -> Option<GraphqlDescription> {
        support::node(&self.syntax, 0usize)
    }
    pub fn schema_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 2usize)
    }
    pub fn root_operation_types(&self) -> SyntaxResult<GraphqlRootOperationTypes> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl Serialize for GraphqlSchemaDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlSchemaDefinitionFields {
    pub description: Option<GraphqlDescription>,
    pub schema_token: SyntaxResult<SyntaxToken>,
    pub directives: GraphqlDirectiveList,
    pub root_operation_types: SyntaxResult<GraphqlRootOperationTypes>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlSchemaExtension {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlSchemaExtension {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlSchemaExtensionFields {
        GraphqlSchemaExtensionFields {
            extend_token: self.extend_token(),
            schema_token: self.schema_token(),
            directives: self.directives(),
            root_operation_types: self.root_operation_types(),
        }
    }
    pub fn extend_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn schema_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 2usize)
    }
    pub fn root_operation_types(&self) -> Option<GraphqlRootOperationTypes> {
        support::node(&self.syntax, 3usize)
    }
}
impl Serialize for GraphqlSchemaExtension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlSchemaExtensionFields {
    pub extend_token: SyntaxResult<SyntaxToken>,
    pub schema_token: SyntaxResult<SyntaxToken>,
    pub directives: GraphqlDirectiveList,
    pub root_operation_types: Option<GraphqlRootOperationTypes>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlSelectionSet {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlSelectionSet {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlSelectionSetFields {
        GraphqlSelectionSetFields {
            l_curly_token: self.l_curly_token(),
            selections: self.selections(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn selections(&self) -> GraphqlSelectionList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlSelectionSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlSelectionSetFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub selections: GraphqlSelectionList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlStringValue {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlStringValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlStringValueFields {
        GraphqlStringValueFields {
            graphql_string_literal_token: self.graphql_string_literal_token(),
        }
    }
    pub fn graphql_string_literal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for GraphqlStringValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlStringValueFields {
    pub graphql_string_literal_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlTypeCondition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlTypeCondition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlTypeConditionFields {
        GraphqlTypeConditionFields {
            on_token: self.on_token(),
            ty: self.ty(),
        }
    }
    pub fn on_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> SyntaxResult<GraphqlNameReference> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for GraphqlTypeCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlTypeConditionFields {
    pub on_token: SyntaxResult<SyntaxToken>,
    pub ty: SyntaxResult<GraphqlNameReference>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlUnionMemberTypes {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlUnionMemberTypes {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlUnionMemberTypesFields {
        GraphqlUnionMemberTypesFields {
            eq_token: self.eq_token(),
            bitwise_or_token: self.bitwise_or_token(),
            members: self.members(),
        }
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn bitwise_or_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn members(&self) -> GraphqlUnionMemberTypeList {
        support::list(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlUnionMemberTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlUnionMemberTypesFields {
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub bitwise_or_token: Option<SyntaxToken>,
    pub members: GraphqlUnionMemberTypeList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlUnionTypeDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlUnionTypeDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlUnionTypeDefinitionFields {
        GraphqlUnionTypeDefinitionFields {
            description: self.description(),
            union_token: self.union_token(),
            name: self.name(),
            directives: self.directives(),
            union_members: self.union_members(),
        }
    }
    pub fn description(&self) -> Option<GraphqlDescription> {
        support::node(&self.syntax, 0usize)
    }
    pub fn union_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlNameBinding> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 3usize)
    }
    pub fn union_members(&self) -> Option<GraphqlUnionMemberTypes> {
        support::node(&self.syntax, 4usize)
    }
}
impl Serialize for GraphqlUnionTypeDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlUnionTypeDefinitionFields {
    pub description: Option<GraphqlDescription>,
    pub union_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlNameBinding>,
    pub directives: GraphqlDirectiveList,
    pub union_members: Option<GraphqlUnionMemberTypes>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlUnionTypeExtension {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlUnionTypeExtension {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlUnionTypeExtensionFields {
        GraphqlUnionTypeExtensionFields {
            extend_token: self.extend_token(),
            union_token: self.union_token(),
            name: self.name(),
            directives: self.directives(),
            union_members: self.union_members(),
        }
    }
    pub fn extend_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn union_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlNameReference> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 3usize)
    }
    pub fn union_members(&self) -> Option<GraphqlUnionMemberTypes> {
        support::node(&self.syntax, 4usize)
    }
}
impl Serialize for GraphqlUnionTypeExtension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlUnionTypeExtensionFields {
    pub extend_token: SyntaxResult<SyntaxToken>,
    pub union_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlNameReference>,
    pub directives: GraphqlDirectiveList,
    pub union_members: Option<GraphqlUnionMemberTypes>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlVariableBinding {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlVariableBinding {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlVariableBindingFields {
        GraphqlVariableBindingFields {
            dollar_token: self.dollar_token(),
            name: self.name(),
        }
    }
    pub fn dollar_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlLiteralName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for GraphqlVariableBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlVariableBindingFields {
    pub dollar_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlLiteralName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlVariableDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlVariableDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlVariableDefinitionFields {
        GraphqlVariableDefinitionFields {
            variable: self.variable(),
            colon_token: self.colon_token(),
            ty: self.ty(),
            default: self.default(),
            directives: self.directives(),
        }
    }
    pub fn variable(&self) -> SyntaxResult<GraphqlVariableBinding> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn ty(&self) -> SyntaxResult<AnyGraphqlType> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn default(&self) -> Option<GraphqlDefaultValue> {
        support::node(&self.syntax, 3usize)
    }
    pub fn directives(&self) -> GraphqlDirectiveList {
        support::list(&self.syntax, 4usize)
    }
}
impl Serialize for GraphqlVariableDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlVariableDefinitionFields {
    pub variable: SyntaxResult<GraphqlVariableBinding>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub ty: SyntaxResult<AnyGraphqlType>,
    pub default: Option<GraphqlDefaultValue>,
    pub directives: GraphqlDirectiveList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlVariableDefinitions {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlVariableDefinitions {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlVariableDefinitionsFields {
        GraphqlVariableDefinitionsFields {
            l_paren_token: self.l_paren_token(),
            elements: self.elements(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn elements(&self) -> GraphqlVariableDefinitionList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for GraphqlVariableDefinitions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlVariableDefinitionsFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub elements: GraphqlVariableDefinitionList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphqlVariableReference {
    pub(crate) syntax: SyntaxNode,
}
impl GraphqlVariableReference {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GraphqlVariableReferenceFields {
        GraphqlVariableReferenceFields {
            dollar_token: self.dollar_token(),
            name: self.name(),
        }
    }
    pub fn dollar_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<GraphqlLiteralName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for GraphqlVariableReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct GraphqlVariableReferenceFields {
    pub dollar_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GraphqlLiteralName>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGraphqlDefinition {
    AnyGraphqlTypeDefinition(AnyGraphqlTypeDefinition),
    AnyGraphqlTypeExtension(AnyGraphqlTypeExtension),
    GraphqlBogusDefinition(GraphqlBogusDefinition),
    GraphqlDirectiveDefinition(GraphqlDirectiveDefinition),
    GraphqlFragmentDefinition(GraphqlFragmentDefinition),
    GraphqlOperationDefinition(GraphqlOperationDefinition),
    GraphqlSchemaDefinition(GraphqlSchemaDefinition),
    GraphqlSchemaExtension(GraphqlSchemaExtension),
    GraphqlSelectionSet(GraphqlSelectionSet),
}
impl AnyGraphqlDefinition {
    pub fn as_any_graphql_type_definition(&self) -> Option<&AnyGraphqlTypeDefinition> {
        match &self {
            AnyGraphqlDefinition::AnyGraphqlTypeDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_any_graphql_type_extension(&self) -> Option<&AnyGraphqlTypeExtension> {
        match &self {
            AnyGraphqlDefinition::AnyGraphqlTypeExtension(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_bogus_definition(&self) -> Option<&GraphqlBogusDefinition> {
        match &self {
            AnyGraphqlDefinition::GraphqlBogusDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_directive_definition(&self) -> Option<&GraphqlDirectiveDefinition> {
        match &self {
            AnyGraphqlDefinition::GraphqlDirectiveDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_fragment_definition(&self) -> Option<&GraphqlFragmentDefinition> {
        match &self {
            AnyGraphqlDefinition::GraphqlFragmentDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_operation_definition(&self) -> Option<&GraphqlOperationDefinition> {
        match &self {
            AnyGraphqlDefinition::GraphqlOperationDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_schema_definition(&self) -> Option<&GraphqlSchemaDefinition> {
        match &self {
            AnyGraphqlDefinition::GraphqlSchemaDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_schema_extension(&self) -> Option<&GraphqlSchemaExtension> {
        match &self {
            AnyGraphqlDefinition::GraphqlSchemaExtension(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_selection_set(&self) -> Option<&GraphqlSelectionSet> {
        match &self {
            AnyGraphqlDefinition::GraphqlSelectionSet(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGraphqlOperationDefinition {
    GraphqlOperationDefinition(GraphqlOperationDefinition),
    GraphqlSelectionSet(GraphqlSelectionSet),
}
impl AnyGraphqlOperationDefinition {
    pub fn as_graphql_operation_definition(&self) -> Option<&GraphqlOperationDefinition> {
        match &self {
            AnyGraphqlOperationDefinition::GraphqlOperationDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_selection_set(&self) -> Option<&GraphqlSelectionSet> {
        match &self {
            AnyGraphqlOperationDefinition::GraphqlSelectionSet(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGraphqlPrimitiveType {
    GraphqlListType(GraphqlListType),
    GraphqlNameReference(GraphqlNameReference),
}
impl AnyGraphqlPrimitiveType {
    pub fn as_graphql_list_type(&self) -> Option<&GraphqlListType> {
        match &self {
            AnyGraphqlPrimitiveType::GraphqlListType(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_name_reference(&self) -> Option<&GraphqlNameReference> {
        match &self {
            AnyGraphqlPrimitiveType::GraphqlNameReference(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGraphqlSelection {
    GraphqlBogusSelection(GraphqlBogusSelection),
    GraphqlField(GraphqlField),
    GraphqlFragmentSpread(GraphqlFragmentSpread),
    GraphqlInlineFragment(GraphqlInlineFragment),
}
impl AnyGraphqlSelection {
    pub fn as_graphql_bogus_selection(&self) -> Option<&GraphqlBogusSelection> {
        match &self {
            AnyGraphqlSelection::GraphqlBogusSelection(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_field(&self) -> Option<&GraphqlField> {
        match &self {
            AnyGraphqlSelection::GraphqlField(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_fragment_spread(&self) -> Option<&GraphqlFragmentSpread> {
        match &self {
            AnyGraphqlSelection::GraphqlFragmentSpread(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_inline_fragment(&self) -> Option<&GraphqlInlineFragment> {
        match &self {
            AnyGraphqlSelection::GraphqlInlineFragment(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGraphqlType {
    AnyGraphqlPrimitiveType(AnyGraphqlPrimitiveType),
    GraphqlBogusType(GraphqlBogusType),
    GraphqlNonNullType(GraphqlNonNullType),
}
impl AnyGraphqlType {
    pub fn as_any_graphql_primitive_type(&self) -> Option<&AnyGraphqlPrimitiveType> {
        match &self {
            AnyGraphqlType::AnyGraphqlPrimitiveType(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_bogus_type(&self) -> Option<&GraphqlBogusType> {
        match &self {
            AnyGraphqlType::GraphqlBogusType(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_non_null_type(&self) -> Option<&GraphqlNonNullType> {
        match &self {
            AnyGraphqlType::GraphqlNonNullType(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGraphqlTypeDefinition {
    GraphqlEnumTypeDefinition(GraphqlEnumTypeDefinition),
    GraphqlInputObjectTypeDefinition(GraphqlInputObjectTypeDefinition),
    GraphqlInterfaceTypeDefinition(GraphqlInterfaceTypeDefinition),
    GraphqlObjectTypeDefinition(GraphqlObjectTypeDefinition),
    GraphqlScalarTypeDefinition(GraphqlScalarTypeDefinition),
    GraphqlUnionTypeDefinition(GraphqlUnionTypeDefinition),
}
impl AnyGraphqlTypeDefinition {
    pub fn as_graphql_enum_type_definition(&self) -> Option<&GraphqlEnumTypeDefinition> {
        match &self {
            AnyGraphqlTypeDefinition::GraphqlEnumTypeDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_input_object_type_definition(
        &self,
    ) -> Option<&GraphqlInputObjectTypeDefinition> {
        match &self {
            AnyGraphqlTypeDefinition::GraphqlInputObjectTypeDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_interface_type_definition(&self) -> Option<&GraphqlInterfaceTypeDefinition> {
        match &self {
            AnyGraphqlTypeDefinition::GraphqlInterfaceTypeDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_object_type_definition(&self) -> Option<&GraphqlObjectTypeDefinition> {
        match &self {
            AnyGraphqlTypeDefinition::GraphqlObjectTypeDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_scalar_type_definition(&self) -> Option<&GraphqlScalarTypeDefinition> {
        match &self {
            AnyGraphqlTypeDefinition::GraphqlScalarTypeDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_union_type_definition(&self) -> Option<&GraphqlUnionTypeDefinition> {
        match &self {
            AnyGraphqlTypeDefinition::GraphqlUnionTypeDefinition(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGraphqlTypeExtension {
    GraphqlEnumTypeExtension(GraphqlEnumTypeExtension),
    GraphqlInputObjectTypeExtension(GraphqlInputObjectTypeExtension),
    GraphqlInterfaceTypeExtension(GraphqlInterfaceTypeExtension),
    GraphqlObjectTypeExtension(GraphqlObjectTypeExtension),
    GraphqlScalarTypeExtension(GraphqlScalarTypeExtension),
    GraphqlUnionTypeExtension(GraphqlUnionTypeExtension),
}
impl AnyGraphqlTypeExtension {
    pub fn as_graphql_enum_type_extension(&self) -> Option<&GraphqlEnumTypeExtension> {
        match &self {
            AnyGraphqlTypeExtension::GraphqlEnumTypeExtension(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_input_object_type_extension(
        &self,
    ) -> Option<&GraphqlInputObjectTypeExtension> {
        match &self {
            AnyGraphqlTypeExtension::GraphqlInputObjectTypeExtension(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_interface_type_extension(&self) -> Option<&GraphqlInterfaceTypeExtension> {
        match &self {
            AnyGraphqlTypeExtension::GraphqlInterfaceTypeExtension(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_object_type_extension(&self) -> Option<&GraphqlObjectTypeExtension> {
        match &self {
            AnyGraphqlTypeExtension::GraphqlObjectTypeExtension(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_scalar_type_extension(&self) -> Option<&GraphqlScalarTypeExtension> {
        match &self {
            AnyGraphqlTypeExtension::GraphqlScalarTypeExtension(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_union_type_extension(&self) -> Option<&GraphqlUnionTypeExtension> {
        match &self {
            AnyGraphqlTypeExtension::GraphqlUnionTypeExtension(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyGraphqlValue {
    GraphqlBogusValue(GraphqlBogusValue),
    GraphqlBooleanValue(GraphqlBooleanValue),
    GraphqlEnumValue(GraphqlEnumValue),
    GraphqlFloatValue(GraphqlFloatValue),
    GraphqlIntValue(GraphqlIntValue),
    GraphqlListValue(GraphqlListValue),
    GraphqlNullValue(GraphqlNullValue),
    GraphqlObjectValue(GraphqlObjectValue),
    GraphqlStringValue(GraphqlStringValue),
    GraphqlVariableReference(GraphqlVariableReference),
}
impl AnyGraphqlValue {
    pub fn as_graphql_bogus_value(&self) -> Option<&GraphqlBogusValue> {
        match &self {
            AnyGraphqlValue::GraphqlBogusValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_boolean_value(&self) -> Option<&GraphqlBooleanValue> {
        match &self {
            AnyGraphqlValue::GraphqlBooleanValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_enum_value(&self) -> Option<&GraphqlEnumValue> {
        match &self {
            AnyGraphqlValue::GraphqlEnumValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_float_value(&self) -> Option<&GraphqlFloatValue> {
        match &self {
            AnyGraphqlValue::GraphqlFloatValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_int_value(&self) -> Option<&GraphqlIntValue> {
        match &self {
            AnyGraphqlValue::GraphqlIntValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_list_value(&self) -> Option<&GraphqlListValue> {
        match &self {
            AnyGraphqlValue::GraphqlListValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_null_value(&self) -> Option<&GraphqlNullValue> {
        match &self {
            AnyGraphqlValue::GraphqlNullValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_object_value(&self) -> Option<&GraphqlObjectValue> {
        match &self {
            AnyGraphqlValue::GraphqlObjectValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_string_value(&self) -> Option<&GraphqlStringValue> {
        match &self {
            AnyGraphqlValue::GraphqlStringValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_graphql_variable_reference(&self) -> Option<&GraphqlVariableReference> {
        match &self {
            AnyGraphqlValue::GraphqlVariableReference(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for GraphqlAlias {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_ALIAS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_ALIAS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlAlias")
                .field("value", &support::DebugSyntaxResult(self.value()))
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlAlias").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlAlias> for SyntaxNode {
    fn from(n: GraphqlAlias) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlAlias> for SyntaxElement {
    fn from(n: GraphqlAlias) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlArgument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_ARGUMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_ARGUMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlArgument")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("GraphqlArgument").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlArgument> for SyntaxNode {
    fn from(n: GraphqlArgument) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlArgument> for SyntaxElement {
    fn from(n: GraphqlArgument) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlArguments {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_ARGUMENTS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_ARGUMENTS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlArguments")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("arguments", &self.arguments())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlArguments").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlArguments> for SyntaxNode {
    fn from(n: GraphqlArguments) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlArguments> for SyntaxElement {
    fn from(n: GraphqlArguments) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlArgumentsDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_ARGUMENTS_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_ARGUMENTS_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlArgumentsDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlArgumentsDefinition")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("arguments", &self.arguments())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlArgumentsDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlArgumentsDefinition> for SyntaxNode {
    fn from(n: GraphqlArgumentsDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlArgumentsDefinition> for SyntaxElement {
    fn from(n: GraphqlArgumentsDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlBooleanValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_BOOLEAN_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_BOOLEAN_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlBooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlBooleanValue")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlBooleanValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlBooleanValue> for SyntaxNode {
    fn from(n: GraphqlBooleanValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlBooleanValue> for SyntaxElement {
    fn from(n: GraphqlBooleanValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlDefaultValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_DEFAULT_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_DEFAULT_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlDefaultValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlDefaultValue")
                .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("GraphqlDefaultValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlDefaultValue> for SyntaxNode {
    fn from(n: GraphqlDefaultValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlDefaultValue> for SyntaxElement {
    fn from(n: GraphqlDefaultValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlDescription {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_DESCRIPTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_DESCRIPTION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlDescription")
                .field(
                    "graphql_string_value",
                    &support::DebugSyntaxResult(self.graphql_string_value()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlDescription").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlDescription> for SyntaxNode {
    fn from(n: GraphqlDescription) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlDescription> for SyntaxElement {
    fn from(n: GraphqlDescription) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlDirective")
                .field("at_token", &support::DebugSyntaxResult(self.at_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "arguments",
                    &support::DebugOptionalElement(self.arguments()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlDirective> for SyntaxNode {
    fn from(n: GraphqlDirective) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlDirective> for SyntaxElement {
    fn from(n: GraphqlDirective) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlDirectiveDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_DIRECTIVE_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_DIRECTIVE_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlDirectiveDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlDirectiveDefinition")
                .field(
                    "description",
                    &support::DebugOptionalElement(self.description()),
                )
                .field(
                    "directive_token",
                    &support::DebugSyntaxResult(self.directive_token()),
                )
                .field("at_token", &support::DebugSyntaxResult(self.at_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "arguments",
                    &support::DebugOptionalElement(self.arguments()),
                )
                .field(
                    "repeatable_token",
                    &support::DebugOptionalElement(self.repeatable_token()),
                )
                .field("on_token", &support::DebugSyntaxResult(self.on_token()))
                .field(
                    "bitwise_or_token",
                    &support::DebugOptionalElement(self.bitwise_or_token()),
                )
                .field("locations", &self.locations())
                .finish()
        } else {
            f.debug_struct("GraphqlDirectiveDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlDirectiveDefinition> for SyntaxNode {
    fn from(n: GraphqlDirectiveDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlDirectiveDefinition> for SyntaxElement {
    fn from(n: GraphqlDirectiveDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlDirectiveLocation {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_DIRECTIVE_LOCATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_DIRECTIVE_LOCATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlDirectiveLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlDirectiveLocation")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlDirectiveLocation").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlDirectiveLocation> for SyntaxNode {
    fn from(n: GraphqlDirectiveLocation) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlDirectiveLocation> for SyntaxElement {
    fn from(n: GraphqlDirectiveLocation) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlEnumTypeDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_ENUM_TYPE_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_ENUM_TYPE_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlEnumTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlEnumTypeDefinition")
                .field(
                    "description",
                    &support::DebugOptionalElement(self.description()),
                )
                .field("enum_token", &support::DebugSyntaxResult(self.enum_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("directives", &self.directives())
                .field(
                    "enum_values",
                    &support::DebugOptionalElement(self.enum_values()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlEnumTypeDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlEnumTypeDefinition> for SyntaxNode {
    fn from(n: GraphqlEnumTypeDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlEnumTypeDefinition> for SyntaxElement {
    fn from(n: GraphqlEnumTypeDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlEnumTypeExtension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_ENUM_TYPE_EXTENSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_ENUM_TYPE_EXTENSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlEnumTypeExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlEnumTypeExtension")
                .field(
                    "extend_token",
                    &support::DebugSyntaxResult(self.extend_token()),
                )
                .field("enum_token", &support::DebugSyntaxResult(self.enum_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("directives", &self.directives())
                .field(
                    "enum_values",
                    &support::DebugOptionalElement(self.enum_values()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlEnumTypeExtension").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlEnumTypeExtension> for SyntaxNode {
    fn from(n: GraphqlEnumTypeExtension) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlEnumTypeExtension> for SyntaxElement {
    fn from(n: GraphqlEnumTypeExtension) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlEnumValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_ENUM_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_ENUM_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlEnumValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlEnumValue")
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("GraphqlEnumValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlEnumValue> for SyntaxNode {
    fn from(n: GraphqlEnumValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlEnumValue> for SyntaxElement {
    fn from(n: GraphqlEnumValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlEnumValueDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_ENUM_VALUE_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_ENUM_VALUE_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlEnumValueDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlEnumValueDefinition")
                .field(
                    "description",
                    &support::DebugOptionalElement(self.description()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .field("directives", &self.directives())
                .finish()
        } else {
            f.debug_struct("GraphqlEnumValueDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlEnumValueDefinition> for SyntaxNode {
    fn from(n: GraphqlEnumValueDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlEnumValueDefinition> for SyntaxElement {
    fn from(n: GraphqlEnumValueDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlEnumValuesDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_ENUM_VALUES_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_ENUM_VALUES_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlEnumValuesDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlEnumValuesDefinition")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("values", &self.values())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlEnumValuesDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlEnumValuesDefinition> for SyntaxNode {
    fn from(n: GraphqlEnumValuesDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlEnumValuesDefinition> for SyntaxElement {
    fn from(n: GraphqlEnumValuesDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlField {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_FIELD as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_FIELD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlField")
                .field("alias", &support::DebugOptionalElement(self.alias()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "arguments",
                    &support::DebugOptionalElement(self.arguments()),
                )
                .field("directives", &self.directives())
                .field(
                    "selection_set",
                    &support::DebugOptionalElement(self.selection_set()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlField").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlField> for SyntaxNode {
    fn from(n: GraphqlField) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlField> for SyntaxElement {
    fn from(n: GraphqlField) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlFieldDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_FIELD_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_FIELD_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlFieldDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlFieldDefinition")
                .field(
                    "description",
                    &support::DebugOptionalElement(self.description()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "arguments",
                    &support::DebugOptionalElement(self.arguments()),
                )
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("ty", &support::DebugSyntaxResult(self.ty()))
                .field("directives", &self.directives())
                .finish()
        } else {
            f.debug_struct("GraphqlFieldDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlFieldDefinition> for SyntaxNode {
    fn from(n: GraphqlFieldDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlFieldDefinition> for SyntaxElement {
    fn from(n: GraphqlFieldDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlFieldsDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_FIELDS_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_FIELDS_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlFieldsDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlFieldsDefinition")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("fields", &self.fields())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlFieldsDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlFieldsDefinition> for SyntaxNode {
    fn from(n: GraphqlFieldsDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlFieldsDefinition> for SyntaxElement {
    fn from(n: GraphqlFieldsDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlFloatValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_FLOAT_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_FLOAT_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlFloatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlFloatValue")
                .field(
                    "graphql_float_literal_token",
                    &support::DebugSyntaxResult(self.graphql_float_literal_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlFloatValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlFloatValue> for SyntaxNode {
    fn from(n: GraphqlFloatValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlFloatValue> for SyntaxElement {
    fn from(n: GraphqlFloatValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlFragmentDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_FRAGMENT_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_FRAGMENT_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlFragmentDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlFragmentDefinition")
                .field(
                    "fragment_token",
                    &support::DebugSyntaxResult(self.fragment_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "type_condition",
                    &support::DebugSyntaxResult(self.type_condition()),
                )
                .field("directives", &self.directives())
                .field(
                    "selection_set",
                    &support::DebugSyntaxResult(self.selection_set()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlFragmentDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlFragmentDefinition> for SyntaxNode {
    fn from(n: GraphqlFragmentDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlFragmentDefinition> for SyntaxElement {
    fn from(n: GraphqlFragmentDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlFragmentSpread {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_FRAGMENT_SPREAD as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_FRAGMENT_SPREAD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlFragmentSpread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlFragmentSpread")
                .field(
                    "dotdotdot_token",
                    &support::DebugSyntaxResult(self.dotdotdot_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("directives", &self.directives())
                .finish()
        } else {
            f.debug_struct("GraphqlFragmentSpread").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlFragmentSpread> for SyntaxNode {
    fn from(n: GraphqlFragmentSpread) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlFragmentSpread> for SyntaxElement {
    fn from(n: GraphqlFragmentSpread) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlImplementsInterfaces {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_IMPLEMENTS_INTERFACES as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_IMPLEMENTS_INTERFACES
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlImplementsInterfaces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlImplementsInterfaces")
                .field(
                    "implements_token",
                    &support::DebugSyntaxResult(self.implements_token()),
                )
                .field(
                    "amp_token",
                    &support::DebugOptionalElement(self.amp_token()),
                )
                .field("interfaces", &self.interfaces())
                .finish()
        } else {
            f.debug_struct("GraphqlImplementsInterfaces").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlImplementsInterfaces> for SyntaxNode {
    fn from(n: GraphqlImplementsInterfaces) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlImplementsInterfaces> for SyntaxElement {
    fn from(n: GraphqlImplementsInterfaces) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlInlineFragment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_INLINE_FRAGMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_INLINE_FRAGMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlInlineFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlInlineFragment")
                .field(
                    "dotdotdot_token",
                    &support::DebugSyntaxResult(self.dotdotdot_token()),
                )
                .field(
                    "type_condition",
                    &support::DebugOptionalElement(self.type_condition()),
                )
                .field("directives", &self.directives())
                .field(
                    "selection_set",
                    &support::DebugSyntaxResult(self.selection_set()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlInlineFragment").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlInlineFragment> for SyntaxNode {
    fn from(n: GraphqlInlineFragment) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlInlineFragment> for SyntaxElement {
    fn from(n: GraphqlInlineFragment) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlInputFieldsDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_INPUT_FIELDS_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_INPUT_FIELDS_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlInputFieldsDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlInputFieldsDefinition")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("fields", &self.fields())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlInputFieldsDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlInputFieldsDefinition> for SyntaxNode {
    fn from(n: GraphqlInputFieldsDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlInputFieldsDefinition> for SyntaxElement {
    fn from(n: GraphqlInputFieldsDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlInputObjectTypeDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_INPUT_OBJECT_TYPE_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_INPUT_OBJECT_TYPE_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlInputObjectTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlInputObjectTypeDefinition")
                .field(
                    "description",
                    &support::DebugOptionalElement(self.description()),
                )
                .field(
                    "input_token",
                    &support::DebugSyntaxResult(self.input_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("directives", &self.directives())
                .field(
                    "input_fields",
                    &support::DebugOptionalElement(self.input_fields()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlInputObjectTypeDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlInputObjectTypeDefinition> for SyntaxNode {
    fn from(n: GraphqlInputObjectTypeDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlInputObjectTypeDefinition> for SyntaxElement {
    fn from(n: GraphqlInputObjectTypeDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlInputObjectTypeExtension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_INPUT_OBJECT_TYPE_EXTENSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_INPUT_OBJECT_TYPE_EXTENSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlInputObjectTypeExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlInputObjectTypeExtension")
                .field(
                    "extend_token",
                    &support::DebugSyntaxResult(self.extend_token()),
                )
                .field(
                    "input_token",
                    &support::DebugSyntaxResult(self.input_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("directives", &self.directives())
                .field(
                    "input_fields",
                    &support::DebugOptionalElement(self.input_fields()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlInputObjectTypeExtension").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlInputObjectTypeExtension> for SyntaxNode {
    fn from(n: GraphqlInputObjectTypeExtension) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlInputObjectTypeExtension> for SyntaxElement {
    fn from(n: GraphqlInputObjectTypeExtension) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlInputValueDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_INPUT_VALUE_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_INPUT_VALUE_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlInputValueDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlInputValueDefinition")
                .field(
                    "description",
                    &support::DebugOptionalElement(self.description()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("ty", &support::DebugSyntaxResult(self.ty()))
                .field("default", &support::DebugOptionalElement(self.default()))
                .field("directives", &self.directives())
                .finish()
        } else {
            f.debug_struct("GraphqlInputValueDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlInputValueDefinition> for SyntaxNode {
    fn from(n: GraphqlInputValueDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlInputValueDefinition> for SyntaxElement {
    fn from(n: GraphqlInputValueDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlIntValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_INT_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_INT_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlIntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlIntValue")
                .field(
                    "graphql_int_literal_token",
                    &support::DebugSyntaxResult(self.graphql_int_literal_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlIntValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlIntValue> for SyntaxNode {
    fn from(n: GraphqlIntValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlIntValue> for SyntaxElement {
    fn from(n: GraphqlIntValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlInterfaceTypeDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_INTERFACE_TYPE_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_INTERFACE_TYPE_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlInterfaceTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlInterfaceTypeDefinition")
                .field(
                    "description",
                    &support::DebugOptionalElement(self.description()),
                )
                .field(
                    "interface_token",
                    &support::DebugSyntaxResult(self.interface_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "implements",
                    &support::DebugOptionalElement(self.implements()),
                )
                .field("directives", &self.directives())
                .field("fields", &support::DebugOptionalElement(self.fields()))
                .finish()
        } else {
            f.debug_struct("GraphqlInterfaceTypeDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlInterfaceTypeDefinition> for SyntaxNode {
    fn from(n: GraphqlInterfaceTypeDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlInterfaceTypeDefinition> for SyntaxElement {
    fn from(n: GraphqlInterfaceTypeDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlInterfaceTypeExtension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_INTERFACE_TYPE_EXTENSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_INTERFACE_TYPE_EXTENSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlInterfaceTypeExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlInterfaceTypeExtension")
                .field(
                    "extend_token",
                    &support::DebugSyntaxResult(self.extend_token()),
                )
                .field(
                    "interface_token",
                    &support::DebugSyntaxResult(self.interface_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "implements",
                    &support::DebugOptionalElement(self.implements()),
                )
                .field("directives", &self.directives())
                .field("fields", &support::DebugOptionalElement(self.fields()))
                .finish()
        } else {
            f.debug_struct("GraphqlInterfaceTypeExtension").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlInterfaceTypeExtension> for SyntaxNode {
    fn from(n: GraphqlInterfaceTypeExtension) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlInterfaceTypeExtension> for SyntaxElement {
    fn from(n: GraphqlInterfaceTypeExtension) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlListType {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_LIST_TYPE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_LIST_TYPE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlListType")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("element", &support::DebugSyntaxResult(self.element()))
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlListType").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlListType> for SyntaxNode {
    fn from(n: GraphqlListType) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlListType> for SyntaxElement {
    fn from(n: GraphqlListType) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlListValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_LIST_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_LIST_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlListValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlListValue")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("elements", &self.elements())
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlListValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlListValue> for SyntaxNode {
    fn from(n: GraphqlListValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlListValue> for SyntaxElement {
    fn from(n: GraphqlListValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlLiteralName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_LITERAL_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_LITERAL_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlLiteralName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlLiteralName")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlLiteralName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlLiteralName> for SyntaxNode {
    fn from(n: GraphqlLiteralName) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlLiteralName> for SyntaxElement {
    fn from(n: GraphqlLiteralName) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlNameBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_NAME_BINDING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_NAME_BINDING
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlNameBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlNameBinding")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlNameBinding").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlNameBinding> for SyntaxNode {
    fn from(n: GraphqlNameBinding) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlNameBinding> for SyntaxElement {
    fn from(n: GraphqlNameBinding) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlNameReference {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_NAME_REFERENCE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_NAME_REFERENCE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlNameReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlNameReference")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlNameReference").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlNameReference> for SyntaxNode {
    fn from(n: GraphqlNameReference) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlNameReference> for SyntaxElement {
    fn from(n: GraphqlNameReference) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlNonNullType {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_NON_NULL_TYPE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_NON_NULL_TYPE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlNonNullType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlNonNullType")
                .field("base", &support::DebugSyntaxResult(self.base()))
                .field("excl_token", &support::DebugSyntaxResult(self.excl_token()))
                .finish()
        } else {
            f.debug_struct("GraphqlNonNullType").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlNonNullType> for SyntaxNode {
    fn from(n: GraphqlNonNullType) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlNonNullType> for SyntaxElement {
    fn from(n: GraphqlNonNullType) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlNullValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_NULL_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_NULL_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlNullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlNullValue")
                .field("null_token", &support::DebugSyntaxResult(self.null_token()))
                .finish()
        } else {
            f.debug_struct("GraphqlNullValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlNullValue> for SyntaxNode {
    fn from(n: GraphqlNullValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlNullValue> for SyntaxElement {
    fn from(n: GraphqlNullValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlObjectField {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_OBJECT_FIELD as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_OBJECT_FIELD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlObjectField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlObjectField")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("GraphqlObjectField").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlObjectField> for SyntaxNode {
    fn from(n: GraphqlObjectField) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlObjectField> for SyntaxElement {
    fn from(n: GraphqlObjectField) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlObjectTypeDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_OBJECT_TYPE_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_OBJECT_TYPE_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlObjectTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlObjectTypeDefinition")
                .field(
                    "description",
                    &support::DebugOptionalElement(self.description()),
                )
                .field("type_token", &support::DebugSyntaxResult(self.type_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "implements",
                    &support::DebugOptionalElement(self.implements()),
                )
                .field("directives", &self.directives())
                .field("fields", &support::DebugOptionalElement(self.fields()))
                .finish()
        } else {
            f.debug_struct("GraphqlObjectTypeDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlObjectTypeDefinition> for SyntaxNode {
    fn from(n: GraphqlObjectTypeDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlObjectTypeDefinition> for SyntaxElement {
    fn from(n: GraphqlObjectTypeDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlObjectTypeExtension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_OBJECT_TYPE_EXTENSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_OBJECT_TYPE_EXTENSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlObjectTypeExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlObjectTypeExtension")
                .field(
                    "extend_token",
                    &support::DebugSyntaxResult(self.extend_token()),
                )
                .field("type_token", &support::DebugSyntaxResult(self.type_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "implements",
                    &support::DebugOptionalElement(self.implements()),
                )
                .field("directives", &self.directives())
                .field("fields", &support::DebugOptionalElement(self.fields()))
                .finish()
        } else {
            f.debug_struct("GraphqlObjectTypeExtension").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlObjectTypeExtension> for SyntaxNode {
    fn from(n: GraphqlObjectTypeExtension) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlObjectTypeExtension> for SyntaxElement {
    fn from(n: GraphqlObjectTypeExtension) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlObjectValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_OBJECT_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_OBJECT_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlObjectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlObjectValue")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("members", &self.members())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlObjectValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlObjectValue> for SyntaxNode {
    fn from(n: GraphqlObjectValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlObjectValue> for SyntaxElement {
    fn from(n: GraphqlObjectValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlOperationDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_OPERATION_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_OPERATION_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlOperationDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlOperationDefinition")
                .field("ty", &support::DebugSyntaxResult(self.ty()))
                .field("name", &support::DebugOptionalElement(self.name()))
                .field(
                    "variables",
                    &support::DebugOptionalElement(self.variables()),
                )
                .field("directives", &self.directives())
                .field(
                    "selection_set",
                    &support::DebugSyntaxResult(self.selection_set()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlOperationDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlOperationDefinition> for SyntaxNode {
    fn from(n: GraphqlOperationDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlOperationDefinition> for SyntaxElement {
    fn from(n: GraphqlOperationDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlOperationType {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_OPERATION_TYPE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_OPERATION_TYPE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlOperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlOperationType")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlOperationType").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlOperationType> for SyntaxNode {
    fn from(n: GraphqlOperationType) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlOperationType> for SyntaxElement {
    fn from(n: GraphqlOperationType) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_ROOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlRoot")
                .field(
                    "bom_token",
                    &support::DebugOptionalElement(self.bom_token()),
                )
                .field("definitions", &self.definitions())
                .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
                .finish()
        } else {
            f.debug_struct("GraphqlRoot").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlRoot> for SyntaxNode {
    fn from(n: GraphqlRoot) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlRoot> for SyntaxElement {
    fn from(n: GraphqlRoot) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlRootOperationTypeDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_ROOT_OPERATION_TYPE_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_ROOT_OPERATION_TYPE_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlRootOperationTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlRootOperationTypeDefinition")
                .field(
                    "operation_type",
                    &support::DebugSyntaxResult(self.operation_type()),
                )
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("named_type", &support::DebugSyntaxResult(self.named_type()))
                .finish()
        } else {
            f.debug_struct("GraphqlRootOperationTypeDefinition")
                .finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlRootOperationTypeDefinition> for SyntaxNode {
    fn from(n: GraphqlRootOperationTypeDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlRootOperationTypeDefinition> for SyntaxElement {
    fn from(n: GraphqlRootOperationTypeDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlRootOperationTypes {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_ROOT_OPERATION_TYPES as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_ROOT_OPERATION_TYPES
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlRootOperationTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlRootOperationTypes")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("root_operation_type", &self.root_operation_type())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlRootOperationTypes").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlRootOperationTypes> for SyntaxNode {
    fn from(n: GraphqlRootOperationTypes) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlRootOperationTypes> for SyntaxElement {
    fn from(n: GraphqlRootOperationTypes) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlScalarTypeDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_SCALAR_TYPE_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_SCALAR_TYPE_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlScalarTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlScalarTypeDefinition")
                .field(
                    "description",
                    &support::DebugOptionalElement(self.description()),
                )
                .field(
                    "scalar_token",
                    &support::DebugSyntaxResult(self.scalar_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("directives", &self.directives())
                .finish()
        } else {
            f.debug_struct("GraphqlScalarTypeDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlScalarTypeDefinition> for SyntaxNode {
    fn from(n: GraphqlScalarTypeDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlScalarTypeDefinition> for SyntaxElement {
    fn from(n: GraphqlScalarTypeDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlScalarTypeExtension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_SCALAR_TYPE_EXTENSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_SCALAR_TYPE_EXTENSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlScalarTypeExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlScalarTypeExtension")
                .field(
                    "extend_token",
                    &support::DebugSyntaxResult(self.extend_token()),
                )
                .field(
                    "scalar_token",
                    &support::DebugSyntaxResult(self.scalar_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("directives", &self.directives())
                .finish()
        } else {
            f.debug_struct("GraphqlScalarTypeExtension").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlScalarTypeExtension> for SyntaxNode {
    fn from(n: GraphqlScalarTypeExtension) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlScalarTypeExtension> for SyntaxElement {
    fn from(n: GraphqlScalarTypeExtension) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlSchemaDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_SCHEMA_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_SCHEMA_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlSchemaDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlSchemaDefinition")
                .field(
                    "description",
                    &support::DebugOptionalElement(self.description()),
                )
                .field(
                    "schema_token",
                    &support::DebugSyntaxResult(self.schema_token()),
                )
                .field("directives", &self.directives())
                .field(
                    "root_operation_types",
                    &support::DebugSyntaxResult(self.root_operation_types()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlSchemaDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlSchemaDefinition> for SyntaxNode {
    fn from(n: GraphqlSchemaDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlSchemaDefinition> for SyntaxElement {
    fn from(n: GraphqlSchemaDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlSchemaExtension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_SCHEMA_EXTENSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_SCHEMA_EXTENSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlSchemaExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlSchemaExtension")
                .field(
                    "extend_token",
                    &support::DebugSyntaxResult(self.extend_token()),
                )
                .field(
                    "schema_token",
                    &support::DebugSyntaxResult(self.schema_token()),
                )
                .field("directives", &self.directives())
                .field(
                    "root_operation_types",
                    &support::DebugOptionalElement(self.root_operation_types()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlSchemaExtension").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlSchemaExtension> for SyntaxNode {
    fn from(n: GraphqlSchemaExtension) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlSchemaExtension> for SyntaxElement {
    fn from(n: GraphqlSchemaExtension) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlSelectionSet {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_SELECTION_SET as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_SELECTION_SET
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlSelectionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlSelectionSet")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("selections", &self.selections())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlSelectionSet").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlSelectionSet> for SyntaxNode {
    fn from(n: GraphqlSelectionSet) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlSelectionSet> for SyntaxElement {
    fn from(n: GraphqlSelectionSet) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlStringValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_STRING_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_STRING_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlStringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlStringValue")
                .field(
                    "graphql_string_literal_token",
                    &support::DebugSyntaxResult(self.graphql_string_literal_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlStringValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlStringValue> for SyntaxNode {
    fn from(n: GraphqlStringValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlStringValue> for SyntaxElement {
    fn from(n: GraphqlStringValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlTypeCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_TYPE_CONDITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_TYPE_CONDITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlTypeCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlTypeCondition")
                .field("on_token", &support::DebugSyntaxResult(self.on_token()))
                .field("ty", &support::DebugSyntaxResult(self.ty()))
                .finish()
        } else {
            f.debug_struct("GraphqlTypeCondition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlTypeCondition> for SyntaxNode {
    fn from(n: GraphqlTypeCondition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlTypeCondition> for SyntaxElement {
    fn from(n: GraphqlTypeCondition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlUnionMemberTypes {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_UNION_MEMBER_TYPES as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_UNION_MEMBER_TYPES
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlUnionMemberTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlUnionMemberTypes")
                .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
                .field(
                    "bitwise_or_token",
                    &support::DebugOptionalElement(self.bitwise_or_token()),
                )
                .field("members", &self.members())
                .finish()
        } else {
            f.debug_struct("GraphqlUnionMemberTypes").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlUnionMemberTypes> for SyntaxNode {
    fn from(n: GraphqlUnionMemberTypes) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlUnionMemberTypes> for SyntaxElement {
    fn from(n: GraphqlUnionMemberTypes) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlUnionTypeDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_UNION_TYPE_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_UNION_TYPE_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlUnionTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlUnionTypeDefinition")
                .field(
                    "description",
                    &support::DebugOptionalElement(self.description()),
                )
                .field(
                    "union_token",
                    &support::DebugSyntaxResult(self.union_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("directives", &self.directives())
                .field(
                    "union_members",
                    &support::DebugOptionalElement(self.union_members()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlUnionTypeDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlUnionTypeDefinition> for SyntaxNode {
    fn from(n: GraphqlUnionTypeDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlUnionTypeDefinition> for SyntaxElement {
    fn from(n: GraphqlUnionTypeDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlUnionTypeExtension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_UNION_TYPE_EXTENSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_UNION_TYPE_EXTENSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlUnionTypeExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlUnionTypeExtension")
                .field(
                    "extend_token",
                    &support::DebugSyntaxResult(self.extend_token()),
                )
                .field(
                    "union_token",
                    &support::DebugSyntaxResult(self.union_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("directives", &self.directives())
                .field(
                    "union_members",
                    &support::DebugOptionalElement(self.union_members()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlUnionTypeExtension").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlUnionTypeExtension> for SyntaxNode {
    fn from(n: GraphqlUnionTypeExtension) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlUnionTypeExtension> for SyntaxElement {
    fn from(n: GraphqlUnionTypeExtension) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlVariableBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_VARIABLE_BINDING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_VARIABLE_BINDING
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlVariableBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlVariableBinding")
                .field(
                    "dollar_token",
                    &support::DebugSyntaxResult(self.dollar_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("GraphqlVariableBinding").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlVariableBinding> for SyntaxNode {
    fn from(n: GraphqlVariableBinding) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlVariableBinding> for SyntaxElement {
    fn from(n: GraphqlVariableBinding) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlVariableDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_VARIABLE_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_VARIABLE_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlVariableDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlVariableDefinition")
                .field("variable", &support::DebugSyntaxResult(self.variable()))
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("ty", &support::DebugSyntaxResult(self.ty()))
                .field("default", &support::DebugOptionalElement(self.default()))
                .field("directives", &self.directives())
                .finish()
        } else {
            f.debug_struct("GraphqlVariableDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlVariableDefinition> for SyntaxNode {
    fn from(n: GraphqlVariableDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlVariableDefinition> for SyntaxElement {
    fn from(n: GraphqlVariableDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlVariableDefinitions {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_VARIABLE_DEFINITIONS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_VARIABLE_DEFINITIONS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlVariableDefinitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlVariableDefinitions")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("elements", &self.elements())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("GraphqlVariableDefinitions").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlVariableDefinitions> for SyntaxNode {
    fn from(n: GraphqlVariableDefinitions) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlVariableDefinitions> for SyntaxElement {
    fn from(n: GraphqlVariableDefinitions) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GraphqlVariableReference {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_VARIABLE_REFERENCE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_VARIABLE_REFERENCE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlVariableReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("GraphqlVariableReference")
                .field(
                    "dollar_token",
                    &support::DebugSyntaxResult(self.dollar_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("GraphqlVariableReference").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<GraphqlVariableReference> for SyntaxNode {
    fn from(n: GraphqlVariableReference) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlVariableReference> for SyntaxElement {
    fn from(n: GraphqlVariableReference) -> SyntaxElement {
        n.syntax.into()
    }
}
impl From<GraphqlBogusDefinition> for AnyGraphqlDefinition {
    fn from(node: GraphqlBogusDefinition) -> AnyGraphqlDefinition {
        AnyGraphqlDefinition::GraphqlBogusDefinition(node)
    }
}
impl From<GraphqlDirectiveDefinition> for AnyGraphqlDefinition {
    fn from(node: GraphqlDirectiveDefinition) -> AnyGraphqlDefinition {
        AnyGraphqlDefinition::GraphqlDirectiveDefinition(node)
    }
}
impl From<GraphqlFragmentDefinition> for AnyGraphqlDefinition {
    fn from(node: GraphqlFragmentDefinition) -> AnyGraphqlDefinition {
        AnyGraphqlDefinition::GraphqlFragmentDefinition(node)
    }
}
impl From<GraphqlOperationDefinition> for AnyGraphqlDefinition {
    fn from(node: GraphqlOperationDefinition) -> AnyGraphqlDefinition {
        AnyGraphqlDefinition::GraphqlOperationDefinition(node)
    }
}
impl From<GraphqlSchemaDefinition> for AnyGraphqlDefinition {
    fn from(node: GraphqlSchemaDefinition) -> AnyGraphqlDefinition {
        AnyGraphqlDefinition::GraphqlSchemaDefinition(node)
    }
}
impl From<GraphqlSchemaExtension> for AnyGraphqlDefinition {
    fn from(node: GraphqlSchemaExtension) -> AnyGraphqlDefinition {
        AnyGraphqlDefinition::GraphqlSchemaExtension(node)
    }
}
impl From<GraphqlSelectionSet> for AnyGraphqlDefinition {
    fn from(node: GraphqlSelectionSet) -> AnyGraphqlDefinition {
        AnyGraphqlDefinition::GraphqlSelectionSet(node)
    }
}
impl AstNode for AnyGraphqlDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyGraphqlTypeDefinition::KIND_SET
        .union(AnyGraphqlTypeExtension::KIND_SET)
        .union(GraphqlBogusDefinition::KIND_SET)
        .union(GraphqlDirectiveDefinition::KIND_SET)
        .union(GraphqlFragmentDefinition::KIND_SET)
        .union(GraphqlOperationDefinition::KIND_SET)
        .union(GraphqlSchemaDefinition::KIND_SET)
        .union(GraphqlSchemaExtension::KIND_SET)
        .union(GraphqlSelectionSet::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            GRAPHQL_BOGUS_DEFINITION
            | GRAPHQL_DIRECTIVE_DEFINITION
            | GRAPHQL_FRAGMENT_DEFINITION
            | GRAPHQL_OPERATION_DEFINITION
            | GRAPHQL_SCHEMA_DEFINITION
            | GRAPHQL_SCHEMA_EXTENSION
            | GRAPHQL_SELECTION_SET => true,
            k if AnyGraphqlTypeDefinition::can_cast(k) => true,
            k if AnyGraphqlTypeExtension::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRAPHQL_BOGUS_DEFINITION => {
                AnyGraphqlDefinition::GraphqlBogusDefinition(GraphqlBogusDefinition { syntax })
            }
            GRAPHQL_DIRECTIVE_DEFINITION => {
                AnyGraphqlDefinition::GraphqlDirectiveDefinition(GraphqlDirectiveDefinition {
                    syntax,
                })
            }
            GRAPHQL_FRAGMENT_DEFINITION => {
                AnyGraphqlDefinition::GraphqlFragmentDefinition(GraphqlFragmentDefinition {
                    syntax,
                })
            }
            GRAPHQL_OPERATION_DEFINITION => {
                AnyGraphqlDefinition::GraphqlOperationDefinition(GraphqlOperationDefinition {
                    syntax,
                })
            }
            GRAPHQL_SCHEMA_DEFINITION => {
                AnyGraphqlDefinition::GraphqlSchemaDefinition(GraphqlSchemaDefinition { syntax })
            }
            GRAPHQL_SCHEMA_EXTENSION => {
                AnyGraphqlDefinition::GraphqlSchemaExtension(GraphqlSchemaExtension { syntax })
            }
            GRAPHQL_SELECTION_SET => {
                AnyGraphqlDefinition::GraphqlSelectionSet(GraphqlSelectionSet { syntax })
            }
            _ => {
                let syntax = match AnyGraphqlTypeDefinition::try_cast(syntax) {
                    Ok(any_graphql_type_definition) => {
                        return Some(AnyGraphqlDefinition::AnyGraphqlTypeDefinition(
                            any_graphql_type_definition,
                        ));
                    }
                    Err(syntax) => syntax,
                };
                if let Some(any_graphql_type_extension) = AnyGraphqlTypeExtension::cast(syntax) {
                    return Some(AnyGraphqlDefinition::AnyGraphqlTypeExtension(
                        any_graphql_type_extension,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyGraphqlDefinition::GraphqlBogusDefinition(it) => &it.syntax,
            AnyGraphqlDefinition::GraphqlDirectiveDefinition(it) => &it.syntax,
            AnyGraphqlDefinition::GraphqlFragmentDefinition(it) => &it.syntax,
            AnyGraphqlDefinition::GraphqlOperationDefinition(it) => &it.syntax,
            AnyGraphqlDefinition::GraphqlSchemaDefinition(it) => &it.syntax,
            AnyGraphqlDefinition::GraphqlSchemaExtension(it) => &it.syntax,
            AnyGraphqlDefinition::GraphqlSelectionSet(it) => &it.syntax,
            AnyGraphqlDefinition::AnyGraphqlTypeDefinition(it) => it.syntax(),
            AnyGraphqlDefinition::AnyGraphqlTypeExtension(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyGraphqlDefinition::GraphqlBogusDefinition(it) => it.syntax,
            AnyGraphqlDefinition::GraphqlDirectiveDefinition(it) => it.syntax,
            AnyGraphqlDefinition::GraphqlFragmentDefinition(it) => it.syntax,
            AnyGraphqlDefinition::GraphqlOperationDefinition(it) => it.syntax,
            AnyGraphqlDefinition::GraphqlSchemaDefinition(it) => it.syntax,
            AnyGraphqlDefinition::GraphqlSchemaExtension(it) => it.syntax,
            AnyGraphqlDefinition::GraphqlSelectionSet(it) => it.syntax,
            AnyGraphqlDefinition::AnyGraphqlTypeDefinition(it) => it.into_syntax(),
            AnyGraphqlDefinition::AnyGraphqlTypeExtension(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyGraphqlDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyGraphqlDefinition::AnyGraphqlTypeDefinition(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlDefinition::AnyGraphqlTypeExtension(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlDefinition::GraphqlBogusDefinition(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlDefinition::GraphqlDirectiveDefinition(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlDefinition::GraphqlFragmentDefinition(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlDefinition::GraphqlOperationDefinition(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlDefinition::GraphqlSchemaDefinition(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlDefinition::GraphqlSchemaExtension(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlDefinition::GraphqlSelectionSet(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGraphqlDefinition> for SyntaxNode {
    fn from(n: AnyGraphqlDefinition) -> SyntaxNode {
        match n {
            AnyGraphqlDefinition::AnyGraphqlTypeDefinition(it) => it.into(),
            AnyGraphqlDefinition::AnyGraphqlTypeExtension(it) => it.into(),
            AnyGraphqlDefinition::GraphqlBogusDefinition(it) => it.into(),
            AnyGraphqlDefinition::GraphqlDirectiveDefinition(it) => it.into(),
            AnyGraphqlDefinition::GraphqlFragmentDefinition(it) => it.into(),
            AnyGraphqlDefinition::GraphqlOperationDefinition(it) => it.into(),
            AnyGraphqlDefinition::GraphqlSchemaDefinition(it) => it.into(),
            AnyGraphqlDefinition::GraphqlSchemaExtension(it) => it.into(),
            AnyGraphqlDefinition::GraphqlSelectionSet(it) => it.into(),
        }
    }
}
impl From<AnyGraphqlDefinition> for SyntaxElement {
    fn from(n: AnyGraphqlDefinition) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GraphqlOperationDefinition> for AnyGraphqlOperationDefinition {
    fn from(node: GraphqlOperationDefinition) -> AnyGraphqlOperationDefinition {
        AnyGraphqlOperationDefinition::GraphqlOperationDefinition(node)
    }
}
impl From<GraphqlSelectionSet> for AnyGraphqlOperationDefinition {
    fn from(node: GraphqlSelectionSet) -> AnyGraphqlOperationDefinition {
        AnyGraphqlOperationDefinition::GraphqlSelectionSet(node)
    }
}
impl AstNode for AnyGraphqlOperationDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        GraphqlOperationDefinition::KIND_SET.union(GraphqlSelectionSet::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, GRAPHQL_OPERATION_DEFINITION | GRAPHQL_SELECTION_SET)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRAPHQL_OPERATION_DEFINITION => {
                AnyGraphqlOperationDefinition::GraphqlOperationDefinition(
                    GraphqlOperationDefinition { syntax },
                )
            }
            GRAPHQL_SELECTION_SET => {
                AnyGraphqlOperationDefinition::GraphqlSelectionSet(GraphqlSelectionSet { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyGraphqlOperationDefinition::GraphqlOperationDefinition(it) => &it.syntax,
            AnyGraphqlOperationDefinition::GraphqlSelectionSet(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyGraphqlOperationDefinition::GraphqlOperationDefinition(it) => it.syntax,
            AnyGraphqlOperationDefinition::GraphqlSelectionSet(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGraphqlOperationDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyGraphqlOperationDefinition::GraphqlOperationDefinition(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyGraphqlOperationDefinition::GraphqlSelectionSet(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGraphqlOperationDefinition> for SyntaxNode {
    fn from(n: AnyGraphqlOperationDefinition) -> SyntaxNode {
        match n {
            AnyGraphqlOperationDefinition::GraphqlOperationDefinition(it) => it.into(),
            AnyGraphqlOperationDefinition::GraphqlSelectionSet(it) => it.into(),
        }
    }
}
impl From<AnyGraphqlOperationDefinition> for SyntaxElement {
    fn from(n: AnyGraphqlOperationDefinition) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GraphqlListType> for AnyGraphqlPrimitiveType {
    fn from(node: GraphqlListType) -> AnyGraphqlPrimitiveType {
        AnyGraphqlPrimitiveType::GraphqlListType(node)
    }
}
impl From<GraphqlNameReference> for AnyGraphqlPrimitiveType {
    fn from(node: GraphqlNameReference) -> AnyGraphqlPrimitiveType {
        AnyGraphqlPrimitiveType::GraphqlNameReference(node)
    }
}
impl AstNode for AnyGraphqlPrimitiveType {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        GraphqlListType::KIND_SET.union(GraphqlNameReference::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, GRAPHQL_LIST_TYPE | GRAPHQL_NAME_REFERENCE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRAPHQL_LIST_TYPE => {
                AnyGraphqlPrimitiveType::GraphqlListType(GraphqlListType { syntax })
            }
            GRAPHQL_NAME_REFERENCE => {
                AnyGraphqlPrimitiveType::GraphqlNameReference(GraphqlNameReference { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyGraphqlPrimitiveType::GraphqlListType(it) => &it.syntax,
            AnyGraphqlPrimitiveType::GraphqlNameReference(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyGraphqlPrimitiveType::GraphqlListType(it) => it.syntax,
            AnyGraphqlPrimitiveType::GraphqlNameReference(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGraphqlPrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyGraphqlPrimitiveType::GraphqlListType(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlPrimitiveType::GraphqlNameReference(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGraphqlPrimitiveType> for SyntaxNode {
    fn from(n: AnyGraphqlPrimitiveType) -> SyntaxNode {
        match n {
            AnyGraphqlPrimitiveType::GraphqlListType(it) => it.into(),
            AnyGraphqlPrimitiveType::GraphqlNameReference(it) => it.into(),
        }
    }
}
impl From<AnyGraphqlPrimitiveType> for SyntaxElement {
    fn from(n: AnyGraphqlPrimitiveType) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GraphqlBogusSelection> for AnyGraphqlSelection {
    fn from(node: GraphqlBogusSelection) -> AnyGraphqlSelection {
        AnyGraphqlSelection::GraphqlBogusSelection(node)
    }
}
impl From<GraphqlField> for AnyGraphqlSelection {
    fn from(node: GraphqlField) -> AnyGraphqlSelection {
        AnyGraphqlSelection::GraphqlField(node)
    }
}
impl From<GraphqlFragmentSpread> for AnyGraphqlSelection {
    fn from(node: GraphqlFragmentSpread) -> AnyGraphqlSelection {
        AnyGraphqlSelection::GraphqlFragmentSpread(node)
    }
}
impl From<GraphqlInlineFragment> for AnyGraphqlSelection {
    fn from(node: GraphqlInlineFragment) -> AnyGraphqlSelection {
        AnyGraphqlSelection::GraphqlInlineFragment(node)
    }
}
impl AstNode for AnyGraphqlSelection {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GraphqlBogusSelection::KIND_SET
        .union(GraphqlField::KIND_SET)
        .union(GraphqlFragmentSpread::KIND_SET)
        .union(GraphqlInlineFragment::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            GRAPHQL_BOGUS_SELECTION
                | GRAPHQL_FIELD
                | GRAPHQL_FRAGMENT_SPREAD
                | GRAPHQL_INLINE_FRAGMENT
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRAPHQL_BOGUS_SELECTION => {
                AnyGraphqlSelection::GraphqlBogusSelection(GraphqlBogusSelection { syntax })
            }
            GRAPHQL_FIELD => AnyGraphqlSelection::GraphqlField(GraphqlField { syntax }),
            GRAPHQL_FRAGMENT_SPREAD => {
                AnyGraphqlSelection::GraphqlFragmentSpread(GraphqlFragmentSpread { syntax })
            }
            GRAPHQL_INLINE_FRAGMENT => {
                AnyGraphqlSelection::GraphqlInlineFragment(GraphqlInlineFragment { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyGraphqlSelection::GraphqlBogusSelection(it) => &it.syntax,
            AnyGraphqlSelection::GraphqlField(it) => &it.syntax,
            AnyGraphqlSelection::GraphqlFragmentSpread(it) => &it.syntax,
            AnyGraphqlSelection::GraphqlInlineFragment(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyGraphqlSelection::GraphqlBogusSelection(it) => it.syntax,
            AnyGraphqlSelection::GraphqlField(it) => it.syntax,
            AnyGraphqlSelection::GraphqlFragmentSpread(it) => it.syntax,
            AnyGraphqlSelection::GraphqlInlineFragment(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGraphqlSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyGraphqlSelection::GraphqlBogusSelection(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlSelection::GraphqlField(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlSelection::GraphqlFragmentSpread(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlSelection::GraphqlInlineFragment(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGraphqlSelection> for SyntaxNode {
    fn from(n: AnyGraphqlSelection) -> SyntaxNode {
        match n {
            AnyGraphqlSelection::GraphqlBogusSelection(it) => it.into(),
            AnyGraphqlSelection::GraphqlField(it) => it.into(),
            AnyGraphqlSelection::GraphqlFragmentSpread(it) => it.into(),
            AnyGraphqlSelection::GraphqlInlineFragment(it) => it.into(),
        }
    }
}
impl From<AnyGraphqlSelection> for SyntaxElement {
    fn from(n: AnyGraphqlSelection) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GraphqlBogusType> for AnyGraphqlType {
    fn from(node: GraphqlBogusType) -> AnyGraphqlType {
        AnyGraphqlType::GraphqlBogusType(node)
    }
}
impl From<GraphqlNonNullType> for AnyGraphqlType {
    fn from(node: GraphqlNonNullType) -> AnyGraphqlType {
        AnyGraphqlType::GraphqlNonNullType(node)
    }
}
impl AstNode for AnyGraphqlType {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyGraphqlPrimitiveType::KIND_SET
        .union(GraphqlBogusType::KIND_SET)
        .union(GraphqlNonNullType::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            GRAPHQL_BOGUS_TYPE | GRAPHQL_NON_NULL_TYPE => true,
            k if AnyGraphqlPrimitiveType::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRAPHQL_BOGUS_TYPE => AnyGraphqlType::GraphqlBogusType(GraphqlBogusType { syntax }),
            GRAPHQL_NON_NULL_TYPE => {
                AnyGraphqlType::GraphqlNonNullType(GraphqlNonNullType { syntax })
            }
            _ => {
                if let Some(any_graphql_primitive_type) = AnyGraphqlPrimitiveType::cast(syntax) {
                    return Some(AnyGraphqlType::AnyGraphqlPrimitiveType(
                        any_graphql_primitive_type,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyGraphqlType::GraphqlBogusType(it) => &it.syntax,
            AnyGraphqlType::GraphqlNonNullType(it) => &it.syntax,
            AnyGraphqlType::AnyGraphqlPrimitiveType(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyGraphqlType::GraphqlBogusType(it) => it.syntax,
            AnyGraphqlType::GraphqlNonNullType(it) => it.syntax,
            AnyGraphqlType::AnyGraphqlPrimitiveType(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyGraphqlType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyGraphqlType::AnyGraphqlPrimitiveType(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlType::GraphqlBogusType(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlType::GraphqlNonNullType(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGraphqlType> for SyntaxNode {
    fn from(n: AnyGraphqlType) -> SyntaxNode {
        match n {
            AnyGraphqlType::AnyGraphqlPrimitiveType(it) => it.into(),
            AnyGraphqlType::GraphqlBogusType(it) => it.into(),
            AnyGraphqlType::GraphqlNonNullType(it) => it.into(),
        }
    }
}
impl From<AnyGraphqlType> for SyntaxElement {
    fn from(n: AnyGraphqlType) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GraphqlEnumTypeDefinition> for AnyGraphqlTypeDefinition {
    fn from(node: GraphqlEnumTypeDefinition) -> AnyGraphqlTypeDefinition {
        AnyGraphqlTypeDefinition::GraphqlEnumTypeDefinition(node)
    }
}
impl From<GraphqlInputObjectTypeDefinition> for AnyGraphqlTypeDefinition {
    fn from(node: GraphqlInputObjectTypeDefinition) -> AnyGraphqlTypeDefinition {
        AnyGraphqlTypeDefinition::GraphqlInputObjectTypeDefinition(node)
    }
}
impl From<GraphqlInterfaceTypeDefinition> for AnyGraphqlTypeDefinition {
    fn from(node: GraphqlInterfaceTypeDefinition) -> AnyGraphqlTypeDefinition {
        AnyGraphqlTypeDefinition::GraphqlInterfaceTypeDefinition(node)
    }
}
impl From<GraphqlObjectTypeDefinition> for AnyGraphqlTypeDefinition {
    fn from(node: GraphqlObjectTypeDefinition) -> AnyGraphqlTypeDefinition {
        AnyGraphqlTypeDefinition::GraphqlObjectTypeDefinition(node)
    }
}
impl From<GraphqlScalarTypeDefinition> for AnyGraphqlTypeDefinition {
    fn from(node: GraphqlScalarTypeDefinition) -> AnyGraphqlTypeDefinition {
        AnyGraphqlTypeDefinition::GraphqlScalarTypeDefinition(node)
    }
}
impl From<GraphqlUnionTypeDefinition> for AnyGraphqlTypeDefinition {
    fn from(node: GraphqlUnionTypeDefinition) -> AnyGraphqlTypeDefinition {
        AnyGraphqlTypeDefinition::GraphqlUnionTypeDefinition(node)
    }
}
impl AstNode for AnyGraphqlTypeDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GraphqlEnumTypeDefinition::KIND_SET
        .union(GraphqlInputObjectTypeDefinition::KIND_SET)
        .union(GraphqlInterfaceTypeDefinition::KIND_SET)
        .union(GraphqlObjectTypeDefinition::KIND_SET)
        .union(GraphqlScalarTypeDefinition::KIND_SET)
        .union(GraphqlUnionTypeDefinition::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            GRAPHQL_ENUM_TYPE_DEFINITION
                | GRAPHQL_INPUT_OBJECT_TYPE_DEFINITION
                | GRAPHQL_INTERFACE_TYPE_DEFINITION
                | GRAPHQL_OBJECT_TYPE_DEFINITION
                | GRAPHQL_SCALAR_TYPE_DEFINITION
                | GRAPHQL_UNION_TYPE_DEFINITION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRAPHQL_ENUM_TYPE_DEFINITION => {
                AnyGraphqlTypeDefinition::GraphqlEnumTypeDefinition(GraphqlEnumTypeDefinition {
                    syntax,
                })
            }
            GRAPHQL_INPUT_OBJECT_TYPE_DEFINITION => {
                AnyGraphqlTypeDefinition::GraphqlInputObjectTypeDefinition(
                    GraphqlInputObjectTypeDefinition { syntax },
                )
            }
            GRAPHQL_INTERFACE_TYPE_DEFINITION => {
                AnyGraphqlTypeDefinition::GraphqlInterfaceTypeDefinition(
                    GraphqlInterfaceTypeDefinition { syntax },
                )
            }
            GRAPHQL_OBJECT_TYPE_DEFINITION => {
                AnyGraphqlTypeDefinition::GraphqlObjectTypeDefinition(GraphqlObjectTypeDefinition {
                    syntax,
                })
            }
            GRAPHQL_SCALAR_TYPE_DEFINITION => {
                AnyGraphqlTypeDefinition::GraphqlScalarTypeDefinition(GraphqlScalarTypeDefinition {
                    syntax,
                })
            }
            GRAPHQL_UNION_TYPE_DEFINITION => {
                AnyGraphqlTypeDefinition::GraphqlUnionTypeDefinition(GraphqlUnionTypeDefinition {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyGraphqlTypeDefinition::GraphqlEnumTypeDefinition(it) => &it.syntax,
            AnyGraphqlTypeDefinition::GraphqlInputObjectTypeDefinition(it) => &it.syntax,
            AnyGraphqlTypeDefinition::GraphqlInterfaceTypeDefinition(it) => &it.syntax,
            AnyGraphqlTypeDefinition::GraphqlObjectTypeDefinition(it) => &it.syntax,
            AnyGraphqlTypeDefinition::GraphqlScalarTypeDefinition(it) => &it.syntax,
            AnyGraphqlTypeDefinition::GraphqlUnionTypeDefinition(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyGraphqlTypeDefinition::GraphqlEnumTypeDefinition(it) => it.syntax,
            AnyGraphqlTypeDefinition::GraphqlInputObjectTypeDefinition(it) => it.syntax,
            AnyGraphqlTypeDefinition::GraphqlInterfaceTypeDefinition(it) => it.syntax,
            AnyGraphqlTypeDefinition::GraphqlObjectTypeDefinition(it) => it.syntax,
            AnyGraphqlTypeDefinition::GraphqlScalarTypeDefinition(it) => it.syntax,
            AnyGraphqlTypeDefinition::GraphqlUnionTypeDefinition(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGraphqlTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyGraphqlTypeDefinition::GraphqlEnumTypeDefinition(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlTypeDefinition::GraphqlInputObjectTypeDefinition(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyGraphqlTypeDefinition::GraphqlInterfaceTypeDefinition(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyGraphqlTypeDefinition::GraphqlObjectTypeDefinition(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyGraphqlTypeDefinition::GraphqlScalarTypeDefinition(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyGraphqlTypeDefinition::GraphqlUnionTypeDefinition(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGraphqlTypeDefinition> for SyntaxNode {
    fn from(n: AnyGraphqlTypeDefinition) -> SyntaxNode {
        match n {
            AnyGraphqlTypeDefinition::GraphqlEnumTypeDefinition(it) => it.into(),
            AnyGraphqlTypeDefinition::GraphqlInputObjectTypeDefinition(it) => it.into(),
            AnyGraphqlTypeDefinition::GraphqlInterfaceTypeDefinition(it) => it.into(),
            AnyGraphqlTypeDefinition::GraphqlObjectTypeDefinition(it) => it.into(),
            AnyGraphqlTypeDefinition::GraphqlScalarTypeDefinition(it) => it.into(),
            AnyGraphqlTypeDefinition::GraphqlUnionTypeDefinition(it) => it.into(),
        }
    }
}
impl From<AnyGraphqlTypeDefinition> for SyntaxElement {
    fn from(n: AnyGraphqlTypeDefinition) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GraphqlEnumTypeExtension> for AnyGraphqlTypeExtension {
    fn from(node: GraphqlEnumTypeExtension) -> AnyGraphqlTypeExtension {
        AnyGraphqlTypeExtension::GraphqlEnumTypeExtension(node)
    }
}
impl From<GraphqlInputObjectTypeExtension> for AnyGraphqlTypeExtension {
    fn from(node: GraphqlInputObjectTypeExtension) -> AnyGraphqlTypeExtension {
        AnyGraphqlTypeExtension::GraphqlInputObjectTypeExtension(node)
    }
}
impl From<GraphqlInterfaceTypeExtension> for AnyGraphqlTypeExtension {
    fn from(node: GraphqlInterfaceTypeExtension) -> AnyGraphqlTypeExtension {
        AnyGraphqlTypeExtension::GraphqlInterfaceTypeExtension(node)
    }
}
impl From<GraphqlObjectTypeExtension> for AnyGraphqlTypeExtension {
    fn from(node: GraphqlObjectTypeExtension) -> AnyGraphqlTypeExtension {
        AnyGraphqlTypeExtension::GraphqlObjectTypeExtension(node)
    }
}
impl From<GraphqlScalarTypeExtension> for AnyGraphqlTypeExtension {
    fn from(node: GraphqlScalarTypeExtension) -> AnyGraphqlTypeExtension {
        AnyGraphqlTypeExtension::GraphqlScalarTypeExtension(node)
    }
}
impl From<GraphqlUnionTypeExtension> for AnyGraphqlTypeExtension {
    fn from(node: GraphqlUnionTypeExtension) -> AnyGraphqlTypeExtension {
        AnyGraphqlTypeExtension::GraphqlUnionTypeExtension(node)
    }
}
impl AstNode for AnyGraphqlTypeExtension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GraphqlEnumTypeExtension::KIND_SET
        .union(GraphqlInputObjectTypeExtension::KIND_SET)
        .union(GraphqlInterfaceTypeExtension::KIND_SET)
        .union(GraphqlObjectTypeExtension::KIND_SET)
        .union(GraphqlScalarTypeExtension::KIND_SET)
        .union(GraphqlUnionTypeExtension::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            GRAPHQL_ENUM_TYPE_EXTENSION
                | GRAPHQL_INPUT_OBJECT_TYPE_EXTENSION
                | GRAPHQL_INTERFACE_TYPE_EXTENSION
                | GRAPHQL_OBJECT_TYPE_EXTENSION
                | GRAPHQL_SCALAR_TYPE_EXTENSION
                | GRAPHQL_UNION_TYPE_EXTENSION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRAPHQL_ENUM_TYPE_EXTENSION => {
                AnyGraphqlTypeExtension::GraphqlEnumTypeExtension(GraphqlEnumTypeExtension {
                    syntax,
                })
            }
            GRAPHQL_INPUT_OBJECT_TYPE_EXTENSION => {
                AnyGraphqlTypeExtension::GraphqlInputObjectTypeExtension(
                    GraphqlInputObjectTypeExtension { syntax },
                )
            }
            GRAPHQL_INTERFACE_TYPE_EXTENSION => {
                AnyGraphqlTypeExtension::GraphqlInterfaceTypeExtension(
                    GraphqlInterfaceTypeExtension { syntax },
                )
            }
            GRAPHQL_OBJECT_TYPE_EXTENSION => {
                AnyGraphqlTypeExtension::GraphqlObjectTypeExtension(GraphqlObjectTypeExtension {
                    syntax,
                })
            }
            GRAPHQL_SCALAR_TYPE_EXTENSION => {
                AnyGraphqlTypeExtension::GraphqlScalarTypeExtension(GraphqlScalarTypeExtension {
                    syntax,
                })
            }
            GRAPHQL_UNION_TYPE_EXTENSION => {
                AnyGraphqlTypeExtension::GraphqlUnionTypeExtension(GraphqlUnionTypeExtension {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyGraphqlTypeExtension::GraphqlEnumTypeExtension(it) => &it.syntax,
            AnyGraphqlTypeExtension::GraphqlInputObjectTypeExtension(it) => &it.syntax,
            AnyGraphqlTypeExtension::GraphqlInterfaceTypeExtension(it) => &it.syntax,
            AnyGraphqlTypeExtension::GraphqlObjectTypeExtension(it) => &it.syntax,
            AnyGraphqlTypeExtension::GraphqlScalarTypeExtension(it) => &it.syntax,
            AnyGraphqlTypeExtension::GraphqlUnionTypeExtension(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyGraphqlTypeExtension::GraphqlEnumTypeExtension(it) => it.syntax,
            AnyGraphqlTypeExtension::GraphqlInputObjectTypeExtension(it) => it.syntax,
            AnyGraphqlTypeExtension::GraphqlInterfaceTypeExtension(it) => it.syntax,
            AnyGraphqlTypeExtension::GraphqlObjectTypeExtension(it) => it.syntax,
            AnyGraphqlTypeExtension::GraphqlScalarTypeExtension(it) => it.syntax,
            AnyGraphqlTypeExtension::GraphqlUnionTypeExtension(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGraphqlTypeExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyGraphqlTypeExtension::GraphqlEnumTypeExtension(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlTypeExtension::GraphqlInputObjectTypeExtension(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyGraphqlTypeExtension::GraphqlInterfaceTypeExtension(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyGraphqlTypeExtension::GraphqlObjectTypeExtension(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlTypeExtension::GraphqlScalarTypeExtension(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlTypeExtension::GraphqlUnionTypeExtension(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGraphqlTypeExtension> for SyntaxNode {
    fn from(n: AnyGraphqlTypeExtension) -> SyntaxNode {
        match n {
            AnyGraphqlTypeExtension::GraphqlEnumTypeExtension(it) => it.into(),
            AnyGraphqlTypeExtension::GraphqlInputObjectTypeExtension(it) => it.into(),
            AnyGraphqlTypeExtension::GraphqlInterfaceTypeExtension(it) => it.into(),
            AnyGraphqlTypeExtension::GraphqlObjectTypeExtension(it) => it.into(),
            AnyGraphqlTypeExtension::GraphqlScalarTypeExtension(it) => it.into(),
            AnyGraphqlTypeExtension::GraphqlUnionTypeExtension(it) => it.into(),
        }
    }
}
impl From<AnyGraphqlTypeExtension> for SyntaxElement {
    fn from(n: AnyGraphqlTypeExtension) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GraphqlBogusValue> for AnyGraphqlValue {
    fn from(node: GraphqlBogusValue) -> AnyGraphqlValue {
        AnyGraphqlValue::GraphqlBogusValue(node)
    }
}
impl From<GraphqlBooleanValue> for AnyGraphqlValue {
    fn from(node: GraphqlBooleanValue) -> AnyGraphqlValue {
        AnyGraphqlValue::GraphqlBooleanValue(node)
    }
}
impl From<GraphqlEnumValue> for AnyGraphqlValue {
    fn from(node: GraphqlEnumValue) -> AnyGraphqlValue {
        AnyGraphqlValue::GraphqlEnumValue(node)
    }
}
impl From<GraphqlFloatValue> for AnyGraphqlValue {
    fn from(node: GraphqlFloatValue) -> AnyGraphqlValue {
        AnyGraphqlValue::GraphqlFloatValue(node)
    }
}
impl From<GraphqlIntValue> for AnyGraphqlValue {
    fn from(node: GraphqlIntValue) -> AnyGraphqlValue {
        AnyGraphqlValue::GraphqlIntValue(node)
    }
}
impl From<GraphqlListValue> for AnyGraphqlValue {
    fn from(node: GraphqlListValue) -> AnyGraphqlValue {
        AnyGraphqlValue::GraphqlListValue(node)
    }
}
impl From<GraphqlNullValue> for AnyGraphqlValue {
    fn from(node: GraphqlNullValue) -> AnyGraphqlValue {
        AnyGraphqlValue::GraphqlNullValue(node)
    }
}
impl From<GraphqlObjectValue> for AnyGraphqlValue {
    fn from(node: GraphqlObjectValue) -> AnyGraphqlValue {
        AnyGraphqlValue::GraphqlObjectValue(node)
    }
}
impl From<GraphqlStringValue> for AnyGraphqlValue {
    fn from(node: GraphqlStringValue) -> AnyGraphqlValue {
        AnyGraphqlValue::GraphqlStringValue(node)
    }
}
impl From<GraphqlVariableReference> for AnyGraphqlValue {
    fn from(node: GraphqlVariableReference) -> AnyGraphqlValue {
        AnyGraphqlValue::GraphqlVariableReference(node)
    }
}
impl AstNode for AnyGraphqlValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GraphqlBogusValue::KIND_SET
        .union(GraphqlBooleanValue::KIND_SET)
        .union(GraphqlEnumValue::KIND_SET)
        .union(GraphqlFloatValue::KIND_SET)
        .union(GraphqlIntValue::KIND_SET)
        .union(GraphqlListValue::KIND_SET)
        .union(GraphqlNullValue::KIND_SET)
        .union(GraphqlObjectValue::KIND_SET)
        .union(GraphqlStringValue::KIND_SET)
        .union(GraphqlVariableReference::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            GRAPHQL_BOGUS_VALUE
                | GRAPHQL_BOOLEAN_VALUE
                | GRAPHQL_ENUM_VALUE
                | GRAPHQL_FLOAT_VALUE
                | GRAPHQL_INT_VALUE
                | GRAPHQL_LIST_VALUE
                | GRAPHQL_NULL_VALUE
                | GRAPHQL_OBJECT_VALUE
                | GRAPHQL_STRING_VALUE
                | GRAPHQL_VARIABLE_REFERENCE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRAPHQL_BOGUS_VALUE => AnyGraphqlValue::GraphqlBogusValue(GraphqlBogusValue { syntax }),
            GRAPHQL_BOOLEAN_VALUE => {
                AnyGraphqlValue::GraphqlBooleanValue(GraphqlBooleanValue { syntax })
            }
            GRAPHQL_ENUM_VALUE => AnyGraphqlValue::GraphqlEnumValue(GraphqlEnumValue { syntax }),
            GRAPHQL_FLOAT_VALUE => AnyGraphqlValue::GraphqlFloatValue(GraphqlFloatValue { syntax }),
            GRAPHQL_INT_VALUE => AnyGraphqlValue::GraphqlIntValue(GraphqlIntValue { syntax }),
            GRAPHQL_LIST_VALUE => AnyGraphqlValue::GraphqlListValue(GraphqlListValue { syntax }),
            GRAPHQL_NULL_VALUE => AnyGraphqlValue::GraphqlNullValue(GraphqlNullValue { syntax }),
            GRAPHQL_OBJECT_VALUE => {
                AnyGraphqlValue::GraphqlObjectValue(GraphqlObjectValue { syntax })
            }
            GRAPHQL_STRING_VALUE => {
                AnyGraphqlValue::GraphqlStringValue(GraphqlStringValue { syntax })
            }
            GRAPHQL_VARIABLE_REFERENCE => {
                AnyGraphqlValue::GraphqlVariableReference(GraphqlVariableReference { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyGraphqlValue::GraphqlBogusValue(it) => &it.syntax,
            AnyGraphqlValue::GraphqlBooleanValue(it) => &it.syntax,
            AnyGraphqlValue::GraphqlEnumValue(it) => &it.syntax,
            AnyGraphqlValue::GraphqlFloatValue(it) => &it.syntax,
            AnyGraphqlValue::GraphqlIntValue(it) => &it.syntax,
            AnyGraphqlValue::GraphqlListValue(it) => &it.syntax,
            AnyGraphqlValue::GraphqlNullValue(it) => &it.syntax,
            AnyGraphqlValue::GraphqlObjectValue(it) => &it.syntax,
            AnyGraphqlValue::GraphqlStringValue(it) => &it.syntax,
            AnyGraphqlValue::GraphqlVariableReference(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyGraphqlValue::GraphqlBogusValue(it) => it.syntax,
            AnyGraphqlValue::GraphqlBooleanValue(it) => it.syntax,
            AnyGraphqlValue::GraphqlEnumValue(it) => it.syntax,
            AnyGraphqlValue::GraphqlFloatValue(it) => it.syntax,
            AnyGraphqlValue::GraphqlIntValue(it) => it.syntax,
            AnyGraphqlValue::GraphqlListValue(it) => it.syntax,
            AnyGraphqlValue::GraphqlNullValue(it) => it.syntax,
            AnyGraphqlValue::GraphqlObjectValue(it) => it.syntax,
            AnyGraphqlValue::GraphqlStringValue(it) => it.syntax,
            AnyGraphqlValue::GraphqlVariableReference(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGraphqlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyGraphqlValue::GraphqlBogusValue(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlValue::GraphqlBooleanValue(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlValue::GraphqlEnumValue(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlValue::GraphqlFloatValue(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlValue::GraphqlIntValue(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlValue::GraphqlListValue(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlValue::GraphqlNullValue(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlValue::GraphqlObjectValue(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlValue::GraphqlStringValue(it) => std::fmt::Debug::fmt(it, f),
            AnyGraphqlValue::GraphqlVariableReference(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGraphqlValue> for SyntaxNode {
    fn from(n: AnyGraphqlValue) -> SyntaxNode {
        match n {
            AnyGraphqlValue::GraphqlBogusValue(it) => it.into(),
            AnyGraphqlValue::GraphqlBooleanValue(it) => it.into(),
            AnyGraphqlValue::GraphqlEnumValue(it) => it.into(),
            AnyGraphqlValue::GraphqlFloatValue(it) => it.into(),
            AnyGraphqlValue::GraphqlIntValue(it) => it.into(),
            AnyGraphqlValue::GraphqlListValue(it) => it.into(),
            AnyGraphqlValue::GraphqlNullValue(it) => it.into(),
            AnyGraphqlValue::GraphqlObjectValue(it) => it.into(),
            AnyGraphqlValue::GraphqlStringValue(it) => it.into(),
            AnyGraphqlValue::GraphqlVariableReference(it) => it.into(),
        }
    }
}
impl From<AnyGraphqlValue> for SyntaxElement {
    fn from(n: AnyGraphqlValue) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyGraphqlDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGraphqlOperationDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGraphqlPrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGraphqlSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGraphqlType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGraphqlTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGraphqlTypeExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGraphqlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlArgumentsDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlBooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlDefaultValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlDirectiveDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlDirectiveLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlEnumTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlEnumTypeExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlEnumValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlEnumValueDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlEnumValuesDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlFieldDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlFieldsDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlFloatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlFragmentDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlFragmentSpread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlImplementsInterfaces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlInlineFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlInputFieldsDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlInputObjectTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlInputObjectTypeExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlInputValueDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlIntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlInterfaceTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlInterfaceTypeExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlListValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlLiteralName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlNameBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlNameReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlNonNullType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlNullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlObjectField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlObjectTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlObjectTypeExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlObjectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlOperationDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlOperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlRootOperationTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlRootOperationTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlScalarTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlScalarTypeExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlSchemaDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlSchemaExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlSelectionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlStringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlTypeCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlUnionMemberTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlUnionTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlUnionTypeExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlVariableBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlVariableDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlVariableDefinitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GraphqlVariableReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GraphqlBogus {
    syntax: SyntaxNode,
}
impl GraphqlBogus {
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
impl AstNode for GraphqlBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_BOGUS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphqlBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GraphqlBogus> for SyntaxNode {
    fn from(n: GraphqlBogus) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlBogus> for SyntaxElement {
    fn from(n: GraphqlBogus) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GraphqlBogusDefinition {
    syntax: SyntaxNode,
}
impl GraphqlBogusDefinition {
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
impl AstNode for GraphqlBogusDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_BOGUS_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_BOGUS_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlBogusDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphqlBogusDefinition")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GraphqlBogusDefinition> for SyntaxNode {
    fn from(n: GraphqlBogusDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlBogusDefinition> for SyntaxElement {
    fn from(n: GraphqlBogusDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GraphqlBogusSelection {
    syntax: SyntaxNode,
}
impl GraphqlBogusSelection {
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
impl AstNode for GraphqlBogusSelection {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_BOGUS_SELECTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_BOGUS_SELECTION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlBogusSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphqlBogusSelection")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GraphqlBogusSelection> for SyntaxNode {
    fn from(n: GraphqlBogusSelection) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlBogusSelection> for SyntaxElement {
    fn from(n: GraphqlBogusSelection) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GraphqlBogusType {
    syntax: SyntaxNode,
}
impl GraphqlBogusType {
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
impl AstNode for GraphqlBogusType {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_BOGUS_TYPE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_BOGUS_TYPE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlBogusType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphqlBogusType")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GraphqlBogusType> for SyntaxNode {
    fn from(n: GraphqlBogusType) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlBogusType> for SyntaxElement {
    fn from(n: GraphqlBogusType) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GraphqlBogusValue {
    syntax: SyntaxNode,
}
impl GraphqlBogusValue {
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
impl AstNode for GraphqlBogusValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_BOGUS_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_BOGUS_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GraphqlBogusValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphqlBogusValue")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GraphqlBogusValue> for SyntaxNode {
    fn from(n: GraphqlBogusValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<GraphqlBogusValue> for SyntaxElement {
    fn from(n: GraphqlBogusValue) -> SyntaxElement {
        n.syntax.into()
    }
}
biome_rowan::declare_node_union! { pub AnyGraphqlBogusNode = GraphqlBogus | GraphqlBogusDefinition | GraphqlBogusSelection | GraphqlBogusType | GraphqlBogusValue }
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GraphqlArgumentDefinitionList {
    syntax_list: SyntaxList,
}
impl GraphqlArgumentDefinitionList {
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
impl AstNode for GraphqlArgumentDefinitionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_ARGUMENT_DEFINITION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_ARGUMENT_DEFINITION_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GraphqlArgumentDefinitionList> {
        if Self::can_cast(syntax.kind()) {
            Some(GraphqlArgumentDefinitionList {
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
impl Serialize for GraphqlArgumentDefinitionList {
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
impl AstNodeList for GraphqlArgumentDefinitionList {
    type Language = Language;
    type Node = GraphqlInputValueDefinition;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GraphqlArgumentDefinitionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GraphqlArgumentDefinitionList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GraphqlArgumentDefinitionList {
    type Item = GraphqlInputValueDefinition;
    type IntoIter = AstNodeListIterator<Language, GraphqlInputValueDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GraphqlArgumentDefinitionList {
    type Item = GraphqlInputValueDefinition;
    type IntoIter = AstNodeListIterator<Language, GraphqlInputValueDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GraphqlArgumentList {
    syntax_list: SyntaxList,
}
impl GraphqlArgumentList {
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
impl AstNode for GraphqlArgumentList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_ARGUMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_ARGUMENT_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GraphqlArgumentList> {
        if Self::can_cast(syntax.kind()) {
            Some(GraphqlArgumentList {
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
impl Serialize for GraphqlArgumentList {
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
impl AstNodeList for GraphqlArgumentList {
    type Language = Language;
    type Node = GraphqlArgument;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GraphqlArgumentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GraphqlArgumentList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GraphqlArgumentList {
    type Item = GraphqlArgument;
    type IntoIter = AstNodeListIterator<Language, GraphqlArgument>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GraphqlArgumentList {
    type Item = GraphqlArgument;
    type IntoIter = AstNodeListIterator<Language, GraphqlArgument>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GraphqlDefinitionList {
    syntax_list: SyntaxList,
}
impl GraphqlDefinitionList {
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
impl AstNode for GraphqlDefinitionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_DEFINITION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_DEFINITION_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GraphqlDefinitionList> {
        if Self::can_cast(syntax.kind()) {
            Some(GraphqlDefinitionList {
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
impl Serialize for GraphqlDefinitionList {
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
impl AstNodeList for GraphqlDefinitionList {
    type Language = Language;
    type Node = AnyGraphqlDefinition;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GraphqlDefinitionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GraphqlDefinitionList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GraphqlDefinitionList {
    type Item = AnyGraphqlDefinition;
    type IntoIter = AstNodeListIterator<Language, AnyGraphqlDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GraphqlDefinitionList {
    type Item = AnyGraphqlDefinition;
    type IntoIter = AstNodeListIterator<Language, AnyGraphqlDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GraphqlDirectiveList {
    syntax_list: SyntaxList,
}
impl GraphqlDirectiveList {
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
impl AstNode for GraphqlDirectiveList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_DIRECTIVE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_DIRECTIVE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GraphqlDirectiveList> {
        if Self::can_cast(syntax.kind()) {
            Some(GraphqlDirectiveList {
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
impl Serialize for GraphqlDirectiveList {
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
impl AstNodeList for GraphqlDirectiveList {
    type Language = Language;
    type Node = GraphqlDirective;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GraphqlDirectiveList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GraphqlDirectiveList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GraphqlDirectiveList {
    type Item = GraphqlDirective;
    type IntoIter = AstNodeListIterator<Language, GraphqlDirective>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GraphqlDirectiveList {
    type Item = GraphqlDirective;
    type IntoIter = AstNodeListIterator<Language, GraphqlDirective>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GraphqlDirectiveLocationList {
    syntax_list: SyntaxList,
}
impl GraphqlDirectiveLocationList {
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
impl AstNode for GraphqlDirectiveLocationList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_DIRECTIVE_LOCATION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_DIRECTIVE_LOCATION_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GraphqlDirectiveLocationList> {
        if Self::can_cast(syntax.kind()) {
            Some(GraphqlDirectiveLocationList {
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
impl Serialize for GraphqlDirectiveLocationList {
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
impl AstSeparatedList for GraphqlDirectiveLocationList {
    type Language = Language;
    type Node = GraphqlDirectiveLocation;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GraphqlDirectiveLocationList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GraphqlDirectiveLocationList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GraphqlDirectiveLocationList {
    type Item = SyntaxResult<GraphqlDirectiveLocation>;
    type IntoIter = AstSeparatedListNodesIterator<Language, GraphqlDirectiveLocation>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GraphqlDirectiveLocationList {
    type Item = SyntaxResult<GraphqlDirectiveLocation>;
    type IntoIter = AstSeparatedListNodesIterator<Language, GraphqlDirectiveLocation>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GraphqlEnumValueList {
    syntax_list: SyntaxList,
}
impl GraphqlEnumValueList {
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
impl AstNode for GraphqlEnumValueList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_ENUM_VALUE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_ENUM_VALUE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GraphqlEnumValueList> {
        if Self::can_cast(syntax.kind()) {
            Some(GraphqlEnumValueList {
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
impl Serialize for GraphqlEnumValueList {
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
impl AstNodeList for GraphqlEnumValueList {
    type Language = Language;
    type Node = GraphqlEnumValueDefinition;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GraphqlEnumValueList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GraphqlEnumValueList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GraphqlEnumValueList {
    type Item = GraphqlEnumValueDefinition;
    type IntoIter = AstNodeListIterator<Language, GraphqlEnumValueDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GraphqlEnumValueList {
    type Item = GraphqlEnumValueDefinition;
    type IntoIter = AstNodeListIterator<Language, GraphqlEnumValueDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GraphqlFieldDefinitionList {
    syntax_list: SyntaxList,
}
impl GraphqlFieldDefinitionList {
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
impl AstNode for GraphqlFieldDefinitionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_FIELD_DEFINITION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_FIELD_DEFINITION_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GraphqlFieldDefinitionList> {
        if Self::can_cast(syntax.kind()) {
            Some(GraphqlFieldDefinitionList {
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
impl Serialize for GraphqlFieldDefinitionList {
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
impl AstNodeList for GraphqlFieldDefinitionList {
    type Language = Language;
    type Node = GraphqlFieldDefinition;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GraphqlFieldDefinitionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GraphqlFieldDefinitionList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GraphqlFieldDefinitionList {
    type Item = GraphqlFieldDefinition;
    type IntoIter = AstNodeListIterator<Language, GraphqlFieldDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GraphqlFieldDefinitionList {
    type Item = GraphqlFieldDefinition;
    type IntoIter = AstNodeListIterator<Language, GraphqlFieldDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GraphqlImplementsInterfaceList {
    syntax_list: SyntaxList,
}
impl GraphqlImplementsInterfaceList {
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
impl AstNode for GraphqlImplementsInterfaceList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_IMPLEMENTS_INTERFACE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_IMPLEMENTS_INTERFACE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GraphqlImplementsInterfaceList> {
        if Self::can_cast(syntax.kind()) {
            Some(GraphqlImplementsInterfaceList {
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
impl Serialize for GraphqlImplementsInterfaceList {
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
impl AstSeparatedList for GraphqlImplementsInterfaceList {
    type Language = Language;
    type Node = GraphqlNameReference;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GraphqlImplementsInterfaceList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GraphqlImplementsInterfaceList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GraphqlImplementsInterfaceList {
    type Item = SyntaxResult<GraphqlNameReference>;
    type IntoIter = AstSeparatedListNodesIterator<Language, GraphqlNameReference>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GraphqlImplementsInterfaceList {
    type Item = SyntaxResult<GraphqlNameReference>;
    type IntoIter = AstSeparatedListNodesIterator<Language, GraphqlNameReference>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GraphqlInputFieldList {
    syntax_list: SyntaxList,
}
impl GraphqlInputFieldList {
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
impl AstNode for GraphqlInputFieldList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_INPUT_FIELD_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_INPUT_FIELD_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GraphqlInputFieldList> {
        if Self::can_cast(syntax.kind()) {
            Some(GraphqlInputFieldList {
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
impl Serialize for GraphqlInputFieldList {
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
impl AstNodeList for GraphqlInputFieldList {
    type Language = Language;
    type Node = GraphqlInputValueDefinition;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GraphqlInputFieldList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GraphqlInputFieldList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GraphqlInputFieldList {
    type Item = GraphqlInputValueDefinition;
    type IntoIter = AstNodeListIterator<Language, GraphqlInputValueDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GraphqlInputFieldList {
    type Item = GraphqlInputValueDefinition;
    type IntoIter = AstNodeListIterator<Language, GraphqlInputValueDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GraphqlListValueElementList {
    syntax_list: SyntaxList,
}
impl GraphqlListValueElementList {
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
impl AstNode for GraphqlListValueElementList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_LIST_VALUE_ELEMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_LIST_VALUE_ELEMENT_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GraphqlListValueElementList> {
        if Self::can_cast(syntax.kind()) {
            Some(GraphqlListValueElementList {
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
impl Serialize for GraphqlListValueElementList {
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
impl AstNodeList for GraphqlListValueElementList {
    type Language = Language;
    type Node = AnyGraphqlValue;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GraphqlListValueElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GraphqlListValueElementList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GraphqlListValueElementList {
    type Item = AnyGraphqlValue;
    type IntoIter = AstNodeListIterator<Language, AnyGraphqlValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GraphqlListValueElementList {
    type Item = AnyGraphqlValue;
    type IntoIter = AstNodeListIterator<Language, AnyGraphqlValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GraphqlObjectValueMemberList {
    syntax_list: SyntaxList,
}
impl GraphqlObjectValueMemberList {
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
impl AstNode for GraphqlObjectValueMemberList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_OBJECT_VALUE_MEMBER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_OBJECT_VALUE_MEMBER_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GraphqlObjectValueMemberList> {
        if Self::can_cast(syntax.kind()) {
            Some(GraphqlObjectValueMemberList {
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
impl Serialize for GraphqlObjectValueMemberList {
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
impl AstNodeList for GraphqlObjectValueMemberList {
    type Language = Language;
    type Node = GraphqlObjectField;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GraphqlObjectValueMemberList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GraphqlObjectValueMemberList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GraphqlObjectValueMemberList {
    type Item = GraphqlObjectField;
    type IntoIter = AstNodeListIterator<Language, GraphqlObjectField>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GraphqlObjectValueMemberList {
    type Item = GraphqlObjectField;
    type IntoIter = AstNodeListIterator<Language, GraphqlObjectField>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GraphqlRootOperationTypeDefinitionList {
    syntax_list: SyntaxList,
}
impl GraphqlRootOperationTypeDefinitionList {
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
impl AstNode for GraphqlRootOperationTypeDefinitionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        GRAPHQL_ROOT_OPERATION_TYPE_DEFINITION_LIST as u16,
    ));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_ROOT_OPERATION_TYPE_DEFINITION_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GraphqlRootOperationTypeDefinitionList> {
        if Self::can_cast(syntax.kind()) {
            Some(GraphqlRootOperationTypeDefinitionList {
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
impl Serialize for GraphqlRootOperationTypeDefinitionList {
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
impl AstNodeList for GraphqlRootOperationTypeDefinitionList {
    type Language = Language;
    type Node = GraphqlRootOperationTypeDefinition;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GraphqlRootOperationTypeDefinitionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GraphqlRootOperationTypeDefinitionList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GraphqlRootOperationTypeDefinitionList {
    type Item = GraphqlRootOperationTypeDefinition;
    type IntoIter = AstNodeListIterator<Language, GraphqlRootOperationTypeDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GraphqlRootOperationTypeDefinitionList {
    type Item = GraphqlRootOperationTypeDefinition;
    type IntoIter = AstNodeListIterator<Language, GraphqlRootOperationTypeDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GraphqlSelectionList {
    syntax_list: SyntaxList,
}
impl GraphqlSelectionList {
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
impl AstNode for GraphqlSelectionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_SELECTION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_SELECTION_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GraphqlSelectionList> {
        if Self::can_cast(syntax.kind()) {
            Some(GraphqlSelectionList {
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
impl Serialize for GraphqlSelectionList {
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
impl AstNodeList for GraphqlSelectionList {
    type Language = Language;
    type Node = AnyGraphqlSelection;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GraphqlSelectionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GraphqlSelectionList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GraphqlSelectionList {
    type Item = AnyGraphqlSelection;
    type IntoIter = AstNodeListIterator<Language, AnyGraphqlSelection>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GraphqlSelectionList {
    type Item = AnyGraphqlSelection;
    type IntoIter = AstNodeListIterator<Language, AnyGraphqlSelection>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GraphqlUnionMemberTypeList {
    syntax_list: SyntaxList,
}
impl GraphqlUnionMemberTypeList {
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
impl AstNode for GraphqlUnionMemberTypeList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_UNION_MEMBER_TYPE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_UNION_MEMBER_TYPE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GraphqlUnionMemberTypeList> {
        if Self::can_cast(syntax.kind()) {
            Some(GraphqlUnionMemberTypeList {
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
impl Serialize for GraphqlUnionMemberTypeList {
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
impl AstSeparatedList for GraphqlUnionMemberTypeList {
    type Language = Language;
    type Node = GraphqlNameReference;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GraphqlUnionMemberTypeList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GraphqlUnionMemberTypeList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GraphqlUnionMemberTypeList {
    type Item = SyntaxResult<GraphqlNameReference>;
    type IntoIter = AstSeparatedListNodesIterator<Language, GraphqlNameReference>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GraphqlUnionMemberTypeList {
    type Item = SyntaxResult<GraphqlNameReference>;
    type IntoIter = AstSeparatedListNodesIterator<Language, GraphqlNameReference>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GraphqlVariableDefinitionList {
    syntax_list: SyntaxList,
}
impl GraphqlVariableDefinitionList {
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
impl AstNode for GraphqlVariableDefinitionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRAPHQL_VARIABLE_DEFINITION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRAPHQL_VARIABLE_DEFINITION_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GraphqlVariableDefinitionList> {
        if Self::can_cast(syntax.kind()) {
            Some(GraphqlVariableDefinitionList {
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
impl Serialize for GraphqlVariableDefinitionList {
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
impl AstNodeList for GraphqlVariableDefinitionList {
    type Language = Language;
    type Node = GraphqlVariableDefinition;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GraphqlVariableDefinitionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GraphqlVariableDefinitionList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &GraphqlVariableDefinitionList {
    type Item = GraphqlVariableDefinition;
    type IntoIter = AstNodeListIterator<Language, GraphqlVariableDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for GraphqlVariableDefinitionList {
    type Item = GraphqlVariableDefinition;
    type IntoIter = AstNodeListIterator<Language, GraphqlVariableDefinition>;
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
