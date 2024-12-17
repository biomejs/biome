<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-dark-transparent.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg">
    <img alt="Montre la bannière de Biome, avec son logo et la phrase 'Biome - Toolchain of the web'." src="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg" width="700">
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

  [हिन्दी](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.hi.md) | [English](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.md) | Français | [繁體中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-TW.md) | [简体中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-CN.md) | [日本語](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.ja.md) | [Português do Brasil](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.pt-BR.md) | [한국어](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.kr.md) | [Русский](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.ru.md) | [Українська](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.uk.md)
</div>

<br>

**Biome** fournit un ensemble d’outils performants conçus pour maintenir des projets web.

**Biome est un [outil de formatage rapide](./benchmark#formatting)** pour _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_ et _GraphQL_ qui atteint **[97 % de compatibilité avec _Prettier_](https://console.algora.io/challenges/prettier)**.

**Biome est un [outil de linting performant](https://github.com/biomejs/biome/tree/main/benchmark#linting)** pour _JavaScript_, _TypeScript_, _JSX_, _CSS_ et _GraphQL_ qui comprend **[plus de 270 règles](https://biomejs.dev/fr/linter/rules/)** d’ESLint, de typescript-eslint, et [d’autres sources](https://github.com/biomejs/biome/discussions/3).
Il **fournit des diagnostics détaillés et contextualisés** qui vous aident à améliorer votre code et à devenir un meilleur programmeur !

**Biome** est conçu dès le départ pour être utilisé [dans un éditeur de manière interactive](https://biomejs.dev/fr/guides/integrate-in-editor/).
Il peut formater et analyser du code malformé pendant que vous l’écrivez.

### Installation

```shell
npm install --save-dev --save-exact @biomejs/biome
```

### Utilisation

```shell
# formater les fichiers
npx @biomejs/biome format --write ./src

# linter les fichiers et appliquer les corrections sûres
npx @biomejs/biome lint --write ./src

# exécuter le formatage, le linting, etc. et appliquer les corrections sûres
npx @biomejs/biome check --write ./src

# vérifier tous les fichiers par rapport au formatage, au linting, etc. dans les environnements d’intégration continue
npx @biomejs/biome ci ./src
```

Si vous voulez essayer Biome sans l’installer, utilisez le [bac à sable en ligne](https://biomejs.dev/playground/), compilé en WebAssembly.

## Documentation

Consultez notre [page d’accueil][biomejs] pour en savoir plus sur Biome
ou allez directement sur le [guide de démarrage][getting-started] pour commencer à utiliser Biome.

## Plus sur Biome

**Biome** a des réglages par défaut robustes et ne demande pas de configuration.

**Biome** vise à prendre en charge [les principaux langages][language-support] du développement web moderne.

**Biome** [ne requiert pas Node.js](https://biomejs.dev/fr/guides/manual-installation/) pour fonctionnner.

**Biome** a une prise en charge du LSP de premier ordre, avec un analyseur sophistiqué qui représente le code source en toute fidélité et une excellente récupération des erreurs.

**Biome** unifie des fonctionnalités précédemment disponibles sous forme d’outils séparés. Partir d’une base partagée nous permet de fournir une expérience cohérente pour traiter le code, afficher les erreurs, paralléliser le travail, la mise en cache et la configuration.

En savoir plus sur la [philosophie de notre projet][biome-philosophy].

**Biome** est sous [licence MIT](https://github.com/biomejs/biome/tree/main/LICENSE-MIT) ou [licence Apache 2.0](https://github.com/biomejs/biome/tree/main/LICENSE-APACHE) et modéré selon les termes du [Code de conduite des contributeurs](https://github.com/biomejs/biome/tree/main/CODE_OF_CONDUCT.md).

## Financement

Vous pouvez financer le projet de différentes manières.

### Sponsoring et financement du projet

Vous pouvez sponsoriser ou financer le projet via [Open collective](https://opencollective.com/biome) ou [GitHub sponsors](https://github.com/sponsors/biomejs).

Biome offre un programme de sponsoring simple qui permet aux entreprises d’obtenir de la visibilité et la reconnaissance de divers développeurs.

### Financement d’un problème

Nous utilisons [Polar.sh](https://polar.sh/biomejs) pour voter pour des fonctionnalités spécifiques que vous aimeriez voir implémentées et pour les promouvoir. Jetez un coup d'œil aux fonctionnalités demandées et aidez-nous à les financer.

## Sponsors

### Sponsors Or

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


### Sponsors Argent

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

### Sponsors Bronze

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

[biomejs]: https://biomejs.dev/fr/
[biome-philosophy]: https://biomejs.dev/fr/internals/philosophy/
[language-support]: https://biomejs.dev/fr/internals/language-support/
[getting-started]: https://biomejs.dev/fr/guides/getting-started/
