use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_factory::make;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyHtmlAttributeInitializer, HtmlAttribute, HtmlAttributeInitializerClause,
    HtmlRoot, HtmlSyntaxKind, HtmlSyntaxToken, T, element_ext::AnyHtmlTagElement,
};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TextRange, TriviaPiece};
use biome_rule_options::use_consistent_block_lang::{
    BlockLangOptions, UseConsistentBlockLangOptions,
};

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Enforce which languages the top-level blocks of Vue and Svelte components use.
    ///
    /// A top-level block is an element at the root of a component file, such as
    /// `<script>`, `<style>`, or `<template>`. Its `lang` attribute tells the build
    /// tool which language the block is written in, such as `lang="ts"` for
    /// TypeScript or `lang="scss"` for SCSS. Without `lang`, a `<script>` block
    /// is JavaScript and a `<style>` block is CSS.
    ///
    /// Using the same languages in every component keeps a codebase consistent.
    /// For example, this rule can make sure that every component is written in
    /// TypeScript, or that no component uses a CSS preprocessor that the project
    /// doesn't support.
    ///
    /// This rule checks the `lang` attribute of the top-level blocks listed in the
    /// [`blocks`](#blocks) option, and can require that a block is present.
    /// Elements nested inside other elements aren't checked.
    /// Without `blocks` configured, this rule doesn't check anything.
    ///
    /// ## Options
    ///
    /// ### `blocks`
    ///
    /// The top-level blocks to check. Each key is the tag name of a block, without
    /// angle brackets, such as `script`, `style`, or a Vue custom block like `i18n`.
    /// Blocks that aren't listed aren't checked.
    ///
    /// Each block accepts the following properties:
    ///
    /// - `lang`: the values allowed for the `lang` attribute. When set, it must
    ///   contain at least one value. Values must match exactly, so `"ts"` doesn't
    ///   allow `lang="TS"`. When omitted, the `lang` attribute of the block isn't
    ///   checked.
    /// - `allowNoLang`: when `true`, the block may also omit the `lang` attribute.
    ///   Only applies when `lang` is set. Default: `false`.
    /// - `required`: when `true`, every component must contain at least one block
    ///   with this tag name. Works with or without `lang`. Default: `false`.
    ///
    /// The rule doesn't check a `lang` attribute whose value is computed, such as
    /// `lang={language}` in Svelte.
    ///
    /// The following configuration requires a TypeScript `<script>` block in every
    /// component, and allows `<style>` blocks to use SCSS or plain CSS:
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "blocks": {
    ///       "script": { "lang": ["ts"], "required": true },
    ///       "style": { "lang": ["scss"], "allowNoLang": true }
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// The `<script>` block doesn't specify `lang="ts"`:
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <script>
    /// </script>
    /// ```
    ///
    /// The `<style>` block uses Less, which isn't allowed:
    ///
    /// ```svelte,expect_diagnostic,use_options
    /// <script lang="ts">
    /// </script>
    ///
    /// <style lang="less">
    /// </style>
    /// ```
    ///
    /// The component has no `<script>` block:
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///   <div></div>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue,use_options
    /// <script lang="ts">
    /// </script>
    ///
    /// <style>
    /// </style>
    ///
    /// <style lang="scss">
    /// </style>
    /// ```
    ///
    pub UseConsistentBlockLang {
        version: "next",
        name: "useConsistentBlockLang",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Vue, RuleDomain::Svelte],
        sources: &[
            RuleSource::EslintVueJs("block-lang").inspired(),
            RuleSource::EslintSvelte("block-lang").inspired(),
        ],
        fix_kind: FixKind::Unsafe,
    }
}

pub struct BlockLangState {
    /// The index of the block in the configured `blocks`.
    block_index: usize,
    kind: BlockLangViolation,
}

impl Rule for UseConsistentBlockLang {
    type Query = Ast<HtmlRoot>;
    type State = BlockLangState;
    type Signals = Box<[Self::State]>;
    type Options = UseConsistentBlockLangOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<HtmlFileSource>();
        if !source_type.is_vue() && !source_type.is_svelte() {
            return Box::default();
        }

        let Some(blocks) = ctx.options().blocks.as_ref() else {
            return Box::default();
        };
        let mut found = vec![false; blocks.len()];
        let mut signals = Vec::new();

