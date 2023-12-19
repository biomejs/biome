<p align="center">
    <img alt="Biome - Web'in alet takımı"
         src="https://raw.githubusercontent.com/biomejs/resources/main/biome-logo-slogan.svg"
         width="400">
</p>

<div align="center">

[![Discord sohbet][discord-badge]][discord-url]
[![Ana dalda CI][ci-badge]][ci-url]
[![npm sürümü][npm-badge]][npm-url]
[![VSCode sürümü][vscode-badge]][vscode-url]
[![Open VSX sürümü][open-vsx-badge]][open-vsx-url]

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

**Biome**, web projeleri için performanslı bir alet takımıdır, söz konusu projelerin sağlığını korumak için geliştirici araçları sağlamayı amaçlar.

**Biome, _JavaScript_, _TypeScript_, _JSX_ ve _JSON_ için [hızlı bir formatlayıcıdır](./benchmark#formatting)** ve **[_Prettier_ ile %96 uyumludur](https://console.algora.io/challenges/prettier)**.

**Biome, _JavaScript_, _TypeScript_ ve _JSX_ için [performanslı bir derleyicidir](https://github.com/biomejs/biome/tree/main/benchmark#linting)** ve ESLint, TypeScript ESLint ve [diğer kaynaklardan](https://github.com/biomejs/biome/discussions/3) **170'den fazla kural** içerir.
Kodunuzu geliştirmenize ve daha iyi bir programcı olmanıza yardımcı olacak **ayrıntılı ve bağlamsal tanılar sunar**!

**Biome**, [bir editör içinde etkileşimli olarak kullanılmak](https://biomejs.dev/guides/integrate-in-editor/) üzere baştan tasarlanmıştır.
Yazarken hatalı kodu formatlayabilir ve lintleyebilirsiniz.

### Kurulum

```shell
npm install --save-dev --save-exact @biomejs/biome
```

### Kullanım

```shell
# dosyaları formatla
npx @biomejs/biome format --write ./src

# dosyaları lintle
npx @biomejs/biome lint ./src

# format, lint vb. çalıştır ve güvenli önerileri uygula
npx @biomejs/biome check --apply ./src

# CI ortamlarında tüm dosyaları format, lint vb. ile kontrol et
npx @biomejs/biome ci ./src
```

Biome'u yüklemek istemeden denemek istiyorsanız, WebAssembly'ye derlenmiş [oyun alanını](https://biomejs.dev/playground/) kullanabilirsiniz.

## Dokümantasyon

Biome hakkında daha fazla bilgi edinmek için [ana sayfamızı][biomejs] ziyaret edin,
veya Biome'u kullanmaya başlamak için doğrudan [Başlangıç Rehberi'ne][getting-started] gidin.

## Biome Hakkında Daha Fazlası

**Biome**, sağlam varsayılanlara sahiptir ve yapılandırma gerektirmez.

**Biome**, modern web geliştirmenin [tüm dillerini][language-support] desteklemeyi amaçlar.

**Biome**'un işlev görmesi için [Node.js gerektirmez](https://biomejs.dev/guides/manual-installation/).

**Biome**, birinci sınıf LSP desteği ile, kaynak metni tam sadakatle temsil eden ve üst düzey hata iyileştirmeye sahip sofistike bir ayrıştırıcıya sahiptir.

**Biome**, daha önce ayrı araçlar olmuş işlevselliği birleştirir. Ortak bir temel üzerine inşa ederek, kod işleme, hata gösterme, işi paralelleştirme, önbellekleme ve yapılandırma için uyumlu bir deneyim sunmamızı sağlar.

[Projemizin felsefesi][biome-philosophy] hakkında daha fazlasını okuyun.

**Biome**, [MIT lisansı](https://github.com/biomejs/biome/tree/main/LICENSE-MIT) altındadır veya [Apache 2.0 lisansı](https://github.com/biomejs/biome/tree/main/LICENSE-APACHE) altındadır ve [Katılımcı Sözleşmesi Davranış Kuralları](https://github.com/biomejs/biome/tree/main/CODE_OF_CONDUCT.md) altında yönetilir.

## Sponsorlar

### Altın Sponsorlar

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://shiguredo.jp/" target="_blank"><img src="https://shiguredo.jp/official_shiguredo_logo.svg" height="120"></a>
      </td>
    </tr>
  </tbody>
</table>

### Bronz Sponsorlar

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

## Çeviriler

- [English](./README.md)
- [简体中文](./README.zh-CN.md)
- [日本語](./README.ja.md)
- Türkçe

[biomejs]: https://biomejs.dev/tr/
[biome-philosophy]: https://biomejs.dev/tr/internals/philosophy/
[language-support]: https://biomejs.dev/tr/internals/language-support/
[getting-started]: https://biomejs.dev/tr/guides/getting-started/
