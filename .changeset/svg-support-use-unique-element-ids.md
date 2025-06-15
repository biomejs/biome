---
"@biomejs/biome": patch
---

The `useUniqueElementIds` rule now ignores SVG elements and their children. SVG elements are allowed to have static `id` attributes since they have local scope within the SVG context, unlike HTML elements where IDs must be globally unique.

This change allows SVG elements like `<circle>`, `<path>`, `<rect>`, `<linearGradient>`, etc. to use static `id` attributes without triggering the rule, while continuing to flag HTML elements with static IDs.

The rule now correctly handles:
- Direct SVG elements with static `id` attributes
- Nested elements within SVG contexts  
- React.createElement calls with SVG element types
- All 82 standard SVG element types

Example of now-valid code:
```jsx
function SvgIcon() {
  return (
    <svg viewBox="0 0 24 24">
      <circle id="circle1" cx="12" cy="12" r="10" />
      <path id="path1" d="M12 2L2 7v10l10 5 10-5V7z" />
    </svg>
  );
}
```
