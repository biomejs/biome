---
"@biomejs/biome": patch
---

Fixed [#4841](https://github.com/biomejs/biome/issues/4841): Shebang and top leading comments in `.cjs` files are now handled correctly

**Example: shebang only (keep it as is)**

```
#!/usr/bin/env node
```

**Example: comments only (keep it as is)**

```
// comment
```

**Example: with shebang**

```diff
- #!/usr/bin/env node"use strict";
+ #!/usr/bin/env node
+ "use strict";
let some_variable = "some value";
```

**Example: with comment**

```diff
- // comment
- "use strict"; // comment
+ "use strict";
+ // comment
let some_variable = "some value";
```

**Example: with shebang and comment**

```diff
- #!/usr/bin/env node"use strict";
- // comment
+ #!/usr/bin/env node
+ "use strict";
+ // comment
let some_variable = "some value";
```
