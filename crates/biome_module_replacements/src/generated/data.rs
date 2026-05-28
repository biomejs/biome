//! Generated file, do not edit by hand, see `xtask/codegen`

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnownUrlType {
    Mdn,
    Node,
    E18e,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnownUrl {
    Descriptor {
        url_type: KnownUrlType,
        id: &'static str,
    },
    Raw(&'static str),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EngineConstraint {
    pub engine: &'static str,
    pub min_version: Option<&'static str>,
    pub max_version: Option<&'static str>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModuleReplacementLike {
    pub id: &'static str,
    pub engines: &'static [EngineConstraint],
    pub preferred: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocumentedModuleReplacement {
    pub common: ModuleReplacementLike,
    pub replacement_module: &'static str,
    pub url: Option<KnownUrl>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeWebFeatureId {
    pub feature_id: &'static str,
    pub compat_key: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeNodeFeatureId {
    pub module_name: &'static str,
    pub export_name: Option<&'static str>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeModuleReplacement {
    pub common: ModuleReplacementLike,
    pub url: KnownUrl,
    pub description: Option<&'static str>,
    pub web_feature_id: Option<NativeWebFeatureId>,
    pub node_feature_id: Option<NativeNodeFeatureId>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimpleModuleReplacement {
    pub common: ModuleReplacementLike,
    pub description: &'static str,
    pub example: Option<&'static str>,
    pub url: Option<KnownUrl>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemovalReplacement {
    pub common: ModuleReplacementLike,
    pub description: &'static str,
    pub url: Option<KnownUrl>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleReplacement {
    Documented(DocumentedModuleReplacement),
    Native(NativeModuleReplacement),
    Simple(SimpleModuleReplacement),
    Removal(RemovalReplacement),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModuleReplacementMapping {
    pub mapping_type: &'static str,
    pub module_name: &'static str,
    pub replacements: &'static [&'static str],
    pub url: Option<KnownUrl>,
}
pub static MODULE_REPLACEMENTS_MAPPINGS: &[(&str, ModuleReplacementMapping)] = &[
    (
        "@75lb/deep-merge",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "@75lb/deep-merge",
            replacements: &["defu", "@fastify/deepmerge"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "deep-merge",
            }),
        },
    ),
    (
        "@cypress/request",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "@cypress/request",
            replacements: &["fetch", "ofetch", "ky"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "fetch",
            }),
        },
    ),
    (
        "@iarna/toml",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "@iarna/toml",
            replacements: &["smol-toml", "Bun.TOML"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "toml",
            }),
        },
    ),
    (
        "@jsdevtools/ez-spawn",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "@jsdevtools/ez-spawn",
            replacements: &["tinyexec"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "ez-spawn",
            }),
        },
    ),
    (
        "@ljharb/through",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "@ljharb/through",
            replacements: &["node:stream"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "through",
            }),
        },
    ),
    (
        "@supabase/node-fetch",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "@supabase/node-fetch",
            replacements: &["fetch", "ofetch", "ky"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "fetch",
            }),
        },
    ),
    (
        "abort-controller",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "abort-controller",
            replacements: &["AbortController"],
            url: None,
        },
    ),
    (
        "aggregate-error",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "aggregate-error",
            replacements: &["AggregateError"],
            url: None,
        },
    ),
    (
        "airbnb-js-shims",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "airbnb-js-shims",
            replacements: &["airbnb-js-shims"],
            url: None,
        },
    ),
    (
        "arg",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "arg",
            replacements: &["util.parseArgs", "mri"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "parseargs",
            }),
        },
    ),
    (
        "arr-diff",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "arr-diff",
            replacements: &["snippet::array-difference"],
            url: None,
        },
    ),
    (
        "arr-filter",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "arr-filter",
            replacements: &["Array.prototype.filter"],
            url: None,
        },
    ),
    (
        "arr-flatten",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "arr-flatten",
            replacements: &["snippet::array-flatten"],
            url: None,
        },
    ),
    (
        "arr-map",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "arr-map",
            replacements: &["Array.prototype.map"],
            url: None,
        },
    ),
    (
        "array-back",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array-back",
            replacements: &["snippet::array-coerce"],
            url: None,
        },
    ),
    (
        "array-buffer-byte-length",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array-buffer-byte-length",
            replacements: &["ArrayBuffer.prototype.byteLength"],
            url: None,
        },
    ),
    (
        "array-each",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array-each",
            replacements: &["Array.prototype.forEach"],
            url: None,
        },
    ),
    (
        "array-every",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array-every",
            replacements: &["Array.prototype.every"],
            url: None,
        },
    ),
    (
        "array-ify",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array-ify",
            replacements: &["snippet::array-coerce"],
            url: None,
        },
    ),
    (
        "array-includes",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array-includes",
            replacements: &["Array.prototype.includes"],
            url: None,
        },
    ),
    (
        "array-initial",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array-initial",
            replacements: &["snippet::array-slice-exclude-last-n"],
            url: None,
        },
    ),
    (
        "array-last",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array-last",
            replacements: &["snippet::array-last"],
            url: None,
        },
    ),
    (
        "array-map",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array-map",
            replacements: &["Array.prototype.map"],
            url: None,
        },
    ),
    (
        "array-range",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array-range",
            replacements: &["snippet::array-from-count-with-start"],
            url: None,
        },
    ),
    (
        "array-slice",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array-slice",
            replacements: &["Array.prototype.slice"],
            url: None,
        },
    ),
    (
        "array-union",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array-union",
            replacements: &["snippet::array-union"],
            url: None,
        },
    ),
    (
        "array-uniq",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array-uniq",
            replacements: &["snippet::array-unique"],
            url: None,
        },
    ),
    (
        "array-unique",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array-unique",
            replacements: &["snippet::array-unique"],
            url: None,
        },
    ),
    (
        "array.from",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.from",
            replacements: &["Array.from"],
            url: None,
        },
    ),
    (
        "array.of",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.of",
            replacements: &["Array.of"],
            url: None,
        },
    ),
    (
        "array.prototype.at",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.at",
            replacements: &["Array.prototype.at"],
            url: None,
        },
    ),
    (
        "array.prototype.concat",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.concat",
            replacements: &["Array.prototype.concat"],
            url: None,
        },
    ),
    (
        "array.prototype.copywithin",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.copywithin",
            replacements: &["Array.prototype.copyWithin"],
            url: None,
        },
    ),
    (
        "array.prototype.entries",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.entries",
            replacements: &["Array.prototype.entries"],
            url: None,
        },
    ),
    (
        "array.prototype.every",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.every",
            replacements: &["Array.prototype.every"],
            url: None,
        },
    ),
    (
        "array.prototype.filter",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.filter",
            replacements: &["Array.prototype.filter"],
            url: None,
        },
    ),
    (
        "array.prototype.find",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.find",
            replacements: &["Array.prototype.find"],
            url: None,
        },
    ),
    (
        "array.prototype.findindex",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.findindex",
            replacements: &["Array.prototype.findIndex"],
            url: None,
        },
    ),
    (
        "array.prototype.findlast",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.findlast",
            replacements: &["Array.prototype.findLast"],
            url: None,
        },
    ),
    (
        "array.prototype.findlastindex",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.findlastindex",
            replacements: &["Array.prototype.findLastIndex"],
            url: None,
        },
    ),
    (
        "array.prototype.flat",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.flat",
            replacements: &["Array.prototype.flat"],
            url: None,
        },
    ),
    (
        "array.prototype.flatmap",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.flatmap",
            replacements: &["Array.prototype.flatMap"],
            url: None,
        },
    ),
    (
        "array.prototype.foreach",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.foreach",
            replacements: &["Array.prototype.forEach"],
            url: None,
        },
    ),
    (
        "array.prototype.indexof",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.indexof",
            replacements: &["Array.prototype.indexOf"],
            url: None,
        },
    ),
    (
        "array.prototype.join",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.join",
            replacements: &["Array.prototype.join"],
            url: None,
        },
    ),
    (
        "array.prototype.keys",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.keys",
            replacements: &["Array.prototype.keys"],
            url: None,
        },
    ),
    (
        "array.prototype.lastindexof",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.lastindexof",
            replacements: &["Array.prototype.lastIndexOf"],
            url: None,
        },
    ),
    (
        "array.prototype.map",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.map",
            replacements: &["Array.prototype.map"],
            url: None,
        },
    ),
    (
        "array.prototype.push",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.push",
            replacements: &["Array.prototype.push"],
            url: None,
        },
    ),
    (
        "array.prototype.reduce",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.reduce",
            replacements: &["Array.prototype.reduce"],
            url: None,
        },
    ),
    (
        "array.prototype.reduceright",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.reduceright",
            replacements: &["Array.prototype.reduceRight"],
            url: None,
        },
    ),
    (
        "array.prototype.slice",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.slice",
            replacements: &["Array.prototype.slice"],
            url: None,
        },
    ),
    (
        "array.prototype.some",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.some",
            replacements: &["Array.prototype.some"],
            url: None,
        },
    ),
    (
        "array.prototype.splice",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.splice",
            replacements: &["Array.prototype.splice"],
            url: None,
        },
    ),
    (
        "array.prototype.toreversed",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.toreversed",
            replacements: &["Array.prototype.toReversed"],
            url: None,
        },
    ),
    (
        "array.prototype.tosorted",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.tosorted",
            replacements: &["Array.prototype.toSorted"],
            url: None,
        },
    ),
    (
        "array.prototype.unshift",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.unshift",
            replacements: &["Array.prototype.unshift"],
            url: None,
        },
    ),
    (
        "array.prototype.values",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "array.prototype.values",
            replacements: &["Array.prototype.values"],
            url: None,
        },
    ),
    (
        "arraybuffer.prototype.slice",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "arraybuffer.prototype.slice",
            replacements: &["ArrayBuffer.prototype.slice"],
            url: None,
        },
    ),
    (
        "arraybuffer.slice",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "arraybuffer.slice",
            replacements: &["ArrayBuffer.prototype.slice"],
            url: None,
        },
    ),
    (
        "arrify",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "arrify",
            replacements: &["snippet::array-coerce"],
            url: None,
        },
    ),
    (
        "as-array",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "as-array",
            replacements: &["snippet::array-coerce"],
            url: None,
        },
    ),
    (
        "async-function",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "async-function",
            replacements: &["snippet::async-function-constructor"],
            url: None,
        },
    ),
    (
        "asynciterator.prototype",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "asynciterator.prototype",
            replacements: &["AsyncIterator.prototype"],
            url: None,
        },
    ),
    (
        "atob",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "atob",
            replacements: &["atob"],
            url: None,
        },
    ),
    (
        "atob-lite",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "atob-lite",
            replacements: &["atob"],
            url: None,
        },
    ),
    (
        "axios",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "axios",
            replacements: &["fetch", "ofetch", "ky"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "fetch",
            }),
        },
    ),
    (
        "base64-js",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "base64-js",
            replacements: &["snippet::base64"],
            url: None,
        },
    ),
    (
        "base64id",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "base64id",
            replacements: &["snippet::base64-id"],
            url: None,
        },
    ),
    (
        "bcrypt",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "bcrypt",
            replacements: &["bcryptjs", "node:crypto", "crypto", "Bun.CryptoHasher"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "bcrypt",
            }),
        },
    ),
    (
        "bluebird",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "bluebird",
            replacements: &["Promise", "nativebird"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "bluebird-q",
            }),
        },
    ),
    (
        "body-parser",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "body-parser",
            replacements: &["milliparsec"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "body-parser",
            }),
        },
    ),
    (
        "btoa",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "btoa",
            replacements: &["btoa"],
            url: None,
        },
    ),
    (
        "buf-compare",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "buf-compare",
            replacements: &["Buffer.compare"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "buf-compare",
            }),
        },
    ),
    (
        "buffer-crc32",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "buffer-crc32",
            replacements: &["zlib.crc32"],
            url: None,
        },
    ),
    (
        "buffer-equal",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "buffer-equal",
            replacements: &["Buffer.prototype.equals"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "buffer-equal",
            }),
        },
    ),
    (
        "buffer-equal-constant-time",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "buffer-equal-constant-time",
            replacements: &["crypto.timingSafeEqual"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "buffer-equal-constant-time",
            }),
        },
    ),
    (
        "buffer-equals",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "buffer-equals",
            replacements: &["Buffer.prototype.equals"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "buffer-equals",
            }),
        },
    ),
    (
        "buffer-from",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "buffer-from",
            replacements: &["Buffer.from"],
            url: None,
        },
    ),
    (
        "builtin-modules",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "builtin-modules",
            replacements: &["builtinModules"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "builtin-modules",
            }),
        },
    ),
    (
        "call-bind",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "call-bind",
            replacements: &["snippet::call-bind"],
            url: None,
        },
    ),
    (
        "chalk",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "chalk",
            replacements: &["util.styleText", "picocolors", "ansis"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "chalk",
            }),
        },
    ),
    (
        "clean-webpack-plugin",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "clean-webpack-plugin",
            replacements: &["webpack.output.clean"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "clean-webpack-plugin",
            }),
        },
    ),
    (
        "cli-color",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "cli-color",
            replacements: &["util.styleText", "picocolors", "ansis"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "chalk",
            }),
        },
    ),
    (
        "clipboard",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "clipboard",
            replacements: &["ClipboardAPI"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "clipboardy",
            }),
        },
    ),
    (
        "clipboard-copy",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "clipboard-copy",
            replacements: &["ClipboardAPI"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "clipboardy",
            }),
        },
    ),
    (
        "clipboardy",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "clipboardy",
            replacements: &["tinyclip", "ClipboardAPI"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "clipboardy",
            }),
        },
    ),
    (
        "clone-regexp",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "clone-regexp",
            replacements: &["snippet::regexp-copy"],
            url: None,
        },
    ),
    (
        "collection-map",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "collection-map",
            replacements: &["Array.prototype.map", "Object.keys", "Object.values"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "collection-map",
            }),
        },
    ),
    (
        "colors",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "colors",
            replacements: &["util.styleText", "picocolors", "ansis"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "chalk",
            }),
        },
    ),
    (
        "commander",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "commander",
            replacements: &["sade", "cleye"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "cli-builders",
            }),
        },
    ),
    (
        "concat-map",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "concat-map",
            replacements: &["Array.prototype.flatMap"],
            url: None,
        },
    ),
    (
        "copy-text-to-clipboard",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "copy-text-to-clipboard",
            replacements: &["ClipboardAPI"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "clipboardy",
            }),
        },
    ),
    (
        "copy-to-clipboard",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "copy-to-clipboard",
            replacements: &["ClipboardAPI"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "clipboardy",
            }),
        },
    ),
    (
        "core-util-is",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "core-util-is",
            replacements: &["util.types"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "core-util-is",
            }),
        },
    ),
    (
        "cosmiconfig",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "cosmiconfig",
            replacements: &["lilconfig"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "cosmiconfig",
            }),
        },
    ),
    (
        "cpx",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "cpx",
            replacements: &["cpx2"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "cpx",
            }),
        },
    ),
    (
        "crc-32",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "crc-32",
            replacements: &["zlib.crc32"],
            url: None,
        },
    ),
    (
        "cross-fetch",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "cross-fetch",
            replacements: &["fetch", "ofetch", "ky"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "fetch",
            }),
        },
    ),
    (
        "crypto-js",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "crypto-js",
            replacements: &["node:crypto", "crypto", "Bun.CryptoHasher"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "crypto-js",
            }),
        },
    ),
    (
        "data-view-buffer",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "data-view-buffer",
            replacements: &["DataView.prototype.buffer"],
            url: None,
        },
    ),
    (
        "data-view-byte-length",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "data-view-byte-length",
            replacements: &["DataView.prototype.byteLength"],
            url: None,
        },
    ),
    (
        "data-view-byte-offset",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "data-view-byte-offset",
            replacements: &["DataView.prototype.byteOffset"],
            url: None,
        },
    ),
    (
        "date",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "date",
            replacements: &["Date"],
            url: None,
        },
    ),
    (
        "date-now",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "date-now",
            replacements: &["Date.now"],
            url: None,
        },
    ),
    (
        "dateformat",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "dateformat",
            replacements: &["Intl.DateTimeFormat"],
            url: None,
        },
    ),
    (
        "debug",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "debug",
            replacements: &["obug"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "debug",
            }),
        },
    ),
    (
        "deep-equal",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "deep-equal",
            replacements: &["dequal", "util.isDeepStrictEqual", "Bun.deepEquals"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "deep-equal",
            }),
        },
    ),
    (
        "deep-equal-json",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "deep-equal-json",
            replacements: &["dequal", "util.isDeepStrictEqual", "Bun.deepEquals"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "deep-equal",
            }),
        },
    ),
    (
        "defaults",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "defaults",
            replacements: &["Object.assign"],
            url: None,
        },
    ),
    (
        "define-accessor-property",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "define-accessor-property",
            replacements: &["Object.defineProperty"],
            url: None,
        },
    ),
    (
        "define-data-property",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "define-data-property",
            replacements: &["Object.defineProperty"],
            url: None,
        },
    ),
    (
        "define-properties",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "define-properties",
            replacements: &["Object.defineProperties"],
            url: None,
        },
    ),
    (
        "define-property",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "define-property",
            replacements: &["Object.defineProperty"],
            url: None,
        },
    ),
    (
        "depcheck",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "depcheck",
            replacements: &["knip"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "depcheck",
            }),
        },
    ),
    (
        "detect-package-manager",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "detect-package-manager",
            replacements: &["package-manager-detector"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "detect-package-manager",
            }),
        },
    ),
    (
        "dot-prop",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "dot-prop",
            replacements: &["dlv"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "dot-prop",
            }),
        },
    ),
    (
        "dotenv",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "dotenv",
            replacements: &["--env-file"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "dotenv",
            }),
        },
    ),
    (
        "dottie",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "dottie",
            replacements: &["dlv", "dset"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "dot-prop",
            }),
        },
    ),
    (
        "duplexer",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "duplexer",
            replacements: &["node:stream"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "duplexer",
            }),
        },
    ),
    (
        "duplexer2",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "duplexer2",
            replacements: &["node:stream"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "duplexer",
            }),
        },
    ),
    (
        "emoji-regex",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "emoji-regex",
            replacements: &["emoji-regex-xs", "unicodeClassEscape"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "emoji-regex",
            }),
        },
    ),
    (
        "error-cause",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "error-cause",
            replacements: &["Error.prototype.cause"],
            url: None,
        },
    ),
    (
        "es-aggregate-error",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "es-aggregate-error",
            replacements: &["AggregateError"],
            url: None,
        },
    ),
    (
        "es-create-array-iterator",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "es-create-array-iterator",
            replacements: &["Array.prototype.entries"],
            url: None,
        },
    ),
    (
        "es-define-property",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "es-define-property",
            replacements: &["Object.defineProperty"],
            url: None,
        },
    ),
    (
        "es-errors",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "es-errors",
            replacements: &["es-errors"],
            url: None,
        },
    ),
    (
        "es-get-iterator",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "es-get-iterator",
            replacements: &["snippet::get-iterator"],
            url: None,
        },
    ),
    (
        "es-map",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "es-map",
            replacements: &["Map"],
            url: None,
        },
    ),
    (
        "es-set",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "es-set",
            replacements: &["Set"],
            url: None,
        },
    ),
    (
        "es-set-tostringtag",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "es-set-tostringtag",
            replacements: &["snippet::set-tostringtag"],
            url: None,
        },
    ),
    (
        "es-shim-unscopables",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "es-shim-unscopables",
            replacements: &["Array.prototype[Symbol.unscopables]"],
            url: None,
        },
    ),
    (
        "es-string-html-methods",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "es-string-html-methods",
            replacements: &["es-string-html-methods"],
            url: None,
        },
    ),
    (
        "es5-shim",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "es5-shim",
            replacements: &["es5-shim"],
            url: None,
        },
    ),
    (
        "es6-error",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "es6-error",
            replacements: &["Error"],
            url: None,
        },
    ),
    (
        "es6-map",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "es6-map",
            replacements: &["Map"],
            url: None,
        },
    ),
    (
        "es6-promise",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "es6-promise",
            replacements: &["Promise"],
            url: None,
        },
    ),
    (
        "es6-set",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "es6-set",
            replacements: &["Set"],
            url: None,
        },
    ),
    (
        "es6-shim",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "es6-shim",
            replacements: &["es6-shim"],
            url: None,
        },
    ),
    (
        "es7-shim",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "es7-shim",
            replacements: &["es7-shim"],
            url: None,
        },
    ),
    (
        "escape-string-regexp",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "escape-string-regexp",
            replacements: &["RegExp.escape"],
            url: None,
        },
    ),
    (
        "eslint-plugin-es",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "eslint-plugin-es",
            replacements: &["eslint-plugin-es-x"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "eslint-plugin-es",
            }),
        },
    ),
    (
        "eslint-plugin-eslint-comments",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "eslint-plugin-eslint-comments",
            replacements: &["@eslint-community/eslint-plugin-eslint-comments"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "eslint-plugin-eslint-comments",
            }),
        },
    ),
    (
        "eslint-plugin-import",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "eslint-plugin-import",
            replacements: &["eslint-plugin-import-x"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "eslint-plugin-import",
            }),
        },
    ),
    (
        "eslint-plugin-node",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "eslint-plugin-node",
            replacements: &["eslint-plugin-n"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "eslint-plugin-node",
            }),
        },
    ),
    (
        "eslint-plugin-react",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "eslint-plugin-react",
            replacements: &["@eslint-react/eslint-plugin"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "eslint-plugin-react",
            }),
        },
    ),
    (
        "eslint-plugin-vitest",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "eslint-plugin-vitest",
            replacements: &["@vitest/eslint-plugin"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "eslint-plugin-vitest",
            }),
        },
    ),
    (
        "event-stream",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "event-stream",
            replacements: &["node:stream"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "event-stream",
            }),
        },
    ),
    (
        "event-target-shim",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "event-target-shim",
            replacements: &["EventTarget"],
            url: None,
        },
    ),
    (
        "execa",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "execa",
            replacements: &["tinyexec", "nanoexec", "Bun.Shell"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "execa",
            }),
        },
    ),
    (
        "express",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "express",
            replacements: &["h3", "tinyhttp", "hono", "elysia"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "express",
            }),
        },
    ),
    (
        "extend",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "extend",
            replacements: &["structuredClone", "defu"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "extend",
            }),
        },
    ),
    (
        "extend-shallow",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "extend-shallow",
            replacements: &["Object.assign"],
            url: None,
        },
    ),
    (
        "ez-spawn",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "ez-spawn",
            replacements: &["tinyexec"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "ez-spawn",
            }),
        },
    ),
    (
        "ezspawn",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "ezspawn",
            replacements: &["tinyexec"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "ez-spawn",
            }),
        },
    ),
    (
        "faker",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "faker",
            replacements: &["@faker-js/faker"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "faker",
            }),
        },
    ),
    (
        "fast-glob",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "fast-glob",
            replacements: &["tinyglobby"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "fast-glob",
            }),
        },
    ),
    (
        "feather-icons",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "feather-icons",
            replacements: &["lucide"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "feather",
            }),
        },
    ),
    (
        "filter-array",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "filter-array",
            replacements: &["Array.prototype.filter"],
            url: None,
        },
    ),
    (
        "filter-obj",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "filter-obj",
            replacements: &["snippet::object-filter"],
            url: None,
        },
    ),
    (
        "find-cache-dir",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "find-cache-dir",
            replacements: &["empathic"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "find-cache-dir",
            }),
        },
    ),
    (
        "find-cache-directory",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "find-cache-directory",
            replacements: &["empathic"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "find-cache-directory",
            }),
        },
    ),
    (
        "find-file-up",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "find-file-up",
            replacements: &["empathic"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "find-file-up",
            }),
        },
    ),
    (
        "find-index",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "find-index",
            replacements: &["Array.prototype.findIndex"],
            url: None,
        },
    ),
    (
        "find-pkg",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "find-pkg",
            replacements: &["empathic"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "find-pkg",
            }),
        },
    ),
    (
        "find-up",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "find-up",
            replacements: &["empathic", "pkg-types"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "find-up",
            }),
        },
    ),
    (
        "for-each",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "for-each",
            replacements: &["for...of"],
            url: None,
        },
    ),
    (
        "for-in",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "for-in",
            replacements: &["for...in"],
            url: None,
        },
    ),
    (
        "for-own",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "for-own",
            replacements: &["snippet::for-own"],
            url: None,
        },
    ),
    (
        "foreachasync",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "foreachasync",
            replacements: &["for...of"],
            url: None,
        },
    ),
    (
        "fs-extra",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "fs-extra",
            replacements: &["fs"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "fs-extra",
            }),
        },
    ),
    (
        "fs-then-native",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "fs-then-native",
            replacements: &["fsPromises"],
            url: None,
        },
    ),
    (
        "fs.extra",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "fs.extra",
            replacements: &["fs"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "fs-extra",
            }),
        },
    ),
    (
        "fs.realpath",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "fs.realpath",
            replacements: &["fs.realpath"],
            url: None,
        },
    ),
    (
        "function-bind",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "function-bind",
            replacements: &["Function.prototype.bind"],
            url: None,
        },
    ),
    (
        "function.prototype.name",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "function.prototype.name",
            replacements: &["Function.prototype.name"],
            url: None,
        },
    ),
    (
        "functions-have-names",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "functions-have-names",
            replacements: &["Function.prototype.name"],
            url: None,
        },
    ),
    (
        "gaxios",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "gaxios",
            replacements: &["fetch", "ofetch", "ky"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "fetch",
            }),
        },
    ),
    (
        "get-proto",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "get-proto",
            replacements: &["Object.getPrototypeOf"],
            url: None,
        },
    ),
    (
        "get-stream",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "get-stream",
            replacements: &["Buffer"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "get-stream",
            }),
        },
    ),
    (
        "get-symbol-description",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "get-symbol-description",
            replacements: &["Symbol.prototype.description"],
            url: None,
        },
    ),
    (
        "get-value",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "get-value",
            replacements: &["dlv"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "dot-prop",
            }),
        },
    ),
    (
        "glob",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "glob",
            replacements: &["tinyglobby", "fs.glob", "fdir"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "glob",
            }),
        },
    ),
    (
        "global",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "global",
            replacements: &["globalThis"],
            url: None,
        },
    ),
    (
        "globalthis",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "globalthis",
            replacements: &["globalThis"],
            url: None,
        },
    ),
    (
        "globby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "globby",
            replacements: &["tinyglobby"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "globby",
            }),
        },
    ),
    (
        "globule",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "globule",
            replacements: &["tinyglobby"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "globby",
            }),
        },
    ),
    (
        "gopd",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "gopd",
            replacements: &["Object.getOwnPropertyDescriptor"],
            url: None,
        },
    ),
    (
        "grapheme-splitter",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "grapheme-splitter",
            replacements: &["Intl.Segmenter", "unicode-segmenter"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "graphemer",
            }),
        },
    ),
    (
        "graphemer",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "graphemer",
            replacements: &["Intl.Segmenter", "unicode-segmenter"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "graphemer",
            }),
        },
    ),
    (
        "graphviz",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "graphviz",
            replacements: &["ts-graphviz"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "graphviz",
            }),
        },
    ),
    (
        "gzip-size",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "gzip-size",
            replacements: &["zlib.gzipSync"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "gzip-size",
            }),
        },
    ),
    (
        "has",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "has",
            replacements: &["Object.prototype.hasOwnProperty"],
            url: None,
        },
    ),
    (
        "has-ansi",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "has-ansi",
            replacements: &["snippet::has-ansi"],
            url: None,
        },
    ),
    (
        "has-bigints",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "has-bigints",
            replacements: &["has-bigints"],
            url: None,
        },
    ),
    (
        "has-dynamic-import",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "has-dynamic-import",
            replacements: &["has-dynamic-import"],
            url: None,
        },
    ),
    (
        "has-flag",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "has-flag",
            replacements: &["snippet::has-argv"],
            url: None,
        },
    ),
    (
        "has-optional-chaining",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "has-optional-chaining",
            replacements: &["has-optional-chaining"],
            url: None,
        },
    ),
    (
        "has-own-prop",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "has-own-prop",
            replacements: &["Object.prototype.hasOwnProperty"],
            url: None,
        },
    ),
    (
        "has-property-descriptors",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "has-property-descriptors",
            replacements: &["has-property-descriptors"],
            url: None,
        },
    ),
    (
        "has-proto",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "has-proto",
            replacements: &["has-proto"],
            url: None,
        },
    ),
    (
        "has-symbols",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "has-symbols",
            replacements: &["has-symbols"],
            url: None,
        },
    ),
    (
        "has-template-literals",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "has-template-literals",
            replacements: &["has-template-literals"],
            url: None,
        },
    ),
    (
        "has-tostringtag",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "has-tostringtag",
            replacements: &["has-tostringtag"],
            url: None,
        },
    ),
    (
        "has-typed-arrays",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "has-typed-arrays",
            replacements: &["has-typed-arrays"],
            url: None,
        },
    ),
    (
        "hasown",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "hasown",
            replacements: &["Object.prototype.hasOwnProperty"],
            url: None,
        },
    ),
    (
        "ieee754",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "ieee754",
            replacements: &["DataView"],
            url: None,
        },
    ),
    (
        "index-of",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "index-of",
            replacements: &["Array.prototype.indexOf"],
            url: None,
        },
    ),
    (
        "indexof",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "indexof",
            replacements: &["Array.prototype.indexOf"],
            url: None,
        },
    ),
    (
        "inherits",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "inherits",
            replacements: &["extends", "util.inherits"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "inherits",
            }),
        },
    ),
    (
        "invariant",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "invariant",
            replacements: &["tiny-invariant"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "invariant",
            }),
        },
    ),
    (
        "iota-array",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "iota-array",
            replacements: &["snippet::array-from-count"],
            url: None,
        },
    ),
    (
        "is-arguments",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-arguments",
            replacements: &["snippet::is-arguments"],
            url: None,
        },
    ),
    (
        "is-array-buffer",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-array-buffer",
            replacements: &["snippet::is-arraybuffer"],
            url: None,
        },
    ),
    (
        "is-arrayish",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-arrayish",
            replacements: &["Array.isArray"],
            url: None,
        },
    ),
    (
        "is-async-function",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-async-function",
            replacements: &["snippet::is-async-function"],
            url: None,
        },
    ),
    (
        "is-bigint",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-bigint",
            replacements: &["snippet::is-bigint"],
            url: None,
        },
    ),
    (
        "is-boolean-object",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-boolean-object",
            replacements: &["snippet::is-boolean"],
            url: None,
        },
    ),
    (
        "is-buffer",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-buffer",
            replacements: &["Buffer.isBuffer"],
            url: None,
        },
    ),
    (
        "is-builtin-module",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-builtin-module",
            replacements: &["isBuiltin", "builtinModules"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "is-builtin-module",
            }),
        },
    ),
    (
        "is-ci",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-ci",
            replacements: &["snippet::is-ci"],
            url: None,
        },
    ),
    (
        "is-core-module",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-core-module",
            replacements: &["isBuiltin", "builtinModules"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "is-builtin-module",
            }),
        },
    ),
    (
        "is-date-object",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-date-object",
            replacements: &["snippet::is-date"],
            url: None,
        },
    ),
    (
        "is-error",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-error",
            replacements: &["Error.isError"],
            url: None,
        },
    ),
    (
        "is-even",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-even",
            replacements: &["snippet::is-even"],
            url: None,
        },
    ),
    (
        "is-finite",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-finite",
            replacements: &["Number.isFinite"],
            url: None,
        },
    ),
    (
        "is-generator-function",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-generator-function",
            replacements: &["snippet::is-generator-function"],
            url: None,
        },
    ),
    (
        "is-in-ssh",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-in-ssh",
            replacements: &["snippet::is-in-ssh"],
            url: None,
        },
    ),
    (
        "is-nan",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-nan",
            replacements: &["Number.isNaN"],
            url: None,
        },
    ),
    (
        "is-negative",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-negative",
            replacements: &["snippet::is-negative"],
            url: None,
        },
    ),
    (
        "is-negative-zero",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-negative-zero",
            replacements: &["snippet::is-negative-zero"],
            url: None,
        },
    ),
    (
        "is-npm",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-npm",
            replacements: &["snippet::is-npm"],
            url: None,
        },
    ),
    (
        "is-number",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-number",
            replacements: &["snippet::is-number"],
            url: None,
        },
    ),
    (
        "is-number-object",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-number-object",
            replacements: &["snippet::is-number"],
            url: None,
        },
    ),
    (
        "is-obj",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-obj",
            replacements: &["snippet::is-object-or-function"],
            url: None,
        },
    ),
    (
        "is-object",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-object",
            replacements: &["snippet::is-object"],
            url: None,
        },
    ),
    (
        "is-odd",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-odd",
            replacements: &["snippet::is-odd"],
            url: None,
        },
    ),
    (
        "is-plain-obj",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-plain-obj",
            replacements: &["snippet::is-object"],
            url: None,
        },
    ),
    (
        "is-plain-object",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-plain-object",
            replacements: &["snippet::is-object"],
            url: None,
        },
    ),
    (
        "is-primitive",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-primitive",
            replacements: &["snippet::is-primitve"],
            url: None,
        },
    ),
    (
        "is-regex",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-regex",
            replacements: &["snippet::is-regexp"],
            url: None,
        },
    ),
    (
        "is-regexp",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-regexp",
            replacements: &["snippet::is-regexp"],
            url: None,
        },
    ),
    (
        "is-stream",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-stream",
            replacements: &["snippet::is-stream"],
            url: None,
        },
    ),
    (
        "is-string",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-string",
            replacements: &["snippet::is-string"],
            url: None,
        },
    ),
    (
        "is-symbol",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-symbol",
            replacements: &["snippet::is-symbol"],
            url: None,
        },
    ),
    (
        "is-travis",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-travis",
            replacements: &["snippet::is-travis"],
            url: None,
        },
    ),
    (
        "is-url-superb",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-url-superb",
            replacements: &["URL.canParse"],
            url: None,
        },
    ),
    (
        "is-whitespace",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-whitespace",
            replacements: &["snippet::is-whitespace"],
            url: None,
        },
    ),
    (
        "is-windows",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "is-windows",
            replacements: &["snippet::is-windows"],
            url: None,
        },
    ),
    (
        "isarray",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "isarray",
            replacements: &["Array.isArray"],
            url: None,
        },
    ),
    (
        "iserror",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "iserror",
            replacements: &["Error.isError"],
            url: None,
        },
    ),
    (
        "isobject",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "isobject",
            replacements: &["snippet::is-object"],
            url: None,
        },
    ),
    (
        "isomorphic-fetch",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "isomorphic-fetch",
            replacements: &["fetch", "ofetch", "ky"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "fetch",
            }),
        },
    ),
    (
        "isstream",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "isstream",
            replacements: &["snippet::is-stream"],
            url: None,
        },
    ),
    (
        "iterate-iterator",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "iterate-iterator",
            replacements: &["for...of"],
            url: None,
        },
    ),
    (
        "iterate-value",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "iterate-value",
            replacements: &["for...of"],
            url: None,
        },
    ),
    (
        "iterator.prototype",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "iterator.prototype",
            replacements: &["Iterator.prototype"],
            url: None,
        },
    ),
    (
        "jquery",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "jquery",
            replacements: &["jquery"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "jquery",
            }),
        },
    ),
    (
        "js-base64",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "js-base64",
            replacements: &["snippet::base64"],
            url: None,
        },
    ),
    (
        "js-yaml",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "js-yaml",
            replacements: &["yaml", "Bun.YAML"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "js-yaml",
            }),
        },
    ),
    (
        "jsonfile",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "jsonfile",
            replacements: &["snippet::json-file"],
            url: None,
        },
    ),
    (
        "jsonwebtoken",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "jsonwebtoken",
            replacements: &["jose"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "jsonwebtoken",
            }),
        },
    ),
    (
        "jsx-ast-utils",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "jsx-ast-utils",
            replacements: &["jsx-ast-utils-x"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "jsx-ast-utils",
            }),
        },
    ),
    (
        "kind-of",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "kind-of",
            replacements: &["snippet::typeof"],
            url: None,
        },
    ),
    (
        "last-char",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "last-char",
            replacements: &["snippet::char-last"],
            url: None,
        },
    ),
    (
        "last-index-of",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "last-index-of",
            replacements: &["Array.prototype.lastIndexOf"],
            url: None,
        },
    ),
    (
        "left-pad",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "left-pad",
            replacements: &["String.prototype.padStart"],
            url: None,
        },
    ),
    (
        "lint-staged",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lint-staged",
            replacements: &["nano-staged"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lint-staged",
            }),
        },
    ),
    (
        "lodash",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash-amd",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash-amd",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash-compat",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash-compat",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash-es",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash-es",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash-fp",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash-fp",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash-node",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash-node",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.add",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.add",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.after",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.after",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.ary",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.ary",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.assign",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.assign",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.assignin",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.assignin",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.assigninwith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.assigninwith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.assignwith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.assignwith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.at",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.at",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.attempt",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.attempt",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.before",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.before",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.bind",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.bind",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.bindall",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.bindall",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.bindkey",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.bindkey",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.callback",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.callback",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.camelcase",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.camelcase",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.capitalize",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.capitalize",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.castarray",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.castarray",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.ceil",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.ceil",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.chunk",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.chunk",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.clamp",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.clamp",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.clone",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.clone",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.clonedeep",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.clonedeep",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.clonedeepwith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.clonedeepwith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.clonewith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.clonewith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.compact",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.compact",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.compose",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.compose",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.concat",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.concat",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.cond",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.cond",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.conforms",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.conforms",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.conformsto",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.conformsto",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.constant",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.constant",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.contains",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.contains",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.countby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.countby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.create",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.create",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.createcallback",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.createcallback",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.curry",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.curry",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.curryright",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.curryright",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.debounce",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.debounce",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.deburr",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.deburr",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.defaults",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.defaults",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.defaultsdeep",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.defaultsdeep",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.defaultto",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.defaultto",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.defer",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.defer",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.delay",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.delay",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.difference",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.difference",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.differenceby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.differenceby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.differencewith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.differencewith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.divide",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.divide",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.drop",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.drop",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.dropright",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.dropright",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.droprightwhile",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.droprightwhile",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.dropwhile",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.dropwhile",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.endswith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.endswith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.eq",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.eq",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.escape",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.escape",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.escaperegexp",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.escaperegexp",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.every",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.every",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.fill",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.fill",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.filter",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.filter",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.find",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.find",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.findindex",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.findindex",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.findkey",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.findkey",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.findlast",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.findlast",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.findlastindex",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.findlastindex",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.findlastkey",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.findlastkey",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.findwhere",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.findwhere",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.first",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.first",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.flatmap",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.flatmap",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.flatmapdeep",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.flatmapdeep",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.flatmapdepth",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.flatmapdepth",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.flatten",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.flatten",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.flattendeep",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.flattendeep",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.flattendepth",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.flattendepth",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.flip",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.flip",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.floor",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.floor",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.flow",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.flow",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.flowright",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.flowright",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.foreach",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.foreach",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.foreachright",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.foreachright",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.forin",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.forin",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.forinright",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.forinright",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.forown",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.forown",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.forownright",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.forownright",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.frompairs",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.frompairs",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.functions",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.functions",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.functionsin",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.functionsin",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.get",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.get",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.groupby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.groupby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.gt",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.gt",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.gte",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.gte",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.has",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.has",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.hasin",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.hasin",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.head",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.head",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.identity",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.identity",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.includes",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.includes",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.indexby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.indexby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.indexof",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.indexof",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.initial",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.initial",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.inrange",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.inrange",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.intersection",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.intersection",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.intersectionby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.intersectionby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.intersectionwith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.intersectionwith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.invert",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.invert",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.invertby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.invertby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.invoke",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.invoke",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.invokemap",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.invokemap",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.isarguments",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isarguments",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.isarray",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isarray",
            replacements: &["Array.isArray"],
            url: None,
        },
    ),
    (
        "lodash.isarraybuffer",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isarraybuffer",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.isarraylike",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isarraylike",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.isarraylikeobject",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isarraylikeobject",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.isboolean",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isboolean",
            replacements: &["snippet::is-boolean"],
            url: None,
        },
    ),
    (
        "lodash.isbuffer",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isbuffer",
            replacements: &["Buffer.isBuffer"],
            url: None,
        },
    ),
    (
        "lodash.isdate",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isdate",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.iselement",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.iselement",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.isempty",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isempty",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.isequal",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isequal",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.isequalwith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isequalwith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.iserror",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.iserror",
            replacements: &["Error.isError"],
            url: None,
        },
    ),
    (
        "lodash.isfinite",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isfinite",
            replacements: &["Number.isFinite"],
            url: None,
        },
    ),
    (
        "lodash.isfunction",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isfunction",
            replacements: &["snippet::is-function"],
            url: None,
        },
    ),
    (
        "lodash.isinteger",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isinteger",
            replacements: &["Number.isInteger"],
            url: None,
        },
    ),
    (
        "lodash.islength",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.islength",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.ismap",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.ismap",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.ismatch",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.ismatch",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.ismatchwith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.ismatchwith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.isnan",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isnan",
            replacements: &["Number.isNaN"],
            url: None,
        },
    ),
    (
        "lodash.isnative",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isnative",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.isnil",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isnil",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.isnull",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isnull",
            replacements: &["snippet::is-null"],
            url: None,
        },
    ),
    (
        "lodash.isnumber",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isnumber",
            replacements: &["snippet::is-number"],
            url: None,
        },
    ),
    (
        "lodash.isobject",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isobject",
            replacements: &["snippet::is-object"],
            url: None,
        },
    ),
    (
        "lodash.isobjectlike",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isobjectlike",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.isplainobject",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isplainobject",
            replacements: &["snippet::is-object"],
            url: None,
        },
    ),
    (
        "lodash.isregexp",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isregexp",
            replacements: &["snippet::is-regexp"],
            url: None,
        },
    ),
    (
        "lodash.issafeinteger",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.issafeinteger",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.isset",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isset",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.isstring",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isstring",
            replacements: &["snippet::is-string"],
            url: None,
        },
    ),
    (
        "lodash.issymbol",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.issymbol",
            replacements: &["snippet::is-symbol"],
            url: None,
        },
    ),
    (
        "lodash.istypedarray",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.istypedarray",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.isundefined",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isundefined",
            replacements: &["snippet::is-undefined"],
            url: None,
        },
    ),
    (
        "lodash.isweakmap",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isweakmap",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.isweakset",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.isweakset",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.iteratee",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.iteratee",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.join",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.join",
            replacements: &["Array.prototype.join"],
            url: None,
        },
    ),
    (
        "lodash.kebabcase",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.kebabcase",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.keyby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.keyby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.keys",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.keys",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.keysin",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.keysin",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.last",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.last",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.lastindexof",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.lastindexof",
            replacements: &["Array.prototype.lastIndexOf"],
            url: None,
        },
    ),
    (
        "lodash.lowercase",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.lowercase",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.lowerfirst",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.lowerfirst",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.lt",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.lt",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.lte",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.lte",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.map",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.map",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.mapkeys",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.mapkeys",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.mapvalues",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.mapvalues",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.matches",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.matches",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.matchesproperty",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.matchesproperty",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.max",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.max",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.maxby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.maxby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.mean",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.mean",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.meanby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.meanby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.memoize",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.memoize",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.merge",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.merge",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.mergewith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.mergewith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.method",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.method",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.methodof",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.methodof",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.min",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.min",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.minby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.minby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.mixin",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.mixin",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.modargs",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.modargs",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.multiply",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.multiply",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.negate",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.negate",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.noop",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.noop",
            replacements: &["snippet::noop"],
            url: None,
        },
    ),
    (
        "lodash.now",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.now",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.nth",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.nth",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.ntharg",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.ntharg",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.omit",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.omit",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.omitby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.omitby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.once",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.once",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.orderby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.orderby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.over",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.over",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.overargs",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.overargs",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.overevery",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.overevery",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.oversome",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.oversome",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.pad",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.pad",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.padend",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.padend",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.padleft",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.padleft",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.padright",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.padright",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.padstart",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.padstart",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.pairs",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.pairs",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.parseint",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.parseint",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.partial",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.partial",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.partialright",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.partialright",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.partition",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.partition",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.pick",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.pick",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.pickby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.pickby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.pluck",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.pluck",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.property",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.property",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.propertyof",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.propertyof",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.pull",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.pull",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.pullall",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.pullall",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.pullallwith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.pullallwith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.pullat",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.pullat",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.random",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.random",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.range",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.range",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.rangeright",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.rangeright",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.rearg",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.rearg",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.reduce",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.reduce",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.reduceright",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.reduceright",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.reescape",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.reescape",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.reevaluate",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.reevaluate",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.reinterpolate",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.reinterpolate",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.reject",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.reject",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.remove",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.remove",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.repeat",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.repeat",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.replace",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.replace",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.rest",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.rest",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.restparam",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.restparam",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.result",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.result",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.reverse",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.reverse",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.round",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.round",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.sample",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.sample",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.samplesize",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.samplesize",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.set",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.set",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.setwith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.setwith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.shuffle",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.shuffle",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.size",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.size",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.slice",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.slice",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.snakecase",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.snakecase",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.some",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.some",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.sortby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.sortby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.sortbyall",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.sortbyall",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.sortbyorder",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.sortbyorder",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.sortedindex",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.sortedindex",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.sortedindexby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.sortedindexby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.sortedindexof",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.sortedindexof",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.sortedlastindex",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.sortedlastindex",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.sortedlastindexby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.sortedlastindexby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.sortedlastindexof",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.sortedlastindexof",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.sorteduniq",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.sorteduniq",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.sorteduniqby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.sorteduniqby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.split",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.split",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.spread",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.spread",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.startcase",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.startcase",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.startswith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.startswith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.stubarray",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.stubarray",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.stubfalse",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.stubfalse",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.stubobject",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.stubobject",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.stubstring",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.stubstring",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.stubtrue",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.stubtrue",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.subtract",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.subtract",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.sum",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.sum",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.sumby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.sumby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.support",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.support",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.tail",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.tail",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.take",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.take",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.takeright",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.takeright",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.takerightwhile",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.takerightwhile",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.takewhile",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.takewhile",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.template",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.template",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.templatesettings",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.templatesettings",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.throttle",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.throttle",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.times",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.times",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.toarray",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.toarray",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.tofinite",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.tofinite",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.tointeger",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.tointeger",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.tolength",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.tolength",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.tolower",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.tolower",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.tonumber",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.tonumber",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.topairs",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.topairs",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.topairsin",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.topairsin",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.topath",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.topath",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.toplainobject",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.toplainobject",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.tosafeinteger",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.tosafeinteger",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.tostring",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.tostring",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.toupper",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.toupper",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.transform",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.transform",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.trim",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.trim",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.trimend",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.trimend",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.trimleft",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.trimleft",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.trimright",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.trimright",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.trimstart",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.trimstart",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.trunc",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.trunc",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.truncate",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.truncate",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.unary",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.unary",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.unescape",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.unescape",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.union",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.union",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.unionby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.unionby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.unionwith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.unionwith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.uniqby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.uniqby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.uniqueid",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.uniqueid",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.uniqwith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.uniqwith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.unset",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.unset",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.unzip",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.unzip",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.unzipwith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.unzipwith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.update",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.update",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.updatewith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.updatewith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.uppercase",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.uppercase",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.upperfirst",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.upperfirst",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.values",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.values",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.valuesin",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.valuesin",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.where",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.where",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.without",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.without",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.words",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.words",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.wrap",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.wrap",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.xor",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.xor",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.xorby",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.xorby",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.xorwith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.xorwith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.zip",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.zip",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.zipobject",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.zipobject",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.zipobjectdeep",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.zipobjectdeep",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "lodash.zipwith",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lodash.zipwith",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "long",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "long",
            replacements: &["BigInt"],
            url: None,
        },
    ),
    (
        "lower-case",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "lower-case",
            replacements: &["snippet::to-lower"],
            url: None,
        },
    ),
    (
        "make-dir",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "make-dir",
            replacements: &["fs.mkdir"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "mkdirp",
            }),
        },
    ),
    (
        "map-obj",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "map-obj",
            replacements: &["snippet::object-map"],
            url: None,
        },
    ),
    (
        "materialize-css",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "materialize-css",
            replacements: &["@materializecss/materialize", "@material/web"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "materialize-css",
            }),
        },
    ),
    (
        "math-log2",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "math-log2",
            replacements: &["Math.log2"],
            url: None,
        },
    ),
    (
        "math-random",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "math-random",
            replacements: &["snippet::math-random"],
            url: None,
        },
    ),
    (
        "math.acosh",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "math.acosh",
            replacements: &["Math.acosh"],
            url: None,
        },
    ),
    (
        "math.atanh",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "math.atanh",
            replacements: &["Math.atanh"],
            url: None,
        },
    ),
    (
        "math.cbrt",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "math.cbrt",
            replacements: &["Math.cbrt"],
            url: None,
        },
    ),
    (
        "math.clz32",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "math.clz32",
            replacements: &["Math.clz32"],
            url: None,
        },
    ),
    (
        "math.f16round",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "math.f16round",
            replacements: &["Math.f16round"],
            url: None,
        },
    ),
    (
        "math.fround",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "math.fround",
            replacements: &["Math.fround"],
            url: None,
        },
    ),
    (
        "math.imul",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "math.imul",
            replacements: &["Math.imul"],
            url: None,
        },
    ),
    (
        "math.log10",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "math.log10",
            replacements: &["Math.log10"],
            url: None,
        },
    ),
    (
        "math.log1p",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "math.log1p",
            replacements: &["Math.log1p"],
            url: None,
        },
    ),
    (
        "math.sign",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "math.sign",
            replacements: &["Math.sign"],
            url: None,
        },
    ),
    (
        "md5",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "md5",
            replacements: &["node:crypto"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "md5",
            }),
        },
    ),
    (
        "meow",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "meow",
            replacements: &["sade", "cleye"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "cli-builders",
            }),
        },
    ),
    (
        "minimist",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "minimist",
            replacements: &["util.parseArgs", "mri"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "parseargs",
            }),
        },
    ),
    (
        "mkdirp",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "mkdirp",
            replacements: &["fs.mkdir"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "mkdirp",
            }),
        },
    ),
    (
        "mockdate",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "mockdate",
            replacements: &["node:test", "vitest", "bun:test"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "mockdate",
            }),
        },
    ),
    (
        "moment",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "moment",
            replacements: &["day.js", "date-fns", "luxon", "Date"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "moment",
            }),
        },
    ),
    (
        "native-promise-only",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "native-promise-only",
            replacements: &["Promise"],
            url: None,
        },
    ),
    (
        "node-environment-flags",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "node-environment-flags",
            replacements: &["process.allowedNodeEnvironmentFlags"],
            url: None,
        },
    ),
    (
        "node-fetch",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "node-fetch",
            replacements: &["fetch", "ofetch", "ky"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "fetch",
            }),
        },
    ),
    (
        "node-int64",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "node-int64",
            replacements: &["BigInt"],
            url: None,
        },
    ),
    (
        "node-telegram-bot-api",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "node-telegram-bot-api",
            replacements: &["grammy"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "node-telegram-bot-api",
            }),
        },
    ),
    (
        "node.extend",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "node.extend",
            replacements: &["Object.assign"],
            url: None,
        },
    ),
    (
        "npm-run-all",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "npm-run-all",
            replacements: &["npm-run-all2", "concurrently", "wireit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "npm-run-all",
            }),
        },
    ),
    (
        "number-is-integer",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "number-is-integer",
            replacements: &["Number.isInteger"],
            url: None,
        },
    ),
    (
        "number.isfinite",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "number.isfinite",
            replacements: &["Number.isFinite"],
            url: None,
        },
    ),
    (
        "number.isinteger",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "number.isinteger",
            replacements: &["Number.isInteger"],
            url: None,
        },
    ),
    (
        "number.isnan",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "number.isnan",
            replacements: &["Number.isNaN"],
            url: None,
        },
    ),
    (
        "number.issafeinteger",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "number.issafeinteger",
            replacements: &["Number.isSafeInteger"],
            url: None,
        },
    ),
    (
        "number.parsefloat",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "number.parsefloat",
            replacements: &["Number.parseFloat"],
            url: None,
        },
    ),
    (
        "number.parseint",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "number.parseint",
            replacements: &["Number.parseInt"],
            url: None,
        },
    ),
    (
        "number.prototype.toexponential",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "number.prototype.toexponential",
            replacements: &["Number.prototype.toExponential"],
            url: None,
        },
    ),
    (
        "object-assign",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "object-assign",
            replacements: &["Object.assign"],
            url: None,
        },
    ),
    (
        "object-hash",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "object-hash",
            replacements: &["ohash", "crypto", "Bun.CryptoHasher"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "object-hash",
            }),
        },
    ),
    (
        "object-is",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "object-is",
            replacements: &["Object.is"],
            url: None,
        },
    ),
    (
        "object-keys",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "object-keys",
            replacements: &["Object.keys"],
            url: None,
        },
    ),
    (
        "object-path",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "object-path",
            replacements: &["dlv", "dset"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "dot-prop",
            }),
        },
    ),
    (
        "object.assign",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "object.assign",
            replacements: &["Object.assign"],
            url: None,
        },
    ),
    (
        "object.defineproperties",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "object.defineproperties",
            replacements: &["Object.defineProperties"],
            url: None,
        },
    ),
    (
        "object.entries",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "object.entries",
            replacements: &["Object.entries"],
            url: None,
        },
    ),
    (
        "object.fromentries",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "object.fromentries",
            replacements: &["Object.fromEntries"],
            url: None,
        },
    ),
    (
        "object.getownpropertydescriptors",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "object.getownpropertydescriptors",
            replacements: &["Object.getOwnPropertyDescriptors"],
            url: None,
        },
    ),
    (
        "object.getprototypeof",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "object.getprototypeof",
            replacements: &["Object.getPrototypeOf"],
            url: None,
        },
    ),
    (
        "object.hasown",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "object.hasown",
            replacements: &["Object.hasOwn"],
            url: None,
        },
    ),
    (
        "object.keys",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "object.keys",
            replacements: &["Object.keys"],
            url: None,
        },
    ),
    (
        "object.map",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "object.map",
            replacements: &["snippet::object-map"],
            url: None,
        },
    ),
    (
        "object.reduce",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "object.reduce",
            replacements: &["snippet::object-reduce"],
            url: None,
        },
    ),
    (
        "object.values",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "object.values",
            replacements: &["Object.values"],
            url: None,
        },
    ),
    (
        "ora",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "ora",
            replacements: &["nanospinner", "picospinner"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "ora",
            }),
        },
    ),
    (
        "package-up",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "package-up",
            replacements: &["empathic", "pkg-types"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "find-up",
            }),
        },
    ),
    (
        "pad-left",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "pad-left",
            replacements: &["String.prototype.padStart"],
            url: None,
        },
    ),
    (
        "parseint",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "parseint",
            replacements: &["parseInt"],
            url: None,
        },
    ),
    (
        "path-exists",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "path-exists",
            replacements: &["fs.access", "fs.existsSync", "Bun.file"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "path-exists",
            }),
        },
    ),
    (
        "path-parse",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "path-parse",
            replacements: &["path.parse"],
            url: None,
        },
    ),
    (
        "pbkdf2",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "pbkdf2",
            replacements: &["node:crypto", "crypto"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "pbkdf2",
            }),
        },
    ),
    (
        "pinkie-promise",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "pinkie-promise",
            replacements: &["Promise"],
            url: None,
        },
    ),
    (
        "pkg-dir",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "pkg-dir",
            replacements: &["empathic"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "pkg-dir",
            }),
        },
    ),
    (
        "pkg-up",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "pkg-up",
            replacements: &["empathic", "pkg-types"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "find-up",
            }),
        },
    ),
    (
        "portal-vue",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "portal-vue",
            replacements: &["Teleport"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "portal-vue",
            }),
        },
    ),
    (
        "portfinder",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "portfinder",
            replacements: &["get-port"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "portfinder",
            }),
        },
    ),
    (
        "preferred-pm",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "preferred-pm",
            replacements: &["package-manager-detector"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "detect-package-manager",
            }),
        },
    ),
    (
        "promise.allsettled",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "promise.allsettled",
            replacements: &["Promise.allSettled"],
            url: None,
        },
    ),
    (
        "promise.any",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "promise.any",
            replacements: &["Promise.any"],
            url: None,
        },
    ),
    (
        "promise.prototype.finally",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "promise.prototype.finally",
            replacements: &["Promise.prototype.finally"],
            url: None,
        },
    ),
    (
        "promish",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "promish",
            replacements: &["Promise"],
            url: None,
        },
    ),
    (
        "q",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "q",
            replacements: &["Promise", "nativebird"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "bluebird-q",
            }),
        },
    ),
    (
        "qs",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "qs",
            replacements: &["URLSearchParams", "fast-querystring", "picoquery", "neoqs"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "qs",
            }),
        },
    ),
    (
        "querystring",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "querystring",
            replacements: &["URLSearchParams", "fast-querystring", "picoquery", "neoqs"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "qs",
            }),
        },
    ),
    (
        "queue-microtask",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "queue-microtask",
            replacements: &["queueMicrotask"],
            url: None,
        },
    ),
    (
        "queue-tick",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "queue-tick",
            replacements: &["queueMicrotask"],
            url: None,
        },
    ),
    (
        "raf",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "raf",
            replacements: &["requestAnimationFrame"],
            url: None,
        },
    ),
    (
        "random-bytes",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "random-bytes",
            replacements: &["crypto.randomBytes"],
            url: None,
        },
    ),
    (
        "randomatic",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "randomatic",
            replacements: &["nanoid"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "shortid",
            }),
        },
    ),
    (
        "react-feather",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "react-feather",
            replacements: &["lucide-react"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "feather",
            }),
        },
    ),
    (
        "react-helmet",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "react-helmet",
            replacements: &["react:metadata", "react-helmet-async"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "react-helmet",
            }),
        },
    ),
    (
        "read-package-up",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "read-package-up",
            replacements: &["pkg-types", "empathic"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "read-package-up",
            }),
        },
    ),
    (
        "read-pkg",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "read-pkg",
            replacements: &["pkg-types", "node:fs"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "read-pkg",
            }),
        },
    ),
    (
        "read-pkg-up",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "read-pkg-up",
            replacements: &["pkg-types", "empathic"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "read-pkg-up",
            }),
        },
    ),
    (
        "readable-stream",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "readable-stream",
            replacements: &["node:stream", "streams"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "readable-stream",
            }),
        },
    ),
    (
        "reflect.getprototypeof",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "reflect.getprototypeof",
            replacements: &["Reflect.getPrototypeOf"],
            url: None,
        },
    ),
    (
        "reflect.ownkeys",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "reflect.ownkeys",
            replacements: &["Reflect.ownKeys"],
            url: None,
        },
    ),
    (
        "regexp.prototype.flags",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "regexp.prototype.flags",
            replacements: &["RegExp.prototype.flags"],
            url: None,
        },
    ),
    (
        "remove",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "remove",
            replacements: &["fs.rm"],
            url: None,
        },
    ),
    (
        "repeat-string",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "repeat-string",
            replacements: &["String.prototype.repeat"],
            url: None,
        },
    ),
    (
        "request",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "request",
            replacements: &["fetch", "ofetch", "ky"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "fetch",
            }),
        },
    ),
    (
        "resolve",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "resolve",
            replacements: &["import.meta.resolve", "exsolve", "oxc-resolver"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "resolve",
            }),
        },
    ),
    (
        "retry-request",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "retry-request",
            replacements: &["ofetch", "ky"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "fetch",
            }),
        },
    ),
    (
        "rimraf",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "rimraf",
            replacements: &["fs.rm", "fs.rmdir", "premove"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "rimraf",
            }),
        },
    ),
    (
        "safe-buffer",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "safe-buffer",
            replacements: &["safe-buffer"],
            url: None,
        },
    ),
    (
        "safe-compare",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "safe-compare",
            replacements: &["crypto.timingSafeEqual"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "buffer-equal-constant-time",
            }),
        },
    ),
    (
        "safer-buffer",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "safer-buffer",
            replacements: &["safe-buffer"],
            url: None,
        },
    ),
    (
        "scmp",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "scmp",
            replacements: &["crypto.timingSafeEqual"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "buffer-equal-constant-time",
            }),
        },
    ),
    (
        "secure-compare",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "secure-compare",
            replacements: &["crypto.timingSafeEqual"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "buffer-equal-constant-time",
            }),
        },
    ),
    (
        "set-value",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "set-value",
            replacements: &["dset"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "dot-prop",
            }),
        },
    ),
    (
        "setprototypeof",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "setprototypeof",
            replacements: &["Object.setPrototypeOf"],
            url: None,
        },
    ),
    (
        "shebang-regex",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "shebang-regex",
            replacements: &["snippet::shebang-regex"],
            url: None,
        },
    ),
    (
        "shortid",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "shortid",
            replacements: &["nanoid"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "shortid",
            }),
        },
    ),
    (
        "slash",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "slash",
            replacements: &["snippet::unix-paths"],
            url: None,
        },
    ),
    (
        "sort-object",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "sort-object",
            replacements: &["sort-object", "sort-object-keys", "sortobject"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "sort-object",
            }),
        },
    ),
    (
        "source-map-explorer",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "source-map-explorer",
            replacements: &[
                "rollup-plugin-visualizer",
                "sonda",
                "webpack-bundle-analyzer",
            ],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "source-map-explorer",
            }),
        },
    ),
    (
        "split",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "split",
            replacements: &["readline.createInterface"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "split",
            }),
        },
    ),
    (
        "split-lines",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "split-lines",
            replacements: &["snippet::split-lines"],
            url: None,
        },
    ),
    (
        "sqlite3",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "sqlite3",
            replacements: &["node:sqlite", "better-sqlite3"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "sqlite3",
            }),
        },
    ),
    (
        "stream-buffers",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "stream-buffers",
            replacements: &["node:stream"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "stream-buffers",
            }),
        },
    ),
    (
        "string-width",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string-width",
            replacements: &["fast-string-width", "Bun.stringWidth"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "string-width",
            }),
        },
    ),
    (
        "string.prototype.at",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.prototype.at",
            replacements: &["String.prototype.at"],
            url: None,
        },
    ),
    (
        "string.prototype.includes",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.prototype.includes",
            replacements: &["String.prototype.includes"],
            url: None,
        },
    ),
    (
        "string.prototype.lastindexof",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.prototype.lastindexof",
            replacements: &["String.prototype.lastIndexOf"],
            url: None,
        },
    ),
    (
        "string.prototype.matchall",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.prototype.matchall",
            replacements: &["String.prototype.matchAll"],
            url: None,
        },
    ),
    (
        "string.prototype.padend",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.prototype.padend",
            replacements: &["String.prototype.padEnd"],
            url: None,
        },
    ),
    (
        "string.prototype.padleft",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.prototype.padleft",
            replacements: &["String.prototype.padStart"],
            url: None,
        },
    ),
    (
        "string.prototype.padright",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.prototype.padright",
            replacements: &["String.prototype.padEnd"],
            url: None,
        },
    ),
    (
        "string.prototype.padstart",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.prototype.padstart",
            replacements: &["String.prototype.padStart"],
            url: None,
        },
    ),
    (
        "string.prototype.repeat",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.prototype.repeat",
            replacements: &["String.prototype.repeat"],
            url: None,
        },
    ),
    (
        "string.prototype.replaceall",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.prototype.replaceall",
            replacements: &["String.prototype.replaceAll"],
            url: None,
        },
    ),
    (
        "string.prototype.split",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.prototype.split",
            replacements: &["String.prototype.split"],
            url: None,
        },
    ),
    (
        "string.prototype.substr",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.prototype.substr",
            replacements: &["String.prototype.substr"],
            url: None,
        },
    ),
    (
        "string.prototype.trim",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.prototype.trim",
            replacements: &["String.prototype.trim"],
            url: None,
        },
    ),
    (
        "string.prototype.trimend",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.prototype.trimend",
            replacements: &["String.prototype.trimEnd"],
            url: None,
        },
    ),
    (
        "string.prototype.trimleft",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.prototype.trimleft",
            replacements: &["String.prototype.trimStart"],
            url: None,
        },
    ),
    (
        "string.prototype.trimright",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.prototype.trimright",
            replacements: &["String.prototype.trimEnd"],
            url: None,
        },
    ),
    (
        "string.prototype.trimstart",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.prototype.trimstart",
            replacements: &["String.prototype.trimStart"],
            url: None,
        },
    ),
    (
        "string.raw",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "string.raw",
            replacements: &["String.raw"],
            url: None,
        },
    ),
    (
        "strip-ansi",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "strip-ansi",
            replacements: &["util.stripVTControlCharacters", "Bun.stripANSI"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "strip-ansi",
            }),
        },
    ),
    (
        "symbol.prototype.description",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "symbol.prototype.description",
            replacements: &["Symbol.prototype.description"],
            url: None,
        },
    ),
    (
        "teeny-request",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "teeny-request",
            replacements: &["fetch", "ofetch", "ky"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "fetch",
            }),
        },
    ),
    (
        "temp",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "temp",
            replacements: &["fs.mkdtemp", "Deno.makeTempDir"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "tempy",
            }),
        },
    ),
    (
        "tempy",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "tempy",
            replacements: &["fs.mkdtemp", "Deno.makeTempDir"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "tempy",
            }),
        },
    ),
    (
        "then-request",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "then-request",
            replacements: &["fetch", "ofetch", "ky"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "fetch",
            }),
        },
    ),
    (
        "through",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "through",
            replacements: &["node:stream"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "through",
            }),
        },
    ),
    (
        "through2",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "through2",
            replacements: &["node:stream"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "through",
            }),
        },
    ),
    (
        "to-buffer",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "to-buffer",
            replacements: &["Buffer.from"],
            url: None,
        },
    ),
    (
        "toarray",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "toarray",
            replacements: &["snippet::array-coerce"],
            url: None,
        },
    ),
    (
        "tokml",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "tokml",
            replacements: &["@placemarkio/tokml"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "tokml",
            }),
        },
    ),
    (
        "toml",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "toml",
            replacements: &["smol-toml", "Bun.TOML"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "toml",
            }),
        },
    ),
    (
        "traverse",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "traverse",
            replacements: &["neotraverse"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "traverse",
            }),
        },
    ),
    (
        "trim",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "trim",
            replacements: &["String.prototype.trim"],
            url: None,
        },
    ),
    (
        "typed-array-buffer",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "typed-array-buffer",
            replacements: &["%TypedArray%.prototype.buffer"],
            url: None,
        },
    ),
    (
        "typed-array-byte-length",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "typed-array-byte-length",
            replacements: &["%TypedArray%.prototype.byteLength"],
            url: None,
        },
    ),
    (
        "typed-array-byte-offset",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "typed-array-byte-offset",
            replacements: &["%TypedArray%.prototype.byteOffset"],
            url: None,
        },
    ),
    (
        "typed-array-length",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "typed-array-length",
            replacements: &["%TypedArray%.prototype.length"],
            url: None,
        },
    ),
    (
        "typedarray",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "typedarray",
            replacements: &["%TypedArray%"],
            url: None,
        },
    ),
    (
        "typedarray.prototype.slice",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "typedarray.prototype.slice",
            replacements: &["%TypedArray%.prototype.slice"],
            url: None,
        },
    ),
    (
        "uid-safe",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "uid-safe",
            replacements: &["crypto.randomUUID"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "uuidv4",
            }),
        },
    ),
    (
        "underscore",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "underscore",
            replacements: &["lodash-underscore", "es-toolkit"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        },
    ),
    (
        "uniq",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "uniq",
            replacements: &["snippet::array-unique"],
            url: None,
        },
    ),
    (
        "upper-case",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "upper-case",
            replacements: &["snippet::to-uppercase"],
            url: None,
        },
    ),
    (
        "uri-js",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "uri-js",
            replacements: &["URL", "uri-js-replace", "fast-uri"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "uri-js",
            }),
        },
    ),
    (
        "url-parse",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "url-parse",
            replacements: &["URL"],
            url: None,
        },
    ),
    (
        "utf8",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "utf8",
            replacements: &["TextEncoder", "Buffer.prototype.toString"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "utf8",
            }),
        },
    ),
    (
        "util.promisify",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "util.promisify",
            replacements: &["util.promisify"],
            url: None,
        },
    ),
    (
        "uuid",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "uuid",
            replacements: &["crypto.randomUUID"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "uuidv4",
            }),
        },
    ),
    (
        "uuidv4",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "uuidv4",
            replacements: &["crypto.randomUUID"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "uuidv4",
            }),
        },
    ),
    (
        "weak-map",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "weak-map",
            replacements: &["WeakMap"],
            url: None,
        },
    ),
    (
        "wellknown",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "wellknown",
            replacements: &["betterknown"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "wellknown",
            }),
        },
    ),
    (
        "whatwg-fetch",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "whatwg-fetch",
            replacements: &["fetch", "ofetch", "ky"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "fetch",
            }),
        },
    ),
    (
        "whatwg-url",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "whatwg-url",
            replacements: &["URL"],
            url: None,
        },
    ),
    (
        "wrap-ansi",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "wrap-ansi",
            replacements: &["fast-wrap-ansi"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "wrap-ansi",
            }),
        },
    ),
    (
        "xmldom",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "xmldom",
            replacements: &["@xmldom/xmldom"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "xmldom",
            }),
        },
    ),
    (
        "xtend",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "xtend",
            replacements: &["Object.assign"],
            url: None,
        },
    ),
    (
        "yargs",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "yargs",
            replacements: &["sade", "cleye"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "cli-builders",
            }),
        },
    ),
    (
        "yargs-parser",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "yargs-parser",
            replacements: &["util.parseArgs", "mri"],
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "parseargs",
            }),
        },
    ),
    (
        "year",
        ModuleReplacementMapping {
            mapping_type: "module",
            module_name: "year",
            replacements: &["snippet::year"],
            url: None,
        },
    ),
];
pub static MODULE_REPLACEMENTS: &[(&str, ModuleReplacement)] = &[
    (
        "%TypedArray%",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "%TypedArray%",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/TypedArray",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "typed-arrays",
                compat_key: "javascript.builtins.TypedArray",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "%TypedArray%.prototype.buffer",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "%TypedArray%.prototype.buffer",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/TypedArray/buffer",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "typed-arrays",
                compat_key: "javascript.builtins.TypedArray.buffer",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "%TypedArray%.prototype.byteLength",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "%TypedArray%.prototype.byteLength",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/TypedArray/byteLength",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "typed-arrays",
                compat_key: "javascript.builtins.TypedArray.byteLength",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "%TypedArray%.prototype.byteOffset",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "%TypedArray%.prototype.byteOffset",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/TypedArray/byteOffset",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "typed-arrays",
                compat_key: "javascript.builtins.TypedArray.byteOffset",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "%TypedArray%.prototype.length",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "%TypedArray%.prototype.length",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/TypedArray/length",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "typed-arrays",
                compat_key: "javascript.builtins.TypedArray.length",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "%TypedArray%.prototype.slice",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "%TypedArray%.prototype.slice",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/TypedArray/slice",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "typed-arrays",
                compat_key: "javascript.builtins.TypedArray.slice",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "--env-file",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "--env-file",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/cli.html#--env-filefile",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: None,
        }),
    ),
    (
        "@eslint-community/eslint-plugin-eslint-comments",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "@eslint-community/eslint-plugin-eslint-comments",
                engines: &[],
                preferred: false,
            },
            replacement_module: "@eslint-community/eslint-plugin-eslint-comments",
            url: None,
        }),
    ),
    (
        "@eslint-react/eslint-plugin",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "@eslint-react/eslint-plugin",
                engines: &[],
                preferred: false,
            },
            replacement_module: "@eslint-react/eslint-plugin",
            url: None,
        }),
    ),
    (
        "@faker-js/faker",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "@faker-js/faker",
                engines: &[],
                preferred: false,
            },
            replacement_module: "@faker-js/faker",
            url: None,
        }),
    ),
    (
        "@fastify/deepmerge",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "@fastify/deepmerge",
                engines: &[],
                preferred: false,
            },
            replacement_module: "@fastify/deepmerge",
            url: None,
        }),
    ),
    (
        "@material/web",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "@material/web",
                engines: &[],
                preferred: false,
            },
            replacement_module: "@material/web",
            url: None,
        }),
    ),
    (
        "@materializecss/materialize",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "@materializecss/materialize",
                engines: &[],
                preferred: false,
            },
            replacement_module: "@materializecss/materialize",
            url: None,
        }),
    ),
    (
        "@placemarkio/tokml",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "@placemarkio/tokml",
                engines: &[],
                preferred: false,
            },
            replacement_module: "@placemarkio/tokml",
            url: None,
        }),
    ),
    (
        "@vitest/eslint-plugin",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "@vitest/eslint-plugin",
                engines: &[],
                preferred: false,
            },
            replacement_module: "@vitest/eslint-plugin",
            url: None,
        }),
    ),
    (
        "@xmldom/xmldom",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "@xmldom/xmldom",
                engines: &[],
                preferred: false,
            },
            replacement_module: "@xmldom/xmldom",
            url: None,
        }),
    ),
    (
        "AbortController",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "AbortController",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/API/AbortController",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "aborting",
                compat_key: "api.AbortController.AbortController",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "AggregateError",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "AggregateError",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/AggregateError",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "promise-any",
                compat_key: "javascript.builtins.AggregateError",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.from",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.from",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/from",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-from",
                compat_key: "javascript.builtins.Array.from",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.isArray",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.isArray",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/isArray",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-isarray",
                compat_key: "javascript.builtins.Array.isArray",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.of",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.of",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/of",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-of",
                compat_key: "javascript.builtins.Array.of",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.at",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.at",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/at",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-at",
                compat_key: "javascript.builtins.Array.at",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.concat",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.concat",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/concat",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array",
                compat_key: "javascript.builtins.Array.concat",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.copyWithin",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.copyWithin",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/copyWithin",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-copywithin",
                compat_key: "javascript.builtins.Array.copyWithin",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.entries",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.entries",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/entries",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-iterators",
                compat_key: "javascript.builtins.Array.entries",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.every",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.every",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/every",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-iteration-methods",
                compat_key: "javascript.builtins.Array.every",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.filter",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.filter",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/filter",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-iteration-methods",
                compat_key: "javascript.builtins.Array.filter",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.find",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.find",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/find",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-find",
                compat_key: "javascript.builtins.Array.find",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.findIndex",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.findIndex",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/findIndex",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-find",
                compat_key: "javascript.builtins.Array.findIndex",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.findLast",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.findLast",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/findLast",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-findlast",
                compat_key: "javascript.builtins.Array.findLast",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.findLastIndex",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.findLastIndex",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/findLastIndex",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-findlast",
                compat_key: "javascript.builtins.Array.findLastIndex",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.flat",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.flat",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/flat",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-flat",
                compat_key: "javascript.builtins.Array.flat",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.flatMap",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.flatMap",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/flatMap",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-flat",
                compat_key: "javascript.builtins.Array.flatMap",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.forEach",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.forEach",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/forEach",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-iteration-methods",
                compat_key: "javascript.builtins.Array.forEach",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.includes",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.includes",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/includes",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-includes",
                compat_key: "javascript.builtins.Array.includes",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.indexOf",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.indexOf",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/indexOf",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-iteration-methods",
                compat_key: "javascript.builtins.Array.indexOf",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.join",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.join",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/join",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array",
                compat_key: "javascript.builtins.Array.join",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.keys",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.keys",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/keys",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-iterators",
                compat_key: "javascript.builtins.Array.keys",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.lastIndexOf",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.lastIndexOf",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/lastIndexOf",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-iteration-methods",
                compat_key: "javascript.builtins.Array.lastIndexOf",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.map",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.map",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/map",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-iteration-methods",
                compat_key: "javascript.builtins.Array.map",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.push",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.push",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/push",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array",
                compat_key: "javascript.builtins.Array.push",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.reduce",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.reduce",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/reduce",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-iteration-methods",
                compat_key: "javascript.builtins.Array.reduce",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.reduceRight",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.reduceRight",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/reduceRight",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-iteration-methods",
                compat_key: "javascript.builtins.Array.reduceRight",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.slice",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.slice",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/slice",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array",
                compat_key: "javascript.builtins.Array.slice",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.some",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.some",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/some",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-iteration-methods",
                compat_key: "javascript.builtins.Array.some",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.splice",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.splice",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/splice",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-splice",
                compat_key: "javascript.builtins.Array.splice",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.toReversed",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.toReversed",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/toReversed",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-by-copy",
                compat_key: "javascript.builtins.Array.toReversed",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.toSorted",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.toSorted",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/toSorted",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-by-copy",
                compat_key: "javascript.builtins.Array.toSorted",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.unshift",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.unshift",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/unshift",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array",
                compat_key: "javascript.builtins.Array.unshift",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype.values",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype.values",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/values",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "array-iterators",
                compat_key: "javascript.builtins.Array.values",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Array.prototype[Symbol.unscopables]",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Array.prototype[Symbol.unscopables]",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Array/@@unscopables",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "with",
                compat_key: "javascript.builtins.Array.@@unscopables",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "ArrayBuffer.prototype.byteLength",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "ArrayBuffer.prototype.byteLength",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/ArrayBuffer/byteLength",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "typed-arrays",
                compat_key: "javascript.builtins.ArrayBuffer.byteLength",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "ArrayBuffer.prototype.slice",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "ArrayBuffer.prototype.slice",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/ArrayBuffer/slice",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "typed-arrays",
                compat_key: "javascript.builtins.ArrayBuffer.slice",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "AsyncIterator.prototype",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "AsyncIterator.prototype",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/AsyncIterator",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "async-iterators",
                compat_key: "javascript.builtins.AsyncIterator",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "BigInt",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "BigInt",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/BigInt",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "bigint",
                compat_key: "javascript.builtins.BigInt",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Buffer",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Buffer",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/buffer.html",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:buffer",
                export_name: Some("Buffer"),
            }),
        }),
    ),
    (
        "Buffer.compare",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Buffer.compare",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/buffer.html#static-method-buffercomparebuf1-buf2",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:buffer",
                export_name: Some("Buffer"),
            }),
        }),
    ),
    (
        "Buffer.from",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Buffer.from",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/buffer.html#static-method-bufferfromarray",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "buffer",
                export_name: None,
            }),
        }),
    ),
    (
        "Buffer.isBuffer",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Buffer.isBuffer",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/buffer.html#static-method-bufferisbufferobj",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "buffer",
                export_name: None,
            }),
        }),
    ),
    (
        "Buffer.prototype.equals",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Buffer.prototype.equals",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/buffer.html#bufequalsotherbuffer",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:buffer",
                export_name: Some("Buffer"),
            }),
        }),
    ),
    (
        "Buffer.prototype.toString",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Buffer.prototype.toString",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/buffer.html#buftostringencoding-start-end",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:buffer",
                export_name: Some("Buffer"),
            }),
        }),
    ),
    (
        "Bun.CryptoHasher",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Bun.CryptoHasher",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Raw("https://bun.com/docs/runtime/hashing"),
            description: None,
            web_feature_id: None,
            node_feature_id: None,
        }),
    ),
    (
        "Bun.Shell",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Bun.Shell",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Raw("https://bun.com/docs/runtime/shell"),
            description: None,
            web_feature_id: None,
            node_feature_id: None,
        }),
    ),
    (
        "Bun.TOML",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Bun.TOML",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Raw("https://bun.com/docs/runtime/toml"),
            description: None,
            web_feature_id: None,
            node_feature_id: None,
        }),
    ),
    (
        "Bun.YAML",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Bun.YAML",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Raw("https://bun.com/docs/runtime/yaml"),
            description: None,
            web_feature_id: None,
            node_feature_id: None,
        }),
    ),
    (
        "Bun.deepEquals",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Bun.deepEquals",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Raw("https://bun.com/docs/runtime/utils#bun-deepequals"),
            description: None,
            web_feature_id: None,
            node_feature_id: None,
        }),
    ),
    (
        "Bun.file",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Bun.file",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Raw("https://bun.com/reference/bun/file"),
            description: None,
            web_feature_id: None,
            node_feature_id: None,
        }),
    ),
    (
        "Bun.stringWidth",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Bun.stringWidth",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Raw("https://bun.com/docs/runtime/utils#bun-stringwidth"),
            description: None,
            web_feature_id: None,
            node_feature_id: None,
        }),
    ),
    (
        "Bun.stripANSI",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Bun.stripANSI",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Raw("https://bun.com/docs/runtime/utils#bun-stripansi"),
            description: None,
            web_feature_id: None,
            node_feature_id: None,
        }),
    ),
    (
        "ClipboardAPI",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "ClipboardAPI",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/API/Clipboard_API",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "async-clipboard",
                compat_key: "api.Clipboard",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "DataView",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "DataView",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/DataView",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "typed-arrays",
                compat_key: "javascript.builtins.DataView",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "DataView.prototype.buffer",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "DataView.prototype.buffer",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/DataView/buffer",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "typed-arrays",
                compat_key: "javascript.builtins.DataView.buffer",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "DataView.prototype.byteLength",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "DataView.prototype.byteLength",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/DataView/byteLength",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "typed-arrays",
                compat_key: "javascript.builtins.DataView.byteLength",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "DataView.prototype.byteOffset",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "DataView.prototype.byteOffset",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/DataView/byteOffset",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "typed-arrays",
                compat_key: "javascript.builtins.DataView.byteOffset",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Date",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Date",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Date",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "date",
                compat_key: "javascript.builtins.Date",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Date.now",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Date.now",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Date/now",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "date",
                compat_key: "javascript.builtins.Date.now",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Deno.makeTempDir",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Deno.makeTempDir",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Raw("https://docs.deno.com/api/deno/~/Deno.makeTempDir"),
            description: None,
            web_feature_id: None,
            node_feature_id: None,
        }),
    ),
    (
        "Error",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Error",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Error",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "javascript",
                compat_key: "javascript.builtins.Error",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Error.isError",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Error.isError",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Error/isError",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "is-error",
                compat_key: "javascript.builtins.Error.isError",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Error.prototype.cause",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Error.prototype.cause",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Error/cause",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "error-cause",
                compat_key: "javascript.builtins.Error.cause",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "EventTarget",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "EventTarget",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/API/EventTarget",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "events",
                compat_key: "api.EventTarget",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Function.prototype.bind",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Function.prototype.bind",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Function/bind",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "functions",
                compat_key: "javascript.builtins.Function.bind",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Function.prototype.name",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Function.prototype.name",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Function/name",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "functions",
                compat_key: "javascript.builtins.Function.name",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Intl.DateTimeFormat",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Intl.DateTimeFormat",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "intl",
                compat_key: "javascript.builtins.Intl.DateTimeFormat.DateTimeFormat",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Intl.Segmenter",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Intl.Segmenter",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Intl/Segmenter",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "intl-segmenter",
                compat_key: "javascript.builtins.Intl.Segmenter",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Iterator.prototype",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Iterator.prototype",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Iterator",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "iterator-methods",
                compat_key: "javascript.builtins.Iterator.Iterator",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Map",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Map",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Map",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "map",
                compat_key: "javascript.builtins.Map",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Math.acosh",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Math.acosh",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Math/acosh",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.Math.acosh",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Math.atanh",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Math.atanh",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Math/atanh",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.Math.atanh",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Math.cbrt",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Math.cbrt",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Math/cbrt",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.Math.cbrt",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Math.clz32",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Math.clz32",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Math/clz32",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.Math.clz32",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Math.f16round",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Math.f16round",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Math/f16round",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "float16array",
                compat_key: "javascript.builtins.Math.f16round",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Math.fround",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Math.fround",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Math/fround",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.Math.fround",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Math.imul",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Math.imul",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Math/imul",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.Math.imul",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Math.log10",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Math.log10",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Math/log10",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.Math.log10",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Math.log1p",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Math.log1p",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Math/log1p",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.Math.log1p",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Math.log2",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Math.log2",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Math/log2",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.Math.log2",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Math.sign",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Math.sign",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Math/sign",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.Math.sign",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Number.isFinite",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Number.isFinite",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Number/isFinite",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.Number.isFinite",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Number.isInteger",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Number.isInteger",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Number/isInteger",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.Number.isInteger",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Number.isNaN",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Number.isNaN",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Number/isNaN",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.Number.isNaN",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Number.isSafeInteger",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Number.isSafeInteger",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Number/isSafeInteger",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.Number.isSafeInteger",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Number.parseFloat",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Number.parseFloat",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Number/parseFloat",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.Number.parseFloat",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Number.parseInt",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Number.parseInt",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Number/parseInt",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.Number.parseInt",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Number.prototype.toExponential",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Number.prototype.toExponential",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Number/toExponential",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.Number.toExponential",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Object.assign",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Object.assign",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Object/assign",
            },
            description: Some(
                "`Object.assign`, or if deep clones are needed, use `structuredClone`",
            ),
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "object-object",
                compat_key: "javascript.builtins.Object.assign",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Object.defineProperties",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Object.defineProperties",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Object/defineProperties",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "object-object",
                compat_key: "javascript.builtins.Object.defineProperties",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Object.defineProperty",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Object.defineProperty",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Object/defineProperty",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "object-object",
                compat_key: "javascript.builtins.Object.defineProperty",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Object.entries",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Object.entries",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Object/entries",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "object-object",
                compat_key: "javascript.builtins.Object.entries",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Object.fromEntries",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Object.fromEntries",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Object/fromEntries",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "object-object",
                compat_key: "javascript.builtins.Object.fromEntries",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Object.getOwnPropertyDescriptor",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Object.getOwnPropertyDescriptor",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Object/getOwnPropertyDescriptor",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "object-object",
                compat_key: "javascript.builtins.Object.getOwnPropertyDescriptor",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Object.getOwnPropertyDescriptors",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Object.getOwnPropertyDescriptors",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Object/getOwnPropertyDescriptors",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "object-object",
                compat_key: "javascript.builtins.Object.getOwnPropertyDescriptors",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Object.getPrototypeOf",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Object.getPrototypeOf",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Object/getPrototypeOf",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "object-object",
                compat_key: "javascript.builtins.Object.getPrototypeOf",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Object.hasOwn",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Object.hasOwn",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Object/hasOwn",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "object-hasown",
                compat_key: "javascript.builtins.Object.hasOwn",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Object.is",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Object.is",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Object/is",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "object-object",
                compat_key: "javascript.builtins.Object.is",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Object.keys",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Object.keys",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Object/keys",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "object-object",
                compat_key: "javascript.builtins.Object.keys",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Object.prototype.hasOwnProperty",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Object.prototype.hasOwnProperty",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Object/hasOwnProperty",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "object-object",
                compat_key: "javascript.builtins.Object.hasOwnProperty",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Object.setPrototypeOf",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Object.setPrototypeOf",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Object/setPrototypeOf",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "object-object",
                compat_key: "javascript.builtins.Object.setPrototypeOf",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Object.values",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Object.values",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Object/values",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "object-object",
                compat_key: "javascript.builtins.Object.values",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Promise",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Promise",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Promise",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "promise",
                compat_key: "javascript.builtins.Promise",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Promise.allSettled",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Promise.allSettled",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Promise/allSettled",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "promise-allsettled",
                compat_key: "javascript.builtins.Promise.allSettled",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Promise.any",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Promise.any",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Promise/any",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "promise-any",
                compat_key: "javascript.builtins.Promise.any",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Promise.prototype.finally",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Promise.prototype.finally",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Promise/finally",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "promise-finally",
                compat_key: "javascript.builtins.Promise.finally",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Reflect.getPrototypeOf",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Reflect.getPrototypeOf",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Reflect/getPrototypeOf",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "proxy-reflect",
                compat_key: "javascript.builtins.Reflect.getPrototypeOf",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Reflect.ownKeys",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Reflect.ownKeys",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Reflect/ownKeys",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "proxy-reflect",
                compat_key: "javascript.builtins.Reflect.ownKeys",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "RegExp.escape",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "RegExp.escape",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/RegExp/escape",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "regexp-escape",
                compat_key: "javascript.builtins.RegExp.escape",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "RegExp.prototype.flags",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "RegExp.prototype.flags",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/RegExp/flags",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "regexp",
                compat_key: "javascript.builtins.RegExp.flags",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Set",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Set",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Set",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "set",
                compat_key: "javascript.builtins.Set",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "String.prototype.at",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "String.prototype.at",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/String/at",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "string-at",
                compat_key: "javascript.builtins.String.at",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "String.prototype.includes",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "String.prototype.includes",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/String/includes",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "string-includes",
                compat_key: "javascript.builtins.String.includes",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "String.prototype.lastIndexOf",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "String.prototype.lastIndexOf",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/String/lastIndexOf",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "strings",
                compat_key: "javascript.builtins.String.lastIndexOf",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "String.prototype.matchAll",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "String.prototype.matchAll",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/String/matchAll",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "string-matchall",
                compat_key: "javascript.builtins.String.matchAll",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "String.prototype.padEnd",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "String.prototype.padEnd",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/String/padEnd",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "string-pad",
                compat_key: "javascript.builtins.String.padEnd",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "String.prototype.padStart",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "String.prototype.padStart",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/String/padStart",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "string-pad",
                compat_key: "javascript.builtins.String.padStart",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "String.prototype.repeat",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "String.prototype.repeat",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/String/repeat",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "string-repeat",
                compat_key: "javascript.builtins.String.repeat",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "String.prototype.replaceAll",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "String.prototype.replaceAll",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/String/replaceAll",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "string-replaceall",
                compat_key: "javascript.builtins.String.replaceAll",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "String.prototype.split",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "String.prototype.split",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/String/split",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "strings",
                compat_key: "javascript.builtins.String.split",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "String.prototype.substr",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "String.prototype.substr",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/String/substr",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "html-wrapper-methods",
                compat_key: "javascript.builtins.String.substr",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "String.prototype.trim",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "String.prototype.trim",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/String/trim",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "strings",
                compat_key: "javascript.builtins.String.trim",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "String.prototype.trimEnd",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "String.prototype.trimEnd",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/String/trimEnd",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "string-trim-startend",
                compat_key: "javascript.builtins.String.trimEnd",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "String.prototype.trimStart",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "String.prototype.trimStart",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/String/trimStart",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "string-trim-startend",
                compat_key: "javascript.builtins.String.trimStart",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "String.raw",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "String.raw",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/String/raw",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "string-raw",
                compat_key: "javascript.builtins.String.raw",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Symbol.prototype.description",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "Symbol.prototype.description",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Symbol/description",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "symbol",
                compat_key: "javascript.builtins.Symbol.description",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "Teleport",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "Teleport",
                engines: &[],
                preferred: false,
            },
            replacement_module: "Teleport",
            url: None,
        }),
    ),
    (
        "TextEncoder",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "TextEncoder",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/API/TextEncoder",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "text-encoding",
                compat_key: "api.TextEncoder",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "URL",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "URL",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/API/URL",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "url",
                compat_key: "api.URL",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "URL.canParse",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "URL.canParse",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/API/URL/canParse_static",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "url-canparse",
                compat_key: "api.URL.canParse_static",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "URLSearchParams",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "URLSearchParams",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/API/URLSearchParams",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "url",
                compat_key: "api.URLSearchParams",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "WeakMap",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "WeakMap",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/WeakMap",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "weakmap",
                compat_key: "javascript.builtins.WeakMap",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "airbnb-js-shims",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "airbnb-js-shims",
                engines: &[],
                preferred: false,
            },
            description: "Every modern environment has native support for all of the APIs included in this library.",
            url: None,
        }),
    ),
    (
        "ansis",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "ansis",
                engines: &[],
                preferred: false,
            },
            replacement_module: "ansis",
            url: None,
        }),
    ),
    (
        "atob",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "atob",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/API/Window/atob",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "base64encodedecode",
                compat_key: "api.atob",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "bcryptjs",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "bcryptjs",
                engines: &[],
                preferred: false,
            },
            replacement_module: "bcryptjs",
            url: None,
        }),
    ),
    (
        "better-sqlite3",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "better-sqlite3",
                engines: &[],
                preferred: false,
            },
            replacement_module: "better-sqlite3",
            url: None,
        }),
    ),
    (
        "betterknown",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "betterknown",
                engines: &[],
                preferred: false,
            },
            replacement_module: "betterknown",
            url: None,
        }),
    ),
    (
        "btoa",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "btoa",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/API/Window/btoa",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "base64encodedecode",
                compat_key: "api.btoa",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "builtinModules",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "builtinModules",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/module.html#modulebuiltinmodules",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:module",
                export_name: Some("builtinModules"),
            }),
        }),
    ),
    (
        "bun:test",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "bun:test",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Raw("https://bun.com/docs/test"),
            description: None,
            web_feature_id: None,
            node_feature_id: None,
        }),
    ),
    (
        "cleye",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "cleye",
                engines: &[],
                preferred: false,
            },
            replacement_module: "cleye",
            url: None,
        }),
    ),
    (
        "concurrently",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "concurrently",
                engines: &[],
                preferred: false,
            },
            replacement_module: "concurrently",
            url: None,
        }),
    ),
    (
        "cpx2",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "cpx2",
                engines: &[],
                preferred: false,
            },
            replacement_module: "cpx2",
            url: None,
        }),
    ),
    (
        "crypto",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "crypto",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/API/Web_Crypto_API",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "web-cryptography",
                compat_key: "api.crypto",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "crypto.randomBytes",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "crypto.randomBytes",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/crypto.html#cryptorandombytessize-callback",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "crypto",
                export_name: Some("randomBytes"),
            }),
        }),
    ),
    (
        "crypto.randomUUID",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "crypto.randomUUID",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/API/Crypto/randomUUID",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "web-cryptography",
                compat_key: "api.Crypto.randomUUID",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "crypto.timingSafeEqual",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "crypto.timingSafeEqual",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/crypto.html#cryptotimingsafeequala-b",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "crypto",
                export_name: Some("timingSafeEqual"),
            }),
        }),
    ),
    (
        "date-fns",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "date-fns",
                engines: &[],
                preferred: false,
            },
            replacement_module: "date-fns",
            url: None,
        }),
    ),
    (
        "day.js",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "day.js",
                engines: &[],
                preferred: false,
            },
            replacement_module: "day.js",
            url: None,
        }),
    ),
    (
        "defu",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "defu",
                engines: &[],
                preferred: false,
            },
            replacement_module: "defu",
            url: None,
        }),
    ),
    (
        "dequal",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "dequal",
                engines: &[],
                preferred: false,
            },
            replacement_module: "dequal",
            url: None,
        }),
    ),
    (
        "dlv",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "dlv",
                engines: &[],
                preferred: false,
            },
            replacement_module: "dlv",
            url: None,
        }),
    ),
    (
        "dset",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "dset",
                engines: &[],
                preferred: false,
            },
            replacement_module: "dset",
            url: None,
        }),
    ),
    (
        "elysia",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "elysia",
                engines: &[],
                preferred: false,
            },
            replacement_module: "elysia",
            url: None,
        }),
    ),
    (
        "emoji-regex-xs",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "emoji-regex-xs",
                engines: &[],
                preferred: false,
            },
            replacement_module: "emoji-regex-xs",
            url: None,
        }),
    ),
    (
        "empathic",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "empathic",
                engines: &[],
                preferred: false,
            },
            replacement_module: "empathic",
            url: None,
        }),
    ),
    (
        "es-errors",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "es-errors",
                engines: &[],
                preferred: false,
            },
            description: "`Error` and its subclasses are natively built into all modern environments.",
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Error",
            }),
        }),
    ),
    (
        "es-string-html-methods",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "es-string-html-methods",
                engines: &[],
                preferred: false,
            },
            description: "All the methods exported by this modules are generally available on the `String` prototype in modern environments.",
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/String#html_wrapper_methods",
            }),
        }),
    ),
    (
        "es-toolkit",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "es-toolkit",
                engines: &[],
                preferred: false,
            },
            replacement_module: "es-toolkit",
            url: None,
        }),
    ),
    (
        "es5-shim",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "es5-shim",
                engines: &[],
                preferred: false,
            },
            description: "Every modern environment has support for ES5 APIs.",
            url: None,
        }),
    ),
    (
        "es6-shim",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "es6-shim",
                engines: &[],
                preferred: false,
            },
            description: "Every modern environment has support for ES6/ES2015 APIs.",
            url: None,
        }),
    ),
    (
        "es7-shim",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "es7-shim",
                engines: &[],
                preferred: false,
            },
            description: "Every modern environment has support for ES7/ES2016 apis.",
            url: None,
        }),
    ),
    (
        "eslint-plugin-es-x",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "eslint-plugin-es-x",
                engines: &[],
                preferred: false,
            },
            replacement_module: "eslint-plugin-es-x",
            url: None,
        }),
    ),
    (
        "eslint-plugin-import-x",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "eslint-plugin-import-x",
                engines: &[],
                preferred: false,
            },
            replacement_module: "eslint-plugin-import-x",
            url: None,
        }),
    ),
    (
        "eslint-plugin-n",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "eslint-plugin-n",
                engines: &[],
                preferred: false,
            },
            replacement_module: "eslint-plugin-n",
            url: None,
        }),
    ),
    (
        "exsolve",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "exsolve",
                engines: &[],
                preferred: false,
            },
            replacement_module: "exsolve",
            url: None,
        }),
    ),
    (
        "extends",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "extends",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Classes/extends",
            },
            description: Some(
                "You can use ES6 classes `extends` syntax for prototype inheritance.",
            ),
            web_feature_id: None,
            node_feature_id: None,
        }),
    ),
    (
        "fast-querystring",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "fast-querystring",
                engines: &[],
                preferred: false,
            },
            replacement_module: "fast-querystring",
            url: None,
        }),
    ),
    (
        "fast-string-width",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "fast-string-width",
                engines: &[],
                preferred: false,
            },
            replacement_module: "fast-string-width",
            url: None,
        }),
    ),
    (
        "fast-uri",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "fast-uri",
                engines: &[],
                preferred: false,
            },
            replacement_module: "fast-uri",
            url: None,
        }),
    ),
    (
        "fast-wrap-ansi",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "fast-wrap-ansi",
                engines: &[],
                preferred: false,
            },
            replacement_module: "fast-wrap-ansi",
            url: None,
        }),
    ),
    (
        "fdir",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "fdir",
                engines: &[],
                preferred: false,
            },
            replacement_module: "fdir",
            url: None,
        }),
    ),
    (
        "fetch",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "fetch",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/API/Fetch_API",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "fetch",
                compat_key: "api.fetch",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "for...in",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "for...in",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Statements/for...in",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "javascript",
                compat_key: "javascript.statements.for_in",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "for...of",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "for...of",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Statements/for...of",
            },
            description: Some("`for...of` (using `Object.entries` if dealing with objects)"),
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "iterators",
                compat_key: "javascript.statements.for_of",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "fs",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "fs",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/fs.html",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:fs",
                export_name: None,
            }),
        }),
    ),
    (
        "fs.access",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "fs.access",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/fs.html#fspromisesaccesspath-mode",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:fs/promises",
                export_name: Some("access"),
            }),
        }),
    ),
    (
        "fs.existsSync",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "fs.existsSync",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/fs.html#fsexistssyncpath",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:fs",
                export_name: Some("existsSync"),
            }),
        }),
    ),
    (
        "fs.glob",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "fs.glob",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/fs.html#fspromisesglobpattern-options",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:fs",
                export_name: Some("glob"),
            }),
        }),
    ),
    (
        "fs.mkdir",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "fs.mkdir",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/fs.html#fspromisesmkdirpath-options",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:fs",
                export_name: Some("mkdir"),
            }),
        }),
    ),
    (
        "fs.mkdtemp",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "fs.mkdtemp",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/fs.html#fspromisesmkdtempprefix-options",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:fs/promises",
                export_name: Some("mkdtemp"),
            }),
        }),
    ),
    (
        "fs.realpath",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "fs.realpath",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/fs.html#fsrealpathpath-options-callback",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "fs",
                export_name: Some("realpath"),
            }),
        }),
    ),
    (
        "fs.rm",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "fs.rm",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/fs.html#fspromisesrmpath-options",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:fs/promises",
                export_name: Some("rm"),
            }),
        }),
    ),
    (
        "fs.rmdir",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "fs.rmdir",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/fs.html#fspromisesrmdirpath-options",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:fs/promises",
                export_name: Some("rmdir"),
            }),
        }),
    ),
    (
        "fsPromises",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "fsPromises",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/fs.html#promises-api",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "fs",
                export_name: None,
            }),
        }),
    ),
    (
        "get-port",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "get-port",
                engines: &[],
                preferred: false,
            },
            replacement_module: "get-port",
            url: None,
        }),
    ),
    (
        "globalThis",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "globalThis",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/globalThis",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "globalthis",
                compat_key: "javascript.builtins.globalThis",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "grammy",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "grammy",
                engines: &[],
                preferred: false,
            },
            replacement_module: "grammy",
            url: None,
        }),
    ),
    (
        "h3",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "h3",
                engines: &[],
                preferred: false,
            },
            replacement_module: "h3",
            url: None,
        }),
    ),
    (
        "has-bigints",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "has-bigints",
                engines: &[],
                preferred: false,
            },
            description: "Every modern environment has support for `BigInt`. You can just remove the check.",
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/BigInt",
            }),
        }),
    ),
    (
        "has-dynamic-import",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "has-dynamic-import",
                engines: &[],
                preferred: false,
            },
            description: "Every modern environment has support for dynamic `import()`. You can just remove the check.",
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Operators/import",
            }),
        }),
    ),
    (
        "has-optional-chaining",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "has-optional-chaining",
                engines: &[],
                preferred: false,
            },
            description: "Every modern environment has support for optional chaining. You can just remove the check.",
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Operators/Optional_chaining",
            }),
        }),
    ),
    (
        "has-property-descriptors",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "has-property-descriptors",
                engines: &[],
                preferred: false,
            },
            description: "Every modern environment has full property descriptor support. You can just remove the check.",
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Object/getOwnPropertyDescriptors",
            }),
        }),
    ),
    (
        "has-proto",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "has-proto",
                engines: &[],
                preferred: false,
            },
            description: "Every modern environment has support for `__proto__`. You can just remove the check.",
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Operators/Object_initializer#prototype_setter",
            }),
        }),
    ),
    (
        "has-symbols",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "has-symbols",
                engines: &[],
                preferred: false,
            },
            description: "Every modern environment has support for `Symbol`. You can just remove the check.",
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Symbol",
            }),
        }),
    ),
    (
        "has-template-literals",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "has-template-literals",
                engines: &[],
                preferred: false,
            },
            description: "Every modern environment has support for template literals. You can just remove the check.",
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Template_literals",
            }),
        }),
    ),
    (
        "has-tostringtag",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "has-tostringtag",
                engines: &[],
                preferred: false,
            },
            description: "Every modern environment has support for `Symbol.toStringTag`. You can just remove the check.",
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/Symbol/toStringTag",
            }),
        }),
    ),
    (
        "has-typed-arrays",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "has-typed-arrays",
                engines: &[],
                preferred: false,
            },
            description: "Every modern environment has support for typed arrays. You can just remove the check.",
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/TypedArray",
            }),
        }),
    ),
    (
        "hono",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "hono",
                engines: &[],
                preferred: false,
            },
            replacement_module: "hono",
            url: None,
        }),
    ),
    (
        "import.meta.resolve",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "import.meta.resolve",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Operators/import.meta/resolve",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "js-modules",
                compat_key: "javascript.operators.import_meta.resolve",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "isBuiltin",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "isBuiltin",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/module.html#moduleisbuiltinmodulename",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:module",
                export_name: Some("isBuiltin"),
            }),
        }),
    ),
    (
        "jose",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "jose",
                engines: &[],
                preferred: false,
            },
            replacement_module: "jose",
            url: None,
        }),
    ),
    (
        "jquery",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "jquery",
                engines: &[],
                preferred: false,
            },
            description: "jQuery can be replaced with native DOM and Web APIs",
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "jquery",
            }),
        }),
    ),
    (
        "jsx-ast-utils-x",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "jsx-ast-utils-x",
                engines: &[],
                preferred: false,
            },
            replacement_module: "jsx-ast-utils-x",
            url: None,
        }),
    ),
    (
        "knip",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "knip",
                engines: &[],
                preferred: false,
            },
            replacement_module: "knip",
            url: None,
        }),
    ),
    (
        "ky",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "ky",
                engines: &[],
                preferred: false,
            },
            replacement_module: "ky",
            url: None,
        }),
    ),
    (
        "lilconfig",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "lilconfig",
                engines: &[],
                preferred: false,
            },
            replacement_module: "lilconfig",
            url: None,
        }),
    ),
    (
        "lodash-underscore",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "lodash-underscore",
                engines: &[],
                preferred: false,
            },
            description: "lodash and Underscore can be replaced with native JavaScript APIs",
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "lodash-underscore",
            }),
        }),
    ),
    (
        "lucide",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "lucide",
                engines: &[],
                preferred: false,
            },
            replacement_module: "lucide",
            url: None,
        }),
    ),
    (
        "lucide-react",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "lucide-react",
                engines: &[],
                preferred: false,
            },
            replacement_module: "lucide-react",
            url: None,
        }),
    ),
    (
        "luxon",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "luxon",
                engines: &[],
                preferred: false,
            },
            replacement_module: "luxon",
            url: None,
        }),
    ),
    (
        "milliparsec",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "milliparsec",
                engines: &[],
                preferred: false,
            },
            replacement_module: "milliparsec",
            url: None,
        }),
    ),
    (
        "mri",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "mri",
                engines: &[],
                preferred: false,
            },
            replacement_module: "mri",
            url: None,
        }),
    ),
    (
        "nano-staged",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "nano-staged",
                engines: &[],
                preferred: false,
            },
            replacement_module: "nano-staged",
            url: None,
        }),
    ),
    (
        "nanoexec",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "nanoexec",
                engines: &[],
                preferred: false,
            },
            replacement_module: "nanoexec",
            url: None,
        }),
    ),
    (
        "nanoid",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "nanoid",
                engines: &[],
                preferred: false,
            },
            replacement_module: "nanoid",
            url: None,
        }),
    ),
    (
        "nanospinner",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "nanospinner",
                engines: &[],
                preferred: false,
            },
            replacement_module: "nanospinner",
            url: None,
        }),
    ),
    (
        "nativebird",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "nativebird",
                engines: &[],
                preferred: false,
            },
            replacement_module: "nativebird",
            url: None,
        }),
    ),
    (
        "neoqs",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "neoqs",
                engines: &[],
                preferred: false,
            },
            replacement_module: "neoqs",
            url: None,
        }),
    ),
    (
        "neotraverse",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "neotraverse",
                engines: &[],
                preferred: false,
            },
            replacement_module: "neotraverse",
            url: None,
        }),
    ),
    (
        "node:crypto",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "node:crypto",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/crypto.html",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:crypto",
                export_name: None,
            }),
        }),
    ),
    (
        "node:fs",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "node:fs",
                engines: &[],
                preferred: false,
            },
            replacement_module: "node:fs",
            url: None,
        }),
    ),
    (
        "node:sqlite",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "node:sqlite",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/sqlite.html",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:sqlite",
                export_name: None,
            }),
        }),
    ),
    (
        "node:stream",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "node:stream",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/stream.html",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:stream",
                export_name: None,
            }),
        }),
    ),
    (
        "node:test",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "node:test",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/test.html",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "test",
                export_name: None,
            }),
        }),
    ),
    (
        "npm-run-all2",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "npm-run-all2",
                engines: &[],
                preferred: false,
            },
            replacement_module: "npm-run-all2",
            url: None,
        }),
    ),
    (
        "obug",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "obug",
                engines: &[],
                preferred: false,
            },
            replacement_module: "obug",
            url: None,
        }),
    ),
    (
        "ofetch",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "ofetch",
                engines: &[],
                preferred: false,
            },
            replacement_module: "ofetch",
            url: None,
        }),
    ),
    (
        "ohash",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "ohash",
                engines: &[],
                preferred: false,
            },
            replacement_module: "ohash",
            url: None,
        }),
    ),
    (
        "oxc-resolver",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "oxc-resolver",
                engines: &[],
                preferred: false,
            },
            replacement_module: "oxc-resolver",
            url: None,
        }),
    ),
    (
        "package-manager-detector",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "package-manager-detector",
                engines: &[],
                preferred: false,
            },
            replacement_module: "package-manager-detector",
            url: None,
        }),
    ),
    (
        "parseInt",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "parseInt",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Global_Objects/parseInt",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "number",
                compat_key: "javascript.builtins.parseInt",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "path.parse",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "path.parse",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/path.html#pathparsepath",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "path",
                export_name: Some("parse"),
            }),
        }),
    ),
    (
        "picocolors",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "picocolors",
                engines: &[],
                preferred: false,
            },
            replacement_module: "picocolors",
            url: None,
        }),
    ),
    (
        "picoquery",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "picoquery",
                engines: &[],
                preferred: false,
            },
            replacement_module: "picoquery",
            url: None,
        }),
    ),
    (
        "picospinner",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "picospinner",
                engines: &[],
                preferred: false,
            },
            replacement_module: "picospinner",
            url: None,
        }),
    ),
    (
        "pkg-types",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "pkg-types",
                engines: &[],
                preferred: false,
            },
            replacement_module: "pkg-types",
            url: None,
        }),
    ),
    (
        "premove",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "premove",
                engines: &[],
                preferred: false,
            },
            replacement_module: "premove",
            url: None,
        }),
    ),
    (
        "process.allowedNodeEnvironmentFlags",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "process.allowedNodeEnvironmentFlags",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/process.html#processallowednodeenvironmentflags",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "process",
                export_name: Some("allowedNodeEnvironmentFlags"),
            }),
        }),
    ),
    (
        "queueMicrotask",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "queueMicrotask",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/API/Window/queueMicrotask",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "queuemicrotask",
                compat_key: "api.queueMicrotask",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "react-helmet-async",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "react-helmet-async",
                engines: &[],
                preferred: false,
            },
            replacement_module: "react-helmet-async",
            url: None,
        }),
    ),
    (
        "react:metadata",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "react:metadata",
                engines: &[],
                preferred: false,
            },
            replacement_module: "react",
            url: None,
        }),
    ),
    (
        "readline.createInterface",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "readline.createInterface",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/readline.html#readlinecreateinterfaceoptions",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:readline",
                export_name: Some("createInterface"),
            }),
        }),
    ),
    (
        "requestAnimationFrame",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "requestAnimationFrame",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/API/Window/requestAnimationFrame",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "request-animation-frame",
                compat_key: "api.Window.requestAnimationFrame",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "rollup-plugin-visualizer",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "rollup-plugin-visualizer",
                engines: &[],
                preferred: false,
            },
            replacement_module: "rollup-plugin-visualizer",
            url: None,
        }),
    ),
    (
        "sade",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "sade",
                engines: &[],
                preferred: false,
            },
            replacement_module: "sade",
            url: None,
        }),
    ),
    (
        "safe-buffer",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "safe-buffer",
                engines: &[],
                preferred: false,
            },
            description: "All modern versions of Node have `Buffer.from` for safely creating buffers from arbitrary types.",
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/buffer.html",
            }),
        }),
    ),
    (
        "smol-toml",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "smol-toml",
                engines: &[],
                preferred: false,
            },
            replacement_module: "smol-toml",
            url: None,
        }),
    ),
    (
        "snippet::array-coerce",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::array-coerce",
                engines: &[],
                preferred: false,
            },
            description: "You can use a combination of a ternary operator and `Array.isArray` to make sure a value, `undefined`, `null` or an array is always returned as an array.",
            example: Some(
                "(val == null ? [] : Array.isArray(val) ? val : [val])\n// Or if you need to convert an iterable into an array\nArray.from(iterable)",
            ),
            url: None,
        }),
    ),
    (
        "snippet::array-difference",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::array-difference",
                engines: &[],
                preferred: false,
            },
            description: "You can use a combination of `filter` and `includes` to calculate the difference between two arrays.",
            example: Some("const difference = (a, b) => a.filter((item) => !b.includes(item))"),
            url: None,
        }),
    ),
    (
        "snippet::array-flatten",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::array-flatten",
                engines: &[],
                preferred: false,
            },
            description: "You can use `Array.prototype.flat` with `Infinity` as an argument to fully flatten an array.",
            example: Some("array.flat(Infinity)"),
            url: None,
        }),
    ),
    (
        "snippet::array-from-count",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::array-from-count",
                engines: &[],
                preferred: false,
            },
            description: "You can use `Array.from` to create an array of sequential integers",
            example: Some("Array.from({ length: n }, (_, i) => i);"),
            url: None,
        }),
    ),
    (
        "snippet::array-from-count-with-start",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::array-from-count-with-start",
                engines: &[],
                preferred: false,
            },
            description: "You can use `Array.from` to create an array of sequential integers starting from a specific integer",
            example: Some("Array.from({ length: end - start }, (_, i) => i + start);"),
            url: None,
        }),
    ),
    (
        "snippet::array-last",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::array-last",
                engines: &[],
                preferred: false,
            },
            description: "You can use `arr.at(-1)` if supported or `arr[arr.length - 1]` to get the last element of an array.",
            example: Some(
                "const last = (arr) => arr.at(-1);\n// or in older environments\nconst lastLegacy = (arr) => arr[arr.length - 1]",
            ),
            url: None,
        }),
    ),
    (
        "snippet::array-slice-exclude-last-n",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::array-slice-exclude-last-n",
                engines: &[],
                preferred: false,
            },
            description: "You can get all but the last n elements using `array.slice`",
            example: Some("array.slice(0, array.length - n)"),
            url: None,
        }),
    ),
    (
        "snippet::array-union",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::array-union",
                engines: &[],
                preferred: false,
            },
            description: "You can use a combination of the spread operator and `Set` to create a union of two arrays.",
            example: Some("const union = (a, b) => [...new Set([...a, ...b])]"),
            url: None,
        }),
    ),
    (
        "snippet::array-unique",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::array-unique",
                engines: &[],
                preferred: false,
            },
            description: "You can convert to and from a `Set` to remove duplicates from an array.",
            example: Some("const unique = (arr) => [...new Set(arr)]"),
            url: None,
        }),
    ),
    (
        "snippet::async-function-constructor",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::async-function-constructor",
                engines: &[],
                preferred: false,
            },
            description: "You can get the `AsyncFunction` using `async function`.",
            example: Some("const AsyncFunction = (async () => {}).constructor"),
            url: None,
        }),
    ),
    (
        "snippet::base64",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::base64",
                engines: &[],
                preferred: false,
            },
            description: "Every modern runtime provides a way to convert byte array to and from base64.",
            example: Some(
                "// From base64 to Uint8Array\nconst bytes = Uint8Array.fromBase64(base64)\n// From Uint8Array to base64\nconst base64 = bytes.toBase64()",
            ),
            url: None,
        }),
    ),
    (
        "snippet::base64-id",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::base64-id",
                engines: &[],
                preferred: false,
            },
            description: "You can use `crypto.randomBytes` with `Buffer.prototype.toString` to generate a random base64 id",
            example: Some(
                "import crypto from 'node:crypto'\nconst id = crypto.randomBytes(15).toString('base64').replaceAll('+', '-').replaceAll('/', '_')",
            ),
            url: None,
        }),
    ),
    (
        "snippet::call-bind",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::call-bind",
                engines: &[],
                preferred: false,
            },
            description: "Every modern runtime provides a way to bind to the `call` method of a function.",
            example: Some("const fnBound = Function.call.bind(fn)"),
            url: None,
        }),
    ),
    (
        "snippet::char-last",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::char-last",
                engines: &[],
                preferred: false,
            },
            description: "You can use `str.at(-1)` if supported or `str[str.length - 1]` to get the last character of a string.",
            example: Some(
                "const last = (str) => str.at(-1);\n// or in older environments\nconst lastLegacy = (str) => str[str.length - 1]",
            ),
            url: None,
        }),
    ),
    (
        "snippet::for-own",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::for-own",
                engines: &[],
                preferred: false,
            },
            description: "You can use `Object.keys(obj).forEach` to iterate over the own enumerable properties of an object.",
            example: Some(
                "Object.keys(obj).forEach(key => {\n  const value = obj[key];\n  // do something\n});",
            ),
            url: None,
        }),
    ),
    (
        "snippet::get-iterator",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::get-iterator",
                engines: &[],
                preferred: false,
            },
            description: "Every modern runtime provides a way to get the iterator function through the `Symbol.iterator` symbol.",
            example: Some("const iterator = obj[Symbol.iterator]?.()"),
            url: None,
        }),
    ),
    (
        "snippet::has-ansi",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::has-ansi",
                engines: &[],
                preferred: false,
            },
            description: "You can use the `includes` method on the string to check if a specific ANSI byte is present.",
            example: Some("string.includes(\"\\u001b\") || string.includes(\"\\u009b\")"),
            url: None,
        }),
    ),
    (
        "snippet::has-argv",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::has-argv",
                engines: &[],
                preferred: false,
            },
            description: "You can use the `includes` method on the `process.argv` array to check if a flag is present.",
            example: Some("process.argv.includes('--flag')"),
            url: None,
        }),
    ),
    (
        "snippet::is-arguments",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-arguments",
                engines: &[],
                preferred: false,
            },
            description: "You can use `Object.prototype.toString.call(obj) === \"[object Arguments]\"`",
            example: Some(
                "const isArguments = (val) => Object.prototype.toString.call(val) === \"[object Arguments]\";",
            ),
            url: None,
        }),
    ),
    (
        "snippet::is-arraybuffer",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-arraybuffer",
                engines: &[],
                preferred: false,
            },
            description: "You can use `instanceof ArrayBuffer`, or if cross-realm, use `Object.prototype.toString.call(obj) === \"[object ArrayBuffer]\"`",
            example: Some(
                "const isArrayBuffer = obj instanceof ArrayBuffer;\n// for cross-realm\nconst isArrayBufferCrossRealm = Object.prototype.toString.call(obj) === \"[object ArrayBuffer]\"",
            ),
            url: None,
        }),
    ),
    (
        "snippet::is-async-function",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-async-function",
                engines: &[],
                preferred: false,
            },
            description: "You can use `typeof` and `Object.prototype.toString.call` to check if it's an async function",
            example: Some(
                "const isAsyncFunction = (obj) => typeof obj === \"function\" && Object.prototype.toString.call(obj) === \"[object AsyncFunction]\"",
            ),
            url: None,
        }),
    ),
    (
        "snippet::is-bigint",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-bigint",
                engines: &[],
                preferred: false,
            },
            description: "You can use `typeof` to check if a value is a bigint.",
            example: Some("const isString = (value) => typeof value === \"bigint\";"),
            url: None,
        }),
    ),
    (
        "snippet::is-boolean",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-boolean",
                engines: &[],
                preferred: false,
            },
            description: "You can use `typeof` to check if a value is a boolean primitive, and `Object.prototype.toString.call` to check if it's a `Boolean` object.",
            example: Some("Object.prototype.toString.call(v) === \"[object Boolean]\""),
            url: None,
        }),
    ),
    (
        "snippet::is-ci",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-ci",
                engines: &[],
                preferred: false,
            },
            description: "Every major CI provider sets a `CI` environment variable that you can use to detect if you're running in a CI environment.",
            example: Some("Boolean(process.env.CI)"),
            url: None,
        }),
    ),
    (
        "snippet::is-date",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-date",
                engines: &[],
                preferred: false,
            },
            description: "You can use `instanceof Date`, or if cross-realm, use `Object.prototype.toString.call(v) === \"[object Date]\"`",
            example: Some(
                "const isDate = v instanceof Date;\n// for cross-realm\nconst isDateCrossRealm = Object.prototype.toString.call(v) === \"[object Date]\"",
            ),
            url: None,
        }),
    ),
    (
        "snippet::is-even",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-even",
                engines: &[],
                preferred: false,
            },
            description: "You can use the modulo operator to check if a number is even.",
            example: Some("(n % 2) === 0"),
            url: None,
        }),
    ),
    (
        "snippet::is-function",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-function",
                engines: &[],
                preferred: false,
            },
            description: "You can use `typeof` to check if a value is a function.",
            example: Some("const isString = (value) => typeof value === \"function\";"),
            url: None,
        }),
    ),
    (
        "snippet::is-generator-function",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-generator-function",
                engines: &[],
                preferred: false,
            },
            description: "You can use `typeof` and `Object.prototype.toString.call` to check if a value is a generator function",
            example: Some(
                "const isGeneratorFunction = (obj) => typeof obj === \"function\" && Object.prototype.toString.call(obj) === \"[object GeneratorFunction]\"",
            ),
            url: None,
        }),
    ),
    (
        "snippet::is-in-ssh",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-in-ssh",
                engines: &[],
                preferred: false,
            },
            description: "You can check if the current environment is SSH by checking if the `SSH_CONNECTION` environment variable is set.",
            example: Some("const isInSsh = Boolean(process.env.SSH_CONNECTION);"),
            url: None,
        }),
    ),
    (
        "snippet::is-negative",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-negative",
                engines: &[],
                preferred: false,
            },
            description: "You can check if a number is less than 0 to determine if it's negative.",
            example: Some("(n) => n < 0"),
            url: None,
        }),
    ),
    (
        "snippet::is-negative-zero",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-negative-zero",
                engines: &[],
                preferred: false,
            },
            description: "You can use `Object.is` to check if a value is negative zero.",
            example: Some("Object.is(n, -0)"),
            url: None,
        }),
    ),
    (
        "snippet::is-npm",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-npm",
                engines: &[],
                preferred: false,
            },
            description: "If the current environment is npm the `npm_config_user_agent` environment variable will be set and start with `\"npm\"`.",
            example: Some("const isNpm = process.env.npm_config_user_agent?.startsWith(\"npm\")"),
            url: None,
        }),
    ),
    (
        "snippet::is-null",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-null",
                engines: &[],
                preferred: false,
            },
            description: "You can check if a value is `null` using regular equality checks.",
            example: Some("value === null"),
            url: None,
        }),
    ),
    (
        "snippet::is-number",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-number",
                engines: &[],
                preferred: false,
            },
            description: "You can check if a value is a number by using `typeof` or coercing it to a number and using `Number.isFinite`.",
            example: Some(
                "const isNumber = (v) => typeof v === \"number\" || (typeof v === \"string\" && Number.isFinite(+v));",
            ),
            url: None,
        }),
    ),
    (
        "snippet::is-object",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-object",
                engines: &[],
                preferred: false,
            },
            description: "You can use `typeof` to check if a value is an object and `Object.getPrototypeOf` to ensure it's a plain object.",
            example: Some(
                "const isObject = (obj) => obj && typeof obj === \"object\" && (Object.getPrototypeOf(obj) === null || Object.getPrototypeOf(obj) === Object.prototype);",
            ),
            url: None,
        }),
    ),
    (
        "snippet::is-object-or-function",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-object-or-function",
                engines: &[],
                preferred: false,
            },
            description: "You can use `typeof` to check if a value is an object or a function.",
            example: Some(
                "const isObjectOrFunction = (v) => v !== null && (typeof v === \"object\" || typeof v === \"function\");",
            ),
            url: None,
        }),
    ),
    (
        "snippet::is-odd",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-odd",
                engines: &[],
                preferred: false,
            },
            description: "You can use the modulo operator to check if a number is odd.",
            example: Some("(n % 2) === 1"),
            url: None,
        }),
    ),
    (
        "snippet::is-primitve",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-primitve",
                engines: &[],
                preferred: false,
            },
            description: "You can check `typeof` of a value to determine if it's a primitive. Note that `typeof null` is `\"object\"` so you need to check for `null` separately.",
            example: Some(
                "const isPrimitive = (value) => value === null || (typeof value !== \"function\" && typeof value !== \"object\");",
            ),
            url: None,
        }),
    ),
    (
        "snippet::is-regexp",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-regexp",
                engines: &[],
                preferred: false,
            },
            description: "You can use `instanceof RegExp` to check if a value is a regular expression, or if cross-realm, use `Object.prototype.toString.call(v) === \"[object RegExp]\"`.",
            example: Some(
                "const isRegExp = (v) => v instanceof RegExp;\n// for cross-realm\nconst isRegExpCrossRealm = Object.prototype.toString.call(v) === \"[object RegExp]\";",
            ),
            url: None,
        }),
    ),
    (
        "snippet::is-stream",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-stream",
                engines: &[],
                preferred: false,
            },
            description: "`node:stream` provides `isReadable` and `isWritable` methods that can be used to check if a stream is readable or writable.",
            example: Some(
                "import { isReadable, isWritable } from 'node:stream';\nconst isStream = (stream) => isReadable(stream) || isWritable(stream);",
            ),
            url: None,
        }),
    ),
    (
        "snippet::is-string",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-string",
                engines: &[],
                preferred: false,
            },
            description: "You can use `typeof` to check if a value is a string.",
            example: Some("const isString = (value) => typeof value === \"string\";"),
            url: None,
        }),
    ),
    (
        "snippet::is-symbol",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-symbol",
                engines: &[],
                preferred: false,
            },
            description: "You can use `typeof` to check if a value is a symbol.",
            example: Some("const isSymbol = (value) => typeof value === \"symbol\";"),
            url: None,
        }),
    ),
    (
        "snippet::is-travis",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-travis",
                engines: &[],
                preferred: false,
            },
            description: "You can check if the current environment is Travis CI by checking if the `TRAVIS` environment variable is set.",
            example: Some("const isTravis = () => \"TRAVIS\" in process.env;"),
            url: None,
        }),
    ),
    (
        "snippet::is-undefined",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-undefined",
                engines: &[],
                preferred: false,
            },
            description: "You can use `typeof` to check if a value is `undefined`.",
            example: Some("typeof value === \"undefined\";"),
            url: None,
        }),
    ),
    (
        "snippet::is-whitespace",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-whitespace",
                engines: &[],
                preferred: false,
            },
            description: "You can check if a string contains only whitespace with RegExp or by trimming it and comparing it to an empty string.",
            example: Some("const isWhitespace = (str) => /^\\s*$/.test(str);"),
            url: None,
        }),
    ),
    (
        "snippet::is-windows",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::is-windows",
                engines: &[],
                preferred: false,
            },
            description: "You can check if the current environment is Windows by checking if `process.platform` is equal to \"win32\".",
            example: Some("const isWindows = () => process.platform === \"win32\";"),
            url: None,
        }),
    ),
    (
        "snippet::json-file",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::json-file",
                engines: &[],
                preferred: false,
            },
            description: "You can use `JSON` and `node:fs` to read and write JSON files.",
            example: Some(
                "import * as fs from 'node:fs/promises'\nfs.readFile(file, 'utf8').then(JSON.parse)\nfs.writeFile(file, JSON.stringify(data, null, 2) + '\\n')",
            ),
            url: None,
        }),
    ),
    (
        "snippet::math-random",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::math-random",
                engines: &[],
                preferred: false,
            },
            description: "You can use `Math.random()` or `crypto.getRandomValues` if cryptographic randomness is required.",
            example: Some(
                "crypto.getRandomValues(new Uint32Array(1))[0] / (2 ** 32);\n// or\nMath.random();",
            ),
            url: None,
        }),
    ),
    (
        "snippet::noop",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::noop",
                engines: &[],
                preferred: false,
            },
            description: "You can use an arrow function `() => {}` for a noop function.",
            example: Some("const noop = () => {}"),
            url: None,
        }),
    ),
    (
        "snippet::object-filter",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::object-filter",
                engines: &[],
                preferred: false,
            },
            description: "You can use `Object.fromEntries` with `Object.entries` and `Array.prototype.filter` to filter an object's properties.",
            example: Some(
                "const objectFilter = (obj, fn) => Object.fromEntries(Object.entries(obj).filter(fn));",
            ),
            url: None,
        }),
    ),
    (
        "snippet::object-map",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::object-map",
                engines: &[],
                preferred: false,
            },
            description: "You can use `Object.fromEntries` with `Object.entries` and `Array.prototype.map` to map an object's properties.",
            example: Some(
                "const objectMap = (obj, fn) => Object.fromEntries(Object.entries(obj).map(([k, v]) => fn(k, v)));",
            ),
            url: None,
        }),
    ),
    (
        "snippet::object-reduce",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::object-reduce",
                engines: &[],
                preferred: false,
            },
            description: "You can use `Object.entries` and `Array.prototype.reduce`.",
            example: Some(
                "const objectReduce = (obj, fn, initial) => Object.entries(obj).reduce((acc, [k, v]) => fn(acc, k, v), initial);",
            ),
            url: None,
        }),
    ),
    (
        "snippet::regexp-copy",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::regexp-copy",
                engines: &[],
                preferred: false,
            },
            description: "You can create a copy of a regular expression using the `RegExp` constructor.",
            example: Some("const copyRegExp = (regexp) => new RegExp(regexp);"),
            url: None,
        }),
    ),
    (
        "snippet::set-tostringtag",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::set-tostringtag",
                engines: &[],
                preferred: false,
            },
            description: "You can set the `toStringTag` of an object using `Object.defineProperty`.",
            example: Some(
                "const setToStringTag = (target, value) => Object.defineProperty(target, Symbol.toStringTag, { value, configurable: true });",
            ),
            url: None,
        }),
    ),
    (
        "snippet::shebang-regex",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::shebang-regex",
                engines: &[],
                preferred: false,
            },
            description: "You can use `/^#!(.+)/` regex.",
            example: Some("const shebangRegex = /^#!(.+)/;"),
            url: None,
        }),
    ),
    (
        "snippet::split-lines",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::split-lines",
                engines: &[],
                preferred: false,
            },
            description: "You can split a string into lines using a regular expression.",
            example: Some("const splitLines = (str) => str.split(/\\r?\\n/);"),
            url: None,
        }),
    ),
    (
        "snippet::to-lower",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::to-lower",
                engines: &[],
                preferred: false,
            },
            description: "You can convert a string to lowercase using `toLocaleLowerCase` or `toLowerCase`.",
            example: Some("const toLower = (str) => str.toLocaleLowerCase();"),
            url: None,
        }),
    ),
    (
        "snippet::to-uppercase",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::to-uppercase",
                engines: &[],
                preferred: false,
            },
            description: "You can convert a string to uppercase using `toLocaleUpperCase` or `toUpperCase`.",
            example: Some("const toUpper = (str) => str.toLocaleUpperCase();"),
            url: None,
        }),
    ),
    (
        "snippet::typeof",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::typeof",
                engines: &[],
                preferred: false,
            },
            description: "You can use `typeof` to get the type of a value, or `Object.prototype.toString.call` to get the internal [[Class]] of an object.",
            example: Some(
                "const typeOf = (value) => typeof value;\n// for more specific types\nconst classOf = (value) => Object.prototype.toString.call(value);",
            ),
            url: None,
        }),
    ),
    (
        "snippet::unix-paths",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::unix-paths",
                engines: &[],
                preferred: false,
            },
            description: "You can check the start of a path for the Windows extended-length path prefix and if it's not present, replace backslashes with forward slashes.",
            example: Some("path.startsWith('\\\\\\\\?\\\\') ? path : path.replace(/\\\\/g, '/')"),
            url: None,
        }),
    ),
    (
        "snippet::year",
        ModuleReplacement::Simple(SimpleModuleReplacement {
            common: ModuleReplacementLike {
                id: "snippet::year",
                engines: &[],
                preferred: false,
            },
            description: "You can use `new Date().getUTCFullYear()` to get the current year.",
            example: Some("new Date().getUTCFullYear()"),
            url: None,
        }),
    ),
    (
        "sonda",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "sonda",
                engines: &[],
                preferred: false,
            },
            replacement_module: "sonda",
            url: None,
        }),
    ),
    (
        "sort-object",
        ModuleReplacement::Removal(RemovalReplacement {
            common: ModuleReplacementLike {
                id: "sort-object",
                engines: &[],
                preferred: false,
            },
            description: "`Object.entries` and `Array.prototype.sort` can be used to sort object keys.",
            url: Some(KnownUrl::Descriptor {
                url_type: KnownUrlType::E18e,
                id: "sort-object",
            }),
        }),
    ),
    (
        "sort-object-keys",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "sort-object-keys",
                engines: &[],
                preferred: false,
            },
            replacement_module: "sort-object-keys",
            url: None,
        }),
    ),
    (
        "sortobject",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "sortobject",
                engines: &[],
                preferred: false,
            },
            replacement_module: "sortobject",
            url: None,
        }),
    ),
    (
        "streams",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "streams",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/API/Streams_API",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "streams",
                compat_key: "api.ReadableStream",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "structuredClone",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "structuredClone",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/API/Window/structuredClone",
            },
            description: None,
            web_feature_id: Some(NativeWebFeatureId {
                feature_id: "structured-clone",
                compat_key: "api.structuredClone",
            }),
            node_feature_id: None,
        }),
    ),
    (
        "tiny-invariant",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "tiny-invariant",
                engines: &[],
                preferred: false,
            },
            replacement_module: "tiny-invariant",
            url: None,
        }),
    ),
    (
        "tinyclip",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "tinyclip",
                engines: &[],
                preferred: false,
            },
            replacement_module: "tinyclip",
            url: None,
        }),
    ),
    (
        "tinyexec",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "tinyexec",
                engines: &[],
                preferred: false,
            },
            replacement_module: "tinyexec",
            url: None,
        }),
    ),
    (
        "tinyglobby",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "tinyglobby",
                engines: &[],
                preferred: false,
            },
            replacement_module: "tinyglobby",
            url: None,
        }),
    ),
    (
        "tinyhttp",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "tinyhttp",
                engines: &[],
                preferred: false,
            },
            replacement_module: "@tinyhttp/app",
            url: None,
        }),
    ),
    (
        "ts-graphviz",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "ts-graphviz",
                engines: &[],
                preferred: false,
            },
            replacement_module: "ts-graphviz",
            url: None,
        }),
    ),
    (
        "unicode-segmenter",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "unicode-segmenter",
                engines: &[],
                preferred: false,
            },
            replacement_module: "unicode-segmenter",
            url: None,
        }),
    ),
    (
        "unicodeClassEscape",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "unicodeClassEscape",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Mdn,
                id: "Web/JavaScript/Reference/Regular_expressions/Unicode_character_class_escape",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: None,
        }),
    ),
    (
        "uri-js-replace",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "uri-js-replace",
                engines: &[],
                preferred: false,
            },
            replacement_module: "uri-js-replace",
            url: None,
        }),
    ),
    (
        "util.inherits",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "util.inherits",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/util.html#utilinheritsconstructor-superconstructor",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:util",
                export_name: Some("inherits"),
            }),
        }),
    ),
    (
        "util.isDeepStrictEqual",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "util.isDeepStrictEqual",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/util.html#utilisdeepstrictequalval1-val2-options",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:util",
                export_name: Some("isDeepStrictEqual"),
            }),
        }),
    ),
    (
        "util.parseArgs",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "util.parseArgs",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/util.html#utilparseargsconfig",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:util",
                export_name: Some("parseArgs"),
            }),
        }),
    ),
    (
        "util.promisify",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "util.promisify",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/util.html#utilpromisifyoriginal",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "util",
                export_name: Some("promisify"),
            }),
        }),
    ),
    (
        "util.stripVTControlCharacters",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "util.stripVTControlCharacters",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/util.html#utilstripvtcontrolcharactersstr",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:util",
                export_name: Some("stripVTControlCharacters"),
            }),
        }),
    ),
    (
        "util.styleText",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "util.styleText",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/util.html#utilstyletextformat-text-options",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:util",
                export_name: Some("styleText"),
            }),
        }),
    ),
    (
        "util.types",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "util.types",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/util.html#utiltypes",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:util",
                export_name: Some("types"),
            }),
        }),
    ),
    (
        "vitest",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "vitest",
                engines: &[],
                preferred: false,
            },
            replacement_module: "vitest",
            url: None,
        }),
    ),
    (
        "webpack-bundle-analyzer",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "webpack-bundle-analyzer",
                engines: &[],
                preferred: false,
            },
            replacement_module: "webpack-bundle-analyzer",
            url: None,
        }),
    ),
    (
        "webpack.output.clean",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "webpack.output.clean",
                engines: &[],
                preferred: false,
            },
            replacement_module: "webpack",
            url: None,
        }),
    ),
    (
        "wireit",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "wireit",
                engines: &[],
                preferred: false,
            },
            replacement_module: "wireit",
            url: None,
        }),
    ),
    (
        "yaml",
        ModuleReplacement::Documented(DocumentedModuleReplacement {
            common: ModuleReplacementLike {
                id: "yaml",
                engines: &[],
                preferred: false,
            },
            replacement_module: "yaml",
            url: None,
        }),
    ),
    (
        "zlib.crc32",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "zlib.crc32",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/zlib.html#zlibcrc32data-value",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "zlib",
                export_name: Some("crc32"),
            }),
        }),
    ),
    (
        "zlib.gzipSync",
        ModuleReplacement::Native(NativeModuleReplacement {
            common: ModuleReplacementLike {
                id: "zlib.gzipSync",
                engines: &[],
                preferred: false,
            },
            url: KnownUrl::Descriptor {
                url_type: KnownUrlType::Node,
                id: "api/zlib.html#zlibgzipsyncbuffer-options",
            },
            description: None,
            web_feature_id: None,
            node_feature_id: Some(NativeNodeFeatureId {
                module_name: "node:zlib",
                export_name: Some("gzipSync"),
            }),
        }),
    ),
];
