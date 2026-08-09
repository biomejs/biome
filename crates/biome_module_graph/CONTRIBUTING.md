# Contributing to type inference

This guide explains how to extend the Salsa-backed JavaScript and TypeScript
inference engine in `biome_module_graph`. It focuses on choosing request and
query boundaries that preserve incremental reuse and make inference work
observable.

For the representation of inferred types, also read
[`../biome_js_type_info/CONTRIBUTING.md`](../biome_js_type_info/CONTRIBUTING.md).

## Mental model

Type inference has five distinct layers:

```text
syntax and semantic collection
    -> raw module tables
    -> analyzer-facing requests
    -> tracked Salsa queries
    -> resolver helpers
```

### Collection and resolution

During a module-graph scan, `resolve_js_module` processes each parsed JavaScript
or TypeScript file with `TypeInferenceMode::RawTypesOnly`. `JsModuleInfoCollector`
uses the syntax tree and semantic model to record three inputs:

- `raw_types`: declaration and structural type data owned by the module;
- `raw_expressions`: expression ranges mapped to type references;
- `raw_binding_types`: binding ranges mapped to type references.

A raw reference identifies another entry in the module table, a global type, or
a name or import that still needs resolution. It is not an inferred type.

For analyzer consumers, resolution starts when a request invokes a tracked
query. A lookup query selects the raw reference for the requested range or table entry,
and `ResolutionCtx` converts it to `InferredTypeData`. It follows module-table
references, resolves globals, and uses export and lookup queries for imports.
Named declarations remain symbolic `LocalTypeHandle` values until a consumer
normalizes or inspects their structure. In this code, "local" means owned by one
module's type table; it does not mean block- or function-scoped source code.

Start at `src/module_graph.rs`, `src/js_module_info/collector.rs`, and
`src/db/type_inference/resolver.rs`.

### Analyzer-facing requests

A request is a typed operation used by analyzer rules. It stores module and
source-range inputs, composes tracked queries through
`TypeInferenceRequestContext`, and returns one defined result, such as a
normalized expression type or the expected type of a call argument.

Requests live under `src/type_inference/requests/`. Every concrete request
implements `TypeInferenceRequestMetadata` beside its execution implementation,
which gives the request a stable profile ID and human label.

### Tracked queries

Tracked queries are the incremental computation units. Expression, binding,
module type, and export queries resolve one entry from a raw table.
Normalization, call matching, and whole-module inference have separate query
boundaries.

The query facade is `src/db/queries/type_inference.rs`. Files under
`src/db/queries/type_inference/` are organized by subject: `lookups`, `exports`,
`promises`, `calls`, `normalization`, and `module_types`. Compound Salsa keys
live in the `interned` module. Algorithms used by one family may live in a child module;
resolution helpers shared by multiple families live under
`src/db/type_inference/`.

### Resolver helpers

Cross-module relationships must remain references to data owned by the source
module. Do not copy or clone inferred data from another module into the current
module, including behind `Arc`.

## Vocabulary

| Term | Meaning |
| --- | --- |
| Raw table | Unresolved type descriptions and references collected for one module |
| Module type handle | `LocalTypeHandle` reference to a declaration in one module's raw table |
| Request | Typed analyzer operation that computes one inference result from module and range inputs |
| Classification | Explicit `Match`, `NoMatch`, or `Indeterminate` result for a requested condition |
| Lookup query | Query for one expression, binding, export, or module type entry |
| Normalization | Bounded flattening pass that resolves nested module type handles and removes redundant wrappers |
| Namespace expansion | Discovery and resolution of all visible namespace exports |
| Whole-module inference | Resolution of every raw type, expression, and binding in one module |
| Bottom-up scheduling | Untracked dependency-first preparation for whole-module inference |
| Cycle result | Result or recovery path used when recursive tracked queries form a cycle |

"On demand" describes the normal import strategy, not a constant-work
guarantee. A request can still traverse imports, recursively normalize a large
type, or expand a namespace.

