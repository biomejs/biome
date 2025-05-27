---
"@biomejs/biome": minor
---

Added the new rule [`useAdjacentGetterSetter`](https://biomejs.dev/linter/rules/use-adjacent-getter-setter), which enforces getters and setters for the same property
to be adjacent in class and object definitions.

**Example (Invalid): Name getter and setter are not adjacent:**

```js
class User {
  get name() { return this._name; }
  constructor() {}
  set name(value) { this._name = value; }
}
```

**Example (Invalid): Getter should go before the setter.

```js
const user = {
  set name(value) { this._name = value; },
  get name() { return this._name; }
};
```

**Example (Valid): Name getter and setter are adjacent:**

```js
class User {
  get name() { return this._name; }
  set name(value) { this._name = value; }
  get age() { return this._age; }
  set age(age) { this._age = age; }
}
```
