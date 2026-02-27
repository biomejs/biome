<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-dark-transparent.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg">
    <img alt="Muestra el banner de Biome, con su respectivo logo y la frase 'Biome - Toolchain of the web'." src="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg" width="700">
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

  [हिन्दी](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.hi.md) | [English](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.md) | Español | [Français](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.fr.md) | [繁體中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-TW.md) | [简体中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-CN.md) | [日本語](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.ja.md) | [Polski](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.pl.md) | [Português do Brasil](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.pt-BR.md) | [한국어](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.kr.md) | [Русский](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.ru.md) | [Українська](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.uk.md)
</div>

<br>

**Biome** es una toolchain de alto rendimiento para proyectos web. Su objetivo es proveer herramientas que mantengan la salud de dichos proyectos.

**Biome es un [formatter rápido](https://github.com/biomejs/biome/tree/main/benchmark#formatting)** para _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_ y _GraphQL_ que alcanza un **[97% de compatibilidad con _Prettier_](https://console.algora.io/challenges/prettier)**.

**Biome es un [linter de alto rendimiento](https://github.com/biomejs/biome/tree/main/benchmark#linting)** para _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_, y _GraphQL_ que incluye **[más de 340 reglas](https://biomejs.dev/linter/javascript/rules/)** de ESLint, typescript-eslint, y [otras fuentes](https://github.com/biomejs/biome/discussions/3).
Produce **diagnósticos detallados y contextualizados** que ayudan a mejorar tu código y convertirte en un mejor programador!

**Biome** fue diseñado desde cero para usarse [interactivamente dentro de un editor](https://biomejs.dev/guides/editors/first-party-extensions/).
Puede formatear y analizar código mal formado mientras lo estás escribiendo.

### Instalación

```shell
npm install --save-dev --save-exact @biomejs/biome
```

### Uso

```shell
# formatea archivos
npx @biomejs/biome format --write

# analiza archivos y aplica correcciones seguras
npx @biomejs/biome lint --write

# ejecuta format, lint, etc. y aplica correcciones seguras
npx @biomejs/biome check --write

# valida todos los archivos con format, lint, etc. en entornos CI
npx @biomejs/biome ci
```

Si querés probar Biome sin instalarlo, usá el [playground online](https://biomejs.dev/playground/), compilado a WebAssembly.

## Documentación

Visitá nuestra [página principal][biomejs] para aprender más sobre Biome,
o andá directamente a la [guía de inicio][getting-started] para empezar a usarlo.

## Más sobre Biome

**Biome** tiene valores predeterminados sanos y no requiere configuración.

**Biome** busca soportar [todos los lenguajes principales][language-support] del desarrollo web moderno.

**Biome** [no necesita Node.js](https://biomejs.dev/guides/manual-installation/) para funcionar.
**Biome** tiene soporte de primera clase para LSP, con un parser sofisticado que representa el texto fuente con total fidelidad y excelente recuperación de errores.

**Biome** apunta a ofrecer una *Experiencia de Desarrollador* de alta calidad, con diagnósticos descriptivos y gran rendimiento.

**Biome** unifica funcionalidades que antes estaban en herramientas separadas. Construir sobre una base compartida nos permite dar una experiencia cohesiva para procesar código, mostrar errores, paralelizar trabajo, usar caché y manejar configuración.

Leé más sobre nuestra [filosofía del proyecto][biome-philosophy].

**Biome** está bajo licencia [MIT](https://github.com/biomejs/biome/tree/main/LICENSE-MIT) o [Apache 2.0 licensed](https://github.com/biomejs/biome/tree/main/LICENSE-APACHE)y moderado bajo el [Código de Conducta](https://github.com/biomejs/biome/tree/main/CODE_OF_CONDUCT.md).

## Financiamiento

Podés financiar el proyecto de distintas maneras

### Patrocinios y financiamiento

Podés patrocinar o financiar el proyecto a través de [Open collective](https://opencollective.com/biome) o [GitHub sponsors](https://github.com/sponsors/biomejs)

Biome ofrece un programa de patrocinio simple que permite a las empresas obtener visibilidad y reconocimiento entre varios desarrolladores.

Biome ofrece [soporte empresarial](https://biomejs.dev/enterprise), donde colaboradores principales pueden ser contratados para trabajar en proyectos enfocados a compañías.

## Sponsors

### Sponsors Platino

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

### Sponsors Plata

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

### Sponsors Bronce

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
