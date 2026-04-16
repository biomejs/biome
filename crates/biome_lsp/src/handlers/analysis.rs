use crate::diagnostics::LspError;
use crate::session::Session;
use crate::utils;
use crate::utils::text_edit;
use anyhow::{Context, Result};
use biome_analyze::{
    ActionCategory, RuleCategoriesBuilder, SUPPRESSION_INLINE_ACTION_CATEGORY,
    SUPPRESSION_TOP_LEVEL_ACTION_CATEGORY, SourceActionKind,
};
use biome_configuration::analyzer::{AnalyzerSelector, RuleSelector};
use biome_diagnostics::Error;
use biome_fs::BiomePath;
use biome_line_index::LineIndex;
use biome_lsp_converters::from_proto;
use biome_rowan::{TextRange, TextSize};
use biome_service::file_handlers::astro::AstroFileHandler;
use biome_service::file_handlers::svelte::SvelteFileHandler;
use biome_service::file_handlers::vue::VueFileHandler;
use biome_service::workspace::{
    CheckFileSizeParams, FeaturesBuilder, FileFeaturesResult, FixFileMode, FixFileParams,
    GetFileContentParams, IgnoreKind, PathIsIgnoredParams, ProjectKey, PullActionsParams,
    SupportsFeatureParams,
};
use biome_service::{WorkspaceError, extension_error};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Sub;
use tower_lsp_server::ls_types::{
    self as lsp, CodeActionKind, CodeActionOrCommand, CodeActionParams, CodeActionResponse, Uri,
};
use tracing::{debug, info};

const FIX_ALL_CATEGORY: ActionCategory = ActionCategory::Source(SourceActionKind::FixAll);
const ORGANIZE_IMPORTS_CATEGORY: ActionCategory =
    ActionCategory::Source(SourceActionKind::OrganizeImports);

fn fix_all_kind() -> CodeActionKind {
    match FIX_ALL_CATEGORY.to_str() {
        Cow::Borrowed(kind) => CodeActionKind::from(kind),
        Cow::Owned(kind) => CodeActionKind::from(kind),
    }
}

/// Queries the [`AnalysisServer`] for code actions of the file matching its path
///
/// If the AnalysisServer has no matching file, results in error.
#[tracing::instrument(level = "debug", skip_all,
    fields(
        uri = display(& params.text_document.uri.as_str()),
        range = debug(params.range),
        only = debug(& params.context.only),
        diagnostics = debug(& params.context.diagnostics)
    ), err)]
