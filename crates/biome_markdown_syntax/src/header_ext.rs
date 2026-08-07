use crate::{MdHeader, MdSetextHeader};
use biome_rowan::AstNodeList;

impl MdHeader {
    /// Returns the level, as a number, of the current header
    pub fn level(&self) -> usize {
        self.before().len()
    }
}

impl MdSetextHeader {
    /// Returns the level, as a number, of the current header.
    ///
    /// The underline determines the level: `=` is level 1, `-` is level 2.
    pub fn level(&self) -> usize {
        match self.underline_token() {
            Ok(underline) if !underline.text().trim_start().starts_with('=') => 2,
            _ => 1,
        }
    }
}
