use crate::services::semantic::{SemanticModelBuilderVisitor, SemanticServices};
use crate::{
    JsRuleAction,
    react::{ReactLibrary, is_global_react_import},
};
use biome_rule_options::no_unused_imports::NoUnusedImportsOptions;

use biome_analyze::{
    AddVisitor, FixKind, FromServices, Phase, Phases, QueryKey, Queryable, Rule, RuleDiagnostic,
    RuleKey, RuleMetadata, RuleSource, ServiceBag, ServicesDiagnostic, SyntaxVisitor, Visitor,
    VisitorContext, VisitorFinishContext, context::RuleContext, declare_lint_rule,
    options::JsxRuntime,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_factory::make::{js_identifier_binding, js_module, js_module_item_list};
use biome_js_semantic::{ReferencesExtensions, SemanticModel};
use biome_js_syntax::{
    AnyJsBinding, AnyJsClassMember, AnyJsCombinedSpecifier, AnyJsDeclaration, AnyJsImportClause,
    AnyJsNamedImportSpecifier, AnyTsTypeMember, JsExport, JsLanguage, JsNamedImportSpecifiers,
    JsStaticMemberAssignment, JsSyntaxNode, T, TsEnumMember,
};
use biome_jsdoc_comment::JsdocComment;
use biome_rowan::{
    AstNode, AstSeparatedElement, AstSeparatedList, BatchMutationExt, Language, NodeOrToken,
    SyntaxNode, TextRange, TriviaPieceKind, WalkEvent, declare_node_union,
};
use regex::Regex;
use rustc_hash::FxHashSet;
use std::ops::{Deref, DerefMut};
use std::sync::LazyLock;

declare_lint_rule! {
    /// Disallow unused imports.
    ///
    /// Unused imports might be the result of an incomplete refactoring.
    /// The code fix can remove comments associated with an `import`.
    /// See the last invalid example.
    ///
    /// Note that the leading trivia, e.g., comments or newlines preceding
    /// the unused imports will also be removed. So that comment directives
    /// like `@ts-expect-error` won't be transferred to a wrong place.
    ///
    /// ## Options
    ///
    /// This rule respects the [`jsxRuntime`](https://biomejs.dev/reference/configuration/#javascriptjsxruntime)
    /// setting and will make an exception for React globals if it is set to
    /// `"reactClassic"`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import A from 'mod';
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import * as A from 'mod';
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// import { type A, B } from 'mod';
    ///
    /// export { B }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// // Header comment
    /// import /*inner comment */ A from 'mod'; // Associated comment
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// // Another header comment
    /// import {
    ///     // A's header comment
    ///     type A, // A's comment
    ///     // B's header comment
    ///     B,
    /// } from 'mod';
    ///
    /// export { B }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// import { A, type B } from 'mod';
    ///
    /// function f(arg: B): A {
    ///     return new A(arg);
    /// }
    /// ```
    ///
    /// One notable exception is when the import is intended to be used for type augmentation.
    ///
    /// ```ts
    /// import type {} from '@mui/lab/themeAugmentation';
    /// ```
    pub NoUnusedImports {
        version: "1.3.0",
        name: "noUnusedImports",
        language: "js",
        sources: &[RuleSource::EslintUnusedImports("no-unused-imports").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

/// Model for tracking any types referenced in JsDoc or TSDoc comments (like {@link TypeName} or @param {TypeName})
#[derive(Clone, Default)]
struct JsDocTypeModel(FxHashSet<String>);

impl Deref for JsDocTypeModel {
    type Target = FxHashSet<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JsDocTypeModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default)]
struct JsDocTypeCollectorVisitor {
    jsdoc_types: JsDocTypeModel,
}

declare_node_union! {
    pub AnyJsWithTypeReferencingJsDoc = AnyJsDeclaration | AnyJsClassMember | AnyTsTypeMember | TsEnumMember | JsExport | JsStaticMemberAssignment
}

impl Visitor for JsDocTypeCollectorVisitor {
    type Language = JsLanguage;
    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        _ctx: VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(node) => {
                if AnyJsWithTypeReferencingJsDoc::can_cast(node.kind()) {
                    load_jsdoc_types_from_node(&mut self.jsdoc_types, node);
                }
            }
            WalkEvent::Leave(_) => {}
        }
    }

    fn finish(self: Box<Self>, ctx: VisitorFinishContext<JsLanguage>) {
        ctx.services.insert_service(self.jsdoc_types);
    }
}

fn load_jsdoc_types_from_node(model: &mut JsDocTypeModel, node: &SyntaxNode<JsLanguage>) {
    JsdocComment::for_each(node, |comment| {
        load_jsdoc_types_from_jsdoc_comment(model, comment)
    });
}

static JSDOC_INLINE_TAG_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\{@(link|see)\s*([^}| #\.]+)(?:[^}]+)?\}").unwrap());

static JSDOC_TYPE_TAG_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"@(param|returns|type|typedef)\s*\{([^}]+)\}").unwrap());

static JSDOC_TYPE_PART_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\w+)").unwrap());

