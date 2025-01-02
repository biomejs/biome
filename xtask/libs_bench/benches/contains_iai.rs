mod contains;
use contains::*;

// iai do not support setup, so we basically run the setup and
// the whole setup + test. To see the difference.
// https://github.com/bheisler/iai/pull/24

iai::main!(
    contains_hashset_setup,
    contains_btreeset_setup,
    contains_bloom_setup,
    contains_trie_setup,
    contains_slice_setup,
    contains_fst_setup,
    contains_binary_search_setup,
    contains_memchr_setup,
);
