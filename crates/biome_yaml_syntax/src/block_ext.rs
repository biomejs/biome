use crate::{AnyYamlBlockInBlockContent, AnyYamlBlockNode};

impl AnyYamlBlockNode {
    pub fn is_nested_block_collection(&self) -> bool {
        matches!(
            self,
            Self::YamlBlockInBlockNode(node)
                if matches!(
                    node.content(),
                    Ok(
                        AnyYamlBlockInBlockContent::YamlBlockMapping(_)
                            | AnyYamlBlockInBlockContent::YamlBlockSequence(_)
                    )
                )
        )
    }
}
