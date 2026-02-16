# PR #8960 Review Comments - Full Transcript

All review comments organized by file path in chronological order.

**Total Comments: 94**

---

## .changeset/no-playwright-element-handle-fix.md

**2 comment(s)**

### Comment 1

- **Commenter**: dyc3
- **Created**: 2026-02-05T23:02:52Z
- **Comment ID**: 2771521176
- **Line**: 1
- **Resolved**: False

**Comment Body:**

remove this changeset

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T00:26:50Z
- **Comment ID**: 2776520122
- **Line**: 1
- **Resolved**: False

**Comment Body:**

fixed; nice catch.

---

## .changeset/no-playwright-element-handle.md

**2 comment(s)**

### Comment 1

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T01:15:54Z
- **Comment ID**: 2761717848
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

**Tiny wording tweak on Line 5.**  
LanguageTool’s suggestion reads a touch cleaner.

<details>
<summary>✏️ Suggested edit</summary>

```diff
-Added the rule [`noPlaywrightElementHandle`](https://biomejs.dev/linter/rules/no-playwright-element-handle/). Prefers locators over element handles.
+Added the rule [`noPlaywrightElementHandle`](https://biomejs.dev/linter/rules/no-playwright-element-handle/). Prefers locators to element handles.
```
</details>

<!-- suggestion_start -->

<details>
<summary>📝 Committable suggestion</summary>

> ‼️ **IMPORTANT**
> Carefully review the code before committing. Ensure that it accurately replaces the highlighted code, contains no missing lines, and has no issues with indentation. Thoroughly test & benchmark the code to ensure it meets the requirements.

```suggestion
Added the rule [`noPlaywrightElementHandle`](https://biomejs.dev/linter/rules/no-playwright-element-handle/). Prefers locators to element handles.
```

</details>

<!-- suggestion_end -->

<details>
<summary>🧰 Tools</summary>

<details>
<summary>🪛 LanguageTool</summary>

[grammar] ~5-~5: Consider using “to” with “prefer”.
Context: ...r/rules/no-playwright-element-handle/). Prefers locators over element handles.  ```js const el = await page.$...

(PREFER_OVER_TO)

</details>

</details>

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In @.changeset/no-playwright-element-handle.md at line 5, Change the wording for
the new rule description to use the imperative form: update the sentence
"Prefers locators over element handles." to "Prefer locators over element
handles." in the .changeset/no-playwright-element-handle.md entry for the
noPlaywrightElementHandle rule.
```

</details>

<!-- fingerprinting:phantom:poseidon:eagle -->

<!-- This is an auto-generated comment by CodeRabbit -->

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T03:30:14Z
- **Comment ID**: 2762001628
- **Resolved**: False

**Comment Body:**

✅ Fixed in commit b2b9446a7c - Changed "Prefers locators over element handles" to "Prefers locators to element handles".

---

## .changeset/no-playwright-skipped-test-fix.md

**2 comment(s)**

### Comment 1

- **Commenter**: dyc3
- **Created**: 2026-02-05T23:02:30Z
- **Comment ID**: 2771520248
- **Line**: 1
- **Resolved**: False

**Comment Body:**

This changeset is also wrong

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T00:27:00Z
- **Comment ID**: 2776520391
- **Line**: 1
- **Resolved**: False

**Comment Body:**

nice catch!

---

## .changeset/no-playwright-useless-await.md

**2 comment(s)**

### Comment 1

- **Commenter**: dyc3
- **Created**: 2026-02-05T23:02:16Z
- **Comment ID**: 2771519596
- **Resolved**: False

**Comment Body:**

nit: should start with "Added the nursery rule ..."

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T00:26:36Z
- **Comment ID**: 2776519805
- **Resolved**: False

**Comment Body:**

fixed in all files.

---

## .changeset/no-playwright-wait-for-selector-fix.md

**2 comment(s)**

### Comment 1

- **Commenter**: dyc3
- **Created**: 2026-02-05T23:01:47Z
- **Comment ID**: 2771518338
- **Line**: 1
- **Resolved**: False

**Comment Body:**

This changeset is also wrong

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T00:26:26Z
- **Comment ID**: 2776519588
- **Line**: 1
- **Resolved**: False

**Comment Body:**

thanks for catching, fixed.

---

## .changeset/no-playwright-wait-for-timeout-note.md

**2 comment(s)**

### Comment 1

- **Commenter**: dyc3
- **Created**: 2026-02-05T23:01:27Z
- **Comment ID**: 2771517243
- **Line**: 1
- **Resolved**: False

**Comment Body:**

This changeset is wrong

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T00:26:18Z
- **Comment ID**: 2776519418
- **Line**: 1
- **Resolved**: False

**Comment Body:**

thanks for catching, fixed.

---

## .changeset/no-skipped-tests-enhancement.md

**1 comment(s)**

### Comment 1

- **Commenter**: dyc3
- **Created**: 2026-02-08T17:53:16Z
- **Comment ID**: 2779594148
- **Line**: 5
- **Resolved**: False

**Comment Body:**

```suggestion
Enhanced `noSkippedTests` to detect Playwright patterns (`.fixme`, `test.describe`, `test.step`, bracket notation, bare calls). Added `allowConditional` option.
```

---

## [GENERAL PR COMMENTS]

**7 comment(s)**

### Comment 1

- **Commenter**: changeset-bot[bot]
- **Created**: 2026-02-04T01:05:11Z
- **Comment ID**: 3844658609

**Comment Body:**

###  🦋  Changeset detected

Latest commit: 3b3af3490e5888bbaa46fe82de50b4a088a4e7c9

**The changes in this PR will be included in the next version bump.**

<details><summary>This PR includes changesets to release 13 packages</summary>

  | Name                          | Type  |
| ----------------------------- | ----- |
| @biomejs/biome                | Patch |
| @biomejs/cli-win32-x64        | Patch |
| @biomejs/cli-win32-arm64      | Patch |
| @biomejs/cli-darwin-x64       | Patch |
| @biomejs/cli-darwin-arm64     | Patch |
| @biomejs/cli-linux-x64        | Patch |
| @biomejs/cli-linux-arm64      | Patch |
| @biomejs/cli-linux-x64-musl   | Patch |
| @biomejs/cli-linux-arm64-musl | Patch |
| @biomejs/wasm-web             | Patch |
| @biomejs/wasm-bundler         | Patch |
| @biomejs/wasm-nodejs          | Patch |
| @biomejs/backend-jsonrpc      | Patch |

</details>

