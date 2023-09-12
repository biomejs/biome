#![cfg_attr(not(feature = "biome_all"), no_main)]

#[path = "biome_common.rs"]
mod biome_common;

use libfuzzer_sys::Corpus;

pub fn do_fuzz(case: &[u8]) -> Corpus {
    biome_common::fuzz_json_formatter(case)
}

#[cfg(not(feature = "biome_all"))]
libfuzzer_sys::fuzz_target!(|case: &[u8]| -> Corpus { do_fuzz(case) });
