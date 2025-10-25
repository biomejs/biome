use std::ops::Range;

use crate::{
    JsRuleAction,
    lint::correctness::no_unused_variables::is_unused,
    services::{control_flow::AnyJsControlFlowRoot, semantic::Semantic},
    utils::rename::{AnyJsRenamableDeclaration, RenameSymbolExtensions},
};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{CanBeImportedExported, SemanticModel};
use biome_js_syntax::{
    AnyJsClassMember, AnyJsObjectMember, AnyJsVariableDeclaration, AnyTsTypeMember, JsFileSource,
    JsIdentifierBinding, JsLiteralExportName, JsLiteralMemberName, JsModuleItemList,
    JsPrivateClassMemberName, JsShorthandPropertyObjectMember, JsSyntaxKind, JsSyntaxToken,
    JsVariableDeclarator, JsVariableKind, Modifier, TsDeclarationModule, TsIdentifierBinding,
    TsLiteralEnumMemberName, TsTypeParameterName,
    binding_ext::{AnyJsBindingDeclaration, AnyJsIdentifierBinding},
};
use biome_rowan::{
    AstNode, BatchMutationExt, SyntaxResult, TextRange, TextSize, declare_node_union,
};
use biome_string_case::{Case, Cases};
use biome_unicode_table::is_js_ident;
use enumflags2::BitFlags;
use smallvec::SmallVec;

pub use biome_rule_options::use_naming_convention::{
    Convention, Formats, Kind, RestrictedModifier, RestrictedModifiers, Scope, Selector,
    UseNamingConventionOptions,
};

