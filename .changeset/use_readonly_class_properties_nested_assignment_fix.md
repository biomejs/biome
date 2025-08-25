---
"@biomejs/biome": patch
---

Fixed [#7310](https://github.com/biomejs/biome/issues/7310): [`useReadonlyClassProperties`](https://biomejs.dev/linter/rules/use-readonly-class-properties/) correctly handles nested assignments, avoiding false positives when a class property is assigned within another assignment expression.

Example of code that previously triggered a false positive but is now correctly ignored:

```ts
class test {
  private thing: number = 0; // incorrectly flagged

  public incrementThing(): void {
    const temp = {x: 0};
    temp.x = this.thing++;
  }
}
```
