use crate::WorkspaceSettings;
use crate::server_test_utils::*;
use anyhow::{Context, Result};
use biome_configuration::{Configuration, FormatterConfiguration};
use futures::channel::mpsc::channel;
use std::collections::HashMap;
use std::str::FromStr;
use tower_lsp_server::ls_types::{
    self as lsp, DocumentOnTypeFormattingParams, FormattingOptions, Position,
    TextDocumentIdentifier, TextDocumentPositionParams, TextEdit, Uri,
};

fn position_after(source: &str, needle: &str) -> Position {
    let offset = source.find(needle).expect("needle should exist") + needle.len();
    let prefix = &source[..offset];
    let line = prefix.bytes().filter(|byte| *byte == b'\n').count() as u32;
    let character = prefix.rsplit('\n').next().unwrap_or(prefix).chars().count() as u32;

    Position::new(line, character)
}

fn formatting_options() -> FormattingOptions {
    FormattingOptions {
        tab_size: 2,
        insert_spaces: true,
        properties: HashMap::default(),
        trim_trailing_whitespace: None,
        insert_final_newline: None,
        trim_final_newlines: None,
    }
}

async fn request_on_type_formatting(
    server: &mut Server,
    uri: Uri,
    position: Position,
    ch: &str,
) -> Result<Option<Vec<TextEdit>>> {
    server
        .request(
            "textDocument/onTypeFormatting",
            "on_type_formatting",
            DocumentOnTypeFormattingParams {
                text_document_position: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri },
                    position,
                },
                ch: ch.to_string(),
                options: formatting_options(),
            },
        )
        .await?
        .context("on type formatting returned no response")
}

fn assert_no_edits(edits: Option<Vec<TextEdit>>) {
    assert_eq!(edits.unwrap_or_default(), Vec::<TextEdit>::new());
}

