# Biome JetBrains IDEs Plugin

https://biomejs.dev plugin for JetBrains IDEs.

## features

- Linting
- Quickfix
- Formatting

---

## Installation

### From JetBrains IDEs

- Press `⌘Сmd,` to open the IDE settings and then select Plugins.
![Plugins](https://resources.jetbrains.com/help/img/idea/2023.2/ws_plugins_settings.png)
- Search for Biome and click `install`

### From disk

- Download the plugin .zip from releases tab.
- Press `⌘Сmd,` to open the IDE settings and then select Plugins.
- On the Plugins page, click The Settings button and then click Install Plugin from Disk….

## Building and running the plugin

Build and run the plugin requires:

- Java development kit 17+
- IntelliJ IDEA (Ultimate edition or community edition)

### Running the plugin on IDEA

```shell
./gradlew runIde
```

### Build the plugin

```shell
./gradlew buildPlugin
```
