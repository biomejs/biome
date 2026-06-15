---
"@biomejs/biome": minor
---

Added the `sortFirst` option to the [useSortedAttributes](https://biomejs.dev/assist/actions/use-sorted-attributes/) assist (JSX and HTML). It takes a list of attribute names that are sorted before all other attributes, in the order given, while the remaining attributes keep their usual sort. This is useful to keep attributes such as `key` first.

With the following configuration:

```json
{
  "assist": {
    "actions": {
      "source": {
        "useSortedAttributes": {
          "level": "on",
          "options": { "sortFirst": ["key"] }
        }
      }
    }
  }
}
```

The attributes are reordered so that `key` comes first:

```jsx
// Before
<div className={styles.investment} key={investment.id} />;

// After
<div key={investment.id} className={styles.investment} />;
```
