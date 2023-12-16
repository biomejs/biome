# Contributing Guidelines for our docs

Are you a native speaker of a language other than English? We'd love to have your help!

We're using [Starlight](https://starlight.astro.build/) for our docs website, which allows us to have multiple languages in the same repository. This means that you can contribute to the docs in your language without having to worry about maintaining a separate repository. Consider reading the [Starlight i18n docs](https://starlight.astro.build/guides/i18n/) for more information.

## How to contribute

(Read the [contribution guidelines](../CONTRIBUTING.md) first for general information about contributing to this project.)

### Initial Translation

If the docs are not available in your language yet and you would like to translate them, you can do so by following these steps:

1. Find or create an issue for the language you want to translate to. If there is no issue for your language yet, feel free to create one. This is to avoid duplicate work, and also allows for community to track the progress of the translation.
   - If existing issue is found, please comment on the issue to let others know you're working on it. We recommend creating single PR for each doc, so that it's easier to review.

2. Make sure the i18n config in Starlight already configured, or you can configure it then create a PR for it first, read ["Configure i18n"](https://starlight.astro.build/guides/i18n/#configure-i18n).

3. Progressively copy only file/folder you want to translate from `src/content/docs` directory to a new/existing directory (exclude all other translated code, e.g. `ja`, `pt-br`, etc.) with the name of your language, e.g. `src/content/docs/de` for German. (If you're not sure what the language code is, you can find it [here](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).). Note: Starlight will automatically fallback to English if a translation for specific doc is not available.

4. Translate the files you copied in the previous step and create a PR or Draft PR for each file you translated.

### Tips

- We configure the sidebar translation in `astro.config.ts`
- When you're done, mark the pull request as ready for review.

### Reviewing

We aim to have 1-2 reviews on each PR before merging. This allows for some back and forth and ensures that the quality of the docs is high, and the tone is consistent.

We'd highly appreciate it if you knew someone who speaks the language you're translating into to review your PR. This can help speed up the process of getting your contribution merged.

## Credits

This document was inspired by the translation contribution guidelines for [t3-oss/create-t3-app](https://github.com/t3-oss/create-t3-app/blob/main/www/TRANSLATIONS.md).
