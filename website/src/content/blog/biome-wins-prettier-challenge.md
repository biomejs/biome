---
title: Biome formatter wins the Prettier challenge
description: Biome formatter now 95% compatible with Prettier
summary: Biome formatter now 95% compatible with Prettier
authors:
  - ema
  - team
pubDate: 2023-11-27
coverImage:
  lightSrc: "@/assets/blog/prettier-challenge.png"
  darkSrc: "@/assets/blog/prettier-challenge.png"
  alt: The Prettier challenge banner, with the Biome logo over it
socialImage: "@/assets/blog/prettier-challenge.png"
---

With the release of Biome **`v1.4.0`**, we claim the bounty of the [Prettier challenge](https://console.algora.io/challenges/prettier)!

With `v1.4.0`, you'll get a better formatter experience, more formatting options, new VSCode features, new sponsors and more!

You can upgrade Biome by running the following command:

```bash
npm install --save-dev --save-exact @biomejs/biome@1.4.0
pnpm update --save-exact @biomejs/biome@1.4.0
yarn upgrade --exact @biomejs/biome@1.4.0
```

## Better formatter

Biome formatter has now **over 96% in terms of compatibility** against [Prettier](https://prettier.io/)! This score is computed for JavaScript, TypeScript, and JSX formatting.

Merit of challenge that was launched by [Christopher Chedeau](https://twitter.com/Vjeux/status/1722733472522142022), one of the Prettier's creators.

The challenge attracted the attention of many people, and some of them decided to contribute to Biome to claim part of the bounty. I did see something amazing: contributors had an amazing coordination, they took ownership of the tasks and delivered the solution in a matter of hours.

I believe the main factors that made this possible are three:

1. Money. It's a fact, and it's completely fine if someone decides to contribute only for earning a small stipend out of it.
2. Communication. We used [GitHub](https://github.com/biomejs/biome/issues/720) as only medium of coordination. We provided information, instructions and help on how to deliver.
3. Infrastructure. Biome relies on a solid testing infrastructure, built by [previous](https://github.com/MichaReiser) Rome Tools [employees](https://github.com/ematipico) and [contributors](https://github.com/IWANABETHATGUY/). It's able to catch every reformat bug, provide granular diffs and warn the user if the emitted output is the different from the one emitted by Prettier.

Before the challenge, Biome had roughly a compatibility rate of 85%, based on our internal metrics (JavaScript, TypeScript and JSX, on options parity). Even though 85% might seem high,
the impact of a low number such as 15% on big code bases is huge, and people might feel intimidated by so many changes, causing early adopters to receive frictions when bring Biome to their team. A member of our community shared some insights:

> As a great example of how much even just that last 5% has improved things for large codebases (and specifically with `bracketSpacing` and now `bracketSameLine` implemented) i ran it one project in our monorepo [...].
>
> Just last week, this number `[diagnostics]` was more than 6,000. Even with the bracket options ignored, it was still more than 1000, and now there are only 200 left!

Although the challenge is over, we are committed to improve even more the compatibility score with prettier. Any contribution in this regard is very welcome.

The challenge has also uncovered some cases in Prettier's emitted output that we decided to not follow. We have created a [new section](/formatter/#differences-with-prettier) in our website that explains them. Our hope is to make this section smaller with the time.

If there's a divergence that isn't documented in our website, you should consider that a bug and file an issue.

#### New formatting options

With this challenge, we added new options to the formatter:

- [`lineEnding`](/reference/configuration#formatterlineending)

  Use this option to match the line endings of your OS. We support `lf` (line feed - `\n`), `cr` (carriage return - `\r`) and `crlf` (carriage return line feed - `\r\n`).

- [`bracketSameLine`](/reference/configuration#formatterbracketsameline)

  ```jsx title="example.js"
  // Existing behavior. Now also the default, meaning `bracketSameLine: false`.
  <Foo
    className={somethingReallyLongThatForcesThisToWrap}
    anotherReallyLongAttribute={withAValueThatsSurelyTooLong}
    soThatEverythingWraps
  >
    Hello
  </Foo>

  <Foo
    selfClosingTags={likeThisOne}
    stillPlaceTheBracket={onItsOwnLine}
    toIndicateThat={itClosesItself}
  />
  ```

  After formatting with `"bracketSameLine": true`:

  ```jsx title="example.js"
  // New behavior, with `bracketSameLine: true`.
  <Foo
    className={somethingReallyLongThatForcesThisToWrap}
    anotherReallyLongAttribute={withAValueThatsSurelyTooLong}
    soThatEverythingWraps>
    Hello
  </Foo>

  <Foo
    selfClosingTags={likeThisOne}
    stillPlaceTheBracket={onItsOwnLine}
  />
  ```

- [`bracketSpacing`](/reference/configuration#formatterbracketspacing)

  ```js title="example.js"
  import { sort } from "sort.js";
  const value = { sort };
  ```

  After formatting with `"bracketSpacing": false`:

  ```js title="example.js"
  import {sort} from "sort.js";
  const value = {sort};
  ```

## VSCode extension goodies

The VSCode has been moved to a [new repository](https://github.com/biomejs/biome-vscode).

We removed the bundled binary from the extension, and you'll be able to download the version that you want. Here's a small video of how it works:

<video width="1200" height="800" controls>
  <source src="https://github.com/biomejs/biome-vscode/assets/649677/c7c1bf81-10a5-4cd6-bbdf-019d983a2d6a" type="video/mp4">
</video>

From today, we release a [**nightly**](https://marketplace.visualstudio.com/items?itemName=biomejs.biome-nightly) version of the extension. This is a version meant for early adopters and to test things before they are officially released.

## Some CLI goodies

People that rely on Biome LSP will be pleased that they can now pass a custom configuration to the command `lsp-proxy`, using the option `--config-path`. The same option is accepted by the command `start`:

```shell
biome --config-path=../path/where/config/is lsp-proxy
biome --config-path=../path/where/config/is start
```

The CLI now exposes the option `--diagnostic-level`, that allows to filter the kind of diagnostics printed to terminal.

```shell
biome check --diagnostic-level=error ./src
```

## New lint rules, and promoted rule

Biome is a linter too, and it features [177 rules](https://biomejs.dev/linter/rules/)! In this release, some rules are promoted and new ones are available.

### New rules

- [noDefaultExport](https://biomejs.dev/linter/rules/no-default-export)

  ```jsx
  export default function f() {}
  ```

  <pre class="language-text"><code class="language-text">nursery/noDefaultExport.js:1:8 <a href="https://biomejs.dev/linter/rules/no-default-export">lint/nursery/noDefaultExport</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid </span><span style="color: Orange;"><strong>default</strong></span><span style="color: Orange;"> exports.</span>

  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>export default function f() {};
     <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
      <strong>2 │ </strong>

  <strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Default exports cannot be easily discovered inside an editor and don't encourage the use of consistent names through a code base.</span>

  <strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use a named export instead.</span>

  </code></pre>

- [noAriaHiddenOnFocusable](https://biomejs.dev/linter/rules/no-aria-hidden-on-focusable)

  ```jsx
  <div aria-hidden="true" tabIndex="0" />
  ```

  <pre class="language-text"><code class="language-text">nursery/noAriaHiddenOnFocusable.js:1:1 <a href="https://biomejs.dev/linter/rules/no-aria-hidden-on-focusable">lint/nursery/noAriaHiddenOnFocusable</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Disallow </span><span style="color: Tomato;"><strong>aria-hidden=&quot;true&quot;</strong></span><span style="color: Tomato;"> from being set on focusable elements.</span>

  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div aria-hidden=&quot;true&quot; tabIndex=&quot;0&quot; /&gt;
     <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
      <strong>2 │ </strong>

  <strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;"><strong>aria-hidden</strong></span><span style="color: lightgreen;"> should not be set to </span><span style="color: lightgreen;"><strong>true</strong></span><span style="color: lightgreen;"> on focusable elements because this can lead to confusing behavior for screen reader users.</span>

  <strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the aria-hidden attribute from the element.</span>

  <strong>  </strong><strong>  1 │ </strong>&lt;div<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">i</span><span style="color: Tomato;">a</span><span style="color: Tomato;">-</span><span style="color: Tomato;">h</span><span style="color: Tomato;">i</span><span style="color: Tomato;">d</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">t</span><span style="color: Tomato;">r</span><span style="color: Tomato;">u</span><span style="color: Tomato;">e</span><span style="color: Tomato;">&quot;</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>tabIndex=&quot;0&quot;<span style="opacity: 0.8;">·</span>/&gt;
  <strong>  </strong><strong>    │ </strong>     <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>
  </code></pre>

- [noImplicitAnyLet](https://biomejs.dev/linter/rules/no-implicit-any-let)

  ```ts
  var a;
  a = 2;
  ```

  <pre class="language-text"><code class="language-text">nursery/noImplicitAnyLet.js:1:5 <a href="https://biomejs.dev/linter/rules/no-implicit-any-let">lint/nursery/noImplicitAnyLet</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This variable implicitly has the </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> type.</span>

  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var a;
     <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
      <strong>2 │ </strong>a = 2;
      <strong>3 │ </strong>

  <strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Variable declarations without type annotation and initialization have implicitly the </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;"> type. Declare type or initialize the variable with some value.</span>

  </code></pre>

- [useAwait](https://biomejs.dev/linter/rules/use-await)

  ```jsx
  async function fetchData() {
    // Missing `await` for the promise returned by `fetch`
    return fetch("/data");
  }
  ```

  <pre class="language-text"><code class="language-text">nursery/useAwait.js:1:1 <a href="https://biomejs.dev/linter/rules/use-await">lint/nursery/useAwait</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>async</strong></span><span style="color: Tomato;"> function lacks an </span><span style="color: Tomato;"><strong>await</strong></span><span style="color: Tomato;"> expression.</span>

  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>async function fetchData() {
     <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>// Missing `await` for the promise returned by `fetch`
  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>  return fetch('/data');
  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>}
     <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
      <strong>5 │ </strong>

  <strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Remove this </span><span style="color: lightgreen;"><strong>async</strong></span><span style="color: lightgreen;"> modifier, or add an </span><span style="color: lightgreen;"><strong>await</strong></span><span style="color: lightgreen;"> expression in the function.</span>

  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>async function fetchData() {
     <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>// Missing `await` for the promise returned by `fetch`
  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>  return fetch('/data');
  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>}
     <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
      <strong>5 │ </strong>

  <strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;"><strong>Async</strong></span><span style="color: lightgreen;"> functions without </span><span style="color: lightgreen;"><strong>await</strong></span><span style="color: lightgreen;"> expressions may not need to be declared </span><span style="color: lightgreen;"><strong>async</strong></span><span style="color: lightgreen;">.</span>

  </code></pre>

- [useValidAriaRole](https://biomejs.dev/linter/rules/use-valid-aria-role)

  ```jsx
  <div role="datepicker"></div>
  ```

  <pre class="language-text"><code class="language-text">nursery/useValidAriaRole.js:1:1 <a href="https://biomejs.dev/linter/rules/use-valid-aria-role">lint/nursery/useValidAriaRole</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Enforce that elements with ARIA roles must use a valid, non-abstract ARIA role.</span>

  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div role=&quot;datepicker&quot;&gt;&lt;/div&gt;
     <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
      <strong>2 │ </strong>

  <strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Check </span><span style="color: lightgreen;"><a href="https://www.w3.org/TR/wai-aria/#namefromauthor">WAI-ARIA</a></span><span style="color: lightgreen;"> for valid roles or provide options accordingly.</span>

  <strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the invalid </span><span style="color: lightgreen;"><strong>role</strong></span><span style="color: lightgreen;"> attribute.
  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;"> Check the list of all </span><span style="color: lightgreen;"><a href="https://www.w3.org/TR/wai-aria/#role_definitions">valid</a></span><span style="color: lightgreen;"> role attributes.</span>

  <strong>  </strong><strong>  1 │ </strong>&lt;div<span style="opacity: 0.8;">·</span><span style="color: Tomato;">r</span><span style="color: Tomato;">o</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">d</span><span style="color: Tomato;">a</span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">p</span><span style="color: Tomato;">i</span><span style="color: Tomato;">c</span><span style="color: Tomato;">k</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">&quot;</span>&gt;&lt;/div&gt;
  <strong>  </strong><strong>    │ </strong>     <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>
  </code></pre>

- [useRegexLiterals](https://biomejs.dev/linter/rules/use-regex-literals)

```jsx
new RegExp("abc", "u");
```

<pre class="language-text"><code class="language-text">nursery/useRegexLiterals.js:1:1 <a href="https://biomejs.dev/linter/rules/use-regex-literals">lint/nursery/useRegexLiterals</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Use a regular expression literal instead of the </span><span style="color: Orange;"><strong>RegExp</strong></span><span style="color: Orange;"> constructor.</span>

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>new RegExp(&quot;abc&quot;, &quot;u&quot;);
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>

<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Regular expression literals avoid some escaping required in a string literal, and are easier to analyze statically.</span>

<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use a </span><span style="color: lightgreen;"><strong>literal notation</strong></span><span style="color: lightgreen;"> instead.</span>

    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>w</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>R</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>g</strong></span><span style="color: Tomato;"><strong>E</strong></span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>(</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;">a</span><span style="color: Tomato;">b</span><span style="color: Tomato;">c</span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>,</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;">u</span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>)</strong></span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>/</strong></span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">b</span><span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;"><strong>/</strong></span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>

</code></pre>

### Recommended rules

- [a11y/noAccessKey](https://biomejs.dev/linter/rules/no-access-key)

  ```jsx
  <input type="submit" accessKey="s" value="Submit" />
  ```

  <pre class="language-text"><code class="language-text">a11y/noAccessKey.js:1:22 <a href="https://biomejs.dev/linter/rules/no-access-key">lint/a11y/noAccessKey</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>accessKey</strong></span><span style="color: Tomato;"> attribute to reduce inconsistencies between keyboard shortcuts and screen reader keyboard comments.</span>

  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;input type=&quot;submit&quot; accessKey=&quot;s&quot; value=&quot;Submit&quot; /&gt;
     <strong>   │ </strong>                     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
      <strong>2 │ </strong>

  <strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Assigning keyboard shortcuts using the </span><span style="color: lightgreen;"><strong>accessKey</strong></span><span style="color: lightgreen;"> attribute leads to inconsistent keyboard actions across applications.</span>

  <strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the </span><span style="color: lightgreen;"><strong>accessKey</strong></span><span style="color: lightgreen;"> attribute.</span>

  <strong>  </strong><strong>  1 │ </strong>&lt;input<span style="opacity: 0.8;">·</span>type=&quot;submit&quot;<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="color: Tomato;">c</span><span style="color: Tomato;">c</span><span style="color: Tomato;">e</span><span style="color: Tomato;">s</span><span style="color: Tomato;">s</span><span style="color: Tomato;">K</span><span style="color: Tomato;">e</span><span style="color: Tomato;">y</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">s</span><span style="color: Tomato;">&quot;</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>value=&quot;Submit&quot;<span style="opacity: 0.8;">·</span>/&gt;
  <strong>  </strong><strong>    │ </strong>                     <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>
  </code></pre>

- [a11y/useHeadingContent](https://biomejs.dev/linter/rules/use-heading-content)

  ```jsx
  <h1 />
  ```

  <pre class="language-text"><code class="language-text">a11y/useHeadingContent.js:1:1 <a href="https://biomejs.dev/linter/rules/use-heading-content">lint/a11y/useHeadingContent</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>heading</strong></span><span style="color: Tomato;">  elements.</span>

  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;h1 /&gt;
     <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
      <strong>2 │ </strong>

  <strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All headings on a page should have content that is accessible to screen readers.</span>

  </code></pre>

- [complexity/useSimpleNumberKeys](https://biomejs.dev/linter/rules/use-simple-number-keys)

  ```jsx
  ({ 0x1: 1 });
  ```

  <pre class="language-text"><code class="language-text">complexity/useSimpleNumberKeys.js:1:4 <a href="https://biomejs.dev/linter/rules/use-simple-number-keys">lint/complexity/useSimpleNumberKeys</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Hexadecimal number literal is not allowed here.</span>

  <strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>({ 0x1: 1 });
     <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
      <strong>2 │ </strong>

  <strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Replace 0x1 with 1</span>

  <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">(</span><span style="color: Tomato;">{</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>0</strong></span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;">:</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">1</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">}</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
    <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;">:</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">1</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">}</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">;</span>
  <strong>2</strong> <strong>2</strong><strong> │ </strong>

  </code></pre>

### Promoted rules

- [a11y/noInteractiveElementToNoninteractiveRole](https://biomejs.dev/linter/rules/no-interactive-element-to-noninteractive-role)
- [complexity/noThisInStatic](https://biomejs.dev/linter/rules/no-this-in-static)
- [complexity/useArrowFunction](https://biomejs.dev/linter/rules/use-arrow-function)
- [correctness/noEmptyCharacterClassInRegex](https://biomejs.dev/linter/rules/no-empty-character-class-in-regex)
- [correctness/noInvalidNewBuiltin](https://biomejs.dev/linter/rules/no-invalid-new-builtin)
- [style/noUselessElse](https://biomejs.dev/linter/rules/no-useless-else)
- [style/useAsConstAssertion](https://biomejs.dev/linter/rules/use-as-const-assertion)
- [style/useShorthandAssign](https://biomejs.dev/linter/rules/use-shorthand-assign)
- [suspicious/noApproximativeNumericConstant](https://biomejs.dev/linter/rules/no-approximative-numeric-constant)
- [suspicious/noMisleadingInstantiator](https://biomejs.dev/linter/rules/no-misleading-instantiator)
- [suspicious/noMisrefactoredShorthandAssign](https://biomejs.dev/linter/rules/no-misrefactored-shorthand-assign)

### Deprecated rules

- [correctness/noNewSymbol](https://biomejs.dev/linter/rules/no-new-symbol)

The rule is replaced by [correctness/noInvalidNewBuiltin](https://biomejs.dev/linter/rules/no-invalid-new-builtin)

## Homage to our maintainers

Since Biome was forked, new people joined the project. They have been helping with in so many ways that you can't even imagine: new features, side projects, engaging with the community, support, documentation, and more. OSS is not just about coding.

Thank you to:

- [Madeline Gurriarán @SuperchupuDev](https://github.com/SuperchupuDev)
- [Nicolas Hedger @nhedger](https://github.com/nhedger)
- [Victor Teles @victor-teles](https://github.com/victor-teles)

And a big welcome to our new joined maintainer:

- [Jon Egeland @faultyserver](https://github.com/faultyserver)
- [Takayuki Maeda @TaKO8Ki](https://github.com/TaKO8Ki)

## New sponsors

Last but not least, we are proud to announce that we have two new sponsors:

- Gold: Shiguredō (https://shiguredo.jp/)
- Bronze: KANAME (https://www.kanamekey.com/)

If you want to economically contribute to the project and help it to ship more features, you can do so from the [GitHub Sponsorship page](https://github.com/sponsors/biomejs/) or the [Open Collective page](https://opencollective.com/biome).

## What's next

The project is thriving, with more people curious about the project and contributors that want to be involved.

In the next months we will focus on:

- Publishing a Roadmap. **Keep an eye on it**, it will involve a lot of **interesting** work.
- Rebranding the website with a new logo.
- Translate the website in Japanese.

## Translations

- [中文翻译: Biome赢得了Prettier挑战](https://juejin.cn/post/7308643782375768118)
