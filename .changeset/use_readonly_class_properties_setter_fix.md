---
"@biomejs/biome": patch
---

Fixed [#6500](https://github.com/biomejs/biome/issues/6500): The `useReadonlyClassProperties` rule now correctly marks class properties as `readonly` when they are assigned in a constructor, setter or method,
even if the assignment occurs inside an if or else block.

The following code is now correctly detected by the rule:

 ```ts
class Price {
  #price: string;

  @Input()
  set some(value: string | number) {
    if (value === undefined || value === null || value === 'undefined' || value === 'null' || Number.isNaN(value)) {
      this.#price = '';
    } else {
      this.#price = '' + value;
    }
  }
}
 ```
