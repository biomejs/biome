use crate::context::TabWidth;
use crate::js::auxiliary::template_chunk_element::AnyTemplateChunkElement;
use crate::js::auxiliary::template_element::{AnyTemplateElement, TemplateElementOptions};
use crate::prelude::*;
use crate::utils::test_each_template::EachTemplateTable;
use biome_formatter::FormatRuleWithOptions;
use biome_js_syntax::{
    AnyJsTemplateElement, AnyTsTemplateElement, JsLanguage, JsTemplateElementList,
    TsTemplateElementList,
};
use biome_rowan::{declare_node_union, AstNodeListIterator};
use std::iter::FusedIterator;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsTemplateElementList {
    options: FormatJsTemplateElementListOptions,
}

impl FormatRuleWithOptions<JsTemplateElementList> for FormatJsTemplateElementList {
    type Options = FormatJsTemplateElementListOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}

impl FormatRule<JsTemplateElementList> for FormatJsTemplateElementList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsTemplateElementList, f: &mut JsFormatter) -> FormatResult<()> {
        if self.options.is_test_each_pattern {
            EachTemplateTable::from(node, f)?.fmt(f)
        } else {
            AnyTemplateElementList::JsTemplateElementList(node.clone()).fmt(f)
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct FormatJsTemplateElementListOptions {
    pub(crate) is_test_each_pattern: bool,
}

pub(crate) enum AnyTemplateElementList {
    JsTemplateElementList(JsTemplateElementList),
    TsTemplateElementList(TsTemplateElementList),
}

impl Format<JsFormatContext> for AnyTemplateElementList {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let mut indention = TemplateElementIndention::default();
        let mut after_new_line = false;

        for element in self.elements() {
            match element {
                AnyTemplateElementOrChunk::AnyTemplateElement(element) => {
                    let options = TemplateElementOptions {
                        after_new_line,
                        indention,
                    };

                    match &element {
                        AnyTemplateElement::JsTemplateElement(element) => {
                            element.format().with_options(options).fmt(f)?;
                        }
                        AnyTemplateElement::TsTemplateElement(element) => {
                            element.format().with_options(options).fmt(f)?;
                        }
                    }
                }
                AnyTemplateElementOrChunk::AnyTemplateChunkElement(chunk) => {
                    match &chunk {
                        AnyTemplateChunkElement::JsTemplateChunkElement(chunk) => {
                            chunk.format().fmt(f)?;
                        }
                        AnyTemplateChunkElement::TsTemplateChunkElement(chunk) => {
                            chunk.format().fmt(f)?;
                        }
                    }

                    let chunk_token = chunk.template_chunk_token()?;
                    let chunk_text = chunk_token.text();

                    let tab_width = f.options().tab_width();

                    indention = TemplateElementIndention::after_last_new_line(
                        chunk_text, tab_width, indention,
                    );
                    after_new_line = chunk_text.ends_with('\n');
                }
            }
        }

        Ok(())
    }
}

impl AnyTemplateElementList {
    fn elements(&self) -> TemplateElementIterator {
        match self {
            AnyTemplateElementList::JsTemplateElementList(list) => {
                TemplateElementIterator::JsTemplateElementList(list.iter())
            }
            AnyTemplateElementList::TsTemplateElementList(list) => {
                TemplateElementIterator::TsTemplateElementList(list.iter())
            }
        }
    }
}

declare_node_union! {
    AnyTemplateElementOrChunk = AnyTemplateElement | AnyTemplateChunkElement
}

enum TemplateElementIterator {
    JsTemplateElementList(AstNodeListIterator<JsLanguage, AnyJsTemplateElement>),
    TsTemplateElementList(AstNodeListIterator<JsLanguage, AnyTsTemplateElement>),
}

impl Iterator for TemplateElementIterator {
    type Item = AnyTemplateElementOrChunk;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            TemplateElementIterator::JsTemplateElementList(inner) => {
                let result = match inner.next()? {
                    AnyJsTemplateElement::JsTemplateChunkElement(chunk) => {
                        AnyTemplateElementOrChunk::from(AnyTemplateChunkElement::from(chunk))
                    }
                    AnyJsTemplateElement::JsTemplateElement(element) => {
                        AnyTemplateElementOrChunk::from(AnyTemplateElement::from(element))
                    }
                };
                Some(result)
            }
            TemplateElementIterator::TsTemplateElementList(inner) => {
                let result = match inner.next()? {
                    AnyTsTemplateElement::TsTemplateChunkElement(chunk) => {
                        AnyTemplateElementOrChunk::from(AnyTemplateChunkElement::from(chunk))
                    }
                    AnyTsTemplateElement::TsTemplateElement(element) => {
                        AnyTemplateElementOrChunk::from(AnyTemplateElement::from(element))
                    }
                };

                Some(result)
            }
        }
    }
}

impl ExactSizeIterator for TemplateElementIterator {
    fn len(&self) -> usize {
        match self {
            TemplateElementIterator::JsTemplateElementList(inner) => inner.len(),
            TemplateElementIterator::TsTemplateElementList(inner) => inner.len(),
        }
    }
}

impl FusedIterator for TemplateElementIterator {}

/// The indention derived from a position in the source document. Consists of indention level and spaces
#[derive(Debug, Copy, Clone, Default)]
pub struct TemplateElementIndention(u32);

impl TemplateElementIndention {
    /// Returns the indention level
    pub(crate) fn level(&self, tab_width: TabWidth) -> u32 {
        self.0 / u32::from(u8::from(tab_width))
    }

    /// Returns the number of space indents on top of the indent level
    pub(crate) fn align(&self, tab_width: TabWidth) -> u8 {
        (self.0 % u32::from(u8::from(tab_width))) as u8
    }

    /// Computes the indention after the last new line character.
    fn after_last_new_line(
        text: &str,
        tab_width: TabWidth,
        previous_indention: TemplateElementIndention,
    ) -> Self {
        let by_new_line = text.rsplit_once('\n');

        let size = match by_new_line {
            None => previous_indention.0,
            Some((_, after_new_line)) => {
                let tab_width: u32 = u8::from(tab_width).into();
                let mut size: u32 = 0;

                for byte in after_new_line.bytes() {
                    match byte {
                        b'\t' => {
                            // Tabs behave in a way that they are aligned to the nearest
                            // multiple of tab_width:
                            // number of spaces -> added size
                            // 0 -> 4, 1 -> 4, 2 -> 4, 3 -> 4
                            // 4 -> 8, 5 -> 8, 6 -> 8, 7 -> 8 ..
                            // Or in other words, it clips the size to the next multiple of tab width.
                            size = size + tab_width - (size % tab_width);
                        }
                        b' ' => {
                            size += 1;
                        }
                        _ => break,
                    };
                }

                size
            }
        };

        TemplateElementIndention(size)
    }
}
