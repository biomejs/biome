//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
use biome_glimmer_syntax::{
    GlimmerSyntaxElement as SyntaxElement, GlimmerSyntaxNode as SyntaxNode,
    GlimmerSyntaxToken as SyntaxToken, *,
};
use biome_rowan::AstNode;
pub fn glimmer_at_head(at_token: SyntaxToken, name_token: SyntaxToken) -> GlimmerAtHead {
    GlimmerAtHead::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_AT_HEAD,
        [
            Some(SyntaxElement::Token(at_token)),
            Some(SyntaxElement::Token(name_token)),
        ],
    ))
}
pub fn glimmer_attribute_node(
    name_token: SyntaxToken,
    eq_token: SyntaxToken,
    value: GlimmerAttributeValue,
) -> GlimmerAttributeNode {
    GlimmerAttributeNode::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_ATTRIBUTE_NODE,
        [
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn glimmer_block(statements: GlimmerStatementList) -> GlimmerBlockBuilder {
    GlimmerBlockBuilder {
        statements,
        block_params: None,
    }
}
pub struct GlimmerBlockBuilder {
    statements: GlimmerStatementList,
    block_params: Option<GlimmerBlockParams>,
}
impl GlimmerBlockBuilder {
    pub fn with_block_params(mut self, block_params: GlimmerBlockParams) -> Self {
        self.block_params = Some(block_params);
        self
    }
    pub fn build(self) -> GlimmerBlock {
        GlimmerBlock::unwrap_cast(SyntaxNode::new_detached(
            GlimmerSyntaxKind::GLIMMER_BLOCK,
            [
                self.block_params
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.statements.into_syntax())),
            ],
        ))
    }
}
pub fn glimmer_block_params(
    as_token: SyntaxToken,
    opening_pipe_token: SyntaxToken,
    params: GlimmerParamNameList,
    closing_pipe_token: SyntaxToken,
) -> GlimmerBlockParams {
    GlimmerBlockParams::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_BLOCK_PARAMS,
        [
            Some(SyntaxElement::Token(as_token)),
            Some(SyntaxElement::Token(opening_pipe_token)),
            Some(SyntaxElement::Node(params.into_syntax())),
            Some(SyntaxElement::Token(closing_pipe_token)),
        ],
    ))
}
pub fn glimmer_block_statement(
    opening_curly_token: SyntaxToken,
    hash_token: SyntaxToken,
    path: GlimmerPathExpression,
    params: GlimmerParamsList,
    hash: GlimmerHash,
    opening_curly_end_token: SyntaxToken,
    block: GlimmerBlock,
    closing_curly_token: SyntaxToken,
    slash_token: SyntaxToken,
    close_path: GlimmerPathExpression,
    closing_curly_end_token: SyntaxToken,
) -> GlimmerBlockStatementBuilder {
    GlimmerBlockStatementBuilder {
        opening_curly_token,
        hash_token,
        path,
        params,
        hash,
        opening_curly_end_token,
        block,
        closing_curly_token,
        slash_token,
        close_path,
        closing_curly_end_token,
        else_block: None,
    }
}
pub struct GlimmerBlockStatementBuilder {
    opening_curly_token: SyntaxToken,
    hash_token: SyntaxToken,
    path: GlimmerPathExpression,
    params: GlimmerParamsList,
    hash: GlimmerHash,
    opening_curly_end_token: SyntaxToken,
    block: GlimmerBlock,
    closing_curly_token: SyntaxToken,
    slash_token: SyntaxToken,
    close_path: GlimmerPathExpression,
    closing_curly_end_token: SyntaxToken,
    else_block: Option<GlimmerElseBlock>,
}
impl GlimmerBlockStatementBuilder {
    pub fn with_else_block(mut self, else_block: GlimmerElseBlock) -> Self {
        self.else_block = Some(else_block);
        self
    }
    pub fn build(self) -> GlimmerBlockStatement {
        GlimmerBlockStatement::unwrap_cast(SyntaxNode::new_detached(
            GlimmerSyntaxKind::GLIMMER_BLOCK_STATEMENT,
            [
                Some(SyntaxElement::Token(self.opening_curly_token)),
                Some(SyntaxElement::Token(self.hash_token)),
                Some(SyntaxElement::Node(self.path.into_syntax())),
                Some(SyntaxElement::Node(self.params.into_syntax())),
                Some(SyntaxElement::Node(self.hash.into_syntax())),
                Some(SyntaxElement::Token(self.opening_curly_end_token)),
                Some(SyntaxElement::Node(self.block.into_syntax())),
                self.else_block
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.closing_curly_token)),
                Some(SyntaxElement::Token(self.slash_token)),
                Some(SyntaxElement::Node(self.close_path.into_syntax())),
                Some(SyntaxElement::Token(self.closing_curly_end_token)),
            ],
        ))
    }
}
pub fn glimmer_boolean_literal(
    true_token: SyntaxToken,
    false_token: SyntaxToken,
) -> GlimmerBooleanLiteral {
    GlimmerBooleanLiteral::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_BOOLEAN_LITERAL,
        [
            Some(SyntaxElement::Token(true_token)),
            Some(SyntaxElement::Token(false_token)),
        ],
    ))
}
pub fn glimmer_comment_statement(comment_token: SyntaxToken) -> GlimmerCommentStatement {
    GlimmerCommentStatement::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_COMMENT_STATEMENT,
        [Some(SyntaxElement::Token(comment_token))],
    ))
}
pub fn glimmer_concat_statement(parts: GlimmerConcatPartList) -> GlimmerConcatStatement {
    GlimmerConcatStatement::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_CONCAT_STATEMENT,
        [Some(SyntaxElement::Node(parts.into_syntax()))],
    ))
}
pub fn glimmer_element_modifier(
    opening_token: SyntaxToken,
    path: GlimmerPathExpression,
    params: GlimmerParamsList,
    hash: GlimmerHash,
    closing_token: SyntaxToken,
) -> GlimmerElementModifier {
    GlimmerElementModifier::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_ELEMENT_MODIFIER,
        [
            Some(SyntaxElement::Token(opening_token)),
            Some(SyntaxElement::Node(path.into_syntax())),
            Some(SyntaxElement::Node(params.into_syntax())),
            Some(SyntaxElement::Node(hash.into_syntax())),
            Some(SyntaxElement::Token(closing_token)),
        ],
    ))
}
pub fn glimmer_element_node(
    opening: GlimmerStartTag,
    children: GlimmerStatementList,
) -> GlimmerElementNodeBuilder {
    GlimmerElementNodeBuilder {
        opening,
        children,
        closing: None,
    }
}
pub struct GlimmerElementNodeBuilder {
    opening: GlimmerStartTag,
    children: GlimmerStatementList,
    closing: Option<GlimmerEndTag>,
}
impl GlimmerElementNodeBuilder {
    pub fn with_closing(mut self, closing: GlimmerEndTag) -> Self {
        self.closing = Some(closing);
        self
    }
    pub fn build(self) -> GlimmerElementNode {
        GlimmerElementNode::unwrap_cast(SyntaxNode::new_detached(
            GlimmerSyntaxKind::GLIMMER_ELEMENT_NODE,
            [
                Some(SyntaxElement::Node(self.opening.into_syntax())),
                Some(SyntaxElement::Node(self.children.into_syntax())),
                self.closing
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn glimmer_else_block(
    opening_token: SyntaxToken,
    else_token: SyntaxToken,
    closing_token: SyntaxToken,
    block: GlimmerBlock,
) -> GlimmerElseBlockBuilder {
    GlimmerElseBlockBuilder {
        opening_token,
        else_token,
        closing_token,
        block,
        if_token: None,
        condition: None,
    }
}
pub struct GlimmerElseBlockBuilder {
    opening_token: SyntaxToken,
    else_token: SyntaxToken,
    closing_token: SyntaxToken,
    block: GlimmerBlock,
    if_token: Option<SyntaxToken>,
    condition: Option<Expression>,
}
impl GlimmerElseBlockBuilder {
    pub fn with_if_token(mut self, if_token: SyntaxToken) -> Self {
        self.if_token = Some(if_token);
        self
    }
    pub fn with_condition(mut self, condition: Expression) -> Self {
        self.condition = Some(condition);
        self
    }
    pub fn build(self) -> GlimmerElseBlock {
        GlimmerElseBlock::unwrap_cast(SyntaxNode::new_detached(
            GlimmerSyntaxKind::GLIMMER_ELSE_BLOCK,
            [
                Some(SyntaxElement::Token(self.opening_token)),
                Some(SyntaxElement::Token(self.else_token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.condition
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.closing_token)),
                Some(SyntaxElement::Node(self.block.into_syntax())),
            ],
        ))
    }
}
pub fn glimmer_end_tag(
    l_angle_token: SyntaxToken,
    slash_token: SyntaxToken,
    name: GlimmerPathExpression,
    r_angle_token: SyntaxToken,
) -> GlimmerEndTag {
    GlimmerEndTag::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_END_TAG,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Token(slash_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn glimmer_hash(pairs: GlimmerHashPairList) -> GlimmerHash {
    GlimmerHash::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_HASH,
        [Some(SyntaxElement::Node(pairs.into_syntax()))],
    ))
}
pub fn glimmer_hash_pair(
    key_token: SyntaxToken,
    eq_token: SyntaxToken,
    value: Expression,
) -> GlimmerHashPair {
    GlimmerHashPair::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_HASH_PAIR,
        [
            Some(SyntaxElement::Token(key_token)),
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn glimmer_mustache_comment_statement(
    mustache_comment_token: SyntaxToken,
) -> GlimmerMustacheCommentStatement {
    GlimmerMustacheCommentStatement::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_MUSTACHE_COMMENT_STATEMENT,
        [Some(SyntaxElement::Token(mustache_comment_token))],
    ))
}
pub fn glimmer_mustache_statement(
    opening_token: SyntaxToken,
    expression: Expression,
    closing_token: SyntaxToken,
) -> GlimmerMustacheStatementBuilder {
    GlimmerMustacheStatementBuilder {
        opening_token,
        expression,
        closing_token,
        triple_open_token: None,
        triple_close_token: None,
    }
}
pub struct GlimmerMustacheStatementBuilder {
    opening_token: SyntaxToken,
    expression: Expression,
    closing_token: SyntaxToken,
    triple_open_token: Option<SyntaxToken>,
    triple_close_token: Option<SyntaxToken>,
}
impl GlimmerMustacheStatementBuilder {
    pub fn with_triple_open_token(mut self, triple_open_token: SyntaxToken) -> Self {
        self.triple_open_token = Some(triple_open_token);
        self
    }
    pub fn with_triple_close_token(mut self, triple_close_token: SyntaxToken) -> Self {
        self.triple_close_token = Some(triple_close_token);
        self
    }
    pub fn build(self) -> GlimmerMustacheStatement {
        GlimmerMustacheStatement::unwrap_cast(SyntaxNode::new_detached(
            GlimmerSyntaxKind::GLIMMER_MUSTACHE_STATEMENT,
            [
                Some(SyntaxElement::Token(self.opening_token)),
                self.triple_open_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.expression.into_syntax())),
                Some(SyntaxElement::Token(self.closing_token)),
                self.triple_close_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn glimmer_null_literal(null_token: SyntaxToken) -> GlimmerNullLiteral {
    GlimmerNullLiteral::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_NULL_LITERAL,
        [Some(SyntaxElement::Token(null_token))],
    ))
}
pub fn glimmer_number_literal(NUMBER_LITERAL_token: SyntaxToken) -> GlimmerNumberLiteral {
    GlimmerNumberLiteral::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_NUMBER_LITERAL,
        [Some(SyntaxElement::Token(NUMBER_LITERAL_token))],
    ))
}
pub fn glimmer_param_name(name_token: SyntaxToken) -> GlimmerParamName {
    GlimmerParamName::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_PARAM_NAME,
        [Some(SyntaxElement::Token(name_token))],
    ))
}
pub fn glimmer_path_expression(
    head: PathHead,
    tail: GlimmerPathSegmentList,
) -> GlimmerPathExpression {
    GlimmerPathExpression::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_PATH_EXPRESSION,
        [
            Some(SyntaxElement::Node(head.into_syntax())),
            Some(SyntaxElement::Node(tail.into_syntax())),
        ],
    ))
}
pub fn glimmer_path_segment(
    dot_token: SyntaxToken,
    segment_token: SyntaxToken,
) -> GlimmerPathSegment {
    GlimmerPathSegment::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_PATH_SEGMENT,
        [
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Token(segment_token)),
        ],
    ))
}
pub fn glimmer_root(
    statements: GlimmerStatementList,
    eof_token: SyntaxToken,
) -> GlimmerRootBuilder {
    GlimmerRootBuilder {
        statements,
        eof_token,
        bom_token: None,
    }
}
pub struct GlimmerRootBuilder {
    statements: GlimmerStatementList,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
}
impl GlimmerRootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn build(self) -> GlimmerRoot {
        GlimmerRoot::unwrap_cast(SyntaxNode::new_detached(
            GlimmerSyntaxKind::GLIMMER_ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.statements.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn glimmer_start_tag(
    l_angle_token: SyntaxToken,
    name: GlimmerPathExpression,
    attributes: GlimmerAttributeList,
    modifiers: GlimmerElementModifierList,
    r_angle_token: SyntaxToken,
) -> GlimmerStartTagBuilder {
    GlimmerStartTagBuilder {
        l_angle_token,
        name,
        attributes,
        modifiers,
        r_angle_token,
        block_params: None,
        self_closing: None,
    }
}
pub struct GlimmerStartTagBuilder {
    l_angle_token: SyntaxToken,
    name: GlimmerPathExpression,
    attributes: GlimmerAttributeList,
    modifiers: GlimmerElementModifierList,
    r_angle_token: SyntaxToken,
    block_params: Option<GlimmerBlockParams>,
    self_closing: Option<SelfClosing>,
}
impl GlimmerStartTagBuilder {
    pub fn with_block_params(mut self, block_params: GlimmerBlockParams) -> Self {
        self.block_params = Some(block_params);
        self
    }
    pub fn with_self_closing(mut self, self_closing: SelfClosing) -> Self {
        self.self_closing = Some(self_closing);
        self
    }
    pub fn build(self) -> GlimmerStartTag {
        GlimmerStartTag::unwrap_cast(SyntaxNode::new_detached(
            GlimmerSyntaxKind::GLIMMER_START_TAG,
            [
                Some(SyntaxElement::Token(self.l_angle_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.attributes.into_syntax())),
                Some(SyntaxElement::Node(self.modifiers.into_syntax())),
                self.block_params
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.self_closing
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_angle_token)),
            ],
        ))
    }
}
pub fn glimmer_string_literal(STRING_LITERAL_token: SyntaxToken) -> GlimmerStringLiteral {
    GlimmerStringLiteral::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_STRING_LITERAL,
        [Some(SyntaxElement::Token(STRING_LITERAL_token))],
    ))
}
pub fn glimmer_sub_expression(
    l_paren_token: SyntaxToken,
    path: GlimmerPathExpression,
    params: GlimmerParamsList,
    hash: GlimmerHash,
    r_paren_token: SyntaxToken,
) -> GlimmerSubExpression {
    GlimmerSubExpression::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_SUB_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(path.into_syntax())),
            Some(SyntaxElement::Node(params.into_syntax())),
            Some(SyntaxElement::Node(hash.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn glimmer_text_node(text_token: SyntaxToken) -> GlimmerTextNode {
    GlimmerTextNode::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_TEXT_NODE,
        [Some(SyntaxElement::Token(text_token))],
    ))
}
pub fn glimmer_this_head(this_token: SyntaxToken) -> GlimmerThisHead {
    GlimmerThisHead::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_THIS_HEAD,
        [Some(SyntaxElement::Token(this_token))],
    ))
}
pub fn glimmer_undefined_literal(undefined_token: SyntaxToken) -> GlimmerUndefinedLiteral {
    GlimmerUndefinedLiteral::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_UNDEFINED_LITERAL,
        [Some(SyntaxElement::Token(undefined_token))],
    ))
}
pub fn glimmer_var_head(name_token: SyntaxToken) -> GlimmerVarHead {
    GlimmerVarHead::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_VAR_HEAD,
        [Some(SyntaxElement::Token(name_token))],
    ))
}
pub fn self_closing(slash_token: SyntaxToken) -> SelfClosing {
    SelfClosing::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::SELF_CLOSING,
        [Some(SyntaxElement::Token(slash_token))],
    ))
}
pub fn glimmer_attribute_list<I>(items: I) -> GlimmerAttributeList
where
    I: IntoIterator<Item = GlimmerAttributeNode>,
    I::IntoIter: ExactSizeIterator,
{
    GlimmerAttributeList::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_ATTRIBUTE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn glimmer_concat_part_list<I>(items: I) -> GlimmerConcatPartList
where
    I: IntoIterator<Item = AnyConcatPart>,
    I::IntoIter: ExactSizeIterator,
{
    GlimmerConcatPartList::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_CONCAT_PART_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn glimmer_element_modifier_list<I>(items: I) -> GlimmerElementModifierList
where
    I: IntoIterator<Item = GlimmerElementModifier>,
    I::IntoIter: ExactSizeIterator,
{
    GlimmerElementModifierList::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_ELEMENT_MODIFIER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn glimmer_hash_pair_list<I>(items: I) -> GlimmerHashPairList
where
    I: IntoIterator<Item = GlimmerHashPair>,
    I::IntoIter: ExactSizeIterator,
{
    GlimmerHashPairList::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_HASH_PAIR_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn glimmer_param_name_list<I>(items: I) -> GlimmerParamNameList
where
    I: IntoIterator<Item = GlimmerParamName>,
    I::IntoIter: ExactSizeIterator,
{
    GlimmerParamNameList::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_PARAM_NAME_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn glimmer_params_list<I>(items: I) -> GlimmerParamsList
where
    I: IntoIterator<Item = Expression>,
    I::IntoIter: ExactSizeIterator,
{
    GlimmerParamsList::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_PARAMS_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn glimmer_path_segment_list<I>(items: I) -> GlimmerPathSegmentList
where
    I: IntoIterator<Item = GlimmerPathSegment>,
    I::IntoIter: ExactSizeIterator,
{
    GlimmerPathSegmentList::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_PATH_SEGMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn glimmer_statement_list<I>(items: I) -> GlimmerStatementList
where
    I: IntoIterator<Item = Statement>,
    I::IntoIter: ExactSizeIterator,
{
    GlimmerStatementList::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_STATEMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn glimmer_bogus<I>(slots: I) -> GlimmerBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GlimmerBogus::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_BOGUS,
        slots,
    ))
}
pub fn glimmer_bogus_expression<I>(slots: I) -> GlimmerBogusExpression
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GlimmerBogusExpression::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_BOGUS_EXPRESSION,
        slots,
    ))
}
pub fn glimmer_bogus_statement<I>(slots: I) -> GlimmerBogusStatement
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GlimmerBogusStatement::unwrap_cast(SyntaxNode::new_detached(
        GlimmerSyntaxKind::GLIMMER_BOGUS_STATEMENT,
        slots,
    ))
}
