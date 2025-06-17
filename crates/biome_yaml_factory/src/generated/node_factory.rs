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
pub fn yaml_block_collection(content: AnyYamlBlockCollectionContent) -> YamlBlockCollectionBuilder {
    YamlBlockCollectionBuilder {
        content,
        properties: None,
    }
}
pub struct YamlBlockCollectionBuilder {
    content: AnyYamlBlockCollectionContent,
    properties: Option<AnyYamlPropertiesCombination>,
}
impl YamlBlockCollectionBuilder {
    pub fn with_properties(mut self, properties: AnyYamlPropertiesCombination) -> Self {
        self.properties = Some(properties);
        self
    }
    pub fn build(self) -> YamlBlockCollection {
        YamlBlockCollection::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_BLOCK_COLLECTION,
            [
                self.properties
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.content.into_syntax())),
            ],
        ))
    }
}
pub fn yaml_block_map_explicit_entry(
    key: YamlBlockMapExplicitKey,
) -> YamlBlockMapExplicitEntryBuilder {
    YamlBlockMapExplicitEntryBuilder { key, value: None }
}
pub struct YamlBlockMapExplicitEntryBuilder {
    key: YamlBlockMapExplicitKey,
    value: Option<YamlBlockMapExplicitValue>,
}
impl YamlBlockMapExplicitEntryBuilder {
    pub fn with_value(mut self, value: YamlBlockMapExplicitValue) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> YamlBlockMapExplicitEntry {
        YamlBlockMapExplicitEntry::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_BLOCK_MAP_EXPLICIT_ENTRY,
            [
                Some(SyntaxElement::Node(self.key.into_syntax())),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn yaml_block_map_explicit_key(
    question_mark_token: SyntaxToken,
) -> YamlBlockMapExplicitKeyBuilder {
    YamlBlockMapExplicitKeyBuilder {
        question_mark_token,
        key: None,
    }
}
pub struct YamlBlockMapExplicitKeyBuilder {
    question_mark_token: SyntaxToken,
    key: Option<AnyYamlBlockIndented>,
}
impl YamlBlockMapExplicitKeyBuilder {
    pub fn with_key(mut self, key: AnyYamlBlockIndented) -> Self {
        self.key = Some(key);
        self
    }
    pub fn build(self) -> YamlBlockMapExplicitKey {
        YamlBlockMapExplicitKey::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_BLOCK_MAP_EXPLICIT_KEY,
            [
                Some(SyntaxElement::Token(self.question_mark_token)),
                self.key
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn yaml_block_map_explicit_value(colon_token: SyntaxToken) -> YamlBlockMapExplicitValueBuilder {
    YamlBlockMapExplicitValueBuilder {
        colon_token,
        value: None,
    }
}
pub struct YamlBlockMapExplicitValueBuilder {
    colon_token: SyntaxToken,
    value: Option<AnyYamlBlockIndented>,
}
impl YamlBlockMapExplicitValueBuilder {
    pub fn with_value(mut self, value: AnyYamlBlockIndented) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> YamlBlockMapExplicitValue {
        YamlBlockMapExplicitValue::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_BLOCK_MAP_EXPLICIT_VALUE,
            [
                Some(SyntaxElement::Token(self.colon_token)),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn yaml_block_map_implicit_entry(
    value: YamlBlockMapImplicitValue,
) -> YamlBlockMapImplicitEntryBuilder {
    YamlBlockMapImplicitEntryBuilder { value, key: None }
}
pub struct YamlBlockMapImplicitEntryBuilder {
    value: YamlBlockMapImplicitValue,
    key: Option<AnyYamlBlockMapImplicitKey>,
}
impl YamlBlockMapImplicitEntryBuilder {
    pub fn with_key(mut self, key: AnyYamlBlockMapImplicitKey) -> Self {
        self.key = Some(key);
        self
    }
    pub fn build(self) -> YamlBlockMapImplicitEntry {
        YamlBlockMapImplicitEntry::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_BLOCK_MAP_IMPLICIT_ENTRY,
            [
                self.key
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.value.into_syntax())),
            ],
        ))
    }
}
pub fn yaml_block_map_implicit_value(colon_token: SyntaxToken) -> YamlBlockMapImplicitValueBuilder {
    YamlBlockMapImplicitValueBuilder {
        colon_token,
        value: None,
        newline_token: None,
    }
}
pub struct YamlBlockMapImplicitValueBuilder {
    colon_token: SyntaxToken,
    value: Option<AnyYamlBlockNode>,
    newline_token: Option<SyntaxToken>,
}
impl YamlBlockMapImplicitValueBuilder {
    pub fn with_value(mut self, value: AnyYamlBlockNode) -> Self {
        self.value = Some(value);
        self
    }
    pub fn with_newline_token(mut self, newline_token: SyntaxToken) -> Self {
        self.newline_token = Some(newline_token);
        self
    }
    pub fn build(self) -> YamlBlockMapImplicitValue {
        YamlBlockMapImplicitValue::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_BLOCK_MAP_IMPLICIT_VALUE,
            [
                Some(SyntaxElement::Token(self.colon_token)),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.newline_token.map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn yaml_block_mapping(entries: YamlBlockMapEntryList) -> YamlBlockMappingBuilder {
    YamlBlockMappingBuilder {
        entries,
        indent_token: None,
        dedent_token: None,
    }
}
pub struct YamlBlockMappingBuilder {
    entries: YamlBlockMapEntryList,
    indent_token: Option<SyntaxToken>,
    dedent_token: Option<SyntaxToken>,
}
impl YamlBlockMappingBuilder {
    pub fn with_indent_token(mut self, indent_token: SyntaxToken) -> Self {
        self.indent_token = Some(indent_token);
        self
    }
    pub fn with_dedent_token(mut self, dedent_token: SyntaxToken) -> Self {
        self.dedent_token = Some(dedent_token);
        self
    }
    pub fn build(self) -> YamlBlockMapping {
        YamlBlockMapping::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_BLOCK_MAPPING,
            [
                self.indent_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.entries.into_syntax())),
                self.dedent_token.map(|token| SyntaxElement::Token(token)),
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
pub fn yaml_block_sequence(entries: YamlBlockSequenceEntryList) -> YamlBlockSequenceBuilder {
    YamlBlockSequenceBuilder {
        entries,
        indent_token: None,
        dedent_token: None,
    }
}
pub struct YamlBlockSequenceBuilder {
    entries: YamlBlockSequenceEntryList,
    indent_token: Option<SyntaxToken>,
    dedent_token: Option<SyntaxToken>,
}
impl YamlBlockSequenceBuilder {
    pub fn with_indent_token(mut self, indent_token: SyntaxToken) -> Self {
        self.indent_token = Some(indent_token);
        self
    }
    pub fn with_dedent_token(mut self, dedent_token: SyntaxToken) -> Self {
        self.dedent_token = Some(dedent_token);
        self
    }
    pub fn build(self) -> YamlBlockSequence {
        YamlBlockSequence::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_BLOCK_SEQUENCE,
            [
                self.indent_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.entries.into_syntax())),
                self.dedent_token.map(|token| SyntaxElement::Token(token)),
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
    value: Option<AnyYamlBlockIndented>,
}
impl YamlBlockSequenceEntryBuilder {
    pub fn with_value(mut self, value: AnyYamlBlockIndented) -> Self {
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
pub fn yaml_compact_mapping(entries: YamlBlockSequenceEntryList) -> YamlCompactMapping {
    YamlCompactMapping::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_COMPACT_MAPPING,
        [Some(SyntaxElement::Node(entries.into_syntax()))],
    ))
}
pub fn yaml_compact_sequence(entries: YamlBlockSequenceEntryList) -> YamlCompactSequence {
    YamlCompactSequence::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_COMPACT_SEQUENCE,
        [Some(SyntaxElement::Node(entries.into_syntax()))],
    ))
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
pub fn yaml_flow_in_block_node(flow: AnyYamlFlowNode) -> YamlFlowInBlockNodeBuilder {
    YamlFlowInBlockNodeBuilder {
        flow,
        newline_token: None,
    }
}
pub struct YamlFlowInBlockNodeBuilder {
    flow: AnyYamlFlowNode,
    newline_token: Option<SyntaxToken>,
}
impl YamlFlowInBlockNodeBuilder {
    pub fn with_newline_token(mut self, newline_token: SyntaxToken) -> Self {
        self.newline_token = Some(newline_token);
        self
    }
    pub fn build(self) -> YamlFlowInBlockNode {
        YamlFlowInBlockNode::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_FLOW_IN_BLOCK_NODE,
            [
                Some(SyntaxElement::Node(self.flow.into_syntax())),
                self.newline_token.map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
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
        entry: None,
    }
}
pub struct YamlFlowMapExplicitEntryBuilder {
    question_mark_token: SyntaxToken,
    entry: Option<YamlFlowMapImplicitEntry>,
}
impl YamlFlowMapExplicitEntryBuilder {
    pub fn with_entry(mut self, entry: YamlFlowMapImplicitEntry) -> Self {
        self.entry = Some(entry);
        self
    }
    pub fn build(self) -> YamlFlowMapExplicitEntry {
        YamlFlowMapExplicitEntry::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_FLOW_MAP_EXPLICIT_ENTRY,
            [
                Some(SyntaxElement::Token(self.question_mark_token)),
                self.entry
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn yaml_flow_map_implicit_entry(
    colon_token: SyntaxToken,
    value: AnyYamlFlowNode,
) -> YamlFlowMapImplicitEntryBuilder {
    YamlFlowMapImplicitEntryBuilder {
        colon_token,
        value,
        key: None,
    }
}
pub struct YamlFlowMapImplicitEntryBuilder {
    colon_token: SyntaxToken,
    value: AnyYamlFlowNode,
    key: Option<AnyYamlFlowMapImplicitKey>,
}
impl YamlFlowMapImplicitEntryBuilder {
    pub fn with_key(mut self, key: AnyYamlFlowMapImplicitKey) -> Self {
        self.key = Some(key);
        self
    }
    pub fn build(self) -> YamlFlowMapImplicitEntry {
        YamlFlowMapImplicitEntry::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_FLOW_MAP_IMPLICIT_ENTRY,
            [
                self.key
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.colon_token)),
                Some(SyntaxElement::Node(self.value.into_syntax())),
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
    I: IntoIterator<Item = YamlBlockSequenceEntry>,
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
    I: IntoIterator<Item = YamlDocument>,
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
