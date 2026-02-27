use crate::AsFormat;
use crate::prelude::HtmlFormatContext;
use crate::svelte::value::directive_value::FormatSvelteDirectiveValueOptions;
use biome_formatter::Buffer;
use biome_formatter::formatter::Formatter;
use biome_formatter::write;
use biome_formatter::{Format, FormatResult};
use biome_html_syntax::{
    HtmlSyntaxNode, HtmlSyntaxToken, SvelteAnimateDirective, SvelteBindDirective,
    SvelteClassDirective, SvelteDirectiveValue, SvelteInDirective, SvelteOutDirective,
    SvelteStyleDirective, SvelteTransitionDirective, SvelteUseDirective,
};
use biome_rowan::{AstNode, SyntaxResult};

pub(crate) struct FmtSvelteDirective<'a> {
    _node: &'a HtmlSyntaxNode,
    token: SyntaxResult<HtmlSyntaxToken>,
    value: SyntaxResult<SvelteDirectiveValue>,
    allows_compact: bool,
}

impl<'a> From<&'a SvelteAnimateDirective> for FmtSvelteDirective<'a> {
    fn from(value: &'a SvelteAnimateDirective) -> Self {
        Self {
            token: value.animate_token(),
            value: value.value(),
            _node: value.syntax(),
            allows_compact: false,
        }
    }
}

impl<'a> From<&'a SvelteInDirective> for FmtSvelteDirective<'a> {
    fn from(value: &'a SvelteInDirective) -> Self {
        Self {
            token: value.in_token(),
            value: value.value(),
            _node: value.syntax(),
            allows_compact: false,
        }
    }
}

impl<'a> From<&'a SvelteOutDirective> for FmtSvelteDirective<'a> {
    fn from(value: &'a SvelteOutDirective) -> Self {
        Self {
            token: value.out_token(),
            value: value.value(),
            _node: value.syntax(),
            allows_compact: false,
        }
    }
}

impl<'a> From<&'a SvelteBindDirective> for FmtSvelteDirective<'a> {
    fn from(value: &'a SvelteBindDirective) -> Self {
        Self {
            token: value.bind_token(),
            value: value.value(),
            _node: value.syntax(),
            allows_compact: true,
        }
    }
}

impl<'a> From<&'a SvelteTransitionDirective> for FmtSvelteDirective<'a> {
    fn from(value: &'a SvelteTransitionDirective) -> Self {
        Self {
            token: value.transition_token(),
            value: value.value(),
            _node: value.syntax(),
            allows_compact: false,
        }
    }
}

impl<'a> From<&'a SvelteClassDirective> for FmtSvelteDirective<'a> {
    fn from(value: &'a SvelteClassDirective) -> Self {
        Self {
            token: value.class_token(),
            value: value.value(),
            _node: value.syntax(),
            allows_compact: false,
        }
    }
}
impl<'a> From<&'a SvelteStyleDirective> for FmtSvelteDirective<'a> {
    fn from(value: &'a SvelteStyleDirective) -> Self {
        Self {
            token: value.style_token(),
            value: value.value(),
            _node: value.syntax(),
            allows_compact: false,
        }
    }
}

impl<'a> From<&'a SvelteUseDirective> for FmtSvelteDirective<'a> {
    fn from(value: &'a SvelteUseDirective) -> Self {
        Self {
            token: value.use_token(),
            value: value.value(),
            _node: value.syntax(),
            allows_compact: false,
        }
    }
}

impl<'a> Format<HtmlFormatContext> for FmtSvelteDirective<'a> {
    fn fmt(&self, f: &mut Formatter<HtmlFormatContext>) -> FormatResult<()> {
        write!(
            f,
            [
                self.token.format(),
                self.value
                    .format()?
                    .with_options(FormatSvelteDirectiveValueOptions {
                        compact: self.allows_compact
                    })
            ]
        )
    }
}
