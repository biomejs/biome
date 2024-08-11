# Parser

This document will explain how to contribute to creating a new Parser with Biome.

## Grammar

Everything starts with writing the grammar for the language. We use a fork of [`ungrammar`](https://github.com/rust-analyzer/ungrammar), a simple DSL
for writing grammar for languages.

Internally, we have a codegen script that reads `.ungram` files and all the files needed to write a parser.

### Create a new language

From the root of this repository, head to `xtask/codegen` folder. That's where you'll start doing your work.

Let's say you want to create a new language that has the extension `html`, you'll have to follow these instructions:

1. Create a new `html.ungram` file in this folder.
   Add this legend to the `.ungram` file.

  ```text
  // This grammar specifies the structure of [LANGUAGE]'s concrete syntax tree.
  // It does not specify parsing rules (ambiguities, precedence, etc are out of scope).
  // Tokens are processed -- contextual keywords are recognised, compound operators glued.
  //
  // Legend:
  //
  //   //                    -- comment
  //   Name =                -- non-terminal definition
  //   'ident'               -- token (terminal)
  //   A B                   -- sequence
  //   A | B                 -- alternation
  //   A*                    -- zero or more repetition
  //   (A (',' A)* ','?)            -- repetition of node A separated by ',' and allowing a trailing comma
  //   (A (',' A)*)             -- repetition of node A separated by ',' without a trailing comma
  //   A?                    -- zero or one repetition
  //   (A)                   -- same as A
  //   label:A               -- suggested name for field of AST node
  ```
1. Create a new file called `src/html_kinds_src.rs`. This file must return a static `KindSrc`.
1. Create two new crates: `biome_html_syntax` and `biome_html_factory`. Use `cargo new --lib crates/biome_html_syntax`.
1. Create a `generated/` folder inside the `src/` folder of the newly created crates.
1. Add a new variant to `LanguageKind`, inside `language_kind.rs` file. The new variant will be `Html`. You'll have to implement
   all methods and cover the new variant.
1. Add a new prefix `html_` to `LANGUAGE_PREFIXES` inside `language_kind.rs`.
1. Once you covered all variants, run the command `cargo codegen grammar`.


### Conventions when writing a new grammar in Biome

- All nodes **must** start with the prefix of the language, e.g. `HtmlSimpleAttribute`.
- Unions of nodes **must** start with `Any*`, e.g. `AnyHtmlAttribute`.
- Nodes for enclosing syntax errors must have the **Bogus** word, e.g. `HtmlBogusAttribute`.
- **Bogus** nodes **must be part of a variant**, e.g.
  ```text
  AnyHtmlAttribute =
    HtmlSimpleAttribute
    HtmlBogusAttribute
  ```
- Nodes that represent a list **must** end with the postfix **List**, e.g. `HtmlAttributeList`.
- Lists are **never** optional. They are mandatory and empty by default, e.g.
  ```text
  HtmlTag =
    attributes: HtmlAttributeList
  ```

### Run the codegen

Once you're finished, you'll have to generate the files from the grammar. Use the command `just gen-grammar`. This command accepts a list of known languages. If you don't pass anything, the command will generate all the available grammars.

```shell
# generate grammar of the HTML language
just gen-grammar html

# generate grammar of the HTML and CSS languages
just gen-grammar html css

# generate grammar of all languages
just gen-grammar
```

## Parsing

There are generally three actors you'll need to have in order to create a parser in Biome:
- [a lexer](#implement-a-lexer);
- [a token source](#implement-a-token-source);
- [a parser](#implement-a-parser);

### Implement a lexer

The lexer is the entity in charge of consuming each character coming from the source code and emitting "tokens".

The lexer is the lower primitive of a Biome parser. It usually consumes characters as bytes, but it can also consume characters with different encodings, e.g., UTF-8, UTF-16, etc.

Create a `lexer/mod.rs` inside the parser crate.

```rust,ignore
use biome_beta_syntax::BetaSyntaxKind;
use biome_parser::ParseDiagnostic;

pub(crate) struct BetaLexer<'source> {
    /// Source text
    source: &'source str,

    /// The start byte position in the source text of the next token.
    position: usize,

    /// the current token
    current_kind: BetaSyntaxKind,

    /// diagnostics emitted during the parsing phase
    diagnostics: Vec<ParseDiagnostic>,
}
```

The import and implementation of the `Lexer` trait from the `biome_parser` crate.

```rust,ignore
impl<'source> Lexer<'source> for BetaLexer<'source> {}
```

The `Lexer` requires a bunch of functions to be implemented.

### Implement a token source

The token source is the second brick needed to create a parser. The token source is a thin layer that wraps the lexer, and it implements few functionalities:
- Lookahead: check for possible next tokens/characters without advancing the parsing.
- Re-lexing: a feature that allows the consumption of tokens with a different context.
- Checkpoint: save the current status of the lexing phase, and restore it if a certain parsing logic is incorrect.

If you want to have lookahead, you need to:
- Wrap your lexer with a `BufferedLexer`.
- Implement `TokenSourceWithBufferedLexer` for your token source.
- Implement `LexerWithCheckpoint` for your lexer.

```rust,ignore
use biome_parser::lexer::BufferedLexer;
use biome_beta_syntax::BetaSyntaxKind;
use crate::lexer::{BetaLexer};

pub(crate) struct BetaTokenSource<'src> {
    lexer: BufferedLexer<BetaSyntaxKind, BetaLexer<'src>>,
}

impl<'source> TokenSourceWithBufferedLexer<BetaLexer<'source>> for BetaTokenSource<'source> {
    fn lexer(&mut self) -> &mut BufferedLexer<BetaSyntaxKind, BetaLexer<'source>> {
        &mut self.lexer
    }
}
```

### Implement a parser

A parser is a `struct` that must implement the [`Parser`](https://docs.rs/biome_parser/latest/biome_parser/trait.Parser.html) trait.

Notably, the `struct` save the token source, the parser context and possible options:

```rust,ignore
pub(crate) struct BetaParser<'source> {
    context: ParserContext<BetaSyntaxKind>,
    source: BetaTokenSource<'source>,
    // optional, only if the parser is meant to have some options
    options: BetaParserOptions,
}
```
___

## Authoring Parse Rules

This is a short, or not so short, guide to implement parse rules using the Biome parser infrastructure.

### Naming
The convention is to prefix your parse rule with `parse_` and then use the name defined in the grammar file.

For example, `parse_for_statement` or `parse_expression`.

### Signature
Most parse rules take a `&mut` reference to the parser as their only parameter and return a `ParsedSyntax`.

```rust,ignore
fn parse_rule_name(&mut: Parser) -> ParsedSyntax {}
```

You're free to add additional parameters to your function if needed. There are rare cases where you want to consider returning `ConditionalParsedSyntax` as explained in [conditional syntax](#conditional-syntax)


### Parsing a single node

Let's assume you want to parse the JS `if` statement:

```js,ignore
JsIfStatement =
 if
 (
 test: JsAnyExpression
 )
 consequent: JsBlockStatement
 else_clause: JsElseClause?
```

#### Presence Test

Now, the parsing function must first test if the parser is positioned at an `if` statement and return `Absent` if that's not the case.

```rust,ignore
if !p.at(T![if]) {
 return ParsedSyntax::Absent;
}
```

Why return `ParsedSyntax::Absent`? The function must return `ParsedSyntax::Absent` if the rule can't predict by the next token(s) if they form the expected node or not. Doing so allows the calling rule to decide if this is an error and perform an error recovery if necessary.  The second reason is to ensure that the rule doesn't return a node where all children are missing.

Your rule implementation may want to consider more than just the first child to determine if it can parse at least some of the expected children.
For example, the if statement rule could test if the parser is located at an `else` clause and then create an `if` statement where all children are missing except the `else` clause:

```rust,ignore
if !p.at(T![if]) && !p.at(T![else]){
  return Absent
}
```

Your implementation can also call into another parsing rule if the first child is a node and not a token.

```rust,ignore
let assignment_target = parse_assignment_target(p);

if assignment_target.is_absent() {
  return Absent;
}

let my_node = assignment_target.precede_or_missing();
```

But be careful with calling other rules. Your rule mustn't progress the parser - meaning that it can't
advance in the parsing process and consume tokens - if it returns `Absent`.


#### Parse children
The parse rules will guide you in how to write your implementation and the parser infrastructure provides the following convenience APIs:

* Optional token `'ident'?`: Use `p.eat(token)`. It eats the next token if it matches the passed-in token.
* Required token `'ident'`: Use`p.expect(token)`. It eats the next token if it matches the passed-in token.
  It adds an `Expected 'x' but found 'y' instead` error and a missing marker if the token isn't present in the source code.
* Optional node `body: JsBlockStatement?`: Use`parse_block_statement(p).ok(p)`. It parses the block if it is present in the source code and adds a missing marker if it isn't.
* Required node `body: JsBlockStatement`: Use `parse_block_statement(p).or_add_diagnostic(p, error_builder)`:
  it parses the block statement if it is present in the source code and adds a missing marker and an error if not.

Using the above-described rules result in the following implementation for the `if` statement rule.

```rust,ignore
fn parse_if_statement(p: &mut Parser) -> ParsedSyntax {
 if !p.at(T![if]) {
  return Absent;
 }

 let m = p.start();

 p.expect(T![if]);
 p.expect(T!['(']);
 parse_any_expression(p).or_add_diagnostic(p, js_parse_errors::expeced_if_statement);
 p.expect(T![')']);
 parse_block_statement(p).or_add_diagnostic(p, js_parse_errors::expected_block_statement);
// the else block is optional, handle the marker by using `ok`
 parse_else_clause(p).ok();

 Present(m.complete(p, JS_IF_STATEMENT));
}
```

Hold on, what are these *missing* markers? Biome's AST facade uses fixed offsets to retrieve a particular child from a node.
For example, the 3rd child of the if statement is the condition. However, the condition would become the second element
if the opening parentheses `(` isn't present in the source text. That's where missing elements come into play.

### Parsing Lists & Error Recovery

> 1. Performance-Neutral Error Recovery: implement an error recovery mechanism that does not degrade the parsing performance of valid code. This ensures that the parser remains efficient while being more forgiving of errors. We can try to check if the next token is a valid item for a list (e.g. we can use `is_at_item` to check if we have a missing end list token), however versus merely checking for a end list token does introduce a performance consideration, especially in well-formed documents where syntax errors are rare.
>
> 2. Preservation of Valid Tree Structure: modify the parser to retain as much information from the valid parts of the AST tree as possible. Even when encountering invalid parts, the parser should mark them as 'bogus' rather than invalidating the parent node. This approach minimizes the loss of useful information due to isolated syntax errors.


Parsing lists is different from parsing single elements with a fixed set of children because it requires looping until
the parser reaches a terminal token (or the end of the file).

You may remember that `parse_*` methods shouldn't progress parsing if they return `Absent`.
Not progressing the parser is problematic inside `while` loops because it inevitably results in an infinite loop.

That's why you must do error recovery when parsing lists. Luckily, the parser comes with the infrastructure to make error recovery a piece of cake.
The general structure for parsing a list is (yes, that's something the parser infrastructure should provide for you):


Let's try to parse an array:

```js,ignore
[ 1, 3, 6 ]
```

We will use  `ParseSeparatedList` in order to achieve that

```rust,ignore
struct ArrayElementsList;

impl ParseSeparatedList for ArrayElementsList {
    type ParsedElement = CompletedMarker;

    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax<Self::ParsedElement> {
        parse_array_element(p)
    }

    fn is_at_list_end(&self, p: &mut Parser) -> bool {
        p.at_ts(token_set![T![default], T![case], T!['}']])
    }

    fn recover(
        &mut self,
        p: &mut Parser,
        parsed_element: ParsedSyntax<Self::ParsedElement>,
    ) -> parser::RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecoveryTokenSet::new(JS_BOGUS_STATEMENT, STMT_RECOVERY_SET),
            js_parse_error::expected_statement,
        )
    }
}
```

Let's run through this step by step:

```rust,ignore
parsed_element.or_recover(
    p,
    &ParseRecoveryTokenSet::new(JS_BOGUS_STATEMENT, STMT_RECOVERY_SET),
    js_parse_error::expected_case,
)
```

The `or_recover` performs an error recovery if the `parse_array_element` method returns `Absent`;
there's no array element in the source text.

The recovery eats all tokens until it finds one of the tokens specified in the `token_set`,
a line break (if you called `enable_recovery_on_line_break`) or the end of the file.

The recovery doesn't throw the tokens away but instead wraps them inside a `JS_BOGUS_EXPRESSION` node (first parameter).
There exist multiple `BOGUS_*` nodes. You must consult the grammar to understand which `BOGUS*` node is supported in your case.

> You usually want to include the terminal token ending your list, the element separator token, and the token terminating a statement in your recovery set.


Now, the problem with recovery is that it can fail, and there are two reasons:

- the parser reached the end of the file;
- the next token is one of the tokens specified in the recovery set, meaning there is nothing to recover from;

In these cases the `ParseSeparatedList` and `ParseNodeList` will recover the parser for you.

### Conditional Syntax

The conditional syntax allows you to express that some syntax may not be valid in all source files. Some use cases are:

* syntax that is only supported in strict or sloppy mode: for example, `with` statements is not valid when a JavaScript file uses `"use strict"` or is a module;
* syntax that is only supported in certain file types: Typescript, JSX, modules;
* syntax that is only available in specific language versions: experimental features, different versions of the language e.g. (ECMA versions for JavaScript);

The idea is that the parser always parses the syntax regardless of whatever it is supported in this specific file or context.
The main motivation behind doing so is that this gives us perfect error recovery and allows us to use the same code regardless of whether the syntax is supported.

However, conditional syntax must be handled because we want to add a diagnostic if the syntax isn't supported for the current file, and the parsed tokens must be attached somewhere.

Let's have a look at the `with` statement that is only allowed in loose mode/sloppy mode:

```rust,ignore
fn parse_with_statement(p: &mut Parser) -> ParsedSyntax {
 if !p.at(T![with]) {
  return Absent;
 }

 let m = p.start();
 p.bump(T![with]); // with
 parenthesized_expression(p).or_add_diagnostic(p, js_errors::expected_parenthesized_expression);
 parse_statement(p).or_add_diagnostic(p, js_error::expected_statement);
 let with_stmt = m.complete(p, JS_WITH_STATEMENT);

 let conditional = StrictMode.excluding_syntax(p, with_stmt, |p, marker| {
  p.err_builder("`with` statements are not allowed in strict mode", marker.range(p))
 });


}
```

The start of the rule is the same as for any other rule. The exciting bits start with

```rust,ignore
fn parse_something() {
    let conditional = StrictMode.excluding_syntax(p, with_stmt, |p, marker| {
        p.err_builder("`with` statements are not allowed in strict mode", marker.range(p))
    });
}
```

The `StrictMode.excluding_syntax` converts the parsed syntax to a bogus node and uses the diagnostic builder to create a diagnostic if the feature is not supported.

You can convert the `ConditionalParsedSyntax` to a regular `ParsedSyntax` by calling `or_invalid_to_bogus`, which wraps the whole parsed `with` statement in an `BOGUS` node if the parser is in strict mode and otherwise returns the unchanged `with` statement.

What if there's no `BOGUS` node matching the node of your parse rule? You must then return the `ConditionalParsedSyntax` without making the `or_invalid_to_bogus` recovery. It's then up to the caller to recover the potentially invalid syntax.


### Summary

* Parse rules are named `parse_rule_name`
* The parse rules should return a `ParsedSyntax`
* The rule must return `Present` if it consumes any token and, therefore, can parse the node with at least some of its children.
* It returns `Absent` otherwise and must not progress parsing nor add any errors.
* Lists must perform error recovery to avoid infinite loops.
* Consult the grammar to identify the `BOGUS` node that is valid in the context of your rule.

