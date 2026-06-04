use std::path::Path;

/// Relative path of the generated global types module from the workspace root.
const OUTPUT_RELATIVE_PATH: &str = "crates/biome_js_type_info/src/generated/global_types.rs";

/// Emits the global types module with LF-normalized output.
pub(super) fn emit_global_types(
    pin: &crate::generate_global_types::SourcePin,
    mode: xtask_glue::Mode,
    workspace_root: &Path,
) -> anyhow::Result<crate::UpdateResult> {
    let path = workspace_root.join(OUTPUT_RELATIVE_PATH);
    let formatted =
        xtask_glue::reformat_with_command(generated_body(pin), "just gen-global-types")?;
    crate::update(&path, &formatted, &mode)
}

/// Renders the unformatted Rust body for the generated module.
fn generated_body(pin: &crate::generate_global_types::SourcePin) -> String {
    format!(
        r#"// Generated from microsoft/TypeScript {typescript_tag} (git commit {typescript_sha}).

/// Predefined global IDs whose `TypeData` is supplied by this generated module.
pub(crate) const MIGRATED_PREDEFINED_IDS: &[crate::globals::GlobalTypeId] = &[];

/// Registers all generated global type data into the resolver builder.
pub(crate) fn set_generated_global_type_data(builder: &mut crate::globals_builder::GlobalsResolverBuilder) {{
    let _ = builder;
}}
"#,
        typescript_tag = pin.tag(),
        typescript_sha = pin.sha(),
    )
}
