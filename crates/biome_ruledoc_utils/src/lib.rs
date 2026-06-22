mod codeblock;
mod printer;

pub use codeblock::*;
pub use printer::*;

use anyhow::bail;
use biome_analyze::{RuleCategory, RuleMetadata};
use biome_configuration::Configuration;
use biome_db::ParsedSource;
use biome_deserialize::json::deserialize_from_json_ast;
use biome_diagnostics::DiagnosticExt;
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_js_analyze::JsAnalyzerServices;
use biome_js_semantic::semantic_model_from_source;
use biome_json_factory::make;
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonObjectValue};
use biome_languages::{DocumentFileSource, JsFileSource};
use biome_module_graph::{
    ModuleInfoKind, PathInfoCache, resolve_css_module, resolve_html_module, resolve_js_module,
};
use biome_project_layout::ProjectLayout;
use biome_rowan::{AstNode, AstSeparatedList};
use biome_test_utils::{get_added_js_paths, get_css_added_paths, get_html_added_paths};
use biome_workspace_db::WorkspaceDb;
use camino::Utf8PathBuf;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::sync::Arc;

/// Builder that can be used for constructing analyzer services.
///
/// The builder can be reused to create cheap instances of analyzer services
/// for multiple code blocks.
pub struct AnalyzerServicesBuilder {
    module_db: WorkspaceDb,
    project_layout: Arc<ProjectLayout>,
}

impl AnalyzerServicesBuilder {
    /// Creates a service builder from a map of `files`.
    ///
    /// Constructs an in-memory file system from the given files and uses it
    /// to initialise a module graph and project layout.
    ///
    /// # Arguments
    ///
    /// * `files` - A map of file paths to their contents.
    pub fn from_files<S: BuildHasher>(files: HashMap<String, String, S>) -> Self {
        if files.is_empty() {
            let db = WorkspaceDb::default();
            return Self {
                module_db: db,
                project_layout: Default::default(),
            };
        }

        let fs = MemoryFileSystem::default();
        let layout = ProjectLayout::default();
        let path_info_cache = PathInfoCache::default();

        let mut js_paths = Vec::new();
        let mut css_paths = Vec::new();
        let mut html_paths = Vec::new();

        for (path, src) in files {
            let path_buf = Utf8PathBuf::from(path);
            let biome_path = BiomePath::new(&path_buf);
            if biome_path.is_manifest() {
                match biome_path.file_name() {
                    Some("package.json") => {
                        let parsed = parse_json(&src, JsonParserOptions::default());
                        layout.insert_serialized_node_manifest(
                            path_buf.parent().unwrap().into(),
                            &parsed.syntax().as_send().unwrap(),
                        );
                    }
                    Some("tsconfig.json") => {
                        let parsed = parse_json(
                            &src,
                            JsonParserOptions::default()
                                .with_allow_comments()
                                .with_allow_trailing_commas(),
                        );
                        layout.insert_serialized_tsconfig(
                            path_buf.parent().unwrap().into(),
                            &parsed.syntax().as_send().unwrap(),
                        );
                    }
                    _ => unimplemented!("Unhandled manifest: {biome_path}"),
                }
            } else {
                let document_file_source = DocumentFileSource::from_path(&path_buf, false);
                match document_file_source {
                    DocumentFileSource::Js(_) => js_paths.push(biome_path),
                    DocumentFileSource::Css(_) => css_paths.push(biome_path),
                    DocumentFileSource::Html(_) => html_paths.push(biome_path),
                    _ => unimplemented!(
                        "Unhandled file type: {biome_path}. Add a new branch once the module graph understands new module types"
                    ),
                }
            }

            fs.insert(path_buf, src);
        }

        let db = WorkspaceDb::default();

        let js_added_paths = get_added_js_paths(&fs, &js_paths);
        for (path, root, semantic_model) in js_added_paths {
            let (module_info, _, _) = resolve_js_module(
                root,
                path,
                &fs,
                &layout,
                semantic_model,
                &path_info_cache,
                true,
            );
            let md = biome_module_graph::ModuleInfo::new(
                &db,
                path.as_path().to_path_buf(),
                ModuleInfoKind::Js(module_info),
            );
            db.insert_module(path.as_path().to_path_buf(), md);
        }

        let css_added_paths = get_css_added_paths(&fs, &css_paths);
        for (path, root) in css_added_paths {
            let (module_info, _, _) =
                resolve_css_module(root, path, &fs, &layout, &path_info_cache);
            let md = biome_module_graph::ModuleInfo::new(
                &db,
                path.as_path().to_path_buf(),
                ModuleInfoKind::Css(module_info),
            );
            db.insert_module(path.as_path().to_path_buf(), md);
        }

        let html_added_paths = get_html_added_paths(&fs, &html_paths);
        for (path, root, embedded_content) in html_added_paths {
            let (module_info, _, _) = resolve_html_module(
                root,
                &embedded_content,
                path,
                &fs,
                &layout,
                &path_info_cache,
            );
            let md = biome_module_graph::ModuleInfo::new(
                &db,
                path.as_path().to_path_buf(),
                ModuleInfoKind::Html(module_info),
            );
            db.insert_module(path.as_path().to_path_buf(), md);
        }

        Self {
            module_db: db,
            project_layout: Arc::new(layout),
        }
    }

