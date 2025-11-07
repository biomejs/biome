# Unified AST Design for Glimmer

## Goal
Parse JavaScript and HTML separately, then combine into a single traversable AST.

## Current Architecture

```
.gjs file → GlimmerFileHandler → Extract JS → JS Parser → JS AST
                                → Extract templates → HTML Parser → HTML AST (x N)
```

Two separate trees, accessed separately.

## Proposed Architecture

```
.gjs file → GlimmerFileHandler → Extract JS → JS Parser → JS AST ┐
                                → Extract templates → HTML Parser → HTML AST (x N) ┘
                                                                                    │
                                                                                    ↓
                                                        UnifiedASTBuilder.combine()
                                                                                    │
                                                                                    ↓
                                                            Single GlimmerModule tree
```

## Key Insight: Virtual Unification

Instead of converting kinds or rebuilding, we create a **wrapper tree** that delegates to the original trees:

```rust
// The unified node is an enum that wraps both languages
pub enum GlimmerSyntaxNode {
    Js(SyntaxNode<JsLanguage>),
    Html(SyntaxNode<HtmlLanguage>),
}

// The unified tree maintains the mapping
pub struct GlimmerModule {
    js_root: JsModule,
    templates: Vec<TemplateMapping>,
}

struct TemplateMapping {
    // Where in the JS tree is the template marker?
    marker_range: TextRange,
    marker_node: SyntaxNode<JsLanguage>,

    // What HTML tree goes there?
    html_root: HtmlRoot,
}
```

## Traversal Strategy

When traversing the unified tree:

```rust
impl GlimmerModule {
    pub fn descendants(&self) -> impl Iterator<Item = GlimmerSyntaxNode> {
        // Start with JS root
        let js_nodes = self.js_root.descendants();

        // When we hit a template marker node...
        js_nodes.flat_map(|node| {
            if self.is_template_marker(&node) {
                // Replace it with HTML tree nodes
                let template = self.get_template_for_marker(&node);
                Either::Right(template.descendants().map(GlimmerSyntaxNode::Html))
            } else {
                Either::Left(std::iter::once(GlimmerSyntaxNode::Js(node)))
            }
        })
    }
}
```

## Benefits

1. **No kind conversion needed** - Original parsers stay pure
2. **Lazy construction** - Only build unified view when needed
3. **Zero copy** - Original ASTs unchanged
4. **Clean APIs** - Looks like single tree to users
5. **Backward compatible** - Can still access original trees

## Implementation Steps

### Phase 1: Wrapper Types
Create the GlimmerSyntaxNode wrapper and GlimmerModule container.

### Phase 2: Tree Stitching
Implement the logic to map template markers to HTML trees.

### Phase 3: Unified Traversal
Provide Iterator implementations that transparently cross boundaries.

### Phase 4: Semantic Model Extension
Extend semantic model to understand both JS and HTML references.

### Phase 5: Rule Migration
Update rules to use unified tree instead of separate trees.

## Example: How noUnusedImports Would Work

With unified tree:

```rust
impl Rule for NoUnusedImports {
    type Query = Ast<JsImport>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let import = ctx.query();
        let binding = get_binding(import);

        // This now sees BOTH:
        // - JS references: const x = Button()
        // - HTML references: <Button />
        let references = ctx.semantic_model().all_references(binding);

        if references.count() == 0 {
            Some(State { unused: binding })
        } else {
            None
        }
    }
}
```

No need for separate `noUnusedGlimmerComponents` rule!

## Semantic Model Changes

```rust
impl SemanticModel {
    pub fn all_references(&self, binding: &Binding) -> impl Iterator<Item = Reference> {
        // If this is a Glimmer file, search both JS and HTML trees
        if self.is_glimmer() {
            let js_refs = self.find_js_references(binding);
            let html_refs = self.find_html_references(binding);
            js_refs.chain(html_refs)
        } else {
            self.find_js_references(binding)
        }
    }

    fn find_html_references(&self, binding: &Binding) -> impl Iterator<Item = Reference> {
        let name = binding.name();

        // Search HTML trees for element tags matching the name
        self.glimmer_module
            .templates
            .iter()
            .flat_map(|template| {
                template.html_root
                    .descendants()
                    .filter_map(|node| {
                        if let Some(element) = HtmlElement::cast(node) {
                            if element.tag_name() == name {
                                Some(Reference::from_html(element))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
            })
    }
}
```

## Performance Considerations

- **Parsing**: Same as before (no change)
- **Memory**: Small overhead for mapping structs
- **Traversal**: Slightly slower due to indirection
- **Caching**: Can cache unified view if needed

## Open Questions

1. How to handle source locations? (JS positions vs file positions)
2. How to format the unified tree? (Need to split back apart)
3. How to handle mutations? (Changes to one tree affect others)
4. Should we cache the unified view or compute on-demand?
