---
name: type-inference
description: Use this skill when working on Biome's Salsa-backed JavaScript and TypeScript inference, including type-aware lint rules, raw collection or inferred representations, analyzer requests, tracked queries, import or cycle resolution, profiling, and Salsa invalidation tests. Do not use for standalone CSS/HTML module-graph data unrelated to JS/TS inference or for ordinary semantic binding analysis.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

# Salsa-Backed Type Inference

## Scope

Use this skill for JavaScript and TypeScript type inference: raw collection,
Salsa-backed inferred values, analyzer requests, tracked queries, and
inference-specific tests.

Load the corresponding skill for general lint scaffolding, diagnostics,
snapshots, or changesets. CSS and HTML module-graph data is outside this skill
unless it directly participates in JavaScript or TypeScript inference.

## Read the Relevant Guide

Do not load both architecture guides in full for every inference task. Read the
sections matching the boundary being changed:

| Task | Canonical guide |
| --- | --- |
| Raw and inferred representations, collection, inference layers, handles, normalization, work limits | [`biome_js_type_info/CONTRIBUTING.md`](../../../crates/biome_js_type_info/CONTRIBUTING.md) |
| Analyzer requests, tracked queries, widening, profiling, Salsa execution tests | [`biome_module_graph/CONTRIBUTING.md`](../../../crates/biome_module_graph/CONTRIBUTING.md) |

Read both guides when changing the architecture across their boundary. Then
inspect the implementation files for the specific request, query family, or
representation being changed.

Do not use these checked-in files as current examples:

- `crates/biome_js_type_info/src/resolver.rs`
- `crates/biome_js_type_info/src/flattening.rs`
- `crates/biome_js_type_info/src/type.rs`
- `crates/biome_js_type_info/src/conditionals.rs`
- `crates/biome_js_type_info/src/helpers.rs`
- `crates/biome_module_graph/src/js_module_info/module_resolver.rs`

They are not declared by the active crate module trees. Treat them as legacy
residue unless the task explicitly concerns removing or migrating them.

## Mental Model

Type inference has five layers:

```text
syntax and semantic collection
    -> raw module tables
    -> analyzer-facing requests
    -> tracked Salsa queries
    -> resolver helpers
```

Collection walks one module without database access. It records raw types,
expressions, and binding types in `JsModuleInfo`. A request defines one
analyzer-facing result contract. Tracked queries provide memoization and
invalidation boundaries. Resolver helpers evaluate references and inferred
structures inside those boundaries.

Keep the three type worlds distinct:

| World | Main types | Purpose |
| --- | --- | --- |
| Raw collector | `TypeData` / `RawTypeData`, `TypeReference`, `RawTypeId`, `TypeStore` | Module-local syntax, declarations, imports, and deferred expressions |
| Database-backed | `InferredTypeData<'db>`, `LocalTypeHandle`, `GlobalTypeId` | Inferred values and module ownership in tracked computations |
| Analyzer-facing | `InferredType<'db>` | Conservative, bounded inspection for lint rules |

`TypeReference` belongs to the raw world. A lint rule should not pattern-match
raw `TypeData` when `InferredType` already provides the required operation.

Inferred data remains owned by the module that declared it. Do not copy another
module's inferred payload into the current module, including behind `Arc`.
Preserve ownership through references, module-aware handles, global IDs, and
tracked queries.

## Choose the Narrowest Boundary

The inference levels are alternatives, not sequential phases:

| Need | Boundary |
| --- | --- |
| Inspect facts collected in one module | Raw local tables |
| Resolve one expression, binding, export, member, argument, or classification | Targeted request and tracked query |
| Resolve every raw type, expression, and binding in a module | Complete module inference |

A wider boundary is not inherently more correct. Use this decision order when
more than one level can answer:

```text
inspect raw local information
    -> return when the local result is conclusive
    -> resolve the smallest selected reference
    -> preserve uncertainty or widen only when the contract requires it
    -> use complete-module inference as the last resort
```

`infer_module_types` serves contracts that need complete tables.
`infer_module_types_bottom_up` is an untracked external scheduler and must not
be called from a new tracked query.

## Type-Aware Lint Rules

Type-aware JavaScript rules use the analyzer service rather than database
queries directly:

