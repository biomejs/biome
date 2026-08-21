---
name: type-inference
description: Guide for Biome's Salsa-backed JavaScript and TypeScript type inference. Use when implementing type-aware lint rules, changing raw type collection or inferred type representations, adding analyzer requests or tracked queries, resolving imports or cycles, profiling inference, or testing Salsa invalidation.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

# Salsa-Backed Type Inference

## Scope

Use this skill for JavaScript and TypeScript type inference. It covers the raw
collector representation, Salsa-backed inferred values, analyzer requests,
tracked queries, and inference-specific tests.

For general lint-rule scaffolding, diagnostics, snapshots, or changesets, load
the corresponding skill instead of duplicating that guidance here. CSS and HTML
module-graph data is also outside this skill unless it directly participates in
JavaScript or TypeScript type inference.

## Read First

Read both canonical guides before changing inference architecture:

1. `crates/biome_module_graph/CONTRIBUTING.md` for request and query boundaries,
   widening, profiling, and Salsa execution tests.
2. `crates/biome_js_type_info/CONTRIBUTING.md` for raw and inferred type
   representations, inference layers, handles, normalization, and work limits.

Then read the implementation files for the boundary being changed. Do not use
the following checked-in files as current examples:

- `crates/biome_js_type_info/src/resolver.rs`
- `crates/biome_js_type_info/src/flattening.rs`
- `crates/biome_js_type_info/src/type.rs`
- `crates/biome_js_type_info/src/conditionals.rs`
- `crates/biome_js_type_info/src/helpers.rs`
- `crates/biome_module_graph/src/js_module_info/module_resolver.rs`

These files are not declared by the active crate module trees. Treat them as
legacy residue unless the task explicitly concerns removing or migrating them.

## Mental Model

Type inference has five layers:

```text
syntax and semantic collection
    -> raw module tables
    -> analyzer-facing requests
    -> tracked Salsa queries
    -> resolver helpers
```

Collection walks one module without database access. It records `raw_types`,
`raw_expressions`, and `raw_binding_types` in `JsModuleInfo`. A request represents
one analyzer-facing result contract. Tracked queries provide memoization and
invalidation boundaries. Resolver helpers evaluate raw references, imports,
expressions, and inferred structures inside those query boundaries.

### Two Type Worlds

Keep the collector and database representations distinct:

| World | Main types | Purpose |
| --- | --- | --- |
| Raw collector | `TypeData` / `RawTypeData`, `TypeReference`, `RawTypeId`, `TypeStore` | Records module-local syntax, declarations, imports, and deferred expressions without database access |
| Database-backed | `InferredTypeData<'db>`, `LocalTypeHandle`, `GlobalTypeId`, Salsa-interned payloads | Carries inferred values and module ownership through tracked computations |
| Analyzer-facing | `InferredType<'db>` | Provides conservative, bounded inspection methods to lint rules |

Relevant sources:

- Raw types: `crates/biome_js_type_info/src/type_data.rs`
- Raw collection: `crates/biome_js_type_info/src/local_inference.rs` and
  `crates/biome_js_type_info/src/type_store.rs`
- Inferred types: `crates/biome_js_type_info/src/interned_types.rs` and
  `crates/biome_js_type_info/src/resolved.rs`
- Analyzer wrapper: `crates/biome_js_type_info/src/inferred_type.rs`

`TypeReference` belongs to the raw world. Database-backed values use module-aware
handles such as `LocalTypeHandle`; they do not use the legacy resolver context.
Do not pattern-match raw `TypeData` from a lint rule when `InferredType` already
provides the required operation.

### Cross-Module Ownership

Inferred data remains owned by the module that declared it. Do not copy or clone
another module's inferred payload into the current module, including behind
`Arc`. Preserve ownership through raw import references, `LocalTypeHandle`,
global IDs, and tracked queries.

## Choose the Narrowest Boundary

The three inference levels are alternatives, not sequential phases:

| Need | Boundary |
| --- | --- |
| Inspect what collection recorded in one module | Raw local tables |
| Resolve one expression, binding, export, member, argument, or classification | Targeted request and tracked query |
| Resolve every raw type, expression, and binding in a module | Complete module inference |

A wider boundary is not inherently more correct. Choose it only when the
operation's result contract requires the wider data. An inconclusive targeted
result does not automatically justify complete-module inference.

When an operation can answer from more than one level, use this escalation
order:

