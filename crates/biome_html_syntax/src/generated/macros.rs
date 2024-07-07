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
            node => match $crate::HtmlSyntaxNode::kind(&node) {
                $crate::HtmlSyntaxKind::HTML_ATTRIBUTE => {
                    let $pattern = unsafe { $crate::HtmlAttribute::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ATTRIBUTE_INITIALIZER_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::HtmlAttributeInitializerClause::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_CLOSING_ELEMENT => {
                    let $pattern = unsafe { $crate::HtmlClosingElement::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_CONTENT => {
                    let $pattern = unsafe { $crate::HtmlContent::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_DIRECTIVE => {
                    let $pattern = unsafe { $crate::HtmlDirective::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ELEMENT => {
                    let $pattern = unsafe { $crate::HtmlElement::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_NAME => {
                    let $pattern = unsafe { $crate::HtmlName::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_OPENING_ELEMENT => {
                    let $pattern = unsafe { $crate::HtmlOpeningElement::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ROOT => {
                    let $pattern = unsafe { $crate::HtmlRoot::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_SELF_CLOSING_ELEMENT => {
                    let $pattern = unsafe { $crate::HtmlSelfClosingElement::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_STRING => {
                    let $pattern = unsafe { $crate::HtmlString::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_DIRECTIVE => {
                    let $pattern = unsafe { $crate::VueDirective::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_DIRECTIVE_ARGUMENT => {
                    let $pattern = unsafe { $crate::VueDirectiveArgument::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_DIRECTIVE_ARGUMENT_DYNAMIC => {
                    let $pattern =
                        unsafe { $crate::VueDirectiveArgumentDynamic::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_DIRECTIVE_ARGUMENT_STATIC => {
                    let $pattern =
                        unsafe { $crate::VueDirectiveArgumentStatic::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_DIRECTIVE_MODIFIER => {
                    let $pattern = unsafe { $crate::VueDirectiveModifier::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_DIRECTIVE_VALUE => {
                    let $pattern = unsafe { $crate::VueDirectiveValue::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_TEMPLATE_INTERPOLATION => {
                    let $pattern = unsafe { $crate::VueTemplateInterpolation::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_V_BIND_SHORTHAND => {
                    let $pattern = unsafe { $crate::VueVBindShorthand::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_V_ON_SHORTHAND => {
                    let $pattern = unsafe { $crate::VueVOnShorthand::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_BOGUS => {
                    let $pattern = unsafe { $crate::HtmlBogus::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_BOGUS_ATTRIBUTE => {
                    let $pattern = unsafe { $crate::HtmlBogusAttribute::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_BOGUS_ELEMENT => {
                    let $pattern = unsafe { $crate::HtmlBogusElement::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ATTRIBUTE_LIST => {
                    let $pattern = unsafe { $crate::HtmlAttributeList::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ELEMENT_LIST => {
                    let $pattern = unsafe { $crate::HtmlElementList::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_DIRECTIVE_MODIFIER_LIST => {
                    let $pattern = unsafe { $crate::VueDirectiveModifierList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
