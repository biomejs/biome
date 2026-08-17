use crate::{MdHeader, MdSetextHeader};
use biome_rowan::AstNodeList;

impl MdHeader {
    /// Returns the level, as a number, of the current header
    pub fn level(&self) -> usize {
        self.before().len()
    }
}

impl MdSetextHeader {
    pub fn is_level_1(&self) -> bool {
        self.underline_token()
            .is_ok_and(|underline| underline.text_trimmed().starts_with('='))
    }

    pub fn is_level_2(&self) -> bool {
        self.underline_token()
            .is_ok_and(|underline| underline.text_trimmed().starts_with('-'))
    }
}
