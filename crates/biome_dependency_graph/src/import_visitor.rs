use biome_js_syntax::{AnyJsImportLike, AnyJsRoot};
use biome_rowan::{AstNode, WalkEvent};
use camino::{Utf8Path, Utf8PathBuf};
use oxc_resolver::ResolverGeneric;

use crate::{
    dependency_graph::{Import, ModuleImports},
    resolver_cache::ResolverCache,
};

pub(crate) struct ImportVisitor<'a> {
    root: AnyJsRoot,
    directory: &'a Utf8Path,
    resolver: &'a ResolverGeneric<ResolverCache<'a>>,
    module_imports: ModuleImports,
}

impl<'a> ImportVisitor<'a> {
    pub fn new(
        root: AnyJsRoot,
        directory: &'a Utf8Path,
        resolver: &'a ResolverGeneric<ResolverCache<'a>>,
    ) -> Self {
        Self {
            root,
            directory,
            resolver,
            module_imports: Default::default(),
        }
    }

    pub fn find_module_imports(mut self) -> ModuleImports {
        let iter = self.root.syntax().preorder();
        for event in iter {
            match event {
                WalkEvent::Enter(node) => {
                    if let Some(import) = AnyJsImportLike::cast_ref(&node) {
                        self.visit_import(import);
                    }
                }
                WalkEvent::Leave(_) => {}
            }
        }

        self.module_imports
    }

    fn visit_import(&mut self, node: AnyJsImportLike) {
        let Some(specifier) = node.inner_string_text() else {
            return;
        };

        let import = match self.resolver.resolve(self.directory, specifier.text()) {
            Ok(resolution) => Import {
                resolved_path: Utf8PathBuf::from_path_buf(resolution.into_path_buf()).ok(),
            },
            Err(_error) => Import {
                resolved_path: None,
            },
        };

        match node {
            AnyJsImportLike::JsModuleSource(_) => {
                self.module_imports
                    .static_imports
                    .insert(specifier.to_string(), import);
            }
            AnyJsImportLike::JsCallExpression(_) | AnyJsImportLike::JsImportCallExpression(_) => {
                self.module_imports
                    .dynamic_imports
                    .insert(specifier.to_string(), import);
            }
        }
    }
}
