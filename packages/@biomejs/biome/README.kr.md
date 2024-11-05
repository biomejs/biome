<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-dark-transparent.svg">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg">
        <img alt="Biome의 로고와 'Biome - Toolchain of the web'이라는 문구가 들어 있는 Biome의 배너를 표시합니다." src="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg" width="700">
    </picture>
</p>

<div align="center">

[![CI on main][ci-badge]][ci-url]
[![Discord chat][discord-badge]][discord-url]
[![npm version][npm-badge]][npm-url]
[![VSCode version][vscode-badge]][vscode-url]
[![Open VSX version][open-vsx-badge]][open-vsx-url]
[![Polar bounties][polar-badge]][polar-url]
  
[ci-badge]: https://github.com/biomejs/biome/actions/workflows/main.yml/badge.svg
[ci-url]: https://github.com/biomejs/biome/actions/workflows/main.yml
[discord-badge]: https://badgen.net/discord/online-members/BypW39g6Yc?icon=discord&label=discord&color=60a5fa
[discord-url]: https://discord.gg/BypW39g6Yc
[npm-badge]: https://badgen.net/npm/v/@biomejs/biome?icon=npm&color=60a5fa&label=%40biomejs%2Fbiome
[npm-url]: https://www.npmjs.com/package/@biomejs/biome/v/latest
[vscode-badge]: https://badgen.net/vs-marketplace/v/biomejs.biome?label=vscode&icon=visualstudio&color=60a5fa
[vscode-url]: https://marketplace.visualstudio.com/items?itemName=biomejs.biome
[open-vsx-badge]: https://badgen.net/open-vsx/version/biomejs/biome?label=open-vsx&color=60a5fa
[open-vsx-url]: https://open-vsx.org/extension/biomejs/biome
[polar-badge]: https://polar.sh/embed/seeks-funding-shield.svg?org=biomejs
[polar-url]: https://polar.sh/biomejs

</div>

<!-- Insert new entries lexicographically by language code.
     For example given below is the same order as these files appear on page:
     https://github.com/biomejs/biome/tree/main/packages/%40biomejs/biome -->
<div align="center">

[हिन्दी](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.hi.md) | [English](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.md) | [繁體中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-TW.md) | [简体中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-CN.md) | [日本語](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.ja.md) | [Português do Brasil](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.pt-br.md) | 한국어

</div>


**Biome**은 웹 프로젝트를 위한 고성능 툴체인으로, 프로젝트를 건전하게 유지하기 위한 개발자 도구를 제공하는 것을 목표로 합니다.

