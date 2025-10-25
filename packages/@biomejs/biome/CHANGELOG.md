# @biomejs/biome

## 2.2.7

### Patch Changes

- [#7715](https://github.com/biomejs/biome/pull/7715) [`b622425`](https://github.com/biomejs/biome/commit/b6224257e43b1ffda9f4a80564d83616ecfb27c4) Thanks [@Netail](https://github.com/Netail)! - Added the nursery rule [`noEmptySource`](https://biomejs.dev/linter/rules/no-empty-source/), disallowing meaningless js, css, json & graphql files to prevent codebase clutter.

- [#7714](https://github.com/biomejs/biome/pull/7714) [`c7e5a14`](https://github.com/biomejs/biome/commit/c7e5a1424441b09cf505cff31b93fcd1bcc4fd3e) Thanks [@MeGaNeKoS](https://github.com/MeGaNeKoS)! - Increased the maximum line limit for [noExcessiveLinesPerFunction](https://biomejs.dev/linter/rules/no-excessive-lines-per-function/) from 255 to 65,535 to better support large JSX/front-end components.

- [#5868](https://github.com/biomejs/biome/pull/5868) [`2db73ae`](https://github.com/biomejs/biome/commit/2db73aefb3d526041338d7174978524c4677b47e) Thanks [@bushuai](https://github.com/bushuai)! - Fixed [#5856](https://github.com/biomejs/biome/issues/5856), `noRedundantUseStrict` now keeps leading trivia

- [#7756](https://github.com/biomejs/biome/pull/7756) [`d665c97`](https://github.com/biomejs/biome/commit/d665c970338d8b334381e68eae4a26c5da0ac9a5) Thanks [@ematipico](https://github.com/ematipico)! - Improved the diagnostic message of the rule [`noDuplicateTestHooks`](https://biomejs.dev/linter/rules/no-duplicate-test-hooks/).

## 2.2.6

### Patch Changes

- [#7071](https://github.com/biomejs/biome/pull/7071) [`a8e7301`](https://github.com/biomejs/biome/commit/a8e73018a8c9e34a182624a91389e19d1fa7817f) Thanks [@ptkagori](https://github.com/ptkagori)! - Added the [`useQwikMethodUsage`](https://biomejs.dev/linter/rules/use-qwik-method-usage) lint rule for the Qwik domain.

  This rule validates Qwik hook usage. Identifiers matching `useXxx` must be called only within serialisable reactive contexts (for example, inside `component$`, route loaders/actions, or within other Qwik hooks), preventing common Qwik antipatterns.

  **Invalid:**

  ```js
  // Top-level hook call is invalid.
  const state = useStore({ count: 0 });

  function helper() {
    // Calling a hook in a non-reactive function is invalid.
    const loc = useLocation();
  }
  ```

  **Valid:**

  ```js
  component$(() => {
    const state = useStore({ count: 0 }); // OK inside component$.
    return <div>{state.count}</div>;
  });

  const handler = $(() => {
    const loc = useLocation(); // OK inside a $-wrapped closure.
    console.log(loc.params);
  });
  ```

- [#7685](https://github.com/biomejs/biome/pull/7685) [`52071f5`](https://github.com/biomejs/biome/commit/52071f54bc1a3c5d1d2ee6039c5feead836638ed) Thanks [@denbezrukov](https://github.com/denbezrukov)! - Fixed [#6981](https://github.com/biomejs/biome/issues/6981): The [NoUnknownPseudoClass](https://biomejs.dev/linter/rules/no-unknown-pseudo-class/) rule no longer reports local pseudo-classes when CSS Modules are used.

- [#7640](https://github.com/biomejs/biome/pull/7640) [`899f7b2`](https://github.com/biomejs/biome/commit/899f7b28ec9cc457d02565d69212e7c29b5b5aff) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#7638](https://github.com/biomejs/biome/issues/7638): [`useImportExtensions`](https://biomejs.dev/linter/rules/use-import-extensions/) no longer emits diagnostics on valid import paths that end with a query or hash.

  #### Example

  ```js
  // This no longer warns if `index.css` exists:
  import style from "../theme/index.css?inline";
  ```

- [#7071](https://github.com/biomejs/biome/pull/7071) [`a8e7301`](https://github.com/biomejs/biome/commit/a8e73018a8c9e34a182624a91389e19d1fa7817f) Thanks [@ptkagori](https://github.com/ptkagori)! - Added the [`useQwikValidLexicalScope`](https://biomejs.dev/linter/rules/use-qwik-valid-lexical-scope) rule to the Qwik domain.

  This rule helps you avoid common bugs in Qwik components by checking that your variables and functions are declared in the correct place.

  **Invalid:**

  ```js
  // Invalid: state defined outside the component's lexical scope.
  let state = useStore({ count: 0 });
  const Component = component$(() => {
    return (
      <button onClick$={() => state.count++}>Invalid: {state.count}</button>
    );
  });
  ```

  **Valid:**

  ```js
  // Valid: state initialised within the component's lexical scope and captured by the event.
  const Component = component$(() => {
    const state = useStore({ count: 0 });
    return <button onClick$={() => state.count++}>Valid: {state.count}</button>;
  });
  ```

- [#7620](https://github.com/biomejs/biome/pull/7620) [`5beb1ee`](https://github.com/biomejs/biome/commit/5beb1eefe134f4dc713cfb28bfa1cbae38319975) Thanks [@Netail](https://github.com/Netail)! - Added the rule [`useDeprecatedDate`](https://biomejs.dev/linter/rules/use-deprecated-date/), which makes a deprecation date required for the graphql `@deprecated` directive.

  ##### Invalid

  ```graphql
  query {
    member @deprecated(reason: "Use `members` instead") {
      id
    }
  }
  ```

  ##### Valid

  ```graphql
  query {
    member
      @deprecated(reason: "Use `members` instead", deletionDate: "2099-12-25") {
      id
    }
  }
  ```

- [#7709](https://github.com/biomejs/biome/pull/7709) [`d6da4d5`](https://github.com/biomejs/biome/commit/d6da4d5a272d61420997e26aef80f53298515665) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#7704](https://github.com/biomejs/biome/issues/7704): The [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) rule now correctly adds an object dependency when its method is called within the closure.

  For example:

  ```js
  function Component(props) {
    useEffect(() => {
      props.foo();
    }, []);
  }
  ```

  will now be fixed to:

  ```js
  function Component(props) {
    useEffect(() => {
      props.foo();
    }, [props]);
  }
  ```

- [#7624](https://github.com/biomejs/biome/pull/7624) [`309ae41`](https://github.com/biomejs/biome/commit/309ae41c1a29e50d71300d3e63f6c64ee6ecb968) Thanks [@lucasweng](https://github.com/lucasweng)! - Fixed [#7595](https://github.com/biomejs/biome/issues/7595): [`noUselessEscapeInString`](https://biomejs.dev/linter/rules/no-useless-escape-in-string/) no longer reports `$\{` escape in template literals.

- [#7665](https://github.com/biomejs/biome/pull/7665) [`29e4229`](https://github.com/biomejs/biome/commit/29e422939f25595dca4f19735a27258d97545288) Thanks [@ryan-m-walker](https://github.com/ryan-m-walker)! - Fixed [#7619](https://github.com/biomejs/biome/issues/7619): Added support for parsing the CSS `:state()` pseudo-class.

  ```css
  custom-selector:state(checked) {
  }
  ```

- [#7608](https://github.com/biomejs/biome/pull/7608) [`41df59b`](https://github.com/biomejs/biome/commit/41df59bfc6d49190b9c35fa262def3ecfcc6abd2) Thanks [@ritoban23](https://github.com/ritoban23)! - Fixed [#7604](https://github.com/biomejs/biome/issues/7604): the `useMaxParams` rule now highlights parameter lists instead of entire function bodies. This provides more precise error highlighting. Previously, the entire function was highlighted; now only the parameter list is highlighted, such as `(a, b, c, d, e, f, g, h)`.

- [#7643](https://github.com/biomejs/biome/pull/7643) [`459a6ac`](https://github.com/biomejs/biome/commit/459a6aca67290e8b974802bd693738f79883d67e) Thanks [@daivinhtran](https://github.com/daivinhtran)! - Fixed [#7580](https://github.com/biomejs/biome/issues/7580): Include plugin in summary report

## 2.2.5

### Patch Changes

- [#7597](https://github.com/biomejs/biome/pull/7597) [`5c3d542`](https://github.com/biomejs/biome/commit/5c3d542e65fee652dc4e52f3ec2de0441c3f3aec) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#6432](https://github.com/biomejs/biome/issues/6432): [`useImportExtensions`](https://biomejs.dev/linter/rules/use-import-extensions/) now works correctly with aliased paths.

- [#7269](https://github.com/biomejs/biome/pull/7269) [`f18dac1`](https://github.com/biomejs/biome/commit/f18dac1d662c426d036894a59755eb26f5668aaf) Thanks [@CDGardner](https://github.com/CDGardner)! - Fixed [#6648](https://github.com/biomejs/biome/issues/6648), where Biome's `noUselessFragments` contained inconsistencies with ESLint for fragments only containing text.

  Previously, Biome would report that fragments with only text were unnecessary under the `noUselessFragments` rule. Further analysis of ESLint's behavior towards these cases revealed that text-only fragments (`<>A</a>`, `<React.Fragment>B</React.Fragment>`, `<RenamedFragment>B</RenamedFragment>`) would not have `noUselessFragments` emitted for them.

  On the Biome side, instances such as these would emit `noUselessFragments`, and applying the suggested fix would turn the text content into a proper JS string.

  ```js
  // Ended up as: - const t = "Text"
  const t = <>Text</>

  // Ended up as: - const e = t ? "Option A" : "Option B"
  const e = t ? <>Option A</> : <>Option B</>

  /* Ended up as:
    function someFunc() {
      return "Content desired to be a multi-line block of text."
    }
  */
  function someFunc() {
    return <>
      Content desired to be a multi-line
      block of text.
    <>
  }
  ```

  The proposed update was to align Biome's reaction to this rule with ESLint's; the aforementioned examples will now be supported from Biome's perspective, thus valid use of fragments.

  ```js
  // These instances are now valid and won't be called out by noUselessFragments.

  const t = <>Text</>
  const e = t ? <>Option A</> : <>Option B</>

  function someFunc() {
    return <>
      Content desired to be a multi-line
      block of text.
    <>
  }
  ```

- [#7498](https://github.com/biomejs/biome/pull/7498) [`002cded`](https://github.com/biomejs/biome/commit/002cded543e6aa5f5cf55f48312f40c83975a22f) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#6893](https://github.com/biomejs/biome/issues/6893): The [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) rule now correctly adds a dependency that is captured in a shorthand object member. For example:

  ```jsx
  useEffect(() => {
    console.log({ firstId, secondId });
  }, []);
  ```

  is now correctly fixed to:

  ```jsx
  useEffect(() => {
    console.log({ firstId, secondId });
  }, [firstId, secondId]);
  ```

- [#7509](https://github.com/biomejs/biome/pull/7509) [`1b61631`](https://github.com/biomejs/biome/commit/1b61631c63f161fa8163365571825c99aed3eaae) Thanks [@siketyan](https://github.com/siketyan)! - Added a new lint rule [`noReactForwardRef`](https://biomejs.dev/linter/rules/no-react-forward-ref/), which detects usages of `forwardRef` that is no longer needed and deprecated in React 19.

  For example:

  ```jsx
  export const Component = forwardRef(function Component(props, ref) {
    return <div ref={ref} />;
  });
  ```

  will be fixed to:

  ```jsx
  export const Component = function Component({ ref, ...props }) {
    return <div ref={ref} />;
  };
  ```

  Note that the rule provides an unsafe fix, which may break the code. Don't forget to review the code after applying the fix.

- [#7520](https://github.com/biomejs/biome/pull/7520) [`3f06e19`](https://github.com/biomejs/biome/commit/3f06e19c6eb8476ad9de4e3dac00c50a2d6f0aed) Thanks [@arendjr](https://github.com/arendjr)! - Added new nursery rule [`noDeprecatedImports`](https://biomejs.dev/linter/rules/no-deprecated-imports/) to flag imports of deprecated symbols.

  #### Invalid example

  ```js
  // foo.js
  import { oldUtility } from "./utils.js";
  ```

  ```js
  // utils.js
  /**
   * @deprecated
   */
  export function oldUtility() {}
  ```

  #### Valid examples

  ```js
  // foo.js
  import { newUtility, oldUtility } from "./utils.js";
  ```

  ```js
  // utils.js
  export function newUtility() {}

  // @deprecated (this is not a JSDoc comment)
  export function oldUtility() {}
  ```

- [#7457](https://github.com/biomejs/biome/pull/7457) [`9637f93`](https://github.com/biomejs/biome/commit/9637f9308fe39f7e94d42419cd430cc2a55d5473) Thanks [@kedevked](https://github.com/kedevked)! - Added `style` and `requireForObjectLiteral` options to the lint rule [`useConsistentArrowReturn`](https://biomejs.dev/linter/rules/use-consistent-arrow-return/).

  This rule enforces a consistent return style for arrow functions. It can be configured with the following options:
  - `style`: (default: `asNeeded`)
    - `always`: enforces that arrow functions always have a block body.
    - `never`: enforces that arrow functions never have a block body, when possible.
    - `asNeeded`: enforces that arrow functions have a block body only when necessary (e.g. for object literals).

  #### `style: "always"`

  Invalid:

  ```js
  const f = () => 1;
  ```

  Valid:

  ```js
  const f = () => {
    return 1;
  };
  ```

  #### `style: "never"`

  Invalid:

  ```js
  const f = () => {
    return 1;
  };
  ```

  Valid:

  ```js
  const f = () => 1;
  ```

  #### `style: "asNeeded"`

  Invalid:

  ```js
  const f = () => {
    return 1;
  };
  ```

  Valid:

  ```js
  const f = () => 1;
  ```

  #### `style: "asNeeded"` and `requireForObjectLiteral: true`

  Valid:

  ```js
  const f = () => {
    return { a: 1 };
  };
  ```

- [#7510](https://github.com/biomejs/biome/pull/7510) [`527cec2`](https://github.com/biomejs/biome/commit/527cec2ca10df23754e9958d17baefca6a559154) Thanks [@rriski](https://github.com/rriski)! - Implements [#7339](https://github.com/biomejs/biome/discussions/7339). GritQL patterns can now use native Biome AST nodes using their `PascalCase` names, in addition to the existing TreeSitter-compatible `snake_case` names.

  ```grit
  engine biome(1.0)
  language js(typescript,jsx)

  or {
    // TreeSitter-compatible pattern
    if_statement(),

    // Native Biome AST node pattern
    JsIfStatement()
  } as $stmt where {
    register_diagnostic(
      span=$stmt,
      message="Found an if statement"
    )
  }
  ```

- [#7574](https://github.com/biomejs/biome/pull/7574) [`47907e7`](https://github.com/biomejs/biome/commit/47907e7d9badbe0c41c6a23bdd962676de216db0) Thanks [@kedevked](https://github.com/kedevked)! - Fixed [7574](https://github.com/biomejs/biome/pull/7574). The diagnostic message for the rule `useSolidForComponent` now correctly emphasizes `<For />` and provides a working hyperlink to the Solid documentation.

- [#7497](https://github.com/biomejs/biome/pull/7497) [`bd70f40`](https://github.com/biomejs/biome/commit/bd70f40cb933c1df0c171a9048b62da432093308) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#7320](https://github.com/biomejs/biome/issues/7320): The [`useConsistentCurlyBraces`](https://biomejs.dev/linter/rules/use-consistent-curly-braces/) rule now correctly detects a string literal including `"` inside a JSX attribute value.

- [#7522](https://github.com/biomejs/biome/pull/7522) [`1af9931`](https://github.com/biomejs/biome/commit/1af993134ba2d9158f6824c2f002c90133c0e3f4) Thanks [@Netail](https://github.com/Netail)! - Added extra references to external rules to improve migration for the following rules: `noUselessFragments` & `noNestedComponentDefinitions`

- [#7597](https://github.com/biomejs/biome/pull/7597) [`5c3d542`](https://github.com/biomejs/biome/commit/5c3d542e65fee652dc4e52f3ec2de0441c3f3aec) Thanks [@arendjr](https://github.com/arendjr)! - Fixed an issue where `package.json` manifests would not be correctly discovered
  when evaluating files in the same directory.

- [#7565](https://github.com/biomejs/biome/pull/7565) [`38d2098`](https://github.com/biomejs/biome/commit/38d2098bb3a81adaf73a19807c1e62d352405764) Thanks [@siketyan](https://github.com/siketyan)! - The resolver can now correctly resolve `.ts`, `.tsx`, `.d.ts`, `.js` files by `.js` extension if exists, based on [the file extension substitution in TypeScript](https://www.typescriptlang.org/docs/handbook/modules/reference.html#file-extension-substitution).

  For example, the linter can now detect the floating promise in the following situation, if you have enabled the `noFloatingPromises` rule.

  **`foo.ts`**

  ```ts
  export async function doSomething(): Promise<void> {}
  ```

  **`bar.ts`**

  ```ts
  import { doSomething } from "./foo.js"; // doesn't exist actually, but it is resolved to `foo.ts`

  doSomething(); // floating promise!
  ```

- [#7542](https://github.com/biomejs/biome/pull/7542) [`cadad2c`](https://github.com/biomejs/biome/commit/cadad2cadbd3852873cbd3f721c26ae7ceb3f39a) Thanks [@mdevils](https://github.com/mdevils)! - Added the rule [`noVueDuplicateKeys`](https://biomejs.dev/linter/rules/no-vue-duplicate-keys/), which prevents duplicate keys in Vue component definitions.

  This rule prevents the use of duplicate keys across different Vue component options such as `props`, `data`, `computed`, `methods`, and `setup`. Even if keys don't conflict in the script tag, they may cause issues in the template since Vue allows direct access to these keys.

  ##### Invalid examples

  ```vue
  <script>
  export default {
    props: ["foo"],
    data() {
      return {
        foo: "bar",
      };
    },
  };
  </script>
  ```

  ```vue
  <script>
  export default {
    data() {
      return {
        message: "hello",
      };
    },
    methods: {
      message() {
        console.log("duplicate key");
      },
    },
  };
  </script>
  ```

  ```vue
  <script>
  export default {
    computed: {
      count() {
        return this.value * 2;
      },
    },
    methods: {
      count() {
        this.value++;
      },
    },
  };
  </script>
  ```

  ##### Valid examples

  ```vue
  <script>
  export default {
    props: ["foo"],
    data() {
      return {
        bar: "baz",
      };
    },
    methods: {
      handleClick() {
        console.log("unique key");
      },
    },
  };
  </script>
  ```

  ```vue
  <script>
  export default {
    computed: {
      displayMessage() {
        return this.message.toUpperCase();
      },
    },
    methods: {
      clearMessage() {
        this.message = "";
      },
    },
  };
  </script>
  ```

- [#7546](https://github.com/biomejs/biome/pull/7546) [`a683acc`](https://github.com/biomejs/biome/commit/a683acc30bf85d1337760aa1500eb892ebc8e0ac) Thanks [@siketyan](https://github.com/siketyan)! - Internal data for Unicode strings have been updated to Unicode 17.0.

- [#7497](https://github.com/biomejs/biome/pull/7497) [`bd70f40`](https://github.com/biomejs/biome/commit/bd70f40cb933c1df0c171a9048b62da432093308) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#7256](https://github.com/biomejs/biome/issues/7256): The [`useConsistentCurlyBraces`](https://biomejs.dev/linter/rules/use-consistent-curly-braces/) rule now correctly ignores a string literal with braces that contains only whitespaces. Previously, literals that contains single whitespace were only allowed.

- [#7565](https://github.com/biomejs/biome/pull/7565) [`38d2098`](https://github.com/biomejs/biome/commit/38d2098bb3a81adaf73a19807c1e62d352405764) Thanks [@siketyan](https://github.com/siketyan)! - The [`useImportExtensions`](https://biomejs.dev/linter/rules/use-import-extensions/) rule now correctly detects imports with an invalid extension. For example, importing `.ts` file with `.js` extension is flagged by default. If you are using TypeScript with neither the `allowImportingTsExtensions` option nor the `rewriteRelativeImportExtensions` option, it's recommended to turn on the `forceJsExtensions` option of the rule.

- [#7581](https://github.com/biomejs/biome/pull/7581) [`8653921`](https://github.com/biomejs/biome/commit/86539215dde0c29eae0a6975b442637048a8673b) Thanks [@lucasweng](https://github.com/lucasweng)! - Fixed [#7470](https://github.com/biomejs/biome/issues/7470): solved a false positive for [`noDuplicateProperties`](https://biomejs.dev/linter/rules/no-duplicate-properties/). Previously, declarations in `@container` and `@starting-style` at-rules were incorrectly flagged as duplicates of identical declarations at the root selector.

  For example, the linter no longer flags the `display` declaration in `@container` or the `opacity` declaration in `@starting-style`.

  ```css
  a {
    display: block;
    @container (min-width: 600px) {
      display: none;
    }
  }

  [popover]:popover-open {
    opacity: 1;
    @starting-style {
      opacity: 0;
    }
  }
  ```

- [#7529](https://github.com/biomejs/biome/pull/7529) [`fea905f`](https://github.com/biomejs/biome/commit/fea905f0af9fc992a17fe1dcdbc3e0e63fae9d65) Thanks [@qraqras](https://github.com/qraqras)! - Fixed [#7517](https://github.com/biomejs/biome/issues/7517): the [`useOptionalChain`](https://biomejs.dev/linter/rules/use-optional-chain/) rule no longer suggests changes for typeof checks on global objects.

  ```ts
  // ok
  typeof window !== "undefined" && window.location;
  ```

- [#7476](https://github.com/biomejs/biome/pull/7476) [`c015765`](https://github.com/biomejs/biome/commit/c015765af2defb042285d96588fcb5f531eb8b6f) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a bug where the suppression action for `noPositiveTabindex` didn't place the suppression comment in the correct position.

- [#7511](https://github.com/biomejs/biome/pull/7511) [`a0039fd`](https://github.com/biomejs/biome/commit/a0039fd5457d0df18242feed5d21ff868ceb0693) Thanks [@arendjr](https://github.com/arendjr)! - Added nursery rule [`noUnusedExpressions`](https://biomejs.dev/linter/rules/no-unused-expressions/) to flag expressions used as a statement that is neither an assignment nor a function call.

  #### Invalid examples

  ```js
  f; // intended to call `f()` instead
  ```

  ```js
  function foo() {
    0; // intended to `return 0` instead
  }
  ```

  #### Valid examples

  ```js
  f();
  ```

  ```js
  function foo() {
    return 0;
  }
  ```

- [#7564](https://github.com/biomejs/biome/pull/7564) [`40e515f`](https://github.com/biomejs/biome/commit/40e515f73275ad0023ec03e95551a3bbb79b84a1) Thanks [@turbocrime](https://github.com/turbocrime)! - Fixed [#6617](https://github.com/biomejs/biome/issues/6617): improved [`useIterableCallbackReturn`](https://biomejs.dev/linter/rules/use-iterable-callback-return/) to correctly handle arrow functions with a single-expression `void` body.

  Now the following code doesn't trigger the rule anymore:

  ```js
  [].forEach(() => void null);
  ```

## 2.2.4

### Patch Changes

- [#7453](https://github.com/biomejs/biome/pull/7453) [`aa8cea3`](https://github.com/biomejs/biome/commit/aa8cea31af675699e18988fe79242ae5d5215af1) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#7242](https://github.com/biomejs/biome/issues/7242): Aliases specified in
  `package.json`'s `imports` section now support having multiple targets as part of an array.

- [#7454](https://github.com/biomejs/biome/pull/7454) [`ac17183`](https://github.com/biomejs/biome/commit/ac171839a31600225e3b759470eaa026746e9cf4) Thanks [@arendjr](https://github.com/arendjr)! - Greatly improved performance of
  `noImportCycles` by eliminating allocations.

  In one repository, the total runtime of Biome with only `noImportCycles` enabled went from ~23s down to ~4s.

- [#7447](https://github.com/biomejs/biome/pull/7447) [`7139aad`](https://github.com/biomejs/biome/commit/7139aad75b6e8045be6eb09425fb82eb035fb704) Thanks [@rriski](https://github.com/rriski)! - Fixes [#7446](https://github.com/biomejs/biome/issues/7446). The GritQL
  `$...` spread metavariable now correctly matches members in object literals, aligning its behavior with arrays and function calls.

- [#6710](https://github.com/biomejs/biome/pull/6710) [`98cf9af`](https://github.com/biomejs/biome/commit/98cf9af0a4e02434983899ce49d92209a6abab02) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#4723](https://github.com/biomejs/biome/issues/7423): Type inference now recognises
  _index signatures_ and their accesses when they are being indexed as a string.

  #### Example

  ```ts
  type BagOfPromises = {
    // This is an index signature definition. It declares that instances of type
    // `BagOfPromises` can be indexed using arbitrary strings.
    [property: string]: Promise<void>;
  };

  let bag: BagOfPromises = {};
  // Because `bag.iAmAPromise` is equivalent to `bag["iAmAPromise"]`, this is
  // considered an access to the string index, and a Promise is expected.
  bag.iAmAPromise;
  ```

- [#7415](https://github.com/biomejs/biome/pull/7415) [`d042f18`](https://github.com/biomejs/biome/commit/d042f18f556edfd4fecff562c8f197dbec81a5e7) Thanks [@qraqras](https://github.com/qraqras)! - Fixed [#7212](https://github.com/biomejs/biome/issues/7212), now the [`useOptionalChain`](https://biomejs.dev/linter/rules/use-optional-chain/) rule recognizes optional chaining using
  `typeof` (e.g., `typeof foo !== 'undefined' && foo.bar`).

- [#7419](https://github.com/biomejs/biome/pull/7419) [`576baf4`](https://github.com/biomejs/biome/commit/576baf4faf568e8b6a295f457f70894235ffdb59) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#7323](https://github.com/biomejs/biome/issues/7323). [`noUnusedPrivateClassMembers`](https://biomejs.dev/linter/rules/no-unused-private-class-members/) no longer reports as unused TypeScript
  `private` members if the rule encounters a computed access on `this`.

  In the following example, `member` as previously reported as unused. It is no longer reported.

  ```ts
  class TsBioo {
    private member: number;

    set_with_name(name: string, value: number) {
      this[name] = value;
    }
  }
  ```

- [`351bccd`](https://github.com/biomejs/biome/commit/351bccdfe49a6173cb1446ef2a8a9171c8d78c26) Thanks [@ematipico](https://github.com/ematipico)! - Added the new nursery lint rule
  `noJsxLiterals`, which disallows the use of string literals inside JSX.

  The rule catches these cases:

  ```jsx
  <>
    <div>test</div> {/* test is invalid */}
    <>test</>
    <div>
      {/* this string is invalid */}
      asdjfl test foo
    </div>
  </>
  ```

- [#7406](https://github.com/biomejs/biome/pull/7406) [`b906112`](https://github.com/biomejs/biome/commit/b90611223dbab116c4c1678a374c1a48c29a34a0) Thanks [@mdevils](https://github.com/mdevils)! - Fixed an issue (#6393) where the [useHookAtTopLevel](https://biomejs.dev/linter/rules/use-hook-at-top-level/) rule reported excessive diagnostics for nested hook calls.

  The rule now reports only the offending top-level call site, not sub-hooks of composite hooks.

  ```js
  // Before: reported twice (useFoo and useBar).
  function useFoo() {
    return useBar();
  }
  function Component() {
    if (cond) useFoo();
  }
  // After: reported once at the call to useFoo().
  ```

- [#7461](https://github.com/biomejs/biome/pull/7461) [`ea585a9`](https://github.com/biomejs/biome/commit/ea585a9394a4126370b865f565ad43b757e736ab) Thanks [@arendjr](https://github.com/arendjr)! - Improved performance of
  `noPrivateImports` by eliminating allocations.

  In one repository, the total runtime of Biome with only `noPrivateImports` enabled went from ~3.2s down to ~1.4s.

- [`351bccd`](https://github.com/biomejs/biome/commit/351bccdfe49a6173cb1446ef2a8a9171c8d78c26) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#7411](https://github.com/biomejs/biome/issues/7411). The Biome Language Server had a regression where opening an editor with a file already open wouldn't load the project settings correctly.

- [#7142](https://github.com/biomejs/biome/pull/7142) [`53ff5ae`](https://github.com/biomejs/biome/commit/53ff5ae34428f042bb5b80c19862c9cf69fc6359) Thanks [@Netail](https://github.com/Netail)! - Added the new nursery rule [`noDuplicateDependencies`](https://biomejs.dev/linter/rules/no-duplicate-dependencies/), which verifies that no dependencies are duplicated between the
  `bundledDependencies`, `bundleDependencies`, `dependencies`, `devDependencies`, `overrides`,
  `optionalDependencies`, and `peerDependencies` sections.

  For example, the following snippets will trigger the rule:

  ```json
  {
    "dependencies": {
      "foo": ""
    },
    "devDependencies": {
      "foo": ""
    }
  }
  ```

  ```json
  {
    "dependencies": {
      "foo": ""
    },
    "optionalDependencies": {
      "foo": ""
    }
  }
  ```

  ```json
  {
    "dependencies": {
      "foo": ""
    },
    "peerDependencies": {
      "foo": ""
    }
  }
  ```

- [`351bccd`](https://github.com/biomejs/biome/commit/351bccdfe49a6173cb1446ef2a8a9171c8d78c26) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#3824](https://github.com/biomejs/biome/issues/3824). Now the option CLI
  `--color` is correctly applied to logging too.

## 2.2.3

### Patch Changes

- [#7353](https://github.com/biomejs/biome/pull/7353) [`4d2b719`](https://github.com/biomejs/biome/commit/4d2b7190f855a88bdae467a2efc00b81721bee62) Thanks [@JeetuSuthar](https://github.com/JeetuSuthar)! - Fixed [#7340](https://github.com/biomejs/biome/issues/7340): The linter now allows the
  `navigation` property for view-transition in CSS.

  Previously, the linter incorrectly flagged `navigation: auto` as an unknown property. This fix adds
  `navigation` to the list of known CSS properties, following the [CSS View Transitions spec](https://www.w3.org/TR/css-view-transitions-2/#view-transition-navigation-descriptor).

- [#7275](https://github.com/biomejs/biome/pull/7275) [`560de1b`](https://github.com/biomejs/biome/commit/560de1bf3f22f4a8a5cdc224256a34dbb9d78481) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#7268](https://github.com/biomejs/biome/issues/7268): Files that are explicitly passed as CLI arguments are now correctly ignored if they reside in an ignored folder.

- [#7358](https://github.com/biomejs/biome/pull/7358) [`963a246`](https://github.com/biomejs/biome/commit/963a24643cbf4d91cca81569b33a8b7e21b4dd0b) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#7085](https://github.com/biomejs/biome/issues/7085), now the rule
  `noDescendingSpecificity` correctly calculates the specificity of selectors when they are included inside a media query.

- [#7387](https://github.com/biomejs/biome/pull/7387) [`923674d`](https://github.com/biomejs/biome/commit/923674dbf8cc4c23ab569cd00ae0a0cf2a3ab791) Thanks [@qraqras](https://github.com/qraqras)! - Fixed [#7381](https://github.com/biomejs/biome/issues/7381), now the [`useOptionalChain`](https://biomejs.dev/ja/linter/rules/use-optional-chain/) rule recognizes optional chaining using Yoda expressions (e.g.,
  `undefined !== foo && foo.bar`).

- [#7316](https://github.com/biomejs/biome/pull/7316) [`f9636d5`](https://github.com/biomejs/biome/commit/f9636d5de1e8aef742d145a886f05a4cd79eca31) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#7289](https://github.com/biomejs/biome/issues/7289). The rule [`useImportType`](https://biomejs.dev/linter/rules/use-import-type/) now inlines `import type` into
  `import { type }` when the `style` option is set to `inlineType`.

  Example:

  ```ts
  import type { T } from "mod";
  // becomes
  import { type T } from "mod";
  ```

- [#7350](https://github.com/biomejs/biome/pull/7350) [`bb4d407`](https://github.com/biomejs/biome/commit/bb4d407747dd29df78776f143ad63657f869be11) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#7261](https://github.com/biomejs/biome/issues/7261): two characters
  `・` (KATAKANA MIDDLE DOT, U+30FB) and
  `･` (HALFWIDTH KATAKANA MIDDLE DOT, U+FF65) are no longer considered as valid characters in identifiers. Property keys containing these character(s) are now preserved as string literals.

- [#7377](https://github.com/biomejs/biome/pull/7377) [`811f47b`](https://github.com/biomejs/biome/commit/811f47b35163e70dce106f62d0aea4ef9e6b91bb) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a bug where the Biome Language Server didn't correctly compute the diagnostics of a monorepo setting, caused by an incorrect handling of the project status.

- [#7245](https://github.com/biomejs/biome/pull/7245) [`fad34b9`](https://github.com/biomejs/biome/commit/fad34b9db9778fe964ff7dbc489de0bfad2d3ece) Thanks [@kedevked](https://github.com/kedevked)! - Added the new lint rule
  `useConsistentArrowReturn`.

  This rule enforces a consistent return style for arrow functions.

  ### Invalid

  ```js
  const f = () => {
    return 1;
  };
  ```

  This rule is a port of ESLint's [arrow-body-style](https://eslint.org/docs/latest/rules/arrow-body-style) rule.

- [#7370](https://github.com/biomejs/biome/pull/7370) [`e8032dd`](https://github.com/biomejs/biome/commit/e8032ddfdd734a1441335d82b49db478248e6992) Thanks [@fireairforce](https://github.com/fireairforce)! - Support dynamic
  `import defer` and `import source`. The syntax looks like:

  ```ts
  import.source("foo");
  import.source("x", { with: { attr: "val" } });
  import.defer("foo");
  import.defer("x", { with: { attr: "val" } });
  ```

- [#7369](https://github.com/biomejs/biome/pull/7369) [`b1f8cbd`](https://github.com/biomejs/biome/commit/b1f8cbd88619deb269b2028eb0578657987848c5) Thanks [@siketyan](https://github.com/siketyan)! - Range suppressions are now supported for Grit plugins.

  For JavaScript, you can suppress a plugin as follows:

  ```js
  // biome-ignore-start lint/plugin/preferObjectSpread: reason
  Object.assign({ foo: "bar" }, baz);
  // biome-ignore-end lint/plugin/preferObjectSpread: reason
  ```

  For CSS, you can suppress a plugin as follows:

  ```css
  body {
    /* biome-ignore-start lint/plugin/useLowercaseColors: reason */
    color: #fff;
    /* biome-ignore-end lint/plugin/useLowercaseColors: reason */
  }
  ```

- [#7384](https://github.com/biomejs/biome/pull/7384) [`099507e`](https://github.com/biomejs/biome/commit/099507eb07f14f7d383f848fb6c659b5a6ccfd92) Thanks [@ematipico](https://github.com/ematipico)! - Reduced the severity of certain diagnostics emitted when Biome deserializes the configuration files. Now these diagnostics are emitted as
  `Information` severity, which means that they won't interfere when running commands with `--error-on-warnings`

- [#7302](https://github.com/biomejs/biome/pull/7302) [`2af2380`](https://github.com/biomejs/biome/commit/2af2380b8210e74efea467139a8a4cb4747c8af4) Thanks [@unvalley](https://github.com/unvalley)! - Fixed [#7301](https://github.com/biomejs/biome/issues/7301): [`useReadonlyClassProperties`](https://biomejs.dev/linter/rules/use-readonly-class-properties/) now correctly skips JavaScript files.

- [#7288](https://github.com/biomejs/biome/pull/7288) [`94d85f8`](https://github.com/biomejs/biome/commit/94d85f8fe54305e8fa070490bb2f7c86a91c5e92) Thanks [@ThiefMaster](https://github.com/ThiefMaster)! - Fixed [#7286](https://github.com/biomejs/biome/issues/7286). Files are now formatted with JSX behavior when
  `javascript.parser.jsxEverywhere` is explicitly set.

  Previously, this flag was only used for parsing, but not for formatting, which resulted in incorrect formatting of conditional expressions when JSX syntax is used in
  `.js` files.

- [#7311](https://github.com/biomejs/biome/pull/7311) [`62154b9`](https://github.com/biomejs/biome/commit/62154b93e0aa1609afb3d2b1f5468b63ab79374a) Thanks [@qraqras](https://github.com/qraqras)! - Added the new nursery rule
  `noUselessCatchBinding`. This rule disallows unnecessary catch bindings.

  ```diff
  try {
      // Do something
  - } catch (unused) {}
  + } catch {}
  ```

- [#7349](https://github.com/biomejs/biome/pull/7349) [`45c1dfe`](https://github.com/biomejs/biome/commit/45c1dfe32879f4bbb75cbf9b3ee86e304a02aaa1) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4298](https://github.com/biomejs/biome/issues/4298). Biome now correctly formats CSS declarations when it contains one single value:

  ```diff
  .bar {
  -  --123456789012345678901234567890: var(--1234567890123456789012345678901234567);
  +  --123456789012345678901234567890: var(
  +    --1234567890123456789012345678901234567
  +  );
  }
  ```

- [#7295](https://github.com/biomejs/biome/pull/7295) [`7638e84`](https://github.com/biomejs/biome/commit/7638e84b026c8b008fa1efdd795b8c0bff0733ab) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#7130](https://github.com/biomejs/biome/issues/7130). Removed the emission of a false-positive diagnostic. Biome no longer emits the following diagnostic:

  ```
  lib/main.ts:1:5 suppressions/unused ━━━━━━━━━━━━━━━━━━━━━━━━━

    ⚠ Suppression comment has no effect because the tool is not enabled.

    > 1 │ /** biome-ignore-all assist/source/organizeImports: For the lib root file, we don't want to organize exports */
        │     ^^^^^^^^^^^^^^^^

  ```

- [#7377](https://github.com/biomejs/biome/pull/7377) [`811f47b`](https://github.com/biomejs/biome/commit/811f47b35163e70dce106f62d0aea4ef9e6b91bb) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#7371](https://github.com/biomejs/biome/issues/7371) where the Biome Language Server didn't correctly recompute the diagnostics when updating a nested configuration file.

- [#7348](https://github.com/biomejs/biome/pull/7348) [`ac27fc5`](https://github.com/biomejs/biome/commit/ac27fc56dbb14c8f8507ffc4b7d6bf27aa3780db) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#7079](https://github.com/biomejs/biome/issues/7079). Now the rule [`useSemanticElements`](https://biomejs.dev/linter/rules/use-semantic-elements/) doesn't trigger components and custom elements.

- [#7389](https://github.com/biomejs/biome/pull/7389) [`ab06a7e`](https://github.com/biomejs/biome/commit/ab06a7ea9523ecb39ebf74a14600a02332e9d4e1) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#7344](https://github.com/biomejs/biome/issues/7344). [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) no longer reports interfaces defined in global declarations.

  Interfaces declared in global declarations augment existing interfaces. Thus, they must be ignored.

  In the following example, `useNamingConvention` reported `HTMLElement`. It is now ignored.

  ```ts
  export {};
  declare global {
    interface HTMLElement {
      foo(): void;
    }
  }
  ```

- [#7315](https://github.com/biomejs/biome/pull/7315) [`4a2bd2f`](https://github.com/biomejs/biome/commit/4a2bd2f38d1f449e55f88be351fcc1cf1d561e69) Thanks [@vladimir-ivanov](https://github.com/vladimir-ivanov)! - Fixed [#7310](https://github.com/biomejs/biome/issues/7310): [`useReadonlyClassProperties`](https://biomejs.dev/linter/rules/use-readonly-class-properties/) correctly handles nested assignments, avoiding false positives when a class property is assigned within another assignment expression.

  Example of code that previously triggered a false positive but is now correctly ignored:

  ```ts
  class test {
    private thing: number = 0; // incorrectly flagged

    public incrementThing(): void {
      const temp = { x: 0 };
      temp.x = this.thing++;
    }
  }
  ```

## 2.2.2

### Patch Changes

- [#7266](https://github.com/biomejs/biome/pull/7266) [`b270bb5`](https://github.com/biomejs/biome/commit/b270bb59978efafeef48e0b7d834c9b3958bae51) Thanks [@ematipico](https://github.com/ematipico)! - Fixed an issue where Biome got stuck when analyzing some files. This is usually caused by a bug in the inference engine. Now Biome has some guards in place in case the number of types grows too much, and if that happens, a diagnostic is emitted and the inference is halted.

- [#7281](https://github.com/biomejs/biome/pull/7281) [`6436180`](https://github.com/biomejs/biome/commit/6436180f4a3b257e2de018bac45c99a76eff58be) Thanks [@ematipico](https://github.com/ematipico)! - Fixed an issue where the function
  `scanProject` wouldn't work as expected.

- [#7285](https://github.com/biomejs/biome/pull/7285) [`1511d0c`](https://github.com/biomejs/biome/commit/1511d0c1fdbab576701f12e9dbfca11141b60e3f) Thanks [@rriski](https://github.com/rriski)! - Partially fixed [#6782](https://github.com/biomejs/biome/issues/6782): JSX node kinds are now supported in GritQL AST nodes.

- [#7249](https://github.com/biomejs/biome/pull/7249) [`dff85c0`](https://github.com/biomejs/biome/commit/dff85c05ec1ecfd252028476828d63d15b0ed60f) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#748](https://github.com/biomejs/biome-vscode/issues/748), where Biome Language Server didn't show the unsafe fixes when requesting the quick fixes. Now all LSP editors will show also opt-in, unsafe fixes.

- [#7266](https://github.com/biomejs/biome/pull/7266) [`b270bb5`](https://github.com/biomejs/biome/commit/b270bb59978efafeef48e0b7d834c9b3958bae51) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#7020](https://github.com/biomejs/biome/issues/7020): Resolved an issue with analysing types of static member expressions involving unions. If the object type was a union that referenced nested unions, it would trigger an infinite loop as it tried to keep expanding nested unions, and the set of types would grow indefinitely.

- [#7209](https://github.com/biomejs/biome/pull/7209) [`679b70e`](https://github.com/biomejs/biome/commit/679b70e8a5141250f74a11ce7e615b15fc711914) Thanks [@patrickshipe](https://github.com/patrickshipe)! - Resolved an overcorrection in [`useImportExtensions`](https://biomejs.dev/linter/rules/use-import-extensions/) when importing explicit index files.

  Imports that explicitly reference an index file are now preserved and no longer rewritten to nested index paths.

  #### Example

  ```diff
  // Before
  -      import "./sub/index";
  +      import "./sub/index/index.js";

  // After
  -      import "./sub/index";
  +      import "./sub/index.js";
  ```

- [#7270](https://github.com/biomejs/biome/pull/7270) [`953f9c6`](https://github.com/biomejs/biome/commit/953f9c6f019412caf14f983d5abb4c331605eb57) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#6172](https://github.com/biomejs/biome/issues/6172): Resolved an issue with inferring types for rest parameters. This issue caused rest-parameter types to be incorrect, and in some cases caused extreme performance regressions in files that contained many methods with rest-parameter definitions.

- [#7234](https://github.com/biomejs/biome/pull/7234) [`b7aa111`](https://github.com/biomejs/biome/commit/b7aa111c1c88c33d9c1a35d391b23e79e11dfd43) Thanks [@JeetuSuthar](https://github.com/JeetuSuthar)! - Fixed [#7233](https://github.com/biomejs/biome/issues/7233): The useIndexOf rule now correctly suggests using indexOf() instead of findIndex().

  The diagnostic message was incorrectly recommending Array#findIndex() over Array#indexOf(), when it should recommend the opposite for simple equality checks.

- [#7283](https://github.com/biomejs/biome/pull/7283) [`0b07f45`](https://github.com/biomejs/biome/commit/0b07f4574581d9189c1386c2255caca7338c15e9) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#7236](https://github.com/biomejs/biome/issues/7236). Now Biome correctly migrates JSONC configuration files when they are passed using
  `--config-path`.

- [#7239](https://github.com/biomejs/biome/pull/7239) [`1d643d8`](https://github.com/biomejs/biome/commit/1d643d850120663e16663574ca3457184cdd4c27) Thanks [@minht11](https://github.com/minht11)! - Fixed an issue where Svelte globals ($state and so on) were not properly recognized inside
  `.svelte.test.ts/js` and `.svelte.spec.ts/js` files.

- [#7264](https://github.com/biomejs/biome/pull/7264) [`62fdbc8`](https://github.com/biomejs/biome/commit/62fdbc80154f5a92d54af861c31dd334f25c16fc) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a regression where when using
  `--log-kind-pretty` wasn't working anymore as expected.

- [#7244](https://github.com/biomejs/biome/pull/7244) [`660031b`](https://github.com/biomejs/biome/commit/660031b6707ddeae29388f1d0b4089b64c048e40) Thanks [@JeetuSuthar](https://github.com/JeetuSuthar)! - Fixed [#7225](https://github.com/biomejs/biome/issues/7225): The
  `noExtraBooleanCast` rule now preserves parentheses when removing `Boolean` calls inside negations.

  ```js
  // Before
  !Boolean(b0 && b1);
  // After
  !(b0 && b1); // instead of !b0 && b1
  ```

- [#7298](https://github.com/biomejs/biome/pull/7298) [`46a8e93`](https://github.com/biomejs/biome/commit/46a8e93a65310df566526e6b3fb778455aee2d0b) Thanks [@unvalley](https://github.com/unvalley)! - Fixed [#6695](https://github.com/biomejs/biome/issues/6695): [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) now correctly reports TypeScript parameter properties with modifiers.

  Previously, constructor parameter properties with modifiers like `private` or
  `readonly` were not checked against naming conventions. These properties are now treated consistently with regular class properties.

## 2.2.0

### Minor Changes

- [#5506](https://github.com/biomejs/biome/pull/5506) [`1f8755b`](https://github.com/biomejs/biome/commit/1f8755bfcbcd913be9fc1961b45b5c7ade8695c3) Thanks [@sakai-ast](https://github.com/sakai-ast)! - The
  `noRestrictedImports` rule has been enhanced with a new
  `patterns` option. This option allows for more flexible and powerful import restrictions using gitignore-style patterns.

  You can now define patterns to restrict entire groups of modules. For example, you can disallow imports from any path under
  `import-foo/` except for `import-foo/baz`.

  ```json
  {
    "options": {
      "patterns": [
        {
          "group": ["import-foo/*", "!import-foo/baz"],
          "message": "import-foo is deprecated, except for modules in import-foo/baz."
        }
      ]
    }
  }
  ```

  **Invalid examples**

  ```js
  import foo from "import-foo/foo";
  import bar from "import-foo/bar";
  ```

  **Valid examples**

  ```js
  import baz from "import-foo/baz";
  ```

  Additionally, the `patterns` option introduces
  `importNamePattern` to restrict specific import names using regular expressions. The following example restricts the import names that match
  `x` , `y` or `z` letters from modules under `import-foo/`.

  ```json
  {
    "options": {
      "patterns": [
        {
          "group": ["import-foo/*"],
          "importNamePattern": "[xyz]"
        }
      ]
    }
  }
  ```

  **Invalid examples**

  ```js
  import { x } from "import-foo/foo";
  ```

  **Valid examples**

  ```js
  import { foo } from "import-foo/foo";
  ```

  Furthermore, you can use the
  `invertImportNamePattern` boolean option to reverse this logic. When set to true, only the import names that match the
  `importNamePattern` will be allowed. The following configuration only allows the import names that match `x` , `y` or
  `z` letters from modules under `import-foo/`.

  ```json
  {
    "options": {
      "patterns": [
        {
          "group": ["import-foo/*"],
          "importNamePattern": "[xyz]",
          "invertImportNamePattern": true
        }
      ]
    }
  }
  ```

  **Invalid examples**

  ```js
  import { foo } from "import-foo/foo";
  ```

  **Valid examples**

  ```js
  import { x } from "import-foo/foo";
  ```

- [#6506](https://github.com/biomejs/biome/pull/6506) [`90c5d6b`](https://github.com/biomejs/biome/commit/90c5d6b857f9fb985f919d601872b3650f1e1e5e) Thanks [@nazarhussain](https://github.com/nazarhussain)! - Allow customization of the sort order for different sorting actions. These actions now support a sort option:
  - [`assist/source/useSortedKeys`](https://biomejs.dev/assist/actions/use-sorted-keys/) now has a `sortOrder` option
  - [`assist/source/useSortedAttributes`](https://biomejs.dev/assist/actions/use-sorted-attributes/) now has a
    `sortOrder` option
  - [`assist/source/organizeImports`](https://biomejs.dev/assist/actions/organize-imports/) now has an
    `identifierOrder` option

  For each of these options, the supported values are the same:
  1. **`natural`**. Compares two strings using a natural ASCII order. Uppercase letters come first (e.g.
     `A < a < B < b`) and number are compared in a human way (e.g. `9` < `10`). This is the default value.
  2. **`lexicographic`
     **. Strings are ordered lexicographically by their byte values. This orders Unicode code points based on their positions in the code charts. This is not necessarily the same as “alphabetical” order, which varies by language and locale.

- [#7159](https://github.com/biomejs/biome/pull/7159) [`df3afdf`](https://github.com/biomejs/biome/commit/df3afdf0e29ebb1db6ec4cf6f54ec822c82e38ab) Thanks [@ematipico](https://github.com/ematipico)! - Added the new rule
  `useBiomeIgnoreFolder`. Since v2.2, Biome correctly prevents the indexing and crawling of folders.

  However, the correct pattern has changed. This rule attempts to detect incorrect usage, and promote the new pattern:

  ```diff
  // biome.json
  {
    "files": {
      "includes": [
  -      "!dist/**",
  -      "!**/fixtures/**",
  +      "!dist",
  +      "!**/fixtures",
      ]
    }
  }
  ```

- [#6989](https://github.com/biomejs/biome/pull/6989) [`85b1128`](https://github.com/biomejs/biome/commit/85b11289efbda3061438dfb52ceb186d2142a646) Thanks [@arendjr](https://github.com/arendjr)! - Fixed minor inconsistencies in how
  `files.includes` was being handled.

  Previously, Biome sometimes failed to properly ignore the contents of a folder if you didn't specify the
  `/**` at the end of a glob pattern. This was unfortunate, because it meant we still had to traverse the folder and then apply the glob to every entry inside it.

  This is no longer an issue and we now recommend to ignore folders without using the `/**` suffix.

- [#7118](https://github.com/biomejs/biome/pull/7118) [`a78e878`](https://github.com/biomejs/biome/commit/a78e8781411d151cddec9425763df18ccd2e669b) Thanks [@avshalomt2](https://github.com/avshalomt2)! - Added support for
  `.graphqls` files. Biome can now format and lint GraphQL files that have the extension `.graphqls`

- [#6159](https://github.com/biomejs/biome/pull/6159) [`f02a296`](https://github.com/biomejs/biome/commit/f02a296eae7e3a8dfeddbf1a034e2bb67e8c9c2d) Thanks [@bavalpey](https://github.com/bavalpey)! - Added a new option to Biome's JavaScript formatter,
  `javascript.formatter.operatorLinebreak`, to configure whether long lines should be broken before or after binary operators.

  For example, the following configuration:

  ```json5
  {
    formatter: {
      javascript: {
        operatorLinebreak: "before", // defaults to "after"
      },
    },
  }
  ```

  Will cause this JavaScript file:

  ```js
  const VERY_LONG_CONDITION_1234123412341234123412341234 = false;

  if (
    VERY_LONG_CONDITION_1234123412341234123412341234 &&
    VERY_LONG_CONDITION_1234123412341234123412341234 &&
    VERY_LONG_CONDITION_1234123412341234123412341234 &&
    VERY_LONG_CONDITION_1234123412341234123412341234
  ) {
    console.log("DONE");
  }
  ```

  to be formatted like this:

  ```js
  const VERY_LONG_CONDITION_1234123412341234123412341234 = false;

  if (
    VERY_LONG_CONDITION_1234123412341234123412341234 &&
    VERY_LONG_CONDITION_1234123412341234123412341234 &&
    VERY_LONG_CONDITION_1234123412341234123412341234 &&
    VERY_LONG_CONDITION_1234123412341234123412341234
  ) {
    console.log("DONE");
  }
  ```

- [#7137](https://github.com/biomejs/biome/pull/7137) [`a653a0f`](https://github.com/biomejs/biome/commit/a653a0fb3fa8c6777c9d03829cd88adcfc6b6877) Thanks [@ematipico](https://github.com/ematipico)! - Promoted multiple lint rules from nursery to stable groups and renamed several rules for consistency.

  #### Promoted rules

  The following rules have been promoted from nursery to stable groups:

  ##### CSS
  - Promoted [`noImportantStyles`](https://biomejs.dev/linter/rules/no-important-styles) to the `complexity` group.
  - Promoted [`noUnknownAtRules`](https://biomejs.dev/linter/rules/no-unknown-at-rules) to the `suspicious` group.

  ##### GraphQL
  - Promoted [`useGraphqlNamedOperations`](https://biomejs.dev/linter/rules/use-graphql-named-operations) to the
    `correctness` group.
  - Promoted [`useGraphqlNamingConvention`](https://biomejs.dev/linter/rules/use-graphql-naming-convention) to the
    `style` group.

  ##### JavaScript/TypeScript
  - Promoted [`noExcessiveLinesPerFunction`](https://biomejs.dev/linter/rules/no-excessive-lines-per-function) to the
    `complexity` group.
  - Promoted [`noImplicitCoercions`](https://biomejs.dev/linter/rules/no-implicit-coercions) to the `complexity` group.
  - Promoted [`useIndexOf`](https://biomejs.dev/linter/rules/use-index-of) to the `complexity` group.
  - Promoted [`noGlobalDirnameFilename`](https://biomejs.dev/linter/rules/no-global-dirname-filename) to the
    `correctness` group.
  - Promoted [`noNestedComponentDefinitions`](https://biomejs.dev/linter/rules/no-nested-component-definitions) to the
    `correctness` group.
  - Promoted [`noProcessGlobal`](https://biomejs.dev/linter/rules/no-process-global) to the `correctness` group.
  - Promoted [`noReactPropAssignments`](https://biomejs.dev/linter/rules/no-react-prop-assignments) to the
    `correctness` group.
  - Promoted [`noRestrictedElements`](https://biomejs.dev/linter/rules/no-restricted-elements) to the
    `correctness` group.
  - Promoted [`noSolidDestructuredProps`](https://biomejs.dev/linter/rules/no-solid-destructured-props) to the
    `correctness` group.
  - Promoted [`useJsonImportAttributes`](https://biomejs.dev/linter/rules/use-json-import-attributes) to the
    `correctness` group.
  - Promoted [`useParseIntRadix`](https://biomejs.dev/linter/rules/use-parse-int-radix) to the `correctness` group.
  - Promoted [`useSingleJsDocAsterisk`](https://biomejs.dev/linter/rules/use-single-js-doc-asterisk) to the
    `correctness` group.
  - Promoted [`useUniqueElementIds`](https://biomejs.dev/linter/rules/use-unique-element-ids) to the
    `correctness` group.
  - Promoted [`noAwaitInLoops`](https://biomejs.dev/linter/rules/no-await-in-loops) to the `performance` group.
  - Promoted [`noUnwantedPolyfillio`](https://biomejs.dev/linter/rules/no-unwanted-polyfillio) to the
    `performance` group.
  - Promoted [`useGoogleFontPreconnect`](https://biomejs.dev/linter/rules/use-google-font-preconnect) to the
    `performance` group.
  - Promoted [`useSolidForComponent`](https://biomejs.dev/linter/rules/use-solid-for-component) to the
    `performance` group.
  - Promoted [`noMagicNumbers`](https://biomejs.dev/linter/rules/no-magic-numbers) to the `style` group.
  - Promoted [`useConsistentObjectDefinitions`](https://biomejs.dev/linter/rules/use-consistent-object-definitions) to the
    `style` group.
  - Promoted [`useExportsLast`](https://biomejs.dev/linter/rules/use-exports-last) to the `style` group.
  - Promoted [`useGroupedAccessorPairs`](https://biomejs.dev/linter/rules/use-grouped-accessor-pairs) to the
    `style` group.
  - Promoted [`useNumericSeparators`](https://biomejs.dev/linter/rules/use-numeric-separators) to the `style` group.
  - Promoted [`useObjectSpread`](https://biomejs.dev/linter/rules/use-object-spread) to the `style` group.
  - Promoted [`useReadonlyClassProperties`](https://biomejs.dev/linter/rules/use-readonly-class-properties) to the
    `style` group.
  - Promoted [`useSymbolDescription`](https://biomejs.dev/linter/rules/use-symbol-description) to the `style` group.
  - Promoted [`useUnifiedTypeSignatures`](https://biomejs.dev/linter/rules/use-unified-type-signatures) to the
    `style` group.
  - Promoted [`noBitwiseOperators`](https://biomejs.dev/linter/rules/no-bitwise-operators) to the `suspicious` group.
  - Promoted [`noConstantBinaryExpressions`](https://biomejs.dev/linter/rules/no-constant-binary-expressions) to the
    `suspicious` group.
  - Promoted [`noTsIgnore`](https://biomejs.dev/linter/rules/no-ts-ignore) to the `suspicious` group.
  - Promoted [`noUnassignedVariables`](https://biomejs.dev/linter/rules/no-unassigned-variables) to the
    `suspicious` group.
  - Promoted [`noUselessRegexBackrefs`](https://biomejs.dev/linter/rules/no-useless-regex-backrefs) to the
    `suspicious` group.
  - Promoted [`noUselessEscapeInString`](https://biomejs.dev/linter/rules/no-useless-escape-in-string/) to the
    `suspicious` group.
  - Promoted [`useIterableCallbackReturn`](https://biomejs.dev/linter/rules/use-iterable-callback-return/) to the
    `suspicious` group.
  - Promoted [`useStaticResponseMethods`](https://biomejs.dev/linter/rules/use-static-response-methods) to the
    `suspicious` group.

  #### Renamed rules

  The following rules have been renamed during promotion. The migration tool will automatically update your configuration:
  - Renamed `noAwaitInLoop` to [`noAwaitInLoops`](https://biomejs.dev/linter/rules/no-await-in-loops).
  - Renamed `noConstantBinaryExpression` to [`noConstantBinaryExpressions`](https://biomejs.dev/linter/rules/no-constant-binary-expressions).
  - Renamed `noDestructuredProps` to [`noSolidDestructuredProps`](https://biomejs.dev/linter/rules/no-solid-destructured-props).
  - Renamed `noImplicitCoercion` to [`noImplicitCoercions`](https://biomejs.dev/linter/rules/no-implicit-coercions).
  - Renamed `noReactPropAssign` to [`noReactPropAssignments`](https://biomejs.dev/linter/rules/no-react-prop-assignments).
  - Renamed `noUnknownAtRule` to [`noUnknownAtRules`](https://biomejs.dev/linter/rules/no-unknown-at-rules).
  - Renamed `noUselessBackrefInRegex` to [`noUselessRegexBackrefs`](https://biomejs.dev/linter/rules/no-useless-regex-backrefs).
  - Renamed `useAdjacentGetterSetter` to [`useGroupedAccessorPairs`](https://biomejs.dev/linter/rules/use-grouped-accessor-pairs).
  - Renamed `useConsistentObjectDefinition` to [`useConsistentObjectDefinitions`](https://biomejs.dev/linter/rules/use-consistent-object-definitions).
  - Renamed `useConsistentResponse` to [`useStaticResponseMethods`](https://biomejs.dev/linter/rules/use-static-response-methods).
  - Renamed `useForComponent` to [`useSolidForComponent`](https://biomejs.dev/linter/rules/use-solid-for-component).
  - Renamed `useJsonImportAttribute` to [`useJsonImportAttributes`](https://biomejs.dev/linter/rules/use-json-import-attributes).
  - Renamed `useNamedOperation` to [`useGraphqlNamedOperations`](https://biomejs.dev/linter/rules/use-graphql-named-operations).
  - Renamed `useNamingConvention` to [`useGraphqlNamingConvention`](https://biomejs.dev/linter/rules/use-graphql-naming-convention).
  - Renamed `useUnifiedTypeSignature` to [`useUnifiedTypeSignatures`](https://biomejs.dev/linter/rules/use-unified-type-signatures).

  Configuration files using the old rule names will need to be updated. Use the migration tool to automatically update your configuration:

  ```bash
  biome migrate --write
  ```

- [#7159](https://github.com/biomejs/biome/pull/7159) [`df3afdf`](https://github.com/biomejs/biome/commit/df3afdf0e29ebb1db6ec4cf6f54ec822c82e38ab) Thanks [@ematipico](https://github.com/ematipico)! - Added the new rule
  `noBiomeFirstException`. This rule prevents the incorrect usage of patterns inside `files.includes`.

  This rule catches if the first element of the array contains `!`. This mistake will cause Biome to analyze no files:

  ```json5
  // biome.json
  {
    files: {
      includes: ["!dist/**"], // this is an error
    },
  }
  ```

- [#6923](https://github.com/biomejs/biome/pull/6923) [`0589f08`](https://github.com/biomejs/biome/commit/0589f085ee444418c742f5e5eb7fae0522d83ea0) Thanks [@ptkagori](https://github.com/ptkagori)! - Added Qwik Domain to Biome

  This release introduces **Qwik domain support
  ** in Biome, enabling Qwik developers to use Biome as a linter and formatter for their projects.
  - Added the Qwik domain infrastructure to Biome.
  - Enabled the following rules for Qwik:
    - [`useJsxKeyInIterable`](https://biomejs.dev/linter/rules/use-jsx-key-in-iterable)
    - [`noReactSpecificProps`](https://biomejs.dev/linter/rules/no-react-specific-props)

- [#6989](https://github.com/biomejs/biome/pull/6989) [`85b1128`](https://github.com/biomejs/biome/commit/85b11289efbda3061438dfb52ceb186d2142a646) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#6965](https://github.com/biomejs/biome/issues/6965): Implemented smarter scanner for project rules.

  Previously, if project rules were enabled, Biome's scanner would scan all dependencies regardless of whether they were used by/reachable from source files or not. While this worked for a first version, it was far from optimal.

  The new scanner first scans everything listed under the
  `files.includes` setting, and then descends into the dependencies that were discovered there, including transitive dependencies. This has three main advantages:
  - Dependencies that are not reachable from your source files don't get indexed.
  - Dependencies that have multiple type definitions, such as those with separate definitions for CommonJS and ESM imports, only have the relevant definitions indexed.
  - If `vcs.useIgnoreFile` is enabled, `.gitignore` gets respected as well. Assuming you have folders such as
    `build/` or `dist/` configured there, those will be automatically ignored by the scanner.

  The change in the scanner also has a more nuanced impact: Previously, if you used
  `files.includes` to ignore a file in an included folder, the scanner would still index this file. Now the file is fully ignored,
  _unless you import it_.

  As a user you should notice better scanner performance (if you have project rules enabled), and hopefully you need to worry less about configuring [`files.experimentalScannerIgnores`](https://biomejs.dev/reference/configuration/#filesexperimentalscannerignores). Eventually our goal is still to deprecate that setting, so if you're using it today, we encourage you to see which ignores are still necessary there, and whether you can achieve the same effect by ignoring paths using
  `files.includes` instead.

  None of these changes affect the scanner if no project rules are enabled.

- [#6731](https://github.com/biomejs/biome/pull/6731) [`d6a05b5`](https://github.com/biomejs/biome/commit/d6a05b5fa9358a5b1689b326724eaa7e2a86468d) Thanks [@ematipico](https://github.com/ematipico)! - The
  `--reporter=summary` has been greatly enhanced. It now shows the list of files that contains violations, the files shown are clickable and can be opened from the editor.

  Below an example of the new version:

  ```
  reporter/parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    i The following files have parsing errors.

    - index.css

  reporter/format ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    i The following files needs to be formatted.

    - index.css
    - index.ts
    - main.ts

  reporter/violations ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    i Some lint rules or assist actions reported some violations.

    Rule Name                                        Diagnostics

    lint/correctness/noUnknownFunction               14 (2 error(s), 12 warning(s), 0 info(s))
    lint/suspicious/noImplicitAnyLet                 16 (12 error(s), 4 warning(s), 0 info(s))
    lint/suspicious/noDoubleEquals                   8 (8 error(s), 0 warning(s), 0 info(s))
    assist/source/organizeImports                    2 (2 error(s), 0 warning(s), 0 info(s))
    lint/suspicious/noRedeclare                      12 (12 error(s), 0 warning(s), 0 info(s))
    lint/suspicious/noDebugger                       8 (8 error(s), 0 warning(s), 0 info(s))

  ```

- [#6896](https://github.com/biomejs/biome/pull/6896) [`527db7f`](https://github.com/biomejs/biome/commit/527db7f7c142f8c95c6d4513603530220a4cc95c) Thanks [@ematipico](https://github.com/ematipico)! - Added new functions to the
  `@biomejs/wasm-*` packages:
  - `fileExists`: returns whether the input file exists in the workspace.
  - `isPathIgnored`: returns whether the input path is ignored.
  - `updateModuleGraph`: updates the internal module graph of the input path.
  - `getModuleGraph`: it returns a serialized version of the internal module graph.
  - `scanProject`: scans the files and directories in the project to build the internal module graph.

- [#6398](https://github.com/biomejs/biome/pull/6398) [`d1a315d`](https://github.com/biomejs/biome/commit/d1a315d19e970341c8e6582c1f6f80b42c77ecb5) Thanks [@josh-](https://github.com/josh-)! - Added support for tracking stable results in user-provided React hooks that return objects to [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) to compliment existing support for array return values. For example:

  ```json5
  // biome.json
  {
    // rule options
    useExhaustiveDependencies: {
      level: "error",
      options: {
        hooks: [
          {
            name: "useCustomHook",
            stableResult: ["setMyState"],
          },
        ],
      },
    },
  }
  ```

  This will allow the following to be validated:

  ```js
  const { myState, setMyState } = useCustomHook();
  const toggleMyState = useCallback(() => {
    setMyState(!myState);
  }, [myState]); // Only `myState` needs to be specified here.
  ```

- [#7201](https://github.com/biomejs/biome/pull/7201) [`2afaa49`](https://github.com/biomejs/biome/commit/2afaa49b814b12b52a1ffa06ed6c67d21ea57e1a) Thanks [@Conaclos](https://github.com/Conaclos)! - Implemented [#7174](https://github.com/biomejs/biome/issues/7174). [`useConst`](https://biomejs.dev/linter/rules/use-const/) no longer reports variables that are read before being written.

  Previously,
  `useConst` reported uninitialised variables that were read in an inner function before being written, as shown in the following example:

  ```js
  let v;
  function f() {
    return v;
  }
  v = 0;
  ```

  This can produce false positives in the case where `f` is called before
  `v` has been written, as in the following code:

  ```js
  let v;
  function f() {
    return v;
  }
  console.log(f()); // print `undefined`
  v = 0;
  ```

  Although this is an expected behavior of the original implementation, we consider it problematic since the rule’s fix is marked as safe. To avoid false positives like this, the rule now ignores the previous examples. However, this has the disadvantage of resulting in false negatives, such as not reporting the first example.

### Patch Changes

- [#7156](https://github.com/biomejs/biome/pull/7156) [`137d111`](https://github.com/biomejs/biome/commit/137d1118e4598a0ef2c0104e45cb00a8bf179199) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#7152](https://github.com/biomejs/biome/issues/7152). Now the rule
  `noDuplicateFontNames` correctly detects font names with spaces e.g.
  `Liberation Mono`. The diagnostic of the rule now points to the first instances of the repeated font.

  The following example doesn't trigger the rule anymore:

  ```css
  c {
    font-family:
      SF Mono,
      Liberation Mono,
      sans-serif;
  }
  d {
    font:
      1em SF Mono,
      Liberation Mono,
      sans-serif;
  }
  ```

- [#6907](https://github.com/biomejs/biome/pull/6907) [`7331bb9`](https://github.com/biomejs/biome/commit/7331bb9979143c355d861eadcde4f075e6b70910) Thanks [@ematipico](https://github.com/ematipico)! - Added a new
  **experimental option** that allows parsing of `.html` files that contain interpolation syntax.

  ```json5
  // biome.json
  {
    html: {
      // This is the new, experimental option.
      parser: {
        interpolation: true,
      },
    },
  }
  ```

  ```html
  <h1>{{ $title }}</h1>
  ```

- [#7124](https://github.com/biomejs/biome/pull/7124) [`3f436b8`](https://github.com/biomejs/biome/commit/3f436b84bb62320c16c1ca1ac5b419e4d9abefb3) Thanks [@Jayllyz](https://github.com/Jayllyz)! - Added the rule [`useMaxParams`](https://biomejs.dev/linter/rules/use-max-params).

  This rule enforces a maximum number of parameters for functions to improve code readability and maintainability. Functions with many parameters are difficult to read, understand, and maintain because they require memorizing parameter order and types.

  ```js
  // Invalid - too many parameters (default max: 4)
  function processData(
    name,
    age,
    email,
    phone,
    address,
    city,
    country,
    zipCode,
  ) {
    // ...
  }

  // Valid - within parameter limit
  function processData(userData) {
    const { name, age, email, phone, address, city, country, zipCode } =
      userData;
    // ...
  }

  function calculateSum(a, b, c) {
    return a + b + c;
  }
  ```

- [#7161](https://github.com/biomejs/biome/pull/7161) [`1a14a59`](https://github.com/biomejs/biome/commit/1a14a59c52f9389220e7682de5632b7d7291a4e4) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#7160](https://github.com/biomejs/biome/issues/7160). Now Biome correctly computes ignored files when using
  `formatter.includes`, `linter.includes` and `assist.includes` inside nested configurations that use `"extends": "//"`.

- [#7081](https://github.com/biomejs/biome/pull/7081) [`a081bbe`](https://github.com/biomejs/biome/commit/a081bbef37a4b329ace1cb0eb88c36f6c6162af1) Thanks [@Jayllyz](https://github.com/Jayllyz)! - Added the rule [`noNextAsyncClientComponent`](https://biomejs.dev/linter/rules/no-next-async-client-component).

  This rule prevents the use of async functions for client components in Next.js applications. Client components marked with "use client" directive should not be async as this can cause hydration mismatches, break component rendering lifecycle, and lead to unexpected behavior with React's concurrent features.

  ```jsx
  "use client";

  // Invalid - async client component
  export default async function MyComponent() {
    return <div>Hello</div>;
  }

  // Valid - synchronous client component
  export default function MyComponent() {
    return <div>Hello</div>;
  }
  ```

- [#7171](https://github.com/biomejs/biome/pull/7171) [`5241690`](https://github.com/biomejs/biome/commit/5241690265c584cfb4e6827e82a496801f039197) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#7162](https://github.com/biomejs/biome/issues/7162): The
  `noUndeclaredDependencies` rule now considers a type-only import as a dev dependency.

  For example, the following code is no longer reported:

  **`package.json`**:

  ```json
  {
    "devDependencies": {
      "type-fest": "*"
    }
  }
  ```

  **`foo.ts`**:

  ```ts
  import type { SetRequired } from "type-fest";
  ```

  Note that you still need to declare the package in the `devDependencies` section in `package.json`.

## 2.1.4

### Patch Changes

- [#7121](https://github.com/biomejs/biome/pull/7121) [`b9642ab`](https://github.com/biomejs/biome/commit/b9642abc6d05135180f4243df30524cf40ba12df) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#7111](https://github.com/biomejs/biome/issues/7111): Imported symbols using aliases are now correctly recognised.

- [#7103](https://github.com/biomejs/biome/pull/7103) [`80515ec`](https://github.com/biomejs/biome/commit/80515ecad8cc272feeae4c17762d3b150acd88e7) Thanks [@omasakun](https://github.com/omasakun)! - Fixed [#6933](https://github.com/biomejs/biome/issues/6933) and [#6994](https://github.com/biomejs/biome/issues/6994).

  When the values of private member assignment expressions, increment expressions, etc. are used, those private members are no longer marked as unused.

- [#6887](https://github.com/biomejs/biome/pull/6887) [`0cc38f5`](https://github.com/biomejs/biome/commit/0cc38f59cd9ddf0fdcd12d6f8cb3642743cc4406) Thanks [@ptkagori](https://github.com/ptkagori)! - Added the [`noQwikUseVisibleTask`](https://biomejs.dev/linter/rules/no-qwik-use-visible-task) rule to Qwik.

  This rule is intended for use in Qwik applications to warn about the use of
  `useVisibleTask$()` functions which require careful consideration before use.

  **Invalid:**

  ```js
  useVisibleTask$(() => {
    console.log("Component is visible");
  });
  ```

  **Valid:**

  ```js
  useTask$(() => {
    console.log("Task executed");
  });
  ```

- [#7084](https://github.com/biomejs/biome/pull/7084) [`50ca155`](https://github.com/biomejs/biome/commit/50ca1553f08348ab1e92dc7cf04013c85ff743a4) Thanks [@ematipico](https://github.com/ematipico)! - Added the new nursery rule
  `noUnnecessararyConditions`, which detects whenever some conditions don't change during the life cycle of the program, and truthy or false, hence deemed redundant.

  For example, the following snippets will trigger the rule:

  ```js
  // Always truthy literal conditions
  if (true) {
    console.log("always runs");
  }
  ```

  ```ts
  // Unnecessary condition on constrained string type
  function foo(arg: "bar" | "baz") {
    if (arg) {
      // This check is unnecessary
    }
  }
  ```

- [#6887](https://github.com/biomejs/biome/pull/6887) [`0cc38f5`](https://github.com/biomejs/biome/commit/0cc38f59cd9ddf0fdcd12d6f8cb3642743cc4406) Thanks [@ptkagori](https://github.com/ptkagori)! - Added the [`useImageSize`](https://biomejs.dev/linter/rules/use-image-size) rule to Biome.

  The `useImageSize` rule enforces the use of width and height attributes on
  `<img>` elements for performance reasons. This rule is intended to prevent layout shifts and improve Core Web Vitals by ensuring images have explicit dimensions.

  **Invalid:**

  ```jsx
  <img src="/image.png" />
  <img src="https://example.com/image.png" />
  <img src="/image.png" width="200" />
  <img src="/image.png" height="200" />
  ```

  **Valid:**

  ```jsx
  <img width="200" height="600" src="/static/images/portrait-01.webp" />
  <img width="100" height="100" src="https://example.com/image.png" />
  ```

- [#6887](https://github.com/biomejs/biome/pull/6887) [`0cc38f5`](https://github.com/biomejs/biome/commit/0cc38f59cd9ddf0fdcd12d6f8cb3642743cc4406) Thanks [@ptkagori](https://github.com/ptkagori)! - Added the [`useAnchorHref`](https://biomejs.dev/linter/rules/use-anchor-href) rule to Biome.

  The `useAnchorHref` rule enforces the presence of an `href` attribute on
  `<a>` elements in JSX. This rule is intended to ensure that anchor elements are always valid and accessible.

  **Invalid:**

  ```jsx
  <a>Link</a>
  ```

  ```jsx
  <a target="_blank">External</a>
  ```

  **Valid:**

  ```jsx
  <a href="/home">Home</a>
  ```

  ```jsx
  <a href="https://example.com" target="_blank">
    External
  </a>
  ```

- [#7100](https://github.com/biomejs/biome/pull/7100) [`29fcb05`](https://github.com/biomejs/biome/commit/29fcb0540ed817d92a3f663132b658541706765b) Thanks [@Jayllyz](https://github.com/Jayllyz)! - Added the rule [`noNonNullAssertedOptionalChain`](https://biomejs.dev/linter/rules/no-non-null-asserted-optional-chain).

  This rule prevents the use of non-null assertions (`!`) immediately after optional chaining expressions (
  `?.`). Optional chaining is designed to safely handle nullable values by returning
  `undefined` when the chain encounters `null` or
  `undefined`. Using a non-null assertion defeats this purpose and can lead to runtime errors.

  ```ts
  // Invalid - non-null assertion after optional chaining
  obj?.prop!;
  obj?.method()!;
  obj?.[key]!;
  obj?.prop!;

  // Valid - proper optional chaining usage
  obj?.prop;
  obj?.method();
  obj?.prop ?? defaultValue;
  obj!.prop?.method();
  ```

- [#7129](https://github.com/biomejs/biome/pull/7129) [`9f4538a`](https://github.com/biomejs/biome/commit/9f4538ab8bad8a974b8e408641b1fd4770d26c79) Thanks [@drwpow](https://github.com/drwpow)! - Removed option, combobox, listbox roles from [useSemanticElements](https://biomejs.dev/linter/rules/use-semantic-elements/) suggestions

- [#7106](https://github.com/biomejs/biome/pull/7106) [`236deaa`](https://github.com/biomejs/biome/commit/236deaadca077051f6e2ef01cfdbbc55cc1c3d78) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#6985](https://github.com/biomejs/biome/issues/6985): Inference of return types no longer mistakenly picks up return types of nested functions.

- [#7102](https://github.com/biomejs/biome/pull/7102) [`d3118c6`](https://github.com/biomejs/biome/commit/d3118c6ac3bba0ca29251fa7fc5ba36a9e4456b0) Thanks [@omasakun](https://github.com/omasakun)! - Fixed [#7101](https://github.com/biomejs/biome/issues/7101): [`noUnusedPrivateClassMembers`](https://biomejs.dev/linter/rules/no-unused-private-class-members/) now handles members declared as part of constructor arguments:
  1. If a class member defined in a constructor argument is only used within the constructor, it removes the
     `private` modifier and makes it a plain method argument.
  1. If it is not used at all, it will prefix it with an underscore, similar to `noUnusedFunctionParameter`.

- [#7104](https://github.com/biomejs/biome/pull/7104) [`5395297`](https://github.com/biomejs/biome/commit/53952972cd5786cfdcc3deda0c226d6488ef1aee) Thanks [@harxki](https://github.com/harxki)! - Reverting to prevent regressions around ref handling

- [#7143](https://github.com/biomejs/biome/pull/7143) [`1a6933a`](https://github.com/biomejs/biome/commit/1a6933aaf2c5b57d70a60d607b5cab68d532eeb4) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#6799](https://github.com/biomejs/biome/issues/6799): The [`noImportCycles`](https://biomejs.dev/linter/rules/no-import-cycles/) rule now ignores type-only imports if the new
  `ignoreTypes` option is enabled (enabled by default).

  > [!WARNING]
  > **Breaking Change**: The
  > `noImportCycles` rule no longer detects import cycles that include one or more type-only imports by default.
  > To keep the old behaviour, you can turn off the `ignoreTypes` option explicitly:
  >
  > ```json
  > {
  >   "linter": {
  >     "rules": {
  >       "nursery": {
  >         "noImportCycles": {
  >           "options": {
  >             "ignoreTypes": false
  >           }
  >         }
  >       }
  >     }
  >   }
  > }
  > ```

- [#7099](https://github.com/biomejs/biome/pull/7099) [`6cc84cb`](https://github.com/biomejs/biome/commit/6cc84cb547480f83119d2cba5542e2d2afc65b4d) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#7062](https://github.com/biomejs/biome/issues/7062): Biome now correctly considers extended configs when determining the mode for the scanner.

- [#6887](https://github.com/biomejs/biome/pull/6887) [`0cc38f5`](https://github.com/biomejs/biome/commit/0cc38f59cd9ddf0fdcd12d6f8cb3642743cc4406) Thanks [@ptkagori](https://github.com/ptkagori)! - Added the [`useQwikClasslist`](https://biomejs.dev/linter/rules/use-qwik-classlist) rule to Biome.

  This rule is intended for use in Qwik applications to encourage the use of the built-in
  `class` prop (which accepts a string, object, or array) instead of the `classnames` utility library.

  **Invalid:**

  ```jsx
  <div class={classnames({ active: true, disabled: false })} />
  ```

  **Valid:**

  ```jsx
  <div classlist={{ active: true, disabled: false }} />
  ```

- [#7019](https://github.com/biomejs/biome/pull/7019) [`57c15e6`](https://github.com/biomejs/biome/commit/57c15e6df5b6257ffb9f69d7614c3455a1f5c870) Thanks [@fireairforce](https://github.com/fireairforce)! - Added support in the JS parser for
  `import source`(a [stage3 proposal](https://github.com/tc39/proposal-source-phase-imports)). The syntax looks like:

  ```ts
  import source foo from "<specifier>";
  ```

- [#7053](https://github.com/biomejs/biome/pull/7053) [`655049e`](https://github.com/biomejs/biome/commit/655049e9e38f536b33fff6d7b160299f0b446908) Thanks [@jakeleventhal](https://github.com/jakeleventhal)! - Added the [`useConsistentTypeDefinitions`](https://biomejs.dev/linter/rules/use-consistent-type-definitions) rule.

  This rule enforces consistent usage of either `interface` or `type` for object type definitions in TypeScript.

  The rule accepts an option to specify the preferred style:
  - `interface` (default): Prefer using `interface` for object type definitions
  - `type`: Prefer using `type` for object type definitions

  Examples:

  ```ts
  // With default option (interface)
  // ❌ Invalid
  type Point = { x: number; y: number };

  // ✅ Valid
  interface Point {
    x: number;
    y: number;
  }

  // With option { style: "type" }
  // ❌ Invalid
  interface Point {
    x: number;
    y: number;
  }

  // ✅ Valid
  type Point = { x: number; y: number };
  ```

  The rule will automatically fix simple cases where conversion is straightforward.

## 2.1.3

### Patch Changes

- [#7057](https://github.com/biomejs/biome/pull/7057) [`634a667`](https://github.com/biomejs/biome/commit/634a667ac8e9f74a4633895eab4bd4695ffffa1d) Thanks [@mdevils](https://github.com/mdevils)! - Added the rule [`noVueReservedKeys`](https://biomejs.dev/linter/rules/no-vue-reserved-keys/), which prevents the use of reserved Vue keys.

  It prevents the use of Vue reserved keys such as those starting with like `$el`, `$data`,
  `$props`) and keys starting with
  `\_` in data properties, which can cause conflicts and unexpected behavior in Vue components.

  ##### Invalid example

  ```vue
  <script>
  export default {
    data: {
      $el: "",
      _foo: "bar",
    },
  };
  </script>
  ```

  ```vue
  <script>
  export default {
    computed: {
      $data() {
        return this.someData;
      },
    },
  };
  </script>
  ```

  ##### Valid examples

  ```vue
  <script>
  export default {
    data() {
      return {
        message: "Hello Vue!",
        count: 0,
      };
    },
  };
  </script>
  ```

  ```vue
  <script>
  export default {
    computed: {
      displayMessage() {
        return this.message;
      },
    },
  };
  </script>
  ```

- [#6941](https://github.com/biomejs/biome/pull/6941) [`734d708`](https://github.com/biomejs/biome/commit/734d708bd84f32d72e5972cc27c194d5da46a3c0) Thanks [@JamBalaya56562](https://github.com/JamBalaya56562)! - Added
  `@eslint-react/no-nested-component-definitions` as a rule source for
  `noNestedComponentDefinitions`. Now it will get picked up by `biome migrate --eslint`.

- [#6463](https://github.com/biomejs/biome/pull/6463) [`0a16d54`](https://github.com/biomejs/biome/commit/0a16d54c2cffbf13c5144b53021923734f1c234e) Thanks [@JamBalaya56562](https://github.com/JamBalaya56562)! - Fixed a website link for the
  `useComponentExportOnlyModules` linter rule to point to the correct URL.

- [#6944](https://github.com/biomejs/biome/pull/6944) [`e53f2fe`](https://github.com/biomejs/biome/commit/e53f2fe03827a8dcad2184178ecfaee0e35af992) Thanks [@sterliakov](https://github.com/sterliakov)! - Fixed [#6910](https://github.com/biomejs/biome/issues/6910): Biome now ignores type casts and assertions when evaluating numbers for
  `noMagicNumbers` rule.

- [#6991](https://github.com/biomejs/biome/pull/6991) [`476cd55`](https://github.com/biomejs/biome/commit/476cd55e4e5b1b03335e14c65ad01b2bbb4b8d42) Thanks [@denbezrukov](https://github.com/denbezrukov)! - Fixed [#6973](https://github.com/biomejs/biome/issues/6973): Add support for parsing the :active-view-transition-type() pseudo-class

  ```css
  :active-view-transition-type(first second) {
  }
  ```

- [#6992](https://github.com/biomejs/biome/pull/6992) [`0b1e194`](https://github.com/biomejs/biome/commit/0b1e19474e323c7354fccff0c5654d47024c7b91) Thanks [@ematipico](https://github.com/ematipico)! - Added a new JSON rule called
  `noQuickfixBiome`, which disallow the use of code action `quickfix.biome` inside code editor settings.

- [#6943](https://github.com/biomejs/biome/pull/6943) [`249306d`](https://github.com/biomejs/biome/commit/249306db32b6a912f39d2c88a1b0d702b8b97a9b) Thanks [@JamBalaya56562](https://github.com/JamBalaya56562)! - Fixed
  `@vitest/eslint-plugin` source url.

- [#6947](https://github.com/biomejs/biome/pull/6947) [`4c7ed0f`](https://github.com/biomejs/biome/commit/4c7ed0fda858424a21fb1766270aaa74838a46a1) Thanks [@JamBalaya56562](https://github.com/JamBalaya56562)! - Fixed ESLint migration for the rule
  `prefer-for` from `eslint-plugin-solid` to Biome's `useForComponent`.

- [#6976](https://github.com/biomejs/biome/pull/6976) [`72ebadc`](https://github.com/biomejs/biome/commit/72ebadce0e192932d237d9a31c45cb230c8bbd91) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#6692](https://github.com/biomejs/biome/issues/6692): The rules
  `noUnusedVariables` and
  `noUnusedFunctionParameters` no longer cause an infinite loop when the suggested name is not applicable (e.g. the suggested name is already declared in the scope).

- [#6990](https://github.com/biomejs/biome/pull/6990) [`333f5d0`](https://github.com/biomejs/biome/commit/333f5d0a11dc1b2c029c657905bc73d3daf72477) Thanks [@rvanlaarhoven](https://github.com/rvanlaarhoven)! - Fixed the documentation URL for
  `lint/correctness/noUnknownPseudoClass`

- [#7000](https://github.com/biomejs/biome/pull/7000) [`4021165`](https://github.com/biomejs/biome/commit/402116575ef570da02ccbce521645a3975b3e8ce) Thanks [@harxki](https://github.com/harxki)! - Fixed [#6795](https://github.com/biomejs/biome/issues/6795):
  `noUnassignedVariables` now correctly recognizes variables used in JSX `ref` attributes.

- [#7044](https://github.com/biomejs/biome/pull/7044) [`b091ddf`](https://github.com/biomejs/biome/commit/b091ddf73d323a6929b9601f05ede7e91e4d4cbb) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6622](https://github.com/biomejs/biome/issues/6622), now the rule
  `useSemanticElements` works for JSX self-closing elements too.

- [#7014](https://github.com/biomejs/biome/pull/7014) [`c4864e8`](https://github.com/biomejs/biome/commit/c4864e85ebbb1bbfbb8274c59bb6af9413d8f157) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#6516](https://github.com/biomejs/biome/issues/6516): The
  `biome migrate` command no longer break the member list with trailing comments.

- [#6979](https://github.com/biomejs/biome/pull/6979) [`29cb6da`](https://github.com/biomejs/biome/commit/29cb6da9a1e8f20af59f5e681b9d2aa1a23e8b27) Thanks [@unvalley](https://github.com/unvalley)! - Fixed [#6767](https://github.com/biomejs/biome/issues/6767):
  `useSortedClasses` now correctly removes leading and trailing whitespace in className.

  Previously, trailing spaces in className were not fully removed.

  ```jsx
  // Think we have this code:
  <div className="text-sm font-bold            " />

  // Before: applied fix, but a trailing space was preserved
  <div className="font-bold text-sm " />

  // After: applied fix, trailing spaces removed
  <div className="font-bold text-sm" />
  ```

- [#7055](https://github.com/biomejs/biome/pull/7055) [`ee4828d`](https://github.com/biomejs/biome/commit/ee4828da9be5898c67b7feabfaaa296ad172109f) Thanks [@dyc3](https://github.com/dyc3)! - Added the nursery rule [`useReactFunctionComponents`](https://biomejs.dev/linter/rules/use-react-function-components/). This rule enforces the preference to use function components instead of class components.

  Valid:

  ```jsx
  function Foo() {
    return <div>Hello, world!</div>;
  }
  ```

  Invalid:

  ```jsx
  class Foo extends React.Component {
    render() {
      return <div>Hello, world!</div>;
    }
  }
  ```

- [#6924](https://github.com/biomejs/biome/pull/6924) [`2d21be9`](https://github.com/biomejs/biome/commit/2d21be9437fd77a1c534a1ea156d9a9421c17d30) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#113](https://github.com/biomejs/biome-zed/issues/113), where the Biome Language Server didn't correctly update the diagnostics when the configuration file is modified in the editor. Now the diagnostics are correctly updated every time the configuration file is modified and saved.

- [#6931](https://github.com/biomejs/biome/pull/6931) [`e6b2380`](https://github.com/biomejs/biome/commit/e6b238063f92bc95d951e3a78dac42408d0814c0) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#6915](https://github.com/biomejs/biome/issues/6915):
  `useHookAtTopLevel` no longer hangs when rules call themselves recursively.

- [#7012](https://github.com/biomejs/biome/pull/7012) [`01c0ab4`](https://github.com/biomejs/biome/commit/01c0ab43ad7785e093e5069dda1d5e6969958bf8) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#5837](https://github.com/biomejs/biome/issues/5837): Invalid suppression comments such as
  `biome-ignore-all-start` or `biome-ignore-all-end` no longer causes a panic.

- [#6949](https://github.com/biomejs/biome/pull/6949) [`48462f8`](https://github.com/biomejs/biome/commit/48462f81ba4e98a95236365a5f9759fc41c045d7) Thanks [@fireairforce](https://github.com/fireairforce)! - Support parse
  `import defer`(which is a [stage3 proposal](https://github.com/tc39/proposal-defer-import-eval)). The syntax look like this:

  ```ts
  import defer * as foo from "<specifier>";
  ```

- [#6938](https://github.com/biomejs/biome/pull/6938) [`5feb5a6`](https://github.com/biomejs/biome/commit/5feb5a675adb246b04b1540cba16ff1c5fd49cb1) Thanks [@vladimir-ivanov](https://github.com/vladimir-ivanov)! - Fixed [#6919](https://github.com/biomejs/biome/issues/6919) and [#6920](https://github.com/biomejs/biome/issues/6920):
  `useReadonlyClassProperties` now does checks for mutations in async class methods.

  Example:

  ```typescript
  class Counter3 {
    private counter: number;
    async count() {
      this.counter = 1;
      const counterString = `${this.counter++}`;
    }
  }
  ```

- [#6942](https://github.com/biomejs/biome/pull/6942) [`cfda528`](https://github.com/biomejs/biome/commit/cfda528169dcceb8422a0488b39a3b1b27a24645) Thanks [@sterliakov](https://github.com/sterliakov)! - Fixed [#6939](https://github.com/biomejs/biome/issues/6939). Biome now understands
  `this` binding in classes outside of methods.

## 2.1.2

### Patch Changes

- [#6865](https://github.com/biomejs/biome/pull/6865) [`b35bf64`](https://github.com/biomejs/biome/commit/b35bf6448fb1950c922e627254588e96748e287f) Thanks [@denbezrukov](https://github.com/denbezrukov)! - Fix [#6485](https://github.com/biomejs/biome/issues/6485): Handle multiple semicolons correctly in blocks (#6485)

  ```css
  div {
    box-sizing: border-box;
    color: red;
  }
  ```

- [#6798](https://github.com/biomejs/biome/pull/6798) [`3579ffa`](https://github.com/biomejs/biome/commit/3579ffaae4e86835b001fee4ab7dd8aabb03ae54) Thanks [@dyc3](https://github.com/dyc3)! - Fixed [#6762](https://github.com/biomejs/biome/issues/6762), Biome now knows that
  `~/.config/zed/settings.json` and `~/.config/Code/User/settings.json` allows comments by default.

- [#6839](https://github.com/biomejs/biome/pull/6839) [`4cd62d8`](https://github.com/biomejs/biome/commit/4cd62d8ae2e5cb24d6f308e05b38003486294548) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6838](https://github.com/biomejs/biome/issues/6838), where the Biome File Watcher incorrectly watched and stored ignored files, causing possible memory leaks when those files were dynamically created (e.g. built files).

- [#6879](https://github.com/biomejs/biome/pull/6879) [`0059cd9`](https://github.com/biomejs/biome/commit/0059cd9b5e6ba33cabb5e153bd03e2041effb0cd) Thanks [@denbezrukov](https://github.com/denbezrukov)! - Refactor: remove one level of indirection for CSS declarations with semicolon Previously, accessing a declaration from a list required an extra step:

  ```rust
  item
  .as_any_css_declaration_with_semicolon()
  .as_css_declaration_with_semicolon()
  ```

  Now, it can be done directly with:

  ```rust
  item.as_css_declaration_with_semicolon()
  ```

- [#6839](https://github.com/biomejs/biome/pull/6839) [`4cd62d8`](https://github.com/biomejs/biome/commit/4cd62d8ae2e5cb24d6f308e05b38003486294548) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a bug where the Biome Language Server didn't correctly ignore specific files when
  `vcs.useIgnoreFile` is set to `true`.

- [#6884](https://github.com/biomejs/biome/pull/6884) [`5ff50f8`](https://github.com/biomejs/biome/commit/5ff50f8291ca6f8f59fccfc326c8f0bdc3127842) Thanks [@arendjr](https://github.com/arendjr)! - Improved the performance of
  `noImportCycles` by ~30%.

- [#6903](https://github.com/biomejs/biome/pull/6903) [`241dd9e`](https://github.com/biomejs/biome/commit/241dd9e487226fc58b4ceceaf3164e36d8e22d3b) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#6829](https://github.com/biomejs/biome/issues/6829): Fixed a false positive reported by
  `useImportExtensions` when importing a `.js` file that had a matching `.d.ts` file in the same folder.

- [#6846](https://github.com/biomejs/biome/pull/6846) [`446112e`](https://github.com/biomejs/biome/commit/446112e79d695c50ca9cc9f2d25c91cf03115f50) Thanks [@darricheng](https://github.com/darricheng)! - Fixed an issue where biome was using the wrong string quotes when the classes string has quotes, resulting in invalid code after applying the fix.

- [#6823](https://github.com/biomejs/biome/pull/6823) [`eebc48e`](https://github.com/biomejs/biome/commit/eebc48e0120958a39186f510278e1e5eacad3f1c) Thanks [@arendjr](https://github.com/arendjr)! - Improved [#6172](https://github.com/biomejs/biome/issues/6172): Optimised the way function arguments are stored in Biome's type inference. This led to about 10% performance improvement in
  `RedisCommander.d.ts` and about 2% on `@next/font` type definitions.

- [#6878](https://github.com/biomejs/biome/pull/6878) [`3402976`](https://github.com/biomejs/biome/commit/340297602c1162928735d1c073d7a409c22e90bd) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a bug where the Biome Language Server would apply an unsafe fix when using the code action
  `quickfix.biome`.

  Now Biome no longer applies an unsafe code fix when using the code action `quickfix.biome`.

- [#6794](https://github.com/biomejs/biome/pull/6794) [`4d5fc0e`](https://github.com/biomejs/biome/commit/4d5fc0ef38f8c4ad820e297749efc83e983b5a91) Thanks [@vladimir-ivanov](https://github.com/vladimir-ivanov)! - Fixed [#6719](https://github.com/biomejs/biome/issues/6719): The
  `noInvalidUseBeforeDeclaration` rule covers additional use cases.

  Examples:

  ```ts
  type Bar = { [BAR]: true };
  const BAR = "bar";
  ```

  ```ts
  interface Bar {
    child: { grandChild: { [BAR]: typeof BAR; enumFoo: EnumFoo } };
  }
  const BAR = "bar";
  enum EnumFoo {
    BAR = "bar",
  }
  ```

- [#6863](https://github.com/biomejs/biome/pull/6863) [`531e97e`](https://github.com/biomejs/biome/commit/531e97e3f691e3ff34d2382fab414072ecb68e8b) Thanks [@dyc3](https://github.com/dyc3)! - Biome now considers whether the linter is enabled when figuring out how the project should be scanned. Resolves [#6815](https://github.com/biomejs/biome/issues/6815).

- [#6832](https://github.com/biomejs/biome/pull/6832) [`bdbc2b1`](https://github.com/biomejs/biome/commit/bdbc2b10ac21dcb35b41e93b17e712ba80f421ca) Thanks [@togami2864](https://github.com/togami2864)! - Fixed [#6165](https://github.com/biomejs/biome/issues/6165): Fixed false negative in [`noUnusedPrivateClassMembers`](https://biomejs.dev/linter/rules/no-unused-private-class-members/) rule when checking member usage in classes

- [#6839](https://github.com/biomejs/biome/pull/6839) [`4cd62d8`](https://github.com/biomejs/biome/commit/4cd62d8ae2e5cb24d6f308e05b38003486294548) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a bug where the root ignore file wasn't correctly loaded during the scanning phase, causing false positives and incorrect expectations among users.

  Now, when using `vcs.useIgnoreFile`, the **the globs specified in the ignore file from the project root
  ** will have the same semantics as the `files.includes` setting of the root configuration.

  Refer to the [relative web page](https://biomejs.dev/internals/architecture/#configuring-the-scanner) to understand how they work.

- [#6898](https://github.com/biomejs/biome/pull/6898) [`5beb024`](https://github.com/biomejs/biome/commit/5beb024d8e9af8733bc115ba4b07d20036fe336e) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#6891](https://github.com/biomejs/biome/issues/6891): Improved type inference for array indices.

  **Example:**

  ```ts
  const numbers: number[];
  numbers[42]; // This now infers to `number | undefined`.
  ```

- [#6809](https://github.com/biomejs/biome/pull/6809) [`8192451`](https://github.com/biomejs/biome/commit/819245188e587d0a5ede53aa07899a2cb9fcce4f) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#6796](https://github.com/biomejs/biome/issues/6796): Fixed a false positive that happened in
  `noFloatingPromises` when calling functions that were declared as part of `for ... of` syntax inside
  `async` functions.

  Instead, the variables declared inside
  `for ... of` loops are now correctly inferred if the expression being iterated evaluates to an
  `Array` (support for other iterables will follow later).

  **Invalid example**

  ```tsx
  const txStatements: Array<(tx) => Promise<any>> = [];

  db.transaction((tx: any) => {
    for (const stmt of txStatements) {
      // We correctly flag this resolves to a `Promise`:
      stmt(tx);
    }
  });
  ```

  **Valid example**

  ```tsx
  async function valid(db) {
    const txStatements: Array<(tx: any) => void> = [(tx) => tx.insert().run()];

    db.transaction((tx: any) => {
      for (const stmt of txStatements) {
        // We don't flag a false positive here anymore:
        stmt(tx);
      }
    });
  }
  ```

- [#6757](https://github.com/biomejs/biome/pull/6757) [`13a0818`](https://github.com/biomejs/biome/commit/13a0818be8cc08efd303829252cbc3e64bcbca3a) Thanks [@mdevils](https://github.com/mdevils)! - Added the rule [`noVueReservedProps`](https://biomejs.dev/linter/rules/no-vue-reserved-props/), resolves [#6309](https://github.com/biomejs/biome/issues/6309).

  It prevents the use of reserved Vue prop names such as `key` and
  `ref` which can cause conflicts and unexpected behavior in Vue components.

  ##### Invalid example

  ```js
  import { defineComponent } from "vue";

  export default defineComponent({
    props: ["ref", "key", "foo"],
  });
  ```

  ```vue
  <script setup>
  defineProps({
    ref: String,
    key: String,
    foo: String,
  });
  </script>
  ```

  ##### Valid examples

  ```js
  import { defineComponent } from "vue";

  export default defineComponent({
    props: ["foo"],
  });
  ```

  ```vue
  <script setup>
  defineProps({ foo: String });
  </script>
  ```

- [#6840](https://github.com/biomejs/biome/pull/6840) [`1a57b51`](https://github.com/biomejs/biome/commit/1a57b51097c7bf4faeb0dcc5330d49e17f86789b) Thanks [@denbezrukov](https://github.com/denbezrukov)! - Allow multiple identifiers in ::part() pseudo-element selector.

  ```css
  ::part(first second) {
  }
  ```

- [#6845](https://github.com/biomejs/biome/pull/6845) [`4fd44ec`](https://github.com/biomejs/biome/commit/4fd44ec17a3ac6a5486ac94f01e85e62310b8061) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#6510](https://github.com/biomejs/biome/issues/6510): The scanner no longer shows diagnostics on inaccessible files unless
  `--verbose` is used.

- [#6844](https://github.com/biomejs/biome/pull/6844) [`b7e2d4d`](https://github.com/biomejs/biome/commit/b7e2d4d3a8b2654278596eaecdccc30405457fc8) Thanks [@sterliakov](https://github.com/sterliakov)! - Fixed [#6837](https://github.com/biomejs/biome/issues/6837): Fixed regression with multiple consecutive line suppression comments using instances (like
  `// biome-ignore lint/correctness/useExhaustiveDependencies(depName): reason`).

- [#6818](https://github.com/biomejs/biome/pull/6818) [`5f3f5a6`](https://github.com/biomejs/biome/commit/5f3f5a6e8c12b56dc36bcfb4f8d5077eb33ccf08) Thanks [@siketyan](https://github.com/siketyan)! - Fixed an issue where
  `textDocument/codeAction` in the LSP could respond with outdated text edits after the workspace watcher observed outdated changes to the file.

- [#6804](https://github.com/biomejs/biome/pull/6804) [`3e6ab16`](https://github.com/biomejs/biome/commit/3e6ab1663ab15f9f00ae069ee790e5fd90327082) Thanks [@arendjr](https://github.com/arendjr)! -
  `noFloatingPromises` will no longer suggest to add `await` keyword inside synchronous callbacks nested inside
  `async` functions.

- [#6901](https://github.com/biomejs/biome/pull/6901) [`c9e969a`](https://github.com/biomejs/biome/commit/c9e969a84158b29d175cd04ea8b921c737b7ed8f) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#6777](https://github.com/biomejs/biome/issues/6777): Fixed type inference handling of
  `this` to avoid infinite recursion.

  Thanks to @sterliakov for the thorough investigation!

- [#6855](https://github.com/biomejs/biome/pull/6855) [`d1581c7`](https://github.com/biomejs/biome/commit/d1581c7c874b2917132a864d1c65df041ad9181b) Thanks [@vladimir-ivanov](https://github.com/vladimir-ivanov)! - Fixed [#6775](https://github.com/biomejs/biome/issues/6775):
  `useReadonlyClassProperties` now also captures mutations inside function arguments.

  Example:

  ```ts
  class Counter {
    private counter: number;
    count() {
      console.log(this.counter++);
      const counterString = `${this.counter++}`;
    }
  }
  ```

- [#6839](https://github.com/biomejs/biome/pull/6839) [`4cd62d8`](https://github.com/biomejs/biome/commit/4cd62d8ae2e5cb24d6f308e05b38003486294548) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a bug where Biome didn't throw any error when
  `vcs.useIgnoreFile` is set to
  `true`, and there wasn't any ignore file read. Now Biome correctly throws an error if no ignore files are found.

- [#6911](https://github.com/biomejs/biome/pull/6911) [`6d68074`](https://github.com/biomejs/biome/commit/6d68074bf2a2ca4bc514398a180524394690fafe) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#6838](https://github.com/biomejs/biome/issues/6838): Reduce resource consumption in the Biome Language Server by using non-recursive filesystem watchers instead of recursive ones.

  Watchers are responsible for notifying Biome of changes to files in the filesystem. We used to set up a single recursive watcher, but that meant that Biome would receive filesystem notifications for
  _all_ files in your project, even for ignored folders such as `build/` or `dist/` folders.

  With this patch, we set up non-recursive watchers only for the folders that are relevant to a project.

  Related to this, we also solved an issue where incoming notifications were incorrectly filtered, causing ignored files to be processed and stored in our module graph anyway.

## 2.1.1

### Patch Changes

- [#6781](https://github.com/biomejs/biome/pull/6781) [`9bbd34f`](https://github.com/biomejs/biome/commit/9bbd34f8d4be3dd4ba4c63746a5b2915e578e339) Thanks [@siketyan](https://github.com/siketyan)! - Fixed the
  `FileFeaturesResult` interface in the WASM API was defined as a mapped object but the actual value was a `Map` object.

- [#6761](https://github.com/biomejs/biome/pull/6761) [`cf3c2ce`](https://github.com/biomejs/biome/commit/cf3c2ce3ac28a36eee948ad689794783b0ba23ef) Thanks [@dyc3](https://github.com/dyc3)! - Fixed [#6759](https://github.com/biomejs/biome/issues/6759), a false positive for
  `noFocusedTests` that was triggered by calling any function with the name `fit` on any object.

  The following code will now pass the `noFocusedTests` rule:

  ```js
  import foo from "foo";
  foo.fit();
  ```

## 2.1.0

### Minor Changes

- [#6512](https://github.com/biomejs/biome/pull/6512) [`0c0bf82`](https://github.com/biomejs/biome/commit/0c0bf82c92ee4e853172f44e38af57afde6de2ce) Thanks [@arendjr](https://github.com/arendjr)! - The rule [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-misused-promises/) can now detect floating arrays of
  `Promise`s.

  **Invalid examples**

  ```ts
  // This gets flagged because the Promises are not handled.
  [1, 2, 3].map(async (x) => x + 1);
  ```

  **Valid examples**

  ```ts
  await Promise.all([1, 2, 3].map(async (x) => x + 1));
  ```

- [#6637](https://github.com/biomejs/biome/pull/6637) [`6918085`](https://github.com/biomejs/biome/commit/6918085e14b8e34bfd0adc472acce22c31484ab3) Thanks [@arendjr](https://github.com/arendjr)! - Type inference is now able to handle the sequence operator (
  `,`), as well as post- and pre-update operators: `++`.

  **Example**

  ```ts
  let x = 5;

  // We now infer that `x++` resolves to a number, while the expression as a whole
  // becomes a Promise:
  (x++, new Promise((resolve) => resolve("comma")));
  ```

- [#6752](https://github.com/biomejs/biome/pull/6752) [`c9eaca4`](https://github.com/biomejs/biome/commit/c9eaca4b944acfd18b700c65c904806b11c318d5) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#6646](https://github.com/biomejs/biome/issues/6646):
  `.gitignore` files are now picked up even when running Biome from a nested directory, or when the ignore file itself is ignored through
  `files.includes`.

- [#6746](https://github.com/biomejs/biome/pull/6746) [`90aeead`](https://github.com/biomejs/biome/commit/90aeeadf80700aee9f29121511d0c4c9019a49d6) Thanks [@arendjr](https://github.com/arendjr)! -
  `biome migrate` no longer enables style rules that were recommended in v1, because that would be undesirable for users upgrading from 2.0.

  Users who are upgrading from Biome 1.x are therefore advised to first upgrade to Biome 2.0, and run the migration, before continuing to Biome 2.1 or later.

- [#6583](https://github.com/biomejs/biome/pull/6583) [`d415a3f`](https://github.com/biomejs/biome/commit/d415a3f6f204cc7b109dc08f6117fe97ef07b216) Thanks [@arendjr](https://github.com/arendjr)! - Added the nursery rule [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/).

  It signals `Promise`s in places where conditionals or iterables are expected.

  **Invalid examples**

  ```ts
  const promise = Promise.resolve("value");

  // Using a `Promise` as conditional is always truthy:
  if (promise) {
    /* ... */
  }

  // Spreading a `Promise` has no effect:
  console.log({ foo: 42, ...promise });

  // This does not `await` the `Promise`s from the callbacks,
  // so it does not behave as you may expect:
  [1, 2, 3].forEach(async (value) => {
    await fetch(`/${value}`);
  });
  ```

  **Valid examples**

  ```ts
  const promise = Promise.resolve("value");

  if (await promise) {
    /* ... */
  }

  console.log({ foo: 42, ...(await promise) });
  ```

- [#6405](https://github.com/biomejs/biome/pull/6405) [`cd4a9bb`](https://github.com/biomejs/biome/commit/cd4a9bbdcbc176fa2294fd5a2a2565a13b12a51d) Thanks [@vladimir-ivanov](https://github.com/vladimir-ivanov)! - Added the
  `ignoreRestSiblings` option to the `noUnusedFunctionParameters` rule.

  This option is used to ignore unused function parameters that are siblings of the rest parameter.

  The default is
  `false`, which means that unused function parameters that are siblings of the rest parameter will be reported.

  **Example**

  ```json
  {
    "rules": {
      "noUnusedFunctionParameters": ["error", { "ignoreRestSiblings": true }]
    }
  }
  ```

- [#6614](https://github.com/biomejs/biome/pull/6614) [`0840021`](https://github.com/biomejs/biome/commit/0840021860fcc5e9055f781dce84e80353f9f5ce) Thanks [@arendjr](https://github.com/arendjr)! - We have implemented a more targeted version of the scanner, which ensures that if you provide file paths to handle on the CLI, the scanner will exclude directories that are not relevant to those paths.

  Note that for many commands, such as `biome check` and
  `biome format`, the file paths to handle are implicitly set to the current working directory if you do not provide any path explicitly. The targeted scanner also works with such implicit paths, which means that if you run Biome from a subfolder, other folders that are part of the project are automatically exempted.

  Use cases where you invoke Biome from the root of the project without providing a path, as well as those where project rules are enabled, are not expected to see performance benefits from this.

  Implemented [#6234](https://github.com/biomejs/biome/issues/6234), and fixed [#6483](https://github.com/biomejs/biome/issues/6483) and [#6563](https://github.com/biomejs/biome/issues/6563).

- [#6488](https://github.com/biomejs/biome/pull/6488) [`c5ee385`](https://github.com/biomejs/biome/commit/c5ee38569fc0b91ea9411da25560d3a1076870c6) Thanks [@ianzone](https://github.com/ianzone)! -
  `nx.json` and `project.json` have been added to the list of well-known files.

- [#6720](https://github.com/biomejs/biome/pull/6720) [`52e36ae`](https://github.com/biomejs/biome/commit/52e36ae827d2c9f02520298d6518a00b22db38b8) Thanks [@minht11](https://github.com/minht11)! - Added
  `$` symbol to [organizeImports](https://biomejs.dev/assist/actions/organize-imports) `:ALIAS:` group.

  `import { action } from '$lib'` will be treated as alias import.

### Patch Changes

- [#6712](https://github.com/biomejs/biome/pull/6712) [`2649ac6`](https://github.com/biomejs/biome/commit/2649ac625de963bf7411368cdd06142bda362322) Thanks [@sterliakov](https://github.com/sterliakov)! - Fixed [#6595](https://github.com/biomejs/biome/issues/6595): Biome now supports
  `// biome-ignore-all` file-level suppressions in files that start with a shebang (`#!`).

- [#6758](https://github.com/biomejs/biome/pull/6758) [`28dc49e`](https://github.com/biomejs/biome/commit/28dc49eacb9da1073d56070eb70b10ed636a1799) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#6573](https://github.com/biomejs/biome/issues/6573): Grit plugins can now match bare imports.

  **Example**

  The following snippet:

  ```grit
  `import $source`
  ```

  will now match:

  ```ts
  import "main.css";
  ```

- [#6550](https://github.com/biomejs/biome/pull/6550) [`b424f46`](https://github.com/biomejs/biome/commit/b424f4682cdcba5bf4cd6eb4b34486b631ddfbdc) Thanks [@arendjr](https://github.com/arendjr)! - Type inference is now able to handle logical expressions:
  `&&`, `||`, and `??`.

  **Examples**

  ```ts
  // We can now infer that because `true` is truthy, the entire expression
  // evaluates to a `Promise`.
  true && Promise.reject("logical operator bypass");

  // And we know that this doesn't:
  false && Promise.reject("logical operator bypass");

  // Truthiness, falsiness, and non-nullishness can all be determined on more
  // complex expressions as well. So the following also works:
  type Nullish = null | undefined;

  type Params = {
    booleanOption: boolean | Nullish;
    falsyOption: false | Nullish;
  };

  function foo({ booleanOption, falsyOption }: Params) {
    // This may be a Promise:
    booleanOption ?? Promise.reject("logical operator bypass");

    // But this never is:
    falsyOption && Promise.reject("logical operator bypass");
  }
  ```

- [#6413](https://github.com/biomejs/biome/pull/6413) [`4aa0e50`](https://github.com/biomejs/biome/commit/4aa0e50a91f457a059b225f140d9fa44ea08a8fb) Thanks [@wojtekmaj](https://github.com/wojtekmaj)! - Improved error message in [`useDateNow`](https://biomejs.dev/linter/rules/use-date-now/) rule.

- [#6673](https://github.com/biomejs/biome/pull/6673) [`341e062`](https://github.com/biomejs/biome/commit/341e062bc28f32adc2ee44c26ab4fb0574750319) Thanks [@dyc3](https://github.com/dyc3)! - Fixed a case where the HTML formatter would mangle embedded language tags if
  `whitespaceSensitivity` was set to `strict`

- [#6642](https://github.com/biomejs/biome/pull/6642) [`a991229`](https://github.com/biomejs/biome/commit/a99122902eb01907f03565d2c7e56186d01764d3) Thanks [@unvalley](https://github.com/unvalley)! - Fixed [#4494](https://github.com/biomejs/biome/issues/4494): The
  `noSecrets` rule now correctly uses the `entropyThreshold` option to detect secret like strings.

- [#6520](https://github.com/biomejs/biome/pull/6520) [`0c43545`](https://github.com/biomejs/biome/commit/0c43545934ba50ca0dbb0581f274e0e41a7e26e7) Thanks [@arendjr](https://github.com/arendjr)! - Type inference is now able to handle ternary conditions in type aliases.

  Note that we don't attempt to evaluate the condition itself. The resulting type is simply a union of both conditional outcomes.

  **Example**

  ```ts
  type MaybeResult<T> = T extends Function ? Promise<string> : undefined;

  // We can now detect this function _might_ return a `Promise`:
  function doStuff<T>(input: T): MaybeResult<T> {
    /* ... */
  }
  ```

- [#6711](https://github.com/biomejs/biome/pull/6711) [`1937691`](https://github.com/biomejs/biome/commit/1937691bb7041026475e2f9fc88a2841c5bfacc4) Thanks [@sterliakov](https://github.com/sterliakov)! - Fixed [#6654](https://github.com/biomejs/biome/issues/6654): Fixed range highlighting of
  `<explanation>` placeholder in inline suppression block comments.

- [#6756](https://github.com/biomejs/biome/pull/6756) [`d12b26f`](https://github.com/biomejs/biome/commit/d12b26f60865e910a3d300e04f216a36ffc63f8e) Thanks [@dyc3](https://github.com/dyc3)! - Fixed [#6669](https://github.com/biomejs/biome/issues/6669): Added an exception to
  `noUnusedImports` to allow type augmentation imports.

  ```ts
  import type {} from "@mui/lab/themeAugmentation";
  ```

- [#6643](https://github.com/biomejs/biome/pull/6643) [`df15ad6`](https://github.com/biomejs/biome/commit/df15ad6e9a99ec3dba17cc4e6e4081736c93b3a7) Thanks [@skewb1k](https://github.com/skewb1k)!

- Fixed [#4994](https://github.com/biomejs/biome/discussions/4994): LSP server registered some capabilities even when the client did not support dynamic registration.

- [#6599](https://github.com/biomejs/biome/pull/6599) [`5e611fa`](https://github.com/biomejs/biome/commit/5e611fae93c794cdbd290f88cc1676bc6aea090d) Thanks [@vladimir-ivanov](https://github.com/vladimir-ivanov)! - Fixed [#6380](https://github.com/biomejs/biome/issues/6380): The
  `noFocusedTests` rule now correctly displays the function name in the diagnostic message when a test is focused.

  Every instance of a focused test function (like `fdescribe`, `fit`, `ftest` and
  `only`) had the word 'only' hardcoded. This has been updated to use the actual function name, so the message is now more accurate and specific.

  Example for `fdescribe`:

  ```text
    i The 'fdescribe' method is often used for debugging or during implementation.

    i Consider removing 'f' prefix from 'fdescribe' to ensure all tests are executed.
  ```

- [#6671](https://github.com/biomejs/biome/pull/6671) [`0c9ab43`](https://github.com/biomejs/biome/commit/0c9ab43bea6ed4005c96ac6e4e7c5553cae16192) Thanks [@vladimir-ivanov](https://github.com/vladimir-ivanov)! - Fixed [#6634](https://github.com/biomejs/biome/issues/6634): The
  `useReadonlyClassProperties` rule now correctly flags mutations in class getters and in arrow functions within class properties.

  Examples:

  ```ts
  class GetterWithMutationValue {
    #value: string;

    get value() {
      if (!this.#value) {
        this.#value = "defaultValue";
      }

      return this.#value;
    }
  }
  ```

  ```ts
  class ClassPropertyArrowFunctionWithMutation {
    private bar: string | null = null;

    readonly action = () => {
      this.bar = "init";
    };
  }
  ```

- [#6682](https://github.com/biomejs/biome/pull/6682) [`ca04cea`](https://github.com/biomejs/biome/commit/ca04ceab45ceb445522ebf95fdb90a6117995ea5) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6668](https://github.com/biomejs/biome/issues/6668): Biome Assist is now enabled by default for CSS files.

- [#6525](https://github.com/biomejs/biome/pull/6525) [`66b089c`](https://github.com/biomejs/biome/commit/66b089c9031bf02808426c1cd67b53d75663cca7) Thanks [@arendjr](https://github.com/arendjr)! - Type inference can now infer the return types of functions and methods without annotations.

  **Examples**

  ```ts
  const sneakyObject = {
    doSomething() {
      return Promise.resolve("This is a floating promise!");
    },
  };

  // We can now detect that `doSomething()` returns a `Promise`.
  sneakyObject.doSomething();
  ```

- [#6531](https://github.com/biomejs/biome/pull/6531) [`c06df79`](https://github.com/biomejs/biome/commit/c06df798908d7e624b03edc3be2a06ca249ad520) Thanks [@arendjr](https://github.com/arendjr)! - Biome's type inference now detects the type of properties with getters.

  **Examples**

  ```ts
  const sneakyObject2 = {
    get something() {
      return new Promise((_, reject) => reject("This is a floating promise!"));
    },
  };
  // We now detect this is a Promise:
  sneakyObject2.something;
  ```

- [#6587](https://github.com/biomejs/biome/pull/6587) [`a330fcc`](https://github.com/biomejs/biome/commit/a330fcc9ad6901d82b6f460d4bf50d7bdca7efbd) Thanks [@Conaclos](https://github.com/Conaclos)! -
  `organizeImports` is now able to sort named specifiers and import attributes with bogus nodes.

- [#6618](https://github.com/biomejs/biome/pull/6618) [`6174869`](https://github.com/biomejs/biome/commit/6174869dc0b6df82cda3fc5c1b7603157371a069) Thanks [@Shinyaigeek](https://github.com/Shinyaigeek)! - Fixed [#6610](https://github.com/biomejs/biome/issues/6610): JSON import attributes are now correctly detected when they contain extra whitespace.

- [#6753](https://github.com/biomejs/biome/pull/6753) [`fce5d2c`](https://github.com/biomejs/biome/commit/fce5d2cd3708e3010e0a9acdef184c01a79929bb) Thanks [@dyc3](https://github.com/dyc3)! - Improved the error messages when Biome is provided incompatible arguments on the CLI.

- [#6587](https://github.com/biomejs/biome/pull/6587) [`a330fcc`](https://github.com/biomejs/biome/commit/a330fcc9ad6901d82b6f460d4bf50d7bdca7efbd) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#6491](https://github.com/biomejs/biome/issues/6491): The action of
  `useSortedKeys` removed comments or wrongly transferred them to distinct nodes.

- [#6696](https://github.com/biomejs/biome/pull/6696) [`92964a7`](https://github.com/biomejs/biome/commit/92964a7ae076b9b08b83da329e2b8a5825e30da9) Thanks [@unvalley](https://github.com/unvalley)! - Fixed [#6633](https://github.com/biomejs/biome/6633): The
  `noImplicitCoercion` rule no longer reports diagnostics for `1 / value` expressions.

  ```js
  1 / value; // no error
  ```

- [#6683](https://github.com/biomejs/biome/pull/6683) [`43d871e`](https://github.com/biomejs/biome/commit/43d871e0f8b331dfece2b1671152e6336e673ec8) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6537](https://github.com/biomejs/biome/issues/6537): Biome no longer removes the trailing comma from JSON files when
  `formatter.json.trailingCommas` is explicitly set to `"all"`.

- [#6693](https://github.com/biomejs/biome/pull/6693) [`bfdce0b`](https://github.com/biomejs/biome/commit/bfdce0be416db38ab18e68a41ddd0ab82177c14b) Thanks [@dyc3](https://github.com/dyc3)! - Fixed [#6691](https://github.com/biomejs/biome/issues/6691): The HTML parser will now consider
  `.` to be a valid character for tag names.

- [#6716](https://github.com/biomejs/biome/pull/6716) [`ead03d1`](https://github.com/biomejs/biome/commit/ead03d1089dd2e7a11a926008fd2b66b12e1f36c) Thanks [@siketyan](https://github.com/siketyan)! - The Biome LSP server no longer responds with an error for a
  `textDocument/codeActions` request when Biome doesn't support a feature for the file (e.g. Code actions aren't supported in GritQL files).

- [#6679](https://github.com/biomejs/biome/pull/6679) [`7bf9a60`](https://github.com/biomejs/biome/commit/7bf9a608e1592fd595f658f5f800e12d51835d34) Thanks [@marko-hologram](https://github.com/marko-hologram)! - Fixed [#6638](https://github.com/biomejs/biome/issues/6638): JavaScript formatter
  `overrides` options now correctly override `expand` option. JSON formatter `overrides` options now correctly override
  `bracketSpacing` and `expand` options.

- [#6717](https://github.com/biomejs/biome/pull/6717) [`7f5b541`](https://github.com/biomejs/biome/commit/7f5b5410613c5f1e0b26fdca5fa7c67b70f1fdb9) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#6688](https://github.com/biomejs/biome/issues/6688): the
  `noUselessFragments` no longer reports `<Fragment />` elements that includes HTML character entities.

- [#6600](https://github.com/biomejs/biome/pull/6600) [`853e1b5`](https://github.com/biomejs/biome/commit/853e1b54c365c18d8065499797ba172596b614cb) Thanks [@daivinhtran](https://github.com/daivinhtran)! - Fixed [#4677](https://github.com/biomejs/biome/issues/4677): The
  `noUnusedImports` rule won't produce diagnostics for types used in comments of static members anymore.

- [#6662](https://github.com/biomejs/biome/pull/6662) [`3afc804`](https://github.com/biomejs/biome/commit/3afc8040e6fa3f60addb0ad06ea86babbdd712e9) Thanks [@arendjr](https://github.com/arendjr)! - If a nested configuration file is ignored by the root configuration, it will now actually be ignored.

  Biome has an exception in place for configuration files so they cannot be ignored, because the configuration files are vital to Biome itself. But this exception was incorrectly applied to nested configurations as well. Now only the root configuration is exempt from being ignored.

- [#6596](https://github.com/biomejs/biome/pull/6596) [`c0718ca`](https://github.com/biomejs/biome/commit/c0718ca610a655e675182ac6c0424301aa64c325) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6566](https://github.com/biomejs/biome/issues/6566): Biome no longer errors when using the option
  `--files-ignore-unknown=true` in `stdin` mode.

  Biome has also become less strict when using `--stdin-file-path` in
  `stdin` mode. It will no longer error if the file path doesn't contain an extension, but instead it will return the original content.

- [#6562](https://github.com/biomejs/biome/pull/6562) [`153eda7`](https://github.com/biomejs/biome/commit/153eda75003d01e1b1c4c120b9516eee47e5692e) Thanks [@vladimir-ivanov](https://github.com/vladimir-ivanov)! - Added the nursery rule [noMagicNumbers](https://github.com/biomejs/biome/issues/4333). The rule detects and reports the use of "magic numbers" — numeric literals that are used directly in code without being assigned to a named constant.

  **Example**

  ```js
  let total = price * 1.23; // Magic number for tax rate will highlight 1.23 as magic number
  ```

- [#6663](https://github.com/biomejs/biome/pull/6663) [`af78d6d`](https://github.com/biomejs/biome/commit/af78d6d00f61a118d6b178bc2238c63bd83a0299) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6656](https://github.com/biomejs/biome/issues/6656): Biome now correctly formats HTML void elements such as
  `<meta>` when they contain a self-closing slash.

  ```diff
  - <meta foo="bar" />
  + <meta foo="bar">
  ```

- [#6732](https://github.com/biomejs/biome/pull/6732) [`31e4396`](https://github.com/biomejs/biome/commit/31e439674493da76e0ce213e5660be3d903efbef) Thanks [@vladimir-ivanov](https://github.com/vladimir-ivanov)! - Resolved [#6281](https://github.com/biomejs/biome/issues/6281): Improved performance of handling
  `package.json` files in the scanner.

- [#6625](https://github.com/biomejs/biome/pull/6625) [`19cb475`](https://github.com/biomejs/biome/commit/19cb4750a1181f1e5c6c58fa169a94e812f10d25) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#6616](https://github.com/biomejs/biome/issues/6616): Fixed an issue with extending configurations that contained an explicit
  `root` field while the configuration in the project did not.

- [#6650](https://github.com/biomejs/biome/pull/6650) [`19aab18`](https://github.com/biomejs/biome/commit/19aab181dc6405ff48a1010d0a82aa731fb588b3) Thanks [@sterliakov](https://github.com/sterliakov)! - Fixed [#6621](https://github.com/biomejs/biome/issues/6621): Improved handling of multiple adjacent line suppressions. Biome now handles such suppressions separately, tracking whether each one is used.

- [#6700](https://github.com/biomejs/biome/pull/6700) [`cdd6e17`](https://github.com/biomejs/biome/commit/cdd6e179b0d90f27cfdd73da1e56157bf3dd9d73) Thanks [@denbezrukov](https://github.com/denbezrukov)! - Fixed [#6680](https://github.com/biomejs/biome/issues/6680): Biome incorrectly formatted container-style queries by inserting misplaced spaces.

  ```diff
  - @container style (--responsive: true) {}
  + @container style(--responsive: true) {}
  ```

- [#6709](https://github.com/biomejs/biome/pull/6709) [`ecf3954`](https://github.com/biomejs/biome/commit/ecf39549cd7c72c1811ba4dda6051e8622a19cf2) Thanks [@dyc3](https://github.com/dyc3)! - Fixed [#6038](https://github.com/biomejs/biome/issues/6038): Fixed a false positive in
  `noShadow` where a function parameter in a type definition was erroneously flagged as a violation.

- [#6593](https://github.com/biomejs/biome/pull/6593) [`a4acbb7`](https://github.com/biomejs/biome/commit/a4acbb7d02eab2b1d1d7de5ff67c131b92388540) Thanks [@arendjr](https://github.com/arendjr)! - Type inference is now able to handle ternary conditions in expressions.

  **Examples**

  ```ts
  const condition = Math.random() > -1; // Always true, but dynamic to linter

  // We now detect that this may return a `Promise`.
  condition ? Promise.reject("ternary bypass") : null;

  // On the other hand, we know the following is never a `Promise`:
  const alwaysFalsy = 0;
  alwaysFalsy ? Promise.reject("ternary bypass") : null;
  ```

- [#6428](https://github.com/biomejs/biome/pull/6428) [`4b501d3`](https://github.com/biomejs/biome/commit/4b501d3ac6214fd1331548260ccaf9db83e18de4) Thanks [@siketyan](https://github.com/siketyan)! - Added
  `MemoryFileSystem` to the WASM API.

  You can now insert a file from your JS code:

  ```js
  import { MemoryFileSystem, Workspace } from "@biomejs/wasm-web";

  const fs = new MemoryFileSystem();
  const workspace = Workspace.withFileSystem(fs);

  fs.insert("/index.js", new TextEncoder().encode("let foo = 1;"));
  fs.remove("/index.js");
  ```

- [#6594](https://github.com/biomejs/biome/pull/6594) [`626d4a1`](https://github.com/biomejs/biome/commit/626d4a1462794dbd67e2f503812f62c6d40b3aa6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6528](https://github.com/biomejs/biome/issues/6528): Biome didn't return the correct output when applying
  `source.fixAll.biome` inside Astro/Vue/Svelte files that contained safe fixed.

## 2.0.6

### Patch Changes

- [#6557](https://github.com/biomejs/biome/pull/6557) [`fd68458`](https://github.com/biomejs/biome/commit/fd68458f40767cb1aeb9eb444a03c5dd6f3f7c0d) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a bug where Biome didn't provide all the available code actions when requested by the editor.

- [#6511](https://github.com/biomejs/biome/pull/6511) [`72623fa`](https://github.com/biomejs/biome/commit/72623fa30470bbb97bae24514233d4d8a39507ec) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#6492](https://github.com/biomejs/biome/issues/6492). The
  `organizeImports` assist action no longer duplicates a comment at the start of the file when
  `:BLANK_LINE:` precedes the first import group.

- [#6557](https://github.com/biomejs/biome/pull/6557) [`fd68458`](https://github.com/biomejs/biome/commit/fd68458f40767cb1aeb9eb444a03c5dd6f3f7c0d) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6287](https://github.com/biomejs/biome/issues/6287) where Biome Language Server didn't adhere to the
  `settings.requireConfiguration` option when pulling diagnostics and code actions. Note that for this configuration be correctly applied, your editor must support dynamic registration capabilities.

- [#6551](https://github.com/biomejs/biome/pull/6551) [`0b63b1d`](https://github.com/biomejs/biome/commit/0b63b1d95c32ba61b2dcda4195d860397de3b589) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#6536](https://github.com/biomejs/biome/issues/6536).
  `useSortedKeys` no longer panics in some edge cases where object spreads are involved.

- [#6503](https://github.com/biomejs/biome/pull/6503) [`9a8fe0f`](https://github.com/biomejs/biome/commit/9a8fe0f9313b2df93df56b3446340cc04a0e1958) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6482](https://github.com/biomejs/biome/issues/6482) where nursery rules that belonged to a domain were incorrectly enabled.

- [#6565](https://github.com/biomejs/biome/pull/6565) [`e85761c`](https://github.com/biomejs/biome/commit/e85761c72058e2c039ff16707781f7e0aa19d2a9) Thanks [@daivinhtran](https://github.com/daivinhtran)! - Fixed [#4677](https://github.com/biomejs/biome/issues/4677): Now the
  `noUnusedImports` rule won't produce diagnostics for types used in JSDoc comment of exports.

- [#6166](https://github.com/biomejs/biome/pull/6166) [`b8cbd83`](https://github.com/biomejs/biome/commit/b8cbd839935fd0e672cb0fc2051df0e2fb9e5d1a) Thanks [@mehm8128](https://github.com/mehm8128)! - Added the nursery rule [noExcessiveLinesPerFunction](https://biomejs.dev/linter/rules/no-excessive-lines-per-function/). This rule restrict a maximum number of lines of code in a function body.

  The following code is now reported as invalid when the limit of maximum lines is set to 2:

  ```js
  function foo() {
    const x = 0;
    const y = 1;
    const z = 2;
  }
  ```

  The following code is now reported as valid when the limit of maximum lines is set to 3:

  ```jsx
  const bar = () => {
    const x = 0;
    const z = 2;
  };
  ```

- [#6553](https://github.com/biomejs/biome/pull/6553) [`5f42630`](https://github.com/biomejs/biome/commit/5f42630f7b457070c7c1ad17cee28eae2e9951cc) Thanks [@denbezrukov](https://github.com/denbezrukov)! - Fixed [#6547](https://github.com/biomejs/biome/issues/6547). Now the Biome CSS parser correctly parses
  `@starting-style` when it's used inside other at-rules. The following example doesn't raise an error anymore:

  ```css
  @layer my-demo-layer {
    @starting-style {
      div.showing {
        background-color: red;
      }
    }
  }
  ```

- [#6458](https://github.com/biomejs/biome/pull/6458) [`05402e3`](https://github.com/biomejs/biome/commit/05402e395f6e356b690e1cad740294183fafeb84) Thanks [@ematipico](https://github.com/ematipico)! - Fixed an issue where the rule
  `useSemanticElements` used the incorrect range when positioning suppression comments.

- [#6560](https://github.com/biomejs/biome/pull/6560) [`6d8a6b9`](https://github.com/biomejs/biome/commit/6d8a6b9a31788565455d6a6138ef6c1fe67421d5) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#6559](https://github.com/biomejs/biome/issues/6559): the error message on detected a large file was outdated and referred a removed configuration option
  `files.ignore`.

- [#6458](https://github.com/biomejs/biome/pull/6458) [`05402e3`](https://github.com/biomejs/biome/commit/05402e395f6e356b690e1cad740294183fafeb84) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6384](https://github.com/biomejs/biome/issues/6384). The rule [`useAltText`](https://biomejs/dev/linter/rules/no-alt-text) now emits a diagnostic with a correct range, so suppression comments can work correctly.

- [#6518](https://github.com/biomejs/biome/pull/6518) [`7a56288`](https://github.com/biomejs/biome/commit/7a56288e0c7f366d6aa30100432227f3501afb61) Thanks [@wojtekmaj](https://github.com/wojtekmaj)! - Fixed #6508, where the rule
  `noUselessFragments` incorrectly flagged Fragments containing HTML entities as unnecessary.

- [#6517](https://github.com/biomejs/biome/pull/6517) [`c5217cf`](https://github.com/biomejs/biome/commit/c5217cfb21653add3d3add930102bea8fb7b5833) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#6515](https://github.com/biomejs/biome/issues/6515). When using the
  `extends` field to extend a configuration from an NPM package, we now accept the
  _condition names_ `"biome"` and `"default"` for exporting the configuration in the `package.json`.

  This means that where previously your `package.json` had to contain an export declaration similar to this:

  ```json
  {
    "exports": {
      ".": "./biome.json"
    }
  }
  ```

  You may now use one of these as well:

  ```json
  {
    "exports": {
      ".": {
        "biome": "./biome.json"
      }
    }
  }
  ```

  Or:

  ```json
  {
    "exports": {
      ".": {
        "default": "./biome.json"
      }
    }
  }
  ```

- [#6219](https://github.com/biomejs/biome/pull/6219) [`a3a3715`](https://github.com/biomejs/biome/commit/a3a371552a84eaaf24ce1bd8e63e3c1243b285a9) Thanks [@huangtiandi1999](https://github.com/huangtiandi1999)! - Added new nursery rule [`noUnassignedVariables`](https://biomejs.dev/linter/rules/no-unassigned-variables/), which disallows `let` or
  `var` variables that are read but never assigned.

  The following code is now reported as invalid:

  ```js
  let x;
  if (x) {
    console.log(1);
  }
  ```

  The following code is now reported as valid:

  ```js
  let x = 1;
  if (x) {
    console.log(1);
  }
  ```

- [#6395](https://github.com/biomejs/biome/pull/6395) [`f62e748`](https://github.com/biomejs/biome/commit/f62e7481c2a94271869651d2b32bde5d54adbc73) Thanks [@mdevils](https://github.com/mdevils)! - Added the new nursery rule [`noImplicitCoercion`](https://biomejs.dev/linter/rules/no-implicit-coercion), which disallows shorthand type conversions in favor of explicit type conversion functions.

  **Example (Invalid): Boolean conversion using double negation:**

  ```js
  !!foo;
  !!(foo + bar);
  ```

  **Example (Invalid): Number conversion using unary operators:**

  ```js
  +foo;
  -(-foo);
  foo - 0;
  foo * 1;
  foo / 1;
  ```

  **Example (Invalid): String conversion using concatenation:**

  ```js
  "" + foo;
  foo + "";
  `` + foo;
  foo += "";
  ```

  **Example (Invalid): Index checking using bitwise NOT:**

  ```js
  ~foo.indexOf(1);
  ~foo.bar.indexOf(2);
  ```

  **Example (Valid): Using explicit type conversion functions:**

  ```js
  Boolean(foo);
  Number(foo);
  String(foo);
  foo.indexOf(1) !== -1;
  ```

- [#6544](https://github.com/biomejs/biome/pull/6544) [`f28b075`](https://github.com/biomejs/biome/commit/f28b075b4fd28e49f18ae131878f67ce9a831c5a) Thanks [@daivinhtran](https://github.com/daivinhtran)! - Fixed [#6536](https://github.com/biomejs/biome/issues/6530). Now the rule
  `noUselessFragments` produces diagnostics for a top-level useless fragment that is in a return statement.

- [#6320](https://github.com/biomejs/biome/pull/6320) [`5705f1a`](https://github.com/biomejs/biome/commit/5705f1aa9e41bfaea53edf255a18167b52a5fd9b) Thanks [@mdevils](https://github.com/mdevils)! - Added the new nursery rule [`useUnifiedTypeSignature`](https://biomejs.dev/linter/rules/use-unified-type-signature), which disallows overload signatures that can be unified into a single signature.

  Overload signatures that can be merged into a single signature are redundant and should be avoided. This rule helps simplify function signatures by combining overloads by making parameters optional and/or using type unions.

  **Example (Invalid): Overload signatures that can be unified:**

  ```ts
  function f(a: number): void;
  function f(a: string): void;
  ```

  ```ts
  interface I {
    a(): void;
    a(x: number): void;
  }
  ```

  **Example (Valid): Unified signatures:**

  ```ts
  function f(a: number | string): void {}
  ```

  ```ts
  interface I {
    a(x?: number): void;
  }
  ```

  **Example (Valid): Different return types cannot be merged:**

  ```ts
  interface I {
    f(): void;
    f(x: number): number;
  }
  ```

- [#6545](https://github.com/biomejs/biome/pull/6545) [`2782175`](https://github.com/biomejs/biome/commit/2782175c445d4e5f979497ea76beda0276783909) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6529](https://github.com/biomejs/biome/issues/6529), where the Biome Language Server would emit an error when the user would open a file that isn't part of its workspace (
  `node_modules` or external files). Now the language server doesn't emit any errors and it exits gracefully.

- [#6524](https://github.com/biomejs/biome/pull/6524) [`a27b825`](https://github.com/biomejs/biome/commit/a27b8253b2f0d5e5618e9b26eebaaa5da55ed69a) Thanks [@vladimir-ivanov](https://github.com/vladimir-ivanov)! - Fixed [#6500](https://github.com/biomejs/biome/issues/6500): The
  `useReadonlyClassProperties` rule now correctly marks class properties as
  `readonly` when they are assigned in a constructor, setter or method, even if the assignment occurs inside an if or else block.

  The following code is now correctly detected by the rule:

  ```ts
  class Price {
    #price: string;

    @Input()
    set some(value: string | number) {
      if (
        value === undefined ||
        value === null ||
        value === "undefined" ||
        value === "null" ||
        Number.isNaN(value)
      ) {
        this.#price = "";
      } else {
        this.#price = "" + value;
      }
    }
  }
  ```

- [#6355](https://github.com/biomejs/biome/pull/6355) [`e128ea9`](https://github.com/biomejs/biome/commit/e128ea9eb44bcf5558ab6b08214884d1c087686d) Thanks [@anthonyshew](https://github.com/anthonyshew)! - Added a new nursery rule
  `noAlert` that disallows the use of `alert`, `confirm` and `prompt`.

  The following code is deemed incorrect:

  ```js
  alert("here!");
  ```

- [#6548](https://github.com/biomejs/biome/pull/6548) [`37e9799`](https://github.com/biomejs/biome/commit/37e979978b406c3e132fd5093bfb21e811c93d2d) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6459](https://github.com/biomejs/biome/issues/6459), where the Biome LSP was not taking into account the correct settings when applying
  `source.fixAll.biome` code action.

## 2.0.5

### Patch Changes

- [#6461](https://github.com/biomejs/biome/pull/6461) [`38862e6`](https://github.com/biomejs/biome/commit/38862e645c07935f2daf52799dce38656d589d40) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6419](https://github.com/biomejs/biome/issues/6419), a regression where stdin mode would create a temporary new file instead of using the one provided by the user. This was an intended regression.

  Now Biome will use the file path passed via `--std-file-path`, and apply the configuration that matches it.

- [#6480](https://github.com/biomejs/biome/pull/6480) [`050047f`](https://github.com/biomejs/biome/commit/050047f4a3c1379abcf3cf57f1bfecd20bb7d8c1) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#6371](https://github.com/biomejs/biome/issues/6371).
  [useNamingConvention](https://biomejs.dev/linter/rules/use-naming-convention/) now checks the string case of objects' property shorthand.

- [#6477](https://github.com/biomejs/biome/pull/6477) [`b98379d`](https://github.com/biomejs/biome/commit/b98379d42d97540c3bd911263a0af1eb7bc4803e) Thanks [@ematipico](https://github.com/ematipico)! - Fixed an issue where Biome formatter didn't format consistently CSS value separated by commas.

  ```diff
  .font-heading {
  - font-feature-settings: var(--heading-salt), var(--heading-ss06),
  -   var(--heading-ss11), var(--heading-cv09), var(--heading-liga),
  -   var(--heading-calt);

  +  font-feature-settings:
  +    var(--heading-salt), var(--heading-ss06), var(--heading-ss11),
  +    var(--heading-cv09), var(--heading-liga), var(--heading-calt);
  }

  ```

- [#6248](https://github.com/biomejs/biome/pull/6248) [`ec7126c`](https://github.com/biomejs/biome/commit/ec7126ca3d6777344191f3463b430a44fce02489) Thanks [@fireairforce](https://github.com/fireairforce)! - Fixed grit pattern matching for different kinds of import statements.

  The grit pattern `import $imports from "foo"` will match the following code:

  ```ts
  import bar from "foo";
  import { bar } from "foo";
  import { bar, baz } from "foo";
  ```

## 2.0.4

### Patch Changes

- [#6450](https://github.com/biomejs/biome/pull/6450) [`7472d9e`](https://github.com/biomejs/biome/commit/7472d9e07fd6e8afab385276678f3d39c7497bab) Thanks [@ematipico](https://github.com/ematipico)! - Fixed an issue where the binary wasn't correctly mapped.

## 2.0.3

### Patch Changes

- [#6439](https://github.com/biomejs/biome/pull/6439) [`7e4da4e`](https://github.com/biomejs/biome/commit/7e4da4edb811f9598e446c77fd26bc3802b6d3dd) Thanks [@ematipico](https://github.com/ematipico)! - Fixed an issue where the correct rights aren't added to the binary during publishing

- [#6297](https://github.com/biomejs/biome/pull/6297) [`cc4b8c9`](https://github.com/biomejs/biome/commit/cc4b8c90017f9c04eab393abc60b3f94a35e3cfa) Thanks [@vladimir-ivanov](https://github.com/vladimir-ivanov)! - Added a new lint
  `useReadonlyClassProperties` rule. This rule is a port of ESLint's [prefer-readonly](https://typescript-eslint.io/rules/prefer-readonly/) rule.

  Example:

  ```ts
  class Example {
    // All properties below can be marked as readonly
    public constantValue = 42;
    protected initializedInConstructor: string;
    private privateField = true;

    constructor(initializedInConstructor: string) {
      this.initializedInConstructor = initializedInConstructor;
    }
  }
  ```

## 2.0.2

### Patch Changes

- [#6436](https://github.com/biomejs/biome/pull/6436) [`ec7c63d`](https://github.com/biomejs/biome/commit/ec7c63df520103b5d8ea0090c59486574e7370dd) Thanks [@ematipico](https://github.com/ematipico)! - Fixed an issue where binaries weren't copied anymore inside the
  `@biomejs/cli-*` packages.

## 2.0.1

### Patch Changes

- [#6425](https://github.com/biomejs/biome/pull/6425) [`00e97ad`](https://github.com/biomejs/biome/commit/00e97aded825e72e63db7827de20dc84ac8a123b) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#6391](https://github.com/biomejs/biome/issues/6391): the rule [`noUselessFragments`](https://biomejs.dev/linter/rules/no-useless-fragments/) no longer reports a fragment that contains whitespaces which aren't trimmed by the runtime.

- [#6417](https://github.com/biomejs/biome/pull/6417) [`dd88565`](https://github.com/biomejs/biome/commit/dd885655b576869eb624d4a31d2d09bcb6c623a4) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6360](https://github.com/biomejs/biome/issues/6360): The following pseudo classes and elements are no longer reported by
  `noUnknownPseudoClass` or `noUnknownPseudoElement` rules.
  - `:open`
  - `::details-content`
  - `::prefix`
  - `::search-text`
  - `::suffix`

- [#6417](https://github.com/biomejs/biome/pull/6417) [`dd88565`](https://github.com/biomejs/biome/commit/dd885655b576869eb624d4a31d2d09bcb6c623a4) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6357](https://github.com/biomejs/biome/issues/6357), where the boolean values weren't correctly merged when using the
  `extends` functionality. Now Biome correctly merges the values.

- [#6417](https://github.com/biomejs/biome/pull/6417) [`dd88565`](https://github.com/biomejs/biome/commit/dd885655b576869eb624d4a31d2d09bcb6c623a4) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6341](https://github.com/biomejs/biome/issues/6341): Fixed an issue where Biome would throw an error for the language tags
  `nb` and `nn`.

- [#6385](https://github.com/biomejs/biome/pull/6385) [`94142dd`](https://github.com/biomejs/biome/commit/94142dd84b3a4b680c08007cd4947ca7d44273a8) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#6377](https://github.com/biomejs/biome/issues/6377): The rule [noSelfCompare](https://biomejs.dev/linter/rules/no-self-compare/) now correctly compares two function calls with different arguments.

- [#6417](https://github.com/biomejs/biome/pull/6417) [`dd88565`](https://github.com/biomejs/biome/commit/dd885655b576869eb624d4a31d2d09bcb6c623a4) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6278](https://github.com/biomejs/biome/issues/6278):
  `useExhaustiveDependencies` no longer adds duplicated dependencies into the list.

- [#6417](https://github.com/biomejs/biome/pull/6417) [`dd88565`](https://github.com/biomejs/biome/commit/dd885655b576869eb624d4a31d2d09bcb6c623a4) Thanks [@ematipico](https://github.com/ematipico)! - Fix #6396, where
  `vi.useFakeTimers()` and `vi.useRealTimers()` incorrectly triggered React Hooks-related rules

- [#6417](https://github.com/biomejs/biome/pull/6417) [`dd88565`](https://github.com/biomejs/biome/commit/dd885655b576869eb624d4a31d2d09bcb6c623a4) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a bug where Biome didn't correctly discover nested configuration files when using the
  `lint` command and the linter is disabled in the root configuration.

- [#6422](https://github.com/biomejs/biome/pull/6422) [`594ec50`](https://github.com/biomejs/biome/commit/594ec5008761c6263a43f72f1cbb7e9aafaf8a46) Thanks [@ematipico](https://github.com/ematipico)! - Removed the experimental rename feature from Biome LSP, which caused some issues inside existing editors such as Zed.

- [#6388](https://github.com/biomejs/biome/pull/6388) [`c6942d2`](https://github.com/biomejs/biome/commit/c6942d291297322234f9f145fc2fbf8506dc7673) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#6375](https://github.com/biomejs/biome/issues/6375): the formatter no longer inserts an extra empty line before a semicolon when it has leading comments.

## 2.0.0

### Major Changes

- Biome now resolves globs and paths from the configuration. Before, paths and globs were resolved from the working directory.

- Biome now raises a **warning** diagnostic for suppression comments that have `<explanation>` as reason.

  `<explanation>` is provided as a placeholder when applying the suppression code fix from LSP editors.

- Removed the `--config-path` argument from the `biome lsp-proxy` and `biome start` commands.

  The option was overriding the configuration path for all workspaces opened in the Biome daemon, which led to a configuration mismatch problem when multiple projects are opened in some editors or IDEs.

  If you are using one of our official plugins for IDEs or editors, it is recommended to update it to the latest version of the plugin, or you will get unexpected behavior.

  If you are a developer of a plugin, please update your plugin to use the
  `workspace/configuration` response instead of using the
  `--config-path` argument. Biome's LSP will resolve a configuration in the workspace automatically, so it is recommended to keep it empty unless you are using a custom configuration path.

- Downgraded some code fixes to unsafe which were previously safe.

  The following rules have now a unsafe fix:
  - [`noFlatMapIdentity`](https://biomejs.dev/linter/rules/no-flat-map-identity)
  - [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports)

  If you want to keep applying these fixes automatically, [configure the rule fix](https://next.biomejs.dev/linter/#configure-the-code-fix) as safe:

  ```json
  {
    "linter": {
      "rules": {
        "correctness": {
          "noFlatMapIdentity": {
            "level": "error",
            "fix": "safe"
          },
          "noUnusedImports": {
            "level": "error",
            "fix": "safe"
          }
        }
      }
    }
  }
  ```

- Previously the lint rules `noControlCharactersInRegex` and
  `noMisleadingCharacterClass` checked both regular expression literals like
  `/regex/` and dynamically built regular expressions like `new RegExp("regex")`.

  Checking dynamically built regular expressions has many limitations, edge cases, and complexities. In addition, other rules that lint regular expressions don't check dynamically built regular expressions.

  Rather than add support for other rules and have half-baked checking, we decided to remove support for dynamically built regular expressions.

  Now the lint rules `noControlCharactersInRegex` and
  `noMisleadingCharacterClass` only check literals of regular expressions.

- The lint rule [`noRestrictedGlobals`](https://biomejs.dev/linter/rules/no-restricted-globals/) now supports customizing message for each global name.

  For example, the following configuration:

  ```json
  {
    "options": {
      "deniedGlobals": {
        "$": "jQuery is not allowed. Use native DOM manipulation instead."
      }
    }
  }
  ```

  emits a diagnostic:

  ```
  index.js:1:13 lint/style/noRestrictedGlobals ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    ⚠ Do not use the global variable $.

    > 1 │ console.log($)
        │             ^
      2 │

    ℹ jQuery is not allowed. Use native DOM manipulation instead.
  ```

  Breaking Change: The option `deniedGlobals` is now a record instead of an array. Run
  `biome migrate` to migrate the configuration automatically.

- Removed `include` and `ignore` fields in favor of the new field `includes`.

  The Biome configuration file allows users to specify which files should be processed using [glob patterns](<https://en.wikipedia.org/wiki/Glob_(programming)>). Prior to Biome 2.0, this was done using the
  `include` and `ignore` fields. In Biome 2.0, `include` and `ignore` are removed and replaced by
  `includes`. You can run `biome migrate` to convert `include` and `ignore` into `includes` automatically.

  `includes` uses a different glob pattern format that fixes [many](https://github.com/biomejs/biome/issues/2421) [issues](https://github.com/biomejs/biome/issues/3345) and many other limitations that Biome users reported.

  `includes` accepts an array of glob patterns. A glob pattern starting with a
  `!` is a negated pattern also called exception. This replaces
  `ignore` patterns and allows users to create chains of include and ignore patterns. Thus, it is now possible to include again a file previously ignored. This was not possible with
  `include` and `ignore`, because `ignore` has priority over `include`.

  The semantics of `*` and `**/*` have changed too. Before, with `include` and `ignore`, the glob `*` was interpreted as
  `**/*`. Now, with `includes`, the globs `*` and
  `**/*` are interpreted differently. The first pattern matches all files that are inside a folder. The second pattern recursively matches all files
  **and sub-folders** inside a folder.

  Let's take an example. Given the following file hierarchy of a project...

  ```
  ├── biome.json
  ├── src
  │   ├── file.js
  │   ├── file.ts
  │   ├── out.gen.js
  │   ├── file.test.js
  │   └── test
  │       └── special.test.js
  └── test ...
  ```

  ...we want:
  1. Ignore all files ending with `.test.js`, except `special.test.ts`.
  2. Ignore all files of the `test` directory. The `test` directory is located at the root of the project.
  3. Execute the linter on files in the `src` directory, that don't end with `.gen.js`. The
     `src` directory is located at the root of the project.
  4. Enable the `noDefaultExport` lint rule on files ending with `.ts`.

  Prior to Biome 2.0, the configuration might look like:

  ```json
  {
    "files": {
      "ignore": ["*.test.js", "test"]
    },
    "linter": {
      "include": ["src/**"],
      "ignore": ["*.gen.js"],
      "enabled": true
    },
    "overrides": [
      {
        "include": ["*.ts"],
        "linter": { "rules": { "style": { "noDefaultExport": "on" } } }
      }
    ]
  }
  ```

  Unfortunately, the configuration doesn't quite fit what we want:
  1. There is no way to ignore files and unignore one of them. Thus, we ignore all files ending with
     `.test.js`, including `special.test.ts`.
  2. The configuration ignores all directories named `test`, including `src/test`.
  3. The linter is executed on all files of all directories named `src`

  All these issues and limitations are fixed with `includes`. Here the migrated configuration:

  ```json
  {
    "files": {
      "includes": ["**", "!**/*.test.js", "**/special.test.ts", "!test"]
    },
    "linter": {
      "includes": ["src/**", "!**/*.gen.js"],
      "enabled": true
    },
    "overrides": [
      {
        "includes": ["**/*.ts"],
        "linter": { "rules": { "style": { "noDefaultExport": "on" } } }
      }
    ]
  }
  ```

  1. All files named
     `special.test.ts` are unignored because the pattern appear after the pattern that ignore files ending with
     `.test.js`.
  2. Only the `test` directory at the project's root is ignored because the pattern doesn't start with `**/`.
  3. The linter is executed on the `src` directory at the project's root only.

  Because `includes` pattern have a different pattern format than `include` and `ignore` we made some adjustments:
  - We added the pattern `**` in `files.includes` to ensure that all files are included before ignoring some of them.
  - We added the prefix `**/` for patterns that must match at any level of the file hierarchy.

- `noUndeclaredVariables` no longer reports TypeScript types.

  In TypeScript projects, developers often use global declaration files to declare global types. Biome is currently unable to detect these global types. This creates many false positives for
  `noUndeclaredVariables`.

  TypeScript is better suited to perform this kind of check. As proof of this, TypeScript ESLint doesn't provide any rule that extends the
  `no-undef` ESLint rule.

  This is why Biome 1.9 introduced a new option
  `checkTypes` which, when it is set to false, ignores undeclared type references. The option was set to
  `true` by default.

  This option is now set to `false` by default. To get the previous behavior, you have to set `checkTypes` to `true`:

  ```json
  {
    "linter": {
      "rules": {
        "correctness": {
          "noUndeclaredVariables": {
            "level": "on",
            "options": { "checkTypes": true }
          }
        }
      }
    }
  }
  ```

- The rule `noUnusedVariables` no longer reports unused function parameters. Use [`noUnusedFunctionParameters`](https://biomejs.dev/linter/rules/no-unused-function-parameters/).

- Fixed [#5564](https://github.com/biomejs/biome/issues/5564).
  `noTypeOnlyImportAttributes` now ignores files ending with the extension `.ts` when the type field of
  `package.json` is set to `commonjs`.

- The Biome formatter no longer adds a trailing comma in `.json` files, even when
  `json.formatter.trailingCommas` is set to `true`.

- [Prettier 3.4](https://prettier.io/blog/2024/11/26/3.4.0.html) introduced a change in their normalization process of string literals: it no longer unescapes useless escape sequences. Biome now matches the new behavior of Prettier when formatting code. This affects the JSON and JavaScript formatters.

- Reduced accepted values for formatter options:
  - The option `--quote-style` doesn't accept `Single` and `Double` anymore.
  - The option `--quote-properties` doesn't accept `AsNeeded` and `Preserve` anymore.
  - The option `--semicolons` doesn't accept `AsNeeded` and `Always` anymore.
  - The option `--arrow-parenthesis` doesn't accept `AsNeeded` and `Always` anymore.
  - The option `--trailing-commas` doesn't accept `ES5`, `All` and `None` anymore.
  - The option `--attribute-position` doesn't accept `Single` and `Multiline` anymore.

- Removed the option `enumMemberCase` from the lint rule `useNamingConvention`.

  `enumMemberCase` is an option that allows to customize the enforced case for TypeScript's enum members. The option was introduced prior to the
  `conventions` option that allows to do the same thing.

  The following configuration...

  ```json
  {
    "linter": {
      "rules": {
        "style": {
          "useNamingConvention": {
            "level": "on",
            "options": {
              "enumMemberCase": "PascalCase"
            }
          }
        }
      }
    }
  }
  ```

  ...must be rewritten as:

  ```json
  {
    "linter": {
      "rules": {
        "style": {
          "useNamingConvention": {
            "level": "on",
            "options": {
              "conventions": [
                {
                  "selector": { "kind": "enumMember" },
                  "formats": ["PascalCase"]
                }
              ]
            }
          }
        }
      }
    }
  }
  ```

  Run `biome migrate --write` to turn `enumMemberCase` into `conventions` automatically.

- Removed support for `BIOME_LOG_DIR`.

  The environment variable `BIOME_LOG_DIR` isn't supported anymore.

  Use `BIOME_LOG_PATH` instead.

- Remove deprecated rules.

  The following _deprecated_ rules have been deleted:
  - `noInvalidNewBuiltin`
  - `noNewSymbol`
  - `useShorthandArrayType`
  - `useSingleCaseStatement`
  - `noConsoleLog`

  Run the command `biome migrate --write` to update the configuration.

- Removed the deprecated `indentSize` option.

  The deprecated option `indentSize`, and its relative CLI options, has been removed:
  - Configuration file: `formatter.indentSize`
  - Configuration file: `javascript.formatter.indentSize`
  - Configuration file: `json.formatter.indentSize`
  - CLI option `--indent-size`
  - CLI option `--javascript-formatter-indent-size`
  - CLI option `--json-formatter-indent-size`

  Use `indentWidth` and its relative CLI options instead.

- Removed `ROME_BINARY`. Use `BIOME_BINARY` instead.

- Removed support for legacy suppressions.

  Biome used to support "legacy suppressions" that looked like this:

  ```js
  // biome-ignore lint(complexity/useWhile): reason
  ```

  This format is no longer supported.

- Removed support for `max_line_length` from `.editorconfig`, as it isn't part of the official spec anymore.

- Removed support for `rome-ignore` suppression comments.

  Use `biome-ignore` suppression comments instead.

- Removed support for `rome.json`.

  Use `biome.json` or `biome.jsonc` instead.

- Removed the option `all` from the linter.

  The options `linter.rules.all` and `linter.rules.<group>.all` has been removed.

  The number of rules in Biome have increased in scope and use cases, and sometimes some of them can conflict with each other.

  The option was useful at the beginning, but now it's deemed harmful, because it can unexpected behaviours in users projects.

  To automatically remove it, run the following command:

  ```shell
  biome migrate --write
  ```

- Removed the option `trailingComma` from the configuration and the CLI. Use the option `trailingCommas` instead:

  ```diff
  {
    "javascript": {
      "formatter": {
  -      "trailingComma": "es5"
  +      "trailingCommas": "es5"
      }
    }
  }
  ```

  ```diff
  -biome format --trailing-comma=es5
  +biome format --trailing-commas=es5
  ```

- Removed `--apply` and `--apply-unsafe`.

  The CLI options `--apply` and `--apply-unasfe` aren't accepted anymore. Use `--write` and `--write --unafe` instead:

  ```diff
  -biome check --apply-unsafe
  +biome check --write --unsafe
  ```

  ```diff
  -biome check --apply
  +biome check --write
  ```

- Removed support for `assert` syntax.

  Biome now longer supports the `assert` syntax, use the new `with` syntax instead

  ```diff
  -import {test} from "foo.json" assert { for: "for" }
  -export * from "mod" assert { type: "json" }
  +import {test} from "foo.json" with { for: "for" }
  +export * from "mod" with { type: "json" }
  ```

- Fixed [#5495](https://github.com/biomejs/biome/issues/5495): The rule
  [`noBlankTarget`](https://biomejs.dev/linter/rules/no-blank-target/) has been updated to accept the
  `rel="noopener"` in addition to `rel="noreferrer"`. In addition, an option has been added that allows
  `rel="noreferrer"` to be disabled.

  The rule has been moved from the `a11y` group to the `security` group.

- The rule `useImportRestrictions` has been renamed to [`noPrivateImports`](https://biomejs.dev/linter/rules/no-private-imports), and its functionality has been significantly upgraded.

  Previously, the rule would assume that any direct imports from modules inside other directories should be forbidden due to their
  _package private_ visibility.

  The updated rule allows configuring the default visibility of exports, and recognises JSDoc comments to override this visibility. The default visibility is now
  `**public**`, but can be set to `**package**`, or even
  `**private**`. Refer to the [documentation of the rule](https://biomejs.dev/linter/rules/no-private-imports) to understand how to leverage the JSDoc comments.

  `noPrivateImports` is now recommended by default.

- The Biome daemon now reuses its workspace across connections. This allows multiple clients to reuse the same documents and other cached data that we extract from them.

  This primarily affects our IDE extensions: If you open multiple IDEs/windows for the same project, they'll connect to the same daemon and reuse each other's workspace.

  The Biome CLI is unaffected unless you opt in with the `--use-server` argument.

- Biome no longer treats too large files as errors.

  Previously, files that exceed the configured size limit would throw an error, and the CLI would exit with an error code.

  Now, the CLI ignores the file, emits an _information_ diagnostic and doesn't exit with an error code.

- Change the group of some rules, promote nursery rules and update the recommended rule set.

  The following rules have been moved to a new group:
  - [complexity/noArguments](https://biomejs.dev/linter/rules/no-arguments)
  - [complexity/noCommaOperator](https://biomejs.dev/linter/rules/no-comma-operator)
  - [complexity/noFlatMapIdentity](https://biomejs.dev/linter/rules/no-flat-map-identity)
  - [complexity/noUselessContinue](https://biomejs.dev/linter/rules/no-useless-continue)
  - [complexity/useNumericLiterals](https://biomejs.dev/linter/rules/use-numeric-literals)
  - [correctness/useValidTypeof](https://biomejs.dev/linter/rules/use-valid-typeof)
  - [performance/noNamespaceImport](https://biomejs.dev/linter/rules/no-namespace-import/)
  - [style/useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals)
  - [suspicious/noWith](https://biomejs.dev/linter/rules/no-with)

  New rules are incubated in the nursery group. Once stable, we promote them to a stable group. Use the
  `biome migrate` command to automatically migrate nursery rules that have been promoted.

  The following CSS rules have been promoted:
  - [correctness/noMissingVarFunction](https://biomejs.dev/linter/rules/no-missing-var-function)
  - [correctness/noUnknownPseudoClass](https://biomejs.dev/linter/rules/no-unknown-pseudo-class)
  - [correctness/noUnknownPseudoElement](https://biomejs.dev/linter/rules/no-unknown-pseudo-element)
  - [correctness/noUnknownTypeSelector](https://biomejs.dev/linter/rules/no-unknown-type-selector)
  - [style/noDescendingSpecificity](https://biomejs.dev/linter/rules/no-descending-specificity)
  - [style/noValueAtRule](https://biomejs.dev/linter/rules/no-value-at-rule)
  - [suspcious/noDuplicateCustomProperties](https://biomejs.dev/linter/rules/no-duplicate-custom-properties)
  - [suspcious/noDuplicateProperties](https://biomejs.dev/linter/rules/no-duplicate-properties)

  The following GraphQL rules have been promoted:
  - [style/useDeprecatedReason](https://biomejs.dev/linter/rules/use-deprecated-reason)
  - [suspicious/noDuplicatedFields](https://biomejs.dev/linter/rules/no-duplicated-fields)

  The following JavaScript rules have been promoted:
  - [a11y/noStaticElementInteractions](https://biomejs.dev/linter/rules/no-static-element-interactions)
  - [a11y/useAriaPropsSupportedByRole](https://biomejs.dev/linter/rules/use-aria-props-supported-by-role)(recommended)
  - [a11y/useValidAutocomplete](https://biomejs.dev/linter/rules/use-valid-autocomplete)
  - [complexity/noUselessEscapeInRegex](https://biomejs.dev/linter/rules/no-useless-escape-in-regex)
  - [complexity/noUselessStringRaw](https://biomejs.dev/linter/rules/no-useless-string-raw)
  - [performance/noDynamicNamespaceImportAccess](https://biomejs.dev/linter/rules/no-dynamic-namespace-import-access)
  - [performance/noImgElement](https://biomejs.dev/linter/rules/no-img-element)
  - [style/noCommonJs](https://biomejs.dev/linter/rules/no-common-js)
  - [style/noEnum](https://biomejs.dev/linter/rules/no-enum)
  - [style/noExportedImports](https://biomejs.dev/linter/rules/no-exported-imports)
  - [style/noHeadElement](https://biomejs.dev/linter/rules/no-head-element)
  - [style/noNestedTernary](https://biomejs.dev/linter/rules/no-nested-ternary)
  - [style/noProcessEnv](https://biomejs.dev/linter/rules/no-process-env)
  - [style/noRestrictedImports](https://biomejs.dev/linter/rules/no-restricted-imports)
  - [style/noRestrictedTypes](https://biomejs.dev/linter/rules/no-restricted-types)
  - [style/noSubstr](https://biomejs.dev/linter/rules/no-substr)
  - [style/useAtIndex](https://biomejs.dev/linter/rules/use-at-index)
  - [style/useCollapsedIf](https://biomejs.dev/linter/rules/use-collapsed-if)
  - [style/useComponentExportOnlyModules](https://biomejs.dev/linter/rules/use-component-export-only-modules)
  - [style/useConsistentCurlyBraces](https://biomejs.dev/linter/rules/use-consistent-curly-braces)
  - [style/useConsistentMemberAccessibility](https://biomejs.dev/linter/rules/use-consistent-member-accessibility)
  - [style/useTrimStartEnd](https://biomejs.dev/linter/rules/use-trim-start-end)
  - [suspicious/noDocumentCookie](https://biomejs.dev/linter/rules/no-document-cookie)
  - [suspicious/noDocumentImportInPage](https://biomejs.dev/linter/rules/no-document-import-in-page)
  - [suspicious/noDuplicateElseIf](https://biomejs.dev/linter/rules/no-duplicate-else-if)
  - [suspicious/noHeadImportInDocument](https://biomejs.dev/linter/rules/no-head-import-in-document)
  - [suspicious/noIrregularWhitespace](https://biomejs.dev/linter/rules/no-irregular-whitespace)
  - [suspicious/noOctalEscape](https://biomejs.dev/linter/rules/no-octal-escape)
  - [suspicious/noTemplateCurlyInString](https://biomejs.dev/linter/rules/no-template-curly-in-string)
  - [suspicious/useAdjacentOverloadSignatures](https://biomejs.dev/linter/rules/use-adjacent-overload-signatures)
  - [suspicious/useGoogleFontDisplay](https://biomejs.dev/linter/rules/use-google-font-display)
  - [suspicious/useGuardForIn](https://biomejs.dev/linter/rules/use-guard-for-in)
  - [suspicious/useStrictMode](https://biomejs.dev/linter/rules/use-strict-mode)

  Moreover, the following JavaScript rules are now recommended:
  - [complexity/noUselessUndefinedInitialization](https://biomejs.dev/linter/rules/no-useless-undefined-initialization)
  - [complexity/useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals)
  - [correctness/noConstantMathMinMaxClamp](https://biomejs.dev/linter/rules/no-constant-math-min-max-clamp)
  - [correctness/noUnusedFunctionParameters](https://biomejs.dev/linter/rules/no-unused-function-parameters) (recommended by ESLint)
  - [correctness/noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports)
  - [correctness/noUnusedPrivateClassMembers](https://biomejs.dev/linter/rules/no-unused-private-class-members) (recommended by ESLint)
  - [correctness/noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables) (recommended by ESLint)
  - [complexity/useDateNow](https://biomejs.dev/linter/rules/use-date-now)

  And the following style rules are no longer recommended:
  - [style/useNumberNamespace](https://biomejs.dev/linter/rules/use-number-namespace)
  - [style/useAsConstAssertion](https://biomejs.dev/linter/rules/use-as-const-assertion)
  - [style/noParameterAssign](https://biomejs.dev/linter/rules/no-parameter-assign)
  - [style/noInferrableTypes](https://biomejs.dev/linter/rules/no-inferrable-types)
  - [style/useDefaultParameterLast](https://biomejs.dev/linter/rules/use-default-parameter-last)
  - [style/noUnusedTemplateLiteral](https://biomejs.dev/linter/rules/no-unused-template-literal)
  - [style/useEnumInitializers](https://biomejs.dev/linter/rules/use-enum-initializers)
  - [style/noUselessElse](https://biomejs.dev/linter/rules/no-useless-else)
  - [style/useSelfClosingElements](https://biomejs.dev/linter/rules/use-self-closing-elements)
  - [style/useSingleVarDeclarator](https://biomejs.dev/linter/rules/use-single-var-declarator)

- Update the default severity level of lint rules.

  Every diagnostic emitted by Biome has a severity level set to `error`, `warn`, or
  `info`. Previously, all recommended lint rules had a default severity level set to
  `error`. All other lint rules had a default severity level set to `warn`.

  We have adjusted the default severity level of every rule, whether recommended or not, to better communicate the
  _severity_ that a diagnostic highlights.
  - Rules that report hard errors, likely erroneous code, dangerous code, or accessibility issues now have a default severity level of
    `error`.
  - Rules that report possibly erroneous codes, or code that could be cleaner if rewritten in another way now have a default severity level of
    `warn`.
  - Rules that reports stylistic suggestions now have a default severity level of `info`.

  You can use the CLI option `--diagnostic-level=error` to display only errors, or
  `--diagnostic-level=warning` to display both errors and warnings. By default, all diagnostics are shown. You can also use the CLI option
  `--error-on-warnings` to make the command fail when warnings are emitted.

- Reworked some recommended rules recommended to be less pedantic and blocking. This is a **breaking change
  ** if your project relied on those rules to block the CI in case of violations; if that's the case, you should raise their severity level to
  **error**.

  Some rules aren't recommended anymore, and some others return a different severity.

  The following rules return a **warning** diagnostic:
  - `noDelete`
  - `noForEach`
  - `noSuspiciousSemicolonInJsx`
  - `noThisInStatic`
  - `noUnusedLabels`

  The following rules return an **information** diagnostic:
  - `noUselessCatch`
  - `noUselessConstructor`
  - `noUselessEmptyExport`
  - `noUselessFragments`
  - `noUselessLabel`
  - `noUselessLoneBlockStatements`
  - `noUselessSwitchCase`
  - `noUselessTernary`
  - `noUselessThisAlias`
  - `noUselessTypeConstraint`
  - `noFlatMapIdentity`

  The following rules aren't recommended anymore:
  - `noDelete`
  - `noForEach`

  The rule `noRenderReturnValue` and `useExhaustiveDependencies` are only recommended when the
  `react` domain is enabled.

- Renamed the global option `--skip-errors` to `--skip-parse-errors`.

- Remove the code action `quickfix.suppressRule`.

  The code action `quickfix.suppressRule` was removed in favour of two new code actions:
  - `quickfix.suppressRule.inline.biome`: a code action that adds a suppression comment for each violation.
  - `quickfix.suppressRule.topLevel.biome`: a code action that adds a suppression comment at the top of the file which suppresses a rule for the whole file.

  Given the following code

  ```js
  let foo = "one";
  debugger;
  ```

  The code action `quickfix.suppressRule.inline.biome` will result in the following code:

  ```js
  // biome-ignore lint/style/useConst: <explanation>
  let foo = "one";
  // biome-ignore lint/suspicious/noDebugger: <explanation>
  debugger;
  ```

  The code action `quickfix.suppressRule.topLevel.biome`, instead, will result in the following code:

  ```js
  /** biome-ignore lint/suspicious/noDebugger: <explanation> */
  /** biome-ignore lint/style/useConst: <explanation> */

  let foo = "one";
  debugger;
  ```

- Changed default formatting of `package.json`.

  When Biome encounters a file called
  `package.json`, by default it will format the file with all objects and arrays expanded.

  ```diff
  - { "name": "project", "dependencies": { "foo": "latest" } }
  + {
  +  "projectName": "project",
  +  "dependencies": {
  +    "foo": "^1.0.0"
  +  }
  + }
  ```

- The `organizeImports` is now part of Biome Assist.

- The rule [`noVar`](https://biomejs.dev/linter/rules/no-var/) now belongs to the `suspicious` group

- The rule [`useWhile`](https://biomejs.dev/linter/rules/use-while/) now belongs to the `complexity` group.

- The rule [`useImportExtensions`](https://biomejs.dev/linter/rules/use-import-extensions/) has been updated to suggest actual file extensions instead of guesses based on heuristics.

  As part of this, the `suggestedExtensions` option has been removed. A simpler, new option called
  `forceJsExtensions` has been introduced for those who use
  `tsc`'s `"module": "node16"` setting.

  The rule also no longer reports diagnostics to add an extension when the path doesn't exist at all, with or without extension.

- Fixed [#4545](https://github.com/biomejs/biome/issues/4545): [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) now correctly ignores declarations inside TypeScript's external modules.

  The following interface name is no longer reported by the rule:

  ```ts
  declare module "myExternalModule" {
    export interface my_INTERFACE {}
  }
  ```

- The rule [`useAltText`](https://biomejs.dev/linter/rules/use-alt-text/) no longer checks the element's attributes containing object spread.

  The following code doesn't trigger the rule anymore:

  ```jsx
  <img src="test.png" alt={alt} {...restProps}></img>
  ```

- The rule [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) no longer accepts non-ASCII characters by default.

  Prior to Biome 2.0, non-ASCII names were accepted by default. They are now rejected.

  For example, the following code is now reported as invalid by the `useNamingConvention` rule.

  ```js
  let johnCafé;
  ```

  If you want to allow non ASCII filenames and non-ASCII identifiers, you need to set the
  `requireAscii` options in your Biome configuration file to `false`:

  ```json
  {
      "linter": {
          "rules": {
              "style": {
                  "useFilenamingConvention": {
                      "level": "on",
                      "options": {
                          "requireAscii": false
                      }
                  }
                  "useFilenamingConvention": {
                      "level": "on",
                      "options": {
                          "requireAscii": false
                      }
                  }
              }
          }
      }
  }
  ```

- Renamed the rule `noUnnecessaryContinue` to `noUselessContinue`. Run the command
  `biome migrate` to update your configuration.

- Renamed the rule `noMultipleSpacesInRegularExpressionLiterals` to `noAdjacentSpacesInRegex`. Run the command
  `biome migrate` to update your configuration.

### Minor Changes

- An option called `allowNoReferrer` has been added to the
  [`noBlankTarget`](https://biomejs.dev/linter/rules/no-blank-target/) rule.

  By default, `noBlankTarget` accepts both `rel="noopener"` and `rel="noreferrer"`
  with links that have `target="_blank"`. This is because the latter
  _implies_ the former, so either one is sufficient to mitigate the security risk.

  However, allowing
  `rel="noreferrer"` may still be undesirable, because it can break tracking, which may be an undesirable side-effect. As such, you can set
  `allowNoReferrer: false` to _only_ accept `rel="noopener"`.

- Added new option
  `javascript.parser.jsxEverywhere`. This new option allows to control whether Biome should expect JSX syntax in `.js`/
  `.mjs`/`.cjs` files.

  When `jsxEverywhere` is set to `false`, having JSX syntax like `<div></div>` inside `.js`/`.mjs`/
  `.cjs` files will result in a **parsing error**.

  Despite the name of the option, JSX is never supported inside
  `.ts` files. This is because TypeScript generics syntax may conflict with JSX in such files.

  This option defaults to `true`.

- Add a new JS assist rule -
  `useSortedKeys` which enforces ordering of a JS object properties. This rule will consider spread/calculated keys e.g
  `[k]: 1` as non-sortable. Instead, whenever it encounters a non-sortable key, it will sort all the previous sortable keys up until the nearest non-sortable key, if one exist. This prevents breaking the override of certain keys using spread keys.

  Source: https://perfectionist.dev/rules/sort-objects

  ```js
  // Base
  // from
  const obj = {
    b: 1,
    a: 1,
    ...g,
    ba: 2,
    ab: 1,
    set aab(v) {
      this._aab = v;
    },
    [getProp()]: 2,
    aba: 2,
    abc: 3,
    abb: 3,
    get aaa() {
      return "";
    },
  };
  // to
  const obj = {
    a: 1,
    b: 1,
    ...g,
    set aab(v) {
      this._aab = v;
    },
    ab: 1,
    ba: 2,
    [getProp()]: 2,
    get aaa() {
      return "";
    },
    aba: 2,
    abb: 3,
    abc: 3,
  };
  ```

- Added the new rule [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises).

- Added the new rule [`noImportCycles`](https://biomejs.dev/linter/rules/no-import-cycles).

- Added the new rule [`noTsIgnore`](https://biomejs.dev/linter/rules/no-ts-ignore).

- Added the new rule [`noUnwantedPolyfillio`](https://biomejs.dev/linter/rules/no-unwanted-polyfillio).

- Added a format option
  `expand` for Javascript and JSON formatters. The option allows to enforce the formatting of arrays and objects on multiple lines, regardless of their length. It has three options:

  When set to
  `auto` (default), objects are expanded if the first property has a leading newline. Arrays are collapsed when they fit to a single line. For example, both styles below are considered as already formatted:

  ```js
  const obj = {
    foo: "bar",
  };
  ```

  ```js
  const obj = { foo: "bar" };
  ```

  When set to `always`, objects and arrays are always expanded.

  When set to
  `never`, objects and arrays are never expanded when they fit in a single line. It is equivalent to Prettier's [Object Wrap](https://prettier.io/docs/options#object-wrap) option with
  `collapse`.

- The nursery rule [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/) has been added.

  Importing a non-existing export is an error at runtime or build time. With this rule, Biome can detect such incorrect imports and report errors for them.

  Note that if you use TypeScript, you probably don't want to use this rule, since TypeScript already performs such checks for you.

- The rule [`noFocusedTests`](https://biomejs.dev/linter/rules/no-focused-tests/) can now detect the usage of focused tests inside loops.

  ```js
  // invalid
  describe.only.each([["a"], ["b"]])("%s", (a) => {});
  it.only.each([["a"], ["b"]])("%s", (a) => {});
  test.only.each([["a"], ["b"]])("%s", (a) => {});

  // valid
  describe.each([["a"], ["b"]])("%s", (a) => {});
  it.each([["a"], ["b"]])("%s", (a) => {});
  test.each([["a"], ["b"]])("%s", (a) => {});
  ```

- Linter groups now accept new options to enable/disable all rules that belong to a group, and control the severity of the rules that belong to those groups.

  For example, you can downgrade the severity of rules that belong to `"style"` to emit `"info"` diagnostics:

  ```json
  {
    "linter": {
      "rules": {
        "style": "info"
      }
    }
  }
  ```

  You can also enable all rules that belong to a group using the default severity of the rule using the `"on"` option:

  ```json
  {
    "linter": {
      "rules": {
        "complexity": "on"
      }
    }
  }
  ```

- Biome assist is a new feature of the Biome analyzer. The assist is meant to provide **actions
  **. Actions differ from linter rules in that they aren't meant to signal errors.

  The assist will provide code actions that users can opt into via configuration or via IDEs/editors, using the Language Server Protocol.

  The assist **is enabled by default**. However, you can turn if off via configuration:

  ```json
  {
    "assist": {
      "enabled": false
    }
  }
  ```

  You can turn on the actions that you want to use in your configuration. For example, you can enable the
  `useSortedKeys` action like this:

  ```json
  {
    "assist": {
      "actions": {
        "source": {
          "useSortedKeys": "on"
        }
      }
    }
  }
  ```

  Alternatively, IDE/editor users can decide which action to apply on save _directly from the editor
  settings_, as long as the assist is enabled.

  For example, in VS Code you can apply the `useSortedKeys` action when saving a file by adding the following snippet in
  `settings.json`:

  ```json
  {
    "editor.codeActionsOnSave": {
      "source.biome.useSortedKeys": "explicit"
    }
  }
  ```

  In Zed, you can achieve the same by adding the following snippet in `~/.config/zed/settings.json`:

  ```json
  {
    "code_actions_on_format": {
      "source.biome.useSortedKeys": true
    }
  }
  ```

- Biome migrate eslint outputs a better overriding behavior.

  A Biome rule can have multiple ESLint equivalent rules. For example, [useLiteralKeys](https://biomejs.dev/linter/rules/use-literal-keys/) has two ESLint equivalent rules: [dot-notation](https://eslint.org/docs/latest/rules/dot-notation) and [@typescript-eslint/dot-notation](https://typescript-eslint.io/rules/dot-notation/).

  Previously, Biome wouldn't always enable a Biome rule even if one of its equivalent rules was enabled. Now Biome uses the higher severity level of all the equivalent ESLint rules to set the severity level of the Biome rule.

  The following ESLint configuration...

  ```json
  {
    "rules": {
      "@typescript-eslint/dot-notation": "error",
      "dot-notation": "off"
    }
  }
  ```

  ...is now migrated to...

  ```json
  {
    "linter": {
      "rules": {
        "complexity": {
          "useLiteralKeys": "error"
        }
      }
    }
  }
  ```

  ...because `error` is higher than `off`.

- Add [useSymbolDescription](https://biomejs.dev/linter/rules/use-symbol-description/).

- Enhanced the command
  `migrate eslint`. Now the command shows which ESLint rules were migrated, and which rules aren't supported yet.

  ```
  ./eslint.config.js migrate ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    ℹ 42% (3/7) of the rules have been migrated.

    ℹ Migrated rules:

    - getter-return
    - prefer-const
    - @typescript-eslint/require-await

    ℹ Rules that can be migrated to an inspired rule using --include-inspired:

    - @typescript-eslint/parameter-properties

    ℹ Rules that can be migrated to a nursery rule using --include-nursery:

    - @typescript-eslint/switch-exhaustiveness-check

    ℹ Stylistic rules that the formatter may support (manual migration required):

    - semi

    ℹ Unsupported rules:

    - block-scoped-var

  configuration ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    ℹ Migration results:

    - ./biome.json: configuration successfully migrated.
  ```

- Suppression of syntax rules

  Added support for suppressing syntax rules. Syntax rules are particular rules meant **to complement the parser
  **, hence they can't be configured.

  Biome now allows to suppress those rules. This can, for example, be useful in case the rule is affected by a bug. However, this is more an escape hatch, so if a syntax rule requires a suppression, please file an issue.

  Example:

  ```typescript
  // biome-ignore syntax/correctness/noTypeOnlyImportAttributes: bug
  import type { MyType } from "my-esm-pkg" with { "resolution-mode": "import" };
  ```

  Biome now requires all `biome-ignore-start` suppressions to have an equivalent `biome-ignore-end` comment.

- Add a new lint rule
  `noConstantBinaryExpression`. This rule is inspired from ESLint's [no-constant-binary-expression](https://eslint.org/docs/latest/rules/no-constant-binary-expression) rule.

- The CLI options `--only` and `--skip` now accept rule and action names without prefixing the group name.

  Previously `--only=noDebugger` was rejected. You had to add the group name: `--only=suspicious/noDebugger`.

- Fixed [#3574](https://github.com/biomejs/biome/issues/3574):
  `noUnusedImports` now reports empty named imports and suggests their removal.

  The rule now suggests the removal of empty named imports such as:

  ```diff
  - import {} from "mod";
  ```

- Added the new rule [`useAdjacentGetterSetter`](https://biomejs.dev/linter/rules/use-adjacent-getter-setter), which enforces getters and setters for the same property to be adjacent in class and object definitions.

  **Example (Invalid): Name getter and setter are not adjacent:**

  ```js
  class User {
    get name() {
      return this._name;
    }
    constructor() {}
    set name(value) {
      this._name = value;
    }
  }
  ```

  \*\*Example (Invalid): Getter should go before the setter.

  ```js
  const user = {
    set name(value) {
      this._name = value;
    },
    get name() {
      return this._name;
    },
  };
  ```

  **Example (Valid): Name getter and setter are adjacent:**

  ```js
  class User {
    get name() {
      return this._name;
    }
    set name(value) {
      this._name = value;
    }
    get age() {
      return this._age;
    }
    set age(age) {
      this._age = age;
    }
  }
  ```

- Added new rule [useConsistentResponse](https://biomejs.dev/linter/rules/use-consistent-response) which suggests to use static [Response.json()](https://developer.mozilla.org/en-US/docs/Web/API/Response/json) and [Response.redirect()](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect_static) methods instead of
  `new Response` when possible.

  Example:

  ```js
  new Response(JSON.stringify({ value: 1 }));
  Response.json({ value: 1 });
  ```

- Biome users can now configure code actions from linter rules as well as assist actions directly in the settings of their IDE/editor.

  For example, let's consider the lint rule [`noSwitchDeclarations`](https://biomejs.dev/linter/rules/no-switch-declarations/), which has an unsafe fix. Previously, if you wanted to use this rule, you were "forced" to enable it via configuration, and if you wanted to apply its fix when you saved a file, you were forced to mark the fix as safe:

  ```json
  {
    "linter": {
      "rules": {
        "correctness": {
          "noSwitchDeclarations": {
            "level": "error",
            "fix": "safe"
          }
        }
      }
    }
  }
  ```

  Now, you can benefit from the code action without making the fix safe for the entire project. IDEs and editors that are LSP compatible allow to list a series of "filters" or code actions that can be applied on save. In the case of VS Code, you will need to add the following snippet in the
  `settings.json`:

  ```json
  {
    "editor.codeActionsOnSave": {
      "quickfix.biome.correctness.noSwitchDeclarations": "explicit"
    }
  }
  ```

  Upon save, Biome will inform the editor the apply the code action of the rule `noSwitchDeclarations`.

- Fixed [#3401](https://github.com/biomejs/biome/issues/3401):
  `noUnusedImports` now keeps comments separated from the import with a blank line.

  For example:

  ```diff
    // Orphan comment

  - // Header comment
  - import {} from "mod";
  ```

- Added a new `propertyAssignment` option to the
  `noParameterAssign` rule. This option allows to configure whether property assignments on function parameters are permitted. By default,
  `propertyAssignment` is set to `allow`. Setting it to
  `deny` enforces stricter immutability by disallowing property mutations on function parameters.

- The formatter option `bracketSpacing` is now also supported in JSON files.

- `useValidTypeof` now accepts comparisons with variables.

  Previously, the rule required to compare a `typeof` expression against another
  `typeof` expression or a valid string literal. We now accept more cases, notably comparison against a variable:

  ```js
  if (typeof foo === bar) {
    // ...
  }
  ```

- Added the new rule [`noNestedComponentDefinitions`](https://biomejs.dev/linter/rules/no-nested-component-definitions), which disallows nested component definitions in React components.

  This rule is useful for preventing potential performance issues and improving code readability by ensuring that components are defined at the top level.

  **Example (Invalid):**

  ```jsx
  function ParentComponent() {
    function ChildComponent() {
      return <div>Hello</div>;
    }
    return <ChildComponent />;
  }
  ```

  **Example (Valid):**

  ```jsx
  function ChildComponent() {
    return <div>Hello</div>;
  }
  function ParentComponent() {
    return <ChildComponent />;
  }
  ```

- Added the new rule [`noDestructuredProps`](https://biomejs.dev/linter/rules/no-destructured-props/), which disallow the use of destructured props in Solid projects.

- You can now enable lint rules using the default severity suggested by Biome using the new variant
  `"on"`, when enabling a rule.

  For example, the default severity of the rule `style.noVar` is `error`, so you would use
  `"on"`, and then linting a code that uses `var`, will result in an error:

  ```json
  {
    "linter": {
      "recommended": false,
      "rules": {
        "style": {
          "noVar": "on"
        }
      }
    }
  }
  ```

  ```js
  // main.js
  var name = "tobias";
  ```

  The command `biome lint main.js` will result in an error due to the default severity assigned to `noVar`.

  Refer to the documentation page of each rule to know their suggested diagnostic severity, or use the command
  `biome explain <RULE_NAME>`:

  ```shell
  biome explain noVar
  ```

- Biome VCS integration now supports nested ignore files.

  For `git`, if a `.gitignore` is found in a nested folder `root/packages/foo/`, and it contains the pattern
  `dist/`, only files and directories inside `root/packages/foo/dist` are matched.

- Added the rule [useUniqueElementIds](https://biomejs.dev/linter/rules/use-unique-element-ids/). This rule disallows the use of static IDs in React components. It encourages to generate unique IDs for accessibility purposes using [`useId`](https://react.dev/reference/react/useId).

  The following code is now reported as invalid:

  ```jsx
  function App() {
    return <div id="static-id" />;
  }
  ```

  The following code is now reported as valid:

  ```jsx
  import { useId } from "react";
  function App() {
    const id = useId();
    return <div id={id} />;
  }
  ```

- Added the new JavaScript rule [`useConsistentObjectDefinition`](https://biomejs.dev/linter/rules/use-consistent-object-definition/) rule. The rule enforces a consistent style for the definition of objects:

  By default, the rule enforces a shorthand style:

  ```js
  const validShorthand = {
    // Property shorthand
    foo,

    // Method shorthand
    method() {
      return "method";
    },
  };
  ```

  Alternatively, the rule can be configured to enforce an explicit style:

  ```js
  const invalidExplicit = {
    // Basic property shorthand violations
    foo: foo,

    // Method shorthand violations
    method: function () {
      return "method";
    },
  };
  ```

- Introduced more advanced logging capabilities:

  Every Biome CLI command can now be passed a
  `--log-file=<path>` argument, which will write all log messages for that invocation to the given path instead of
  `stdout`.

  In addition, the `--log-level` parameter now also accepts a `tracing` value. When
  `--log-level=tracing` is used, Biome also prints timing information from tracing spans to the log.

  Combined with Biome's ability to print logs in JSON format, and the
  `jq` command line utility, this allows you to perform advanced analysis on Biome's internal performance.

  For example, if you want to figure out which paths take the longest when building the module graph, you can use the following commands:

  ```sh
  biome lint --log-level=tracing --log-kind=json --log-file=tracing.json
  cat tracing.json | jq '. | select(.span.name == "update_module_graph") | { path: .span.path, time_busy: .["time.busy"], time_idle: .["time.idle"] }' > filtered.json
  ```

  Now you will have a file called
  `filtered.json` with all the relevant timings, together with the paths used during the invocations.

- Added options to `suspicious/noConfusingLabels` to allow specific labels.

- Fixed [#4549](https://github.com/biomejs/biome/issues/4549): [noUnknownProperty](https://biomejs.dev/linter/rules/no-unknown-property/) now accepts more known CSS properties.

  ```diff
  - ['anchor-default', 'anchor-scroll', 'inset-area', 'position-animation', 'position-fallback', 'position-fallback-bounds', 'position-try-options']
  + ['anchor-scope', 'interpolate-size', 'line-fit-edge', 'masonry', 'masonry-auto-tracks', 'masonry-direction', 'masonry-fill', 'masonry-flow', 'masonry-slack', 'masonry-template-areas', 'masonry-template-tracks', 'position-anchor', 'position-area', 'position-try-fallbacks', 'position-visibility', 'scroll-start-target', 'text-box', 'view-transition-class', 'view-transition-group']
  ```

  This change replaces deprecated properties, improving CSS validation.

- LSP clients can now override the configuration path for each workspace, by responding to
  `workspace/configuration` requests.

- Added the new CSS rule [`noImportantStyles`](https://biomejs.dev/linter/rules/no-important-styles), which prevents the use of
  `!important` inside CSS declarations.

- Biome now emits a warning diagnostic if the configuration contains an out-of-sync schema URL.

- Introduced a new configuration setting `files.experimentalScannerIgnores`.

  This setting may be used to configure a set of file and folder names that should be unconditionally ignored by Biome's scanner.

  Biome maintains an internal list of default ignore entries, which is based on user feedback and which may change in any release. This setting allows overriding this internal list completely.

  This is considered an advanced feature that users
  _should_ not need to tweak themselves, but they can as a last resort. This setting can only be configured in root configurations, and is ignored in nested configs.

  Entries must be file or folder _names_. Specific paths and globs are not supported.

  Examples where this may be useful:

  ```jsonc
  {
    "files": {
      "experimentalScannerIgnores": [
        // You almost certainly don't want to scan your `.git` folder, which
        // is why it's already ignored by default:
        ".git",

        // But the scanner does scan `node_modules` by default. If you
        // *really* don't want this, you can ignore it like this:
        "node_modules",

        // But it's probably better to ignore a specific dependency.
        // For instance, one that happens to be particularly slow to scan:
        "RedisCommander.d.ts",
      ],
    },
  }
  ```

  Please be aware that rules relying on the module graph or type inference information may be negatively affected if dependencies of your project aren't
  (fully) scanned.

- Added the new rule [useSingleJsDocAsterisk](https://biomejs.dev/linter/rules/use-single-js-doc-asterisk/) which enforces JSDoc comment lines to start with a single asterisk.

  ```js
  // Invalid
  /**
   ** Description
   */

  // Valid
  /**
   * Description
   */
  ```

- The CLI flag `--javascript-attribute-position` was renamed to
  `--javascript-formatter-attribute-position` for consistency.

- Introduced the `domains` linter feature. The Biome linter now has a new way to opt-in rules, with a concept called
  `domains`.

  Domains can be seen as concepts shared by different rules.

  You can enable and disable multiple rules that belong to a domain. When you assign
  `"all"`, Biome will enable all the rules, when you assign
  `"none"`, Biome will disable the rules, when you assign "recommended", Biome will enable all rules of the domain that are recommended.

  ```json5
  // biome.jsonc
  {
    linter: {
      domains: {
        test: "all", // all rules that belong to this domain are enabled
        react: "recommended", // only the recommended rules from this domain are enabled
        solid: "none", // rules related to Solid are disabled
      },
    },
  }
  ```

  New domains introduced:
  - `test`: it will enable rules:
    - `noExportsInTest`
    - `noExcessiveNestedTestSuites`
    - `noDuplicateTestHooks`
    - `noFocusedTests`
      And it will inject the following globals:
    - `after`
    - `afterAll`
    - `afterEach`
    - `before`
    - `beforeEach`
    - `beforeAll`
    - `describe`
    - `it`
    - `expect`
    - `test`
  - `next`: it will enable rules for Next.js projects:
    - `useExhaustiveDependencies`
    - `useHookAtTopLevel`
    - `noImgElement`
    - `noHeadImportInDocument`
    - `noHeadImportInDocument`
  - `react`: it will enable rules for React projects:
    - `useExhaustiveDependencies`
    - `useHookAtTopLevel`
  - `solid`: it will enable rules for Solid projects:
    - `noReactSpecificProps`

  For more information regarding how Biome enables rules via domains, please refer to the documentation page of each rule.

- Biome now prints diagnostics sorted by their severity. The order is the following:
  1. information
  2. warning
  3. error

  This means that _error_ diagnostics are printed **last**, so users can see them first.

- Added the new rule [`useIterableCallbackReturn`](https://biomejs.dev/linter/rules/use-iterable-callback-return), which enforces consistent return values in iterable callbacks.

  The following methods require a return value in their callback:
  - `every`
  - `filter`
  - `find`
  - `findIndex`
  - `findLast`
  - `findLastIndex`
  - `flatMap`
  - `map`
  - `reduce`
  - `reduceRight`
  - `some`
  - `sort`
  - `toSorted`
    — `from` (when called on `Array`)

  The rule disallows a return value inside the callback of the method `forEach`.

  Examples:

  ```js
  [].map(() => {
    // Missing return value
  });
  ```

  ```js
  [].forEach(() => {
    return 1; // Disallowed
  });
  ```

- Added the new rule [`noReactPropAssign`](https://biomejs.dev/linter/rules/no-react-prop-assign), based on the react-hooks rule [react-hooks/react-compiler](https://www.npmjs.com/package/eslint-plugin-react-hooks)

  The following code is now reported as invalid:

  ```jsx
  function Foo(props) {
    props.bar = `Hello ${props.bar}`;
    return <div>{props.bar}</div>;
  }
  ```

  The following code is now reported as valid:

  ```jsx
  function Foo({ bar }) {
    bar = `Hello ${bar}`;
    return <div>{bar}</div>;
  }
  ```

- Added new rule [`noBitwiseOperators`](https://biomejs.dev/linter/rules/no-bitwise-operators/), which disallows bitwise operators.

- The Biome analyzer now supports a new top-level suppression. These suppression have to be placed at the top of the file, and they must be followed by two newlines (
  `\n\n\`).

  The analyzer rules specified inside the block comment will be suppressed for the whole file.

  In the example, we suppress the rules `lint/style/useConst` and `lint/suspicious/noDebugger` for the whole file:

  ```js
  // main.js
  /**
   * biome-ignore-all lint/style/useConst: i like let
   * biome-ignore-all lint/suspicious/noDebugger: needed now
   */

  let path = "/path";
  let _tmp = undefined;
  debugger;
  ```

  In this other example, we suppress `lint/suspicious/noEmptyBlock` for a whole CSS file:

  ```css
  /**
  /* biome-ignore-all lint/suspicious/noEmptyBlock: it's fine to have empty blocks
  */

  a {
  }
  span {
  }
  ```

  A new diagnostic is emitted if `biome-ignore-all` suppression isn't placed at the top of the file:

  ```block
  file.js:3:1 suppressions/incorrect ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    ! Top level suppressions can only be used at the beginning of the file.

      2 │ let foo = 2;
    > 3 │ /**
        │ ^^^
    > 4 │ * biome-ignore-all lint/style/useConst: reason
    > 5 │ */
        │ ^^
      6 │ let bar = 33;

    i Rename this to biome-ignore

      2 │ let foo = 2;
      3 │ /**
    > 4 │ * biome-ignore-all lint/style/useConst: reason
        │   ^^^^^^^^^^^^^^^^
      5 │ */
      6 │ let bar = 33;


  ```

- Added the new rule [`useNumericSeparators`](https://biomejs.dev/linter/rules/use-numeric-separators), which encourages the use of numeric separators to improve readability.

- [useImportExtensions](https://biomejs.dev/linter/rules/use-import-extensions/) now checks imports with sub extensions.

  ```js
  - import 'styles.css'
  + import 'styles.css.ts'
  ```

- It's possible to override the option
  `files.maxSize`. This option is helpful if you need to process specific files that exceed the default `maxSize`:

  ```json
  {
  	"overrides": [{
  		"includes": ["dist/**"]
  		"files": {
  			"maxSize": 20000
  		}
  	}]
  }
  ```

- Added the new CLI option called `--threads` to the
  `ci` command. It allows to control the numbers of threads that can be used when using the Biome CLI.

  It's possible to use the environment variable `BIOME_THREADS` as an alternatives.

  This feature is useful when running the CLI in environments that have limited resources, for example CI/CD.

  ```shell
  biome ci --threads=1
  BIOME_THREADS=1 biome ci
  ```

- Added the new rule [`useObjectSpread`](https://biomejs.dev/linter/rules/use-object-spread), which prefers object spread syntax over
  `Object.assign()` when constructing new objects.

  **Example (Invalid): Using Object.assign with an empty object:**

  ```js
  Object.assign({}, foo);
  Object.assign({}, { foo: "bar" });
  ```

  **Example (Invalid): Using Object.assign with object literal as first argument:**

  ```js
  Object.assign({ foo: "bar" }, baz);
  Object.assign({}, baz, { foo: "bar" });
  ```

  **Example (Valid): Using object spread syntax:**

  ```js
  ({ ...foo });
  ({ ...baz, foo: "bar" });
  ```

  **Example (Valid): Modifying existing objects is allowed:**

  ```js
  Object.assign(foo, { bar: baz });
  Object.assign(foo, bar, baz);
  ```

- Added an option to the `lint` command called
  `--suppress`. The new option suppresses a violation instead of applying a rule fix. The option accepts a string that is used as
  _reason_ of the suppression comment.

  When running the following command, it will add the suppression comment:

  ```shell
  biome lint --write --suppress="Migration to Biome"
  ```

  ```js
  debugger;
  foo == bar;
  ```

  ```diff
  + // biome-ignore lint/suspicious/noDebugger: Migration to Biome
  debugger;
  + // biome-ignore lint/suspicious/noDoubleEquals: Migration to Biome
  foo == bar;
  ```

- Add an `ignoreRestSiblings` option into [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables).

  When this option is set to `false`, the rule will **not** ignore variables that created using the rest pattern:

  ```json
  {
    "linter": {
      "rules": {
        "correctness": {
          "noUnusedVariables": {
            "level": "error",
            "options": {
              "ignoreRestSiblings": false
            }
          }
        }
      }
    }
  }
  ```

  ```js
  const { lorem, ...test } = bar; // the variable "test" will trigger the rule
  console.log(lorem);
  ```

- Upgraded some unsafe fixes to safe fixes.

  The following rules have now a safe fix:
  - [noExtraBooleanCast](https://biomejs.dev/linter/rules/no-extra-boolean-cast)
  - [noNonoctalDecimalEscape](https://biomejs.dev/linter/rules/no-nonoctal-decimal-escape)
  - [noSwitchDeclarations](https://biomejs.dev/linter/rules/no-switch-declarations)
  - [noThisInStatic](https://biomejs.dev/linter/rules/no-this-in-static)
  - [noUnusedTemplateLiteral](https://biomejs.dev/linter/rules/no-unused-template-literal)
  - [noUselessContinue](https://biomejs.dev/linter/rules/no-useless-continue)
  - [noUselessElse](https://biomejs.dev/linter/rules/no-useless-else)
  - [noUselessStringConcat](https://biomejs.dev/linter/rules/no-useless-string-concat)
  - [useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals)
  - [useExponentiationOperator](https://biomejs.dev/linter/rules/use-exponentiation-operator)
  - [useNumberToFixedDigitsArgument](https://biomejs.dev/linter/rules/use-number-to-fixed-digits-argument)
  - [useNumericLiterals](https://biomejs.dev/linter/rules/use-numeric-literals)
  - [useSimplifiedLogicExpression](https://biomejs.dev/linter/rules/use-simplified-logic-expression)

- Added support for monorepos. The feature will work _out of the box_ for the majority of the users. If your project has
  **nested configuration** files, use the command `biome migrate` from the _root of the project_.

  Monorepo support in Biome is done in a single way. Create a
  `biome.json` at the root of the project. This configuration file is now called the root configuration. Then, each nested configuration file must specify the new field
  `"root": false`.

  We also introduced a new microsyntax for _extending a nested configuration from the root configuration_, which is
  `"extends": "//"`. This new syntax means “this config
  _extends_ from the root config”. When using this microsyntax, you **may omit** the
  `"root": false` field as it is implied.

  Note that nested configs are not required to extend from the root config, and you can still have independent nested configs, as well as nested configs that extend from other files. In those cases,
  `"root": false` must be specified explicitly.

- Added support for formatting `.html` files. The formatting is considered **experimental,
  ** and it's only opt-in via configuration:

  ```json
  {
    "html": {
      "formatter": {
        "enabled": true
      }
    }
  }
  ```

  Biome formatter attempts to format as Prettier, however some default options might differ.

  An option `html.formatter.selfCloseVoidElements` allows to control whether the trailing
  `/` of [void elements](https://html.spec.whatwg.org/#void-elements) should be printed.

  **By default**, Biome formatter will _remove_ the `/`:

  ```diff
  - <input />
  + <input>
  ```

  If you come from Prettier and you want to keep the same formatting behaviour, you should set the option to `"always"`:

  ```json
  {
    "html": {
      "formatter": {
        "selfCloseVoidElements": "always"
      }
    }
  }
  ```

  ```diff
  - <input>
  + <input />
  ```

  Use to the command `biome migrate prettier` to apply this change automatically.

- Added an **unsafe** fix to the rule [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies).

  For example, this violation will provide the following code fix:

  ```js
  import { useEffect } from "react";

  function MyComponent() {
    let a = 1;
    useEffect(() => {}, [a]);
  }
  ```

  ```
    × This hook specifies more dependencies than necessary: a

      3 │ function MyComponent() {
      4 │   let a = 1;
    > 5 │   useEffect(() => {}, [a]);
        │   ^^^^^^^^^
      6 │ }
      7 │

    i This dependency can be removed from the list.

      3 │ function MyComponent() {
      4 │   let a = 1;
    > 5 │   useEffect(() => {}, [a]);
        │                        ^
      6 │ }
      7 │

    i Unsafe fix: Remove the extra dependencies from the list.

      5 │ ··useEffect(()·=>·{},·[a]);
        │                        -
  ```

- The rule `useExhaustiveDependencies` now reports a diagnostic when the dependency list is not an array literal.

- Added the new rule [`useIndexOf`](https://biomejs.dev/linter/rules/use-index-of), based on the unicorn rule [prefer-array-index-of](https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/prefer-array-index-of.md)

- Added a new rule [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/), which detects any missing cases for switch statements. Currently, it supports only literal union types.

  For example:

  ```ts
  type Day =
    | "Monday"
    | "Tuesday"
    | "Wednesday"
    | "Thursday"
    | "Friday"
    | "Saturday"
    | "Sunday";

  const day: Day = "Monday";
  let result = 0;

  switch (day) {
    case "Monday": {
      result = 1;
      break;
    }
  }
  ```

  The switch statement is missing other cases than
  `'Monday'`, which will cause a runtime error. To fix this issue, add missing cases or a default case to the statement.

- Fixed [#4416](https://github.com/biomejs/biome/pull/4416): The rules [`useExportType`](https://biomejs.dev/linter/rules/use-export-type/) and [`useImportType`](https://biomejs.dev/linter/rules/use-import-type/) now ignore TypeScript declaration files.

- Added the new rule [`useForComponent`](https://biomejs.dev/linter/rules/use-for-component/).

  This rule enforces usage of Solid's `<For />` component for mapping an array to JSX elements.

- Added new lint rule [`noShadow`](http://biomejs.dev/linter/rules/no-shadow), a port of eslint's `no-shadow`.

  This rule disallows variable declarations from shadowing variables declared in an outer scope. For example:

  ```js
  const foo = 1;

  function bar() {
    const foo = 2; // This variable shadows the outer foo
  }
  ```

- Add `style` option for the [useImportType](https://biomejs.dev/linter/rules/use-import-type/) rule.

  The rule now allows enforcing an import style for importing types. See the rule documentation for more details.

- Added the new rule [`useJsonImportAttribute`](https://biomejs.dev/linter/rules/use-json-import-attribute) to enforce the use of import attributes for JSON modules.

  This rule ensures that all imports of `.json` files include the
  `with { type: "json" }` assertion, which is required to inform the JavaScript runtime that the imported file should be parsed as JSON.

  ```diff
  - import jsonData from './data.json';
  + import jsonData from './data.json' with { type: "json" };
  ```

  ```diff
  - import jsonData from './data.json' with { someOtherAttribute: "value" };
  + import jsonData from './data.json' with { type: "json", someOtherAttribute: "value" };
  ```

  This rule is based on the proposal in issue [#6043](https://github.com/biomejs/biome/issues/6043).

- [useLiteralKeys](https://biomejs.dev/linter/rules/use-literal-keys/) now handles numeric keys and is declared as being the same rule as the ESLint [no-useless-computed-key](https://eslint.org/docs/latest/rules/no-useless-computed-key) rule.

- `useNamingConmvention` now ignores unused variables prefixed with an underscore `_`.

  This avoids conflicts with the unsafe fix of
  `noUnusedVariables`. The following code is now accepted because the variable is unused and prefixed with an underscore.

  ```js
  const _Unknown_Style = 0;
  ```

- The package now requires `v2` of the WebAssembly packages. The internal APIs of Workspace are now `camelCase`.

- The rule [useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals/) now provides a code fix.

  ```diff
  - const xs = new Array();
  + const xs = [];
  ```

  The code fix is currently marked as unsafe. We plan to make it safe in a future release of Biome.

- The command `migrate` is now able to migrate nested configuration files.

- Added the new rule [`noRestrictedElements`](https://biomejs.dev/linter/rules/no-restricted-elements), which prevents use of the specified HTML elements and components.

- Added the new lint rule [`noAwaitInLoop`](https://biomejs.dev/linter/rules/no-await-in-loop).

### Patch Changes

- Fix [#5001](https://github.com/biomejs/biome/issues/5001), where the CSS formatter removes whitespace from selector preceded by a comment

- Fixed [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/) rule to suggest unsafe fix for unused function declarations.

- Fixed a false positive of `noUselessEscapeInRegex` where
  `\k` was reported as useless in non-Unicode regular expressions.

- Fixed an issue where the ordering of the diagnostics wasn't predictable.

- Fixed a bug where the environment variable `BIOME_CONFIG_PATH` wasn't correctly picked up.

- Biome logs a warning in case a folder contains `biome.json` and `biome.jsonc`, and it will use
  `biome.json` by default.

- [noUndeclaredVariables](https://biomejs.dev/linter/rules/no-undeclared-variables/) is now able to bind read of value to a type-only import in ambient contexts ([#4526](https://github.com/biomejs/biome/issues/4526)).

  In the following code, `A` is now correctly bound to the type-only import. Previously,
  `A` was reported as an undeclared variable.

  ```ts
  import type { A } from "mod";

  declare class B extends A {}
  ```

- Fix [#4317](https://github.com/biomejs/biome/issues/4317), setter parameter can contain a trailing comma, the following example will now parsed correctly:

  ```ts
  export class DummyClass {
    set input(value: string) {}
  }
  ```

- Fix [#4575](https://github.com/biomejs/biome/issues/4575), don't wrap selector indentation after css comments.

- Fix [#4258](https://github.com/biomejs/biome/issues/4258), where fixed css parse error with @-moz-document url-prefix().

- Fixed [#4391](https://github.com/biomejs/biome/issues/4391): Some files from the
  `.vscode` directory are no longer incorrectly parsed as JSON.

- The `biome format` command now correctly handles the
  `--skip-errors` option, allowing it to skip files with syntax errors and continue formatting the remaining valid files. When this option is used, skipped syntax errors are reported as information, since the user is already aware of them.

- `biome migrate eslint` now correctly resolves the scoped package named `eslint-config`.

- Fixed [#3836](https://github.com/biomejs/biome/issues/3836): The CSS parser will now correctly parse the following:

  ```css
  .foo {
    color: red;
  }
  ```

- Fixed a bug where the related diagnostics attached to the main diagnostics didn't have a correct message.

- Fixed `noAccumulatingSpread` not reporting calls to `Object.assign`. The following code will now be reported:

  ```js
  let a = [{ a: 1 }, { b: 2 }];
  a.reduce((acc, val) => Object.assign(acc, val), []);
  ```

- The `summary` reporter doesn't take `--max-diagnostics` into account anymore.

- Fixed [#4553](https://github.com/biomejs/biome/issues/4553):
  `noUselessFragments` will now correctly fix JSX attributes:

  ```jsx
  <Suspense
    fallback={
      <>
        <span>Loading...</span>
      </>
    }
  >
    {children}
  </Suspense>
  ```

  becomes:

  ```jsx
  <Suspense fallback={<span>Loading...</span>}>{children}</Suspense>
  ```

- Fixed [#4528](https://github.com/biomejs/biome/issues/4528):
  `biome migrate eslint` now correctly handles shared ESLint configuration that don't follow the ESLint naming convention.

  ESLint recommends that a package that exports a shared configuration be prefixed with `eslint-config-` or simply named
  `eslint-config`. This is only a recommendation. Packages that export shared configurations can have arbitrary names. Biome is now able to load any package.

- Fixed [#4993](https://github.com/biomejs/biome/issues/4993): [`useAwait`](https://biomejs.dev/linter/rules/use-await/) now correctly warn on functions with decorator with callback argument.

- Fixed [#4756](https://github.com/biomejs/biome/issues/4756):
  `noDuplicateProperties` now throws lint errors properly when we use `@supports`.

- Fixed [#5981](https://github.com/biomejs/biome/issues/5981), where `noUnknownPseudoClass` didn't take
  `:global` into consideration when `cssModules` is enabled.

- Fixed [#2406](https://github.com/biomejs/biome/issues/2406): Biome longer expands properties of object type annotations in the only function parameter to align with Prettier.

- Fixed [#4740](https://github.com/biomejs/biome/issues/4740):
  `biome migrate eslint` now correctly handles ESLint configuration with `null` values in file lists.

- Fixed [#4202](https://github.com/biomejs/biome/issues/4202): Align with Prettier in formatting test functions.

- Fixed [#342](https://github.com/biomejs/biome/issues/342): The JavaScript parser now properly handles unterminated string literals, such as:

  ```jsx
  function Comp() {
    return (
        <a rel="
  ```

- Fixed a bug where syntax rules didn't provide an automatic way to suppress the rule. Now the LSP will show suppression actions if a syntax rule is violated.

- Fixed a CSS parser error: `@-moz-document url-prefix(https://example.com)` and
  `@-moz-document domain(example.com)` are now valid.

- Fixed [#4967](https://github.com/biomejs/biome/issues/4967): The fix for
  `useArrowFunction` no longer breaks function bodies starting with `{`.

- Fixed [#5998](https://github.com/biomejs/biome/issues/5998). The rule
  `noUnknownPseudoElement` now correctly checks names of pseudo-element functions.

- Fixed [#5024](https://github.com/biomejs/biome/issues/5024): Added `useJsxKeyInIterable` rule to React domain.

- Fixed [#5410](https://github.com/biomejs/biome/issues/5410). Biome now correctly parse an
  `.editorconfig` that includes character classes in glob patterns.

- Fixed [#2260](https://github.com/biomejs/biome/2260): The LSP server now returns correct text edits for the specified range in
  `textDocument/rangeFormatting` and `textDocument/onTypeFormatting` requests.

- [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/) no longer reports top-level variables in a global declaration file as unused.

- Type exports now support renaming types to `default`.

  The following code is now parsed successfully:

  ```ts
  export { type A as default } from "./b.ts";
  ```

- Added proper support for arrow functions in the lint rule https://biomejs.dev/linter/rules/use-explicit-type/

- The rule [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) no longer suggests renaming top-level variables in a global declaration file.

- Improved context in error messages when migrating Prettier configurations

- Allowed single spaces in `useConsistentCurlyBraces` rule.

- Fixed [#4413](https://github.com/biomejs/biome/issues/4413): The GraphQL formatter no longer adds a new line at the start of block comments on Windows.

- Fixed [#5407](https://github.com/biomejs/biome/issues/5407). Now the
  `noUnusedImports` code fix correctly keeps top-level comments that were attached to lone imports.

- Fixed [#3859](https://github.com/biomejs/biome/issues/3859): the
  `--skip-parse-errors` option is now applied to commands: `lint`, `check`, and `ci`.

- The `rage` command now prints the configuration path relative to the working directory, if applicable.

- Fixed [#5606](https://github.com/biomejs/biome/issues/5606): We now correctly handle
  `.mjs` extensions in Node.js packages with `"type": "commonjs"`.

- Fixed [#1597](https://github.com/biomejs/biome/issues/1597):
  `useExhaustiveDependencies` no longer gets confused about the stability of dependencies by parentheses or type assertions.

- Fixed [#4751](https://github.com/biomejs/biome/issues/4751) by checking fragments inside
  `JSXElement` and conditional expressions.

  For example, the following two cases will now be reported:

  ```jsx
  <section>
    <>
      <div />
      <div />
    </>
  </section>
  ```

  ```jsx
  showFullName ? <>{fullName}</> : <>{firstName}</>;
  ```

- The rule `noFallthroughSwitchCase` no longer panics on some incomplete code snippets.

- Fixed [#5007](https://github.com/biomejs/biome/issues/5007): Resolved false positives in `noMissingVarFunction` for
  `container-name`.

- Fixed [#4841](https://github.com/biomejs/biome/issues/4841): Shebang and top leading comments in
  `.cjs` files are now handled correctly

  **Example: shebang only (keep it as is)**

  ```
  #!/usr/bin/env node
  ```

  **Example: comments only (keep it as is)**

  ```
  // comment
  ```

  **Example: with shebang**

  ```diff
  - #!/usr/bin/env node"use strict";
  + #!/usr/bin/env node
  + "use strict";
  let some_variable = "some value";
  ```

  **Example: with comment**

  ```diff
  - // comment
  - "use strict"; // comment
  + "use strict";
  + // comment
  let some_variable = "some value";
  ```

  **Example: with shebang and comment**

  ```diff
  - #!/usr/bin/env node"use strict";
  - // comment
  + #!/usr/bin/env node
  + "use strict";
  + // comment
  let some_variable = "some value";
  ```

- Fixes [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) missing dependencies being defined after the hook itself failure.

  Example:

  ```jsx
  import { useState, useEffect } from "react";

  function MyComponent() {
    useEffect(() => {
      console.log(a);
    }, []);

    let a = 1;
  }
  ```

- Fixed [#4714](https://github.com/biomejs/biome/pull/4714): Suppression comments no longer fail on functions that themselves contain suppression comments.

  This now works correctly:

  ```ts
  // biome-ignore lint/complexity/useArrowFunction: this suppression now works
  const foo0 = function (bar: string) {
    // biome-ignore lint/style/noParameterAssign: even if there are other suppressions inside
    bar = "baz";
  };
  ```

- Add @vitest/eslint-plugin to list of Biome rule sources

- Fixed `useHookAtTopLevel` rule to properly detect React components wrapped in `memo` and
  `forwardRef`, and correctly handle property accessors in control flow analysis.

  The rule now correctly identifies hooks in components like:

  ```js
  const TestMemo = memo(
    forwardRef((props, ref) => {
      useEffect(() => {
        const [test, setTest] = useState(1); // now properly flagged
      }, []);
      return <div ref={ref}>test</div>;
    }),
  );
  ```

  And properly handles property accessors:

  ```js
  function ReactComponent() {
    const testObj = {
      get print() {
        return "hello"; // no longer considered component return
      },
    };
    const callback = useCallback(() => {}, []);
    return <></>;
  }
  ```

- [noMisleadingCharacterClass](https://biomejs.dev/linter/rules/no-misleading-character-class/) no longer panics on malformed escape sequences that end with a multi-byte character ([#4587](https://github.com/biomejs/biome/issues/4587)).

- Fixed the flag
  `--bracket-spacing` that was duplicated between the global configuration and the language-specific override for JavaScript.

- Fixed [#4715](https://github.com/biomejs/biome/issues/4715): The
  `useJsxKeyInIterable` rule now reports missing keys inside `switch` and `if` statements.

  ```jsx
  const data = [
    { value: "a", type: "string" },
    { value: 9, type: "number" },
    { value: "c", type: "string" },
  ];

  const MyComponent = () => {
    return (
      <>
        {/* if statements */}
        {data.map((x) => {
          if (x.type === "string") {
            return <div>{x.value}</div>; // no key, emits diagnostic
          } else {
            return <div>{x.value}</div>; // no key, emits diagnostic
          }
        })}

        {/* switch statements */}
        {data.map((x) => {
          switch (x.type) {
            case "string":
              return <div>{x.value}</div>; // no key, emits diagnostic
            case "number":
              return <div>{x.value}</div>; // no key, emits diagnostic
            default:
              return <div key={x.value}>{x.value}</div>;
          }
        })}
      </>
    );
  };
  ```

- Fixed [#4121](https://github.com/biomejs/biome/issues/4326): The CSS formatter no longer indents a selector when it has leading comments.

- Fixed an issue where react lint rules could panic Biome when some incorrect code was analyzed.

- Fixed [#4982](https://github.com/biomejs/biome/issues/4982): the JavaScript parser now throws a syntax error for the following code:

  ```ts
  type T = import;
  type U = typeof import;
  ```

- Fixed a bug with the `--verbose` CLI flag. Now the printed paths are **relative** to the working directory.

- Fixed [`noNoninteractiveElementToInteractiveRole`](https://biomejs.dev/linter/rules/no-noninteractive-element-to-interactive-role/) mistakenly flagging
  `<li role="treeitem">`,

- Fixed [#4622](https://github.com/biomejs/biome/issues/4622): Our JavaScript parser can now gracefully handle situations where we detect the parser to have stalled.

  This means we don't fail with an assertion anymore, but invalid code can trigger a regular diagnostic in such cases.

- Fixed [#342](https://github.com/biomejs/biome/issues/342): The JavaScript parser now correctly handles invalid object member names, such as:

  ```js
  ({
    params: { [paramName: string]: number } = {}
  })
  ```

- Fixed [#6211](https://github.com/biomejs/biome/issues/6211): previously the import organizer emitted broken code when it merged an import at the start of the file with another import and placed the merged result after a third import.

  The following code is now correctly organized:

  ```diff
  - import { B } from "bc";
  - import { C } from "bc";
    import { A } from "a";
  + import { B, C } from "bc";
  ```

- Fixed [#4334](https://github.com/biomejs/biome/issues/4334): The formatter no longer inserts trailing a comma inside dynamic
  `import` expressions.

- Fixed [#5629](https://github.com/biomejs/biome/issues/5629): useHookAtTopLevel no longer report false-positives where the hook is at the top-level in a class method.

- Fixed [#5900](https://github.com/biomejs/biome/issues/5900): `biome migrate eslint` now support a nested
  `files` property in ESLint flat configs.

- Fixed [#3895](https://github.com/biomejs/biome/issues/3895): [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports/) no longer reports used values imported as types in an external module.

- Fixed a case where the code fix for `noUselessFragments` would remove more than just the fragment.

- Fixed [#5919](https://github.com/biomejs/biome/issues/5919). Now Biome correctly loads the configuration passed via
  `--config-path` when its path starts with `./` e.g. `--confi-path=./project/biome.json`

- Fixed [#5031](https://github.com/biomejs/biome/issues/5031): CSS formatting has been improved for numbers:

  ```diff
  .class {
  -	padding: .5em;
  -	marding: 1.0;
  +	padding: 0.5em;
  +	marding: 1;
  }
  ```

- Fixed [#5989](https://github.com/biomejs/biome/issues/5989) where large octal escape sequences led to an overflow.

- Implement improved error handling for the supports at rule

- Fix [#5053](https://github.com/biomejs/biome/issues/5053), now the rule correctly handles
  `console.log` inside arrow function expressions.

- Fix [#6105](https://github.com/biomejs/biome/issues/6105): css lint rules
  `useSortedProperties` should skip unknown properties.

- Fixed [#3229](https://github.com/biomejs/biome/issues/3229): Made formatting of compound selectors more consistent.

- Fixed a bug where passing `--max-diagnostics=0` would return a zero code even when errors were emitted.

- Fixed a bug where Biome didn't report any error when
  `--stdin-file-path` didn't have any extension. Now Biome returns an error if
  `--stdin-file-path` doesn't have an extension.

- Fixed [#5601](https://github.com/biomejs/biome/issues/5601): The [`useSortedClasses`](https://biomejs.dev/linter/rules/use-sorted-classes/) rule now properly preserves the original JSX quote style when sorting utility classes, preventing syntax errors.

- The fix for `useSelfClosingElements` was marked as safe and the error message was improved.

- Fixed overrides that include language-specific settings from having an effect for some languages

- Fixed [#6144](https://github.com/biomejs/biome/issues/6144): [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports/) reported incorrectly imports that were used as the type of parameters with the same name. In the following code, the import
  `name` was reported as unused.

  ```ts
  import name from "mod";
  function f(name: name.Readable): void {}
  ```

- The lint rules [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) and [`useFilenamingConvention`](https://biomejs.dev/linter/rules/use-filenaming-convention/) now accept character escapes at the start of a regex group.

  Both these rules provide options that allow matching names against a regular expression. Previously, an escaped character at the start of a regex group reported an error. They are now accepted.

  For example, the following configuration is now valid doesn't emit an error anymore.

  ```json
  {
    "linter": {
      "rules": {
        "style": {
          "useNamingConvention": {
            "level": "on",
            "options": {
              "conventions": [
                {
                  "selector": {
                    "kind": "let"
                  },
                  "match": "(\\n.*)"
                }
              ]
            }
          }
        }
      }
    }
  }
  ```

- Fixed [#5617](https://github.com/biomejs/biome/issues/5617): [noDuplicateObjectKeys](https://biomejs.dev/linter/rules/no-duplicate-object-keys/) now transfers the leading comments of the removed member.

- Fixed [#5409](https://github.com/biomejs/biome/issues/5409): [noParameterAssign](https://biomejs.dev/linter/rules/no-parameter-assign) now reports reassigned parameter of unparenthesized arrow functions.

  The following code is now reported as invalid.

  ```js
  const f = (param) => {
    param = {}; // Reassigning a function parameter is confusing.
  };
  ```

- Fixed [#4875](https://github.com/biomejs/biome/issues/4875): Relative file paths are now clickable in the Jetbrains IDE terminal.

- Fixed [#4719](https://github.com/biomejs/biome/issues/4719):
  `bracketSameLine` now performs as expected when a comment is placed before the last JSX attribute.

- Fixed [#4564](https://github.com/biomejs/biome/issues/4564): Biome no longer panics when a multi-byte character is found in a unicode escape sequence.

- Fixed [#4950](https://github.com/biomejs/biome/issues/4950): Resolved a false positive of character class range operators in regular expressions.

- Fixed handling of top-level variables by
  `useExplicitType` rule ([#5932](https://github.com/biomejs/biome/issues/5932)). Biome now allows all variables with explicit annotations, as well as variables with trivial RHS. Biome no longer emits duplicated errors when an untyped function is assigned to an untyped variable.

- Fixed [#4947](https://github.com/biomejs/biome/issues/4947): The
  `useTemplate` lint rule now ignores concatenated literals folded to multiple lines.

- Fixed [#4568](https://github.com/biomejs/biome/issues/4568): Broken import statements no longer can cause a panic in
  `useExhaustiveDependencies`.

- Fixed [#6042](https://github.com/biomejs/biome/pull/6042): [`noUselessEscapeInString`](https://next.biomejs.dev/linter/rules/no-useless-escape-in-string/) now reports useless escapes after skipping \${ in template literals.

- Fixed [#6229](https://github.com/biomejs/biome/issues/6229) where the fix of
  `noUnusedImports` emitted an invalid syntax. Now the following case emits a code fix that is syntactically correct:

  ```js
  import Used, { NotUsed } from "foo";

  Used();
  ```

- Fix [#5682](https://github.com/biomejs/biome/issues/5682): Object patterns with a nested assignment pattern no longer break properties.

  For example, the following code:

  ```js
  const { foo: { bar } = { bar: false } } = props;
  ```

  is used to be formatted into:

  ```js
  const { foo: { bar } = { bar: false } } = props;
  ```

  , while Prettier does not expand properties in this case.

- Fixed #5620, [noConsole](https://biomejs.dev/linter/rules/no-console/) rule now correctly handles indirect
  `console.log` calls and references.

- When pulling code actions from the LSP, now the first choice suggested by the client will be the safe fix.

- Fixed [#6022](https://github.com/biomejs/biome/issues/6022), now the rule
  `noDuplicateProperties` doesn't trigger properties defined inside the `@keyframes` at rule

- Enhanced the error message of the diagnostics emitted when Biome can't parse a suppression comment.

- Fixed link to the docs inside CLI markup

- Fixed a bug where a suppression comment with an empty explanation was valid.

  Now a suppression comment `// biome-ignore lint:` will raise a **warning** diagnostic.

- Fixed [#4026](https://github.com/biomejs/biome/issues/4026): Comments in
  `grid-template` are no longer moved by the formatter.

- Fixed [#3394](https://github.com/biomejs/biome/issues/3394): Resolved a false positive in `useSortedClasses`.

- Fixed [#342](https://github.com/biomejs/biome/issues/342) and [#4562](https://github.com/biomejs/biome/issues/4562): Biome no longer crashes when a
  `declare` statement is followed by an unexpected token.

- Fixed false positive in the rule [`noUnknownFunction`](https://biomejs.dev/linter/rules/no-unknown-function) where the [`tech`](https://developer.mozilla.org/en-US/docs/Web/CSS/@font-face/src#tech) function was incorrectly flagged as an unknown function.

- Fixed [#4511](https://github.com/biomejs/biome/issues/4511): [noLabelWithoutControl](https://biomejs.dev/linter/rules/no-label-without-control/) now detects
  `<button>` tags as input.

- Fixed [#6039](https://github.com/biomejs/biome/issues/6039): [`noUselessEscapeInString`](https://next.biomejs.dev/linter/rules/no-useless-escape-in-string/) no longer reports
  `\${` escape in template literals.

- Fixed [#5985](https://github.com/biomejs/biome/issues/5985), which caused the import organizer to fail the merging of a default import with a named import. The following code is now correctly organized:

  ```diff
  - import moment from 'moment';
  - import { Moment } from 'moment';
  + import moment, { Moment } from 'moment';
  ```

- Fixed an issue where the `explain` command didn't the diagnostic category when a rule was explained.

- Improved the diagnostic of the rule `noUnusedVariables`. The rule message now provides the name of the unused binding.

- Added `RegExpStringIterator` to the analyzer globals.

- Fixed [#4208](https://github.com/biomejs/biome/issues/4208): [noUselessFragments](https://biomejs.dev/linter/rules/no-useless-fragments/) now handles
  `JsxAttributeInitializerClause`, ensuring that fragments inside expressions like `<A b=<></> />` are preserved.

- Fixed [#4533](https://github.com/biomejs/biome/issues/4533):
  `noUnknownPseudoClass` no longer reports pseudo classes after a webkit scrollbar pseudo element.

  The following code will no longer report a diagnostic:

  ```css
  ::-webkit-scrollbar-thumb:hover {
  }
  ```

- Updates the [`useJsxKeyInIterable`](https://biomejs.dev/linter/rules/use-jsx-key-in-iterable/) rule to more closely match the behavior of the ESLint plugin (e.g. mark the whole fragment as incorrect when no key is present). This also adds the option to check shorthand fragments (
  `<></>`)

- Renamed the rule `noDuplicatedFields` to `noDuplicateFields`. Run the command
  `biome migrate` to update your configuration.

- Fixed an issue where ignored files were incorrectly tracked by the Daemon.

- Fixed [#5116](https://github.com/biomejs/biome/issues/5116): [noUnknownPseudoElement](https://biomejs.dev/linter/rules/no-unknown-pseudo-element/) now supports
  `::slotted`.

- Fixed [#5979](https://github.com/biomejs/biome/issues/5979):
  `biome search` now correctly skips files that don't match the pattern's target language.

- Fixed [#4323](https://github.com/biomejs/biome/issues/4258): Fixed the case where
  `useSemanticElement` accidentally showed recommendations for `role="searchbox"` instead of `role="search"`.

- Support setting `indent_size` to `tab` in `.editorconfig`, the following config will not cause error:

  ```editorconfig
  root = true
  [*]
  indent_size = tab
  ```

- Fixed [#4565](https://github.com/biomejs/biome/issues/4565): [noControlCharactersInRegex](https://biomejs.dev/linter/rules/no-control-characters-in-regex) no longer panics when it encounters an unterminated unicode escape sequence.

- Fixed [#5770](https://github.com/biomejs/biome/issues/5770), Biome's configuration file is now respected by the
  `migrate` command during migration

- Fixed an issue where the lexer didn't report errors for unterminated regex or string literals, such as the following cases:

  ```js
  "string
  'str
  /\\217483
  ```

- The [`useKeyWithClickEvents`](https://biomejs.dev/linter/rules/use-key-with-click-events/) rule has been improved with better support for ARIA roles.

  Key improvements:
  1. **Accessibility checks**:

     Now the rule correctly handles the following cases:
     - If an element is hidden from screen readers
     - If an element has the presentation role
     - If an element is interactive

     ```jsx
     // No errors
     <div aria-hidden="true" onClick={() => {}} /> // hidden from screen reader
     <div role="presentation" onClick={() => {}} /> // presentation role
     <button onClick={() => {}} /> // interactive role
     ```

     This change ensures the rule is more accurate and helpful.

  2. **Checks spread syntax**:

     Spread syntax used to be ignored, but has been changed to be pointed out for more stringent checking.

     ```jsx
     // Errors
     <div {...props} onClick={() => {}} />
     // No errors
     <div {...props} onClick={() => {}} onKeyDown={foo} />;
     ```

  3. **Refactor**:

     Now the rule uses the aria roles to determine if an element is interactive.

     The changes shown here are meant to be closer to the original [jsx-eslint's
     `click-events-have-key-events` rule](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/click-events-have-key-events.md).

- Fixed [#6029](https://github.com/biomejs/biome/issues/6029): A new line before the semicolon in the previous statement is now kept after formatting.

  For example, the following code:

  ```js
  const foo = 3;

  [1, 2, 3].map((x) => x * 2);
  ```

  when `javascript.formatter.semicolons` is `always`, it becomes:

  ```js
  const foo = 3;

  [1, 2, 3].map((x) => x * 2);
  ```

  when `javascript.formatter.semicolons` is `asNeeded`, the original code is considered as already formatted.

- [useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals/) now reports all expressions using the
  `Array` constructors.

  Previously, the rule reported only use of the `Array` constructor in expressions statements.

  ```js
  // This was reported
  new Array();
  // This was not reported
  const xs = new Array();
  ```

- Improved error handling for the container at-rule.

- Fixed [#4665](https://github.com/biomejs/biome/issues/4665): the LSP previously identified
  `.cjs` files as ESM files, making rules like `noRedundantUseStrict`
  reports incorrectly valid `"use strict"` directives.

- Fixed [#5382](https://github.com/biomejs/biome/issues/5382):
  `useExportType` no longer reports an identifier that bound by both a variable and a type.

- Fixed [#5826](https://github.com/biomejs/biome/issues/5826): [`useNumericSeparators`](https://next.biomejs.dev/linter/rules/use-numeric-separators/) no longer reports single-digit
  `0`.

- Fixed [#5307](https://github.com/biomejs/biome/issues/5307), where CSS value lists were wrapped in a way that did not preserve semantic structure.

  Biome now ensures that CSS value lists follow a more readable format, aligning with Prettier's behavior.

  Before:

  ```css
  * {
    box-shadow:
      0 0 0 1px #fff,
      0 0 0 3.2px rgba(89, 89, 235, 0.25),
      0 0 0 3.2px rgba(89, 89, 235, 0.25),
      0 0 0 3.2px red,
      0 0 0 3.2px rgba(89, 89, 235, 0.25);
  }
  ```

  After:

  ```css
  * {
    box-shadow:
      0 0 0 1px #fff,
      0 0 0 3.2px rgba(89, 89, 235, 0.25),
      0 0 0 3.2px rgba(89, 89, 235, 0.25),
      0 0 0 3.2px red,
      0 0 0 3.2px rgba(89, 89, 235, 0.25);
  }
  ```

- `tsconfig.*.json` files will now be treated the same as `tsconfig.json` files.

- The `summary` reporter now prints the files processed and the files fixed when passing the `--verbose` flag.

- Fixed [#5693](https://github.com/biomejs/biome/issues/5693): [`useRegexLiterals`](https://biomejs.dev/linter/rules/use-regex-literals/) now correctly handle useless escaped character in string literals.

- [useRegexLiterals](https://biomejs.dev/linter/rules/use-regex-literals) now suggests a correct fix when the pattern contains an escaped anti-slash
  `\/`.

  Previously the rule suggested the following fix that led to a syntax error:

  ```diff
  - new RegExp("\/");
  + /\\//
  ```

  The rule now suggests a correct fix:

  ```diff
  - new RegExp("\/");
  + /\//
  ```

  Fixed [#5487](https://github.com/biomejs/biome/issues/5487).

- Fixed [`useConsistentCurlyBraces breaks react/no-unescaped-entities rule`](https://github.com/biomejs/biome/issues/5391)

  Added a check for forbidden characters: `>`, `"`, `'` and
  `}`. If any of these characters are detected, curly braces will be preserved.

  Example:

  ```jsx
  function MyComponent() {
    return <Foo>Jupiter {">"} Venus</Foo>;
  }
  ```

- The rule [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) now suggests a rename that preserves uppercase if possible.

  For instance, Biome suggested renaming `HTMLWrapper` as `htmlWrapper`:

  ```diff
  - import HTMLWrapper from "HTMLWrapper.tsx";
  + import htmlWrapper from "HTMLWrapper.tsx";

    function component() {
  -   return <HTMLWrapper> </HTMLWrapper>;
  +   return <htmlWrapper> </HTMLWrapper>;
    }
  ```

  Since both `PascalCase` and `CamelCase` are accepted, Biome now suggests renaming `HTMLWrapper` as `HtmlWrapper`:

  ```diff
  - import HTMLWrapper from "HTMLWrapper.tsx";
  + import HtmlWrapper from "HTMLWrapper.tsx";

    function component() {
  -   return <HTMLWrapper> </HTMLWrapper>;
  +   return <HtmlWrapper> </HTMLWrapper>;
    }
  ```

- Fix a parsing error when a `JsxElementName` is `JsxMemberExpression`, and a
  `JsLogicalExpression` before it without a semicolon.

  The following case will now not throw error:

  ```jsx
  import React from "react";

  let b = 0;

  function A() {
    const a = b > 0 && b < 1;

    return <React.Fragment>{a}</React.Fragment>;
  }
  ```

- Fixed Biome being unable to parse `insert_final_newline = unset` in EditorConfig files.

- Fixed [#4530](https://github.com/biomejs/biome/issues/4530): [useArrowFunction](https://biomejs.dev/linter/rules/use-arrow-function/) now preserves directives.

  Previously the rule removed the directives when a function expression was turned into an arrow function. The rule now correctly keeps the directives.

  ```diff
  - const withDirective = function () {
  + const withDirective = () => {
      "use server";
      return 0;
    }
  ```

- Fixed [#4855](https://github.com/biomejs/biome/issues/4855): [useSortedClasses](https://biomejs.dev/linter/rules/use-sorted-classes/) now suggests code fixes that match the JSX quote style of the formatter.
