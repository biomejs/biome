#![cfg_attr(not(feature = "biome_all"), no_main)]

#[path = "biome_common.rs"]
mod biome_common;

use biome_js_syntax::JsFileSource;
use libfuzzer_sys::Corpus;

pub fn do_fuzz(case: &[u8]) -> Corpus {
    let parse_type = JsFileSource::ts();
    biome_common::fuzz_js_formatter_with_source_type(case, parse_type)
}

#[cfg(not(feature = "biome_all"))]
libfuzzer_sys::fuzz_target!(|case: &[u8]| -> Corpus { do_fuzz(case) });