## Request architecture

Every production analyzer flow enters through
`execute_type_inference_request`. This boundary associates one caller, request
identity, source origin, and implementation reference with the entire flow.

A request implementation:

1. stores module inputs, source ranges, and static options;
2. implements `TypeInferenceRequestMetadata` with one stable ID and label;
3. declares its implementation reference;
4. returns an exact source origin;
5. composes operations through `TypeInferenceRequestContext`;
6. does not start timers or emit profiling events directly.

`TypeInferenceRequest` extends `TypeInferenceRequestMetadata`, so a request
cannot compile without its own static identity. Runtime inputs cannot select a
different identity. If an option changes the result contract, model the
alternatives as distinct request types and share private input-resolution
helpers between them.

`TypedService` converts syntax nodes and semantic bindings into request inputs.
It should not orchestrate raw query calls.

### Adding a request

Before adding a request, search existing implementations of
`TypeInferenceRequestMetadata`. Reuse a request when it has the same output and
uncertainty behavior. Caller identity, syntax wrappers, input extraction, and
the exact source origin do not create request boundaries.

Add a request only for a new result contract. State the result and each form of
uncertainty precisely, and give the request one canonical `execute`
implementation. Do not encode a tri-state classification as `Option<bool>`;
return `TypeInferenceClassification` so matches, conclusive non-matches, and
indeterminate results remain explicit. If only input extraction differs, reuse
the request and share an input helper.

Add the request to the matching subject file under
`src/type_inference/requests/`. Implement `Sealed`,
`TypeInferenceRequestMetadata`, and `TypeInferenceRequest`; see
`NormalizedExpressionTypeRequest` and `ExpectedCallArgumentTypeRequest` for
complete examples. Then:

1. re-export the request from `requests/mod.rs` and `type_inference/mod.rs`;
2. add a thin `TypedService` method that constructs and executes it;
3. add a flow test that asserts the request uses the intended queries;
4. verify that the flow does not execute whole-module inference unless its
   contract requires it.

Do not add one request per lint rule. The rule is the caller; the request is a
reusable inference operation.

### Request metadata

Like an analyzer rule's metadata trait, `TypeInferenceRequestMetadata` is
implemented by each request type and required by `TypeInferenceRequest`. A
request therefore cannot compile without its static identity.

Request IDs are stable maintenance identifiers printed in profiles. Use
lowercase words separated by hyphens after the `request.` prefix:

```text
request.expected-call-argument-type
```

Labels are concise human names for the result. IDs and labels must never contain
source text, symbol names, inferred-type strings, import specifiers, or paths.
Explain output, uncertainty, and widening behavior in the request's rustdoc.

## Adding a tracked query

A request usually composes existing queries. Add a tracked query only when the
result needs an independently memoized invalidation boundary.

### Query file structure

Within a subject file, keep tracked queries, cycle callbacks, and ordinary
helpers in separate regions, in that order. A query region contains only
`#[salsa::tracked]` functions.

### Query acceptance criteria

A new query must satisfy all of these criteria:

1. **Smallest semantic result**: it returns no more information than its
   consumer needs. Prefer a classification or projection query over a
   normalized type when the consumer asks one property.
2. **Two-parameter signature**: the database is the first parameter and one
   Salsa input or interned value is the second. If the query needs multiple
   logical values, enclose them in a new type under `interned`; syntax nodes are
   never query parameters.
3. **Minimal dependencies**: the query reads only inputs needed for its result.
   A Salsa execution test proves that an unrelated edit reuses the result and an
   edit to a consumed input recomputes it.
4. **Defined uncertainty**: missing input, unsupported shapes, ambiguity,
   cycles, and exhausted budgets have documented behavior.
5. **Cycle behavior**: recursive queries define a `cycle_result` callback. The
   callback either runs an explicit recovery algorithm, such as the
   whole-module SCC fallback, or preserves uncertainty. Type queries return
   `Unknown`; classification queries return `Indeterminate`; they do not invent
   a definite type or match result.
