use crate::server_test_utils::*;
use anyhow::{Context, Result};
use biome_fs::MemoryFileSystem;
use futures::channel::mpsc::channel;
use std::str::FromStr;
use std::sync::Arc;
use tower_lsp_server::ls_types::{
    self as lsp, ClientCapabilities, CodeActionContext, CodeActionKind, CodeActionOrCommand,
    CodeActionParams, CodeActionResponse, DidChangeTextDocumentParams, DocumentFormattingParams,
    FormattingOptions, PartialResultParams, Position, Range, TextDocumentContentChangeEvent,
    TextDocumentIdentifier, TextEdit, VersionedTextDocumentIdentifier, WorkDoneProgressParams,
};

#[tokio::test]
async fn whole_document_formatting_matches_cli_in_each_svelte_project() -> Result<()> {
    let fs = MemoryFileSystem::default();
    fs.insert(
        to_utf8_file_path_buf(uri!("test_one/biome.json")),
        r#"{
            "javascript": { "formatter": { "semicolons": "asNeeded" } },
            "html": { "experimentalFullSupportEnabled": true, "formatter": { "enabled": true } }
        }"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("test_two/biome.json")),
        r#"{
            "formatter": { "formatWithErrors": true },
            "html": { "experimentalFullSupportEnabled": true, "formatter": { "enabled": true } }
        }"#,
    );
    let documents = [
        (
            uri!("test_one/Label.svelte"),
            r#"<script>
let label=$state(`first
  second`)
</script>

<p class = 'label'>{label}<i>!</i> <b>next</b></p>
"#,
            r#"<script>
let label = $state(`first
  second`)
</script>

<p class="label">{label}<i>!</i> <b>next</b></p>
"#,
        ),
        (
            uri!("test_two/Recovery.svelte"),
            r#"<script>let count=$state(0)</script>
<style lang="less">  @ink: blue;
    p { color: @ink }  </style>
<p>𝄞{  count +  } <b class = "after">after</b></p>
"#,
            r#"<script>
let count = $state(0);
</script>
<style lang="less">  @ink: blue;
    p { color: @ink }  </style>
<p>𝄞{  count +  } <b class="after">after</b></p>
"#,
        ),
    ];
    for (uri, input, _) in &documents {
        fs.insert(to_utf8_file_path_buf(uri.clone()), *input);
    }

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);
    let (sender, _receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize_projects().await?;
    server.initialized().await?;
    server.load_configuration().await?;
    for (uri, input, _) in &documents {
        server
            .open_named_document(input, uri.clone(), "svelte")
            .await?;
    }

    for (uri, input, expected) in documents {
        for pass in 1..=2 {
            let edits: Option<Vec<TextEdit>> = server
                .request(
                    "textDocument/formatting",
                    "formatting",
                    DocumentFormattingParams {
                        text_document: TextDocumentIdentifier { uri: uri.clone() },
                        options: FormattingOptions {
                            tab_size: 2,
                            ..Default::default()
                        },
                        work_done_progress_params: Default::default(),
                    },
                )
                .await?
                .context("missing formatting response")?;
            let edits = edits.context("formatting not available")?;
            if pass == 2 {
                assert!(edits.is_empty(), "second pass changed {uri:?}: {edits:?}");
                continue;
            }
            let changes = edits
                .into_iter()
                .rev()
                .map(|edit| TextDocumentContentChangeEvent {
                    range: Some(edit.range),
                    range_length: None,
                    text: edit.new_text,
                })
                .collect();
            let output = crate::utils::apply_document_changes(
                biome_lsp_converters::negotiated_encoding(&ClientCapabilities::default()),
                input.to_string(),
                changes,
            );
            assert_eq!(output, expected, "{uri:?}");
            server
                .notify(
                    "textDocument/didChange",
                    DidChangeTextDocumentParams {
                        text_document: VersionedTextDocumentIdentifier {
                            uri: uri.clone(),
                            version: 1,
                        },
                        content_changes: vec![TextDocumentContentChangeEvent {
                            range: None,
                            range_length: None,
                            text: output,
                        }],
                    },
                )
                .await?;
        }
    }

    server.shutdown().await?;
    reader.abort();
    Ok(())
}

async fn wait_for_document_diagnostics(
    receiver: &mut futures::channel::mpsc::Receiver<ServerNotification>,
    uri: &lsp::Uri,
) -> lsp::PublishDiagnosticsParams {
    let notification = wait_for_notification(receiver, |notification| {
        matches!(
            notification,
            ServerNotification::PublishDiagnostics(params) if &params.uri == uri
        )
    })
    .await;
    let Some(ServerNotification::PublishDiagnostics(params)) = notification else {
        panic!("expected diagnostics for {uri:?}");
    };
    params
}