pub(crate) fn code_actions(
    session: &Session,
    params: CodeActionParams,
) -> Result<Option<CodeActionResponse>, LspError> {
    info!("Code actions request");
    let features = FeaturesBuilder::new().with_linter().with_assist().build();
    let url = params.text_document.uri.clone();
    let path = session.file_path(&url)?;
    let Some(doc) = session.document(&url) else {
        return Ok(None);
    };
    if !session.workspace.file_exists(path.clone().into())? {
        return Ok(None);
    }

    if session.workspace.is_path_ignored(PathIsIgnoredParams {
        path: path.clone(),
        is_dir: false,
        project_key: doc.project_key,
        features,
        ignore_kind: IgnoreKind::Ancestors,
    })? {
        return Ok(Some(Vec::new()));
    }

    let FileFeaturesResult {
        features_supported: file_features,
    } = &session.workspace.file_features(SupportsFeatureParams {
        project_key: doc.project_key,
        path: path.clone(),
        features,
        inline_config: session.inline_config(),
        skip_ignore_check: false,
        not_requested_features: FeaturesBuilder::new()
            .with_search()
            .with_formatter()
            .build(),
    })?;

    if !file_features.supports_lint() && !file_features.supports_assist() {
        info!("Linter and assist are disabled.");
        return Ok(Some(Vec::new()));
    }
    let mut categories = RuleCategoriesBuilder::default();
    if file_features.supports_lint() {
        categories = categories.with_lint();
    }
    if file_features.supports_assist() {
        categories = categories.with_assist();
    }

    let size_limit_result = session.workspace.check_file_size(CheckFileSizeParams {
        project_key: doc.project_key,
        path: path.clone(),
    })?;
    if size_limit_result.is_too_large() {
        return Ok(None);
    }

    let mut has_fix_all = false;
    let mut has_organize_imports = false;
    let mut filters = Vec::new();
    if let Some(filter) = &params.context.only {
        for kind in filter {
            let kind = kind.as_str();
            if FIX_ALL_CATEGORY.matches(kind) {
                has_fix_all = true;
            }
            if ORGANIZE_IMPORTS_CATEGORY.matches(kind) {
                has_organize_imports = true;
            }
            filters.push(kind);
        }
    }

    let position_encoding = session.position_encoding();

    let diagnostics = params.context.diagnostics;
    let content = session.workspace.get_file_content(GetFileContentParams {
        project_key: doc.project_key,
        path: path.clone(),
    })?;
    let offset = match path.extension() {
        Some("vue") => VueFileHandler::start(content.as_str()),
        Some("astro") => AstroFileHandler::start(content.as_str()),
        Some("svelte") => SvelteFileHandler::start(content.as_str()),
        _ => None,
    };

    let cursor_range = from_proto::text_range(&doc.line_index, params.range, position_encoding)
        .with_context(|| {
            format!(
                "failed to access range {:?} in document {} {:?}",
                params.range,
                url.as_str(),
                &doc.line_index,
            )
        })?;
    let cursor_range = if let Some(offset) = offset {
        if cursor_range.start().gt(&TextSize::from(offset)) {
            TextRange::new(
                cursor_range.start().sub(TextSize::from(offset)),
                cursor_range.end().sub(TextSize::from(offset)),
            )
        } else {
            cursor_range
        }
    } else {
        cursor_range
    };
    debug!("Cursor range {:?}", &cursor_range);
    let supports_resolve = session.supports_code_action_resolve();
    let result = match session.workspace.pull_actions(PullActionsParams {
        project_key: doc.project_key,
        path: path.clone(),
        range: Some(cursor_range),
        // TODO: compute skip and only based on configuration
        skip: vec![],
        only: vec![],
        suppression_reason: None,
        enabled_rules: filters
            .iter()
            .filter_map(|filter| RuleSelector::from_lsp_filter(filter))
            .map(AnalyzerSelector::from)
            .collect(),
        categories: categories.build(),
        inline_config: session.inline_config(),
        compute_actions: !supports_resolve,
    }) {
        Ok(result) => result,
        Err(err) => {
            return if matches!(
                err,
                WorkspaceError::FileIgnored(_) | WorkspaceError::SourceFileNotSupported(_)
            ) {
                Ok(Some(Vec::new()))
            } else {
                Err(err.into())
            };
        }
    };

    debug!("Filters: {:?}", &filters);
    debug!("Has fix all: {}", has_fix_all);
    // Generate an additional code action to apply all safe fixes on the
    // document if the action category "source.fixAll" was explicitly requested
    // by the language client
    let fix_all = if has_fix_all {
        if supports_resolve {
            // Defer fix_all to codeAction/resolve
            let data = CodeActionResolveData {
                url: url.to_string(),
                rule: None,
                kind: CodeActionResolveKind::FixAll,
                range: cursor_range,
                project_key: doc.project_key,
            };
            Some(CodeActionOrCommand::CodeAction(lsp::CodeAction {
                title: String::from("Apply all safe fixes in the document."),
                kind: Some(fix_all_kind()),
                diagnostics: None,
                edit: None,
                command: None,
                is_preferred: Some(true),
                disabled: None,
                data: serde_json::to_value(data).ok(),
            }))
        } else {
            fix_all(
                session,
                &url,
                path,
                &doc.line_index,
                &diagnostics,
                None,
                has_organize_imports,
            )?
        }
    } else {
        None
    };

    let mut has_fixes = false;

    debug!("Actions: {:?}", &result.actions.len());

    let mut actions: Vec<_> = result
        .actions
        .into_iter()
        .filter_map(|action| {
            debug!(
                "Action: {:?}, and applicability {:?}",
                &action.category,
                action.suggestion.as_ref().map(|s| &s.applicability)
            );

            // Filter out source.organizeImports.biome action when assist is not supported.
            if action.category.matches("source.organizeImports.biome")
                && !file_features.supports_assist()
            {
                return None;
            }
            // Filter out quickfix.biome action when lint and assist aren't
            if action.category.matches("quickfix.biome")
                && !file_features.supports_lint()
                && !file_features.supports_assist()
            {
                return None;
            }

            // Filter out suppressions if the linter and assist aren't supported
            if (action.category.matches(SUPPRESSION_INLINE_ACTION_CATEGORY)
                || action
                    .category
                    .matches(SUPPRESSION_TOP_LEVEL_ACTION_CATEGORY))
                && !file_features.supports_lint()
                && !file_features.supports_assist()
            {
                return None;
            }

            if action.category.matches("source.biome") && !file_features.supports_assist() {
                return None;
            }
            // Remove actions that do not match the categories requested by the
            // language client
            let matches_filters = filters.iter().any(|filter| action.category.matches(filter));

            if !filters.is_empty() && !matches_filters {
                return None;
            }

            // Build resolve data if we deferred edit computation
            let resolve_data = if supports_resolve && action.suggestion.is_none() {
                let (group, rule) = action.rule_name.as_ref()?;
                let kind = if action.category.matches(SUPPRESSION_INLINE_ACTION_CATEGORY) {
                    CodeActionResolveKind::InlineSuppression
                } else if action
                    .category
                    .matches(SUPPRESSION_TOP_LEVEL_ACTION_CATEGORY)
                {
                    CodeActionResolveKind::TopLevelSuppression
                } else {
                    CodeActionResolveKind::RuleFix
                };
                Some(CodeActionResolveData {
                    url: url.to_string(),
                    rule: RuleSelector::from_group_and_rule(group, rule),
                    kind,
                    range: cursor_range,
                    project_key: doc.project_key,
                })
            } else {
                None
            };

            let mut lsp_action = utils::code_fix_to_lsp(
                &url,
                &doc.line_index,
                position_encoding,
                &diagnostics,
                action,
            )
            .ok()??;

            if let Some(data) = resolve_data {
                lsp_action.data = serde_json::to_value(data).ok();
            }

            has_fixes |= lsp_action.diagnostics.is_some();
            Some(CodeActionOrCommand::CodeAction(lsp_action))
        })
        .chain(fix_all)
        .collect();

    // If any actions is marked as fixing a diagnostic, hide other actions
    // that do not fix anything (refactor opportunities) to reduce noise
    if has_fixes {
        actions.retain(|action| {
            if let CodeActionOrCommand::CodeAction(action) = action {
                action.kind.as_ref() == Some(&fix_all_kind()) || action.diagnostics.is_some()
            } else {
                true
            }
        });
    }

    for action in &actions {
        match action {
            CodeActionOrCommand::Command(cmd) => {
                debug!("Suggested command: {}", cmd.title)
            }
            CodeActionOrCommand::CodeAction(action) => {
                debug!("Suggested action: {}", &action.title);
            }
        }
    }

    Ok(Some(actions))
}

