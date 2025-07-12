---
"@biomejs/biome": patch
---

 [#6775](https://github.com/biomejs/biome/issues/6775): Expanded `use_readonly_class_properties` to also capture mutations inside call expression arguments list

Example:

```ts
class Counter {
  private counter: number
  count() {
    console.log(this.counter++)
    const counterString = `${this.counter++}`
  }
}
```
