use crate::diagnostics::LspError;
use crate::session::Session;
use crate::utils;
use anyhow::{Context, Result};
use biome_analyze::{
    ActionCategory, RuleCategoriesBuilder, SourceActionKind, SUPPRESSION_INLINE_ACTION_CATEGORY,
};
use biome_configuration::analyzer::RuleSelector;
use biome_diagnostics::{Applicability, Error};
use biome_fs::BiomePath;
use biome_lsp_converters::from_proto;
use biome_lsp_converters::line_index::LineIndex;
use biome_rowan::{TextRange, TextSize};
use biome_service::file_handlers::{AstroFileHandler, SvelteFileHandler, VueFileHandler};
use biome_service::workspace::{
    CheckFileSizeParams, FeaturesBuilder, FixFileMode, FixFileParams, GetFileContentParams,
    PullActionsParams, SupportsFeatureParams,
};
use biome_service::WorkspaceError;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Sub;
use tower_lsp::lsp_types::{
    self as lsp, CodeActionKind, CodeActionOrCommand, CodeActionParams, CodeActionResponse,
};
use tracing::{debug, info};

const FIX_ALL_CATEGORY: ActionCategory = ActionCategory::Source(SourceActionKind::FixAll);

fn fix_all_kind() -> CodeActionKind {
    match FIX_ALL_CATEGORY.to_str() {
        Cow::Borrowed(kind) => CodeActionKind::from(kind),
        Cow::Owned(kind) => CodeActionKind::from(kind),
    }
}

/// Queries the [`AnalysisServer`] for code actions of the file matching its path
///
/// If the AnalysisServer has no matching file, results in error.
#[tracing::instrument(level = "debug", skip_all, fields(uri = display(& params.text_document.uri), range = debug(params.range), only = debug(& params.context.only), diagnostics = debug(& params.context.diagnostics)), err)]
pub(crate) fn code_actions(
    session: &Session,
    params: CodeActionParams,
) -> Result<Option<CodeActionResponse>, LspError> {
    let url = params.text_document.uri.clone();
    let path = session.file_path(&url)?;
    let doc = session.document(&url)?;

    let file_features = &session.workspace.file_features(SupportsFeatureParams {
        project_key: doc.project_key,
        path: path.clone(),
        features: FeaturesBuilder::new().with_linter().with_assist().build(),
    })?;

    if !file_features.supports_lint() && !file_features.supports_assist() {
        info!("Linter, assist and organize imports are disabled");
        return Ok(Some(Vec::new()));
    }

    let size_limit_result = session.workspace.check_file_size(CheckFileSizeParams {
        project_key: doc.project_key,
        path: path.clone(),
    })?;
    if size_limit_result.is_too_large() {
        return Ok(None);
    }

    let mut has_fix_all = false;
    let mut has_quick_fix = false;
    let mut filters = Vec::new();
    if let Some(filter) = &params.context.only {
        for kind in filter {
            let kind = kind.as_str();
            if FIX_ALL_CATEGORY.matches(kind) {
                has_fix_all = true;
            } else if ActionCategory::QuickFix(Cow::Borrowed("")).to_str() == kind {
                // The action is a on-save quick-fixes
                has_quick_fix = true;
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
                "failed to access range {:?} in document {url} {:?}",
                params.range, &doc.line_index,
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
            .collect(),
    }) {
        Ok(result) => result,
        Err(err) => {
            return if matches!(err, WorkspaceError::FileIgnored(_)) {
                Ok(Some(Vec::new()))
            } else {
                Err(err.into())
            };
        }
    };

    debug!("Filters: {:?}", &filters);

    // Generate an additional code action to apply all safe fixes on the
    // document if the action category "source.fixAll" was explicitly requested
    // by the language client
    let fix_all = if has_fix_all {
        fix_all(session, &url, path, &doc.line_index, &diagnostics, offset)?
    } else {
        None
    };

    let mut has_fixes = false;

    let mut actions: Vec<_> = result
        .actions
        .into_iter()
        .filter_map(|action| {
            // Don't apply unsafe fixes when the code action is on-save quick-fixes
            if has_quick_fix && action.suggestion.applicability == Applicability::MaybeIncorrect {
                return None;
            }
            // Filter out source.organizeImports.biome action when assist is not supported.
            if action.category.matches("source.organizeImports.biome")
                && !file_features.supports_assist()
            {
                return None;
            }
            // Filter out quickfix.biome action when lint is not supported.
            if action.category.matches("quickfix.biome") && !file_features.supports_lint() {
                return None;
            }

            // Filter out suppressions if the linter isn't supported
            if (action.category.matches(SUPPRESSION_INLINE_ACTION_CATEGORY)
                || action.category.matches(SUPPRESSION_INLINE_ACTION_CATEGORY))
                && !file_features.supports_lint()
            {
                return None;
            }

            // Filter out the suppressions if the client is requesting a fix all signal.
            // Fix all should apply only the safe changes.
            if has_fix_all
                && (action.category.matches(SUPPRESSION_INLINE_ACTION_CATEGORY)
                    || action.category.matches(SUPPRESSION_INLINE_ACTION_CATEGORY))
            {
                return None;
            }

            if action.category.matches("source.biome") && !file_features.supports_assist() {
                return None;
            }
            // Remove actions that do not match the categories requested by the
            // language client
            let matches_filters = filters.iter().any(|filter| {
                debug!(
                    "Filter {:?}, category {:?}",
                    filter,
                    action.category.to_str()
                );
                action.category.matches(filter)
            });
            if !filters.is_empty() && !matches_filters {
                return None;
            }

            let action = utils::code_fix_to_lsp(
                &url,
                &doc.line_index,
                position_encoding,
                &diagnostics,
                action,
                offset,
            )
            .ok()?;

            has_fixes |= action.diagnostics.is_some();
            Some(CodeActionOrCommand::CodeAction(action))
        })
        .rev()
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

    debug!("Suggested actions: \n{:?}", &actions);

    Ok(Some(actions))
}

/// Generate a "fix all" code action for the given document
#[tracing::instrument(level = "debug", skip(session))]
fn fix_all(
    session: &Session,
    url: &lsp::Url,
    path: BiomePath,
    line_index: &LineIndex,
    diagnostics: &[lsp::Diagnostic],
    offset: Option<u32>,
) -> Result<Option<CodeActionOrCommand>, Error> {
    let doc = session.document(url)?;

    let should_format = session
        .workspace
        .file_features(SupportsFeatureParams {
            project_key: doc.project_key,
            path: path.clone(),
            features: FeaturesBuilder::new().with_formatter().build(),
        })?
        .supports_format();
    let size_limit_result = session.workspace.check_file_size(CheckFileSizeParams {
        project_key: doc.project_key,
        path: path.clone(),
    })?;
    if size_limit_result.is_too_large() {
        return Ok(None);
    }

    let fixed = session.workspace.fix_file(FixFileParams {
        project_key: doc.project_key,
        path,
        fix_file_mode: FixFileMode::SafeFixes,
        should_format,
        only: vec![],
        skip: vec![],
        enabled_rules: vec![],
        suppression_reason: None,
        rule_categories: RuleCategoriesBuilder::default()
            .with_syntax()
            .with_lint()
            .with_assist()
            .build(),
    })?;

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
            new_text: fixed.code,
        }],
    );

    let edit = lsp::WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    };

    Ok(Some(CodeActionOrCommand::CodeAction(lsp::CodeAction {
        title: String::from("Fix all auto-fixable issues"),
        kind: Some(fix_all_kind()),
        diagnostics: Some(diagnostics),
        edit: Some(edit),
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    })))
}
