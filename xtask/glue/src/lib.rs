//! Codegen tools mostly used to generate ast and syntax definitions. Adapted from rust analyzer's codegen

pub mod glue;

use std::{
    env,
    fmt::Display,
    path::{Path, PathBuf},
    sync::OnceLock,
};

pub use crate::glue::{pushd, pushenv};

pub use anyhow::{Context as _, Error, Result, anyhow, bail, ensure};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    Overwrite,
    Verify,
}

pub fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(2)
    .unwrap()
    .to_path_buf()
}

pub fn run_rustfmt(mode: Mode) -> Result<()> {
    let _dir = pushd(project_root());
    let _e = pushenv("RUSTUP_TOOLCHAIN", "stable");
    ensure_rustfmt()?;
    match mode {
        Mode::Overwrite => run!("cargo fmt"),
        Mode::Verify => run!("cargo fmt -- --check"),
    }?;
    Ok(())
}

pub fn reformat(text: impl Display) -> Result<String> {
    reformat_without_preamble(text).map(prepend_generated_preamble)
}

pub fn reformat_with_command(text: impl Display, command: impl Display) -> Result<String> {
    reformat_without_preamble(text).map(|formatted| {
        format!("//! This is a generated file. Don't modify it by hand! Run '{command}' to re-generate the file.\n\n{formatted}")
    })
}

pub const PREAMBLE: &str = "Generated file, do not edit by hand, see `xtask/codegen`";
pub fn prepend_generated_preamble(content: impl Display) -> String {
    format!("//! {PREAMBLE}\n\n{content}")
}

pub fn reformat_without_preamble(text: impl Display) -> Result<String> {
    let _e = pushenv("RUSTUP_TOOLCHAIN", "stable");
    ensure_rustfmt()?;
    let output = run!(
        "rustfmt";
        <text.to_string().as_bytes()
    )?;

    Ok(format!("{output}\n"))
}

static IS_RUSTFMT_CHECKED: OnceLock<()> = OnceLock::new();

pub fn ensure_rustfmt() -> Result<()> {
    if IS_RUSTFMT_CHECKED.get().is_some() {
        return Ok(());
    }

    let out = run!("rustfmt --version")?;
    if !out.contains("stable") {
        bail!(
            "Failed to run rustfmt from toolchain 'stable'. \
             Please run `rustup component add rustfmt --toolchain stable` to install it.",
        )
    }

    // e.g. "rustfmt 1.8.0-stable (4d91de4e48 2025-02-17)"
    let (_, version) = out.split_once(' ').unwrap_or_default();
    let mut version = version
        .split('.')
        .filter_map(|s| str::parse::<usize>(s).ok());
    let major = version.next();
    let minor = version.next();
    if major != Some(1) || minor.is_none_or(|minor| minor < 8) {
        // `--style_edition=2024` requires 1.8.0 or later.
        bail!(
            "The installed rustfmt is outdated, 1.8.0 or later is required. \
             Please run `rustup update stable` to update it.",
        )
    }

    IS_RUSTFMT_CHECKED.get_or_init(|| ());

    Ok(())
}
