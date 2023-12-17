```README.md
<p align="center">
    <img alt="Biome - Rangkaian Alat Web"
         src="https://raw.githubusercontent.com/biomejs/resources/main/biome-logo-slogan.svg"
         width="400">
</p>

<div align="center">

[![Obrolan Discord][discord-badge]][discord-url]
[![CI pada utama][ci-badge]][ci-url]
[![versi npm][npm-badge]][npm-url]
[![versi VSCode][vscode-badge]][vscode-url]
[![versi Open VSX][open-vsx-badge]][open-vsx-url]

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

**Biome** adalah rangkaian alat yang berkinerja tinggi untuk proyek web, yang bertujuan untuk menyediakan alat pengembang untuk memelihara kesehatan proyek tersebut.

**Biome adalah [pemformat cepat](./benchmark#formatting)** untuk _JavaScript_, _TypeScript_, _JSX_, dan _JSON_ yang mencapai **[kompatibilitas 96% dengan _Prettier_](https://console.algora.io/challenges/prettier)**.

**Biome adalah [linter yang berkinerja tinggi](https://github.com/biomejs/biome/tree/main/benchmark#linting)** untuk _JavaScript_, _TypeScript_, dan _JSX_ yang menampilkan **[lebih dari 170 aturan](https://biomejs.dev/linter/rules/)** dari ESLint, TypeScript ESLint, dan [sumber lainnya](https://github.com/biomejs/biome/discussions/3).
Ini **mengeluarkan diagnostik yang terperinci dan berkonteks** yang membantu Anda meningkatkan kode dan menjadi programmer yang lebih baik!

**Biome** dirancang dari awal untuk digunakan [secara interaktif dalam editor](https://biomejs.dev/guides/integrate-in-editor/).
Dapat memformat dan melintas kode yang salah bentuk saat Anda menulisnya.

### Instalasi

```shell
npm install --save-dev --save-exact @biomejs/biome
```

### Penggunaan

```shell
# format berkas
npx @biomejs/biome format --write ./src

# lint berkas
npx @biomejs/biome lint ./src

# jalankan format, lint, dll. dan terapkan saran aman
npx @biomejs/biome check --apply ./src

# periksa semua berkas terhadap format, lint, dll. di lingkungan CI
npx @biomejs/biome ci ./src
```

Jika Anda ingin mencoba Biome tanpa menginstalnya, gunakan [taman bermain daring](https://biomejs.dev/playground/), yang dikompilasikan ke WebAssembly.

## Dokumentasi

Lihat [halaman utama kami][biomejs] untuk mempelajari lebih lanjut tentang Biome,
atau langsung ke [Panduan Memulai][getting-started] untuk mulai menggunakan Biome.

## Lebih Lanjut tentang Biome

**Biome** memiliki standar default yang masuk akal dan tidak memerlukan konfigurasi.

**Biome** bertujuan untuk mendukung [semua bahasa utama][language-support] pengembangan web modern.

**Biome** [tidak mem

erlukan Node.js](<https://biomejs.dev/guides/manual-installation/>) untuk berfungsi.

**Biome** memiliki dukungan LSP kelas satu, dengan parser canggih yang merepresentasikan teks sumber secara penuh dan pemulihan kesalahan yang sangat baik.

**Biome** menyatukan fungsionalitas yang sebelumnya merupakan alat terpisah. Berdasarkan dasar bersama memungkinkan kami untuk menyediakan pengalaman yang kohesif untuk memproses kode, menampilkan kesalahan, memparalelkan pekerjaan, caching, dan konfigurasi.

Baca lebih lanjut tentang [filosofi proyek kami][biome-philosophy].

**Biome** dilisensikan [MIT](https://github.com/biomejs/biome/tree/main/LICENSE-MIT) atau [Apache 2.0](https://github.com/biomejs/biome/tree/main/LICENSE-APACHE) dan dimoderasi di bawah [Kode Etik Kontributor](https://github.com/biomejs/biome/tree/main/CODE_OF_CONDUCT.md).

## Sponsor

### Sponsor Emas

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://shiguredo.jp/" target="_blank"><img src="https://shiguredo.jp/official_shiguredo_logo.svg" height="120"></a>
      </td>
    </tr>
  </tbody>
</table>

### Sponsor Perunggu

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

## Terjemahan

- [English](./README.md)
- [简体中文](./README.zh-CN.md)
- [日本語](./README.ja.md)
- Bahasa Indonesia

[biomejs]: https://biomejs.dev/id/
[biome-philosophy]: https://biomejs.dev/id/internals/philosophy/
[language-support]: https://biomejs.dev/id/internals/language-support/
[getting-started]: https://biomejs.dev/id/guides/getting-started/