```text
inspect raw local information
    -> return when the local result is conclusive
    -> resolve the smallest selected reference with a targeted query
    -> preserve uncertainty or widen only when the contract requires it
    -> use complete-module inference as the last resort
```

This is a decision order, not a pipeline that every request must execute. A
request whose contract inherently requires database resolution may start with a
targeted query, but it must still avoid complete-module inference unless the
complete tables are required.

Targeted lookups are the default. `infer_module_types` is for contracts that
need complete tables. `infer_module_types_bottom_up` is an untracked external
scheduler for complete inference and must not be called from a new tracked
query.

## Type-Aware Lint Rules

Type-aware JavaScript rules use the analyzer service rather than database
queries directly:

1. Declare `domains: &[RuleDomain::Types]` in rule metadata.
2. Use `Typed<N>` as the rule query.
3. Call an existing inference method on `RuleContext`, backed by `TypedService`.
4. Inspect returned `InferredType` values with their bounded helper methods.
5. Handle classifications explicitly as `Match`, `NoMatch`, or `Indeterminate`.

Start with `crates/biome_js_analyze/src/services/typed.rs`. Current consumers
include `no_floating_promises.rs` and `no_misused_promises.rs` under
`crates/biome_js_analyze/src/lint/nursery/`.

Prefer a classification request when a rule asks one property, such as whether
an expression is Promise-like. Do not normalize and traverse a complete type
when a narrower classifier can answer the rule's question.

## Changing Raw Inference

When adding or changing syntax-derived type information:

1. Define the raw representation in
   `crates/biome_js_type_info/src/type_data.rs`.
2. Collect it from syntax in
   `crates/biome_js_type_info/src/local_inference.rs` or
   `crates/biome_module_graph/src/js_module_info/collector.rs`. Preserve
   unresolved operands as `TypeReference` values.
3. Add or update conversion to the inferred representation in
   `crates/biome_js_type_info/src/interned_types.rs`.
4. Add custom database-backed evaluation under
   `crates/biome_module_graph/src/db/type_inference/` only when the variant needs
   behavior beyond generic raw-to-inferred conversion.
5. Audit raw/inferred conversion, semantic variant matches, and formatting in
   `interned_types.rs`, `inferred_type.rs`, `format_type_info.rs`, and
   `format_inferred_type_info.rs`.

Collector snapshots must prove the raw structure remains deferred when database
resolution is required. Query tests must separately prove the inferred result.
Change `type_traversal.rs` only when the generic traversal engine or its limits
change; type-specific child scheduling belongs to semantic visitor matches.

## Adding an Analyzer Request

Requests live under `crates/biome_module_graph/src/type_inference/requests/`.
Add one only for a reusable result contract, not for an individual lint rule.

1. Search existing `TypeInferenceRequestMetadata` implementations for a matching
   output and uncertainty contract.
2. Implement `Sealed`, `TypeInferenceRequestMetadata`, and
   `TypeInferenceRequest` with a stable ID, label, implementation reference, and
   exact source origin.
3. Compose operations through `TypeInferenceRequestContext`; do not construct
   low-level query inputs in analyzer code.
