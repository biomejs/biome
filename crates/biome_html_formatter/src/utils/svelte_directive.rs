use crate::AsFormat;
use crate::prelude::HtmlFormatContext;
use biome_formatter::Buffer;
use biome_formatter::formatter::Formatter;
use biome_formatter::write;
use biome_formatter::{Format, FormatResult};
use biome_html_syntax::{
    HtmlSyntaxToken, SvelteAnimateDirective, SvelteBindDirective, SvelteClassDirective,
    SvelteDirectiveValue, SvelteInDirective, SvelteOutDirective, SvelteStyleDirective,
    SvelteTransitionDirective, SvelteUseDirective,
};
use biome_rowan::SyntaxResult;

pub(crate) struct FmtSvelteDirective {
    token: SyntaxResult<HtmlSyntaxToken>,
    value: SyntaxResult<SvelteDirectiveValue>,
}

impl From<&SvelteAnimateDirective> for FmtSvelteDirective {
    fn from(value: &SvelteAnimateDirective) -> Self {
        Self {
            token: value.animate_token(),
            value: value.value(),
        }
    }
}

impl From<&SvelteInDirective> for FmtSvelteDirective {
    fn from(value: &SvelteInDirective) -> Self {
        Self {
            token: value.in_token(),
            value: value.value(),
        }
    }
}

impl From<&SvelteOutDirective> for FmtSvelteDirective {
    fn from(value: &SvelteOutDirective) -> Self {
        Self {
            token: value.out_token(),
            value: value.value(),
        }
    }
}

impl From<&SvelteBindDirective> for FmtSvelteDirective {
    fn from(value: &SvelteBindDirective) -> Self {
        Self {
            token: value.bind_token(),
            value: value.value(),
        }
    }
}

impl From<&SvelteTransitionDirective> for FmtSvelteDirective {
    fn from(value: &SvelteTransitionDirective) -> Self {
        Self {
            token: value.transition_token(),
            value: value.value(),
        }
    }
}

impl From<&SvelteClassDirective> for FmtSvelteDirective {
    fn from(value: &SvelteClassDirective) -> Self {
        Self {
            token: value.class_token(),
            value: value.value(),
        }
    }
}
impl From<&SvelteStyleDirective> for FmtSvelteDirective {
    fn from(value: &SvelteStyleDirective) -> Self {
        Self {
            token: value.style_token(),
            value: value.value(),
        }
    }
}

impl From<&SvelteUseDirective> for FmtSvelteDirective {
    fn from(value: &SvelteUseDirective) -> Self {
        Self {
            token: value.use_token(),
            value: value.value(),
        }
    }
}

impl Format<HtmlFormatContext> for FmtSvelteDirective {
    fn fmt(&self, f: &mut Formatter<HtmlFormatContext>) -> FormatResult<()> {
        write!(f, [self.token.format(), self.value.format()])
    }
}
