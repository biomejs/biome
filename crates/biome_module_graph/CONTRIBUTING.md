# Contributing to type inference

This guide describes how the Salsa-backed JavaScript and TypeScript inference
engine in `biome_module_graph` is built, and how to extend it while keeping
incremental reuse and making inference work observable.

Two other documents complete it:

- [`../biome_js_type_info/CONTRIBUTING.md`](../biome_js_type_info/CONTRIBUTING.md)
  describes the representation of types, collection from syntax, handles, and
  normalization.
- [`NARROWING.md`](./NARROWING.md) describes what narrowing does: what
  narrows, the correctness contract, the deliberate differences from
  TypeScript, and the limits. This guide describes how narrowing is built.

## Architecture

Type inference has five layers:

```text
syntax and semantic collection
    -> raw module tables
    -> analyzer-facing requests
    -> tracked Salsa queries
    -> resolver helpers
```

**Collection** runs during a module-graph scan. `resolve_js_module` processes
each parsed JavaScript or TypeScript file with
`TypeInferenceMode::RawTypesOnly`, and `JsModuleInfoCollector` uses the syntax
tree and the semantic model to record the module's raw tables:

- `raw_types`: declaration and structural type data owned by the module;
- `raw_expressions`: expression ranges mapped to type references;
- `raw_binding_types`: binding ranges mapped to type references.

