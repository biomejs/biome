#[macro_export]
macro_rules! declare_migration {
    (  $vis:vis $id:ident {
        version: $version:literal,
        name: $name:tt,
        $( $key:ident: $value:expr, )*
    } ) => {
        $vis enum $id {}

        impl biome_analyze::RuleMeta for $id {
            type Group = $crate::analyzers::MigrationGroup;
            const METADATA: biome_analyze::RuleMetadata =
                biome_analyze::RuleMetadata::new($version, $name, "", "") $( .$key($value) )*;
        }

        // Declare a new `rule_category!` macro in the module context that
        // expands to the category of this rule
        // This is implemented by calling the `group_category!` macro from the
        // parent module (that should be declared by a call to `declare_group!`)
        // and providing it with the name of this rule as a string literal token
        #[expect(unused_macros)]
        macro_rules! rule_category {
            () => { super::group_category!( $name ) };
        }
    };
}