**Biome**은 _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_ 및 _GraphQL_을 위한 **[고속 포매터](./benchmark#formatting)**로, **[_Prettier_와의 호환성 97%](https://console.algora.io/challenges/prettier)**를 달성하고 있습니다.

**Biome**은 _JavaScript_, _TypeScript_, _JSX_, _CSS_ 및 _GraphQL_을 위한 **[고성능 린터](https://github.com/biomejs/biome/tree/main/benchmark#linting)**로, ESLint, typescript-eslint 및 [기타 리소스](https://github.com/biomejs/biome/discussions/3)에서 **[270개 이상의 규칙](https://biomejs.dev/linter/rules/)**을 제공합니다. 코드를 개선하고, 더 나은 프로그래머가 되는 데 도움이 되는 **상세하고 문맥에 맞는 진단을 출력**합니다!

**Biome**은 처음부터 [**에디터 내에서 인터랙티브하게**](https://biomejs.dev/guides/integrate-in-editor/) 사용하도록 설계되었습니다. 작성하는 동안 잘못된 코드를 포매팅하고 린팅할 수 있습니다.

### 설치

```shell
npm install --save-dev --save-exact @biomejs/biome
```

### 사용법

```shell
# 파일의 format을 체크
npx @biomejs/biome format --write ./src

# 파일을 린팅하고 안전한 수정 사항 적용
npx @biomejs/biome lint --write ./src

# 포매팅, 린팅 등을 실행하고 안전한 수정 사항 적용
npx @biomejs/biome check --write ./src

# CI 환경에서 모든 파일에 대해 포매팅, 린팅 등을 확인
npx @biomejs/biome ci ./src
```

Biome을 설치하지 않고 실행해보고 싶다면 WebAssembly로 컴파일된 [온라인 플레이그라운드](https://biomejs.dev/playground/)를 사용하세요.

## 문서

Biome에 대해 자세히 알아보려면 [홈페이지][biomejs]를 확인하거나 [시작하기][getting-started] 가이드로 바로 이동하여 Biome을 시작하세요.

## Biome에 대해 자세히 알아보기

**Biome**은 기본값이 이성적으로 설정되어 있어 다른 설정이 필요하지 않습니다.

**Biome**은 최신 웹 개발의 [모든 주요 언어][language-support]를 지원하는 것을 목표로 합니다.

**Biome**은 작동하는 데 [Node.js가 필요하지 않습니다.](https://biomejs.dev/guides/manual-installation/)

**Biome**은 소스 텍스트를 충실하게 표현하는 정교한 구문 분석기와 최고 수준의 오류 복구 기능을 갖춘 최고 수준의 LSP를 지원합니다.

**Biome**은 지금까지 개별 도구로 사용되던 기능을 통합합니다. 공통된 기반을 구축함으로써 코드 처리, 오류 표시, 병렬 작업, 캐싱 및 구성에 대한 일관된 경험을 제공합니다.

더 많은 정보는 [프로젝트 철학][biome-philosophy]에서 확인할 수 있습니다.

**Biome**은 [MIT 라이선스](https://github.com/biomejs/biome/tree/main/LICENSE-MIT) 또는 [Apache 2.0 라이선스](https://github.com/biomejs/biome/tree/main/LICENSE-APACHE)이며 [기여자 행동 규범](https://github.com/biomejs/biome/tree/main/CODE_OF_CONDUCT.md)에 따라 관리되고 있습니다.

## 펀딩

다양한 방법으로 프로젝트를 지원할 수 있습니다.

### 프로젝트 스폰서십 및 펀딩

[Open Collective](https://opencollective.com/biome) 또는 [GitHub Sponsors](https://github.com/sponsors/biomejs)를 통해 프로젝트를 지원할 수 있습니다.

Biome은 기업이 다양한 개발자들 사이에서 인지도를 얻을 수 있는 간단한 스폰서십 프로그램을 제공합니다.

### 이슈 펀딩

우리는 투표와 여러분들이 원하는 신기능 추진을 위해 [Polar.sh](https://polar.sh/biomejs)을 사용하고 있습니다. 백로그를 체크하고 지원해주세요!

<a href="https://polar.sh/biomejs"><img src="https://polar.sh/embed/fund-our-backlog.svg?org=biomejs" /></a>

## 후원

### 골드 스폰서

### 실버 스폰서

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://l2beat.com/" target="_blank"><img src="https://images.opencollective.com/l2beat/c2b2a27/logo/256.png" height="100"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://www.phoenixlabs.dev/" target="_blank"><img src="https://images.opencollective.com/phoenix-labs/2824ed4/logo/100.png?height=100" height="100"></a>
      </td>
    </tr>
  </tbody>
</table>

### 브론즈 스폰서

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://www.kanamekey.com" target="_blank"><img src="https://images.opencollective.com/kaname/d15fd98/logo/256.png?height=80" width="80"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://nanabit.dev/" target="_blank"><img src="https://images.opencollective.com/nanabit/d15fd98/logo/256.png?height=80" width="80"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://vital.io/" target="_blank"><img src="https://avatars.githubusercontent.com/u/25357309?s=200" width="80"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://coderabbit.ai/" target="_blank"><img src="https://avatars.githubusercontent.com/u/132028505?s=200&v=4" width="80"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://forge42.dev/" target="_blank"><img src="https://avatars.githubusercontent.com/u/161314831?s=200&v=4" width="80"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://transloadit.com/" target="_blank"><img src="https://avatars.githubusercontent.com/u/125754?s=200&v=4" width="80"></a>
      </td>
    </tr>
  </tbody>
</table>

[biomejs]: https://biomejs.dev/
[biome-philosophy]: https://biomejs.dev/internals/philosophy/
[language-support]: https://biomejs.dev/internals/language-support/
[getting-started]: https://biomejs.dev/guides/getting-started/