declare_lint_rule! {
    /// Enforce naming conventions for everything across a codebase.
    ///
    /// Enforcing [naming conventions](https://en.wikipedia.org/wiki/Naming_convention_(programming)) helps to keep the codebase consistent,
    /// and reduces overhead when thinking about the name [case] of a variable.
    ///
    /// The following section describes the default conventions enforced by the rule.
    /// You can also enforce custom conventions with the [rule options](#options).
    ///
    /// ## Naming conventions
    ///
    /// All names can be prefixed and suffixed with underscores `_` and dollar signs `$`.
    /// Unused variables with a name prefixed with `_` are completely ignored.
    /// This avoids conflicts with the `noUnusedVariables` rule.
    ///
    /// ### Variable and parameter names
    ///
    /// All variables and function parameters are in [`camelCase`] or [`PascalCase`].
    /// Catch parameters are in [`camelCase`].
    ///
    /// Additionally, global variables declared as `const` or `var` may be in [`CONSTANT_CASE`].
    /// Global variables are declared at module or script level.
    /// Variables declared in a TypeScript `namespace` are also considered global.
    ///
    /// ```js
    /// function f(param, _unusedParam) {
    ///     let localValue = 0;
    ///     try {
    ///         /* ... */
    ///     } catch (customError) {
    ///         /* ... */
    ///     }
    /// }
    ///
    /// export const A_CONSTANT = 5;
    ///
    /// let aVariable = 0;
    ///
    /// export namespace ns {
    ///     export const ANOTHER_CONSTANT = "";
    /// }
    /// ```
    ///
    /// Examples of incorrect names:
    ///
    /// ```js,expect_diagnostic
    /// let a_value = 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const fooYPosition = 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f(FIRST_PARAM) {}
    /// ```
    ///
    /// ### Function names
    ///
    /// - A `function` name is in [`camelCase`] or [`PascalCase`].
    /// - A global `function` can also be in `UPPERCASE`.
    ///   This allows supporting the frameworks that require some function to use valid [HTTP method names](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods).
    ///
    /// ```jsx
    /// function trimString(s) { /*...*/ }
    ///
    /// function Component() {
    ///     return <div></div>;
    /// }
    ///
    /// export function GET() { /*...*/ }
    /// ```
    ///
    /// ### TypeScript `enum` names
    ///
    /// A _TypeScript_ `enum` name is in [`PascalCase`].
    ///
    /// `enum` members are by default in [`PascalCase`].
    /// However, you can configure the [case] of `enum` members.
    /// See [options](#options) for more details.
    ///
    /// ```ts
    /// enum Status {
    ///     Open,
    ///     Close,
    /// }
    /// ```
    ///
    /// ### Classes
    ///
    /// - A class name is in [`PascalCase`].
    ///
    /// - Static property and static getter names are in [`camelCase`] or [`CONSTANT_CASE`].
    ///
    /// - Class property and method names are in [`camelCase`].
    ///
    /// ```js
    /// class Person {
    ///     static MAX_FRIEND_COUNT = 256;
    ///
    ///     static get SPECIAL_PERSON_INSTANCE() { /*...*/ }
    ///
    ///     initializedProperty = 0;
    ///
    ///     specialMethod() {}
    /// }
    /// ```
    ///
    /// ### TypeScript `type` aliases and `interface`
    ///
    /// - A `type` alias or an interface name are in [`PascalCase`].
    ///
    /// - Member names of a type are in [`camelCase`].
    ///
    /// - `readonly` property and getter names can also be in [`CONSTANT_CASE`].
    ///
    /// ```ts
    /// type Named = {
    ///     readonly fullName: string;
    ///
    ///     specialMethod(): void;
    /// };
    ///
    /// interface Named {
    ///     readonly fullName: string;
    ///
    ///     specialMethod(): void;
    /// }
    ///
    /// interface PersonConstructor {
    ///     readonly MAX_FRIEND_COUNT: number;
    ///
    ///     get SPECIAL_PERSON_INSTANCE(): Person;
    ///
    ///     new(): Person;
    /// }
    /// ```
    ///
    /// Examples of an incorrect type alias:
    ///
    /// ```ts,expect_diagnostic
    /// type person = { fullName: string };
    /// ```
    ///
    /// ### Literal object member names
    ///
    /// - Literal object members are in [`camelCase`].
    ///
    /// ```js
    /// const alice = {
    ///     fullName: "Alice",
    /// }
    /// ```
    ///
    /// Example of an incorrect name:
    ///
    /// ```js,expect_diagnostic
    /// const alice = {
    ///     full_name: "Alice",
    /// }
    /// ```
    ///
    /// ### Import and export aliases and namespaces
    ///
    /// Import and export namespaces are in [`camelCase`] or [`PascalCase`].
    ///
    /// ```js
    /// import * as myLib from "my-lib";
    /// import * as Framework from "framework";
    ///
    /// export * as myLib from "my-lib";
    /// export * as Framework from "framework";
    /// ```
    ///
    /// `import` and `export` aliases are in [`camelCase`], [`PascalCase`], or [`CONSTANT_CASE`]:
    ///
    /// ```js
    /// import assert, {
    ///     deepStrictEqual as deepEqual,
    ///     AssertionError as AssertError
    /// } from "node:assert";
    /// ```
    ///
    /// Examples of an incorrect name:
    ///
    /// ```ts,expect_diagnostic
    /// import * as MY_LIB from "my-lib";
    /// ```
    ///
    /// ### TypeScript type parameter names
    ///
    /// A _TypeScript_ type parameter name is in [`PascalCase`].
    ///
    /// ```ts
    /// function id<Val>(value: Val): Val { /* ... */}
    /// ```
    ///
    /// ### TypeScript `namespace` names
    ///
    /// A _TypeScript_ `namespace` name is in [`camelCase`] or in [`PascalCase`].
    ///
    /// ```ts
    /// namespace mathExtra {
    ///     /*...*/
    /// }
    ///
    /// namespace MathExtra {
    ///     /*...*/
    /// }
    /// ```
    ///
    /// ## Ignored declarations
    ///
    /// Note that some declarations are always ignored.
    /// You cannot apply a convention to them.
    /// This is the case for:
    ///
    /// - Member names that are not identifiers
    ///
    ///   ```js
    ///   class C {
    ///     ["not an identifier"]() {}
    ///   }
    ///   ```
    ///
    /// - Named imports
    ///
    ///  ```js
    ///   import { an_IMPORT } from "mod"
    ///   ```
    ///
    /// - Destructured object properties
    ///
    ///   ```js
    ///   const { destructed_PROP } = obj;
    ///   ```
    ///
    /// - Class members marked with `override`:
    ///
    ///   ```ts
    ///   class C extends B {
    ///     override overridden_METHOD() {}
    ///   }
    ///   ```
    ///
    /// - Declarations inside an external TypeScript module
    ///
    ///   ```ts
    ///   declare module "myExternalModule" {
    ///     export interface my_INTERFACE {}
    ///   }
    ///   ```
    ///
    /// - Declarations inside a global declaration
    ///
    ///   ```ts
    ///   declare global {
    ///     interface HTMLElement {}
    ///   }
    ///   ```
    ///
    /// ## Options
    ///
    /// The rule provides several options that are detailed in the following subsections.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "strictCase": false,
    ///         "requireAscii": false,
    ///         "conventions": [
    ///             {
    ///                 "selector": {
    ///                     "kind": "classMember",
    ///                     "modifiers": ["private"]
    ///                 },
    ///                 "match": "_(.+)",
    ///                 "formats": ["camelCase"]
    ///             }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// ### strictCase
    ///
    /// When this option is set to `true`, it forbids consecutive uppercase characters in [`camelCase`] and [`PascalCase`].
    ///
    /// **Default:** `true`
    ///
    /// For instance, `HTTPServer` or `aHTTPServer` are not permitted for `strictCase: true`.
    /// These names should be renamed to `HttpServer` and `aHttpServer`:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "strictCase": true
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// class HTTPServer {
    /// }
    /// ```
    ///
    /// When `strictCase` is set to `false`, consecutive uppercase characters are allowed.
    /// For example, `HTTPServer` and `aHTTPServer` would be considered valid then:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "strictCase": false
    ///     }
    /// }
    /// ```
    ///
    /// ```js,use_options
    /// class HTTPServer {
    /// }
    /// ```
    ///
    /// ### requireAscii
    ///
    /// When `true`, names must only consist of ASCII characters only,
    /// forbidding names like `café` or `안녕하세요` that include non-ASCII characters.
    ///
    /// When `requireAscii` is set to `false`, names may include non-ASCII characters.
    /// For example, `café` and `안녕하세요` would be considered valid then.
    ///
    /// **Default:** `true`
    ///
    /// ### conventions
    ///
    /// The `conventions` option allows applying custom conventions.
    /// The option takes an array of conventions.
    /// Every convention is an object that includes an optional `selector` and one or more requirements (`match` and `formats`).
    ///
    /// For example, you can enforce the use of [`CONSTANT_CASE`] for global `const` declarations:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "conventions": [
    ///             {
    ///                 "selector": {
    ///                     "kind": "const",
    ///                     "scope": "global"
    ///                 },
    ///                 "formats": ["CONSTANT_CASE"]
    ///             }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// A selector describes which declarations the convention applies to.
    /// You can select a declaration based on several criteria:
    ///
    /// - `kind`: the kind of the declaration among:
    ///   - `any` (default kind if the kind is unset)
    ///   - `typeLike`: classes, enums, type aliases, and interfaces
    ///   - `class`
    ///   - `enum`
    ///   - `enumMember`
    ///   - `interface`
    ///   - `typeAlias`
    ///   - `function`: named function declarations and expressions
    ///   - `namespaceLike`: TypeScript namespaces, import and export namespaces (`import * as namespace from`)
    ///   - `namespace`: TypeScript namespaces
    ///   - `importNamespace`
    ///   - `exportNamespace`
    ///   - `importAlias`: default imports and aliases of named imports
    ///   - `exportAlias`: aliases of re-exported names
    ///   - `variable`: const, let, using, and var declarations
    ///   - `const`
    ///   - `let`
    ///   - `var`
    ///   - `using`
    ///   - `functionParameter`
    ///   - `catchParameter`
    ///   - `indexParameter`: parameters of index signatures
    ///   - `typeParameter`: generic type parameter
    ///   - `classMember`: class properties, parameter properties, methods, getters, and setters
    ///   - `classProperty`: class properties, including parameter properties
    ///   - `classMethod`
    ///   - `classGetter`
    ///   - `classSetter`
    ///   - `objectLiteralMember`: literal object properties, methods, getters, and setters
    ///   - `objectLiteralProperty`
    ///   - `objectLiteralMethod`
    ///   - `objectLiteralGetter`
    ///   - `objectLiteralSetter`
    ///   - `typeMember`: properties, methods, getters, and setters declared in type aliases and interfaces
    ///   - `typeProperty`
    ///   - `typeMethod`
    ///   - `typeGetter`
    ///   - `typeSetter`
    /// - `modifiers`: an array of modifiers among:
    ///   - `abstract`: applies to class members and classes
    ///   - `private`: applies to class members
    ///   - `protected`: applies to class members
    ///   - `readonly`: applies to class members and type members
    ///   - `static`: applies to class members
    /// - `scope`: where the declaration appears. Allowed values:
    ///   - `any`: anywhere (default value if the scope is unset)
    ///   - `global`: the global scope (also includes the namespace scopes)
    ///
    /// For each declaration,
    /// the `conventions` array is traversed in-order until a selector selects the declaration.
    /// The requirements of the convention are so verified on the declaration.
    ///
    /// A convention must set at least one requirement among:
    ///
    /// - `match`: a regular expression that the name of the declaration must match.
    /// - `formats`: the string [case] that the name must follow.
    ///   The supported cases are: [`PascalCase`], [`CONSTANT_CASE`], [`camelCase`], and [`snake_case`].
    ///
    /// If only `formats` is set, it's checked against the name of the declaration.
    /// In the following configuration, we require `static readonly` class properties to be in [`CONSTANT_CASE`].
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "conventions": [
    ///             {
    ///                 "selector": {
    ///                     "kind": "classProperty",
    ///                     "modifiers": ["static", "readonly"]
    ///                 },
    ///                 "formats": ["CONSTANT_CASE"]
    ///             }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// The following code is then reported by the rule:
    ///
    /// ```ts,use_options,expect_diagnostic
    /// class C {
    ///     static readonly prop = 0;
    /// }
    /// ```
    ///
    /// A convention can make another one useless.
    /// In the following configuration, the second convention is useless because the first one always applies to class members, including class properties.
    /// You should always place first more specific conventions.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "conventions": [
    ///             {
    ///                 "selector": { "kind": "classMember" },
    ///                 "formats": ["camelCase"]
    ///             },
    ///             {
    ///                 "selector": { "kind": "classProperty" },
    ///                 "formats": ["camelCase", "CONSTANT_CASE"]
    ///             }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// If only `match` is set and the regular expression has no capturing groups,
    /// then `match` is checked against the name of the declaration directly.
    /// In the following configuration, all variable names must have a minimum of 3 characters and a maximum of 20 characters.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "conventions": [
    ///             {
    ///                 "selector": { "kind": "variable" },
    ///                 "match": ".{3,20}"
    ///             }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// If both `match` and `formats` are set, then `formats` is checked against the first capture of the regular expression.
    /// Only the first capture is tested. Other captures are ignored.
    /// If nothing is captured, then `formats` is ignored.
    ///
    /// In the following example, we require that:
    ///
    /// - A private property starts with `_` and consists of at least two characters.
    /// - The captured name (the name without the leading `_`) is in [`camelCase`].
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "conventions": [
    ///             {
    ///                 "selector": { "kind": "classMember", "modifiers": ["private"] },
    ///                 "match": "_(.+)",
    ///                 "formats": ["camelCase"]
    ///             }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// If `match` is set and `formats` is unset, then the part of the name captured by the regular expression is forwarded to the next conventions of the array that selects the declaration.
    /// The following configuration has exactly the same effect as the previous one.
    /// The first convention applies to any private class member name.
    /// It stipulates that the name must have a leading underscore.
    /// The regular expression captures the part of the name without the leading underscore.
    /// Because `formats` is not set, the capture is forwarded to the next convention that applies to a private class member name.
    /// In our case, the next convention applies.
    /// The capture is then checked against `formats`.
    ///
    /// ```jsonc,options
    /// {
    ///     "options": {
    ///         "conventions": [
    ///             {
    ///                 "selector": { "kind": "classMember", "modifiers": ["private"] },
    ///                 "match": "_(.+)"
    ///                 // We don't need to specify `formats` because the capture is forwarded to the next conventions.
    ///             }, {
    ///                 "selector": { "kind": "classMember", "modifiers": ["private"] },
    ///                 "formats": ["camelCase"]
    ///             }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// The forwarding has particularly useful to factorize some conventions.
    /// For example, the following configuration...
    ///
    /// ```jsonc,options
    /// {
    ///     "options": {
    ///         "conventions": [
    ///             {
    ///                 "selector": { "kind": "classMember", "modifiers": ["private"] },
    ///                 "match": "_(.+)",
    ///                 "formats": ["camelCase"]
    ///             }, {
    ///                 "selector": { "kind": "classMember" },
    ///                 "formats": ["camelCase"]
    ///             }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// can be factorized to...
    ///
    /// ```jsonc,options
    /// {
    ///     "options": {
    ///         "conventions": [
    ///             {
    ///                 "selector": { "kind": "classMember", "modifiers": ["private"] },
    ///                 "match": "_(.+)"
    ///             }, {
    ///                 "selector": { "kind": "classMember" },
    ///                 "formats": ["camelCase"]
    ///             }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// If a declaration is not selected or if a capture is forwarded while there are no more conventions,
    /// then the declaration name is verified against the default conventions.
    /// Because the default conventions already ensure that class members are in ["camelCase"],
    /// the previous example can be simplified to:
    ///
    /// ```jsonc,options
    /// {
    ///     "options": {
    ///         "conventions": [
    ///             {
    ///                 "selector": { "kind": "classMember", "modifiers": ["private"] },
    ///                 "match": "_(.+)"
    ///                 // We don't need to specify `formats` because the capture is forwarded to the next conventions.
    ///             }
    ///             // default conventions
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// If the capture is identical to the initial name (it is not a part of the initial name),
    /// then, leading and trailing underscore and dollar signs are trimmed before being checked against default conventions.
    /// In the previous example, the capture is a part of the name because `_` is not included in the capture, thus, no trimming is performed.
    ///
    /// You can reset all default conventions by adding a convention at the end of the array that accepts anything:
    ///
    /// ```jsonc,options
    /// {
    ///     "options": {
    ///         "conventions": [
    ///             // your conventions
    ///             // ...
    ///
    ///             // Otherwise, accept anything
    ///             {
    ///                 "match": ".*"
    ///             }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// Let's take a more complex example with the following conventions:
    ///
    /// 1. A variable name is `i`, `j`, or follows the next selected convention (convention (2)).
    /// 2. An identifier contains at least two characters and follow the next selected convention (the default convention).
    /// 3. A `private` class member name starts with an underscore `_` and the name without the underscore follows the next selected convention (convention (4) for some of them, and the default convention for others).
    /// 4. A `static readonly` class property name is in [`CONSTANT_CASE`].
    /// 5. A global constant is in [`CONSTANT_CASE`] and can be enclosed by double underscores or to be named `_SPECIAL_`.
    /// 6. An interface name starts with `I`, except for interfaces ending with `Error`, and is in [`PascalCase`].
    /// 7. All other names follow the default conventions
    ///
    /// ```jsonc,options
    /// {
    ///     "options": {
    ///         "conventions": [
    ///             {
    ///                 "selector": {
    ///                     "kind": "variable"
    ///                 },
    ///                 "match": "[ij]|(.*)"
    ///             },
    ///             {
    ///                 "match": "(.{2,})"
    ///             },
    ///             {
    ///                 "selector": {
    ///                     "kind": "classMember",
    ///                     "modifiers": ["private"]
    ///                 },
    ///                 "match": "_(.*)"
    ///             }, {
    ///                 "selector": {
    ///                     "kind": "classProperty",
    ///                     "modifiers": ["static", "readonly"]
    ///                 },
    ///                 "formats": ["CONSTANT_CASE"]
    ///             }, {
    ///                 "selector": {
    ///                     "kind": "const",
    ///                     "scope": "global"
    ///                 },
    ///                 "match": "__(.+)__|_SPECIAL_|(.+)",
    ///                 "formats": ["CONSTANT_CASE"]
    ///             }, {
    ///                 "selector": {
    ///                     "kind": "interface"
    ///                 },
    ///                 "match": "I(.*)|(.*?)Error",
    ///                 "formats": ["PascalCase"]
    ///             }
    ///             // default conventions
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// Hers some examples:
    ///
    /// - A private class property named `_` is reported by the rule because it contains a single character.
    ///   According to the second convention, the name should contain at least two characters.
    /// - A variable `a_variable` is reported by the rule because it doesn't respect the default convention that forbid variable names in [`snake_case`].
    ///   The variable name is first verified against the first convention.
    ///   It is forwarded to the second convention, which is also respected, because it is neither `i` nor `j`.
    ///   The name is captured and is forwarded to the next convention.
    ///   In our case, the next convention is the default one.
    ///
    /// ### Regular expression syntax
    ///
    /// The `match` option takes a regular expression that supports the following syntaxes:
    ///
    /// - Greedy quantifiers `*`, `?`, `+`, `{n}`, `{n,m}`, `{n,}`, `{m}`
    /// - Non-greedy quantifiers `*?`, `??`, `+?`, `{n}?`, `{n,m}?`, `{n,}?`, `{m}?`
    /// - Any character matcher `.`
    /// - Character classes `[a-z]`, `[xyz]`, `[^a-z]`
    /// - Alternations `|`
    /// - Capturing groups `()`
    /// - Non-capturing groups `(?:)`
    /// - Case-insensitive groups `(?i:)` and case-sensitive groups `(?-i:)`
    /// - A limited set of escaped characters including all special characters
    ///   and regular string escape characters `\f`, `\n`, `\r`, `\t`, `\v`.
    ///   Note that you can also escape special characters using character classes.
    ///   For example, `\$` and `[$]` are two valid patterns that escape `$`.
    ///
    /// [case]: https://en.wikipedia.org/wiki/Naming_convention_(programming)#Examples_of_multiple-word_identifier_formats
    /// [`camelCase`]: https://en.wikipedia.org/wiki/Camel_case
    /// [`PascalCase`]: https://en.wikipedia.org/wiki/Camel_case
    /// [`CONSTANT_CASE`]: https://en.wikipedia.org/wiki/Snake_case
    /// [`snake_case`]: https://en.wikipedia.org/wiki/Snake_case
    pub UseNamingConvention {
        version: "1.0.0",
        name: "useNamingConvention",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("naming-convention").inspired()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseNamingConvention {
    type Query = Semantic<AnyIdentifierBindingLike>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = UseNamingConventionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();
        let name_token = node.name_token().ok()?;
        let mut name = name_token.text_trimmed();
        let mut name_range_start = 0;
        if name_token.kind() == JsSyntaxKind::JS_STRING_LITERAL {
            name_range_start += 1;
            name = &name[1..name.len() - 1];
            if name.is_empty() || !is_js_ident(name) {
                // Ignore non-identifier strings
                return None;
            }
        }
        if name.starts_with('_')
            && let Ok(binding) = &node.try_into()
            && is_unused(ctx.model(), binding)
        {
            // Always ignore unused variables prefixed with `_`.
            // This notably avoids a conflict with the `noUnusedVariables` lint rule.
            return None;
        }
        if options.require_ascii && !name.is_ascii() {
            return Some(State {
                convention_selector: Selector::default(),
                name_range: Range {
                    start: 0u16,
                    end: name.len() as u16,
                },
                suggestion: Suggestion::Ascii,
            });
        }
        let node_selector = selector_from_name(node)?;
        let mut is_not_trimmed = true;
        for convention in options
            .conventions
            .iter()
            .filter(|convention| node_selector.contains(convention.selector))
        {
            if let Some(matching) = &convention.matching {
                let Some(capture) = matching.captures(name) else {
                    return Some(State {
                        convention_selector: convention.selector,
                        name_range: Range {
                            start: name_range_start as u16,
                            end: (name_range_start + name.len()) as u16,
                        },
                        suggestion: Suggestion::Match(matching.to_string().into_boxed_str()),
                    });
                };
                if let Some(first_capture) = capture.iter().skip(1).find_map(|x| x) {
                    name_range_start += first_capture.start();
                    let captured = first_capture.as_str();
                    is_not_trimmed = name.len() == captured.len();
                    name = captured;
                    if name.is_empty() {
                        // Empty string are always valid.
                        return None;
                    }
                } else {
                    // Match without any capture implies a valid case
                    return None;
                }
            }
            if !convention.formats.is_empty() {
                let actual_case = Case::identify(name, options.strict_case);
                if (*convention.formats | Case::Uni).contains(actual_case) {
                    // Valid case
                    return None;
                }
                return Some(State {
                    convention_selector: convention.selector,
                    name_range: Range {
                        start: name_range_start as u16,
                        end: (name_range_start + name.len()) as u16,
                    },
                    suggestion: Suggestion::Formats(convention.formats),
                });
            }
        }
        let default_convention = default_convention(node_selector);
        // We only tim the name if it was not trimmed yet
        if is_not_trimmed {
            let (prefix_len, trimmed_name) = trim_underscore_dollar(name);
            name_range_start += prefix_len;
            name = trimmed_name;
        }
        let actual_case = Case::identify(name, options.strict_case);
        if (*default_convention.formats | Case::Uni).contains(actual_case) || name.is_empty() {
            // Valid case
            return None;
        }
        Some(State {
            convention_selector: default_convention.selector,
            name_range: Range {
                start: name_range_start as u16,
                end: (name_range_start + name.len()) as u16,
            },
            suggestion: Suggestion::Formats(default_convention.formats),
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let State {
            convention_selector,
            name_range,
            suggestion,
        } = state;
        let options = ctx.options();
        let node = ctx.query();
        let name_token = node.name_token().ok()?;
        let name_token_range = name_token.text_trimmed_range();
        let name = name_token.text_trimmed();
        let trimmed_info = if name_range.len() < name.len() {
            " part"
        } else {
            ""
        };
        match suggestion {
            Suggestion::Ascii => {
                Some(RuleDiagnostic::new(
                    rule_category!(),
                    name_token_range,
                    markup! {
                        "This "<Emphasis>{format_args!("{convention_selector}")}</Emphasis>" name should be in ASCII because "<Emphasis>"requireAscii"</Emphasis>" is set to `true`."
                    },
                ).note(markup! {
                    "If you want to use non-ASCII names, then set the "<Emphasis>"requireAscii"</Emphasis>" option to `false`.\nSee the rule "<Hyperlink href="https://biomejs.dev/linter/rules/use-naming-convention#options">"options"</Hyperlink>" for more details."
                }))
            }
            Suggestion::Match(regex) => {
                let name_token_range = name_token_range.add_start(TextSize::from(name_range.start as u32)).sub_end(name_token_range.len() - TextSize::from(name_range.len() as u32));
                Some(RuleDiagnostic::new(
                    rule_category!(),
                    name_token_range,
                    markup! {
                        "This "<Emphasis>{format_args!("{convention_selector}")}</Emphasis>" name"{trimmed_info}" should match the following regex "<Emphasis>"/"{regex.as_ref()}"/"</Emphasis>"."
                    },
                ))
            }
            Suggestion::Formats(expected_cases) => {
                let name_token_range = TextRange::at(name_token_range.start() + TextSize::from(name_range.start as u32), TextSize::from(name_range.len() as u32));
                if options.strict_case && (expected_cases.contains(Case::Camel) || expected_cases.contains(Case::Pascal)) {
                    let trimmed_name = &name[(name_range.start as _)..(name_range.end as _)];
                    let actual_case = Case::identify(trimmed_name, false);
                    if matches!(actual_case, Case::Camel | Case::Pascal)
                        && Case::identify(trimmed_name, true) == Case::Unknown
                    {
                        return Some(RuleDiagnostic::new(
                            rule_category!(),
                            name_token_range,
                            markup! {
                                "Two consecutive uppercase characters are not allowed in "{format_args!("{actual_case}")}" because "<Emphasis>"strictCase"</Emphasis>" is set to `true`."
                            },
                        ).note(markup! {
                            "If you want to use consecutive uppercase characters in "{format_args!("{actual_case}")}", then set the "<Emphasis>"strictCase"</Emphasis>" option to `false`.\nSee the rule "<Hyperlink href="https://biomejs.dev/linter/rules/use-naming-convention#options">"options"</Hyperlink>" for more details."
                        }));
                    }
                }
                let expected_case_names = expected_cases
                    .into_iter()
                    .map(|case| case.to_string())
                    .collect::<SmallVec<[_; 4]>>()
                    .join(" or ");
                Some(RuleDiagnostic::new(
                    rule_category!(),
                    name_token_range,
                    markup! {
                        "This "<Emphasis>{format_args!("{convention_selector}")}</Emphasis>" name"{trimmed_info}" should be in "<Emphasis>{expected_case_names}</Emphasis>"."
                    },
                ))
            }
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let State {
            name_range,
            suggestion: Suggestion::Formats(expected_cases),
            ..
        } = state
        else {
            return None;
        };
        let node = ctx.query();
        let is_declaration_file = ctx
            .source_type::<JsFileSource>()
            .language()
            .is_definition_file();
        if is_declaration_file
            && let Some(items) = node
                .syntax()
                .ancestors()
                .skip(1)
                .find_map(JsModuleItemList::cast)
        {
            // A declaration file without exports and imports is a global declaration file.
            // All types are available in every files of the project.
            // Thus, it is ok if types are not used locally.
            let is_top_level = items.parent::<TsDeclarationModule>().is_some();
            if is_top_level && items.into_iter().all(|x| x.as_any_js_statement().is_some()) {
                return None;
            }
        }
        let model = ctx.model();
        if let Some(renamable) = renamable(node, model) {
            let node = ctx.query();
            let name_token = &node.name_token().ok()?;
            // This assertion hold because only identifiers are renamable.
            debug_assert!(name_token.kind() != JsSyntaxKind::JS_STRING_LITERAL);
            let name = name_token.text_trimmed();
            let is_name_capitalized = name.chars().next().is_some_and(|c| c.is_uppercase());
            let preferred_case = if is_name_capitalized {
                // Try to preserve the capitalization by preferring cases starting with a capital letter
                expected_cases
                    .into_iter()
                    .find(|&case| Cases::from(case).contains(Case::NumberableCapital))
                    .unwrap_or(expected_cases.into_iter().next()?)
            } else {
                expected_cases.into_iter().next()?
            };
            let new_name_part =
                preferred_case.convert(&name[(name_range.start as _)..(name_range.end as _)]);
            let mut new_name =
                String::with_capacity(name.len() + new_name_part.len() - name_range.len());
            new_name.push_str(&name[..(name_range.start as _)]);
            new_name.push_str(&new_name_part);
            new_name.push_str(&name[(name_range.end as _)..]);
            if name == new_name || new_name.is_empty() {
                return None;
            }
            let mut mutation = ctx.root().begin();
            let renamed = mutation.rename_any_renamable_node(model, &renamable, &new_name[..]);
            if renamed {
                return Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Rename this symbol in "<Emphasis>{preferred_case.to_string()}</Emphasis>"." }.to_owned(),
                    mutation,
                ));
            }
        }
        None
    }
}

declare_node_union! {
    /// Ast nodes that defines a name.
    pub AnyIdentifierBindingLike =
        JsIdentifierBinding |
        JsLiteralMemberName |
        JsShorthandPropertyObjectMember |
        JsPrivateClassMemberName |
        JsLiteralExportName |
        TsIdentifierBinding |
        TsLiteralEnumMemberName |
        TsTypeParameterName
}
impl AnyIdentifierBindingLike {
    fn name_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsIdentifierBinding(binding) => binding.name_token(),
            Self::JsLiteralMemberName(member_name) => member_name.value(),
            Self::JsShorthandPropertyObjectMember(member_name) => member_name.name()?.value_token(),
            Self::JsPrivateClassMemberName(member_name) => member_name.id_token(),
            Self::JsLiteralExportName(export_name) => export_name.value(),
            Self::TsIdentifierBinding(binding) => binding.name_token(),
            Self::TsLiteralEnumMemberName(member_name) => member_name.value(),
            Self::TsTypeParameterName(type_parameter) => type_parameter.ident_token(),
        }
    }
}
impl TryFrom<&AnyIdentifierBindingLike> for AnyJsIdentifierBinding {
    type Error = ();
    fn try_from(value: &AnyIdentifierBindingLike) -> Result<Self, Self::Error> {
        match value {
            AnyIdentifierBindingLike::JsIdentifierBinding(binding) => {
                Ok(Self::JsIdentifierBinding(binding.clone()))
            }
            AnyIdentifierBindingLike::TsIdentifierBinding(binding) => {
                Ok(Self::TsIdentifierBinding(binding.clone()))
            }
            AnyIdentifierBindingLike::TsLiteralEnumMemberName(binding) => {
                Ok(Self::TsLiteralEnumMemberName(binding.clone()))
            }
            AnyIdentifierBindingLike::TsTypeParameterName(binding) => {
                Ok(Self::TsTypeParameterName(binding.clone()))
            }
            AnyIdentifierBindingLike::JsLiteralMemberName(_)
            | AnyIdentifierBindingLike::JsShorthandPropertyObjectMember(_)
            | AnyIdentifierBindingLike::JsPrivateClassMemberName(_)
            | AnyIdentifierBindingLike::JsLiteralExportName(_) => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct State {
    // Selector of the convention which is not fulfilled.
    convention_selector: Selector,
    // Range of the name where the suggestion applies
    name_range: Range<u16>,
    suggestion: Suggestion,
}

#[derive(Debug)]
pub enum Suggestion {
    /// Use only ASCII characters
    Ascii,
    /// Use a name that matches this regex
    Match(Box<str>),
    /// Use a name that follows one of these formats
    Formats(Formats),
}

fn renamable(
    node: &AnyIdentifierBindingLike,
    model: &SemanticModel,
) -> Option<AnyJsRenamableDeclaration> {
    match node {
        AnyIdentifierBindingLike::JsIdentifierBinding(binding) => {
            if binding.is_exported(model) {
                return None;
            }
            // Property parameters are also class properties.
            // Shorthand binding patterns such as `const { a_a } = x;` should not be renamed.
            // Shorthand named import specifiers such as `import { a_a } from "mod";` should not be renamed.
            if let Some(
                AnyJsBindingDeclaration::TsPropertyParameter(_)
                | AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(_)
                | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_),
            ) = binding.declaration()
            {
                return None;
            }

            Some(AnyJsRenamableDeclaration::JsIdentifierBinding(
                binding.clone(),
            ))
        }
        AnyIdentifierBindingLike::TsIdentifierBinding(binding) => {
            if binding.is_exported(model) {
                return None;
            }
            Some(AnyJsRenamableDeclaration::TsIdentifierBinding(
                binding.clone(),
            ))
        }
        _ => None,
    }
}

fn selector_from_name(js_name: &AnyIdentifierBindingLike) -> Option<Selector> {
    match js_name {
        AnyIdentifierBindingLike::JsIdentifierBinding(binding) => {
            selector_from_binding_declaration(&binding.declaration()?)
        }
        AnyIdentifierBindingLike::TsIdentifierBinding(binding) => {
            selector_from_binding_declaration(&binding.declaration()?)
        }
        AnyIdentifierBindingLike::JsLiteralMemberName(member_name) => {
            if let Some(member) = member_name.parent::<AnyJsClassMember>() {
                selector_from_class_member(&member)
            } else if let Some(member) = member_name.parent::<AnyTsTypeMember>() {
                selector_from_type_member(&member)
            } else if let Some(member) = member_name.parent::<AnyJsObjectMember>() {
                selector_from_object_member(&member)
            } else {
                None
            }
        }
        AnyIdentifierBindingLike::JsShorthandPropertyObjectMember(_) => {
            Some(Kind::ObjectLiteralProperty.into())
        }
        AnyIdentifierBindingLike::JsPrivateClassMemberName(member_name) => {
            selector_from_class_member(&member_name.parent::<AnyJsClassMember>()?)
        }
        AnyIdentifierBindingLike::JsLiteralExportName(export_name) => {
            let parent = export_name.syntax().parent()?;
            match parent.kind() {
                JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER
                | JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER => None,
                JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER => Some(Kind::ExportAlias.into()),
                JsSyntaxKind::JS_EXPORT_AS_CLAUSE => {
                    if parent.parent()?.kind() == JsSyntaxKind::JS_EXPORT_FROM_CLAUSE {
                        Some(Kind::ExportNamespace.into())
                    } else {
                        Some(Kind::ExportAlias.into())
                    }
                }
                _ => None,
            }
        }
        AnyIdentifierBindingLike::TsLiteralEnumMemberName(_) => Some(Kind::EnumMember.into()),
        AnyIdentifierBindingLike::TsTypeParameterName(_) => Some(Kind::TypeParameter.into()),
    }
}

fn selector_from_class_member(member: &AnyJsClassMember) -> Option<Selector> {
    let (kind, modifiers): (_, BitFlags<_>) = match member {
        AnyJsClassMember::JsBogusMember(_)
        | AnyJsClassMember::JsMetavariable(_)
        | AnyJsClassMember::JsConstructorClassMember(_)
        | AnyJsClassMember::TsConstructorSignatureClassMember(_)
        | AnyJsClassMember::JsEmptyClassMember(_)
        | AnyJsClassMember::JsStaticInitializationBlockClassMember(_) => return None,
        AnyJsClassMember::TsIndexSignatureClassMember(member) => {
            (Kind::ClassProperty, (&member.modifiers()).into())
        }
        AnyJsClassMember::JsGetterClassMember(getter) => {
            (Kind::ClassGetter, (&getter.modifiers()).into())
        }
        AnyJsClassMember::TsGetterSignatureClassMember(member) => {
            (Kind::ClassGetter, (&member.modifiers()).into())
        }
        AnyJsClassMember::JsMethodClassMember(member) => {
            (Kind::ClassMethod, (&member.modifiers()).into())
        }
        AnyJsClassMember::TsMethodSignatureClassMember(member) => {
            (Kind::ClassMethod, (&member.modifiers()).into())
        }
        AnyJsClassMember::JsPropertyClassMember(member) => {
            (Kind::ClassProperty, (&member.modifiers()).into())
        }
        AnyJsClassMember::TsPropertySignatureClassMember(member) => {
            (Kind::ClassProperty, (&member.modifiers()).into())
        }
        AnyJsClassMember::TsInitializedPropertySignatureClassMember(member) => {
            (Kind::ClassProperty, (&member.modifiers()).into())
        }
        AnyJsClassMember::JsSetterClassMember(member) => {
            (Kind::ClassSetter, (&member.modifiers()).into())
        }
        AnyJsClassMember::TsSetterSignatureClassMember(member) => {
            (Kind::ClassSetter, (&member.modifiers()).into())
        }
    };
    if modifiers.contains(Modifier::Override) {
        // Ignore explicitly overridden members
        None
    } else {
        Some(Selector::with_modifiers(
            kind,
            to_restricted_modifiers(modifiers),
        ))
    }
}

fn selector_from_binding_declaration(decl: &AnyJsBindingDeclaration) -> Option<Selector> {
    match decl {
            AnyJsBindingDeclaration::JsArrayBindingPatternElement(_)
            | AnyJsBindingDeclaration::JsArrayBindingPatternRestElement(_)
            | AnyJsBindingDeclaration::JsObjectBindingPatternProperty(_)
            | AnyJsBindingDeclaration::JsObjectBindingPatternRest(_) => {
                selector_from_parent_binding_pattern_declaration(&decl.parent_binding_pattern_declaration()?)
            }
            AnyJsBindingDeclaration::JsVariableDeclarator(var) => {
                selector_from_variable_declarator(var, scope_from_declaration(decl)?)
            }
            AnyJsBindingDeclaration::JsArrowFunctionExpression(_)
            | AnyJsBindingDeclaration::JsBogusParameter(_)
            | AnyJsBindingDeclaration::JsFormalParameter(_)
            | AnyJsBindingDeclaration::JsRestParameter(_) => Some(Kind::FunctionParameter.into()),
            AnyJsBindingDeclaration::JsCatchDeclaration(_) => Some(Kind::CatchParameter.into()),
            AnyJsBindingDeclaration::TsPropertyParameter(param) => {
                let modifiers: BitFlags<Modifier> = (&param.modifiers()).into();
                if modifiers.contains(Modifier::Override) {
                    // Ignore explicitly overridden members
                    None
                } else {
                    Some(Selector::with_modifiers(
                        Kind::ClassProperty,
                        to_restricted_modifiers(modifiers),
                    ))
                }
            },
            AnyJsBindingDeclaration::TsIndexSignatureParameter(member_name) => {
                if let Some(member) = member_name.parent::<>() {
                    selector_from_class_member(&member)
                } else if let Some(member) = member_name.parent::<AnyTsTypeMember>() {
                    selector_from_type_member(&member)
                } else if let Some(member) = member_name.parent::<AnyJsObjectMember>() {
                    selector_from_object_member(&member)
                } else {
                    Some(Kind::IndexParameter.into())
                }
            }
            AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_) => Some(
                Selector::with_scope(Kind::ImportNamespace, Scope::Global)),
            AnyJsBindingDeclaration::JsFunctionDeclaration(_)
            | AnyJsBindingDeclaration::JsFunctionExpression(_)
            | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_)
            | AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_)
            | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_) => {
                Some(Selector::with_scope(Kind::Function, scope_from_declaration(decl)?))
            }
            AnyJsBindingDeclaration::TsImportEqualsDeclaration(_)
            | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_)
            | AnyJsBindingDeclaration::JsNamedImportSpecifier(_) =>
                Some(Selector::with_scope(Kind::ImportAlias, Scope::Global)),
            AnyJsBindingDeclaration::TsModuleDeclaration(_) =>
                Some(Selector::with_scope(Kind::Namespace, Scope::Global)),
            AnyJsBindingDeclaration::TsTypeAliasDeclaration(_) =>
                Some(Selector::with_scope(Kind::TypeAlias, scope_from_declaration(decl)?)),
            AnyJsBindingDeclaration::JsClassDeclaration(class) => {
                Some(Selector {
                    kind: Kind::Class,
                    modifiers: if class.abstract_token().is_some() {
                        RestrictedModifier::Abstract.into()
                    } else {
                        RestrictedModifiers::default()
                    },
                    scope: scope_from_declaration(decl)?,
                })
            }
            AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(class) => {
                Some(Selector {
                    kind: Kind::Class,
                    modifiers: if class.abstract_token().is_some() {
                        RestrictedModifier::Abstract.into()
                    } else {
                        RestrictedModifiers::default()
                    },
                    scope: scope_from_declaration(decl)?,
                })
            }
            AnyJsBindingDeclaration::JsClassExpression(_) => {
                Some(Selector::with_scope(Kind::Class, scope_from_declaration(decl)?))
            }
            AnyJsBindingDeclaration::TsInterfaceDeclaration(_) =>
                Some(Selector::with_scope(Kind::Interface, scope_from_declaration(decl)?)),
            AnyJsBindingDeclaration::TsEnumDeclaration(_) =>
                Some(Selector::with_scope(Kind::Enum, scope_from_declaration(decl)?)),
            AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(_)
            | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsBogusNamedImportSpecifier(_)
            // Type parameters should be handled at call site
            | AnyJsBindingDeclaration::TsInferType(_)
            | AnyJsBindingDeclaration::TsMappedType(_)
            | AnyJsBindingDeclaration::TsTypeParameter(_)
            | AnyJsBindingDeclaration::TsEnumMember(_) => None,
            // External modules are identified by source specifiers and are out
            // of scope of this rule.
            AnyJsBindingDeclaration::TsExternalModuleDeclaration(_) => None
        }
}

