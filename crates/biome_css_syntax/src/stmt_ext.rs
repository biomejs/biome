use crate::CssSyntaxToken;
use crate::generated::{
    CssDeclarationBlock, CssDeclarationOrAtRuleBlock, CssDeclarationOrRuleBlock,
    CssFontFeatureValuesBlock, CssKeyframesBlock, CssPageAtRuleBlock, CssRuleBlock,
};
use biome_rowan::{AstNodeList, SyntaxResult, declare_node_union};

declare_node_union! {
    pub CssBlockLike = CssKeyframesBlock | CssDeclarationOrAtRuleBlock | CssDeclarationBlock | CssRuleBlock | CssFontFeatureValuesBlock | CssPageAtRuleBlock | CssDeclarationOrRuleBlock
}

impl CssBlockLike {
    /// Retrieves the left curly token "{" of the css block-like.
    pub fn l_curly_token(&self) -> SyntaxResult<CssSyntaxToken> {
        match self {
            Self::CssKeyframesBlock(block) => block.l_curly_token(),
            Self::CssDeclarationOrAtRuleBlock(block) => block.l_curly_token(),
            Self::CssDeclarationBlock(block) => block.l_curly_token(),
            Self::CssRuleBlock(block) => block.l_curly_token(),
            Self::CssFontFeatureValuesBlock(block) => block.l_curly_token(),
            Self::CssPageAtRuleBlock(block) => block.l_curly_token(),
            Self::CssDeclarationOrRuleBlock(block) => block.l_curly_token(),
        }
    }

    /// Retrieves the right curly token "}" of the css block-like.
    pub fn r_curly_token(&self) -> SyntaxResult<CssSyntaxToken> {
        match self {
            Self::CssKeyframesBlock(block) => block.r_curly_token(),
            Self::CssDeclarationOrAtRuleBlock(block) => block.r_curly_token(),
            Self::CssDeclarationBlock(block) => block.r_curly_token(),
            Self::CssRuleBlock(block) => block.r_curly_token(),
            Self::CssFontFeatureValuesBlock(block) => block.r_curly_token(),
            Self::CssPageAtRuleBlock(block) => block.r_curly_token(),
            Self::CssDeclarationOrRuleBlock(block) => block.r_curly_token(),
        }
    }

    /// Checks if the css block-like is empty, even if it may have comments inside.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::CssKeyframesBlock(block) => block.items().is_empty(),
            Self::CssDeclarationOrAtRuleBlock(block) => block.items().is_empty(),
            Self::CssDeclarationBlock(block) => block.declarations().is_empty(),
            Self::CssRuleBlock(block) => block.rules().is_empty(),
            Self::CssFontFeatureValuesBlock(block) => block.items().is_empty(),
            Self::CssPageAtRuleBlock(block) => block.items().is_empty(),
            Self::CssDeclarationOrRuleBlock(block) => block.items().is_empty(),
        }
    }

    /// Checks if the css block-like has only empty declarations, meaning it may have
    /// declarations that are empty or have only semicolons, but no actual declarations.
    /// # Panics
    /// This function will panic if the block-like is empty.
    pub fn has_only_empty_declarations(&self) -> bool {
        debug_assert!(!self.is_empty());

        match self {
            Self::CssDeclarationBlock(block) => block
                .declarations()
                .iter()
                .all(|decl| decl.as_css_empty_declaration().is_some()),
            Self::CssDeclarationOrAtRuleBlock(block) => block
                .items()
                .iter()
                .all(|item| item.as_css_empty_declaration().is_some()),
            Self::CssDeclarationOrRuleBlock(block) => block
                .items()
                .iter()
                .all(|item| item.as_css_empty_declaration().is_some()),
            _ => false,
        }
    }

    /// Checks if the css block-like is empty without comments inside.
    pub fn is_empty_without_comments(&self) -> bool {
        self.is_empty()
            && !self
                .l_curly_token()
                .is_ok_and(|token| token.has_trailing_comments())
            && !self
                .r_curly_token()
                .is_ok_and(|token| token.has_leading_comments())
    }
}
