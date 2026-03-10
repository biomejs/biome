---
name: type-inference
description: Guide for working with Biome's module graph and type inference system. Use when implementing type-aware lint rules, understanding type resolution, working on the module graph infrastructure, or implementing type inference for new features.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

## Purpose

Use this skill when working with Biome's type inference system and module graph. Covers type references, resolution phases, and the architecture designed for IDE performance.

## Prerequisites

1. Read `crates/biome_js_type_info/CONTRIBUTING.md` for architecture details
2. Understand Biome's focus on IDE support and instant updates
3. Familiarity with TypeScript type system concepts

## Key Concepts

### Module Graph Constraint

**Critical rule**: No module may copy or clone data from another module, not even behind `Arc`.

**Why**: Any module can be updated at any time (IDE file changes). Copying data would create stale references that are hard to invalidate.

**Solution**: Use `TypeReference` instead of direct type references.

### Type Data Structure

Types are stored in `TypeData` enum with many variants:

```rust
// Simplified — see crates/biome_js_type_info/src/type_data.rs for the full enum
enum TypeData {
    Unknown,                            // Inference not implemented
    Global,                             // Global type reference
    BigInt, Boolean, Null, Number,      // Primitive types
    String, Symbol, Undefined,
    Function(Box<Function>),            // Function with parameters
    Object(Box<Object>),                // Object with properties
    Class(Box<Class>),                  // Class definition
    Interface(Box<Interface>),          // Interface definition
    Union(Box<Union>),                  // Union type (A | B)
    Intersection(Box<Intersection>),    // Intersection type (A & B)
    Tuple(Box<Tuple>),                  // Tuple type
    Literal(Box<Literal>),              // Literal type ("foo", 42)
    Reference(TypeReference),           // Reference to another type
    TypeofExpression(Box<TypeofExpression>), // typeof an expression
    // ... plus Conditional, Generic, TypeOperator, InstanceOf,
    //     keyword variants (AnyKeyword, NeverKeyword, VoidKeyword, etc.)
}
```

### Type References

Instead of direct type references, use `TypeReference`:

```rust
enum TypeReference {
    Qualifier(Box<TypeReferenceQualifier>),  // Name-based reference
    Resolved(ResolvedTypeId),                 // Resolved to type ID
    Import(Box<TypeImportQualifier>),         // Import reference
}
```

**Note:** There is no `Unknown` variant. Unknown types are represented as `TypeReference::Resolved(GLOBAL_UNKNOWN_ID)`. Use `TypeReference::unknown()` to create one.

## Type Resolution Phases

### 1. Local Inference

**What**: Derives types from expressions without surrounding context.

**Example**: For `a + b`, creates:
```rust
TypeData::TypeofExpression(TypeofExpression::Addition {
    left: TypeReference::from(TypeReferenceQualifier::from_name("a")),
    right: TypeReference::from(TypeReferenceQualifier::from_name("b"))
})
```

**Where**: Implemented in `local_inference.rs`

**Output**: Types with unresolved `TypeReference::Qualifier` references

### 2. Module-Level ("Thin") Inference

**What**: Resolves references within a single module's scope.

**Process**:
1. Takes results from local inference
2. Looks up qualifiers in local scopes
3. Converts to `TypeReference::Resolved` if found locally
4. Converts to `TypeReference::Import` if from import statement
5. Falls back to globals (like `Array`, `Promise`)
6. Uses `TypeReference::Unknown` if nothing found

**Where**: Implemented in `js_module_info/collector.rs`

**Output**: Types with resolved local references, import markers, or unknown

### 3. Full Inference

**What**: Resolves import references across module boundaries.

**Process**:
1. Has access to entire module graph
2. Resolves `TypeReference::Import` by following imports
3. Converts to `TypeReference::Resolved` after following imports

**Where**: Implemented in `js_module_info/module_resolver.rs`

**Limitation**: Results cannot be cached (would become stale on file changes)

## Working with Type Resolvers

### Available Resolvers

```rust
// 1. For tests
HardcodedSymbolResolver

// 2. For globals (Array, Promise, etc.)
GlobalsResolver

// 3. For thin inference (single module)
JsModuleInfoCollector

// 4. For full inference (across modules)
ModuleResolver
```

### Using a Resolver

```rust
use biome_js_type_info::{TypeResolver, ResolvedTypeData};

fn analyze_type(resolver: &impl TypeResolver, type_ref: TypeReference) {
    // Resolve the reference
    let resolved_data: ResolvedTypeData = resolver.resolve_type(type_ref);

    // Get raw data for pattern matching
    match resolved_data.as_raw_data() {
        TypeData::String => { /* handle string */ },
        TypeData::Number => { /* handle number */ },
        TypeData::Function(func) => { /* handle function */ },
        _ => { /* handle others */ }
    }

    // Resolve nested references
    if let TypeData::Reference(inner_ref) = resolved_data.as_raw_data() {
        let inner_data = resolver.resolve_type(*inner_ref);
        // Process inner type
    }
}
```

