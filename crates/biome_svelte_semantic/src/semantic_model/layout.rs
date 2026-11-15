//! The layout pass: reads the host document of a component into scripts,
//! styles, elements, attributes, logic blocks and the scopes of the
//! template.
//!
//! This pass looks at the HTML tree only. It records where each embedded
//! expression is expected, and the snippet pass attaches the snippets the
//! workspace parsed to those places.

use super::builder::{ModelBuilder, NewSymbol, synthetic_name};
use super::model::*;
use super::patterns::pattern_bindings;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyHtmlAttributeInitializer, AnyHtmlComponentObjectName, AnyHtmlContent,
    AnyHtmlElement, AnyHtmlTagName, AnyHtmlTextExpression, AnySvelteAwaitClauses,
    AnySvelteBindingAssignmentBinding, AnySvelteBindingProperty, AnySvelteBlock,
    AnySvelteBlockItem, AnySvelteDestructuredName, AnySvelteDirective,
    AnySvelteDirectiveInitializerClause, AnySvelteEachName, AnySvelteMemberObject,
    AnySvelteTemplateElement, HtmlAttributeList, HtmlElementList, HtmlRoot, HtmlSyntaxNode,
    HtmlTextExpression, SvelteDirectiveValue, SvelteName,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, TextRange, TokenText};
use rustc_hash::FxHashMap;

/// Where an embedded expression of the template sits, recorded before the
/// snippets are read.
#[derive(Clone, Copy)]
pub(crate) struct ExpectedSnippet {
    pub(crate) host: SnippetHost,
    /// The scope the free names of the expression are looked up in.
    pub(crate) scope: ScopeId,
    pub(crate) in_snippet_block: bool,
}

/// A name the host document refers to outside of any snippet.
pub(crate) struct MarkupRef {
    pub(crate) name: TokenText,
    pub(crate) range: TextRange,
    pub(crate) site: RefSite,
    pub(crate) path: Box<[TokenText]>,
    pub(crate) access: Access,
    pub(crate) scope: ScopeId,
    pub(crate) in_snippet_block: bool,
}

/// What the layout pass leaves for the later passes.
#[derive(Default)]
pub(crate) struct Layout {
    /// The expressions of the template, by the range of their text in the
    /// host document.
    pub(crate) expected: FxHashMap<TextRange, ExpectedSnippet>,
    pub(crate) markup_refs: Vec<MarkupRef>,
    /// For each `{#snippet}`, the scope its name is declared in.
    pub(crate) snippet_name_scopes: Vec<(BlockId, ScopeId)>,
    /// For each `{#each}`, the variables it declares for its items.
    pub(crate) each_items: Vec<(BlockId, Vec<SymbolId>)>,
    /// The `runes` attribute of `<svelte:options>`, when its value is an
    /// expression.
    pub(crate) runes_attr: Option<AttrId>,
}

/// The place in the template the pass is reading.
#[derive(Clone, Copy)]
struct Context {
    parent: Option<ElementId>,
    scope: ScopeId,
    occurrence: Occurrence,
    in_snippet_block: bool,
    /// Whether the children are at the root of the document.
    is_root: bool,
}

pub(crate) fn collect_layout(builder: &mut ModelBuilder, root: &HtmlRoot) -> Layout {
    let module = builder.add_scope(ScopeKind::Module, None);
    let instance = builder.add_scope(ScopeKind::Instance, Some(module));
    let template = builder.add_scope(ScopeKind::Template, Some(instance));
    builder.data.module_scope = Some(module);
    builder.data.instance_scope = Some(instance);
    builder.data.template_scope = Some(template);

    let mut visitor = LayoutVisitor {
        builder,
        layout: Layout::default(),
    };
    visitor.visit_children(
        &root.html(),
        Context {
            parent: None,
            scope: template,
            occurrence: Occurrence::Once,
            in_snippet_block: false,
            is_root: true,
        },
    );
    visitor.layout
}

/// The parts of an element the layout pass reads, for both `<a></a>` and
/// `<a />`.
struct Tag {
    node: HtmlSyntaxNode,
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
                    node: element.syntax().clone(),
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
                    node: element.syntax().clone(),
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

/// The range of the text of an expression, which is also the content range
/// of the snippet the workspace parses from it.
fn expression_range(expression: &HtmlTextExpression) -> Option<TextRange> {
    Some(expression.html_literal_token().ok()?.text_range())
}

struct LayoutVisitor<'a> {
    builder: &'a mut ModelBuilder,
    layout: Layout,
}

