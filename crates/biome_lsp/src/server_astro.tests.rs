use crate::server_test_utils::*;
use anyhow::{Context, Result};
use biome_fs::MemoryFileSystem;
use futures::channel::mpsc::channel;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tower_lsp_server::ls_types::{
    self as lsp, ClientCapabilities, DocumentFormattingParams, FormattingOptions,
    TextDocumentContentChangeEvent, TextDocumentIdentifier, TextEdit, WorkDoneProgressParams,
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
