use biome_frameworks::{DeclarationKind, Facet, FrameworkContributions, LOADERS, load_frameworks};
use biome_package::{Manifest, PackageJson};
use camino::Utf8Path;
use std::collections::BTreeSet;
use support::TestHost;

mod support;

/// The files of an Astro project that uses Starlight and a Starlight plugin,
/// without configuration and without content files.
const PROJECT: &[(&str, &str)] = &[
    (
        "/project/package.json",
        r#"{ "dependencies": { "astro": "^7.0.0", "@astrojs/starlight": "^0.42.0" } }"#,
    ),
    ("/project/src/styles/index.css", ""),
    (
        "/project/node_modules/astro/package.json",
        r#"{ "name": "astro", "version": "7.4.2" }"#,
    ),
    ("/project/node_modules/astro/client.d.ts", ""),
    (
        "/project/node_modules/@astrojs/react/package.json",
        r#"{ "name": "@astrojs/react", "version": "7.1.1", "exports": { ".": "./dist/index.js" } }"#,
    ),
    (
        "/project/node_modules/@astrojs/starlight/package.json",
        r#"{ "name": "@astrojs/starlight", "version": "0.42.7" }"#,
    ),
    (
        "/project/node_modules/@astrojs/starlight/dist/style/props.css",
        "",
    ),
    (
        "/project/node_modules/@astrojs/starlight/dist/style/reset.css",
        "",
    ),
    (
        "/project/node_modules/@astrojs/starlight/dist/components/PageFrame.astro",
        "",
    ),
    (
        "/project/node_modules/@astrojs/starlight/dist/user-components/Badge.astro",
        "",
    ),
    (
        "/project/node_modules/starlight-blog/package.json",
        r#"{ "name": "starlight-blog", "version": "0.30.2", "exports": { "./styles": "./styles.css" } }"#,
    ),
    ("/project/node_modules/starlight-blog/styles.css", ""),
];

/// The files of [PROJECT] that the current layout of Starlight has.
const STARLIGHT_DIST_FILES: &[&str] = &[
    "/project/node_modules/@astrojs/starlight/dist/style/props.css",
    "/project/node_modules/@astrojs/starlight/dist/style/reset.css",
    "/project/node_modules/@astrojs/starlight/dist/components/PageFrame.astro",
    "/project/node_modules/@astrojs/starlight/dist/user-components/Badge.astro",
];

/// Files as `(path, content)` pairs.
type Files = &'static [(&'static str, &'static str)];

const STARLIGHT_CONFIG: &str = r#"import starlight from "@astrojs/starlight";
export default { integrations: [starlight()] };
"#;

/// Loads the frameworks of [PROJECT] with the configuration `config`, after
/// removing the files at `removed` and adding the files of `added`. An empty
/// `config` stands for a project without configuration file.
fn load(config: &str, removed: &[&str], added: &[(&str, &str)]) -> FrameworkContributions {
    let mut files: Vec<_> = PROJECT
        .iter()
        .copied()
        .filter(|(path, _)| !removed.contains(path))
        .collect();
    if !config.is_empty() {
        files.push(("/project/astro.config.ts", config));
    }
    files.extend_from_slice(added);
    let host = TestHost::new(&files);
    let manifest = PackageJson::read_manifest(&host.0, Utf8Path::new("/project/package.json"))
        .into_deserialized()
        .expect("the package manifest must be valid");
    load_frameworks(&host, LOADERS, Utf8Path::new("/project"), &manifest)
}

fn visits(contributions: &FrameworkContributions) -> Vec<&str> {
    contributions
        .visits
        .iter()
        .map(|path| path.as_str())
        .collect()
}

fn declared_properties(contributions: &FrameworkContributions) -> Vec<&str> {
    contributions
        .declarations
        .iter()
        .filter(|declaration| declaration.kind == DeclarationKind::CssCustomProperty)
        .map(|declaration| declaration.name.text())
        .collect()
}

// #region ASTRO

#[test]
fn loads_astro_starlight_and_their_integrations() {
    let contributions = load(
        r#"import react from "@astrojs/react";
import starlight from "@astrojs/starlight";
import { defineConfig, fontProviders } from "astro/config";
import starlightBlog from "starlight-blog";

const plugins = [starlightBlog({ title: "Blog" })];

function lunariaDashboard() {
    return { name: "lunaria-dashboard", hooks: {} };
}

export default defineConfig({
    fonts: [
        {
            name: "Geist",
            cssVariable: "--font-geist",
            provider: fontProviders.fontsource(),
        },
    ],
    integrations: [
        react(),
        starlight({ title: "Docs", plugins, customCss: ["./src/styles/index.css"] }),
        lunariaDashboard(),
    ],
});
"#,
        &[],
        &[("/project/src/content/docs/index.mdx", "")],
    );

    assert_eq!(
        visits(&contributions),
        [
            "/project/node_modules/astro/client.d.ts",
            "/project/node_modules/@astrojs/starlight/dist/style/props.css",
            "/project/node_modules/@astrojs/starlight/dist/style/reset.css",
            "/project/node_modules/@astrojs/starlight/dist/components/PageFrame.astro",
            "/project/node_modules/@astrojs/starlight/dist/user-components/Badge.astro",
            "/project/src/styles/index.css",
            "/project/node_modules/starlight-blog/styles.css",
        ]
    );
    assert_eq!(declared_properties(&contributions), ["--font-geist"]);
    // The MDX page has class attributes that Biome can't read.
    assert_eq!(
        contributions.incomplete,
        BTreeSet::from([Facet::ClassUsages])
    );
}

