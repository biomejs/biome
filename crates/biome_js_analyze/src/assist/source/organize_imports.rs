use biome_analyze::{
    ActionCategory, Ast, FixKind, Rule, SourceActionKind, context::RuleContext, declare_source_rule,
};
use biome_console::markup;
use biome_deserialize::{Deserializable, DeserializableValue, DeserializationContext};
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::JsModule;
use biome_rowan::BatchMutationExt;

use crate::JsRuleAction;

pub mod legacy;
pub mod util;

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
    /// ```ts title="example.ts"
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
    /// ```ts title="example.ts"
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
    /// ```js name="file.js"
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
    /// ```js name="file.js"
    /// import { internal } from "#internal";  // Import group 1
    /// import sibling from "./sibling";      // Import group 1
    /// import "z";  // Import group 2
    /// import "a";  // Import group 3
    /// import assert from "node:assert";  // Import group 4
    /// import React from "react";         // Import group 4
    /// ```
    ///
    /// ## Import sorting via VSCode extension
    ///
    /// Any LSP-compatible editor supports imports sorting through the "Organize Imports" code action.
    /// By default, this action can be run using the <kbd title="Shift">⇧</kbd>+<kbd>Alt</kbd>+<kbd>O</kbd> keyboard shortcut, or is accessible through the _Command Palette_ (<kbd>Ctrl</kbd>/<kbd title="Cmd">⌘</kbd>+<kbd title="Shift">⇧</kbd>+<kbd>P</kbd>) by selecting _Organize Imports_.
    ///
    /// You can add the following to your VSCode extension configuration if you want the action to run automatically on save instead of calling it manually:
    ///
    /// ```json title="settings.json"
    /// {
    /// 	"editor.codeActionsOnSave":{
    /// 		"source.organizeImports.biome": "explicit"
    /// 	}
    /// }
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
    type State = State;
    type Signals = Option<Self::State>;
    type Options = Options;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let root = ctx.query();
        legacy::run(root).map(State::Legacy)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        match state {
            State::Legacy(groups) => {
                legacy::action(ctx.query(), groups, &mut mutation)?;
            }
            State::Modern => {}
        }
        Some(JsRuleAction::new(
            ActionCategory::Source(SourceActionKind::OrganizeImports),
            ctx.metadata().applicability(),
            markup! { "Organize Imports (Biome)" },
            mutation,
        ))
    }
}

#[derive(Debug)]
pub enum State {
    Legacy(legacy::ImportGroups),
    Modern,
}

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, Deserializable, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct Options {
    legacy: bool,
    import_groups: Box<[ImportGroup]>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum ImportGroup {
    Predefined(PredefinedImportGroup),
    Custom(biome_glob::Glob),
}
impl Deserializable for ImportGroup {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        Some(
            if let Some(predefined) = Deserializable::deserialize(ctx, value, name) {
                ImportGroup::Predefined(predefined)
            } else {
                ImportGroup::Custom(Deserializable::deserialize(ctx, value, name)?)
            },
        )
    }
}

#[derive(Clone, Debug, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum PredefinedImportGroup {
    #[serde(rename = ":blank-line:")]
    BlankLine,
    #[serde(rename = ":bun:")]
    Bun,
    #[serde(rename = ":node:")]
    Node,
    #[serde(rename = ":types:")]
    Types,
}
