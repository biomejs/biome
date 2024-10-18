use biome_analyze::{
    context::RuleContext, declare_source_rule, ActionCategory, Ast, FixKind, Rule, SourceActionKind,
};
use biome_console::markup;
use biome_js_syntax::JsModule;
use biome_rowan::BatchMutationExt;

use crate::JsRuleAction;

pub mod util;
pub mod legacy;

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
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for OrganizeImports {
    type Query = Ast<JsModule>;
    type State = legacy::ImportGroups;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let root = ctx.query();
        legacy::run(root)
    }

    fn action(ctx: &RuleContext<Self>, groups: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        legacy::action(ctx.query(), groups, &mut mutation)?;
        Some(JsRuleAction::new(
            ActionCategory::Source(SourceActionKind::OrganizeImports),
            ctx.metadata().applicability(),
            markup! { "Organize Imports (Biome)" },
            mutation,
        ))
    }
}
