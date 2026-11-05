use crate::server_test_utils::*;
use anyhow::{Context, Result};
use biome_fs::MemoryFileSystem;
use futures::channel::mpsc::channel;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tower_lsp_server::ls_types::{
    self as lsp, ClientCapabilities, CodeActionCapabilityResolveSupport,
    CodeActionClientCapabilities, CodeActionContext, CodeActionKind, CodeActionOrCommand,
    CodeActionParams, CodeActionResponse, DidChangeTextDocumentParams, DocumentFormattingParams,
    FormattingOptions, InitializeParams, InitializeResult, PartialResultParams, Position, Range,
    TextDocumentClientCapabilities, TextDocumentContentChangeEvent, TextDocumentIdentifier,
    TextEdit, Uri, VersionedTextDocumentIdentifier, WorkDoneProgressParams, WorkspaceFolder,
};

const INPUT: &str = r#"---
import Card from "./Card.astro";
const title="Hello";
const items=[{id:1,label:"One"}];
---
<!doctype html>
<html lang="en"><head><title>{title}</title></head><body><Card title={title}/><><ul class:list={["items",{active:items.length>0}]}>{items.map((item)=><li data-id={item.id}>{item.label}</li>)}</ul></><script>document.documentElement.dataset.ready="true";</script><style>.items{display:grid}</style><pre is:raw>{title}</pre></body></html>
"#;

const EXPECTED: &str = r#"---
import Card from "./Card.astro";
const title = "Hello";
const items = [{ id: 1, label: "One" }];
---

<!doctype html>
<html lang="en">
	<head>
		<title>{title}</title>
	</head>
	<body>
		<Card title={title} />
		<>
			<ul class:list={["items", { active: items.length > 0 }]}>
				{items.map((item) => <li data-id={item.id}>{item.label}</li>)}
			</ul>
		</>
		<script>
		document.documentElement.dataset.ready = "true";
		</script>
		<style>
		.items {
			display: grid;
		}
		</style>
		<pre is:raw>{title}</pre>
	</body>
</html>
"#;