/// Resolve kinds for code actions, used in `codeAction/resolve` data tokens.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub(crate) enum CodeActionResolveKind {
    RuleFix,
    InlineSuppression,
    TopLevelSuppression,
    FixAll,
}

/// Data stored in `lsp::CodeAction::data` for deferred resolution.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct CodeActionResolveData {
    pub url: String,
    pub rule: Option<RuleSelector>,
    pub kind: CodeActionResolveKind,
    pub range: TextRange,
    pub project_key: ProjectKey,
}

fn resolve_code_action_data(data: Option<Value>) -> Result<(CodeActionResolveData, Uri)> {
    let data = data
        .as_ref()
        .context("Missing code action data. This is usually caused by the client not supporting codeAction/resolve.")?;

    let resolve_data: CodeActionResolveData = serde_json::from_value(data.clone())
        .context("Failed to deserialize code action resolve data. This is an internal error, please report it.")?;

    let url: Uri = resolve_data
        .url
        .parse()
        .map_err(|e| anyhow::anyhow!("Failed to parse URI {}: {e}", resolve_data.url))?;

    Ok((resolve_data, url))
}

/// Resolve a code action by computing the actual text edit.
///
/// Called when the user selects a code action from the lightbulb menu.
/// The action's `data` field contains a [`CodeActionResolveData`] token that
/// identifies which rule and action category to compute.
pub(crate) fn code_action_resolve(
    session: &Session,
    params: lsp::CodeAction,
) -> Result<lsp::CodeAction, LspError> {
    let (resolve_data, url) = resolve_code_action_data(params.data.clone())?;

    let path = session.file_path(&url)?;
    let Some(doc) = session.document(&url) else {
        return Err(extension_error(&path).into());
    };
    let position_encoding = session.position_encoding();

    // Handle fix_all resolve
    if matches!(resolve_data.kind, CodeActionResolveKind::FixAll) {
        let result = fix_all(
            session,
            &url,
            path,
            &doc.line_index,
            &[],
            None,
            true, // include_organize_imports
        );
        let mut resolved = params;
        if let Ok(Some(CodeActionOrCommand::CodeAction(fix_all_action))) = result {
            resolved.edit = fix_all_action.edit;
        }
        return Ok(resolved);
    }

    // TODO: handle this error in a better way
    let rule_selector =
        AnalyzerSelector::Rule(resolve_data.rule.context("The rule doesn't exist")?);

    let result = session.workspace.pull_actions(PullActionsParams {
        project_key: resolve_data.project_key,
        path: path.clone(),
        range: Some(resolve_data.range),
        suppression_reason: None,
        only: vec![rule_selector],
        skip: vec![],
        enabled_rules: vec![],
        categories: RuleCategoriesBuilder::default()
            .with_lint()
            .with_assist()
            .build(),
        inline_config: session.inline_config(),
        compute_actions: true,
    })?;

    // Find the action matching the requested kind
    let target_action = result.actions.into_iter().find(|action| {
        let category_matches = match resolve_data.kind {
            CodeActionResolveKind::RuleFix => {
                !action.category.matches(SUPPRESSION_INLINE_ACTION_CATEGORY)
                    && !action
                        .category
                        .matches(SUPPRESSION_TOP_LEVEL_ACTION_CATEGORY)
            }
            CodeActionResolveKind::InlineSuppression => {
                action.category.matches(SUPPRESSION_INLINE_ACTION_CATEGORY)
            }
            CodeActionResolveKind::TopLevelSuppression => action
                .category
                .matches(SUPPRESSION_TOP_LEVEL_ACTION_CATEGORY),
            CodeActionResolveKind::FixAll => false,
        };
        category_matches && action.suggestion.is_some()
    });

    let Some(action) = target_action else {
        // Action no longer available (file changed, etc.)
        return Ok(params);
    };

    // Build the edit from the computed suggestion and fill it into the
    // original CodeAction, preserving title/kind/diagnostics/data per the
    // LSP spec (codeAction/resolve must return the same object enriched).
    let suggestion = action
        .suggestion
        .context("Expected a valid suggestion, but none was found. This is an internal error, please report it.")?;
    let offset = action.offset.map(u32::from);
    let edits = text_edit(
        &doc.line_index,
        suggestion.suggestion,
        position_encoding,
        offset,
    )?;

    let mut changes = HashMap::new();
    changes.insert(url.clone(), edits);

    let mut resolved = params;
    resolved.edit = Some(lsp::WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    });

    Ok(resolved)
}

