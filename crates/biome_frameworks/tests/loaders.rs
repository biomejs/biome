use biome_frameworks::{
    Activation, ConfigValue, Contribution, Declaration, DeclarationKind, Facet, FrameworkLoader,
    InstalledPackage, LoaderContext, LoaderHost, Origin, ResolutionKind, load_frameworks,
    package_name,
};
use biome_fs::{FileSystem, MemoryFileSystem};
use biome_glob::Glob;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModel, SemanticModelOptions, semantic_model};
use biome_js_syntax::AnyJsRoot;
use biome_languages::JsFileSource;
use biome_package::{Manifest, PackageJson};
use biome_rowan::{TextRange, TextSize};
use camino::{Utf8Path, Utf8PathBuf};
use std::collections::BTreeSet;

// #region TEST HOST AND LOADERS

/// Reads files from memory. Packages are installed in the `node_modules`
/// directory of the package that uses them, and only relative specifiers
/// resolve.
struct TestHost(MemoryFileSystem);

impl TestHost {
    fn new(files: &[(&str, &str)]) -> Self {
        let fs = MemoryFileSystem::default();
        for (path, content) in files {
            fs.insert(Utf8PathBuf::from(*path), *content);
        }
        Self(fs)
    }
}

impl LoaderHost for TestHost {
    fn is_file(&self, path: &Utf8Path) -> bool {
        self.0.path_is_file(path)
    }

    fn js_file(&self, path: &Utf8Path) -> Option<(AnyJsRoot, SemanticModel)> {
        let text = self.0.read_file_from_path(path).ok()?;
        let source = JsFileSource::try_from(path).ok()?;
        let parsed = parse(&text, source, JsParserOptions::default());
        if parsed.has_errors() {
            return None;
        }
        let root = parsed.tree();
        let model = semantic_model(&root, SemanticModelOptions::default());
        Some((root, model))
    }

    fn installed_package(&self, package_path: &Utf8Path, name: &str) -> Option<InstalledPackage> {
        let path = package_path.join("node_modules").join(name);
        let manifest =
            PackageJson::read_manifest(&self.0, &path.join("package.json")).into_deserialized()?;
        Some(InstalledPackage { path, manifest })
    }

    fn resolve(
        &self,
        directory: &Utf8Path,
        specifier: &str,
        _kind: ResolutionKind,
    ) -> Option<Utf8PathBuf> {
        let path = directory.join(specifier.strip_prefix("./")?);
        self.0.path_is_file(&path).then_some(path)
    }

    fn glob(&self, directory: &Utf8Path, glob: &Glob) -> Vec<Utf8PathBuf> {
        let mut paths: Vec<_> = self
            .0
            .files
            .read()
            .keys()
            .filter(|path| {
                path.strip_prefix(directory)
                    .is_ok_and(|relative| glob.is_match(relative))
            })
            .cloned()
            .collect();
        paths.sort();
        paths
    }
}

/// A framework configured like Astro: `framework.config.mjs` exports
/// `defineConfig({ styles, fonts, integrations })`. The test host can't read
/// `framework.config.json` as JavaScript.
#[derive(Debug)]
struct RootLoader;

impl FrameworkLoader for RootLoader {
    fn package_name(&self) -> &'static str {
        "framework"
    }

    fn supported_versions(&self) -> &'static str {
        ">=1.0.0"
    }

    fn activation(&self) -> Activation {
        Activation::Dependency {
            config_files: &[
                "framework.config.ts",
                "framework.config.mjs",
                "framework.config.json",
            ],
        }
    }

    fn load(&self, ctx: &LoaderContext, options: &ConfigValue) -> Vec<Contribution> {
        let mut contributions = vec![Contribution::Visit(
            ctx.framework().path.join("dist/global.css"),
        )];
        let options = match options.as_call() {
            Some(call) if call.export.text() == "defineConfig" => call.options(),
            _ => options.clone(),
        };

        match options.get("styles") {
            Some(ConfigValue::Array(styles)) => {
                for style in styles {
                    match style
                        .as_str()
                        .and_then(|style| ctx.resolve(style, ResolutionKind::Stylesheet))
                    {
                        Some(path) => contributions.push(Contribution::Visit(path)),
                        None => contributions.push(Contribution::Incomplete(Facet::Styles)),
                    }
                }
            }
            Some(_) => contributions.push(Contribution::Incomplete(Facet::Styles)),
            None => {}
        }

        if let Some(fonts) = options.get("fonts").and_then(ConfigValue::as_array) {
            for font in fonts {
                if let Some(ConfigValue::String { value, range }) = font.get("cssVariable")
                    && let Some(origin) = ctx.origin(*range)
                {
                    contributions.push(Contribution::Declare(Declaration {
                        kind: DeclarationKind::CssCustomProperty,
                        name: value.clone(),
                        origin,
                    }));
                }
            }
        }

        if let Some(integrations) = options.get("integrations").and_then(ConfigValue::as_array) {
            for integration in integrations {
                if let Some(call) = integration.as_call()
                    && let Some(package) = call.package_name()
                {
                    contributions.push(Contribution::Integration {
                        package: package.to_string().into(),
                        options: call.options(),
                    });
                }
            }
        }

        contributions
    }
}

