use biome_analyze::{SUPPRESSION_INLINE_ACTION_CATEGORY, SUPPRESSION_TOP_LEVEL_ACTION_CATEGORY};
use biome_line_index::WideEncoding;
use biome_lsp_converters::{PositionEncoding, negotiated_encoding};
use tower_lsp_server::lsp_types::{
    ClientCapabilities, CodeActionKind, CodeActionOptions, CodeActionProviderCapability,
    DocumentOnTypeFormattingOptions, OneOf, PositionEncodingKind, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind, WorkspaceFoldersServerCapabilities,
    WorkspaceServerCapabilities,
};

pub(crate) const DEFAULT_CODE_ACTION_CAPABILITIES: &[&str] = &[
    "quickfix.biome",
    // quickfix.suppressRule
    SUPPRESSION_TOP_LEVEL_ACTION_CATEGORY,
    SUPPRESSION_INLINE_ACTION_CATEGORY,
    // import sorting
    "source.organizeImports.biome",
    // fix all
    "source.fixAll.biome",
    // general refactors
    "refactor.biome",
    "refactor.extract.biome",
    "refactor.inline.biome",
    "refactor.rewrite.biome",
    // source actions
    "source.biome",
];

/// The capabilities to send from server as part of [`InitializeResult`]
///
/// [`InitializeResult`]: tower_lsp_server::lsp::InitializeResult
pub(crate) fn server_capabilities(capabilities: &ClientCapabilities) -> ServerCapabilities {
    let supports_formatter_dynamic_registration = capabilities
        .text_document
        .as_ref()
        .and_then(|text_document| text_document.formatting.as_ref())
        .and_then(|formatting| formatting.dynamic_registration)
        .and_then(|supported| {
            if supported {
                None
            } else {
                Some(OneOf::Left(true))
            }
        });

    let supports_range_formatter_dynamic_registration = capabilities
        .text_document
        .as_ref()
        .and_then(|text_document| text_document.range_formatting.as_ref())
        .and_then(|range_formatting| range_formatting.dynamic_registration)
        .and_then(|supported| {
            if supported {
                None
            } else {
                Some(OneOf::Left(true))
            }
        });

    let supports_on_type_formatter_dynamic_registration = capabilities
        .text_document
        .as_ref()
        .and_then(|text_document| text_document.on_type_formatting.as_ref())
        .and_then(|on_type_formatting| on_type_formatting.dynamic_registration)
        .and_then(|supported| {
            if supported {
                None
            } else {
                Some(DocumentOnTypeFormattingOptions {
                    first_trigger_character: String::from("}"),
                    more_trigger_character: Some(vec![String::from("]"), String::from(")")]),
                })
            }
        });

    let code_action_provider = capabilities
        .text_document
        .as_ref()
        .and_then(|text_document| text_document.code_action.as_ref())
        .and_then(|code_action| {
            if code_action.dynamic_registration.unwrap_or(false) {
                None
            } else if code_action.code_action_literal_support.as_ref().is_some() {
                Some(CodeActionProviderCapability::from(CodeActionOptions {
                    code_action_kinds: Some(
                        DEFAULT_CODE_ACTION_CAPABILITIES
                            .iter()
                            .map(|item| CodeActionKind::from(*item))
                            .collect::<Vec<_>>(),
                    ),
                    ..Default::default()
                }))
            } else {
                Some(CodeActionProviderCapability::Simple(true))
            }
        });

    ServerCapabilities {
        position_encoding: Some(match negotiated_encoding(capabilities) {
            PositionEncoding::Utf8 => PositionEncodingKind::UTF8,
            PositionEncoding::Wide(wide) => match wide {
                WideEncoding::Utf16 => PositionEncodingKind::UTF16,
                WideEncoding::Utf32 => PositionEncodingKind::UTF32,
            },
        }),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        document_formatting_provider: supports_formatter_dynamic_registration,
        document_range_formatting_provider: supports_range_formatter_dynamic_registration,
        document_on_type_formatting_provider: supports_on_type_formatter_dynamic_registration,
        code_action_provider,
        rename_provider: None,
        workspace: Some(WorkspaceServerCapabilities {
            workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                supported: Some(true),
                change_notifications: Some(OneOf::Left(true)),
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}