fn load_jsdoc_types_from_jsdoc_comment(model: &mut JsDocTypeModel, comment: &str) {
    // regex matching for typical JSDoc patterns containing types `{@link ClassName}` and `@sometag {TypeSyntax}`
    // This should likely become a proper parser.
    // This will definitely fail in object types like { a: { b: number } }

    for (_, [_tag_name, type_name]) in JSDOC_INLINE_TAG_REGEX
        .captures_iter(comment)
        .map(|c| c.extract())
    {
        model.insert(type_name.to_string());
    }

    for (_, [_tag_name, type_name]) in JSDOC_TYPE_TAG_REGEX
        .captures_iter(comment)
        .map(|c| c.extract())
    {
        // can the TS type parser be used here? for now we simply match with words
        for (_, [type_part]) in JSDOC_TYPE_PART_REGEX
            .captures_iter(type_name)
            .map(|c| c.extract())
        {
            model.insert(type_part.to_string());
        }
    }
}

pub struct JsDocTypeServices {
    jsdoc_types: JsDocTypeModel,
    semantic_services: SemanticServices,
}
impl JsDocTypeServices {
    fn jsdoc_types(&self) -> &JsDocTypeModel {
        &self.jsdoc_types
    }

    fn semantic_model(&self) -> &SemanticModel {
        self.semantic_services.model()
    }
}

impl FromServices for JsDocTypeServices {
    fn from_services(
        rule_key: &RuleKey,
        rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let jsdoc_types: &JsDocTypeModel = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["JsDocTypeModel"]))?;
        Ok(Self {
            jsdoc_types: jsdoc_types.clone(),
            semantic_services: SemanticServices::from_services(rule_key, rule_metadata, services)?,
        })
    }
}

impl Phase for JsDocTypeServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

#[derive(Clone)]
pub struct NoUnusedImportsQuery(pub AnyJsImportClause);

impl Queryable for NoUnusedImportsQuery {
    type Input = JsSyntaxNode;
    type Output = AnyJsImportClause;

    type Language = JsLanguage;
    type Services = JsDocTypeServices;

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        root: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor::new(root));
        analyzer.add_visitor(Phases::Syntax, JsDocTypeCollectorVisitor::default);
        analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(AnyJsImportClause::KIND_SET)
    }

    fn unwrap_match(_services: &ServiceBag, node: &Self::Input) -> Self::Output {
        AnyJsImportClause::unwrap_cast(node.clone())
    }
}

