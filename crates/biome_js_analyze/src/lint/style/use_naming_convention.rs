use std::ops::{Deref, Range};

use crate::{
    services::{control_flow::AnyJsControlFlowRoot, semantic::Semantic},
    utils::{
        rename::{AnyJsRenamableDeclaration, RenameSymbolExtensions},
        restricted_regex::RestrictedRegex,
    },
    JsRuleAction,
};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_deserialize::{
    DeserializableValidator, DeserializationContext, DeserializationDiagnostic,
};
use biome_deserialize_macros::Deserializable;
use biome_js_semantic::{CanBeImportedExported, SemanticModel};
use biome_js_syntax::{
    binding_ext::AnyJsBindingDeclaration, AnyJsClassMember, AnyJsObjectMember,
    AnyJsVariableDeclaration, AnyTsTypeMember, JsFileSource, JsIdentifierBinding,
    JsLiteralExportName, JsLiteralMemberName, JsMethodModifierList, JsModuleItemList,
    JsPrivateClassMemberName, JsPropertyModifierList, JsSyntaxKind, JsSyntaxToken,
    JsVariableDeclarator, JsVariableKind, Modifier, TsDeclarationModule, TsIdentifierBinding,
    TsIndexSignatureModifierList, TsLiteralEnumMemberName, TsMethodSignatureModifierList,
    TsPropertySignatureModifierList, TsTypeParameterName,
};
use biome_rowan::{
    declare_node_union, AstNode, BatchMutationExt, SyntaxResult, TextRange, TextSize,
};
use biome_string_case::{Case, Cases};
use biome_unicode_table::is_js_ident;
use enumflags2::BitFlags;
use smallvec::SmallVec;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

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
    /// All names can be prefixed and suffixed by underscores `_` and dollar signs `$`.
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
    ///   :::caution
    ///   **Bug:** Declarations inside external TypeScript modules are currently not ignored.
    ///   This is a bug, and is tracked under [#4545](https://github.com/biomejs/biome/issues/4545).
    ///
    ///   Until the bug is fixed, we recommend one of the following workarounds:
    ///
    ///   - Move the type declarations for external modules into separate `.d.ts` files,
    ///     and use [overrides](https://biomejs.dev/reference/configuration/#overrides)
    ///     in your [`biome.json`](https://biomejs.dev/reference/configuration/)
    ///     to disable the `useNamingConvention` rule for those files:
    ///
    ///     ```jsonc,full_options
    ///     {
    ///       "linter": {
    ///         "rules": {
    ///           "style": {
    ///             "useNamingConvention": "warn"
    ///           }
    ///           // ...
    ///         }
    ///       },
    ///       // ...
    ///       "overrides": [
    ///         {
    ///           "includes": ["typings/*.d.ts"],
    ///           "linter": {
    ///             "rules": {
    ///               "style": {
    ///                 "useNamingConvention": "off"
    ///               }
    ///             }
    ///           }
    ///         }
    ///       ]
    ///     }
    ///     ```
    ///
    ///   - Use [`// biome-ignore lint/style/useNamingConvention: <explanation>`](https://biomejs.dev/linter/#ignore-code)
    ///     to ignore the problematic lines.
    ///   :::
    ///
    ///   ```ts,ignore
    ///   declare module "myExternalModule" {
    ///     export interface my_INTERFACE {}
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
    /// ### conventions (Since v1.8.0)
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
    /// the `conventions` array is traversed until a selector selects the declaration.
    /// The requirements of the convention are so verified on the declaration.
    ///
    /// A convention must set at least one requirement among:
    ///
    /// - `match`: a regular expression that the name of the declaration must match.
    /// - `formats`: the string [case] that the name must follow.
    ///   The supported cases are: [`PascalCase`], [`CONSTANT_CASE`], [`camelCase`], and [`snake_case`].
    ///
    /// If both `match` and `formats` are set, then `formats` is checked against the first capture of the regular expression.
    /// Only the first capture is tested. Other captures are ignored.
    /// If nothing is captured, then `formats` is ignored.
    ///
    /// In the following example, we check the following conventions:
    ///
    /// - A private property starts with `_` and consists of at least two characters.
    /// - The captured name (the name without the leading `_`) is in [`camelCase`].
    /// - An enum member is in [`PascalCase`] or [`CONSTANT_CASE`].
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "conventions": [
    ///             {
    ///                 "selector": {
    ///                     "kind": "classMember",
    ///                     "modifiers": ["private"]
    ///                 },
    ///                 "match": "_(.+)",
    ///                 "formats": ["camelCase"]
    ///             },
    ///             {
    ///                 "selector": {
    ///                     "kind": "enumMember"
    ///                 },
    ///                 "formats": ["PascalCase", "CONSTANT_CASE"]
    ///             }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// If `match` is set and `formats` is unset,
    /// then the part of the name captured by the regular expression is forwarded to the next conventions of the array.
    /// In the following example, we require that private class members start with `_` and all class members are in ["camelCase"].
    ///
    /// ```jsonc,options
    /// {
    ///     "options": {
    ///         "conventions": [
    ///             {
    ///                 "selector": {
    ///                     "kind": "classMember",
    ///                     "modifiers": ["private"]
    ///                 },
    ///                 "match": "_(.+)"
    ///                 // We don't need to specify `formats` because the capture is forwarded to the next conventions.
    ///             },
    ///             {
    ///                 "selector": {
    ///                     "kind": "classMember"
    ///                 },
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
    ///                 "selector": {
    ///                     "kind": "classMember",
    ///                     "modifiers": ["private"]
    ///                 },
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
    /// In the previous example, the capture is a part of the name because `_` is not included in the capture.
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
    /// - Accept variable names `i`, `j`, and check all other names against the next conventions.
    /// - All identifiers must contain at least two characters.
    /// - We require `private` class members to start with an underscore `_`.
    /// - We require `static readonly` class properties to be in [`CONSTANT_CASE`].
    ///   A `private static readonly` property must also start with an underscore as dictated by the previous convention.
    /// - We require global constants to be in [`CONSTANT_CASE`] and
    ///   we allow these constants to be enclosed by double underscores or to be named `_SPECIAL_`.
    /// - We require interfaces to start with `I`, except for interfaces ending with `Error`,
    ///   and to be in [`PascalCase`].
    /// - All other names follow the default conventions
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
    ///                 "match": "_(.+)"
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
        sources: &[RuleSource::EslintTypeScript("naming-convention")],
        source_kind: RuleSourceKind::Inspired,
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseNamingConvention {
    type Query = Semantic<AnyIdentifierBindingLike>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NamingConventionOptions;

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
        let node_selector = Selector::from_name(node)?;
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
        let default_convention = node_selector.default_convention();
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
            },
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
        if is_declaration_file {
            if let Some(items) = node
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
            if name == new_name {
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
        JsPrivateClassMemberName |
        JsLiteralExportName |
        TsIdentifierBinding |
        TsLiteralEnumMemberName |
        TsTypeParameterName
}

impl AnyIdentifierBindingLike {
    fn name_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyIdentifierBindingLike::JsIdentifierBinding(binding) => binding.name_token(),
            AnyIdentifierBindingLike::JsLiteralMemberName(member_name) => member_name.value(),
            AnyIdentifierBindingLike::JsPrivateClassMemberName(member_name) => {
                member_name.id_token()
            }
            AnyIdentifierBindingLike::JsLiteralExportName(export_name) => export_name.value(),
            AnyIdentifierBindingLike::TsIdentifierBinding(binding) => binding.name_token(),
            AnyIdentifierBindingLike::TsLiteralEnumMemberName(member_name) => member_name.value(),
            AnyIdentifierBindingLike::TsTypeParameterName(type_parameter) => {
                type_parameter.ident_token()
            }
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

/// Rule's options.
#[derive(Debug, Clone, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NamingConventionOptions {
    /// If `false`, then consecutive uppercase are allowed in _camel_ and _pascal_ cases.
    /// This does not affect other [Case].
    #[serde(default = "enabled", skip_serializing_if = "bool::clone")]
    pub strict_case: bool,

    /// If `false`, then non-ASCII characters are allowed.
    #[serde(default = "enabled", skip_serializing_if = "bool::clone")]
    pub require_ascii: bool,

    /// Custom conventions.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub conventions: Box<[Convention]>,
}
impl Default for NamingConventionOptions {
    fn default() -> Self {
        Self {
            strict_case: true,
            require_ascii: true,
            conventions: Vec::new().into_boxed_slice(),
        }
    }
}

const fn enabled() -> bool {
    true
}
fn is_default<T: Default + Eq>(value: &T) -> bool {
    value == &T::default()
}

#[derive(
    Clone, Debug, Default, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(deny_unknown_fields)]
#[deserializable(with_validator)]
pub struct Convention {
    /// Declarations concerned by this convention
    #[serde(default, skip_serializing_if = "is_default")]
    pub selector: Selector,

    /// Regular expression to enforce
    #[serde(default, rename = "match", skip_serializing_if = "Option::is_none")]
    pub matching: Option<RestrictedRegex>,

    /// String cases to enforce
    #[serde(default, skip_serializing_if = "is_default")]
    pub formats: Formats,
}

impl DeserializableValidator for Convention {
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        _name: &str,
        range: biome_rowan::TextRange,
    ) -> bool {
        if self.formats.is_empty() && self.matching.is_none() {
            ctx.report(
                DeserializationDiagnostic::new(
                    "At least one field among `formats` and `match` must be set.",
                )
                .with_range(range),
            );
            false
        } else {
            true
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum InvalidSelector {
    IncompatibleModifiers(Modifier, Modifier),
    UnsupportedModifiers(Kind, Modifier),
    UnsupportedScope(Kind, Scope),
}
impl std::error::Error for InvalidSelector {}
impl std::fmt::Display for InvalidSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidSelector::IncompatibleModifiers(modifier1, modifier2) => {
                write!(
                    f,
                    "The `{modifier1}` and `{modifier2}` modifiers cannot be used together.",
                )
            }
            InvalidSelector::UnsupportedModifiers(kind, modifier) => {
                write!(
                    f,
                    "The `{modifier}` modifier cannot be used with the `{kind}` kind."
                )
            }
            InvalidSelector::UnsupportedScope(kind, scope) => {
                let scope = scope.to_string();
                let scope = scope.trim_end();
                write!(
                    f,
                    "The `{scope}` scope cannot be used with the `{kind}` kind."
                )
            }
        }
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[deserializable(with_validator)]
#[serde(deny_unknown_fields)]
pub struct Selector {
    /// Declaration kind
    #[serde(default, skip_serializing_if = "is_default")]
    pub kind: Kind,

    /// Modifiers used on the declaration
    #[serde(default, skip_serializing_if = "is_default")]
    pub modifiers: Modifiers,

    /// Scope of the declaration
    #[serde(default, skip_serializing_if = "is_default")]
    pub scope: Scope,
}

impl Selector {
    /// Returns an error if the current selector is not valid.
    pub fn check(self) -> Result<(), InvalidSelector> {
        if self.modifiers.contains(Modifier::Abstract) {
            if self.kind != Kind::Class && !Kind::ClassMember.contains(self.kind) {
                return Err(InvalidSelector::UnsupportedModifiers(
                    self.kind,
                    Modifier::Abstract,
                ));
            }
            if self.modifiers.contains(Modifier::Static) {
                return Err(InvalidSelector::IncompatibleModifiers(
                    Modifier::Abstract,
                    Modifier::Static,
                ));
            }
        }
        if self.modifiers.contains(Modifier::Readonly)
            && !matches!(
                self.kind,
                Kind::ClassProperty | Kind::IndexParameter | Kind::TypeProperty
            )
        {
            return Err(InvalidSelector::UnsupportedModifiers(
                self.kind,
                Modifier::Readonly,
            ));
        }
        if self.modifiers.intersects(Modifier::CLASS_MEMBER_ONLY)
            && !Kind::ClassMember.contains(self.kind)
        {
            let modifiers = self.modifiers.0 & Modifier::CLASS_MEMBER_ONLY;
            if let Some(modifier) = modifiers.iter().next() {
                return Err(InvalidSelector::UnsupportedModifiers(self.kind, modifier));
            }
        }
        // The rule doesn't allow `Modifier::Public`.
        // So we only need to check for `Modifier::Private`/`Modifier::Protected` incompatibility.
        let accessibility = Modifier::Private | Modifier::Protected;
        if *self.modifiers & accessibility == accessibility {
            return Err(InvalidSelector::IncompatibleModifiers(
                Modifier::Private,
                Modifier::Protected,
            ));
        }
        let abstarct_or_static = Modifier::Abstract | Modifier::Static;
        if *self.modifiers & abstarct_or_static == abstarct_or_static {
            return Err(InvalidSelector::IncompatibleModifiers(
                Modifier::Abstract,
                Modifier::Static,
            ));
        }
        if self.scope == Scope::Global
            && !Kind::Variable.contains(self.kind)
            && !Kind::Function.contains(self.kind)
            && !Kind::TypeLike.contains(self.kind)
        {
            return Err(InvalidSelector::UnsupportedScope(self.kind, Scope::Global));
        }
        Ok(())
    }
}

impl DeserializableValidator for Selector {
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        _name: &str,
        range: biome_rowan::TextRange,
    ) -> bool {
        if let Err(error) = self.check() {
            ctx.report(DeserializationDiagnostic::new(format_args!("{error}")).with_range(range));
            return false;
        }
        true
    }
}

impl From<Kind> for Selector {
    fn from(kind: Kind) -> Self {
        Self {
            kind,
            modifiers: Modifiers::default(),
            scope: Scope::Any,
        }
    }
}
impl std::fmt::Display for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.scope, self.modifiers, self.kind)
    }
}
impl Selector {
    fn with_modifiers(kind: Kind, modifiers: impl Into<Modifiers>) -> Self {
        Self {
            kind,
            modifiers: modifiers.into(),
            ..Default::default()
        }
    }

