use super::*;
use biome_js_syntax::{JsSyntaxNode, TextRange};
use std::rc::Rc;

#[derive(Debug)]
pub struct SemanticModelGlobalBindingData {
    pub(crate) references: Vec<SemanticModelGlobalReferenceData>,
}

#[derive(Debug)]
pub struct SemanticModelGlobalReferenceData {
    pub(crate) range: TextRange,
    pub(crate) ty: SemanticModelReferenceType,
}

pub struct GlobalReference {
    pub(crate) data: Rc<SemanticModelData>,
    pub(crate) global_id: u32,
    pub(crate) id: u32,
}

impl GlobalReference {
    pub fn syntax(&self) -> &JsSyntaxNode {
        let reference = &self.data.globals[self.global_id as usize].references[self.id as usize];
        &self.data.binding_nodes[&reference.range.start()]
    }

    /// Returns if this reference is just reading its binding
    pub fn is_read(&self) -> bool {
        let reference = &self.data.globals[self.global_id as usize].references[self.id as usize];
        matches!(reference.ty, SemanticModelReferenceType::Read { .. })
    }

    /// Returns if this reference is writing its binding
    pub fn is_write(&self) -> bool {
        let reference = &self.data.globals[self.global_id as usize].references[self.id as usize];
        matches!(reference.ty, SemanticModelReferenceType::Write { .. })
    }
}
