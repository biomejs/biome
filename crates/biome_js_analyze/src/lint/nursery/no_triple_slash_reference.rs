use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    JsImport, JsModule, JsModuleSource, JsScript, JsSyntaxKind,
    JsSyntaxToken, TextSize, TsDeclarationModule, TsExternalModuleReference,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{
    AstNode, TextRange, TriviaPieceKind, WalkEvent, declare_node_union,
};
use biome_rule_options::no_triple_slash_reference::{
    NoTripleSlashReferenceOptions, TripleSlashReferenceUsage, TripleSlashTypesReferenceUsage,
};

declare_lint_rule! {
    /// Disallow triple-slash reference comments that can be replaced by imports.
    ///
    /// A triple-slash reference is a special TypeScript comment at the top of a file, such as
    /// `/// <reference path="./globals.d.ts" />`. It tells TypeScript to load another file,
    /// package, or built-in library before checking the current file. This was the main way to
    /// share code between files before JavaScript had `import` statements.
    ///
    /// An `import` statement shows which file or package the code depends on, and every modern
    /// tool understands it. A triple-slash reference adds types to the whole file without making
    /// the dependency visible in the code.
    ///
    /// By default, this rule reports:
    /// - every `/// <reference path="..." />` comment;
    /// - every `/// <reference types="..." />` comment for a package that the file also imports,
    ///   because the import already loads the package's types.
    ///
    /// Only comments at the very top of the file are checked, before any code. TypeScript treats
    /// `///` comments that come later as ordinary comments.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// /// <reference path="./globals.d.ts" />
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// /// <reference types="lodash" />
    /// import { debounce } from "lodash";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// /// <reference types="vite/client" />
    /// /// <reference lib="dom" />
    /// import "./globals";
    /// ```
    ///
    /// ## Options
    ///
    /// Each option controls one kind of reference. `"always"` allows the reference and
    /// `"never"` reports it.
    ///
    /// ### `lib`
    ///
    /// Type: `"always" | "never"`
    ///
    /// Default: `"always"`
    ///
    /// Controls `/// <reference lib="..." />` comments, which load a built-in TypeScript library
    /// such as `dom` or `es2020`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "lib": "never"
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```ts,expect_diagnostic,use_options
    /// /// <reference lib="dom" />
    /// ```
    ///
    /// ### `path`
    ///
    /// Type: `"always" | "never"`
    ///
    /// Default: `"never"`
    ///
    /// Controls `/// <reference path="..." />` comments, which load another file.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "path": "always"
    ///     }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```ts,use_options
    /// /// <reference path="./globals.d.ts" />
    /// ```
    ///
    /// ### `types`
    ///
    /// Type: `"always" | "never" | "preferImport"`
    ///
    /// Default: `"preferImport"`
    ///
    /// Controls `/// <reference types="..." />` comments, which load the types of a package.
    /// With `"preferImport"`, the comment is reported only when the file also imports the same
    /// package. With `"never"`, every such comment is reported.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "types": "never"
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```ts,expect_diagnostic,use_options
    /// /// <reference types="vite/client" />
    /// ```
    ///
    pub NoTripleSlashReference {
        version: "2.6.8",
        name: "noTripleSlashReference",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("triple-slash-reference").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

declare_node_union! {
    pub AnyJsProgram = JsModule | JsScript | TsDeclarationModule
}

impl Rule for NoTripleSlashReference {
    type Query = Ast<AnyJsProgram>;
    type State = TripleSlashReference;
    type Signals = Box<[Self::State]>;
    type Options = NoTripleSlashReferenceOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let options = ctx.options();
        let lib = options.lib();
        let path = options.path();
        let types = options.types();
        if lib == TripleSlashReferenceUsage::Always
            && path == TripleSlashReferenceUsage::Always
            && types == TripleSlashTypesReferenceUsage::Always
        {
            return Box::default();
        }

        let mut signals = Vec::new();
        // References that are reported only if the file imports the same package.
        let mut types_references = Vec::new();
        let mut token = ctx.query().syntax().first_token();
        while let Some(current) = token {
            let is_file_prefix = FILE_PREFIX_TOKENS.contains(current.kind());
            let trailing = is_file_prefix
                .then(|| current.trailing_trivia().pieces())
                .into_iter()
                .flatten();
            for piece in current.leading_trivia().pieces().chain(trailing) {
                if piece.kind() != TriviaPieceKind::SingleLineComment {
                    continue;
                }
                let Some((kind, value)) = parse_triple_slash_reference(piece.text()) else {
                    continue;
                };
                let comment_range = piece.text_range();
                let value_range = TextRange::new(
                    comment_range.start() + TextSize::from(value.start as u32),
                    comment_range.start() + TextSize::from(value.end as u32),
                );
                let reported = match kind {
                    ReferenceKind::Lib => lib == TripleSlashReferenceUsage::Never,
                    ReferenceKind::Path => path == TripleSlashReferenceUsage::Never,
                    ReferenceKind::Types => match types {
                        TripleSlashTypesReferenceUsage::Always => false,
                        TripleSlashTypesReferenceUsage::Never => true,
                        TripleSlashTypesReferenceUsage::PreferImport => {
                            types_references.push((current.clone(), comment_range, value_range));
                            false
                        }
                    },
                };
                if reported {
                    signals.push(TripleSlashReference {
                        token: current.clone(),
                        comment_range,
                        value_range,
                        problem: ReferenceProblem::Disallowed(kind),
                    });
                }
            }
            if !is_file_prefix {
                break;
            }
            token = current.next_token();
        }

        if !types_references.is_empty() {
            let mut preorder = ctx.query().syntax().preorder_of_kinds(AnyJsImportWithSource::KIND_SET);
            while let Some(event) = preorder.next() {
                let WalkEvent::Enter(node) = event else {
                    continue;
                };
                preorder.skip_subtree();
                let Some(source) = AnyJsImportWithSource::cast(node).and_then(|import| import.source())
                else {
                    continue;
                };
                let Ok(specifier) = source.inner_string_text() else {
                    continue;
                };
                let import_range = source.range();
                types_references.retain(|(token, comment_range, value_range)| {
                    if reference_value(token, *value_range) != specifier.text() {
                        return true;
                    }
                    signals.push(TripleSlashReference {
                        token: token.clone(),
                        comment_range: *comment_range,
                        value_range: *value_range,
                        problem: ReferenceProblem::AlreadyImported(import_range),
                    });
                    false
                });
                if types_references.is_empty() {
                    break;
                }
            }
        }

        signals.sort_unstable_by_key(|signal| signal.comment_range.start());
        signals.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let value = reference_value(&state.token, state.value_range);
        Some(match state.problem {
            ReferenceProblem::Disallowed(kind) => RuleDiagnostic::new(
                rule_category!(),
                state.comment_range,
                markup! {
                    "Unexpected triple-slash reference to "<Emphasis>{value}</Emphasis>"."
                },
            )
            .note(markup! {
                "A triple-slash reference loads code for the whole file without showing which parts the file uses."
            })
            .note(match kind {
                ReferenceKind::Lib => markup! {
                    "Add the library to the "<Emphasis>"lib"</Emphasis>" option of your "<Emphasis>"tsconfig.json"</Emphasis>" file instead."
                },
                ReferenceKind::Path => markup! {
                    "Use an "<Emphasis>"import"</Emphasis>" statement to load the file instead."
                },
                ReferenceKind::Types => markup! {
                    "Import from the package instead, or add the package to the "<Emphasis>"types"</Emphasis>" option of your "<Emphasis>"tsconfig.json"</Emphasis>" file."
                },
            }),
            ReferenceProblem::AlreadyImported(import_range) => RuleDiagnostic::new(
                rule_category!(),
                state.comment_range,
                markup! {
                    "This triple-slash reference to "<Emphasis>{value}</Emphasis>" is redundant."
                },
            )
            .detail(
                import_range,
                markup! {
                    "The file already imports "<Emphasis>{value}</Emphasis>" here, which also loads its types."
                },
            )
            .note(markup! {
                "Remove the triple-slash reference."
            }),
        })
    }
}

/// A triple-slash reference comment that the rule reports.
pub struct TripleSlashReference {
    /// The token whose trivia contains the comment.
    token: JsSyntaxToken,
    comment_range: TextRange,
    /// The range of the value between the quotes, such as `node` in `types="node"`.
    value_range: TextRange,
    problem: ReferenceProblem,
}

pub enum ReferenceProblem {
    /// The options disallow this kind of reference.
    Disallowed(ReferenceKind),
    /// The file imports the same package. Holds the range of the import's source.
    AlreadyImported(TextRange),
}

#[derive(Clone, Copy)]
pub enum ReferenceKind {
    Lib,
    Path,
    Types,
}

/// Tokens that can come before the comments at the top of a file: the byte order mark and the
/// `#!` line. Comments on the same line as one of these tokens are part of its trailing trivia.
const FILE_PREFIX_TOKENS: TokenSet<JsSyntaxKind> =
    token_set![JsSyntaxKind::UNICODE_BOM, JsSyntaxKind::JS_SHEBANG];

declare_node_union! {
    /// An `import ... from "source"` statement, or the `require("source")` part of
    /// `import x = require("source")`.
    pub AnyJsImportWithSource = JsImport | TsExternalModuleReference
}

impl AnyJsImportWithSource {
    fn source(&self) -> Option<JsModuleSource> {
        match self {
            Self::JsImport(import) => import.import_clause().ok()?.source().ok(),
            Self::TsExternalModuleReference(reference) => {
                reference.source().ok()?.as_js_module_source().cloned()
            }
        }
    }
}

/// Returns the text in `range`, which lies inside the leading or trailing trivia of `token`.
fn reference_value(token: &JsSyntaxToken, range: TextRange) -> &str {
    let range = range - token.text_range().start();
    &token.text()[range]
}

/// Parses a comment such as `/// <reference types="node" />`, and returns the kind of
/// reference and the byte range of its value inside `comment`.
///
/// Attributes can appear in any order, as in
/// `/// <reference resolution-mode="import" types="pkg" />`. The first `lib`, `path`, or
/// `types` attribute decides the kind.
fn parse_triple_slash_reference(comment: &str) -> Option<(ReferenceKind, std::ops::Range<usize>)> {
    let mut rest = comment
        .strip_prefix("///")?
        .trim_start()
        .strip_prefix("<reference")?;
    loop {
        rest = rest.trim_start();
        let name_end = rest.find(|c: char| c == '=' || c.is_whitespace())?;
        let (name, after_name) = rest.split_at(name_end);
        let after_equals = after_name.trim_start().strip_prefix('=')?.trim_start();
        let quote = after_equals.chars().next().filter(|c| matches!(c, '"' | '\''))?;
        let value = &after_equals[1..];
        let value_len = value.find(quote)?;
        let kind = match name {
            "lib" => Some(ReferenceKind::Lib),
            "path" => Some(ReferenceKind::Path),
            "types" => Some(ReferenceKind::Types),
            _ => None,
        };
        if let Some(kind) = kind {
            let start = comment.len() - value.len();
            return Some((kind, start..start + value_len));
        }
        rest = &value[value_len + 1..];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(comment: &str) -> Option<&str> {
        parse_triple_slash_reference(comment).map(|(_, range)| &comment[range])
    }

    #[test]
    fn parses_references() {
        assert_eq!(parse(r#"/// <reference types="node" />"#), Some("node"));
        assert_eq!(parse("///<reference path='./a.d.ts'/>"), Some("./a.d.ts"));
        assert_eq!(parse(r#"/// <reference lib = "dom" />"#), Some("dom"));
        assert_eq!(
            parse(r#"/// <reference resolution-mode="import" types="pkg" />"#),
            Some("pkg")
        );
        assert_eq!(
            parse(r#"/// <reference types="pkg" resolution-mode="require" />"#),
            Some("pkg")
        );
    }

    #[test]
    fn ignores_other_comments() {
        assert_eq!(parse(r#"// <reference types="node" />"#), None);
        assert_eq!(parse(r#"//// <reference types="node" />"#), None);
        assert_eq!(parse(r#"/// <reference no-default-lib="true" />"#), None);
        assert_eq!(parse(r#"/// <amd-module name="foo" />"#), None);
        assert_eq!(parse(r#"/// <reference types="node />"#), None);
        assert_eq!(parse("/// <reference types=node />"), None);
    }
}