impl Rule for NoUnusedImports {
    type Query = NoUnusedImportsQuery;
    type State = Unused;
    type Signals = Option<Self::State>;
    type Options = NoUnusedImportsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyJsImportClause::JsImportBareClause(_) => {
                // ignore bare imports (aka side-effect imports) such as `import "mod"`.
                None
            }
            AnyJsImportClause::JsImportCombinedClause(clause) => {
                let default_local_name = clause.default_specifier().ok()?.local_name().ok()?;
                let is_default_import_unused = is_unused(ctx, &default_local_name);
                let (is_combined_unused, named_import_range) = match clause.specifier().ok()? {
                    AnyJsCombinedSpecifier::JsNamedImportSpecifiers(specifiers) => {
                        match unused_named_specifiers(ctx, &specifiers) {
                            Some(Unused::AllImports(range) | Unused::EmptyStatement(range)) => {
                                (true, range)
                            }
                            Some(Unused::NamedImports(unused_named_specifers)) => {
                                return Some(if is_default_import_unused {
                                    Unused::DefaultNamedImport(
                                        default_local_name.range(),
                                        unused_named_specifers,
                                    )
                                } else {
                                    Unused::NamedImports(unused_named_specifers)
                                });
                            }
                            _ => (false, specifiers.range()),
                        }
                    }
                    AnyJsCombinedSpecifier::JsNamespaceImportSpecifier(specifier) => {
                        let local_name = specifier.local_name().ok()?;
                        (is_unused(ctx, &local_name), local_name.range())
                    }
                };
                match (is_default_import_unused, is_combined_unused) {
                    (true, true) => Some(Unused::AllImports(TextRange::new(
                        default_local_name.range().start(),
                        named_import_range.end(),
                    ))),
                    (true, false) => Some(Unused::DefaultImport(default_local_name.range())),
                    (false, true) => Some(Unused::CombinedImport(named_import_range)),
                    (false, false) => None,
                }
            }
            AnyJsImportClause::JsImportDefaultClause(clause) => {
                let local_name = clause.default_specifier().ok()?.local_name().ok()?;
                is_unused(ctx, &local_name).then_some(Unused::AllImports(local_name.range()))
            }
            AnyJsImportClause::JsImportNamedClause(clause) => {
                // exception: allow type augmentation imports
                // eg. `import type {} from ...`
                // https://github.com/biomejs/biome/issues/6669
                if clause.type_token().is_some()
                    && clause.named_specifiers().ok()?.specifiers().is_empty()
                {
                    return None;
                }

                unused_named_specifiers(ctx, &clause.named_specifiers().ok()?)
            }
            AnyJsImportClause::JsImportNamespaceClause(clause) => {
                let local_name = clause.namespace_specifier().ok()?.local_name().ok()?;
                is_unused(ctx, &local_name).then_some(Unused::AllImports(local_name.range()))
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state {
            Unused::EmptyStatement(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This "<Emphasis>"import"</Emphasis>" is empty."
                },
            ),
            Unused::AllImports(range)
            | Unused::DefaultImport(range)
            | Unused::CombinedImport(range) => {
                let msg = match ctx.query() {
                    AnyJsImportClause::JsImportDefaultClause(_)
                    | AnyJsImportClause::JsImportNamedClause(_)
                    | AnyJsImportClause::JsImportNamespaceClause(_) => {
                        markup! {
                            "This "<Emphasis>"import"</Emphasis>" is unused."
                        }
                    }
                    _ => {
                        markup! {
                            "These "<Emphasis>"imports"</Emphasis>" are unused."
                        }
                    }
                };
                RuleDiagnostic::new(rule_category!(), range, msg)
            }
            Unused::DefaultNamedImport(default_import_range, unused_named_imports) => {
                let range = TextRange::new(
                    default_import_range.start(),
                    unused_named_imports.last()?.range().end(),
                );
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "Several of these "<Emphasis>"imports"</Emphasis>" are unused."
                    },
                )
            }
            Unused::NamedImports(unused_named_imports) => {
                let range = TextRange::new(
                    unused_named_imports.first()?.range().start(),
                    unused_named_imports.last()?.range().end(),
                );
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "Several of these "<Emphasis>"imports"</Emphasis>" are unused."
                    },
                )
            }
        };
        Some(diagnostic.note(markup! {
            "Unused imports might be the result of an incomplete refactoring."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let root = ctx.root();
        let mut mutation = root.clone().begin();

        match state {
            Unused::EmptyStatement(_) | Unused::AllImports(_) => {
                let parent = node.syntax().parent()?;
                let leading_trivia = parent.first_leading_trivia()?;
                let mut leading_trivia_pieces = leading_trivia.pieces().collect::<Vec<_>>();
                let blank_line_pos = leading_trivia_pieces
                    .windows(2)
                    .rposition(|window| window[0].is_newline() && window[1].is_newline());
                if let Some(blank_line_pos) = blank_line_pos {
                    // keep all leading trivia until the last blank line.
                    leading_trivia_pieces.truncate(blank_line_pos + 1);
                    if let Some(prev_sibling) = parent.prev_sibling() {
                        let new_prev_sibling = prev_sibling
                            .clone()
                            .append_trivia_pieces(leading_trivia_pieces)?;
                        mutation.replace_element_discard_trivia(
                            prev_sibling.into(),
                            new_prev_sibling.into(),
                        );
                    } else if let Some(next_sibling) = parent.next_sibling() {
                        let new_next_sibling = next_sibling
                            .clone()
                            .prepend_trivia_pieces(leading_trivia_pieces)?;
                        mutation.replace_element_discard_trivia(
                            next_sibling.into(),
                            new_next_sibling.into(),
                        );
                    } else if let Some(root) = root.as_js_module() {
                        mutation.remove_element(parent.clone().into());
                        let mut new_root = js_module(
                            root.directives(),
                            js_module_item_list(vec![]),
                            root.eof_token().ok()?,
                        );
                        if let Some(bom_token) = root.bom_token() {
                            new_root = new_root.with_bom_token(bom_token);
                        }
                        if let Some(interpreter_token) = root.interpreter_token() {
                            new_root = new_root.with_interpreter_token(interpreter_token);
                        }
                        let new_root = new_root
                            .build()
                            .prepend_trivia_pieces(leading_trivia_pieces)?;
                        mutation.replace_node_discard_trivia(root.clone(), new_root);
                    }
                }
                mutation.remove_element(parent.into());
            }
            Unused::DefaultImport(_) => {
                let prev_clause = node.as_js_import_combined_clause()?.clone();
                let new_clause: AnyJsImportClause = match prev_clause.specifier().ok()? {
                    AnyJsCombinedSpecifier::JsNamedImportSpecifiers(named_specifiers) => {
                        let new_clause = make::js_import_named_clause(
                            named_specifiers,
                            prev_clause.from_token().ok()?,
                            prev_clause.source().ok()?,
                        );
                        if let Some(attributes) = prev_clause.assertion() {
                            new_clause.with_assertion(attributes)
                        } else {
                            new_clause
                        }
                        .build()
                        .into()
                    }
                    AnyJsCombinedSpecifier::JsNamespaceImportSpecifier(specifier) => {
                        let new_clause = make::js_import_namespace_clause(
                            specifier,
                            prev_clause.from_token().ok()?,
                            prev_clause.source().ok()?,
                        );
                        if let Some(attributes) = prev_clause.assertion() {
                            new_clause.with_assertion(attributes)
                        } else {
                            new_clause
                        }
                        .build()
                        .into()
                    }
                };
                mutation.replace_node(prev_clause.into(), new_clause);
            }
            Unused::CombinedImport(_) => {
                let prev_clause = node.as_js_import_combined_clause()?.clone();
                let default_specifier = prev_clause.default_specifier().ok()?;
                let local_name = default_specifier.local_name().ok()?;
                let mut local_name = local_name.as_js_identifier_binding()?.name_token().ok()?;
                local_name =
                    local_name.with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]);

                let new_clause = make::js_import_default_clause(
                    prev_clause.default_specifier().ok()?.with_local_name(
                        AnyJsBinding::JsIdentifierBinding(js_identifier_binding(local_name)),
                    ),
                    prev_clause.from_token().ok()?,
                    prev_clause.source().ok()?,
                );
                let new_clause = if let Some(attributes) = prev_clause.assertion() {
                    new_clause.with_assertion(attributes)
                } else {
                    new_clause
                }
                .build();
                mutation.replace_node::<AnyJsImportClause>(prev_clause.into(), new_clause.into());
            }
            Unused::DefaultNamedImport(_, unused_named_specifiers) => {
                let prev_clause = node.as_js_import_combined_clause()?.clone();
                let Ok(AnyJsCombinedSpecifier::JsNamedImportSpecifiers(named_specifiers)) =
                    prev_clause.specifier()
                else {
                    return None;
                };
                let (specifiers, separators): (Vec<_>, Vec<_>) = named_specifiers
                    .specifiers()
                    .elements()
                    .filter_map(
                        |AstSeparatedElement {
                             node,
                             trailing_separator,
                         }| Some((node.ok()?, trailing_separator.ok()?)),
                    )
                    .filter(|(node, _)| !unused_named_specifiers.contains(node))
                    .unzip();
                let used_specifiers = make::js_named_import_specifier_list(
                    specifiers,
                    separators.into_iter().flatten().collect::<Vec<_>>(),
                );
                let used_named_specifiers = make::js_named_import_specifiers(
                    named_specifiers.l_curly_token().ok()?,
                    used_specifiers,
                    named_specifiers.r_curly_token().ok()?,
                );
                let new_clause = make::js_import_named_clause(
                    used_named_specifiers,
                    prev_clause.from_token().ok()?,
                    prev_clause.source().ok()?,
                );
                let new_clause = if let Some(attributes) = prev_clause.assertion() {
                    new_clause.with_assertion(attributes)
                } else {
                    new_clause
                }
                .build();
                mutation.replace_node::<AnyJsImportClause>(prev_clause.into(), new_clause.into());
            }
            Unused::NamedImports(unused_named_specifiers) => {
                for unused_specifier in unused_named_specifiers {
                    if let Some(NodeOrToken::Token(next_token)) =
                        unused_specifier.syntax().next_sibling_or_token()
                        && next_token.kind() == T![,]
                    {
                        mutation.remove_token(next_token);
                    }
                    mutation.remove_node(unused_specifier.clone());
                }
            }
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the unused imports." }.to_owned(),
            mutation,
        ))
    }
}

