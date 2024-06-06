use crate::generated::{
    CssDeclarationBlock, CssDeclarationOrAtRuleBlock, CssDeclarationOrRuleBlock,
    CssFontFeatureValuesBlock, CssKeyframesBlock, CssPageAtRuleBlock, CssRuleBlock,
};
use crate::CssSyntaxToken;
use biome_rowan::{declare_node_union, AstNodeList, SyntaxResult};

declare_node_union! {
    pub CssBlockLike = CssKeyframesBlock | CssDeclarationOrAtRuleBlock | CssDeclarationBlock | CssRuleBlock | CssFontFeatureValuesBlock | CssPageAtRuleBlock | CssDeclarationOrRuleBlock
}

impl CssBlockLike {
    pub fn l_curly_token(&self) -> SyntaxResult<CssSyntaxToken> {
        match self {
            CssBlockLike::CssKeyframesBlock(block) => block.l_curly_token(),
            CssBlockLike::CssDeclarationOrAtRuleBlock(block) => block.l_curly_token(),
            CssBlockLike::CssDeclarationBlock(block) => block.l_curly_token(),
            CssBlockLike::CssRuleBlock(block) => block.l_curly_token(),
            CssBlockLike::CssFontFeatureValuesBlock(block) => block.l_curly_token(),
            CssBlockLike::CssPageAtRuleBlock(block) => block.l_curly_token(),
            CssBlockLike::CssDeclarationOrRuleBlock(block) => block.l_curly_token(),
        }
    }

    pub fn r_curly_token(&self) -> SyntaxResult<CssSyntaxToken> {
        match self {
            CssBlockLike::CssKeyframesBlock(block) => block.r_curly_token(),
            CssBlockLike::CssDeclarationOrAtRuleBlock(block) => block.r_curly_token(),
            CssBlockLike::CssDeclarationBlock(block) => block.r_curly_token(),
            CssBlockLike::CssRuleBlock(block) => block.r_curly_token(),
            CssBlockLike::CssFontFeatureValuesBlock(block) => block.r_curly_token(),
            CssBlockLike::CssPageAtRuleBlock(block) => block.r_curly_token(),
            CssBlockLike::CssDeclarationOrRuleBlock(block) => block.r_curly_token(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            CssBlockLike::CssKeyframesBlock(block) => block.items().is_empty(),
            CssBlockLike::CssDeclarationOrAtRuleBlock(block) => block.items().is_empty(),
            CssBlockLike::CssDeclarationBlock(block) => block.declarations().is_empty(),
            CssBlockLike::CssRuleBlock(block) => block.rules().is_empty(),
            CssBlockLike::CssFontFeatureValuesBlock(block) => block.items().is_empty(),
            CssBlockLike::CssPageAtRuleBlock(block) => block.items().is_empty(),
            CssBlockLike::CssDeclarationOrRuleBlock(block) => block.items().is_empty(),
        }
    }

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
