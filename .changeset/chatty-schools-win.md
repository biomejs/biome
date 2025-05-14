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

**Example (Invalid): Getter should go before the setter when configured with order "getBeforeSet"**

```js,expect_diagnostic,use_options
const user = {
  set name(value) { this._name = value; },
  get name() { return this._name; }
};
```

**Example (Valid): Name getter and setter are adjacent:**

```js,use_options
class User {
  get name() { return this._name; }
  set name(value) { this._name = value; }
  get age() { return this._age; }
  set age(age) { this._age = age; }
}
```

Option `order` can be used to specify the expected ordering of getters and setters:

 - `"anyOrder"` (default): Accessors for the same property must be adjacent, but can be in any order
 - `"getBeforeSet"`: Getter must come before setter
 - `"setBeforeGet"`: Setter must come before getter
