---
"@biomejs/biome": patch
---

Fixed [#191](https://github.com/biomejs/biome-zed/issues/191): Improved the performance of how the Biome Language Server pulls code actions and diagnostics.

Before, code actions were pulled and computed all at once in one request. This approach couldn't work in big files, and caused Biome to stale and have CPU usage spikes up to 100%.

Now, code actions are pulled and computed lazily, and Biome won't choke anymore in big files.