async fn apply_safe_fixes(
    server: &mut Server,
    uri: &lsp::Uri,
    input: &str,
    diagnostics: &[lsp::Diagnostic],
) -> Result<String> {
    let actions: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "svelte_safe_fixes",
            CodeActionParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                range: Range::new(
                    Position::new(0, 0),
                    Position::new(input.lines().count() as u32, 0),
                ),
                context: CodeActionContext {
                    diagnostics: diagnostics.to_vec(),
                    only: Some(vec![CodeActionKind::new("source.fixAll.biome")]),
                    ..Default::default()
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
            },
        )
        .await?
        .context("codeAction returned None")?;
    let [CodeActionOrCommand::CodeAction(action)] = actions.as_slice() else {
        panic!("expected one fix-all action: {actions:?}");
    };
    assert_eq!(
        action.kind,
        Some(CodeActionKind::new("source.fixAll.biome"))
    );
    assert_eq!(action.diagnostics.as_deref(), Some(diagnostics));
    let mut changes = action
        .edit
        .clone()
        .context("fix-all action had no edit")?
        .changes
        .context("fix-all edit had no changes")?;
    let edits = changes
        .remove(uri)
        .context("fix-all edit did not target the document")?;
    let [edit] = edits.as_slice() else {
        panic!("expected one full-document edit: {edits:?}");
    };
    Ok(edit.new_text.clone())
}

#[tokio::test]
async fn lint_matches_cli_for_components_and_modules() -> Result<()> {
    let component_uri = uri!("Component.svelte");
    let component_input = r#"<!-- biome-ignore-start lint/complexity/noUselessStringConcat: intentional concatenation -->
<script context="module">
export const preserved = "a" + "b";
</script>
<!-- biome-ignore-end lint/complexity/noUselessStringConcat: intentional concatenation -->

<script>
const fixed = "c" + "d";
</script>

<p>{preserved}{fixed}</p>
"#;
    let component_expected = r#"<!-- biome-ignore-start lint/complexity/noUselessStringConcat: intentional concatenation -->
<script context="module">
export const preserved = "a" + "b";
</script>
<!-- biome-ignore-end lint/complexity/noUselessStringConcat: intentional concatenation -->

<script>
const fixed = "cd";
</script>

<p>{preserved}{fixed}</p>
"#;
    let module_uri = uri!("state.svelte.ts");
    let module_input = r#"// biome-ignore lint/complexity/noUselessStringConcat: intentional concatenation
export const preserved = "a" + "b";
export const fixed = "c" + "d";
"#;
    let module_expected = r#"// biome-ignore lint/complexity/noUselessStringConcat: intentional concatenation
export const preserved = "a" + "b";
export const fixed = "cd";
"#;

    let fs = MemoryFileSystem::default();
    fs.insert(
        to_utf8_file_path_buf(uri!("biome.json")),
        r#"{
            "formatter": { "enabled": false },
            "html": {
                "experimentalFullSupportEnabled": true,
                "linter": { "enabled": true }
            },
            "linter": {
                "rules": {
                    "preset": "none",
                    "complexity": { "noUselessStringConcat": "warn" }
                }
            },
            "assist": { "enabled": false }
        }"#,
    );
    fs.insert(
        to_utf8_file_path_buf(component_uri.clone()),
        component_input,
    );
    fs.insert(to_utf8_file_path_buf(module_uri.clone()), module_input);

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);
    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;
    server.load_configuration().await?;

    server
        .open_named_document(component_input, component_uri.clone(), "svelte")
        .await?;
    let component_diagnostics = wait_for_document_diagnostics(&mut receiver, &component_uri).await;
    let [component_diagnostic] = component_diagnostics.diagnostics.as_slice() else {
        panic!(
            "expected one component diagnostic: {:?}",
            component_diagnostics.diagnostics
        );
    };
    assert_eq!(
        component_diagnostic.code,
        Some(lsp::NumberOrString::String(
            "lint/complexity/noUselessStringConcat".to_string()
        ))
    );
    assert_eq!(
        component_diagnostic.range,
        Range::new(Position::new(7, 14), Position::new(7, 23))
    );
    assert_eq!(
        component_diagnostic.severity,
        Some(lsp::DiagnosticSeverity::WARNING)
    );
    assert_eq!(
        component_diagnostic.message,
        "Useless string concatenation."
    );
    let component_output = apply_safe_fixes(
        &mut server,
        &component_uri,
        component_input,
        &component_diagnostics.diagnostics,
    )
    .await?;
    assert_eq!(component_output, component_expected);
    server
        .notify(
            "textDocument/didChange",
            DidChangeTextDocumentParams {
                text_document: VersionedTextDocumentIdentifier {
                    uri: component_uri.clone(),
                    version: 1,
                },
                content_changes: vec![TextDocumentContentChangeEvent {
                    range: None,
                    range_length: None,
                    text: component_output,
                }],
            },
        )
        .await?;
    let component_diagnostics = wait_for_document_diagnostics(&mut receiver, &component_uri).await;
    assert_eq!(component_diagnostics.version, Some(1));
    assert!(component_diagnostics.diagnostics.is_empty());

    server
        .open_named_document(module_input, module_uri.clone(), "typescript")
        .await?;
    let module_diagnostics = wait_for_document_diagnostics(&mut receiver, &module_uri).await;
    let [module_diagnostic] = module_diagnostics.diagnostics.as_slice() else {
        panic!(
            "expected one module diagnostic: {:?}",
            module_diagnostics.diagnostics
        );
    };
    assert_eq!(
        module_diagnostic.code,
        Some(lsp::NumberOrString::String(
            "lint/complexity/noUselessStringConcat".to_string()
        ))
    );
    assert_eq!(
        module_diagnostic.range,
        Range::new(Position::new(2, 21), Position::new(2, 30))
    );
    assert_eq!(
        module_diagnostic.severity,
        Some(lsp::DiagnosticSeverity::WARNING)
    );
    assert_eq!(module_diagnostic.message, "Useless string concatenation.");
    let module_output = apply_safe_fixes(
        &mut server,
        &module_uri,
        module_input,
        &module_diagnostics.diagnostics,
    )
    .await?;
    assert_eq!(module_output, module_expected);
    server
        .notify(
            "textDocument/didChange",
            DidChangeTextDocumentParams {
                text_document: VersionedTextDocumentIdentifier {
                    uri: module_uri.clone(),
                    version: 1,
                },
                content_changes: vec![TextDocumentContentChangeEvent {
                    range: None,
                    range_length: None,
                    text: module_output,
                }],
            },
        )
        .await?;
    let module_diagnostics = wait_for_document_diagnostics(&mut receiver, &module_uri).await;
    assert_eq!(module_diagnostics.version, Some(1));
    assert!(module_diagnostics.diagnostics.is_empty());

    server.shutdown().await?;
    reader.abort();
    Ok(())
}