    pub fn build_for_js_parse(
        &mut self,
        path: Utf8PathBuf,
        parse: biome_js_parser::Parse<biome_js_parser::AnyJsRoot>,
        file_source: JsFileSource,
    ) -> JsAnalyzerServices<'_> {
        let source_index = self
            .module_db
            .insert_source(DocumentFileSource::Js(file_source));
        let parsed_source = ParsedSource::new(
            &self.module_db,
            path.clone(),
            parse.into(),
            source_index,
            vec![],
        );
        self.module_db.insert_file(&path, parsed_source);

        JsAnalyzerServices::from((
            self.module_db.rc_module_db(),
            self.project_layout.clone(),
            file_source,
        ))
        .with_language_db(self.module_db.rc_language_db())
        .with_semantic_model(semantic_model_from_source(&self.module_db, parsed_source))
    }
}

/// Parse the options fragment for a lint rule and return the parsed options.
pub fn parse_rule_options(
    group: &'static str,
    rule_metadata: &RuleMetadata,
    category: RuleCategory,
    block: &CodeBlock,
    code: &str,
) -> anyhow::Result<Option<Configuration>> {
    let DocumentFileSource::Json(file_source) = block.document_file_source() else {
        bail!(
            "The following non-JSON code block for '{group}/{}' was marked as containing configuration options. Only JSON code blocks can used to provide configuration options.\n\n{code}",
            rule_metadata.name
        );
    };

    // Record the diagnostics emitted during configuration parsing to later check
    // if what was emitted matches the expectations set for this code block.
    let mut diagnostics = DiagnosticWriter::default();

    let parse = biome_json_parser::parse_json(code, JsonParserOptions::from(&file_source));

    if parse.has_errors() {
        for diag in parse.into_diagnostics() {
            let error = diag
                .with_file_path(block.file_path())
                .with_file_source_code(code);
            diagnostics.write_parse_error(error);
        }
        if block.expect_diagnostic {
            return Ok(None);
        } else {
            diagnostics.print_all_diagnostics();
            bail!("Please fix the parse errors above.");
        };
    }

    let parsed_root = parse.tree();
    let parsed_options = parsed_root.value()?;

    let root = match block.options {
        OptionsParsingMode::NoOptions => {
            unreachable!("parse_rule_options should only be called for options blocks")
        }
        OptionsParsingMode::RuleOptionsOnly => {
            // By convention, the configuration blocks in the documentation
            // only contain the settings for the lint rule itself, like so:
            //
            // ```json,options
            // {
            //     "options": {
            //         ...
            //     }
            // }
            // ```
            //
            // We therefore extend the JSON AST with some synthetic elements
            // to make it match the structure expected by the configuration parse:
            //
            // {
            //     "linter": {
            //         "rules": {
            //             "<group>": {
            //                 "<rule>": {<options>}
            //             }
            //         }
            //     }
            // }
            let lint_or_assist = if category == RuleCategory::Lint {
                "linter"
            } else {
                "assist"
            };
            let rules_or_actions = if category == RuleCategory::Lint {
                "rules"
            } else {
                "actions"
            };
            let parsed_options = make::json_object_value(
                make::token(biome_json_syntax::JsonSyntaxKind::L_CURLY),
                make::json_member_list(
                    [
                        make_member(
                            "level",
                            make::json_string_value(make::json_string_literal("on")),
                        ),
                        // we extract the "options: {}" portion.
                        parsed_options
                            .as_json_object_value()
                            .unwrap()
                            .json_member_list()
                            .first()
                            .unwrap()
                            .unwrap(),
                    ],
                    [make::token(biome_json_syntax::JsonSyntaxKind::COMMA)],
                ),
                make::token(biome_json_syntax::JsonSyntaxKind::R_CURLY),
            );
            let synthetic_tree = make_json_object_with_single_member(
                lint_or_assist,
                make_json_object_with_single_member(
                    rules_or_actions,
                    make_json_object_with_single_member(
                        group,
                        make_json_object_with_single_member(rule_metadata.name, parsed_options),
                    ),
                ),
            );

            // Create a new JsonRoot from the synthetic AST
            let eof_token = parsed_root.eof_token()?;
            let mut root_builder = make::json_root(synthetic_tree.into(), eof_token);
            if let Some(bom_token) = parsed_root.bom_token() {
                root_builder = root_builder.with_bom_token(bom_token);
            }
            let synthetic_root = root_builder.build();

            // Adjust source code spans to account for the synthetic nodes
            // so that errors are reported at the correct source code locations:
            let original_offset = parsed_root.value().ok().map(|v| AstNode::range(&v).start());
            let wrapped_offset = synthetic_root
                .value()
                .ok()
                .and_then(|v| get_first_member(v, lint_or_assist))
                .and_then(|v| get_first_member(v, rules_or_actions))
                .and_then(|v| get_first_member(v, group))
                .and_then(|v| get_first_member(v, rule_metadata.name))
                .map(|v| AstNode::range(&v).start());
            diagnostics.subtract_offset = wrapped_offset
                .zip(original_offset)
                .and_then(|(wrapped, original)| wrapped.checked_sub(original))
                .unwrap_or_default();

            synthetic_root
        }
        OptionsParsingMode::FullConfiguration => {
            // In some rare cases, we want to be able to display full JSON configuration
            // instead, e.t. to be able to show off per-file overrides:
            //
            // ```json,full-options
            // {
            //     "linter": {
            //         "rules": {
            //             "<group>": {
            //                 "<rule>": {<options>}
            //             }
            //         }
            //     }
            // }
            // ```
            parsed_root
        }
    };

    // Deserialize the configuration from the partially-synthetic AST,
    // and report any errors encountered during deserialization.
    let deserialized = deserialize_from_json_ast::<Configuration>(&root, "");
    let (config, deserialize_diagnostics) = deserialized.consume();

    if !deserialize_diagnostics.is_empty() {
        for diag in deserialize_diagnostics {
            let error = diag
                .with_file_path(block.file_path())
                .with_file_source_code(code);
            diagnostics.write_diagnostic(error);
        }
        if block.expect_diagnostic {
            return Ok(None);
        } else {
            diagnostics.print_all_diagnostics();
            bail!("Please fix the configuration errors above.");
        };
    }

    if config.is_none() {
        bail!(
            "Failed to deserialize configuration options for '{group}/{}' from the following code block due to unknown error.\n\n{code}",
            rule_metadata.name
        );
    }

    Ok(config)
}

