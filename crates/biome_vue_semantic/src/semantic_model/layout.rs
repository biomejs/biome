//! The layout pass: reads the host document of a single-file component into
//! blocks, elements, attributes and conditional chains.
//!
//! This pass looks at the HTML tree only. Directive values are JavaScript and
//! are attached to their attributes by the snippet pass.

use super::builder::{ModelBuilder, synthetic_name};
use super::model::*;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyHtmlAttributeInitializer, AnyHtmlContent, AnyHtmlElement, AnyHtmlTagName,
    AnyVueDirective, AnyVueDirectiveArgument, HtmlAttributeInitializerClause, HtmlAttributeList,
    HtmlElementList, HtmlRoot, HtmlSyntaxKind, VueDirectiveArgument, VueModifierList,
    VueVForIdentifierBinding, VueVForValue,
};
use biome_rowan::{AstNode, AstNodeList, TextRange, TextSize, TokenText};

/// A `v-for` directive, whose variables the host parser reads itself.
pub(crate) struct VFor {
    pub(crate) element: ElementId,
    /// The name and range of each variable the directive introduces.
    pub(crate) bindings: Vec<(TokenText, TextRange)>,
}

/// What the layout pass leaves for the later passes.
#[derive(Default)]
pub(crate) struct Layout {
    pub(crate) v_fors: Vec<VFor>,
    /// Classes found so far, with the element that carries each.
    pub(crate) class_entries: Vec<(ElementId, ClassEntryData)>,
    /// Branches whose condition is the value of the given attribute.
    pub(crate) branch_conditions: Vec<(ChainId, usize, AttrId)>,
    /// Static `ref="name"` attributes: the element, the name and its range.
    pub(crate) template_refs: Vec<(ElementId, TokenText, TextRange)>,
}

pub(crate) fn collect_layout(builder: &mut ModelBuilder, root: &HtmlRoot) -> Layout {
    let mut layout = Layout::default();
    visit_children(builder, &mut layout, &root.html(), None);
    layout
}

/// The parts of an element the layout pass reads, for both `<a></a>` and
/// `<a />`.
struct Tag {
    range: TextRange,
    start_tag: TextRange,
    name: AnyHtmlTagName,
    attributes: HtmlAttributeList,
    children: Option<HtmlElementList>,
    /// The range between the opening and closing tags.
    content: TextRange,
    self_closing: bool,
}

impl Tag {
    fn from_element(element: &AnyHtmlElement) -> Option<Self> {
        match element {
            AnyHtmlElement::HtmlElement(element) => {
                let opening = element.opening_element().ok()?;
                let start_tag = opening.syntax().text_trimmed_range();
                let content_end = element
                    .closing_element()
                    .ok()
                    .map_or(element.syntax().text_trimmed_range().end(), |closing| {
                        closing.syntax().text_trimmed_range().start()
                    });
                Some(Self {
                    range: element.syntax().text_trimmed_range(),
                    start_tag,
                    name: opening.name().ok()?,
                    attributes: opening.attributes(),
                    children: Some(element.children()),
                    content: TextRange::new(start_tag.end(), content_end.max(start_tag.end())),
                    self_closing: false,
                })
            }
            AnyHtmlElement::HtmlSelfClosingElement(element) => {
                let range = element.syntax().text_trimmed_range();
                Some(Self {
                    range,
                    start_tag: range,
                    name: element.name().ok()?,
                    attributes: element.attributes(),
                    children: None,
                    content: TextRange::empty(range.end()),
                    self_closing: true,
                })
            }
            _ => None,
        }
    }
}

