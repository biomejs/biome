use fastbloom_rs::{BloomFilter, FilterBuilder, Membership};
use qp_trie::Trie;
use std::collections::{BTreeSet, HashSet};

pub fn keywords() -> Vec<String> {
    let repeat = std::env::var("ROME_BENCH_CONTAINS_REPEAT")
        .unwrap_or_else(|_| "1".to_string())
        .parse()
        .unwrap();
    let v = &["undefined", "NaN", "Infinity", "arguments", "eval"].repeat(repeat);
    v.iter()
        .enumerate()
        .map(|(i, x)| format!("{x}{i}"))
        .collect()
}

pub fn contains_slice_setup() -> Vec<String> {
    keywords()
}

pub fn contains_binary_search_setup() -> Vec<String> {
    let mut words = keywords();
    words.sort();
    words
}

pub fn contains_hashset_setup() -> HashSet<String> {
    let mut set = HashSet::new();
    for k in keywords() {
        set.insert(k.to_string());
    }
    set
}

pub fn contains_btreeset_setup() -> BTreeSet<String> {
    let mut set = BTreeSet::new();
    for k in keywords() {
        set.insert(k.to_string());
    }
    set
}

pub fn contains_bloom_setup() -> BloomFilter {
    let builder = FilterBuilder::new(100_000_000, 0.01);
    let mut set = BloomFilter::new(builder);

    for k in keywords() {
        set.add(k.as_bytes());
    }

    set
}

pub fn contains_trie_setup() -> Trie<Vec<u8>, i32> {
    let mut set = Trie::new();

    for k in keywords() {
        set.insert(k.into_bytes(), 0);
    }

    set
}

pub fn contains_fst_setup() -> fst::Set<Vec<u8>> {
    let w = vec![];
    let mut set = fst::SetBuilder::new(w).unwrap();

    let mut keywords = keywords().clone();
    keywords.sort();

    for k in keywords {
        let _ = set.insert(k);
    }
    set.into_set()
}

pub fn contains_memchr_setup() -> Vec<String> {
    contains_binary_search_setup()
}
