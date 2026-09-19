use biome_console::fmt::{Display, Formatter};
use rustc_hash::FxHashMap;
use std::{io, sync::LazyLock};

#[derive(Debug)]
pub(crate) struct IterableMethodInfo {
    pub method_name: &'static str,
    pub global_name: Option<&'static str>,
    pub callback_argument_position: usize,
    pub return_value_required: bool,
}

pub(crate) static ITERABLE_METHOD_INFOS: LazyLock<FxHashMap<&'static str, IterableMethodInfo>> =
    LazyLock::new(|| {
        let mut map = FxHashMap::default();
        for method_name in [
            "every",
            "filter",
            "find",
            "findIndex",
            "findLast",
            "findLastIndex",
            "flatMap",
            "map",
            "reduce",
            "reduceRight",
            "some",
            "sort",
            "toSorted",
        ] {
            map.insert(
                method_name,
                IterableMethodInfo {
                    method_name,
                    global_name: None,
                    callback_argument_position: 0,
                    return_value_required: true,
                },
            );
        }
        map.insert(
            "forEach",
            IterableMethodInfo {
                method_name: "forEach",
                global_name: None,
                callback_argument_position: 0,
                return_value_required: false,
            },
        );
        map.insert(
            "from",
            IterableMethodInfo {
                method_name: "from",
                global_name: Some("Array"),
                callback_argument_position: 1,
                return_value_required: true,
            },
        );
        map
    });

impl Display for IterableMethodInfo {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> io::Result<()> {
        if let Some(global_name) = self.global_name {
            write!(fmt, "{}.{}() method", global_name, self.method_name)
        } else {
            write!(fmt, "{}() iterable method", self.method_name)
        }
    }
}
