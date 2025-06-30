---
"@biomejs/biome": patch
---

Added `MemoryFileSystem` to the WASM API.
You can now insert a file from your JS code:

```js
import { MemoryFileSystem, Workspace } from "@biomejs/wasm-web";

const fs = new MemoryFileSystem();
const workspace = Workspace.withFileSystem(fs);

fs.insert("/index.js", new TextEncoder().encode("let foo = 1;"));
fs.remove("/index.js");
```