    fn with_scope(kind: Kind, scope: Scope) -> Self {
        Self {
            kind,
            scope,
            ..Default::default()
        }
    }

    fn from_name(js_name: &AnyIdentifierBindingLike) -> Option<Selector> {
        match js_name {
            AnyIdentifierBindingLike::JsIdentifierBinding(binding) => {
                Selector::from_binding_declaration(&binding.declaration()?)
            }
            AnyIdentifierBindingLike::TsIdentifierBinding(binding) => {
                Selector::from_binding_declaration(&binding.declaration()?)
            }
            AnyIdentifierBindingLike::JsLiteralMemberName(member_name) => {
                if let Some(member) = member_name.parent::<AnyJsClassMember>() {
                    Selector::from_class_member(&member)
                } else if let Some(member) = member_name.parent::<AnyTsTypeMember>() {
                    Selector::from_type_member(&member)
                } else if let Some(member) = member_name.parent::<AnyJsObjectMember>() {
                    Selector::from_object_member(&member)
                } else {
                    None
                }
            }
            AnyIdentifierBindingLike::JsPrivateClassMemberName(member_name) => {
                Selector::from_class_member(&member_name.parent::<AnyJsClassMember>()?)
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

    fn from_class_member(member: &AnyJsClassMember) -> Option<Selector> {
        let Selector {
            kind,
            modifiers,
            scope,
        } = match member {
            AnyJsClassMember::JsBogusMember(_)
            | AnyJsClassMember::JsMetavariable(_)
            | AnyJsClassMember::JsConstructorClassMember(_)
            | AnyJsClassMember::TsConstructorSignatureClassMember(_)
            | AnyJsClassMember::JsEmptyClassMember(_)
            | AnyJsClassMember::JsStaticInitializationBlockClassMember(_) => return None,
            AnyJsClassMember::TsIndexSignatureClassMember(getter) => {
                Selector::with_modifiers(Kind::IndexParameter, getter.modifiers())
            }
            AnyJsClassMember::JsGetterClassMember(getter) => {
                Selector::with_modifiers(Kind::ClassGetter, getter.modifiers())
            }
            AnyJsClassMember::TsGetterSignatureClassMember(getter) => {
                Selector::with_modifiers(Kind::ClassGetter, getter.modifiers())
            }
            AnyJsClassMember::JsMethodClassMember(method) => {
                Selector::with_modifiers(Kind::ClassMethod, method.modifiers())
            }
            AnyJsClassMember::TsMethodSignatureClassMember(method) => {
                Selector::with_modifiers(Kind::ClassMethod, method.modifiers())
            }
            AnyJsClassMember::JsPropertyClassMember(property) => {
                Selector::with_modifiers(Kind::ClassProperty, property.modifiers())
            }
            AnyJsClassMember::TsPropertySignatureClassMember(property) => {
                Selector::with_modifiers(Kind::ClassProperty, property.modifiers())
            }
            AnyJsClassMember::TsInitializedPropertySignatureClassMember(property) => {
                Selector::with_modifiers(Kind::ClassProperty, property.modifiers())
            }
            AnyJsClassMember::JsSetterClassMember(setter) => {
                Selector::with_modifiers(Kind::ClassSetter, setter.modifiers())
            }
            AnyJsClassMember::TsSetterSignatureClassMember(setter) => {
                Selector::with_modifiers(Kind::ClassSetter, setter.modifiers())
            }
        };
        // Ignore explicitly overridden members
        (!modifiers.contains(Modifier::Override)).then_some(Selector {
            kind,
            modifiers,
            scope,
        })
    }

    fn from_binding_declaration(decl: &AnyJsBindingDeclaration) -> Option<Selector> {
        match decl {
            AnyJsBindingDeclaration::JsArrayBindingPatternElement(_)
            | AnyJsBindingDeclaration::JsArrayBindingPatternRestElement(_)
            | AnyJsBindingDeclaration::JsObjectBindingPatternProperty(_)
            | AnyJsBindingDeclaration::JsObjectBindingPatternRest(_) => {
                Self::from_parent_binding_pattern_declaration(&decl.parent_binding_pattern_declaration()?)
            }
            AnyJsBindingDeclaration::JsVariableDeclarator(var) => {
                Selector::from_variable_declarator(var, Scope::from_declaration(decl)?)
            }
            AnyJsBindingDeclaration::JsArrowFunctionExpression(_)
            | AnyJsBindingDeclaration::JsBogusParameter(_)
            | AnyJsBindingDeclaration::JsFormalParameter(_)
            | AnyJsBindingDeclaration::JsRestParameter(_) => Some(Kind::FunctionParameter.into()),
            AnyJsBindingDeclaration::JsCatchDeclaration(_) => Some(Kind::CatchParameter.into()),
            AnyJsBindingDeclaration::TsPropertyParameter(_) => Some(Kind::ClassProperty.into()),
            AnyJsBindingDeclaration::TsIndexSignatureParameter(member_name) => {
                if let Some(member) = member_name.parent::<>() {
                    Selector::from_class_member(&member)
                } else if let Some(member) = member_name.parent::<AnyTsTypeMember>() {
                    Selector::from_type_member(&member)
                } else if let Some(member) = member_name.parent::<AnyJsObjectMember>() {
                    Selector::from_object_member(&member)
                } else {
                    Some(Kind::IndexParameter.into())
                }
            },
            AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_) => Some(Selector::with_scope(Kind::ImportNamespace, Scope::Global)),
            AnyJsBindingDeclaration::JsFunctionDeclaration(_)
            | AnyJsBindingDeclaration::JsFunctionExpression(_)
            | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_)
            | AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_)
            | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_) => {
                Some(Selector::with_scope(Kind::Function, Scope::from_declaration(decl)?))
            }
            AnyJsBindingDeclaration::TsImportEqualsDeclaration(_)
            | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_)
            | AnyJsBindingDeclaration::JsNamedImportSpecifier(_) => Some(Selector::with_scope(Kind::ImportAlias, Scope::Global)),
            AnyJsBindingDeclaration::TsModuleDeclaration(_) => Some(Selector::with_scope(Kind::Namespace, Scope::Global)),
            AnyJsBindingDeclaration::TsTypeAliasDeclaration(_) => Some(Selector::with_scope(Kind::TypeAlias, Scope::from_declaration(decl)?)),
            AnyJsBindingDeclaration::JsClassDeclaration(class) => {
                Some(Selector {
                    kind: Kind::Class,
                    modifiers: if class.abstract_token().is_some() {
                        Modifier::Abstract.into()
                    } else {
                        Modifiers::default()
                    },
                    scope: Scope::from_declaration(decl)?,
                })
            }
            AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(class) => {
                Some(Selector {
                    kind: Kind::Class,
                    modifiers: if class.abstract_token().is_some() {
                        Modifier::Abstract.into()
                    } else {
                        Modifiers::default()
                    },
                    scope: Scope::from_declaration(decl)?,
                })
            }
            AnyJsBindingDeclaration::JsClassExpression(_) => {
                Some(Selector::with_scope(Kind::Class, Scope::from_declaration(decl)?))
            }
            AnyJsBindingDeclaration::TsInterfaceDeclaration(_) => Some(Selector::with_scope(Kind::Interface, Scope::from_declaration(decl)?)),
            AnyJsBindingDeclaration::TsEnumDeclaration(_) => Some(Selector::with_scope(Kind::Enum, Scope::from_declaration(decl)?)),
            AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(_)
            | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsBogusNamedImportSpecifier(_)
            // Type parameters should be handled at call site
            | AnyJsBindingDeclaration::TsInferType(_)
            | AnyJsBindingDeclaration::TsMappedType(_)
            | AnyJsBindingDeclaration::TsTypeParameter(_)
            | AnyJsBindingDeclaration::TsEnumMember(_) => None,
        }
    }

    fn from_parent_binding_pattern_declaration(decl: &AnyJsBindingDeclaration) -> Option<Selector> {
        let scope = Scope::from_declaration(decl)?;
        if let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = decl {
            Selector::from_variable_declarator(declarator, scope)
        } else {
            Some(Selector::with_scope(Kind::Variable, scope))
        }
    }

    fn from_variable_declarator(var: &JsVariableDeclarator, scope: Scope) -> Option<Selector> {
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

    fn from_object_member(member: &AnyJsObjectMember) -> Option<Selector> {
        match member {
            AnyJsObjectMember::JsBogusMember(_) | AnyJsObjectMember::JsSpread(_) => None,
            AnyJsObjectMember::JsGetterObjectMember(_) => Some(Kind::ObjectLiteralGetter.into()),
            AnyJsObjectMember::JsMethodObjectMember(_) => Some(Kind::ObjectLiteralMethod.into()),
            AnyJsObjectMember::JsPropertyObjectMember(_)
            | AnyJsObjectMember::JsShorthandPropertyObjectMember(_) => {
                Some(Kind::ObjectLiteralProperty.into())
            }
            AnyJsObjectMember::JsSetterObjectMember(_) => Some(Kind::ObjectLiteralSetter.into()),
        }
    }

    fn from_type_member(member: &AnyTsTypeMember) -> Option<Selector> {
        match member {
            AnyTsTypeMember::JsBogusMember(_)
            | AnyTsTypeMember::TsCallSignatureTypeMember(_)
            | AnyTsTypeMember::TsConstructSignatureTypeMember(_) => None,
            AnyTsTypeMember::TsIndexSignatureTypeMember(property) => {
                Some(if property.readonly_token().is_some() {
                    Selector::with_modifiers(Kind::IndexParameter, Modifier::Readonly)
                } else {
                    Kind::IndexParameter.into()
                })
            }
            AnyTsTypeMember::TsGetterSignatureTypeMember(_) => Some(Kind::TypeGetter.into()),
            AnyTsTypeMember::TsMethodSignatureTypeMember(_) => Some(Kind::TypeMethod.into()),
            AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
                Some(if property.readonly_token().is_some() {
                    Selector::with_modifiers(Kind::TypeProperty, Modifier::Readonly)
                } else {
                    Kind::TypeProperty.into()
                })
            }
            AnyTsTypeMember::TsSetterSignatureTypeMember(_) => Some(Kind::TypeSetter.into()),
        }
    }

    /// Returns the list of default [Case] for `self`.
    /// The preferred case comes first in the list.
    fn default_convention(self) -> Convention {
        let kind = self.kind;
        match kind {
            Kind::TypeProperty if self.modifiers.contains(Modifier::Readonly) => Convention {
                selector: Selector::with_modifiers(self.kind, Modifier::Readonly),
                matching: None,
                formats: Formats(Case::Camel | Case::Constant),
            },
            Kind::TypeGetter => Convention {
                selector: kind.into(),
                matching: None,
                formats: Formats(Case::Camel | Case::Constant),
            },
            Kind::Function if Scope::Global.contains(self.scope) => Convention {
                selector: Selector::with_scope(kind, Scope::Global),
                matching: None,
                formats: Formats(Case::Camel | Case::Pascal | Case::Upper),
            },
            Kind::Variable | Kind::Const | Kind::Var if Scope::Global.contains(self.scope) => {
                Convention {
                    selector: Selector::with_scope(kind, Scope::Global),
                    matching: None,
                    formats: Formats(Case::Camel | Case::Pascal | Case::Constant),
                }
            }
            Kind::Any | Kind::ExportAlias | Kind::ImportAlias => Convention {
                selector: kind.into(),
                matching: None,
                formats: Formats(Case::Camel | Case::Pascal | Case::Constant),
            },
            Kind::ClassProperty | Kind::ClassGetter
                if self.modifiers.contains(Modifier::Static) =>
            {
                Convention {
                    selector: Selector::with_modifiers(kind, Modifier::Static),
                    matching: None,
                    formats: Formats(Case::Camel | Case::Constant),
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
                formats: Formats(Case::Camel.into()),
            },
            Kind::TypeLike
            | Kind::Class
            | Kind::Enum
            | Kind::Interface
            | Kind::TypeAlias
            | Kind::TypeParameter => Convention {
                selector: kind.into(),
                matching: None,
                formats: Formats(Case::Pascal.into()),
            },
            Kind::EnumMember => Convention {
                selector: kind.into(),
                matching: None,
                formats: Formats(Case::Pascal.into()),
            },
            Kind::Variable | Kind::Const | Kind::Var | Kind::Let => Convention {
                selector: kind.into(),
                matching: None,
                formats: Formats(Case::Camel | Case::Pascal),
            },
            Kind::Function
            | Kind::ExportNamespace
            | Kind::ImportNamespace
            | Kind::Namespace
            | Kind::NamespaceLike
            | Kind::FunctionParameter => Convention {
                selector: kind.into(),
                matching: None,
                formats: Formats(Case::Camel | Case::Pascal),
            },
        }
    }

    fn contains(&self, other: Selector) -> bool {
        other.kind.contains(self.kind)
            && self.modifiers.contains(other.modifiers.0)
            && other.scope.contains(self.scope)
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserializable,
    Eq,
    Hash,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum Kind {
    /// All kinds
    #[default]
    Any,
    /// All type definitions: classes, enums, interfaces, and type aliases
    TypeLike,
    Class,
    Enum,
    /// Named function declarations and expressions
    Function,
    Interface,
    EnumMember,
    /// TypeScript namespaces, import and export namespaces
    NamespaceLike,
    /// TypeScript mamespaces
    Namespace,
    ImportNamespace,
    ExportNamespace,
    // All variable declaration: const, let, using, var
    Variable,
    Const,
    Let,
    Using,
    Var,
    /// All function parameters, but parameter properties
    FunctionParameter,
    CatchParameter,
    IndexParameter,
    /// All generic type parameters
    TypeParameter,
    // All re-export default exports and aliases of re-exported names
    ExportAlias,
    // All default imports and aliases of named imports
    ImportAlias,
    /// All class members: properties, methods, getters, and setters
    ClassMember,
    /// All class properties, including parameter properties
    ClassProperty,
    ClassGetter,
    ClassSetter,
    ClassMethod,
    /// All object literal members: properties, methods, getters, and setters
    ObjectLiteralMember,
    ObjectLiteralProperty,
    ObjectLiteralGetter,
    ObjectLiteralSetter,
    ObjectLiteralMethod,
    TypeAlias,
    /// All members defined in type alaises and interfaces
    TypeMember,
    /// All getters defined in type alaises and interfaces
    TypeGetter,
    /// All properties defined in type alaises and interfaces
    TypeProperty,
    /// All setters defined in type alaises and interfaces
    TypeSetter,
    /// All methods defined in type alaises and interfaces
    TypeMethod,
}

impl Kind {
    pub fn contains(self, other: Self) -> bool {
        self == other
            || matches!(
                (self, other),
                (Self::Any, _)
                    | (
                        Self::Variable,
                        Self::Const | Self::Let | Self::Using | Self::Var,
                    )
                    | (
                        Self::ClassMember,
                        Self::ClassGetter
                            | Self::ClassMethod
                            | Self::ClassProperty
                            | Self::ClassSetter
                    )
                    | (
                        Self::ObjectLiteralMember,
                        Self::ObjectLiteralGetter
                            | Self::ObjectLiteralMethod
                            | Self::ObjectLiteralProperty
                            | Self::ObjectLiteralSetter
                    )
                    | (
                        Self::TypeMember,
                        Self::TypeGetter
                            | Self::TypeMethod
                            | Self::TypeParameter
                            | Self::TypeProperty
                            | Self::TypeSetter
                    )
                    | (
                        Self::NamespaceLike,
                        Self::ExportNamespace | Self::ImportNamespace | Self::Namespace
                    )
                    | (
                        Self::TypeLike,
                        Self::Class
                            | Self::Enum
                            | Self::EnumMember
                            | Self::Interface
                            | Self::TypeAlias
                            | Self::TypeParameter
                    )
            )
    }
}
impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Self::Any => "declaration",
            Self::CatchParameter => "catch parameter",
            Self::Class => "class",
            Self::ClassGetter => "class getter",
            Self::ClassMember => "class member",
            Self::ClassMethod => "class method",
            Self::ClassProperty => "class property",
            Self::ClassSetter => "class setter",
            Self::Const => "const",
            Self::Enum => "enum",
            Self::EnumMember => "enum member",
            Self::ExportAlias => "export alias",
            Self::ExportNamespace => "export namespace",
            Self::Function => "function",
            Self::ImportAlias => "import alias",
            Self::ImportNamespace => "import namespace",
            Self::IndexParameter => "index parameter",
            Self::Interface => "interface",
            Self::Let => "let",
            Self::Namespace => "namespace",
            Self::NamespaceLike => "namespace",
            Self::ObjectLiteralGetter => "object getter",
            Self::ObjectLiteralMember => "object member",
            Self::ObjectLiteralMethod => "object method",
            Self::ObjectLiteralProperty => "object property",
            Self::ObjectLiteralSetter => "object setter",
            Self::FunctionParameter => "function parameter",
            Self::TypeAlias => "type alias",
            Self::TypeGetter => "getter",
            Self::TypeLike => "type",
            Self::TypeMember => "type member",
            Self::TypeMethod => "method",
            Self::TypeParameter => "type parameter",
            Self::TypeProperty => "property",
            Self::TypeSetter => "setter",
            Self::Using => "using",
            Self::Var => "var",
            Self::Variable => "variable",
        };
        write!(f, "{repr}")
    }
}

