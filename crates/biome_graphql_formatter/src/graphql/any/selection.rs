//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_graphql_syntax::AnyGraphqlSelection;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGraphqlSelection;
impl FormatRule<AnyGraphqlSelection> for FormatAnyGraphqlSelection {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &AnyGraphqlSelection, f: &mut GraphqlFormatter) -> FormatResult<()> {
        match node {
            AnyGraphqlSelection::GraphqlBogusSelection(node) => node.format().fmt(f),
            AnyGraphqlSelection::GraphqlField(node) => node.format().fmt(f),
            AnyGraphqlSelection::GraphqlFragmentSpread(node) => node.format().fmt(f),
            AnyGraphqlSelection::GraphqlInlineFragment(node) => node.format().fmt(f),
        }
    }
}