fn visit_children(
    builder: &mut ModelBuilder,
    layout: &mut Layout,
    children: &HtmlElementList,
    parent: Option<ElementId>,
) {
    let mut chain: Vec<(ElementId, BranchKind, Option<AttrId>)> = Vec::new();
    for child in children.iter() {
        let Some(tag) = Tag::from_element(&child) else {
            // Whitespace between two branches does not break a chain. Any
            // other content does.
            let is_blank = matches!(
                &child,
                AnyHtmlElement::AnyHtmlContent(AnyHtmlContent::HtmlContent(content))
                    if content.value_token().is_ok_and(|token| token.text_trimmed().trim().is_empty())
            );
            if !is_blank {
                flush_chain(builder, layout, &mut chain, parent);
            }
            continue;
        };
        let element = visit_tag(builder, layout, &tag, parent);

        let branch = builder.data.elements[element.index()]
            .attrs
            .clone()
            .find_map(|index| {
                let attr = AttrId(index);
                let kind = match &builder.data.attrs[attr.index()].name {
                    AttrName::Directive {
                        kind: DirectiveKind::If,
                        ..
                    } => BranchKind::If,
                    AttrName::Directive {
                        kind: DirectiveKind::ElseIf,
                        ..
                    } => BranchKind::ElseIf,
                    AttrName::Directive {
                        kind: DirectiveKind::Else,
                        ..
                    } => BranchKind::Else,
                    _ => return None,
                };
                Some((kind, attr))
            });
        match branch {
            Some((BranchKind::If, attr)) => {
                flush_chain(builder, layout, &mut chain, parent);
                chain.push((element, BranchKind::If, Some(attr)));
            }
            Some((kind, attr)) => {
                // A `v-else` or `v-else-if` with no `v-if` before it still
                // forms a chain, so that rules can see it is orphaned.
                chain.push((element, kind, (kind == BranchKind::ElseIf).then_some(attr)));
                if kind == BranchKind::Else {
                    flush_chain(builder, layout, &mut chain, parent);
                }
            }
            None => flush_chain(builder, layout, &mut chain, parent),
        }
    }
    flush_chain(builder, layout, &mut chain, parent);
}

fn flush_chain(
    builder: &mut ModelBuilder,
    layout: &mut Layout,
    chain: &mut Vec<(ElementId, BranchKind, Option<AttrId>)>,
    parent: Option<ElementId>,
) {
    if chain.is_empty() {
        return;
    }
    let id = ChainId::new(builder.data.chains.len());
    let mut branches = Vec::with_capacity(chain.len());
    for (index, (element, kind, condition)) in chain.drain(..).enumerate() {
        builder.data.elements[element.index()].chain = Some((id, index as u8));
        if let Some(attr) = condition {
            layout.branch_conditions.push((id, index, attr));
        }
        branches.push(BranchData {
            element,
            kind,
            condition: None,
        });
    }
    builder.data.chains.push(ChainData {
        parent,
        branches: branches.into_boxed_slice(),
    });
}

fn visit_tag(
    builder: &mut ModelBuilder,
    layout: &mut Layout,
    tag: &Tag,
    parent: Option<ElementId>,
) -> ElementId {
    let id = ElementId::new(builder.data.elements.len());
    let tag_text = tag.name.syntax().text_trimmed().to_string();
    let tag_name = match &tag.name {
        AnyHtmlTagName::HtmlTagName(name) => name.token_text_trimmed(),
        _ => None,
    }
    .or_else(|| tag.name.token_text_trimmed())
    .filter(|name| name.text() == tag_text)
    .unwrap_or_else(|| synthetic_name(&tag_text));
    let is_component =
        !matches!(tag.name, AnyHtmlTagName::HtmlTagName(_)) || tag_text.contains('-');

    let parent_scope = parent.map(|parent| builder.data.elements[parent.index()].scope);
    let scope = builder.add_scope(ScopeKind::Element(id), parent_scope);

    builder.data.elements.push(ElementData {
        range: tag.range,
        start_tag: tag.start_tag,
        parent,
        tag: tag_name,
        is_component,
        tag_ref: None,
        attrs: 0..0,
        classes: 0..0,
        scope,
        chain: None,
        self_closing: tag.self_closing,
        class_open: false,
    });

    let first_attr = builder.data.attrs.len() as u32;
    for attribute in tag.attributes.iter() {
        visit_attribute(builder, layout, &attribute, id);
    }
    builder.data.elements[id.index()].attrs = first_attr..builder.data.attrs.len() as u32;

    if parent.is_none() {
        add_block(builder, tag, id, &tag_text);
    }

    // The content of `<script>` and `<style>` is not HTML, so the parser
    // gives those elements no element children.
    if let Some(children) = &tag.children {
        visit_children(builder, layout, children, Some(id));
    }
    id
}

