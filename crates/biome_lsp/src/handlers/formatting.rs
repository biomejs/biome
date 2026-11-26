use crate::diagnostics::LspError;
use crate::session::Session;
use crate::utils::text_edit;
use anyhow::Context;
use biome_fs::BiomePath;
use biome_languages::DocumentFileSource;
use biome_lsp_converters::from_proto;
use biome_rowan::{TextLen, TextRange, TextSize};
use biome_service::Workspace;
use biome_service::file_handlers::astro::AstroFileHandler;
use biome_service::file_handlers::svelte::SvelteFileHandler;
use biome_service::file_handlers::vue::VueFileHandler;
use biome_service::workspace::{
    CheckFileSizeParams, FeatureKind, FeaturesBuilder, FeaturesSupported, FileFeaturesResult,
    FormatFileParams, FormatOnTypeParams, FormatRangeParams, GetFileContentParams, IgnoreKind,
    SupportsFeatureParams,
};
use biome_service::{WorkspaceError, extension_error};
use tower_lsp_server::ls_types::*;

#[tracing::instrument(level = "debug", skip(session), err)]
pub(crate) fn format(
    session: &Session,
    params: DocumentFormattingParams,
) -> Result<Option<Vec<TextEdit>>, LspError> {
    let url = params.text_document.uri;
    let Some(doc) = session.document(&url) else {
        return Ok(None);
    };
    let path = doc.path.clone();
    if !session
        .workspace_for_request()
        .file_exists(path.clone().into())?
    {
        return Ok(None);
    }

    let features = FeaturesBuilder::new().with_formatter().build();

    let FileFeaturesResult {
        features_supported: file_features,
    } = session
        .workspace_for_request()
        .file_features(SupportsFeatureParams {
            project_key: doc.project_key,
            path: path.clone(),
            features,
            inline_config: session.inline_config(),
            skip_ignore_check: false,
            not_requested_features: FeaturesBuilder::new().with_search().build(),
            ignore_kind: IgnoreKind::Ancestors,
        })?;
    if !file_features.supports_format() {
        return notify_user(file_features, &url, doc.file_source);
    }

    let size_limit_result =
        session
            .workspace_for_request()
            .check_file_size(CheckFileSizeParams {
                project_key: doc.project_key,
                path: path.clone(),
            })?;
    if size_limit_result.is_too_large() {
        return Ok(None);
    }

    let printed = session
        .workspace_for_request()
        .format_file(FormatFileParams {
            project_key: doc.project_key,
            path: path.clone(),
            inline_config: session.inline_config(),
        })?;

    let mut output = printed.into_code();
    let input = session
        .workspace_for_request()
        .get_file_content(GetFileContentParams {
            project_key: doc.project_key,
            path: path.clone(),
        })?;
    if output.is_empty() {
        return Ok(None);
    }
    if !file_features.supports_full_html_support() {
        match doc.extension() {
            Some("astro") => {
                output = AstroFileHandler::output(input.as_str(), output.as_str());
            }
            Some("vue") => {
                output = VueFileHandler::output(input.as_str(), output.as_str());
            }
            Some("svelte") => {
                output = SvelteFileHandler::output(input.as_str(), output.as_str());
            }
            _ => {}
        }
    }

    let indels = biome_text_edit::TextEdit::from_unicode_words(input.as_str(), output.as_str());
    let position_encoding = session.position_encoding();
    let edits = text_edit(&doc.line_index, indels, position_encoding, None)?;

    Ok(Some(edits))
}

