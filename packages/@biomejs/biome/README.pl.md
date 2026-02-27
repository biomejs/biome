<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-dark-transparent.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg">
    <img alt="Shows the banner of Biome, with its logo and the phrase 'Biome - Toolchain of the web'." src="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg" width="700">
  </picture>

  <br>
  <br>

  [![CI on main][ci-badge]][ci-url]
  [![Discord chat][discord-badge]][discord-url]
  [![npm version][npm-badge]][npm-url]
  [![VSCode version][vscode-badge]][vscode-url]
  [![Open VSX version][open-vsx-badge]][open-vsx-url]

  [ci-badge]: https://github.com/biomejs/biome/actions/workflows/main.yml/badge.svg
  [ci-url]: https://github.com/biomejs/biome/actions/workflows/main.yml
  [discord-badge]: https://badgen.net/discord/online-members/BypW39g6Yc?icon=discord&label=discord&color=60a5fa
  [discord-url]: https://biomejs.dev/chat
  [npm-badge]: https://badgen.net/npm/v/@biomejs/biome?icon=npm&color=60a5fa&label=%40biomejs%2Fbiome
  [npm-url]: https://www.npmjs.com/package/@biomejs/biome/v/latest
  [vscode-badge]: https://img.shields.io/visual-studio-marketplace/v/biomejs.biome?label=Visual%20Studio%20Marketplace&labelColor=374151&color=60a5fa
  [vscode-url]: https://marketplace.visualstudio.com/items?itemName=biomejs.biome
  [open-vsx-badge]: https://img.shields.io/visual-studio-marketplace/v/biomejs.biome?label=Open%20VSX%20Registry&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4KPHN2ZyB2aWV3Qm94PSI0LjYgNSA5Ni4yIDEyMi43IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogIDxwYXRoIGQ9Ik0zMCA0NC4yTDUyLjYgNUg3LjN6TTQuNiA4OC41aDQ1LjNMMjcuMiA0OS40em01MSAwbDIyLjYgMzkuMiAyMi42LTM5LjJ6IiBmaWxsPSIjYzE2MGVmIi8+CiAgPHBhdGggZD0iTTUyLjYgNUwzMCA0NC4yaDQ1LjJ6TTI3LjIgNDkuNGwyMi43IDM5LjEgMjIuNi0zOS4xem01MSAwTDU1LjYgODguNWg0NS4yeiIgZmlsbD0iI2E2MGVlNSIvPgo8L3N2Zz4=&labelColor=374151&color=60a5fa
  [open-vsx-url]: https://open-vsx.org/extension/biomejs/biome

<!-- Insert new entries lexicographically by language code.
     For example given below is the same order as these files appear on page:
     https://github.com/biomejs/biome/tree/main/packages/@biomejs/biome -->

  [हिन्दी](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.hi.md) | [English](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.md) | [Español](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.es.md) | [Français](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.fr.md) | [繁體中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-TW.md) | [简体中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-CN.md) | [日本語](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.ja.md) | Polski | [Português do Brasil](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.pt-BR.md) | [한국어](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.kr.md) | [Русский](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.ru.md) | [Українська](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.uk.md)
</div>

<br>

**Biome** to wydajny łańcuch narzędzi dla projektów webowych, którego celem jest dostarczenie narzędzi deweloperskich do utrzymania zdrowia tych projektów.