#[derive(Debug, Deserializable, Copy, Clone, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
#[repr(u16)]
pub enum RestrictedModifier {
    Abstract = Modifier::Abstract as u16,
    Private = Modifier::Private as u16,
    Protected = Modifier::Protected as u16,
    Readonly = Modifier::Readonly as u16,
    Static = Modifier::Static as u16,
}

impl From<RestrictedModifier> for Modifier {
    fn from(modifier: RestrictedModifier) -> Self {
        match modifier {
            RestrictedModifier::Abstract => Modifier::Abstract,
            RestrictedModifier::Private => Modifier::Private,
            RestrictedModifier::Protected => Modifier::Protected,
            RestrictedModifier::Readonly => Modifier::Readonly,
            RestrictedModifier::Static => Modifier::Static,
        }
    }
}
impl From<Modifier> for RestrictedModifier {
    fn from(modifier: Modifier) -> Self {
        match modifier {
            Modifier::Abstract => RestrictedModifier::Abstract,
            Modifier::Private => RestrictedModifier::Private,
            Modifier::Protected => RestrictedModifier::Protected,
            Modifier::Readonly => RestrictedModifier::Readonly,
            Modifier::Static => RestrictedModifier::Static,
            _ => unreachable!("Unsupported case"),
        }
    }
}
impl From<RestrictedModifier> for BitFlags<Modifier> {
    fn from(modifier: RestrictedModifier) -> Self {
        Modifier::from(modifier).into()
    }
}

