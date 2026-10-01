use biome_rowan::{SyntaxError, SyntaxResult, TokenText};

use crate::{
    AnyYamlBlockNode, AnyYamlFlowNode, AnyYamlJsonContent, AnyYamlMappingImplicitKey,
    YamlAliasNode, YamlFlowJsonNode, YamlFlowYamlNode, inner_string_text,
};

impl YamlAliasNode {
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}

impl YamlFlowYamlNode {
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        let scalar = self.content().ok_or(SyntaxError::MissingRequiredChild)?;
        Ok(inner_string_text(&scalar.value_token()?))
    }
}

impl YamlFlowJsonNode {
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        match self.content()? {
            AnyYamlJsonContent::YamlDoubleQuotedScalar(scalar) => {
                Ok(inner_string_text(&scalar.value_token()?))
            }
            AnyYamlJsonContent::YamlSingleQuotedScalar(scalar) => {
                Ok(inner_string_text(&scalar.value_token()?))
            }
            AnyYamlJsonContent::YamlFlowMapping(_) | AnyYamlJsonContent::YamlFlowSequence(_) => {
                Err(SyntaxError::MissingRequiredChild)
            }
        }
    }
}

impl AnyYamlFlowNode {
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        match self {
            Self::YamlAliasNode(node) => node.inner_string_text(),
            Self::YamlFlowYamlNode(node) => node.inner_string_text(),
            Self::YamlFlowJsonNode(node) => node.inner_string_text(),
            Self::YamlBogusFlowNode(_) => Err(SyntaxError::UnexpectedBogusNode),
        }
    }
}

impl AnyYamlMappingImplicitKey {
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        match self {
            Self::YamlAliasNode(node) => node.inner_string_text(),
            Self::YamlFlowYamlNode(node) => node.inner_string_text(),
            Self::YamlFlowJsonNode(node) => node.inner_string_text(),
        }
    }
}

impl AnyYamlBlockNode {
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        match self {
            Self::YamlFlowInBlockNode(node) => node.flow()?.inner_string_text(),
            Self::YamlBlockInBlockNode(_) => Err(SyntaxError::MissingRequiredChild),
            Self::YamlBogusBlockNode(_) => Err(SyntaxError::UnexpectedBogusNode),
        }
    }
}
