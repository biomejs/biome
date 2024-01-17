# Analyzer

The analyzer is a generic crate aimed to implement a visitor-like infrastructure, where
it's possible to inspect a piece of AST and emit diagnostics or actions based on a
static check.

## Folder structure

First, you need to identify the crate where you want to implement the rule.
If the rule is going to be implemented for the JavaScript language (and its super languages),
then the rule will be implemented inside the `biome_js_analyze` crate.

Rules are divided by capabilities:

- `analyzers/` folder contains rules that don't require any particular capabilities, via the `Ast<>` query type;

- `semantic_analyzer/` folder contains rules that require the use of the semantic model, via `Semantic<>` query type;

- `aria_analyzers/` folder contains rules that require the use of ARIA metadata, via `Aria<>` query type;

- `assists/` folder contains rules that contribute to refactor code, with not associated diagnostics.
  These are rules that are usually meant for editors and IDEs.

Most of the rules will go under `analyzers/` or `semantic_analyzer/`.

Inside these four folders, we have a folder for each group that _Biome_ supports.

When implementing **new rules**, they have to be implemented under the group `nursery`.
New rules should always be considered unstable/not exhaustive.

In addition to selecting a group, rules may be flagged as `recommended`.
The **recommended rules** are enabled in the default configuration of the _Biome_ linter.
As a general principle, a recommended rule should catch actual programming errors.
For instance, detecting a coding pattern that will throw an exception at runtime.
Pedantic rules that check for specific unwanted patterns but may have high false positive rates,
should be left off from the recommended set.
Rules intended to be recommended should be flagged as such even if they are still part of the `nursery` group,
as unstable rules are only enabled by default on unstable builds.
This gives the project time to test the rule, find edge cases, etc.

## Lint rules

