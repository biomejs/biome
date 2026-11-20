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
- `raw_binding_types`: binding ranges mapped to type references. The
  bindings of interfaces of one name in one scope, and of a class with them,
  all map to one merged type, as TypeScript merges these declarations (see
  `JsModuleInfoCollector::merge_type_declarations`).

A raw reference identifies another entry in the module table, a global type, or
a name or import that still needs resolution. It is not an inferred type.

For analyzer consumers, resolution starts when a request invokes a tracked
query. A lookup query selects the raw reference for the requested range or table entry,
and `ResolutionCtx` converts it to `InferredTypeData`. It follows module-table
references, resolves globals, and uses export and lookup queries for imports.
Named declarations remain symbolic `LocalTypeHandle` values until a consumer
normalizes or inspects their structure. In this code, "local" means owned by one
module's type table; it does not mean block- or function-scoped source code.

Some declarations merge interfaces into declarations that their module
doesn't own: module augmentations (`declare module "./a" { ... }`),
`declare global` blocks, and the top-level interfaces of declaration files
without imports or exports. TypeScript applies them to the whole program.
Collection records them in `JsModuleInfoInner::contributions` and makes their
names refer to the declaration that they merge into. The tracked queries in
`src/db/queries/type_inference/augmentations.rs` index them across every
indexed module: `module_augmentations` per module, `augmentation_index` over
the graph, and projections for one target module or one global, which
backdate unless their own entry changes. None of them resolves a type, so
they never join an inference cycle. `ResolutionCtx::resolve_raw_type_id` adds
a declaration's contributions to its data as handles, and
`resolve_global_type` adds a global's through
`TypeDb::global_contributions`. Nothing else builds a declaration's data, so
a handle to it has the same data in every context; data built without its
contributions would make `instanceof` see two declarations. A contribution's
members are resolved only by whoever resolves its handle, in its own module.

Resolution widens fresh literal types where TypeScript does, as in
`let n = 1`, which declares `n` as `number`
(`src/db/type_inference/widening.rs`). Collection marks the locations, as
the type-info guide describes, and resolution tells a fresh literal type from
a declared one by the raw expressions behind the value: a literal, the
operands of a conditional or logical expression, and the initializer of a
`const` declared without an annotation, also in another module, found with
`find_export_origin` without resolving a type. The properties of an object
literal are widened where the object is resolved. A call keeps their
literal types to select an overload, as TypeScript's contextual typing
does, and infers generic parameters from the widened type
(`ResolvedCallArgument::WidenedArgument`).

Start at `src/module_graph.rs`, `src/js_module_info/collector.rs`, and
`src/db/type_inference/resolver.rs`.

### Flow-sensitive expression reads

An expression lookup can give a variable a more specific type at a particular
read. For example, after `if (value === null) return`, a variable typed
`string | null` can be read as `string`. The type stored for the variable's
declaration does not change.

[`NARROWING.md`](./NARROWING.md) is the reference for what narrowing does: its
correctness contract, which runtime tests narrow and which do not yet, its
limits, and where Biome deliberately differs from TypeScript. A change to
narrowing updates the rows it affects there. This section describes how
narrowing is built.

An execution root is a body analyzed independently, such as a module,
function, or method. Each root gets its own graph. A read in a closure, such
as an arrow function, continues the flow of the root that creates the
closure from the point where it creates it, as TypeScript does; any other
read never sees facts from an enclosing root. Biome uses its semantic model to
distinguish variables with the same name.

Flow is split like types: collection gathers syntax-only facts, and tracked
queries do everything that needs a resolved type.

#### Collection

When type inference is enabled, collection (`src/js_module_info/flow.rs`)
builds a graph of the possible paths through each execution root that tests
or writes a variable that it reads, its own or one of an enclosing root, or
a path on its own `this`. Its parts live in `src/js_module_info/flow/`:

- `scanner.rs` selects roots while the module visitor walks the syntax tree,
  so collection never walks the whole module again. A root is selected when
  its own code contains a condition, counting every `switch` statement,
  every `for...in` loop, every logical assignment, and every `?.` that a key
  or arguments follow in its chain, a write (an assignment, `++`, `--`,
  `delete`, a declarator with an initializer whose type annotation is a union
  or that destructures a pattern, the head of a `for...of` loop that
  assigns a target, or a call of `push` or `unshift` on a variable when the
  root also declares a variable without a type annotation initialized with
  `[]`), or a statement call that may assert: the expression of an
  expression statement, or an operand of the comma operator there, not in
  parentheses, whose callee is a dotted name and that passes an argument or
  calls a method (`is_statement_call`, `may_assert`). A root that only such
  calls select keeps them, and collection builds it only when one of their
  callees can assert as far as the semantic model tells
  (`GuardFunctions::keeps_statement_call`). It also records whether the root's own code
  uses `this`, so a root whose only subjects are paths on `this` still gets a
  graph. The scanner rejects a root when its own code contains a `with`
  statement or bogus syntax, and a root over its node or depth limit. Declaration files (`.d.ts`, `.d.mts`, `.d.cts`, in any letter
  case) contain no code that runs, so the visitor does not look for roots in
  them.
