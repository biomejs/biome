use biome_analyze::{
    ActionCategory, Ast, FixKind, Rule, RuleDiagnostic, SourceActionKind, context::RuleContext,
    declare_source_rule,
};
use biome_console::markup;
use biome_diagnostics::category;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCombinedSpecifier, AnyJsExportClause, AnyJsImportClause, AnyJsModuleItem, JsModule,
    JsSyntaxKind, T,
};
use biome_rowan::{AstNode, BatchMutationExt, TextRange, TriviaPieceKind, chain_trivia_pieces};
use biome_rule_options::{organize_imports::OrganizeImportsOptions, sort_order::SortOrder};
use import_key::{ImportInfo, ImportKey};
use rustc_hash::FxHashMap;
use specifiers_attributes::{
    are_import_attributes_sorted, merge_export_specifiers, merge_import_specifiers,
    sort_attributes, sort_export_specifiers, sort_import_specifiers,
};

use crate::JsRuleAction;
use util::{attached_trivia, detached_trivia, has_detached_leading_comment, leading_newlines};

pub mod import_key;
pub mod specifiers_attributes;
mod util;

declare_source_rule! {
    /// Provides a code action to sort the imports and exports in the file using a built-in or custom order.
    ///
    /// Imports and exports are first separated into chunks, before being sorted.
    /// Imports or exports of a chunk are then grouped according to the user-defined groups.
    /// Within a group, imports are sorted using a built-in order that depends on the import/export kind, whether the import/export has attributes and the source being imported from.
    /// **source** is also often called **specifier** in the JavaScript ecosystem.
    ///
    /// ```js,ignore
    /// import A from "@my/lib" with { "attribute1": "value" };
    /// ^^^^^^^^       ^^^^^^^         ^^^^^^^^^^^^^^^^^^^^^
    ///   kind         source                attributes
    ///
    /// export * from "@my/lib" with { "attribute1": "value" };
    /// ^^^^^^^^       ^^^^^^^         ^^^^^^^^^^^^^^^^^^^^^
    ///   kind         source                attributes
    /// ```
    ///
    ///
    /// ## Chunk of imports and chunk of exports
    ///
    /// A **chunk** is a sequence of adjacent imports or exports.
    /// A chunk contains only imports or exports, not both at the same time.
    /// The following example includes two chunks.
    /// The first chunk consists of the three imports and the second chunk consists of the three exports.
    ///
    /// ```js,ignore
    /// // chunk 1
    /// import A from "a";
    /// import * as B from "b";
    /// import { C } from "c";
    /// // chunk 2
    /// export * from "d";
    /// export * as F from "e";
    /// export { F } from "f";
    /// ```
    ///
    /// Chunks also end as soon as a statement or a **side-effect import** (also called _bare import_) is encountered.
    /// Every side-effect import forms an independent chunk.
    /// The following example contains six chunks:
    ///
    /// ```js,ignore
    /// // chunk 1
    /// import A from "a";
    /// import * as B from "b";
    /// // chunk 2
    /// import "x";
    /// // chunk 3
    /// import "y";
    /// // chunk 4
    /// import { C } from "c";
    /// // chunk 5
    /// export * from "d";
    /// function f() {}
    /// // chunk 6
    /// export * as E from "e";
    /// export { F } from "f";
    /// ```
    ///
    /// 1. The first chunk contains the two first `import` and ends with the appearance of the first side-effect import `import "x"`.
    /// 2. The second chunk contains only the side-effect import `import "x"`.
    /// 3. The third chunk contains only the side-effect import `import "y"`.
    /// 4. The fourth chunk contains a single `import`; The first `export` ends it.
    /// 5. The fifth chunk contains the first `export`; The function declaration ends it.
    /// 6. The sixth chunk contains the last two `export`.
    ///
    /// Chunks are also delimited by detached comments.
    /// A **detached comment** is a comment followed by a blank line.
    /// Comments not followed by a blank line are **attached comments**.
    /// Note that blank lines alone are not taken into account when chunking imports and exports.
    /// The following example contains a detached comment that splits the imports into two chunks:
    ///
    /// ```js,ignore
    /// // Attached comment 1
    /// import A from "a";
    ///
    /// // Attached comment 2
    /// import * as B from "b";
    /// // Detached comment
    ///
    /// import { C } from "c";
    /// ```
    ///
    /// The line `import { C } from "c"` forms the second chunk.
    /// The blank line between the first two imports is ignored so they form a single chunk.
    ///
    /// The sorter ensures that chunks are separated from each other with a blank lines.
    /// Only side-effect imports adjacent to a chunk of imports are not separated by a blank line.
    /// The following code...
    ///
    /// ```js,ignore
    /// import A from "a";
    /// import * as B from "b";
    /// import "x";
    /// import { C } from "c";
    /// export * from "d";
    /// // Detached comment
    ///
    /// export * as F from "e";
    /// // Attached comment
    /// export { F } from "f";
    /// ```
    ///
    /// is sorted as:
    ///
    /// ```js,ignore
    /// import A from "a";
    /// import * as B from "b";
    /// import "x";
    /// import { C } from "c";
    ///
    /// export * from "d";
    ///
    /// // Detached comment
    ///
    /// export * as F from "e";
    /// // Attached comment
    /// export { F } from "f";
    /// ```
    ///
    /// Also, note that blank lines inside a chunk are ignored and preserved.
    /// They can be removed by explicitly defining groups as demonstrated in the next section.
    ///
    ///
    /// ## Import and export sorting
    ///
    /// Once chunks are formed, imports and exports of each chunk are sorted.
    /// Imports and exports are sorted by their source.
    /// Sources are ordered by "distance".
    /// Sources that are "farther" from the current module are put on the top, sources "closer" to the user are put on the bottom.
    /// This leads to the following order:
    ///
    /// 1. URLs such as `https://example.org`.
    /// 2. Packages with a protocol such as `node:path`, `bun:test`, `jsr:@my?lib`, or `npm:lib`.
    /// 3. Packages such as `mylib` or `@my/lib`.
    /// 4. Aliases: sources starting with `@/`, `#`, `~`, `$`, or `%`.
    ///    They usually are [Node.js subpath imports](https://nodejs.org/api/packages.html#subpath-imports) or [TypeScript path aliases](https://www.typescriptlang.org/tsconfig/#paths).
    /// 5. Absolute and relative paths.
    ///
    /// Two imports/exports with the same source category are sorted using a [natural sort order](https://en.wikipedia.org/wiki/Natural_sort_order) tailored to URLs, packages, and paths.
    /// Notably, the order ensures that `A < a < B < b`.
    /// The order takes also numbers into account, e.g. `a9 < a10`.
    ///
    /// For example, the following code...
    ///
    /// ```js,ignore
    /// import sibling from "./file.js";
    /// import internal from "#alias";
    /// import fs from "fs";
    /// import { test } from "node:test";
    /// import path from "node:path";
    /// import parent from "../parent.js";
    /// import scopedLibUsingJsr from "jsr:@scoped/lib";
    /// import data from "https://example.org";
    /// import lib from "lib";
    /// import scopedLib from "@scoped/lib";
    /// ```
    ///
    /// ...is sorted as follows:
    ///
    /// ```js,ignore
    /// import data from "https://example.org";
    /// import scopedLibUsingJsr from "jsr:@scoped/lib";
    /// import path from "node:path";
    /// import { test } from "node:test";
    /// import scopedLib from "@scoped/lib";
    /// import fs from "fs";
    /// import lib from "lib";
    /// import internal from "#alias";
    /// import parent from "../parent.js";
    /// import sibling from "./file.js";
    /// ```
    ///
    /// If two imports or exports share the same source and are in the same chunk, then they are ordered according to their kind as follows:
    ///
    /// 1. Namespace type import / Namespace type export
    /// 2. Default type import
    /// 3. Named type import / Named type export
    /// 4. Namespace import / Namespace export
    /// 5. Combined default and namespace import
    /// 6. Default import
    /// 7. Combined default and named import
    /// 8. Named import / Named export
    ///
    /// Imports and exports with attributes are always placed first.
    /// For example, the following code...
    ///
    /// ```ts,ignore
    /// import * as namespaceImport from "same-source";
    /// import type * as namespaceTypeImport from "same-source";
    /// import type { namedTypeImport } from "same-source";
    /// import defaultNamespaceCombined, * as namespaceCombined from "same-source";
    /// import defaultNamedCombined, { namedCombined } from "same-source";
    /// import defaultImport from "same-source";
    /// import type defaultTypeImport from "same-source";
    /// import { importWithAttribute } from "same-source" with { "attribute": "value" } ;
    /// ```
    ///
    /// is sorted as follows:
    ///
    /// ```ts,ignore
    /// import { importWithAttribute } from "same-source" with { "attribute": "value" } ;
    /// import type * as namespaceTypeImport from "same-source";
    /// import type defaultTypeImport from "same-source";
    /// import type { namedTypeImport } from "same-source";
    /// import * as namespaceImport from "same-source";
    /// import defaultNamespaceCombined, * as namespaceCombined from "same-source";
    /// import defaultImport from "same-source";
    /// import defaultNamedCombined, { namedCombined } from "same-source";
    /// ```
    ///
    /// This default order cannot be changed.
    /// However, users can still customize how imports and exports are sorted using the concept of groups as explained in the following section.
    ///
    ///
    /// ## Import and export groups
    ///
    /// Imports or exports of a chunk are divided into groups before being sorted with the built-in order described in the previous section.
    /// By default every chunk consists of a single group.
    /// These default groups and their order may not be to your taste.
    /// The sorter provides a `groups` option that allows you to customize how the chunks are divided into groups.
    /// The `groups` option is a list of group matchers.
    /// A group matcher is:
    ///
    /// - A predefined group matcher, or
    /// - A glob pattern, or
    /// - An object matcher, or
    /// - A list of glob patterns, predefined group matchers, and object matchers.
    ///
    /// Predefined group matchers are strings in `CONSTANT_CASE` prefixed and suffixed by `:`.
    /// The sorter provides several predefined group matchers:
    ///
    /// - `:ALIAS:`: sources starting with `#`, `@/`, `~`, `$`, or `%`.
    /// - `:BUN:`: sources starting with the protocol `bun:` or that correspond to a built-in Bun module such as `bun`.
    /// - `:NODE:`: sources starting with the protocol `node:` or that correspond to a built-in Node.js module such as `fs` or `path`.
    /// - `:PACKAGE:`: scoped and bare packages.
    /// - `:PACKAGE_WITH_PROTOCOL:`: scoped and bare packages with a protocol.
    /// - `:PATH:`: absolute and relative paths.
    /// - `:URL:`: sources starting with `https://` and `http://`.
    ///
    /// Let's take an example.
    /// In the default configuration, Node.js modules without the `node:` protocol are separated from those with a protocol.
    /// To groups them together, you can use the predefined group `:NODE:`.
    /// Given the following configuration...
    ///
    /// ```json,full_options
    /// {
    ///     "assist": {
    ///         "actions": {
    ///             "source": {
    ///                 "organizeImports": {
    ///                     "level": "on",
    ///                     "options": {
    ///                         "groups": [
    ///                             ":URL:",
    ///                             ":NODE:"
    ///                         ]
    ///                     }
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ...and the following code...
    ///
    /// ```js,ignore
    /// import sibling from "./file.js";
    /// import internal from "#alias";
    /// import fs from "fs";
    /// import { test } from "node:test";
    /// import path from "node:path";
    /// import parent from "../parent.js";
    /// import scopedLibUsingJsr from "jsr:@scoped/lib";
    /// import data from "https://example.org";
    /// import lib from "lib";
    /// import scopedLib from "@scoped/lib";
    /// ```
    ///
    /// ...we end up with the following sorted result where the imports of `node:path` and the `fs` Node.js module are grouped together:
    ///
    /// ```js,ignore
    /// import data from "https://example.org";
    /// import fs from "fs";
    /// import path from "node:path";
    /// import { test } from "node:test";
    /// import scopedLibUsingJsr from "jsr:@scoped/lib";
    /// import scopedLib from "@scoped/lib";
    /// import lib from "lib";
    /// import internal from "#alias";
    /// import parent from "../parent.js";
    /// import sibling from "./file.js";
    /// ```
    ///
    /// Note that all imports that don't match a group matcher are always placed at the end.
    ///
    ///
    /// Group matchers can also be glob patterns and list of glob patterns.
    /// Glob patterns select imports and exports with a source that matches the pattern.
    /// In the following example, we create two groups: one that gathers imports/exports with a source starting with `@my/lib` except `@my/lib/special` and the other that gathers imports/exports starting with `@/`.
    ///
    /// ```json
    /// {
    ///     "options": {
    ///         "groups": [
    ///             ["@my/lib", "@my/lib/**", "!@my/lib/special", "!@my/lib/special/**"],
    ///             "@/**"
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// By applying this configuration to the following code...
    ///
    /// ```js,ignore
    /// import lib from "@my/lib";
    /// import aliased from "@/alias";
    /// import path from "@my/lib/special";
    /// import test from "@my/lib/path";
    /// ```
    ///
    /// ...we obtain the following sorted result.
    /// Imports with the sources `@my/lib` and `@my/lib/path` form the first group.
    /// They match the glob patterns `@my/lib` and `@my/lib/**` respectively.
    /// The import with the source `@my/lib/special` is not placed in this first group because it is rejected by the exception `!@my/lib/special`.
    /// The import with the source `@/alias` is placed in a second group because it matches the glob pattern `@/**`.
    /// Finally, other imports are placed at the end.
    ///
    /// ```js,ignore
    /// import lib from "@my/lib";
    /// import test from "@my/lib/path";
    /// import aliased from "@/alias";
    /// import path from "@my/lib/special";
    /// ```
    ///
    /// Note that `@my/lib` matches `@my/lib` but not `@my/lib/**`.
    /// Conversely, `@my/lib/subpath` matches `@my/lib/**`, but not `@my/lib`.
    /// So, you have to specify both glob patterns if you want to accept all imports/exports that start with `@my/lib`.
    /// The prefix `!` indicates an exception.
    /// You can create exceptions of exceptions by following an exception by a regular glob pattern.
    /// For example `["@my/lib", "@my/lib/**", "!@my/lib/special", "!@my/lib/special/**", "@my/lib/special/*/accepted/**"]` allows you to accepts all sources matching `@my/lib/special/*/accepted/**`.
    /// Note that the predefined groups can also be negated. `!:NODE:` matches all sources that don't match `:NODE:`.
    /// For more details on the supported glob patterns, see the dedicated section.
    ///
    /// Finally, group matchers can be object matchers.
    /// An object matcher allows to match type-only imports and exports.
    ///
    /// Given the following configuration:
    ///
    /// ```json
    /// {
    ///     "options": {
    ///         "groups": [
    ///             { "type": false, "source": ["@my/lib", "@my/lib/**"] },
    ///             ["@my/lib", "@my/lib/**"]
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// The following code:
    ///
    /// ```ts,ignore
    /// import type { T } from "@my/lib";
    /// import { V } from "@my/lib";
    /// ```
    ///
    /// is sorted as follows:
    ///
    /// ```ts,ignore
    /// import { V } from "@my/lib";
    /// import type { T } from "@my/lib";
    /// ```
    ///
    /// The object matcher `{ "type": false, "source": ["@my/lib", "@my/lib/**"] }` match against imports and exports without the `type` keyword with a source that matches one of the glob pattern of the list `["@my/lib", "@my/lib/**"]`.
    ///
    /// The sorter allows the separation of two groups with a blank line using the predefined string `:BLANK_LINE:`.
    /// Given the following configuration...
    ///
    /// ```json
    /// {
    ///     "options": {
    ///         "groups": [
    ///             [":BUN:", ":NODE:"],
    ///             ":BLANK_LINE:",
    ///             ["@my/lib", "@my/lib/**", "!@my/lib/special", "!@my/lib/special/**"],
    ///             "@/**"
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// ...the following code...
    ///
    /// ```js,ignore
    /// import test from "bun:test";
    /// import path from "node:path";
    /// import lib from "@my/lib";
    /// import libPath from "@my/lib/path";
    /// import libSpecial from "@my/lib/special";
    /// import aliased from "@/alias";
    /// ```
    ///
    /// ...is sorted as:
    ///
    /// ```js,ignore
    /// import path from "node:path";
    ///
    /// import lib from "@my/lib";
    /// import test from "@my/lib/path";
    /// import aliased from "@/alias";
    /// import path from "@my/lib/special";
    /// ```
    ///
    /// Groups are matched in order.
    /// This means that one group matcher can shadow succeeding groups.
    /// For example, in the following configuration, the group matcher `:URL:` is never matched because all imports and exports match the first matcher `**`.
    ///
    /// ```json
    /// {
    ///     "options": {
    ///         "groups": [
    ///             "**",
    ///             ":URL:"
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    ///
    /// ## Comment handling
    ///
    /// When sorting imports and exports, attached comments are moved with their import or export,
    /// and detached comments (comments followed by a blank line) are left where they are.
    ///
    /// However, there is an exception to the rule.
    /// If a comment appears at the top of the file, it is considered as detached even if no blank line follows.
    /// This ensures that copyright notice and file header comments stay at the top of the file.
    ///
    /// For example, the following code...
    ///
    /// ```js,ignore
    /// // Copyright notice and file header comment
    /// import F from "f";
    /// // Attached comment for `e`
    /// import E from "e";
    /// // Attached comment for `d`
    /// import D from "d";
    /// // Detached comment (new chunk)
    ///
    /// // Attached comment for `b`
    /// import B from "b";
    /// // Attached comment for `a`
    /// import A from "a";
    /// ```
    ///
    /// ...is sorted as follows.
    /// A blank line is automatically added after the header comment to ensure that the attached comment doesn't merge with the header comment.
    ///
    /// ```js,ignore
    /// // Copyright notice and file header comment
    ///
    /// // Attached comment for `d`
    /// import D from "d";
    /// // Attached comment for `e`
    /// import E from "e";
    /// import F from "f";
    ///
    /// // Detached comment (new chunk)
    ///
    /// // Attached comment for `a`
    /// import A from "a";
    /// // Attached comment for `b`
    /// import B from "b";
    /// ```
    ///
    ///
    /// ## Import and export merging
    ///
    /// The organizer also merges imports and exports that can be merged.
    ///
    /// For example, the following code:
    ///
    /// ```ts,ignore
    /// import type { T1 } from "package";
    /// import type { T2 } from "package";
    /// import * as ns from "package";
    /// import D1 from "package";
    /// import D2 from "package";
    /// import { A } from "package";
    /// import { B } from "package";
    /// ```
    ///
    /// is merged as follows:
    ///
    /// ```ts,ignore
    /// import type { T1, T2 } from "package";
    /// import D1, * as ns from "package";
    /// import D2, { A, B } from "package";
    /// ```
    ///
    ///
    /// ## Named imports, named exports and attributes sorting
    ///
    /// The sorter also sorts named imports, named exports, as well as attributes.
    /// It uses a natural sort order for comparing numbers.
    ///
    /// The following code...
    ///
    /// ```js,ignore
    /// import { a, b, A, B, c10, c9 } from "a";
    ///
    /// export { a, b, A, B, c10, c9 } from "a";
    ///
    /// import special from  "special" with { "type": "ty", "metadata": "data" };
    /// ```
    ///
    /// ...is sorted as follows:
    ///
    /// ```js,ignore
    /// import { A, a, B, b, c9, c10 } from "a";
    ///
    /// export { A, a, B, b, c9, c10 } from "a";
    ///
    /// import special from  "special" with { "metadata": "data", "type": "ty" };
    /// ```
    ///
    ///
    /// ## Supported glob patterns
    ///
    /// You need to understand the structure of a source to understand which source matches a glob.
    /// A source is divided in source segments.
    /// Every source segment is delimited by the separator `/` or the start/end of the source.
    /// For instance `src/file.js` consists of two source segments: `src` and `file.js`.
    ///
    /// - star `*` that matches zero or more characters inside a source segment
    ///
    ///   `file.js` matches `*.js`.
    ///   Conversely, `src/file.js` doesn't match `*.js`
    ///
    /// - globstar `**` that matches zero or more source segments
    ///   `**` must be enclosed by separators `/` or the start/end of the glob.
    ///   For example, `**a` is not a valid glob.
    ///   Also, `**` must not be followed by another globstar.
    ///   For example, `**/**` is not a valid glob.
    ///
    ///   `file.js` and `src/file.js` match `**` and `**/*.js`
    ///   Conversely, `README.txt` doesn't match `**/*.js` because the source ends with `.txt`.
    ///
    /// - Use `\*` to escape `*`
    ///
    ///   `\*` matches the literal `*` character in a source.
    ///
    /// - `?`, `[`, `]`, `{`, and `}` must be escaped using `\`.
    ///   These characters are reserved for possible future use.
    ///
    /// - Use `!` as first character to negate a glob
    ///
    ///   `file.js` matches `!*.test.js`.
    ///   `src/file.js` matches `!*.js` because the source contains several segments.
    ///
    ///
    /// ## Common configurations
    ///
    /// This section provides some examples of common configurations.
    ///
    /// ### Placing `import type` and `export type` at the start of the chunks
    ///
    /// ```json
    /// {
    ///     "options": {
    ///         "groups": [
    ///             { "type": true }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// Note that you can want to use the lint rule [`useImportType`](https://next.biomejs.dev/linter/rules/use-import-type/) and its [`style`](https://next.biomejs.dev/linter/rules/use-import-type/#style) to enforce the use of `import type` instead of `import { type }`.
    ///
    /// ### Placing `import type` and `export type` at the end of the chunks
    ///
    /// ```json
    /// {
    ///     "options": {
    ///         "groups": [
    ///             { "type": false }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// ## Change the sorting of import identifiers to lexicographic sorting
    /// This only applies to the named import/exports and not the source itself.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "identifierOrder": "lexicographic"
    ///     }
    /// }
    /// ```
    /// ```js,use_options,expect_diagnostic
    /// import { var1, var2, var21, var11, var12, var22 } from 'my-package'
    /// ```
    ///
    /// ## Change the sorting of import identifiers to logical sorting
    /// This is the default behavior incase you do not override. This only applies to the named import/exports and not the source itself.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "identifierOrder": "natural"
    ///     }
    /// }
    /// ```
    /// ```js,use_options,expect_diagnostic
    /// import { var1, var2, var21, var11, var12, var22 } from 'my-package'
    /// ```
    ///
    pub OrganizeImports {
        version: "1.0.0",
        name: "organizeImports",
        language: "js",
        recommended: true,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for OrganizeImports {
    type Query = Ast<JsModule>;
    type State = Box<[Issue]>;
    type Signals = Option<Self::State>;
    type Options = OrganizeImportsOptions;

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        ctx.query()
            .items()
            .into_iter()
            .find(|item| {
                matches!(
                    item,
                    AnyJsModuleItem::JsImport(_) | AnyJsModuleItem::JsExport(_)
                )
            })
            .map(|item| item.range())
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("assist/source/organizeImports"),
            Self::text_range(ctx, state),
            markup! {
                "The imports and exports are not sorted."
            },
        ))
    }

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        struct ChunkBuilder {
            slot_indexes: std::ops::Range<u32>,
            max_key: ImportKey,
        }
        impl ChunkBuilder {
            fn new(key: ImportKey) -> Self {
                Self {
                    slot_indexes: key.slot_index..key.slot_index,
                    max_key: key,
                }
            }
        }

        fn report_unsorted_chunk(chunk: Option<ChunkBuilder>, result: &mut Vec<Issue>) {
            if let Some(chunk) = chunk
                && !chunk.slot_indexes.is_empty()
            {
                result.push(Issue::UnsortedChunkPrefix {
                    slot_indexes: chunk.slot_indexes,
                });
            }
        }

        let root = ctx.query();
        let mut result = Vec::new();
        let options = ctx.options();
        let sort_order = options.identifier_order;
        let mut chunk: Option<ChunkBuilder> = None;
        let mut prev_kind: Option<JsSyntaxKind> = None;
        let mut prev_group = 0;
        for item in root.items() {
            if let Some((info, specifiers, attributes)) = ImportInfo::from_module_item(&item) {
                let prev_is_distinct = prev_kind.is_some_and(|kind| kind != item.syntax().kind());
                // A detached comment marks the start of a new chunk
                if prev_is_distinct || has_detached_leading_comment(item.syntax()) {
                    // The chunk ends, here
                    report_unsorted_chunk(chunk.take(), &mut result);
                    prev_group = 0;
                }
                let key = ImportKey::new(info, &options.groups);
                let blank_line_separated_groups = options
                    .groups
                    .separated_by_blank_line(prev_group, key.group);
                let starts_chunk = chunk.is_none();
                let leading_newline_count = leading_newlines(item.syntax()).count();
                let are_specifiers_unsorted =
                    specifiers.is_some_and(|specifiers| !specifiers.are_sorted(sort_order));
                let are_attributes_unsorted = attributes.is_some_and(|attributes| {
                    // Assume the attributes are sorted if there are any bogus nodes.
                    !(are_import_attributes_sorted(&attributes, sort_order).unwrap_or(true))
                });
                let newline_issue = if leading_newline_count == 1
                    // A chunk must start with a blank line (two newlines)
                    // if an export or a statement precedes it.
                    && ((starts_chunk && prev_is_distinct) ||
                    // Some groups must be separated by a blank line
                    blank_line_separated_groups)
                {
                    NewLineIssue::MissingNewLine
                } else if leading_newline_count > 1
                    && !starts_chunk
                    // Ignore blank lines when groups are not explicitly set
                    && !options.groups.is_empty()
                    && !blank_line_separated_groups
                {
                    // An import inside a chunk must not start with a blank line
                    // if groups are explicitly set
                    NewLineIssue::ExtraNewLine
                } else {
                    NewLineIssue::None
                };
                if are_specifiers_unsorted
                    || are_attributes_unsorted
                    || !matches!(newline_issue, NewLineIssue::None)
                {
                    // Report the violation of one of the previous requirement
                    result.push(Issue::UnorganizedItem {
                        slot_index: key.slot_index,
                        are_specifiers_unsorted,
                        are_attributes_unsorted,
                        newline_issue,
                    });
                }
                if let Some(chunk) = &mut chunk {
                    // Check if the items are in order
                    if chunk.max_key > key || chunk.max_key.is_mergeable(&key) {
                        chunk.slot_indexes.end = key.slot_index + 1;
                    } else {
                        prev_group = key.group;
                        chunk.max_key = key;
                    }
                } else {
                    // New chunk
                    prev_group = key.group;
                    chunk = Some(ChunkBuilder::new(key));
                }
            } else if chunk.is_some() {
                // This is either
                // - a bare (side-effect) import
                // - a buggy import or export
                // a statement
                //
                // In any case, the chunk ends here
                report_unsorted_chunk(chunk.take(), &mut result);
                prev_group = 0;
                // A statement must be separated of a chunk with a blank line
                if let AnyJsModuleItem::AnyJsStatement(statement) = &item
                    && leading_newlines(statement.syntax()).count() == 1
                {
                    result.push(Issue::AddLeadingNewline {
                        slot_index: statement.syntax().index() as u32,
                    });
                }
            }
            prev_kind = Some(item.syntax().kind());
        }
        // Report the last chunk
        report_unsorted_chunk(chunk.take(), &mut result);
        if result.is_empty() {
            None
        } else {
            Some(result.into())
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        struct KeyedItem {
            /// Key associated to `item` before any merging.
            key: ImportKey,
            /// Was `item` obtained by merging it with previous items?
            was_merged: bool,
            /// None if the item has been merged with the next item.
            item: Option<AnyJsModuleItem>,
        }

        let options = ctx.options();
        let sort_order = options.identifier_order;
        let root = ctx.query();
        let items = root.items().into_syntax();
        let mut organized_items: FxHashMap<u32, AnyJsModuleItem> = FxHashMap::default();
        let mut import_keys: Vec<KeyedItem> = Vec::new();
        let mut mutation = ctx.root().begin();
        for issue in state {
            match issue {
                Issue::AddLeadingNewline { slot_index } => {
                    let item = root
                        .items()
                        .into_iter()
                        .nth(*slot_index as usize)?
                        .into_syntax();
                    if leading_newlines(&item).count() >= 1 {
                        let newline = item.first_leading_trivia()?.pieces().next();
                        let new_item = item.clone().prepend_trivia_pieces(newline)?;
                        mutation.replace_element_discard_trivia(item.into(), new_item.into());
                    }
                }
                Issue::UnorganizedItem {
                    slot_index,
                    are_attributes_unsorted,
                    are_specifiers_unsorted,
                    newline_issue,
                } => {
                    let item = root.items().into_iter().nth(*slot_index as usize)?;
                    let item: AnyJsModuleItem = match item {
                        AnyJsModuleItem::AnyJsStatement(_) => {
                            continue;
                        }
                        AnyJsModuleItem::JsExport(export) => {
                            let mut clause = export.export_clause().ok()?;
                            if *are_specifiers_unsorted {
                                // Sort named specifiers
                                if let AnyJsExportClause::JsExportNamedFromClause(cast) = &clause
                                    && let Some(sorted_specifiers) =
                                        sort_export_specifiers(&cast.specifiers(), sort_order)
                                {
                                    clause = cast.clone().with_specifiers(sorted_specifiers).into();
                                }
                            }
                            if *are_attributes_unsorted {
                                // Sort import attributes
                                let sorted_attrs = clause
                                    .attribute()
                                    .and_then(|attrs| sort_attributes(attrs, sort_order));
                                clause = clause.with_attribute(sorted_attrs);
                            }
                            export.with_export_clause(clause).into()
                        }
                        AnyJsModuleItem::JsImport(import) => {
                            let mut clause = import.import_clause().ok()?;
                            if *are_specifiers_unsorted {
                                // Sort named specifiers
                                if let Some(sorted_specifiers) =
                                    clause.named_specifiers().and_then(|specifiers| {
                                        sort_import_specifiers(specifiers, sort_order)
                                    })
                                {
                                    clause = clause.with_named_specifiers(sorted_specifiers)
                                }
                            }
                            if *are_attributes_unsorted {
                                // Sort import attributes
                                let sorted_attrs = clause
                                    .attribute()
                                    .and_then(|attrs| sort_attributes(attrs, sort_order));
                                clause = clause.with_attribute(sorted_attrs);
                            }
                            import.with_import_clause(clause).into()
                        }
                    };
                    let mut item = item.into_syntax();
                    // Fix newlines
                    match newline_issue {
                        NewLineIssue::None => {}
                        NewLineIssue::ExtraNewLine => {
                            // Remove extra newlines
                            let leading_trivia = item
                                .first_leading_trivia()?
                                .pieces()
                                .skip(leading_newlines(&item).count() - 1);
                            item = item.with_leading_trivia_pieces(leading_trivia)?;
                        }
                        NewLineIssue::MissingNewLine => {
                            // Add missing newline
                            let newline = leading_newlines(&item).next();
                            item = item.prepend_trivia_pieces(newline.into_iter())?
                        }
                    }
                    // Save the node
                    organized_items.insert(*slot_index, AnyJsModuleItem::cast(item)?);
                }
                Issue::UnsortedChunkPrefix { slot_indexes } => {
                    debug_assert!(import_keys.is_empty(), "import_keys was previously drained");
                    // Collect all import keys and the associated items.
                    import_keys.reserve(slot_indexes.len());
                    import_keys.extend(
                        ctx.query()
                            .items()
                            .into_iter()
                            .skip(slot_indexes.start as usize)
                            .take(slot_indexes.len())
                            .filter_map(|item| {
                                let info = ImportInfo::from_module_item(&item)?.0;
                                let item = organized_items.remove(&info.slot_index).unwrap_or(item);
                                Some(KeyedItem {
                                    key: ImportKey::new(info, &options.groups),
                                    was_merged: false,
                                    item: Some(item),
                                })
                            }),
                    );
                    // Sort imports based on their import key
                    import_keys.sort_unstable_by(
                        |KeyedItem { key: k1, .. }, KeyedItem { key: k2, .. }| k1.cmp(k2),
                    );

                    // Merge imports/exports
                    // We use `while` and indexing to allow both iteration and mutation of `import_keys`.
                    let mut i = import_keys.len() - 1;
                    while i > 0 {
                        let KeyedItem {
                            key: prev_key,
                            item: prev_item,
                            ..
                        } = &import_keys[i - 1];
                        let KeyedItem { key, item, .. } = &import_keys[i];
                        if prev_key.is_mergeable(key)
                            && let Some(merged) =
                                merge(prev_item.as_ref(), item.as_ref(), sort_order)
                        {
                            import_keys[i - 1].was_merged = true;
                            import_keys[i - 1].item = Some(merged);
                            import_keys[i].item = None;
                        }
                        i -= 1;
                    }
                    // Swap the items to obtain a sorted chunk
                    let mut prev_group: u16 = 0;
                    for (
                        index,
                        KeyedItem {
                            key,
                            was_merged,
                            item: new_item,
                        },
                    ) in (slot_indexes.start..).zip(import_keys.drain(..))
                    {
                        let old_item = items.element_in_slot(index)?;
                        let Some(new_item) = new_item else {
                            mutation.remove_element(old_item);
                            continue;
                        };
                        let mut new_item = new_item.into_syntax();
                        let old_item = old_item.into_node()?;
                        let blank_line_separated_groups = index != 0
                            && options
                                .groups
                                .separated_by_blank_line(prev_group, key.group);
                        prev_group = key.group;
                        // Don't make any change if it is the same node and no change have to be done
                        if !blank_line_separated_groups && index == key.slot_index && !was_merged {
                            continue;
                        }
                        if index == slot_indexes.start {
                            if index == key.slot_index && was_merged {
                                // Merged imports always have a leading newline.
                                // We remove it if the merged import is at the start and
                                // if the old first import has no leading newline.
                                if index == 0 && leading_newlines(&old_item).count() == 0 {
                                    new_item = new_item.trim_leading_trivia()?;
                                }
                            } else if let Some(detached) = detached_trivia(&old_item) {
                                if leading_newlines(&old_item).count() == 1 {
                                    let newline = old_item.first_leading_trivia()?.pieces().take(1);
                                    new_item = new_item.prepend_trivia_pieces(
                                        chain_trivia_pieces(newline, detached),
                                    )?;
                                } else {
                                    new_item = new_item.prepend_trivia_pieces(detached)?;
                                }
                            } else if index == 0 && leading_newlines(&old_item).count() == 0 {
                                // We are at the top of the file.
                                // Keep header (possibly Copyright notice)
                                let header_trivia = old_item.first_leading_trivia()?;
                                if header_trivia.is_empty() {
                                    new_item = new_item.trim_leading_trivia()?;
                                } else {
                                    new_item =
                                        new_item.prepend_trivia_pieces(header_trivia.pieces())?;
                                }
                            }
                        } else if let Some(attached) = attached_trivia(&new_item) {
                            // Transfer attached comment
                            new_item = new_item.with_leading_trivia_pieces(attached)?;
                        } else if key.slot_index == 0 && leading_newlines(&new_item).count() == 0 {
                            // Don't copy the header trivia
                            let first_token = new_item.first_token()?;
                            let new_first_token = first_token
                                .clone()
                                .with_leading_trivia([(TriviaPieceKind::Newline, "\n")]);
                            new_item = new_item
                                .replace_child(first_token.into(), new_first_token.into())?;
                        }
                        // Add newline for group separation
                        if index != 0
                            && blank_line_separated_groups
                            && leading_newlines(&new_item).count() == 1
                        {
                            let newline = leading_newlines(&new_item).next();
                            new_item = new_item.prepend_trivia_pieces(newline)?;
                        }
                        mutation.replace_element_discard_trivia(old_item.into(), new_item.into());
                    }
                }
            }
        }
        let items = ctx.query().items().into_syntax();
        for (slot_index, item) in organized_items {
            let Some(replaced_import) = items.element_in_slot(slot_index) else {
                continue;
            };
            mutation.replace_element_discard_trivia(replaced_import, item.into());
        }
        Some(JsRuleAction::new(
            ActionCategory::Source(SourceActionKind::OrganizeImports),
            ctx.metadata().applicability(),
            "Organize Imports (Biome)",
            mutation,
        ))
    }
}

