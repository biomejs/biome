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

  [हिन्दी](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.hi.md) | [English](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.md) | [Français](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.fr.md) | [繁體中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-TW.md) | [简体中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-CN.md) | [日本語](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.ja.md) | [Português do Brasil](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.pt-BR.md) | 한국어 | [Русский](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.ru.md) | [Українська](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.uk.md)
</div>

<br>

**Biome**은 웹 프로젝트를 위한 고성능 툴체인으로, 프로젝트의 품질을 유지하기 위한 개발자 도구를 제공하는 것을 목표로 합니다.

**Biome**은 _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_ 및 *GraphQL*을 위한 **[고속 포매터](https://github.com/biomejs/biome/tree/main/benchmark#formatting)로**, **[*Prettier*와 97%의 호환성](https://console.algora.io/challenges/prettier)을** 자랑합니다.

**Biome**은 _JavaScript_, _TypeScript_, _JSX_, _CSS_ 및 *GraphQL*을 위한 **[고성능 린터](https://github.com/biomejs/biome/tree/main/benchmark#linting)로**, ESLint, typescript-eslint 등 [다양한 소스](https://github.com/biomejs/biome/discussions/3)에서 가져온 **[270개 이상의 규칙](https://biomejs.dev/linter/rules/)을** 제공합니다. **맥락을 고려한 상세한 진단 정보**를 제공하여 코드 품질 향상과 개발 실력 향상에 도움을 줍니다!

**Biome**은 처음부터 [에디터와 상호작용](https://biomejs.dev/guides/editors/first-party-extensions/)하도록 설계되었습니다. 코드를 작성하는 동안에도 포매팅과 린팅을 수행할 수 있습니다.

### 설치

```shell
npm install --save-dev --save-exact @biomejs/biome
```

### 사용법

```shell
# 파일 포매팅
npx @biomejs/biome format --write ./src

# 파일 린팅 및 안전한 수정 사항 적용
npx @biomejs/biome lint --write ./src

# 포매팅, 린팅 등 모든 검사를 수행하고 안전한 수정 사항 적용
npx @biomejs/biome check --write ./src

# CI 환경에서 모든 파일에 대해 포매팅, 린팅 등 검사 수행
npx @biomejs/biome ci ./src
```

설치하지 않고 Biome을 사용해보고 싶다면, 웹 어셈블리로 컴파일된 [온라인 플레이그라운드](https://biomejs.dev/playground/)를 이용해보세요.

## 문서

Biome에 대한 자세한 내용은 [홈페이지][biomejs]에서 확인할 수 있으며,
바로 사용하려면 [시작 가이드][getting-started]로 이동하세요.

## Biome에 대해 자세히 알아보기

**Biome**은 합리적인 기본 설정을 제공하므로 별도의 설정이 필요하지 않습니다.

**Biome**은 현대 웹 개발에서 사용되는 [모든 주요 언어][language-support]를 지원하는 것을 목표로 합니다.

**Biome**은 [Node.js 없이](https://biomejs.dev/guides/manual-installation/)도 실행할 수 있습니다.

**Biome**은 최고 수준의 LSP를 지원하며, 소스 텍스트를 완벽하게 표현하고 뛰어난 오류 복구 기능을 갖춘 정교한 파서를 사용합니다.

**Biome**은 이전에 개별 도구로 분리되어 있던 기능들을 통합했습니다. 공통된 기반을 구축하여 코드 처리, 오류 표시, 작업 병렬화, 캐싱 및 구성에 대해 일관된 경험을 제공합니다.

더 많은 정보는 [프로젝트 철학][biome-philosophy]에서 확인할 수 있습니다.

**Biome**은 [MIT 라이선스](https://github.com/biomejs/biome/tree/main/LICENSE-MIT) 또는 [Apache 2.0 라이선스](https://github.com/biomejs/biome/tree/main/LICENSE-APACHE)를 따르며, [기여자 서약 행동 강령](https://github.com/biomejs/biome/tree/main/CODE_OF_CONDUCT.md)에 따라 관리됩니다.

## 후원

다양한 방법으로 프로젝트를 후원할 수 있습니다.

### 프로젝트 후원

[Open Collective](https://opencollective.com/biome)나 [GitHub Sponsors](https://github.com/sponsors/biomejs)를 통해 프로젝트를 후원할 수 있습니다.

Biome은 기업이 다양한 개발자들 사이에서 인지도를 얻을 수 있는 간단한 후원 프로그램을 제공합니다.

### 이슈 후원

[Polar.sh](https://polar.sh/biomejs)를 통해 구현되었으면 하는 기능들에 투표하고 개발을 지원할 수 있습니다. 백로그를 확인하고 도움을 주세요:

## 스폰서

### Platinum Sponsors

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://vercel.com/?utm_source=biome&utm_medium=readme" target="_blank">
          <picture>
            <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/biomejs/resources/refs/heads/main/sponsors/vercel-dark.png" />
            <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/biomejs/resources/refs/heads/main/sponsors/vercel-light.png" />
            <img src="https://raw.githubusercontent.com/biomejs/resources/refs/heads/main/sponsors/vercel-light.png" width="500" alt="Vercel" />
          </picture>
        </a>
      </td>
    </tr>
  </tbody>
</table>

### Gold Sponsors

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://depot.dev/?utm_source=biome&utm_medium=readme" target="_blank">
          <picture>
            <source media="(prefers-color-scheme: light)" srcset="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-light@3x.png" />
            <source media="(prefers-color-scheme: dark)" srcset="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-dark@3x.png" />
            <img src="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-light@3x.png" width="400" alt="Depot" />
          </picture>
        </a>
      </td>
    </tr>
  </tbody>
</table>

### 실버 스폰서

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://l2beat.com/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/l2beat/c2b2a27/logo/256.png" height="100" alt="L2BEAT logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://www.phoenixlabs.dev/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/phoenix-labs/2824ed4/logo/100.png?height=100" height="100" alt="Phoenix Labs logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://lokalise.com/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/14294501?s=200&v=4" height="100" alt="Lokalise logo"></a>
      </td>
    </tr>
  </tbody>
</table>

### 브론즈 스폰서

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://nanabit.dev/?utm_source=biome&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/nanabit/d15fd98/logo/256.png?height=80" width="80" alt="Nanabit logo"></a>
      </td>
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
    </tr>
  </tbody>
</table>

[biomejs]: https://biomejs.dev/
[biome-philosophy]: https://biomejs.dev/internals/philosophy/
[language-support]: https://biomejs.dev/internals/language-support/
[getting-started]: https://biomejs.dev/guides/getting-started/