        for element in ctx.query().html() {
            let Some(element) = element.as_any_html_tag_element() else {
                continue;
            };
            let Some(tag_name) = element.tag_name() else {
                continue;
            };
            let Some((block_index, _, block)) = blocks.get_full(tag_name.text()) else {
                continue;
            };
            found[block_index] = true;

            if let Some(kind) = check_block_lang(&element, block) {
                signals.push(BlockLangState { block_index, kind });
            }
        }

        for (block_index, (_, block)) in blocks.iter().enumerate() {
            if block.required() && !found[block_index] {
                signals.push(BlockLangState {
                    block_index,
                    kind: BlockLangViolation::MissingBlock,
                });
            }
        }

        signals.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (tag_name, block) = ctx
            .options()
            .blocks
            .as_ref()?
            .get_index(state.block_index)?;
        let tag_name = tag_name.as_ref();

        let diagnostic = match &state.kind {
            BlockLangViolation::MissingLang { element, .. } => RuleDiagnostic::new(
                rule_category!(),
                element.range(),
                markup! {
                    "This "<Emphasis>"<"{tag_name}">"</Emphasis>" block is missing the "<Emphasis>"lang"</Emphasis>" attribute."
                },
            ),
            BlockLangViolation::DisallowedLang { attribute } => {
                let value = attribute.as_static_value()?;
                RuleDiagnostic::new(
                    rule_category!(),
                    attribute.range(),
                    markup! {
                        "The language "<Emphasis>{value.text()}</Emphasis>" isn't allowed in "<Emphasis>"<"{tag_name}">"</Emphasis>" blocks."
                    },
                )
            }
            BlockLangViolation::MissingBlock => RuleDiagnostic::new(
                rule_category!(),
                TextRange::empty(ctx.query().syntax().text_range_with_trivia().start()),
                markup! {
                    "This component is missing a "<Emphasis>"<"{tag_name}">"</Emphasis>" block."
                },
            ),
        };

        let diagnostic = if matches!(state.kind, BlockLangViolation::MissingBlock) {
            diagnostic.note(markup! {
                "This project requires every component to have a "<Emphasis>"<"{tag_name}">"</Emphasis>" block so that every component is written the same way."
            })
        } else {
            diagnostic.note(markup! {
                "This project restricts the languages of "<Emphasis>"<"{tag_name}">"</Emphasis>" blocks so that every component is written the same way."
            })
        };

