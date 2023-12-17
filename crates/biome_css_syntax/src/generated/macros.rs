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
            node => match $crate::CssSyntaxNode::kind(&node) {
                $crate::CssSyntaxKind::CSS_AT_KEYWORD => {
                    let $pattern = unsafe { $crate::CssAtKeyword::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_AT_RULE => {
                    let $pattern = unsafe { $crate::CssAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_AT_RULE_SEMICOLON => {
                    let $pattern = unsafe { $crate::CssAtRuleSemicolon::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BLOCK_DECLARATION_LIST => {
                    let $pattern = unsafe { $crate::CssBlockDeclarationList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CURLY_BRACKETS_BLOCK => {
                    let $pattern = unsafe { $crate::CssCurlyBracketsBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DECLARATION => {
                    let $pattern = unsafe { $crate::CssDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DECLARATION_IMPORTANT => {
                    let $pattern = unsafe { $crate::CssDeclarationImportant::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DELIM => {
                    let $pattern = unsafe { $crate::CssDelim::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DIMENSION => {
                    let $pattern = unsafe { $crate::CssDimension::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_FUNCTION_BLOCK => {
                    let $pattern = unsafe { $crate::CssFunctionBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_FUNCTION_TOKEN => {
                    let $pattern = unsafe { $crate::CssFunctionToken::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_HASH => {
                    let $pattern = unsafe { $crate::CssHash::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_IDENTIFIER => {
                    let $pattern = unsafe { $crate::CssIdentifier::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_NUMBER => {
                    let $pattern = unsafe { $crate::CssNumber::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PERCENTAGE => {
                    let $pattern = unsafe { $crate::CssPercentage::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PRESERVED_TOKEN_KEY => {
                    let $pattern = unsafe { $crate::CssPreservedTokenKey::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_QUALIFIED_RULE => {
                    let $pattern = unsafe { $crate::CssQualifiedRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SIMPLE_CURLY_BRACKETS_BLOCK => {
                    let $pattern =
                        unsafe { $crate::CssSimpleCurlyBracketsBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SIMPLE_PARENTHESES_BLOCK => {
                    let $pattern =
                        unsafe { $crate::CssSimpleParenthesesBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SIMPLE_SQUARE_BRACKETS_BLOCK => {
                    let $pattern =
                        unsafe { $crate::CssSimpleSquareBracketsBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_STRING => {
                    let $pattern = unsafe { $crate::CssString::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_STYLE_SHEET => {
                    let $pattern = unsafe { $crate::CssStyleSheet::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS => {
                    let $pattern = unsafe { $crate::CssBogus::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_AT_RULE_COMPONENT_VALUE => {
                    let $pattern = unsafe { $crate::CssAtRuleComponentValue::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_COMPONENT_VALUE_LIST => {
                    let $pattern = unsafe { $crate::CssComponentValueList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CURLY_BRACKETS_BLOCK_CONTENT => {
                    let $pattern =
                        unsafe { $crate::CssCurlyBracketsBlockContent::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DECLARATION_LIST => {
                    let $pattern = unsafe { $crate::CssDeclarationList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_QUALIFIED_RULE_PRELUDE => {
                    let $pattern = unsafe { $crate::CssQualifiedRulePrelude::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SIMPLE_COMPONENT_VALUE_LIST => {
                    let $pattern =
                        unsafe { $crate::CssSimpleComponentValueList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_STYLE_SHEET_CONTENT => {
                    let $pattern = unsafe { $crate::CssStyleSheetContent::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
