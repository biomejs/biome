//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
use biome_rowan::AstNode;
use biome_yaml_syntax::{
    YamlSyntaxElement as SyntaxElement, YamlSyntaxNode as SyntaxNode,
    YamlSyntaxToken as SyntaxToken, *,
};
pub fn yaml_alias_node(value_token: SyntaxToken) -> YamlAliasNode {
    YamlAliasNode::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_ALIAS_NODE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn yaml_anchor_property(value_token: SyntaxToken) -> YamlAnchorProperty {
    YamlAnchorProperty::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_ANCHOR_PROPERTY,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn yaml_block_map_explicit_entry(
    question_mark_token: SyntaxToken,
) -> YamlBlockMapExplicitEntryBuilder {
    YamlBlockMapExplicitEntryBuilder {
        question_mark_token,
        key: None,
        colon_token: None,
        value: None,
    }
}
pub struct YamlBlockMapExplicitEntryBuilder {
    question_mark_token: SyntaxToken,
    key: Option<AnyYamlBlockNode>,
    colon_token: Option<SyntaxToken>,
    value: Option<AnyYamlBlockNode>,
}
impl YamlBlockMapExplicitEntryBuilder {
    pub fn with_key(mut self, key: AnyYamlBlockNode) -> Self {
        self.key = Some(key);
        self
    }
    pub fn with_colon_token(mut self, colon_token: SyntaxToken) -> Self {
        self.colon_token = Some(colon_token);
        self
    }
    pub fn with_value(mut self, value: AnyYamlBlockNode) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> YamlBlockMapExplicitEntry {
        YamlBlockMapExplicitEntry::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_BLOCK_MAP_EXPLICIT_ENTRY,
            [
                Some(SyntaxElement::Token(self.question_mark_token)),
                self.key
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.colon_token.map(|token| SyntaxElement::Token(token)),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn yaml_block_map_implicit_entry(colon_token: SyntaxToken) -> YamlBlockMapImplicitEntryBuilder {
    YamlBlockMapImplicitEntryBuilder {
        colon_token,
        key: None,
        value: None,
    }
}
pub struct YamlBlockMapImplicitEntryBuilder {
    colon_token: SyntaxToken,
    key: Option<AnyYamlMappingImplicitKey>,
    value: Option<AnyYamlBlockNode>,
}
impl YamlBlockMapImplicitEntryBuilder {
    pub fn with_key(mut self, key: AnyYamlMappingImplicitKey) -> Self {
        self.key = Some(key);
        self
    }
    pub fn with_value(mut self, value: AnyYamlBlockNode) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> YamlBlockMapImplicitEntry {
        YamlBlockMapImplicitEntry::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_BLOCK_MAP_IMPLICIT_ENTRY,
            [
                self.key
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.colon_token)),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn yaml_block_mapping(
    mapping_start_token: SyntaxToken,
    entries: YamlBlockMapEntryList,
    mapping_end_token: SyntaxToken,
) -> YamlBlockMappingBuilder {
    YamlBlockMappingBuilder {
        mapping_start_token,
        entries,
        mapping_end_token,
        properties: None,
    }
}
pub struct YamlBlockMappingBuilder {
    mapping_start_token: SyntaxToken,
    entries: YamlBlockMapEntryList,
    mapping_end_token: SyntaxToken,
    properties: Option<AnyYamlPropertiesCombination>,
}
impl YamlBlockMappingBuilder {
    pub fn with_properties(mut self, properties: AnyYamlPropertiesCombination) -> Self {
        self.properties = Some(properties);
        self
    }
    pub fn build(self) -> YamlBlockMapping {
        YamlBlockMapping::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_BLOCK_MAPPING,
            [
                Some(SyntaxElement::Token(self.mapping_start_token)),
                self.properties
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.entries.into_syntax())),
                Some(SyntaxElement::Token(self.mapping_end_token)),
            ],
        ))
    }
}
pub fn yaml_block_scalar(content: AnyYamlBlockScalarContent) -> YamlBlockScalarBuilder {
    YamlBlockScalarBuilder {
        content,
        properties: None,
    }
}
pub struct YamlBlockScalarBuilder {
    content: AnyYamlBlockScalarContent,
    properties: Option<AnyYamlPropertiesCombination>,
}
impl YamlBlockScalarBuilder {
    pub fn with_properties(mut self, properties: AnyYamlPropertiesCombination) -> Self {
        self.properties = Some(properties);
        self
    }
    pub fn build(self) -> YamlBlockScalar {
        YamlBlockScalar::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_BLOCK_SCALAR,
            [
                self.properties
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.content.into_syntax())),
            ],
        ))
    }
}
pub fn yaml_block_sequence(
    sequence_start_token: SyntaxToken,
    entries: YamlBlockSequenceEntryList,
    sequence_end_token: SyntaxToken,
) -> YamlBlockSequenceBuilder {
    YamlBlockSequenceBuilder {
        sequence_start_token,
        entries,
        sequence_end_token,
        properties: None,
    }
}
pub struct YamlBlockSequenceBuilder {
    sequence_start_token: SyntaxToken,
    entries: YamlBlockSequenceEntryList,
    sequence_end_token: SyntaxToken,
    properties: Option<AnyYamlPropertiesCombination>,
}
impl YamlBlockSequenceBuilder {
    pub fn with_properties(mut self, properties: AnyYamlPropertiesCombination) -> Self {
        self.properties = Some(properties);
        self
    }
    pub fn build(self) -> YamlBlockSequence {
        YamlBlockSequence::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_BLOCK_SEQUENCE,
            [
                Some(SyntaxElement::Token(self.sequence_start_token)),
                self.properties
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.entries.into_syntax())),
                Some(SyntaxElement::Token(self.sequence_end_token)),
            ],
        ))
    }
}
pub fn yaml_block_sequence_entry(minus_token: SyntaxToken) -> YamlBlockSequenceEntryBuilder {
    YamlBlockSequenceEntryBuilder {
        minus_token,
        value: None,
    }
}
pub struct YamlBlockSequenceEntryBuilder {
    minus_token: SyntaxToken,
    value: Option<AnyYamlBlockNode>,
}
impl YamlBlockSequenceEntryBuilder {
    pub fn with_value(mut self, value: AnyYamlBlockNode) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> YamlBlockSequenceEntry {
        YamlBlockSequenceEntry::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_BLOCK_SEQUENCE_ENTRY,
            [
                Some(SyntaxElement::Token(self.minus_token)),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn yaml_directive(value_token: SyntaxToken) -> YamlDirective {
    YamlDirective::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_DIRECTIVE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn yaml_document(directives: YamlDirectiveList) -> YamlDocumentBuilder {
    YamlDocumentBuilder {
        directives,
        bom_token: None,
        dashdashdash_token: None,
        node: None,
        dotdotdot_token: None,
    }
}
pub struct YamlDocumentBuilder {
    directives: YamlDirectiveList,
    bom_token: Option<SyntaxToken>,
    dashdashdash_token: Option<SyntaxToken>,
    node: Option<AnyYamlBlockNode>,
    dotdotdot_token: Option<SyntaxToken>,
}
impl YamlDocumentBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn with_dashdashdash_token(mut self, dashdashdash_token: SyntaxToken) -> Self {
        self.dashdashdash_token = Some(dashdashdash_token);
        self
    }
    pub fn with_node(mut self, node: AnyYamlBlockNode) -> Self {
        self.node = Some(node);
        self
    }
    pub fn with_dotdotdot_token(mut self, dotdotdot_token: SyntaxToken) -> Self {
        self.dotdotdot_token = Some(dotdotdot_token);
        self
    }
    pub fn build(self) -> YamlDocument {
        YamlDocument::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_DOCUMENT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                self.dashdashdash_token
                    .map(|token| SyntaxElement::Token(token)),
                self.node
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.dotdotdot_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn yaml_double_quoted_scalar(value_token: SyntaxToken) -> YamlDoubleQuotedScalar {
    YamlDoubleQuotedScalar::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_DOUBLE_QUOTED_SCALAR,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn yaml_flow_in_block_node(
    flow_start_token: SyntaxToken,
    flow: AnyYamlFlowNode,
    flow_end_token: SyntaxToken,
) -> YamlFlowInBlockNode {
    YamlFlowInBlockNode::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_FLOW_IN_BLOCK_NODE,
        [
            Some(SyntaxElement::Token(flow_start_token)),
            Some(SyntaxElement::Node(flow.into_syntax())),
            Some(SyntaxElement::Token(flow_end_token)),
        ],
    ))
}
pub fn yaml_flow_json_node() -> YamlFlowJsonNodeBuilder {
    YamlFlowJsonNodeBuilder {
        properties: None,
        content: None,
    }
}
pub struct YamlFlowJsonNodeBuilder {
    properties: Option<AnyYamlPropertiesCombination>,
    content: Option<AnyYamlJsonContent>,
}
impl YamlFlowJsonNodeBuilder {
    pub fn with_properties(mut self, properties: AnyYamlPropertiesCombination) -> Self {
        self.properties = Some(properties);
        self
    }
    pub fn with_content(mut self, content: AnyYamlJsonContent) -> Self {
        self.content = Some(content);
        self
    }
    pub fn build(self) -> YamlFlowJsonNode {
        YamlFlowJsonNode::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_FLOW_JSON_NODE,
            [
                self.properties
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.content
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn yaml_flow_map_explicit_entry(
    question_mark_token: SyntaxToken,
) -> YamlFlowMapExplicitEntryBuilder {
    YamlFlowMapExplicitEntryBuilder {
        question_mark_token,
        key: None,
        colon_token: None,
        value: None,
    }
}
pub struct YamlFlowMapExplicitEntryBuilder {
    question_mark_token: SyntaxToken,
    key: Option<AnyYamlMappingImplicitKey>,
    colon_token: Option<SyntaxToken>,
    value: Option<AnyYamlFlowNode>,
}
impl YamlFlowMapExplicitEntryBuilder {
    pub fn with_key(mut self, key: AnyYamlMappingImplicitKey) -> Self {
        self.key = Some(key);
        self
    }
    pub fn with_colon_token(mut self, colon_token: SyntaxToken) -> Self {
        self.colon_token = Some(colon_token);
        self
    }
    pub fn with_value(mut self, value: AnyYamlFlowNode) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> YamlFlowMapExplicitEntry {
        YamlFlowMapExplicitEntry::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_FLOW_MAP_EXPLICIT_ENTRY,
            [
                Some(SyntaxElement::Token(self.question_mark_token)),
                self.key
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.colon_token.map(|token| SyntaxElement::Token(token)),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn yaml_flow_map_implicit_entry() -> YamlFlowMapImplicitEntryBuilder {
    YamlFlowMapImplicitEntryBuilder {
        key: None,
        colon_token: None,
        value: None,
    }
}
pub struct YamlFlowMapImplicitEntryBuilder {
    key: Option<AnyYamlMappingImplicitKey>,
    colon_token: Option<SyntaxToken>,
    value: Option<AnyYamlFlowNode>,
}
impl YamlFlowMapImplicitEntryBuilder {
    pub fn with_key(mut self, key: AnyYamlMappingImplicitKey) -> Self {
        self.key = Some(key);
        self
    }
    pub fn with_colon_token(mut self, colon_token: SyntaxToken) -> Self {
        self.colon_token = Some(colon_token);
        self
    }
    pub fn with_value(mut self, value: AnyYamlFlowNode) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> YamlFlowMapImplicitEntry {
        YamlFlowMapImplicitEntry::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_FLOW_MAP_IMPLICIT_ENTRY,
            [
                self.key
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.colon_token.map(|token| SyntaxElement::Token(token)),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn yaml_flow_mapping(
    l_curly_token: SyntaxToken,
    entries: YamlFlowMapEntryList,
    r_curly_token: SyntaxToken,
) -> YamlFlowMapping {
    YamlFlowMapping::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_FLOW_MAPPING,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(entries.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn yaml_flow_sequence(
    l_brack_token: SyntaxToken,
    entries: YamlFlowSequenceEntryList,
    r_brack_token: SyntaxToken,
) -> YamlFlowSequence {
    YamlFlowSequence::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_FLOW_SEQUENCE,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(entries.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn yaml_flow_yaml_node() -> YamlFlowYamlNodeBuilder {
    YamlFlowYamlNodeBuilder {
        properties: None,
        content: None,
    }
}
pub struct YamlFlowYamlNodeBuilder {
    properties: Option<AnyYamlPropertiesCombination>,
    content: Option<YamlPlainScalar>,
}
impl YamlFlowYamlNodeBuilder {
    pub fn with_properties(mut self, properties: AnyYamlPropertiesCombination) -> Self {
        self.properties = Some(properties);
        self
    }
    pub fn with_content(mut self, content: YamlPlainScalar) -> Self {
        self.content = Some(content);
        self
    }
    pub fn build(self) -> YamlFlowYamlNode {
        YamlFlowYamlNode::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_FLOW_YAML_NODE,
            [
                self.properties
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.content
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn yaml_folded_scalar(value_token: SyntaxToken) -> YamlFoldedScalar {
    YamlFoldedScalar::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_FOLDED_SCALAR,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn yaml_literal_scalar(value_token: SyntaxToken) -> YamlLiteralScalar {
    YamlLiteralScalar::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_LITERAL_SCALAR,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn yaml_plain_scalar(value_token: SyntaxToken) -> YamlPlainScalar {
    YamlPlainScalar::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_PLAIN_SCALAR,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn yaml_properties_anchor_first(
    anchor: YamlAnchorProperty,
) -> YamlPropertiesAnchorFirstBuilder {
    YamlPropertiesAnchorFirstBuilder { anchor, tag: None }
}
pub struct YamlPropertiesAnchorFirstBuilder {
    anchor: YamlAnchorProperty,
    tag: Option<YamlTagProperty>,
}
impl YamlPropertiesAnchorFirstBuilder {
    pub fn with_tag(mut self, tag: YamlTagProperty) -> Self {
        self.tag = Some(tag);
        self
    }
    pub fn build(self) -> YamlPropertiesAnchorFirst {
        YamlPropertiesAnchorFirst::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_PROPERTIES_ANCHOR_FIRST,
            [
                Some(SyntaxElement::Node(self.anchor.into_syntax())),
                self.tag
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn yaml_properties_tag_first(tag: YamlTagProperty) -> YamlPropertiesTagFirstBuilder {
    YamlPropertiesTagFirstBuilder { tag, anchor: None }
}
pub struct YamlPropertiesTagFirstBuilder {
    tag: YamlTagProperty,
    anchor: Option<YamlAnchorProperty>,
}
impl YamlPropertiesTagFirstBuilder {
    pub fn with_anchor(mut self, anchor: YamlAnchorProperty) -> Self {
        self.anchor = Some(anchor);
        self
    }
    pub fn build(self) -> YamlPropertiesTagFirst {
        YamlPropertiesTagFirst::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_PROPERTIES_TAG_FIRST,
            [
                Some(SyntaxElement::Node(self.tag.into_syntax())),
                self.anchor
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn yaml_root(documents: YamlDocumentList, eof_token: SyntaxToken) -> YamlRoot {
    YamlRoot::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_ROOT,
        [
            Some(SyntaxElement::Node(documents.into_syntax())),
            Some(SyntaxElement::Token(eof_token)),
        ],
    ))
}
pub fn yaml_single_quoted_scalar(value_token: SyntaxToken) -> YamlSingleQuotedScalar {
    YamlSingleQuotedScalar::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_SINGLE_QUOTED_SCALAR,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn yaml_tag_property(value_token: SyntaxToken) -> YamlTagProperty {
    YamlTagProperty::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_TAG_PROPERTY,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn yaml_block_map_entry_list<I>(items: I) -> YamlBlockMapEntryList
where
    I: IntoIterator<Item = AnyYamlBlockMapEntry>,
    I::IntoIter: ExactSizeIterator,
{
    YamlBlockMapEntryList::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_BLOCK_MAP_ENTRY_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn yaml_block_sequence_entry_list<I>(items: I) -> YamlBlockSequenceEntryList
where
    I: IntoIterator<Item = AnyYamlBlockSequenceEntry>,
    I::IntoIter: ExactSizeIterator,
{
    YamlBlockSequenceEntryList::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_BLOCK_SEQUENCE_ENTRY_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn yaml_directive_list<I>(items: I) -> YamlDirectiveList
where
    I: IntoIterator<Item = YamlDirective>,
    I::IntoIter: ExactSizeIterator,
{
    YamlDirectiveList::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_DIRECTIVE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn yaml_document_list<I>(items: I) -> YamlDocumentList
where
    I: IntoIterator<Item = AnyYamlDocument>,
    I::IntoIter: ExactSizeIterator,
{
    YamlDocumentList::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_DOCUMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn yaml_flow_map_entry_list<I, S>(items: I, separators: S) -> YamlFlowMapEntryList
where
    I: IntoIterator<Item = AnyYamlFlowMapEntry>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = YamlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    YamlFlowMapEntryList::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_FLOW_MAP_ENTRY_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn yaml_flow_sequence_entry_list<I, S>(items: I, separators: S) -> YamlFlowSequenceEntryList
where
    I: IntoIterator<Item = AnyYamlFlowSequenceEntry>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = YamlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    YamlFlowSequenceEntryList::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_FLOW_SEQUENCE_ENTRY_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn yaml_bogus<I>(slots: I) -> YamlBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    YamlBogus::unwrap_cast(SyntaxNode::new_detached(YamlSyntaxKind::YAML_BOGUS, slots))
}
pub fn yaml_bogus_block_map_entry<I>(slots: I) -> YamlBogusBlockMapEntry
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    YamlBogusBlockMapEntry::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_BOGUS_BLOCK_MAP_ENTRY,
        slots,
    ))
}
pub fn yaml_bogus_block_node<I>(slots: I) -> YamlBogusBlockNode
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    YamlBogusBlockNode::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_BOGUS_BLOCK_NODE,
        slots,
    ))
}
pub fn yaml_bogus_flow_node<I>(slots: I) -> YamlBogusFlowNode
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    YamlBogusFlowNode::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_BOGUS_FLOW_NODE,
        slots,
    ))
}
