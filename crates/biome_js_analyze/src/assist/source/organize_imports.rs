use biome_analyze::{
    context::RuleContext, declare_source_rule, ActionCategory, Ast, FixKind, Rule, SourceActionKind,
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
    /// Provides a whole-source code action to sort the imports in the file
    /// using import groups and natural ordering.
    ///
    /// ## Examples
    ///
    /// ```js
    /// import React, {
    ///     FC,
    ///     useEffect,
    ///     useRef,
    ///     ChangeEvent,
    ///     KeyboardEvent,
    /// } from 'react';
    /// import { logger } from '@core/logger';
    /// import { reduce, debounce } from 'lodash';
    /// import { Message } from '../Message';
    /// import { createServer } from '@server/node';
    /// import { Alert } from '@ui/Alert';
    /// import { repeat, filter, add } from '../utils';
    /// import { initializeApp } from '@core/app';
    /// import { Popup } from '@ui/Popup';
    /// import { createConnection } from '@server/database';
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
