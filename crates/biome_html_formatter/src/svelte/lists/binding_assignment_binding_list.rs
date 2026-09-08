use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::SvelteBindingAssignmentBindingList;
use biome_rowan::AstSeparatedList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteBindingAssignmentBindingList;
impl FormatRule<SvelteBindingAssignmentBindingList> for FormatSvelteBindingAssignmentBindingList {
    type Context = HtmlFormatContext;
    fn fmt(
        &self,
        node: &SvelteBindingAssignmentBindingList,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let mut join = f.join_nodes_with_space();

        for binding_assignment in node.elements() {
            let separator = binding_assignment.trailing_separator()?;

            match binding_assignment.node() {
                Ok(node) => join.entry(
                    node.syntax(),
                    &format_with(|f| {
                        write!(f, [node.format()])?;

                        if let Some(separator) = separator {
                            write!(f, [separator.format()])?;
                        }

                        Ok(())
                    }),
                ),
                Err(error) => {
                    let separator = separator.ok_or(error)?;
                    let parent = separator.parent().ok_or(error)?;
                    join.entry(&parent, &separator.format());
                }
            }
        }

        join.finish()
    }
}
