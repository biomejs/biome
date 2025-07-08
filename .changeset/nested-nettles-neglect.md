---
"@biomejs/biome": patch
---

If a nested configuration file is ignored by the root configuration, it will now actually be ignored.

Biome has an exception in place for configuration files so they cannot be ignored, because the configuration files are vital to Biome itself. But this exception was incorrectly applied to nested configurations as well. Now only the root configuration is exempt from being ignored.