fn selector_from_parent_binding_pattern_declaration(
    decl: &AnyJsBindingDeclaration,
) -> Option<Selector> {
    let scope = scope_from_declaration(decl)?;
    if let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = decl {
        selector_from_variable_declarator(declarator, scope)
    } else {
        Some(Selector::with_scope(Kind::Variable, scope))
    }
}

fn selector_from_variable_declarator(var: &JsVariableDeclarator, scope: Scope) -> Option<Selector> {
    let var_declaration = var
        .syntax()
        .ancestors()
        .find_map(AnyJsVariableDeclaration::cast)?;
    let var_kind = var_declaration.variable_kind().ok()?;
    let kind = match var_kind {
        JsVariableKind::Const => Kind::Const,
        JsVariableKind::Let => Kind::Let,
        JsVariableKind::Using => Kind::Using,
        JsVariableKind::Var => Kind::Var,
    };
    Some(Selector::with_scope(kind, scope))
}

fn selector_from_object_member(member: &AnyJsObjectMember) -> Option<Selector> {
    match member {
        AnyJsObjectMember::JsBogusMember(_)
        | AnyJsObjectMember::JsSpread(_)
        | AnyJsObjectMember::JsMetavariable(_) => None,
        AnyJsObjectMember::JsGetterObjectMember(_) => Some(Kind::ObjectLiteralGetter.into()),
        AnyJsObjectMember::JsMethodObjectMember(_) => Some(Kind::ObjectLiteralMethod.into()),
        AnyJsObjectMember::JsPropertyObjectMember(_)
        | AnyJsObjectMember::JsShorthandPropertyObjectMember(_) => {
            Some(Kind::ObjectLiteralProperty.into())
        }
        AnyJsObjectMember::JsSetterObjectMember(_) => Some(Kind::ObjectLiteralSetter.into()),
    }
}

