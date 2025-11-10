# Ember Demo App - Biome Lint Rules Showcase

This is a **deliberately broken** Ember application that demonstrates all 13 implemented Biome Ember lint rules.

## Purpose

This app contains **intentional violations** of each Ember lint rule so you can:
- See the rules in action on real code
- Understand what each rule detects
- Learn how to fix common Ember anti-patterns

## Structure

```
ember-demo-app/
├── app/
│   ├── components/
│   │   ├── user-profile.gjs         # Template rules (accesskey, autofocus, inline-styles, positive-tabindex)
│   │   ├── search-form.gjs          # More template violations
│   │   ├── classic-button.js        # Classic class violations
│   │   └── actions-demo.js          # Actions hash violation
│   ├── mixins/
│   │   └── validatable.js           # Mixin (imported illegally)
│   ├── services/
│   │   └── analytics.js             # get(), getWithDefault() violations
│   ├── utils/
│   │   └── dom-helpers.js           # Global jQuery violations
│   └── routes/
│       └── index.js                 # Old shims, classic classes
└── tests/
    └── integration/
        └── user-profile-test.js     # pauseTest violation
```

## Running Biome

### Prerequisites

First, build the debug Biome binary from the repository root:

```bash
cd ../..
cargo build --bin biome
```

### Check for violations

```bash
# From this directory (examples/ember-demo-app/)
npm run lint

# Or use the biome binary directly
../../target/debug/biome check app/
```

### Expected Output

You should see **13+ violations** across different files:

```
app/components/user-profile.gjs:12:7
  ✖ noEmberAccesskeyAttribute: Avoid using the "accesskey" attribute

app/components/user-profile.gjs:15:7
  ✖ noEmberAutofocus: Avoid using the "autofocus" attribute

app/components/user-profile.gjs:18:12
  ✖ noEmberInlineStyles: Avoid using inline style attributes

app/components/search-form.gjs:9:18
  ✖ noEmberPositiveTabindex: Avoid positive tabindex values

app/components/classic-button.js:5:1
  ✖ noEmberClassicClasses: Use native JavaScript classes instead of .extend()

app/components/actions-demo.js:8:3
  ✖ noEmberActionsHash: Use @action decorator instead of actions hash

app/services/analytics.js:2:10
  ✖ noEmberGet: Use native property access instead of get()

app/services/analytics.js:2:16
  ✖ noEmberGetWithDefault: Use optional chaining instead of getWithDefault()

app/utils/dom-helpers.js:3:3
  ✖ noEmberGlobalJquery: Avoid using global jQuery

app/routes/index.js:2:8
  ✖ noEmberOldShims: Don't import from deprecated shim modules

app/mixins/validatable.js:2:8
  ✖ noEmberMixins: Don't import from /mixins/ directories

tests/integration/user-profile-test.js:5:10
  ✖ noEmberPauseTest: Remove pauseTest before committing
```

## Violations by File

### 1. user-profile.gjs
- ✖ noEmberAccesskeyAttribute (line ~12)
- ✖ noEmberAutofocus (line ~15)
- ✖ noEmberInlineStyles (line ~18)
- ✖ noEmberPositiveTabindex (line ~21)

### 2. search-form.gjs
- ✖ noEmberPositiveTabindex (line ~9)

### 3. classic-button.js
- ✖ noEmberClassicClasses (line ~5)

### 4. actions-demo.js
- ✖ noEmberActionsHash (line ~8)

### 5. analytics.js
- ✖ noEmberGet (line ~6)
- ✖ noEmberGetWithDefault (line ~11)

### 6. dom-helpers.js
- ✖ noEmberGlobalJquery (line ~3, ~8)

### 7. index.js (route)
- ✖ noEmberOldShims (line ~2)
- ✖ noEmberClassicClasses (line ~5)

### 8. validatable.js (mixin)
- ✖ Imported in other files (triggers noEmberMixins)

### 9. user-profile-test.js
- ✖ noEmberPauseTest (line ~5, ~12)

## Learning Path

1. **Run Biome** to see all violations
2. **Open each file** and read the comments explaining the violations
3. **Check the "FIXED VERSION"** comments to see correct patterns
4. **Try fixing one rule** at a time and re-run Biome

## Fixing the App

Each file includes comments showing both:
- ❌ The **VIOLATION** (what triggers the rule)
- ✅ The **FIX** (how to write it correctly)

To create a clean version:
1. Replace `.extend()` with native classes
2. Remove `actions` hash, use `@action` decorator
3. Replace `get()` and `getWithDefault()` with native access
4. Remove global `$` and `jQuery` usage
5. Update old shim imports
6. Delete mixins, use composition
7. Remove `pauseTest` from tests
8. In templates:
   - Remove `accesskey` attributes
   - Remove `autofocus` attributes
   - Remove `style` attributes (use CSS classes)
   - Remove positive `tabindex` values

## Next Steps

After understanding these violations:
1. Apply these patterns to your own Ember app
2. Configure Biome in your project's `biome.json`
3. Run `biome check` in CI to catch violations early
4. Use `biome check --write` for auto-fixes (where available)

## Notes

- This app is **intentionally broken** for demonstration
- It won't run - it's purely for static analysis
- All 13 rules are showcased with real-world examples
- Use this as a reference when migrating to modern Ember
