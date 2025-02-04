# Analyzer

The analyzer is a generic crate aimed to implement a visitor-like infrastructure, where
it's possible to inspect a piece of AST and emit diagnostics or actions based on a
static check.

The analyzer allows implementors to create **four different** types of rules:
- **Syntax**: This rule checks the syntax according to the language specification and emits error diagnostics accordingly.
- **Lint**: This rule performs static analysis of the source code to detect invalid or error-prone patterns, and emits diagnostics along with proposed fixes.
- **Assist**: This rule detects refactoring opportunities and emits code action signals.
- **Transformation**: This rule detects transformations that should be applied to the code.

### Table of Contents

- [Creating a Rule](#creating-a-rule)
  - [Guidelines](#guidelines)
    - [Naming Conventions for Rules](#naming-conventions-for-rules)
    - [What a Rule should say to the User](#what-a-rule-should-say-to-the-user)
    - [Placement of New Rules](#placement-of-new-rules)
  - [Creating and Implementing the Rule](#creating-and-implementing-the-rule)
  - [Coding Tips for Rules](#coding-tips-for-rules)
    - [`declare_lint_rule!` macro](#declare_lint_rule-macro)
    - [`rule_category!` macro](#rule_category-macro)
    - [Rule Options](#rule-options)
    - [Navigating the CST (Concrete Syntax Tree)](#navigating-the-cst-concrete-syntax-tree)
    - [Querying multiple node types via `declare_node_union!`](#querying-multiple-node-types-via-declare_node_union)
    - [Semantic Model](#semantic-model)
    - [Multiple Signals](#multiple-signals)
    - [Code Actions](#code-actions)
    - [Custom Syntax Tree Visitors](#custom-syntax-tree-visitors)
    - [Common Logic Mistakes](#common-logic-mistakes)
  - [Testing the Rule](#testing-the-rule)
    - [Quick Test](#quick-test)
    - [Snapshot Tests](#snapshot-tests)
    - [Run the Snapshot Tests](#run-the-snapshot-tests)
  - [Documenting the Rule](#documenting-the-rule)
    - [General Structure](#general-structure)
    - [Associated Language(s)](#associated-languages)
    - [Code Blocks](#code-blocks)
    - [Using Rule Options](#using-rule-options)
    - [Full Documentation Example](#full-documentation-example)
  - [Code generation](#code-generation)
  - [Commiting your work](#commiting-your-work)
  - [Sidenote: Deprecating a rule](#sidenote-deprecating-a-rule)

## Creating a Rule

When creating or updating a lint rule, you need to be aware that there's a lot of generated code inside our toolchain.
Our CI ensures that this code is not out of sync and fails otherwise.
See the [code generation section](#code-generation) for more details.

To create a new rule, you have to create and update several files.
Because it is a bit tedious, _Biome_ provides an easy way to create and test your rule using [Just](https://just.systems/man/en/).
_Just_ is not part of the rust toolchain, you have to install it with [a package manager](https://just.systems/man/en/packages.html).

### Guidelines

#### Naming Conventions for Rules

_Biome_ follows a naming convention according to what the rule does:

1. _Forbid &lt;a concept&gt;_

   ```
   no<Concept>
   ```

   When a rule's sole intention is to **forbid a single concept** - such as disallowing the use of `debugger` statements - the rule should be named using the `no` prefix.

   > [!NOTE]
   > For example, the rule to disallow the use of `debugger` statements is named `noDebugger`.

2. _Mandate &lt;a concept&gt;_

   ```
   use<Concept>
   ```

   When a rule's sole intention is to **mandate a single concept** - such as forcing the use of correct values for a certain attribute or the use of identifiers following a naming convention - the rule should be named using the `use` prefix.

   > [!NOTE]
   > For example, the rule to mandating the use valid values for the HTML `lang` attribute is named `useValidLang`.

#### What a Rule should say to the User

A rule should be informative to the user, and give as much explanation as possible.

When writing a rule, you must adhere to the following **pillars**:

1. Explain to the user **what** the error is.
   Generally, this is the message of the diagnostic.

2. Explain to the user ***why*** the error is triggered.
   Generally, this is implemented with an additional output node.

3. Tell the user **what** they **should do**. Generally, this is implemented using a [code action](#code-actions).
   If a code action is not applicable a note should tell the user what they should do to fix the error.

#### Placement of New Rules

New rules **must** be placed inside the `nursery` group. This group is meant as an incubation space, exempt from semantic versioning. Once a rule is stable, it's promoted to a group that fits it. This is done in a minor/major release.

> [!TIP]
> As a developer, you aren't forced to make a rule perfect in one PR. Instead, you are encouraged to lay out a plan and to split the work into multiple PRs.
>
> If you aren't familiar with Biome's APIs, this is an option that you have. If you decide to use this option, you should make sure to describe your plan in an issue.

### Creating and Implementing the Rule

Let's say we want to create a new **lint** rule called `useMyRuleName`, follow these steps:

1. **Generate the code for your rule** by running this command
   _(Hint: Replace `useMyRuleName` with your custom name as recommended by the [naming convention](#guideline-naming-convention-for-rules))_:

   ```shell
   # Example: Create a new JS lint rule
   just new-js-lintrule useMyRuleName

   # Or, to create a new lint/assist rule for JSON/CSS/GraphQL/..:
   # $ just new-css-lintrule useMyRuleName
   # $ just new-graphql-lintrule useMyRuleName
   # $ just new-js-assistrule useMyRuleName
   # $ just new-json-assistrule useMyRuleName
   ```
   The script `just new-js-lintrule` script will generate a bunch of files for the _JavaScript_ language inside the `biome_js_analyze` crate.
   Among the other files, you'll find a file called `use_my_rule_name.rs` inside the `biome_js_analyze/lib/src/lint/nursery` folder. You'll implement your rule in this file.

2. Let's have a look at the generated code in  `use_my_rule_name.rs`:


   ```rust
   ...
   impl Rule for UseMyRuleName {
       type Query = Ast<JsIdentifierBinding>;
       type State = ();
       type Signals = Option<Self::State>;
       type Options = ();

       fn run(ctx: &RuleContext<Self>) -> Self::Signals {
           ...
       }

       fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
           ...
       }
   }
   ```

   We can observe a few details from this snippet:

   - The **`Options`** type can be used to define [additional options](#rule-options) for your rule:

     ```rust
     type Options = ();
     ```

     Use `()` if your rule does not have additional options.

   - The **`Query`** type defines the entities for which your your rule's `UseMyRuleName::run` function will be invoked:
     ```rust
     type Query = Ast<JsIdentifierBinding>;
     ```

     _→ The `Ast<>` query type, for example, allows you to query the AST/CST of a program for nodes of a specific type._

     _→ For more advanced use cases, it is also possible to define [custom query types](#custom-syntax-tree-visitors)._

   - The **`run`** function will be invoked for each match of `Query`.
     It should return either `Some` to report a diagnostic for that match, or `None`.

     _→ It is also possible to report multiple diagnostics and/or code actions for a single `Query` match._
     _See [Multiple Signals](#multiple-signals) for instructions._

     ```rust
     type Signals = Option<Self::State>;

     fn run(ctx: &RuleContext<Self>) -> Self::Signals {
       let matched_node = ctx.query();
       ...
     }
     ```

   - The **`diagnostic`** function will be invoked for each signal returned by `run`, and turns these signals into `RuleDiagnostic` instances that define the message(s) reported to the user.

     ```rust
     fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> { ... }
     ```

   - The `State` type can be used to pass additional information for each signal reported by the `run` function to the `diagnostic` and `action` functions.
     Use `()` if you don't need to pass additional information:

     ```rust
     type State = ();
     ```

3. Optional: Use `Options` to [define custom options](#rule-options) for your rule.
   We'll leave it as `()` for now.

4. Use `Query` to define the node type that you want to analyze.
   We'll leave it as `Ast<JsIdentifierBinding>` for our example:

   ```rust
   type Query = Ast<JsIdentifierBinding>;
   ```

5. Implement the `run` function. This function is called every time the analyzer finds a match for the query specified by the rule, and may return zero or more "signals":

   ```rust
   fn run(ctx: &RuleContext<Self>) -> Self::Signals {
       let binding = ctx.query();

       if binding.name_token().ok()?.text() == "prohibited_identifier" {
           Some(())
       } else {
           None
       }
   }
   ```

6. Implement the `diagnostic` function to define what the user will see.

   Follow the [guidelines & pillars](#explain-a-rule-to-the-user) when writing the messages.
   Please also keep [Biome's technical principals](https://biomejs.dev/internals/philosophy/#technical) in mind when writing those messages and implementing your diagnostic rule.

   ```rust
   fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic>
       let node = ctx.query();
       Some(
           RuleDiagnostic::new(
               rule_category!(),
               node.range(),
               markup! {
                   "Use of a prohibited identifier name"
               },
           )
           .note(markup! {
               "This note will give you more information."
           }),
       )
   }
   ```


6. Optional: Implement the `action` function if your rule is able to provide a [code action](#code-actions):

   ```rust
   impl Rule for UseAwesomeTricks {
       fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
           let mut mutation = ctx.root().mutation();
           Some(JsRuleAction::new(
               ctx.action_category(ctx.category(), ctx.group()),
               ctx.metadata().applicability(),
               markup! { "<MESSAGE>" }.to_owned(),
               mutation,
           ))
       }
   }
   ```

   It may return zero or one code action.
   Rules can return a code action that can be **safe** or **unsafe**. If a rule returns a code action, you must add `fix_kind` to the macro `declare_lint_rule`.

   ```rust
   use biome_analyze::FixKind;
   declare_lint_rule!{
     fix_kind: FixKind::Safe,
   }
   ```
   When returning a code action, you must pass the `category` and the `applicability` fields.
   `category` must be `ctx.action_category(ctx.category(), ctx.group())`.
   `applicability` is derived from the metadata [`fix_kind`](#code-actions).
   In other words, the code transformation should always result in code that doesn't change the behavior of the logic.
   In the case of `noVar`, it is not always safe to turn `var` to `const` or `let`.

Don't forget to format your code with `just f` and lint with `just l`.

That's it! Now, let's [test the rule](#testing-the-rule).

### Coding Tips for Rules

Below, there are many tips and guidelines on how to create a lint rule using Biome infrastructure.


#### `declare_lint_rule!` macro

This macro is used to declare an analyzer rule type, and implement the [RuleMeta] trait for it.

The macro itself expects the following syntax:

```rust
use biome_analyze::declare_lint_rule;

declare_lint_rule! {
    /// Documentation
    pub(crate) ExampleRule {
        version: "next",
        name: "myRuleName",
        language: "js",
        recommended: false,
    }
}
```

##### Biome lint rules inspired by other lint rules

If a **lint** rule is inspired by an existing rule from other ecosystems (ESLint, ESLint plugins, clippy, etc.), you can add a new metadata to the macro called `source`. Its value is `&'static [RuleSource]`, which is a reference to a slice of `RuleSource` elements, each representing a different source.

If you're implementing a lint rule that matches the behaviour of the ESLint rule `no-debugger`, you'll use the variant `::ESLint` and pass the name of the rule:

```rust
use biome_analyze::{declare_lint_rule, RuleSource};

declare_lint_rule! {
    /// Documentation
    pub(crate) ExampleRule {
        version: "next",
        name: "myRuleName",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("no-debugger")],
    }
}
```

If the rule you're implementing has a different behaviour or option, you can add the `source_kind` metadata and use the `RuleSourceKind::Inspired` type. If there are multiple sources, we assume that each source has the same `source_kind`.

```rust
use biome_analyze::{declare_lint_rule, RuleSource, RuleSourceKind};

declare_lint_rule! {
    /// Documentation
    pub(crate) ExampleRule {
        version: "next",
        name: "myRuleName",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("no-debugger")],
        source_kind: RuleSourceKind::Inspired,
    }
}
```

By default, `source_kind` is always `RuleSourceKind::SameLogic`.

#### `rule_category!` macro

Declaring a rule using [`declare_lint_rule!`](#declare_lint_rule-macro) will cause a new `rule_category!`
macro to be declared in the surrounding module. This macro can be used to
refer to the corresponding diagnostic category for this lint rule, if it
has one. Using this macro instead of getting the category for a diagnostic
by dynamically parsing its string name has the advantage of statically
injecting the category at compile time and checking that it is correctly
registered to the `biome_diagnostics` library.

```rust
declare_lint_rule! {
    /// Documentation
    pub(crate) ExampleRule {
        version: "next",
        name: "myRuleName",
        language: "js",
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

#### Rule severity

The macro accepts a `severity` field, of type `biome_diagnostics::Severity`. By default, rules without `severity` will start with `Severity::Information`.

If you want to change the default severity, you need to assign it:

```diff
+ use biome_diagnostics::Severity;

declare_lint_rule! {
    /// Documentation
    pub(crate) ExampleRule {
        version: "next",
        name: "myRuleName",
        language: "js",
        recommended: false,
+       severity: Severity::Warning,
    }
}
```

#### Rule domains

Domains are very specific ways to collect rules that belong to the same "concept". Domains are a way for users to opt-in/opt-out rules that belong to the same domain.

Some examples of domains: testing, specific framework, specific runtime, specific library. A rule can belong to multiple domains.

```diff
+ use biome_analyze::RuleDomain;


declare_lint_rule! {
    /// Documentation
    pub(crate) ExampleRule {
        version: "next",
        name: "myRuleName",
        language: "js",
        recommended: true,
+       domains: &[RuleDomain::Test],
    }
}
```

Rule domains can unlock various perks in the Biome analyzer:
- A domain can define a number of `package.json` dependencies. When a user has one or more of these dependencies, Biome will automatically enable the recommended rules that belong to the domain. To add/update/remove dependencies to a domain, check the function `RuleDomain::manifest_dependencies`.
- A domain can define a number of "globals". These globals will be used by other rules, and improve the UX of them. To add/update/remove globals to a domain, check the function `RuleDomain::globals`.

When a rule is **recommended** and _has domains_, the rule is enabled only when the user enables the relative domains via `"recommneded"` or `"all"`.
Instead, if the rule is **recommended** but _doesn't have domains_, the rule is always enabled by default.

> [!NOTE]
> Before adding a new domain, please consult with the maintainers of the project.

#### Rule Options

Some rules may allow customization [using per-rule options in `biome.json`](https://biomejs.dev/linter/#rule-options).

> [!NOTE]
> We try to keep rule options to a minimum and only provide them when needed.
> Before adding an option, it's worth a discussion.
>
> If provided, options should follow our [technical philosophy](https://biomejs.dev/internals/philosophy/#technical).

##### Options for our example rule

Let's assume that the rule we want to implement supports the following options:

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
          "options": {
            "behavior": "A",
            "threshold": 30,
            "behaviorExceptions": ["f"],
          }
        }
      }
    }
  }
}
```

##### Representing the rule options in Rust

The first step is to create the Rust data representation of the rule's options.

```rust
use biome_deserialize_macros::Deserializable;

#[derive(Clone, Debug, Default, Deserializable)]
pub struct MyRuleOptions {
    behavior: Behavior,
    threshold: u8,
    behavior_exceptions: Box<[Box<str>]>
}

#[derive(Clone, Debug, Default, Deserializable)]
pub enum Behavior {
    #[default]
    A,
    B,
    C,
}
```

Note that we use a boxed slice `Box<[Box<str>]>` instead of `Vec<String>`.
This allows saving memory: [boxed slices and boxed str use 2 words instead of three words](https://nnethercote.github.io/perf-book/type-sizes.html#boxed-slices).

To allow deserializing instances of the types `MyRuleOptions` and `Behavior`,
they have to implement the `Deserializable` trait from the `biome_deserialize` crate.
This is what the `Deserializable` keyword in the `#[derive]` statements above did.
It's a so-called derive macros, which generates the implementation for the `Deserializable` trait
for you.

With these types in place, you can set the associated type `Options` of the rule:

```rust
impl Rule for MyRule {
    type Query = Semantic<JsCallExpression>;
    type State = Fix;
    type Signals = Vec<Self::State>;
    type Options = MyRuleOptions;
}
```

##### Retrieving the rule options within a Rule

A rule can retrieve the options that apply to the location of the currently matched node with:

```rust
let options = ctx.options();
```

Modifications of the configuration via e.g. [_`extends`_](https://biomejs.dev/reference/configuration/#extends)
and [_`overrides`_](https://biomejs.dev/reference/configuration/#overrides) (in `biome.json`)
that apply only to a subset of files are automatically taken into account,
and do not need to be handled by the rule itself.

##### Implementing JSON deserialization/serialization support

> [!WARNING]
> Although we use `serde`s attribute syntax, we do not actually use the `serde` crate for (de)serialization of `biome.json`.
>
> We instead provide a ***`serde`-inspired*** implementation in `biome_deserialize` and `biome_deserialize_macros` that [differs in some aspects](../biome_deserialize/README.md), like being fault-tolerant.

The compiler should warn you that `MyRuleOptions` does not implement some required types.
We currently require implementing _serde_'s traits `Deserialize`/`Serialize`.

Also, we use other `serde` macros to adjust the JSON configuration:
- `rename_all = "camelCase"`: it renames all fields in camel-case, so they are in line with the naming style of the `biome.json`.
- `deny_unknown_fields`: it raises an error if the configuration contains extraneous fields.
- `default`: it uses the `Default` value when the field is missing from `biome.json`. This macro makes the field optional.

Because we use `schemars`to generate a JSON schema for `biome.json`, our options type must support the `schemars::JsonSchema` trait as well.

You can simply use the derive macros provided by `serde`, `biome_deserialize` and `schemars` to generate the necessary implementations automatically:

```rust
use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, Deserializable)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
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

##### Testing & Documenting Rule Options

As with every other user-facing aspect of a rule, the effect that options have on a rule's operation should be both documented and tested, as explained in more detail in the section [Documenting the rule](#documenting-the-rule).

#### Navigating the CST (Concrete Syntax Tree)

When navigating the nodes and tokens of certain nodes, you will notice straight away that the majority of those methods will return a `Result` (`SyntaxResult`).

Generally, you will end up navigating the CST inside the `run` function, and this function will usually return an `Option` or a `Vec`.

- If the `run` function returns an `Option`, you're encouraged to transform the `Result` into an `Option` and use the try operator `?`. This will make your coding way easier:

  ```rust
  fn run() -> Self::Signals {
    let prev_val = js_object_member.value().ok()?;
  }
  ```
- If the `run` function returns a `Vec`, you're encouraged to use the `let else` trick to reduce code branching:

  ```rust
    fn run() -> Self::Signals {
      let Ok(prev_val) = js_object_member.value() else {
        return vec![]
      };
    }
  ```

#### Querying multiple node types via `declare_node_union!`

There are times when you might need to query multiple nodes at once. Instead of querying the root of the CST, you can use the macro `declare_node_union!` to "join" multiple nodes into an `enum`:

```rust
use biome_rowan::{declare_node_union, AstNode};
use biome_js_syntax::{AnyJsFunction, JsMethodObjectMember, JsMethodClassMember};

declare_node_union! {
  pub AnyFunctionLike = AnyJsFunction | JsMethodObjectMember | JsMethodClassMember
}
```

When creating a new node like this, we internally prefix them with `Any*` and postfix them with `*Like`. This is our internal naming convention.

The type `AnyFunctionLike` implements the trait `AstNode`, which means that it implements all methods such as `syntax`, `children`, etc.

#### Semantic Model

The semantic model provides information about the references of a binding (declaration) within a program, indicating if it is written (e.g., `const a = 4`), read (e.g., `const b = a`, where `a` is read), or exported.


##### How to use the query `Semantic<>` in a lint rule

We have a for loop that creates an index i, and we need to identify where this index is used inside the body of the loop

```js
for (let i = 0; i < array.length; i++) {
  array[i] = i
}
```

To get started we need to create a new rule using the semantic type `type Query = Semantic<JsForStatement>;`
We can now use the `ctx.model()` to get information about bindings in the for loop.

```rust
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

#### Multiple Signals

Some rules require you to find all possible cases upfront in `run` function.
To achieve that you can change Signals type from `Option<Self::State>` to an iterable data structure such as `Vec<Self::State>` or `Box<[Self::State]>`.
This will call the diagnostic/action function for every item of the data structure.
We prefer to use `Box<[_]>` over `Vec<_>` because it takes less memory.
You can easily convert a `Vec<_>` into a `Box<[_]>` using the `Vec::into_boxed_slice()` method.

Taking previous example and modifying it a bit we can apply diagnostic for each item easily.

```rust
impl Rule for ForLoopCountReferences {
    type Query = Semantic<JsForStatement>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let model = ctx.model();

        ...

        // Get all write references
        let write_references = binding.all_writes(model);

        // Find all places where variable is being written to and get node ranges
        let write_ranges = write_references.into_iter().map(|write| {
            let syntax = write.syntax();
            let range = syntax.text_range();

            Some(range)
        }).collect::<Vec<_>>();

        write_ranges.into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        // This will be called for each vector item
    }
}
```

#### Code Actions

A rule can provide one or more code actions. Code actions provide the final user with the option to fix or change their code.

In a lint rule, for example, it signals an opportunity for the user to fix the diagnostic emitted by the rule.

First, you have to add a new metadata called `fix_kind` that specifies whether the fixes emitted by the rule are considered [**"safe"**](https://biomejs.dev/linter/#safe-fixes) or [**"unsafe"**](https://biomejs.dev/linter/#unsafe-fixes).

```rust
use biome_analyze::{declare_lint_rule, FixKind};

declare_lint_rule! {
    /// Documentation
    pub(crate) ExampleRule {
        version: "next",
        name: "myRuleName",
        language: "js",
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}
```

Then, you'll have to implement the `action` function of the `Rule` trait and return a `JsRuleAction`.

`JsRuleAction` needs, among other things, a `mutation` type, which you will use to store all additions, deletions and replacements that will be executed when the user applies the action:

```rust
impl Rule for ExampleRule {
    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
      let mut mutation = ctx.root().begin();

      Some(JsRuleAction::new(
        ctx.action_category(ctx.category(), ctx.group()),
        ctx.metadata().applicability(),
        markup! { "Remove the '"{name.text_trimmed()}"' element." }.to_owned(),
        mutation,
      ))
    }
}
```

The `ctx.metadata().applicability()` function will compute the `Applicability` type from the `fix_kind` value you provided at the beginning inside the `declare_lint_rule!` macro.

#### Custom Syntax Tree Visitors

Some lint rules may need to deeply inspect the child nodes of a query match
before deciding on whether they should emit a signal or not. These rules can be
inefficient to implement using the query system, as they will lead to redundant
traversal passes being executed over the same syntax tree. To make this more
efficient, you can implement a custom `Queryable` type and associated
`Visitor` to emit it as part of the analyzer's main traversal pass. As an
example, here's how this could be done to implement the `useYield` rule:

```rust
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

#### Common Logic Mistakes

There are some common mistakes that can lead to bugs or false positives in lint rules. These tips should help you avoid them and write more robust rules.

##### Not checking if a variable is global

Some rules aim to ban certain functions or variables (eg. `noConsoleLog` bans `console.log`). A common mistake make this check without considering if the variable is global or not. This can lead to false positives if the variable is declared in a local scope.

```js
console.log(); // <-- This should be reported because `console` is a global variable
const console = { log() {} };
console.log(); // <-- This should not be reported because `console` is redeclared as a local variable
```

To avoid this, you should consult the semantic model to check if the variable is global or not.

### Testing the Rule

#### Quick Test

A swift way to test your rule is to go inside the `biome_js_analyze/src/lib.rs` file (this will change based on where you're implementing the rule) and modify the `quick_test` function.

Usually this test is ignored, so remove/_comment_ the `#[ignore]` macro and change the `let SOURCE` variable to whatever source code you need to test. Then update the rule filter, and add your rule:

```rust
let rule_filter = RuleFilter::Rule("nursery", "useAwesomeTrick");
```

Now from your terminal, switch to the `crates/biome_js_analyze` folder and run the test using `cargo`:

```shell
cd crates/biome_js_analyze
cargo t quick_test
```

Remember that if you added `dbg!` macros inside your source code, you'll have to use `--show-output`:

```shell
cargo t quick_test -- --show-output
```

The test is designed to **show** diagnostics and code actions if the rule correctly emits the signal. If nothing is shown, your logic didn't emit any signal.

#### Snapshot Tests

> [!TIP]
> Most of the testing of the rules themselves is done by snapshot tests using the [`insta`](https://docs.rs/insta/latest/insta/) library.
>
> A rule is run against a set of known inputs, and its diagnostic output (in text form) is compared against a known-good example output.
> Check our main [contribution document](https://github.com/biomejs/biome/blob/main/CONTRIBUTING.md#testing) for additional information on how to deal with the snapshot tests.


Inside the `tests/specs/` folder, rules are divided by group and rule name.
The test infrastructure is rigid around the association of the pair "group/rule name", which means that _**if your test cases are placed inside the wrong group, you won't see any diagnostics**_.

Since each new rule will start from `nursery`, that's where we'll start.
If you used `just new-js-lintrule`, a folder with the name of the rule should already exist there.
Otherwise, create a folder called `myRuleName/`, and then create one or more files for the different cases you want to test.

> [!NOTE]
> A common pattern is to create files prefixed by `invalid` or `valid`.
> - The files prefixed by `invalid` contain code that is reported by the rule.
> - The files prefixed by `valid` contain code that is not reported by the rule.

##### `.jsonc` files

Files ending with the extension `.jsonc` are handled differently.
These files should contain an array of strings where each string is a code snippet.
For instance, for the rule `noVar`, the file `invalidScript.jsonc` contains:

```jsonc
["var x = 1; foo(x);", "for (var x of [1,2,3]) { foo(x); }"]
```

Note that the code in those `.jsonc` files is interpreted in a _script environment_.
This means that you cannot use syntax that belongs to _ECMAScript modules_ such as `import` and `export`.

#### Run the Snapshot Tests

Run the command:

```shell
just test-lintrule myRuleName
```

and if you've done everything correctly, you should see some snapshots emitted with diagnostics and code actions.

Check our main [contribution document](https://github.com/biomejs/biome/blob/main/CONTRIBUTING.md#testing) to know how to deal with the snapshot tests.

### Documenting the Rule

The documentation needs to adhere to the following rules:

#### General Structure

- The **first** paragraph of the documentation is used as a brief description of the rule,
  and it **must** be written in one single line. Breaking the paragraph into multiple lines
  will break the table contents of the rules overview page.
- The next paragraphs can be used to further document the rule with as many details as you see fit.
- The documentation must have a `## Examples` header, followed by two headers: `### Invalid` and `### Valid`.
  `### Invalid` must go first because we need to show when the rule is triggered.
- Rule options if any, must be documented in the `## Options` section.

#### Associated Language(s)

- Update the `language` field in the `declare_lint_rule!` macro to the language the rule primarily applies to.
  - If your rule applies to any JavaScript, you can leave it as `js`.
  - If your rule only makes sense in a specific JavaScript dialect, you should set it to `jsx`, `ts`, or `tsx`, whichever is most appropriate.

#### Code Blocks

> [!TIP]
> The build process will automatically check each (properly marked) code block in a rule's documentation comment to ensure that:
>
> 1. The `### Valid` examples contain valid, parseable code, and the rule
>    does not report any diagnostics for them.
> 2. Each `### Invalid` example reports _exactly one_ diagnostic.
>    The output of the diagnostic will also be shown in the [generated documentation
>    for that rule](https://biomejs.dev/linter/rules/no-header-scope/#invalid) at [biomejs.dev](https://biomejs.dev/).
>
> To make this work, all code blocks must adhere to a few rules, as listed below:

- **Language**

  Each code block must have a _language_ defined (so that the correct syntax highlighting and analyzer options are applied).

- **Valid/Invalid snippets**

  When adding _invalid_ snippets in the `### Invalid` section, you must use the
  `expect_diagnostic` code block property. We use this property to generate a diagnostic
  and attach it to the snippet. A given snippet **must emit only ONE diagnostic**.

  When adding _valid_ snippets in the `### Valid` section, you can use one single snippet for all different valid cases.

- **Ignoring snippets**

  You can use the code block property `ignore` to tell the code generation script to **not generate a diagnostic for an invalid snippet** and **exclude it from the automatic validation** described above.

  Please use this sparingly and prefer automatically validated snippets, as this avoids out-of-date documentation when the implementation is changed.

- **Hiding lines**

  Although usually not necessary, it is possible to prevent code lines from being shown in the output by prefixing them with `# `.

  You should usually prefer to show a concise but complete sample snippet instead.

- **Ordering of code block properties**

  In addition to the language, a code block can be tagged with a few additional properties like `expect_diagnostic`, `options`, `full_options`, `use_options` and/or `ignore`.

  The parser does not care about the order, but for consistency, modifiers should always be ordered as follows:

  ````rust
  /// ```<language>[,expect_diagnostic][,(options|full_options|use_options)][,ignore]
  /// ```
  ````

  e.g.

  ````rust
  /// ```tsx,expect_diagnostic,use_options,ignore
  /// ```
  ````

#### Using Rule Options

All code blocks are interpreted as sample code that should be analyzed using the rule's default options by default, unless the codeblock is marked with `options`, `full_options` or `use_options`.
Codeblocks can therefore be of one of three types:

- Valid/Invalid **example snippets** using the **default settings** are marked as described above:

  ````rust
  /// ### Valid
  ///
  /// ```js
  /// var some_valid_example = true;
  /// ```
  ````

  ````rust
  /// ### Invalid
  ///
  /// ```ts,expect_diagnostic
  /// const some_invalid_example: UndeclaredType = false;
  /// ```
  ````

- Valid **configuration option snippets** that contain only the settings for the rule itself should be written in `json` or `jsonc` together with the code block property `options`:

  ````rust
  /// ### Valid
  ///
  /// ```json,options
  /// {
  ///     "options": {
  ///         "behavior": "A",
  ///         "threshold": 30,
  ///         "behaviorExceptions": ["f"]
  ///     }
  /// }
  /// ```
  ````

  Although usually not needed, you can show syntactically or semantically invalid configuration option snippets by adding `expect_diagnostic` in addition to `options`. As for normal snippets, a given snippet **must emit only ONE diagnostic**:

  ````rust
  /// ### Invalid
  ///
  /// ```json,expect_diagnostic,options
  /// {
  ///     "options": {
  ///         "behavior": "invalid-value"
  ///     }
  /// }
  /// ```
  ````

- Usually, the shown configuration option snippets only need to change rule-specific options.

  If you need to show off a **full `biome.json` configuration** instead, you can use `full_options` instead of `options` to change the parsing mode.

  ````rust
  /// ```jsonc,full_options
  /// {
  ///   "linter": {
  ///     "rules": {
  ///       "style": {
  ///         "useNamingConvention": "warn"
  ///       }
  ///     }
  ///   },
  ///   // ...
  ///   "overrides": [
  ///     {
  ///       // Override useNamingConvention for external module typing declarations
  ///       "include": ["typings/*.d.ts"],
  ///       "linter": {
  ///         "rules": {
  ///           "style": {
  ///             "useNamingConvention": "off"
  ///           }
  ///         }
  ///       }
  ///     }
  ///   ]
  /// }
  /// ```
  ````

  Although probably never needed, it is possible to define an expected-to-be-invalid full configuration snippet as follows:

  ````rust
  /// ```jsonc,expect_diagnostic,full_options
  /// {
  ///   "linter": {
  ///     // ...
  ///   }
  /// }
  /// ```
  ````

- A **valid** configuration option example can be followed by one or more valid/invalid code snippets that use these options, possibly with interleaving text.
  Those code snippets have to be marked with `use_options`:

  ````rust
  /// ### Valid/Invalid
  ///
  /// A configuration could look like this:
  ///
  /// ```json,options
  /// {
  ///     "options": {
  ///       "your-custom-option": "..."
  ///     }
  /// }
  /// ```
  ///
  /// And a usage looks like this:
  ///
  /// ```js,use_options
  /// var some_valid_example = true;
  /// ```
  ///
  /// And an "invalid" usage that triggers the rule looks like this:
  ///
  /// ```js,expect_diagnostic,use_options
  /// var this_should_trigger_the_rule = true;
  /// ```
  ````

#### Full Documentation Example

Here's an example of how the final documentation could look like:

```rust
use biome_analyze::declare_lint_rule;
declare_lint_rule! {
    /// Disallow the use of `var`.
    ///
    /// _ES2015_ allows to create variables with block scope instead of function scope
    /// using the `let` and `const` keywords.
    /// Block scope is common in many other programming languages and helps to avoid mistakes.
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
        language: "js",
        recommended: false,
    }
}
```

### Code generation

For simplicity, use `just` to run all the commands with:

```shell
just gen-analyzer
```

### Commiting your work

Once the rule is implemented, tested and documented, you are ready to open a pull request!

Stage and commit your changes:

```shell
> git add -A
> git commit -m 'feat(biome_js_analyze): myRuleName'
```

Then push your changes to your forked repository and open a pull request.

### Sidenote: Deprecating a rule

There are occasions when a rule must be deprecated to avoid breaking changes.
There are different reasons for deprecation, so the `declare_lint_rule!` macro enables you to specify the reason via an additional `deprecated:` field:

```rust
use biome_analyze::declare_lint_rule;

declare_lint_rule! {
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
        language: "js",
        deprecated: "Use the rule `noAnotherVar`",
        recommended: false,
    }
}
```
