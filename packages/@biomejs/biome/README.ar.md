<p align="center">
    <img alt="Biome - أدوات الويب"
         src="https://raw.githubusercontent.com/biomejs/resources/main/biome-logo-slogan.svg"
         width="400">
</p>

<div align="center">

[![Discord chat][discord-badge]][discord-url]
[![CI on main][ci-badge]][ci-url]
[![نسخة npm][npm-badge]][npm-url]
[![نسخة VSCode][vscode-badge]][vscode-url]
[![نسخة Open VSX][open-vsx-badge]][open-vsx-url]

[discord-badge]: https://badgen.net/discord/online-members/BypW39g6Yc?icon=discord&label=discord&color=green
[discord-url]: https://discord.gg/BypW39g6Yc
[ci-badge]: https://github.com/biomejs/biome/actions/workflows/main.yml/badge.svg
[ci-url]: https://github.com/biomejs/biome/actions/workflows/main.yml
[npm-badge]: https://badgen.net/npm/v/@biomejs/biome?icon=npm&color=green&label=%40biomejs%2Fbiome
[npm-url]: https://www.npmjs.com/package/@biomejs/biome/v/latest
[vscode-badge]: https://badgen.net/vs-marketplace/v/biomejs.biome?label=vscode&icon=visualstudio&color=green
[vscode-url]: https://marketplace.visualstudio.com/items?itemName=biomejs.biome
[open-vsx-badge]: https://badgen.net/open-vsx/version/biomejs/biome?label=open-vsx&color=green
[open-vsx-url]: https://open-vsx.org/extension/biomejs/biome

</div>

**Biome** هو أداة متطورة لمشاريع الويب، وتهدف إلى توفير أدوات للمطورين للحفاظ على صحة هذه المشاريع.

**Biome هو [منسق سريع](./benchmark#formatting)** لـ _JavaScript_، _TypeScript_، _JSX_، و _JSON_ ويحقق **[توافق بنسبة 96% مع _Prettier_](https://console.algora.io/challenges/prettier)**.

**Biome هو [مدقق أداء](https://github.com/biomejs/biome/tree/main/benchmark#linting)** لـ _JavaScript_، _TypeScript_، و _JSX_ يتضمن **[أكثر من 170 قاعدة](https://biomejs.dev/linter/rules/)** من ESLint، TypeScript ESLint، و [مصادر أخرى](https://github.com/biomejs/biome/discussions/3).
يُخرج تشخيصات مفصلة وموضوعية تساعدك على تحسين كودك وتصبح مبرمجًا أفضل!

**Biome** مصمم من البداية ليُستخدم [بشكل تفاعلي ضمن المحرر](https://biomejs.dev/guides/integrate-in-editor/).
يمكنه تنسيق وفحص الكود المشوه أثناء كتابته.

### التثبيت

```shell
npm install --save-dev --save-exact @biomejs/biome
```

### الاستخدام

```shell
# تنسيق الملفات
npx @biomejs/biome format --write ./src

# فحص الملفات
npx @biomejs/biome lint ./src

# تشغيل التنسيق، الفحص، إلخ. وتطبيق الاقتراحات الآمنة
npx @biomejs/biome check --apply ./src

# التحقق من جميع الملفات ضد التنسيق، الفحص، إلخ. في بيئات التكامل المستمر
npx @biomejs/biome ci ./src


```

إذا أردت تجربة Biome دون تثبيته، استخدم [الملعب الإلكتروني](https://biomejs.dev/playground/)، المترجم إلى WebAssembly.

## الوثائق

راجع [الصفحة الرئيسية][biomejs] لمعرفة المزيد عن Biome،
أو انتقل مباشرةً إلى [دليل البدء][getting-started] لبدء استخدام Biome.

## المزيد عن Biome

**Biome** يأتي بإعدادات افتراضية منطقية ولا يتطلب تهيئة.

**Biome** يهدف لدعم [جميع لغات الويب الرئيسية][language-support] المعاصرة.

**Biome** [لا يتطلب Node.js](https://biomejs.dev/guides/manual-installation/) للعمل.

**Biome** يدعم LSP بشكل كامل، مع محلل متطور يمثل النص المصدري بدقة كاملة واستعادة خطأ ممتازة.

**Biome** يوحد وظائف كانت في السابق أدوات منفصلة. البناء على قاعدة مشتركة يسمح لنا بتوفير تجربة متماسكة لمعالجة الكود، عرض الأخطاء، توازي العمل، التخزين المؤقت، والتكوين.

اقرأ المزيد عن [فلسفة مشروعنا][biome-philosophy].

**Biome** [مرخص بموجب ترخيص MIT](https://github.com/biomejs/biome/tree/main/LICENSE-MIT) أو [ترخيص Apache 2.0](https://github.com/biomejs/biome/tree/main/LICENSE-APACHE) ومشرف عليه بموجب [ميثاق سلوك المساهم](https://github.com/biomejs/biome/tree/main/CODE_OF_CONDUCT.md).

## الرعاة

### الرعاة الذهبيون

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://shiguredo.jp/" target="_blank"><img src="https://shiguredo.jp/official_shiguredo_logo.svg" height="120"></a>
      </td>
    </tr>
  </tbody>
</table>

### الرعاة البرونزيون

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://www.kanamekey.com" target="_blank"><img src="https://images.opencollective.com/kaname/d15fd98/logo/256.png?height=80" width="80"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://nanabit.dev/" target="_blank"><img src="https://images.opencollective.com/nanabit/d15fd98/logo/256.png?height=80" width="80"></a>
      </td>
    </tr>
  </tbody>
</table>

## الترجمات

- [English](./README.md)
- [简体中文](./README.zh-CN.md)
- [日本語](./README.ja.md)
- العربية

[biomejs]: https://biomejs.dev/ar/
[biome-philosophy]: https://biomejs.dev/ar/internals/philosophy/
[language-support]: https://biomejs.dev/ar/internals/language-support/
[getting-started]: https://biomejs.dev/ar/guides/getting-started/
