#![deny(clippy::use_self)]

use case::CaseExt;
use globwalk::{GlobWalker, GlobWalkerBuilder};
use proc_macro::TokenStream;
use proc_macro_error2::*;
use proc_macro2::Span;
use quote::*;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use syn::parse::ParseStream;

struct Arguments {
    pattern: syn::ExprLit,
    called_function: syn::Path,
    file_type: syn::ExprLit,
}

struct Variables {
    test_name: String,
    test_full_path: String,
    test_expected_fullpath: String,
    test_directory: String,
}

struct AllFiles(GlobWalker);

impl Iterator for AllFiles {
    type Item = Result<PathBuf, &'static str>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(Ok(entry)) => {
                    let file_name = match entry.file_name().to_str().ok_or("File name not UTF8") {
                        Ok(v) => v,
                        Err(e) => return Some(Err(e)),
                    };

                    if file_name.contains("expected") {
                        continue;
                    }
                    let meta = match entry.metadata().map_err(|_| "Cannot open file") {
                        Ok(v) => v,
                        Err(e) => return Some(Err(e)),
                    };
                    if meta.is_file() {
                        let path = entry.path().to_path_buf();
                        break Some(Ok(path));
                    }
                }
                _ => break None,
            }
        }
    }
}

fn transform_file_name(input: &str) -> String {
    let mut result = biome_string_case::Case::Snake.convert(input);

    let is_keyword = matches!(
        result.as_str(),
        "await"
            | "break"
            | "try"
            | "do"
            | "for"
            | "return"
            | "if"
            | "while"
            | "in"
            | "async"
            | "else"
            | "as"
            | "abstract"
            | "enum"
            | "static"
            | "yield"
            | "type"
            | "super"
            | "typeof"
            | "const"
    );

    if is_keyword {
        result.push('_');
    } else if matches!(result.bytes().next(), Some(b'0'..=b'9')) {
        result.insert(0, '_');
    }

    result
}

#[derive(Default)]
struct Modules {
    modules: HashMap<String, Modules>,
    tests: Vec<proc_macro2::TokenStream>,
}

impl Modules {
    fn insert<'a>(
        &mut self,
        mut path: impl Iterator<Item = &'a str>,
        test: proc_macro2::TokenStream,
    ) {
        match path.next() {
            Some(module) => {
                let name = transform_file_name(module);
                self.modules.entry(name).or_default().insert(path, test);
            }
            None => {
                self.tests.push(test);
            }
        }
    }

    fn print(self, output: &mut proc_macro2::TokenStream) {
        for (name, module) in self.modules {
            let name = syn::Ident::new(&name, Span::call_site());

            let mut stream = proc_macro2::TokenStream::new();
            module.print(&mut stream);
            output.extend(quote! {
                mod #name { #stream }
            });
        }

        output.extend(self.tests);
    }
}

impl Arguments {
    fn get_all_files(&self) -> Result<AllFiles, &str> {
        let base = std::env::var("CARGO_MANIFEST_DIR")
            .map_err(|_| "Cannot find CARGO_MANIFEST_DIR. Are you using cargo?")?;
        let glob = match &self.pattern.lit {
            syn::Lit::Str(v) => v.value(),
            _ => return Err("Only string literals supported"),
        };
        let walker = GlobWalkerBuilder::new(base, glob)
            .build()
            .map_err(|_| "Cannot walk the requested glob")?;

        Ok(AllFiles(walker))
    }

    fn get_variables<P: AsRef<Path>>(path: P) -> Option<Variables> {
        let path = path.as_ref();
        let file_stem = path.file_stem()?;
        let file_stem = file_stem.to_str()?;
        let test_name = format!(
            "{}{}",
            file_stem.to_snake(),
            if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                format!("_{extension}")
            } else {
                String::new()
            }
        );
        let test_directory = path.parent().unwrap().display().to_string();

        let test_full_path = path.display().to_string();
        let extension = match path.extension() {
            Some(ext) => format!(".{}", ext.to_str().unwrap_or("")),
            None => String::new(),
        };