fn selector_from_type_member(member: &AnyTsTypeMember) -> Option<Selector> {
    match member {
        AnyTsTypeMember::JsBogusMember(_)
        | AnyTsTypeMember::TsCallSignatureTypeMember(_)
        | AnyTsTypeMember::TsConstructSignatureTypeMember(_) => None,
        AnyTsTypeMember::TsIndexSignatureTypeMember(property) => {
            Some(if property.readonly_token().is_some() {
                Selector::with_modifiers(Kind::IndexParameter, RestrictedModifier::Readonly)
            } else {
                Kind::IndexParameter.into()
            })
        }
        AnyTsTypeMember::TsGetterSignatureTypeMember(_) => Some(Kind::TypeGetter.into()),
        AnyTsTypeMember::TsMethodSignatureTypeMember(_) => Some(Kind::TypeMethod.into()),
        AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
            Some(if property.readonly_token().is_some() {
                Selector::with_modifiers(Kind::TypeProperty, RestrictedModifier::Readonly)
            } else {
                Kind::TypeProperty.into()
            })
        }
        AnyTsTypeMember::TsSetterSignatureTypeMember(_) => Some(Kind::TypeSetter.into()),
    }
}

/// Returns the list of default [Case] for `self`.
/// The preferred case comes first in the list.
fn default_convention(selector: Selector) -> Convention {
    let kind = selector.kind;
    match kind {
        Kind::TypeProperty if selector.modifiers.contains(RestrictedModifier::Readonly) => {
            Convention {
                selector: Selector::with_modifiers(selector.kind, RestrictedModifier::Readonly),
                matching: None,
                formats: (Case::Camel | Case::Constant).into(),
            }
        }
        Kind::TypeGetter => Convention {
            selector: kind.into(),
            matching: None,
            formats: (Case::Camel | Case::Constant).into(),
        },
        Kind::Function if Scope::Global.contains(selector.scope) => Convention {
            selector: Selector::with_scope(kind, Scope::Global),
            matching: None,
            formats: (Case::Camel | Case::Pascal | Case::Upper).into(),
        },
        Kind::Variable | Kind::Const | Kind::Var if Scope::Global.contains(selector.scope) => {
            Convention {
                selector: Selector::with_scope(kind, Scope::Global),
                matching: None,
                formats: (Case::Camel | Case::Pascal | Case::Constant).into(),
            }
        }
        Kind::Any | Kind::ExportAlias | Kind::ImportAlias => Convention {
            selector: kind.into(),
            matching: None,
            formats: (Case::Camel | Case::Pascal | Case::Constant).into(),
        },
        Kind::ClassProperty | Kind::ClassGetter
            if selector.modifiers.contains(RestrictedModifier::Static) =>
        {
            Convention {
                selector: Selector::with_modifiers(kind, RestrictedModifier::Static),
                matching: None,
                formats: (Case::Camel | Case::Constant).into(),
            }
        }
        Kind::CatchParameter
        | Kind::ClassGetter
        | Kind::ClassMember
        | Kind::ClassMethod
        | Kind::ClassProperty
        | Kind::ClassSetter
        | Kind::IndexParameter
        | Kind::ObjectLiteralGetter
        | Kind::ObjectLiteralProperty
        | Kind::ObjectLiteralMember
        | Kind::ObjectLiteralMethod
        | Kind::ObjectLiteralSetter
        | Kind::TypeMember
        | Kind::TypeMethod
        | Kind::TypeProperty
        | Kind::TypeSetter
        | Kind::Using => Convention {
            selector: kind.into(),
            matching: None,
            formats: (Cases::from(Case::Camel)).into(),
        },
        Kind::TypeLike
        | Kind::Class
        | Kind::Enum
        | Kind::Interface
        | Kind::TypeAlias
        | Kind::TypeParameter => Convention {
            selector: kind.into(),
            matching: None,
            formats: Cases::from(Case::Pascal).into(),
        },
        Kind::EnumMember => Convention {
            selector: kind.into(),
            matching: None,
            formats: Cases::from(Case::Pascal).into(),
        },
        Kind::Variable | Kind::Const | Kind::Var | Kind::Let => Convention {
            selector: kind.into(),
            matching: None,
            formats: (Case::Camel | Case::Pascal).into(),
        },
        Kind::Function
        | Kind::ExportNamespace
        | Kind::ImportNamespace
        | Kind::Namespace
        | Kind::NamespaceLike
        | Kind::FunctionParameter => Convention {
            selector: kind.into(),
            matching: None,
            formats: (Case::Camel | Case::Pascal).into(),
        },
    }
}