- `builder.rs` builds the graph of one selected root from start, join,
  condition, and write nodes. It records the incoming flow point of every
  reachable read of a subject and, for each condition node, the subjects its
  guard may narrow. A read of a property path is recorded after its object
  and key are evaluated. Expressions are evaluated in the order in which
  JavaScript runs them, with the conditions that TypeScript's binder adds:
  the key and the arguments after a `?.` start from a condition that its
  object is not nullish, as `bindOptionalChain` does, and the right operand
  of `&&=`, `||=`, or `??=` from a test of its target, as
  `bindLogicalLikeExpression` does. A function evaluates its parameters, in
  order, before its body. A default of a parameter or of the pattern of a
  declaration runs only when the value is `undefined`, so the flow after it
  joins the flow before it, as `bindInitializer` does. As in TypeScript's
  binder, a declaration evaluates its pattern before its initializer, and a
  destructuring assignment its pattern, with unconditional defaults, before
  its value. A branch that a condition written as the literal `true` or
  `false` never takes has no flow, as TypeScript's `createFlowCondition`
  decides (`never_has_outcome`): in `if`, loops, conditional expressions,
  and the operands of `&&` and `||`, through parentheses and `!` around such
  an operator, but not for `(false)`, `!false`, or the operands of `??`.
  Neither does the code after a statement call of a local function that
  never returns (`GuardFunctions::local_call_ends`): a function declaration
  with one signature whose return type annotation is `never`, or an
  `asserts value` function whose argument is always false. Reads there are
  not recorded, so they keep their declared types, and such a path adds
  nothing where it joins the others.
  Each write of a subject adds a write node (`FlowWriteEffect`, with the
  assigned `FlowAssigned` value) that also joins the innermost exception
  join, as `createFlowMutation` does: assignments of every kind, `++`, `--`,
  `delete`, destructuring targets, loop-head targets, the variables of an
  annotated declarator, and those of a `var` declarator with an initializer,
  which a read can reach before the declarator, written without a value.
  A declarator of a variable whose type evolves with its writes
  (`FlowEvolving`, as `evolving_declaration` decides) writes its
  initializer, `FlowWriteEffect::EmptyArray` for `[]`, and
  `BuiltRoot::initializers` lists those write nodes. For such an array, each
  argument of a call of `push` or `unshift` on it (`mutated_array`), after
  the arguments run, and the value of an element assignment with `=`
  (`element_write`) add a `FlowWriteEffect::Elements` write node, and an
  assignment of `[]` an `EmptyArray` one; `WrittenTargets::mutations` lists
  the variables that they add to. A value that Biome may type narrower than
  TypeScript (`value_callees`), or that reads a variable whose type evolves
  with its writes (`reads_evolving`), whose declared type doesn't follow
  them, gets none, so the write gives the declared type, or `Unknown` for a
  variable whose type evolves with its writes. A call of a local function
  without a return type annotation, a function declaration or a function or
  arrow expression that initializes a variable (`inferred_local_body`),
  counts as typed as in TypeScript when each value that the function
  returns passes the same checks, through at most four nested functions,
  because Biome infers its return type with the `undefined` of an end that
  a path reaches. The value of an assignment with `=`, `??=`, `||=`, or
  `&&=` to a variable whose type evolves with its writes, in a scope other
  than its declaration, is covered instead (`FlowAssigned::covered`, as
  `is_covered` decides) when an inferred return type is the only reason
  Biome may type it narrower: the collector's declared type joins those
  values (`widen_binding_from_writable_references`), so the write gives
  Biome's type for the value. A value that a local function returns and
  that reads a variable whose type evolves keeps the write uncovered. A call
  whose callee's annotation the syntax doesn't show, an import or a method
  (`FlowCallee`), is kept in `FlowAssigned::callees`, and the reading
  context confirms it where it resolves the assigned types
  (`ResolutionCtx::has_annotated_return`): an import by the declaration
  that it names, and a method by every type of its object, which must get
  the method from the lib, from a declaration file, or from a class or an
  interface that declares it with a return type annotation
  (`member_return`, which reads the declaration that the type's scope
  records).
  `BuiltRoot::written` lists the targets with a write node. The code after
  an optional chain continues from the flow after its root, as
  `bindOptionalChainFlow` restores it, unless a key or an argument has a
  write node; then it joins each `?.`'s short circuit with the chain's end. A `?.` whose object the raw tables show can't be
  nullish (`raw_may_be_nullish` in `biome_js_type_info`), such as a variable
  that holds an object literal, adds no condition, so it doesn't make the
  object a tested subject. The builder skips class members: methods, accessors,
  constructors, and static blocks are roots of their own, and TypeScript
  analyzes a class field that has an initializer apart from the code around
  the class. It also skips `declare` statements, which never run. It records
  each closure that it skips in reachable code and that continues the flow
  where it is created (`AnyContinuingClosure`: a function or arrow
  expression, or a method or accessor of an object literal or of a class
  expression) with the flow point there, its capture point
  (`BuiltRoot::captures`).
- The builder models the control flow of blocks, `if`, `while`, `do...while`,
  `for`, `for...in`, `for...of`, `switch`, `try`, labeled statements,
  `break`, `continue`, `return`, and `throw`, as TypeScript's binder does. A
  `for...in` or `for...of` loop evaluates its expression once, before the
  loop. Its header, where each iteration starts, joins the flow after the
  expression with the end of the body and each `continue`, as
  `bindForInOrForOfStatement` does, and each iteration evaluates the head,
  such as the defaults of a destructuring pattern, from the header, then
  writes an assignment target of the head. The header is also the exit, so
  the code after the loop continues from it and from each `break`, and a
  test in the body never reaches it. A `let` or a `const` that the head
  declares gets no write node: the header joins the entry, where the
  variable has its declared type, so the facts that the end of the body
  carries back never make a read narrower than that type. A `var` is
  hoisted, so code before the loop can test or write it, and each iteration
  writes it without a value, which gives it its declared type. A `switch` evaluates every case test
  from the flow after its discriminant. Its clauses form groups, a run of
  clauses without statements followed by the next clause, and a group's
  statements start from the join of a condition for each of its cases, a
  condition that no case matched when the group contains `default`, and the
  fall-through from the group before. Without `default`, the code after the
  statement also continues from the bypass, the condition that no case
  matched. `switch (true)` chains the conditions instead, as TypeScript's
  `narrowTypeBySwitchOnTrue` does: a group starts where every case before it
  failed to match. A `try` statement starts its catch clause from the join
  that the exceptions of its try block reach, as `bindTryStatement` does: an
  exception can leave the try block at any point, and conditions only
  narrow, so no point inside the block is narrower than its start or one of
  its write nodes, which the join holds. The builder keeps such a join for
  each enclosing try block and catch clause. The code after the statement
  continues from the ends of the try block and the catch clause, and `break`
  and `continue` reach their targets directly. A finally block is built
  twice. The first build starts from the join of those ends, the exceptions,
  and each `return` in the try block and the catch clause, and records the
  reads of the block. The second build starts from the ends only and records
  no reads, and the code after the statement continues from it, because that
  code runs only when the statement completed normally, as TypeScript's
  `createReduceLabel` over the normal exits says. Any other statement, a
  declaration such as a class or an enum, is an opaque region. The builder
  evaluates the expressions of a region, as it evaluates any expression, from
  the flow point before it, and the next statement continues from the
  result. The builder joins the target of any `break` or `continue` that
  leaves a region from the point before the region, which is never narrower
  than a point inside it while the region has no write node; a region with
  both gets no graph. The remaining regions are declarations, which contain
  no such jump.