#[tracing::instrument(level = "debug", skip(session), err)]
pub(crate) fn format_range(
    session: &Session,
    params: DocumentRangeFormattingParams,
) -> Result<Option<Vec<TextEdit>>, LspError> {
    let url = params.text_document.uri;
    let Some(doc) = session.document(&url) else {
        let Some(file_path) = Session::filesystem_path(&url) else {
            return Ok(None);
        };
        let path: BiomePath = file_path.into();
        return Err(extension_error(&path).into());
    };
    let path = doc.path.clone();
    if !session
        .workspace_for_request()
        .file_exists(path.clone().into())?
    {
        return Ok(None);
    }

    let features = FeaturesBuilder::new().with_formatter().build();
    let FileFeaturesResult {
        features_supported: file_features,
    } = session
        .workspace_for_request()
        .file_features(SupportsFeatureParams {
            project_key: doc.project_key,
            path: path.clone(),
            features,
            inline_config: session.inline_config(),
            skip_ignore_check: false,
            not_requested_features: FeaturesBuilder::new().with_search().build(),
            ignore_kind: IgnoreKind::Ancestors,
        })?;
    if !file_features.supports_format() {
        return notify_user(file_features, &url, doc.file_source);
    }

    let size_limit_result =
        session
            .workspace_for_request()
            .check_file_size(CheckFileSizeParams {
                project_key: doc.project_key,
                path: path.clone(),
            })?;
    if size_limit_result.is_too_large() {
        return Ok(None);
    }

    let position_encoding = session.position_encoding();
    let format_range = from_proto::text_range(&doc.line_index, params.range, position_encoding)
        .with_context(|| {
            format!(
                "failed to convert range {:?} in document {}",
                params.range.end,
                url.as_str()
            )
        })?;
    let content = session
        .workspace_for_request()
        .get_file_content(GetFileContentParams {
            project_key: doc.project_key,
            path: path.clone(),
        })?;

    let Some((format_range, script_offset)) = embedded_format_range(
        format_range,
        file_features.supports_full_html_support(),
        doc.extension(),
        content.as_str(),
    ) else {
        return Ok(None);
    };

    let formatted = session
        .workspace_for_request()
        .format_range(FormatRangeParams {
            project_key: doc.project_key,
            path: path.clone(),
            range: format_range,
            inline_config: session.inline_config(),
        })?;

    let formatted_range = formatted.range().map_or_else(
        || TextRange::up_to(content.text_len()),
        |range| range + script_offset,
    );
    let indels = biome_text_edit::TextEdit::from_unicode_words(
        &content.as_str()[formatted_range],
        formatted.as_code(),
    );
    let position_encoding = session.position_encoding();
    let edits = text_edit(
        &doc.line_index,
        indels,
        position_encoding,
        Some(formatted_range.start().into()),
    )?;

    Ok(Some(edits))
}

#[tracing::instrument(level = "debug", skip(session), err)]
pub(crate) fn format_on_type(
    session: &Session,
    params: DocumentOnTypeFormattingParams,
) -> Result<Option<Vec<TextEdit>>, LspError> {
    let url = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let Some(doc) = session.document(&url) else {
        let Some(file_path) = Session::filesystem_path(&url) else {
            return Ok(None);
        };
        let path: BiomePath = file_path.into();
        return Err(extension_error(&path).into());
    };
    let path = doc.path.clone();
    if !session
        .workspace_for_request()
        .file_exists(path.clone().into())?
    {
        return Ok(None);
    }

    let features = FeaturesBuilder::new().with_formatter().build();
    let FileFeaturesResult {
        features_supported: file_features,
    } = session
        .workspace_for_request()
        .file_features(SupportsFeatureParams {
            project_key: doc.project_key,
            path: path.clone(),
            features,
            inline_config: session.inline_config(),
            skip_ignore_check: false,
            not_requested_features: FeaturesBuilder::new().with_search().build(),
            ignore_kind: IgnoreKind::Ancestors,
        })?;
    if !file_features.supports_format() {
        return notify_user(file_features, &url, doc.file_source);
    }

    let size_limit_result =
        session
            .workspace_for_request()
            .check_file_size(CheckFileSizeParams {
                project_key: doc.project_key,
                path: path.clone(),
            })?;
    if size_limit_result.is_too_large() {
        return Ok(None);
    }

    let position_encoding = session.position_encoding();
    let offset =
        from_proto::offset(&doc.line_index, position, position_encoding).with_context(|| {
            format!(
                "failed to access position {position:?} in document {}",
                url.as_str()
            )
        })?;

    let formatted = session
        .workspace_for_request()
        .format_on_type(FormatOnTypeParams {
            project_key: doc.project_key,
            path: path.clone(),
            offset,
            inline_config: session.inline_config(),
        })?;

    let content = session
        .workspace_for_request()
        .get_file_content(GetFileContentParams {
            project_key: doc.project_key,
            path: path.clone(),
        })?;

    let formatted_range = formatted
        .range()
        .unwrap_or_else(|| TextRange::up_to(content.text_len()));
    let indels = biome_text_edit::TextEdit::from_unicode_words(
        &content.as_str()[formatted_range],
        formatted.as_code(),
    );
    let edits = text_edit(
        &doc.line_index,
        indels,
        position_encoding,
        Some(formatted_range.start().into()),
    )?;

    Ok(Some(edits))
}

