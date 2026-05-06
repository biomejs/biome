//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

#![allow(clippy::use_self)]
#![expect(clippy::default_constructed_unit_structs)]
use crate::{
    AsFormat, FormatBogusNodeRule, FormatNodeRule, IntoFormat, YamlFormatContext, YamlFormatter,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<biome_yaml_syntax::YamlAliasNode>
    for crate::yaml::auxiliary::alias_node::FormatYamlAliasNode
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlAliasNode,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlAliasNode>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlAliasNode {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlAliasNode,
        crate::yaml::auxiliary::alias_node::FormatYamlAliasNode,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::alias_node::FormatYamlAliasNode::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlAliasNode {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlAliasNode,
        crate::yaml::auxiliary::alias_node::FormatYamlAliasNode,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::alias_node::FormatYamlAliasNode::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlAnchorProperty>
    for crate::yaml::auxiliary::anchor_property::FormatYamlAnchorProperty
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlAnchorProperty,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlAnchorProperty>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlAnchorProperty {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlAnchorProperty,
        crate::yaml::auxiliary::anchor_property::FormatYamlAnchorProperty,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::anchor_property::FormatYamlAnchorProperty::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlAnchorProperty {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlAnchorProperty,
        crate::yaml::auxiliary::anchor_property::FormatYamlAnchorProperty,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::anchor_property::FormatYamlAnchorProperty::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlBlockContent>
    for crate::yaml::auxiliary::block_content::FormatYamlBlockContent
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlBlockContent,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlBlockContent>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockContent {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlBlockContent,
        crate::yaml::auxiliary::block_content::FormatYamlBlockContent,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::block_content::FormatYamlBlockContent::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockContent {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlBlockContent,
        crate::yaml::auxiliary::block_content::FormatYamlBlockContent,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::block_content::FormatYamlBlockContent::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlBlockInBlockNode>
    for crate::yaml::auxiliary::block_in_block_node::FormatYamlBlockInBlockNode
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlBlockInBlockNode,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlBlockInBlockNode>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockInBlockNode {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlBlockInBlockNode,
        crate::yaml::auxiliary::block_in_block_node::FormatYamlBlockInBlockNode,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::block_in_block_node::FormatYamlBlockInBlockNode::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockInBlockNode {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlBlockInBlockNode,
        crate::yaml::auxiliary::block_in_block_node::FormatYamlBlockInBlockNode,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::block_in_block_node::FormatYamlBlockInBlockNode::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlBlockKeepIndicator>
    for crate::yaml::auxiliary::block_keep_indicator::FormatYamlBlockKeepIndicator
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlBlockKeepIndicator,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlBlockKeepIndicator>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockKeepIndicator {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlBlockKeepIndicator,
        crate::yaml::auxiliary::block_keep_indicator::FormatYamlBlockKeepIndicator,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::block_keep_indicator::FormatYamlBlockKeepIndicator::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockKeepIndicator {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlBlockKeepIndicator,
        crate::yaml::auxiliary::block_keep_indicator::FormatYamlBlockKeepIndicator,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::block_keep_indicator::FormatYamlBlockKeepIndicator::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlBlockMapExplicitEntry>
    for crate::yaml::auxiliary::block_map_explicit_entry::FormatYamlBlockMapExplicitEntry
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlBlockMapExplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlBlockMapExplicitEntry>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockMapExplicitEntry {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlBlockMapExplicitEntry,
        crate::yaml::auxiliary::block_map_explicit_entry::FormatYamlBlockMapExplicitEntry,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: yaml :: auxiliary :: block_map_explicit_entry :: FormatYamlBlockMapExplicitEntry :: default ())
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockMapExplicitEntry {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlBlockMapExplicitEntry,
        crate::yaml::auxiliary::block_map_explicit_entry::FormatYamlBlockMapExplicitEntry,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: yaml :: auxiliary :: block_map_explicit_entry :: FormatYamlBlockMapExplicitEntry :: default ())
    }
}
impl FormatRule<biome_yaml_syntax::YamlBlockMapImplicitEntry>
    for crate::yaml::auxiliary::block_map_implicit_entry::FormatYamlBlockMapImplicitEntry
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlBlockMapImplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlBlockMapImplicitEntry>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockMapImplicitEntry {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlBlockMapImplicitEntry,
        crate::yaml::auxiliary::block_map_implicit_entry::FormatYamlBlockMapImplicitEntry,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: yaml :: auxiliary :: block_map_implicit_entry :: FormatYamlBlockMapImplicitEntry :: default ())
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockMapImplicitEntry {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlBlockMapImplicitEntry,
        crate::yaml::auxiliary::block_map_implicit_entry::FormatYamlBlockMapImplicitEntry,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: yaml :: auxiliary :: block_map_implicit_entry :: FormatYamlBlockMapImplicitEntry :: default ())
    }
}
impl FormatRule<biome_yaml_syntax::YamlBlockMapping>
    for crate::yaml::auxiliary::block_mapping::FormatYamlBlockMapping
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlBlockMapping,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlBlockMapping>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockMapping {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlBlockMapping,
        crate::yaml::auxiliary::block_mapping::FormatYamlBlockMapping,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::block_mapping::FormatYamlBlockMapping::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockMapping {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlBlockMapping,
        crate::yaml::auxiliary::block_mapping::FormatYamlBlockMapping,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::block_mapping::FormatYamlBlockMapping::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlBlockSequence>
    for crate::yaml::auxiliary::block_sequence::FormatYamlBlockSequence
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlBlockSequence,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlBlockSequence>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockSequence {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlBlockSequence,
        crate::yaml::auxiliary::block_sequence::FormatYamlBlockSequence,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::block_sequence::FormatYamlBlockSequence::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockSequence {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlBlockSequence,
        crate::yaml::auxiliary::block_sequence::FormatYamlBlockSequence,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::block_sequence::FormatYamlBlockSequence::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlBlockSequenceEntry>
    for crate::yaml::auxiliary::block_sequence_entry::FormatYamlBlockSequenceEntry
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlBlockSequenceEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlBlockSequenceEntry>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockSequenceEntry {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlBlockSequenceEntry,
        crate::yaml::auxiliary::block_sequence_entry::FormatYamlBlockSequenceEntry,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::block_sequence_entry::FormatYamlBlockSequenceEntry::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockSequenceEntry {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlBlockSequenceEntry,
        crate::yaml::auxiliary::block_sequence_entry::FormatYamlBlockSequenceEntry,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::block_sequence_entry::FormatYamlBlockSequenceEntry::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlBlockStripIndicator>
    for crate::yaml::auxiliary::block_strip_indicator::FormatYamlBlockStripIndicator
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlBlockStripIndicator,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlBlockStripIndicator>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockStripIndicator {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlBlockStripIndicator,
        crate::yaml::auxiliary::block_strip_indicator::FormatYamlBlockStripIndicator,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::block_strip_indicator::FormatYamlBlockStripIndicator::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockStripIndicator {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlBlockStripIndicator,
        crate::yaml::auxiliary::block_strip_indicator::FormatYamlBlockStripIndicator,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::block_strip_indicator::FormatYamlBlockStripIndicator::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlDirective>
    for crate::yaml::auxiliary::directive::FormatYamlDirective
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlDirective,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlDirective>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlDirective,
        crate::yaml::auxiliary::directive::FormatYamlDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::directive::FormatYamlDirective::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlDirective {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlDirective,
        crate::yaml::auxiliary::directive::FormatYamlDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::directive::FormatYamlDirective::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlDocument>
    for crate::yaml::auxiliary::document::FormatYamlDocument
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlDocument,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlDocument>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlDocument {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlDocument,
        crate::yaml::auxiliary::document::FormatYamlDocument,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::document::FormatYamlDocument::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlDocument {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlDocument,
        crate::yaml::auxiliary::document::FormatYamlDocument,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::document::FormatYamlDocument::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlDoubleQuotedScalar>
    for crate::yaml::auxiliary::double_quoted_scalar::FormatYamlDoubleQuotedScalar
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlDoubleQuotedScalar,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlDoubleQuotedScalar>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlDoubleQuotedScalar {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlDoubleQuotedScalar,
        crate::yaml::auxiliary::double_quoted_scalar::FormatYamlDoubleQuotedScalar,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::double_quoted_scalar::FormatYamlDoubleQuotedScalar::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlDoubleQuotedScalar {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlDoubleQuotedScalar,
        crate::yaml::auxiliary::double_quoted_scalar::FormatYamlDoubleQuotedScalar,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::double_quoted_scalar::FormatYamlDoubleQuotedScalar::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlFlowInBlockNode>
    for crate::yaml::auxiliary::flow_in_block_node::FormatYamlFlowInBlockNode
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlFlowInBlockNode,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlFlowInBlockNode>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowInBlockNode {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlFlowInBlockNode,
        crate::yaml::auxiliary::flow_in_block_node::FormatYamlFlowInBlockNode,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::flow_in_block_node::FormatYamlFlowInBlockNode::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowInBlockNode {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlFlowInBlockNode,
        crate::yaml::auxiliary::flow_in_block_node::FormatYamlFlowInBlockNode,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::flow_in_block_node::FormatYamlFlowInBlockNode::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlFlowJsonNode>
    for crate::yaml::auxiliary::flow_json_node::FormatYamlFlowJsonNode
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlFlowJsonNode,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlFlowJsonNode>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowJsonNode {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlFlowJsonNode,
        crate::yaml::auxiliary::flow_json_node::FormatYamlFlowJsonNode,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::flow_json_node::FormatYamlFlowJsonNode::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowJsonNode {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlFlowJsonNode,
        crate::yaml::auxiliary::flow_json_node::FormatYamlFlowJsonNode,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::flow_json_node::FormatYamlFlowJsonNode::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlFlowMapExplicitEntry>
    for crate::yaml::auxiliary::flow_map_explicit_entry::FormatYamlFlowMapExplicitEntry
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlFlowMapExplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlFlowMapExplicitEntry>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowMapExplicitEntry {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlFlowMapExplicitEntry,
        crate::yaml::auxiliary::flow_map_explicit_entry::FormatYamlFlowMapExplicitEntry,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: yaml :: auxiliary :: flow_map_explicit_entry :: FormatYamlFlowMapExplicitEntry :: default ())
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowMapExplicitEntry {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlFlowMapExplicitEntry,
        crate::yaml::auxiliary::flow_map_explicit_entry::FormatYamlFlowMapExplicitEntry,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: yaml :: auxiliary :: flow_map_explicit_entry :: FormatYamlFlowMapExplicitEntry :: default ())
    }
}
impl FormatRule<biome_yaml_syntax::YamlFlowMapImplicitEntry>
    for crate::yaml::auxiliary::flow_map_implicit_entry::FormatYamlFlowMapImplicitEntry
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlFlowMapImplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlFlowMapImplicitEntry>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowMapImplicitEntry {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlFlowMapImplicitEntry,
        crate::yaml::auxiliary::flow_map_implicit_entry::FormatYamlFlowMapImplicitEntry,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: yaml :: auxiliary :: flow_map_implicit_entry :: FormatYamlFlowMapImplicitEntry :: default ())
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowMapImplicitEntry {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlFlowMapImplicitEntry,
        crate::yaml::auxiliary::flow_map_implicit_entry::FormatYamlFlowMapImplicitEntry,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: yaml :: auxiliary :: flow_map_implicit_entry :: FormatYamlFlowMapImplicitEntry :: default ())
    }
}
impl FormatRule<biome_yaml_syntax::YamlFlowMapping>
    for crate::yaml::auxiliary::flow_mapping::FormatYamlFlowMapping
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlFlowMapping,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlFlowMapping>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowMapping {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlFlowMapping,
        crate::yaml::auxiliary::flow_mapping::FormatYamlFlowMapping,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::flow_mapping::FormatYamlFlowMapping::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowMapping {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlFlowMapping,
        crate::yaml::auxiliary::flow_mapping::FormatYamlFlowMapping,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::flow_mapping::FormatYamlFlowMapping::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlFlowSequence>
    for crate::yaml::auxiliary::flow_sequence::FormatYamlFlowSequence
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlFlowSequence,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlFlowSequence>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowSequence {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlFlowSequence,
        crate::yaml::auxiliary::flow_sequence::FormatYamlFlowSequence,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::flow_sequence::FormatYamlFlowSequence::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowSequence {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlFlowSequence,
        crate::yaml::auxiliary::flow_sequence::FormatYamlFlowSequence,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::flow_sequence::FormatYamlFlowSequence::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlFlowYamlNode>
    for crate::yaml::auxiliary::flow_yaml_node::FormatYamlFlowYamlNode
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlFlowYamlNode,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlFlowYamlNode>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowYamlNode {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlFlowYamlNode,
        crate::yaml::auxiliary::flow_yaml_node::FormatYamlFlowYamlNode,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::flow_yaml_node::FormatYamlFlowYamlNode::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowYamlNode {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlFlowYamlNode,
        crate::yaml::auxiliary::flow_yaml_node::FormatYamlFlowYamlNode,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::flow_yaml_node::FormatYamlFlowYamlNode::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlFoldedScalar>
    for crate::yaml::auxiliary::folded_scalar::FormatYamlFoldedScalar
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlFoldedScalar,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlFoldedScalar>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlFoldedScalar {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlFoldedScalar,
        crate::yaml::auxiliary::folded_scalar::FormatYamlFoldedScalar,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::folded_scalar::FormatYamlFoldedScalar::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlFoldedScalar {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlFoldedScalar,
        crate::yaml::auxiliary::folded_scalar::FormatYamlFoldedScalar,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::folded_scalar::FormatYamlFoldedScalar::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlIndentationIndicator>
    for crate::yaml::auxiliary::indentation_indicator::FormatYamlIndentationIndicator
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlIndentationIndicator,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlIndentationIndicator>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlIndentationIndicator {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlIndentationIndicator,
        crate::yaml::auxiliary::indentation_indicator::FormatYamlIndentationIndicator,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::indentation_indicator::FormatYamlIndentationIndicator::default(
            ),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlIndentationIndicator {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlIndentationIndicator,
        crate::yaml::auxiliary::indentation_indicator::FormatYamlIndentationIndicator,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::indentation_indicator::FormatYamlIndentationIndicator::default(
            ),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlLiteralScalar>
    for crate::yaml::auxiliary::literal_scalar::FormatYamlLiteralScalar
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlLiteralScalar,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlLiteralScalar>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlLiteralScalar {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlLiteralScalar,
        crate::yaml::auxiliary::literal_scalar::FormatYamlLiteralScalar,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::literal_scalar::FormatYamlLiteralScalar::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlLiteralScalar {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlLiteralScalar,
        crate::yaml::auxiliary::literal_scalar::FormatYamlLiteralScalar,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::literal_scalar::FormatYamlLiteralScalar::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlPlainScalar>
    for crate::yaml::auxiliary::plain_scalar::FormatYamlPlainScalar
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlPlainScalar,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlPlainScalar>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlPlainScalar {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlPlainScalar,
        crate::yaml::auxiliary::plain_scalar::FormatYamlPlainScalar,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::plain_scalar::FormatYamlPlainScalar::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlPlainScalar {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlPlainScalar,
        crate::yaml::auxiliary::plain_scalar::FormatYamlPlainScalar,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::plain_scalar::FormatYamlPlainScalar::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlRoot> for crate::yaml::auxiliary::root::FormatYamlRoot {
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_yaml_syntax::YamlRoot, f: &mut YamlFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlRoot>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlRoot {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlRoot,
        crate::yaml::auxiliary::root::FormatYamlRoot,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::root::FormatYamlRoot::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlRoot {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlRoot,
        crate::yaml::auxiliary::root::FormatYamlRoot,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::root::FormatYamlRoot::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlSingleQuotedScalar>
    for crate::yaml::auxiliary::single_quoted_scalar::FormatYamlSingleQuotedScalar
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlSingleQuotedScalar,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlSingleQuotedScalar>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlSingleQuotedScalar {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlSingleQuotedScalar,
        crate::yaml::auxiliary::single_quoted_scalar::FormatYamlSingleQuotedScalar,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::single_quoted_scalar::FormatYamlSingleQuotedScalar::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlSingleQuotedScalar {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlSingleQuotedScalar,
        crate::yaml::auxiliary::single_quoted_scalar::FormatYamlSingleQuotedScalar,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::single_quoted_scalar::FormatYamlSingleQuotedScalar::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlTagProperty>
    for crate::yaml::auxiliary::tag_property::FormatYamlTagProperty
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlTagProperty,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_yaml_syntax::YamlTagProperty>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlTagProperty {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlTagProperty,
        crate::yaml::auxiliary::tag_property::FormatYamlTagProperty,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::auxiliary::tag_property::FormatYamlTagProperty::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlTagProperty {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlTagProperty,
        crate::yaml::auxiliary::tag_property::FormatYamlTagProperty,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::auxiliary::tag_property::FormatYamlTagProperty::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockHeaderList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlBlockHeaderList,
        crate::yaml::lists::block_header_list::FormatYamlBlockHeaderList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::lists::block_header_list::FormatYamlBlockHeaderList::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockHeaderList {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlBlockHeaderList,
        crate::yaml::lists::block_header_list::FormatYamlBlockHeaderList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::lists::block_header_list::FormatYamlBlockHeaderList::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockMapEntryList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlBlockMapEntryList,
        crate::yaml::lists::block_map_entry_list::FormatYamlBlockMapEntryList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::lists::block_map_entry_list::FormatYamlBlockMapEntryList::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockMapEntryList {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlBlockMapEntryList,
        crate::yaml::lists::block_map_entry_list::FormatYamlBlockMapEntryList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::lists::block_map_entry_list::FormatYamlBlockMapEntryList::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockSequenceEntryList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlBlockSequenceEntryList,
        crate::yaml::lists::block_sequence_entry_list::FormatYamlBlockSequenceEntryList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: yaml :: lists :: block_sequence_entry_list :: FormatYamlBlockSequenceEntryList :: default ())
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlBlockSequenceEntryList {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlBlockSequenceEntryList,
        crate::yaml::lists::block_sequence_entry_list::FormatYamlBlockSequenceEntryList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: yaml :: lists :: block_sequence_entry_list :: FormatYamlBlockSequenceEntryList :: default ())
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlDirectiveList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlDirectiveList,
        crate::yaml::lists::directive_list::FormatYamlDirectiveList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::lists::directive_list::FormatYamlDirectiveList::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlDirectiveList {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlDirectiveList,
        crate::yaml::lists::directive_list::FormatYamlDirectiveList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::lists::directive_list::FormatYamlDirectiveList::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlDocumentList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlDocumentList,
        crate::yaml::lists::document_list::FormatYamlDocumentList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::lists::document_list::FormatYamlDocumentList::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlDocumentList {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlDocumentList,
        crate::yaml::lists::document_list::FormatYamlDocumentList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::lists::document_list::FormatYamlDocumentList::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowMapEntryList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlFlowMapEntryList,
        crate::yaml::lists::flow_map_entry_list::FormatYamlFlowMapEntryList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::lists::flow_map_entry_list::FormatYamlFlowMapEntryList::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowMapEntryList {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlFlowMapEntryList,
        crate::yaml::lists::flow_map_entry_list::FormatYamlFlowMapEntryList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::lists::flow_map_entry_list::FormatYamlFlowMapEntryList::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowSequenceEntryList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlFlowSequenceEntryList,
        crate::yaml::lists::flow_sequence_entry_list::FormatYamlFlowSequenceEntryList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::lists::flow_sequence_entry_list::FormatYamlFlowSequenceEntryList::default(
            ),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlFlowSequenceEntryList {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlFlowSequenceEntryList,
        crate::yaml::lists::flow_sequence_entry_list::FormatYamlFlowSequenceEntryList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::lists::flow_sequence_entry_list::FormatYamlFlowSequenceEntryList::default(
            ),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlPropertyList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlPropertyList,
        crate::yaml::lists::property_list::FormatYamlPropertyList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::lists::property_list::FormatYamlPropertyList::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlPropertyList {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlPropertyList,
        crate::yaml::lists::property_list::FormatYamlPropertyList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::lists::property_list::FormatYamlPropertyList::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlBogus> for crate::yaml::bogus::bogus::FormatYamlBogus {
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_yaml_syntax::YamlBogus, f: &mut YamlFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_yaml_syntax::YamlBogus>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlBogus {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlBogus,
        crate::yaml::bogus::bogus::FormatYamlBogus,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::yaml::bogus::bogus::FormatYamlBogus::default())
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlBogus {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlBogus,
        crate::yaml::bogus::bogus::FormatYamlBogus,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::yaml::bogus::bogus::FormatYamlBogus::default())
    }
}
impl FormatRule<biome_yaml_syntax::YamlBogusBlockHeader>
    for crate::yaml::bogus::bogus_block_header::FormatYamlBogusBlockHeader
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlBogusBlockHeader,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_yaml_syntax::YamlBogusBlockHeader>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlBogusBlockHeader {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlBogusBlockHeader,
        crate::yaml::bogus::bogus_block_header::FormatYamlBogusBlockHeader,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::bogus::bogus_block_header::FormatYamlBogusBlockHeader::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlBogusBlockHeader {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlBogusBlockHeader,
        crate::yaml::bogus::bogus_block_header::FormatYamlBogusBlockHeader,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::bogus::bogus_block_header::FormatYamlBogusBlockHeader::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlBogusBlockMapEntry>
    for crate::yaml::bogus::bogus_block_map_entry::FormatYamlBogusBlockMapEntry
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlBogusBlockMapEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_yaml_syntax::YamlBogusBlockMapEntry>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlBogusBlockMapEntry {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlBogusBlockMapEntry,
        crate::yaml::bogus::bogus_block_map_entry::FormatYamlBogusBlockMapEntry,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::bogus::bogus_block_map_entry::FormatYamlBogusBlockMapEntry::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlBogusBlockMapEntry {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlBogusBlockMapEntry,
        crate::yaml::bogus::bogus_block_map_entry::FormatYamlBogusBlockMapEntry,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::bogus::bogus_block_map_entry::FormatYamlBogusBlockMapEntry::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlBogusBlockNode>
    for crate::yaml::bogus::bogus_block_node::FormatYamlBogusBlockNode
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlBogusBlockNode,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_yaml_syntax::YamlBogusBlockNode>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlBogusBlockNode {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlBogusBlockNode,
        crate::yaml::bogus::bogus_block_node::FormatYamlBogusBlockNode,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::bogus::bogus_block_node::FormatYamlBogusBlockNode::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlBogusBlockNode {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlBogusBlockNode,
        crate::yaml::bogus::bogus_block_node::FormatYamlBogusBlockNode,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::bogus::bogus_block_node::FormatYamlBogusBlockNode::default(),
        )
    }
}
impl FormatRule<biome_yaml_syntax::YamlBogusFlowNode>
    for crate::yaml::bogus::bogus_flow_node::FormatYamlBogusFlowNode
{
    type Context = YamlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_yaml_syntax::YamlBogusFlowNode,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_yaml_syntax::YamlBogusFlowNode>::fmt(self, node, f)
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::YamlBogusFlowNode {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::YamlBogusFlowNode,
        crate::yaml::bogus::bogus_flow_node::FormatYamlBogusFlowNode,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::bogus::bogus_flow_node::FormatYamlBogusFlowNode::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::YamlBogusFlowNode {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::YamlBogusFlowNode,
        crate::yaml::bogus::bogus_flow_node::FormatYamlBogusFlowNode,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::bogus::bogus_flow_node::FormatYamlBogusFlowNode::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlBlockHeader {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::AnyYamlBlockHeader,
        crate::yaml::any::block_header::FormatAnyYamlBlockHeader,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::any::block_header::FormatAnyYamlBlockHeader::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlBlockHeader {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::AnyYamlBlockHeader,
        crate::yaml::any::block_header::FormatAnyYamlBlockHeader,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::any::block_header::FormatAnyYamlBlockHeader::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlBlockInBlockContent {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::AnyYamlBlockInBlockContent,
        crate::yaml::any::block_in_block_content::FormatAnyYamlBlockInBlockContent,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::any::block_in_block_content::FormatAnyYamlBlockInBlockContent::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlBlockInBlockContent {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::AnyYamlBlockInBlockContent,
        crate::yaml::any::block_in_block_content::FormatAnyYamlBlockInBlockContent,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::any::block_in_block_content::FormatAnyYamlBlockInBlockContent::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlBlockMapEntry {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::AnyYamlBlockMapEntry,
        crate::yaml::any::block_map_entry::FormatAnyYamlBlockMapEntry,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::any::block_map_entry::FormatAnyYamlBlockMapEntry::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlBlockMapEntry {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::AnyYamlBlockMapEntry,
        crate::yaml::any::block_map_entry::FormatAnyYamlBlockMapEntry,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::any::block_map_entry::FormatAnyYamlBlockMapEntry::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlBlockNode {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::AnyYamlBlockNode,
        crate::yaml::any::block_node::FormatAnyYamlBlockNode,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::any::block_node::FormatAnyYamlBlockNode::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlBlockNode {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::AnyYamlBlockNode,
        crate::yaml::any::block_node::FormatAnyYamlBlockNode,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::any::block_node::FormatAnyYamlBlockNode::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlBlockSequenceEntry {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::AnyYamlBlockSequenceEntry,
        crate::yaml::any::block_sequence_entry::FormatAnyYamlBlockSequenceEntry,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::any::block_sequence_entry::FormatAnyYamlBlockSequenceEntry::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlBlockSequenceEntry {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::AnyYamlBlockSequenceEntry,
        crate::yaml::any::block_sequence_entry::FormatAnyYamlBlockSequenceEntry,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::any::block_sequence_entry::FormatAnyYamlBlockSequenceEntry::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlDocument {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::AnyYamlDocument,
        crate::yaml::any::document::FormatAnyYamlDocument,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::any::document::FormatAnyYamlDocument::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlDocument {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::AnyYamlDocument,
        crate::yaml::any::document::FormatAnyYamlDocument,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::any::document::FormatAnyYamlDocument::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlFlowMapEntry {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::AnyYamlFlowMapEntry,
        crate::yaml::any::flow_map_entry::FormatAnyYamlFlowMapEntry,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::any::flow_map_entry::FormatAnyYamlFlowMapEntry::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlFlowMapEntry {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::AnyYamlFlowMapEntry,
        crate::yaml::any::flow_map_entry::FormatAnyYamlFlowMapEntry,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::any::flow_map_entry::FormatAnyYamlFlowMapEntry::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlFlowNode {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::AnyYamlFlowNode,
        crate::yaml::any::flow_node::FormatAnyYamlFlowNode,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::any::flow_node::FormatAnyYamlFlowNode::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlFlowNode {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::AnyYamlFlowNode,
        crate::yaml::any::flow_node::FormatAnyYamlFlowNode,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::any::flow_node::FormatAnyYamlFlowNode::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlFlowSequenceEntry {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::AnyYamlFlowSequenceEntry,
        crate::yaml::any::flow_sequence_entry::FormatAnyYamlFlowSequenceEntry,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::any::flow_sequence_entry::FormatAnyYamlFlowSequenceEntry::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlFlowSequenceEntry {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::AnyYamlFlowSequenceEntry,
        crate::yaml::any::flow_sequence_entry::FormatAnyYamlFlowSequenceEntry,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::any::flow_sequence_entry::FormatAnyYamlFlowSequenceEntry::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlJsonContent {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::AnyYamlJsonContent,
        crate::yaml::any::json_content::FormatAnyYamlJsonContent,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::any::json_content::FormatAnyYamlJsonContent::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlJsonContent {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::AnyYamlJsonContent,
        crate::yaml::any::json_content::FormatAnyYamlJsonContent,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::any::json_content::FormatAnyYamlJsonContent::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlMappingImplicitKey {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::AnyYamlMappingImplicitKey,
        crate::yaml::any::mapping_implicit_key::FormatAnyYamlMappingImplicitKey,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::any::mapping_implicit_key::FormatAnyYamlMappingImplicitKey::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlMappingImplicitKey {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::AnyYamlMappingImplicitKey,
        crate::yaml::any::mapping_implicit_key::FormatAnyYamlMappingImplicitKey,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::any::mapping_implicit_key::FormatAnyYamlMappingImplicitKey::default(),
        )
    }
}
impl AsFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlProperty {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_yaml_syntax::AnyYamlProperty,
        crate::yaml::any::property::FormatAnyYamlProperty,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::yaml::any::property::FormatAnyYamlProperty::default(),
        )
    }
}
impl IntoFormat<YamlFormatContext> for biome_yaml_syntax::AnyYamlProperty {
    type Format = FormatOwnedWithRule<
        biome_yaml_syntax::AnyYamlProperty,
        crate::yaml::any::property::FormatAnyYamlProperty,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::yaml::any::property::FormatAnyYamlProperty::default(),
        )
    }
}
