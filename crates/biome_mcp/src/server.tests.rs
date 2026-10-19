use crate::server::INSTRUCTIONS;
use crate::server_test_utils::{
    TestServer, assert_tool_error_snapshot, assert_tool_snapshot, text_of,
};
use biome_fs::TemporaryFs;

/// A small React and TypeScript project. `biome.json` excludes the
/// `generated` folder, `node_modules` holds a dependency, and
/// `vendor.min.js` is a single long line.
const FILES: &[(&str, &str)] = &[
    (
        r#"biome.json"#,
        r#"{ "files": { "includes": ["**", "!**/generated"] } }"#,
    ),
    (
        r#"package.json"#,
        r#"{ "name": "app", "version": "1.0.0", "dependencies": { "react": "^19.0.0" } }
"#,
    ),
    (
        r#"src/App.tsx"#,
        r#"import { useEffect, useState } from "react";
import { formatDate } from "./utils/date";

export function App({ userId }: { userId: string }) {
	const [user, setUser] = useState(null);

	useEffect(() => {
		fetchUser(userId).then((data) => {
			console.log("fetched", data);
			setUser(data);
		});
	}, [userId]);

	return <p>{formatDate(user?.createdAt)}</p>;
}
"#,
    ),
    (
        r#"src/utils/date.ts"#,
        r#"export function formatDate(value: unknown): string {
	const date = value as any;
	console.log(date);
	return date.toISOString();
}
"#,
    ),
    (
        r#"src/styles/app.css"#,
        r#".title {
	color: red;
}

.subtitle {
	color: blue;
}
"#,
    ),
    (
        r#"src/vendor.min.js"#,
        r#"!function(){var v0=0,v1=1,v2=2,v3=3,v4=4,v5=5,v6=6,v7=7,v8=8,v9=9,v10=10,v11=11,v12=12,v13=13,v14=14,v15=15,v16=16,v17=17,v18=18,v19=19,v20=20,v21=21,v22=22,v23=23,v24=24,v25=25,v26=26,v27=27,v28=28,v29=29,v30=30,v31=31,v32=32,v33=33,v34=34,v35=35,v36=36,v37=37,v38=38,v39=39;console.log(v1,v2);var total=v0+v1+v2+v3+v4+v5+v6+v7+v8+v9+v10+v11+v12+v13+v14+v15+v16+v17+v18+v19+v20+v21+v22+v23+v24+v25+v26+v27+v28+v29+v30+v31+v32+v33+v34+v35+v36+v37+v38+v39+v40+v41+v42+v43+v44+v45+v46+v47+v48+v49+v50+v51+v52+v53+v54+v55+v56+v57+v58+v59;return total}();
"#,
    ),
    (
        r#"src/generated/client.js"#,
        r#"console.log("generated");
"#,
    ),
    (
        r#"node_modules/react/index.js"#,
        r#"console.log("react");
"#,
    ),
];

#[tokio::test]
async fn describes_itself_and_lists_its_tools() {
    let server = TestServer::new(FILES).await;

    let info = server.server_info();
    let tools = server.list_tools().await;
    server.shutdown().await;

    let server_info = info
        .server_info
        .as_ref()
        .expect("the server should name itself");
    assert_eq!(server_info.name, "biome");
    assert_eq!(server_info.version, biome_configuration::VERSION);
    assert_eq!(info.instructions.as_deref(), Some(INSTRUCTIONS));
    let names: Vec<_> = tools.iter().map(|tool| tool.name.as_ref()).collect();
    assert_eq!(names, ["search"]);
}

#[tokio::test]
async fn reports_matches_in_each_output() {
    let server = TestServer::new(FILES).await;
    assert_tool_snapshot(
        "reports_matches_in_each_output",
        &server,
        "search",
        &[
            r#"{"pattern": "`console.log($arguments)`"}"#,
            r#"{"pattern": "`$function($arguments)`", "output": "files"}"#,
            r#"{"pattern": "`useEffect($callback, $dependencies)`", "output": "matches"}"#,
            r#"{"pattern": "`useEffect($callback, $dependencies)`", "output": "captures"}"#,
            r#"{"pattern": "`useState(null)`", "output": "captures"}"#,
        ],
    )
    .await;
    server.shutdown().await;
}