fn add_block(builder: &mut ModelBuilder, tag: &Tag, element: ElementId, tag_text: &str) {
    let kind = match tag_text {
        "template" => BlockKind::Template,
        "script" => BlockKind::Script,
        "style" => BlockKind::Style,
        _ => BlockKind::Custom,
    };
    let mut block = BlockData {
        kind,
        element,
        content: tag.content,
        lang: None,
        setup: false,
        scoped: false,
        module: false,
        has_src: false,
        is_empty: tag.children.as_ref().is_none_or(|children| {
            children
                .syntax()
                .text_trimmed()
                .to_string()
                .trim()
                .is_empty()
        }),
        snippet: None,
    };
    for index in builder.data.elements[element.index()].attrs.clone() {
        let attr = &builder.data.attrs[index as usize];
        let AttrName::Plain(name) = &attr.name else {
            continue;
        };
        match name.text() {
            "lang" => {
                if let AttrValueData::Static(value) = &attr.value {
                    block.lang = Some(value.clone());
                }
            }
            "setup" => block.setup = true,
            "scoped" => block.scoped = true,
            "module" => block.module = true,
            "src" => block.has_src = true,
            _ => {}
        }
    }
    builder.data.blocks.push(block);
}

fn visit_attribute(
    builder: &mut ModelBuilder,
    layout: &mut Layout,
    attribute: &AnyHtmlAttribute,
    element: ElementId,
) {
    let range = attribute.syntax().text_trimmed_range();
    match attribute {
        AnyHtmlAttribute::HtmlAttribute(plain) => {
            let Some(name) = plain.name().ok().and_then(|name| name.token_text_trimmed()) else {
                return;
            };
            let (value, value_range) = match plain
                .initializer()
                .and_then(|initializer| initializer.value().ok())
            {
                None => (AttrValueData::Absent, None),
                Some(AnyHtmlAttributeInitializer::HtmlString(string)) => {
                    match (string.inner_string_text(), string.inner_string_range()) {
                        (Ok(text), Ok(range)) => (AttrValueData::Static(text), Some(range)),
                        _ => (AttrValueData::Unparsed, None),
                    }
                }
                Some(_) => (AttrValueData::Unparsed, None),
            };
            if let (AttrValueData::Static(text), Some(value_range)) = (&value, value_range) {
                match name.text() {
                    "class" => {
                        for (class, class_range) in split_classes(text.text(), value_range.start())
                        {
                            layout.class_entries.push((
                                element,
                                ClassEntryData {
                                    name: class.into(),
                                    range: class_range,
                                    certainty: Certainty::Always,
                                    from_expr: false,
                                },
                            ));
                        }
                    }
                    "ref" => layout
                        .template_refs
                        .push((element, text.clone(), value_range)),
                    _ => {}
                }
            }
            builder.data.attrs.push(AttrData {
                element,
                range,
                name: AttrName::Plain(name),
                value,
                value_range,
            });
        }
        AnyHtmlAttribute::AnyVueDirective(directive) => {
            let Some(parts) = DirectiveParts::from_directive(directive) else {
                return;
            };
            let kind = DirectiveKind::from_name(parts.name.text());
            let (value, value_range) = match parts
                .initializer
                .and_then(|initializer| initializer.value().ok())
            {
                None => (AttrValueData::Absent, None),
                Some(AnyHtmlAttributeInitializer::HtmlString(string)) => (
                    // The snippet pass replaces this once it has found the
                    // snippet that holds the value.
                    AttrValueData::Unparsed,
                    string.inner_string_range().ok(),
                ),
                Some(AnyHtmlAttributeInitializer::VueVForValue(v_for)) => {
                    layout.v_fors.push(VFor {
                        element,
                        bindings: v_for_bindings(&v_for),
                    });
                    (
                        AttrValueData::Unparsed,
                        Some(v_for.syntax().text_trimmed_range()),
                    )
                }
                Some(_) => (AttrValueData::Unparsed, None),
            };
            builder.data.attrs.push(AttrData {
                element,
                range,
                name: AttrName::Directive {
                    kind,
                    name: parts.name,
                    arg: parts.arg,
                    modifiers: parts.modifiers,
                    shorthand: parts.shorthand,
                    name_ref: None,
                },
                value,
                value_range,
            });
        }
        _ => {}
    }
}

/// The parts of a directive, for both the long and the short forms.
struct DirectiveParts {
    name: TokenText,
    arg: DirectiveArg,
    modifiers: Box<[TokenText]>,
    shorthand: bool,
    initializer: Option<HtmlAttributeInitializerClause>,
}