6. **Bounded traversal**: recursive shape, overload, import, and export walks
   have an explicit work budget. Do not turn incomplete work into a definite
   negative. Return the request's documented indeterminate result unless the
   helper explicitly documents partial-result semantics.
7. **Observable boundary**: the query belongs to one registered query family
   and has one implementation reference.
8. **Independent validation**: tests prove both correctness and selective Salsa
   execution.

Interning an input gives equal values a shared identity. It does not memoize the
query result by itself.

### Query families

`TypeInferenceQueryKind` identifies the subject module that owns a tracked
query: lookups, exports, promises, calls, normalization, or module types. Add a
query to its existing family without adding an enum variant. Add a family only
when the query needs a new subject module with a distinct responsibility.

### Graph discovery and type resolution

Keep graph-only export discovery separate from type resolution. Origin and
namespace-name queries should inspect module graph inputs without caching
provisional inferred types. The owning expression, binding, or export can then
be resolved with a lookup query.

This separation prevents graph discovery from unnecessarily joining inference
cycles.

### Tracked and untracked entry points

Tracked queries may call other tracked queries directly. New tracked queries
must not call the public bottom-up scheduler. The guarded deep-import fallback
in `imports.rs` is the existing exception.

Code outside a tracked query may use bottom-up scheduling when it deliberately
needs whole-module inference for an import graph. The scheduler is untracked
because it prepares dependencies iteratively and then invokes the tracked
`infer_module_types` query.

## Work that widens inference

Review every new request and query for the following transitions.

### Recursive normalization

Normalization rewrites a type into the directly inspectable form required by a
consumer. It resolves module type handles throughout nested unions,
intersections, tuples, instances, and `typeof` wrappers, then removes wrappers
that no longer add information. It preserves identities such as classes and
interfaces when later operations depend on them.

Do not normalize when a targeted classifier can produce the required result.
Analyzer convenience methods must state whether they normalize their result.
Review the entire request flow, not only its first lookup query.

### Namespace expansion

A namespace import or re-export may discover and resolve every visible export,
including exports reached through blanket re-exports. Treat namespace work as
proportional to the visible namespace, not as an individual lookup.

### Deep import fallback

On-demand import resolution has a recursion limit. Crossing the limit invokes
bottom-up whole-module inference to avoid consuming the Rust stack. A request
that normally stays lookup-oriented can therefore become graph-wide on a
sufficiently deep chain.

The transition is implemented in `src/db/type_inference/imports.rs`.

### Whole-module APIs

`infer_module_types` resolves every raw type-table entry, expression, and
binding in a module. Do not call it from a new request unless the request
contract requires all three tables.

`infer_module_types_bottom_up` additionally prepares static imports and
re-exports dependency-first. Do not call it from a new tracked query.

### Cycle recovery

Lookup query cycles return `Unknown` for type results or `Indeterminate` for
classifications. Whole-module cycles use an SCC fallback that blocks members of
the active component while preserving resolvable dependencies outside it.

Raw declaration recursion is different from a Salsa query cycle. Preserve
symbolic module type handles for recursive declarations rather than eagerly
collapsing them to `Unknown`.

### Call inference

Expected-argument inference may resolve the callee and every sibling argument
before matching signatures. Count this as one request with multiple tracked
query executions; do not describe it as a single constant-cost projection.

## Result semantics

Do not hide distinct outcomes behind `Option<bool>`. A classification request
returns `TypeInferenceClassification`:

- `Match` means available type information proves the requested condition.
- `NoMatch` means available type information proves the condition does not hold.
- `Indeterminate` means inference cannot decide because information is missing,
  unsupported, ambiguous, cyclic, or beyond a work budget.

`None` remains appropriate when a request has no result to return and that
absence has one documented meaning. `Unknown` is an inferred type value whose
structure cannot be resolved. Missing raw entries indicate that collection did
not produce the requested key. Ambiguous exports remain ambiguous rather than
selecting the first match.

