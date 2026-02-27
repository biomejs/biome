use crate::css_module_info::{CssImport, CssImports, CssModuleInfo};
use crate::module_graph::ModuleGraphFsProxy;
use biome_css_syntax::{AnyCssImportUrl, AnyCssRoot};
use biome_resolver::{ResolveOptions, ResolvedPath, resolve};
use biome_rowan::{AstNode, Text, WalkEvent};
use camino::Utf8Path;
use std::ops::DerefMut;

pub const SUPPORTED_EXTENSIONS: &[&str] = &["css"];

pub(crate) struct CssModuleVisitor<'a> {
    root: AnyCssRoot,
    directory: &'a Utf8Path,
    fs_proxy: &'a ModuleGraphFsProxy<'a>,
}

impl<'a> CssModuleVisitor<'a> {
    pub(crate) fn new(
        root: AnyCssRoot,
        directory: &'a Utf8Path,
        fs_proxy: &'a ModuleGraphFsProxy,
    ) -> Self {
        Self {
            root,
            directory,
            fs_proxy,
        }
    }

    pub(crate) fn visit(self) -> CssModuleInfo {
        let mut imports = CssImports::default();
        let iter = self.root.syntax().preorder();
        for event in iter {
            match event {
                WalkEvent::Enter(node) => {
                    if let Some(node) = AnyCssImportUrl::cast(node) {
                        self.visit_any_css_import_url(node, &mut imports);
                    }
                }
                WalkEvent::Leave(_) => {}
            }
        }

        CssModuleInfo::new(imports)
    }

    fn visit_any_css_import_url(&self, node: AnyCssImportUrl, imports: &mut CssImports) {
        let Some(specifier) = node.inner_string_text() else {
            return;
        };

        let resolved_path = self.resolved_path_from_specifier(&specifier);

        let text: Text = specifier.into();
        imports.deref_mut().insert(
            text.clone(),
            CssImport {
                specifier: text,
                resolved_path,
            },
        );
    }

    fn resolved_path_from_specifier(&self, specifier: &str) -> ResolvedPath {
        let options = ResolveOptions {
            assume_relative: true,
            condition_names: &[],
            default_files: &[],
            extensions: SUPPORTED_EXTENSIONS,
            extension_aliases: &[],
            ..Default::default()
        };
        let resolved_path = resolve(specifier, self.directory, self.fs_proxy, &options);
        ResolvedPath::new(resolved_path)
    }
}