impl DirectiveParts {
    fn from_directive(directive: &AnyVueDirective) -> Option<Self> {
        Some(match directive {
            AnyVueDirective::VueDirective(directive) => Self {
                name: directive.name_token().ok()?.token_text_trimmed(),
                arg: directive
                    .arg()
                    .map_or(DirectiveArg::None, |arg| wrapped_argument(&arg)),
                modifiers: modifiers(&directive.modifiers()),
                shorthand: false,
                initializer: directive.initializer(),
            },
            AnyVueDirective::VueVBindShorthandDirective(directive) => Self {
                name: synthetic_name("v-bind"),
                arg: directive
                    .arg()
                    .map_or(DirectiveArg::None, |arg| wrapped_argument(&arg)),
                modifiers: modifiers(&directive.modifiers()),
                shorthand: true,
                initializer: directive.initializer(),
            },
            AnyVueDirective::VueVOnShorthandDirective(directive) => Self {
                name: synthetic_name("v-on"),
                arg: directive
                    .arg()
                    .map_or(DirectiveArg::None, |arg| argument(&arg)),
                modifiers: modifiers(&directive.modifiers()),
                shorthand: true,
                initializer: directive.initializer(),
            },
            AnyVueDirective::VueVSlotShorthandDirective(directive) => Self {
                name: synthetic_name("v-slot"),
                arg: directive
                    .arg()
                    .map_or(DirectiveArg::None, |arg| argument(&arg)),
                modifiers: modifiers(&directive.modifiers()),
                shorthand: true,
                initializer: directive.initializer(),
            },
            AnyVueDirective::VueBogusDirective(_) => return None,
        })
    }
}

fn wrapped_argument(arg: &VueDirectiveArgument) -> DirectiveArg {
    arg.arg().map_or(DirectiveArg::None, |arg| argument(&arg))
}

fn argument(arg: &AnyVueDirectiveArgument) -> DirectiveArg {
    match arg {
        AnyVueDirectiveArgument::VueStaticArgument(arg) => {
            arg.name_token().map_or(DirectiveArg::None, |token| {
                DirectiveArg::Static(token.token_text_trimmed())
            })
        }
        AnyVueDirectiveArgument::VueDynamicArgument(arg) => {
            arg.name_token().map_or(DirectiveArg::None, |token| {
                DirectiveArg::Dynamic(token.token_text_trimmed())
            })
        }
        AnyVueDirectiveArgument::VueBogusDirectiveArgument(_) => DirectiveArg::None,
    }
}

fn modifiers(list: &VueModifierList) -> Box<[TokenText]> {
    list.iter()
        .filter_map(|modifier| modifier.modifier_token().ok())
        .map(|token| token.token_text_trimmed())
        .collect()
}

/// Returns the variables a `v-for` introduces: `item` and `index` for
/// `(item, index) in items`.
fn v_for_bindings(v_for: &VueVForValue) -> Vec<(TokenText, TextRange)> {
    let Ok(binding) = v_for.binding() else {
        return Vec::new();
    };
    binding
        .syntax()
        .descendants()
        .filter_map(VueVForIdentifierBinding::cast)
        .filter(|identifier| {
            // In `{ key: alias }` the key names a property and introduces no
            // variable.
            !identifier.syntax().parent().is_some_and(|parent| {
                parent.kind() == HtmlSyntaxKind::VUE_V_FOR_OBJECT_PROPERTY_BINDING
                    && parent.first_child().as_ref() == Some(identifier.syntax())
            })
        })
        .filter_map(|identifier| {
            let token = identifier.name_token().ok()?;
            Some((token.token_text_trimmed(), token.text_trimmed_range()))
        })
        .collect()
}

/// Splits a `class` value into class names, with the range of each.
///
/// `start` is the position of the first byte of `value`.
pub(crate) fn split_classes(value: &str, start: TextSize) -> Vec<(&str, TextRange)> {
    let position = |index: usize| start + TextSize::from(index as u32);
    let mut classes = Vec::new();
    let mut class_start = None;
    for (index, character) in value.char_indices() {
        match (character.is_whitespace(), class_start) {
            (false, None) => class_start = Some(index),
            (true, Some(from)) => {
                classes.push((
                    &value[from..index],
                    TextRange::new(position(from), position(index)),
                ));
                class_start = None;
            }
            _ => {}
        }
    }
    if let Some(from) = class_start {
        classes.push((
            &value[from..],
            TextRange::new(position(from), position(value.len())),
        ));
    }
    classes
}
