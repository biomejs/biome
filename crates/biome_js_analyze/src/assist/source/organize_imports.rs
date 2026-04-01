pub mod import_key;
pub mod specifiers_attributes;
mod util;

use crate::JsRuleAction;
use biome_analyze::{
    ActionCategory, Ast, FixKind, Rule, RuleDiagnostic, RuleSource, SourceActionKind,
    context::RuleContext, declare_source_rule,
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
    are_import_attributes_sorted, merge_export_from_specifiers, merge_export_specifiers,
    merge_import_specifiers, sort_attributes, sort_export_from_specifiers, sort_export_specifiers,
    sort_import_specifiers,
};
use util::{attached_trivia, detached_trivia, has_detached_leading_comment, leading_newlines};

declare_source_rule! {
    /// Sorts and organizes import and export statements in your JavaScript and TypeScript files.
    ///
    /// By default, imports are sorted by "distance" from the current file -- external dependencies appear first, local files appear last.
    /// The organizer also merges duplicate imports from the same source and sorts named specifiers alphabetically.
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
    /// The default sort order is:
    ///
    /// 1. URLs (`https://example.org`)
    /// 2. Packages with a protocol (`node:path`, `bun:test`, `jsr:@my/lib`)
    /// 3. Bare and scoped packages (`lib`, `@scoped/lib`)
    /// 4. Aliases (`#internal`, `@/components`, `~/utils`)
    /// 5. Relative and absolute paths (`./file.js`, `../parent.js`)
    ///
    /// Within each category, imports are sorted using a [natural sort order](https://en.wikipedia.org/wiki/Natural_sort_order) where `A < a < B < b` and `a9 < a10`.
    ///
    ///
    /// ## Options
    ///
    /// ### `groups`
    ///
    /// Customize how imports are grouped and separated.
    /// By default, all imports in a block form a single group sorted by the built-in order.
    /// The `groups` option lets you split imports into separate groups, optionally separated by blank lines.
    ///
    /// A common setup: separate Node.js builtins, third-party packages, and local imports with blank lines:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "groups": [
    ///             [":BUN:", ":NODE:"],
    ///             ":BLANK_LINE:",
    ///             ":PACKAGE:",
    ///             ":BLANK_LINE:",
    ///             [":ALIAS:", ":PATH:"]
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// With this configuration, the following code...
    ///
    /// ```js,ignore
    /// import aliased from "@/components/Button";
    /// import lib from "lib";
    /// import path from "node:path";
    /// import sibling from "./file.js";
    /// import scopedLib from "@scoped/lib";
    /// import fs from "fs";
    /// ```
    ///
    /// ...is sorted as:
    ///
    /// ```js,ignore
    /// import fs from "fs";
    /// import path from "node:path";
    ///
    /// import scopedLib from "@scoped/lib";
    /// import lib from "lib";
    ///
    /// import aliased from "@/components/Button";
    /// import sibling from "./file.js";
    /// ```
    ///
    /// Each entry in the `groups` array is a **group matcher** that can be:
    ///
    /// - A **predefined group** like `:NODE:` or `:PACKAGE:`
    /// - A **glob pattern** like `@my/lib/**`
    /// - An **object matcher** like `{ "type": true }` for type-only imports
    /// - A **list** combining any of the above, e.g. `[":BUN:", ":NODE:"]`
    /// - `:BLANK_LINE:` to insert a blank line between groups
    ///
    /// Imports that don't match any group are placed at the end.
    /// Groups are matched in order, so earlier matchers take priority.
    ///
    /// #### Predefined groups
    ///
    /// - `:URL:` -- sources starting with `https://` or `http://`
    /// - `:NODE:` -- Node.js built-in modules (`node:path`, `fs`, `path`, etc.)
    /// - `:BUN:` -- Bun built-in modules (`bun:test`, `bun`, etc.)
    /// - `:PACKAGE_WITH_PROTOCOL:` -- packages with a protocol (`jsr:@my/lib`, `npm:lib`)
    /// - `:PACKAGE:` -- bare and scoped packages (`lib`, `@scoped/lib`)
    /// - `:ALIAS:` -- path aliases starting with `#`, `@/`, `~`, `$`, or `%`
    /// - `:PATH:` -- absolute and relative paths
    ///
    /// #### Glob patterns
    ///
    /// Use glob patterns to match specific import sources.
    /// Prefix with `!` to exclude sources from a group.
    /// You can also negate predefined groups: `!:NODE:` matches everything that isn't a Node.js built-in.
    ///
    /// ```json,options
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
    /// Given this configuration and the following code...
    ///
    /// ```js,ignore
    /// import lib from "@my/lib";
    /// import aliased from "@/alias";
    /// import path from "@my/lib/special";
    /// import test from "@my/lib/path";
    /// ```
    ///
    /// ...the result is:
    ///
    /// ```js,ignore
    /// import lib from "@my/lib";
    /// import test from "@my/lib/path";
    /// import aliased from "@/alias";
    /// import path from "@my/lib/special";
    /// ```
    ///
    /// Note that `@my/lib` matches the literal source `@my/lib` but not `@my/lib/subpath`.
    /// Conversely, `@my/lib/**` matches `@my/lib/subpath` but not `@my/lib` itself.
    /// Specify both patterns to match all imports starting with `@my/lib`.
    /// See the [Glob pattern reference](#glob-pattern-reference) section for full details.
    ///
    /// #### Object matchers for type imports
    ///
    /// Use an object matcher to separate `import type` from regular imports:
    ///
    /// ```json,options
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
    /// Setting `"type": false` matches only non-type imports. Setting `"type": true` matches only `import type` / `export type` statements.
    ///
    /// ### `identifierOrder`
    ///
    /// Controls how named specifiers inside `{ ... }` are sorted.
    /// Accepts `"natural"` (default) or `"lexicographic"`.
    ///
    /// - `"natural"`: sorts numbers by their value (`var9` before `var10`)
    /// - `"lexicographic"`: sorts character by character (`var10` before `var9`)
    ///
    /// #### Lexicographic sorting
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
    /// #### Natural sorting (default)
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
    ///
    /// ## Common configurations
    ///
    /// ### Separate Node.js builtins from third-party and local imports
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "groups": [
    ///             [":BUN:", ":NODE:"],
    ///             ":BLANK_LINE:",
    ///             ":PACKAGE:",
    ///             ":BLANK_LINE:",
    ///             [":ALIAS:", ":PATH:"]
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// ### Group a specific library's imports together
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "groups": [
    ///             [":BUN:", ":NODE:"],
    ///             ":BLANK_LINE:",
    ///             ["react", "react/**", "react-dom", "react-dom/**"],
    ///             ":BLANK_LINE:",
    ///             ":PACKAGE:"
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// Given the following code...
    ///
    /// ```js,ignore
    /// import lib from "lib";
    /// import { useState } from "react";
    /// import path from "node:path";
    /// import { render } from "react-dom/client";
    /// ```
    ///
    /// ...the result is:
    ///
    /// ```js,ignore
    /// import path from "node:path";
    ///
    /// import { useState } from "react";
    /// import { render } from "react-dom/client";
    ///
    /// import lib from "lib";
    /// ```
    ///
    /// ### Place `import type` at the start
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "groups": [
    ///             { "type": true }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// You may also want to use the lint rule [`useImportType`](https://biomejs.dev/linter/rules/use-import-type/) and its [`style`](https://biomejs.dev/linter/rules/use-import-type/#style) option to enforce `import type` instead of `import { type }`.
    ///
    /// ### Place `import type` at the end
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "groups": [
    ///             { "type": false }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// ### Separate type imports with a blank line
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "groups": [
    ///             { "type": true },
    ///             ":BLANK_LINE:",
    ///             { "type": false }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// Given the following code...
    ///
    /// ```ts,ignore
    /// import { useState } from "react";
    /// import type { FC } from "react";
    /// import type { User } from "./types";
    /// import { fetchUser } from "./api";
    /// ```
    ///
    /// ...the result is:
    ///
    /// ```ts,ignore
    /// import type { FC } from "react";
    /// import type { User } from "./types";
    ///
    /// import { fetchUser } from "./api";
    /// import { useState } from "react";
    /// ```
    ///
    /// ### Separate monorepo packages from external packages
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "groups": [
    ///             ":PACKAGE:",
    ///             ":BLANK_LINE:",
    ///             ["@mycompany", "@mycompany/**"]
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// Given the following code...
    ///
    /// ```js,ignore
    /// import { Button } from "@mycompany/ui";
    /// import express from "express";
    /// import { db } from "@mycompany/db";
    /// import { handler } from "./handler.js";
    /// ```
    ///
    /// ...the result is:
    ///
    /// ```js,ignore
    /// import express from "express";
    ///
    /// import { Button } from "@mycompany/ui";
    /// import { db } from "@mycompany/db";
    /// import { handler } from "./handler.js";
    /// ```
    ///
    /// ### Place CSS/style imports at the bottom
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "groups": [
    ///             ["**", "!**/*.css", "!**/*.scss"],
    ///             ":BLANK_LINE:",
    ///             ["**/*.css", "**/*.scss"]
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// Given the following code...
    ///
    /// ```js,ignore
    /// import "./styles/reset.css";
    /// import { useState } from "react";
    /// import styles from "./Component.module.css";
    /// import { Button } from "@/components/Button";
    /// ```
    ///
    /// ...the result is:
    ///
    /// ```js,ignore
    /// import { Button } from "@/components/Button";
    /// import { useState } from "react";
    ///
    /// import styles from "./Component.module.css";
    /// import "./styles/reset.css";
    /// ```
    ///
    /// ### Group test utilities together
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "groups": [
    ///             ["vitest", "vitest/**", "@testing-library", "@testing-library/**", "jest", "@jest/**"],
    ///             ":BLANK_LINE:",
    ///             ":PACKAGE:"
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// Given the following code...
    ///
    /// ```js,ignore
    /// import { render } from "@testing-library/react";
    /// import { Button } from "@/components/Button";
    /// import { describe, it, expect } from "vitest";
    /// import { server } from "./mocks/server";
    /// ```
    ///
    /// ...the result is:
    ///
    /// ```js,ignore
    /// import { describe, it, expect } from "vitest";
    /// import { render } from "@testing-library/react";
    ///
    /// import { Button } from "@/components/Button";
    /// import { server } from "./mocks/server";
    /// ```
    ///
    /// ### Place framework imports first
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "groups": [
    ///             ["next", "next/**"],
    ///             ":BLANK_LINE:",
    ///             ":PACKAGE:"
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// Given the following code...
    ///
    /// ```js,ignore
    /// import express from "express";
    /// import Link from "next/link";
    /// import { useRouter } from "next/navigation";
    /// import { db } from "./db";
    /// ```
    ///
    /// ...the result is:
    ///
    /// ```js,ignore
    /// import Link from "next/link";
    /// import { useRouter } from "next/navigation";
    ///
    /// import express from "express";
    /// import { db } from "./db";
    /// ```
    ///
    /// ### Maximize import merging with `useImportType`
    ///
    /// The organizer merges imports from the same source when possible.
    /// To merge type-only imports (`import type { T }`) with regular imports (`import { V }`), enable [`useImportType`](https://biomejs.dev/linter/rules/use-import-type/) with `inlineType`:
    ///
    /// ```json
    /// {
    ///   "linter": {
    ///     "rules": {
    ///       "style": {
    ///         "useImportType": {
    ///           "level": "on",
    ///           "options": { "style": "inlineType" }
    ///         }
    ///       }
    ///     }
    ///   },
    ///   "assist": {
    ///     "actions": { "source": { "organizeImports": "on" } }
    ///   }
    /// }
    /// ```
    ///
    /// With this config, these imports...
    ///
    /// ```ts,ignore
    /// import type { T1 } from "package";
    /// import type { T2 } from "package";
    /// import { A } from "package";
    /// import { B } from "package";
    /// ```
    ///
    /// ...are merged into:
    ///
    /// ```ts,ignore
    /// import { A, B, type T1, type T2 } from "package";
    /// ```
    ///
    ///
    /// ## How it works
    ///
    /// This section explains the internal mechanics of the organizer in detail.
    ///
    /// ### Import anatomy
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
    /// **source** is also often called **specifier** in the JavaScript ecosystem.
    ///
    ///
    /// ### Chunks
    ///
    /// Before sorting, imports and exports are divided into **chunks**.
    /// A chunk is a sequence of adjacent imports or exports (not a mix of both).
    /// The organizer never moves imports across chunk boundaries.
    ///
    /// Chunks are separated by:
    /// - Switching between imports and exports
    /// - Any non-import/export statement
    /// - Side-effect imports (`import "polyfill"`) -- each forms its own chunk
    /// - Detached comments (a comment followed by a blank line)
    ///
    /// ```js,ignore
    /// // chunk 1
    /// import A from "a";
    /// import * as B from "b";
    /// // chunk 2 (side-effect import)
    /// import "x";
    /// // chunk 3
    /// import { C } from "c";
    /// // chunk 4 (switches to exports)
    /// export * from "d";
    /// function f() {}
    /// // chunk 5 (after a statement)
    /// export * as E from "e";
    /// export { F } from "f";
    /// ```
    ///
    /// Blank lines alone do **not** create new chunks. Only detached comments (a comment followed by a blank line) split chunks:
    ///
    /// ```js,ignore
    /// // Attached comment
    /// import A from "a";
    ///
    /// // Still same chunk (blank line alone doesn't split)
    /// import * as B from "b";
    /// // Detached comment (followed by blank line)
    ///
    /// // New chunk starts here
    /// import { C } from "c";
    /// ```
    ///
    /// The organizer ensures blank lines between different chunks.
    /// Side-effect imports adjacent to a chunk are not separated by a blank line.
    ///
    ///
    /// ### Sorting within a chunk
    ///
    /// Imports are sorted by their source using the distance-based order described above.
    ///
    /// When two imports share the same source, they are ordered by kind:
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
    /// Imports with attributes (`with { ... }`) are always placed first.
    ///
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
    /// This kind ordering cannot be changed.
    ///
    ///
    /// ### Named specifier and attribute sorting
    ///
    /// Named imports, named exports, and import attributes are also sorted.
    ///
    /// ```js,ignore
    /// import { a, b, A, B, c10, c9 } from "a";
    ///
    /// export { a, b, A, B, c10, c9 } from "a";
    ///
    /// import special from  "special" with { "type": "ty", "metadata": "data" };
    /// ```
    ///
    /// becomes:
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
    /// ### Import merging
    ///
    /// Imports from the same source in the same chunk are merged when possible:
    ///
    /// ```ts,ignore
    /// import type { T1 } from "package";
    /// import type { T2 } from "package";
    /// import * as ns from "package";
    /// import D1 from "package";
    /// import D2 from "package";
    /// import { A } from "package";
    /// import { B } from "package";
    /// import { type T3 } from "package";
    /// ```
    ///
    /// becomes:
    ///
    /// ```ts,ignore
    /// import type { T1, T2 } from "package";
    /// import D1, * as ns from "package";
    /// import D2, { A, B, type T3 } from "package";
    /// ```
    ///
    /// With [`useImportType`](https://biomejs.dev/linter/rules/use-import-type/) set to `separatedType`, the result is:
    ///
    /// ```ts,ignore
    /// import type { T1, T2, T3 } from "package";
    /// import { V1 } from "package";
    /// import D1, * as ns from "package";
    /// import D2, { A, B } from "package";
    /// ```
    ///
    ///
    /// ### Comment handling
    ///
    /// Comments directly above an import (attached comments) move with that import when it is sorted.
    /// Comments followed by a blank line (detached comments) stay in place and create a new chunk.
    ///
    /// File-header comments (at the very top of the file) are always treated as detached, even without a blank line.
    /// This preserves copyright notices and license headers.
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
    /// becomes:
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
    /// ### Glob pattern reference
    ///
    /// A source is split into segments by `/`. For example, `src/file.js` has two segments: `src` and `file.js`.
    ///
    /// - `*` -- matches zero or more characters within a single segment.
    ///   `file.js` matches `*.js`, but `src/file.js` does not.
    ///
    /// - `**` -- matches zero or more segments.
    ///   Must be enclosed by `/` or be at the start/end.
    ///   `file.js` and `src/file.js` both match `**/*.js`.
    ///
    /// - `!` -- negates a pattern when used as the first character.
    ///   `file.js` matches `!*.test.js`.
    ///   Exceptions can be layered: `["@my/lib/**", "!@my/lib/internal/**", "@my/lib/internal/allowed/**"]`.
    ///
    /// - `\*` -- matches a literal `*` character.
    ///
    /// - `?`, `[`, `]`, `{`, `}` -- reserved characters, must be escaped with `\`.
    ///
    pub OrganizeImports {
        version: "1.0.0",
        name: "organizeImports",
        language: "js",
        recommended: true,
        fix_kind: FixKind::Safe,
        sources: &[RuleSource::Eslint("sort-imports").inspired(), RuleSource::Eslint("no-duplicate-imports").inspired(), RuleSource::EslintImport("order").inspired()],
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
        let groups = options.groups.as_ref();
        let sort_order = options.identifier_order.unwrap_or_default();
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
                let key = ImportKey::new(info, groups);
                let blank_line_separated_groups = groups
                    .is_some_and(|groups| groups.separated_by_blank_line(prev_group, key.group));
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
                    && !groups.is_none_or(|groups| groups.is_empty())
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
                // - a statement
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
        let groups = options.groups.as_ref();
        let sort_order = options.identifier_order.unwrap_or_default();
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
                                        sort_export_from_specifiers(&cast.specifiers(), sort_order)
                                {
                                    clause = cast.clone().with_specifiers(sorted_specifiers).into();
                                } else if let AnyJsExportClause::JsExportNamedClause(cast) = &clause
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
                                    key: ImportKey::new(info, groups),
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
                            && groups.is_some_and(|groups| {
                                groups.separated_by_blank_line(prev_group, key.group)
                            });
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
            let merged_item = match (clause1, clause2) {
                (
                    AnyJsExportClause::JsExportNamedFromClause(clause1),
                    AnyJsExportClause::JsExportNamedFromClause(clause2),
                ) => {
                    let specifiers1 = clause1.specifiers();
                    let specifiers2 = clause2.specifiers();
                    let merged_specifiers =
                        merge_export_from_specifiers(&specifiers1, &specifiers2, sort_order)?;
                    let merged_specifiers = clause1.with_specifiers(merged_specifiers);
                    item2.clone().with_export_clause(merged_specifiers.into())
                }
                (
                    AnyJsExportClause::JsExportNamedClause(clause1),
                    AnyJsExportClause::JsExportNamedClause(clause2),
                ) => {
                    let specifiers1 = clause1.specifiers();
                    let specifiers2 = clause2.specifiers();
                    let merged_specifiers =
                        merge_export_specifiers(&specifiers1, &specifiers2, sort_order)?;
                    let merged_specifiers = clause1.with_specifiers(merged_specifiers);
                    item2.clone().with_export_clause(merged_specifiers.into())
                }
                _ => return None,
            };
            let item1_leading_trivia = item1.syntax().first_leading_trivia()?;
            let merged_item = if item1_leading_trivia.is_empty() {
                merged_item
            } else {
                merged_item
                    .trim_leading_trivia()?
                    .prepend_trivia_pieces(item1_leading_trivia.pieces())?
            };
            Some(merged_item.into())
        }
        (AnyJsModuleItem::JsImport(item1), AnyJsModuleItem::JsImport(item2)) => {
            let clause1 = item1.import_clause().ok()?;
            let clause2 = item2.import_clause().ok()?;
            let merged_item = match (clause1, clause2) {
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
                    item2.clone().with_import_clause(merged_clause.into())
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
                    let merged_specifiers =
                        merge_import_specifiers(specifiers1, &specifiers2, sort_order)?;
                    let merged_clause = clause1.with_specifier(merged_specifiers.into());
                    item2.clone().with_import_clause(merged_clause.into())
                }
                (
                    AnyJsImportClause::JsImportNamedClause(clause1),
                    AnyJsImportClause::JsImportNamedClause(clause2),
                ) => {
                    let specifiers1 = clause1.named_specifiers().ok()?;
                    let specifiers2 = clause2.named_specifiers().ok()?;
                    let merged_specifiers =
                        merge_import_specifiers(specifiers1, &specifiers2, sort_order)?;
                    let merged_clause = clause1.with_named_specifiers(merged_specifiers);
                    item2.clone().with_import_clause(merged_clause.into())
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
                    item2.clone().with_import_clause(merged_clause.into())
                }
                _ => return None,
            };
            let item1_leading_trivia = item1.syntax().first_leading_trivia()?;
            let merged_item = if item1_leading_trivia.is_empty() {
                merged_item
            } else {
                merged_item
                    .trim_leading_trivia()?
                    .prepend_trivia_pieces(item1_leading_trivia.pieces())?
            };
            Some(merged_item.into())
        }
        _ => None,
    }
}
