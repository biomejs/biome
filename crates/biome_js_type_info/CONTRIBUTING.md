# Biome Type Architecture

In order to contribute to Biome's type inference, it's good to understand our
type architecture.

## Architecture Constraints

The main thing to understand about Biome is that we put our **User Experience**
front and center. Whether it's our
[Rule Pillars](https://biomejs.dev/linter/#rule-pillars), our Batteries-Included
approach, the
[`biome migrate`](https://biomejs.dev/guides/migrate-eslint-prettier/) command
for users coming from other tools, or our focus on IDE support, we know that
without users we are nowhere.

And it's precisely this last point, our IDE support, that's so important here.
IDE support was already an important consideration in our
[approach to multi-file support](https://github.com/biomejs/biome/discussions/4653),
and this seeps through into our type inference architecture.

For many tools, such as bundlers, it is sufficient to optimise the performance
for CLI usage. Development servers may have an interest in optimising hot-reload
performance as well, but they tend to do so by pushing responsibility to the
client instead of rebuilding their bundles faster.

For Biome, priorities are different: If a user changes file A, they want the
diagnostics for file B to update in their IDE, regardless of whether it has
dependencies on file A. Updates need to happen near-instantaneously, and
the IDE is not a client we can offload responsibility to.

## Module Graph

Biome's [module graph](../biome_module_graph/) is central to our multi-file
support and is designed with these considerations in mind. And our type
architecture is built upon this module graph. The module graph is effectively
just a [fancy hash map](https://github.com/ibraheemdev/papaya/) that contains
entries for every module (every JS/TS file in a repository), including metadata
such as which other modules that module depends on, which symbols it exports,
and yes, also which types it contains.

The key constraint the module graph operates under is this: No module may copy
or clone data from another module, not even if that data is behind an
[`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html).
The reason for this is simple: Because of our focus on IDE support, we maintain
the idea that any module in the module graph may be updated at any point in time
due to a user action. Whenever that happens, we shouldn't have trouble figuring
out which other modules need their data to be invalidated, which might happen if
modules were to copy each other's data.

Some other tools use complex systems to track dependencies between modules, both
explicit dependencies as well as implicit ones, so they can do very granular
cache invalidation. With Biome we're trying radical simplicity instead: just
make sure we don't have such dependencies between entries in our module graph.
So far, that appears to be working well enough, but naturally, it comes with its
own challenges.

## Type Data Structures

In Biome, the most basic data structure for type information is a giant `enum`,
called `TypeData`, defined in [`type_info.rs`](src/type_info.rs).

This enum has many different variants in order to cover all the different kinds
of types that TypeScript supports. But a few are specifically
interesting to mention here:

* `TypeData::Unknown` is important because our implementation of type inference
  is only a partial implementation. Whenever something is not implemented, we
  default to `Unknown` to indicate that, well, the type is unknown. This is
  practically identical to the `unknown` keyword that exists in TypeScript, but
  we do have a separate `TypeData::UnknownKeyword` variant for that so that we
  can distinguish between situations where our inference falls short versus
  situations where we _can't_ infer because the user explicitly used `unknown`.
  They're semantically identical, so the difference is only for measuring the
  effectiveness of our inference.
* Complex types such as `TypeData::Function` and `TypeData::Object` carry extra
  information, such as definitions of function parameters and object properties.
  Because function parameters and object properties themselves also have a type,
  we can recognise that `TypeData` is potentially a circular data structure.
* Rather than allowing the data structure itself to become circular/recursive,
  we use `TypeReference` to refer to other types. And because we try to avoid
  duplicating types if we can, we have `TypeData::Reference` to indicate a type
  is nothing but a reference to another type.

## Why Use Type References?

Theoretically, we _could_ use `Arc` and let types reference each other directly.
But remember that module graph mentioned above? If a type from module A were to
reference a type from module B, and we'd store the type from module B behind an
`Arc`, then what would happen if module B were replaced in our module graph?

The result would be that the module graph would have an updated version of
module B, but the types in module A would hang on to old versions of those
types, because the `Arc` would keep those old versions alive. Of course we could
try to mitigate that, but solutions tend to become either very complex or very
slow, and possibly both.

We wanted simplicity, so we opted to sidestep this problem using
`TypeReference`s instead.

But even though the constraints of our module graph were our primary reason for
choosing to use type references, they have other advantages too:

* By not putting the type data behind `Arc`s, we can store data for multiple
  types in a linear vector. This improves data locality, and with it,
  performance.
* Storing type data in a vector also makes it more convenient to see which types
  have been registered, which in turn helps with debugging and test snapshots.
* Not having to deal with recursive data structures made some of our algorithms
  easier to reason about as well. If we want to perform some action on every
  type, we just run it on the vector instead of traversing a graph
  while tracking which parts of the graph have already been visited.

## Type Resolution Phases

Type references come in multiple variants:

```rs
enum TypeReference {
    Qualifier(TypeReferenceQualifier),
    Resolved(ResolvedTypeId),
    Import(TypeImportQualifier),
    Unknown,
}
```

The reason for these variants is that _type resolution_, the process of
resolving type references, works in multiple phases.

Biome recognises three levels of type inference, and has different resolution
phases to support those...

### Local Inference

_Local inference_ is when we look at an expression and derive a type definition.
For example, consider this seemingly trivial example:

```js
a + b
```

It looks like this should be easy, but because local inference doesn't have any
context such as definitions from surrounding scopes, it will never be able to
understand what `a` or `b` refers to.

Therefore, local inference cannot resolve this to a _concrete_ type. But with
the help of type references, we can rewrite the expression into something
useful:

```rs
TypeData::TypeofExpression(TypeofExpression::Addition {
    left: TypeReference::from(TypeReferenceQualifier::from_name("a")),
    right: TypeReference::from(TypeReferenceQualifier::from_name("b"))
})
```

Local inference doesn't do any type resolution yet, it only creates type
references. So in most cases we won't know a concrete type yet, but it still
provides a useful starting point for later inference.

Local inference is implemented in [`local_inference.rs`](src/local_inference.rs).

### Module-Level ("Thin") Inference

_Module-level inference_, sometimes called: "thin inference", allows us to put
those types from the local inference phase into context. This is where we look
at a module as a whole, take its import and export definitions, look at the
scopes that are created, as well as the types derived using local inference, and
apply another round of inference to it.

Within the scope of a module, we do our first round of type resolution: We take
all the references of the variant `TypeReference::Qualifier` (the only ones
created thus far), and attempt to look them up in the relevant scopes. If a
local scope declaration is found, we consider the type _resolved_ and convert
the reference into a `TypeReference::Resolved` variant with an associated
`ResolvedTypeId` structure, which looks like this:

```rs
struct ResolvedTypeId(ResolverId, TypeId)
```

Both `ResolverId` and `TypeId` are a `u32` internally, so this is a really
compact representation for referencing another type, not bigger than a regular
64-bit pointer. The `TypeId` is a literal index into a vector where types are
stored, while the `ResolverId` is a slightly more complex identifier that allows
us to determine _which_ vector we need to look in, because every module will
have its own vector (and there are a few more places to look besides).

Another possibility is that the qualifier references a binding from an
_import statement_, such as `import { a } from "./a.ts"`. In this case, we
cannot fully resolve the type yet, because thin inference cannot look beyond the
boundaries of its own module. But we can mark this case as an explicit import
reference. This is what the `TypeReference::Import` variant is for.

And if the qualifier exists neither as a local declaration, nor as an imported
binding, then we know it must come from the global scope, where we can find
predefined bindings such as `Array` and `Promise`, or the `window` object. If a
global reference is found, it also gets converted to a `TypeReference::Resolved`
variant, where the `ResolverId` can be used to indicate this type can be looked
up from a vector of predefined types.

But ultimately, if not even a global declaration was found, then we're at a loss
and fall back to `TypeReference::Unknown`.

Thin inference is implemented in
[`js_module_info/collector.rs`](../biome_module_graph/src/js_module_info/collector.rs).

## Full Inference

_Full inference_ is where we can tie all the loose ends together. It's where we
have the entire module graph at our disposal, so that whenever we run into an
unresolved `TypeReference::Import` variant, we can resolve it on the spot, at
which point it becomes a `TypeReference::Resolved` variant again.

Today, results from our full inference cannot be cached for the same reason
we've seen before: Such a cache would get stale the moment a module is replaced,
and we don't want to have complex cache invalidation schemes.

Full inference is implemented in
[`scoped_resolver.rs`](../biome_module_graph/src/js_module_info/scoped_resolver.rs).

## Type Resolvers

The thing about having all these type references all over the place is that you
need to perform explicit type resolution to follow these references. That's why
we have _type resolvers_. There's a `TypeResolver` trait, defined in
[`resolver.rs`](src/resolver.rs). As of today, we have 4 implementations of it:

* **`HardcodedSymbolResolver`**. This one is purely for test purposes.
* **`GlobalsResolver`**. This is the one that is responsible for resolving
  globals such as `Promise` and `Array`. The way we do this is still rather
  primitive with hardcoded, predefined symbols. At some point we probably should
  be able to use TypeScript's own global `.d.ts` files, such as
  [`es2023.array.d.ts`](https://github.com/microsoft/TypeScript/blob/main/src/lib/es2023.array.d.ts),
  directly.
* **`JsModuleInfoCollector`**. This one is responsible for collecting
  information about a module, and for performing thin inference on it.
* **`ModuleResolver`**. This is the one that is responsible for our actual full
  inference, that is able to infer _across_ modules. Compare this to the
  `JsModuleInfoCollector` which only collects information inside a single
  module.

I've mentioned before that types are stored in vectors. Those type vectors are
stored inside `TypeStore` structures which are kept inside the various
`TypeResolver` implementations. The nice thing about `TypeStore` is that it
provides lookups that are as fast as a vector when the `TypeId` is known, while
also maintaining a hash table for when the `TypeId` is not known.

## Flattening

Apart from type resolution, there's one other, last important piece to type
inference: _type flattening_.

Let's look at the `a + b` expression again. After local inference, it was
interpreted as this:

```rs
TypeData::TypeofExpression(TypeofExpression::Addition {
    left: TypeReference::from(TypeReferenceQualifier::from_name("a")),
    right: TypeReference::from(TypeReferenceQualifier::from_name("b"))
})
```

But at some point, supposedly one of the resolvers is going to be able to
resolve `a` and `b`, and the expression becomes something such as:

```rs
TypeData::TypeofExpression(TypeofExpression::Addition {
    left: TypeReference::from(ResolvedTypeId(/* resolver ID and type ID */)),
    right: TypeReference::from(ResolvedTypeId(/* resolver ID and type ID */))
})
```

At this point we know the actual types we are dealing with. If the types for
both `left` and `right` resolve to `TypeData::Number`, the entire expression can
be _flattened_ to `TypeData::Number`, because that's the result of adding two
numbers. And in most other cases it will become `TypeData::String` instead.

Flattening is implemented in [`flattening.rs`](src/flattening.rs).

## `ResolvedTypeData`

One more important data structure to be aware of is `ResolvedTypeData`. Whenever
we request type data from a resolver, we don't receive a `&TypeData` reference,
but `ResolvedTypeData`.

The reason for this structure is that it tracks the `ResolverId` so we remember
where this type data was found. This is important if you want to resolve
`TypeReference`s that are part of the type data and you need to make subsequent
calls to the resolver.

`ResolvedTypeData` has an `as_raw_data()` method that returns the raw
`&TypeData` reference. This is often used for matching against the variants of
the `TypeData` enum. But keep in mind that any data that you retrieve this way
cannot be used with a resolver unless you explicitly and manually apply the
right `ResolverId` to it! Unfortunately we cannot enforce this through the type
system, and **mistakes can lead to panics**.
