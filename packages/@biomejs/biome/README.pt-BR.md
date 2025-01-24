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

  [हिन्दी](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.hi.md) | [English](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.md) | [Français](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.fr.md) | [繁體中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-TW.md) | [简体中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-CN.md) | [日本語](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.ja.md) | Português do Brasil | [한국어](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.kr.md) | [Русский](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.ru.md) | [Українська](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.uk.md)
</div>

<br>

**Biome** é um conjunto de ferramentas de alto desempenho para projetos web, visando fornecer recursos de desenvolvimento para manter a saúde desses projetos.

**Biome é um [formatador rápido](./benchmark#formatting)** para _JavaScript_, _TypeScript_, _JSX_, e _JSON_ que atinge **[97% de compatibilidade com o _Prettier_](https://console.algora.io/challenges/prettier)**.

**Biome é um [linter eficiente](https://github.com/biomejs/biome/tree/main/benchmark#linting)** para _JavaScript_, _TypeScript_, e _JSX_ que possui **[mais de 270 regras](https://biomejs.dev/linter/rules/)** do ESLint, typescript-eslint, e de [outras fontes](https://github.com/biomejs/biome/discussions/3).
Ele **fornece diagnósticos detalhados e contextualizados** que ajudam você a melhorar seu código e se tornar um programador melhor!

**Biome** é projetado desde o início para ser usado [interativamente dentro de um editor](https://biomejs.dev/guides/integrate-in-editor/).
Isso permite formatar e lintar códigos malformados enquanto você programa.

### Instalação

```shell
npm install --save-dev --save-exact @biomejs/biome
```

### Uso

```shell
# formatar arquivos
npx @biomejs/biome format --write ./src

# lintar arquivos
npx @biomejs/biome lint ./src

# executar formatação, lint, etc. e aplicar as sugestões seguras
npx @biomejs/biome check --write ./src

# verificar todos os arquivos contra formatação, lint, etc. em ambientes CI
npx @biomejs/biome ci ./src
```

Se você quiser experimentar o Biome sem instalá-lo, use o [playground online](https://biomejs.dev/playground/), compilado para WebAssembly.

## Documentação

Confira nossa [página inicial][biomejs] para saber mais sobre o Biome,
ou vá ao [Guia de Introdução][getting-started] para começar a usar o Biome.

## Mais sobre o Biome

**Biome** tem padrões robustos e não requer configuração.

**Biome** visa suportar [todas as principais linguagens][language-support] do desenvolvimento web moderno.

**Biome** [não requer Node.js](https://biomejs.dev/guides/manual-installation/) para funcionar.

**Biome** tem suporte de primeira linha para LSP, com um

 parser sofisticado que representa o texto-fonte em sua total fidelidade e recuperação de erro de ponta.

**Biome** unifica funcionalidades que anteriormente eram ferramentas separadas. Construindo sobre uma base compartilhada, podemos fornecer uma experiência coesa para processar código, exibir erros, paralelizar trabalho, cache e configuração.

Leia mais sobre nossa [filosofia de projeto][biome-philosophy].

**Biome** é licenciado sob [MIT](https://github.com/biomejs/biome/tree/main/LICENSE-MIT) ou [Apache 2.0](https://github.com/biomejs/biome/tree/main/LICENSE-APACHE) e moderado sob o [Código de Conduta do Contribuidor](https://github.com/biomejs/biome/tree/main/CODE_OF_CONDUCT.md).

## Patrocinadores

### Patrocinadores Ouro

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


### Patrocinadores Prata

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

### Patrocinadores Bronze

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

[biomejs]: https://biomejs.dev/pt-br/
[biome-philosophy]: https://biomejs.dev/pt-br/internals/philosophy/
[language-support]: https://biomejs.dev/pt-br/internals/language-support/
[getting-started]: https://biomejs.dev/pt-br/guides/getting-started/