/// An integration whose `extra` option names a stylesheet.
#[derive(Debug)]
struct PluginLoader;

impl FrameworkLoader for PluginLoader {
    fn package_name(&self) -> &'static str {
        "plugin"
    }

    fn supported_versions(&self) -> &'static str {
        "*"
    }

    fn activation(&self) -> Activation {
        Activation::Integration
    }

    fn load(&self, ctx: &LoaderContext, options: &ConfigValue) -> Vec<Contribution> {
        options
            .get("extra")
            .and_then(ConfigValue::as_str)
            .and_then(|extra| ctx.resolve(extra, ResolutionKind::Stylesheet))
            .map(Contribution::Visit)
            .into_iter()
            .collect()
    }
}

/// An integration that registers the integration `next`.
#[derive(Debug)]
struct ChainLoader {
    name: &'static str,
    next: &'static str,
}

impl FrameworkLoader for ChainLoader {
    fn package_name(&self) -> &'static str {
        self.name
    }

    fn supported_versions(&self) -> &'static str {
        "*"
    }

    fn activation(&self) -> Activation {
        Activation::Integration
    }

    fn load(&self, ctx: &LoaderContext, _options: &ConfigValue) -> Vec<Contribution> {
        let origin = ctx
            .origin(TextRange::default())
            .expect("the options must come from a configuration file");
        vec![
            Contribution::Declare(Declaration {
                kind: DeclarationKind::JsBinding,
                name: self.name.into(),
                origin,
            }),
            Contribution::Integration {
                package: self.next.into(),
                options: ConfigValue::new_empty_object(),
            },
        ]
    }
}

const PACKAGE_JSON: (&str, &str) = (
    "/project/package.json",
    r#"{ "dependencies": { "framework": "^1.0.0" } }"#,
);

const FRAMEWORK_PACKAGE_JSON: (&str, &str) = (
    "/project/node_modules/framework/package.json",
    r#"{ "name": "framework", "version": "1.2.0" }"#,
);

fn package_manifest(host: &TestHost) -> PackageJson {
    PackageJson::read_manifest(&host.0, Utf8Path::new("/project/package.json"))
        .into_deserialized()
        .expect("the package manifest must be valid")
}

fn load(
    host: &TestHost,
    loaders: &[&dyn FrameworkLoader],
) -> biome_frameworks::FrameworkContributions {
    load_frameworks(
        host,
        loaders,
        Utf8Path::new("/project"),
        &package_manifest(host),
    )
}

/// Returns the visited paths, with `/` separators on every platform.
fn visits(contributions: &biome_frameworks::FrameworkContributions) -> Vec<String> {
    contributions
        .visits
        .iter()
        .map(|path| path.as_str().replace('\\', "/"))
        .collect()
}

/// Evaluates the default export of `source`, a `.mjs` file.
fn evaluate(source: &str) -> ConfigValue {
    let root = parse(
        source,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    )
    .tree();
    let model = semantic_model(&root, SemanticModelOptions::default());
    ConfigValue::from_default_export(&root, &model)
}

// #endregion

// #region DRIVER