        let Some(langs) = block.lang.as_deref() else {
            return Some(diagnostic);
        };
        let allowed = AllowedLangs {
            langs,
            allow_no_lang: block.allow_no_lang(),
        };
        Some(diagnostic.note(markup! {
            "The "<Emphasis>"lang"</Emphasis>" attribute of "<Emphasis>"<"{tag_name}">"</Emphasis>" blocks must be "{allowed}"."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let (_, block) = ctx
            .options()
            .blocks
            .as_ref()?
            .get_index(state.block_index)?;
        let mut mutation = ctx.root().begin();

        let message = match &state.kind {
            BlockLangViolation::MissingLang { element, attribute } => {
                // Only fix when there is a single choice.
                let Some([lang]) = block.lang.as_deref() else {
                    return None;
                };
                match attribute {
                    Some(attribute) => {
                        mutation.replace_node(attribute.clone(), with_lang_value(attribute, lang)?);
                    }
                    None => {
                        let old_attributes = element.attributes();
                        let mut items: Vec<AnyHtmlAttribute> = old_attributes.iter().collect();
                        items.push(AnyHtmlAttribute::HtmlAttribute(make_lang_attribute(lang)));
                        // `replace_node` would swap the leading space of the new
                        // attribute for the old list's leading trivia, which is
                        // empty when the element has no attributes.
                        mutation.replace_node_discard_trivia(
                            old_attributes,
                            make::html_attribute_list(items),
                        );
                    }
                }
                markup! { "Set the "<Emphasis>"lang"</Emphasis>" attribute to "<Emphasis>{lang.as_ref()}</Emphasis>"." }
                    .to_owned()
            }
            BlockLangViolation::DisallowedLang { attribute } => match block.lang.as_deref() {
                // Only fix when there is a single choice.
                Some([lang]) if !block.allow_no_lang() => {
                    mutation.replace_node(attribute.clone(), with_lang_value(attribute, lang)?);
                    markup! { "Set the "<Emphasis>"lang"</Emphasis>" attribute to "<Emphasis>{lang.as_ref()}</Emphasis>"." }
                        .to_owned()
                }
                _ => return None,
            },
            BlockLangViolation::MissingBlock => return None,
        };

        Some(biome_analyze::RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message,
            mutation,
        ))
    }
}

pub enum BlockLangViolation {
    /// The block has no `lang` attribute, or the attribute has no value.
    MissingLang {
        element: AnyHtmlTagElement,
        attribute: Option<HtmlAttribute>,
    },
    /// The value of the `lang` attribute isn't allowed.
    DisallowedLang { attribute: HtmlAttribute },
    /// The file doesn't contain a required block.
    MissingBlock,
}

/// Checks the `lang` attribute of a top-level block against its options.
///
/// Returns `None` when the attribute is allowed, when the options don't set
/// `lang`, or when the attribute's value is computed and can't be checked.
fn check_block_lang(
    element: &AnyHtmlTagElement,
    block: &BlockLangOptions,
) -> Option<BlockLangViolation> {
    // Without `lang`, the block is only checked for presence.
    let langs = block.lang.as_deref()?;
    let attribute = match element.attributes().find_attribute_by_name("lang") {
        Some(AnyHtmlAttribute::HtmlAttribute(attribute)) => Some(attribute),
        // Dynamic values, such as the Svelte shorthand `{lang}`, can't be checked.
        Some(_) => return None,
        None => None,
    };

    // Attributes without a value, such as `<script lang>`, are treated as missing.
    let attribute = match attribute {
        Some(attribute) if attribute.initializer().is_some() => attribute,
        attribute => {
            return (!block.allow_no_lang()).then(|| BlockLangViolation::MissingLang {
                element: element.clone(),
                attribute,
            });
        }
    };

    // Dynamic values, such as `lang={lang}` in Svelte, can't be checked.
    let value = attribute.as_static_value()?;
    let value = value.text();
    if langs.iter().any(|lang| lang.as_ref() == value) {
        return None;
    }

    Some(BlockLangViolation::DisallowedLang { attribute })
}

/// Creates a `lang="..."` attribute preceded by a space.
fn make_lang_attribute(lang: &str) -> HtmlAttribute {
    let name = HtmlSyntaxToken::new_detached(
        HtmlSyntaxKind::HTML_LITERAL,
        " lang",
        [TriviaPiece::whitespace(1)],
        [],
    );
    make::html_attribute(make::html_attribute_name(name))
        .with_initializer(make_lang_initializer(lang))
        .build()
}

/// Returns a copy of `attribute` with its value replaced by `lang`, written as
/// `lang="..."`.
///
/// The whitespace after the attribute name is dropped so that `lang = 'js'`
/// becomes `lang="ts"`. The whitespace around the whole attribute is restored
/// when the caller swaps it in with `BatchMutation::replace_node`.
fn with_lang_value(attribute: &HtmlAttribute, lang: &str) -> Option<HtmlAttribute> {
    let name_token = attribute
        .name()
        .ok()?
        .value_token()
        .ok()?
        .with_trailing_trivia_pieces([]);
    Some(
        make::html_attribute(make::html_attribute_name(name_token))
            .with_initializer(make_lang_initializer(lang))
            .build(),
    )
}

fn make_lang_initializer(lang: &str) -> HtmlAttributeInitializerClause {
    make::html_attribute_initializer_clause(
        make::token(T![=]),
        AnyHtmlAttributeInitializer::HtmlString(make::html_string(make::html_string_literal(
            lang,
        ))),
    )
}

/// Formats the values allowed for the `lang` attribute of a block, such as
/// "omitted or one of ts, tsx".
struct AllowedLangs<'a> {
    langs: &'a [Box<str>],
    allow_no_lang: bool,
}

impl biome_console::fmt::Display for AllowedLangs<'_> {
    fn fmt(&self, f: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        if self.allow_no_lang {
            f.write_str("omitted or ")?;
        }
        if self.langs.len() > 1 {
            f.write_str("one of ")?;
        }
        for (index, lang) in self.langs.iter().enumerate() {
            if index > 0 {
                f.write_str(", ")?;
            }
            f.write_markup(markup! { <Emphasis>{lang.as_ref()}</Emphasis> })?;
        }
        Ok(())
    }
}