impl LayoutVisitor<'_> {
    fn expect(&mut self, expression: &HtmlTextExpression, host: SnippetHost, context: Context) {
        if let Some(range) = expression_range(expression) {
            self.layout.expected.insert(
                range,
                ExpectedSnippet {
                    host,
                    scope: context.scope,
                    in_snippet_block: context.in_snippet_block,
                },
            );
        }
    }

    fn visit_children(&mut self, children: &HtmlElementList, context: Context) {
        for child in children.iter() {
            match &child {
                AnyHtmlElement::HtmlElement(_) | AnyHtmlElement::HtmlSelfClosingElement(_) => {
                    if let Some(tag) = Tag::from_element(&child) {
                        self.visit_tag(&tag, context);
                    }
                }
                AnyHtmlElement::AnyHtmlContent(AnyHtmlContent::AnyHtmlTextExpression(
                    expression,
                )) => match expression {
                    AnyHtmlTextExpression::HtmlSingleTextExpression(text) => {
                        if let Some(expression) = text.expression() {
                            self.expect(&expression, SnippetHost::Text(context.parent), context);
                        }
                    }
                    AnyHtmlTextExpression::AnySvelteBlock(block) => {
                        self.visit_block(block, context);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn visit_tag(&mut self, tag: &Tag, context: Context) {
        let id = ElementId::new(self.builder.data.elements.len());
        let tag_text = tag.name.syntax().text_trimmed().to_string();
        let tag_name = tag
            .name
            .token_text_trimmed()
            .filter(|name| name.text() == tag_text)
            .unwrap_or_else(|| synthetic_name(&tag_text));
        let kind = match &tag.name {
            AnyHtmlTagName::HtmlComponentName(_) | AnyHtmlTagName::HtmlMemberName(_) => {
                ElementKind::Component
            }
            _ if tag_text == "svelte:element" => ElementKind::DynamicElement,
            _ if tag_text.starts_with("svelte:") => ElementKind::Special,
            _ => ElementKind::Dom,
        };
        let node = self.builder.add_html_node(&tag.node);
        self.builder.data.elements.push(ElementData {
            range: tag.range,
            start_tag: tag.start_tag,
            content: tag.content,
            parent: context.parent,
            tag: tag_name,
            kind,
            tag_ref: None,
            attrs: 0..0,
            scope: context.scope,
            occurrence: context.occurrence,
            self_closing: tag.self_closing,
            node,
        });

        if kind == ElementKind::Component {
            self.add_tag_reference(&tag.name, id, context);
        }

        let first_attr = self.builder.data.attrs.len() as u32;
        for attribute in tag.attributes.iter() {
            self.visit_attribute(&attribute, id, context);
        }
        self.builder.data.elements[id.index()].attrs =
            first_attr..self.builder.data.attrs.len() as u32;

        if context.is_root {
            match tag_text.as_str() {
                "script" => {
                    self.add_script(tag, id);
                    return;
                }
                "style" => {
                    self.add_style(tag, id);
                    return;
                }
                "svelte:options" => self.read_options(id),
                _ => {}
            }
        }

        let Some(children) = &tag.children else {
            return;
        };
        // The children of an element see the snippets declared among them,
        // which nothing outside the element sees.
        let mut child_context = Context {
            parent: Some(id),
            scope: self
                .builder
                .add_scope(ScopeKind::Children(id), Some(context.scope)),
            is_root: false,
            ..context
        };
        // The children of a component make up its `children` snippet, which
        // the component can render any number of times.
        if kind == ElementKind::Component {
            child_context.occurrence = context.occurrence.then(Occurrence::Any);
        }
        self.visit_children(children, child_context);
    }

    /// Records the reference from a component tag to the variable that holds
    /// the component: `Button` for `<Button>`, `ui` for `<ui.Button>`.
    fn add_tag_reference(&mut self, name: &AnyHtmlTagName, element: ElementId, context: Context) {
        let mut path = Vec::new();
        let mut current = match name {
            AnyHtmlTagName::HtmlMemberName(member) => {
                AnyHtmlComponentObjectName::HtmlMemberName(member.clone())
            }
            AnyHtmlTagName::HtmlComponentName(name) => {
                AnyHtmlComponentObjectName::HtmlComponentName(name.clone())
            }
            _ => return,
        };
        let token = loop {
            current = match current {
                AnyHtmlComponentObjectName::HtmlMemberName(member) => {
                    if let Some(token) = member
                        .member()
                        .ok()
                        .and_then(|member| member.value_token().ok())
                    {
                        path.push(token.token_text_trimmed());
                    }
                    let Ok(object) = member.object() else {
                        return;
                    };
                    object
                }
                AnyHtmlComponentObjectName::HtmlComponentName(name) => {
                    break name.value_token().ok();
                }
                AnyHtmlComponentObjectName::HtmlTagName(name) => {
                    break name.value_token().ok();
                }
            };
        };
        let Some(token) = token else {
            return;
        };
        path.reverse();
        self.layout.markup_refs.push(MarkupRef {
            name: token.token_text_trimmed(),
            range: token.text_trimmed_range(),
            site: RefSite::TagName(element),
            path: path.into_boxed_slice(),
            access: Access::Read,
            scope: context.scope,
            in_snippet_block: context.in_snippet_block,
        });
    }

    fn add_script(&mut self, tag: &Tag, element: ElementId) {
        let mut kind = ScriptKind::Instance;
        let mut lang = None;
        let mut has_generics = false;
        for index in self.builder.data.elements[element.index()].attrs.clone() {
            let attr = &self.builder.data.attrs[index as usize];
            let AttrName::Plain(name) = &attr.name else {
                continue;
            };
            match (name.text(), &attr.value) {
                ("module", _) => kind = ScriptKind::Module,
                ("context", AttrValueData::Static(value)) if value.text() == "module" => {
                    kind = ScriptKind::Module;
                }
                ("lang", AttrValueData::Static(value)) => lang = Some(value.clone()),
                ("generics", _) => has_generics = true,
                _ => {}
            }
        }
        let scope = match kind {
            ScriptKind::Instance => self.builder.data.instance_scope,
            ScriptKind::Module => self.builder.data.module_scope,
        };
        self.builder.data.scripts.push(ScriptData {
            kind,
            element,
            content: tag.content,
            lang,
            snippet: None,
            generics: None,
            has_generics,
            scope: scope.expect("the layout pass creates the script scopes first"),
        });
    }

    fn add_style(&mut self, tag: &Tag, element: ElementId) {
        let lang = self.builder.data.elements[element.index()]
            .attrs
            .clone()
            .find_map(|index| {
                let attr = &self.builder.data.attrs[index as usize];
                match (&attr.name, &attr.value) {
                    (AttrName::Plain(name), AttrValueData::Static(value))
                        if name.text() == "lang" =>
                    {
                        Some(value.clone())
                    }
                    _ => None,
                }
            });
        self.builder.data.styles.push(StyleData {
            element,
            content: tag.content,
            lang,
        });
    }

    /// Reads `runes` from `<svelte:options>` when it is written alone or with
    /// a quoted value. A value written as an expression, as in
    /// `runes={false}`, is read by the snippet pass.
    fn read_options(&mut self, element: ElementId) {
        for index in self.builder.data.elements[element.index()].attrs.clone() {
            let attr = &self.builder.data.attrs[index as usize];
            if !matches!(&attr.name, AttrName::Plain(name) if name.text() == "runes") {
                continue;
            }
            match &attr.value {
                AttrValueData::Absent => self.builder.data.runes_option = Some(true),
                AttrValueData::Static(value) => {
                    self.builder.data.runes_option = match value.text() {
                        "true" => Some(true),
                        "false" => Some(false),
                        _ => None,
                    };
                }
                _ => self.layout.runes_attr = Some(AttrId(index)),
            }
        }
    }

    fn visit_attribute(
        &mut self,
        attribute: &AnyHtmlAttribute,
        element: ElementId,
        context: Context,
    ) {
        let id = AttrId::new(self.builder.data.attrs.len());
        let range = attribute.syntax().text_trimmed_range();
        let (name, value) = match attribute {
            AnyHtmlAttribute::HtmlAttribute(plain) => {
                let Some(name) = plain
                    .name()
                    .ok()
                    .and_then(|name| name.value_token().ok())
                    .map(|token| token.token_text_trimmed())
                else {
                    return;
                };
                let value = match plain
                    .initializer()
                    .and_then(|initializer| initializer.value().ok())
                {
                    None => AttrValueData::Absent,
                    Some(value) => self.initializer_value(&value, id, context),
                };
                (AttrName::Plain(name), value)
            }
            AnyHtmlAttribute::HtmlAttributeSingleTextExpression(shorthand) => {
                let Ok(expression) = shorthand.expression() else {
                    return;
                };
                let name = expression
                    .html_literal_token()
                    .map_or_else(|_| synthetic_name(""), |token| token.token_text_trimmed());
                self.expect(&expression, SnippetHost::AttrValue(id), context);
                (AttrName::Shorthand(name), AttrValueData::Expr(None))
            }
            AnyHtmlAttribute::HtmlSpreadAttribute(spread) => {
                if let Ok(expression) = spread.argument() {
                    self.expect(&expression, SnippetHost::AttrValue(id), context);
                }
                (AttrName::Spread, AttrValueData::Expr(None))
            }
            AnyHtmlAttribute::SvelteAttachAttribute(attach) => {
                if let Ok(expression) = attach.expression() {
                    self.expect(&expression, SnippetHost::AttrValue(id), context);
                }
                (AttrName::Attach, AttrValueData::Expr(None))
            }
            AnyHtmlAttribute::AnySvelteDirective(directive) => {
                let Some((kind, directive_value)) = directive_parts(directive) else {
                    return;
                };
                let Ok(property) = directive_value.property() else {
                    return;
                };
                let name = synthetic_name(&property.syntax().text_trimmed().to_string());
                let modifiers = directive_value
                    .modifiers()
                    .iter()
                    .filter_map(|modifier| modifier.name().ok()?.ident_token().ok())
                    .map(|token| token.token_text_trimmed())
                    .collect();
                if kind.names_a_function() {
                    self.add_directive_name_reference(&property, id, context);
                }
                let value = match directive_value.initializer() {
                    None => {
                        self.add_implicit_reference(kind, &property, id, context);
                        AttrValueData::Absent
                    }
                    Some(AnySvelteDirectiveInitializerClause::HtmlAttributeInitializerClause(
                        initializer,
                    )) => match initializer.value() {
                        Ok(value) => self.initializer_value(&value, id, context),
                        Err(_) => AttrValueData::Absent,
                    },
                    Some(
                        AnySvelteDirectiveInitializerClause::SvelteBindFunctionBindingInitializerClause(
                            initializer,
                        ),
                    ) => {
                        if let Ok(binding) = initializer.value() {
                            if let Ok(get) = binding.get() {
                                self.expect(&get, SnippetHost::BindGetter(id), context);
                            }
                            if let Ok(set) = binding.set() {
                                self.expect(&set, SnippetHost::BindSetter(id), context);
                            }
                        }
                        AttrValueData::FunctionBinding {
                            get: None,
                            set: None,
                        }
                    }
                };
                (
                    AttrName::Directive {
                        kind,
                        name,
                        modifiers,
                        name_ref: None,
                    },
                    value,
                )
            }
            _ => return,
        };
        let node = self.builder.add_html_node(attribute.syntax());
        self.builder.data.attrs.push(AttrData {
            element,
            range,
            name,
            value,
            implicit_ref: None,
            node,
        });
    }

    fn initializer_value(
        &mut self,
        value: &AnyHtmlAttributeInitializer,
        id: AttrId,
        context: Context,
    ) -> AttrValueData {
        match value {
            AnyHtmlAttributeInitializer::HtmlString(string) => match string.inner_string_text() {
                Ok(text) => AttrValueData::Static(text),
                Err(_) => AttrValueData::Absent,
            },
            AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(expression) => {
                if let Ok(expression) = expression.expression() {
                    self.expect(&expression, SnippetHost::AttrValue(id), context);
                }
                AttrValueData::Expr(None)
            }
            AnyHtmlAttributeInitializer::SvelteTemplateAttributeValue(template) => {
                let mut parts = Vec::new();
                for (index, element) in template.elements().iter().enumerate() {
                    match element {
                        AnySvelteTemplateElement::SvelteTemplateChunkElement(chunk) => {
                            if let Ok(token) = chunk.html_template_chunk_token() {
                                parts.push(ValuePartData::Text(token.token_text_trimmed()));
                            }
                        }
                        AnySvelteTemplateElement::HtmlAttributeSingleTextExpression(expression) => {
                            if let Ok(expression) = expression.expression() {
                                self.expect(
                                    &expression,
                                    SnippetHost::AttrPart(id, index as u16),
                                    context,
                                );
                            }
                            parts.push(ValuePartData::Expr(None));
                        }
                    }
                }
                AttrValueData::Interpolated(parts.into_boxed_slice())
            }
            AnyHtmlAttributeInitializer::VueVForValue(_) => AttrValueData::Absent,
        }
    }
}

impl LayoutVisitor<'_> {
    /// Records the reference from the name of a directive to the function it
    /// names, as `fade` in `transition:fade` or `tooltip` in
    /// `use:tooltip.show`.
    fn add_directive_name_reference(
        &mut self,
        property: &AnySvelteBindingProperty,
        attr: AttrId,
        context: Context,
    ) {
        let Some((token, path)) = property_name(property) else {
            return;
        };
        self.layout.markup_refs.push(MarkupRef {
            name: token.token_text_trimmed(),
            range: token.text_trimmed_range(),
            site: RefSite::DirectiveName(attr),
            path,
            access: Access::Call,
            scope: context.scope,
            in_snippet_block: context.in_snippet_block,
        });
    }

    /// Records the reference a directive without a value makes through its
    /// name: `bind:value` binds the variable `value`, `class:active` and
    /// `style:color` read the variables `active` and `color`.
    fn add_implicit_reference(
        &mut self,
        kind: DirectiveKind,
        property: &AnySvelteBindingProperty,
        attr: AttrId,
        context: Context,
    ) {
        let access = match kind {
            DirectiveKind::Bind => Access::Write,
            DirectiveKind::Class | DirectiveKind::Style => Access::Read,
            _ => return,
        };
        let token = match property {
            AnySvelteBindingProperty::SvelteName(name) => name.ident_token().ok(),
            AnySvelteBindingProperty::SvelteLiteral(literal) => literal.value_token().ok(),
            AnySvelteBindingProperty::SvelteMemberProperty(_) => None,
        };
        let Some(token) = token.filter(|token| is_identifier(token.text_trimmed())) else {
            return;
        };
        self.layout.markup_refs.push(MarkupRef {
            name: token.token_text_trimmed(),
            range: token.text_trimmed_range(),
            site: RefSite::DirectiveShorthand(attr),
            path: Box::default(),
            access,
            scope: context.scope,
            in_snippet_block: context.in_snippet_block,
        });
    }

    fn add_block(&mut self, kind: BlockKind, node: &HtmlSyntaxNode, context: Context) -> BlockId {
        let id = BlockId::new(self.builder.data.blocks.len());
        let html_node = self.builder.add_html_node(node);
        let first_branch = self.builder.data.branches.len() as u32;
        self.builder.data.blocks.push(BlockData {
            kind,
            range: node.text_trimmed_range(),
            parent: context.parent,
            expression: None,
            key: None,
            branches: first_branch..first_branch,
            node: html_node,
        });
        id
    }

    /// Adds a branch of `block` whose content is `children`, and returns the
    /// context its content is read in.
    fn add_branch(
        &mut self,
        block: BlockId,
        kind: BranchKind,
        children: &HtmlElementList,
        context: Context,
    ) -> (BranchId, Context) {
        let id = BranchId::new(self.builder.data.branches.len());
        let scope = self
            .builder
            .add_scope(ScopeKind::Branch(id), Some(context.scope));
        self.builder.data.branches.push(BranchData {
            block,
            kind,
            range: children.syntax().text_trimmed_range(),
            condition: None,
            scope,
        });
        self.builder.data.blocks[block.index()].branches.end = id.0 + 1;
        let occurrence = match kind {
            BranchKind::EachBody | BranchKind::SnippetBody => Occurrence::Any,
            BranchKind::KeyBody => Occurrence::Once,
            _ => Occurrence::AtMostOnce,
        };
        let branch_context = Context {
            scope,
            occurrence: context.occurrence.then(occurrence),
            in_snippet_block: context.in_snippet_block || kind == BranchKind::SnippetBody,
            is_root: false,
            ..context
        };
        (id, branch_context)
    }

    /// Declares a variable of a block in the scope of `branch`.
    fn add_block_symbol(
        &mut self,
        name: TokenText,
        range: TextRange,
        kind: SymbolKind,
        context: Context,
    ) -> SymbolId {
        self.builder.add_symbol(NewSymbol {
            name,
            range,
            declaration: range,
            snippet: None,
            scope: context.scope,
            kind,
            declaration_kind: DeclarationKind::BlockVariable,
            node: None,
        })
    }

    /// Declares the variables of a pattern the host parser keeps as text, as
    /// in `{:then { data }}`.
    fn add_text_pattern(
        &mut self,
        expression: &HtmlTextExpression,
        kind: SymbolKind,
        context: Context,
    ) -> Vec<SymbolId> {
        let Ok(token) = expression.html_literal_token() else {
            return Vec::new();
        };
        pattern_bindings(token.text(), token.text_range().start())
            .into_iter()
            .map(|(name, range)| self.add_block_symbol(synthetic_name(&name), range, kind, context))
            .collect()
    }

    fn visit_block(&mut self, block: &AnySvelteBlock, context: Context) {
        match block {
            AnySvelteBlock::SvelteIfBlock(if_block) => {
                let id = self.add_block(BlockKind::If, if_block.syntax(), context);
                if let Ok(opening) = if_block.opening_block() {
                    let (branch, branch_context) =
                        self.add_branch(id, BranchKind::If, &opening.children(), context);
                    if let Ok(expression) = opening.expression() {
                        self.expect(&expression, SnippetHost::Condition(branch), context);
                    }
                    self.visit_children(&opening.children(), branch_context);
                }
                for clause in if_block.else_if_clauses() {
                    let (branch, branch_context) =
                        self.add_branch(id, BranchKind::ElseIf, &clause.children(), context);
                    if let Ok(expression) = clause.expression() {
                        self.expect(&expression, SnippetHost::Condition(branch), context);
                    }
                    self.visit_children(&clause.children(), branch_context);
                }
                if let Some(clause) = if_block.else_clause() {
                    let (_, branch_context) =
                        self.add_branch(id, BranchKind::Else, &clause.children(), context);
                    self.visit_children(&clause.children(), branch_context);
                }
            }
            AnySvelteBlock::SvelteEachBlock(each_block) => {
                let id = self.add_block(BlockKind::Each, each_block.syntax(), context);
                let children = each_block.children();
                let (_, body_context) =
                    self.add_branch(id, BranchKind::EachBody, &children, context);
                if let Ok(opening) = each_block.opening_block() {
                    // The list is evaluated outside the variables of the block.
                    if let Ok(list) = opening.list() {
                        self.expect(&list, SnippetHost::EachList(id), context);
                    }
                    let mut items = Vec::new();
                    let (name, index, key) = match opening.item() {
                        Some(AnySvelteBlockItem::SvelteEachAsKeyedItem(item)) => {
                            (item.name().ok(), item.index(), item.key())
                        }
                        Some(AnySvelteBlockItem::SvelteEachKeyedItem(item)) => {
                            (None, item.index(), None)
                        }
                        None => (None, None, None),
                    };
                    if let Some(name) = name {
                        self.add_each_name(&name, &mut items, body_context);
                    }
                    if let Some(token) = index
                        .and_then(|index| index.value().ok())
                        .and_then(|name| name.ident_token().ok())
                    {
                        self.add_block_symbol(
                            token.token_text_trimmed(),
                            token.text_trimmed_range(),
                            SymbolKind::EachIndex,
                            body_context,
                        );
                    }
                    if let Some(expression) = key.and_then(|key| key.expression().ok()) {
                        self.expect(&expression, SnippetHost::EachKey(id), body_context);
                    }
                    self.layout.each_items.push((id, items));
                }
                self.visit_children(&children, body_context);
                if let Some(clause) = each_block.else_clause() {
                    let (_, else_context) =
                        self.add_branch(id, BranchKind::EachElse, &clause.children(), context);
                    self.visit_children(&clause.children(), else_context);
                }
            }
            AnySvelteBlock::SvelteAwaitBlock(await_block) => {
                let id = self.add_block(BlockKind::Await, await_block.syntax(), context);
                if let Ok(opening) = await_block.opening_block() {
                    if let Ok(expression) = opening.expression() {
                        self.expect(&expression, SnippetHost::AwaitPromise(id), context);
                    }
                    // `{#await promise then value}` shows its content once the
                    // promise resolves, and `{#await promise catch error}` once
                    // it rejects.
                    let (kind, name) = if let Some(clause) = opening.then_clause() {
                        (BranchKind::Then, clause.name())
                    } else if let Some(clause) = opening.catch_clause() {
                        (BranchKind::Catch, clause.name())
                    } else {
                        (BranchKind::Pending, None)
                    };
                    let (_, branch_context) =
                        self.add_branch(id, kind, &opening.children(), context);
                    self.add_await_name(kind, name.as_ref(), branch_context);
                    self.visit_children(&opening.children(), branch_context);
                }
                for clause in await_block.clauses() {
                    let (kind, name, children) = match clause {
                        AnySvelteAwaitClauses::SvelteAwaitThenBlock(then) => {
                            (BranchKind::Then, then.name(), then.children())
                        }
                        AnySvelteAwaitClauses::SvelteAwaitCatchBlock(catch) => {
                            (BranchKind::Catch, catch.name(), catch.children())
                        }
                        AnySvelteAwaitClauses::SvelteBogusBlock(_) => continue,
                    };
                    let (_, branch_context) = self.add_branch(id, kind, &children, context);
                    self.add_await_name(kind, name.as_ref(), branch_context);
                    self.visit_children(&children, branch_context);
                }
            }
            AnySvelteBlock::SvelteKeyBlock(key_block) => {
                let id = self.add_block(BlockKind::Key, key_block.syntax(), context);
                if let Some(expression) = key_block
                    .opening_block()
                    .ok()
                    .and_then(|opening| opening.expression().ok())
                {
                    self.expect(&expression, SnippetHost::KeyExpression(id), context);
                }
                let children = key_block.children();
                let (_, body_context) =
                    self.add_branch(id, BranchKind::KeyBody, &children, context);
                self.visit_children(&children, body_context);
            }
            AnySvelteBlock::SvelteSnippetBlock(snippet_block) => {
                let id = self.add_block(BlockKind::Snippet, snippet_block.syntax(), context);
                let Ok(opening) = snippet_block.opening_block() else {
                    return;
                };
                let children = opening.children();
                let (_, body_context) =
                    self.add_branch(id, BranchKind::SnippetBody, &children, context);
                // The parameters belong to the body, and the name to the
                // scope the block is written in.
                if let Ok(expression) = opening.expression() {
                    self.expect(&expression, SnippetHost::SnippetSignature(id), body_context);
                }
                self.layout.snippet_name_scopes.push((id, context.scope));
                self.visit_children(&children, body_context);
            }
            AnySvelteBlock::SvelteConstBlock(tag) => {
                if let Ok(expression) = tag.expression() {
                    self.expect(&expression, SnippetHost::ConstTag, context);
                }
            }
            AnySvelteBlock::SvelteDeclarationBlock(tag) => {
                if let Ok(declaration) = tag.declaration() {
                    self.expect(&declaration, SnippetHost::DeclarationTag, context);
                }
            }
            AnySvelteBlock::SvelteRenderBlock(tag) => {
                if let Ok(expression) = tag.expression() {
                    self.expect(&expression, SnippetHost::RenderTag, context);
                }
            }
            AnySvelteBlock::SvelteHtmlBlock(tag) => {
                if let Ok(expression) = tag.expression() {
                    self.expect(&expression, SnippetHost::HtmlTag, context);
                }
            }
            AnySvelteBlock::SvelteDebugBlock(tag) => {
                for name in tag.bindings().iter().flatten() {
                    if let Ok(token) = name.ident_token() {
                        self.layout.expected.insert(
                            token.text_range(),
                            ExpectedSnippet {
                                host: SnippetHost::DebugTag,
                                scope: context.scope,
                                in_snippet_block: context.in_snippet_block,
                            },
                        );
                    }
                }
            }
            AnySvelteBlock::SvelteBogusBlock(_) => {}
        }
    }

    fn add_await_name(
        &mut self,
        kind: BranchKind,
        name: Option<&HtmlTextExpression>,
        context: Context,
    ) {
        let symbol_kind = match kind {
            BranchKind::Then => SymbolKind::AwaitValue,
            BranchKind::Catch => SymbolKind::AwaitError,
            _ => return,
        };
        if let Some(name) = name {
            self.add_text_pattern(name, symbol_kind, context);
        }
    }

    /// Declares the variables of the item of `{#each}`, which the host parser
    /// reads as a name or a destructuring pattern.
    fn add_each_name(
        &mut self,
        name: &AnySvelteEachName,
        items: &mut Vec<SymbolId>,
        context: Context,
    ) {
        match name {
            AnySvelteEachName::SvelteName(name) => {
                items.extend(self.add_svelte_name(name, context));
            }
            AnySvelteEachName::AnySvelteDestructuredName(pattern) => {
                self.add_destructured_name(pattern, items, context);
            }
            AnySvelteEachName::HtmlTextExpression(expression) => {
                items.extend(self.add_text_pattern(expression, SymbolKind::EachItem, context));
            }
        }
    }

    fn add_svelte_name(&mut self, name: &SvelteName, context: Context) -> Option<SymbolId> {
        let token = name.ident_token().ok()?;
        Some(self.add_block_symbol(
            token.token_text_trimmed(),
            token.text_trimmed_range(),
            SymbolKind::EachItem,
            context,
        ))
    }

    fn add_destructured_name(
        &mut self,
        pattern: &AnySvelteDestructuredName,
        items: &mut Vec<SymbolId>,
        context: Context,
    ) {
        let names = match pattern {
            AnySvelteDestructuredName::SvelteCurlyDestructuredName(pattern) => pattern.names(),
            AnySvelteDestructuredName::SvelteSquareDestructuredName(pattern) => pattern.names(),
        };
        for binding in names.iter().flatten() {
            self.add_assignment_binding(&binding, items, context);
        }
    }

    fn add_assignment_binding(
        &mut self,
        binding: &AnySvelteBindingAssignmentBinding,
        items: &mut Vec<SymbolId>,
        context: Context,
    ) {
        match binding {
            AnySvelteBindingAssignmentBinding::SvelteName(name) => {
                items.extend(self.add_svelte_name(name, context));
            }
            AnySvelteBindingAssignmentBinding::AnySvelteDestructuredName(pattern) => {
                self.add_destructured_name(pattern, items, context);
            }
            AnySvelteBindingAssignmentBinding::SvelteRestBinding(rest) => {
                if let Ok(name) = rest.name() {
                    items.extend(self.add_svelte_name(&name, context));
                }
            }
            AnySvelteBindingAssignmentBinding::SvelteRenameBinding(rename) => {
                if let Ok(name) = rename.name() {
                    self.add_assignment_binding(&name, items, context);
                }
            }
        }
    }
}

/// Returns the kind of a directive and the part after its prefix.
fn directive_parts(
    directive: &AnySvelteDirective,
) -> Option<(DirectiveKind, SvelteDirectiveValue)> {
    Some(match directive {
        AnySvelteDirective::SvelteBindDirective(directive) => {
            (DirectiveKind::Bind, directive.value().ok()?)
        }
        AnySvelteDirective::SvelteClassDirective(directive) => {
            (DirectiveKind::Class, directive.value().ok()?)
        }
        AnySvelteDirective::SvelteStyleDirective(directive) => {
            (DirectiveKind::Style, directive.value().ok()?)
        }
        AnySvelteDirective::SvelteUseDirective(directive) => {
            (DirectiveKind::Use, directive.value().ok()?)
        }
        AnySvelteDirective::SvelteTransitionDirective(directive) => {
            (DirectiveKind::Transition, directive.value().ok()?)
        }
        AnySvelteDirective::SvelteInDirective(directive) => {
            (DirectiveKind::In, directive.value().ok()?)
        }
        AnySvelteDirective::SvelteOutDirective(directive) => {
            (DirectiveKind::Out, directive.value().ok()?)
        }
        AnySvelteDirective::SvelteAnimateDirective(directive) => {
            (DirectiveKind::Animate, directive.value().ok()?)
        }
    })
}

/// Returns the token of the first name of a directive's name, and the
/// members read after it: `a` and `[b, c]` for `a.b.c`.
fn property_name(
    property: &AnySvelteBindingProperty,
) -> Option<(biome_html_syntax::HtmlSyntaxToken, Box<[TokenText]>)> {
    match property {
        AnySvelteBindingProperty::SvelteName(name) => {
            Some((name.ident_token().ok()?, Box::default()))
        }
        AnySvelteBindingProperty::SvelteLiteral(literal) => {
            let token = literal.value_token().ok()?;
            is_identifier(token.text_trimmed()).then(|| (token, Box::default()))
        }
        AnySvelteBindingProperty::SvelteMemberProperty(member) => {
            let mut path = Vec::new();
            let mut current = AnySvelteMemberObject::SvelteMemberProperty(member.clone());
            let token = loop {
                current = match current {
                    AnySvelteMemberObject::SvelteMemberProperty(member) => {
                        path.push(
                            member
                                .member()
                                .ok()?
                                .ident_token()
                                .ok()?
                                .token_text_trimmed(),
                        );
                        member.object().ok()?
                    }
                    AnySvelteMemberObject::SvelteName(name) => break name.ident_token().ok()?,
                };
            };
            path.reverse();
            Some((token, path.into_boxed_slice()))
        }
    }
}

/// Whether `text` can be the name of a JavaScript variable.
pub(crate) fn is_identifier(text: &str) -> bool {
    let mut chars = text.chars();
    chars
        .next()
        .is_some_and(|first| first == '_' || first == '$' || first.is_alphabetic())
        && chars.all(|char| char == '_' || char == '$' || char.is_alphanumeric())
}
