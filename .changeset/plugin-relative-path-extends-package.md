---
"@biomejs/biome": patch
---

Fixed the resolution of relative `plugins` paths declared inside a configuration file that is pulled in through `extends` (for example a shared configuration package resolved through `node_modules`). Such paths are now resolved relative to the directory of the configuration file that declares them, instead of the directory of the configuration file that extends it — matching the behaviour already in place for `extends: "//"` (see the related report in [#10360](https://github.com/biomejs/biome/issues/10360)).