/// Returns the scope of `node` or `None` if the scope cannot be determined or
/// if the scope is an external module.
fn scope_from_declaration(node: &AnyJsBindingDeclaration) -> Option<Scope> {
    let control_flow_root = node.syntax().ancestors().skip(1).find(|x| {
        AnyJsControlFlowRoot::can_cast(x.kind())
            || x.kind() == JsSyntaxKind::TS_DECLARATION_MODULE
            || x.kind() == JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION
            || x.kind() == JsSyntaxKind::TS_GLOBAL_DECLARATION
    })?;
    match control_flow_root.kind() {
        JsSyntaxKind::JS_MODULE
        | JsSyntaxKind::JS_SCRIPT
        | JsSyntaxKind::TS_DECLARATION_MODULE
        | JsSyntaxKind::TS_MODULE_DECLARATION => Some(Scope::Global),
        // Ignore declarations in external module declaration and global declarations.
        JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION | JsSyntaxKind::TS_GLOBAL_DECLARATION => None,
        _ => Some(Scope::Any),
    }
}

/// trim underscores and dollar signs from `name` and returns the length of the trimmed prefix.
fn trim_underscore_dollar(name: &str) -> (usize, &str) {
    let prefix_len = name
        .bytes()
        .take_while(|c| matches!(c, b'_' | b'$'))
        .count();
    let name = &name[prefix_len..];
    let suffix_len = name
        .bytes()
        .rev()
        .take_while(|c| matches!(c, b'_' | b'$'))
        .count();
    let name = &name[..(name.len() - suffix_len)];
    (prefix_len, name)
}

fn to_restricted_modifiers(bitflag: enumflags2::BitFlags<Modifier>) -> RestrictedModifiers {
    bitflag
        .into_iter()
        .filter_map(|modifier| match modifier {
            Modifier::Private => Some(RestrictedModifier::Private),
            Modifier::Protected => Some(RestrictedModifier::Protected),
            Modifier::Static => Some(RestrictedModifier::Static),
            Modifier::Abstract => Some(RestrictedModifier::Abstract),
            Modifier::Readonly => Some(RestrictedModifier::Readonly),
            Modifier::Decorator
            | Modifier::BogusAccessibility
            | Modifier::Public
            | Modifier::Declare
            | Modifier::Override
            | Modifier::Accessor => None,
        })
        .collect()
}
