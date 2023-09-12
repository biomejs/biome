use crate::prelude::*;
use biome_formatter::FormatRuleWithOptions;

use crate::js::auxiliary::template_element::{
    AnyTemplateElement, FormatTemplateElement, TemplateElementOptions,
};
use biome_js_syntax::TsTemplateElement;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTemplateElement {
    options: TemplateElementOptions,
}

impl FormatRuleWithOptions<TsTemplateElement> for FormatTsTemplateElement {
    type Options = TemplateElementOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}

impl FormatNodeRule<TsTemplateElement> for FormatTsTemplateElement {
    fn fmt_fields(
        &self,
        node: &TsTemplateElement,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        let element = AnyTemplateElement::from(node.clone());
        FormatTemplateElement::new(element, self.options).fmt(formatter)
    }
}
