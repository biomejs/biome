# Releases

This document is meant to explain the release process of Biome, and hopefully answer some questions.

The team doesn't provide ETAs (Estimated Time of Arrival). The team believes that enforcing deadlines to a group of volunteers is counterproductive, and can have negative effects on people.

## `npm` packages

### Prerelease

We publish pre-releases of the main `@biomejs/biome` package twice a week. These releases are built from `main`, they are meant for testing and verify that bugs are fixed.

These releases are published to `pkg.pr.new`, and an automated message is sent on [Discord](https://biomejs.dev/chat), in the `#release` channel.

> [!WARNING]
> **Don't** use prerelease in **production**. Artifacts in `pkg.pr.new` are purged after roughly 30 days.

### Beta release

Beta releases are published manually by the [Core Contributors team](/GOVERNANCE.md#core-contributor), only this team has the rights to publish these releases.

These releases can be released on request, usually right before a stable release.

Beta releases are published on GitHub, and `npmjs.org` under the `beta` tag.

### Stable release

Beta releases are published manually by the [Core Contributors team](/GOVERNANCE.md#core-contributor), only this team has the rights to publish these releases.

Stable releases are published on GitHub, and `npmjs.org` under the `latest` tag.


## Crates

The Biome internal crates are published on `crates.io` on demand to avoid too much work on our end.
You can reach us out on [discord](https://biomejs.dev/chat) if you require a version of the crates to be published.

As for now, no changelogs or tags are published when new crates are released.

All Biome crates are published under a one single version.
