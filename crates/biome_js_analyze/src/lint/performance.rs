//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_accumulating_spread;
pub mod no_await_in_loops;
pub mod no_barrel_file;
pub mod no_delete;
pub mod no_dynamic_namespace_import_access;
pub mod no_img_element;
pub mod no_namespace_import;
pub mod no_re_export_all;
pub mod no_unwanted_polyfillio;
pub mod use_google_font_preconnect;
pub mod use_solid_for_component;
pub mod use_top_level_regex;
declare_lint_group! { pub Performance { name : "performance" , rules : [self :: no_accumulating_spread :: NoAccumulatingSpread , self :: no_await_in_loops :: NoAwaitInLoops , self :: no_barrel_file :: NoBarrelFile , self :: no_delete :: NoDelete , self :: no_dynamic_namespace_import_access :: NoDynamicNamespaceImportAccess , self :: no_img_element :: NoImgElement , self :: no_namespace_import :: NoNamespaceImport , self :: no_re_export_all :: NoReExportAll , self :: no_unwanted_polyfillio :: NoUnwantedPolyfillio , self :: use_google_font_preconnect :: UseGoogleFontPreconnect , self :: use_solid_for_component :: UseSolidForComponent , self :: use_top_level_regex :: UseTopLevelRegex ,] } }