#[tokio::test]
async fn reports_cross_snippet_reassignments() -> Result<()> {
    let uri = uri!("Component.svelte");
    let input = r#"<script>
import value from "pkg";
function update() {}
class Widget {}
</script>

<button
    onclick={() => {
        value = 1;
        update = () => {};
        Widget = class {};
    }}
>
    Update
</button>
"#;
    let updated = r#"<script>
import value from "pkg";
function update() {}
class Widget {}
</script>

<button>Update</button>
"#;

    let fs = MemoryFileSystem::default();
    fs.insert(
        to_utf8_file_path_buf(uri!("biome.json")),
        r#"{
            "formatter": { "enabled": false },
            "html": {
                "experimentalFullSupportEnabled": true,
                "linter": { "enabled": true }
            },
            "linter": {
                "rules": {
                    "preset": "none",
                    "suspicious": {
                        "noClassAssign": "error",
                        "noFunctionAssign": "error",
                        "noImportAssign": "error"
                    }
                }
            },
            "assist": { "enabled": false }
        }"#,
    );
    fs.insert(to_utf8_file_path_buf(uri.clone()), input);

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);
    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;
    server.load_configuration().await?;
    server
        .open_named_document(input, uri.clone(), "svelte")
        .await?;

    let diagnostics = wait_for_document_diagnostics(&mut receiver, &uri).await;
    let expected = [
        (
            "lint/suspicious/noImportAssign",
            Range::new(Position::new(8, 8), Position::new(8, 13)),
        ),
        (
            "lint/suspicious/noFunctionAssign",
            Range::new(Position::new(9, 8), Position::new(9, 14)),
        ),
        (
            "lint/suspicious/noClassAssign",
            Range::new(Position::new(10, 8), Position::new(10, 14)),
        ),
    ];
    assert_eq!(diagnostics.diagnostics.len(), expected.len());
    for (diagnostic, (code, range)) in diagnostics.diagnostics.iter().zip(expected) {
        assert_eq!(
            diagnostic.code,
            Some(lsp::NumberOrString::String(code.to_string()))
        );
        assert_eq!(diagnostic.range, range);
        assert_eq!(diagnostic.severity, Some(lsp::DiagnosticSeverity::ERROR));
    }

    server
        .notify(
            "textDocument/didChange",
            DidChangeTextDocumentParams {
                text_document: VersionedTextDocumentIdentifier {
                    uri: uri.clone(),
                    version: 1,
                },
                content_changes: vec![TextDocumentContentChangeEvent {
                    range: None,
                    range_length: None,
                    text: updated.to_string(),
                }],
            },
        )
        .await?;
    let diagnostics = wait_for_document_diagnostics(&mut receiver, &uri).await;
    assert_eq!(diagnostics.version, Some(1));
    assert!(diagnostics.diagnostics.is_empty());

    server.shutdown().await?;
    reader.abort();
    Ok(())
}
