# Glimmer (GJS/GTS) Support - Test Suite

This directory contains test files for Biome's Glimmer support implementation.

## ✅ What Works

### File Recognition & Parsing
- `.gjs` files recognized as JavaScript + Glimmer templates
- `.gts` files recognized as TypeScript + Glimmer templates
- Templates extracted before parsing to avoid syntax errors
- JavaScript/TypeScript parsed correctly with templates removed

### Formatting
- **JavaScript/TypeScript code is formatted** according to Biome rules
- **Templates are preserved exactly as-is** (not formatted yet)
- **Semicolons handled correctly**:
  - Class member templates: NO semicolon
  - Assignment templates: semicolon present  
  - All expression contexts: correct punctuation

### Multiple Template Support
Files can contain multiple `<template>` blocks in any combination:
- **Class body**: `class Foo { <template>...</template> }`
- **Assignment**: `const Foo = <template>...</template>;`
- **Export**: `export default <template>...</template>;`
- **Arrays**: `[<template>...</template>, <template>...</template>]`
- **Objects**: `{ foo: <template>...</template> }`
- **Function args**: `fn(<template>...</template>)`
- **Return statements**: `return <template>...</template>;`
- **Ternary expressions**: `x ? <template>...</template> : <template>...</template>`

### Linting
- JavaScript linting rules work on the JS/TS portions
- Template content is ignored by linters (for now)

## 📁 Test Files

### Basic Tests
- **`example.gjs`** - Single template in class body
- **`example.gts`** - TypeScript with Glimmer template
- **`multi-template.gjs`** - Multiple templates (all contexts)

### Edge Cases
- **`test-edge-cases.gjs`** - Arrays, objects, function args, returns, ternaries
- **`test-class-assignment.gjs`** - Assignment inside class vs class member
- **`lint-test.gjs`** - Linting behavior test

## 🧪 Running Tests

```bash
# Format a file
../target/debug/biome format example.gjs --write

# Check (format + lint)
../target/debug/biome check example.gjs

# Lint only
../target/debug/biome lint example.gjs

# Format all test files
../target/debug/biome format *.gjs *.gts --write
```

## 🔧 Implementation Details

### Template Extraction Strategy
1. Templates replaced with identifier placeholders: `__BIOME_GLIMMER_TEMPLATE_N__`
2. Original source analyzed to record trailing semicolons
3. JS/TS parsed and formatted normally
4. Placeholders replaced with original templates
5. Semicolons kept/removed based on original source

### Why Identifiers Work
Identifiers are valid in all Glimmer template contexts:
- **Class body**: Treated as field declaration
- **Expression**: Treated as identifier reference
- **Formatter**: Adds semicolons where appropriate for identifiers

### Semicolon Logic
We check the **original source** to determine if a semicolon should be kept:
- If original had semicolon after `</template>`, we keep it
- If original had no semicolon, we remove the formatter-added one
- This handles all edge cases without context detection heuristics

## 📋 Current Limitations

1. **Templates not formatted** - HTML inside `<template>` tags is preserved as-is
2. **No Glimmer-specific linting** - Only JS/TS rules apply
3. **No template-only file support** - Files with ONLY `<template>` (implicit default export)

## 🚀 Next Steps (Future Phases)

### Phase 4: Template Parsing
- Parse templates with HTML parser
- Build Glimmer AST for templates
- Support Glimmer expression syntax (`{{...}}`, `{{#if}}`, etc.)

### Phase 5: Template Formatting
- Format HTML in templates
- Handle Glimmer expressions
- Preserve Glimmer semantics

### Phase 6: Glimmer Linting
- Template-specific lint rules
- Cross-boundary rules (JS + template)
- Glimmer best practices

### Phase 7: Template-Only Files
- Support bare `<template>` (implicit default export)
- Proper file source detection

## ✨ Success Metrics

**Phase 3 Complete**: ✅
- 15+ commits
- Fully compiling
- All test cases passing
- Clean AST-based semicolon handling
- No string matching heuristics
- Comprehensive edge case coverage
