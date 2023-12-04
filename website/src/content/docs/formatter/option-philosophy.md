---
title: Formatter Option Philosophy
description: Configuring an opinionated formatter.
---

>💡 Biome follows the same [Option Philosophy as Prettier](https://prettier.io/docs/en/option-philosophy). The existing set of options for formatting is considered stable, and new options are not likely to be considered.
>
>This document explains some history about how and why Biome got to where it is today, and an outlook for the future.

Biome is an *opinionated formatter*. In an ideal world, that means Biome assumes there is only one correct way to format things and will enforce that style at all times. No matter the project, no matter the setup, code formatted by Biome will always look the same. From another perspective, Biome is its own *automatic style guide*, not a tool for implementing other style guides.

Having such a strong opinion on formatting may seem heavy-handed, but the benefits quickly become clear after adoption. All of the discussions about where spaces should go, whether a line should be broken out, whether a line should be indented, and so many more simply *vanish*. [Trivial, bike-shedding discussions](https://en.wikipedia.org/wiki/Law_of_triviality) no longer distract from focusing on what matters. Code reviews become free of re-formatting requests and cyclical debates. All it takes is trust that Biome does its best to format code cleanly, legibly, and consistently.

Beyond the benefits within individual teams and organizations, the adoption of consistent formatters across the whole web ecosystem benefits everyone, making it easier to retain familiarity when moving between projects and helping newcomers learn and recognize patterns more intuitively without distractions.

In the web ecosystem today, Prettier is by far the most popular code formatter, and it is also strongly opinionated, with a [strict philosophy on adding options](https://prettier.io/docs/en/option-philosophy). Biome aims to be [largely compatible with Prettier](https://biomejs.dev/blog/biome-wins-prettier-challenge), and as such, has adopted many of the opinions that Prettier implements, and configuration is no exception to that.

Biome is proud to have reached such high compatibility with Prettier and make the migration path for users as painless as possible, but this also comes with similar caveats.

## Existing Options

Biome started out with a strict subset of configuration options, targeting the most common and contentious style guidelines in the JavaScript ecosystem: indent styles (tabs vs spaces), indent widths (2 spaces to equal a tab, or 4?), and enforced semicolons. Adding options for these points was considered sufficient enough to address most people’s needs, and there was no strong consideration for adding any others.

Leaning on the [Prettier Option Philosophy](https://prettier.io/docs/en/option-philosophy), Biome had the chance to start fresh and avoid the pitfalls that Prettier had fallen into with its other existing options, like `--bracket-same-line` and `--arrow-parens`:

> …[these] are not the type of options we’re happy to have. They cause a lot of bike-shedding in teams, and we’re sorry for that. Difficult to remove now, these options exist as a historical artifact and should not motivate adding more options (“If *those* options exist, why can’t this one?”).

However, when the [Prettier Challenge was announced](https://twitter.com/Vjeux/status/1722733472522142022), Biome decided to accept the challenge, which required implementing all of the configuration options that Prettier had to achieve full compatibility.

Biome still shares Prettier's philosophy about these options and considers them a legacy feature for compatibility rather than a baseline feature set. Their existence does not indicate that more options will be added, nor should they be used as a rationale to support the existence of other options in the future.

## New Options

Much like Prettier, Biome believes the current set of options is stable, sufficient, and not open for additions or other changes. Requests for additional configuration options are not likely to be considered and may be closed without discussion.

That said, even as Biome has established itself as a capable and robust formatting tool, it is also still relatively new, meaning there is plenty of opportunity to pave the way for new advancements and ideas that may not seem feasible otherwise.

The formatting style of Biome is also considered relatively stable, continuing to match Prettier as much as possible, with [few intentional deviations](https://github.com/biomejs/biome/issues/739). Changes to the style of Biome may be considered and implemented. Still, these are also unlikely to become configurable options and would instead be applied universally for all future versions of Biome.