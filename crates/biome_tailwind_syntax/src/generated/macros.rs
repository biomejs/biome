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
            node => match $crate::TailwindSyntaxNode::kind(&node) {
                $crate::TailwindSyntaxKind::TW_ARBITRARY_CANDIDATE => {
                    let $pattern = unsafe { $crate::TwArbitraryCandidate::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_ARBITRARY_VALUE => {
                    let $pattern = unsafe { $crate::TwArbitraryValue::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_ARBITRARY_VARIANT => {
                    let $pattern = unsafe { $crate::TwArbitraryVariant::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_CSS_VARIABLE_VALUE => {
                    let $pattern = unsafe { $crate::TwCssVariableValue::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_FULL_CANDIDATE => {
                    let $pattern = unsafe { $crate::TwFullCandidate::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_FUNCTIONAL_CANDIDATE => {
                    let $pattern = unsafe { $crate::TwFunctionalCandidate::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_FUNCTIONAL_VARIANT => {
                    let $pattern = unsafe { $crate::TwFunctionalVariant::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_MODIFIER => {
                    let $pattern = unsafe { $crate::TwModifier::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_NAMED_VALUE => {
                    let $pattern = unsafe { $crate::TwNamedValue::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_ROOT => {
                    let $pattern = unsafe { $crate::TwRoot::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_STATIC_CANDIDATE => {
                    let $pattern = unsafe { $crate::TwStaticCandidate::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_STATIC_VARIANT => {
                    let $pattern = unsafe { $crate::TwStaticVariant::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_BOGUS => {
                    let $pattern = unsafe { $crate::TwBogus::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_BOGUS_CANDIDATE => {
                    let $pattern = unsafe { $crate::TwBogusCandidate::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_BOGUS_MODIFIER => {
                    let $pattern = unsafe { $crate::TwBogusModifier::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_BOGUS_VALUE => {
                    let $pattern = unsafe { $crate::TwBogusValue::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_BOGUS_VARIANT => {
                    let $pattern = unsafe { $crate::TwBogusVariant::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_CANDIDATE_LIST => {
                    let $pattern = unsafe { $crate::TwCandidateList::new_unchecked(node) };
                    $body
                }
                $crate::TailwindSyntaxKind::TW_VARIANT_LIST => {
                    let $pattern = unsafe { $crate::TwVariantList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
