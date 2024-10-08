use crate::prelude::*;
use biome_grit_syntax::GritDefinitionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritDefinitionList;
impl FormatRule<GritDefinitionList> for FormatGritDefinitionList {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritDefinitionList, f: &mut GritFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for definition in node {
            let definition = definition?;

            join.entry(
                definition.syntax(),
                &format_or_verbatim(definition.format()),
            );
        }

        join.finish()
    }
}