- `subjects.rs` maps an expression to the subject it reads or tests: a
  variable or `this`, or a property path that starts at one, such as `o.v`,
  `a.b.c`, `t[0]`, or `this.#v`. A static member access with a named property
  or a private name extends a path, and so does an element access whose key
  spells its property's name: a string
  literal without escapes, or a decimal integer literal from 0 to 2^53 - 1
  without leading zeros or separators. JavaScript converts a number key to a
  string, and such an integer converts to its own digits, so `o["v"]` is the
  same subject as `o.v`, and `t[0]` the same as `t["0"]`. Any other key, such
  as `k`, `-0`, or `0x0`, ends the subject. A private name gets a key of its
  own, so `o.#v` is never the same subject as `o["#v"]`. Parentheses are
  skipped, and `o?.v` is the same subject as `o.v`. Each root stores its
  subjects in a table, a path after its prefixes, so the builder and the
  decomposer give a read and a test of the same subject the same id.
- `this` is a subject only in a root where `this_can_narrow` in `subjects.rs`
  holds: where Biome types `this` as TypeScript does and TypeScript narrows
  its properties as those of a variable. That is a class method or accessor
  that is not `static` and has no `this` parameter, an object literal method
  or accessor, or an arrow function whose `this` comes from one of them or
  from a field initializer or a constructor, when Biome can name the class
  or object, and a constructor. In a constructor, and in an arrow function
  that it calls where it is defined, a path on `this` whose field has no
  initializer may be read before it is assigned, as
  `uninitialized_fields` decides. Each root has its own `this`, so a test of
  `this.v` never reaches a read in a nested function.