1. Declare `domains: &[RuleDomain::Types]` in rule metadata.
2. Use `Typed<N>` as the rule query.
3. Call an existing inference method on `RuleContext`, backed by `TypedService`.
4. Inspect the returned `InferredType` through its bounded helpers.
5. Handle classifications explicitly as `Match`, `NoMatch`, or `Indeterminate`.

Start from `crates/biome_js_analyze/src/services/typed.rs` and search current
`Typed<` consumers. Prefer a classification request when a rule asks one
property; do not normalize and traverse a complete type when a narrower
classifier answers the question.

## Changing Raw Inference

When adding syntax-derived type information:

1. Define the raw representation in
   `crates/biome_js_type_info/src/type_data.rs`.
2. Collect it in `crates/biome_js_type_info/src/local_inference.rs` or
   `crates/biome_module_graph/src/js_module_info/collector.rs`, preserving
   unresolved operands as `TypeReference` values.
3. Convert it in `crates/biome_js_type_info/src/interned_types.rs`.
4. Add evaluation under `crates/biome_module_graph/src/db/type_inference/` only
   when generic raw-to-inferred conversion is insufficient.
5. Audit raw and inferred matches, traversal, and formatting for the new variant.

Collector snapshots prove the raw structure remains deferred where database
resolution is required. Query tests separately prove the inferred result.

## Requests and Tracked Queries

Requests live under
`crates/biome_module_graph/src/type_inference/requests/`. Add one for a reusable
result contract, not for an individual lint rule. Reuse a current request when
output and uncertainty behavior match.

A request defines stable metadata, one canonical execution path, exact source
origin, and explicit uncertainty. Compose operations through
`TypeInferenceRequestContext`; analyzer code should not construct low-level query
inputs.

Add a tracked query only when its result needs an independent memoization and
invalidation boundary. A new query must:

- return the smallest semantic result its consumer needs;
- take the database and one Salsa input or interned key;
- read only dependencies that can affect the result;
- define missing-input, ambiguity, cycle, and work-budget behavior;
- preserve `Unknown` or `Indeterminate` rather than inventing a definite result;
- use the current query instrumentation family;
- have correctness and selective-execution tests.

Interning equal inputs gives them shared identity; it does not memoize query
results. Follow the module-graph guide's **Request architecture** and **Adding a
tracked query** sections for current traits, registration, and test requirements.

## Resolution and Result Semantics

On-demand resolution follows the selected lookup path until the request requires
broader work. Normalization resolves reachable handles with bounded traversal;
it is not complete-module inference. Namespace expansion and guarded deep-import
fallback can widen work substantially and must remain explicit and observable.

Do not conflate these outcomes:

| Result | Meaning |
| --- | --- |
| `None` | The request has no result under its documented contract |
| `InferredTypeData::Unknown` | Inference produced a type whose structure is undetermined |
| `InferredTypeData::UnknownKeyword` | Source explicitly uses TypeScript's `unknown` type |
| `TypeInferenceClassification::Indeterminate` | Inference cannot prove a match or non-match |
| `TypeInferenceClassification::Match` | Inference conclusively proves the condition |
| `TypeInferenceClassification::NoMatch` | Inference conclusively disproves the condition |

Unknown or indeterminate information is not a negative result. Preserve it to
avoid false-positive diagnostics. Read the canonical widening, cycle recovery,
and result-semantics sections before changing those paths.

## Testing and Profiling

Test the narrowest affected boundary:

- collector tests for raw records and references;
- request/query tests for results, imports, cycles, and uncertainty;
- Salsa event tests when dependency or invalidation scope changes;
- analyzer fixtures for final diagnostic behavior.

For targeted flows, prove unrelated edits reuse the query and consumed inputs
recompute it. Assert whole-module inference does not execute unless the request
contract or tested fallback requires it.

Use the maintenance profile documented in the module-graph guide when a request
resolves more data than expected. Request and query timings are inclusive and
must not be added together. Load `testing-codegen` for snapshot mechanics.

## Review Checklist

- The implementation uses the correct raw, inferred, or analyzer-facing world.
- The request or query is the narrowest boundary satisfying its contract.
- Missing data, ambiguity, cycles, and exhausted budgets preserve uncertainty.
- Cross-module inferred data remains owned by its source module.
- New request contracts are reusable and have stable metadata.
- New query keys and dependencies are stable Salsa values.
- Whole-module inference is absent or explicitly justified.
- Correctness and selective-execution tests cover the changed boundary.