fn get_first_member<V: Into<AnyJsonValue>>(parent: V, expected_name: &str) -> Option<AnyJsonValue> {
    let parent_value: AnyJsonValue = parent.into();
    let member = parent_value
        .as_json_object_value()?
        .json_member_list()
        .into_iter()
        .next()?
        .ok()?;
    let member_name = member.name().ok()?.inner_string_text()?.to_string();

    if member_name.as_str() == expected_name {
        member.value().ok()
    } else {
        None
    }
}

/// Creates a synthetic JSON AST for an object literal with a single member.
fn make_json_object_with_single_member<V: Into<AnyJsonValue>>(
    name: &str,
    value: V,
) -> JsonObjectValue {
    make::json_object_value(
        make::token(biome_json_syntax::JsonSyntaxKind::L_CURLY),
        make::json_member_list([make_member(name, value)], []),
        make::token(biome_json_syntax::JsonSyntaxKind::R_CURLY),
    )
}

fn make_member<V: Into<AnyJsonValue>>(name: &str, value: V) -> JsonMember {
    make::json_member(
        biome_json_syntax::AnyJsonMemberName::JsonMemberName(make::json_member_name(
            make::json_string_literal(name),
        )),
        make::token(biome_json_syntax::JsonSyntaxKind::COLON),
        value.into(),
    )
}