- `guards.rs` decomposes every condition into guards on subjects, combined
  the way `!`, `&&`, and `||` combine them. For example,
  `typeof value === "string"` becomes a runtime test of `value`, and
  `o.v !== null` a runtime test of `o.v`. A comparison tests one operand
  only when the other is a unit value (`unit_value`), whose type has a
  single value: a literal, `void`, `-1`, a template literal without
  substitutions, or a `const` without a type annotation whose initializer
  is one, which the decomposer follows through the semantic model. The test
  then names that literal by its range, so inference resolves it as it
  resolves a literal written in the comparison. A value that a named or
  default import binds is a test of the import's variable
  (`FlowTest::Imported`): collection can't see the module that declares it,
  so inference follows the import with `find_export_origin` and applies the
  same rules to the declaration there, reading syntax and raw types without
  resolving a type. Other values, such as variables with a type annotation
  and calls, give no test, because Biome may infer their types narrower than
  TypeScript does. When the tested operand is part of
  an optional chain and the test's result shows that the operand is not
  `undefined`, as the true branch of `x?.y === 1` does, the guard also tests
  each receiver of the chain that is a subject as not nullish, such as `x`
  and `x.y` in `x?.y?.z`, combined with the main test as `&&` combines its
  operands. Which branches narrow the receivers follows TypeScript's
  `narrowTypeByOptionalChainContainment`: neither outcome of `??` does, for
  example. Parentheses end a chain, and a test of a chain that starts at a
  parenthesized optional chain, such as `(x?.y)?.z === 1`, narrows nothing:
  TypeScript narrows `x.y` there but not `x`, so a read of `x?.y` adds
  `undefined` back from `x`, while a narrowed path in Biome would keep its
  narrowed type at that read. A call to a local type-guard
  function becomes a runtime test of its argument: collection selects the
  tested argument from the function's collected signature, and inference
  later resolves only the target of its type predicate. Neither step infers
  the function's body. Collection also tells from the raw tables, and from
  the declarations that the target's names refer to, whether every member of
  the target is an object type, as for an interface, a class, or a type
  literal. Such a test, as `isFoo(x)` with `x is Foo`, narrows only a type
  with an object type or `unknown`, so the decomposed condition lists `x`
  with the objects of property guards and among the subjects that
  `instanceof` tests. An imported name can be an alias of a primitive, so it
  doesn't count as an object type, and a target of `any` or `unknown` makes
  the function unsupported. After each statement call, the expression of an
  expression statement or an operand of the comma operator there, not in
  parentheses, as TypeScript's binder adds calls to the flow, the builder
  adds a condition node (`statement_calls`), so the call and its arguments
  still use the types from before it. For a local assertion function,
  `asserts value is T` becomes a runtime test of the argument, as
  `value is T` does, while `asserts value` decomposes the argument as a
  condition that holds, so `assert(value !== null)` excludes `null` from
  `value`.
  A call whose callee is a dotted name, an identifier or `this` followed by
  `.name` accesses, and that no local function decides from its collected
  signature (`GuardFunctions::calls_local_function`), is a call point
  (`FlowCall`), whose guard is `FlowGuard::Call`. Its effect depends on the
  callee's signature, which only inference resolves, so collection records
  the subjects that it may narrow: its subject arguments, the subject of the
  object of its callee, which `this is T` narrows, and, for a statement
  call, the subjects of each argument decomposed as a condition that holds,
  whose guard `FlowCallArgument::condition` keeps for `asserts value`. A call
  in a condition is a call point only when it passes a subject
  (`condition_call_point`). A statement call is one whenever its callee
  follows TypeScript's explicit-type rule as far as the syntax tells
  (`statement_callee`), even when it passes no subject, because its
  signature can end the path: the name that it starts at is a function, a class, a
  namespace that `declare` declares, a variable or a parameter with a type
  annotation, a global, an import, which inference checks, or `this` in a
  class member that is not `static`, and the reading context checks each
  member access along it (`FlowCallKind::Statement`). Collection also
  records whether each argument is always false
  (`FlowCallArgument::is_false`, as TypeScript's `isFalseExpression` reads
  it) and whether the return type of a callee that is a name itself comes
  from an annotation (the `annotated` field of `FlowCallKind::Statement`).
  The decomposed
  condition lists the subjects of call points apart from the others
  (`DecomposedCondition::call_mentions`).
  A runtime test of a property of a subject other than `this`, as in
  `s.kind === "c"`, `s["kind"] === "c"`, or `t[0] === "a"`, also adds a
  property guard on that subject, the object that holds the property, unless
  the property is a private name. The guard is applied after
  the test of the property itself as `&&` combines its operands. The guard
  narrows the object as TypeScript narrows a discriminated union, so `s`
  becomes `Circle` there. Only the immediate object gets one: `o.a.kind`
  narrows `o.a`, not `o`. A call to a type-guard function adds none, because
  TypeScript never narrows `s` through `isC(s.kind)`. The decomposed
  condition lists such objects apart from the subjects it mentions, so the
  builder knows which subjects only property guards narrow.
  A runtime test of a `const` that holds a property adds the same property
  guard, following TypeScript's `getCandidateDiscriminantPropertyAccess`:
  `k === "c"` applies to `s` after `const k = s.kind` or
  `const { kind: k } = s`. The `const` follows the rules of an aliased
  condition below. Its initializer must be a member access, or a subject
  when the `const` is an element of a destructuring pattern. Such an element
  must have no default and must not be a rest element, and the pattern must
  be the declarator's own binding, so `const { a: { kind } } = w` holds
  nothing. For a destructuring element,
  the decomposer adds the property's path, such as `s.kind`, to the subject
  table, so that the write scan can find writes to it.
  A runtime test of a variable that is an element of a destructuring pattern
  also adds a property guard on the pattern, a subject that stands for the
  value the pattern destructures, following TypeScript's
  `getNarrowedTypeOfSymbol`: `kind === "c"` applies to `{ kind, v }` after
  `const { kind, v } = s` and in `function f({ kind, v }: S)`.
  `narrowing_pattern` in `subjects.rs` accepts a pattern that a `const` or a
  parameter whose variables are never written declares, with at least two
  direct elements that have no default and are not rest elements, and the
  subject table records those elements with their keys when it first stores
  the pattern. No write changes the values that a pattern destructured, so
  such a guard is never marked as aliased, and it applies inside an inlined
  aliased condition too.
  A test `"a" in x` becomes a runtime test of `x` whose key is the string or
  decimal integer literal, spelled as a property path requires, so `"a"` and
  `0` are keys while `` `a` ``, `0x0`, and a `const` are not. The decomposed
  condition lists `x` with the objects of property guards, because `in` too
  narrows only a union with two object types. It adds no property guard:
  TypeScript narrows no object through `"a" in s.k`.
  A case of a `switch` becomes the condition that `discriminant === test`
  holds (`decompose_switch_case`), so a case on `s.kind` adds the property
  guard on `s`, and a case on a `const` that holds a property or on an
  element of a pattern adds the guards that a test of it adds. The
  discriminant decides how cases narrow (`SwitchKind`). A case of
  `switch (typeof x)` is `typeof x === test`, without property or pattern
  guards, because TypeScript narrows only the operand there, and a single
  case that is neither a string literal nor a template literal without
  substitutions makes no case narrow. A case of
  `switch (true)` is its test when it is truthy. The condition that a case
  did not match never tests the receivers of an optional chain, as in
  TypeScript, and for `switch (true)` it decomposes the test as falsy only
  when the test gives a boolean (`is_boolean_valued`). A `switch` that
  TypeScript can find exhaustive, without `default`, on a subject or `typeof`
  of one, with cases that all test a unit value or a kind of value, wraps
  its bypass guard in `FlowGuard::Bypass` and is recorded in
  `RootFlow::switches`, at most 64 per root.
  The body of a `for...in` loop whose head declares a variable starts with
  the condition that the loop's expression is not nullish
  (`decompose_for_in`), as TypeScript's `getTypeAtFlowAssignment` assumes
  at the assignment of that variable. It also tests each receiver of an
  optional chain, as in `for (const k in o?.v)`, and adds no property or
  pattern guard, because TypeScript narrows no object through
  `for (const k in s.kind)`.
  A test `x instanceof C` becomes a runtime test of `x` that holds the range
  of `C`, which inference resolves only when the test can narrow. The
  decomposed condition lists `x` with the objects of property guards, and
  also among the subjects that `instanceof` tests, because `instanceof`
  narrows only a type with an object type or `unknown`, which need not be a
  union. It adds no property guard either. In the true branch, the receivers
  of an optional chain narrow too, as in `o?.v instanceof C`, because an
  instance is never nullish.
  A truthiness test of an aliased condition, a `const` that is not exported,
  has no type annotation, and binds a name to an initializer, as `ok` in
  `const ok = x !== null; if (ok)`, also decomposes that initializer and
  combines it with the test of `ok` as `&&` combines its operands. Inside an
  inlined initializer, only a test of a variable whose value cannot change
  between the `const` and the test is kept, following TypeScript's
  `isConstantReference`: a test of `this`, of an exported variable, or of a
  variable declared by a `let` statement at the top level of a file becomes
  `Keep`. A test of a property path that starts at such a variable, or at a
  `this` in the root's own code, is wrapped in `FlowGuard::ReadonlyPath`,
  which applies it only when inference finds every property along the path
  readonly, and so are a property guard whose object is such a path and the
  test of such a receiver of an optional chain. Inlining stops
  once five aliased conditions enclose a test, where TypeScript stops too, so
  a deeper `const` tests only its own truthiness. When decomposition stops
  early, the subjects that inlined initializers read are mentioned too.
  A property guard from a `const` that holds a property, or from a test
  inside an inlined initializer, is marked as aliased: the `const` holds the
  value that the property had where the `const` is declared, and a write can
  change the property before the test.
- `writes.rs` finds the writes of a root that its graph doesn't model, as
  in a function called where it is defined or a class static block. A
  variable narrows only when each of its semantic writes in the root has a
  write node or runs only when called (`runs_only_when_called`). A write
  outside the root, of a variable that an enclosing root declares, never
  counts: it runs in another function, or in the enclosing root before or
  after the root's code. `WrittenPaths` tells
  the writes of paths and variables apart by `BuiltRoot::written`: a path
  that an unmodeled write may change doesn't narrow anywhere in its root,
  and an aliased property guard whose property or object the root writes at
  all becomes `Keep`. Collection also replaces a `FlowGuard::ReadonlyPath`
  guard with `Keep` when the root tests an object along its path with
  `instanceof`, a type-guard function, or `typeof` with a primitive kind or
  `"function"`, because such a test can replace a member of the object's
  type with one whose property is not readonly, while inference decides from
  the declared type. It lists the subjects of the other such guards in
  `RootFlow::readonly_tested`, and those that no other guard narrows in
  `RootFlow::only_readonly_tested`.
  The scan visits the root's own code, including function and arrow
  expressions called where they are defined and class static blocks, but not
  the bodies of other functions, methods, accessors, and constructors, which
  run only when called. A write changes the written path and every path that
  extends it. A write through an element access whose key `subjects.rs`
  accepts, as in `o["v"] = x`, writes the path that the key names, and a
  write through any other key, as in `o[k] = x`, changes every path below its
  object. Every `this` and `super` along a written target stands for the
  root's `this`, so `super.v = x` changes `this.v`.

