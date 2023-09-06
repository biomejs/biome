use crate::converters::{negotiated_encoding, PositionEncoding, WideEncoding};
use tower_lsp::lsp_types::{
    ClientCapabilities, CodeActionProviderCapability, PositionEncodingKind, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind,
};

/// The capabilities to send from server as part of [`InitializeResult`]
///
/// [`InitializeResult`]: lspower::lsp::InitializeResult
pub(crate) fn server_capabilities(capabilities: &ClientCapabilities) -> ServerCapabilities {
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
        code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
        rename_provider: None,
        ..Default::default()
    }
}