#[derive(Debug)]
pub enum Issue {
    AddLeadingNewline {
        // Slot index of a statement that must starts with a blank line
        slot_index: u32,
    },
    /// Prefix of an unsorted chunk of imports or exports
    UnsortedChunkPrefix {
        /// Slot indexes of all the first imports or exports.
        slot_indexes: std::ops::Range<u32>,
    },
    /// Import or export with one or several of the following issues:
    /// - has unsorted specifiers
    /// - has unsorted attributes
    /// - has too many or not enough leading newlines
    UnorganizedItem {
        /// Slot index of the import or export
        slot_index: u32,
        are_attributes_unsorted: bool,
        are_specifiers_unsorted: bool,
        newline_issue: NewLineIssue,
    },
}

#[derive(Debug)]
pub enum NewLineIssue {
    /// No issue
    None,
    ExtraNewLine,
    MissingNewLine,
}

fn merge(
    item1: Option<&AnyJsModuleItem>,
    item2: Option<&AnyJsModuleItem>,
    sort_order: SortOrder,
) -> Option<AnyJsModuleItem> {
    match (item1?, item2?) {
        (AnyJsModuleItem::JsExport(item1), AnyJsModuleItem::JsExport(item2)) => {
            let clause1 = item1.export_clause().ok()?;
            let clause2 = item2.export_clause().ok()?;
            let AnyJsExportClause::JsExportNamedFromClause(clause1) = clause1 else {
                return None;
            };
            let clause2 = clause2.as_js_export_named_from_clause()?;
            let specifiers1 = clause1.specifiers();
            let specifiers2 = clause2.specifiers();
            if let Some(meregd_specifiers) =
                merge_export_specifiers(&specifiers1, &specifiers2, sort_order)
            {
                let meregd_clause = clause1.with_specifiers(meregd_specifiers);
                let merged_item = item2.clone().with_export_clause(meregd_clause.into());

                let item1_leading_trivia = item1.syntax().first_leading_trivia()?;
                let merged_item = if item1_leading_trivia.is_empty() {
                    merged_item
                } else {
                    merged_item
                        .trim_leading_trivia()?
                        .prepend_trivia_pieces(item1.syntax().first_leading_trivia()?.pieces())?
                };
                return Some(merged_item.into());
            }
        }
        (AnyJsModuleItem::JsImport(item1), AnyJsModuleItem::JsImport(item2)) => {
            let clause1 = item1.import_clause().ok()?;
            let clause2 = item2.import_clause().ok()?;
            match (clause1, clause2) {
                (
                    AnyJsImportClause::JsImportDefaultClause(clause1),
                    AnyJsImportClause::JsImportNamespaceClause(clause2),
                )
                | (
                    AnyJsImportClause::JsImportNamespaceClause(clause2),
                    AnyJsImportClause::JsImportDefaultClause(clause1),
                ) => {
                    let default_specifier = clause1.default_specifier().ok()?;
                    let namespace_specifier = clause2.namespace_specifier().ok()?;
                    let comma_token = make::token(T![,])
                        .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);
                    let merged_clause = make::js_import_combined_clause(
                        default_specifier.trim_trailing_trivia()?,
                        comma_token,
                        namespace_specifier.into(),
                        clause2.from_token().ok()?,
                        clause2.source().ok()?,
                    )
                    .build();
                    let merged_item = item2.clone().with_import_clause(merged_clause.into());

                    let item1_leading_trivia = item1.syntax().first_leading_trivia()?;
                    let merged_item = if item1_leading_trivia.is_empty() {
                        merged_item
                    } else {
                        merged_item.trim_leading_trivia()?.prepend_trivia_pieces(
                            item1.syntax().first_leading_trivia()?.pieces(),
                        )?
                    };
                    return Some(merged_item.into());
                }
                (
                    AnyJsImportClause::JsImportCombinedClause(clause1),
                    AnyJsImportClause::JsImportNamedClause(clause2),
                )
                | (
                    AnyJsImportClause::JsImportNamedClause(clause2),
                    AnyJsImportClause::JsImportCombinedClause(clause1),
                ) => {
                    let specifier1 = clause1.specifier().ok()?;
                    let AnyJsCombinedSpecifier::JsNamedImportSpecifiers(specifiers1) = specifier1
                    else {
                        return None;
                    };
                    let specifiers2 = clause2.named_specifiers().ok()?;
                    if let Some(meregd_specifiers) =
                        merge_import_specifiers(specifiers1, &specifiers2, sort_order)
                    {
                        let merged_clause = clause1.with_specifier(meregd_specifiers.into());
                        let merged_item = item2.clone().with_import_clause(merged_clause.into());

                        let item1_leading_trivia = item1.syntax().first_leading_trivia()?;
                        let merged_item = if item1_leading_trivia.is_empty() {
                            merged_item
                        } else {
                            merged_item.trim_leading_trivia()?.prepend_trivia_pieces(
                                item1.syntax().first_leading_trivia()?.pieces(),
                            )?
                        };
                        return Some(merged_item.into());
                    }
                }
                (
                    AnyJsImportClause::JsImportNamedClause(clause1),
                    AnyJsImportClause::JsImportNamedClause(clause2),
                ) => {
                    let specifiers1 = clause1.named_specifiers().ok()?;
                    let specifiers2 = clause2.named_specifiers().ok()?;
                    if let Some(meregd_specifiers) =
                        merge_import_specifiers(specifiers1, &specifiers2, sort_order)
                    {
                        let merged_clause = clause1.with_named_specifiers(meregd_specifiers);
                        let merged_item = item2.clone().with_import_clause(merged_clause.into());
                        let item1_leading_trivia = item1.syntax().first_leading_trivia()?;
                        let merged_item = if item1_leading_trivia.is_empty() {
                            merged_item
                        } else {
                            merged_item.trim_leading_trivia()?.prepend_trivia_pieces(
                                item1.syntax().first_leading_trivia()?.pieces(),
                            )?
                        };
                        return Some(merged_item.into());
                    }
                }
                (
                    AnyJsImportClause::JsImportDefaultClause(clause1),
                    AnyJsImportClause::JsImportNamedClause(clause2),
                )
                | (
                    AnyJsImportClause::JsImportNamedClause(clause2),
                    AnyJsImportClause::JsImportDefaultClause(clause1),
                ) => {
                    let default_specifier = clause1.default_specifier().ok()?;
                    let named_specifiers = clause2.named_specifiers().ok()?;
                    let comma_token = make::token(T![,])
                        .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);
                    let merged_clause = make::js_import_combined_clause(
                        default_specifier.trim_trailing_trivia()?,
                        comma_token,
                        named_specifiers.into(),
                        clause2.from_token().ok()?,
                        clause2.source().ok()?,
                    )
                    .build();
                    let merged_item = item2.clone().with_import_clause(merged_clause.into());
                    let item1_leading_trivia = item1.syntax().first_leading_trivia()?;
                    let merged_item = if item1_leading_trivia.is_empty() {
                        merged_item
                    } else {
                        merged_item.trim_leading_trivia()?.prepend_trivia_pieces(
                            item1.syntax().first_leading_trivia()?.pieces(),
                        )?
                    };
                    return Some(merged_item.into());
                }
                _ => {}
            }
        }
        _ => {}
    }
    None
}