A write node mentions its subject only where the collected types show that
it can change the type (`write_may_change`: `raw_may_be_union`,
`raw_may_have_literal`; a path's declared type isn't collected, so a path
write always mentions it). A subject that only writes mention is in
`RootFlow::assignment_only`, and its candidates are skipped, as those of
`RootFlow::property_only`, when its declared type is neither a union nor a
literal type. `RootFlow::uninitialized` lists the subjects that may be read
before they are assigned (`uninitialized_subjects`), and
`RootFlow::subject_writes` the values that each read subject's writes
assign.

Collection then keeps only the reads that a runtime test, a write, or a call
point of their subject can reach, and only for subjects that can narrow. A
call point leaves the classes of its subjects below as they are: whether it
narrows depends on its callee's signature, which inference finds before it
resolves anything else. A candidate made only by reads of one subject that
only call points reach records the root and the subject
(`ModuleFlow::call_only_candidate`), so inference can skip it when none of
the subject's call points narrows, and such a read doesn't decide how the
variable's declared type starts narrowing. The other reads of a subject that
call points mention are never skipped by the subject's class, and
`RootFlow::subject_calls` lists the call points of each read subject. It keeps a recorded `switch`
only when its bypass reaches a kept read, and lists those reads' points in
`RootFlow::bypassed` with the statements whose bypass reaches them. In the
same way, `RootFlow::called` lists the read points that the node of a
statement call reaches, with the calls, among the root's first 64
statement calls (`RootFlow::ending_calls`), whose node reaches them. A read of an element of a
pattern is also kept when a test of the pattern reaches it: its point becomes
a read point of the pattern, and `RootFlow::pattern_elements` maps the
element to the pattern. A subject that conditions narrow only through
property guards, `in`, `instanceof`, or type-guard functions whose targets
have only object types is in `RootFlow::property_only`, and
every pattern is. The declared type of a pattern is the collected type of the value
that its elements' collected `Destructure` types read from
(`ModuleFlow::pattern_type`). A property
guard or `in` can narrow only a union with at least two object types, so collection
drops a variable or a pattern in it when its collected type shows that it is not one,
without resolving anything (`raw_may_be_object_union` in
`biome_js_type_info`): a primitive, a single object type or class written in
place, a union with at most one object type, or the result of `new`. A
subject in `RootFlow::property_only` that an `instanceof` test or a
type-guard function whose target has only object types checks is also in
`RootFlow::instance_tested`, and collection drops it only when its
collected type shows that it has no object type
(`raw_may_narrow_by_instanceof`). A
variable whose type is a name, an import, or inferred from an expression stays. A variable can narrow in
the root that declares it and in the roots nested in that one, including an
import, when it is neither untyped nor annotated `any`, not supplied by a
destructuring default, which the declared type ignores, and each of its writes
in the root has a write node or runs only when called, as `writes.rs` decides
(`eligible_range`, and `encloses_eligible` once per variable of an enclosing
root). A `var` whose name another declaration of its scope declares, as a
second `var`, a function, or a parameter of its function does, doesn't narrow
(`is_redeclared`), because the semantic model resolves every reference to the
last declaration while each declarator writes the variable. A variable
whose type evolves with its writes, as `evolving_declaration` decides
following TypeScript's `autoType` and `autoArrayType`, can narrow whatever
its collected type, but only in the root that declares it
(`declared_range`): `eligible_range` refuses it to nested roots and
`closure_reads` skips it, because TypeScript gives it an implicit `any`
there. One that nothing writes or adds to narrows as any other variable.
An array also needs a write node for every addition that runs in the root's
flow (`mutations_are_modeled`). Such a variable stays a subject only when a
write node other than its initializer gives it a value, or, for a variable
initialized with `null` or `undefined`, when it has any write node other
than its initializer, and only those write nodes mention it, so only the
reads that they reach become
candidates, together with the reads of a `var` with an initializer that the
start reaches (`BuiltRoot::hoisted`), where it is still `undefined`.
`RootFlow::evolving` records how each such subject evolves, and none is in
`RootFlow::uninitialized` or `RootFlow::assignment_only`. A pattern can
narrow when its elements can and none
of them is written. A
property path can narrow when its variable can, or when it starts at a `this`
that is a subject, and the write scan finds no write without a write node
that may change it; the scan runs only when the root reads a tested path or
has an aliased property guard on an object that can narrow.
Those reads, plus the enclosing expressions that can use a narrowed operand,
become candidates. A test inside a nested function never reaches reads in the
enclosing one, and an array or object literal ends the chain of enclosing
candidates. A candidate that only reads of one variable in
`RootFlow::property_only`, or only reads of elements of one pattern that no
test of the elements themselves reaches, make a candidate records that
variable or pattern (`ModuleFlow::property_only_candidate`), so inference can
skip it as a whole when that variable or pattern can't narrow. A candidate
made only by reads of one variable records the variable too when only
`instanceof` tests and type-guard functions whose targets have only object
types reach those reads, even though other tests of the variable reach its
other reads; a second walk from those other tests finds such reads. A
variable that several roots read has one baseline, so
`ModuleFlow::property_only_bindings` and `ModuleFlow::assignment_only_bindings`
list it only when every root that keeps reads of it lists it in
`RootFlow::property_only` or `RootFlow::assignment_only`. A selected root is
built even when it declares no variable that can narrow, because it can read
one of an enclosing root, and it keeps no graph when relevance keeps no
read.
A read inside a closure that a root creates, directly or through other
closures, can continue the root's flow at the capture point of the outermost
one (`closure_reads`). The root looks for such reads of each variable that it
tests or writes, and of each element of a pattern that it narrows, among the
variable's semantic reads, and keeps those that TypeScript's
`checkIdentifier` continues (`capture_rule`): a `const` that is not an
import, and a parameter, a `catch` variable, or a `let`, except one that is
exported or declared at the top level of a file, read after its last
assignment. The last assignment comes from the variable's semantic writes,
each extended to the end of the outermost statement of TypeScript's list
around it that starts after the declaration (`assignment_position`, as
`extendAssignmentPosition` does), and a write in another function means that
no read continues. Every function between the read and the root must be such
a closure, and each must reach the next one in, or the read, in its own
reachable code (`continues_to`): as its builder found it, for a root that was
built, and otherwise when no `return`, `throw`, `break`, `continue`, or
infinite loop ends before it (`first_exit`), which is wider. Decorators,
computed keys of members, fields, types, `declare` statements, and `with`
statements stop the chain. Roots are collected inner first, so a root finds
whether a nested root already keeps the read. When none does, the read
becomes a read of the root at the capture point. Otherwise the outermost
nested root that keeps the read, or continues from it, gets a capture of the
variable (`RootFlow::captures`), the capture point becomes a read point of
the root, and the nested root's uninitialized start of the variable goes,
because the type at the capture point starts where the variable is declared.
Each closure adds one read point to the root, which all its reads share.

