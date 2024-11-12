use biome_analyze::{SUPPRESSION_INLINE_ACTION_CATEGORY, SUPPRESSION_TOP_LEVEL_ACTION_CATEGORY};
use biome_lsp_converters::{negotiated_encoding, PositionEncoding, WideEncoding};
use tower_lsp::lsp_types::{
    ClientCapabilities, CodeActionKind, CodeActionOptions, CodeActionProviderCapability,
    DocumentOnTypeFormattingOptions, OneOf, PositionEncodingKind, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind,
};

/// The capabilities to send from server as part of [`InitializeResult`]
///
/// [`InitializeResult`]: lspower::lsp::InitializeResult
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
        .and_then(|code_action| code_action.code_action_literal_support.as_ref())
        .map(|_| {
            CodeActionOptions {
                code_action_kinds: Some(vec![
                    CodeActionKind::from("quickfix.biome"),
                    // quickfix.suppressRule
                    CodeActionKind::from(SUPPRESSION_TOP_LEVEL_ACTION_CATEGORY),
                    CodeActionKind::from(SUPPRESSION_INLINE_ACTION_CATEGORY),
                    CodeActionKind::from("source.fixAll.biome"),
                    CodeActionKind::from("source.organizeImports.biome"),
                    CodeActionKind::from("refactor.biome"),
                    CodeActionKind::from("refactor.extract.biome"),
                    CodeActionKind::from("refactor.inline.biome"),
                    CodeActionKind::from("refactor.rewrite.biome"),
                    CodeActionKind::from("source.biome"),
                ]),
                ..Default::default()
            }
            .into()
        })
        .or(Some(CodeActionProviderCapability::Simple(true)));
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
        ..Default::default()
    }
}