#[test]
fn loads_the_frameworks_of_a_package() {
    let config = r#"import { defineConfig } from "framework/config";
import plugin from "plugin";
import styledIntegration from "styled-integration";
import { local } from "./local.mjs";

const styles = ["./src/global.css"];

export default defineConfig({
    styles: [...styles, `./src/theme.css`],
    fonts: [{ cssVariable: "--font-body" }],
    integrations: [plugin({ extra: "./src/extra.css" }), styledIntegration(), local()],
});
"#;
    let host = TestHost::new(&[
        PACKAGE_JSON,
        FRAMEWORK_PACKAGE_JSON,
        ("/project/framework.config.mjs", config),
        ("/project/src/global.css", ""),
        ("/project/src/theme.css", ""),
        ("/project/src/extra.css", ""),
        (
            "/project/node_modules/plugin/package.json",
            r#"{ "name": "plugin", "version": "0.1.0" }"#,
        ),
        (
            "/project/node_modules/styled-integration/package.json",
            r#"{
                "name": "styled-integration",
                "version": "2.0.0",
                "exports": {
                    ".": "./index.js",
                    "./styles": "./dist/styles.css",
                    "./themes/*": { "style": "./dist/themes/*.css" }
                }
            }"#,
        ),
        (
            "/project/node_modules/styled-integration/dist/themes/dark.css",
            "",
        ),
        (
            "/project/node_modules/styled-integration/dist/themes/light.css",
            "",
        ),
        (
            "/project/node_modules/styled-integration/dist/themes/high-contrast/dark.css",
            "",
        ),
        (
            "/project/node_modules/styled-integration/dist/themes/readme.md",
            "",
        ),
    ]);

    let contributions = load(&host, &[&RootLoader, &PluginLoader]);

    // Like in Node.js, the `*` of `./dist/themes/*.css` matches across
    // directories.
    assert_eq!(
        visits(&contributions),
        [
            "/project/node_modules/framework/dist/global.css",
            "/project/src/global.css",
            "/project/src/theme.css",
            "/project/src/extra.css",
            "/project/node_modules/styled-integration/dist/styles.css",
            "/project/node_modules/styled-integration/dist/themes/dark.css",
            "/project/node_modules/styled-integration/dist/themes/high-contrast/dark.css",
            "/project/node_modules/styled-integration/dist/themes/light.css",
        ]
    );

    let start = config
        .find("\"--font-body\"")
        .expect("the configuration must contain the font variable");
    let start = TextSize::try_from(start).expect("the offset must fit");
    assert_eq!(
        contributions.declarations,
        [Declaration {
            kind: DeclarationKind::CssCustomProperty,
            name: "--font-body".into(),
            origin: Origin {
                path: Utf8PathBuf::from("/project/framework.config.mjs"),
                range: TextRange::at(start, TextSize::from(13)),
            },
        }]
    );
    assert!(contributions.incomplete.is_empty());
}

#[test]
fn unknown_configuration_values_make_a_facet_incomplete() {
    let host = TestHost::new(&[
        PACKAGE_JSON,
        FRAMEWORK_PACKAGE_JSON,
        (
            "/project/framework.config.mjs",
            r#"import { computeStyles } from "./styles.mjs";
export default { styles: [...computeStyles()] };
"#,
        ),
    ]);

    let contributions = load(&host, &[&RootLoader]);

    assert_eq!(contributions.incomplete, BTreeSet::from([Facet::Styles]));
}

#[test]
fn framework_without_configuration_gets_empty_options() {
    let host = TestHost::new(&[PACKAGE_JSON, FRAMEWORK_PACKAGE_JSON]);

    let contributions = load(&host, &[&RootLoader]);

    assert_eq!(
        visits(&contributions),
        ["/project/node_modules/framework/dist/global.css"]
    );
    assert!(contributions.incomplete.is_empty());
}

#[test]
fn unreadable_configuration_makes_a_facet_incomplete() {
    let not_javascript = TestHost::new(&[
        PACKAGE_JSON,
        FRAMEWORK_PACKAGE_JSON,
        ("/project/framework.config.json", "{}"),
    ]);
    let syntax_error = TestHost::new(&[
        PACKAGE_JSON,
        FRAMEWORK_PACKAGE_JSON,
        (
            "/project/framework.config.mjs",
            r#"export default { styles: ["./src/global.css" };"#,
        ),
    ]);

    for host in [not_javascript, syntax_error] {
        let contributions = load(&host, &[&RootLoader]);
        assert_eq!(contributions.incomplete, BTreeSet::from([Facet::Styles]));
    }
}