### Type Flattening

**What**: Converts complex type expressions to concrete types.

**Example**: After resolving `a + b`:
- If both are `TypeData::Number` → Flatten to `TypeData::Number`
- Otherwise → Usually flatten to `TypeData::String`

**Where**: Implemented in `flattening.rs`

## Common Workflows

### Implement Type-Aware Lint Rule

```rust
use biome_analyze::Semantic;
use biome_js_type_info::{TypeResolver, TypeData};

impl Rule for MyTypeRule {
    type Query = Semantic<JsCallExpression>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        // Get type resolver from model
        let resolver = model.type_resolver();

        // Get type of expression
        let expr_type = node.callee().ok()?.infer_type(resolver);

        // Check the type
        match expr_type.as_raw_data() {
            TypeData::Function(_) => { /* valid */ },
            TypeData::Unknown => { /* might be valid, can't tell */ },
            _ => { return Some(()); /* not callable */ }
        }

        None
    }
}
```

### Navigate Type References

```rust
fn is_string_type(resolver: &impl TypeResolver, type_ref: TypeReference) -> bool {
    let resolved = resolver.resolve_type(type_ref);

    // Follow references
    let data = match resolved.as_raw_data() {
        TypeData::Reference(ref_to) => resolver.resolve_type(*ref_to),
        _other => resolved,
    };

    // Check the resolved type
    matches!(data.as_raw_data(), TypeData::String)
}
```

### Work with Function Types

```rust
fn analyze_function(resolver: &impl TypeResolver, type_ref: TypeReference) {
    let resolved = resolver.resolve_type(type_ref);

    if let TypeData::Function(func_type) = resolved.as_raw_data() {
        // Access parameters
        for param in func_type.parameters() {
            let param_type = resolver.resolve_type(param.type_ref());
            // Analyze parameter type
        }

        // Access return type
        let return_type = resolver.resolve_type(func_type.return_type());
    }
}
```

## Architecture Principles

### Why Type References?

**Advantages**:
1. **No stale data**: Module updates don't leave old types in memory
2. **Better performance**: Types stored in vectors (data locality)
3. **Easier debugging**: Can inspect all types in vector
4. **Simpler algorithms**: Process vectors instead of traversing graphs

**Trade-off**: Must explicitly resolve references (not automatic like `Arc`)

### ResolvedTypeId Structure

```rust
struct ResolvedTypeId(ResolverId, TypeId)
```

- `TypeId` (u32): Index into a type vector
- `ResolverId` (u32): Identifies which vector to use
- Total: 64 bits (compact representation)

### ResolvedTypeData

Always work with `ResolvedTypeData` from resolver, not raw `&TypeData`:

```rust
// Good - tracks resolver context
let resolved_data: ResolvedTypeData = resolver.resolve_type(type_ref);

// Be careful - loses resolver context
let raw_data: &TypeData = resolved_data.as_raw_data();
// Can't resolve nested TypeReferences without ResolverId!
```

## Tips

- **Unknown types**: `TypeData::Unknown` means inference not implemented, treat as "could be anything"
- **Follow references**: Always follow `TypeData::Reference` to get actual type
- **Resolver context**: Keep `ResolvedTypeData` when possible, don't extract raw `TypeData` early
- **Performance**: Type vectors are fast - iterate directly instead of recursive traversal
- **IDE focus**: All design decisions prioritize instant IDE updates over CLI performance
- **No caching**: Full inference results can't be cached (would become stale)
- **Globals**: Currently hardcoded, eventually should use TypeScript's `.d.ts` files

## Common Patterns

```rust
// Pattern 1: Resolve and flatten
let type_ref = expr.infer_type(resolver);
let flattened = type_ref.flatten(resolver);

// Pattern 2: Check if type matches
fn is_string_type(resolver: &impl TypeResolver, type_ref: TypeReference) -> bool {
    let resolved = resolver.resolve_type(type_ref);
    matches!(resolved.as_raw_data(), TypeData::String)
}

// Pattern 3: Handle unknown gracefully
match resolved.as_raw_data() {
    TypeData::Unknown | TypeData::UnknownKeyword => {
        // Can't verify, assume valid
        return None;
    }
    TypeData::String => { /* handle */ }
    _ => { /* handle */ }
}
```

## References

- Architecture guide: `crates/biome_js_type_info/CONTRIBUTING.md`
- Module graph: `crates/biome_module_graph/`
- Type resolver trait: `crates/biome_js_type_info/src/resolver.rs`
- Flattening: `crates/biome_js_type_info/src/flattening.rs`
