mod binding;
mod builder;
mod closure;
mod globals;
mod import;
mod is_constant;
mod model;
mod reference;
mod scope;

#[cfg(test)]
mod tests;

use crate::{SemanticEvent, SemanticEventExtractor, SemanticEventExtractorContext};
use biome_js_syntax::{
    AnyJsExpression, AnyJsRoot, JsIdentifierAssignment, JsIdentifierBinding, JsLanguage,
    JsReferenceIdentifier, JsSyntaxKind, JsSyntaxNode, JsxReferenceIdentifier, TextRange, TextSize,
    TsIdentifierBinding,
};
use biome_rowan::AstNode;
pub use closure::*;
use rust_lapper::{Interval, Lapper};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    collections::{BTreeSet, VecDeque},
    iter::FusedIterator,
    rc::Rc,
};

pub use binding::*;
pub use builder::*;

pub use globals::*;
pub use import::*;
pub use is_constant::*;
pub use model::*;
pub use reference::*;
pub use scope::*;

/// Extra options for the [SemanticModel] creation.
#[derive(Default)]
pub struct SemanticModelOptions {
    /// All the allowed globals names
    pub globals: FxHashSet<String>,
    /// The JSX factory name
    pub jsx_factory: Option<String>,
    /// The JSX fragment factory name
    pub jsx_fragment_factory: Option<String>,
}

/// Build the complete [SemanticModel] of a parsed file.
/// For a push based model to build the [SemanticModel], see [SemanticModelBuilder].
pub fn semantic_model(root: &AnyJsRoot, options: SemanticModelOptions) -> SemanticModel {
    let mut extractor = SemanticEventExtractor::default();
    let mut builder = SemanticModelBuilder::new(root.clone());

    let SemanticModelOptions {
        globals,
        jsx_factory,
        jsx_fragment_factory,
    } = options;

    for global in globals {
        builder.push_global(global);
    }

    let ctx = SemanticEventExtractorContext {
        jsx_factory: jsx_factory.as_deref(),
        jsx_fragment_factory: jsx_fragment_factory.as_deref(),
    };

    let root = root.syntax();
    for node in root.preorder() {
        match node {
            biome_js_syntax::WalkEvent::Enter(node) => {
                builder.push_node(&node);
                extractor.enter(&node, &ctx);
            }
            biome_js_syntax::WalkEvent::Leave(node) => extractor.leave(&node),
        }
    }

    while let Some(e) = extractor.pop() {
        builder.push_event(e);
    }

    builder.build()
}