A raw reference identifies another entry in the module table, a global type,
or a name or import that still needs resolution. It is not an inferred type.
When type inference is enabled, collection also records the module's flow,
the syntax-only facts that narrowing needs (see
[Flow-sensitive expression reads](#flow-sensitive-expression-reads)).

**Requests** are the analyzer-facing operations. A lint rule calls a
`TypedService` method, which executes one request. The request composes
tracked queries and returns one defined result, such as a normalized
expression type or the expected type of a call argument (see
[Request architecture](#request-architecture)).

**Tracked queries** are the units of incremental computation. A lookup query
selects the raw reference for a requested range or table entry, and
`ResolutionCtx` converts it to `InferredTypeData`. It follows module-table
references, resolves globals, and uses export and lookup queries for imports
(see [Adding a tracked query](#adding-a-tracked-query)).

**Resolver helpers** evaluate references and inferred structures inside a
query. Named declarations remain symbolic `LocalTypeHandle` values until a
consumer normalizes or inspects their structure. In this code, "local" means
owned by one module's type table; it does not mean block- or function-scoped
source code (see [Resolver helpers](#resolver-helpers)).

### Where the code lives

| Concern | Location |
| --- | --- |
| Module-graph scan and module resolution | `src/module_graph.rs` |
| Raw tables | `src/js_module_info/collector.rs` |
| Flow collection | `src/js_module_info/flow.rs` and `src/js_module_info/flow/` |
| Requests | `src/type_inference/requests/`, with the traits in `src/type_inference/request.rs` and the operations available to requests in `src/type_inference/context.rs` |
| Query families and profiling | `src/type_inference/registry.rs`, `src/type_inference/profiling.rs` |
| Tracked queries | `src/db/queries/type_inference.rs` and `src/db/queries/type_inference/` |
| Compound query keys | `src/db/queries/type_inference/interned.rs` |
| Resolution context | `src/db/type_inference/resolver.rs` |
| Expressions | `src/db/type_inference/expressions.rs` |
| Imports and the deep-import fallback | `src/db/type_inference/imports.rs` |
| Literal widening | `src/db/type_inference/widening.rs` |
| Narrowing | `src/db/type_inference/flow.rs`, `flow_expressions.rs`, and `flow_signatures.rs` |
| Analyzer service | `../biome_js_analyze/src/services/typed.rs` |
| Tests | `tests/spec_tests/`, one file per topic |

### Declarations that other modules extend

Some declarations merge interfaces into declarations that their module
doesn't own: module augmentations (`declare module "./a" { ... }`),
`declare global` blocks, and the top-level interfaces of declaration files
without imports or exports. TypeScript applies them to the whole program.
Collection records them in `JsModuleInfoInner::contributions` and makes their
names refer to the declaration that they merge into.

The tracked queries in `src/db/queries/type_inference/augmentations.rs`
index them across every indexed module: `module_augmentations` per module,
`augmentation_index` over the graph, and projections for one target module
or one global, which backdate unless their own entry changes. None of them
resolves a type, so they never join an inference cycle.

`ResolutionCtx::resolve_raw_type_id` adds a declaration's contributions to
its data as handles, and `resolve_global_type` adds a global's through
`TypeDb::global_contributions`. Nothing else builds a declaration's data, so
a handle to it has the same data in every context. Data built without its
contributions would make `instanceof` see two declarations. A contribution's
members are resolved only by whoever resolves its handle, in its own module.

### Literal widening

Resolution widens fresh literal types where TypeScript does, as in
`let n = 1`, which declares `n` as `number`
(`src/db/type_inference/widening.rs`). Collection marks the locations that
widen with a `TypeofExpression::WidenedLiteral`
([Fresh literal types](../biome_js_type_info/CONTRIBUTING.md#fresh-literal-types)).
Resolution tells a fresh literal type from a
declared one by the raw expressions behind the value: a literal, the operands
of a conditional or logical expression, the initializer of a `const` declared
without an annotation, and a member of an enum read as `E.A` or `E["A"]`,
also in another module, found with `find_export_origin` without resolving a
type. How Biome widens an enum member is in the
[Enums limits](./NARROWING.md#enums).

The properties of an object literal are widened where the object is
resolved. A call keeps their literal types to select an overload, as
TypeScript's contextual typing does, and infers generic parameters from the
widened type (`ResolvedCallArgument::WidenedArgument`).

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
| Execution root | A body that narrowing analyzes on its own, with its own flow graph |
| Subject | What a runtime test checks and a read reads: a variable, `this`, a property path that starts at one, or a destructuring pattern |
| Guard | A condition decomposed into runtime tests on subjects (`FlowGuard`) |
| Candidate | A read, or an expression around one, whose type narrowing may change |
| Baseline | The type that narrowing starts from for one subject |
| Solve | Applying the guards and writes of one root's graph to one subject |
| Call point | A call whose effect on the flow depends on its callee's signature |

"On demand" describes the normal import strategy, not a constant-work
guarantee. A request can still traverse imports, recursively normalize a
large type, or expand a namespace.

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

`TypedService` converts syntax nodes and semantic bindings into request
inputs. It doesn't orchestrate raw query calls.

### Adding a request

Before adding a request, search the existing implementations of
`TypeInferenceRequestMetadata`. Reuse a request when it has the same output
and uncertainty behavior. Caller identity, syntax wrappers, input extraction,
and the exact source origin do not create request boundaries.

Add a request only for a new result contract. State the result and each form
of uncertainty precisely, and give the request one canonical `execute`
implementation. Return `TypeInferenceClassification` for a tri-state
classification, never `Option<bool>`, so matches, conclusive non-matches, and
indeterminate results remain explicit. If only input extraction differs,
reuse the request and share an input helper.

Add the request to the matching subject file under
`src/type_inference/requests/`. Implement `Sealed`,
`TypeInferenceRequestMetadata`, and `TypeInferenceRequest`;
`NormalizedExpressionTypeRequest` and `ExpectedCallArgumentTypeRequest` are
complete examples. Then:

1. re-export the request from `requests/mod.rs` and `type_inference/mod.rs`;
2. add a thin `TypedService` method that constructs and executes it;
3. add a flow test that asserts the request uses the intended queries;
4. verify that the flow does not execute whole-module inference unless its
   contract requires it.

A request is a reusable inference operation, and a lint rule is one of its
callers, so a lint rule never gets a request of its own.

### Request metadata

Like an analyzer rule's metadata trait, `TypeInferenceRequestMetadata` is
implemented by each request type and required by `TypeInferenceRequest`.

Request IDs are stable identities used to aggregate records for the same
request contract. Use lowercase words separated by hyphens after the
`request.` prefix:

```text
request.expected-call-argument-type
```

Labels are concise human names for the result. IDs and labels never contain
source text, symbol names, inferred-type strings, import specifiers, or paths.
The request's rustdoc explains its output, uncertainty, and widening behavior.

## Adding a tracked query

A request usually composes existing queries. Add a tracked query only when the
result needs an independently memoized invalidation boundary.

Expression, binding, module type, and export queries each resolve one entry
from a raw table. Normalization, call matching, and whole-module inference
have separate query boundaries.

The query facade is `src/db/queries/type_inference.rs`. Files under
`src/db/queries/type_inference/` are organized by subject: `lookups`,
`exports`, `augmentations`, `promises`, `calls`, `normalization`, and
`module_types`. Compound Salsa keys live in the `interned` module. Algorithms
used by one family may live in a child module; resolution helpers shared by
several families live under `src/db/type_inference/`.

### Query file structure

Within a subject file, keep tracked queries, cycle callbacks, and ordinary
helpers in separate regions, in that order. A query region contains only
`#[salsa::tracked]` functions.

### Query acceptance criteria

A new query satisfies all of these criteria:

1. **Smallest semantic result**: it returns no more information than its
   consumer needs. Prefer a classification or projection query over a
   normalized type when the consumer asks one property.
2. **Two-parameter signature**: the database is the first parameter and one
   Salsa input or interned value is the second. A query that needs several
   logical values takes a new type under `interned` that encloses them.
   Syntax nodes are never query parameters.
3. **Minimal dependencies**: the query reads only inputs needed for its
   result. A Salsa execution test proves that an unrelated edit reuses the
   result and an edit to a consumed input recomputes it.
4. **Defined uncertainty**: missing input, unsupported shapes, ambiguity,
   cycles, and exhausted budgets have documented behavior.
5. **Cycle behavior**: a recursive query defines a `cycle_result` callback.
   The callback either runs an explicit recovery algorithm, such as the
   whole-module SCC fallback, or preserves uncertainty. Type queries return
   `Unknown`; classification queries return `Indeterminate`. Neither invents a
   definite type or match result.
6. **Bounded traversal**: recursive shape, overload, import, and export walks
   have an explicit work budget. Incomplete work never becomes a definite
   negative: the query returns the request's documented indeterminate result
   unless the helper documents partial-result semantics.
7. **Observable boundary**: the query belongs to one registered query family
   and has one implementation reference. Its body runs inside
   `execute_query`, which takes the family and the query's name.
8. **Independent validation**: tests prove both correctness and selective
   Salsa execution.

Interning an input gives equal values a shared identity. It does not memoize
the query result by itself.

### Query families

`TypeInferenceQueryKind` identifies the subject module that owns a tracked
query: lookups, exports, promises, calls, normalization, or module types. The
augmentation queries belong to the exports family. Add a query to its
existing family without adding an enum variant. Add a family only when the
query needs a new subject module with a distinct responsibility.

### Graph discovery and type resolution

Keep graph-only export discovery separate from type resolution. Origin and
namespace-name queries inspect module graph inputs without caching
provisional inferred types. The owning expression, binding, or export can
then be resolved with a lookup query. This separation keeps graph discovery
out of inference cycles.

### Tracked and untracked entry points

Tracked queries may call other tracked queries directly. A new tracked query
never calls the public bottom-up scheduler. The guarded deep-import fallback
in `imports.rs` is the existing exception.

Code outside a tracked query may use bottom-up scheduling when it needs
whole-module inference for an import graph. The scheduler is untracked
because it prepares dependencies iteratively and then invokes the tracked
`infer_module_types` query.

## Resolver helpers

Cross-module relationships remain references to data owned by the source
module. Inferred data from another module is never copied or cloned into the
current module, including behind `Arc`.

`ResolutionCtx` protects itself against endless recursion in two ways: it
remembers which types it is currently resolving, and it gives up after a
fixed number of nested steps. It also remembers the types it has already
resolved. All of this is stored inside one context, so a newly created
context starts with none of it.

For this reason, never create a new context in the middle of a resolution and
keep resolving through it. The new context does not know what the old one was
working on. Two declarations that refer to each other can then keep resolving
each other until the program runs out of stack space and crashes.

Create a new context only where something outside the context still limits
the work:

- at the start of a tracked query. If the same query is requested again
  before it finishes, Salsa returns the query's fallback result instead of
  running it again;
- when following an import, because each import uses up part of a limited
  import budget;
- when sharing the declaration evaluator. It keeps one record of which
  declarations depend on which, across all modules, and uses it to detect
  loops;
- when looking up a member of a namespace import, because that lookup has its
  own step limit that decreases with each step.

To resolve a type that belongs to another module, use a helper that follows
one of these paths, such as `ResolutionCtx::resolve_foreign_type_id`. For
example, in these two files each interface's type parameter refers to the
other file's interface:

```ts
// a.ts
import type { B } from "./b";
export interface A<T extends B<any>> {}

// b.ts
import type { A } from "./a";
export interface B<U extends A<any>> {}
```

Resolving `A`'s type parameter requires `B`, and resolving `B`'s type
parameter requires `A`. If each step creates a new context, no context
notices the loop.

## Work that widens inference

Review every new request and query for the following transitions.

### Recursive normalization

Normalization rewrites a type into the directly inspectable form that a
consumer requires. It resolves module type handles throughout nested unions,
intersections, tuples, instances, and `typeof` wrappers, then removes
wrappers that add no information. It preserves identities such as classes
and interfaces when later operations depend on them.

Prefer a targeted classifier to normalization when it can produce the
required result. Analyzer convenience methods state whether they normalize
their result. Review the entire request flow, not only its first lookup
query.

### Namespace expansion

A namespace import or re-export may discover and resolve every visible
export, including exports reached through blanket re-exports. Treat namespace
work as proportional to the visible namespace, not as an individual lookup.

### Deep import fallback

On-demand import resolution has a recursion limit. Crossing the limit invokes
bottom-up whole-module inference to avoid consuming the Rust stack. A request
that normally stays lookup-oriented can therefore become graph-wide on a
sufficiently deep chain. The transition is implemented in
`src/db/type_inference/imports.rs`.

### Whole-module APIs

`infer_module_types` resolves every raw type-table entry, expression, and
binding in a module. A new request calls it only when its contract requires
all three tables.

`infer_module_types_bottom_up` additionally prepares static imports and
re-exports dependency-first. A tracked query never calls it.

### Cycle recovery

Lookup query cycles return `Unknown` for type results or `Indeterminate` for
classifications. Whole-module cycles use an SCC fallback that blocks members
of the active component while preserving resolvable dependencies outside it.

Raw declaration recursion is different from a Salsa query cycle. Recursive
declarations keep their symbolic module type handles rather than collapsing
eagerly to `Unknown`.

### Call inference

Expected-argument inference may resolve the callee and every sibling argument
before matching signatures. Count this as one request with several tracked
query executions, not as a single constant-cost projection.

Return-type inference resolves the object that a method is read from, as
`xs` in `xs.filter(isString)`, only when a type parameter's constraint names
a type parameter of the method's class or interface (`CallReceiver`),
because resolving an object such as a namespace import can build its whole
export list. The signature that a call point selects resolves the types of
the call's arguments in the reading context, but only for callees with a
generic or overloaded signature that can narrow or end the path.

## Result semantics

A classification request returns `TypeInferenceClassification`, never
`Option<bool>`:

- `Match` means available type information proves the requested condition.
- `NoMatch` means available type information proves the condition does not
  hold.
- `Indeterminate` means inference cannot decide because information is
  missing, unsupported, ambiguous, cyclic, or beyond a work budget.

`None` is appropriate when a request has no result to return and that absence
has one documented meaning. `Unknown` is an inferred type value whose
structure cannot be resolved. A missing raw entry means that collection did
not produce the requested key. Ambiguous exports remain ambiguous rather than
selecting the first match.

Budget exhaustion follows the documented request or helper behavior and is
never a conclusive negative result. Rustdoc on a public request or query
distinguishes every result it can return. The type-info guide describes how
lower-level APIs represent the same outcomes
([Work limits and inconclusive results](../biome_js_type_info/CONTRIBUTING.md#work-limits-and-inconclusive-results)).

## Source and implementation references

Every request has a `TypeInferenceCodeReference`. It records the source file,
line, and symbol adjacent to the canonical `execute` method. Inference
profiles use this reference to lead maintainers from a reported source range
to the implementation that orchestrated the work.

Each recorded query execution retains its own implementation reference, so
several query bodies can share one family. Whole-module records retain the
implementation responsible for widening inference to complete tables. Code
references correspond to the Biome version that produced a report.

Never read `ModuleInfo::path` from inside a tracked query for profiling. The
path is a Salsa input, so reading it changes the query's dependency set. The
outer request boundary snapshots document paths before entering tracked
queries. Query instrumentation records only work nested under a production
request and carries only module identities and `TextRange` values.

## Flow-sensitive expression reads

An expression lookup can give a variable a more specific type at one read
than its declared type. After `if (value === null) return`, a variable typed
`string | null` reads as `string`. The type stored for the variable's
declaration doesn't change.

What narrows, and where Biome deliberately differs from TypeScript, is in
[`NARROWING.md`](./NARROWING.md). A change to what narrows updates it in the
same pull request. This section describes how narrowing is built.

### Overview

Narrowing is split like the rest of inference: collection gathers
syntax-only facts, and tracked queries do everything that needs a resolved
type.

Collection builds a graph of the possible paths through each execution root
that narrows something: a module, a namespace, a function, a method, an
accessor, or a constructor. Inline code, a function or arrow expression called
where it is defined or a class static block, runs in the flow of the root
around it, so it is part of that root's graph. A read in a closure continues
the flow of the root that creates the closure from the point where it creates
it, its capture point. Any other read never sees facts from an enclosing
root. Biome uses its semantic model to tell variables with the same name
apart.

The graph (`RootFlow`) has four kinds of nodes (`FlowNode`): the start,
joins, conditions, and writes. A condition holds a guard (`FlowGuard`), the
condition decomposed into runtime tests on subjects. A write holds the value
that it assigns. Collection records the incoming flow point of every read
that a runtime test, a write, or a call point can reach. Those reads, and the
expressions around them that can use a narrowed operand, are candidates; the
module's flow (`ModuleFlow`) lists them. A lookup skips flow for any other
expression.

For a candidate read, inference resolves the subject's baseline and every
other type that the solve needs, in the context that reads the subject. A
tracked query then solves the root's graph for that subject: it applies the
guards and writes along every path that reaches the read, and keeps every
type still possible where paths join.

### Contracts

These rules implement the
[correctness contract](./NARROWING.md#correctness-contract), and every change
keeps them:

- **Never narrower than TypeScript.** When collection can't tell what
  TypeScript does with a construct, it leaves the subject as it is: the root
  gets no graph, the test becomes `FlowGuard::Keep`, the write assigns no
  value, or the variable isn't eligible. Inference does the same by keeping
  the declared type. Each known read that is narrower than TypeScript's is a
  deliberate choice in `NARROWING.md`, listed with the differences from
  TypeScript; most are in code that TypeScript types as impossible, finds
  unreachable, or reports as an error.
- **Collection resolves no type.** It reads the syntax, the semantic model,
  and the module's own raw tables, through the `raw_may_*` functions of
  `biome_js_type_info`. Anything that depends on another module or on a
  resolved type is decided by inference.
- **The solve's key carries what the read resolved.** The context that reads
  a subject resolves the baseline, the assigned and compared types, the
  effects of call points, and the answers about readonly paths, exhaustive
  `switch` statements, and calls that never return. The solve's key carries
  them ([Solve keys](#solve-keys)), so a solve resolves no declared type,
  assigned value, or callee. It resolves only the operands of its own tests,
  such as the constructor of `instanceof` or the target of a type predicate.
- **Inputs are typed without narrowing.** Assigned and compared types are the
  types of their values without flow, so none of them depends on a solve.
- **Lookups and complete module inference agree.** Complete module inference
  narrows each read as an individual expression lookup does, from the types
  that its own context resolved.
- **Impossible is not unreachable.** A read on a path that no value can take
  has type Unknown, not `never`. Reachability is a separate question that
  `UnreachableCodeClassificationRequest` answers.
- **Limits degrade, never guess.** A root over a collection limit keeps its
  declared types; a solve over its limit or in a query cycle gives Unknown
  ([Work limits](#work-limits)).

### Where the flow code lives

| Concern | Code |
| --- | --- |
| Selecting execution roots | `src/js_module_info/flow/scanner.rs` |
| Building a root's graph: evaluation order, control flow, write nodes, inline code | `src/js_module_info/flow/builder.rs` |
| Subjects, property keys, `this`, and destructuring patterns | `src/js_module_info/flow/subjects.rs` |
| Decomposing conditions into guards, and call points | `src/js_module_info/flow/guards.rs` |
| Writes that the graph doesn't model | `src/js_module_info/flow/writes.rs` |
| Relevance, candidates, subject classes, closures, `ModuleFlow`, and `RootFlow` | `src/js_module_info/flow.rs` |
| Flow queries and their cycle results | `src/db/queries/type_inference/lookups.rs` |
| Solve keys (`FlowSubjectRootInput`) | `src/db/queries/type_inference/interned.rs` |
| Baselines, the solve, call effects, reachability, and inferred type predicates | `src/db/type_inference/flow.rs` |
| Expressions that use narrowed operands | `src/db/type_inference/flow_expressions.rs` |
| Signature selection for call points | `src/db/type_inference/flow_signatures.rs` |
| Narrowing operations on types and the raw checks | `../biome_js_type_info/src/narrowing.rs` |
| Type relations for `in`, `instanceof`, and type predicates | `../biome_js_type_info/src/type_relation.rs` |

The rustdoc of these modules and of the types they define describes each
part in detail; this section gives the structure and the rules between
parts.

### Collection

#### Selecting roots

`FlowRootScanner` selects roots while the module visitor walks the syntax
tree, so collection never walks the whole module again. A root is selected
when its own code contains:

- a condition, counting every `switch` statement, every `for...in` loop,
  every logical assignment, and every `?.` that a key or arguments follow in
  its chain;
- a write: an assignment, `++`, `--`, `delete`, a declarator with an
  initializer whose type annotation is a union or that destructures a
  pattern, the head of a `for...of` loop that assigns a target, a call of
  `push` or `unshift` on a variable when the root also declares a variable
  without a type annotation initialized with `[]`, a declarator of a `let`
  or a `var` without a type annotation and without an initializer, which is
  `undefined` until a write, or of a `var` without a type annotation
  initialized with `null`, `undefined`, or `[]`, which is `undefined` before
  the declarator runs (`is_evolving_var`);
- a statement call that may assert (`is_statement_call`, `may_assert`): the
  callee is a dotted name, and the call passes an argument or calls a
  method. A root that only such calls select is built only when one of their
  callees can assert as far as the semantic model tells
  (`GuardFunctions::keeps_statement_call`).

The scanner also records whether the root's own code uses `this`, so a root
whose only subjects are paths on `this` still gets a graph. It rejects a root
when its own code contains a `with` statement or bogus syntax, and a root
over its node or depth limit. Declaration files (`.d.ts`, `.d.mts`, `.d.cts`,
in any letter case) contain no code that runs, so the visitor doesn't look
for roots in them.

Inline code opens no root (`AnyInlineCode::runs_inline`, which the builder
and the closure code share): its conditions, writes, statement calls, and
bindings belong to the root around it. The exception is inline code where
that root's builder doesn't evaluate it: in a class member other than a
static block, a decorator, a computed key of a method or accessor, a type, an
ambient declaration, or an export clause that isn't a statement. Inline code
directly in a root is also selected as a fallback (`SelectedRoot::enclosing`),
which collection builds as a root of its own only when the root around it
isn't built, so its own tests still narrow its reads.

#### Building the graph

`builder.rs` builds the graph of one selected root from start, join,
condition, and write nodes. It records the incoming flow point of every
reachable read of a subject, `this` included, and, for each condition node,
the subjects that its guard may narrow. A read of a property path is recorded
after its object and key are evaluated.

**Evaluation order.** The builder evaluates expressions in the order in
which JavaScript runs them, with the conditions that TypeScript's binder
adds:

- the key and the arguments after a `?.` start from a condition that its
  object isn't nullish;
- the right operand of `&&=`, `||=`, or `??=` starts from a test of its
  target;
- a function evaluates its parameters, in order, before its body;
- a default of a parameter or of a declaration's pattern runs only when the
  value is `undefined`, so the flow after it joins the flow before it.

Two orders follow TypeScript rather than JavaScript, and `NARROWING.md` lists
both among the
[unsound results that Biome copies](./NARROWING.md#typescripts-unsound-results-that-biome-copies):
a declaration evaluates its pattern before its initializer, and a
destructuring assignment its pattern, with unconditional defaults, before its
value; an optional call of inline code runs the body before its arguments.

**Paths that no value takes.** A branch that a condition written as the
literal `true` or `false` never takes has no flow (`never_has_outcome`).
Neither does the code after a statement call of a local function that never
returns (`GuardFunctions::local_call_ends`), because its return type
annotation is `never`, in place or through the module's type aliases
(`annotated_return`). Reads there are not recorded, so
they keep their declared types, and such a path adds nothing where it joins
the others. The syntax that counts is in
[Statements and calls](./NARROWING.md#statements-and-calls).

**Write nodes.** Each write of a subject adds a write node (`FlowWrite`,
whose `FlowWriteEffect` says how it changes the subject, with the assigned
value in `FlowAssigned`). The node also joins the innermost exception join,
so the write reaches the catch clauses around it. `BuiltRoot::written` lists
the targets with a write node. The writes that count are in
[Writes](./NARROWING.md#writes). A `var` declarator with an initializer and
without a type annotation writes its variables without a value, because a
read can reach them before the declarator.

**Assigned values.** A write assigns no value, so the subject gets its
declared type there, or `Unknown` for a variable whose type evolves with its
writes, when Biome may type the value narrower than TypeScript
(`value_callees`) or the value reads a variable whose type evolves with its
writes (`reads_evolving`). A call of a local function without a return type
annotation (`inferred_local_body`) counts as typed, as in TypeScript, when
each value that the function returns passes the same checks, through at most
four nested functions, because Biome infers its return type with the
`undefined` of an end that a path reaches. The value of an assignment with
`=`, `??=`, `||=`, or `&&=` to a variable whose type evolves with its writes,
in a scope other than its declaration, is covered instead
(`FlowAssigned::covered`, as `is_covered` decides) when an inferred return
type is the only reason Biome may type it narrower: the collector's declared
type joins those values (`widen_binding_from_writable_references`), so the
write gives Biome's type for the value. A value that a local function returns
and that reads a variable whose type evolves keeps the write uncovered. A
callee whose annotation the
syntax can't show, an import or a method (`FlowCallee`), is kept in
`FlowAssigned::callees`, and the reading context confirms it
(`ResolutionCtx::has_annotated_return`): an import by the declaration that it
names, and a method by every type of its object, which must get the method
from the lib, from a declaration file, or from a class or interface that
declares it with a return type annotation (`member_return`, which reads the
declaration that the type's scope records). A declaration file declares only
annotated functions, but a variable or a property that it types as `typeof`
of a variable returns what that variable returns, so the reading context
follows the variable (`declared_return`, `declared_member_returns`,
`followed_return_is_annotated`).

**Variables whose type evolves.** A `let` or `var` without a type annotation
whose initializer is missing, `null`, or `undefined` takes the types of the
values assigned to it, and a variable without a type annotation initialized
with `[]` the types of the elements added to it (`FlowEvolving`;
`evolving_declaration` selects such variables). A declarator of such a
variable writes its initializer, with `FlowWriteEffect::EmptyArray` for `[]`,
and `BuiltRoot::initializers` lists those write nodes. For an evolving array,
each argument of a call of `push` or `unshift` on it (`mutated_array`), after
the arguments run, and the value of an element assignment with `=`
(`element_write`) add a `FlowWriteEffect::Elements` write node, and an
assignment of `[]` an `EmptyArray` one. `WrittenTargets::mutations` lists the
variables that they add to.

**Optional chains.** The code after an optional chain continues from the
flow after its root, unless a key or an argument has a write node; then each
`?.`'s short circuit joins the chain's end. A `?.` whose object the raw tables
show can't be nullish (`raw_may_be_nullish`), such as a variable that holds
an object literal, adds no condition, so it doesn't make the object a tested
subject.

**Classes and declarations.** The builder skips class members other than
static blocks: methods, accessors, and constructors are roots of their own,
and a class field initializer is analyzed apart from the code around the
class. A static block runs where its class is evaluated, after its `extends`
clause and in member order. The builder also skips `declare` statements,
which never run.

**Inline code.** A call of inline code evaluates its arguments, then the
function's parameters and body (`inline_body`), with jump targets and finally
blocks of its own and a return join that each `return` reaches. The code
after the call continues from the end of the body joined with the returns. A
`throw` and each write point reach the exception joins of the `try`
statements around the call. When no path leaves the inline code, the flow
after it is a join without predecessors, which ends the path at the end of
the statement.

**Closures.** The builder records each closure that it skips in reachable
code and that continues the flow where it is created (`AnyContinuingClosure`:
a function or arrow expression that isn't inline code, or a method or
accessor of an object literal or of a class expression) with its capture
point (`BuiltRoot::captures`). A closure inside inline code takes its capture
point from the flow where the inline code runs.

#### Control flow

The builder models blocks, `if`, `while`, `do...while`, `for`, `for...in`,
`for...of`, `switch`, `try`, labeled statements, `break`, `continue`,
`return`, and `throw`, as TypeScript's binder does.

- **`for...in` and `for...of`.** The loop evaluates its expression once,
  before the loop. Its header, where each iteration starts, joins the flow
  after the expression with the end of the body and each `continue`. Each
  iteration evaluates the head from the header, such as the defaults of a
  destructuring pattern, then writes an assignment target of the head. The
  header is also the exit, so the code after the loop continues from it and
  from each `break`, and a test in the body never reaches it. A `let` or a
  `const` that the head declares gets no write node: the header joins the
  entry, where the variable has its declared type, so the facts that the end
  of the body carries back never make a read narrower than that type. A `var`
  is hoisted, so code before the loop can test or write it, and each
  iteration writes it without a value.
- **`switch`.** Every case test is evaluated from the flow after the
  discriminant. The clauses form groups, a run of clauses without statements
  followed by the next clause. A group's statements start from the join of a
  condition for each of its cases, a condition that no case matched when the
  group contains `default`, and the fall-through from the group before.
  Without `default`, the code after the statement also continues from the
  bypass, the condition that no case matched. `switch (true)` chains the
  conditions instead: a group starts where every case before it failed to
  match.
- **`try`.** A catch clause starts from the join that the exceptions of its
  try block reach. An exception can leave the try block at any point, and
  conditions only narrow, so no point inside the block is narrower than its
  start or one of its write nodes, which the join holds. The builder keeps
  such a join for each enclosing try block and catch clause. The code after
  the statement continues from the ends of the try block and the catch
  clause, and `break` and `continue` reach their targets directly. A finally
  block is built twice. The first build starts from the join of those ends,
  the exceptions, and each `return` in the try block and the catch clause, and
  records the reads of the block. The second build starts from the ends only
  and records no reads, and the code after the statement continues from it,
  because that code runs only when the statement completed normally.
- **Opaque regions.** Any other statement, such as a class or enum
  declaration, is an opaque region. The builder evaluates the expressions of
  a region from the flow point before it, as it evaluates any expression, and
  the next statement continues from the result. The target of a `break` or
  `continue` that leaves a region joins from the point before the region,
  which is never narrower than a point inside it while the region has no
  write node; a region with both gets no graph.

#### Subjects

`subjects.rs` maps an expression to the subject that it reads or tests
(`subject_of`): a variable or `this`, or a property path that starts at one,
such as `o.v`, `a.b.c`, `t[0]`, or `this.#v` (`FlowSubject`). A static member
access with a named property or a private name extends a path, and so does
an element access whose key spells its property's name; the accepted keys
are in [Property paths](./NARROWING.md#property-paths). JavaScript converts a
number key to a string, and an accepted integer converts to its own digits,
so `t[0]` is the same subject as `t["0"]`. A private name gets a key of its
own (`PropertyKey`), so `o.#v` is never the same subject as `o["#v"]`.
Parentheses are skipped, and `o?.v` is the same subject as `o.v`.

Each root stores its subjects in a table, a path after its prefixes, so the
builder and the decomposer give a read and a test of the same subject the
same id.

`this` is a subject only in a root where `this_can_narrow` holds: where Biome
types `this` as TypeScript does and TypeScript narrows its properties as
those of a variable ([`this` limits](./NARROWING.md#this)). Each root has its
own `this`, so a test of `this.v` never reaches a read in a nested function.
In a constructor, and in an arrow function that it calls where it is
defined, a path on `this` whose field has no initializer may be read before
it is assigned (`uninitialized_fields`).

A destructuring pattern is a subject (`FlowSubject::Pattern`) that stands for
the value that it destructures, so a test of one element can narrow the
others. `narrowing_pattern` accepts a pattern that a `const` or a parameter
whose variables are never written declares, with at least two direct elements
that have no default and are not rest elements. The subject table records
those elements with their keys when it first stores the pattern.

#### Decomposing conditions

`guards.rs` decomposes every condition into a guard on subjects, combined the
way `!`, `&&`, and `||` combine them (`FlowGuard::Both`). Its leaves are:

- `FlowGuard::Test`: a runtime test (`FlowTest`) of one subject, such as
  `typeof value === "string"` on `value` or `o.v !== null` on `o.v`;
- `FlowGuard::Property`: a property guard, a test of a property that narrows
  the object that holds it, as `s.kind === "c"` narrows `s`;
- `FlowGuard::Call`: a call point, whose effect inference decides;
- `FlowGuard::Keep`: a step that narrows nothing;
- `FlowGuard::Incomplete`: decomposition stopped at its work limit or at
  missing syntax, so the result is unknown rather than assumed unaffected.

Three wrappers make a guard depend on an answer that inference finds at the
read: `FlowGuard::ReadonlyPath`, `FlowGuard::Unmatched`, and
`FlowGuard::Bypass`, described below.

The decomposed condition (`DecomposedCondition`) also lists the subjects that
the guard may narrow, in groups that decide the class of each subject in the
root ([Relevance and candidates](#relevance-and-candidates)):

- `mentions`: the subjects that any test narrows;
- `parents`: the subjects that only property guards, `in`, `#p in x`,
  `instanceof`, a comparison of `constructor`, or a type-guard function whose
  target has only object types narrow, because those tests narrow only a type
  with object types;
- `instance_tested`: the subjects in `parents` that a test other than a
  property guard checks;
- `call_mentions`: the subjects that only call points may narrow.

The behavior of each kind of test is in
[Runtime tests](./NARROWING.md#runtime-tests) and the topics of
[Limits](./NARROWING.md#limits). The implementation rules are:

- **Comparisons.** A comparison, strict or loose, tests one operand by the
  other when the other is a unit value (`unit_value`), which the decomposer
  follows through `const` declarations with the semantic model. The test
  names that literal by its range, so inference resolves it as a literal
  written in the comparison, and records whether the comparison is strict:
  loose equality narrows by `NarrowingPredicate::LooseLiteral`. A member of
  an enum that the module declares is a test of the member's name
  (`FlowTest::EnumMember`), and inference reads the member's collected type,
  the literal type of its value when collection computes the value, and
  narrows nothing otherwise. A value that a named or default import binds, or
  a member of one, is a test of the import's variable (`FlowTest::Imported`,
  with the range of the member's name). Collection can't see the module that
  declares it, so inference follows the import with `find_export_origin` and
  applies the same rules to the declaration there, reading syntax and raw
  types without resolving a type.
- **Comparisons by declared type.** When neither operand is a unit value,
  each operand is tested by the declared type of the other
  (`FlowTest::Compared`, `typed_comparison_step`), but only when TypeScript
  gives the other operand that type too (`has_declared_type`). An import that
  holds no unit value narrows the same way when the declaration that it names
  has an explicit type, which inference checks. Other values give no test,
  because Biome may infer their types narrower than TypeScript does, and a
  subject compared with itself gives `Keep`. Where the operands are equal,
  each receiver of an optional chain around the tested operand gets a
  `FlowTest::ComparedReceiver` test, which applies only when the value's
  declared type excludes `undefined`.
- **Comparisons with `true` and `false`.** A comparison of an expression
  that is not a subject with `true` or `false` decomposes the expression as
  truthy or falsy (`boolean_comparison_step`), and a comparison of an aliased
  condition with one also inlines its initializer. Each branch assumes the
  expression truthy or falsy only when its values allow it
  (`BooleanAssumption`, `is_boolean_when`). An expression that, without
  parentheses and `!`, is `&&`, `||`, or `??` decomposes into nothing.
- **Optional chains.** When the tested operand is part of an optional chain
  and the test's result shows that the operand isn't `undefined`, the guard
  also tests each receiver of the chain that is a subject as not nullish,
  combined with the main test as `&&` combines its operands. Which branches
  narrow the receivers follows TypeScript: neither outcome of `??` does, for
  example. Parentheses end a chain, and a test of a chain that starts at a
  parenthesized optional chain narrows nothing, because TypeScript narrows
  the inner path there but not its object, and a narrowed path in Biome would
  keep its narrowed type at a later read of the inner chain.
- **Local type-guard functions.** A call to a local type-guard function
  becomes a runtime test of its argument (`FlowTest::Predicate`). Collection
  selects the tested argument from the function's collected signature, and
  inference later resolves only the target of its type predicate; neither
  step infers the function's body. Collection also tells from the raw tables,
  and from the declarations that the target's names refer to, whether every
  member of the target is an object type. Such a test narrows only a type
  with an object type, `unknown`, or `any`, so the decomposed condition lists
  the argument with the objects of property guards, unless its annotation is
  `any`. An imported name can be an alias of a primitive, so it doesn't count
  as an object type, and a target of `any` or `unknown` makes the function
  unsupported.
- **Statement calls and assertions.** After each statement call, the
  expression of an expression statement or an operand of the comma operator
  anywhere, the builder adds a condition node (`statement_calls`), so the
  call and its arguments still use the types from before it. For a local assertion function,
  `asserts value is T` becomes a runtime test of the argument, and
  `asserts value` decomposes the argument as a condition that holds.
- **Call points.** A call whose callee is a dotted name, and that no local
  function decides from its collected signature
  (`GuardFunctions::calls_local_function`), is a call point (`FlowCall`),
  whose guard is `FlowGuard::Call`. A local function that is generic or
  overloaded, with an overload that returns a type predicate, an assertion,
  or `never` (`selects_signature_by_arguments`), decides nothing at
  collection, because the call's arguments select its signature. Nor does a
  statement call of a local function whose return type annotation names an
  import, in place or through the module's type aliases
  (`AnnotatedReturn::Undecided`), because only inference can tell whether
  that type is `never`. A call in a condition is a call point only when it
  passes a subject (`condition_call_point`). A statement call is one whenever
  its callee follows TypeScript's explicit-type rule as far as the syntax
  tells (`statement_callee`), even when it passes no subject, because its
  signature can end the path; the reading context checks each member access
  along the callee (`FlowCallKind::Statement`). Collection records the
  subjects that a call point may narrow: its subject arguments, the subject of
  the callee's object, which `this is T` narrows, and, for a statement call,
  the subjects of each argument decomposed as a condition that holds, whose
  guard `FlowCallArgument::condition` keeps for `asserts value`. It also
  records the range of each argument (`FlowCallArgument::range`), where
  inference reads its type at the call, whether the argument is always false
  (`FlowCallArgument::is_false`), and whether the return type of a callee
  that is a name comes from an annotation (the `annotated` field of
  `FlowCallKind::Statement`).
- **Functions with inferred type predicates.** A local function whose
  collected return type is `ReturnType::InferredPredicate` may have a type
  predicate inferred from its body (`GuardFunctions::calls_inferred_predicate`).
  A call of it in a condition is a call point, and a statement call of it
  narrows nothing, because such a function never asserts. For each such
  function, collection decomposes the value that it returns, as a condition
  that holds and as one that fails, into a `FlowPredicate`
  (`ModuleFlow::predicate`), with its own subjects and call points, which
  `ModuleFlow::calls` doesn't list. Collection keeps it only when a guard may
  narrow the parameter that the marker names.
- **Property guards.** A runtime test of a property of a subject other than
  `this`, as in `s.kind === "c"`, `s["kind"] === "c"`, or `t[0] === "a"`,
  also adds a property guard on the immediate object, unless the property is
  a private name; `o.a.kind` narrows `o.a`, not `o`. The guard applies after
  the test of the property itself, as `&&` combines its operands. A call to a
  type-guard function adds none. A runtime test of a `const` that holds a
  property, as `k` after `const k = s.kind` or `const { kind: k } = s`, adds
  the same guard; for a destructuring element, the decomposer adds the
  property's path, such as `s.kind`, to the subject table, so that the write
  scan finds writes to it. A runtime test of an element of a narrowing
  pattern adds a property guard on the pattern. No write changes the values
  that a pattern destructured, so such a guard is never marked as aliased,
  and it applies inside an inlined aliased condition too.
- **`in`.** A test `"a" in x` becomes a runtime test of `x` (`FlowTest::In`)
  whose key is a string or decimal integer literal, spelled as a property
  path requires. The decomposed condition lists `x` with the objects of
  property guards, because `in` narrows only a type with an object type,
  `unknown`, or a type parameter. It adds no property guard. A test `#p in x`
  becomes a runtime test of `x` that holds the range at which inference finds
  the type of the innermost class around the test that declares `#p`
  (`FlowTest::PrivateName`), and is listed as an `instanceof` test is.
- **`instanceof` and `constructor`.** A test `x instanceof C` becomes a
  runtime test of `x` (`FlowTest::Instanceof`) that holds the range of `C`,
  which inference resolves only when the test can narrow. The decomposed
  condition lists `x` with the objects of property guards and among the
  subjects that `instanceof` tests, unless the annotation of `x` is `any`. A
  comparison of `x.constructor` with a dotted name becomes the same kind of
  test (`FlowTest::Constructor`) in the branch where the two are equal, and
  narrows nothing in the other branch.
- **`switch` cases.** A case becomes the condition that
  `discriminant === test` holds (`decompose_switch_case`), so a case adds the
  property and pattern guards that the same comparison adds. The discriminant
  decides how cases narrow (`SwitchKind`): a case of `switch (typeof x)` adds
  no property or pattern guards, and a case of `switch (true)` is its test
  when it is truthy. The condition that a case of `switch (true)` didn't match
  decomposes the test as falsy only when the test gives a boolean
  (`is_boolean_valued`). The condition that no case of a `switch` on a value
  or on `typeof` of one matched, for `default` and for the bypass, is wrapped
  in `FlowGuard::Unmatched`, which leaves a type with an `unknown` member as
  it is, and keeps a member that is a type parameter, or an intersection with
  one, whole unless the guard leaves it no value, where an `if` with the same
  test gives an intersection. A `switch` that TypeScript can find exhaustive
  wraps its bypass guard in `FlowGuard::Bypass` and is recorded in
  `RootFlow::switches`, at most 64 per root.
- **`for...in`.** The body of a `for...in` loop whose head declares a
  variable starts with the condition that the loop's expression isn't nullish
  (`decompose_for_in`), which also tests the receivers of its optional chain
  and adds no property or pattern guard.
- **Aliased conditions.** A truthiness test of an aliased condition also
  decomposes its initializer and combines it with the test of the `const` as
  `&&` combines its operands. Inside an inlined initializer, only a test of a
  variable whose value can't change between the `const` and the test is kept;
  any other becomes `Keep`. A test of a property path that starts at such a
  variable, or at a `this` in the root's own code, is wrapped in
  `FlowGuard::ReadonlyPath`, which applies it only when inference finds every
  property along the path readonly; so are a property guard whose object is
  such a path and the test of such a receiver of an optional chain. Inlining
  stops once five aliased conditions enclose a test. When decomposition stops
  early, the subjects that inlined initializers read are mentioned too. A
  property guard from a `const` that holds a property, or from a test inside
  an inlined initializer, is marked as aliased, because a write can change
  the property between the `const` and the test.
- **`any`.** A variable whose annotation is `any`, as the raw tables show
  without resolving anything (`AnyRoot`), narrows only by the tests that
  narrow `any` in TypeScript (`FlowTest::narrows_any`). The decomposer keeps
  the guards of its other tests, which leave `any` as it is, because an
  earlier test can narrow the variable to a type that they narrow further,
  and `switch (typeof x)` needs every case. Those guards mention nothing, so
  they make no read a candidate. No test of a property path on such a
  variable mentions the path, and no property guard, test of a receiver, or
  condition of a decomposition that stops early mentions the variable.
  `instanceof` and a type-guard function whose target has only object types
  test such a variable as an ordinary subject.

#### Writes that the graph doesn't model

`writes.rs` finds the writes of a root that its graph doesn't model: in an
`async` function or a generator called where it is defined, a field
initializer, or unreachable code, and a write through the `this` of a static
block or of a function expression called where it is defined, whose `this`
is not a subject of the root. The scan visits the root's own code, including
inline code, but not the bodies of other functions, methods, accessors, and
constructors, which run only when called.

- A variable narrows only when each of its semantic writes in the root has a
  write node or runs only when called (`runs_only_when_called`). A write
  outside the root, of a variable that an enclosing root declares, never
  counts: it runs in another function, or in the enclosing root before or
  after the root's code.
- A write changes the written path and every path that extends it. A write
  through an element access whose key `subjects.rs` accepts, as in
  `o["v"] = x`, writes the path that the key names, and a write through any
  other key, as in `o[k] = x`, changes every path below its object. Every
  `this` and `super` along a written target stands for the root's `this`.
- `WrittenPaths` tells the writes of paths and variables apart by
  `BuiltRoot::written`: a path that an unmodeled write may change doesn't
  narrow anywhere in its root, and an aliased property guard whose property or
  object the root writes at all becomes `Keep`.
- A `FlowGuard::ReadonlyPath` guard becomes `Keep` when the root tests an
  object along its path with `instanceof`, a type-guard function, or `typeof`
  with a primitive kind or `"function"`, because such a test can replace a
  member of the object's type with one whose property isn't readonly, while
  inference decides from the declared type. The subjects of the other such
  guards are in `RootFlow::readonly_tested`, and those that no other guard
  narrows in `RootFlow::only_readonly_tested`.

#### Relevance and candidates

After building a root, collection keeps only the reads that a runtime test, a
write, or a call point of their subject can reach, and only for subjects that
can narrow. It classifies the subjects so that inference can skip the reads
that can't narrow without resolving their types:

| Field | Holds | Inference uses it to |
| --- | --- | --- |
| `RootFlow::read_points` | the read points of each subject | find the reads that one solve serves |
| `RootFlow::property_only` | subjects that only tests of their shape narrow, the tests that `parents` lists, and every pattern | start from `property_baseline` |
| `RootFlow::instance_tested` | subjects in `property_only` that a test other than a property guard narrows | accept types with a member other than `null`, `undefined`, and the primitives, such as an object type, a type parameter, `unknown`, or `any` |
| `RootFlow::any_tested` | subjects with reads that a test able to narrow `any` tests (`FlowTest::narrows_any`) | skip the solve of a subject whose baseline is `any`, unless it is here or one of its call points narrows (`may_change_baseline`) |
| `RootFlow::assignment_only` | subjects that only writes mention | skip candidates whose declared type is neither a union nor a literal type |
| `RootFlow::uninitialized` | subjects that may be read before they are assigned (`uninitialized_subjects`) | start with `undefined` |
| `RootFlow::evolving` | how each variable whose type evolves with its writes evolves | select the evolving solve |
| `RootFlow::subject_writes` | the values that each read subject's writes assign | resolve the assigned types |
| `RootFlow::subject_compared` | the values that each subject's comparisons by declared type compare with | resolve the compared types |
| `RootFlow::subject_calls` | the call points of each read subject | resolve the call effects |
| `RootFlow::readonly_tested`, `RootFlow::only_readonly_tested` | paths that `FlowGuard::ReadonlyPath` guards test | decide readonly paths |
| `RootFlow::switches`, `RootFlow::bypassed` | the recorded `switch` statements whose bypass reaches a kept read, and those reads' points | decide exhaustiveness |
| `RootFlow::ending_calls`, `RootFlow::called` | the root's first 64 statement calls, and the read points that their nodes reach | decide which calls never return |
| `RootFlow::pattern_elements` | the pattern of each element whose read a test of the pattern reaches | narrow the pattern first |
| `RootFlow::captures` | the variables of enclosing roots that the root continues | start from the capture point |

The rules behind these fields:

- **Write mentions.** A write node mentions its subject only where the
  collected types show that it can change the type (`write_may_change`, with
  `raw_may_be_union` and `raw_may_have_literal`). A path's declared type isn't
  collected, so a path write always mentions it.
- **Call points.** A call point leaves the classes of its subjects as they
  are, because whether it narrows depends on its callee's signature, which
  inference finds before it resolves anything else. A candidate made only by
  reads of one subject that only call points reach records the root and the
  subject (`ModuleFlow::call_only_candidate`), so inference can skip it when
  none of the subject's call points narrows. The other reads of a subject
  that call points mention are never skipped by the subject's class.
- **Object unions.** A property guard or `in` can narrow only a union with
  at least two object types, so collection drops a variable or a pattern in
  `RootFlow::property_only` when its collected type shows that it is not one
  (`raw_may_be_object_union`): a primitive, a single object type or class
  written in place, a union with at most one object type, or the result of
  `new`. A subject in `RootFlow::instance_tested` is dropped only when its
  collected type shows that it has no object type
  (`raw_may_narrow_by_instanceof`). A variable whose type is a name, an
  import, or inferred from an expression stays. The declared type of a
  pattern is the collected type of the value that its elements' collected
  `Destructure` types read from (`ModuleFlow::pattern_type`).
- **Eligible variables.** A variable can narrow in the root that declares it
  and in the roots nested in that one, including an import, when it is not
  untyped, not supplied by a destructuring default, and each of its writes in
  the root has a write node or runs only when called (`eligible_range`, and
  `encloses_eligible` once per variable of an enclosing root). A `var` that
  another declaration of its scope redeclares doesn't narrow
  (`is_redeclared`), because the semantic model resolves every reference to
  the last declaration while each declarator writes the variable.
- **Evolving variables.** A variable whose type evolves with its writes can
  narrow whatever its collected type, but only in the root that declares it
  (`declared_range`): `eligible_range` refuses it to nested roots and
  `closure_reads` skips it. One that nothing writes or adds to narrows as any
  other variable, unless it is declared without an initializer, or it is a
  `var` that a read can reach before its declarator runs
  (`may_read_before_declarator`): a read starts before the declarator, or
  the declarator is nested in another statement. An array also needs a write
  node for every addition that runs in the root's flow
  (`mutations_are_modeled`). Such a variable stays a subject only when a
  write node other than its initializer gives it a value, or, for a
  variable, when it has any write node other than its initializer, has no
  initializer, or is such a `var`. Only those write nodes mention it, so only
  the reads that they reach become candidates, together with the reads that
  the start reaches of a variable without an initializer and of a `var` with
  an initializer (`BuiltRoot::hoisted`). No such subject is in
  `RootFlow::uninitialized` or `RootFlow::assignment_only`.
- **Patterns and paths.** A pattern can narrow when its elements can and none
  of them is written. A read of an element is also kept when a test of the
  pattern reaches it: its point becomes a read point of the pattern. A
  property path can narrow when its variable can, or when it starts at a
  `this` that is a subject, and the write scan finds no write without a write
  node that may change it. The scan runs only when the root reads a tested
  path or has an aliased property guard on an object that can narrow.
- **Candidates.** The kept reads, plus the enclosing expressions that can use
  a narrowed operand, become candidates. A test inside a nested function
  never reaches reads in the enclosing one, and an array or object literal
  ends the chain of enclosing candidates. When a candidate consists only of
  reads of one variable in `RootFlow::property_only`, or only of reads of
  elements of one pattern that no test of the elements reaches, collection
  records that variable or pattern (`ModuleFlow::property_only_candidate`),
  so inference can skip the candidate as a whole. It records the variable too
  when only `instanceof` tests and type-guard functions with object targets
  reach the reads, even though other tests of the variable reach its other
  reads; a second walk from those other tests finds such reads.
- **Module-level classes.** A variable that several roots read has one
  baseline, so `ModuleFlow::property_only_bindings` and
  `ModuleFlow::assignment_only_bindings` list it only when every root that
  keeps reads of it lists it in `RootFlow::property_only` or
  `RootFlow::assignment_only`.
- **Built roots.** A selected root is built even when it declares no variable
  that can narrow, because it can read one of an enclosing root, and it keeps
  no graph when relevance keeps no read.

#### Closures

A read inside a closure that a root creates, directly or through other
closures, can continue the root's flow at the capture point of the outermost
one (`closure_reads`). The variables that continue, and where, are in
[Closures, inline code, and class members](./NARROWING.md#closures-inline-code-and-class-members).

- The root looks for such reads of each variable that it tests or writes, and
  of each element of a pattern that it narrows, among the variable's semantic
  reads, and keeps those that the capture rule allows (`capture_rule`).
- The last assignment of a variable comes from its semantic writes, each
  extended to the end of the outermost statement around it that starts after
  the declaration (`assignment_position`). A write in another function,
  inline code included, means that no read continues.
- Every function between the read and the root, other than inline code, must
  be such a closure, and each must reach the next one in, or the read, in its
  own reachable code (`continues_to`): as its builder found it, for a root
  that was built, and otherwise when no `return`, `throw`, `break`,
  `continue`, or infinite loop ends before it (`first_exit`), which is wider.
  Decorators, computed keys of members, fields, types, `declare` statements,
  and `with` statements stop the chain.
- Roots are collected inner first, so a root finds whether a nested root
  already keeps the read. When none does, the read becomes a read of the root
  at the capture point. Otherwise the outermost nested root that keeps the
  read, or continues from it, gets a capture of the variable
  (`RootFlow::captures`), the capture point becomes a read point of the root,
  and the nested root's uninitialized start of the variable goes, because the
  type at the capture point starts where the variable is declared.
- Each closure adds one read point to the root, which all its reads share.

### Inference

Inference resolves types only for a candidate read that a lookup or
complete module inference asks for, because resolving them can be costly.

#### Baselines

Guards, the subjects that each condition may narrow, and reads refer to a
subject by its id in its root's subject table, so a test and a read of the
same subject match.

- **Variables.** `infer_flow_binding_baseline` resolves the declared type of
  a variable once, for every root that reads it. For an import, that is the
  type of what it imports, which the tracked queries of the import's
  resolution give.
- **`this`.** A read of `this` starts from the type of the `this`
  expression.
- **Property paths.** The declared type of a path is the type of the member
  access at the read, computed from the narrowed type of its object when the
  object has one. Facts about a variable reach its paths only through that
  member lookup.
- **Pattern elements.** A read of an element first narrows the pattern at
  the read, from its declared type resolved by `infer_flow_binding_baseline`
  for the pattern's range. When the pattern narrows there, the element's
  declared type at the read is the element's field on the narrowed pattern,
  resolved as its collected `Destructure` type is
  (`resolve_destructure_field`), and the element's own tests narrow it from
  that type.
- **Captured variables.** A read of a variable that its root continues from
  an enclosing root (`RootFlow::captures`) starts from the variable's type at
  the capture point (`narrow_captured`). `captured_type` takes it from the
  solve of the enclosing root, which starts from the declared type or from
  the enclosing root's own capture, so a chain of closures resolves outward.
  A `let` that may be read before it is assigned keeps the `undefined` of its
  start along the chain. A read in a closure starts it without `undefined`
  when some code assigns it without reading it first, with `=`, a logical
  assignment, a destructuring assignment, or the head of a `for...in` or
  `for...of` loop (`is_definite_assignment`, `FlowCapture::assumes_initialized`,
  `FlowRead::assumes_initialized`), and otherwise gets the declared type where
  `undefined` remains although the declared type lacks it, so a `let` that
  only `++` or `+=` assigns keeps its declared type there.
- **Subject classes.** A subject in `RootFlow::property_only` starts from
  `property_baseline` instead of `narrowing_baseline`. It has no baseline, so
  its reads keep their declared types without a solve, unless its flattened
  type is a union with at least two object types (`may_narrow_by_property`),
  or, for a subject in `RootFlow::instance_tested`, has a member other than
  `null`, `undefined`, and the primitives, such as an object type, a type
  parameter, `unknown`, or `any` (`may_narrow_by_instanceof`). A pattern
  starts from `pattern_baseline`, which is `property_baseline` without the
  `null` and `undefined` members, because destructuring throws on them. To
  flatten the type, local types of the module resolve through the context
  instead of `infer_local_type`, so complete module inference, which holds
  all of them, finds most such variables unable to narrow at the cost of
  cache reads.
  Before evaluating a candidate that `ModuleFlow::property_only_candidate`
  names, both paths check that variable's baseline and skip the candidate
  when there is none, without finding its syntax node. When a call point
  narrows a subject whose reads elsewhere narrow only through tests of its
  shape or only through writes, the read starts from the whole declared type
  (`called_binding_baseline`).

#### Solve keys

`infer_flow_subject_states` applies the guards and writes of one root to all
reads of one subject in a single solve, and `infer_flow_subject_type` takes
the result at the read's incoming flow point. Both take
`FlowSubjectRootInput` as their key. Every input that the solve needs beyond
the collected graph is a field of that key, resolved by the context that
reads the subject, never by the solve's fresh one. A field that doesn't apply
to a subject has a neutral value, so reads that don't need it share one
solve.

| Field | Resolved by | Query that lookups share | Neutral value |
| --- | --- | --- | --- |
| `baseline` | the baseline rules above | `infer_flow_binding_baseline`, for variables and patterns | `None` when no runtime test can change the type |
| `readonly_path` | `is_readonly_path` | none; each read decides it | `false` |
| `exhaustive` | `switch_is_exhaustive`, one bit per statement | `infer_flow_switch_exhaustive` | `0` |
| `assigned` | `assigned_types`, typed without narrowing | `infer_flow_assigned_types` | `None` |
| `evolving` | collection, in `RootFlow::evolving` | none | `FlowEvolving::None` |
| `calls` | `call_effects`, the effects that narrow | `infer_flow_call_effect`, per call | `None` |
| `ends` | `ending_calls`, one bit per call that never returns | `infer_flow_call_effect`, per call | `0` |
| `compared` | `resolve_compared_types`, typed without narrowing | `infer_flow_compared_types` | `None` |

The answers are decided as follows:

- **Readonly paths.** A read of a path in `RootFlow::readonly_tested` decides
  once its baseline exists whether the path is readonly: at every step, the
  declared type of the object, the type of its expression resolved from the
  raw tables without narrowing, must declare the property readonly in every
  member other than `null` and `undefined` (`has_readonly_property`).
  `FlowGuard::ReadonlyPath` guards apply only in the solves of readonly
  paths. A path in `RootFlow::only_readonly_tested` that isn't readonly skips
  the solve.
- **Exhaustive `switch` statements.** A read whose point the bypass of a
  recorded `switch` reaches, and that comes after the statement's cases in
  the source, decides once its baseline exists whether that statement is
  exhaustive: the guard of the bypass, applied to the type of the
  discriminant's read at the statement, must leave `never`. That type is the
  read's flow-sensitive type when narrowing changes it, and its ordinary type
  otherwise. A case that compares with an imported value or a member of an
  enum counts only when inference finds a unit value there, because a case
  whose type isn't a unit type makes TypeScript find no statement exhaustive.
  `FlowGuard::Bypass` gives `never` in a solve whose key has its bit. A read
  never asks about a statement whose cases come after it, so the read of a
  discriminant never needs its own statement's answer.

Call effects and calls that never return are described in
[Call points](#call-points).

#### Applying guards and writes

`ResolutionCtx::apply_guard` applies a guard, and `ResolutionCtx::test_predicate`
turns a runtime test into the narrowing operation that applies it. The
operations live in `biome_js_type_info`:

- `narrow_type` applies truthiness, nullish, `typeof`, and equality tests by
  a `NarrowingPredicate`. `NarrowingPredicate::new_equality` gives the
  predicate of a comparison: a unit value compares as a literal, and any other
  type by `NarrowingPredicate::Equal`, whose false branch narrows nothing.
- `narrow_by_property` applies a property guard. It looks up the property in
  every member of the union and keeps the members whose property type can
  share a value with what the test leaves of the union of those types.
- `narrow_by_in_operator` applies `in`. `property_presence` in
  `type_relation.rs` looks up the property in each member and treats object
  types as closed, but reports that it can't tell for a type whose member
  list Biome may not know completely, such as a class or a built-in type.
- `narrow_by_instanceof` applies `instanceof`. `instance_types` in
  `type_relation.rs` finds the instance types of the constructor, and
  `is_derived_from` compares the members of the narrowed type with them by
  following `extends` clauses to the same declaration.
- `narrow_by_private_name` applies `#p in x` to the class that the test
  holds, resolved from `raw_binding_types` or `raw_expressions` at its range,
  and `narrow_by_constructor` applies a comparison of `x.constructor`. Where
  `#p in x` fails for a static name, the class goes only when it is the whole
  type, because TypeScript removes only the members derived from the class.
- `narrow_by_type_predicate` applies a type predicate. A primitive target,
  such as `string` or `"ready"`, stands for a `typeof` or equality test.
  Other targets compare the members of the narrowed type with those of the
  target in TypeScript's subtype relation (`is_subtype` in
  `type_relation.rs`), where a type parameter of the narrowed code relates to
  another type only through its constraint. When no member remains, the
  result is chosen by the subtype relation and by assignability with the same
  rule for type parameters (`is_assignable_in_generic_body`), and
  `instanceof` and `#p in x` choose the same way. The results are in
  [Type-guard functions](./NARROWING.md#type-guard-functions).
- Narrowing builds every intersection with `intersect_narrowed`, which never
  merges object operands into one type, so a later `instanceof` or type
  predicate still compares each operand by its declaration.

Every narrowing operation takes a resolver, as `keyof` and conditional types
do: `ResolutionCtx::narrowing_resolver` resolves handles in the context that
solves the read. `narrow_type` and `falsy_values` call it only to compare an
object type with primitives and functions in a falsy or `typeof` test, for
example to find that no falsy primitive has the members of
`interface Foo { a: number }`, and to read the constraint of a type
parameter, which narrows to its intersection with the type of the values
that pass, as `T & {}` for truthiness. Tests of primitive types never call
it. The comparison itself, including the member lookups on wrapper types such
as `String`, lives in `biome_js_type_info`. A new operation keeps a member in
both branches when it can't tell whether the member passes the test.

The solve resolves what a test needs only when the test can narrow: the key
of `in` from `raw_expressions` when the subject's declared type has an object
type or is `unknown` or `any`; the constructor of `instanceof`, the class of
`#p in x`, and the constructor of a `constructor` comparison when
`may_narrow_by_instanceof` accepts the declared type; and the target of a
type-guard function from its collected signature, under the same check when
the target has only object types. `GuardSolve` memoizes the result of each
property guard, `in` test, `#p in x` test, `instanceof` test, comparison of
`constructor`, and call to a type-guard function by guard and incoming type,
because a loop applies the same guard again.

Where branches meet, or a loop returns to an earlier point, the solve keeps
every type still possible on a reachable path. A read on a path that no value
can take gets type Unknown, not `never`.

**Writes.** A write node applies `ResolutionCtx::apply_write`: the declared
type reduced by the assigned type (`narrow_by_assignment`),
`base_literal_type` of the incoming type for a compound assignment, or the
declared type for a write without a value or of a proper prefix of the path.
A subject in `RootFlow::uninitialized` starts with `undefined`, and a read
where `undefined` remains although the declared type lacks it gets the
declared type.

**Evolving variables.** A read of a subject in `RootFlow::evolving` has its
own solve, selected by `FlowSubjectRootInput::evolving` (`narrow_evolving`),
whose state is the type that the writes give, from `undefined` at the start,
not the declared type narrowed. `ResolutionCtx::apply_evolving_write` applies
a write: a variable takes the assigned type, and an array's state is
`Array<T>`, which `EmptyArray` starts at `Array<never>` and each `Elements`
write extends with the value's type, so a join gives a union of arrays whose
element types `evolving_array_element` unions at the read. Assigned types are
widened by `evolving_assigned_type`, which gives `Unknown` for a value that
Biome may type otherwise than TypeScript, apart from a covered value, which
gives Biome's type with its literal types widened, and a write of the subject
without such a value gives `Unknown`. A variable whose state is `Unknown` is
Unknown at the read, while an array whose state is `Unknown`, or to which
nothing was added, keeps its declared type (`evolving_array_read`). Runtime
tests narrow a variable from its incoming type and never narrow an array.
When the root adds only undetermined values, or nothing, to an array that is
an array at every read, the read keeps its declared type without a solve.
When every write of a variable gives `null`, `undefined`, or `Unknown`, and at
least one gives `Unknown` (`RootFlow::has_valueless_write`), the read is
`Unknown` without a solve.

**Expressions.** `flow_expressions.rs` uses narrowed operands when inferring
member accesses, calls, logical and conditional expressions, `await`, the
comma operator, and `+`. A conditional expression is the union of its
branches, so its test matters only for a branch that reads the tested
variable, which `conditional_branch` leaves out when the test gives that
variable no value there.

#### Call points

A read of a subject in `RootFlow::subject_calls` resolves what its call points
do in the reading context (`call_effects`), never in the solve's fresh one.
The effect of a call point (`FlowCallEffect`) follows TypeScript's rules for
the signature that decides a call's effect on the flow:

- A call in a condition takes the single signature of its callee's type
  without `null` and `undefined`, which must return a type predicate. A
  statement call takes that of its callee's explicit type, which must assert.
  `has_explicit_callee` checks an import through the declaration that it
  names (`imported_declaration_is_explicit`) and each member access through
  the declared type of its object (`has_explicit_member`).
- A callee with a generic signature or several signatures, one of them with a
  type predicate, an assertion, or a `never` return type, takes the signature
  that the call's arguments select (`select_effect_signature` in
  `flow_signatures.rs`): the signatures in their order, each with the type
  arguments that the arguments infer from a few parameter shapes
  (`instantiate_signature`), and the first one that
  `signature_accepts_arguments` accepts under the subtype relation. An
  acceptance is conclusive only where that relation and the matching that
  call inference uses for return types agree.
- TypeScript checks the arguments with their types at the call, which are
  narrower than Biome's wherever Biome lacks a narrowing. So a signature
  before the accepted one is skipped only when no narrowing of the arguments
  makes it acceptable (`rejects_every_narrowing`) or when it narrows as the
  accepted one does; otherwise the call has no effect.
- A type argument that may be wider than TypeScript's, such as one inferred
  from an argument other than the one that the call narrows, or a literal
  type whose widening Biome can't tell, lets the call narrow only where it is
  truthy (`narrows_falsy` on `FlowCallEffect::Predicate`).
- The arguments have their types at the call. A subject argument has its
  flow-sensitive type, read with `calls_before` set on the context
  (`argument_type_at_call`), so that only the call points that end before the
  call apply. So does the subject that the callee is read from
  (`FlowCallReceiver`, read by `receiver_at_call`), whose type arguments a
  constraint such as `S extends T` refers to. Such a read never depends on
  the effect that it helps decide, so selection has no cycle, and the context
  evaluates it without the flow queries and caches, whose results apply every
  call point.
- A target that is an instance of a generic alias whose body is an
  intersection, as `NonNullable<T>`, takes the call's type arguments
  (`instantiate_intersection_alias`).
- `FlowGuard::Call` then narrows the argument or the object that the
  signature names with `narrow_by_type_predicate`, or applies the argument's
  decomposed condition for `asserts value`.

Lookups share each effect through `infer_flow_call_effect`, keyed by the
range of the call; complete module inference resolves it in its own context
and keeps it per call.

**Calls that never return.** A read whose point the node of a statement call
reaches decides, after the cheap gates and once its baseline exists, which of
those calls never return (`ending_calls`). Their effect is
`FlowCallEffect::Ends` when the return type annotation of the signature's
declaration is `never`, in place or through type aliases, which
`flow_baseline` follows (`has_annotated_callee`), or when an `asserts value`
signature gets an argument that is always false. Such an effect narrows
nothing, so `FlowSubjectRootInput::calls` leaves it out. A read that no path
reaches without crossing one of these calls keeps its declared type without a
solve (`RootFlow::reaches_past`), and so do the reads of a closure created
there. Otherwise the solve's key carries the calls, and `FlowGuard::Call`
gives `never` for each of them, which adds nothing where paths join.

#### Lookups and complete module inference

A lookup of a candidate goes through `infer_flow_expression_type`, which
resolves the baseline, the assigned and compared types, and the call effects
through `infer_flow_binding_baseline`, `infer_flow_assigned_types`,
`infer_flow_compared_types`, and `infer_flow_call_effect`.

Complete module inference evaluates flow candidates in its own resolution
context instead. When it reaches expressions, that context has resolved every
raw type of the module, so a variable's declared type, the member access that
gives a property path its declared type, and the assigned and compared types
read types that it already holds, while the queries would resolve them again
in a fresh context. The context keeps these results, and the answers about
readonly paths, exhaustive `switch` statements, captures, and call effects,
in `LocalFlow`. It solves through `infer_flow_subject_states` and
`infer_flow_subject_type`, as lookups do, so it shares solves with lookups
that start from the same types. Cycle fallback and the import-depth fallback
read imports differently from the flow queries, so they use the queries.

#### Inferred type predicates

A function's type predicate that TypeScript infers from its body is the
target of a `TypeofExpression::InferredPredicate`, which resolution reads from
`infer_flow_inferred_predicate` (`ResolutionCtx::inferred_predicate`). The
behavior is in [Inferred type predicates](./NARROWING.md#inferred-type-predicates).

- The parameter starts from its declared type, resolved in the context that
  resolves the function's type and flattened as a baseline is, and the guard
  of the `FlowPredicate` that holds narrows it. The function has a type
  predicate when that type differs from the declared type, isn't Unknown, and
  the guard that fails leaves `never` of it.
- The call points of the returned value take their effects, and its
  comparisons by declared types their values, from the same context. A work
  limit or a query cycle gives no type predicate.
- Lookups share the answer through the query. Complete module inference
  infers it in its own context while it resolves the function's type, and
  keeps it per function (`ResolutionCtx::local_predicates`), because that
  context resolves the parameter's type anyway.
- The parameter of a callback written in place takes its type from the call
  without the callback (`TypeofExpression::CallArgument`), so resolving the
  call never asks for the callback's type predicate through the parameter.

#### Reachability and implicit returns

`UnreachableCodeClassificationRequest` tells whether no path reaches a
statement. It asks the tracked query `infer_flow_reachability` about the
execution root around the statement, which builds that root's graph from the
module's syntax when it runs (`build_reachability`), with the builder that
collection uses, so reachability costs nothing until a rule or an inferred
return type asks. The query resolves the effect of each statement call,
through `infer_flow_call_effect` when collection kept the call point, decides
each `switch` statement as `switch_is_exhaustive` does, and walks from the
root's start without crossing a call that never returns or the bypass of an
exhaustive `switch`. A narrowed type, even `never`, never makes code
unreachable. A work limit or a query cycle makes the answer `Indeterminate`.

A return type that Biome infers from a function's body asks the same query
whether a path reaches the end of the body, where the function returns
`undefined`. Collection marks that `undefined` with a
`TypeofExpression::ImplicitReturn` when the body may complete, as far as
its syntax shows, and resolution gives `undefined`, or `never` when no path
reaches the end (`ResolutionCtx::resolve_implicit_return`), so the
reachability of a function's end is decided once for every context. A
function with a `return` without a value already returns `undefined`, so
its end gives `undefined` without the query. So does a body whose syntax
shows a path to the end (`reaches_end_by_syntax`): the path crosses no
statement call that the graph keeps or that never returns, uses no `switch`
bypass, and avoids every construct that the walk doesn't model, such as
labels and `continue`. Deciding whether a statement call never returns, or
whether a `switch` is exhaustive, can need the same return type, as for
`box.fail()` with `declare const box: { fail: typeof f }` in the body of
`f`; the query cycle then leaves the end reachable, which keeps
`undefined`.

### Caching

Collected flow belongs to the module input, so an edit replaces it together
with the raw type tables, even when variable declarations and references stay
the same. Salsa caches the type work: all reads of a variable share the type
that narrowing starts from, its reads in one root share one solve, and reads
at the same flow point share their result. Lookups and complete module
inference share the solves and the results when they start from the same
type. These caches recompute when the module input changes; they don't track
edits separately for each function.

### Work limits

Collection, condition decomposition, and the solve each have work limits.

- A root that exceeds a limit while its flow is collected gets no graph, so
  its reads keep their declared types.
- The limit on finding the reads that a test can reach is spent on variables
  before property paths and subjects in `RootFlow::property_only`. When it
  runs out on those, the root keeps its graph and only their reads lose
  narrowing.
- A condition whose decomposition can't finish within its limit gets
  `FlowGuard::Incomplete`, which makes the result unknown rather than
  assuming the expression unaffected.
- When the shared solve exceeds its limit or meets a query cycle,
  `infer_flow_subject_type` solves the read's point on its own, so a read
  never loses precision because other reads share its root. If that solve
  also exhausts its limit or meets a cycle, the result is unknown.

Callers respect an unknown result rather than replace it with a more
confident answer from raw type information.

### Extending narrowing

Start from the behavior: check TypeScript's result in strict mode for each
branch, with the version that `NARROWING.md` names, and decide where Biome
follows it. Where Biome can't follow it, the result is wider than
TypeScript's, never narrower.

#### Adding a runtime test

1. Make sure the scanner selects a root that contains the new condition
   (`scanner.rs`). Most conditions already select their root.
2. Decompose it in the `Decomposer` (`guards.rs`) into a `FlowTest`, a new
   variant when no existing one fits. Hold ranges, never types: a test that
   needs a resolved type, such as the constructor of `instanceof`, keeps the
   range where inference finds it.
3. Decide which group of `DecomposedCondition` lists each subject that the
   test narrows, because that decides the subject's class and which reads
   become candidates. Decide whether the test narrows `any`
   (`FlowTest::narrows_any`), the receivers of an optional chain, and the
   object that holds a property.
4. Apply it in `ResolutionCtx::test_predicate` or `ResolutionCtx::apply_guard`
   (`src/db/type_inference/flow.rs`), through a narrowing operation in
   `biome_js_type_info` that takes a resolver and keeps a member when it can't
   tell. Gate the work on the declared type, as `may_narrow_by_instanceof`
   does, and memoize it in `GuardSolve` when it resolves types.
5. Add tests and update `NARROWING.md`.

#### Adding a flow node

1. Add the variant to `FlowNode` (`src/js_module_info/flow.rs`), with its
   antecedents.
2. Push it from the builder at the point in evaluation order where it takes
   effect.
3. Handle it in every walk over the graph. Collection walks it in
   `successors`, `RootFlow::reaches_past`, and the relevance walks of
   `RootCollector`. Inference walks it in `predecessor_ids`,
   `ResolutionCtx::solve_flow`, and `ResolutionCtx::reachability`. Search for
   `FlowNode::` to find them.
4. Keep every walk bounded: loops make the graph cyclic.
5. A node that ends paths ends them in the reachability walk too, so
   `UnreachableCodeClassificationRequest` and implicit returns agree with
   narrowing.

#### Adding a solve input

When a guard needs a type that depends on resolution:

1. Collect the syntax-only part per subject in `RootFlow`, as
   `RootFlow::subject_compared` does.
2. Add a field to `FlowSubjectRootInput` whose neutral value keeps reads that
   don't need it sharing one solve.
3. Resolve it in the reading context, for lookups and for complete module
   inference, and type values without narrowing.
4. When lookups resolve it often, add a tracked query beside the other flow
   queries in `lookups.rs`, with a cycle result that preserves uncertainty,
   and keep it in `LocalFlow` for complete module inference.
5. Add a Salsa execution test that reads that don't need the input run no
   new query.

### Review checklist for narrowing

- [ ] `NARROWING.md` describes the change: what narrows, a deliberate choice
      with its reason, or a limit.
- [ ] No read is narrower than TypeScript's. A known exception is a
      deliberate choice in `NARROWING.md`, listed with the differences from
      TypeScript.
- [ ] Collection reads only syntax, the semantic model, and the module's own
      raw tables.
- [ ] Every type that the solve uses is in its key, resolved by the reading
      context, and neutral for reads that don't need it.
- [ ] Lookups and complete module inference give each read the same type.
- [ ] A new walk over the graph is bounded, and a new limit degrades to the
      declared type in collection or to Unknown in inference.
- [ ] Reads that can't narrow skip the flow queries, and a Salsa execution
      test shows it.
- [ ] An impossible branch reads as Unknown, and reachability goes through
      `UnreachableCodeClassificationRequest`.

## Profiling inference

Use `--profile-type-inference` to inspect CLI inference work:

```shell
biome lint --profile-type-inference path/to/reproduction
biome check --profile-type-inference path/to/reproduction
```

The report groups records by source file. The default output shows the ten
files with the highest cumulative request time. Within each file, it shows the
top five request groups, top eight query groups, and top five whole-module
groups. It includes the total number of groups, source records, and completed
executions so omitted work remains visible.

Use `--verbose` to show detailed metrics and implementation references for the
top five request records, top eight query records, and top five whole-module
records in each file. Verbose output includes every file while retaining the
per-file limits.

Request and query timings are inclusive and must not be added together. A high
request count with few tracked query executions usually indicates Salsa reuse.
Ranges are zero-based, half-open UTF-8 byte offsets. Profiles contain relative
paths and timings, but not source text, inferred types, or import specifiers.

## Testing

Every behavior change includes correctness tests. Request or query boundary
changes also include selectivity tests. The type-info guide lists the kinds
of tests and the boundary each one proves
([Testing](../biome_js_type_info/CONTRIBUTING.md#testing)).

### Correctness matrix

Cover the cases relevant to the request or query:

- expressions and bindings in the same module;
- recursive declarations;
- named, default, and namespace imports;
- named, blanket, and namespace re-exports;
- missing and ambiguous exports;
- import and declaration cycles, including cycles through generic parameter
  constraints and defaults declared in different modules;
- disabled inference;
- unsupported input shapes;
- traversal budget exhaustion.

### Narrowing tests

Narrowing tests live in the topic files of `tests/spec_tests/`, such as
`closures.test.rs` or `switch.test.rs`. A new topic file is registered in
`tests/spec_tests.rs` as a module with a `#[path]` attribute.

- Mark each read with a `/*marker*/` comment right before the expression, and
  assert the sorted names of the members of its type. Most topic files define
  an `assert_reads` helper for one module, and `assert_file_reads` in
  `tests/spec_tests.rs` reads one module of several. The `assert_reads` of
  some topics, such as `closures.test.rs`, also checks that complete module
  inference gives each read the type that a lookup gives it; use such a
  helper for a change that touches both paths.
- Start each test with a comment that states TypeScript's result in strict
  mode for its reads and, where Biome differs, the deliberate choice or limit
  in `NARROWING.md` that describes it.
- Cover both branches of a test, the code after it, and a read that the test
  doesn't reach, so that narrowing that leaks shows up.

### Salsa execution tests

Use the Salsa event helpers from `biome_db::testing` to assert which tracked
query bodies executed. At minimum, a lookup request test proves that
`infer_module_types` did not execute.

Test cache reuse separately from correctness:

1. execute the same request twice;
2. assert both consumer requests complete;
3. assert the tracked query body executes only when Salsa recomputes it;
4. edit an unrelated input and assert the query remains reusable;
5. edit a dependency and assert the query recomputes.

For narrowing, count the flow queries that a read runs with
`function_query_will_execute_count_by_name`, as `assert_no_flow_queries` in
`narrowing.test.rs` does, to show that reads that can't narrow skip them.

Salsa events are a deterministic recomputation oracle. They are not a
production timing mechanism because they do not include query completion
events.

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

Compare benchmarks by the inference boundary that they invoke and the tables
that they infer, not by their names.

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
- [ ] Every `ResolutionCtx` created during resolution sits behind a tracked
      query, a spent import or step budget, or the shared declaration
      evaluator.
- [ ] Namespace and import-chain breadth were reviewed.
- [ ] Whole-module inference is absent or explicitly justified.
- [ ] Cross-module data remains owned by its source module.
- [ ] Correctness and Salsa execution tests cover the flow.
- [ ] The metadata trait has a stable ID and label, and the request has an
      implementation reference.

A change to narrowing also passes the
[review checklist for narrowing](#review-checklist-for-narrowing).