#[test]
fn every_star_of_an_exported_pattern_matches_the_same_characters() {
    let host = TestHost::new(&[
        PACKAGE_JSON,
        FRAMEWORK_PACKAGE_JSON,
        (
            "/project/framework.config.mjs",
            r#"import themes from "themes";
export default { integrations: [themes()] };
"#,
        ),
        (
            "/project/node_modules/themes/package.json",
            r#"{
                "name": "themes",
                "version": "1.0.0",
                "exports": { "./*": "./dist\\*\\*.css" }
            }"#,
        ),
        ("/project/node_modules/themes/dist/dark/dark.css", ""),
        ("/project/node_modules/themes/dist/dark/light.css", ""),
    ]);

    let contributions = load(&host, &[&RootLoader]);

    assert_eq!(
        visits(&contributions),
        [
            "/project/node_modules/framework/dist/global.css",
            "/project/node_modules/themes/dist/dark/dark.css",
        ]
    );
}

#[test]
fn integrations_that_are_not_installed_make_every_facet_incomplete() {
    let host = TestHost::new(&[
        PACKAGE_JSON,
        FRAMEWORK_PACKAGE_JSON,
        (
            "/project/framework.config.mjs",
            r#"import plugin from "plugin";
import other from "other-integration";
export default { integrations: [plugin(), other()] };
"#,
        ),
    ]);

    // `plugin` has a loader and `other-integration` doesn't.
    for loaders in [
        &[&RootLoader as &dyn FrameworkLoader, &PluginLoader][..],
        &[&RootLoader],
    ] {
        let contributions = load(&host, loaders);
        assert_eq!(contributions.incomplete, BTreeSet::from(Facet::ALL));
    }
}

#[test]
fn missing_or_unsupported_frameworks_make_every_facet_incomplete() {
    let not_installed = TestHost::new(&[PACKAGE_JSON]);
    let unsupported = TestHost::new(&[
        PACKAGE_JSON,
        (
            "/project/node_modules/framework/package.json",
            r#"{ "name": "framework", "version": "0.9.0" }"#,
        ),
    ]);

    for host in [not_installed, unsupported] {
        let contributions = load(&host, &[&RootLoader]);
        assert!(contributions.visits.is_empty());
        assert_eq!(contributions.incomplete, BTreeSet::from(Facet::ALL));
    }
}

#[test]
fn exported_stylesheets_follow_the_node_target_rules() {
    let host = TestHost::new(&[
        PACKAGE_JSON,
        FRAMEWORK_PACKAGE_JSON,
        (
            "/project/framework.config.mjs",
            r#"import styled from "styled";
export default { integrations: [styled()] };
"#,
        ),
        (
            "/project/node_modules/styled/package.json",
            r#"{
                "name": "styled",
                "version": "1.0.0",
                "exports": {
                    "./outside": "../outside.css",
                    "./absolute": "/absolute.css",
                    "./bare": "bare.css",
                    "./vendored": "./node_modules/vendored.css",
                    "./*": "./*.css"
                }
            }"#,
        ),
        ("/project/node_modules/styled/theme.css", ""),
        ("/project/node_modules/styled/dark/theme.css", ""),
        (
            "/project/node_modules/styled/node_modules/vendored/theme.css",
            "",
        ),
    ]);

    let contributions = load(&host, &[&RootLoader]);

    assert_eq!(
        visits(&contributions),
        [
            "/project/node_modules/framework/dist/global.css",
            "/project/node_modules/styled/dark/theme.css",
            "/project/node_modules/styled/theme.css",
        ]
    );
}

