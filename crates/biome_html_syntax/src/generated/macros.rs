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
                $crate::HtmlSyntaxKind::ASTRO_EMBEDDED_CONTENT => {
                    let $pattern = unsafe { $crate::AstroEmbeddedContent::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::ASTRO_FRONTMATTER_ELEMENT => {
                    let $pattern = unsafe { $crate::AstroFrontmatterElement::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ATTRIBUTE => {
                    let $pattern = unsafe { $crate::HtmlAttribute::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ATTRIBUTE_INITIALIZER_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::HtmlAttributeInitializerClause::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ATTRIBUTE_NAME => {
                    let $pattern = unsafe { $crate::HtmlAttributeName::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_CDATA_SECTION => {
                    let $pattern = unsafe { $crate::HtmlCdataSection::new_unchecked(node) };
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
                $crate::HtmlSyntaxKind::HTML_DOUBLE_TEXT_EXPRESSION => {
                    let $pattern = unsafe { $crate::HtmlDoubleTextExpression::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ELEMENT => {
                    let $pattern = unsafe { $crate::HtmlElement::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_EMBEDDED_CONTENT => {
                    let $pattern = unsafe { $crate::HtmlEmbeddedContent::new_unchecked(node) };
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
                $crate::HtmlSyntaxKind::HTML_SINGLE_TEXT_EXPRESSION => {
                    let $pattern = unsafe { $crate::HtmlSingleTextExpression::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_STRING => {
                    let $pattern = unsafe { $crate::HtmlString::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_TAG_NAME => {
                    let $pattern = unsafe { $crate::HtmlTagName::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_TEXT_EXPRESSION => {
                    let $pattern = unsafe { $crate::HtmlTextExpression::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::SVELTE_ATTACH_ATTRIBUTE => {
                    let $pattern = unsafe { $crate::SvelteAttachAttribute::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::SVELTE_CONST_BLOCK => {
                    let $pattern = unsafe { $crate::SvelteConstBlock::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::SVELTE_DEBUG_BLOCK => {
                    let $pattern = unsafe { $crate::SvelteDebugBlock::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::SVELTE_ELSE_CLAUSE => {
                    let $pattern = unsafe { $crate::SvelteElseClause::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::SVELTE_ELSE_IF_CLAUSE => {
                    let $pattern = unsafe { $crate::SvelteElseIfClause::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::SVELTE_HTML_BLOCK => {
                    let $pattern = unsafe { $crate::SvelteHtmlBlock::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::SVELTE_IF_BLOCK => {
                    let $pattern = unsafe { $crate::SvelteIfBlock::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::SVELTE_IF_CLOSING_BLOCK => {
                    let $pattern = unsafe { $crate::SvelteIfClosingBlock::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::SVELTE_IF_OPENING_BLOCK => {
                    let $pattern = unsafe { $crate::SvelteIfOpeningBlock::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::SVELTE_KEY_BLOCK => {
                    let $pattern = unsafe { $crate::SvelteKeyBlock::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::SVELTE_KEY_CLOSING_BLOCK => {
                    let $pattern = unsafe { $crate::SvelteKeyClosingBlock::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::SVELTE_KEY_OPENING_BLOCK => {
                    let $pattern = unsafe { $crate::SvelteKeyOpeningBlock::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::SVELTE_NAME => {
                    let $pattern = unsafe { $crate::SvelteName::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::SVELTE_RENDER_BLOCK => {
                    let $pattern = unsafe { $crate::SvelteRenderBlock::new_unchecked(node) };
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
                $crate::HtmlSyntaxKind::VUE_DYNAMIC_ARGUMENT => {
                    let $pattern = unsafe { $crate::VueDynamicArgument::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_MODIFIER => {
                    let $pattern = unsafe { $crate::VueModifier::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_STATIC_ARGUMENT => {
                    let $pattern = unsafe { $crate::VueStaticArgument::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_V_BIND_SHORTHAND_DIRECTIVE => {
                    let $pattern =
                        unsafe { $crate::VueVBindShorthandDirective::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_V_ON_SHORTHAND_DIRECTIVE => {
                    let $pattern = unsafe { $crate::VueVOnShorthandDirective::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_V_SLOT_SHORTHAND_DIRECTIVE => {
                    let $pattern =
                        unsafe { $crate::VueVSlotShorthandDirective::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::ASTRO_BOGUS_FRONTMATTER => {
                    let $pattern = unsafe { $crate::AstroBogusFrontmatter::new_unchecked(node) };
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
                $crate::HtmlSyntaxKind::HTML_BOGUS_TEXT_EXPRESSION => {
                    let $pattern = unsafe { $crate::HtmlBogusTextExpression::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::SVELTE_BOGUS_BLOCK => {
                    let $pattern = unsafe { $crate::SvelteBogusBlock::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_BOGUS_DIRECTIVE => {
                    let $pattern = unsafe { $crate::VueBogusDirective::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_BOGUS_DIRECTIVE_ARGUMENT => {
                    let $pattern =
                        unsafe { $crate::VueBogusDirectiveArgument::new_unchecked(node) };
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
                $crate::HtmlSyntaxKind::SVELTE_BINDING_LIST => {
                    let $pattern = unsafe { $crate::SvelteBindingList::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::SVELTE_ELSE_IF_CLAUSE_LIST => {
                    let $pattern = unsafe { $crate::SvelteElseIfClauseList::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::VUE_MODIFIER_LIST => {
                    let $pattern = unsafe { $crate::VueModifierList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
