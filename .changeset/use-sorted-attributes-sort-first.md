---
"@biomejs/biome": minor
---

Added the `sortFirst` option to the `useSortedAttributes` assist (JSX and HTML).

`sortFirst` is a list of attribute names that should be sorted before all other attributes, in the order they appear in the list. The remaining attributes are sorted normally, after the listed ones. This is useful to keep attributes such as `key` first.

#### Example

To enable this option, configure it in your `biome.json`:

```json
{
  "assist": {
    "actions": {
      "source": {
        "useSortedAttributes": {
          "level": "on",
          "options": {
            "sortFirst": ["key"]
          }
        }
      }
    }
  }
}
```

With this option, the following JSX element:

```jsx
<div className={styles.delinquentInvestment} key={investment.id} />
```

Will be sorted as:

```jsx
<div key={investment.id} className={styles.delinquentInvestment} />
```
