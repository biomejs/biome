---
"@biomejs/biome": patch
---

Added a new lint `useReadonlyClassProperties` rule.
This rule is a port of ESLint's [prefer-readonly](https://typescript-eslint.io/rules/prefer-readonly/) rule.

Example:

 ```ts
 class Example {
     // All properties below can be marked as readonly
     public constantValue = 42;
     protected initializedInConstructor: string;
     private privateField = true;

     constructor(initializedInConstructor: string) {
         this.initializedInConstructor = initializedInConstructor;
     }
 }
 ```
