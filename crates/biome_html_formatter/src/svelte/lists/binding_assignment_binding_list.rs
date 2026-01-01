use crate::prelude::*;
use biome_html_syntax::{AnySvelteBindingAssignmentBinding, SvelteBindingAssignmentBindingList};
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

        for binding_assignment in node.iter().flatten() {
            match binding_assignment {
                AnySvelteBindingAssignmentBinding::SvelteName(name) => {
                    join.entry(name.syntax(), &name.format())
                }
                AnySvelteBindingAssignmentBinding::SvelteRestBinding(binding) => {
                    join.entry(binding.syntax(), &binding.format())
                }
            }
        }

        join.finish()
    }
}
