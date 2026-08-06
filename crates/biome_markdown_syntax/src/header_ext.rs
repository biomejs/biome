use crate::MdHeader;
use biome_rowan::AstNodeList;

impl MdHeader {
    /// Returns the level, as a number, of the current header
    pub fn level(&self) -> usize {
        self.before().len()
    }
}
