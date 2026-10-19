use crate::server_test_utils::*;
use anyhow::{Context, Result};
use biome_fs::TemporaryFs;
use biome_service::{Watcher, WatcherOptions};
use futures::channel::mpsc::channel;
use std::time::Duration;
use tokio::task::spawn_blocking;
use tokio::time::sleep;
use tower_lsp_server::ls_types::{
    DocumentFormattingParams, FormattingOptions, TextDocumentIdentifier, TextEdit, Uri,
};

#[tokio::test]
async fn watcher_reindex_preserves_open_document_formatting() -> Result<()> {
    const CLIENT_CONTENT: &str = "const veryLongClientName = { a: 1 };\n";
    const DISK_CONTENT: &str = "const x={a:1}\n";

    let mut fs = TemporaryFs::new("watcher_reindex_preserves_open_document_formatting");
    fs.create_file(
        "biome.json",
        r#"{"linter":{"domains":{"project":"recommended"}}}"#,
    );
    let path = fs.create_file("document.js", CLIENT_CONTENT);
    let uri = Uri::from_file_path(path.as_std_path()).unwrap();
    let root_uri = Uri::from_file_path(fs.working_directory.as_std_path()).unwrap();

    let (watcher, instruction_channel) = Watcher::new(WatcherOptions::default())?;
    let mut factory = ServerFactory::new(true, instruction_channel.sender.clone());
    let workspace = factory.workspace();
    let db_state = factory.db_state();
    let watcher_task = spawn_blocking(move || workspace.start_watcher(&db_state, watcher));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);
    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));
    let notifications = tokio::spawn(async move {
        use futures::StreamExt;
        while receiver.next().await.is_some() {}
    });

    server.initialize_with_root(root_uri).await?;
    server.initialized().await?;
    server
        .open_named_document(CLIENT_CONTENT, uri.clone(), "javascript")
        .await?;

    let params = DocumentFormattingParams {
        text_document: TextDocumentIdentifier { uri },
        options: FormattingOptions {
            tab_size: 2,
            insert_spaces: true,
            ..Default::default()
        },
        work_done_progress_params: Default::default(),
    };
    let before: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "before_disk_write",
            params.clone(),
        )
        .await?
        .context("formatting returned no response")?;
    assert_eq!(before, Some(Vec::new()));

    clear_notifications!(factory.service_rx);
    // An external write does not send didChange or close the client's document.
    std::fs::write(&path, DISK_CONTENT)?;
    await_notification!(factory.service_rx);

    let after: Option<Vec<TextEdit>> = server
        .request("textDocument/formatting", "after_disk_write", params)
        .await?
        .context("formatting returned no response")?;
    assert_eq!(after, Some(Vec::new()));

    server.shutdown().await?;
    reader.abort();
    notifications.abort();
    drop(instruction_channel);
    watcher_task.await?;

    Ok(())
}