#[tokio::test]
async fn searches_css_and_json_files() {
    let server = TestServer::new(FILES).await;
    assert_tool_snapshot(
        "searches_css_and_json_files",
        &server,
        "search",
        &[
            r#"{"pattern": "`color: $value`", "language": "css"}"#,
            r#"{"pattern": "JsonMember(name = $name, value = $range) where { $name <: r\".react.\" }", "language": "json"}"#,
        ],
    )
    .await;
    server.shutdown().await;
}

#[tokio::test]
async fn searches_only_the_given_paths() {
    let server = TestServer::new(FILES).await;
    assert_tool_snapshot(
        "searches_only_the_given_paths",
        &server,
        "search",
        &[
            r#"{"pattern": "`console.log($arguments)`", "paths": ["src/utils"]}"#,
            r#"{"pattern": "`console.log($arguments)`", "paths": ["src/styles"]}"#,
        ],
    )
    .await;
    server.shutdown().await;
}

#[tokio::test]
async fn reports_searches_without_matches() {
    let server = TestServer::new(FILES).await;
    assert_tool_snapshot(
        "reports_searches_without_matches",
        &server,
        "search",
        &[r#"{"pattern": "`alert($message)`"}"#],
    )
    .await;
    server.shutdown().await;
}

#[tokio::test]
async fn reports_at_most_max_matches() {
    let server = TestServer::new(FILES).await;
    assert_tool_snapshot(
        "reports_at_most_max_matches",
        &server,
        "search",
        &[
            r#"{"pattern": "`console.log($arguments)`", "maxMatches": 1}"#,
            r#"{"pattern": "`console.log($arguments)`", "output": "files", "maxMatches": 2}"#,
        ],
    )
    .await;
    server.shutdown().await;
}

#[tokio::test]
async fn reports_a_thousand_matches_unless_asked_for_more() {
    let calls = "f();\n".repeat(1001);
    let server = TestServer::new(&[("src/calls.js", &calls)]).await;

    let default = text_of(&server.call_tool("search", r#"{"pattern": "`f()`"}"#).await);
    let raised = text_of(
        &server
            .call_tool("search", r#"{"pattern": "`f()`", "maxMatches": 1001}"#)
            .await,
    );
    server.shutdown().await;

    assert!(
        default.ends_with(
            "Found 1001 matches in 1 file (1 searched).\nShowing the first 1000 matches. Narrow the pattern or the paths, or raise `maxMatches`, to see the rest."
        ),
        "{}",
        default.lines().rev().take(2).collect::<Vec<_>>().join("\n")
    );
    assert!(
        raised.contains("\n1001:1: f();\n"),
        "the last match is missing"
    );
    assert!(
        raised.ends_with("Found 1001 matches in 1 file (1 searched)."),
        "{}",
        raised.lines().last().unwrap_or_default()
    );
}

#[tokio::test]
async fn rejects_invalid_arguments() {
    let server = TestServer::new(FILES).await;
    assert_tool_error_snapshot(
        "rejects_invalid_arguments",
        &server,
        "search",
        &[
            r#"{"pattern": "`console.log("}"#,
            r#"{"pattern": "`console.log($arguments)`", "paths": ["../other"]}"#,
            r#"{"pattern": "`console.log($arguments)`", "paths": ["src/missing.ts"]}"#,
        ],
    )
    .await;
    server.shutdown().await;
}

#[tokio::test]
async fn sees_edits_between_calls() {
    let mut fs = TemporaryFs::new("biome_mcp_sees_edits_between_calls");
    fs.create_file(
        "package.json",
        r#"{ "name": "app", "dependencies": { "react": "^19.0.0" } }"#,
    );
    fs.create_file("src/date.ts", "console.log(date);\n");
    let mut server = TestServer::new_on_disk(&fs).await;
    let manifest_search = r#"{"pattern": "JsonMember(name = $name, value = $range) where { $name <: r\".react.\" }", "language": "json", "output": "captures"}"#;
    let source_search = r#"{"pattern": "`console.log($arguments)`", "output": "captures"}"#;

    let manifest_before = text_of(&server.call_tool("search", manifest_search).await);
    let source_before = text_of(&server.call_tool("search", source_search).await);
    let root = fs.working_directory.clone();
    server
        .edit_and_wait(|| {
            std::fs::write(
                root.join("package.json"),
                r#"{ "name": "app", "dependencies": { "react": "^20.0.0" } }"#,
            )
            .expect("package.json should be writable");
        })
        .await;
    server
        .edit_and_wait(|| {
            std::fs::write(root.join("src/date.ts"), "console.log(\"edited\");\n")
                .expect("date.ts should be writable");
        })
        .await;
    let manifest_after = text_of(&server.call_tool("search", manifest_search).await);
    let source_after = text_of(&server.call_tool("search", source_search).await);
    server
        .edit_and_wait(|| {
            std::fs::remove_file(root.join("src/date.ts")).expect("date.ts should be removable");
        })
        .await;
    let source_deleted = text_of(&server.call_tool("search", source_search).await);
    server
        .edit_and_wait(|| {
            std::fs::write(root.join("src/added.ts"), "console.log(added);\n")
                .expect("added.ts should be writable");
        })
        .await;
    let source_added = text_of(&server.call_tool("search", source_search).await);
    server.shutdown().await;

    assert!(
        manifest_before.contains(r#"$range = "^19.0.0""#),
        "{manifest_before}"
    );
    assert!(
        manifest_after.contains(r#"$range = "^20.0.0""#),
        "{manifest_after}"
    );
    assert!(
        source_before.contains("$arguments = date"),
        "{source_before}"
    );
    assert!(
        source_after.contains(r#"$arguments = "edited""#),
        "{source_after}"
    );
    assert!(
        source_deleted.starts_with("No files were searched"),
        "{source_deleted}"
    );
    assert!(
        source_added.starts_with("src/added.ts\n1:1: $arguments = added"),
        "{source_added}"
    );
}

#[tokio::test]
async fn answers_concurrent_calls() {
    let server = TestServer::new(FILES).await;
    let arguments = r#"{"pattern": "`console.log($arguments)`"}"#;

    let (first, second, third) = tokio::join!(
        server.call_tool("search", arguments),
        server.call_tool("search", arguments),
        server.call_tool("search", arguments),
    );
    server.shutdown().await;

    let reports = [text_of(&first), text_of(&second), text_of(&third)];
    assert!(reports[0].contains("Found 3 matches"), "{}", reports[0]);
    assert!(
        reports.iter().all(|report| report == &reports[0]),
        "{reports:#?}"
    );
}

/// Returns [FILES] with the content of `path` replaced by `content`.
fn files_with(path: &'static str, content: &'static str) -> Vec<(&'static str, &'static str)> {
    let mut files: Vec<_> = FILES
        .iter()
        .copied()
        .filter(|(file_path, _)| *file_path != path)
        .collect();
    files.push((path, content));
    files
}

#[tokio::test]
async fn stops_on_configuration_errors_and_resumes_once_fixed() {
    let server = TestServer::new(&files_with(
        "biome.json",
        r#"{ "files": { "includes": 42 } }"#,
    ))
    .await;
    assert_tool_error_snapshot(
        "stops_on_configuration_errors",
        &server,
        "search",
        &[r#"{"pattern": "`console.log($arguments)`"}"#],
    )
    .await;

    server.write_file(
        "biome.json",
        r#"{ "files": { "includes": ["**", "!**/generated"] } }"#,
    );
    let resumed = server
        .call_tool("search", r#"{"pattern": "`console.log($arguments)`"}"#)
        .await;
    server.shutdown().await;

    assert_ne!(resumed.is_error, Some(true), "{}", text_of(&resumed));
    assert!(
        text_of(&resumed).contains("Found 3 matches"),
        "{}",
        text_of(&resumed)
    );
}

#[tokio::test]
async fn stops_on_nested_configuration_errors_and_resumes_once_fixed() {
    let server = TestServer::new(&files_with(
        "src/styles/biome.json",
        r#"{ "root": false, "files": { "includes": 42 } }"#,
    ))
    .await;
    assert_tool_error_snapshot(
        "stops_on_nested_configuration_errors",
        &server,
        "search",
        &[r#"{"pattern": "`console.log($arguments)`"}"#],
    )
    .await;

    server.write_file(
        "src/styles/biome.json",
        r#"{ "root": false, "files": { "includes": ["**"] } }"#,
    );
    let resumed = server
        .call_tool("search", r#"{"pattern": "`console.log($arguments)`"}"#)
        .await;
    server.shutdown().await;

    assert_ne!(resumed.is_error, Some(true), "{}", text_of(&resumed));
    assert!(
        text_of(&resumed).contains("Found 3 matches"),
        "{}",
        text_of(&resumed)
    );
}

#[tokio::test]
async fn applies_nested_configuration_changes() {
    let mut fs = TemporaryFs::new("biome_mcp_applies_nested_configuration_changes");
    fs.create_file("biome.json", "{}");
    fs.create_file("lib/util.ts", "console.log(util);\n");
    fs.create_file("lib/util.test.ts", "console.log(test);\n");
    let mut server = TestServer::new_on_disk(&fs).await;
    let arguments = r#"{"pattern": "`console.log($arguments)`", "output": "files"}"#;
    let root = fs.working_directory.clone();

    let before = text_of(&server.call_tool("search", arguments).await);
    server
        .edit_and_wait(|| {
            std::fs::write(
                root.join("lib/biome.json"),
                r#"{ "root": false, "files": { "includes": ["**", "!**/*.test.ts"] } }"#,
            )
            .expect("lib/biome.json should be writable");
        })
        .await;
    let added = text_of(&server.call_tool("search", arguments).await);
    server
        .edit_and_wait(|| {
            std::fs::write(
                root.join("lib/biome.json"),
                r#"{ "root": false, "files": { "includes": ["**", "!**/util.ts"] } }"#,
            )
            .expect("lib/biome.json should be writable");
        })
        .await;
    let changed = text_of(&server.call_tool("search", arguments).await);
    server.shutdown().await;

    assert!(before.contains("Found 2 matches"), "{before}");
    assert!(
        added.contains("lib/util.ts (1)") && !added.contains("lib/util.test.ts"),
        "{added}"
    );
    assert!(
        changed.contains("lib/util.test.ts (1)") && !changed.contains("lib/util.ts"),
        "{changed}"
    );
}

#[tokio::test]
async fn searches_the_root_with_the_user_configuration() {
    let files: Vec<_> = FILES
        .iter()
        .copied()
        .filter(|(path, _)| *path != "biome.json")
        .collect();
    let server = TestServer::new_with_user_configuration(
        &files,
        r#"{ "files": { "includes": ["**", "!**/generated"] } }"#,
    )
    .await;

    let report = text_of(
        &server
            .call_tool("search", r#"{"pattern": "`console.log($arguments)`"}"#)
            .await,
    );
    server.shutdown().await;

    assert!(
        report.contains("Found 3 matches in 3 files (3 searched)."),
        "{report}"
    );
}

#[tokio::test]
async fn starts_without_a_project_and_reports_why() {
    let server = TestServer::new_without_project(
        "Biome can't read the working directory: No such file or directory (os error 2)",
    )
    .await;
    let tools = server.list_tools().await;
    assert_eq!(tools.len(), 1);
    assert_tool_error_snapshot(
        "reports_why_the_project_is_missing",
        &server,
        "search",
        &[r#"{"pattern": "`console.log($arguments)`"}"#],
    )
    .await;
    server.shutdown().await;
}

#[tokio::test]
async fn applies_configuration_changes() {
    let server = TestServer::new(FILES).await;
    let arguments = r#"{"pattern": "`console.log($arguments)`"}"#;

    let before = text_of(&server.call_tool("search", arguments).await);
    server.write_file(
        "biome.json",
        r#"{ "files": { "includes": ["**", "!**/generated", "!src/utils"] } }"#,
    );
    let after = text_of(&server.call_tool("search", arguments).await);
    server.shutdown().await;

    assert!(before.contains("src/utils/date.ts"), "{before}");
    assert!(!after.contains("src/utils/date.ts"), "{after}");
    assert!(after.contains("Found 2 matches"), "{after}");
}
