# Semantic Model Integration for Glimmer

## The Goal

Make existing rules like `noUnusedImports` and `noUnusedPrivateClassMembers` automatically work with Glimmer templates **without any modifications**.

## How Semantic Model Works Today

```rust
// 1. Build phase: Walk JS tree and collect bindings/references
let semantic_model = SemanticModelBuilder::new()
    .visit(js_module)  // Only sees JS!
    .build();

// 2. Query phase: Rules ask about bindings
impl Rule for NoUnusedImports {
    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let import = ctx.query();
        let binding = get_binding(import);

        // This only finds JS references!
        let references = ctx.semantic_model().all_references(binding);

        if references.count() == 0 {
            Some(UnusedImportState { binding })
        } else {
            None
        }
    }
}
```

**Problem**: `all_references()` only sees JavaScript references because the semantic model was only built from the JS tree!

## The Solution: Hybrid Semantic Model

Extend the semantic model to include references from templates:

```rust
// Enhanced semantic model that knows about Glimmer
pub struct GlimmerSemanticModel {
    // The JS semantic model (bindings, scopes, etc.)
    js_semantic: SemanticModel,

    // Additional references from HTML templates
    template_references: Vec<TemplateReference>,
}

#[derive(Debug)]
pub struct TemplateReference {
    // What binding does this reference?
    binding_id: BindingId,

    // Where in the template is it used?
    range: TextRange,

    // What kind of usage?
    kind: TemplateReferenceKind,
}

#[derive(Debug)]
pub enum TemplateReferenceKind {
    /// Used as a component: <Button />
    Component { element_node: SyntaxNode<HtmlLanguage> },

    /// Used in mustache expression: {{this.count}}
    Property { path: String },

    /// Used as helper: {{helper arg}}
    Helper { call: String },
}
```

## Building the Hybrid Model

```rust
impl GlimmerSemanticModel {
    pub fn build(glimmer_module: &GlimmerModule) -> Self {
        // Step 1: Build JS semantic model (existing code)
        let js_semantic = SemanticModel::build(glimmer_module.js_module());

        // Step 2: Scan templates for references (NEW!)
        let template_references = scan_template_references(
            glimmer_module.templates(),
            &js_semantic,
        );

        GlimmerSemanticModel {
            js_semantic,
            template_references,
        }
    }

    /// Get all references to a binding (JS + template references)
    pub fn all_references(&self, binding: &Binding) -> impl Iterator<Item = Reference> + '_ {
        let binding_id = binding.id();

        // Find JS references (existing)
        let js_refs = self.js_semantic.all_references(binding);

        // Find template references (NEW!)
        let template_refs = self.template_references
            .iter()
            .filter(move |r| r.binding_id == binding_id)
            .map(|r| Reference::from_template(r));

        // Return both!
        js_refs.chain(template_refs)
    }
}

fn scan_template_references(
    templates: &[TemplateMapping],
    js_semantic: &SemanticModel,
) -> Vec<TemplateReference> {
    let mut references = Vec::new();

    for template in templates {
        // Walk HTML tree looking for component references
        for node in template.html_root.syntax().descendants() {
            if let Some(element) = HtmlElement::cast(node.clone()) {
                if let Ok(opening) = element.opening_element() {
                    if let Ok(name) = opening.name() {
                        if let Ok(value_token) = name.value_token() {
                            let tag_name = value_token.text_trimmed().to_string();

                            // Is this a PascalCase component?
                            if is_pascal_case(&tag_name) {
                                // Find the binding for this name
                                if let Some(binding) = js_semantic.find_binding_by_name(&tag_name) {
                                    references.push(TemplateReference {
                                        binding_id: binding.id(),
                                        range: value_token.text_range(),
                                        kind: TemplateReferenceKind::Component {
                                            element_node: node.clone(),
                                        },
                                    });
                                }
                            }
                        }
                    }
                }
            }

            // TODO: Also scan for {{this.property}} references
            // TODO: Also scan for {{helper}} references
        }
    }

    references
}
```

## Integration Point: SemanticServices

The key is to inject the Glimmer-aware semantic model where rules get their context:

