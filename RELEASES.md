# Releases

This document is meant to explain the release process of Biome, and hopefully answer some questions.

The team doesn't provide ETAs (Estimated Time of Arrival). The team believes that enforcing deadlines for a group of volunteers is counterproductive, and can have negative effects on people.

## `npm` packages

### Prerelease

We publish pre-releases of the main `@biomejs/biome` package twice a week. These releases are built from `main`, they are meant for testing and verifying that bugs are fixed.

Occasionally, we will also **unstable** releases, which might contain new features not currently available in the stable channel.

These releases are published to `pkg.pr.new`, and an automated message is sent on [Discord](https://biomejs.dev/chat), in the `#release` channel.

> [!WARNING]
> **Don't** use prerelease in **production**. Artifacts in `pkg.pr.new` are purged after roughly 30 days.

### Beta release

Beta releases are published manually by the [Core Contributors team](/GOVERNANCE.md#core-contributor), only this team has the right to publish these releases.

These releases can be released on request, usually right before a stable release.

Beta releases are published on GitHub, and `npmjs.org` under the `beta` tag.

### Stable release

Stable releases are published manually by the [Core Contributors team](/GOVERNANCE.md#core-contributor), only this team has the right to publish these releases.

Stable releases are published on GitHub, and `npmjs.org` under the `latest` tag.


## Crates

The Biome internal crates are published on `crates.io` on demand to avoid too much work on our end.
You can reach us out on [discord](https://biomejs.dev/chat) if you require a version of the crates to be published.

The published crates share a single version (either patch or minor) and a single git tag.

The language-specific analyzers are workspace-only. The shared `biome_analyze` crate and its macros
remain published. We also no longer publish new versions of `biome_fs`, `biome_grit_patterns`, `biome_js_runtime`,
`biome_js_transform`, `biome_js_type_info`, `biome_manifest`, `biome_module_graph`, `biome_package`,
`biome_plugin_loader`, `biome_project_layout`, `biome_resolver`, or `biome_rule_options`.
Existing releases remain available on crates.io. This does not change the CLI's capabilities.
