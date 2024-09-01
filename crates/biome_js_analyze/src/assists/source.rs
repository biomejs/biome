//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_assists_group;

pub mod organize_imports;
pub mod sort_jsx_props;

declare_assists_group! {
    pub Source {
        name : "source" ,
        rules : [
            self :: organize_imports :: OrganizeImports ,
            self :: sort_jsx_props :: SortJsxProps ,
        ]
     }
}
