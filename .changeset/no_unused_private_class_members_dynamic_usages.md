---
"@biomejs/biome": patch
---

**Improved detection of used private class members**

The analysis for private class members has been improved: now the tool only considers a private member “used” if it is actually referenced in the code.

- Previously, some private members might have been reported as used even if they weren’t actually accessed.
- With this change, only members that are truly read or called in the code are counted as used.
- Members that are never accessed will now be correctly reported as unused.

This makes reports about unused private members more accurate and helps you clean up truly unused code.

***Example (previously valid)***

```ts
type YesNo = "yes" | "no";

export class SampleYesNo {
  private yes: () => void;
  private no: () => void;
  private dontKnow: () => void; // <- will now report as unused

  on(action: YesNo): void {
    this[action]();
  }
}

```