Other variable types are resolved only when a candidate read is queried,
because resolving them can be costly.

#### Tracked solve

Lookups skip flow for any expression that is not a candidate. Guards, the
subjects each condition may narrow, and reads refer to a subject by its id in
its root's subject table, so a test and a read of the same subject match. For
a candidate read of a variable, `infer_flow_binding_baseline` resolves the
variable's declared type once, for every root that reads it. For an import,
that is the type of what it imports, which the tracked queries of the
import's resolution give. For a read of a property path, the declared
type is the type of the member access at that read, computed from the
narrowed type of its object when the object has one, as TypeScript types
property reads. Facts about the variable therefore reach its
paths only through that member lookup. A read of an element of a pattern
first narrows the pattern at the read, from its declared type resolved by
`infer_flow_binding_baseline` for the pattern's range. When the pattern
narrows there, the element's declared type at the read is the element's
field on the narrowed pattern, resolved as its collected `Destructure` type is
(`resolve_destructure_field`), and the element's own tests narrow it from that
type, as a path's tests narrow it from its member type. `infer_flow_subject_states` then
applies the guards on every path to all reads of the subject in one root in a
single solve, and `infer_flow_subject_type` takes the result at the read's
incoming flow point. Both queries take the starting type as part of their
key, so reads of a subject share one solve when they start from the same
type.

A read of a variable that its root continues from an enclosing root
(`RootFlow::captures`) starts from the variable's type at the capture point
(`narrow_captured`): `captured_type` takes it from the solve of the enclosing
root, which starts from the declared type or from the enclosing root's own
capture, so a chain of closures resolves outward. That type is the baseline
in the key of the read's solve, so the closure's tests narrow it further, and
the solve of the enclosing root is the one its own reads share. A `let` that
may be read before it is assigned keeps the `undefined` of its start along
the chain. A read in a closure starts it without `undefined` when some code
assigns it, as TypeScript's `checkIdentifier` assumes such a variable
assigned in a nested function (`FlowCapture::assumes_initialized`,
`FlowRead::assumes_initialized`), and otherwise gets the declared type where
`undefined` remains although the declared type lacks it. Complete module
inference keeps the type at each capture point in `LocalFlow`.

A write node applies `apply_write`, following `getTypeAtFlowAssignment`:
the declared type reduced by the assigned type (`narrow_by_assignment`,
following `getAssignmentReducedType`), `base_literal_type` of the incoming
type for a compound assignment, or the declared type for a write without a
value or of a proper prefix of the path. The assigned types are part of the
solve's key (`FlowSubjectRootInput::assigned`, `None` without such a write),
resolved by the reading context, never by the solve's fresh one, and typed
without narrowing (`assigned_types`), so no assigned type depends on a
solve. Lookups share them per root and subject through
`infer_flow_assigned_types`. A subject in `RootFlow::uninitialized` starts
with `undefined`, and a read where `undefined` remains although the declared
type lacks it gets the declared type.

A read of a subject in `RootFlow::evolving` has its own solve, selected by
`FlowSubjectRootInput::evolving` (`narrow_evolving`), whose state is the type
that the writes give, from `undefined` at the start, not the declared type
narrowed. `apply_evolving_write` follows `getTypeAtFlowAssignment` and
`getTypeAtFlowArrayMutation` for such subjects: a variable takes the
assigned type, and an array's state is `Array<T>`, which `EmptyArray` starts
at `Array<never>` and each `Elements` write extends with the value's type,
so a join gives a union of arrays whose element types
`evolving_array_element` unions at the read. Their assigned types are
widened by `evolving_assigned_type`, which gives `Unknown` for a value that
Biome may type otherwise than TypeScript, apart from a covered value, which
gives Biome's type with its literal types widened, and a write of the
subject without such a value gives `Unknown`. A variable whose state is
`Unknown` is Unknown at the read, while an array whose state is `Unknown`,
or to which nothing was added, keeps its declared type
(`evolving_array_read`). Runtime tests narrow a variable from its incoming
type, which stands for its declared type, and never narrow an array. When
the values that the root assigns are all `null` or all `undefined`, and
every write of a variable assigns a value, the read keeps its declared type
without a solve, and an array ignores undetermined values there. When every
write of a variable gives `null`, `undefined`, or `Unknown`, and at least
one gives `Unknown` (`RootFlow::has_valueless_write`), the read is `Unknown`
without a solve.

Complete module inference evaluates flow candidates in its own resolution
context instead of through `infer_flow_expression_type`,
`infer_flow_binding_baseline`, and `infer_flow_assigned_types`. When it
reaches expressions, that context has resolved every raw type of the module,
so a variable's declared type, the member access that gives a property path
its declared type, and the assigned types (kept in `LocalFlow`) read types
it already holds, while the queries would resolve them again in a fresh
context. It still solves through `infer_flow_subject_states` and
`infer_flow_subject_type`. Cycle
fallback and the import-depth fallback read imports differently from the
flow queries, so they keep using the queries.

A subject in `RootFlow::property_only` starts from `property_baseline`
instead of `narrowing_baseline`. It has no baseline, so its reads keep their
declared types without a solve, unless its flattened type is a union with at
least two object types (`may_narrow_by_property`), or, for a subject in
`RootFlow::instance_tested`, has an object type or is `unknown`
(`may_narrow_by_instanceof`). A pattern starts from
`pattern_baseline`, which is `property_baseline` without the `null` and
`undefined` members, because destructuring throws on them. To flatten it, local types
of the module resolve through the context instead of `infer_local_type`, so in
complete module inference, which holds all of them, finding that most such
variables can't narrow costs cache reads. Before evaluating a candidate that
`ModuleFlow::property_only_candidate` names, both paths check that variable's
baseline and skip the candidate when there is none, without finding its
syntax node.

