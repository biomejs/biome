---
"@biomejs/biome": patch
---

Added the nursery rule [`noTailwindRestrictedClasses`](https://biomejs.dev/linter/rules/no-tailwind-restricted-classes/) for JavaScript and HTML. It reports the Tailwind CSS classes and variants listed in its options with a custom message, and can replace them with a configured class or variant.

```json
{
  "linter": {
    "rules": {
      "nursery": {
        "noTailwindRestrictedClasses": {
          "level": "error",
          "options": {
            "classes": {
              "bg-green-500": { "message": "Use the success color.", "use": "bg-success" },
              "float-*": "Use flexbox or grid for layout."
            }
          }
        }
      }
    }
  }
}
```

```jsx
<div className="hover:bg-green-500 float-left" />; // both classes are reported
```
