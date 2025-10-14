---
"@biomejs/biome": minor
---

Added the new `checkstyle` reporter. When `--reporter=checkstyle` is passed to the CLI, Biome will emit diagnostics for [Checkstyle format](https://checkstyle.org/):

```xml
<?xml version="1.0" encoding="utf-8"?>
<checkstyle version="4.3">
  <file name="index.ts">
    <error line="1" column="8" severity="warning" message="This import is unused." source="lint/correctness/noUnusedImports" />
    <error line="2" column="10" severity="warning" message="Several of these imports are unused." source="lint/correctness/noUnusedImports" />
    <error line="8" column="5" severity="warning" message="This variable f is unused." source="lint/correctness/noUnusedVariables" />
    <error line="9" column="7" severity="warning" message="This variable f is unused." source="lint/correctness/noUnusedVariables" />
    <error line="1" column="1" severity="error" message="The imports and exports are not sorted." source="assist/source/organizeImports" />
    <error line="4" column="3" severity="error" message="Using == may be unsafe if you are relying on type coercion." source="lint/suspicious/noDoubleEquals" />
    <error line="6" column="1" severity="error" message="This is an unexpected use of the debugger statement." source="lint/suspicious/noDebugger" />
    <error line="8" column="5" severity="error" message="This variable implicitly has the any type." source="lint/suspicious/noImplicitAnyLet" />
    <error line="9" column="7" severity="error" message="This variable implicitly has the any type." source="lint/suspicious/noImplicitAnyLet" />
    <error line="2" column="10" severity="error" message="Shouldn&apos;t redeclare &apos;z&apos;. Consider to delete it or rename it." source="lint/suspicious/noRedeclare" />
    <error line="9" column="7" severity="error" message="Shouldn&apos;t redeclare &apos;f&apos;. Consider to delete it or rename it." source="lint/suspicious/noRedeclare" />
    <error line="0" column="0" severity="error" message="Formatter would have printed the following content:" source="format" />
  </file>
  <file name="main.ts">
    <error line="1" column="8" severity="warning" message="This import is unused." source="lint/correctness/noUnusedImports" />
    <error line="2" column="10" severity="warning" message="Several of these imports are unused." source="lint/correctness/noUnusedImports" />
    <error line="8" column="5" severity="warning" message="This variable f is unused." source="lint/correctness/noUnusedVariables" />
    <error line="9" column="7" severity="warning" message="This variable f is unused." source="lint/correctness/noUnusedVariables" />
    <error line="1" column="1" severity="error" message="The imports and exports are not sorted." source="assist/source/organizeImports" />
    <error line="4" column="3" severity="error" message="Using == may be unsafe if you are relying on type coercion." source="lint/suspicious/noDoubleEquals" />
    <error line="6" column="1" severity="error" message="This is an unexpected use of the debugger statement." source="lint/suspicious/noDebugger" />
    <error line="8" column="5" severity="error" message="This variable implicitly has the any type." source="lint/suspicious/noImplicitAnyLet" />
    <error line="9" column="7" severity="error" message="This variable implicitly has the any type." source="lint/suspicious/noImplicitAnyLet" />
    <error line="2" column="10" severity="error" message="Shouldn&apos;t redeclare &apos;z&apos;. Consider to delete it or rename it." source="lint/suspicious/noRedeclare" />
    <error line="9" column="7" severity="error" message="Shouldn&apos;t redeclare &apos;f&apos;. Consider to delete it or rename it." source="lint/suspicious/noRedeclare" />
    <error line="0" column="0" severity="error" message="Formatter would have printed the following content:" source="format" />
  </file>
</checkstyle>
```