Not sure what this means? [Click here  to learn what changesets are](https://github.com/changesets/changesets/blob/main/docs/adding-a-changeset.md).

[Click here if you're a maintainer who wants to add another changeset to this PR](https://github.com/abossenbroek/biome/new/feat/playwright-eslint-rules?filename=.changeset/yellow-emus-relax.md&value=---%0A%0A---%0A%0Afeat(lint)%3A%20add%20Playwright%20ESLint%20rules%0A)



---

### Comment 2

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T01:15:51Z
- **Comment ID**: 3844685193

**Comment Body:**

<!-- This is an auto-generated comment: summarize by coderabbit.ai -->
<!-- walkthrough_start -->

## Walkthrough

This pull request implements 13 Playwright-related linting rules for Biome, expanding coverage from the ESLint Playwright plugin. It adds a new RuleSource variant (EslintPlaywright) and RuleDomain variant (Playwright) to the core rule system, introduces a Playwright framework utilities module with AST helpers for pattern detection, and implements individual rules targeting Playwright-specific anti-patterns (element handles, eval methods, missing awaits, deprecated APIs, etc.). The changes include comprehensive test coverage for all rules and configuration options structures. An existing rule (noSkippedTests) is also enhanced to recognise Playwright-specific patterns and conditional skipping.

## Possibly related PRs

- [biomejs/biome#8426](https://github.com/biomejs/biome/pull/8426): Modifies the same rule infrastructure to add a new RuleSource/RuleDomain variant (EslintTurbo/Turborepo), using identical patterns for integration
- [biomejs/biome#8864](https://github.com/biomejs/biome/pull/8864): Updates RuleSource/RuleDomain machinery and refactors variant handling alongside rule configuration
- [biomejs/biome#7287](https://github.com/biomejs/biome/pull/7287): Enhances test-pattern detection in expr_ext.rs with similar token/keyword expansion approach

## Suggested reviewers

- dyc3
- ematipico

<!-- walkthrough_end -->


<!-- pre_merge_checks_walkthrough_start -->

<details>
<summary>🚥 Pre-merge checks | ✅ 4</summary>

<details>
<summary>✅ Passed checks (4 passed)</summary>

|         Check name         | Status   | Explanation                                                                                                                                                                                                                                                                                                                       |
| :------------------------: | :------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|         Title check        | ✅ Passed | The PR title 'feat(lint): add Playwright ESLint rules' clearly and concisely describes the primary change—adding Playwright linting rules to the Biome analyser.                                                                                                                                                                  |
|      Description check     | ✅ Passed | The PR description is comprehensive and directly related to the changeset, detailing the rules being implemented, addressing previous feedback, and explaining the motivation and implementation details.                                                                                                                         |
|     Linked Issues check    | ✅ Passed | The changeset implements all 11 Playwright ESLint rules outlined in issue `#7796`. The PR title, description, and code changes collectively deliver the core objectives: comprehensive Playwright linting support with proper detection logic, auto-fixes where applicable, and integration into the Biome analyser infrastructure. |
| Out of Scope Changes check | ✅ Passed | All changes are directly aligned with the stated objectives of implementing Playwright ESLint rules. The only enhancement beyond the initial 11 rules—consolidating noPlaywrightSkippedTest into the existing noSkippedTests rule—is explicitly documented in the PR description as addressing feedback.                          |

</details>

<sub>✏️ Tip: You can configure your own custom pre-merge checks in the settings.</sub>

</details>

<!-- pre_merge_checks_walkthrough_end -->

<!-- finishing_touch_checkbox_start -->

<details>
<summary>✨ Finishing touches</summary>

- [ ] <!-- {"checkboxId": "7962f53c-55bc-4827-bfbf-6a18da830691"} --> 📝 Generate docstrings
<details>
<summary>🧪 Generate unit tests (beta)</summary>

- [ ] <!-- {"checkboxId": "f47ac10b-58cc-4372-a567-0e02b2c3d479", "radioGroupId": "utg-output-choice-group-unknown_comment_id"} -->   Create PR with unit tests
- [ ] <!-- {"checkboxId": "07f1e7d6-8a8e-4e23-9900-8731c2c87f58", "radioGroupId": "utg-output-choice-group-unknown_comment_id"} -->   Post copyable unit tests in a comment

</details>

</details>

<!-- finishing_touch_checkbox_end -->

<!-- tips_start -->

---

Thanks for using [CodeRabbit](https://coderabbit.ai?utm_source=oss&utm_medium=github&utm_campaign=biomejs/biome&utm_content=8960)! It's free for OSS, and your support helps us grow. If you like it, consider giving us a shout-out.

<details>
<summary>❤️ Share</summary>

- [X](https://twitter.com/intent/tweet?text=I%20just%20used%20%40coderabbitai%20for%20my%20code%20review%2C%20and%20it%27s%20fantastic%21%20It%27s%20free%20for%20OSS%20and%20offers%20a%20free%20trial%20for%20the%20proprietary%20code.%20Check%20it%20out%3A&url=https%3A//coderabbit.ai)
- [Mastodon](https://mastodon.social/share?text=I%20just%20used%20%40coderabbitai%20for%20my%20code%20review%2C%20and%20it%27s%20fantastic%21%20It%27s%20free%20for%20OSS%20and%20offers%20a%20free%20trial%20for%20the%20proprietary%20code.%20Check%20it%20out%3A%20https%3A%2F%2Fcoderabbit.ai)
- [Reddit](https://www.reddit.com/submit?title=Great%20tool%20for%20code%20review%20-%20CodeRabbit&text=I%20just%20used%20CodeRabbit%20for%20my%20code%20review%2C%20and%20it%27s%20fantastic%21%20It%27s%20free%20for%20OSS%20and%20offers%20a%20free%20trial%20for%20proprietary%20code.%20Check%20it%20out%3A%20https%3A//coderabbit.ai)
- [LinkedIn](https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fcoderabbit.ai&mini=true&title=Great%20tool%20for%20code%20review%20-%20CodeRabbit&summary=I%20just%20used%20CodeRabbit%20for%20my%20code%20review%2C%20and%20it%27s%20fantastic%21%20It%27s%20free%20for%20OSS%20and%20offers%20a%20free%20trial%20for%20proprietary%20code)

</details>

<sub>Comment `@coderabbitai help` to get the list of available commands and usage tips.</sub>

<!-- tips_end -->

<!-- internal state start -->


<!-- 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 -->

<!-- internal state end -->

---

### Comment 3

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T05:41:56Z
- **Comment ID**: 3845479809

**Comment Body:**

## Review Feedback Status Update

All CodeRabbit review feedback has been addressed:

### Already Fixed (6 items)
- ✅ `{{method}}` interpolation bug in `no_playwright_eval.rs` (commit e4cf020769)
- ✅ `{{method}}` interpolation bug in `no_playwright_element_handle.rs` (commit f22371ab8f)
- ✅ Missing `"clear"` method in METHODS_WITH_FORCE (commit ddaa7e7e9c)
- ✅ Parenthesized await handling in `no_playwright_missing_await.rs` (commit a4b3481a51)
- ✅ Doc example conflict in `no_playwright_page_pause.rs` (commit e5ff40ae5b)
- ✅ Changeset wording "over" → "to" (commit b2b9446a7c)

### Newly Fixed (4 nitpicks)
- ✅ Removed `pub` from `ElementHandleCall` fields (consistency with `EvalMethodCall`)
- ✅ Reused shared `get_page_or_frame_name` helper instead of local duplicate function
- ✅ Added rustdoc comments to `InvalidReason` enum
- ✅ Added clarifying comment to changeset code example

### Skipped with Justification (5 items)
- Empty options structs rustdoc comments - skipped because 381 existing empty options structs in Biome follow the same pattern without rustdoc

All tests pass and linting/formatting checks complete successfully.

---

### Comment 4

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T23:39:57Z
- **Comment ID**: 3850283266

**Comment Body:**

> **Actionable comments posted: 1**
> 
> 🤖 Fix all issues with AI agents
> ```
> In `@crates/biome_js_analyze/src/lint/nursery/no_playwright_useless_await.rs`:
> - Around line 194-260: is_sync_expect_call currently only recognizes expect()
> directly as the object of the matcher, so chains like await
> expect(x).not.toBe(1) are missed; update is_sync_expect_call to also handle when
> member_expr.object() is a JsStaticMemberExpression (e.g. the `.not` modifier)
> whose object is the JsCallExpression for expect() and whose member token text is
> "not" (treat `.not` as a synchronous modifier), then apply the same checks
> (including calling has_async_modifier(&expect_call, call_expr)) so `.not` chains
> are flagged as useless awaits; reference is_sync_expect_call,
> member_expr.object(), JsStaticMemberExpression, and has_async_modifier when
> making this change.
> ```
> 
> 🧹 Nitpick comments (2)
> > crates/biome_js_analyze/src/lint/nursery/use_playwright_valid_describe_callback.rs (2)> `56-66`: **Narrow `InvalidReason` visibility if it’s not part of the public API.**
> > > Keeping it `pub` widens the crate surface for little gain; `pub(crate)` (or private) is plenty unless external users rely on it.
> > > 
> > > 🔧 Suggested tweak
> > > ```diff
> > > -pub enum InvalidReason {
> > > +pub(crate) enum InvalidReason {
> > > ```
> > > 
> > > 
> > >     
> > >       
> > >     
> > > 
> > >       
> > >     
> > > 
> > >     
> > >   
> > > 
> > > 
> > > `68-94`: **Trim an allocation in `is_playwright_describe_call`.**
> > > You can match directly on `names.as_slice()` and drop the intermediate `Vec<&str>`; same behaviour, fewer allocations.
> > > 
> > > ♻️ Suggested tweak
> > > ```diff
> > > -    // Convert to &str slice for pattern matching
> > > -    let names_ref: Vec<&str> = names.iter().map(String::as_str).collect();
> > > -
> > > -    match names_ref.as_slice() {
> > > -        // describe()
> > > -        ["describe"] => Some(true),
> > > -        // test.describe()
> > > -        ["test", "describe"] => Some(true),
> > > -        // test.describe.only() / test.describe.skip()
> > > -        ["test", "describe", modifier] if is_describe_modifier(modifier) => Some(true),
> > > -        // test.describe.parallel() / test.describe.serial()
> > > -        ["test", "describe", mode] if is_describe_mode(mode) => Some(true),
> > > -        // test.describe.parallel.only() / test.describe.serial.only()
> > > -        ["test", "describe", mode, modifier]
> > > -            if is_describe_mode(mode) && is_describe_modifier(modifier) =>
> > > -        {
> > > -            Some(true)
> > > -        }
> > > -        _ => Some(false),
> > > -    }
> > > +    match names.as_slice() {
> > > +        [name] if name == "describe" => Some(true),
> > > +        [test, describe] if test == "test" && describe == "describe" => Some(true),
> > > +        [test, describe, modifier]
> > > +            if test == "test" && describe == "describe" && is_describe_modifier(modifier) =>
> > > +        {
> > > +            Some(true)
> > > +        }
> > > +        [test, describe, mode]
> > > +            if test == "test" && describe == "describe" && is_describe_mode(mode) =>
> > > +        {
> > > +            Some(true)
> > > +        }
> > > +        [test, describe, mode, modifier]
> > > +            if test == "test"
> > > +                && describe == "describe"
> > > +                && is_describe_mode(mode)
> > > +                && is_describe_modifier(modifier) =>
> > > +        {
> > > +            Some(true)
> > > +        }
> > > +        _ => Some(false),
> > > +    }
> > > ```

Addressed.

---

### Comment 5

- **Commenter**: codspeed-hq[bot]
- **Created**: 2026-02-07T00:27:00Z
- **Comment ID**: 3863131033

**Comment Body:**

<!-- __CODSPEED_PERFORMANCE_REPORT_COMMENT__ -->
## [CodSpeed Performance Report](https://codspeed.io/biomejs/biome/branches/abossenbroek%3Afeat%2Fplaywright-eslint-rules?utm_source=github&utm_medium=comment&utm_content=header)

### Merging this PR will **not alter performance**

<sub>Comparing <code>abossenbroek:feat/playwright-eslint-rules</code> (3b3af34) with <code>main</code> (2cba2b3)[^unexpected-base]</sub>
[^unexpected-base]: No successful run was found on <code>main</code> (16fd71d) during the generation of this report, so 2cba2b3 was used instead as the comparison base. There might be some changes unrelated to this pull request in this report.



### Summary

`✅ 58` untouched benchmarks  
`⏩ 95` skipped benchmarks[^skipped]  




[^skipped]: 95 benchmarks were skipped, so the baseline results were used instead. If they were deleted from the codebase, [click here and archive them to remove them from the performance reports](https://codspeed.io/biomejs/biome/branches/abossenbroek%3Afeat%2Fplaywright-eslint-rules?q=is%3Askipped&utm_source=github&utm_medium=comment&utm_content=archive).


---

### Comment 6

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T02:28:01Z
- **Comment ID**: 3863371630

**Comment Body:**

> There's a few changesets that are invalid. The only changesets that should exist should be ones that say they added a rule. The changesets that say they added a code fix or something don't make sense to have because the rules haven't been released yet.
> 
> This is kind of a big PR. I haven't reviewed the actual behavior of all the rules in depth, but at a glance it looks fine. Made a few notes that you should apply generally to all the rules.

Thanks @dyc3 for reviewing; it got absolutely massive. Feel free to share better ways to chunk this work for the subsequent rules to add to cover all playwright linting rules.

---

### Comment 7

- **Commenter**: dyc3
- **Created**: 2026-02-07T12:21:17Z
- **Comment ID**: 3864407273

**Comment Body:**

Generally, we recommend 1 rule per PR, but if the rules are particularly simple then grouping 2-3 in a single PR is still easy enough to review.

---

## crates/biome_js_analyze/src/ast_utils.rs

**4 comment(s)**

### Comment 1

- **Commenter**: dyc3
- **Created**: 2026-02-07T12:35:21Z
- **Comment ID**: 2777494452
- **Resolved**: False

**Comment Body:**

I don't understand this logic. It doesn't check for the `async` keyword anywhere. Could you elaborate?

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T16:25:34Z
- **Comment ID**: 2777748114
- **Resolved**: False

**Comment Body:**

your completly right, I missed `JsFunctionExpression`

---

### Comment 3

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T16:36:58Z
- **Comment ID**: 2777757114
- **Resolved**: False

**Comment Body:**

Good catch — this was indeed buggy. The `is_in_async_context` function delegated to `is_in_async_function`, which **silently skipped `JsFunctionExpression` nodes** during the ancestor walk (only handled arrow functions, declarations, and methods). This meant:

1. **False positive**: A sync `function() {}` expression inside an async function would walk through it into the outer async function, incorrectly returning `true`.
2. **False negative**: An `async function() {}` expression was never recognized as async.
3. **Missing boundaries**: The module-level walk also missed constructors, getters, setters, and static blocks — so autofix could insert invalid `await` in those contexts.

**Fix**: Rewrote both functions to use `AnyFunctionLike` (which covers all function-like nodes including `JsFunctionExpression`) with its `is_async()` method, and added `is_sync_only_function_boundary` for getters/setters/static blocks. Also renamed `is_in_async_context` → `is_await_allowed` since modules aren't really "async contexts" — `await` just happens to be valid there.

---

### Comment 4

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T18:40:13Z
- **Comment ID**: 2777917632
- **Resolved**: False

**Comment Body:**

Thanks for this review question — it turned out to be way more impactful than it looked! Your comment about the missing async keyword check made me dig into is_in_async_function, which revealed the JsFunctionExpression gap. But when I fixed that, I realized the same class of bug (incomplete function boundary lists) was present in three more places across the PR:

- is_call_awaited_or_returned in noPlaywrightMissingAwait — missing constructors, getters, setters, static blocks
- find_conditional_ancestor in noPlaywrightConditionalExpect — same issue
- is_conditional_skip in noPlaywrightSkippedTest — same issue

All fixed now by using the shared is_function_boundary helper. Also found two more edge cases while I was at it: .then()/.catch()/.finally() chains on Promise.all() not being recognized as awaited, and parenthesized arrow bodies (() => (expr)) failing the range comparison.

One well-placed review question, five bugs squashed. Much appreciated!

---

## crates/biome_js_analyze/src/frameworks/playwright.rs

**10 comment(s)**

### Comment 1

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T19:43:43Z
- **Comment ID**: 2765707613
- **Line**: 164
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟠 Major_

<details>
<summary>🧩 Analysis chain</summary>

🏁 Script executed:

```shell
# First, let's locate and examine the file
cd /repo && git ls-files | grep -E "playwright\.rs"
```

Repository: biomejs/biome

Length of output: 112

---

🏁 Script executed:

```shell
# Check the actual implementation
cat -n crates/biome_js_analyze/src/frameworks/playwright.rs | head -150
```

Repository: biomejs/biome

Length of output: 6311

---

🏁 Script executed:

```shell
# Search for usages of is_test_call to understand the impact
rg "is_test_call" --type rust -B 2 -A 2
```

Repository: biomejs/biome

Length of output: 15374

---

🏁 Script executed:

```shell
# Look for the usePlaywrightExpect rule mentioned in the comment
fd "playwright" crates/biome_js_analyze/src/lint --type f
```

Repository: biomejs/biome

Length of output: 1072

---

🏁 Script executed:

```shell
# Check if there's a test file for this function
fd "playwright" crates/biome_js_analyze/tests --type f
```

Repository: biomejs/biome

Length of output: 444

---

🌐 Web query:

`eslint-plugin-playwright expect-expect rule describe blocks`

💡 Result:

`playwright/expect-expect` is meant to enforce **at least one assertion inside each Playwright `test()` body** (“Enforce assertion to be made in a test body”)—it’s **not intended for `test.describe()` callbacks**, which are just grouping/registration code. [1]

If you’re seeing it complain on a `describe` block, it’s usually one of these situations:

- You’re actually running a *different* `expect-expect` rule (commonly `jest/expect-expect`) via a Jest config/extend, rather than `playwright/expect-expect`. Make sure your Playwright tests use `plugin:playwright/recommended` (or the flat equivalent) and that the rule name is `playwright/expect-expect`. [2]
- Your “assertion” isn’t a plain `expect(...)` call (e.g., you wrapped assertions in a helper). In that case, teach the Playwright plugin what counts as an assertion using `settings.playwright.additionalAssertFunctionNames` (or the rule option with the same name). [1]

**Config example (flat):**
```js
export default [
  {
    settings: {
      playwright: {
        additionalAssertFunctionNames: ['assertCustomCondition'],
      },
    },
  },
]
``` 
[1]

**Sources:**  
[1] eslint-plugin-playwright (npm) – global setting `additionalAssertFunctionNames` and rule description for `expect-expect` https://www.npmjs.com/package/eslint-plugin-playwright/v/0.22.1  
[2] eslint-plugin-playwright (GitHub README) – recommended config / rule list https://github.com/playwright-community/eslint-plugin-playwright

---

🏁 Script executed:

```shell
sed -n '1,80p' crates/biome_js_analyze/src/lint/nursery/use_playwright_expect.rs
```

Repository: biomejs/biome

Length of output: 2523

---

🏁 Script executed:

```shell
sed -n '1,150p' crates/biome_js_analyze/src/lint/nursery/use_playwright_expect.rs
```

Repository: biomejs/biome

Length of output: 3698

---



</details>

**Exclude `describe` blocks from `is_test_call`.**

`is_test_call` recursively checks only the object part of member expressions, never the member name itself. This means `test.describe()` incorrectly returns `true`, causing `usePlaywrightExpect` to flag describe blocks (which are just grouping code) as requiring assertions—behaviour that should only apply to actual test functions like `test()` and `it()`.

The proposed fix is correct: add an early return in the `JsStaticMemberExpression` branch to filter out `describe` (and its modifiers like `describe.skip` and `describe.only`).

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/frameworks/playwright.rs` around lines 120 - 141,
The is_test_call function incorrectly treats member expressions like
test.describe() as tests; modify the AnyJsExpression::JsStaticMemberExpression
arm in is_test_call to first inspect the member/property identifier (e.g., via
member.property() -> JsName or token text) and return false if that name is
"describe" (and its modifiers like "describe.skip" / "describe.only"), before
recursing into member.object(); keep existing logic for JsIdentifierExpression
and recursion after this early check so only true test identifiers ("test" or
"it") are accepted.
```

</details>

<!-- fingerprinting:phantom:poseidon:eagle -->

<!-- This is an auto-generated comment by CodeRabbit -->

✅ Addressed in commits c4cd673 to a425297

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T23:03:26Z
- **Comment ID**: 2766338743
- **Line**: 164
- **Resolved**: False

**Comment Body:**

great point. Addressed.

---

### Comment 3

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-05T23:58:07Z
- **Comment ID**: 2771654591
- **Line**: 189
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

<details>
<summary>🧩 Analysis chain</summary>

🏁 Script executed:

```shell
cat -n crates/biome_js_analyze/src/frameworks/playwright.rs | sed -n '150,180p'
```

Repository: biomejs/biome

Length of output: 1243

---

🏁 Script executed:

```shell
# Check for other rustdoc examples in the same file to see the pattern
rg -A 3 "^/// \`" crates/biome_js_analyze/src/frameworks/playwright.rs | head -30
```

Repository: biomejs/biome

Length of output: 262

---

🏁 Script executed:

````shell
# Look for similar function documentation examples in the codebase
rg -B 2 -A 2 '```javascript' crates/biome_js_analyze/src/frameworks/ | head -50
````

Repository: biomejs/biome

Length of output: 39

---



</details>

**Wrap the inline example in a rustdoc code block.**  
The example showing JavaScript test syntax should use code block formatting for consistency with rustdoc conventions.

```diff
-/// `test("name", { retry: () => 2 }, async () => { ... })`
+/// ```javascript
+/// test("name", { retry: () => 2 }, async () => { /* ... */ })
+/// ```
```

<details>
<summary>🤖 Prompt for AI Agents</summary>

````
In `@crates/biome_js_analyze/src/frameworks/playwright.rs` around lines 156 - 177,
Update the rustdoc above get_test_callback to use a fenced code block for the JS
example: replace the inline example `test("name", { retry: () => 2 }, async ()
=> { ... })` with a fenced block starting with ```javascript, the example line,
and closing ``` so the comment becomes a proper rustdoc code block; target the
doc comment immediately above the get_test_callback function.
````

</details>

<!-- fingerprinting:phantom:poseidon:eagle -->

<!-- This is an auto-generated comment by CodeRabbit -->

✅ Addressed in commit 8874566

---

### Comment 4

- **Commenter**: dyc3
- **Created**: 2026-02-07T00:22:53Z
- **Comment ID**: 2776515079
- **Line**: 189
- **Resolved**: False

**Comment Body:**

this is valid

---

### Comment 5

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T00:46:54Z
- **Comment ID**: 2776568358
- **Line**: 189
- **Resolved**: False

**Comment Body:**

Thanks; fixed.

---

### Comment 6

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-07T00:49:21Z
- **Comment ID**: 2776572616
- **Line**: 109
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

**Partial chain returned when `member.object()` fails.**

If `member.object()` returns `Err`, the recursion is skipped but the member name is still pushed. This yields a truncated chain (e.g. just `["skip"]` instead of `["test", "describe", "skip"]`), which could confuse downstream callers expecting a complete chain.

In practice, parse errors here are extremely unlikely, so this is low-risk. If you want to tighten it:

<details>
<summary>Proposed fix</summary>

```diff
         AnyJsExpression::JsStaticMemberExpression(member) => {
             // First recurse on object to get outer names
-            if let Ok(object) = member.object() {
-                collect_member_names_rec(&object, names)?;
-            }
+            let object = member.object().ok()?;
+            collect_member_names_rec(&object, names)?;
             // Then add this member name
```

Same pattern applies to the `JsComputedMemberExpression` arm on lines 100-104.
</details>

<!-- suggestion_start -->

<details>
<summary>📝 Committable suggestion</summary>

> ‼️ **IMPORTANT**
> Carefully review the code before committing. Ensure that it accurately replaces the highlighted code, contains no missing lines, and has no issues with indentation. Thoroughly test & benchmark the code to ensure it meets the requirements.

```suggestion
        AnyJsExpression::JsStaticMemberExpression(member) => {
            // First recurse on object to get outer names
            let object = member.object().ok()?;
            collect_member_names_rec(&object, names)?;
            // Then add this member name
            let m = member.member().ok()?;
            let n = m.as_js_name()?;
            let t = n.value_token().ok()?;
            names.push(t.token_text_trimmed());
            Some(())
```

</details>

<!-- suggestion_end -->

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/frameworks/playwright.rs` around lines 88 - 98,
The static member arm pushes the member name even when member.object() fails,
producing a truncated chain; change the logic in the
AnyJsExpression::JsStaticMemberExpression arm so that if member.object() returns
Err you do not continue (e.g., return None or propagate the error) instead of
just skipping recursion, and only push the member name after a successful
object() and recursive collect_member_names_rec call; apply the same fix to the
AnyJsExpression::JsComputedMemberExpression arm so both arms consistently
require a successful object() before adding the member to names.
```

</details>

<!-- fingerprinting:phantom:poseidon:phoenix -->

<!-- This is an auto-generated comment by CodeRabbit -->

✅ Addressed in commit a833978

---

### Comment 7

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-07T00:49:22Z
- **Comment ID**: 2776572618
- **Line**: 177
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

**`arg.ok()?` discards an already-found callback on a later parse error.**

If the iterator yields an `Err` for a trailing argument, the `?` returns `None` from the whole function — even though a valid callback was already captured. Consider using `filter_map` or just `continue` on error:

<details>
<summary>Suggested tweak</summary>

```diff
     for arg in arg_list.iter() {
-        let arg = arg.ok()?;
+        let Ok(arg) = arg else { continue };
         if let Some(expr) = arg.as_any_js_expression() {
```
</details>

<!-- suggestion_start -->

<details>
<summary>📝 Committable suggestion</summary>

> ‼️ **IMPORTANT**
> Carefully review the code before committing. Ensure that it accurately replaces the highlighted code, contains no missing lines, and has no issues with indentation. Thoroughly test & benchmark the code to ensure it meets the requirements.

```suggestion
    for arg in arg_list.iter() {
        let Ok(arg) = arg else { continue };
        if let Some(expr) = arg.as_any_js_expression() {
```

</details>

<!-- suggestion_end -->

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/frameworks/playwright.rs` around lines 165 - 167,
The loop over arg_list.iter() uses arg.ok()? which returns early on the first
Err and discards any previously captured callback; change the error handling so
a parse error for a trailing argument does not return None for the whole
function — e.g., replace the arg.ok()? usage with explicit handling (match or if
let Ok(arg) = arg { ... } else { continue }) or pre-filter the iterator
(filter_map) so you only inspect successfully parsed args before calling
as_any_js_expression(), ensuring any previously found callback remains valid;
update the block around arg_list.iter(), the local variable arg and the
as_any_js_expression() branch accordingly.
```

</details>

<!-- fingerprinting:phantom:poseidon:phoenix -->

<!-- This is an auto-generated comment by CodeRabbit -->

---

### Comment 8

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T00:53:31Z
- **Comment ID**: 2776578946
- **Line**: 189
- **Resolved**: False

**Comment Body:**

fixed.

---

### Comment 9

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T02:24:42Z
- **Comment ID**: 2776744643
- **Line**: 177
- **Resolved**: False

**Comment Body:**

fixed.

---

### Comment 10

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T02:25:45Z
- **Comment ID**: 2776745560
- **Line**: 109
- **Resolved**: False

**Comment Body:**

fixed.

---

## crates/biome_js_analyze/src/lint/nursery/expect_playwright_expect.rs

**6 comment(s)**

### Comment 1

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T05:21:17Z
- **Comment ID**: 2762252840
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

**Potential false positive: callback selection logic.**

The function returns the *first* function argument, but Playwright/Jest test callbacks are conventionally the *last* argument. Consider `test("name", { retry: () => 2 }, async () => { ... })` — this would incorrectly analyse the retry function instead of the actual test callback.



<details>
<summary>🛠️ Suggested fix</summary>

```diff
 fn get_test_callback(args: &biome_js_syntax::JsCallArguments) -> Option<AnyJsExpression> {
     let arg_list = args.args();
+    let mut callback = None;
 
     // Iterate through arguments to find the callback (function expression or arrow function)
     for arg in arg_list.iter() {
         let arg = arg.ok()?;
         if let Some(expr) = arg.as_any_js_expression() {
             match expr {
                 AnyJsExpression::JsArrowFunctionExpression(_)
                 | AnyJsExpression::JsFunctionExpression(_) => {
-                    return Some(expr.clone());
+                    callback = Some(expr.clone());
                 }
                 _ => {}
             }
         }
     }
 
-    None
+    callback
 }
```

</details>

<!-- suggestion_start -->

<details>
<summary>📝 Committable suggestion</summary>

> ‼️ **IMPORTANT**
> Carefully review the code before committing. Ensure that it accurately replaces the highlighted code, contains no missing lines, and has no issues with indentation. Thoroughly test & benchmark the code to ensure it meets the requirements.

```suggestion
fn get_test_callback(args: &biome_js_syntax::JsCallArguments) -> Option<AnyJsExpression> {
    let arg_list = args.args();
    let mut callback = None;

    // Iterate through arguments to find the callback (function expression or arrow function)
    for arg in arg_list.iter() {
        let arg = arg.ok()?;
        if let Some(expr) = arg.as_any_js_expression() {
            match expr {
                AnyJsExpression::JsArrowFunctionExpression(_)
                | AnyJsExpression::JsFunctionExpression(_) => {
                    callback = Some(expr.clone());
                }
                _ => {}
            }
        }
    }

    callback
}
```

</details>

<!-- suggestion_end -->

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/lint/nursery/expect_playwright_expect.rs` around
lines 126 - 144, get_test_callback currently returns the first function argument
it finds, which yields false positives for calls like test("name", { retry: ...
}, async () => ...) — update get_test_callback (which accepts
&biome_js_syntax::JsCallArguments and returns Option<AnyJsExpression>) to choose
the last argument that is a function/arrow function instead of the first:
iterate the args in reverse (or collect and scan from the end), check
arg.as_any_js_expression() and match AnyJsExpression::JsArrowFunctionExpression
| AnyJsExpression::JsFunctionExpression, and return the last matching
expr.clone(); ensure non-function args are skipped so the actual test callback
is selected.
```

</details>

<!-- fingerprinting:phantom:medusa:ocelot -->

<!-- This is an auto-generated comment by CodeRabbit -->

---

### Comment 2

- **Commenter**: dyc3
- **Created**: 2026-02-04T10:37:08Z
- **Comment ID**: 2763330020
- **Resolved**: False

**Comment Body:**

This doesn't follow our naming guidelines. It must start with `use` or `no`

---

### Comment 3

- **Commenter**: dyc3
- **Created**: 2026-02-04T10:39:35Z
- **Comment ID**: 2763339183
- **Resolved**: False

**Comment Body:**

Would it make sense to move this to `crates/biome_js_analyze/src/frameworks/playwright.rs`

---

### Comment 4

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T19:30:56Z
- **Comment ID**: 2765661171
- **Resolved**: False

**Comment Body:**

done, thanks for pointing me to this.

---

### Comment 5

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T22:51:43Z
- **Comment ID**: 2766310278
- **Resolved**: False

**Comment Body:**

addressed.

---

### Comment 6

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T23:41:03Z
- **Comment ID**: 2766427455
- **Resolved**: False

**Comment Body:**

addressed

---

## crates/biome_js_analyze/src/lint/nursery/no_playwright_conditional_expect.rs

**4 comment(s)**

### Comment 1

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T18:23:33Z
- **Comment ID**: 2765397314
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟠 Major_

**Conditional expects with `.not`/`.resolves` won’t be detected.**  
`is_part_of_expect_chain` doesn’t recurse through member‑of‑member chains, so common patterns like `expect(page).not.toHaveTitle()` never match. That’s a functional miss.


<details>
<summary>🔧 Proposed fix (recurse through member chains)</summary>

```diff
 fn is_part_of_expect_chain(call: &JsCallExpression) -> bool {
     let Ok(callee) = call.callee() else {
         return false;
     };
 
-    match &callee {
-        AnyJsExpression::JsIdentifierExpression(id) => {
-            if let Ok(name) = id.name()
-                && let Ok(token) = name.value_token()
-            {
-                return token.text_trimmed() == "expect";
-            }
-            false
-        }
-        AnyJsExpression::JsStaticMemberExpression(member) => {
-            // expect.soft(), expect.poll(), etc., or expect(...).method()
-            if let Ok(object) = member.object() {
-                match object {
-                    AnyJsExpression::JsIdentifierExpression(id) => {
-                        if let Ok(name) = id.name()
-                            && let Ok(token) = name.value_token()
-                        {
-                            return token.text_trimmed() == "expect";
-                        }
-                    }
-                    AnyJsExpression::JsCallExpression(inner_call) => {
-                        return is_part_of_expect_chain(&inner_call);
-                    }
-                    _ => {}
-                }
-            }
-            false
-        }
-        AnyJsExpression::JsCallExpression(inner_call) => is_part_of_expect_chain(inner_call),
-        _ => false,
-    }
+    fn is_expect_chain_expr(expr: &AnyJsExpression) -> bool {
+        match expr {
+            AnyJsExpression::JsIdentifierExpression(id) => {
+                if let Ok(name) = id.name()
+                    && let Ok(token) = name.value_token()
+                {
+                    return token.text_trimmed() == "expect";
+                }
+                false
+            }
+            AnyJsExpression::JsCallExpression(inner_call) => is_part_of_expect_chain(inner_call),
+            AnyJsExpression::JsStaticMemberExpression(member) => {
+                if let Ok(object) = member.object() {
+                    return is_expect_chain_expr(&object);
+                }
+                false
+            }
+            _ => false,
+        }
+    }
+
+    is_expect_chain_expr(&callee)
 }
```

</details>

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/lint/nursery/no_playwright_conditional_expect.rs`
around lines 131 - 167, is_part_of_expect_chain currently fails to detect
chained member patterns like expect(...).not.toHaveTitle because it only checks
the immediate member.object for an identifier or call; update
is_part_of_expect_chain to recursively traverse member chains by handling
AnyJsExpression::JsStaticMemberExpression as well as
AnyJsExpression::JsCallExpression when encountered as member.object(), so that
encountering a nested member (e.g., member.object() -> JsStaticMemberExpression)
will recurse into that member's object/property and ultimately detect the
identifier "expect" (refer to is_part_of_expect_chain,
AnyJsExpression::JsStaticMemberExpression, AnyJsExpression::JsCallExpression).
```

</details>

<!-- fingerprinting:phantom:poseidon:eagle -->

<!-- This is an auto-generated comment by CodeRabbit -->

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T20:32:16Z
- **Comment ID**: 2765866029
- **Resolved**: False

**Comment Body:**

Good catch; addressed this as it indeed was a missing feature in the code.

---

### Comment 3

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-07T02:34:49Z
- **Comment ID**: 2776766532
- **Line**: 172
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟠 Major_

<details>
<summary>🧩 Analysis chain</summary>

🌐 Web query:

`eslint-plugin-playwright no-conditional-expect catch block`

💡 Result:

`playwright/no-conditional-expect` flags **any `expect(...)` that only runs on some code paths**—and a `catch { ... }` is conditional (it executes only if something throws), so `expect` inside a `catch` will be reported. The rule’s intent is “Disallow calling `expect` conditionally.” [1][2]

### Fix patterns (pick one)

**1) Don’t catch; assert the rejection instead (preferred)**
```ts
await expect(doThing()).rejects.toThrow(/boom/);
```

**2) Keep `catch` for side effects only, then rethrow (no `expect` in `catch`)**
```ts
try {
  await doThing();
} catch (e) {
  await attachLogsOrScreenshot();
  throw e; // let the test fail naturally
}
```

**3) Make the assertion unconditional (move it out of `catch`)**
```ts
let err: unknown;
try {
  await doThing();
} catch (e) {
  err = e;
}
expect(err).toBeTruthy(); // runs every time
```

If you share your exact snippet + what you’re trying to assert in the `catch`, I can rewrite it in the cleanest style.

**Sources:** [1] [2]

---



</details>

**Add `catch` clause detection to match the ESLint source rule.**

The `no-conditional-expect` rule in eslint-plugin-playwright flags `expect()` inside `catch` blocks since they execute only if an error is thrown. The Biome implementation declares `.same()` behaviour but is missing this check.

<details>
<summary>Fix</summary>

```diff
             // Logical expressions that short-circuit
             JsSyntaxKind::JS_LOGICAL_EXPRESSION => return Some("logical expression"),
 
+            // Catch clauses — expect() won't run if the error isn't thrown
+            JsSyntaxKind::JS_CATCH_CLAUSE => return Some("catch clause"),
+
             // Stop at function boundaries (the test callback)
             JsSyntaxKind::JS_FUNCTION_EXPRESSION
```
</details>

<!-- suggestion_start -->

<details>
<summary>📝 Committable suggestion</summary>

> ‼️ **IMPORTANT**
> Carefully review the code before committing. Ensure that it accurately replaces the highlighted code, contains no missing lines, and has no issues with indentation. Thoroughly test & benchmark the code to ensure it meets the requirements.

```suggestion
fn is_in_conditional_context(call: &JsCallExpression) -> Option<&'static str> {
    for ancestor in call.syntax().ancestors().skip(1) {
        match ancestor.kind() {
            // Direct conditional statements
            JsSyntaxKind::JS_IF_STATEMENT => return Some("if statement"),
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => return Some("ternary expression"),
            JsSyntaxKind::JS_SWITCH_STATEMENT => return Some("switch statement"),
            JsSyntaxKind::JS_CASE_CLAUSE | JsSyntaxKind::JS_DEFAULT_CLAUSE => {
                return Some("switch case")
            }

            // Logical expressions that short-circuit
            JsSyntaxKind::JS_LOGICAL_EXPRESSION => return Some("logical expression"),

            // Catch clauses — expect() won't run if the error isn't thrown
            JsSyntaxKind::JS_CATCH_CLAUSE => return Some("catch clause"),

            // Stop at function boundaries (the test callback)
            JsSyntaxKind::JS_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_FUNCTION_DECLARATION => {
                break;
            }

            _ => {}
        }
    }

    None
}
```

</details>

<!-- suggestion_end -->

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/lint/nursery/no_playwright_conditional_expect.rs`
around lines 135 - 161, The is_in_conditional_context function currently misses
catch blocks; update its ancestor.kind() match to detect
JsSyntaxKind::JS_CATCH_FINALLY or JsSyntaxKind::JS_CATCH_CLAUSE (whichever
symbol your parser uses) and return Some("catch clause") so expect() inside
catch blocks is treated as conditional, keeping the existing short-circuit and
function-boundary logic intact; ensure you add the new arm alongside the
existing Js_CASE_CLAUSE/JS_DEFAULT_CLAUSE branch in is_in_conditional_context.
```

</details>

<!-- fingerprinting:phantom:poseidon:phoenix -->

<!-- This is an auto-generated comment by CodeRabbit -->

✅ Addressed in commit deed796

---

### Comment 4

- **Commenter**: dyc3
- **Created**: 2026-02-08T17:55:45Z
- **Comment ID**: 2779596429
- **Line**: 68
- **Resolved**: False

**Comment Body:**

There's a similar rule for jest. https://github.com/jest-community/eslint-plugin-jest/blob/main/docs/rules/no-conditional-expect.md 
vitest too https://github.com/vitest-dev/eslint-plugin-vitest/blob/main/docs/rules/no-conditional-expect.md

I would say put this in the `test` domain, rename it to `noConditionalExpect`, and add those as rule sources.

---

## crates/biome_js_analyze/src/lint/nursery/no_playwright_element_handle.rs

**2 comment(s)**

### Comment 1

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T01:15:54Z
- **Comment ID**: 2761717856
- **Line**: 109
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🔴 Critical_

**Bug: Double braces produce literal text instead of variable interpolation.**

Same issue as in `no_playwright_eval.rs`. `{{method}}` outputs literal `{method}` rather than the actual method name (`$` or `$$`).


<details>
<summary>🐛 Proposed fix</summary>

```diff
 .note(markup! {
-    "Element handles like "<Emphasis>{receiver}"."{{method}}"()"</Emphasis>" are discouraged."
+    "Element handles like "<Emphasis>{receiver}"."{method}"()"</Emphasis>" are discouraged."
 })
```
</details>

<!-- suggestion_start -->

<details>
<summary>📝 Committable suggestion</summary>

> ‼️ **IMPORTANT**
> Carefully review the code before committing. Ensure that it accurately replaces the highlighted code, contains no missing lines, and has no issues with indentation. Thoroughly test & benchmark the code to ensure it meets the requirements.

```suggestion
            .note(markup! {
                "Element handles like "<Emphasis>{receiver}"."{method}"()"</Emphasis>" are discouraged."
            })
```

</details>

<!-- suggestion_end -->

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/lint/nursery/no_playwright_element_handle.rs`
around lines 109 - 111, The markup! invocation is using double braces
"{{method}}" which renders the literal "{method}" instead of interpolating the
variable; update the markup so the method variable is passed/interpolated
correctly (replace the literal "{{method}}" with the actual interpolation
{method}) inside the Emphasis node in the note—look for the markup! block that
builds the note with "Element handles like" and variables receiver and method
and change the method token to use {method} interpolation.
```

</details>

<!-- fingerprinting:phantom:medusa:ocelot -->

<!-- This is an auto-generated comment by CodeRabbit -->

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T03:29:48Z
- **Comment ID**: 2762000983
- **Line**: 109
- **Resolved**: False

**Comment Body:**

✅ Fixed in commit f22371ab8f - Changed `{{method}}` to `{method}` for correct interpolation.

---

## crates/biome_js_analyze/src/lint/nursery/no_playwright_eval.rs

**2 comment(s)**

### Comment 1

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T01:15:54Z
- **Comment ID**: 2761717862
- **Line**: 93
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🔴 Critical_

**Bug: Double braces produce literal text instead of variable interpolation.**

`{{method}}` outputs the literal string `{method}` rather than interpolating the `method` variable. The diagnostic will display `page.{method}()` instead of `page.$eval()`.


<details>
<summary>🐛 Proposed fix</summary>

```diff
 markup! {
-    "Unexpected use of "<Emphasis>{receiver}"."{{method}}"()"</Emphasis>"."
+    "Unexpected use of "<Emphasis>{receiver}"."{method}"()"</Emphasis>"."
 },
```
</details>

<!-- suggestion_start -->

<details>
<summary>📝 Committable suggestion</summary>

> ‼️ **IMPORTANT**
> Carefully review the code before committing. Ensure that it accurately replaces the highlighted code, contains no missing lines, and has no issues with indentation. Thoroughly test & benchmark the code to ensure it meets the requirements.

```suggestion
                markup! {
                    "Unexpected use of "<Emphasis>{receiver}"."{method}"()"</Emphasis>"."
                },
```

</details>

<!-- suggestion_end -->

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/lint/nursery/no_playwright_eval.rs` around lines
96 - 98, The diagnostic string in the markup! macro is using "{{method}}" which
emits the literal "{method}" instead of the method variable; update the
interpolation to use a single-brace expression (replace "{{method}}" with
{method}) inside the Emphasis element so the displayed message uses the actual
method value (refer to the markup! invocation where "Unexpected use of
"<Emphasis>{receiver}"."{{method}}"()"</Emphasis>" is constructed).
```

</details>

<!-- fingerprinting:phantom:medusa:ocelot -->

<!-- This is an auto-generated comment by CodeRabbit -->

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T03:29:43Z
- **Comment ID**: 2762000822
- **Line**: 93
- **Resolved**: False

**Comment Body:**

✅ Fixed in commit e4cf020769 - Changed `{{method}}` to `{method}` for correct interpolation.

---

## crates/biome_js_analyze/src/lint/nursery/no_playwright_force_option.rs

**3 comment(s)**

### Comment 1

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T01:15:54Z
- **Comment ID**: 2761717864
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟠 Major_

<details>
<summary>🧩 Analysis chain</summary>

🏁 Script executed:

```shell
# First, let's see the full file to understand the context and check for tests
cat -n crates/biome_js_analyze/src/lint/nursery/no_playwright_force_option.rs | head -100
```

Repository: biomejs/biome

Length of output: 3825

---

🌐 Web query:

`Playwright locator force option supported methods API documentation`

💡 Result:

In Playwright, the Locator `force` option (“bypass the actionability checks”) is supported on these Locator *action* methods (see each method’s `options.force` in the Locator API docs) [1]:

- `locator.click({ force: true })` [1]  
- `locator.dblclick({ force: true })` [1]  
- `locator.hover({ force: true })` [1]  
- `locator.tap({ force: true })` [1]  
- `locator.fill(value, { force: true })` [1]  
- `locator.clear({ force: true })` [1]  
- `locator.check({ force: true })` / `locator.uncheck({ force: true })` / `locator.setChecked(checked, { force: true })` [1]  
- `locator.dragTo(target, { force: true })` [1]  
- `locator.selectOption(values, { force: true })` [1]  
- `locator.selectText({ force: true })` [1]

**Docs:** Playwright → API reference → `class: Locator` [1].  

**Source**  
[1] https://playwright.dev/docs/api/class-locator

---

🏁 Script executed:

```shell
# Search for test files related to this rule
fd -t f "no_playwright_force_option" crates/ | grep -i test
```

Repository: biomejs/biome

Length of output: 39

---

🏁 Script executed:

```shell
# Also check if there are any tests in the same directory or a tests folder
fd -t f "test" crates/biome_js_analyze/src/lint/nursery/ | head -20
```

Repository: biomejs/biome

Length of output: 127

---

🏁 Script executed:

```shell
# Search for tests that might validate the sorting
rg "METHODS_WITH_FORCE" crates/biome_js_analyze/src/ -A 5 -B 5
```

Repository: biomejs/biome

Length of output: 3040

---



</details>

**Add `clear` method to the list.**

The test enforcing sorted order is good, but Playwright's API documentation shows the `clear()` method also supports the `force` option. This would need to be added alphabetically between "check" and "click" to maintain coverage.

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/lint/nursery/no_playwright_force_option.rs`
around lines 57 - 70, The METHODS_WITH_FORCE array is missing Playwright's
"clear" method; add "clear" into the slice alphabetically (i.e., between "check"
and "click") in the const METHODS_WITH_FORCE so the list remains sorted for the
binary search-based lookup in no_playwright_force_option.rs.
```

</details>

<!-- fingerprinting:phantom:medusa:ocelot -->

<!-- This is an auto-generated comment by CodeRabbit -->

✅ Addressed in commit 4cb849c

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T03:29:55Z
- **Comment ID**: 2762001130
- **Resolved**: False

**Comment Body:**

✅ Fixed in commit ddaa7e7e9c - Added `"clear"` to METHODS_WITH_FORCE array (alphabetically between "check" and "click").

---

### Comment 3

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T05:21:17Z
- **Comment ID**: 2762252849
- **Line**: 97
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

**Handle parenthesised object literals to avoid false negatives.**  
Line 104 only matches direct object expressions, so `click(({ force: true }))` won’t be flagged. Consider unwrapping `JsParenthesizedExpression` before the object check.

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/lint/nursery/no_playwright_force_option.rs`
around lines 100 - 110, The loop over call_expr.arguments() currently only
matches AnyJsExpression::JsObjectExpression directly, missing parenthesised
object literals; update the argument handling to unwrap
JsParenthesizedExpression (recursively if needed) before checking for object
expressions so invocations like click(({ force: true })) are detected; you can
either modify the matching in the loop to handle
AnyJsExpression::JsParenthesizedExpression and extract its inner JsExpression or
add a small helper (used by the loop and/or has_force_true) that returns the
inner unparenthesized JsObjectExpression for inspection.
```

</details>

<!-- fingerprinting:phantom:poseidon:eagle -->

<!-- This is an auto-generated comment by CodeRabbit -->

✅ Addressed in commit d9d3200

---

## crates/biome_js_analyze/src/lint/nursery/no_playwright_missing_await.rs

**6 comment(s)**

### Comment 1

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T01:15:54Z
- **Comment ID**: 2761717866
- **Line**: 433
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

<details>
<summary>🧩 Analysis chain</summary>

🏁 Script executed:

```shell
find crates/biome_js_analyze/tests/specs/nursery -name "*noPlaywrightMissingAwait*" -type f
```

Repository: biomejs/biome

Length of output: 39

---

🏁 Script executed:

```shell
rg -A 3 -B 3 'await\s*\(' crates/biome_js_analyze/tests/specs/nursery --glob '*noPlaywrightMissingAwait*'
```

Repository: biomejs/biome

Length of output: 39

---

🏁 Script executed:

```shell
find crates/biome_js_analyze/tests -type f -name "*laywright*" -o -name "*Playwright*"
```

Repository: biomejs/biome

Length of output: 1279

---

🏁 Script executed:

```shell
fd -i playwright crates/biome_js_analyze/tests/
```

Repository: biomejs/biome

Length of output: 1291

---

🏁 Script executed:

```shell
ls -la crates/biome_js_analyze/tests/specs/nursery/noPlaywrightMissingAwait/
```

Repository: biomejs/biome

Length of output: 283

---

🏁 Script executed:

```shell
cat crates/biome_js_analyze/tests/specs/nursery/noPlaywrightMissingAwait/valid/*.js
```

Repository: biomejs/biome

Length of output: 2468

---

🏁 Script executed:

```shell
cat crates/biome_js_analyze/tests/specs/nursery/noPlaywrightMissingAwait/invalid/*.js
```

Repository: biomejs/biome

Length of output: 2355

---

🏁 Script executed:

```shell
cat -n crates/biome_js_analyze/src/lint/nursery/no_playwright_missing_await.rs | sed -n '374,414p'
```

Repository: biomejs/biome

Length of output: 1883

---

🏁 Script executed:

```shell
rg -A 50 "fn is_call_awaited_or_returned" crates/biome_js_analyze/src/lint/nursery/no_playwright_missing_await.rs | head -60
```

Repository: biomejs/biome

Length of output: 1999

---



</details>

**Handle parenthesised awaits to avoid false positives.**

`await (expect(...).toBeVisible())` won't be recognised as awaited because the code only checks the immediate parent. Walk through parenthesised expression nodes to find the actual await.

<details>
<summary>Suggested fix</summary>

```diff
-    // Check if it's awaited
-    if let Some(parent) = &parent
-        && parent.kind() == biome_js_syntax::JsSyntaxKind::JS_AWAIT_EXPRESSION
-    {
-        return true;
-    }
+    // Check if it's awaited (allow parenthesised expressions)
+    let mut await_parent = parent.clone();
+    while let Some(node) = await_parent.as_ref() {
+        match node.kind() {
+            biome_js_syntax::JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => {
+                await_parent = node.parent();
+            }
+            biome_js_syntax::JsSyntaxKind::JS_AWAIT_EXPRESSION => {
+                return true;
+            }
+            _ => break,
+        }
+    }
```
</details>

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/lint/nursery/no_playwright_missing_await.rs`
around lines 374 - 414, The is_call_awaited_or_returned function fails to detect
awaits wrapped in parenthesized expressions; update its parent traversal so that
before checking for JS_AWAIT_EXPRESSION (and when comparing the arrow function
body) you unwrap any JS_PARENTHESIZED_EXPRESSION nodes by walking up through
parenthesised-expression ancestors to find the real enclosing node, and when
comparing the arrow body use the unwrapped body expression's text range (or
unwrap the call_expr's parent chain similarly) so constructs like await
(expect(...).toBeVisible()) are recognized as awaited.
```

</details>

<!-- fingerprinting:phantom:medusa:eagle -->

<!-- This is an auto-generated comment by CodeRabbit -->

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T03:30:01Z
- **Comment ID**: 2762001219
- **Line**: 433
- **Resolved**: False

**Comment Body:**

✅ Fixed in commit a4b3481a51 - Added traversal through `JS_PARENTHESIZED_EXPRESSION` nodes when checking for await, so `await (expect(...).toBeVisible())` is now correctly recognized.

---

### Comment 3

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T05:21:17Z
- **Comment ID**: 2762252851
- **Line**: 317
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

<details>
<summary>🧩 Analysis chain</summary>

🏁 Script executed:

```shell
cat -n crates/biome_js_analyze/src/lint/nursery/no_playwright_missing_await.rs | head -320 | tail -60
```

Repository: biomejs/biome

Length of output: 2344

---

🏁 Script executed:

```shell
cat -n crates/biome_js_analyze/src/lint/nursery/no_playwright_missing_await.rs | head -350 | tail -50
```

Repository: biomejs/biome

Length of output: 2190

---

🏁 Script executed:

```shell
cat -n crates/biome_js_analyze/src/lint/nursery/no_playwright_missing_await.rs | head -375 | tail -50
```

Repository: biomejs/biome

Length of output: 2161

---

🏁 Script executed:

```shell
rg "get_async_expect_matcher" crates/biome_js_analyze/src/lint/nursery/no_playwright_missing_await.rs -B 3 -A 3
```

Repository: biomejs/biome

Length of output: 531

---

🏁 Script executed:

```shell
rg "MissingAwaitType::ExpectPoll" crates/biome_js_analyze/src/lint/nursery/no_playwright_missing_await.rs -B 2 -A 2
```

Repository: biomejs/biome

Length of output: 395

---

🏁 Script executed:

```shell
rg "expect.poll" crates/biome_js_analyze/src/lint/nursery/no_playwright_missing_await.rs -B 3 -A 3
```

Repository: biomejs/biome

Length of output: 902

---

🏁 Script executed:

```shell
rg "no_playwright_missing_await" crates/biome_js_analyze/tests/ -l
```

Repository: biomejs/biome

Length of output: 39

---

🏁 Script executed:

```shell
fd "no_playwright_missing_await" crates/biome_js_analyze/
```

Repository: biomejs/biome

Length of output: 128

---

🏁 Script executed:

```shell
fd "playwright" crates/biome_js_analyze/tests/ -type f
```

Repository: biomejs/biome

Length of output: 227

---

🏁 Script executed:

```shell
fd "playwright" crates/biome_js_analyze/tests/
```

Repository: biomejs/biome

Length of output: 1434

---

🏁 Script executed:

```shell
ls -la crates/biome_js_analyze/tests/specs/nursery/noPlaywrightMissingAwait/
```

Repository: biomejs/biome

Length of output: 283

---

🏁 Script executed:

```shell
cat crates/biome_js_analyze/tests/specs/nursery/noPlaywrightMissingAwait/invalid/*.js
```

Repository: biomejs/biome

Length of output: 2355

---

🏁 Script executed:

```shell
cat crates/biome_js_analyze/tests/specs/nursery/noPlaywrightMissingAwait/valid/*.js
```

Repository: biomejs/biome

Length of output: 2468

---



</details>

**Guard `expect.poll` detection with an `expect` base.**

`has_poll_in_chain` fires for any `.poll` chain, so unrelated code like `obj.poll().method()` can be incorrectly flagged. Adding `has_expect_in_chain` ensures only actual Playwright `expect.poll` calls are caught.

<details>
<summary>Suggested fix</summary>

```diff
-    if has_poll_in_chain(&object) {
+    if has_poll_in_chain(&object) && has_expect_in_chain(&object) {
         return Some(MissingAwaitType::ExpectPoll);
     }
```
</details>

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/lint/nursery/no_playwright_missing_await.rs`
around lines 285 - 302, The has_poll_in_chain check currently flags any .poll
chain; wrap that detection with a guard that ensures the chain actually starts
with expect by also calling has_expect_in_chain(&object) before returning
MissingAwaitType::ExpectPoll—i.e., change the logic around
has_poll_in_chain(&object) so it only returns Some(MissingAwaitType::ExpectPoll)
when both has_poll_in_chain(&object) and has_expect_in_chain(&object) are true,
keeping the existing behavior and enums (MissingAwaitType::ExpectPoll,
has_poll_in_chain, has_expect_in_chain, ExpectMatcher).
```

</details>

<!-- fingerprinting:phantom:medusa:eagle -->

<!-- This is an auto-generated comment by CodeRabbit -->

✅ Addressed in commit d9d3200

---

### Comment 4

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-07T00:24:07Z
- **Comment ID**: 2776516647
- **Line**: 430
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟠 Major_

**Missing function boundary guard in the return-statement walk causes false negatives.**

The loop that searches for `JS_RETURN_STATEMENT` doesn't stop at `JS_FUNCTION_DECLARATION` or `JS_FUNCTION_EXPRESSION` boundaries — only `JS_ARROW_FUNCTION_EXPRESSION` breaks the loop (line 427). This means a call nested inside an inner (non-arrow) function can "see through" to a `return` in an outer function and incorrectly be considered properly handled.

```js
test('x', async () => {
  return function() {
    // This expect is NOT returned, but the walk finds the outer `return`
    expect(page).toBeVisible();
  }
});
```

<details>
<summary>Proposed fix</summary>

```diff
             biome_js_syntax::JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                 // If it's an arrow function with expression body, check if our call is exactly the body
                 if let Some(arrow) = JsArrowFunctionExpression::cast_ref(&node)
                     && let Ok(body) = arrow.body()
                     && let Some(body_expr) = body.as_any_js_expression()
                 {
                     if call_expr.syntax().text_trimmed_range()
                         == body_expr.syntax().text_trimmed_range()
                     {
                         return true;
                     }
                 }
                 break;
             }
+            biome_js_syntax::JsSyntaxKind::JS_FUNCTION_DECLARATION
+            | biome_js_syntax::JsSyntaxKind::JS_FUNCTION_EXPRESSION
+            | biome_js_syntax::JsSyntaxKind::JS_METHOD_CLASS_MEMBER
+            | biome_js_syntax::JsSyntaxKind::JS_METHOD_OBJECT_MEMBER => {
+                break;
+            }
             _ => {}
```
</details>

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/lint/nursery/no_playwright_missing_await.rs`
around lines 406 - 432, The return-statement walk currently only stops at
JS_ARROW_FUNCTION_EXPRESSION, allowing inner non-arrow functions to see outer
returns and causing false negatives; modify the loop matching node.kind() so
that when encountering JS_FUNCTION_DECLARATION or JS_FUNCTION_EXPRESSION it does
NOT keep searching outward (i.e., treat them as function boundaries and break
the loop rather than continuing), while keeping the existing special-case logic
for JsArrowFunctionExpression (which may still return true if the call_expr
exactly matches the arrow body); update the match to include arms for
biome_js_syntax::JsSyntaxKind::JS_FUNCTION_DECLARATION and
::JS_FUNCTION_EXPRESSION that simply break (or otherwise stop the walk) so calls
inside those inner functions cannot match an outer return.
```

</details>

<!-- fingerprinting:phantom:poseidon:phoenix -->

<!-- This is an auto-generated comment by CodeRabbit -->

✅ Addressed in commit a833978

---

### Comment 5

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T00:54:35Z
- **Comment ID**: 2776580147
- **Line**: 430
- **Resolved**: False

**Comment Body:**

done

---

### Comment 6

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-08T17:00:21Z
- **Comment ID**: 2779517019
- **Line**: 239
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

**`toMatchAriaSnapshot` is missing from `ASYNC_PLAYWRIGHT_MATCHERS`.**

`toMatchAriaSnapshot` returns `Promise<void>` and is a web-first assertion added in Playwright v1.49. Without it in the list, `expect(locator).toMatchAriaSnapshot(...)` won't be flagged when the `await` is missing.

<details>
<summary>Proposed fix</summary>

```diff
     "toHaveValues",
+    "toMatchAriaSnapshot",
     "toPass",
 ];
```
</details>

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/lint/nursery/no_playwright_missing_await.rs`
around lines 206 - 239, The ASYNC_PLAYWRIGHT_MATCHERS array is missing
"toMatchAriaSnapshot", so add "toMatchAriaSnapshot" into the
ASYNC_PLAYWRIGHT_MATCHERS constant (keeping the array sorted as noted for binary
search) so calls like expect(locator).toMatchAriaSnapshot(...) are detected as
async; update the ASYNC_PLAYWRIGHT_MATCHERS slice in
no_playwright_missing_await.rs accordingly.
```

</details>

<!-- fingerprinting:phantom:medusa:phoenix -->

<!-- This is an auto-generated comment by CodeRabbit -->

---

## crates/biome_js_analyze/src/lint/nursery/no_playwright_page_pause.rs

**2 comment(s)**

### Comment 1

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T01:15:54Z
- **Comment ID**: 2761717870
- **Line**: 39
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

**Doc example conflicts with the new waitForSelector rule.**  
The “Valid” snippet uses `page.waitForSelector`, which this PR also bans via `noPlaywrightWaitForSelector`. Swapping it to a locator-based pattern keeps the docs aligned.

<details>
<summary>✏️ Suggested tweak</summary>

```diff
 /// ```js
 /// test('example', async ({ page }) => {
 ///     await page.click('button');
-///     await page.waitForSelector('.result');
+///     await expect(page.locator('.result')).toBeVisible();
 /// });
 /// ```
```
</details>

<!-- suggestion_start -->

<details>
<summary>📝 Committable suggestion</summary>

> ‼️ **IMPORTANT**
> Carefully review the code before committing. Ensure that it accurately replaces the highlighted code, contains no missing lines, and has no issues with indentation. Thoroughly test & benchmark the code to ensure it meets the requirements.

```suggestion
/// ### Valid
    ///
    ///
```

</details>

<!-- suggestion_end -->

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/lint/nursery/no_playwright_page_pause.rs` around
lines 33 - 39, Update the documentation example in no_playwright_page_pause.rs
so it no longer uses the banned page.waitForSelector pattern; replace the
waitForSelector line with a locator-based assertion such as using
page.locator('.result') combined with expect(...).toBeVisible() to match the new
noPlaywrightWaitForSelector rule and keep the "Valid" snippet consistent with
the lint rule.
```

</details>

<!-- fingerprinting:phantom:medusa:eagle -->

<!-- This is an auto-generated comment by CodeRabbit -->

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T03:30:08Z
- **Comment ID**: 2762001458
- **Line**: 39
- **Resolved**: False

**Comment Body:**

✅ Fixed in commit e5ff40ae5b - Changed valid example from `waitForSelector` to locator-based: `await expect(page.locator('.result')).toBeVisible()`

---

## crates/biome_js_analyze/src/lint/nursery/no_playwright_skipped_test.rs

**7 comment(s)**

### Comment 1

- **Commenter**: dyc3
- **Created**: 2026-02-04T10:47:08Z
- **Comment ID**: 2763369380
- **Resolved**: False

**Comment Body:**

You should use `inner_string_text()` to do the trimming. It will give you a `TokenText`, which you can use instead of `String` to avoid string allocations.

There are other places where you allocate with `.to_string()`, that can most likely be replaced with `TokenText`.

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T20:06:19Z
- **Comment ID**: 2765784279
- **Resolved**: False

**Comment Body:**

great catch; i refactored through out the PR to ensure no unnecessary allocation.

---

### Comment 3

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-07T00:24:07Z
- **Comment ID**: 2776516649
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

**Missing `describe.fixme(...)` pattern at length 2.**

`describe.skip` is handled here, but `describe.fixme` is not. For consistency (and to match eslint-plugin-playwright), add the missing arm.


<details>
<summary>Proposed fix</summary>

```diff
             // describe.skip(...)
             if names[0] == "describe" && names[1] == "skip" {
                 return Some(SkippedType::Skip);
             }
+            // describe.fixme(...)
+            if names[0] == "describe" && names[1] == "fixme" {
+                return Some(SkippedType::Fixme);
+            }
```
</details>

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/lint/nursery/no_playwright_skipped_test.rs`
around lines 161 - 164, Add handling for the describe.fixme pattern alongside
the existing describe.skip branch: in the same function where you check names[0]
== "describe" && names[1] == "skip" (in no_playwright_skipped_test.rs), add a
parallel check for names[0] == "describe" && names[1] == "fixme" and return the
appropriate SkippedType variant (e.g., SkippedType::Fixme) so describe.fixme is
treated consistently with describe.skip.
```

</details>

<!-- fingerprinting:phantom:poseidon:phoenix -->

<!-- This is an auto-generated comment by CodeRabbit -->

✅ Addressed in commit a833978

---

### Comment 4

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-07T02:34:49Z
- **Comment ID**: 2776766541
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟠 Major_

<details>
<summary>🧩 Analysis chain</summary>

🏁 Script executed:

```shell
#!/bin/bash
# First, find and read the actual file
fd "no_playwright_skipped_test.rs" --type f
```

Repository: biomejs/biome

Length of output: 187

---

🌐 Web query:

`eslint-plugin-playwright test.skip no arguments behavior`

💡 Result:

In Playwright Test, calling `test.skip()` **with no arguments** inside a test body is valid and means **“always skip this test”**—the test is **immediately aborted at that point** (nothing after the call runs). Playwright notes this is less readable than declaring the test as skipped via `test.skip(title, body)`. [1]

In `eslint-plugin-playwright`, the rule **`playwright/no-skipped-test`** treats `test.skip()` (including the no-args form) as **disallowed** and will report it (the rule’s docs list `test.skip()` as an “incorrect” example, alongside conditional skips). [2]

**Sources:**  
[1] Playwright API docs for `test.skip` / `test.skip()` behavior: ([playwright.dev](https://playwright.dev/docs/api/class-test?utm_source=openai))  
[2] `eslint-plugin-playwright` rule docs showing `test.skip()` as a violation: ([npmjs.com](https://www.npmjs.com/package/eslint-plugin-playwright/v/0.9.0?utm_source=openai))

Citations:

- 1: https://playwright.dev/docs/api/class-test?utm_source=openai
- 2: https://www.npmjs.com/package/eslint-plugin-playwright/v/0.9.0?utm_source=openai

---

🏁 Script executed:

```shell
#!/bin/bash
# Read the file to see the full function context around lines 236-266
wc -l crates/biome_js_analyze/src/lint/nursery/no_playwright_skipped_test.rs
```

Repository: biomejs/biome

Length of output: 131

---

🏁 Script executed:

```shell
#!/bin/bash
# Read the relevant section and surrounding context
sed -n '200,280p' crates/biome_js_analyze/src/lint/nursery/no_playwright_skipped_test.rs | cat -n
```

Repository: biomejs/biome

Length of output: 3600

---



</details>

**Bare `test.skip()` without arguments isn't handled—and should be flagged.**

When `arg_count` is 0, the function skips the condition checks and returns `false`, treating it as a non-skip. However, Playwright's `test.skip()` with no arguments is a valid way to skip the test from within its body, and eslint-plugin-playwright's rule flags it as disallowed. The zero-argument case should likely return `true` to flag it as a skipped test.

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/lint/nursery/no_playwright_skipped_test.rs`
around lines 236 - 266, The check in no_playwright_skipped_test.rs incorrectly
treats test.skip() with zero arguments as non-conditional; inside the call
expression handling where you compute arg_count from call_expr.arguments()
(variables: args, arg_count, first_arg, expr), add an explicit branch for
arg_count == 0 that returns true so bare test.skip() is treated as a skipped
test; keep existing logic for arg_count == 1 || arg_count == 2 to inspect the
first argument via AnyJsExpression/AnyJsLiteralExpression/JsTemplateExpression
as before.
```

</details>

<!-- fingerprinting:phantom:poseidon:phoenix -->

<!-- This is an auto-generated comment by CodeRabbit -->

---

### Comment 5

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T05:13:06Z
- **Comment ID**: 2777047359
- **Resolved**: False

**Comment Body:**

addressed

---

### Comment 6

- **Commenter**: dyc3
- **Created**: 2026-02-07T12:26:24Z
- **Comment ID**: 2777487610
- **Line**: 1
- **Resolved**: False

**Comment Body:**

Does https://biomejs.dev/linter/rules/no-skipped-tests already cover this? if it does, we can simply add the metadata to that rule instead.

---

### Comment 7

- **Commenter**: abossenbroek
- **Created**: 2026-02-08T16:46:19Z
- **Comment ID**: 2779502928
- **Line**: 1
- **Resolved**: False

**Comment Body:**

yes; you're right. I consolidated `noPlaywrightSkippedTest` into the existing `noSkippedTests` and updated the PR description accordingly.

---

## crates/biome_js_analyze/src/lint/nursery/no_playwright_useless_await.rs

**3 comment(s)**

### Comment 1

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T05:50:30Z
- **Comment ID**: 2762326768
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🔴 Critical_

<details>
<summary>🧩 Analysis chain</summary>

🏁 Script executed:

```shell
# Find test files for this rule
find crates/biome_js_analyze/tests -name "*playwright*" -o -name "*useless*await*" | head -20
```

Repository: biomejs/biome

Length of output: 444

---

🏁 Script executed:

```shell
# Look for .not patterns in the source file
rg -n "\.not\|not" crates/biome_js_analyze/src/lint/nursery/no_playwright_useless_await.rs
```

Repository: biomejs/biome

Length of output: 39

---

🏁 Script executed:

```shell
# Get the full context of the file around the snippet
wc -l crates/biome_js_analyze/src/lint/nursery/no_playwright_useless_await.rs
```

Repository: biomejs/biome

Length of output: 132

---

🏁 Script executed:

```shell
# Read the helper function that's called (has_async_modifier) and surrounding context
sed -n '260,330p' crates/biome_js_analyze/src/lint/nursery/no_playwright_useless_await.rs
```

Repository: biomejs/biome

Length of output: 2649

---

🏁 Script executed:

```shell
# Find test files for no_playwright_useless_await rule
find crates/biome_js_analyze/tests -type d -name "*noPlaywrightUselessAwait*"
```

Repository: biomejs/biome

Length of output: 125

---

🏁 Script executed:

```shell
# List all test-related files
fd "noPlaywrightUselessAwait\|no_playwright_useless_await" crates/biome_js_analyze/tests/ 2>/dev/null || fd "" crates/biome_js_analyze/tests/specs/nursery/ | grep -i playwright
```

Repository: biomejs/biome

Length of output: 39

---

🏁 Script executed:

```shell
# Read the full is_sync_expect_call function and surrounding code
sed -n '190,270p' crates/biome_js_analyze/src/lint/nursery/no_playwright_useless_await.rs
```

Repository: biomejs/biome

Length of output: 2921

---

🏁 Script executed:

```shell
# Check for any special handling of 'not' in the entire file
cat crates/biome_js_analyze/src/lint/nursery/no_playwright_useless_await.rs | grep -i "\.not\|SYNC_EXPECT_MATCHERS"
```

Repository: biomejs/biome

Length of output: 243

---

🏁 Script executed:

```shell
# Find the SYNC_EXPECT_MATCHERS definition
rg -A 50 "const SYNC_EXPECT_MATCHERS" crates/biome_js_analyze/src/lint/nursery/no_playwright_useless_await.rs | head -100
```

Repository: biomejs/biome

Length of output: 1552

---

🏁 Script executed:

```shell
# Look at the test files
ls -la crates/biome_js_analyze/tests/specs/nursery/noPlaywrightUselessAwait/
```

Repository: biomejs/biome

Length of output: 283

---

🏁 Script executed:

```shell
# Read test snapshots to see what patterns are tested
cat crates/biome_js_analyze/tests/specs/nursery/noPlaywrightUselessAwait/invalid.js 2>/dev/null || cat crates/biome_js_analyze/tests/specs/nursery/noPlaywrightUselessAwait/*.js | head -200
```

Repository: biomejs/biome

Length of output: 164

---

🏁 Script executed:

```shell
# List test files
ls crates/biome_js_analyze/tests/specs/nursery/noPlaywrightUselessAwait/invalid/
```

Repository: biomejs/biome

Length of output: 233

---

🏁 Script executed:

```shell
# Read the invalid test cases
cat crates/biome_js_analyze/tests/specs/nursery/noPlaywrightUselessAwait/invalid/*.js
```

Repository: biomejs/biome

Length of output: 712

---

🏁 Script executed:

```shell
# Read the valid test cases  
cat crates/biome_js_analyze/tests/specs/nursery/noPlaywrightUselessAwait/valid/*.js
```

Repository: biomejs/biome

Length of output: 1088

---

🏁 Script executed:

```shell
# Search for .not patterns in all test files
rg "\.not\." crates/biome_js_analyze/tests/specs/nursery/noPlaywrightUselessAwait/ -A 2 -B 2
```

Repository: biomejs/biome

Length of output: 1524

---

🏁 Script executed:

```shell
# Check if there are any test cases for expect().not.toBe() WITHOUT resolves/rejects
grep -r "expect.*\.not\." crates/biome_js_analyze/tests/specs/nursery/noPlaywrightUselessAwait/ | grep -v "resolves\|rejects"
```

Repository: biomejs/biome

Length of output: 39

---

🏁 Script executed:

```shell
# Double-check the code logic: what happens with expect(x).not.toBe() structure?
# Let's trace through manually by looking at the actual structure
cat crates/biome_js_analyze/src/lint/nursery/no_playwright_useless_await.rs | head -100
```

Repository: biomejs/biome

Length of output: 2748

---

🏁 Script executed:

```shell
# Check if expect().not.toBe() without async modifiers appears in any test files
find crates/biome_js_analyze/tests/specs/nursery/noPlaywrightUselessAwait -name "*.js" -exec grep -l "\.not\." {} \; | xargs grep -h "\.not\." | grep -v "resolves\|rejects"
```

Repository: biomejs/biome

Length of output: 39

---

🏁 Script executed:

```shell
# Verify the AST structure understanding: check if there's any special handling for .not elsewhere
rg "not" crates/biome_js_analyze/src/lint/nursery/no_playwright_useless_await.rs
```

Repository: biomejs/biome

Length of output: 460

---

🏁 Script executed:

```shell
# Check the snapshot files to see what the expected diagnostic output is
cat crates/biome_js_analyze/tests/specs/nursery/noPlaywrightUselessAwait/valid/expect-with-async-modifiers.js.snap
```

Repository: biomejs/biome

Length of output: 886

---

🏁 Script executed:

```shell
# Search for all .js test files to confirm no plain .not.toBe() tests exist
find crates/biome_js_analyze/tests/specs/nursery/noPlaywrightUselessAwait -name "*.js" -type f
```

Repository: biomejs/biome

Length of output: 807

---

🏁 Script executed:

```shell
# Read each test file to search for the specific pattern
for f in crates/biome_js_analyze/tests/specs/nursery/noPlaywrightUselessAwait/invalid/*.js; do echo "=== $f ==="; cat "$f"; done
```

Repository: biomejs/biome

Length of output: 1213

---



</details>

**Missing test coverage and gap: `.not` modifier chains are not detected as useless awaits.**

The code doesn't handle patterns like `await expect(x).not.toBe(1)`. Since `.not` is a synchronous modifier (unlike `.resolves` or `.rejects`), these should be flagged as useless awaits but currently aren't. The function checks if the immediate object of the matcher is a `JsCallExpression`, but for `.not` chains, it's a `JsStaticMemberExpression`, so the check fails silently.

This is confirmed by the test files—there's no coverage for this case, only for `.not` combined with async modifiers (e.g., `expect(promise).resolves.not.toBe(null)`).

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/lint/nursery/no_playwright_useless_await.rs`
around lines 194 - 260, is_sync_expect_call currently only recognizes expect()
directly as the object of the matcher, so chains like await
expect(x).not.toBe(1) are missed; update is_sync_expect_call to also handle when
member_expr.object() is a JsStaticMemberExpression (e.g. the `.not` modifier)
whose object is the JsCallExpression for expect() and whose member token text is
"not" (treat `.not` as a synchronous modifier), then apply the same checks
(including calling has_async_modifier(&expect_call, call_expr)) so `.not` chains
are flagged as useless awaits; reference is_sync_expect_call,
member_expr.object(), JsStaticMemberExpression, and has_async_modifier when
making this change.
```

</details>

<!-- fingerprinting:phantom:medusa:ocelot -->

<!-- This is an auto-generated comment by CodeRabbit -->

✅ Addressed in commit d9d3200

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T22:47:07Z
- **Comment ID**: 2766298246
- **Resolved**: False

**Comment Body:**

addressed.

---

### Comment 3

- **Commenter**: abossenbroek
- **Created**: 2026-02-05T00:16:45Z
- **Comment ID**: 2766502982
- **Resolved**: False

**Comment Body:**

improve in bf7f82b58d

---

## crates/biome_js_analyze/src/lint/nursery/no_playwright_wait_for_selector.rs

**2 comment(s)**

### Comment 1

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T05:21:17Z
- **Comment ID**: 2762252855
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

<details>
<summary>🧩 Analysis chain</summary>

🏁 Script executed:

```shell
# Read the specific file and lines
sed -n '150,180p' crates/biome_js_analyze/src/lint/nursery/no_playwright_wait_for_selector.rs
```

Repository: biomejs/biome

Length of output: 1091

---

🏁 Script executed:

```shell
# Check if replace_node_transfer_trivia exists in the codebase
rg "replace_node_transfer_trivia" --type rs -A 2 -B 2
```

Repository: biomejs/biome

Length of output: 83

---

🏁 Script executed:

```shell
# Also check how replace_node is typically used in similar files
rg "replace_node\(" crates/biome_js_analyze/src/lint/ -t rs -A 1 | head -30
```

Repository: biomejs/biome

Length of output: 83

---

🏁 Script executed:

```shell
# Search for replace_node_transfer_trivia without type restriction
rg "replace_node_transfer_trivia" -A 2 -B 2
```

Repository: biomejs/biome

Length of output: 1998

---

🏁 Script executed:

```shell
# Search for replace_node method definitions to understand the API
rg "fn replace_node" --context 3
```

Repository: biomejs/biome

Length of output: 4490

---

🏁 Script executed:

```shell
# Check the mutation object type and methods in this file
sed -n '1,50p' crates/biome_js_analyze/src/lint/nursery/no_playwright_wait_for_selector.rs
```

Repository: biomejs/biome

Length of output: 1627

---



</details>

**Use `replace_node_transfer_trivia` to preserve comments and formatting.**

`replace_node` discards trivia (comments and whitespace) from the original node. Since this is a code fixer, preserve the original formatting by using `replace_node_transfer_trivia` with error handling.

```diff
-        mutation.replace_node(call_expr.clone(), wait_for_call);
+        mutation.replace_node_transfer_trivia(call_expr.clone(), wait_for_call)?;
```

<!-- suggestion_start -->

<details>
<summary>📝 Committable suggestion</summary>

> ‼️ **IMPORTANT**
> Carefully review the code before committing. Ensure that it accurately replaces the highlighted code, contains no missing lines, and has no issues with indentation. Thoroughly test & benchmark the code to ensure it meets the requirements.

```suggestion
        mutation.replace_node_transfer_trivia(call_expr.clone(), wait_for_call)?;

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace with "<Emphasis>"locator(selector).waitFor()"</Emphasis>"." }.to_owned(),
            mutation,
```

</details>

<!-- suggestion_end -->

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/lint/nursery/no_playwright_wait_for_selector.rs`
around lines 162 - 168, The code uses mutation.replace_node(call_expr.clone(),
wait_for_call) which discards trivia (comments/whitespace); change it to
mutation.replace_node_transfer_trivia(call_expr.clone(), wait_for_call) and
handle the Result/Option it returns (propagate or log/convert to JsRuleAction
error) so the fixer preserves comments and formatting; update the surrounding
block that constructs the JsRuleAction (where JsRuleAction::new is created) to
use the transfer-trivia call and react to any error accordingly.
```

</details>

<!-- fingerprinting:phantom:medusa:eagle -->

<!-- This is an auto-generated comment by CodeRabbit -->

✅ Addressed in commit d9d3200

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T22:51:18Z
- **Comment ID**: 2766309251
- **Resolved**: False

**Comment Body:**

addressed

---

## crates/biome_js_analyze/src/lint/nursery/use_playwright_expect.rs

**5 comment(s)**

### Comment 1

- **Commenter**: dyc3
- **Created**: 2026-02-04T10:40:25Z
- **Comment ID**: 2763342555
- **Resolved**: False

**Comment Body:**

same here, seems like shared functionality. move to `crates/biome_js_analyze/src/frameworks/playwright.rs`

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T19:39:26Z
- **Comment ID**: 2765692582
- **Resolved**: False

**Comment Body:**

good catch; refactored.

---

### Comment 3

- **Commenter**: dyc3
- **Created**: 2026-02-05T23:14:47Z
- **Comment ID**: 2771548438
- **Line**: 97
- **Resolved**: False

**Comment Body:**

swap the order of these notes. generally, a diagnostic should read:

1. what is the error
2. why is it an error
3. how to fix it

---

### Comment 4

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T00:40:31Z
- **Comment ID**: 2776550161
- **Line**: 97
- **Resolved**: False

**Comment Body:**

thanks, fixed in 4cb849cc5c for all rules

---

### Comment 5

- **Commenter**: dyc3
- **Created**: 2026-02-08T19:48:18Z
- **Comment ID**: 2779754966
- **Line**: 12
- **Resolved**: False

**Comment Body:**

jest and vitest almost certainly have an equivalent rule for this too.

---

## crates/biome_js_analyze/src/lint/nursery/use_playwright_valid_describe_callback.rs

**6 comment(s)**

### Comment 1

- **Commenter**: dyc3
- **Created**: 2026-02-05T23:09:13Z
- **Comment ID**: 2771536051
- **Resolved**: False

**Comment Body:**

nit: these functions should take `&str` instead of `&TokenText`

---

### Comment 2

- **Commenter**: dyc3
- **Created**: 2026-02-05T23:12:17Z
- **Comment ID**: 2771543049
- **Resolved**: False

**Comment Body:**

nit: all helper functions/structs/enums in all the rules must go below the `impl Rule` block. the exception is if you declare a node union to use in the rule's query, then it makes sense to keep it above the rule block because its easier to read.

@coderabbitai can you add this feedback to your learnings?

---

### Comment 3

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-05T23:24:20Z
- **Comment ID**: 2771568957
- **Resolved**: False

**Comment Body:**

`@dyc3`, absolutely! I've added this style guideline to my learnings for future reviews.

---

<details>
<summary>✏️ Learnings added</summary>

```
Learnt from: dyc3
Repo: biomejs/biome PR: 8960
File: crates/biome_js_analyze/src/lint/nursery/use_playwright_valid_describe_callback.rs:74-74
Timestamp: 2026-02-05T23:24:11.386Z
Learning: In crates/biome_analyze/**/*.rs rule files, all helper functions, structs, and enums must be placed below the `impl Rule` block. The only exception is when declaring a node union to use in the rule's Query type, which can be kept above the rule block for better readability.
```

> Note: Learnings are effective only in the context of similar code segments. To apply general coding guidelines, please configure [review instructions](https://docs.coderabbit.ai/guides/review-instructions). You can manage existing learnings in the [app](https://app.coderabbit.ai/learnings).

</details>



---

<details>
<summary>🧠 Learnings used</summary>

```
Learnt from: CR
Repo: biomejs/biome PR: 0
File: crates/biome_analyze/CONTRIBUTING.md:0-0
Timestamp: 2026-01-17T19:45:06.423Z
Learning: Applies to crates/biome_analyze/tests/specs/**/* : In rule documentation code blocks, use `expect_diagnostic` property for invalid snippets, `options` or `full_options` for configuration examples, `use_options` for snippets using specified options, and `ignore` to exclude snippets from automatic validation
```

---

```
Learnt from: CR
Repo: biomejs/biome PR: 0
File: crates/biome_analyze/CONTRIBUTING.md:0-0
Timestamp: 2026-01-17T19:45:06.423Z
Learning: Applies to crates/biome_analyze/**/*.rs : Implement the `diagnostic` function to convert signals into RuleDiagnostic instances with informative messages. Always follow the three pillars: explain WHAT the error is, explain WHY it's triggered, and tell the user WHAT to do
```

---

```
Learnt from: CR
Repo: biomejs/biome PR: 0
File: crates/biome_analyze/CONTRIBUTING.md:0-0
Timestamp: 2026-01-17T19:45:06.423Z
Learning: Applies to crates/biome_analyze/**/*.rs : Lint rules must follow the naming convention: use `no<Concept>` prefix when forbidding a single concept (e.g., `noDebugger`), and `use<Concept>` prefix when mandating a single concept (e.g., `useValidLang`)
```

---

```
Learnt from: CR
Repo: biomejs/biome PR: 0
File: crates/biome_analyze/CONTRIBUTING.md:0-0
Timestamp: 2026-01-17T19:45:06.423Z
Learning: Applies to crates/biome_analyze/**/*.rs : Use `declare_lint_rule!` macro to declare analyzer rules with required metadata: `version` (set to 'next'), `name` (the rule identifier), `language` (e.g., 'js', 'jsx', 'ts', 'tsx'), and optional `recommended`, `severity`, `fix_kind`, `sources`, `domains`, `deprecated`, and `issue_number` fields
```

---

```
Learnt from: CR
Repo: biomejs/biome PR: 0
File: crates/biome_analyze/CONTRIBUTING.md:0-0
Timestamp: 2026-01-17T19:45:06.423Z
Learning: Applies to crates/biome_analyze/**/*.rs : For rules derived from other linters, use the `sources` metadata with `RuleSource::Eslint` and either `.same()` for identical behavior or `.inspired()` for different behavior/options
```

---

```
Learnt from: ematipico
Repo: biomejs/biome PR: 8537
File: crates/biome_js_analyze/src/lint/nursery/no_leaked_render.rs:167-210
Timestamp: 2025-12-22T09:27:13.161Z
Learning: In crates/biome_analyze/**/*analyze/src/**/*.rs, the `fix_kind` field in `declare_lint_rule!` should only be specified when the rule implements the `action` function. Rules that only emit diagnostics without providing code fixes should not include `fix_kind` in their metadata.
```

---

```
Learnt from: CR
Repo: biomejs/biome PR: 0
File: crates/biome_analyze/CONTRIBUTING.md:0-0
Timestamp: 2026-01-17T19:45:06.423Z
Learning: Applies to crates/biome_analyze/**/*.rs : Use `Semantic<T>` query type when rules require semantic analysis (bindings, references, scopes). Use `Ast<T>` for syntax-only analysis. For multiple node types, use `declare_node_union!` macro
```

---

```
Learnt from: CR
Repo: biomejs/biome PR: 0
File: crates/biome_analyze/CONTRIBUTING.md:0-0
Timestamp: 2026-01-17T19:45:06.423Z
Learning: Applies to crates/biome_analyze/**/*.rs : Implement the `Rule` trait with required associated types: `Query` (defines entities to analyze), `State` (holds signal state), `Signals` (return type from `run`, typically `Option<Self::State>` or `Vec<Self::State>`), and `Options` (for rule configuration)
```

---

```
Learnt from: CR
Repo: biomejs/biome PR: 0
File: crates/biome_parser/CONTRIBUTING.md:0-0
Timestamp: 2025-11-24T18:06:03.545Z
Learning: Applies to crates/biome_parser/**/src/**/*.rs : Parse rule functions must be prefixed with `parse_` and use the name defined in the grammar file, e.g., `parse_for_statement` or `parse_expression`
```

---

```
Learnt from: CR
Repo: biomejs/biome PR: 0
File: crates/biome_analyze/CONTRIBUTING.md:0-0
Timestamp: 2026-01-17T19:45:06.423Z
Learning: Applies to crates/biome_analyze/**/*.rs : New rules must be placed in the `nursery` group as an incubation space exempt from semantic versioning, and can be promoted to appropriate groups in minor/major releases
```

---

```
Learnt from: CR
Repo: biomejs/biome PR: 0
File: crates/biome_parser/CONTRIBUTING.md:0-0
Timestamp: 2025-11-24T18:06:03.545Z
Learning: Applies to crates/biome_parser/**/src/**/*.rs : Use `p.eat(token)` for optional tokens, `p.expect(token)` for required tokens, `parse_rule(p).ok(p)` for optional nodes, and `parse_rule(p).or_add_diagnostic(p, error)` for required nodes
```

---

```
Learnt from: CR
Repo: biomejs/biome PR: 0
File: crates/biome_formatter/CONTRIBUTING.md:0-0
Timestamp: 2026-02-05T16:31:32.208Z
Learning: Applies to crates/biome_formatter/**/cst.rs : In `cst.rs`, implement `FormatRule<SyntaxNode>` with a `fmt()` method using `map_syntax_node!` macro and provide `AsFormat` and `IntoFormat` trait implementations
```

---

```
Learnt from: dyc3
Repo: biomejs/biome PR: 8639
File: crates/biome_js_analyze/src/lint/nursery/no_excessive_lines_per_file.rs:101-108
Timestamp: 2025-12-31T15:35:41.261Z
Learning: In crates/biome_analyze/**/*analyze/src/lint/nursery/**/*.rs, the `issue_number` field in `declare_lint_rule!` macro is optional and the vast majority of nursery rules do not need it. Do not recommend adding `issue_number` unless there's a specific reason.
```

---

```
Learnt from: CR
Repo: biomejs/biome PR: 0
File: CONTRIBUTING.md:0-0
Timestamp: 2025-12-21T21:15:03.796Z
Learning: Applies to **/*.rs : Use inline rustdoc documentation for rules, assists, and their options
```

---

```
Learnt from: CR
Repo: biomejs/biome PR: 0
File: crates/biome_parser/CONTRIBUTING.md:0-0
Timestamp: 2025-11-24T18:06:03.545Z
Learning: Applies to crates/biome_parser/**/src/**/*.rs : Parse rules must return `ParsedSyntax::Absent` if the rule can't predict by the next token(s) if they form the expected node, and must not progress the parser in this case
```

---

```
Learnt from: CR
Repo: biomejs/biome PR: 0
File: crates/biome_analyze/CONTRIBUTING.md:0-0
Timestamp: 2026-01-17T19:45:06.423Z
Learning: Applies to crates/biome_analyze/**/*.rs : Choose rule severity based on error type: `error` for hard errors/dangerous/accessibility issues, `warn` for possibly erroneous code, `info` for stylistic suggestions. Correctness/security/a11y rules must be `error`, style rules must be `info` or `warn`, complexity rules must be `warn` or `info`
```

---

```
Learnt from: CR
Repo: biomejs/biome PR: 0
File: crates/biome_analyze/CONTRIBUTING.md:0-0
Timestamp: 2026-01-17T19:45:06.423Z
Learning: Applies to crates/biome_analyze/**/*.rs : Mark rules as work-in-progress by adding `issue_number` field to the `declare_lint_rule!` macro, which adds a footnote to diagnostics and a note on the website with a link to the GitHub issue
```

---

```
Learnt from: CR
Repo: biomejs/biome PR: 0
File: CONTRIBUTING.md:0-0
Timestamp: 2025-12-21T21:15:03.796Z
Learning: For new lint rules in changesets, show an example of invalid case in inline code or code block
```

---

```
Learnt from: CR
Repo: biomejs/biome PR: 0
File: CONTRIBUTING.md:0-0
Timestamp: 2025-12-21T21:15:03.796Z
Learning: For rule changes in changesets, clearly demonstrate what is now invalid that wasn't before, or vice versa
```

</details>



<!-- This is an auto-generated reply by CodeRabbit -->

---

### Comment 4

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-07T00:24:07Z
- **Comment ID**: 2776516651
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

<details>
<summary>🧩 Analysis chain</summary>

🌐 Web query:

`Playwright test.describe.fixme API documentation`

💡 Result:

`test.describe.fixme` is documented in Playwright Test’s **`Test` class API** page under **“test.describe.fixme”**. It declares a `describe` group whose tests are marked **fixme** and **will not be executed**. Supported overloads and args are listed there (including the optional `details` object added in v1.42). [1]

If you also want the broader “fixme” annotation behavior (vs `skip` / `fail`) and when to use it, see **Test annotations**. [2]

**Sources:**  
[1] Playwright Test API — `test.describe.fixme` section https://playwright.dev/docs/api/class-test  
[2] Playwright Test annotations (skip/fail/fixme/slow) https://playwright.dev/docs/test-annotations

---

🏁 Script executed:

```shell
# First, let's check the actual file and understand the context
cat -n crates/biome_js_analyze/src/lint/nursery/use_playwright_valid_describe_callback.rs | head -250 | tail -80
```

Repository: biomejs/biome

Length of output: 3551

---

🏁 Script executed:

```shell
# Also check the eslint-plugin-playwright rule to see what it expects
fd "eslint-plugin-playwright" . --type f
```

Repository: biomejs/biome

Length of output: 39

---



</details>

**Add `fixme` to `is_describe_modifier`.**

Playwright's Test API supports `test.describe.fixme()` for marking describe blocks as fixme (per https://playwright.dev/docs/api/class-test). Currently, the function only recognises `only` and `skip`, so `test.describe.fixme()` won't be validated.

<details>
<summary>Fix</summary>

```diff
 fn is_describe_modifier(s: &str) -> bool {
-    s == "only" || s == "skip"
+    s == "only" || s == "skip" || s == "fixme"
 }
```
</details>

<!-- suggestion_start -->

<details>
<summary>📝 Committable suggestion</summary>

> ‼️ **IMPORTANT**
> Carefully review the code before committing. Ensure that it accurately replaces the highlighted code, contains no missing lines, and has no issues with indentation. Thoroughly test & benchmark the code to ensure it meets the requirements.

```suggestion
fn is_describe_modifier(s: &str) -> bool {
    s == "only" || s == "skip" || s == "fixme"
}
```

</details>

<!-- suggestion_end -->

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In
`@crates/biome_js_analyze/src/lint/nursery/use_playwright_valid_describe_callback.rs`
around lines 211 - 213, The helper function is_describe_modifier currently only
recognizes "only" and "skip", so add "fixme" to its checks; update the
is_describe_modifier function (the one returning s == "only" || s == "skip") to
also return true for "fixme" so that calls like test.describe.fixme() are
treated as a describe modifier and validated accordingly.
```

</details>

<!-- fingerprinting:phantom:poseidon:phoenix -->

<!-- This is an auto-generated comment by CodeRabbit -->

✅ Addressed in commit a833978

---

### Comment 5

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T00:27:14Z
- **Comment ID**: 2776520808
- **Resolved**: False

**Comment Body:**

fixed.

---

### Comment 6

- **Commenter**: abossenbroek
- **Created**: 2026-02-07T00:35:32Z
- **Comment ID**: 2776538418
- **Resolved**: False

**Comment Body:**

Thanks for teaching me something I didn't know about rust.

---

## crates/biome_js_analyze/src/lint/suspicious/no_skipped_tests.rs

**1 comment(s)**

### Comment 1

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-08T17:00:21Z
- **Comment ID**: 2779517021
- **Line**: 199
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

**`#[allow(dead_code)]` on `annotation` is misleading — the field is read in `action()`.**

Line 181 reads `state.annotation` when handling computed member expressions, so the field is not dead code. The attribute and the comment ("Reserved for future diagnostic differentiation") are both stale.

<details>
<summary>Suggested fix</summary>

```diff
 pub struct SkipState {
     range: TextRange,
-    /// The type of annotation: "skip" or "fixme". Reserved for future diagnostic differentiation.
-    #[allow(dead_code)]
+    /// The type of annotation: "skip" or "fixme".
     annotation: &'static str,
 }
```
</details>

<!-- suggestion_start -->

<details>
<summary>📝 Committable suggestion</summary>

> ‼️ **IMPORTANT**
> Carefully review the code before committing. Ensure that it accurately replaces the highlighted code, contains no missing lines, and has no issues with indentation. Thoroughly test & benchmark the code to ensure it meets the requirements.

```suggestion
#[derive(Debug)]
pub struct SkipState {
    range: TextRange,
    /// The type of annotation: "skip" or "fixme".
    annotation: &'static str,
}
```

</details>

<!-- suggestion_end -->

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_js_analyze/src/lint/suspicious/no_skipped_tests.rs` around lines
193 - 199, The #[allow(dead_code)] and the "Reserved for future diagnostic
differentiation" comment on the annotation field in SkipState are stale because
SkipState::annotation is actually read in action(); remove the
#[allow(dead_code)] attribute and update the field comment to accurately
describe its current purpose (e.g., "Type of annotation ('skip' or 'fixme') used
by action() for diagnostic handling") so the code and docs reflect that the
field is used.
```

</details>

<!-- fingerprinting:phantom:medusa:phoenix -->

<!-- This is an auto-generated comment by CodeRabbit -->

---

## crates/biome_rule_options/src/no_playwright_element_handle.rs

**2 comment(s)**

### Comment 1

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T01:15:54Z
- **Comment ID**: 2761717872
- **Line**: 6
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

**Add a brief rustdoc comment for the options type.**  
Right now the options struct is undocumented.

<details>
<summary>💡 Suggested tweak</summary>

```diff
 #[serde(rename_all = "camelCase", deny_unknown_fields, default)]
+/// Options for the `noPlaywrightElementHandle` rule.
 pub struct NoPlaywrightElementHandleOptions {}
```
</details>

  
As per coding guidelines: Use inline rustdoc documentation for rules, assists, and their options.

<!-- suggestion_start -->

<details>
<summary>📝 Committable suggestion</summary>

> ‼️ **IMPORTANT**
> Carefully review the code before committing. Ensure that it accurately replaces the highlighted code, contains no missing lines, and has no issues with indentation. Thoroughly test & benchmark the code to ensure it meets the requirements.

```suggestion
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
/// Options for the `noPlaywrightElementHandle` rule.
pub struct NoPlaywrightElementHandleOptions {}
```

</details>

<!-- suggestion_end -->

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_rule_options/src/no_playwright_element_handle.rs` around lines 1
- 6, Add an inline rustdoc comment for the NoPlaywrightElementHandleOptions
struct: insert a concise /// doc comment directly above the pub struct
NoPlaywrightElementHandleOptions describing what the options configure (e.g.,
options for the NoPlaywrightElementHandle rule), any default behavior, and note
that the struct is currently empty if applicable; mention usage context
(rule/assist) and feature flags only if relevant so documentation follows the
project's guideline for rules, assists, and options.
```

</details>

<!-- fingerprinting:phantom:poseidon:eagle -->

<!-- This is an auto-generated comment by CodeRabbit -->

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T03:30:43Z
- **Comment ID**: 2762002486
- **Line**: 6
- **Resolved**: False

**Comment Body:**

⏭️ Skipped - After investigation, we found that 381 existing empty options structs in `biome_rule_options` don't have rustdoc comments. This is the established Biome pattern for empty options structs that exist for future extensibility. Adding docs like "Options for the X rule" would just restate the struct name without adding value. We'll follow the existing codebase convention.

---

## crates/biome_rule_options/src/no_playwright_missing_await.rs

**2 comment(s)**

### Comment 1

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T01:15:54Z
- **Comment ID**: 2761717876
- **Line**: 6
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

**Add rustdoc + a tiny doctest for this options type.**

<details>
<summary>Suggested diff</summary>

```diff
 use biome_deserialize_macros::{Deserializable, Merge};
 use serde::{Deserialize, Serialize};
+/// Options for the `noPlaywrightMissingAwait` rule.
+///
+/// ## Examples
+/// ```
+/// use biome_rule_options::no_playwright_missing_await::NoPlaywrightMissingAwaitOptions;
+/// let options = NoPlaywrightMissingAwaitOptions::default();
+/// assert_eq!(options, NoPlaywrightMissingAwaitOptions {});
+/// ```
 #[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
 #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
 #[serde(rename_all = "camelCase", deny_unknown_fields, default)]
 pub struct NoPlaywrightMissingAwaitOptions {}
```
</details>
As per coding guidelines: Use inline rustdoc documentation for rules, assists, and their options; Use doc tests (doctest) format with code blocks in rustdoc comments; ensure assertions pass in tests.

<details>
<summary>🤖 Prompt for AI Agents</summary>

````
In `@crates/biome_rule_options/src/no_playwright_missing_await.rs` around lines 1
- 6, Add an inline rustdoc comment above the NoPlaywrightMissingAwaitOptions
struct that includes a fenced rust code block doctest showing usage and
asserting the default equals an empty struct (e.g., create options via
NoPlaywrightMissingAwaitOptions::default() and assert_eq!(options,
NoPlaywrightMissingAwaitOptions {})); ensure the docblock follows
rustdoc/doctest format (/// ```rust ... /// ```), references the type name
NoPlaywrightMissingAwaitOptions, and leaves the existing derives and serde
attributes unchanged so the doctest runs in CI.
````

</details>

<!-- fingerprinting:phantom:poseidon:eagle -->

<!-- This is an auto-generated comment by CodeRabbit -->

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T03:31:17Z
- **Comment ID**: 2762003597
- **Line**: 6
- **Resolved**: False

**Comment Body:**

⏭️ Skipped - Same reason as above: 381 existing empty options structs in Biome don't have rustdoc. Following established codebase convention.

---

## crates/biome_rule_options/src/no_playwright_page_pause.rs

**2 comment(s)**

### Comment 1

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T01:15:54Z
- **Comment ID**: 2761717881
- **Line**: 6
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

**Add rustdoc + a tiny doctest for this options type.**

<details>
<summary>Suggested diff</summary>

```diff
 use biome_deserialize_macros::{Deserializable, Merge};
 use serde::{Deserialize, Serialize};
+/// Options for the `noPlaywrightPagePause` rule.
+///
+/// ## Examples
+/// ```
+/// use biome_rule_options::no_playwright_page_pause::NoPlaywrightPagePauseOptions;
+/// let options = NoPlaywrightPagePauseOptions::default();
+/// assert_eq!(options, NoPlaywrightPagePauseOptions {});
+/// ```
 #[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
 #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
 #[serde(rename_all = "camelCase", deny_unknown_fields, default)]
 pub struct NoPlaywrightPagePauseOptions {}
```
</details>
As per coding guidelines: Use inline rustdoc documentation for rules, assists, and their options; Use doc tests (doctest) format with code blocks in rustdoc comments; ensure assertions pass in tests.

<!-- suggestion_start -->

<details>
<summary>📝 Committable suggestion</summary>

> ‼️ **IMPORTANT**
> Carefully review the code before committing. Ensure that it accurately replaces the highlighted code, contains no missing lines, and has no issues with indentation. Thoroughly test & benchmark the code to ensure it meets the requirements.

```suggestion
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
/// Options for the `noPlaywrightPagePause` rule.
///
/// ## Examples
///
```

</details>

<!-- suggestion_end -->

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_rule_options/src/no_playwright_page_pause.rs` around lines 1 -
6, Add a rustdoc comment with a small doctest for the
NoPlaywrightPagePauseOptions type: above the struct NoPlaywrightPagePauseOptions
add an /// doc block that explains the options and includes a fenced Rust code
example that constructs NoPlaywrightPagePauseOptions::default() and asserts
equality with NoPlaywrightPagePauseOptions {} (using the fully qualified path
biome_rule_options::no_playwright_page_pause::NoPlaywrightPagePauseOptions or
relative path), so the doc test compiles and verifies the Default/Eq behavior;
keep the serde and derive attributes unchanged.
```

</details>

<!-- fingerprinting:phantom:poseidon:eagle -->

<!-- This is an auto-generated comment by CodeRabbit -->

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T03:31:20Z
- **Comment ID**: 2762003664
- **Line**: 6
- **Resolved**: False

**Comment Body:**

⏭️ Skipped - Same reason as above: 381 existing empty options structs in Biome don't have rustdoc. Following established codebase convention.

---

## crates/biome_rule_options/src/no_playwright_wait_for_selector.rs

**2 comment(s)**

### Comment 1

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T01:15:55Z
- **Comment ID**: 2761717886
- **Line**: 6
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

**Add rustdoc + a tiny doctest for this options type.**  
Keeps the public options surface documented and testable.

<details>
<summary>Suggested diff</summary>

```diff
 use biome_deserialize_macros::{Deserializable, Merge};
 use serde::{Deserialize, Serialize};
+/// Options for the `noPlaywrightWaitForSelector` rule.
+///
+/// ## Examples
+/// ```
+/// use biome_rule_options::no_playwright_wait_for_selector::NoPlaywrightWaitForSelectorOptions;
+/// let options = NoPlaywrightWaitForSelectorOptions::default();
+/// assert_eq!(options, NoPlaywrightWaitForSelectorOptions {});
+/// ```
 #[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
 #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
 #[serde(rename_all = "camelCase", deny_unknown_fields, default)]
 pub struct NoPlaywrightWaitForSelectorOptions {}
```
</details>
As per coding guidelines: Use inline rustdoc documentation for rules, assists, and their options; Use doc tests (doctest) format with code blocks in rustdoc comments; ensure assertions pass in tests.

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_rule_options/src/no_playwright_wait_for_selector.rs` around
lines 1 - 6, Add an inline rustdoc comment above the
NoPlaywrightWaitForSelectorOptions struct that includes a fenced doctest code
block showing how to construct the options and asserting equality (e.g., use
biome_rule_options::no_playwright_wait_for_selector::NoPlaywrightWaitForSelectorOptions;
let options = NoPlaywrightWaitForSelectorOptions::default(); assert_eq!(options,
NoPlaywrightWaitForSelectorOptions {})); ensure the comment uses proper rustdoc
triple-slash format and the code block is marked for doctests so it runs during
cargo test; place the doc immediately above the pub struct
NoPlaywrightWaitForSelectorOptions declaration.
```

</details>

<!-- fingerprinting:phantom:poseidon:eagle -->

<!-- This is an auto-generated comment by CodeRabbit -->

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T03:31:22Z
- **Comment ID**: 2762003766
- **Line**: 6
- **Resolved**: False

**Comment Body:**

⏭️ Skipped - Same reason as above: 381 existing empty options structs in Biome don't have rustdoc. Following established codebase convention.

---

## crates/biome_rule_options/src/no_playwright_wait_for_timeout.rs

**2 comment(s)**

### Comment 1

- **Commenter**: coderabbitai[bot]
- **Created**: 2026-02-04T01:15:55Z
- **Comment ID**: 2761717890
- **Line**: 6
- **Resolved**: False

**Comment Body:**

_⚠️ Potential issue_ | _🟡 Minor_

**Add rustdoc + a tiny doctest for this options type.**

<details>
<summary>Suggested diff</summary>

```diff
 use biome_deserialize_macros::{Deserializable, Merge};
 use serde::{Deserialize, Serialize};
+/// Options for the `noPlaywrightWaitForTimeout` rule.
+///
+/// ## Examples
+/// ```
+/// use biome_rule_options::no_playwright_wait_for_timeout::NoPlaywrightWaitForTimeoutOptions;
+/// let options = NoPlaywrightWaitForTimeoutOptions::default();
+/// assert_eq!(options, NoPlaywrightWaitForTimeoutOptions {});
+/// ```
 #[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
 #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
 #[serde(rename_all = "camelCase", deny_unknown_fields, default)]
 pub struct NoPlaywrightWaitForTimeoutOptions {}
```
</details>
As per coding guidelines: Use inline rustdoc documentation for rules, assists, and their options; Use doc tests (doctest) format with code blocks in rustdoc comments; ensure assertions pass in tests.

<!-- suggestion_start -->

<details>
<summary>📝 Committable suggestion</summary>

> ‼️ **IMPORTANT**
> Carefully review the code before committing. Ensure that it accurately replaces the highlighted code, contains no missing lines, and has no issues with indentation. Thoroughly test & benchmark the code to ensure it meets the requirements.

```suggestion
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
/// Options for the `noPlaywrightWaitForTimeout` rule.
///
/// ## Examples
///
```

</details>

<!-- suggestion_end -->

<details>
<summary>🤖 Prompt for AI Agents</summary>

```
In `@crates/biome_rule_options/src/no_playwright_wait_for_timeout.rs` around lines
1 - 6, Add an inline rustdoc comment above the NoPlaywrightWaitForTimeoutOptions
struct that includes a doctest demonstrating default construction and equality:
include a code block that imports
biome_rule_options::no_playwright_wait_for_timeout::NoPlaywrightWaitForTimeoutOptions,
creates let options = NoPlaywrightWaitForTimeoutOptions::default(); and asserts
assert_eq!(options, NoPlaywrightWaitForTimeoutOptions {}); so the doc test
compiles and validates the Default/Eq impls for
NoPlaywrightWaitForTimeoutOptions.
```

</details>

<!-- fingerprinting:phantom:poseidon:eagle -->

<!-- This is an auto-generated comment by CodeRabbit -->

---

### Comment 2

- **Commenter**: abossenbroek
- **Created**: 2026-02-04T03:31:25Z
- **Comment ID**: 2762003844
- **Line**: 6
- **Resolved**: False

**Comment Body:**

⏭️ Skipped - Same reason as above: 381 existing empty options structs in Biome don't have rustdoc. Following established codebase convention.

---

## crates/biome_rule_options/src/no_skipped_tests.rs

**1 comment(s)**

### Comment 1

- **Commenter**: dyc3
- **Created**: 2026-02-08T18:35:26Z
- **Comment ID**: 2779643181
- **Line**: 10
- **Resolved**: False

**Comment Body:**

We can't add options to a rule in a patch release. This will need to go in a different PR on `next`.

---