#[tokio::test]
async fn on_type_formatting_ignores_closing_bracket_inside_string() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .load_configuration_with_settings(WorkspaceSettings {
            inline_config: Some(Configuration {
                formatter: Some(FormatterConfiguration {
                    format_with_errors: Some(true.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        })
        .await?;

    let uri = uri!("document.ts");
    let text = "async function main() {\n  const a = 1;\n}\n\nfunction nice() {\n  console.log(\"    [session]\n}\n";
    server
        .open_named_document(text, uri.clone(), "typescript")
        .await?;

    let edits =
        request_on_type_formatting(&mut server, uri, position_after(text, "[session]"), "]")
            .await?;

    assert_no_edits(edits);

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn on_type_formatting_does_not_edit_formatted_function_parameters() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let uri = uri!("document.js");
    let text = "if (condition) {\n  function x() {}\n}\n";
    server
        .open_named_document(text, uri.clone(), "javascript")
        .await?;

    let edits =
        request_on_type_formatting(&mut server, uri, position_after(text, "function x()"), ")")
            .await?;

    assert_no_edits(edits);

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn on_type_formatting_does_not_edit_formatted_array_type() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let uri = uri!("document.ts");
    let text =
        "type MyType = string;\nconst x = [];\nif (condition) {\n  const y = x as MyType[];\n}\n";
    server
        .open_named_document(text, uri.clone(), "typescript")
        .await?;

    let edits =
        request_on_type_formatting(&mut server, uri, position_after(text, "MyType[]"), "]").await?;

    assert_no_edits(edits);

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn on_type_formatting_formats_closing_paren() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let uri = uri!("document.js");
    let text = "foo(1,2)\n";
    server
        .open_named_document(text, uri.clone(), "javascript")
        .await?;

    let edits = request_on_type_formatting(&mut server, uri, position_after(text, "foo(1,2)"), ")")
        .await?
        .context("expected closing paren to return edits")?;

    assert!(
        !edits.is_empty(),
        "expected closing paren to format the call"
    );

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn on_type_formatting_formats_closing_bracket() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let uri = uri!("document.js");
    let text = "const x = [1,2]\n";
    server
        .open_named_document(text, uri.clone(), "javascript")
        .await?;

    let edits = request_on_type_formatting(&mut server, uri, position_after(text, "[1,2]"), "]")
        .await?
        .context("expected closing bracket to return edits")?;

    assert!(
        !edits.is_empty(),
        "expected closing bracket to format the array"
    );

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn on_type_formatting_does_not_edit_formatted_json_array() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let uri = uri!("document.json");
    let text = "{\n\t\"values\": [1, 2]\n}\n";
    server
        .open_named_document(text, uri.clone(), "json")
        .await?;

    let edits =
        request_on_type_formatting(&mut server, uri, position_after(text, "[1, 2]"), "]").await?;

    assert_no_edits(edits);

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn on_type_formatting_formats_json_closing_bracket() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let uri = uri!("document.json");
    let text = "{\"values\":[1,2]}\n";
    server
        .open_named_document(text, uri.clone(), "json")
        .await?;

    let edits = request_on_type_formatting(&mut server, uri, position_after(text, "[1,2]"), "]")
        .await?
        .context("expected JSON closing bracket to return edits")?;

    assert!(
        !edits.is_empty(),
        "expected JSON closing bracket to format the array"
    );

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn on_type_formatting_does_not_edit_formatted_graphql_selection() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let uri = uri!("document.graphql");
    let text = "query {\n\tfield\n}\n";
    server
        .open_named_document(text, uri.clone(), "graphql")
        .await?;

    let edits =
        request_on_type_formatting(&mut server, uri, position_after(text, "\tfield\n}"), "}")
            .await?;

    assert_no_edits(edits);

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn on_type_formatting_formats_graphql_closing_curly() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let uri = uri!("document.graphql");
    let text = "query{field}\n";
    server
        .open_named_document(text, uri.clone(), "graphql")
        .await?;

    let edits = request_on_type_formatting(&mut server, uri, position_after(text, "}"), "}")
        .await?
        .context("expected GraphQL closing curly to return edits")?;

    assert!(
        !edits.is_empty(),
        "expected GraphQL closing curly to format the selection"
    );

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn on_type_formatting_does_not_edit_formatted_css_block() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let uri = uri!("document.css");
    let text = "a {\n\tcolor: red;\n}\n";
    server.open_named_document(text, uri.clone(), "css").await?;

    let edits = request_on_type_formatting(
        &mut server,
        uri,
        position_after(text, "\tcolor: red;\n}"),
        "}",
    )
    .await?;

    assert_no_edits(edits);

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn on_type_formatting_does_not_edit_css_block_with_leading_comment() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let uri = uri!("document.css");
    let text = "a {\n\tcolor: blue;\n}\n\n/* heading */\nb {\n\tcolor: red;\n}\n";
    server.open_named_document(text, uri.clone(), "css").await?;

    let edits = request_on_type_formatting(
        &mut server,
        uri,
        position_after(text, "\tcolor: red;\n}"),
        "}",
    )
    .await?;

    assert_no_edits(edits);

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn on_type_formatting_formats_css_closing_curly() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let uri = uri!("document.css");
    let text = "a{color:red}\n";
    server.open_named_document(text, uri.clone(), "css").await?;

    let edits = request_on_type_formatting(&mut server, uri, position_after(text, "}"), "}")
        .await?
        .context("expected CSS closing curly to return edits")?;

    assert!(
        !edits.is_empty(),
        "expected CSS closing curly to format the block"
    );

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn on_type_formatting_does_not_edit_formatted_grit_definition() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let uri = uri!("document.grit");
    let text = "pattern console_method_to_info($method) {\n\t`console.$method($message)` => `console.info($message)`\n}\n";
    server
        .open_named_document(text, uri.clone(), "grit")
        .await?;

    let edits = request_on_type_formatting(
        &mut server,
        uri,
        position_after(text, "`console.info($message)`\n}"),
        "}",
    )
    .await?;

    assert_no_edits(edits);

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn on_type_formatting_formats_grit_closing_curly() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let uri = uri!("document.grit");
    let text = "pattern console_method_to_info($method) {`console.$method($message)` => `console.info($message)`}\n";
    server
        .open_named_document(text, uri.clone(), "grit")
        .await?;

    let edits = request_on_type_formatting(&mut server, uri, position_after(text, "}"), "}")
        .await?
        .context("expected Grit closing curly to return edits")?;

    assert!(
        !edits.is_empty(),
        "expected Grit closing curly to format the definition"
    );

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn on_type_formatting_still_formats_closing_curly() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let uri = uri!("document.js");
    let text = "if (condition) {\nfoo()\n}\n";
    server
        .open_named_document(text, uri.clone(), "javascript")
        .await?;

    let edits =
        request_on_type_formatting(&mut server, uri, position_after(text, "}"), "}").await?;
    let edits = edits.context("expected closing curly to return edits")?;

    assert!(
        !edits.is_empty(),
        "expected closing curly to format the block"
    );

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn on_type_formatting_does_not_edit_if_statement_with_leading_comments() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let uri = uri!("document.js");
    let text = r#"function test() {
	const num = Math.random();

	// Add a new check
	// TODO: try removing and re-adding closing paren
	if (num < 0.5 && num < 0.4) {
		console.log("Less than 0.5");
	}
}
"#;
    server
        .open_named_document(text, uri.clone(), "javascript")
        .await?;

    let edits = request_on_type_formatting(
        &mut server,
        uri,
        position_after(text, "num < 0.5 && num < 0.4)"),
        ")",
    )
    .await?;

    assert_no_edits(edits);

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn on_type_formatting_does_not_edit_formatted_method_body() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let uri = uri!("document.js");
    let text = "class A {\n\tfoo() {\n\t\treturn 1;\n\t}\n}\n";
    server
        .open_named_document(text, uri.clone(), "javascript")
        .await?;

    let edits = request_on_type_formatting(
        &mut server,
        uri,
        position_after(text, "\t\treturn 1;\n\t}"),
        "}",
    )
    .await?;

    assert_no_edits(edits);

    server.shutdown().await?;
    reader.abort();

    Ok(())
}