4. Re-export the request and add a thin method to `TypedService`.
5. Test the result and tracked query flow. When the contract is targeted, use
   the complete-inference checks under [Salsa Inputs and
   Invalidation](#salsa-inputs-and-invalidation).

Use `NormalizedExpressionTypeRequest` and
`ExpectedCallArgumentTypeRequest` as current examples. Request infrastructure is
defined in `type_inference/request.rs`, `type_inference/context.rs`, and
`type_inference/requests/` under `crates/biome_module_graph/src/`.

## Adding a Tracked Query

Queries live under
`crates/biome_module_graph/src/db/queries/type_inference/`, organized by subject.
Add a query only when its result needs an independent memoization and
invalidation boundary.

1. Return the smallest semantic result the consumer needs.
2. Use a two-parameter signature: the database and one Salsa input or interned
   key. Add a compound key in `interned.rs` when several logical values are
   required.
3. Define uncertainty behavior. Recursive tracked queries need a `cycle_result`:
   type queries preserve uncertainty as `Unknown`, while classification queries
   use `Indeterminate`.
4. Wrap the observable query body with `execute_query` and the matching
   `TypeInferenceQueryKind` family. Reuse an existing family when its subject
   matches; add one only for a new subject module with a distinct responsibility.
5. Read only dependencies that can affect the result, and verify this with Salsa
   execution tests.

Interning equal query inputs gives them a stable shared identity; it does not
memoize a query result. Memoization belongs to `#[salsa::tracked]` functions.

The query facade is
`crates/biome_module_graph/src/db/queries/type_inference.rs`. Shared resolution
algorithms live under `crates/biome_module_graph/src/db/type_inference/`.

## Resolution, Normalization, and Widening

- On-demand import resolution follows only the selected export and lookup path
  until the request requires broader work.
- Normalization resolves reachable local handles and structural wrappers with a
  bounded traversal. It is not complete-module inference.
- Namespace imports and re-exports can expand every visible export. Treat them
  as namespace-wide work.
- Exhausting the guarded import-depth budget can widen to bottom-up complete
  inference. This recovery may return a conclusive selected type from the
  complete tables; keep the widening explicit and observable.
- Lookup-query cycles preserve uncertainty. Complete-module cycles use the
  existing strongly-connected-component fallback, which may retain information
  resolved outside the active component.

Do not derive a definite result from incomplete work alone. Preserve uncertainty
when bounded traversal or cycle recovery cannot conclude; an explicit recovery
algorithm may return a conclusive result from the data it successfully resolves.

## Result Semantics

Do not conflate these outcomes:

| Result | Meaning |
| --- | --- |
| `None` | The request or query has no result under its documented contract, such as an uncollected range or disabled inference |
| `InferredTypeData::Unknown` | Inference produced a type value whose structure could not be determined |
| `InferredTypeData::UnknownKeyword` | Source code explicitly uses TypeScript's `unknown` type |
| `TypeInferenceClassification::Indeterminate` | Available inference cannot prove either a match or a non-match |
| `TypeInferenceClassification::NoMatch` | Available inference conclusively disproves the requested condition |

Unknown or indeterminate information is not a negative result. Preserve it to
avoid false-positive diagnostics.

## Salsa Inputs and Invalidation

`ModuleInfo` is a Salsa input containing a path and module kind. Path-to-module
associations live outside Salsa, so dynamic path lookup is tracked through
`ModuleGraphGeneration`. Read existing module database APIs rather than bypassing
that generation.

When changing dependencies or invalidation:

1. Run the query once and repeat it to prove cache reuse.
2. Edit an unrelated input and prove the query body is not recomputed.
3. Edit a consumed input and prove the query body is recomputed.
4. Use `biome_db::testing` event helpers to inspect `WillExecute` events.
5. For ordinary targeted flows that should not widen, assert that
   `infer_module_types` did not execute and that the
   `prepare_module_types_bottom_up_for_import_depth` execution count is zero. A
   query-level depth-limit fallback test instead asserts that preparation
   executed and verifies its semantic result. Changes to profiling attribution
   additionally verify the displayed whole-module widening reason.

Representative tests are in
`crates/biome_module_graph/tests/spec_tests/requests.test.rs`, `queries.test.rs`,
and `database.test.rs`.

## Testing

Test the narrowest affected boundary:

| Boundary | Command |
| --- | --- |
| Raw collection | `cargo test -p biome_js_type_info --test local_inference` |
| Raw type data | `cargo test -p biome_js_type_info --test type_data` |
| Queries and requests | `cargo test -p biome_module_graph --test spec_tests` |
| Type-aware lint rule | `just test-lintrule <ruleName>` |
| Snapshot review | `cargo insta review` |

Collector tests verify deferred raw values. Module-graph tests verify resolved
types, request contracts, cycles, imports, and Salsa execution. Analyzer fixtures
verify the final diagnostic behavior.

## Profiling

Use the hidden maintenance profile when a request resolves more data than
expected:

```shell
cargo biome-cli-dev lint \
  --profile-type-inference \
  --verbose \
  --only=nursery/noFloatingPromises \
  path/to/file.ts
```

Profiles attribute tracked query executions and whole-module widening to an
analyzer request. A missing query record can mean the query was not invoked or
Salsa reused its cached result. Request and query timings are inclusive and must
not be added together.

## Review Checklist

- The implementation uses the correct raw, inferred, or analyzer-facing type
  world.
- The request or query is the narrowest boundary that satisfies its contract.
- Missing data, ambiguity, cycles, and work-limit exhaustion preserve
  uncertainty.
- Cross-module inferred data remains owned by its source module.
- Correctness tests and Salsa execution tests cover the changed boundary.
