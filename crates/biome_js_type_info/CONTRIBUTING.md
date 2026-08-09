# Biome Type Architecture

Biome's type inference is designed for incremental IDE use. Any module can be
replaced while analysis is running, so cross-module types must remain tracked by
Salsa rather than copied between module-graph entries.

In this guide, the **collector** is the module-local phase that walks syntax and
records declarations and expressions as raw type information. It does not query
the database or resolve types across modules. **Collection** is the process of
producing that raw information.

For example, collection turns this TypeScript:

```ts
declare function identity<T>(value: T): T;
const result = identity(1);
```

into raw records resembling the following, with table IDs and scope metadata
omitted:

```text
T        = Generic(name: "T")
identity = Function(type_parameters: [T], parameter: T, return: T)
result   = TypeofExpression::Call(callee: identity, arguments: [1])
```

The collector preserves the relationship between `T`, the parameter, and the
return type. It records the call as a deferred expression; it does not yet infer
that `result` has the type of `1`.

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

## Inference Layers

The type system can answer type questions at three levels. These levels are
alternatives, not stages that every operation must run in sequence. Each level
looks at a wider part of the program and therefore records more dependencies.

Start with the question the caller needs to answer:

1. **What facts did the collector record in this module?** Inspect raw local
   information.
2. **What is the answer for this particular reference or expression?** Run a
   targeted query.
3. **What are the inferred types for this module as a whole?** Infer the
   complete module.

A wider level is not inherently more correct. Use it only when the answer
depends on information that a narrower level does not inspect.

### Raw Local Information

Raw local information is the output of the collection phase. It describes syntax
and declarations from one module using `RawTypeData` and `TypeReference`.
Deferred expressions preserve their operands, callees, arguments, and member
references without evaluating them.

In the `identity(1)` example, this level can show that `result` is initialized by
a call to `identity` with the argument `1`:

```text
result = TypeofExpression::Call(callee: identity, arguments: [1])
```

It cannot yet substitute `1` for `T` or determine the call's return type. Use
this level when the recorded structure is enough. It has no database access and
cannot inspect another module's raw table directly.

### Targeted On-Demand Resolution

Targeted resolution answers one semantic question, such as:
- What's the type returned by this expression?
- What's the type returned by this function call?
- What's the type of this binding or reference?

It starts from the corresponding raw reference and evaluates only the paths needed for that answer.
Those paths may resolve qualifiers, follow imports and exports, inspect globals,
or evaluate expressions.

Use this level when raw structure is insufficient but the caller does not need
the rest of the module's inferred types. The query records only the module-graph
dependencies it reaches and does not materialize every inferred type in the
module.

For example, a targeted query for the type of `result` resolves the `identity`
signature, matches `T` to the argument's numeric literal type, and substitutes
that type into the return position:

```text
type of result = 1
```

Given the module and source range of the `result` binding, a caller performs
that lookup directly:

```rust
use biome_module_graph::{BindingTypeInput, infer_binding_type};

let input = BindingTypeInput::new(db, module, result_range);
let result_type = infer_binding_type(db, input);
```

