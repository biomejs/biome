---
"@biomejs/biome": minor
---

Added the `biome completion` command, which enables Tab completion of Biome commands, options, and file paths in Bash, Zsh, fish, and Elvish. Completion also suggests rule names, groups, and domains for `--only`, `--skip`, and `biome explain`.

`biome completion <shell>` prints the script for a shell, and `--write` installs it:

```shell
biome completion --write zsh
```