#[derive(
    Debug,
    Copy,
    Default,
    Deserializable,
    Clone,
    Hash,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(
    from = "SmallVec<[RestrictedModifier; 4]>",
    into = "SmallVec<[RestrictedModifier; 4]>"
)]
pub struct Modifiers(BitFlags<Modifier>);

impl Deref for Modifiers {
    type Target = BitFlags<Modifier>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<Modifier> for Modifiers {
    fn from(value: Modifier) -> Self {
        Modifiers(value.into())
    }
}
impl From<Modifiers> for SmallVec<[RestrictedModifier; 4]> {
    fn from(value: Modifiers) -> Self {
        value.into_iter().map(|modifier| modifier.into()).collect()
    }
}
impl From<SmallVec<[RestrictedModifier; 4]>> for Modifiers {
    fn from(values: SmallVec<[RestrictedModifier; 4]>) -> Self {
        Self::from_iter(values)
    }
}
impl FromIterator<RestrictedModifier> for Modifiers {
    fn from_iter<T: IntoIterator<Item = RestrictedModifier>>(values: T) -> Self {
        Self(
            values
                .into_iter()
                .map(Modifier::from)
                .fold(BitFlags::empty(), |acc, m| acc | m),
        )
    }
}
#[cfg(feature = "schemars")]
impl JsonSchema for Modifiers {
    fn schema_name() -> String {
        "Modifiers".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <std::collections::HashSet<RestrictedModifier>>::json_schema(gen)
    }
}
impl From<JsMethodModifierList> for Modifiers {
    fn from(value: JsMethodModifierList) -> Self {
        Modifiers((&value).into())
    }
}
impl From<JsPropertyModifierList> for Modifiers {
    fn from(value: JsPropertyModifierList) -> Self {
        Modifiers((&value).into())
    }
}
impl From<TsIndexSignatureModifierList> for Modifiers {
    fn from(value: TsIndexSignatureModifierList) -> Self {
        Modifiers((&value).into())
    }
}
impl From<TsMethodSignatureModifierList> for Modifiers {
    fn from(value: TsMethodSignatureModifierList) -> Self {
        Modifiers((&value).into())
    }
}
impl From<TsPropertySignatureModifierList> for Modifiers {
    fn from(value: TsPropertySignatureModifierList) -> Self {
        Modifiers((&value).into())
    }
}
impl std::fmt::Display for Modifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for value in self.0.iter() {
            write!(f, "{value} ")?;
        }
        Ok(())
    }
}

