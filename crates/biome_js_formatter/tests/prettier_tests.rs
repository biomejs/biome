use std::{env, path::Path};

use biome_formatter::IndentStyle;
use biome_formatter_test::test_prettier_snapshot::{PrettierSnapshot, PrettierTestFile};
use biome_js_formatter::context::JsFormatOptions;
use biome_js_syntax::{JsFileSource, LanguageVariant, ModuleKind};

mod language;

tests_macros::gen_tests! {"tests/specs/prettier/{js,typescript,jsx}/**/*.{js,ts,jsx,tsx}", crate::test_snapshot, "script"}

fn test_snapshot(input: &'static str, _: &str, _: &str, _: &str) {
    countme::enable(true);

    let root_path = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/specs/prettier/"
    ));

    let test_file = PrettierTestFile::new(input, root_path);
    let mut source_type = {
        // Prettier testing suite uses JSX tags inside JS files.
        // As there's no way to know in advance which files have JSX syntax, we
        // change the source type only here
        if test_file.file_extension() == "js" {
            JsFileSource::jsx()
        } else if test_file.file_name().contains("jsx") && test_file.file_extension() == "ts" {
            JsFileSource::tsx()
        } else {
            test_file.input_file().try_into().unwrap()
        }
    };

    if is_non_strict_mode(root_path, test_file.input_file()) {
        source_type = source_type.with_module_kind(ModuleKind::Script)
    }

    if is_restricted_typescript(root_path, test_file.input_file()) {
        source_type = source_type.with_variant(LanguageVariant::StandardRestricted)
    }

    let options = JsFormatOptions::new(source_type)
        .with_indent_style(IndentStyle::Space)
        .with_indent_width(2.into());

    let language = language::JsTestFormatLanguage::new(source_type);

    let snapshot = PrettierSnapshot::new(test_file, language, options);

    snapshot.test()
}

fn is_non_strict_mode(root_path: &Path, file_path: &Path) -> bool {
    let test_cases_paths = ["js/with/", "js/sloppy-mode/", "js/identifier/"];

    test_cases_paths.iter().any(|path| {
        file_path
            .strip_prefix(root_path)
            .is_ok_and(|file| file.starts_with(path))
    })
}

fn is_restricted_typescript(root_path: &Path, file_path: &Path) -> bool {
    let test_cases_paths = [
        "typescript/arrows/type_params.ts",
        "typescript/compiler/contextualSignatureInstantiation2.ts",
        "typescript/typeparams/const.ts",
    ];

    test_cases_paths.iter().any(|path| {
        file_path
            .strip_prefix(root_path)
            .is_ok_and(|file| file.starts_with(path))
    })
}