/// Generate the code action `source.fixAll.biome` for the current document.
///
/// When `include_organize_imports` is `false`, the organize imports action is
/// excluded from the fix-all pass. This prevents `source.fixAll.biome` from
/// sorting imports when `source.organizeImports.biome` was not explicitly
/// requested by the editor.
#[tracing::instrument(
    level = "debug",
    skip(
        session,
        url,
        diagnostics,
        offset,
        line_index,
        include_organize_imports
    )
)]
fn fix_all(
    session: &Session,
    url: &Uri,
    path: BiomePath,
    line_index: &LineIndex,
    diagnostics: &[lsp::Diagnostic],
    offset: Option<u32>,
    include_organize_imports: bool,
) -> Result<Option<CodeActionOrCommand>, Error> {
    let Some(doc) = session.document(url) else {
        return Ok(None);
    };
    let analyzer_features = FeaturesBuilder::new().with_linter().with_assist().build();

    if !session.workspace.file_exists(path.clone().into())? {
        return Ok(None);
    }

    if session.workspace.is_path_ignored(PathIsIgnoredParams {
        path: path.clone(),
        is_dir: false,
        project_key: doc.project_key,
        features: analyzer_features,
        ignore_kind: IgnoreKind::Ancestors,
    })? {
        return Ok(None);
    }

    let FileFeaturesResult {
        features_supported: file_features,
    } = session.workspace.file_features(SupportsFeatureParams {
        project_key: doc.project_key,
        path: path.clone(),
        features: FeaturesBuilder::new()
            .with_formatter()
            .with_linter()
            .with_assist()
            .build(),
        inline_config: session.inline_config(),
        skip_ignore_check: false,
        not_requested_features: FeaturesBuilder::new().with_search().build(),
    })?;
    let should_format = file_features.supports_format();

    if session.workspace.is_path_ignored(PathIsIgnoredParams {
        path: path.clone(),
        is_dir: false,
        project_key: doc.project_key,
        features: analyzer_features,
        ignore_kind: IgnoreKind::Ancestors,
    })? {
        return Ok(None);
    }

    let size_limit_result = session.workspace.check_file_size(CheckFileSizeParams {
        project_key: doc.project_key,
        path: path.clone(),
    })?;
    if size_limit_result.is_too_large() {
        return Ok(None);
    }

    let mut categories = RuleCategoriesBuilder::default();
    if file_features.supports_lint() {
        categories = categories.with_lint();
    }
    if file_features.supports_assist() {
        categories = categories.with_assist();
    }

    let mut skip = vec![];
    if !include_organize_imports {
        skip.push(AnalyzerSelector::Rule(RuleSelector::Rule(
            "source",
            "organizeImports",
        )));
    }

    let fixed = session.workspace.fix_file(FixFileParams {
        project_key: doc.project_key,
        path: path.clone(),
        fix_file_mode: FixFileMode::SafeFixes,
        should_format,
        only: vec![],
        skip,
        enabled_rules: vec![],
        suppression_reason: None,
        rule_categories: categories.build(),
        inline_config: session.inline_config(),
    })?;
    let output = if file_features.supports_full_html_support() {
        fixed.code
    } else {
        match path.as_path().extension() {
            Some(extension) => {
                let input = session.workspace.get_file_content(GetFileContentParams {
                    project_key: doc.project_key,
                    path: path.clone(),
                })?;
                match extension {
                    "astro" => AstroFileHandler::output(input.as_str(), fixed.code.as_str()),
                    "vue" => VueFileHandler::output(input.as_str(), fixed.code.as_str()),
                    "svelte" => SvelteFileHandler::output(input.as_str(), fixed.code.as_str()),
                    _ => fixed.code,
                }
            }

            _ => fixed.code,
        }
    };
    if fixed.actions.is_empty() {
        return Ok(None);
    }

    let diagnostics = diagnostics
        .iter()
        .filter_map(|d| {
            let code = d.code.as_ref()?;
            let code = match code {
                lsp::NumberOrString::String(code) => code.as_str(),
                lsp::NumberOrString::Number(_) => return None,
            };

            let code = code.strip_prefix("lint/")?;
            let position_encoding = session.position_encoding();

            let diag_range = from_proto::text_range(line_index, d.range, position_encoding).ok()?;
            let diag_range = if let Some(offset) = offset {
                if diag_range.start().gt(&TextSize::from(offset)) {
                    TextRange::new(
                        diag_range.start().sub(TextSize::from(offset)),
                        diag_range.end().sub(TextSize::from(offset)),
                    )
                } else {
                    diag_range
                }
            } else {
                diag_range
            };
            let has_matching_rule = fixed.actions.iter().any(|action| {
                let Some((group_name, rule_name)) = &action.rule_name else {
                    return false;
                };
                let Some(code) = code.strip_prefix(group_name.as_ref()) else {
                    return false;
                };
                let Some(code) = code.strip_prefix('/') else {
                    return false;
                };
                code == rule_name && action.range.intersect(diag_range).is_some()
            });

            if has_matching_rule {
                Some(d.clone())
            } else {
                None
            }
        })
        .collect();

    let mut changes = HashMap::new();
    changes.insert(
        url.clone(),
        vec![lsp::TextEdit {
            range: lsp::Range {
                start: lsp::Position::new(0, 0),
                end: lsp::Position::new(line_index.len(), 0),
            },
            new_text: output,
        }],
    );

    let edit = lsp::WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    };

    Ok(Some(CodeActionOrCommand::CodeAction(lsp::CodeAction {
        title: String::from("Apply all safe fixes (Biome)"),
        kind: Some(fix_all_kind()),
        diagnostics: Some(diagnostics),
        edit: Some(edit),
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    })))
}