When creating or updating a lint rule, you need to be aware that there's a lot of generated code inside our toolchain.
Our CI ensures that this code is not out of sync and fails otherwise.
See the [code generation section](#code-generation) for more details.

To create a new rule, you have to create and update several files.
Because it is a bit tedious, _Biome_ provides an easy way to create and test your rule using [Just](https://just.systems/man/en/).
_Just_ is not part of the rust toolchain, you have to install it with [a package manager](https://just.systems/man/en/chapter_4.html).

### Choose a name

_Biome_ follows a naming convention according to what the rule do:

1. Forbid a concept

   ```block
   no<Concept>
   ```

   When a rule's sole intention is to **forbid a single concept** - such as disallowing the use of `debugger` statements - the rule should be named using the `no` prefix.
   For example, the rule to disallow the use of `debugger` statements is named `noDebugger`.

1. Mandate a concept

   ```block
   use<Concept>
   ```

   When a rule's sole intention is to **mandate a single concept** - such as forcing the use of camel-casing - the rule should be named using the `use` prefix.
   For example, the rule to mandating the use of camel-cased variable names is named `useCamelCase`.

### What a rule should say to the user

A rule should be informative to the user, and give as much explanation as possible.

When writing a rule, you must adhere to the following **pillars**:
1. Explain to the user the error. Generally, this is the message of the diagnostic.
2. Explain to the user **why** the error is triggered. Generally, this is implemented with an additional node.
3. Tell the user what they should do. Generally, this is implemented using a code action. If a code action is not applicable a note should tell the user what they should do to fix the error.

### Create and implement the rule

Let's say we want to create a new rule called `myRuleName`, which uses the semantic model.

1. Run the command

   ```shell
   just new-lintrule crates/biome_js_analyze/src/semantic_analyzers/nursery myRuleName
   ```

   Rules go in different folders, and the folder depend on the type of query system your rule
   will use:

   - `type Query = Ast<>` -> `analyzers/` folder
   - `type Query = Semantic<>` -> `semantic_analyzers/` folder
   - `type Query = SemanticServices` -> `semantic_analyzers/` folder
   - `type Query = Aria<>` -> `aria_analyzers` folder
   - `type Query = ControlFlowGraph` -> `analyzers/` folder

   The core team will help you out if you don't get the folder right.
   Using the incorrect folder won't break any code.

2. The `Query` needs to have the `Semantic` type, because we want to have access to the semantic model.
   `Query` tells the engine on which AST node we want to trigger the rule.

3. The `State` type doesn't have to be used, so it can be considered optional.
   However, it has to be defined as `type State = ()`.

4. Implement the `run` function:

   This function is called every time the analyzer finds a match for the query specified by the rule,
   and may return zero or more "signals".

5. Implement the `diagnostic` function, to tell the user where's the error and why:

   ```rust,ignore
   impl Rule for UseAwesomeTricks {
       // .. code
       fn diagnostic(_ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {}
   }
   ```

   While implementing the diagnostic, please keep [Biome's technical principals](https://biomejs.dev/internals/philosophy/#technical) in mind.
   This function is called for every signal emitted by the `run` function, and it may return
   zero or one diagnostic.

6. Implement the optional `action` function, if we are able to provide automatic code fix to the rule:

   ```rust,ignore
   impl Rule for UseAwesomeTricks {
       // .. code
       fn action(_ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {}
   }
   ```

   This function is called for every signal emitted by the `run` function.
   It may return zero or one code action.
   Rules can return a code action that can be **safe** or **unsafe**. If a rule returns a code action, you must add `fix_kind` to the macro `declare_rule`.
   ```rust,ignore
   use biome_analyze::FixKind;
   declare_rule!{
     fix_kind: FixKind::Safe,
   }
   ```
   When returning a code action, you must pass the `category` and the `applicability` fields.
   `category` must be `ActionCategory::QuickFix`.
   `applicability` is either `Applicability:MaybeIncorrect` or `Applicability:Always`.
   `Applicability:Always` must only be used when the code transformation is safe.
   In other words, the code transformation should always result in code that does no change the behavior of the code.
   In the case of `noVar`, it is not always safe to turn `var` to `const` or `let`.

Don't forget to format your code with `cargo format` and lint with `cargo lint`.

That's it! Now, let's test the rule.

### Test the rule

Inside the `tests/specs/` folder, rules are divided by group and rule name.
The test infrastructure is rigid around the association of the pair "group/rule name", which means that
_**your test cases are placed inside the wrong group, you won't see any diagnostics**_.

Since each new rule will start from `nursery`, that's where we start.
If you used `just new-lintrule`, a folder that use the name of the rule should exist.
Otherwise, create a folder called `myRuleName/`, and then create one or more files where you want to create different cases.

A common pattern is to create files prefixed by `invalid` or `valid`.
The files prefixed by `invalid` contain code that are reported by the rule.
The files prefixed by `valid` contain code that are not reported by the rule.

Files ending with the extension `.jsonc` are differently handled.
These files should contain an array of strings where each string is a code snippet.
For instance, for the rule `noVar`, the file `invalidScript.jsonc` contains:

```jsonc
["var x = 1; foo(x);", "for (var x of [1,2,3]) { foo(x); }"]
```

Note that code in a file ending with the extension `.jsonc` are in a _script environment_.
This means that you cannot use syntax that belongs to _ECMAScript modules_ such as `import` and `export`.

Run the command

```shell
just test-lintrule myRuleName
```

and if you've done everything correctly,
you should see some snapshots emitted with diagnostics and code actions.

Check our main [contribution document](https://github.com/biomejs/biome/blob/main/CONTRIBUTING.md#testing)
to know how to deal with the snapshot tests.

### Promote a rule

Promoting a rule when is stable can be a tedious work. Internally, we have a script
that does that for you:

```shell
just promote-rule noConsoleLog style
```

The first argument is the name of the rule, in camel case. The second argument
is the name of the group where you're promoting the rule to.

The script will run some checks and some other script for you.

You're now ready to commit the changes [using `git`](#commit-your-work)!

### Document the rule

The documentation needs to adhere to the following rules:
- The **first** paragraph of the documentation is used as brief description of the rule, and it **must** be written in one single line. Breaking the paragraph in multiple lines will break the table content of the rules page.
- The next paragraphs can be used to further document the rule with as many details as you see fit.
- The documentation must have a `## Examples` header, followed by two headers: `### Invalid` and `### Valid`. `### Invalid` must go first because we need to show when the rule is triggered.
- Each code block must have a _language_ defined.
- When adding _invalid_ snippets in the `### Invalid` section, you must use the `expect_diagnostic` code block property. We use this property to generate a diagnostic and attach it to the snippet. A snippet **must emit only ONE diagnostic**.
- When adding _valid_ snippets in the `### Valid` section, you can use one single snippet.
- You can use the code block property `ignore` to tell the code generation script to **not generate a diagnostic for an invalid snippet**.

Here's an example of how the documentation could look like:

```rust,ignore
use biome_analyze::declare_rule;
declare_rule! {
    /// Disallow the use of `var`.
    ///
    /// _ES2015_ allows to create variables with block scope instead of function scope
    /// using the `let` and `const` keywords.
    /// Block scope is common in many other programming languages and help to avoid mistakes.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-var
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var foo = 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var bar = 1;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = 1;
    /// let bar = 1;
    ///```
    pub(crate) NoVar {
        version: "next",
        name: "noVar",
        recommended: false,
    }
}
```

This will cause the documentation generator to ensure the rule does emit
exactly one diagnostic for this code, and to include a snapshot for the
diagnostic in the resulting documentation page.

### Code generation

For simplicity, use `just` to run all the commands with:

```shell
just gen-lint
```

This command runs several sub-commands:

- `cargo codegen-configuration`, **this command must be run first** and, it will update the configuration;

- `cargo lintdoc`, it will update the website with the documentation of the rules, check [`declare_rule`](#declare_rule)
  for more information about it;

- `cargo codegen-bindings`, it will update the TypeScript types released inside the JS APIs;

- `cargo codegen-schema`, it will update the JSON Schema file of the configuration, used by the npm packages.

### Commit your work

Once the rule implemented, tested, and documented, you are ready to open a pull request!

Stage and commit your changes:

```shell
> git add -A
> git commit -m 'feat(biome_js_analyze): myRuleName'
```

To test if everything is ready, run the following command:

```shell
just ready
```

### Rule configuration

Some rules may allow customization using options.
We try to keep rule options to a minimum and only when needed.
Before adding an option, it's worth a discussion.
Options should follow our [technical philosophy](https://biomejs.dev/internals/philosophy/#technical).

Let's assume that the rule we implement support the following options:

- `behavior`: a string among `"A"`, `"B"`, and `"C"`;
- `threshold`: an integer between 0 and 255;
- `behaviorExceptions`: an array of strings.

We would like to set the options in the `biome.json` configuration file:

```json
{
  "linter": {
    "rules": {
      "recommended": true,
      "nursery": {
        "my-rule": {
          "behavior": "A",
          "threshold": 30,
          "behaviorExceptions": ["f"],
        }
      }
    }
  }
}
```

The first step is to create the Rust data representation of the rule's options.

```rust,ignore
use biome_deserializable_macros::Deserializable;

#[derive(Clone, Debug, Default, Deserializable)]
pub struct MyRuleOptions {
    behavior: Behavior,
    threshold: u8,
    behavior_exceptions: Vec<String>
}

#[derive(Clone, Debug, Default, Deserializable)]
pub enum Behavior {
    #[default]
    A,
    B,
    C,
}
```

To allow deserializing instances of the types `MyRuleOptions` and `Behavior`,
they have to implement the `Deserializable` trait from the `biome_deserialize` crate.
This is what the `Deserializable` keyword in the `#[derive]` statements above did.
It's a so-called derive macros, which generates the implementation for the `Deserializable` trait
for you.

With these types in place, you can set the associated type `Options` of the rule:

```rust,ignore
impl Rule for MyRule {
    type Query = Semantic<JsCallExpression>;
    type State = Fix;
    type Signals = Vec<Self::State>;
    type Options = MyRuleOptions;

    ...
}
```

A rule can retrieve its options with:

```rust,ignore
let options = ctx.options();
```

The compiler should warn you that `MyRuleOptions` does not implement some required types.
We currently require implementing _serde_'s traits `Deserialize`/`Serialize`.
You can simply use a derive macros:

```rust,ignore
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MyRuleOptions {
    #[serde(default, skip_serializing_if = "is_default")]
    main_behavior: Behavior,

    #[serde(default, skip_serializing_if = "is_default")]
    extra_behaviors: Vec<Behavior>,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum Behavior {
    #[default]
    A,
    B,
    C,
}
```

### Deprecate a rule

There are occasions when a rule must be deprecated, to avoid breaking changes. The reason
of deprecation can be multiple.

In order to do, the macro allows adding additional field to add the reason for deprecation

```rust,ignore
use biome_analyze::declare_rule;

declare_rule! {
    /// Disallow the use of `var`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a, b;
    /// ```
    pub(crate) NoVar {
        version: "1.0.0",
        name: "noVar",
        deprecated: "Use the rule `noAnotherVar`",
        recommended: false,
    }
}
```

### Custom Visitors

Some lint rules may need to deeply inspect the child nodes of a query match
before deciding on whether they should emit a signal or not. These rules can be
inefficient to implement using the query system, as they will lead to redundant
traversal passes being executed over the same syntax tree. To make this more
efficient, you can implement a custom `Queryable` type and associated
`Visitor` to emit it as part of the analyzer's main traversal pass. As an
example, here's how this could be done to implement the `useYield` rule:

```rust,ignore
// First, create a visitor struct that holds a stack of function syntax nodes and booleans
#[derive(Default)]
struct MissingYieldVisitor {
    stack: Vec<(AnyFunctionLike, bool)>,
}

// Implement the `Visitor` trait for this struct
impl Visitor for MissingYieldVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(node) => {
                // When the visitor enters a function node, push a new entry on the stack
                if let Some(node) = AnyFunctionLike::cast_ref(node) {
                    self.stack.push((node, false));
                }

                if let Some((_, has_yield)) = self.stack.last_mut() {
                    // When the visitor enters a `yield` expression, set the
                    // `has_yield` flag for the top entry on the stack to `true`
                    if JsYieldExpression::can_cast(node.kind()) {
                        *has_yield = true;
                    }
                }
            }
            WalkEvent::Leave(node) => {
                // When the visitor exits a function, if it matches the node of the top-most
                // entry of the stack and the `has_yield` flag is `false`, emit a query match
                if let Some(exit_node) = AnyFunctionLike::cast_ref(node) {
                    if let Some((enter_node, has_yield)) = self.stack.pop() {
                        debug_assert_eq!(enter_node, exit_node);
                        if !has_yield {
                            ctx.match_query(MissingYield(enter_node));
                        }
                    }
                }
            }
        }
    }
}