#[test]
fn astro_without_configuration_visits_its_module_declarations() {
    let contributions = load("", &[], &[]);

    assert_eq!(
        visits(&contributions),
        ["/project/node_modules/astro/client.d.ts"]
    );
    assert!(contributions.incomplete.is_empty());
}

#[test]
fn missing_module_declarations_make_modules_incomplete() {
    let contributions = load("", &["/project/node_modules/astro/client.d.ts"], &[]);

    assert_eq!(contributions.incomplete, BTreeSet::from([Facet::Modules]));
}

#[test]
fn experimental_fonts_declare_custom_properties() {
    let contributions = load(
        r#"export default { experimental: { fonts: [{ cssVariable: "--font-body" }] } };"#,
        &[],
        &[],
    );

    assert_eq!(declared_properties(&contributions), ["--font-body"]);
    assert!(contributions.incomplete.is_empty());
}

#[test]
fn fonts_that_cant_be_known_make_styles_incomplete() {
    for config in [
        r#"import { fonts } from "./fonts.mjs";
export default { fonts };
"#,
        r#"import { variable } from "./fonts.mjs";
export default { fonts: [{ cssVariable: variable }] };
"#,
    ] {
        let contributions = load(config, &[], &[]);
        assert_eq!(
            contributions.incomplete,
            BTreeSet::from([Facet::Styles]),
            "{config}"
        );
    }
}

#[test]
fn nested_integration_arrays_are_flattened() {
    let contributions = load(
        r#"import starlight from "@astrojs/starlight";
export default { integrations: [[starlight()]] };
"#,
        &[],
        &[],
    );

    assert!(
        visits(&contributions)
            .contains(&"/project/node_modules/@astrojs/starlight/dist/style/props.css")
    );
}

#[test]
fn content_files_of_the_source_directory_make_class_usages_incomplete() {
    let cases: [(&str, Files, bool); 5] = [
        // Markdoc files use the `.mdoc` extension.
        ("", &[("/project/src/content/docs/guide.mdoc", "")], true),
        // `.markdown` is one of the Markdown extensions of Astro.
        ("", &[("/project/src/pages/post.markdown", "")], true),
        // `srcDir` moves the source directory.
        (
            r#"export default { srcDir: "./www" };"#,
            &[("/project/www/pages/post.md", "")],
            true,
        ),
        (
            r#"export default { srcDir: "./www" };"#,
            &[("/project/src/pages/post.md", "")],
            false,
        ),
        // The source directory can't be known.
        (
            r#"import { srcDir } from "./paths.mjs";
export default { srcDir };
"#,
            &[],
            true,
        ),
    ];

    for (config, added, is_incomplete) in cases {
        let contributions = load(config, &[], added);
        assert_eq!(
            contributions.incomplete.contains(&Facet::ClassUsages),
            is_incomplete,
            "{config} {added:?}"
        );
    }
}

// #endregion

// #region STARLIGHT

#[test]
fn starlight_files_of_an_older_layout_are_visited() {
    let contributions = load(
        STARLIGHT_CONFIG,
        STARLIGHT_DIST_FILES,
        &[
            (
                "/project/node_modules/@astrojs/starlight/style/props.css",
                "",
            ),
            (
                "/project/node_modules/@astrojs/starlight/components/Page.astro",
                "",
            ),
        ],
    );

    assert_eq!(
        visits(&contributions),
        [
            "/project/node_modules/astro/client.d.ts",
            "/project/node_modules/@astrojs/starlight/style/props.css",
            "/project/node_modules/@astrojs/starlight/components/Page.astro",
        ]
    );
    assert!(contributions.incomplete.is_empty());
}

#[test]
fn starlight_with_an_unknown_layout_is_incomplete() {
    let contributions = load(STARLIGHT_CONFIG, STARLIGHT_DIST_FILES, &[]);

    assert_eq!(
        contributions.incomplete,
        BTreeSet::from([Facet::Styles, Facet::ClassUsages])
    );
}

#[test]
fn custom_stylesheets_that_cant_be_known_make_styles_incomplete() {
    for custom_css in [
        // The value can't be known.
        "stylesheets",
        // The file doesn't exist.
        r#"["./src/missing.css"]"#,
        // An element can't be known.
        r#"["./src/styles/index.css", ...stylesheets]"#,
    ] {
        let config = format!(
            r#"import starlight from "@astrojs/starlight";
import {{ stylesheets }} from "./stylesheets.mjs";
export default {{ integrations: [starlight({{ customCss: {custom_css} }})] }};
"#
        );
        let contributions = load(&config, &[], &[]);
        assert_eq!(
            contributions.incomplete,
            BTreeSet::from([Facet::Styles]),
            "{custom_css}"
        );
    }
}

// #endregion
