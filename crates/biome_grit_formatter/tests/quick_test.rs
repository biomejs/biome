use biome_fs::{BiomePath, MemoryFileSystem};
use biome_service::workspace::{
    ChangeFileParams, FileContent, FormatFileParams, GetFormatterIRParams, OpenFileParams,
    OpenProjectParams, server,
};
use std::sync::Arc;

#[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"function lines($string) {
    return split($string, separator=`\n`)
}

"#;
    let fs = MemoryFileSystem::default();
    let workspace = server(Arc::new(fs), None);

    let project = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new(""),
            open_uninitialized: true,
        })
        .unwrap();

    let path = BiomePath::new("test.grit");
    workspace
        .open_file(OpenFileParams {
            project_key: project.project_key,
            path: path.clone(),
            content: FileContent::FromClient {
                content: src.to_string(),
                version: 0,
            },
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
        })
        .unwrap();

    // Print IR
    if let Ok(ir) = workspace.get_formatter_ir(GetFormatterIRParams {
        project_key: project.project_key,
        path: path.clone(),
    }) {
        println!("{ir}");
    }

    let printed = workspace
        .format_file(FormatFileParams {
            project_key: project.project_key,
            path: path.clone(),
            inline_config: None,
        })
        .unwrap();

    eprintln!("{}", printed.as_code());

    // Idempotency check
    workspace
        .change_file(ChangeFileParams {
            project_key: project.project_key,
            path: path.clone(),
            content: printed.as_code().to_string(),
            version: 1,
            inline_config: None,
        })
        .unwrap();

    let re_printed = workspace
        .format_file(FormatFileParams {
            project_key: project.project_key,
            path: path.clone(),
            inline_config: None,
        })
        .unwrap();

    similar_asserts::assert_eq!(
        re_printed.as_code(),
        printed.as_code(),
        "Formatter is not idempotent"
    );
}