        let mut test_expected_file = path.to_path_buf();
        test_expected_file.pop();
        test_expected_file.push(format!("{file_stem}.expected{extension}"));
        let test_expected_fullpath = test_expected_file.display().to_string();

        Some(Variables {
            test_name,
            test_full_path,
            test_expected_fullpath,
            test_directory,
        })
    }

    pub fn generate(&self) -> Result<TokenStream, &str> {
        let files = self.get_all_files()?;
        let mut modules = Modules::default();

        // Compute static (non-glob) prefix components from the provided glob pattern.
        let pattern_value = match &self.pattern.lit {
            syn::Lit::Str(v) => v.value(),
            _ => String::new(),
        };
        let static_components: Vec<String> = pattern_value
            .split('/')
            .take_while(|comp| {
                // Stop at the first component that contains any glob metacharacters
                !comp.contains('*')
                    && !comp.contains('?')
                    && !comp.contains('[')
                    && !comp.contains('{')
            })
            .map(|s| s.to_string())
            .collect();

        for file in files.flatten() {
            let Variables {
                test_name,
                test_full_path,
                test_expected_fullpath,
                test_directory,
            } = Self::get_variables(file).ok_or("Cannot generate variables for this file")?;

            let test_name = transform_file_name(&test_name);

            // Build module path components based on directories relative to the static (non-glob) prefix.
            let parent_components: Vec<String> = Path::new(&test_full_path)
                .parent()
                .map(|p| {
                    p.components()
                        .filter_map(|c| c.as_os_str().to_str().map(|s| s.to_string()))
                        .collect::<Vec<String>>()
                })
                .unwrap_or_default();

            // Find where the static prefix appears in the file path; include the last static component (e.g. "ok")
            let mut start_idx: Option<usize> = None;
            if !static_components.is_empty() && static_components.len() < parent_components.len() {
                'outer: for i in 0..=parent_components
                    .len()
                    .saturating_sub(static_components.len())
                {
                    for (j, sc) in static_components.iter().enumerate() {
                        if parent_components[i + j] != *sc {
                            continue 'outer;
                        }
                    }
                    start_idx = Some(i + static_components.len() - 1);
                    break;
                }
            }
            // Fallback: try to align by the last component of the static prefix
            if start_idx.is_none()
                && let Some(last_sc) = static_components.last()
                && let Some(pos) = parent_components.iter().rposition(|c| c == last_sc)
            {
                start_idx = Some(pos);
            }
            // Fallback: start after a "tests" directory if present
            if start_idx.is_none()
                && let Some(pos) = parent_components.iter().position(|c| c == "tests")
            {
                start_idx = Some(pos + 1);
            }

            let path_components: Vec<String> = match start_idx {
                Some(start) if start < parent_components.len() => {
                    parent_components[start..].to_vec()
                }
                _ => Vec::new(),
            };
            let path = path_components.iter().map(|s| s.as_str());

            let span = self.pattern.lit.span();
            let test_name = syn::Ident::new(&test_name, span);
            let f = &self.called_function;
            let file_type = &self.file_type;

            modules.insert(
                path,
                quote! {
                    #[test]
                    pub fn #test_name () {
                        let test_file = #test_full_path;
                        let test_expected_file = #test_expected_fullpath;
                        let file_type = #file_type;
                        let test_directory = #test_directory;
                        #f(test_file, test_expected_file, test_directory, file_type);
                    }
                },
            );
        }

        let mut output = proc_macro2::TokenStream::new();
        modules.print(&mut output);
        Ok(output.into())
    }
}

impl syn::parse::Parse for Arguments {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let path: syn::ExprLit = input.parse()?;
        let _: syn::Token!(,) = input.parse()?;
        let call: syn::Path = input.parse()?;
        let _: syn::Token!(,) = input.parse()?;
        let file_type: syn::ExprLit = input.parse()?;
        Ok(Self {
            pattern: path,
            called_function: call,
            file_type,
        })
    }
}

#[proc_macro]
#[proc_macro_error]
pub fn gen_tests(input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(input as Arguments);

    match args.generate() {
        Ok(tokens) => tokens,
        Err(e) => abort!(e, "{}", e),
    }
}
