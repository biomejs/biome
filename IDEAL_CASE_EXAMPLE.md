# Ideal Case: Existing Rules Just Work

## The Vision

With proper semantic model integration, existing rules automatically understand Glimmer templates without any modifications.

## Example File

```javascript
// example.gjs
import Button from './Button';    // ← Should not warn (used in template)
import Card from './Card';        // ← Should not warn (used in template)
import Dialog from './Dialog';    // ← SHOULD warn (unused)

export default class MyComponent {
  #privateCount = 0;              // ← Should not warn (used in template)
  #unusedPrivate = 0;             // ← SHOULD warn (unused)

  get formattedCount() {          // ← Should not warn (used in template)
    return `Count: ${this.#privateCount}`;
  }

  unusedMethod() {                // ← SHOULD warn (unused)
    return 'never called';
  }

  <template>
    <Card>
      <Button onclick={{this.formattedCount}}>
        {{this.#privateCount}}
      </Button>
    </Card>
  </template>
}
```

## Current Behavior (Dual AST)

### ❌ `noUnusedImports` - False Positives

```
warning: 'Button' is imported but never used
  --> example.gjs:1:8
   |
1  | import Button from './Button';
   |        ^^^^^^
   |
note: The import is unused

warning: 'Card' is imported but never used
  --> example.gjs:2:8
   |
2  | import Card from './Card';
   |        ^^^^
```

**Problem**: Can't see `<Button>` and `<Card>` in template

**Workaround**: Need custom `noUnusedGlimmerComponents` rule

### ❌ `noUnusedPrivateClassMembers` - False Positives

```
warning: Private member '#privateCount' is never used
  --> example.gjs:6:3
   |
6  |   #privateCount = 0;
   |   ^^^^^^^^^^^^^
```

**Problem**: Can't see `{{this.#privateCount}}` in template

**Workaround**: Need to disable rule or add ignore comments

### ❌ `noUnusedVariables` - False Positives

```
warning: Getter 'formattedCount' is never used
  --> example.gjs:9:7
   |
9  |   get formattedCount() {
   |       ^^^^^^^^^^^^^^
```

**Problem**: Can't see `{{this.formattedCount}}` in template

**Workaround**: Need custom rule or ignore comments

## Ideal Behavior (Unified Semantic Model)

### ✅ `noUnusedImports` - Correctly Identifies Unused

```
warning: 'Dialog' is imported but never used
  --> example.gjs:3:8
   |
3  | import Dialog from './Dialog';
   |        ^^^^^^
   |
note: Remove this import or use it in your code or template
```

**Success**:
- ✅ Sees `<Button>` in template → Button is used
- ✅ Sees `<Card>` in template → Card is used
- ✅ No `<Dialog>` found → Dialog is unused ← **correct warning!**

### ✅ `noUnusedPrivateClassMembers` - Correctly Identifies Unused

```
warning: Private member '#unusedPrivate' is never used
  --> example.gjs:7:3
   |
7  |   #unusedPrivate = 0;
   |   ^^^^^^^^^^^^^^
```

**Success**:
- ✅ Sees `{{this.#privateCount}}` → privateCount is used
- ✅ No reference to `#unusedPrivate` → Unused ← **correct warning!**

### ✅ `noUnusedVariables` - Correctly Identifies Unused

```
warning: Method 'unusedMethod' is never called
  --> example.gjs:13:3
   |
13 |   unusedMethod() {
   |   ^^^^^^^^^^^^
```

**Success**:
- ✅ Sees `{{this.formattedCount}}` → formattedCount is used
- ✅ No call to `unusedMethod()` → Unused ← **correct warning!**

## How It Works Under the Hood

```rust
// When analyzing example.gjs:

// 1. Build JS semantic model
let js_model = SemanticModel::new(js_ast);
// Bindings: Button, Card, Dialog, #privateCount, #unusedPrivate, ...

// 2. Scan templates for references
let template_refs = scan_templates(html_ast);
// Found references:
//   - Button (used as <Button> at line 17)
//   - Card (used as <Card> at line 16)
//   - this.#privateCount (used in {{...}} at line 18)
//   - this.formattedCount (used in {{...}} at line 17)

// 3. Merge into unified semantic model
let model = GlimmerSemanticModel::new(js_model, template_refs);

// 4. Rules query the unified model
for binding in all_bindings {
    let refs = model.all_references(binding);
    //           ^^^^ Returns BOTH JS and template refs!

    if refs.count() == 0 {
        report_unused(binding);
    }
}
```

## What Users See

### Before (Current)

```json
// .biomejs.json - Need custom configuration
{
  "linter": {
    "rules": {
      "correctness": {
        "noUnusedImports": "off",  // ← Disabled! Too many false positives
        "noUnusedPrivateClassMembers": "off"  // ← Disabled!
      },
      "nursery": {
        "noUnusedGlimmerComponents": "error"  // ← Custom rule needed
      }
    }
  }
}
```

**Pain points**:
- Have to disable useful rules
- Need framework-specific rules
- Inconsistent behavior between frameworks

### After (Ideal)

```json
// .biomejs.json - Default configuration just works!
{
  "linter": {
    "rules": {
      "correctness": {
        "noUnusedImports": "error",  // ← Just works!
        "noUnusedPrivateClassMembers": "error"  // ← Just works!
      }
    }
  }
}
```

**Benefits**:
- ✅ All standard rules work
- ✅ No framework-specific configuration
- ✅ Consistent behavior across all files

## Additional Rules That Would Work

Once semantic model is extended, these would also work automatically:

### `noUnusedLabels` ✅
```gjs
label: for (const item of items) {  // ← Would see {{break label}}
  <template>{{break label}}</template>
}
```

### `noShadowRestrictedNames` ✅
```gjs
const undefined = 1;  // ← Would see {{undefined}} in template
<template>{{undefined}}</template>
```

### `useConst` ✅
```gjs
let count = 0;  // ← Would see that {{count}} is never reassigned
<template>{{count}}</template>
```

### `noVar` ✅
```gjs
var x = 1;  // ← Still catches old-style declarations
<template>{{x}}</template>
```

## What's Required

To achieve this ideal state:

1. **Unified Tree** (previous prototype)
   - Parse JS and HTML separately
   - Stitch together with markers

2. **Template Reference Scanner** (new)
   ```rust
   fn scan_template_references(templates, js_semantic) -> Vec<Reference>
   ```

3. **Semantic Model Extension** (new)
   ```rust
   impl SemanticModel {
       fn all_references(&self, binding) -> impl Iterator<Item = Reference> {
           // Return both JS refs AND template refs
       }
   }
   ```

4. **Test Coverage** (validation)
   ```bash
   # Verify existing rules work
   cargo test noUnusedImports -- glimmer
   cargo test noUnusedPrivateClassMembers -- glimmer
   ```

## Timeline Suggestion

**Now** (Phase 1): Ship what we have ✅
- Dual AST approach
- `noUnusedGlimmerComponents` custom rule
- Proves the concept

**Soon** (Phase 2): Unified tree infrastructure 🔨
- Implement `GlimmerModule` with unified traversal
- Keep existing rules working as-is

**Later** (Phase 3): Semantic model integration 🎯
- Extend semantic model to scan templates
- Watch existing rules start working automatically!

**Finally** (Phase 4): Cleanup 🎉
- Remove `noUnusedGlimmerComponents`
- Update docs to say "all rules work with Glimmer"

## Summary

The ideal case **is achievable**! It requires:

1. ✅ Parse separately (already done)
2. 🔨 Unified tree (prototyped, needs implementation)
3. 🎯 Semantic model extension (clear path forward)
4. 🎉 Profit (all rules just work!)

The investment is worth it because every new rule automatically supports Glimmer, and users get a consistent, intuitive experience.
