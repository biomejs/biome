#![no_main]

mod biome_format_d_ts;
mod biome_format_jsx;
mod biome_format_module;
mod biome_format_script;
mod biome_format_tsx;
mod biome_format_typescript;

use libfuzzer_sys::{fuzz_target, Corpus};

fn do_fuzz(data: &[u8]) -> Corpus {
    let mut keep = Corpus::Reject;
    if let Corpus::Keep = biome_format_d_ts::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    if let Corpus::Keep = biome_format_jsx::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    if let Corpus::Keep = biome_format_module::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    if let Corpus::Keep = biome_format_script::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    if let Corpus::Keep = biome_format_tsx::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    if let Corpus::Keep = biome_format_typescript::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    keep
}

fuzz_target!(|case: &[u8]| -> Corpus { do_fuzz(case) });
