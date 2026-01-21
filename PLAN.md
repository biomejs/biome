I want you to help me designing a new feature for Biome. I want to crate a feature that allows for easy debugging of the configuration. At the moment the configuration can be extended using
the `extends` configuration at resolution time , and `overrides` during the runtime part.

There are also other factors such as the `editorconfig`, which allows to manipulate the configuration defaults.

The extending part, is driven mainly by the `Merge` proc macro. The problem is that, while doing the merging, we lose **who** changed certain field (both resolution and runtime).

I want to design a debugging capability that allows us maintainers and users to understand the following: given a certain field, and an optional file path, which configuration changed its value. There are different factors to take into consideration:
- `extends` is an array, so the last one wins, for a particular field
- Biome supports nested configuration files, so we need to take that into account.
- Given an optional file, also overrides should be taken into considration

Eventually this feature can be used by CLI and LSP both, which means that we must be able to query it from the `Workspace` server, which means that this information
that we store somewhere, must be saved and updated in the Workspace, which means it must be serialisable (at least the input and the output).

Please design a solution for this problem.
