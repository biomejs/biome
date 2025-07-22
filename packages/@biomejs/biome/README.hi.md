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

  हिन्दी | [English](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.md) | [Français](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.fr.md) | [繁體中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-TW.md) | [简体中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-CN.md) |  [日本語](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.ja.md) | [Português do Brasil](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.pt-BR.md) | [한국어](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.kr.md) | [Русский](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.ru.md) | [Українська](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.uk.md)
</div>

<br>

**Biome** वेब[^1] परियोजना[^2]ओं के लिए एक प्रदर्शनकारी उपकरण-श्रृंखला[^3] है, इसका उद्देश्य उक्त परियोजना[^2]ओं के स्वास्थ्य को बनाए रखने के लिए डेवलपर[^4] उपकरण प्रदान करना है।

**Biome** *JavaScript*, *TypeScript*, *JSX* और *JSON* के लिए **एक [तेज़ स्वरूपक](./benchmark#formatting)[^5]** है जो **[*Prettier* के साथ ९७% अनुकूलता](https://console.algora.io/challenges/prettier)[^6]** स्कोर[^7] करता है।

**Biome *JavaScript*, *TypeScript* और *JSX* के लिए एक [प्रदर्शनकारी लिंटर](https://github.com/biomejs/biome/tree/main/benchmark#linting)[^8]** है जिसमें ESLint, typescript-eslint और [अन्य स्रोतों](https://github.com/biomejs/biome/discussions/3) से **[२७० से अधिक नियम](https://biomejs.dev/linter/rules/)** शामिल हैं। यह **विस्तृत[^9] और संदर्भिकृत[^10] निदान[^11]** आउटपुट[^12] करता है जो आपको अपना कोड[^13] बेहतर बनाने और एक बेहतर प्रोग्रामर[^14] बनने में मदद करता है!

**Biome** को शुरू से ही [संपादक](https://biomejs.dev/guides/integrate-in-editor/)[^15] [के भीतर अंतरक्रियात्मक](https://biomejs.dev/guides/integrate-in-editor/)[^16] [रूप से](https://biomejs.dev/guides/editors/first-party-extensions/) उपयोग करने के लिए डिज़ाइन[^17] किया गया है। यह आपके द्वारा लिखे जा रहे विकृत[^18] कोड[^13] को स्वरूप[^5] और लिंट[^8] कर सकता है।

### स्थापना[^19]

```shell
npm install --save-dev --save-exact @biomejs/biome
```

### प्रयोग[^20]

* फ़ाइलें[^21] स्वरूप[^5] करें

  ```shell
  npx @biomejs/biome format --write ./src
  ```

* फ़ाइलें[^21] लिंट[^8] करें

  ```shell
  npx @biomejs/biome lint ./src
  ```

* स्वरूप, लिंट आदि चलाएँ और सुरक्षित सुझाव लागू करें

  ```shell
  npx @biomejs/biome check --write ./src
  ```

* CI वातावरण में सभी फ़ाइलों को स्वरूप, लिंट आदि के विरुद्ध जाँचें

  ```shell
  npx @biomejs/biome ci ./src
  ```

यदि आप Biome को स्थापित[^19] किए बिना चलाना चाहते हैं, तो WebAssembly में संकलित[^22] [ऑनलाइन](https://biomejs.dev/playground/)[^23] [प्रयोगशाला](https://biomejs.dev/playground/)[^24] का उपयोग करें।

## दस्तावेज़ीकरण[^25]

Biome के बारे में अधिक जानने के लिए हमारे [मुखपृष्ठ][biomejs] पर जाएँ, या Biome का उपयोग शुरू करने के लिए सीधे [आरंभ करने की मार्गदर्शिका][आरंभ-करें][^26] पर जाएँ।

## Biome के बारे में और

**Biome** में उचित पूर्व-निर्धारन[^27] हैं और इसके लिए कॉन्फ़िगरेशन[^28] की आवश्यकता नहीं है।

**Biome** का लक्ष्य आधुनिक वेब[^1] विकास की [सभी मुख्य भाषाओं][भाषा-समर्थन] का समर्थन करना है।

**Biome** को कार्य करने के लिए [Node.js की आवश्यकता नहीं है।](https://biomejs.dev/guides/manual-installation/)

**Biome** में प्रथम-श्रेणी का LSP समर्थन है, जिसमें एक परिष्कृत[^29] पार्सर[^30] है जो स्रोत पाठ[^31] को पूर्ण निष्ठा और शीर्ष-स्तरीय त्रुटि[^32] पुनर्प्राप्ति[^33] में प्रस्तुत करता है।

**Biome** उन कार्यक्षमता[^34]ओं को एकीकृत करता है जो पहले अलग-अलग उपकरण थे। साझा आधार पर निर्माण करने से हमें कोड[^13] प्रोसेसिंग, त्रुटि[^32]यों को प्रदर्शित करने, समानांतर कार्य, कैशिंग[^35] और कॉन्फ़िगरेशन[^28] के लिए एक सुसंगत अनुभव प्रदान करने की अनुमति मिलती है।

हमारे [परियोजना दर्शनशास्र][biome-दर्शनशास्र] के बारे में और पढ़ें।

**Biome** [MIT लाइसेंस प्राप्त](https://github.com/biomejs/biome/tree/main/LICENSE-MIT) या [Apache 2.0 लाइसेंस प्राप्त](https://github.com/biomejs/biome/tree/main/LICENSE-APACHE) है और [योगदानकर्ता अनुबंध आचार संहिता](https://github.com/biomejs/biome/tree/main/CODE_OF_CONDUCT.md) के तहत संचालित है।

## वित्तपोषण[^36]

आप परियोजना[^2] को विभिन्न तरीकों से वित्तपोषित[^36] कर सकते हैं।

### परियोजना[^2] प्रायोजन[^37] और वित्तपोषण[^36]

आप [Open Collective](https://opencollective.com/biome) या [GitHub Sponsors](https://github.com/sponsors/biomejs) के माध्यम से परियोजना[^2] को प्रायोजित[^37] या वित्तपोषित[^36] कर सकते हैं।

Biome एक सरल प्रायोजन[^37] कार्यक्रम प्रदान करता है जो कंपनियों को विभिन्न डेवलपरों[^4] के बीच दृश्यता[^38] और मान्यता प्राप्त करने की अनुमति देता है।

### वित्तपोषण[^36] जारी करना

हम [Polar.sh](https://polar.sh/biomejs) का उपयोग उन विशिष्ट[^39] सुविधाओं के पक्ष में वोट[^40] करने और बढ़ावा देने के लिए करते हैं जिन्हें आप देखना और लागू करना चाहते हैं। हमारे बकाया कार्य[^41] की जाँच करें और हमारी मदद करें:

[![वित्तपोषण जारी करें](https://polar.sh/embed/fund-our-backlog.svg?org=biomejs)](https://polar.sh/biomejs/)

## प्रायोजक[^37]

### स्वर्ण प्रायोजक[^42]

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

### रजत प्रायोजक[^43]

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

### कांस्य प्रायोजक[^44]

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
[biome-दर्शनशास्र]: https://biomejs.dev/internals/philosophy/
[भाषा-समर्थन]: https://biomejs.dev/internals/language-support/
[आरंभ-करें]: https://biomejs.dev/guides/getting-started/

## शब्द सूची

नीचे दिए गए तिरछे शब्द आगत शब्द हैं।

[^1]: *वेब* - web: the internet
[^2]: परियोजना - project
[^3]: उपकरण-श्रृंखला - toolchain
[^4]: *डेव/डेवलपर* - dev/developer
[^5]: स्वरूप/स्वरूपक - format/foramtter
[^6]: अनुकूल/अनुकूलता - compatible/compatibility
[^7]: *स्कोर* - score
[^8]: *लिंट/लिंटर* - lint/linter
[^9]: विस्तार/विस्तृत - detail/detailed
[^10]: संदर्भ/संदर्भिकृत - context/contextualized
[^11]: निदान - diagnosis
[^12]: *आउटपुट* - output
[^13]: *कोड* - code
[^14]: *प्रोग्रामर* - programmer
[^15]: संपादक - editor, the text editor: vscode, zed, etc.
[^16]: अंतरक्रिया/अंतरक्रियात्मक - interact/interactive
[^17]: *डिज़ाइन* - design
[^18]: विकृत - malformed
[^19]: स्थापित_करना/स्थापना - install/installation
[^20]: प्रयोग - usage
[^21]: *फ़ाइल* - file
[^22]: संकलित_करना/संकलित/संकलनकर्ता - compile/compiled/compiler
[^23]: *ऑनलाइन* - online
[^24]: प्रयोगशाला - laboratory
[^25]: दस्तावेज़/दस्तावेज़ीकरण - document/documentation
[^26]: मार्गदर्शिका - guide
[^27]: पूर्व-निर्धारित - default
[^28]: *कॉन्फ़िग/कॉन्फ़िगर/कॉन्फ़िगरेशन* - config/configure/configuration
[^29]: परिष्कृत - sophisticated
[^30]: *पार्सर* - parser
[^31]: पाठ - text
[^32]: त्रुटि - error
[^33]: पुनर्प्राप्ति - recovery
[^34]: कार्यक्षमता - functionality
[^35]: *कैश/कैशिंग* - cache/caching, ~~cash/cashing~~
[^36]: वित्तपोषित_करना/वित्तपोषण - fund/funding
[^37]: प्रायोजित/प्रायोजन - sponsor/sponsorship
[^38]: दृश्यता - visibility
[^39]: विशिष्ट - specific
[^40]: *वोट* - vote
[^41]: बकाया_कार्य - backlog
[^42]: स्वर्ण_प्रायोजक - Gold Sponsor
[^43]: रजत_प्रायोजक - Silver Sponsor
[^44]: कांस्य_प्रायोजक - Bronze Sponsor