/// Maps a document range to the script of a document that uses the legacy embedded handlers.
///
/// Returns the range relative to the script, and the offset of the script in the document. Ranges
/// of other documents are returned unchanged with a zero offset. Returns `None` when the range
/// ends before the script, and clamps a range that starts before the script to its start.
fn embedded_format_range(
    range: TextRange,
    supports_full_html_support: bool,
    extension: Option<&str>,
    content: &str,
) -> Option<(TextRange, TextSize)> {
    let offset = if supports_full_html_support {
        None
    } else {
        match extension {
            Some("vue") => VueFileHandler::start(content),
            Some("astro") => AstroFileHandler::start(content),
            Some("svelte") => SvelteFileHandler::start(content),
            _ => None,
        }
    };
    let Some(offset) = offset.map(TextSize::from) else {
        return Some((range, TextSize::from(0)));
    };
    if range.end() <= offset {
        return None;
    }
    let start = range.start().max(offset);
    Some((TextRange::new(start - offset, range.end() - offset), offset))
}

fn notify_user<T>(
    file_features: FeaturesSupported,
    document_uri: &Uri,
    document_file_source: DocumentFileSource,
) -> Result<Option<T>, LspError> {
    let support_kind = file_features.support_kind_for(FeatureKind::Format);
    if support_kind.is_not_enabled() {
        return Ok(None);
    }

    let display_path = Session::filesystem_path(document_uri).map_or_else(
        || document_uri.as_str().to_string(),
        |path| path.to_string(),
    );
    let error = if support_kind.is_ignored() {
        WorkspaceError::file_ignored(display_path)
    } else if support_kind.is_protected() {
        WorkspaceError::protected_file(display_path)
    } else if let Some(file_path) = Session::filesystem_path(document_uri) {
        let path: BiomePath = file_path.into();
        extension_error(&path)
    } else {
        // This is a virtual file
        WorkspaceError::source_file_not_supported(
            document_file_source,
            display_path,
            Session::virtual_document_extension(document_uri).map(str::to_string),
        )
    };

    Err(error.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_range_before_the_script_is_not_formatted() {
        let content = "<template></template><script>\nstatement()\n</script>";
        let range = TextRange::new(TextSize::from(2), TextSize::from(4));
        assert_eq!(
            embedded_format_range(range, false, Some("vue"), content),
            None
        );
    }

    #[test]
    fn embedded_range_starting_before_the_script_is_clamped() {
        let content = "<template></template><script>\nstatement()\n</script>";
        let offset = TextSize::from(VueFileHandler::start(content).unwrap());
        let range = TextRange::new(TextSize::from(2), offset + TextSize::from(6));
        assert_eq!(
            embedded_format_range(range, false, Some("vue"), content),
            Some((TextRange::up_to(TextSize::from(6)), offset))
        );
    }

    #[test]
    fn embedded_offset_is_applied_inside_the_script() {
        let content = "<template></template><script>\nstatement()\n</script>";
        let offset = TextSize::from(VueFileHandler::start(content).unwrap());
        let range = TextRange::new(offset + TextSize::from(2), offset + TextSize::from(6));
        assert_eq!(
            embedded_format_range(range, false, Some("vue"), content),
            Some((TextRange::new(TextSize::from(2), TextSize::from(6)), offset))
        );
    }

    #[test]
    fn full_html_range_uses_document_coordinates() {
        let content = "<template></template><script>\nstatement()\n</script>";
        let offset = TextSize::from(VueFileHandler::start(content).unwrap());
        let range = TextRange::new(offset + TextSize::from(2), offset + TextSize::from(6));
        assert_eq!(
            embedded_format_range(range, true, Some("vue"), content),
            Some((range, TextSize::from(0)))
        );
    }
}