**Biome to [szybki formater](./benchmark#formatting)** dla _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_ i _GraphQL_, który osiąga **[97% kompatybilności z _Prettier_](https://console.algora.io/challenges/prettier)**.

**Biome to [wydajny linter](https://github.com/biomejs/biome/tree/main/benchmark#linting)** dla _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_ i _GraphQL_, który zawiera **[ponad 340 reguł](https://biomejs.dev/linter/javascript/rules/)** z ESLint, typescript-eslint i [innych źródeł](https://github.com/biomejs/biome/discussions/3).
**Wyprowadza szczegółowe i skontekstualizowane diagnostyki**, które pomagają Ci ulepszyć kod i zostać lepszym programistą!

**Biome** jest zaprojektowany od początku do używania [interaktywnie w edytorze](https://biomejs.dev/guides/editors/first-party-extensions/).
Może formatować i sprawdzać błędny kod podczas jego pisania.

### Instalacja

```shell
npm install --save-dev --save-exact @biomejs/biome
```

### Użycie

```shell
# formatuj pliki
npx @biomejs/biome format --write

# sprawdź pliki i zastosuj bezpieczne poprawki
npx @biomejs/biome lint --write

# uruchom formatowanie, linting itp. i zastosuj bezpieczne poprawki
npx @biomejs/biome check --write

# sprawdź wszystkie pliki pod kątem formatowania, lintingu itp. w środowiskach CI
npx @biomejs/biome ci
```

Jeśli chcesz wypróbować Biome bez instalacji, użyj [online playground](https://biomejs.dev/playground/), skompilowanego do WebAssembly.

## Dokumentacja

Sprawdź naszą [stronę główną][biomejs], aby dowiedzieć się więcej o Biome,
lub przejdź bezpośrednio do [przewodnika Getting Started][getting-started], aby zacząć używać Biome.

## Więcej o Biome

**Biome** ma rozsądne domyślne ustawienia i nie wymaga konfiguracji.

**Biome** dąży do wspierania [wszystkich głównych języków][language-support] nowoczesnego rozwoju webowego.

**Biome** [nie wymaga Node.js](https://biomejs.dev/guides/manual-installation/) do funkcjonowania.

**Biome** ma wsparcie LSP pierwszej klasy, z zaawansowanym parserem, który reprezentuje tekst źródłowy z pełną wiernością i najwyższej jakości odzyskiwaniem błędów.

**Biome** chce oferować wysokiej jakości *Doświadczenie Deweloperskie*, z opisowymi diagnostykami i doskonałą wydajnością.

**Biome** łączy funkcjonalności, które wcześniej były oddzielnymi narzędziami. Budowanie na wspólnej bazie pozwala nam zapewnić spójne doświadczenie w przetwarzaniu kodu, wyświetlaniu błędów, równoległej pracy, cachowaniu i konfiguracji.

Przeczytaj więcej o naszej [filozofii projektu][biome-philosophy].

**Biome** jest licencjonowany na [MIT](https://github.com/biomejs/biome/tree/main/LICENSE-MIT) lub [Apache 2.0](https://github.com/biomejs/biome/tree/main/LICENSE-APACHE) i moderowany zgodnie z [Contributor Covenant Code of Conduct](https://github.com/biomejs/biome/tree/main/CODE_OF_CONDUCT.md).

## Finansowanie

Możesz finansować projekt na różne sposoby

### Sponsoring i finansowanie projektu

Możesz sponsorować lub finansować projekt przez [Open collective](https://opencollective.com/biome) lub [GitHub sponsors](https://github.com/sponsors/biomejs)

Biome oferuje prosty program sponsoringu, który pozwala firmom na uzyskanie widoczności i uznania wśród różnych deweloperów.

Biome oferuje [wsparcie enterprise](https://biomejs.dev/enterprise), gdzie Core Contributors mogą być zatrudnieni do pracy nad projektami skupionymi na firmie.

## Sponsorzy

### Platynowi Sponsorzy

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://depot.dev/?utm_source=biome&utm_medium=readme" target="_blank">
          <picture>
            <source media="(prefers-color-scheme: light)" srcset="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-light@3x.png" />
            <source media="(prefers-color-scheme: dark)" srcset="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-dark@3x.png" />
            <img src="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-light@3x.png" width="600" alt="Depot logo" />
          </picture>
        </a>
      </td>
    </tr>
  </tbody>
</table>

### Srebrni Sponsorzy

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://l2beat.com/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/l2beat/c2b2a27/logo/256.png" height="100" alt="L2BEAT logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://lokalise.com/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/14294501?s=200&v=4" height="100" alt="Lokalise logo"></a>
      </td>
    </tr>
  </tbody>
</table>

### Brązowi Sponsorzy

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://vital.io/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/25357309?s=200" width="80" alt="Vital logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://coderabbit.ai/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/132028505?s=200&v=4" width="80" alt="CodeRabbit logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://forge42.dev/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/161314831?s=200&v=4" width="80" alt="Forge42 logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="http://rstudio.org/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/513560?s=200&v=4" width="80" alt="RStudio logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://pennylane.com/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/57875210?s=200&v=4" width="80" alt="Pennylane logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://jetbrains.com/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://resources.jetbrains.com/storage/products/company/brand/logos/jetbrains.png" width="100" alt="JetBrains logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://www.egstock.co.jp/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/egstock/b18c836/logo/256.png?height=256" width="80" alt="EGSTOCK, Inc. logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://www.convex.dev/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/81530787?s=200&v=4" width="80" alt="Convex logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://graphite.dev/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/61942612?s=200&v=4" width="80" alt="Graphite logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://kraken.tech/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/105941848?s=200&v=4" width="80" alt="Kraken Tech logo"></a>
      </td>
    </tr>
  </tbody>
</table>

[biomejs]: https://biomejs.dev/
[biome-philosophy]: https://biomejs.dev/internals/philosophy/
[language-support]: https://biomejs.dev/internals/language-support/
[getting-started]: https://biomejs.dev/guides/getting-started/