A read of a property path in `RootFlow::readonly_tested` decides whether the
path is readonly once its baseline exists (`is_readonly_path`): at every
step, the declared type of the object, the type of its expression resolved
from the raw tables without narrowing, must declare the property readonly in
every member other than `null` and `undefined` (`has_readonly_property` in
`biome_js_type_info`). The answer is part of the key of the solve queries, so
`FlowGuard::ReadonlyPath` guards apply only in the solves of readonly paths,
and paths without such guards share their solves as before. A path in
`RootFlow::only_readonly_tested` that is not readonly skips the solve. The
check resolves through the context that evaluates the read, so in complete
module inference it reads the types that context already holds, and that
context keeps the answer per root and subject.

A read whose point the bypass of a recorded `switch` reaches, and that comes
after the statement's cases in the source, decides once its baseline exists
whether that statement is exhaustive (`switch_is_exhaustive`), as
TypeScript's `computeExhaustiveSwitchStatement` does: the guard of the
bypass, applied to the type of the discriminant's read at the statement,
must leave `never`. That type is the read's flow-sensitive type when
narrowing changes it, and its ordinary type otherwise. A case that compares
with an imported value counts only when inference finds a unit value there,
because TypeScript finds no statement exhaustive when a case's type is not a
unit type, while a case that narrows nothing would leave the other cases to
decide. The answer is part of
the key of the solve queries (`FlowSubjectRootInput::exhaustive`, one bit per
statement), and `FlowGuard::Bypass` gives `never` in a solve whose key has
its bit. A read never asks about a statement whose cases come after it, so
the read of a discriminant never needs its own statement's answer. Lookups
share the answer through `infer_flow_switch_exhaustive`; complete module
inference decides it in its own context and keeps it per root and
statement.

A read of a subject in `RootFlow::subject_calls` resolves what its call
points do in the reading context (`call_effects`), never in the solve's
fresh one, and the solve's key carries the effects that narrow
(`FlowSubjectRootInput::calls`, `None` when none does, so other subjects
share their solves). The effect of a call point (`FlowCallEffect`) follows
TypeScript's `getEffectsSignature`: a call in a condition takes the single
signature of its callee's type without `null` and `undefined`, which must
return a type predicate, and a statement call takes that of its callee's
explicit type, which must assert. `has_explicit_callee` checks an import
through the declaration that it names (`imported_declaration_is_explicit`)
and each member access through the declared type of its object
(`has_explicit_member`). A generic signature or more than one signature
gives no effect. `FlowGuard::Call` then narrows the argument or the object
that the signature names with `narrow_by_type_predicate`, or applies the
argument's decomposed condition for `asserts value`. Lookups share each
effect through `infer_flow_call_effect`, keyed by the range of the call;
complete module inference resolves it in its own context and keeps it per
call. When a call point narrows a subject whose reads elsewhere narrow only
through tests of its shape or only through writes, the read starts from the
whole declared type (`called_binding_baseline`).

A read whose point the node of a statement call reaches decides, after the
cheap gates and once its baseline exists, which of those calls never
return (`ending_calls`): their effect is `FlowCallEffect::Ends` when the
return type annotation of the signature's declaration is `never`
(`has_annotated_callee`), as TypeScript's
`hasTypePredicateOrNeverReturnType` requires, or when an `asserts value`
signature gets an argument that is always false. Such an effect narrows
nothing, so `FlowSubjectRootInput::calls` leaves it out. A read that no path
reaches without crossing one of these calls keeps its declared type without
a solve (`RootFlow::reaches_past`), as TypeScript gives a read whose flow
type is `unreachableNeverType` its declared type, and so do the reads of a
closure created there. Otherwise the solve's key carries the calls
(`FlowSubjectRootInput::ends`, one bit per call, zero for reads that no such
call reaches, so they share their solves), and `FlowGuard::Call` gives
`never` for each of them, which adds nothing where paths join.

`src/db/type_inference/flow.rs` applies each runtime test with `narrow_type`
from `biome_js_type_info`. Where branches meet, or a loop returns to an earlier
point, it keeps every type still possible on a reachable path. A read on a path
that no value can take gets type Unknown, not `never`. `flow_expressions.rs`
uses narrowed operands when inferring member accesses, calls, logical and
conditional expressions, `await`, the comma operator, and `+`. A conditional
expression is the union of its branches, as in TypeScript, so its test matters
only for a branch that reads the tested variable, which `conditional_branch`
leaves out when the test gives that variable no value there, as TypeScript's
`never` does. Complete module inference narrows each read as an individual
expression query does, from the declared type its own context resolved.

`narrow_type` and `falsy_values` take a resolver, as `keyof` and conditional
types do: `ResolutionCtx::narrowing_resolver` resolves handles in the context
that solves the read. Narrowing calls it only to compare an object type with
primitives and functions in a falsy or `typeof` test, for example to find that
no falsy primitive has the members of `interface Foo { a: number }`. Tests of
primitive types never call it. The comparison itself, including the member
lookups on wrapper types such as `String`, lives in `biome_js_type_info`.
A property guard applies `narrow_by_property`, which looks up the property in
every member of the union with the same resolver and keeps the members whose
property type can share a value with what the test leaves of the union of
those types, following TypeScript's `narrowTypeByDiscriminant`. An `in` test
applies `narrow_by_in_operator`, following TypeScript's
`narrowTypeByInKeyword`: `property_presence` in `type_relation.rs` looks up the
property in each member and treats object types as closed, but reports that
it can't tell for a type whose member list Biome may not know completely,
such as a class or a built-in type. The solve resolves the key from
`raw_expressions` only when the subject's declared type is a union with two
object types. An `instanceof` test applies `narrow_by_instanceof`, following
TypeScript's `narrowTypeByInstanceof`: `instance_types` in `type_relation.rs`
finds the instance types of the constructor, and `is_derived_from` compares
the members of the narrowed type with them by following `extends` clauses to
the same declaration. The solve resolves the constructor from
`raw_expressions` only when `may_narrow_by_instanceof` accepts the subject's
declared type. A call to a type-guard function applies
`narrow_by_type_predicate`, following TypeScript's `getNarrowedType`: a
primitive target, such as `string` or `"ready"`, stands for a `typeof` or
equality test, and other targets compare the members of the narrowed type
with those of the target in TypeScript's subtype relation (`is_subtype` in
`type_relation.rs`). The solve resolves the target from the guard's
collected signature, and, for a target with only object types, only when
`may_narrow_by_instanceof` accepts the subject's declared type. It memoizes
the result of each property guard, `in` test, `instanceof` test, and call to
a type-guard function by guard and incoming type, because a loop applies the
same guard again.