#[derive(
    Debug,
    Copy,
    Default,
    Deserializable,
    Clone,
    Hash,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum Scope {
    #[default]
    Any,
    Global,
}

impl Scope {
    /// Returns the scope of `node` or `None` if the scope cannot be determined or
    /// if the scope is an external module.
    fn from_declaration(node: &AnyJsBindingDeclaration) -> Option<Scope> {
        let control_flow_root = node.syntax().ancestors().skip(1).find(|x| {
            AnyJsControlFlowRoot::can_cast(x.kind())
                || x.kind() == JsSyntaxKind::TS_DECLARATION_MODULE
        })?;
        match control_flow_root.kind() {
            JsSyntaxKind::JS_MODULE
            | JsSyntaxKind::JS_SCRIPT
            | JsSyntaxKind::TS_DECLARATION_MODULE
            | JsSyntaxKind::TS_MODULE_DECLARATION => Some(Scope::Global),
            // Ignore declarations in an external module declaration
            JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION => None,
            _ => Some(Scope::Any),
        }
    }

    fn contains(self, scope: Scope) -> bool {
        matches!(self, Self::Any) || self == scope
    }
}
impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Self::Any => "",
            Self::Global => "global ",
        };
        write!(f, "{repr}")
    }
}

/// Supported cases.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserializable,
    Eq,
    Hash,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum Format {
    #[serde(rename = "camelCase")]
    Camel,

    #[serde(rename = "CONSTANT_CASE")]
    Constant,

    #[serde(rename = "PascalCase")]
    #[default]
    Pascal,

    #[serde(rename = "snake_case")]
    Snake,
}

