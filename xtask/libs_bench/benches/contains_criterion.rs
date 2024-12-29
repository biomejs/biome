mod contains;

use contains::*;
use criterion::{criterion_group, criterion_main, Criterion};
use fastbloom_rs::Membership;

const SEARCH_FOR: &[&str] = &[
    "undefined",
    "a",
    "NaN",
    "longVariableName",
    "Infinity",
    "xxxxxxxx",
    "arguments",
    "eval",
];

fn criterion_benchmark(c: &mut Criterion) {
    let set = contains_hashset_setup();
    c.bench_function("contains_hashset", |b| {
        b.iter(|| {
            let mut count = 0;
            for k in SEARCH_FOR {
                count += i32::from(set.contains(*k));
            }
            count
        })
    });

    let set = contains_btreeset_setup();
    c.bench_function("contains_btreeset", |b| {
        b.iter(|| {
            let mut count = 0;
            for k in SEARCH_FOR {
                count = i32::from(set.contains(*k));
            }
            count
        })
    });

    let set = contains_bloom_setup();
    c.bench_function("contains_bloom", |b| {
        b.iter(|| {
            let mut count = 0;
            for k in SEARCH_FOR {
                count += i32::from(set.contains(k.as_bytes()))
            }
            count
        })
    });

    let set = contains_trie_setup();
    c.bench_function("contains_trie", |b| {
        b.iter(|| {
            let mut count = 0;
            for k in SEARCH_FOR {
                count += i32::from(set.contains_key(k.as_bytes()));
            }
            count
        })
    });

    let set = contains_slice_setup();
    c.bench_function("contains_slice", |b| {
        b.iter(|| {
            let mut count = 0;
            for k in SEARCH_FOR {
                count += set.iter().position(|x| x == k).unwrap_or(0);
            }
            count
        })
    });

    let set = contains_fst_setup();
    c.bench_function("contains_fst", |b| {
        b.iter(|| {
            let mut count = 0;
            for k in SEARCH_FOR {
                count += i32::from(set.contains(k));
            }
            count
        })
    });

    let mut set = keywords();
    set.sort();
    c.bench_function("contains_binary_search", |b| {
        b.iter(|| {
            let mut count = 0;
            for k in SEARCH_FOR {
                count += set.binary_search_by(|v| (*k).cmp(v.as_str())).unwrap_or(1);
            }
            count
        })
    });

    let set = contains_memchr_setup();
    c.bench_function("contains_memchr", |b| {
        b.iter(|| {
            {
                let mut count = 0;
                for k in SEARCH_FOR {
                    for item in set.iter() {
                        count += i32::from(
                            memchr::memmem::find(k.as_bytes(), item.as_str().as_bytes()).is_some(),
                        );
                    }
                }
                count
            };
        })
    });
}

criterion_group!(contains, criterion_benchmark);
criterion_main!(contains);
