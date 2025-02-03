---
"@biomejs/biome": patch
---

Fix [#4841](https://github.com/biomejs/biome/issues/4841), shebang and top leading comments in cjs files are now handled correctly

- shebang only (keep it as is)

```
#!/usr/bin/env node
```

- comments only (keep it as is)

```
// comment
```

- with shebang

```diff
- #!/usr/bin/env node"use strict";
+ #!/usr/bin/env node
+ "use strict";
let some_variable = "some value";
```

- with comment

```diff
- // comment
- "use strict"; // comment
+ "use strict";
+ // comment
let some_variable = "some value";
```

- with shebang and comment

```diff
- #!/usr/bin/env node"use strict";
- // comment
+ #!/usr/bin/env node
+ "use strict";
+ // comment
let some_variable = "some value";
```
