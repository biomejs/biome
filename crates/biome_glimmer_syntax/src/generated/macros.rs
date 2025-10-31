//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](biome_rowan::SyntaxNode::kind)"]
#[doc = r" of the provided [biome_rowan::SyntaxNode] and constructs the appropriate"]
#[doc = r" AstNode type for it, then execute the provided expression over it."]
#[doc = r""]
#[doc = r" # Examples"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r" map_syntax_node!(syntax_node, node => node.format())"]
#[doc = r" ```"]
#[macro_export]
macro_rules! map_syntax_node {
    ($ node : expr , $ pattern : pat => $ body : expr) => {
        match $node {
            node => match $crate::GlimmerSyntaxNode::kind(&node) {
                $crate::GlimmerSyntaxKind::GLIMMER_AT_HEAD => {
                    let $pattern = unsafe { $crate::GlimmerAtHead::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_ATTRIBUTE_NODE => {
                    let $pattern = unsafe { $crate::GlimmerAttributeNode::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_BLOCK => {
                    let $pattern = unsafe { $crate::GlimmerBlock::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_BLOCK_PARAMS => {
                    let $pattern = unsafe { $crate::GlimmerBlockParams::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_BLOCK_STATEMENT => {
                    let $pattern = unsafe { $crate::GlimmerBlockStatement::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_BOOLEAN_LITERAL => {
                    let $pattern = unsafe { $crate::GlimmerBooleanLiteral::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_COMMENT_STATEMENT => {
                    let $pattern = unsafe { $crate::GlimmerCommentStatement::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_CONCAT_STATEMENT => {
                    let $pattern = unsafe { $crate::GlimmerConcatStatement::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_ELEMENT_MODIFIER => {
                    let $pattern = unsafe { $crate::GlimmerElementModifier::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_ELEMENT_NODE => {
                    let $pattern = unsafe { $crate::GlimmerElementNode::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_ELSE_BLOCK => {
                    let $pattern = unsafe { $crate::GlimmerElseBlock::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_END_TAG => {
                    let $pattern = unsafe { $crate::GlimmerEndTag::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_HASH => {
                    let $pattern = unsafe { $crate::GlimmerHash::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_HASH_PAIR => {
                    let $pattern = unsafe { $crate::GlimmerHashPair::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_MUSTACHE_COMMENT_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::GlimmerMustacheCommentStatement::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_MUSTACHE_STATEMENT => {
                    let $pattern = unsafe { $crate::GlimmerMustacheStatement::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_NULL_LITERAL => {
                    let $pattern = unsafe { $crate::GlimmerNullLiteral::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_NUMBER_LITERAL => {
                    let $pattern = unsafe { $crate::GlimmerNumberLiteral::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_PARAM_NAME => {
                    let $pattern = unsafe { $crate::GlimmerParamName::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_PATH_EXPRESSION => {
                    let $pattern = unsafe { $crate::GlimmerPathExpression::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_PATH_SEGMENT => {
                    let $pattern = unsafe { $crate::GlimmerPathSegment::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_ROOT => {
                    let $pattern = unsafe { $crate::GlimmerRoot::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_START_TAG => {
                    let $pattern = unsafe { $crate::GlimmerStartTag::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_STRING_LITERAL => {
                    let $pattern = unsafe { $crate::GlimmerStringLiteral::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_SUB_EXPRESSION => {
                    let $pattern = unsafe { $crate::GlimmerSubExpression::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_TEXT_NODE => {
                    let $pattern = unsafe { $crate::GlimmerTextNode::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_THIS_HEAD => {
                    let $pattern = unsafe { $crate::GlimmerThisHead::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_UNDEFINED_LITERAL => {
                    let $pattern = unsafe { $crate::GlimmerUndefinedLiteral::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_VAR_HEAD => {
                    let $pattern = unsafe { $crate::GlimmerVarHead::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::SELF_CLOSING => {
                    let $pattern = unsafe { $crate::SelfClosing::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_BOGUS => {
                    let $pattern = unsafe { $crate::GlimmerBogus::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_BOGUS_EXPRESSION => {
                    let $pattern = unsafe { $crate::GlimmerBogusExpression::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_BOGUS_STATEMENT => {
                    let $pattern = unsafe { $crate::GlimmerBogusStatement::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_ATTRIBUTE_LIST => {
                    let $pattern = unsafe { $crate::GlimmerAttributeList::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_CONCAT_PART_LIST => {
                    let $pattern = unsafe { $crate::GlimmerConcatPartList::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_ELEMENT_MODIFIER_LIST => {
                    let $pattern =
                        unsafe { $crate::GlimmerElementModifierList::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_HASH_PAIR_LIST => {
                    let $pattern = unsafe { $crate::GlimmerHashPairList::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_PARAM_NAME_LIST => {
                    let $pattern = unsafe { $crate::GlimmerParamNameList::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_PARAMS_LIST => {
                    let $pattern = unsafe { $crate::GlimmerParamsList::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_PATH_SEGMENT_LIST => {
                    let $pattern = unsafe { $crate::GlimmerPathSegmentList::new_unchecked(node) };
                    $body
                }
                $crate::GlimmerSyntaxKind::GLIMMER_STATEMENT_LIST => {
                    let $pattern = unsafe { $crate::GlimmerStatementList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
