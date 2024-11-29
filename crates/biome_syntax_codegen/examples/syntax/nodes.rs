#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
use crate::{
    macros::map_syntax_node,
    MiniLanguage as Language, MiniSyntaxElement as SyntaxElement,
    MiniSyntaxElementChildren as SyntaxElementChildren,
    MiniSyntaxKind::{self as SyntaxKind, *},
    MiniSyntaxList as SyntaxList, MiniSyntaxNode as SyntaxNode, MiniSyntaxToken as SyntaxToken,
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
pub struct MiniComplexNode {
    pub(crate) syntax: SyntaxNode,
}
impl MiniComplexNode {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MiniComplexNodeFields {
        MiniComplexNodeFields {
            complex_token: self.complex_token(),
            r_bracket_token: self.r_bracket_token(),
            list: self.list(),
            l_bracket_token: self.l_bracket_token(),
        }
    }
    pub fn complex_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn r_bracket_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn list(&self) -> MiniNodeList {
        support::list(&self.syntax, 2usize)
    }
    pub fn l_bracket_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for MiniComplexNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MiniComplexNodeFields {
    pub complex_token: SyntaxResult<SyntaxToken>,
    pub r_bracket_token: SyntaxResult<SyntaxToken>,
    pub list: MiniNodeList,
    pub l_bracket_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MiniRoot {
    pub(crate) syntax: SyntaxNode,
}
impl MiniRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MiniRootFields {
        MiniRootFields {
            any_mini_node: self.any_mini_node(),
        }
    }
    pub fn any_mini_node(&self) -> SyntaxResult<AnyMiniNode> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for MiniRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MiniRootFields {
    pub any_mini_node: SyntaxResult<AnyMiniNode>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MiniSimpleNode {
    pub(crate) syntax: SyntaxNode,
}
impl MiniSimpleNode {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MiniSimpleNodeFields {
        MiniSimpleNodeFields {
            simple_token: self.simple_token(),
            r_bracket_token: self.r_bracket_token(),
            ident: self.ident(),
            l_bracket_token: self.l_bracket_token(),
        }
    }
    pub fn simple_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn r_bracket_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn ident(&self) -> SyntaxResult<MiniString> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn l_bracket_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for MiniSimpleNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MiniSimpleNodeFields {
    pub simple_token: SyntaxResult<SyntaxToken>,
    pub r_bracket_token: SyntaxResult<SyntaxToken>,
    pub ident: SyntaxResult<MiniString>,
    pub l_bracket_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MiniString {
    pub(crate) syntax: SyntaxNode,
}
impl MiniString {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MiniStringFields {
        MiniStringFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MiniString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MiniStringFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMiniNode {
    MiniComplexNode(MiniComplexNode),
    MiniSimpleNode(MiniSimpleNode),
}
impl AnyMiniNode {
    pub fn as_mini_complex_node(&self) -> Option<&MiniComplexNode> {
        match &self {
            AnyMiniNode::MiniComplexNode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_mini_simple_node(&self) -> Option<&MiniSimpleNode> {
        match &self {
            AnyMiniNode::MiniSimpleNode(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for MiniComplexNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MINI_COMPLEX_NODE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MINI_COMPLEX_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MiniComplexNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MiniComplexNode")
            .field(
                "complex_token",
                &support::DebugSyntaxResult(self.complex_token()),
            )
            .field(
                "r_bracket_token",
                &support::DebugSyntaxResult(self.r_bracket_token()),
            )
            .field("list", &self.list())
            .field(
                "l_bracket_token",
                &support::DebugSyntaxResult(self.l_bracket_token()),
            )
            .finish()
    }
}
impl From<MiniComplexNode> for SyntaxNode {
    fn from(n: MiniComplexNode) -> SyntaxNode {
        n.syntax
    }
}
impl From<MiniComplexNode> for SyntaxElement {
    fn from(n: MiniComplexNode) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MiniRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MINI_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MINI_ROOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MiniRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MiniRoot")
            .field(
                "any_mini_node",
                &support::DebugSyntaxResult(self.any_mini_node()),
            )
            .finish()
    }
}
impl From<MiniRoot> for SyntaxNode {
    fn from(n: MiniRoot) -> SyntaxNode {
        n.syntax
    }
}
impl From<MiniRoot> for SyntaxElement {
    fn from(n: MiniRoot) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MiniSimpleNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MINI_SIMPLE_NODE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MINI_SIMPLE_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MiniSimpleNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MiniSimpleNode")
            .field(
                "simple_token",
                &support::DebugSyntaxResult(self.simple_token()),
            )
            .field(
                "r_bracket_token",
                &support::DebugSyntaxResult(self.r_bracket_token()),
            )
            .field("ident", &support::DebugSyntaxResult(self.ident()))
            .field(
                "l_bracket_token",
                &support::DebugSyntaxResult(self.l_bracket_token()),
            )
            .finish()
    }
}
impl From<MiniSimpleNode> for SyntaxNode {
    fn from(n: MiniSimpleNode) -> SyntaxNode {
        n.syntax
    }
}
impl From<MiniSimpleNode> for SyntaxElement {
    fn from(n: MiniSimpleNode) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for MiniString {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MINI_STRING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MINI_STRING
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MiniString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MiniString")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<MiniString> for SyntaxNode {
    fn from(n: MiniString) -> SyntaxNode {
        n.syntax
    }
}
impl From<MiniString> for SyntaxElement {
    fn from(n: MiniString) -> SyntaxElement {
        n.syntax.into()
    }
}
impl From<MiniComplexNode> for AnyMiniNode {
    fn from(node: MiniComplexNode) -> AnyMiniNode {
        AnyMiniNode::MiniComplexNode(node)
    }
}
impl From<MiniSimpleNode> for AnyMiniNode {
    fn from(node: MiniSimpleNode) -> AnyMiniNode {
        AnyMiniNode::MiniSimpleNode(node)
    }
}
impl AstNode for AnyMiniNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        MiniComplexNode::KIND_SET.union(MiniSimpleNode::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, MINI_COMPLEX_NODE | MINI_SIMPLE_NODE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MINI_COMPLEX_NODE => AnyMiniNode::MiniComplexNode(MiniComplexNode { syntax }),
            MINI_SIMPLE_NODE => AnyMiniNode::MiniSimpleNode(MiniSimpleNode { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyMiniNode::MiniComplexNode(it) => &it.syntax,
            AnyMiniNode::MiniSimpleNode(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyMiniNode::MiniComplexNode(it) => it.syntax,
            AnyMiniNode::MiniSimpleNode(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMiniNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyMiniNode::MiniComplexNode(it) => std::fmt::Debug::fmt(it, f),
            AnyMiniNode::MiniSimpleNode(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMiniNode> for SyntaxNode {
    fn from(n: AnyMiniNode) -> SyntaxNode {
        match n {
            AnyMiniNode::MiniComplexNode(it) => it.into(),
            AnyMiniNode::MiniSimpleNode(it) => it.into(),
        }
    }
}
impl From<AnyMiniNode> for SyntaxElement {
    fn from(n: AnyMiniNode) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyMiniNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MiniComplexNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MiniRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MiniSimpleNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MiniString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MiniBogus {
    syntax: SyntaxNode,
}
impl MiniBogus {
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
impl AstNode for MiniBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MINI_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MINI_BOGUS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MiniBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MiniBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<MiniBogus> for SyntaxNode {
    fn from(n: MiniBogus) -> SyntaxNode {
        n.syntax
    }
}
impl From<MiniBogus> for SyntaxElement {
    fn from(n: MiniBogus) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MiniNodeList {
    syntax_list: SyntaxList,
}
impl MiniNodeList {
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
impl AstNode for MiniNodeList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MINI_NODE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MINI_NODE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<MiniNodeList> {
        if Self::can_cast(syntax.kind()) {
            Some(MiniNodeList {
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
impl Serialize for MiniNodeList {
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
impl AstNodeList for MiniNodeList {
    type Language = Language;
    type Node = AnyMiniNode;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MiniNodeList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MiniNodeList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MiniNodeList {
    type Item = AnyMiniNode;
    type IntoIter = AstNodeListIterator<Language, AnyMiniNode>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MiniNodeList {
    type Item = AnyMiniNode;
    type IntoIter = AstNodeListIterator<Language, AnyMiniNode>;
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