impl From<Format> for Case {
    fn from(value: Format) -> Self {
        match value {
            Format::Camel => Case::Camel,
            Format::Constant => Case::Constant,
            Format::Pascal => Case::Pascal,
            Format::Snake => Case::Snake,
        }
    }
}
impl TryFrom<Case> for Format {
    type Error = &'static str;

    fn try_from(value: Case) -> Result<Self, Self::Error> {
        match value {
            Case::Camel => Ok(Format::Camel),
            Case::Constant => Ok(Format::Constant),
            Case::Pascal => Ok(Format::Pascal),
            Case::Snake => Ok(Format::Snake),
            Case::Kebab
            | Case::Lower
            | Case::Number
            | Case::NumberableCapital
            | Case::Uni
            | Case::Upper
            | Case::Unknown => Err("Unsupported case"),
        }
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserializable,
    Eq,
    Hash,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(from = "SmallVec<[Format; 4]>", into = "SmallVec<[Format; 4]>")]
pub struct Formats(Cases);

impl Deref for Formats {
    type Target = Cases;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<SmallVec<[Format; 4]>> for Formats {
    fn from(values: SmallVec<[Format; 4]>) -> Self {
        Self::from_iter(values)
    }
}
impl FromIterator<Format> for Formats {
    fn from_iter<T: IntoIterator<Item = Format>>(values: T) -> Self {
        Self(values.into_iter().map(|format| format.into()).collect())
    }
}
impl From<Formats> for SmallVec<[Format; 4]> {
    fn from(value: Formats) -> Self {
        value
            .0
            .into_iter()
            .filter_map(|case| case.try_into().ok())
            .collect()
    }
}
#[cfg(feature = "schemars")]
impl JsonSchema for Formats {
    fn schema_name() -> String {
        "Formats".to_string()
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <std::collections::HashSet<Format>>::json_schema(gen)
    }
}

/// trim underscores and dollar signs from `name` and returns the lengtj of the trimmed prefix.
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
