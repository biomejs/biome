// Glimmer formatter stubs
// All Glimmer nodes preserve their original formatting for now

use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::*;
use biome_rowan::AstNode;

// Helper macro to create formatter stubs that preserve original formatting
macro_rules! impl_verbatim_formatter {
    ($node:ty) => {
        impl crate::AsFormat<HtmlFormatContext> for $node {
            type Format<'a> = FormatRefWithRule<'a, $node, FormatVerbatimNode>;

            fn format(&self) -> Self::Format<'_> {
                FormatRefWithRule::new(self, FormatVerbatimNode)
            }
        }

        impl crate::IntoFormat<HtmlFormatContext> for $node {
            type Format = FormatOwnedWithRule<$node, FormatVerbatimNode>;

            fn into_format(self) -> Self::Format {
                FormatOwnedWithRule::new(self, FormatVerbatimNode)
            }
        }
    };
}

// Verbatim formatter that preserves original text
struct FormatVerbatimNode;

impl<T: AstNode<Language = biome_html_syntax::HtmlLanguage>> biome_formatter::FormatRule<T> for FormatVerbatimNode {
    type Context = HtmlFormatContext;

    fn fmt(&self, node: &T, f: &mut HtmlFormatter) -> biome_formatter::FormatResult<()> {
        write!(f, [biome_formatter::format_verbatim_node(node.syntax())])
    }
}

// Apply verbatim formatting to all Glimmer nodes
impl_verbatim_formatter!(GlimmerMustacheExpression);
impl_verbatim_formatter!(GlimmerMustacheComment);
impl_verbatim_formatter!(GlimmerTripleStashExpression);
impl_verbatim_formatter!(GlimmerBlockHelper);
impl_verbatim_formatter!(GlimmerBlockHelperOpening);
impl_verbatim_formatter!(GlimmerBlockHelperClosing);
impl_verbatim_formatter!(GlimmerBlockParams);
impl_verbatim_formatter!(GlimmerBlockParam);
impl_verbatim_formatter!(GlimmerPath);
impl_verbatim_formatter!(GlimmerPathSegment);
impl_verbatim_formatter!(GlimmerPositionalArgument);
impl_verbatim_formatter!(GlimmerNamedArgument);
impl_verbatim_formatter!(GlimmerSubexpression);
impl_verbatim_formatter!(GlimmerStringLiteral);
impl_verbatim_formatter!(GlimmerLiteral);
impl_verbatim_formatter!(GlimmerSplattribute);
impl_verbatim_formatter!(GlimmerElementModifier);
impl_verbatim_formatter!(GlimmerNamedBlock);
impl_verbatim_formatter!(GlimmerNamedBlockOpening);
impl_verbatim_formatter!(GlimmerNamedBlockClosing);
impl_verbatim_formatter!(GlimmerBogusExpression);
