# Multi-Level Import Chain Test

This test demonstrates upward traversal through multiple levels of imports
where each level imports its own CSS file.

## File Structure

```
multilevel.jsx (imports level1.css)
  - uses class "level1" (valid) and "undefined-at-level1" (invalid)

level2.jsx (imports level2.css)
  └─ imports multilevel.jsx
  - uses class "level2" (valid) and "undefined-at-level2" (invalid)

level3.jsx (imports level3.css)
  └─ imports level2.jsx
  - uses class "level3" (valid) and "undefined-at-level3" (invalid)

level4.jsx (imports level4.css)
  └─ imports level3.jsx
  - uses class "level4" (valid) and "undefined-at-level4" (invalid)
```

## Expected Behavior

Each component should have access to CSS from:
1. Its own direct imports
2. CSS imported by parent components in the chain

For example, level3.jsx should see classes from:
- level3.css (directly imported)
- level2.css (imported by level2.jsx which imports level3.jsx)
- level1.css (imported by multilevel.jsx which is imported by level2.jsx)
