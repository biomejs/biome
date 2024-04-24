use crate::grit_node::GritNode;
use grit_util::Language;

pub(crate) struct GritLanguage;

impl Language for GritLanguage {
    type Node<'a> = GritNode;

    fn language_name(&self) -> &'static str {
        todo!()
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        todo!()
    }

    fn is_comment(&self, _node: &Self::Node<'_>) -> bool {
        todo!()
    }

    fn is_metavariable(&self, _node: &Self::Node<'_>) -> bool {
        todo!()
    }
}
