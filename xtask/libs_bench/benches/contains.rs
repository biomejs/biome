#![allow(dead_code)]

use std::collections::{BTreeSet, HashSet};

use fastbloom_rs::{BloomFilter, FilterBuilder, Membership};
use qp_trie::Trie;

pub fn keywords() -> Vec<String> {
    let repeat = std::env::var("BIOME_BENCH_CONTAINS_REPEAT")
        .unwrap_or_else(|_| "1".to_string())
        .parse()
        .unwrap();
    let v = &["undefined", "NaN", "Infinity", "arguments", "eval"].repeat(repeat);
    v.iter()
        .enumerate()
        .map(|(i, x)| format!("{}{}", x, i))
        .collect()
}

pub fn search_for() -> &'static [&'static str] {
    &[
        "undefined",
        "a",
        "NaN",
        "longVariableName",
        "Infinity",
        "xxxxxxxx",
        "arguments",
        "eval",
    ][..]
}

pub fn contains_slice_setup() -> Vec<String> {
    keywords()
}

pub fn contains_slice() -> usize {
    let set = contains_slice_setup();
    let mut count = 0;
    for k in search_for() {
        count += set.iter().position(|x| x == k).unwrap_or(0);
    }
    count
}

pub fn contains_binary_search_setup() -> Vec<String> {
    let mut words = keywords();
    words.sort();
    words
}

pub fn contains_binary_search() -> usize {
    let set = contains_binary_search_setup();
    let mut count = 0;
    for k in search_for() {
        count += set.binary_search_by(|v| (*k).cmp(v.as_str())).unwrap_or(1);
    }
    count
}

pub fn contains_hashset_setup() -> HashSet<String> {
    let mut set = HashSet::new();
    for k in keywords() {
        set.insert(k.to_string());
    }
    set
}

pub fn contains_hashset() -> i32 {
    let set = contains_hashset_setup();
    let mut count = 0;
    for k in search_for() {
        count += i32::from(set.contains(*k));
    }
    count
}

pub fn contains_btreeset_setup() -> BTreeSet<String> {
    let mut set = BTreeSet::new();
    for k in keywords() {
        set.insert(k.to_string());
    }
    set
}

pub fn contains_btreeset() -> i32 {
    let set = contains_btreeset_setup();
    let mut count = 0;
    for k in search_for() {
        count = i32::from(set.contains(*k));
    }
    count
}

pub fn contains_bloom_setup() -> BloomFilter {
    let builder = FilterBuilder::new(100_000_000, 0.01);
    let mut set = BloomFilter::new(builder);

    for k in keywords() {
        set.add(k.as_bytes());
    }

    set
}

pub fn contains_bloom() -> i32 {
    let set = contains_bloom_setup();
    let mut count = 0;
    for k in search_for() {
        count += i32::from(set.contains(k.as_bytes()));
    }
    count
}

pub fn contains_trie_setup() -> Trie<Vec<u8>, i32> {
    let mut set = Trie::new();

    for k in keywords() {
        set.insert(k.into_bytes(), 0);
    }

    set
}

pub fn contains_trie() -> i32 {
    let set = contains_trie_setup();
    let mut count = 0;
    for k in search_for() {
        count += i32::from(set.contains_key(k.as_bytes()));
    }
    count
}

pub fn contains_fst_setup() -> fst::Set<Vec<u8>> {
    let w = vec![];
    let mut set = fst::SetBuilder::new(w).unwrap();

    let mut keywords = keywords().to_vec();
    keywords.sort();

    for k in keywords {
        let _ = set.insert(k);
    }
    set.into_set()
}

pub fn contains_fst() -> i32 {
    let set = contains_fst_setup();
    let mut count = 0;
    for k in search_for() {
        count += i32::from(set.contains(k));
    }
    count
}

pub fn contains_memchr_setup() -> Vec<String> {
    contains_binary_search_setup()
}

pub fn contains_memchr() -> i32 {
    let set = contains_memchr_setup();

    let mut count = 0;
    for k in search_for() {
        for item in set.iter() {
            count +=
                i32::from(memchr::memmem::find(k.as_bytes(), item.as_str().as_bytes()).is_some());
        }
    }
    count
}