// Declare a query match struct type containing a JavaScript function node
pub(crate) struct MissingYield(AnyFunctionLike);

impl QueryMatch for MissingYield {
    fn text_range(&self) -> TextRange {
        self.0.range()
    }
}

// Implement the `Queryable` trait for this type
impl Queryable for MissingYield {
    // `Input` is the type that `ctx.match_query()` is called with in the visitor
    type Input = Self;
    type Language = JsLanguage;
    // `Output` if the type that `ctx.query()` will return in the rule
    type Output = AnyFunctionLike;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        // Register our custom visitor to run in the `Syntax` phase
        analyzer.add_visitor(Phases::Syntax, MissingYieldVisitor::default);
    }

    // Extract the output object from the input type
    fn unwrap_match(services: &ServiceBag, query: &Self::Input) -> Self::Output {
        query.0.clone()
    }
}

impl Rule for UseYield {
    // Declare the custom `MissingYield` queryable as the rule's query
    type Query = MissingYield;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        // Read the function's root node from the queryable output
        let query: &AnyFunctionLike = ctx.query();

        // ...
    }
}
```

### `declare_rule`

This macro is used to declare an analyzer rule type, and implement the [RuleMeta] trait for it.

The macro itself expect the following syntax:

```rust,ignore
use biome_analyze::declare_rule;

