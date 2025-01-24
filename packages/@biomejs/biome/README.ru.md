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

  [हिन्दी](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.hi.md) | [English](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.md) | [Français](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.fr.md) | [繁體中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-TW.md) | [简体中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-CN.md) | [日本語](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.ja.md) | [Português do Brasil](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.pt-BR.md) | [한국어](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.kr.md) | Русский
</div>

<br>

**Biome** - это высокопроизводительный набор инструментов для веб-проектов, предоставляющий разработчикам средства для поддержания их качества и работоспособности.

**Biome - это [быстрый форматер](./benchmark#formatting)** для _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_ и _GraphQL_, обеспечивающий **[совместимость с _Prettier_ на 97%](https://console.algora.io/challenges/prettier)**.

**Biome - это [высокопроизводительный линтер](https://github.com/biomejs/biome/tree/main/benchmark#linting)** для _JavaScript_, _TypeScript_, _JSX_, _CSS_ и _GraphQL_, содержащий **[более 270 правил](https://biomejs.dev/linter/rules/)** из ESLint, typescript-eslint, и [других источников](https://github.com/biomejs/biome/discussions/3).
Он **выводит подробную диагностику с контекстной информацией**, которая помогает вам улучшить ваш код и стать более лучшим программистом!

**Biome** изначально разработан для [интерактивной работы в редакторе](https://biomejs.dev/guides/integrate-in-editor/).
Он может форматировать и исправлять некорректный код по мере его написания.

### Установка

```shell
npm install --save-dev --save-exact @biomejs/biome
```

### Использование

```shell
# форматирование файлов
npx @biomejs/biome format --write ./src

# линт файлов и применение безопасных изменений
npx @biomejs/biome lint --write ./src

# форматирование, линт и другие проверки, а также применение безопасных изменений
npx @biomejs/biome check --write ./src

# форматирование, линт и другие проверки в CI
npx @biomejs/biome ci ./src
```

Если вы хотите попробовать Biome без установки, используйте [онлайн-песочницу](https://biomejs.dev/playground/), скомпилированную в WebAssembly.

## Документация

Посетите нашу [домашнюю страницу][biomejs], чтобы узнать больше о Biome,
или сразу перейдите к руководству [Getting Started][getting-started], чтобы начать использовать Biome.

## Больше о Biome

**Biome** настроен таким образом, чтобы подходить большинству случаев, и не требует дополнительной настройки.

**Biome** нацелен на поддержку [всех основных языков][language-support] современной веб-разработки.

**Biome** [не требует Node.js](https://biomejs.dev/guides/manual-installation/) для работы.

**Biome** имеет первоклассную поддержку LSP с продвинутым парсером, который точно отражает исходный текст и обеспечивает эффективное восстановление после ошибок.

**Biome** объединяет функционал, который ранее предоставляли отдельные инструменты. Построение на общей базе позволяет нам обеспечить целостный опыт обработки кода, отображения ошибок, распараллеливания работы, кэширования и конфигурации.

Почитайте больше о нашей [философии проекта][biome-philosophy].

**Biome** находится под [лицензией MIT](https://github.com/biomejs/biome/tree/main/LICENSE-MIT) или [лицензией Apache 2.0](https://github.com/biomejs/biome/tree/main/LICENSE-APACHE), и модерируется в соотвествии с [Contributor Covenant Code of Conduct](https://github.com/biomejs/biome/tree/main/CODE_OF_CONDUCT.md).

## Финансирование

Вы можете поддержать проект несколькими способами:

### Спонсорство и финансирование проекта

Вы можете спонсировать или финансировать проект через [Open collective](https://opencollective.com/biome) или [GitHub sponsors](https://github.com/sponsors/biomejs).

Biome предлагает простую спонсорскую программу, которая позволяет компаниям получить известность и признание среди различных разработчиков.

### Финансирование ишью

Мы используем [Polar.sh](https://polar.sh/biomejs) для поддержки и продвижения конкретных фич, которые вы хотели бы увидеть и реализовать. Проверьте наш бэклог и помогите нам:

## Спонсоры

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


### Серебряные спонсоры

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://l2beat.com/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/l2beat/c2b2a27/logo/256.png" height="100"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://www.phoenixlabs.dev/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/phoenix-labs/2824ed4/logo/100.png?height=100" height="100"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://lokalise.com/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/14294501?s=200&v=4" height="100"></a>
      </td>
    </tr>
  </tbody>
</table>

### Бронзовые спонсоры

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
