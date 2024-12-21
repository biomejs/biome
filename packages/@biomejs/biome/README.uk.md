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
  [![Polar bounties][polar-badge]][polar-url]

  [ci-badge]: https://github.com/biomejs/biome/actions/workflows/main.yml/badge.svg
  [ci-url]: https://github.com/biomejs/biome/actions/workflows/main.yml
  [discord-badge]: https://badgen.net/discord/online-members/BypW39g6Yc?icon=discord&label=discord&color=60a5fa
  [discord-url]: https://biomejs.dev/chat
  [npm-badge]: https://badgen.net/npm/v/@biomejs/biome?icon=npm&color=60a5fa&label=%40biomejs%2Fbiome
  [npm-url]: https://www.npmjs.com/package/@biomejs/biome/v/latest
  [vscode-badge]: https://badgen.net/vs-marketplace/v/biomejs.biome?label=vscode&icon=visualstudio&color=60a5fa
  [vscode-url]: https://marketplace.visualstudio.com/items?itemName=biomejs.biome
  [open-vsx-badge]: https://badgen.net/open-vsx/version/biomejs/biome?label=open-vsx&color=60a5fa
  [open-vsx-url]: https://open-vsx.org/extension/biomejs/biome
  [polar-badge]: https://polar.sh/embed/seeks-funding-shield.svg?org=biomejs
  [polar-url]: https://polar.sh/biomejs

  <!-- Insert new entries lexicographically by language code.
     For example given below is the same order as these files appear on page:
     https://github.com/biomejs/biome/tree/main/packages/@biomejs/biome -->

  [हिन्दी](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.hi.md) | [English](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.md) | [Français](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.fr.md) | [繁體中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-TW.md) | [简体中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-CN.md) | [日本語](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.ja.md) | [Português do Brasil](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.pt-BR.md) | [한국어](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.kr.md) | Українська
</div>

<br>

**Biome** - це високопродуктивний інструментарій для веб-проєктів, який має на меті надавати інструменти розробникам для підтримки здоров'я проєктів.

**Biome - це [швидкий форматувальник](./benchmark#formatting)** для _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_ та _GraphQL_, який досягає **[97% сумісності з _Prettier_](https://console.algora.io/challenges/prettier)**.

**Biome - це [високопродуктивний лінтер](https://github.com/biomejs/biome/tree/main/benchmark#linting)** для _JavaScript_, _TypeScript_, _JSX_, _CSS_ та _GraphQL_, який містить **[понад 270 правил](https://biomejs.dev/linter/rules/)** з ESLint, typescript-eslint та [інших джерел](https://github.com/biomejs/biome/discussions/3).
Він **виводить детальні та контекстуалізовані діагностичні дані**, які допомагають вам покращити ваш код та стати кращим програмістом!

**Biome** з самого початку розроблений для [інтерактивного використання в редакторі](https://biomejs.dev/guides/integrate-in-editor/). Він може форматувати та лінтити некоректний код під час його написання.

### Встановлення

```shell
npm install --save-dev --save-exact @biomejs/biome
```

### Використання

```shell
# форматування файлів
npx @biomejs/biome format --write ./src

# лінтинг файлів та застосування безпечних виправлень
npx @biomejs/biome lint --write ./src

# запуск форматування, лінтингу тощо та застосування безпечних виправлень
npx @biomejs/biome check --write ./src

# перевірка всіх файлів на відповідність форматуванню, лінтингу тощо в середовищах CI
npx @biomejs/biome ci ./src
```

Якщо ви хочете спробувати Biome без встановлення, скористайтеся [онлайн-пісочницею](https://biomejs.dev/playground/), скомпільованою у WebAssembly.

## Документація

Перегляньте нашу [домашню сторінку][biomejs], щоб дізнатися більше про Biome,
або перейдіть безпосередньо до [посібника з початку роботи][getting-started], щоб почати використовувати Biome.

## Більше про Biome

**Biome** має розумні налаштування за замовчуванням і не потребує конфігурації.

**Biome** прагне підтримувати [всі основні мови][language-support] сучасної веб-розробки.

**Biome** [не потребує Node.js](https://biomejs.dev/guides/manual-installation/) для роботи.

**Biome** має першокласну підтримку LSP, з витонченим парсером, який представляє вихідний текст з повною точністю та найкращим відновленням помилок.

**Biome** об'єднує функціональність, яка раніше була окремими інструментами. Побудова на спільній основі дозволяє нам забезпечити узгоджений досвід обробки коду, відображення помилок, паралельної роботи, кешування та конфігурації.

Дізнайтеся більше про нашу [філософію проєкту][biome-philosophy].

**Biome** має [ліцензію MIT](https://github.com/biomejs/biome/tree/main/LICENSE-MIT) або [ліцензію Apache 2.0](https://github.com/biomejs/biome/tree/main/LICENSE-APACHE) і регулюється [Кодексом поведінки учасників](https://github.com/biomejs/biome/tree/main/CODE_OF_CONDUCT.md).

## Фінансування

Ви можете фінансувати проєкт різними способами

### Спонсорство та фінансування проєкту

Ви можете спонсорувати або фінансувати проєкт через [Open collective](https://opencollective.com/biome) або [GitHub sponsors](https://github.com/sponsors/biomejs)

Biome пропонує просту програму спонсорства, яка дозволяє компаніям отримувати видимість та визнання серед різних розробників.

### Фінансування завдань

Ми використовуємо [Polar.sh](https://polar.sh/biomejs) для голосування та просування конкретних функцій, які ви хотіли б бачити та реалізувати. Перевірте наш список завдань і допоможіть нам:

## Спонсори

### Gold Sponsors

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://vercel.com/?utm_source=biome&utm_medium=readme" target="_blank">
          <picture>
            <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/biomejs/resources/refs/heads/main/sponsors/vercel-dark.png" />
            <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/biomejs/resources/refs/heads/main/sponsors/vercel-light.png" />
            <img src="https://raw.githubusercontent.com/biomejs/resources/refs/heads/main/sponsors/vercel-light.png" width="400" alt="Vercel" />
          </picture>
        </a>
      </td>
    </tr>
  </tbody>
</table>

### Срібні спонсори

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://l2beat.com/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/l2beat/c2b2a27/logo/256.png" height="100"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://www.phoenixlabs.dev/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/phoenix-labs/2824ed4/logo/100.png?height=100" height="100"></a>
      </td>
    </tr>
  </tbody>
</table>
### Бронзові спонсори

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://www.kanamekey.com?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/kaname/d15fd98/logo/256.png?height=80" width="80"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://nanabit.dev/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/nanabit/d15fd98/logo/256.png?height=80" width="80"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://vital.io/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/25357309?s=200" width="80"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://coderabbit.ai/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/132028505?s=200&v=4" width="80"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://forge42.dev/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/161314831?s=200&v=4" width="80"></a>
      </td>
      <td align="center" valign="middle">
        <a href="http://rstudio.org/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/513560?s=200&v=4" width="80"></a>
      </td>
    </tr>
  </tbody>
</table>


[biomejs]: https://biomejs.dev/
[biome-philosophy]: https://biomejs.dev/internals/philosophy/
[language-support]: https://biomejs.dev/internals/language-support/
[getting-started]: https://biomejs.dev/guides/getting-started/
