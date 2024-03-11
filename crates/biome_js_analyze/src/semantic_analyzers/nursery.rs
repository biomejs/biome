//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_console;
pub mod no_re_export_all;
pub mod no_semicolon_in_jsx;
pub mod use_jsx_key_in_iterable;
pub mod use_sorted_classes;

declare_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: no_console :: NoConsole ,
            self :: no_re_export_all :: NoReExportAll ,
            self :: no_semicolon_in_jsx :: NoSemicolonInJsx ,
            self :: use_jsx_key_in_iterable :: UseJsxKeyInIterable ,
            self :: use_sorted_classes :: UseSortedClasses ,
        ]
     }
}
