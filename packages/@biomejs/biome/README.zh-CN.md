<p align="center">
    <img alt="Biome - Toolchain of the web"
         src="https://raw.githubusercontent.com/biomejs/resources/main/biome-logo-slogan.svg"
         width="400">
</p>

<div align="center">

[![Discord chat][discord-badge]][discord-url]
[![CI on main][ci-badge]][ci-url]
[![npm version][npm-badge]][npm-url]
[![VSCode version][vscode-badge]][vscode-url]
[![Open VSX version][open-vsx-badge]][open-vsx-url]

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

**Biome** 是一个用于网络项目的高性能工具链，旨在为开发者提供维护这些项目的工具。

**Biome 是一个[快速的格式化器](./benchmark#formatting)**，适用于 _JavaScript_、_TypeScript_、_JSX_ 和 _JSON_，与 _Prettier_ 的兼容性达到了 **[96%](https://console.algora.io/challenges/prettier)**。

**Biome 是一个[高性能的 linter](https://github.com/biomejs/biome/tree/main/benchmark#linting)**，适用于 _JavaScript_、_TypeScript_ 和 _JSX_，包含了来自 ESLint、TypeSCript ESLint 和 [其他来源](https://github.com/biomejs/biome/discussions/3)的 **[超过 170 条规则](https://biomejs.dev/zh-cn/linter/rules/)**。
它**输出详细且有上下文的诊断信息**，帮助你改进代码，成为一个更好的程序员！

**Biome** 从一开始就设计为[在编辑器中交互式使用](https://biomejs.dev/zh-cn/guides/integrate-in-editor/)。
你在编写代码时，它可以格式化和 lint 不规范的代码。

### 安装

```shell
npm install --save-dev --save-exact @biomejs/biome
```

### 使用

```shell
# 格式化文件
npx @biomejs/biome format --write ./src

# lint 文件
npx @biomejs/biome lint ./src

# 运行格式化，lint 等，并应用安全的建议
npx @biomejs/biome check --apply ./src

# 在 CI 环境中检查所有文件是否符合格式，lint 等
npx @biomejs/biome ci ./src
```

如果你想在不安装的情况下试用 Biome，可以使用[在线 playground](https://biomejs.dev/playground/)，它被编译为 WebAssembly。

## 文档

查看我们的[主页][biomejs]以了解更多关于 Biome 的信息，
或者直接前往[入门指南][getting-started]开始使用 Biome。

## 更多关于 Biome

**Biome** 有合理的默认设置，不需要配置。

**Biome** 旨在支持[所有主要的现代网络开发语言][language-support]。

**Biome** [不需要 Node.js](https://biomejs.dev/zh-cn/guides/manual-installation/)就可以运行。

**Biome** 有一流的 LSP 支持，具有精密的解析器，可以完全保真地表示源文本，并具有顶级的错误恢复能力。

**Biome** 统一了以前分散的功能。基于共享的基础，我们可以提供一个处理代码、显示错误、并行工作、缓存和配置的一致体验。

阅读更多关于我们的[项目理念][biome-philosophy]。

**Biome** 采用 [MIT 许可](https://github.com/biomejs/biome/tree/main/LICENSE-MIT) 或 [Apache 2.0 许可](https://github.com/biomejs/biome/tree/main/LICENSE-APACHE)，并在 [贡献者公约行为准则](https://github.com/biomejs/biome/tree/main/CODE_OF_CONDUCT.md) 下进行管理。

## 赞助商

### 金牌赞助商

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://shiguredo.jp/" target="_blank"><img src="https://shiguredo.jp/official_shiguredo_logo.svg" height="120"></a>
      </td>
    </tr>
  </tbody>
</table>

### 铜牌赞助商

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://www.kanamekey.com" target="_blank"><img src="https://images.opencollective.com/kaname/d15fd98/logo/256.png?height=80" width="80"></a>
      </td>
    </tr>
  </tbody>
</table>

## 翻译

- [English](./README.md)
- 简体中文

[biomejs]: https://biomejs.dev/zh-cn/
[biome-philosophy]: https://biomejs.dev/zh-cn/internals/philosophy/
[language-support]: https://biomejs.dev/zh-cn/internals/language-support/
[getting-started]: https://biomejs.dev/zh-cn/guides/getting-started/
