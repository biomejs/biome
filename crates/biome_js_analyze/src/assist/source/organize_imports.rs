use biome_analyze::{
    ActionCategory, Ast, FixKind, Rule, SourceActionKind, context::RuleContext, declare_source_rule,
};
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{AnyJsExportClause, AnyJsModuleItem, JsModule, JsSyntaxKind, JsSyntaxNode};
use biome_rowan::{AstNode, TriviaPieceKind};
use biome_rowan::{BatchMutationExt, chain_trivia_pieces};
use import_key::{ImportInfo, ImportKey};
use rustc_hash::FxHashMap;
use specifiers_attributes::{
    are_import_attributes_sorted, sort_attributes, sort_export_specifiers, sort_import_specifiers,
};

use crate::JsRuleAction;
use util::{attached_trivia, detached_trivia, has_detached_leading_comment, leading_newline_count};

pub mod comparable_token;
pub mod import_groups;
pub mod import_key;
pub mod import_source;
pub mod specifiers_attributes;
mod util;

declare_source_rule! {
    /// Provides a whole-source code action to sort the imports in the file using import groups and natural ordering.
    ///
    /// ## How imports are sorted
    ///
    /// Import statements are sorted by "distance". Modules that are "farther" from the user are put on the top, modules "closer" to the user are put on the bottom:
    ///
    /// 1. modules imported via `bun:` protocol. This is applicable when writing code run by Bun;
    /// 1. built-in Node.js modules that are explicitly imported using the `node:` protocol and common Node built-ins such as `assert`;
    /// 1. modules imported via `npm:` protocol. This is applicable when writing code run by Deno;
    /// 1. modules that contain the protocol `:`. These are usually considered "virtual modules", modules that are injected by your working environment, e.g. `vite`;
    /// 1. modules imported via URL;
    /// 1. modules imported from libraries;
    /// 1. modules imported via absolute imports;
    /// 1. modules imported from a name prefixed by `#`. This is applicable when using [Node's subpath imports](https://nodejs.org/api/packages.html#subpath-imports);
    /// 1. modules imported via relative imports;
    /// 1. modules that couldn't be identified by the previous criteria;
    ///
    /// For example, given the following code:
    ///
    /// ```ts
    /// import uncle from "../uncle";
    /// import sibling from "./sibling";
    /// import express from "npm:express";
    /// import imageUrl from "url:./image.png";
    /// import { sortBy } from "virtual:utils";
    /// import assert from "node:assert";
    /// import aunt from "../aunt";
    /// import { VERSION } from "https://deno.land/std/version.ts";
    /// import { mock, test } from "node:test";
    /// import { expect } from "bun:test";
    /// import { internal } from "#internal";
    /// import { secret } from "/absolute/path";
    /// import React from "react";
    /// ```
    ///
    /// They will be sorted like this:
    ///
    /// ```ts
    /// import { expect } from "bun:test";
    /// import assert from "node:assert";
    /// import { mock, test } from "node:test";
    /// import express from "npm:express";
    /// import { sortBy } from "virtual:utils";
    /// import { VERSION } from "https://deno.land/std/version.ts";
    /// import React from "react";
    /// import { secret } from "/absolute/path";
    /// import { internal } from "#internal";
    /// import aunt from "../aunt";
    /// import uncle from "../uncle";
    /// import sibling from "./sibling";
    /// import imageUrl from "url:./image.png";
    /// ```
    ///
    /// You can apply the sorting in two ways: via [CLI](#import-sorting-via-cli) or [VSCode extension](#import-sorting-via-vscode-extension).
    ///
    /// ## Grouped imports
    ///
    /// It's widespread to have import statements in a certain order, primarily when you work on a frontend project, and you import CSS files:
    ///
    /// ```js
    /// import "../styles/reset.css";
    /// import "../styles/layout.css";
    /// import { Grid } from "../components/Grid.jsx";
    /// ```
    ///
    /// Another common case is import polyfills or shim files, that needs to stay at the top file:
    ///
    /// ```js
    /// import "../polyfills/array/flatMap";
    /// import { functionThatUsesFlatMap } from "./utils.js";
    /// ```
    ///
    /// In these cases, Biome will sort all these three imports, and it might happen that the order will **break** your application.
    ///
    /// To avoid this, create a "group" of imports. You create a "group" by adding a **new line** to separate the groups.
    ///
    /// By doing so, Biome will limit the sorting only to the import statements that belong to the same group:
    ///
    /// ```js
    /// // group 1, only these two files will be sorted
    /// import "../styles/reset.css";
    /// import "../styles/layout.css";
    ///
    /// // group 2, only this one is sorted
    /// import { Grid } from "../components/Grid.jsx";
    /// ```
    ///
    /// ```js
    /// // group 1, the polyfill/shim
    /// import "../polyfills/array/flatMap";
    ///
    /// // group 2, the files that require the polyfill/shim
    /// import { functionThatUsesFlatMap } from "./utils.js";
    /// ```
    ///
    /// ## Side effect imports
    ///
    /// [Side effect imports](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import#forms_of_import_declarations) are import statements that usually don't import any name:
    ///
    /// ```js
    /// import "./global.js"
    /// ```
    ///
    /// Since it is difficult to determine which side effects a module triggers, the import sorter assumes that each side effect import forms its own import group.
    ///
    /// For example, the following imports form 4 import groups.
    ///
    /// ```js
    /// import sibling from "./sibling";       // Import group 1
    /// import { internal } from "#internal";  // Import group 1
    /// import "z";  // Import group 2
    /// import "a";  // Import group 3
    /// import React from "react";         // Import group 4
    /// import assert from "node:assert";  // Import group 4
    /// ```
    ///
    /// Each group is independently sorted as follows:
    ///
    /// ```js
    /// import { internal } from "#internal";  // Import group 1
    /// import sibling from "./sibling";      // Import group 1
    /// import "z";  // Import group 2
    /// import "a";  // Import group 3
    /// import assert from "node:assert";  // Import group 4
    /// import React from "react";         // Import group 4
    /// ```
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
    type Options = Options;

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
            if let Some(chunk) = chunk {
                if !chunk.slot_indexes.is_empty() {
                    result.push(Issue::UnsortedChunkPrefix {
                        slot_indexes: chunk.slot_indexes,
                    });
                }
            }
        }

        let root = ctx.query();
        let mut result = Vec::new();
        let options = ctx.options();
        let mut chunk: Option<ChunkBuilder> = None;
        let mut prev_item_kind: Option<JsSyntaxKind> = None;
        for item in root.items() {
            if let Some((info, specifiers, attributes)) = ImportInfo::from_module_item(&item) {
                let prev_is_distinct =
                    prev_item_kind.is_some_and(|kind| kind != item.syntax().kind());
                // A detached comment marks the start of a new chunk
                if prev_is_distinct || has_detached_leading_comment(item.syntax()) {
                    // The chunk ends, here
                    report_unsorted_chunk(chunk.take(), &mut result);
                }
                let key = ImportKey::new(info, options.groups.as_ref());
                let starts_chunk = chunk.is_none();
                let leading_newline = leading_newline_count(item.syntax());
                // A chunk must start with a blank line (two newlines)
                // if an export or a statement precedes it.
                // Note that we assume that every import has at least one leading newline because the formatter guarantee it.
                if (starts_chunk && leading_newline == 1 && prev_is_distinct)
                    // An import inside a chunk must not start with a blank line
                    // if groups are explicitly set
                    || (!starts_chunk && leading_newline > 1 && options.groups.is_some())
                    // Specifiers must be sorted
                    || specifiers
                        .is_some_and(|specifiers| !specifiers.are_sorted())
                    // Attributes must be sorted
                    || attributes.is_some_and(|attributes| {
                        !(are_import_attributes_sorted(&attributes).unwrap_or_default())
                    })
                {
                    // Report the violation of one of the previous requirement
                    result.push(Issue::UnorganizedItem {
                        slot_index: key.slot_index,
                        starts_chunk,
                    });
                }
                if let Some(chunk) = &mut chunk {
                    // Check if the items are in order
                    if chunk.max_key > key {
                        chunk.slot_indexes.end = key.slot_index + 1;
                    } else {
                        chunk.max_key = key;
                    }
                } else {
                    // New chunk
                    chunk = Some(ChunkBuilder::new(key))
                }
            } else if chunk.is_some() {
                // This is either
                // - a bare (side-effect) import
                // - a buggy import or export
                // a statement
                //
                // In any case, the chunk ends here
                report_unsorted_chunk(chunk.take(), &mut result);
                // A statement must be separated of a chunk with a blank line
                if let AnyJsModuleItem::AnyJsStatement(statement) = &item {
                    if leading_newline_count(statement.syntax()) == 1 {
                        result.push(Issue::AddLeadingNewline {
                            slot_index: statement.syntax().index() as u32,
                        });
                    }
                }
            }
            prev_item_kind = Some(item.syntax().kind());
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
        let options = ctx.options();
        let root = ctx.query();
        let mut organized_items: FxHashMap<u32, JsSyntaxNode> = FxHashMap::default();
        let mut import_keys: Vec<ImportKey> = Vec::new();
        let mut mutation = ctx.root().begin();
        for issue in state {
            match issue {
                Issue::AddLeadingNewline { slot_index } => {
                    let item = root
                        .items()
                        .into_iter()
                        .nth(*slot_index as usize)?
                        .into_syntax();
                    if leading_newline_count(&item) >= 1 {
                        let newline = item.first_leading_trivia()?.pieces().take(1);
                        let new_item = item.clone().prepend_trivia_pieces(newline)?;
                        mutation.replace_element_discard_trivia(item.into(), new_item.into());
                    }
                }
                Issue::UnorganizedItem {
                    slot_index,
                    starts_chunk,
                } => {
                    let item = root.items().into_iter().nth(*slot_index as usize)?;
                    let prev_sibling = item.syntax().prev_sibling();
                    let item: AnyJsModuleItem = match item {
                        AnyJsModuleItem::AnyJsStatement(_) => {
                            continue;
                        }
                        AnyJsModuleItem::JsExport(export) => {
                            let mut clause = export.export_clause().ok()?;
                            // Sort named specifiers
                            if let AnyJsExportClause::JsExportNamedFromClause(casted) = &clause {
                                if let Some(sorted_specifiers) =
                                    sort_export_specifiers(casted.specifiers())
                                {
                                    clause =
                                        casted.clone().with_specifiers(sorted_specifiers).into();
                                }
                            }
                            // Sort import attributes
                            let sorted_attrs = clause.attribute().and_then(sort_attributes);
                            let clause = clause.with_attribute(sorted_attrs);
                            export.with_export_clause(clause).into()
                        }
                        AnyJsModuleItem::JsImport(import) => {
                            let clause = import.import_clause().ok()?;
                            // Sort named specifiers
                            let clause = if let Some(sorted_specifiers) =
                                clause.named_specifiers().and_then(sort_import_specifiers)
                            {
                                clause.with_named_specifiers(sorted_specifiers)
                            } else {
                                clause
                            };
                            // Sort import attributes
                            let sorted_attrs = clause.attribute().and_then(sort_attributes);
                            let clause = clause.with_attribute(sorted_attrs);
                            import.with_import_clause(clause).into()
                        }
                    };
                    let mut item = item.into_syntax();
                    // Fix newlines
                    let leading_newlines = leading_newline_count(&item);
                    if *starts_chunk
                        && leading_newlines == 1
                        && prev_sibling.is_some_and(|prev| item.kind() != prev.kind())
                    {
                        let newline = item.first_leading_trivia()?.pieces().take(1);
                        item = item.prepend_trivia_pieces(newline)?
                    } else if !starts_chunk && leading_newlines > 1 && options.groups.is_some() {
                        // Remove extra newlines
                        let leading_trivia = item.first_leading_trivia()?;
                        item = item.with_leading_trivia_pieces(
                            leading_trivia.pieces().skip(leading_newlines - 1),
                        )?;
                    }
                    // Save the node
                    organized_items.insert(*slot_index, item);
                }
                Issue::UnsortedChunkPrefix { slot_indexes } => {
                    debug_assert!(import_keys.is_empty(), "import_keys was previously drained");
                    // Collect all import keys.
                    import_keys.reserve(slot_indexes.len());
                    import_keys.extend(
                        ctx.query()
                            .items()
                            .into_iter()
                            .skip(slot_indexes.start as usize)
                            .take(slot_indexes.len())
                            .filter_map(|item| ImportInfo::from_module_item(&item))
                            .map(|(info, _, _)| ImportKey::new(info, options.groups.as_ref())),
                    );
                    // Sort imports based on their import key
                    import_keys.sort_unstable();
                    // Swap the items to obtain a sorted chunk
                    let items = ctx.query().items().into_syntax();
                    for (index, key) in (slot_indexes.start..).zip(import_keys.drain(..)) {
                        // Don't make any change if it is the same node
                        if index == key.slot_index {
                            continue;
                        }
                        let old_item = items.element_in_slot(index)?.into_node()?;
                        let new_item = items.element_in_slot(key.slot_index)?.into_node()?;
                        let new_item = organized_items.remove(&key.slot_index).unwrap_or(new_item);
                        let new_item = if index == slot_indexes.start {
                            if let Some(detached) = detached_trivia(&old_item) {
                                if leading_newline_count(&old_item) == 1 {
                                    let newline = old_item.first_leading_trivia()?.pieces().take(1);
                                    new_item.prepend_trivia_pieces(chain_trivia_pieces(
                                        newline, detached,
                                    ))?
                                } else {
                                    new_item.prepend_trivia_pieces(detached)?
                                }
                            } else if index == 0 && leading_newline_count(&old_item) == 0 {
                                // We are at the top of the file.
                                // Keep header (possibly Copyright notice)
                                let header_trivia = old_item.first_leading_trivia()?;
                                if header_trivia.is_empty() {
                                    new_item.trim_leading_trivia()?
                                } else {
                                    new_item.prepend_trivia_pieces(header_trivia.pieces())?
                                }
                            } else {
                                new_item
                            }
                        } else if let Some(attached) = attached_trivia(&new_item) {
                            // Transfer attached comment
                            new_item.with_leading_trivia_pieces(attached)?
                        } else if key.slot_index == 0 && leading_newline_count(&new_item) == 0 {
                            // Don't copy the header trivia
                            let first_token = new_item.first_token()?;
                            let new_first_token = first_token
                                .clone()
                                .with_leading_trivia([(TriviaPieceKind::Newline, "\n")]);
                            new_item.replace_child(first_token.into(), new_first_token.into())?
                        } else {
                            new_item
                        };
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

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, Deserializable, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct Options {
    groups: Option<import_groups::ImportGroups>,
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
        // `true` if this import or export starts a chunk
        starts_chunk: bool,
    },
}
