---
"@biomejs/biome": minor
---

Added the `biome completion` command, which enables Tab completion of Biome commands and options in Bash, Zsh, fish, and Elvish, and of file paths in Bash and Zsh. Completion also suggests rule names, groups, and domains for `--only`, `--skip`, and `biome explain`.

`biome completion <shell>` prints the script for a shell, and `--write` installs it for Bash, Zsh, and fish:

```shell
biome completion --write zsh
```

For Elvish, add the output of `biome completion elvish` to `rc.elv`.
