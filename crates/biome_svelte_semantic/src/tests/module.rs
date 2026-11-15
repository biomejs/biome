use super::module;
use crate::{FileKind, Rune, RuneTarget};

#[test]
fn modules_are_in_runes_mode() {
    let model = module(
        r#"import { SvelteMap } from 'svelte/reactivity';
export class Counter {
  count = $state(0);
}
export const cache = new SvelteMap();
let total = $state(0);
export function increment() { total += 1; }
"#,
    );
    assert_eq!(model.file_kind(), FileKind::Module);
    assert!(model.is_runes_mode());
    let calls: Vec<_> = model
        .rune_calls()
        .map(|call| (call.rune(), call.target().clone()))
        .collect();
    assert_eq!(
        calls,
        [
            (Rune::State, RuneTarget::ClassField("count".into())),
            (Rune::State, RuneTarget::Variable),
        ]
    );
    let total = model
        .symbols()
        .find(|symbol| symbol.name() == "total")
        .unwrap();
    assert!(total.is_reassigned());
    assert_eq!(total.rune(), Some(Rune::State));
    let exported: Vec<_> = model
        .exports()
        .map(|export| export.name().to_string())
        .collect();
    assert_eq!(exported, ["Counter", "cache", "increment"]);
    let reference = model
        .references()
        .find(|reference| {
            reference.name() == "SvelteMap" && reference.access() == crate::Access::Read
        })
        .unwrap();
    assert_eq!(
        reference.imported_member(),
        Some(("svelte/reactivity", "SvelteMap"))
    );
}
