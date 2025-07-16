---
"@biomejs/biome": patch
---

Fixed [#6838](https://github.com/biomejs/biome/issues/6838): Reduce resource consumption in the Biome Language Server by using non-recursive filesystem watchers instead of recursive ones.

Watchers are responsible for notifying Biome of changes to files in the filesystem. We used to set up a single recursive watcher, but that meant that Biome would receive filesystem notifications for _all_ files in your project, even for ignored folders such as `build/` or `dist/` folders.

With this patch, we set up non-recursive watchers only for the folders that are relevant to a project.

Related to this, we also solved an issue where incoming notifications were incorrectly filtered, causing ignored files to be processed and stored in our module graph anyway.