declare_rule! {
    /// Documentation
    pub(crate) ExampleRule {
        version: "next",
        name: "myRuleName",
        recommended: false,
    }
}
```

### Category Macro

Declaring a rule using `declare_rule!` will cause a new `rule_category!`
macro to be declared in the surrounding module. This macro can be used to
refer to the corresponding diagnostic category for this lint rule, if it
has one. Using this macro instead of getting the category for a diagnostic
by dynamically parsing its string name has the advantage of statically
injecting the category at compile time and checking that it is correctly
registered to the `biome_diagnostics` library.

```rust,ignore
declare_rule! {
    /// Documentation
    pub(crate) ExampleRule {
        version: "next",
        name: "myRuleName",
        recommended: false,
    }
}

impl Rule for ExampleRule {
    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().text_trimmed_range(),
            "message",
        ))
    }
}
```

### Semantic Model

The semantic model provides information about the references of a binding (variable) within a program, indicating if it is written (e.g., `const a = 4`), read (e.g., `const b = a`, where `a` is read), or exported.


#### How to use the query `Semantic<>` in a lint rule

We have a for loop that creates an index i, and we need to identify where this index is used inside the body of the loop

```js
for (let i = 0; i < array.length; i++) {
  array[i] = i
}
```

To get started we need to create a new rule using the semantic type `type Query = Semantic<JsForStatement>;`
We can now use the `ctx.model()` to get information about bindings in the for loop.

```rust,ignore
impl Rule for ForLoopCountReferences {
    type Query = Semantic<JsForStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        // The model holds all informations about the semantic, like scopes and declarations
        let model = ctx.model();

        // Here we are extracting the `let i = 0;` declaration in for loop
        let initializer = node.initializer()?;
        let declarators = initializer.as_js_variable_declaration()?.declarators();
        let initializer = declarators.first()?.ok()?;
        let initializer_id = initializer.id().ok()?;

        // Now we have the binding of this declaration
        let binding = initializer_id
            .as_any_js_binding()?
            .as_js_identifier_binding()?;

        // How many times this variable appers in the code
        let count = binding.all_references(model).count();

        // Get all read references
        let readonly_references = binding.all_reads(model);

        // Get all write references
        let write_references = binding.all_writes(model);
    }
}
```
