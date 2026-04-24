use crate::MdTextual;
use biome_rowan::SyntaxResult;

impl MdTextual {
    pub fn is_empty_and_not_newline(&self) -> SyntaxResult<bool> {
        let token = self.value_token()?;
        Ok(token.text().trim().is_empty() && !self.is_newline()?)
    }

    pub fn is_empty(&self) -> SyntaxResult<bool> {
        let token = self.value_token()?;
        Ok(token.text().trim().is_empty() || self.is_newline()?)
    }

    pub fn is_newline(&self) -> SyntaxResult<bool> {
        let token = self.value_token()?;
        Ok(token.text() == "\n" || token.text() == "\r" || token.text() == "\r\n")
    }
}
