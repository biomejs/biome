#![cfg_attr(not(feature = "rome_all"), no_main)]

#[path = "rome_common.rs"]
mod rome_common;

use biome_js_syntax::JsFileSource;
use libfuzzer_sys::Corpus;

pub fn do_fuzz(case: &[u8]) -> Corpus {
    let parse_type = JsFileSource::js_module();
    rome_common::fuzz_js_parser_with_source_type(case, parse_type)
}

#[cfg(not(feature = "rome_all"))]
libfuzzer_sys::fuzz_target!(|case: &[u8]| -> Corpus { do_fuzz(case) });
