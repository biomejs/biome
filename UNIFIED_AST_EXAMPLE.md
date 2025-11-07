# Unified AST Example

## Input File (example.gjs)

```javascript
import Button from './Button';
import Card from './Card';

export default class MyComponent {
  <template>
    <div>
      <Button>Click me</Button>
    </div>
  </template>
}
```

## Step 1: Extract JavaScript

```javascript
import Button from './Button';
import Card from './Card';

export default class MyComponent {
  __BIOME_GLIMMER_TEMPLATE_0__
}
```

## Step 2: Parse Separately

### JavaScript AST
```
JsModule
├─ JsImport("Button")
├─ JsImport("Card")
└─ JsExportDefaultDeclaration
   └─ JsClass("MyComponent")
      └─ JsIdentifier("__BIOME_GLIMMER_TEMPLATE_0__")  ← Marker!
```

### HTML AST (Template 0)
```
HtmlRoot
└─ HtmlElement("template")
   └─ HtmlElement("div")
      └─ HtmlElement("Button")  ← Uses imported component!
         └─ HtmlContent("Click me")
```

## Step 3: Unified Traversal

When iterating with `unified_descendants()`:

```
1. JsModule                                    (JS)
2. JsImport("Button")                          (JS)
3. JsImport("Card")                            (JS)
4. JsExportDefaultDeclaration                  (JS)
5. JsClass("MyComponent")                      (JS)
6. [Marker detected] → Switch to HTML tree
7.   HtmlRoot                                  (HTML)
8.   HtmlElement("template")                   (HTML)
9.   HtmlElement("div")                        (HTML)
10.  HtmlElement("Button")                     (HTML) ← Found usage!
11.  HtmlContent("Click me")                   (HTML)
12. [Back to JS tree]
13. ... (continue with rest of JS)
```

## How Rules Benefit

### Before (Dual AST):

```rust
// noUnusedGlimmerComponents (custom rule)
impl Rule for NoUnusedGlimmerComponents {
    type Query = Ast<JsModule>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        // 1. Manually collect imports from JS tree
        let imports = collect_imports(ctx.query());

        // 2. Manually get source text
        let source = ctx.query().syntax().text_with_trivia().to_string();

        // 3. Manually parse templates
        let templates = parse_templates(&source);

        // 4. Manually search HTML trees
        let used = find_used_in_templates(&templates);

        // 5. Manually cross-reference
        imports.filter(|i| !used.contains(i))
    }
}
```

### After (Unified AST):

```rust
// noUnusedImports (standard rule, now works for Glimmer!)
impl Rule for NoUnusedImports {
    type Query = Ast<JsImport>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let import = ctx.query();
        let binding = get_binding(import);

        // Semantic model now searches the unified tree
        // It automatically finds references in BOTH:
        // - JS code: const x = Button()
        // - HTML template: <Button />
        let references = ctx.semantic_model().all_references(binding);

        if references.count() == 0 {
            Some(State { unused: binding })
        } else {
            None
        }
    }
}
```

## Key Advantages

### 1. Single Source of Truth
```rust
// One tree to traverse
for node in glimmer_module.unified_descendants() {
    // Process any node, JS or HTML
    process(node);
}
```

### 2. Automatic Cross-Reference
```rust
// Semantic model builder walks unified tree once
let semantic_model = SemanticModelBuilder::new()
    .visit(glimmer_module)  // Visits both JS and HTML!
    .build();
```

### 3. Simpler Rules
```rust
// Rules don't need to know about templates
// They just work on the unified tree
// No special cases!
```

### 4. Better Error Messages
```rust
// Can show context across boundaries
"Component 'Card' is imported but never used"
  --> example.gjs:2:8
   |
2  | import Card from './Card';
   |        ^^^^ imported here
   |
  --> example.gjs:6:5
   |
6  |     <Button>Click me</Button>
   |     ^^^^^^^ only Button is used here
```

## Implementation Complexity

### What's Easy:
- ✅ Parsing (already done separately)
- ✅ Tree wrapping (simple enum)
- ✅ Basic traversal (just follow pointers)

### What's Moderate:
- 🟡 Marker detection (need to recognize template markers)
- 🟡 Position mapping (convert between JS positions and file positions)
- 🟡 Iterator implementation (switching between trees)

### What's Hard:
- 🔴 Semantic model extension (needs to understand both languages)
- 🔴 Formatting (need to split back apart for output)
- 🔴 Mutations (changes to one tree affect the other)

## Recommendation

Start with **read-only unified tree**:
- Phase 1: Implement GlimmerModule with unified traversal
- Phase 2: Extend semantic model to use unified tree
- Phase 3: Update rules to use semantic model
- Phase 4: (Future) Handle mutations if needed

This gives us 80% of the benefit with 20% of the complexity.
