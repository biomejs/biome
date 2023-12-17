//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use biome_css_syntax::{
    CssSyntaxElement as SyntaxElement, CssSyntaxNode as SyntaxNode, CssSyntaxToken as SyntaxToken,
    *,
};
use biome_rowan::AstNode;
pub fn css_at_keyword(value_token: SyntaxToken) -> CssAtKeyword {
    CssAtKeyword::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_KEYWORD,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_at_rule(
    name: CssAtKeyword,
    prelude: CssAtRuleComponentValue,
    css_at_rule_content: CssAtRuleContent,
) -> CssAtRule {
    CssAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_RULE,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(prelude.into_syntax())),
            Some(SyntaxElement::Node(css_at_rule_content.into_syntax())),
        ],
    ))
}
pub fn css_at_rule_semicolon(semicolon_token: SyntaxToken) -> CssAtRuleSemicolon {
    CssAtRuleSemicolon::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_RULE_SEMICOLON,
        [Some(SyntaxElement::Token(semicolon_token))],
    ))
}
pub fn css_block_declaration_list(
    css_declaration_list: CssDeclarationList,
) -> CssBlockDeclarationList {
    CssBlockDeclarationList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BLOCK_DECLARATION_LIST,
        [Some(SyntaxElement::Node(
            css_declaration_list.into_syntax(),
        ))],
    ))
}
pub fn css_curly_brackets_block(
    l_curly_token: SyntaxToken,
    content: CssCurlyBracketsBlockContent,
    r_curly_token: SyntaxToken,
) -> CssCurlyBracketsBlock {
    CssCurlyBracketsBlock::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CURLY_BRACKETS_BLOCK,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(content.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn css_declaration(
    name: CssIdentifier,
    colon_token: SyntaxToken,
    valie: CssComponentValueList,
) -> CssDeclarationBuilder {
    CssDeclarationBuilder {
        name,
        colon_token,
        valie,
        important: None,
    }
}
pub struct CssDeclarationBuilder {
    name: CssIdentifier,
    colon_token: SyntaxToken,
    valie: CssComponentValueList,
    important: Option<CssDeclarationImportant>,
}
impl CssDeclarationBuilder {
    pub fn with_important(mut self, important: CssDeclarationImportant) -> Self {
        self.important = Some(important);
        self
    }
    pub fn build(self) -> CssDeclaration {
        CssDeclaration::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_DECLARATION,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.colon_token)),
                Some(SyntaxElement::Node(self.valie.into_syntax())),
                self.important
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn css_declaration_important(
    excl_token: SyntaxToken,
    important_token: SyntaxToken,
) -> CssDeclarationImportant {
    CssDeclarationImportant::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DECLARATION_IMPORTANT,
        [
            Some(SyntaxElement::Token(excl_token)),
            Some(SyntaxElement::Token(important_token)),
        ],
    ))
}
pub fn css_delim(value_token: SyntaxToken) -> CssDelim {
    CssDelim::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DELIM,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_dimension(value: CssNumber, unit: CssIdentifier) -> CssDimension {
    CssDimension::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DIMENSION,
        [
            Some(SyntaxElement::Node(value.into_syntax())),
            Some(SyntaxElement::Node(unit.into_syntax())),
        ],
    ))
}
pub fn css_function_block(
    css_function_token: CssFunctionToken,
    css_component_value_list: CssComponentValueList,
    r_paren_token: SyntaxToken,
) -> CssFunctionBlock {
    CssFunctionBlock::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_FUNCTION_BLOCK,
        [
            Some(SyntaxElement::Node(css_function_token.into_syntax())),
            Some(SyntaxElement::Node(css_component_value_list.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_function_token(valye_token: SyntaxToken) -> CssFunctionToken {
    CssFunctionToken::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_FUNCTION_TOKEN,
        [Some(SyntaxElement::Token(valye_token))],
    ))
}
pub fn css_hash(value_token: SyntaxToken) -> CssHash {
    CssHash::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_HASH,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_identifier(value_token: SyntaxToken) -> CssIdentifier {
    CssIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_IDENTIFIER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_number(value_token: SyntaxToken) -> CssNumber {
    CssNumber::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_NUMBER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_percentage(value: CssNumber, reminder_token: SyntaxToken) -> CssPercentage {
    CssPercentage::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PERCENTAGE,
        [
            Some(SyntaxElement::Node(value.into_syntax())),
            Some(SyntaxElement::Token(reminder_token)),
        ],
    ))
}
pub fn css_preserved_token_key(value_token: SyntaxToken) -> CssPreservedTokenKey {
    CssPreservedTokenKey::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PRESERVED_TOKEN_KEY,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_qualified_rule(
    prelude: CssQualifiedRulePrelude,
    block: CssCurlyBracketsBlock,
) -> CssQualifiedRule {
    CssQualifiedRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_QUALIFIED_RULE,
        [
            Some(SyntaxElement::Node(prelude.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_simple_curly_brackets_block(
    l_curly_token: SyntaxToken,
    content: CssSimpleComponentValueList,
    r_curly_token: SyntaxToken,
) -> CssSimpleCurlyBracketsBlock {
    CssSimpleCurlyBracketsBlock::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SIMPLE_CURLY_BRACKETS_BLOCK,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(content.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn css_simple_parentheses_block(
    l_paren_token: SyntaxToken,
    content: CssSimpleComponentValueList,
    r_paren_token: SyntaxToken,
) -> CssSimpleParenthesesBlock {
    CssSimpleParenthesesBlock::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SIMPLE_PARENTHESES_BLOCK,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(content.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_simple_square_brackets_block(
    l_brack_token: SyntaxToken,
    content: CssSimpleComponentValueList,
    r_brack_token: SyntaxToken,
) -> CssSimpleSquareBracketsBlock {
    CssSimpleSquareBracketsBlock::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SIMPLE_SQUARE_BRACKETS_BLOCK,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(content.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn css_string(value_token: SyntaxToken) -> CssString {
    CssString::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_STRING,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_style_sheet(
    content: CssStyleSheetContent,
    eof_token: SyntaxToken,
) -> CssStyleSheetBuilder {
    CssStyleSheetBuilder {
        content,
        eof_token,
        bom_token: None,
    }
}
pub struct CssStyleSheetBuilder {
    content: CssStyleSheetContent,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
}
impl CssStyleSheetBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn build(self) -> CssStyleSheet {
        CssStyleSheet::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_STYLE_SHEET,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.content.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn css_at_rule_component_value<I>(items: I) -> CssAtRuleComponentValue
where
    I: IntoIterator<Item = CssComponentValue>,
    I::IntoIter: ExactSizeIterator,
{
    CssAtRuleComponentValue::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_RULE_COMPONENT_VALUE,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_component_value_list<I>(items: I) -> CssComponentValueList
where
    I: IntoIterator<Item = CssComponentValue>,
    I::IntoIter: ExactSizeIterator,
{
    CssComponentValueList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COMPONENT_VALUE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_curly_brackets_block_content<I>(items: I) -> CssCurlyBracketsBlockContent
where
    I: IntoIterator<Item = AnyCssCurlyBracketsBlockContent>,
    I::IntoIter: ExactSizeIterator,
{
    CssCurlyBracketsBlockContent::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CURLY_BRACKETS_BLOCK_CONTENT,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_declaration_list<I, S>(items: I, separators: S) -> CssDeclarationList
where
    I: IntoIterator<Item = CssDeclaration>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssDeclarationList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DECLARATION_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_qualified_rule_prelude<I>(items: I) -> CssQualifiedRulePrelude
where
    I: IntoIterator<Item = CssComponentValue>,
    I::IntoIter: ExactSizeIterator,
{
    CssQualifiedRulePrelude::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_QUALIFIED_RULE_PRELUDE,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_simple_component_value_list<I>(items: I) -> CssSimpleComponentValueList
where
    I: IntoIterator<Item = CssComponentValue>,
    I::IntoIter: ExactSizeIterator,
{
    CssSimpleComponentValueList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SIMPLE_COMPONENT_VALUE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_style_sheet_content<I>(items: I) -> CssStyleSheetContent
where
    I: IntoIterator<Item = AnyCssStylesheetContent>,
    I::IntoIter: ExactSizeIterator,
{
    CssStyleSheetContent::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_STYLE_SHEET_CONTENT,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_bogus<I>(slots: I) -> CssBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogus::unwrap_cast(SyntaxNode::new_detached(CssSyntaxKind::CSS_BOGUS, slots))
}