`infer_expression_type` accepts an `ExpressionTypeInput` for the equivalent
expression lookup. Both queries return `None` when the requested range was not
collected or the module does not support inference. Normalize the returned type
only when the operation requires a normalized result; see
[Handles and Normalization](#handles-and-normalization).

The query does not infer unrelated declarations or expressions in the module.

### Complete Module Inference

Complete inference converts a module's raw type, expression, and binding tables
into `InferredModuleTypes`. Resolving imports may invoke complete inference for
other modules, so this layer can evaluate a cross-module dependency graph. Its
tracked result is reusable by other consumers that may need many related inferred values
or a normalized view of the module's types.

Use this level when the operation concerns the module as a whole or would
otherwise repeat many targeted queries. Do not use it merely because a single
raw reference is unresolved; a targeted query may be able to resolve that
reference without inferring unrelated declarations.

For the same example, complete inference evaluates every recorded binding and
expression in the module. Its result includes the resolved call alongside the
generic function that produced it:

```text
identity = Function(type_parameters: [T], parameter: T, return: T)
result   = 1
```

Targeted and complete inference produce the same type for `result`. Complete
inference additionally materializes the other inferred types so consumers can
reuse them as one module-wide result.

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

## Choosing an Inference Layer

Use the narrowest layer that can satisfy the operation's contract:

1. Raw local information
2. Targeted on-demand resolution
3. Complete module or cross-module inference

Choose based on the answer the operation promises, not on the representation it
happens to receive. Receiving a raw reference does not require staying at the
raw level, and needing one resolved value does not justify inferring the complete
module.

The `identity(1)` example therefore has three valid views: a deferred call at
the raw level, the type `1` from a targeted query, and a module-wide result that
contains both the inferred function and the type `1`. The required view depends
on the caller's question.

Raw traversal must preserve the module that owns each reference. Follow imported
references into their owning module rather than copying them or interpreting
them against the importing module's type table. Resolve the narrowest reference
that contains the required information. Avoid resolving its enclosing function,
call, object, or module unless that structure affects the result.

An inconclusive result does not automatically select the next level. The
caller's contract decides whether to escalate or preserve the result as
unknown.

### Escalation and Resolution Boundaries

Before implementing an inference operation, define:

1. The exact information it needs.
2. The narrowest layer capable of producing that information.
3. The conditions that require escalation.
4. The meaning of an inconclusive result.
5. The maximum scope that may be resolved.

Escalation only broadens the part of the type graph being inspected. It must not
change the meaning of the operation or turn missing information into a positive
or negative answer. Move to a broader layer only when the narrower layer cannot
satisfy the operation's contract.

If an operation handles argument-dependent generics, overloads, or compound
types, a targeted path must preserve those semantics. It must return an
inconclusive result or escalate when it cannot do so.

Before resolving a type, determine which declarations and expressions the
resolution may visit; whether parameters, arguments, members, or imports affect
the answer; and whether the operation needs one selected path or the complete
type. A resolution boundary is too broad when it traverses data that cannot
affect the requested result.

Use the following control flow for operations that can answer from more than one
inference layer:

```text
inspect raw structure
-> return a conclusive result when possible
-> resolve the smallest selected reference when computation is required
-> return an inconclusive result or escalate according to the caller's contract
```

The caller must explicitly define whether an inconclusive result triggers
broader inference or remains unknown.

## Handles and Normalization

Inference queries do not always return a fully expanded type tree. A result may
contain a **handle**: a compact ID that points to type data stored elsewhere.
`InferredTypeData::Local` contains a module key and a local type ID, so the pair
continues to identify the correct type after crossing an import boundary.
`GlobalType` similarly identifies a canonical global without copying its
definition into every result.

A local handle can be pictured as the following simplified value:

```text
Local {
    module: ModuleKey(17),
    type_id: LocalTypeId(3),
}
```

This means "type entry 3 owned by module 17." The handle identifies where the
type is stored; it does not contain the type definition itself.

**Normalization** is the operation that follows handles reachable from an
inferred type, and simplifies wrappers so a consumer can inspect the represented
structure instead of its storage indirections. It does not infer the complete
module or change the type's meaning. Types that contain no handles or
normalizable wrappers are returned unchanged.

For example, inference may defensively wrap a reference before it knows whether
the reference names a class or another type. Once the target resolves to
`Number`, normalization can remove the redundant instance wrappers:

```text
before: InstanceOf(InstanceOf(Number))
after:  Number
```

An `InstanceOf` wrapper remains when it distinguishes a class instance from the
class itself or carries generic arguments.

Consumers must call `normalize_type` or use an `InferredModuleTypes` lookup
before inspecting values that may contain handles. Normalization resolves local
handles, expands non-nominal globals, unwraps deferred `typeof` values, and
collapses structural wrappers. Cycles and exhausted walks degrade to `Unknown`.

For the `identity(1)` example, call inference performs the generic substitution
and infers the numeric literal type `1`. That type contains no handles or
wrappers, so normalization leaves it as `1`. Normalization does not perform the
generic substitution itself.

## Cycles

Salsa invokes `infer_module_types_cycle_result` when tracked module inference
encounters an import cycle. The fallback computes the root's strongly connected
component and resolves the requested module with `CycleFallback`. Imports that
would re-enter the component are suppressed, while imports outside it remain
tracked normally.

Raw self-references use local handles. During raw conversion, re-entering a type
currently being resolved returns its stable local handle; the outer resolution
then stores the completed value in the module table.

## Work Limits and Inconclusive Results

Walks over types and modules must terminate even when the input is recursive,
cyclic, or too large to inspect completely. Such walks use deterministic local
work limits. Reaching a limit is not evidence that a requested condition is true
or false; it means the operation could not finish reliably.

When inference cannot finish, it must preserve that uncertainty. The value used
to represent "could not determine" depends on the API's return type:

- An API that returns a type uses `Unknown`.
- A predicate that returns `Option<bool>` uses `Some(true)` when inference proves
  the predicate, `Some(false)` when it disproves the predicate, and `None` when
  neither answer is reliable. The predicate's name and documentation define
  what is being proved.
- An analyzer-facing classification request uses `TypeInferenceClassification::Indeterminate`.

Classification requests expose three possible results:

```text
Match         = the available information proves the condition
NoMatch       = the available information disproves the condition
Indeterminate = inference cannot reach a reliable conclusion
```

For a diagnostic that requires a positive match, handle the classification
explicitly:

```rust,ignore
match classification {
    TypeInferenceClassification::Match => report(),
    TypeInferenceClassification::NoMatch | TypeInferenceClassification::Indeterminate => {}
}
```

`Indeterminate` may represent unresolved or ambiguous data, a cycle, or an
exhausted work limit. Each request documents how it maps lower-level failures to
its public result. If an operation reaches its work limit, it may retry with a
broader inference layer only when the caller explicitly requires that behavior.
Otherwise, it returns an inconclusive result.

## Module Index Invalidation


Every dynamic module-path lookup must read `ModuleDb::module_graph_generation`.
Structural registry mutations acquire the pending Salsa setter before publishing
map changes and commit the new generation afterward. This prevents a reader from
observing a new map under an old generation.

## Testing

Test the narrowest public boundary affected by the change:

- Collector tests under `biome_js_type_info/tests` verify raw types produced
  from syntax. Their snapshots show module-local identities and deferred
  expressions, not resolved cross-module types.
- Query tests under `biome_module_graph/tests/spec_tests` verify targeted
  lookup, call inference, normalization, imports, cycles, and complete module
  inference. Place a test with the query family whose behavior changed.
- Request tests in `spec_tests/requests.test.rs` verify analyzer-facing
  contracts through `execute_type_inference_request`, including the distinction
  between `Match`, `NoMatch`, and `Indeterminate`.
- Type-aware analyzer fixtures verify the final diagnostic behavior after the
  request result reaches a lint rule.

A semantic test proves the returned type or classification. Add the affected
generic, overload, compound, import, ambiguity, or cycle shape only when the
changed path traverses it. Do not use complete-module inference merely to test a
targeted query.

When a change narrows or widens resolution scope, add query-event assertions for
the scope itself. Assert which tracked query runs and which unrelated query does
not run, especially for parameters, call arguments, imports, normalization, and
`infer_module_types`. Invalidation and backdating changes also assert behavior
after the relevant database input changes.
