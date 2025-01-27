use crate::prelude::*;

use biome_formatter::{write, CstFormatContext};
use biome_js_syntax::{
    AnyJsxAttribute, AnyJsxAttributeValue, AnyJsxElementName, JsSyntaxToken, JsxAttributeList,
    JsxOpeningElement, JsxSelfClosingElement, JsxString, TsTypeArguments,
};
use biome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxOpeningElement;

impl FormatNodeRule<JsxOpeningElement> for FormatJsxOpeningElement {
    fn fmt_fields(&self, node: &JsxOpeningElement, f: &mut JsFormatter) -> FormatResult<()> {
        AnyJsxOpeningElement::from(node.clone()).fmt(f)
    }
}

declare_node_union! {
    pub(super) AnyJsxOpeningElement = JsxSelfClosingElement | JsxOpeningElement
}

impl Format<JsFormatContext> for AnyJsxOpeningElement {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let layout = self.compute_layout(f.context().comments())?;
        let l_angle_token = self.l_angle_token()?;
        let name = self.name()?;
        let type_arguments = self.type_arguments();
        let attributes = self.attributes();

        let format_close = format_with(|f| {
            if let AnyJsxOpeningElement::JsxSelfClosingElement(element) = self {
                write!(f, [element.slash_token().format()])?;
            }

            write!(f, [self.r_angle_token().format()])
        });

        match layout {
            OpeningElementLayout::Inline => {
                write!(
                    f,
                    [
                        l_angle_token.format(),
                        name.format(),
                        type_arguments.format(),
                        space(),
                        format_close
                    ]
                )
            }
            OpeningElementLayout::SingleStringAttribute => {
                let attribute_spacing = if self.is_self_closing() {
                    Some(space())
                } else {
                    None
                };
                write!(
                    f,
                    [
                        l_angle_token.format(),
                        name.format(),
                        type_arguments.format(),
                        space(),
                        attributes.format(),
                        attribute_spacing,
                        format_close
                    ]
                )
            }
            OpeningElementLayout::IndentAttributes {
                name_has_comments,
                last_attribute_has_comments,
            } => {
                let format_inner = format_with(|f| {
                    write!(
                        f,
                        [
                            l_angle_token.format(),
                            name.format(),
                            type_arguments.format(),
                            soft_line_indent_or_space(&attributes.format()),
                        ]
                    )?;

                    let force_bracket_same_line = f.options().bracket_same_line().value();
                    let wants_bracket_same_line = attributes.is_empty() && !name_has_comments;

                    if self.is_self_closing() {
                        write!(f, [soft_line_break_or_space(), format_close])
                    } else if force_bracket_same_line && last_attribute_has_comments {
                        write!(f, [soft_line_break(), format_close])
                    } else if force_bracket_same_line || wants_bracket_same_line {
                        write!(f, [format_close])
                    } else {
                        write!(f, [soft_line_break(), format_close])
                    }
                });

                let has_multiline_string_attribute = attributes
                    .iter()
                    .any(|attribute| is_multiline_string_literal_attribute(&attribute));
                write![
                    f,
                    [group(&format_inner).should_expand(has_multiline_string_attribute)]
                ]
            }
        }
    }
}