#[tokio::test]
async fn formatting_preserves_complete_astro_document() -> Result<()> {
    let uri = uri!("stability.astro");
    let fs = MemoryFileSystem::default();
    fs.insert(
        to_utf8_file_path_buf(uri!("biome.json")),
        r#"{
            "html": {
                "experimentalFullSupportEnabled": true,
                "formatter": { "enabled": true }
            },
            "linter": { "enabled": false },
            "assist": { "enabled": false }
        }"#,
    );

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);
    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;
    server
        .open_named_document(INPUT, uri.clone(), "astro")
        .await?;

    let edits: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
            },
        )
        .await?
        .context("formatting returned None")?;

    let changes = edits
        .context("formatting did not return edits")?
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
        INPUT.to_string(),
        changes,
    );

    assert_eq!(output, EXPECTED);

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[expect(deprecated)]
async fn initialize_multi_project_with_resolve(server: &mut Server) -> Result<()> {
    let _: InitializeResult = server
        .request(
            "initialize",
            "initialize",
            InitializeParams {
                process_id: None,
                root_path: None,
                root_uri: None,
                initialization_options: None,
                capabilities: ClientCapabilities {
                    text_document: Some(TextDocumentClientCapabilities {
                        code_action: Some(CodeActionClientCapabilities {
                            resolve_support: Some(CodeActionCapabilityResolveSupport {
                                properties: vec!["edit".to_string()],
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                trace: None,
                workspace_folders: Some(vec![
                    WorkspaceFolder {
                        name: "package-a".to_string(),
                        uri: uri!("package-a"),
                    },
                    WorkspaceFolder {
                        name: "package-b".to_string(),
                        uri: uri!("package-b"),
                    },
                ]),
                client_info: None,
                locale: None,
                work_done_progress_params: WorkDoneProgressParams::default(),
            },
        )
        .await?
        .context("initialize returned None")?;
    Ok(())
}

async fn request_code_actions(
    server: &mut Server,
    uri: &Uri,
    range: Range,
    only: Option<Vec<CodeActionKind>>,
) -> Result<CodeActionResponse> {
    server
        .request(
            "textDocument/codeAction",
            "code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                range,
                context: CodeActionContext {
                    diagnostics: Vec::new(),
                    only,
                    trigger_kind: None,
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
            },
        )
        .await?
        .context("codeAction returned None")
}

#[tokio::test]
async fn formatting_preserves_astro_after_resolving_fix_all_in_multi_project_workspace()
-> Result<()> {
    let uri = uri!("package-a/index.astro");
    let input = "---\nconst foo= 1;\n---\n<p>{foo}</p>\n";
    let expected = "---\nconst foo = 1;\n---\n\n<p>{foo}</p>\n";
    let fs = MemoryFileSystem::default();
    fs.insert(
        to_utf8_file_path_buf(uri!("biome.json")),
        r#"{
            "formatter": { "enabled": true, "indentStyle": "space", "indentWidth": 2 },
            "html": {
                "experimentalFullSupportEnabled": true,
                "formatter": { "enabled": true }
            }
        }"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("package-a/biome.json")),
        r#"{
            "html": {
                "experimentalFullSupportEnabled": true,
                "formatter": { "enabled": true }
            }
        }"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("package-b/biome.json")),
        r#"{
            "formatter": { "enabled": false },
            "html": {
                "experimentalFullSupportEnabled": true,
                "formatter": { "enabled": false }
            }
        }"#,
    );
    fs.insert(to_utf8_file_path_buf(uri!("package-a/index.astro")), input);
    fs.insert(to_utf8_file_path_buf(uri!("package-b/index.astro")), input);

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);
    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    initialize_multi_project_with_resolve(&mut server).await?;
    server.initialized().await?;
    server.load_configuration().await?;
    server
        .open_named_document(input, uri!("package-b/index.astro"), "astro")
        .await?;
    server
        .open_named_document(input, uri.clone(), "astro")
        .await?;

    let package_b_edits: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting_disabled_in_package_b",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("package-b/index.astro"),
                },
                options: FormattingOptions {
                    tab_size: 2,
                    insert_spaces: true,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: Some(true),
                    insert_final_newline: Some(true),
                    trim_final_newlines: Some(true),
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
            },
        )
        .await?
        .context("package-b formatting returned None")?;
    assert!(
        package_b_edits.is_none(),
        "package-b formatting should be disabled: {package_b_edits:?}"
    );

    let _ = request_code_actions(
        &mut server,
        &uri,
        Range::new(Position::new(0, 0), Position::new(0, 0)),
        None,
    )
    .await?;
    let _ = request_code_actions(
        &mut server,
        &uri,
        Range::new(Position::new(4, 0), Position::new(4, 0)),
        None,
    )
    .await?;

    server
        .notify(
            "textDocument/didChange",
            DidChangeTextDocumentParams {
                text_document: VersionedTextDocumentIdentifier {
                    uri: uri.clone(),
                    version: 1,
                },
                content_changes: vec![TextDocumentContentChangeEvent {
                    range: Some(Range::new(Position::new(4, 0), Position::new(4, 0))),
                    range_length: None,
                    text: " ".to_string(),
                }],
            },
        )
        .await?;
    let _ = request_code_actions(
        &mut server,
        &uri,
        Range::new(Position::new(4, 1), Position::new(4, 1)),
        None,
    )
    .await?;
    server
        .notify(
            "textDocument/didChange",
            DidChangeTextDocumentParams {
                text_document: VersionedTextDocumentIdentifier {
                    uri: uri.clone(),
                    version: 2,
                },
                content_changes: vec![TextDocumentContentChangeEvent {
                    range: Some(Range::new(Position::new(4, 0), Position::new(4, 1))),
                    range_length: None,
                    text: String::new(),
                }],
            },
        )
        .await?;

    let actions = request_code_actions(
        &mut server,
        &uri,
        Range::new(Position::new(0, 0), Position::new(4, 0)),
        Some(vec![CodeActionKind::new("source.fixAll.biome")]),
    )
    .await?;
    let CodeActionOrCommand::CodeAction(action) = actions
        .into_iter()
        .next()
        .context("fix-all action was not returned")?
    else {
        unreachable!("fix-all must be a code action");
    };
    let resolved: tower_lsp_server::ls_types::CodeAction = server
        .request("codeAction/resolve", "resolve_fix_all", action)
        .await?
        .context("codeAction/resolve returned None")?;
    assert!(resolved.edit.is_none());

    let edits: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier { uri },
                options: FormattingOptions {
                    tab_size: 2,
                    insert_spaces: true,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: Some(true),
                    insert_final_newline: Some(true),
                    trim_final_newlines: Some(true),
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
            },
        )
        .await?
        .context("formatting returned None")?;
    let changes = edits
        .context("formatting did not return edits")?
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

    assert_eq!(output, expected);

    server.shutdown().await?;
    reader.abort();

    Ok(())
}
