use biome_js_factory::syntax::JsFileSource;
use biome_js_factory::{
    make,
    syntax::{AnyJsDeclaration, AnyJsModuleItem, AnyJsStatement},
};
use biome_js_formatter::{context::JsFormatOptions, format_node};
use biome_rowan::AstNode;
use biome_service::workspace_types::{generate_type, methods, ModuleQueue};
use quote::{format_ident, quote};
use schemars::gen::{SchemaGenerator, SchemaSettings};
use std::{env, fs, io, path::PathBuf};

fn main() -> io::Result<()> {
    let methods = methods();

    let mut items = Vec::new();
    let mut queue = ModuleQueue::default();

    // FIXME: a lot of this code is duplicated in xtask/codegen/src/generate_bindings.rs
    for method in &methods {
        generate_type(&mut items, &mut queue, &method.params);
        generate_type(&mut items, &mut queue, &method.result);
    }
    // HACK: SupportKind doesn't get picked up in the loop above, so we add it manually
    let support_kind_schema = SchemaGenerator::from(SchemaSettings::openapi3())
        .root_schema_for::<biome_service::workspace::SupportKind>();
    generate_type(&mut items, &mut queue, &support_kind_schema);

    let module = make::js_module(
        make::js_directive_list(None),
        make::js_module_item_list(items.into_iter().map(|(decl, _)| {
            AnyJsModuleItem::AnyJsStatement(match decl {
                AnyJsDeclaration::JsClassDeclaration(decl) => {
                    AnyJsStatement::JsClassDeclaration(decl)
                }
                AnyJsDeclaration::JsFunctionDeclaration(decl) => {
                    AnyJsStatement::JsFunctionDeclaration(decl)
                }
                AnyJsDeclaration::JsVariableDeclaration(decl) => {
                    AnyJsStatement::JsVariableStatement(make::js_variable_statement(decl).build())
                }
                AnyJsDeclaration::TsDeclareFunctionDeclaration(decl) => {
                    AnyJsStatement::TsDeclareFunctionDeclaration(decl)
                }
                AnyJsDeclaration::TsEnumDeclaration(decl) => {
                    AnyJsStatement::TsEnumDeclaration(decl)
                }
                AnyJsDeclaration::TsExternalModuleDeclaration(decl) => {
                    AnyJsStatement::TsExternalModuleDeclaration(decl)
                }
                AnyJsDeclaration::TsGlobalDeclaration(decl) => {
                    AnyJsStatement::TsGlobalDeclaration(decl)
                }
                AnyJsDeclaration::TsImportEqualsDeclaration(decl) => {
                    AnyJsStatement::TsImportEqualsDeclaration(decl)
                }
                AnyJsDeclaration::TsInterfaceDeclaration(decl) => {
                    AnyJsStatement::TsInterfaceDeclaration(decl)
                }
                AnyJsDeclaration::TsModuleDeclaration(decl) => {
                    AnyJsStatement::TsModuleDeclaration(decl)
                }
                AnyJsDeclaration::TsTypeAliasDeclaration(decl) => {
                    AnyJsStatement::TsTypeAliasDeclaration(decl)
                }
            })
        })),
        make::eof(),
    )
    .build();

    // Wasm-bindgen will paste the generated TS code as-is into the final .d.ts file,
    // ensure it looks good by running it through the formatter
    let formatted = format_node(JsFormatOptions::new(JsFileSource::ts()), module.syntax()).unwrap();
    let printed = formatted.print().unwrap();
    let definitions = printed.into_code();

    // Generate wasm-bindgen extern type imports for all the types defined in the TS code
    let types = queue.visited().iter().map(|name| {
        let ident = format_ident!("I{name}");
        if name.contains('_') {
            quote! {
                #[wasm_bindgen(typescript_type = #name)]
                #[expect(non_camel_case_types)]
                pub type #ident;
            }
        } else {
            quote! {
                #[wasm_bindgen(typescript_type = #name)]
                pub type #ident;
            }
        }
    });

    let tokens = quote! {
        #[wasm_bindgen(typescript_custom_section)]
        const TS_TYPEDEFS: &'static str = #definitions;

        #[wasm_bindgen]
        extern "C" {
            #( #types )*
        }
    };

    let out_dir = env::var("OUT_DIR").unwrap();
    fs::write(
        PathBuf::from(out_dir).join("ts_types.rs"),
        tokens.to_string(),
    )?;

    Ok(())
}