impl AnyJsxOpeningElement {
    fn l_angle_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsxOpeningElement::JsxSelfClosingElement(element) => element.l_angle_token(),
            AnyJsxOpeningElement::JsxOpeningElement(element) => element.l_angle_token(),
        }
    }

    fn name(&self) -> SyntaxResult<AnyJsxElementName> {
        match self {
            AnyJsxOpeningElement::JsxSelfClosingElement(element) => element.name(),
            AnyJsxOpeningElement::JsxOpeningElement(element) => element.name(),
        }
    }

    fn type_arguments(&self) -> Option<TsTypeArguments> {
        match self {
            AnyJsxOpeningElement::JsxSelfClosingElement(element) => element.type_arguments(),
            AnyJsxOpeningElement::JsxOpeningElement(element) => element.type_arguments(),
        }
    }

    fn attributes(&self) -> JsxAttributeList {
        match self {
            AnyJsxOpeningElement::JsxSelfClosingElement(element) => element.attributes(),
            AnyJsxOpeningElement::JsxOpeningElement(element) => element.attributes(),
        }
    }

    fn r_angle_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsxOpeningElement::JsxSelfClosingElement(element) => element.r_angle_token(),
            AnyJsxOpeningElement::JsxOpeningElement(element) => element.r_angle_token(),
        }
    }

    fn is_self_closing(&self) -> bool {
        matches!(self, AnyJsxOpeningElement::JsxSelfClosingElement(_))
    }

    fn compute_layout(&self, comments: &JsComments) -> SyntaxResult<OpeningElementLayout> {
        let attributes = self.attributes();
        let name = self.name()?;
        let last_attribute_has_comments = self
            .attributes()
            .last()
            .is_some_and(|attribute| comments.has_trailing_comments(attribute.syntax()));

        let name_has_comments = comments.has_comments(name.syntax())
            || self
                .type_arguments()
                .is_some_and(|arguments| comments.has_comments(arguments.syntax()));

        let layout = if self.is_self_closing() && attributes.is_empty() && !name_has_comments {
            OpeningElementLayout::Inline
        } else if attributes.len() == 1
            && attributes.iter().all(|attribute| {
                is_single_line_string_literal_attribute(&attribute)
                    && !comments.has_comments(attribute.syntax())
            })
            && !name_has_comments
        {
            OpeningElementLayout::SingleStringAttribute
        } else {
            OpeningElementLayout::IndentAttributes {
                name_has_comments,
                last_attribute_has_comments,
            }
        };

        Ok(layout)
    }
}

#[derive(Copy, Clone, Debug)]
enum OpeningElementLayout {
    /// Don't create a group around the element to avoid it breaking ever.
    ///
    /// Applied for elements that have no attributes nor any comment attached to their name.
    ///
    /// ```javascript
    /// <ASuperLongComponentNameThatWouldBreakButDoesntSinceTheComponent<DonTBreakThis>></ASuperLongComponentNameThatWouldBreakButDoesntSinceTheComponent>
    /// ```
    Inline,

    /// Opening element with a single attribute that contains no line breaks, nor has comments.
    ///
    /// ```javascript
    /// <div tooltip="A very long tooltip text that would otherwise make the attribute break onto the same line but it is not because of the single string layout" more></div>;
    /// ```
    SingleStringAttribute,

    /// Default layout that indents the attributes and formats each attribute on its own line.
    ///
    /// ```javascript
    /// <div
    ///   oneAttribute
    ///   another="with value"
    ///   moreAttributes={withSomeExpression}
    /// ></div>;
    /// ```
    IndentAttributes {
        name_has_comments: bool,
        last_attribute_has_comments: bool,
    },
}

/// Returns `true` if this is an attribute with a [JsxString] initializer that does not contain any new line characters.
fn is_single_line_string_literal_attribute(attribute: &AnyJsxAttribute) -> bool {
    as_string_literal_attribute_value(attribute).is_some_and(|string| {
        string
            .value_token()
            .is_ok_and(|text| !text.text_trimmed().contains('\n'))
    })
}

/// Returns `true` if this is an attribute with a [JsxString] initializer that contains at least one new line character.
fn is_multiline_string_literal_attribute(attribute: &AnyJsxAttribute) -> bool {
    as_string_literal_attribute_value(attribute).is_some_and(|string| {
        string
            .value_token()
            .is_ok_and(|text| text.text_trimmed().contains('\n'))
    })
}

/// Returns `Some` if the initializer value of this attribute is a [JsxString].
/// Returns [None] otherwise.
fn as_string_literal_attribute_value(attribute: &AnyJsxAttribute) -> Option<JsxString> {
    use AnyJsxAttribute::*;
    use AnyJsxAttributeValue::*;

    match attribute {
        JsxAttribute(attribute) => {
            attribute
                .initializer()
                .and_then(|initializer| match initializer.value() {
                    Ok(JsxString(string)) => Some(string),

                    _ => None,
                })
        }
        JsxSpreadAttribute(_) => None,
    }
}