Budget exhaustion follows the documented request or helper behavior and must
not be mistaken for a conclusive negative result. Rustdoc on a public request or
query must distinguish every result it can return.

## Source and implementation references

Every request has a `TypeInferenceCodeReference`. It records the source file,
line, and symbol adjacent to the canonical `execute` method. Inference profiles
use this reference to lead maintainers from a reported source range to the
implementation that orchestrated the work.

Query families and whole-module reasons also have stable IDs. Each recorded
query execution retains its own implementation reference, so multiple query
bodies can share one family. Code references correspond to the Biome version
that produced a report.

Do not read `ModuleInfo::path` from inside a tracked query for profiling. The
path is a Salsa input, so reading it changes the query's dependency set. The
outer request boundary snapshots document paths before entering tracked
queries. Query instrumentation records only work nested under a production
request and carries only module identities and `TextRange` values.

## Profiling inference

Maintainers can ask a user to reproduce CLI inference work with the hidden
maintenance option:

```shell
biome lint --profile-type-inference path/to/reproduction
biome check --profile-type-inference path/to/reproduction
```

The default report ranks requests by consumer, tracked query bodies, and
whole-module widening. Use `--verbose` to show every recorded source range and
implementation reference.

Request and query timings are inclusive and must not be added together. A high
request count with few tracked query executions usually indicates Salsa reuse.
Ranges are zero-based, half-open UTF-8 byte offsets. Profiles contain relative
paths and timings, but not source text, inferred types, or import specifiers.

## Testing

Every behavior change must include correctness tests. Request or query boundary
changes must also include selectivity tests.

### Correctness matrix

Cover the cases relevant to the request or query:

- expressions and bindings in the same module;
- recursive declarations;
- named, default, and namespace imports;
- named, blanket, and namespace re-exports;
- missing and ambiguous exports;
- import and declaration cycles;
- disabled inference;
- unsupported input shapes;
- traversal budget exhaustion.

### Salsa execution tests

Use the Salsa event helpers from `biome_db::testing` to assert which tracked
query bodies executed. At minimum, a lookup request test should prove that
`infer_module_types` did not execute.

Test cache reuse separately from correctness:

1. execute the same request twice;
2. assert both consumer requests complete;
3. assert the tracked query body executes only when Salsa recomputes it;
4. edit an unrelated input and assert the query remains reusable;
5. edit a dependency and assert the query recomputes.

Salsa events are a deterministic recomputation oracle. They are not a production
timing mechanism because they do not include query completion events.

### Performance coverage

Benchmark workloads separately:

- cold lookup request;
- warm lookup request;
- dependency edit;
- recursive normalization;
- namespace expansion;
- deep import chain;
- cycle recovery;
- explicit whole-module inference.

Do not compare benchmark names alone. Confirm that the compared benchmarks
invoke the same inference boundary and infer the same tables.

## Review checklist

Before approving a request or query, verify:

- [ ] The consumer enters through a declared request.
- [ ] The request represents a reusable result contract, not a lint rule.
- [ ] The request has an exact source origin.
- [ ] Existing query primitives are reused where possible.
- [ ] New query keys are stable Salsa values.
- [ ] Dependency and invalidation boundaries are stated.
- [ ] Classifications use explicit tri-state results rather than `Option<bool>`.
- [ ] `Indeterminate`, `None`, `Unknown`, ambiguity, and exhaustion are
      distinguished.
- [ ] Recursive traversal has a budget, and recursively dependent tracked
      queries define cycle behavior.
- [ ] Namespace and import-chain breadth were reviewed.
- [ ] Whole-module inference is absent or explicitly justified.
- [ ] Cross-module data remains owned by its source module.
- [ ] Correctness and Salsa execution tests cover the flow.
- [ ] The metadata trait has a stable ID and label, and the request has an
      implementation reference.