#[derive(Debug)]
pub enum Unused {
    /// Empty import such as `import {} from "mod"`
    EmptyStatement(TextRange),
    //// All imports of the statements are unused
    AllImports(TextRange),
    /// The default import of the combined clause is unused. e.g.:
    /// - `import UnusedDefault, * as Ns from "mod"`
    /// - `import UnusedDefault, { A } from "mod"`
    DefaultImport(TextRange),
    /// The imports of the second specifier of the combined clause are unused. e.g.:
    /// - `import Default, * as UnusedNs from "mod"`
    /// - `import Default, { UnusedA }from "mod"`
    CombinedImport(TextRange),
    /// The default and some named imports in a combined clause are unused. e.g.:
    /// - `import UnusedDefault, { UnusedA, B, UnusedC } from "mod"`
    DefaultNamedImport(TextRange, Box<[AnyJsNamedImportSpecifier]>),
    /// Some named specifoers are unused. e.g.:
    /// - import { UnusedA, B, UnusedC } from "mod"`
    /// - `import Default, { UnusedA, B, UnusedC }from "mod"`
    NamedImports(Box<[AnyJsNamedImportSpecifier]>),
}

fn unused_named_specifiers(
    ctx: &RuleContext<NoUnusedImports>,
    named_specifiers: &JsNamedImportSpecifiers,
) -> Option<Unused> {
    let specifiers = named_specifiers.specifiers();
    let len = specifiers.len();
    if len == 0 {
        // `import {} from`
        Some(Unused::EmptyStatement(specifiers.range()))
    } else {
        let mut unused_imports = Vec::new();
        for specifier in specifiers.into_iter().flatten() {
            let Some(local_name) = specifier.local_name() else {
                continue;
            };
            if is_unused(ctx, &local_name) {
                unused_imports.push(specifier);
            }
        }
        if unused_imports.is_empty() {
            // All imports are used
            None
        } else if unused_imports.len() == len {
            // All imports are unused
            Some(Unused::AllImports(named_specifiers.range()))
        } else {
            Some(Unused::NamedImports(unused_imports.into_boxed_slice()))
        }
    }
}

fn is_unused(ctx: &RuleContext<NoUnusedImports>, local_name: &AnyJsBinding) -> bool {
    let AnyJsBinding::JsIdentifierBinding(binding) = &local_name else {
        return false;
    };
    if ctx.jsx_runtime() == JsxRuntime::ReactClassic
        && is_global_react_import(binding, ReactLibrary::React)
    {
        return false;
    }

    let jsdoc_types = ctx.jsdoc_types();
    if binding
        .name_token()
        .is_ok_and(|name| jsdoc_types.contains(name.text_trimmed()))
    {
        return false;
    }

    let model = ctx.semantic_model();
    binding.all_references(model).next().is_none()
}