```rust
// In biome_js_analyze/src/services.rs (or similar)

pub struct SemanticServices {
    model: SemanticModel,
    // NEW: Optional Glimmer extensions
    glimmer_extensions: Option<GlimmerSemanticExtensions>,
}

pub struct GlimmerSemanticExtensions {
    template_references: Vec<TemplateReference>,
}

impl SemanticServices {
    pub fn from_node(
        root: &JsModule,
        file_source: &JsFileSource,
    ) -> Self {
        let mut model = SemanticModel::new(root);

        let glimmer_extensions = if file_source.as_embedding_kind().is_glimmer() {
            // Get the original file content
            let source_text = root.syntax().text_with_trivia().to_string();

            // Parse templates
            let templates = parse_glimmer_templates(&source_text);

            // Scan for template references
            let template_references = scan_template_references(&templates, &model);

            Some(GlimmerSemanticExtensions {
                template_references,
            })
        } else {
            None
        };

        SemanticServices {
            model,
            glimmer_extensions,
        }
    }
}

// Extend the semantic model API
impl SemanticModel {
    /// Get all references to a binding (now includes Glimmer templates!)
    pub fn all_references(&self, binding: &Binding) -> AllReferences {
        // This is where we inject template awareness!
        AllReferences {
            binding_id: binding.id(),
            js_references: self.get_js_references(binding),
            template_references: self.get_template_references(binding), // NEW!
        }
    }

    fn get_template_references(&self, binding: &Binding) -> Vec<Reference> {
        // Access the glimmer extensions from somewhere...
        // (Details depend on how we store the extensions)
        if let Some(extensions) = self.glimmer_extensions() {
            extensions
                .template_references
                .iter()
                .filter(|r| r.binding_id == binding.id())
                .map(|r| Reference::from_template(r))
                .collect()
        } else {
            Vec::new()
        }
    }
}
```

## Result: Rules Just Work!

Once the semantic model is extended, **no changes needed** to existing rules:

### noUnusedImports - Works Automatically! ✅

```rust
impl Rule for NoUnusedImports {
    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();

        // This now returns BOTH:
        // - JS references: const x = Button()
        // - Template references: <Button />
        let references = ctx.semantic_model().all_references(binding);

        if references.count() == 0 {
            // Truly unused!
            Some(State { unused: binding })
        } else {
            // Used (in JS OR template)
            None
        }
    }
}
```

### noUnusedPrivateClassMembers - Works Automatically! ✅

```rust
impl Rule for NoUnusedPrivateClassMembers {
    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let member = ctx.query();

        // This now sees {{this.#privateProperty}} in templates!
        let references = ctx.semantic_model().all_references(member);

        if references.count() == 0 {
            Some(State { unused: member })
        } else {
            None
        }
    }
}
```

### noUnusedVariables - Works Automatically! ✅

```rust
impl Rule for NoUnusedVariables {
    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let variable = ctx.query();

        // This now sees {{myVariable}} in templates!
        let references = ctx.semantic_model().all_references(variable);

        if references.count() == 0 {
            Some(State { unused: variable })
        } else {
            None
        }
    }
}
```

## Implementation Plan

### Phase 1: Template Reference Scanner
```rust
// Create a new module: biome_js_semantic/src/glimmer.rs
pub fn scan_template_references(
    templates: &[TemplateInfo],
    semantic_model: &SemanticModel,
) -> Vec<TemplateReference>
```

### Phase 2: Extend SemanticModel
```rust
// Add optional glimmer_extensions field
pub struct SemanticModel {
    // ... existing fields
    glimmer_extensions: Option<Arc<GlimmerSemanticExtensions>>,
}

// Modify all_references() to check glimmer_extensions
```

### Phase 3: Integrate in SemanticServices
```rust
// When building semantic services, detect Glimmer files
// and add template references
impl FromServices for SemanticServices {
    fn from_services(...) -> Self {
        // Build model
        // If Glimmer, scan templates
        // Attach extensions
    }
}
```

### Phase 4: Test Existing Rules
```bash
# These should now work automatically:
cargo test noUnusedImports -- glimmer
cargo test noUnusedVariables -- glimmer
cargo test noUnusedPrivateClassMembers -- glimmer
```

## What About Performance?

**Parsing cost**: Only paid for `.gjs`/`.gts` files (small % of codebase)

**Memory cost**: Minimal - just stores references, not full trees

**Scanning cost**: Linear in template size (very fast)

**Caching**: Can cache template references per file

## Benefits Recap

✅ **Zero rule changes** - Existing rules work automatically
✅ **Consistent behavior** - Same logic for JS and templates
✅ **Better DX** - One rule to understand, not separate Glimmer rules
✅ **Future-proof** - New rules automatically support Glimmer
✅ **Accurate** - True semantic understanding, not heuristics

## The One Missing Piece

We still need the **unified tree traversal** (from the previous prototype) to make the semantic model builder visit both trees. But once that's in place, everything else flows naturally.

## Recommendation

1. ✅ **Keep current approach** for now (works, ships quickly)
2. 🔨 **Build unified tree** as next step (enables semantic integration)
3. 🎯 **Extend semantic model** to scan templates (makes rules work)
4. 🎉 **Delete `noUnusedGlimmerComponents`** (no longer needed!)

The investment in proper semantic integration pays off by making ALL rules work, not just one!