#### Caching

Collected flow belongs to the module input, so an edit replaces it together
with the raw type tables, even when variable declarations and references stay
the same. Salsa caches the type work: all reads of a variable share the type
that narrowing starts from, its reads in one root share one narrowing
calculation, and reads at the same flow point share their result. Lookups and
complete module inference share the calculation and the results when they
start from the same type. These caches recompute when the module input
changes; they do not track edits separately for each function.

#### Work limits

Collection, condition decomposition, and the solve each have work limits. A
root that exceeds a limit while its flow is collected gets no graph, so its
reads keep their declared types. The limit on finding the reads that a test
can reach is spent on variables before property paths and subjects in
`RootFlow::property_only`; when it runs out on those, the root keeps its graph
and only their reads lose narrowing. If decomposing a condition cannot finish
within its work limit, the guard makes the result unknown rather than assuming
the expression is unaffected. When the shared solve exceeds its work limit or
meets a query cycle, `infer_flow_subject_type` solves the read's point on its
own, so a read never loses precision because other reads share its root. If
that solve also exhausts its work limit or a query cycle occurs, the result is
unknown. Callers must respect that result rather than replace it with a more
confident answer from raw type information.

### Analyzer-facing requests

A request is a typed operation used by analyzer rules. It stores module and
source-range inputs, composes tracked queries through
`TypeInferenceRequestContext`, and returns one defined result, such as a
normalized expression type or the expected type of a call argument.

Requests live under `src/type_inference/requests/`. Every concrete request
implements `TypeInferenceRequestMetadata` beside its execution implementation,
which gives the request a stable profile ID and human label.

`UnreachableCodeClassificationRequest` tells whether no path reaches a
statement, as TypeScript decides it. It asks the tracked query
`infer_flow_reachability` about the execution root around the statement,
which builds that root's graph from the module's syntax when it runs
(`build_reachability`), with the builder that collection uses, so
reachability costs nothing until a rule or an inferred return type asks. The
query resolves the effect
of each statement call, through `infer_flow_call_effect` when collection
kept the call point, decides each `switch` statement as
`switch_is_exhaustive` does, and walks from the root's start without
crossing a call that never returns or the bypass of an exhaustive `switch`.
A narrowed type, even `never`, never makes code unreachable. A work limit or
a query cycle makes the answer `Indeterminate`.

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

### Tracked queries

Tracked queries are the incremental computation units. Expression, binding,
module type, and export queries resolve one entry from a raw table.
Normalization, call matching, and whole-module inference have separate query
boundaries.

The query facade is `src/db/queries/type_inference.rs`. Files under
`src/db/queries/type_inference/` are organized by subject: `lookups`, `exports`,
`augmentations`, `promises`, `calls`, `normalization`, and `module_types`. Compound Salsa keys
live in the `interned` module. Algorithms used by one family may live in a child module;
resolution helpers shared by multiple families live under
`src/db/type_inference/`.

### Resolver helpers

Cross-module relationships must remain references to data owned by the source
module. Do not copy or clone inferred data from another module into the current
module, including behind `Arc`.

`ResolutionCtx` protects itself against endless recursion in two ways: it
remembers which types it is currently resolving, and it gives up after a fixed
number of nested steps. It also remembers the types it has already resolved.
All of this is stored inside one context, so a newly created context starts
with none of it.

For this reason, do not create a new context in the middle of a resolution and
keep resolving through it. The new context does not know what the old one was
working on. Two declarations that refer to each other can then keep resolving
each other until the program runs out of stack space and crashes.

Create a new context only where something outside the context still limits the
work:

- at the start of a tracked query. If the same query is requested again before
  it finishes, Salsa returns the query's fallback result instead of running it
  again;
- when following an import, because each import uses up part of a limited
  import budget;
- when sharing the declaration evaluator. It keeps one record of which
  declarations depend on which, across all modules, and uses it to detect
  loops;
- when looking up a member of a namespace import, because that lookup has its
  own step limit that decreases with each step.

To resolve a type that belongs to another module, use a helper that follows one
of these paths, such as `ResolutionCtx::resolve_foreign_type_id`. For example,
in these two files each interface's type parameter refers to the other file's
interface:

```ts
// a.ts
import type { B } from "./b";
export interface A<T extends B<any>> {}

// b.ts
import type { A } from "./a";
export interface B<U extends A<any>> {}
```

Resolving `A`'s type parameter requires `B`, and resolving `B`'s type parameter
requires `A`. If each step creates a new context, no context notices the loop.

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

Request IDs are stable identities used to aggregate records for the same
request contract. Use lowercase words separated by hyphens after the
`request.` prefix:

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

Each recorded query execution retains its own implementation reference, so
multiple query bodies can share one family. Whole-module records retain the
implementation responsible for widening inference to complete tables. Code
references correspond to the Biome version that produced a report.

Do not read `ModuleInfo::path` from inside a tracked query for profiling. The
path is a Salsa input, so reading it changes the query's dependency set. The
outer request boundary snapshots document paths before entering tracked
queries. Query instrumentation records only work nested under a production
request and carries only module identities and `TextRange` values.

## Profiling inference

Use `--profile-type-inference` to inspect CLI inference work:

```shell
biome lint --profile-type-inference path/to/reproduction
biome check --profile-type-inference path/to/reproduction
```

The report groups records by source file. The default output shows the ten files
with the highest cumulative request time. Within each file, it shows the top
five request groups, top eight query groups, and top five whole-module groups.
It includes the total number of groups, source records, and completed executions
so omitted work remains visible.

Use `--verbose` to show detailed metrics and implementation references for the
top five request records, top eight query records, and top five whole-module
records in each file. Verbose output includes every file while retaining the
per-file limits.

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
- import and declaration cycles, including cycles through generic parameter
  constraints and defaults declared in different modules;
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
- [ ] Every `ResolutionCtx` created during resolution sits behind a tracked
      query, a spent import or step budget, or the shared declaration
      evaluator.
- [ ] Namespace and import-chain breadth were reviewed.
- [ ] Whole-module inference is absent or explicitly justified.
- [ ] Cross-module data remains owned by its source module.
- [ ] Correctness and Salsa execution tests cover the flow.
- [ ] The metadata trait has a stable ID and label, and the request has an
      implementation reference.