#[test]
fn loaders_run_only_for_dependencies_of_the_package() {
    let host = TestHost::new(&[
        ("/project/package.json", r#"{ "dependencies": {} }"#),
        FRAMEWORK_PACKAGE_JSON,
    ]);

    let contributions = load(&host, &[&RootLoader]);

    assert_eq!(contributions, Default::default());
}

/// Loads the integrations that `config` registers, where `first` registers
/// `second`, `second` registers `third`, and `third` registers `first`, and
/// returns the names of the integrations that ran, in order.
/// Returns the names that the chain declares, and the facets that it leaves
/// incomplete.
fn run_integration_chain(config: &str) -> (Vec<String>, BTreeSet<Facet>) {
    let packages: Vec<_> = ["first", "second", "third"]
        .into_iter()
        .map(|name| {
            (
                format!("/project/node_modules/{name}/package.json"),
                format!(r#"{{ "name": "{name}", "version": "1.0.0" }}"#),
            )
        })
        .collect();
    let mut files: Vec<_> = packages
        .iter()
        .map(|(path, content)| (path.as_str(), content.as_str()))
        .collect();
    files.extend([
        PACKAGE_JSON,
        FRAMEWORK_PACKAGE_JSON,
        ("/project/framework.config.mjs", config),
    ]);
    let host = TestHost::new(&files);

    let first = ChainLoader {
        name: "first",
        next: "second",
    };
    let second = ChainLoader {
        name: "second",
        next: "third",
    };
    let third = ChainLoader {
        name: "third",
        next: "first",
    };
    let contributions = load(&host, &[&RootLoader, &first, &second, &third]);
    let declared = contributions
        .declarations
        .iter()
        .map(|declaration| declaration.name.to_string())
        .collect();
    (declared, contributions.incomplete)
}

#[test]
fn integrations_nest_at_most_three_levels_and_load_once() {
    // `framework` registers `first` twice, at the second level. `first`
    // registers `second` at the third level, and `second` registers `third`
    // at the fourth level, which isn't loaded, so every facet is incomplete.
    let (declared, incomplete) = run_integration_chain(
        r#"import first from "first";
export default { integrations: [first(), first()] };
"#,
    );

    assert_eq!(declared, ["first", "second"]);
    assert_eq!(incomplete, BTreeSet::from(Facet::ALL));
}

#[test]
fn integrations_load_breadth_first() {
    // `framework` registers `first` and `second` at the second level, so
    // `second` runs there and registers `third` at the third level. `first`
    // also registers `second`, which already ran. `third` registers `first`
    // at the fourth level, which already ran too, so nothing is incomplete.
    let (declared, incomplete) = run_integration_chain(
        r#"import first from "first";
import second from "second";
export default { integrations: [first(), second()] };
"#,
    );

    assert_eq!(declared, ["first", "second", "third"]);
    assert!(incomplete.is_empty());
}

// #endregion

// #region CONFIG VALUES

#[test]
fn evaluates_literals_arrays_objects_and_constants() {
    let value = evaluate(
        r#"const base = { title: "Docs", count: 2 };
const list = ["a", `b`];
export default { ...base, list: [...list, true], nested: { list } };
"#,
    );

    assert_eq!(
        value.get("title").and_then(ConfigValue::as_str),
        Some("Docs")
    );
    assert_eq!(value.get("count"), Some(&ConfigValue::Number(2.0)));
    let list = |value: &ConfigValue| -> Vec<Option<String>> {
        value
            .as_array()
            .expect("the value must be an array")
            .iter()
            .map(|element| match element {
                ConfigValue::Boolean(boolean) => Some(boolean.to_string()),
                element => element.as_str().map(str::to_string),
            })
            .collect()
    };
    assert_eq!(
        list(value.get("list").expect("list must exist")),
        [Some("a".into()), Some("b".into()), Some("true".into())]
    );
    assert_eq!(
        list(
            value
                .get("nested")
                .and_then(|nested| nested.get("list"))
                .expect("nested.list must exist")
        ),
        [Some("a".into()), Some("b".into())]
    );
    assert_eq!(value.get("missing"), None);
}

#[test]
fn keeps_calls_to_imported_functions() {
    let value = evaluate(
        r#"import { defineConfig as define } from "framework/config";
import integration from "@scope/integration/sub";
export default define({ integrations: [integration({ option: 1 }, ...rest)] });
"#,
    );

    let call = value.as_call().expect("the default export must be a call");
    assert_eq!(call.specifier.text(), "framework/config");
    assert_eq!(call.export.text(), "defineConfig");
    assert_eq!(call.package_name(), Some("framework"));

    let integration = call
        .options()
        .get("integrations")
        .and_then(|integrations| integrations.as_array()?.first()?.as_call().cloned())
        .expect("the integration must be a call");
    assert_eq!(integration.export.text(), "default");
    assert_eq!(integration.package_name(), Some("@scope/integration"));
    assert_eq!(
        integration.arguments.get(1),
        Some(&ConfigValue::Unknown),
        "a spread argument is unknown"
    );
}

#[test]
fn values_that_need_running_code_are_unknown() {
    let value = evaluate(
        r#"import { imported } from "./values.mjs";
let mutable = "a";
const { destructured } = { destructured: "b" };
const cycle = other;
const other = cycle;
function local() {}
export default {
    imported,
    mutable,
    destructured,
    cycle,
    call: local(),
    [computed]: 1,
};
"#,
    );

    for name in [
        "imported",
        "mutable",
        "destructured",
        "cycle",
        "call",
        // The computed property can be named `missing`.
        "missing",
    ] {
        assert_eq!(value.get(name), Some(&ConfigValue::Unknown), "{name}");
    }
}

#[test]
fn later_properties_replace_earlier_ones() {
    let value = evaluate(
        r#"import { unknown } from "./values.mjs";
export default { first: 1, ...unknown, second: 2 };
"#,
    );

    assert_eq!(value.get("first"), Some(&ConfigValue::Unknown));
    assert_eq!(value.get("second"), Some(&ConfigValue::Number(2.0)));
}

#[test]
fn file_without_default_export_is_unknown() {
    assert_eq!(evaluate("export const config = {};"), ConfigValue::Unknown);
}

#[test]
fn package_names_of_specifiers() {
    assert_eq!(package_name("astro"), Some("astro"));
    assert_eq!(package_name("astro/config"), Some("astro"));
    assert_eq!(
        package_name("@astrojs/starlight"),
        Some("@astrojs/starlight")
    );
    assert_eq!(
        package_name("@astrojs/starlight/props"),
        Some("@astrojs/starlight")
    );
    assert_eq!(package_name("./local.mjs"), None);
    assert_eq!(package_name("/absolute.mjs"), None);
    assert_eq!(package_name("#integrations/local"), None);
    assert_eq!(package_name("node:path"), None);
    assert_eq!(package_name("jsr:@std/path"), None);
    assert_eq!(package_name("@scope/../outside"), None);
    assert_eq!(package_name("a\\..\\outside"), None);
}

#[test]
fn variables_that_the_file_can_change_are_unknown() {
    let value = evaluate(
        r#"import { defineConfig } from "framework/config";
import integration from "integration";
import { addDefaults } from "./defaults.mjs";
const pushed = ["a"];
pushed.push("b");
const assigned = { styles: [] };
assigned.styles = ["c"];
const merged = {};
Object.assign(merged, { styles: ["d"] });
const passed = ["e"];
change(passed);
const added = ["g"];
addDefaults(added);
const wrapped = ["h"];
extend({ list: wrapped });
const iterated = ["i"];
[iterated].forEach((list) => list.push("j"));
const constructed = ["k"];
const aliased = ["l"];
const alias = aliased;
alias.push("m");
const read = ["f"];
const options = { read };
const shared = ["n"];
const first = [shared];
const second = [shared];
export default defineConfig({
    pushed,
    assigned,
    merged,
    passed,
    added,
    wrapped,
    iterated,
    constructed,
    instance: new Local(constructed),
    aliased,
    read,
    imported: integration(read),
    options,
    both: [first, second],
});
"#,
    );
    let options = value
        .as_call()
        .expect("the default export must be a call")
        .options();

    for name in [
        "pushed",
        "assigned",
        "merged",
        "passed",
        "added",
        "wrapped",
        "iterated",
        "constructed",
        "aliased",
    ] {
        assert_eq!(options.get(name), Some(&ConfigValue::Unknown), "{name}");
    }
    for name in ["read", "both"] {
        assert!(
            options.get(name).and_then(ConfigValue::as_array).is_some(),
            "`{name}` is only read"
        );
    }
}

#[test]
fn escape_sequences_decode_exactly_or_make_values_unknown() {
    // The configurations contain the backslashes built here. A backslash
    // followed by `01` is a legacy octal escape, followed by `uD83D` is half
    // of a surrogate pair, and followed by `u0043` is the letter `C`.
    let backslash = '\\';
    let strings = evaluate(&format!(
        r#"export default {{ octal: "{backslash}01", surrogate: "{backslash}uD83D", letter: "{backslash}u0043" }};"#
    ));
    let names = evaluate(&format!(
        r#"export default {{ identifier{backslash}u0043ss: [], "string{backslash}u0043ss": [] }};"#
    ));

    assert_eq!(strings.get("octal"), Some(&ConfigValue::Unknown));
    assert_eq!(strings.get("surrogate"), Some(&ConfigValue::Unknown));
    assert_eq!(
        strings.get("letter").and_then(ConfigValue::as_str),
        Some("C")
    );
    assert_eq!(
        names.get("stringCss"),
        Some(&ConfigValue::Array(Vec::new()))
    );
    assert_eq!(names.get("identifierCss"), Some(&ConfigValue::Unknown));
}

#[test]
fn numbers_and_numeric_names_match_their_runtime_values() {
    let value = evaluate(
        r#"export default { 0x10: "sixteen", 1.0: "one", 10: "ten", hex: 0x1000000000000081 };"#,
    );

    assert_eq!(value.get("10").and_then(ConfigValue::as_str), Some("ten"));
    // `0x10` names the property `16`, and `1.0` names `1`.
    assert_eq!(value.get("16"), Some(&ConfigValue::Unknown));
    assert_eq!(value.get("1"), Some(&ConfigValue::Unknown));
    assert_eq!(
        value.get("hex"),
        Some(&ConfigValue::Number(1_152_921_504_606_847_200.0))
    );
}

#[test]
fn array_holes_keep_the_positions_of_elements() {
    let value = evaluate(r#"export default [, "./a.css"];"#);

    assert_eq!(
        value.as_array().map(|elements| elements.len()),
        Some(2),
        "the hole keeps its position"
    );
}

#[test]
fn imports_with_escape_sequences_are_unknown() {
    let backslash = '\\';
    let value = evaluate(&format!(
        r#"import integration from "{backslash}u0040scope/integration";
export default integration();
"#
    ));

    assert_eq!(value, ConfigValue::Unknown);
}

#[test]
fn template_with_carriage_return_is_unknown() {
    let value = evaluate("export default { path: `./a\r\n.css` };");

    assert_eq!(value.get("path"), Some(&ConfigValue::Unknown));
}

#[test]
fn variable_without_references_hides_changes() {
    let value = evaluate(
        r#"import { registerThemes } from "./themes.mjs";
const customCss = ["./a.css"];
const themes = registerThemes(customCss);
export default { customCss };
"#,
    );

    assert_eq!(value.get("customCss"), Some(&ConfigValue::Unknown));
}

#[test]
fn long_chains_of_variables_are_unknown() {
    let mut source = String::from("const v0 = [\"x\"];\n");
    for index in 1..=5_000 {
        let previous = index - 1;
        source.push_str(&format!("const v{index} = [v{previous}];\n"));
    }
    source.push_str("export default { first: v0, last: v5000 };\n");

    let value = evaluate(&source);

    assert_eq!(value.get("first"), Some(&ConfigValue::Unknown));
}

#[test]
fn prototype_property_makes_other_properties_unknown() {
    let value = evaluate(
        r#"import { shared } from "./shared.mjs";
export default { __proto__: shared, ["__proto__"]: 1 };
"#,
    );

    assert_eq!(value.get("customCss"), Some(&ConfigValue::Unknown));
}

#[test]
fn properties_of_calls_are_unknown() {
    let value = evaluate(
        r#"import { buildOptions } from "./options.mjs";
import integration from "integration";
export default { integration: integration(buildOptions()) };
"#,
    );
    let integration = value
        .get("integration")
        .and_then(ConfigValue::as_call)
        .expect("the integration must be a call");

    assert_eq!(
        integration.options().get("styles"),
        Some(&ConfigValue::Unknown)
    );
}

#[test]
fn evaluation_stops_after_a_fixed_number_of_expressions() {
    // Each variable spreads the previous one twice, so the last one has
    // 2^41 elements.
    let mut source = String::from("const v0 = [\"x\", \"x\"];\n");
    for index in 1..=40 {
        let previous = index - 1;
        source.push_str(&format!(
            "const v{index} = [...v{previous}, ...v{previous}];\n"
        ));
    }
    source.push_str("export default { styles: v40 };\n");

    let value = evaluate(&source);

    let styles = value
        .get("styles")
        .and_then(ConfigValue::as_array)
        .expect("styles must be an array");
    assert!(styles.contains(&ConfigValue::Unknown));
}

#[test]
fn template_with_escape_sequences_is_unknown() {
    // `\x62` is an escape sequence for `b`.
    let escape = format!("{}x62", '\\');
    let value = evaluate(&format!(
        "export default {{ path: `./src/a{escape}.css`, plain: `./b.css` }};"
    ));

    assert_eq!(value.get("path"), Some(&ConfigValue::Unknown));
    assert_eq!(
        value.get("plain").and_then(ConfigValue::as_str),
        Some("./b.css")
    );
}

// #endregion
