# Biome Type Architecture

Biome's type inference is designed for incremental IDE use. Any module can be
replaced while analysis is running, so cross-module types must remain tracked by
Salsa rather than copied between module-graph entries.

## Two Type Worlds

The type system has two intentionally separate representations.

The raw collector representation is [`RawTypeData`](src/type_data.rs). It owns
`TypeReference` values and is stored in `JsModuleInfo`. The collector has no
database access and performs no cross-module or global resolution. Its
`TypeStore` is only a stable, module-local table.

The database-backed representation is
[`InferredTypeData`](src/interned_types.rs). Complex payloads are Salsa interned
handles, so inferred values are cheap to copy and participate in dependency
tracking. Public inferred names come from [`resolved.rs`](src/resolved.rs):
database-backed names use the `Inferred` prefix when a raw type has the same
conceptual name.

Raw references distinguish identities explicitly:

```rust,ignore
enum TypeReference {
    Qualifier(TypeReferenceQualifier),
    Resolved(RawTypeId),
    Import(TypeImportQualifier),
}

enum RawTypeId {
    Local(TypeId),
    Global(GlobalTypeId),
}
```

`RawTypeId::Local` addresses the current module's raw table. A global ID names a
canonical database-native global. Neither should be inspected as an inferred
type without resolution.

## Collection

Local inference in [`local_inference.rs`](src/local_inference.rs) derives raw
types from syntax. For example, `a + b` becomes a deferred expression containing
references to `a` and `b`; the collector does not attempt to discover their
cross-module values.

[`RawTypeCollector`](src/type_store.rs) is the narrow interface used during this
phase. It registers raw values and creates local references. Imports and scoped
qualifiers remain explicit so later database queries can resolve them against
the current module graph and semantic model.

## Database Inference

`infer_module_types` is the tracked dependency query in
[`queries.rs`](../biome_module_graph/src/db/queries.rs). External consumers call
`infer_module_types_bottom_up`, whose nontracked iterative walk warms imports
innermost-first. This avoids deep Rust and Salsa revalidation stacks while
preserving backdating at each tracked module query. Backdating means that Salsa
keeps the previous `changed_at` revision when running the query again produces
the same result. This avoids rerunning a dependent query solely because the
dependency was checked again.

`resolve_raw_types` converts a module's raw table into `InferredModuleTypes`.
The result contains:

- the module key used by local inferred handles;
- the resolved type table;
- named declaration IDs;
- expression types keyed by source range;
- binding types keyed by source range.

Imports are resolved through tracked module queries. Globals come from the
memoized database-native global table. Expression evaluation and structural
normalization happen in the module graph, not in the collector.

## Handles And Normalization

`InferredTypeData::Local` contains both a module key and a local type ID. The
pair identifies a type within one database, including when the handle crosses an
import boundary. `GlobalType` contains a canonical global ID and remains
memberless until expanded.

Consumers must call `normalize_type` or use an `InferredModuleTypes` lookup
before inspecting values that may contain handles. Normalization resolves local
handles, expands non-nominal globals, unwraps deferred `typeof` values, and
collapses structural wrappers. Cycles and exhausted walks degrade to `Unknown`.

## Cycles

Salsa invokes `infer_module_types_cycle_result` when tracked module inference
encounters an import cycle. The fallback computes the root's strongly connected
component and resolves the requested module with `CycleFallback`. Imports that
would re-enter the component are suppressed, while imports outside it remain
tracked normally.

Raw self-references use local handles. During raw conversion, re-entering a type
currently being resolved returns its stable local handle; the outer resolution
then stores the completed value in the module table.

## Budgets And Tri-State Results

Potentially cyclic or expanding walks that cross inferred type or module-graph
edges use deterministic query-local budgets. Deduplicated revisits do not
consume another step. A bounded walk documents its conservative fallback:
incomplete type results generally degrade to `Unknown`, while incomplete
fallible predicates return `None`.

For `Option<bool>` predicates:

```rust,ignore
match ty.is_promise_instance() {
    Some(true) => report(),
    Some(false) => {}
    None => return, // Do not diagnose from partial inference.
}
```

`Some(true)` and `Some(false)` are complete answers. `None` means unresolved
data, a cycle, or budget exhaustion prevented a reliable answer.

## Module Index Invalidation

Every dynamic module-path lookup must read `ModuleDb::module_graph_generation`.
Structural registry mutations acquire the pending Salsa setter before publishing
map changes and commit the new generation afterward. This prevents a reader from
observing a new map under an old generation.

## Testing

Use module-graph v2 tests for database inference, invalidation, backdating,
cycles, and normalization. Raw collector snapshots should describe only local
table identities; they must not imply that unresolved collector entries are
canonical globals. Type-aware analyzer fixtures verify the final lint-facing
behavior.
