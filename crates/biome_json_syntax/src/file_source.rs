use biome_rowan::FileSourceError;
use std::{ffi::OsStr, path::Path};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct JsonFileSource {
    allow_trailing_commas: bool,
    allow_comments: bool,
}

impl JsonFileSource {
    // Well-known JSON files
    // This list should be SORTED!
    // Source: https://github.com/github-linguist/linguist/blob/4ac734c15a96f9e16fd12330d0cb8de82274f700/lib/linguist/languages.yml#L3203-L3218
    // Note: we shouldn't include machine generated files
    const WELL_KNOWN_JSON_FILES: &'static [&'static str] = &[
        ".all-contributorsrc",
        ".arcconfig",
        ".auto-changelog",
        // Uses `JSON.parse`
        // https://github.com/bower/bower/blob/a0d44443245cbe52f3f0bd90c3f41274bc040c7a/packages/bower-config/lib/util/rc.js#L61
        ".bowerrc",
        ".c8rc",
        ".htmlhintrc",
        ".imgbotconfig",
        // Uses `JSON.parse`
        // https://github.com/reid/node-jslint/blob/1eefcc48116fe4fbc3b585ebd2922b3509f4f4de/lib/options.js#L31
        ".jslintrc",
        ".nycrc",
        ".tern-config",
        ".tern-project",
        // Uses the `readJSON` from `fs-extra`
        // https://github.com/vuejs/vue-cli/blob/f0f254e4bc81ed322eeb9f7de346e987e845068e/packages/%40vue/cli/lib/config.js#L8
        ".vuerc",
        ".watchmanconfig",
        "mcmod.info",
    ];

    // Well-known JSON-like files that support comments but no trailing commas
    // This list should be SORTED!
    // Note: we shouldn't include machine generated files
    const WELL_KNOWN_JSON_ALLOW_COMMENTS_FILES: &'static [&'static str] = &[
        // Uses `yam` to parse the config, which only strip comments
        // https://github.com/ember-cli/ember-cli/blob/0f7d0ccb52cdd20f17efbee79dd3f08ec6019cfc/lib/utilities/get-config.js#L7
        // https://github.com/twokul/yam/blob/773e42548511977bf1e2b9b95c60496fe7f8a9df/lib/utils/io-utils.js#L45
        ".ember-cli",
        // `.eslintrc` is parsed as yaml, so we shouldn't include it here:
        // https://github.com/eslint/eslintrc/blob/fb8d7ffbb27214686318a07e16ac8878aaafc805/lib/config-array-factory.js#L205-L225

        // Uses `strip-json-comments`, which doesn't allow trailing commas by default
        // https://github.com/eslint/eslintrc/blob/fb8d7ffbb27214686318a07e16ac8878aaafc805/lib/config-array-factory.js#L192
        // https://github.com/sindresorhus/strip-json-comments/blob/85611bf8a07246bca27f949c997a1460c57bbe9f/index.js#L19
        ".eslintrc.json",
        // Uses `strip-json-comments`, which doesn't allow trailing commas by default
        // https://github.com/jscs-dev/node-jscs/blob/38d33a0e455d965106ad3c8948c870f8f4e5dda9/lib/cli-config.js#L81
        ".jscsrc",
        // `.jsfmtrc` can be either an `ini` file or a `json` file (which will be parsed after `strip-with-comments`), so we shouldn't include it here.
        // https://github.com/rdio/jsfmt?tab=readme-ov-file#jsfmtrc

        // Uses `strip-json-comments`, which doesn't allow trailing commas by default
        // https://github.com/jshint/jshint/blob/0a5644f8f529e252e7dd0c0d54334ae435b13de0/src/cli.js#L538
        ".jshintrc",
        // Just strip comments
        // https://github.com/palantir/tslint/blob/285fc1db18d1fd24680d6a2282c6445abf1566ee/src/configuration.ts#L268
        "tslint.json",
    ];

    // Well-known JSON-like files that support comments and trailing commas
    // This list should be SORTED!
    // Note: we shouldn't include machine generated files
    const WELL_KNOWN_JSON_ALLOW_COMMENTS_AND_TRAILING_COMMAS_FILES: &'static [&'static str] = &[
        // Uses `json5`, we treat them as JSONC for now:
        // https://github.com/babel/babel/blob/3956c75123e713c5fa1d3279f6f92cfeac290173/packages/babel-core/src/config/files/configuration.ts#L341
        ".babelrc",
        ".babelrc.json",
        // https://docs.github.com/en/codespaces/setting-up-your-project-for-codespaces/adding-a-dev-container-configuration/introduction-to-dev-containers#editing-the-devcontainerjson-file
        ".devcontainer.json",
        // Uses `jsonc-parser`:
        // https://github.com/webhintio/hint/blob/6ef9b7cd0c9129ca5a53f30ef51812622ad3d459/packages/hint/src/lib/config.ts#L248
        // https://github.com/webhintio/hint/blob/6ef9b7cd0c9129ca5a53f30ef51812622ad3d459/packages/utils-fs/src/load-json-file.ts#L1C35-L1C47
        ".hintrc",
        ".hintrc.json",
        // Uses `jsonc_parser` and allows comments and trailing commas
        // https://github.com/swc-project/swc/blob/ad932f0921411364b801b32f60eaf98f8629e812/crates/swc/src/lib.rs#L1028-L1029
        ".swcrc",
        // Uses `jju`, default is JSON5, we treat them as JSONC for now:
        // https://github.com/microsoft/rushstack/blob/38f0de8ba9f29d337564409eba5639287784b756/apps/api-extractor/src/api/ExtractorConfig.ts#L532
        // https://github.com/microsoft/rushstack/blob/38f0de8ba9f29d337564409eba5639287784b756/libraries/node-core-library/src/JsonFile.ts#L218
        // https://github.com/microsoft/rushstack/blob/38f0de8ba9f29d337564409eba5639287784b756/libraries/node-core-library/src/JsonFile.ts#L583-L585
        "api-documenter.json",
        "api-extractor.json",
        // See `.babelrc`
        "babel.config.json",
        // Uses `jsonc-parser`, and allows comments and trailing commas by default
        // https://github.com/denoland/deno/blob/5a716d1d06f73800b280259204789260774d465d/cli/tools/registry/pm.rs#L114
        "deno.json",
        // See `.devcontainer.json`
        "devcontainer.json",
        // Uses `jsonc-parser`, and allows comments and trailing commas by default
        // https://github.com/dprint/dprint/blob/f523f4db9750af5e73a9cdd3384ed9cd7e223e53/crates/dprint/src/configuration/manipulation.rs#L85
        "dprint.json",
        // See `tsconfig.json`
        "jsconfig.json",
        // Uses `jsonc-parser`, and allows comments and trailing commas by default
        // https://github.com/jsr-io/jsr/blob/32d3481d32f566079d33ba2ec1b598ea0c38b32c/api/src/tarball.rs#L194-L198
        // https://docs.rs/jsonc-parser/0.23.0/jsonc_parser/struct.ParseOptions.html
        "jsr.json",
        // vscode files
        "language-configuration.json",
        // Uses its own parser
        // https://github.com/microsoft/TypeScript/blob/a2d37a5c606803c92c00069e01d7964529e01bee/src/compiler/commandLineParser.ts#L2111-L2117
        // https://github.com/microsoft/TypeScript/blob/a2d37a5c606803c92c00069e01d7964529e01bee/src/compiler/parser.ts#L1433
        // https://github.com/microsoft/TypeScript/blob/a2d37a5c606803c92c00069e01d7964529e01bee/src/compiler/parser.ts#L3536
        // https://github.com/microsoft/TypeScript/blob/a2d37a5c606803c92c00069e01d7964529e01bee/src/compiler/parser.ts#L2583-L2587
        "tsconfig.json",
        // Uses the parser from TypeScript
        // https://github.com/TypeStrong/typedoc/blob/30e614cd9e7b5a154afa6a78f2e54f16550bfb4f/src/lib/utils/options/readers/typedoc.ts#L74
        "typedoc.json",
        // vscode files
        "typescript.json",
    ];

    pub fn json() -> Self {
        Self {
            allow_comments: false,
            allow_trailing_commas: false,
        }
    }

    pub fn json_allow_comments() -> Self {
        Self {
            allow_comments: true,
            allow_trailing_commas: false,
        }
    }

    pub fn json_allow_comments_and_trailing_commas() -> Self {
        Self {
            allow_comments: true,
            allow_trailing_commas: true,
        }
    }

    pub fn with_allow_trailing_commas(mut self) -> Self {
        self.allow_trailing_commas = true;
        self
    }

    pub fn allow_trailing_commas(&self) -> bool {
        self.allow_trailing_commas
    }

    pub fn with_allow_comments(mut self) -> Self {
        self.allow_comments = true;
        self
    }

    pub fn allow_comments(&self) -> bool {
        self.allow_comments
    }

    pub fn is_well_known_json_file(file_name: &str) -> bool {
        Self::WELL_KNOWN_JSON_FILES
            .binary_search(&file_name)
            .is_ok()
    }

    pub fn is_well_known_json_allow_comments_file(file_name: &str) -> bool {
        Self::WELL_KNOWN_JSON_ALLOW_COMMENTS_FILES
            .binary_search(&file_name)
            .is_ok()
    }

    pub fn is_well_known_json_allow_comments_and_trailing_commas_file(filename: &str) -> bool {
        Self::WELL_KNOWN_JSON_ALLOW_COMMENTS_AND_TRAILING_COMMAS_FILES
            .binary_search(&filename)
            .is_ok()
    }

    /// Try to return the JSON file source corresponding to this file name from well-known files
    pub fn try_from_well_known(file_name: &str) -> Result<Self, FileSourceError> {
        if Self::is_well_known_json_allow_comments_and_trailing_commas_file(file_name) {
            return Ok(Self::json_allow_comments_and_trailing_commas());
        }
        if Self::is_well_known_json_allow_comments_file(file_name) {
            return Ok(Self::json_allow_comments());
        }
        if Self::is_well_known_json_file(file_name) {
            return Ok(Self::json());
        }
        Err(FileSourceError::UnknownFileName(file_name.into()))
    }

    /// Try to return the JSON file source corresponding to this file extension
    pub fn try_from_extension(extension: &str) -> Result<Self, FileSourceError> {
        match extension {
            // https://github.com/github-linguist/linguist/blob/4ac734c15a96f9e16fd12330d0cb8de82274f700/lib/linguist/languages.yml#L3183-L3202
            // https://www.w3.org/TR/json-ld/#application-ld-json
            "json" | "jsonld" | "webapp" | "webmanifest" => Ok(Self::json()),
            // https://github.com/github-linguist/linguist/blob/4ac734c15a96f9e16fd12330d0cb8de82274f700/lib/linguist/languages.yml#L3230-L3246
            "jsonc"
            | "code-snippets"
            | "code-workspace"
            | "sublime-build"
            | "sublime-commands"
            | "sublime-completions"
            | "sublime-keymap"
            | "sublime-macro"
            | "sublime-menu"
            | "sublime-mousemap"
            | "sublime-project"
            | "sublime-settings"
            | "sublime-theme"
            | "sublime-workspace"
            | "sublime_metrics"
            | "sublime_session" => Ok(Self::json_allow_comments_and_trailing_commas()),
            _ => Err(FileSourceError::UnknownExtension(
                Default::default(),
                extension.into(),
            )),
        }
    }

    /// Try to return the JSON file source corresponding to this language ID
    ///
    /// See the [LSP spec] and [VS Code spec] for a list of language identifiers
    ///
    /// The language ID for code snippets is registered by [VS Code built-in extensions]
    ///
    /// [LSP spec]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
    /// [VS Code spec]: https://code.visualstudio.com/docs/languages/identifiers
    /// [VS Code built-in extensions]: https://github.com/microsoft/vscode/blob/f0ce480524258473372e0a2e9a07af6f64526010/extensions/json/package.json#L83
    pub fn try_from_language_id(language_id: &str) -> Result<Self, FileSourceError> {
        match language_id {
            "json" => Ok(Self::json()),
            "jsonc" | "snippets" => Ok(Self::json_allow_comments_and_trailing_commas()),
            _ => Err(FileSourceError::UnknownLanguageId(language_id.into())),
        }
    }
}

impl TryFrom<&Path> for JsonFileSource {
    type Error = FileSourceError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file_name = path
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or_else(|| FileSourceError::MissingFileName(path.into()))?;

        if let Ok(file_source) = Self::try_from_well_known(file_name) {
            return Ok(file_source);
        }

        // We assume the file extensions are case-insensitive
        // and we use the lowercase form of them for pattern matching
        let extension = &path
            .extension()
            .and_then(OsStr::to_str)
            .map(str::to_lowercase)
            .ok_or_else(|| FileSourceError::MissingFileExtension(path.into()))?;

        Self::try_from_extension(extension)
    }
}

#[test]
fn test_order() {
    for items in JsonFileSource::WELL_KNOWN_JSON_ALLOW_COMMENTS_AND_TRAILING_COMMAS_FILES.windows(2)
    {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in JsonFileSource::WELL_KNOWN_JSON_ALLOW_COMMENTS_FILES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in JsonFileSource::WELL_KNOWN_JSON_FILES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
