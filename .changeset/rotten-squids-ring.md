---
"@biomejs/biome": minor
---

Added the ability to load the configuration from new known paths. Biome now attempts to load the configuration files
from the following locations:
- `$XDG_CONFIG_HOME` or `$HOME/.config/biome` on Linux
- `/Users/$USER/Library/Application Support/biome` on macOS
- `C:\Users\$USER\AppData\Roaming\biome\config` on Windows

The priority how Biome will attempt to load the configuration files is the following:
1. project folder (working directory)
2. parent folders
3. config home
   1. `$XDG_CONFIG_HOME` or `$HOME/.config/biome` on Linux
   2. `/Users/$USER/Library/Application Support/biome` on macOS
   3. `C:\Users\$USER\AppData\Roaming\biome\config` on Windows
