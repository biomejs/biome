# Tree Traversal

Every `SyntaxNode` and `SyntaxToken` a walk yields allocates a reference-counted cursor. The APIs below read the green tree directly and allocate fewer cursors, or none. Pick the narrowest one that answers the question.

## Choosing a Walk

| Need | Use | Instead of |
| --- | --- | --- |
| Nodes of a few kinds in a subtree | `node.preorder_of_kinds(Kinds::KIND_SET)` | `descendants().filter_map(Kinds::cast)`, `preorder()` + `cast` |
| Prune a subtree mid-walk | `skip_subtree()` right after its `WalkEvent::Enter` | Tracking a depth counter and ignoring events |
| Kind, text, range, or trivia of every token | `node.borrowed_tokens()` | `descendants_tokens(Direction::Next)` |
| One slot of a node | `node.slot(index)` | `slots().nth(index)` |
| Whether a subtree has any comment or skipped trivia | `has_comments_descendants()`, `has_skipped_descendants()` | Scanning its tokens' trivia |

## `preorder_of_kinds`

Yields `WalkEvent::Enter` and `WalkEvent::Leave` only for nodes whose kind is in the set, in the order `preorder()` would. Nodes of other kinds are still traversed in the green tree, but no `SyntaxNode` is created for them unless one of their descendants is yielded.

Build the set from typed nodes so it stays in sync with the grammar:

```rust
const KINDS: SyntaxKindSet<JsLanguage> =
    AnyFunctionLike::KIND_SET.union(JsYieldExpression::KIND_SET);
```

Every `AstNode` and every `declare_node_union!` union has `KIND_SET`. `SyntaxKindSet::union` and `SyntaxKindSet::from_raw` are `const`; `SyntaxKindSet::of` is not.

The walk visits every green node of the subtree regardless of the set. To stop descending, call `skip_subtree()` after an `Enter`; the next event is that node's `Leave`. A common case is not descending into nested functions:

```rust
let mut events = body.syntax().preorder_of_kinds(KINDS);
while let Some(event) = events.next() {
    let WalkEvent::Enter(node) = event else { continue };
    if AnyFunctionLike::can_cast(node.kind()) {
        events.skip_subtree();
        continue;
    }
    // ...
}
```

Include the pruning kinds in the set: a node the walk never yields can't be skipped. `preorder()` has the same `skip_subtree()`.

## `borrowed_tokens`

Yields `BorrowedToken`s in source order without allocating per token. A `BorrowedToken` exposes `kind()`, `text()`, `text_range()`, `text_trimmed_range()`, `leading_trivia_pieces()`, and `trailing_trivia_pieces()`. Trivia pieces expose `kind()`, `text()`, and `text_range()`. All ranges are file ranges. `text()` includes trivia and there is no trimmed-text accessor; compare by `kind()`, or slice `text()` with `text_trimmed_range()` offset by `text_range().start()`.

A borrowed token can't navigate to its parent or siblings and can't be stored in rule `State`. When a match needs a `SyntaxToken`, store the range or look the token up only for the hit:

```rust
for token in root.borrowed_tokens() {
    for trivia in token.leading_trivia_pieces() {
        if trivia.kind().is_comment() && trivia.text().contains(DIRECTIVE) {
            let range = trivia.text_range();
            if let Some(token) = root.covering_element(range).into_token() {
                matches.push((token, range));
            }
        }
    }
}
```

Reach for `descendants_tokens()` only when most tokens need tree navigation.

## Analyzer `Visitor`s

Most lint rules need no custom `Visitor`: `Ast<Node>` and `Semantic<Node>` queries already ride the analyzer's kind-filtered walk. Write one only when a rule carries state across the walk, such as a stack of enclosing functions, that its query can't provide.

Every custom `Visitor` implements `visited_kinds`. The analyzer walks a phase with `preorder_of_kinds` over the union of its visitors' sets. If any visitor of the phase returns `None` (the trait default), the whole phase falls back to a full `preorder()`, which slows every rule in that phase.

```rust
impl Visitor for MissingYieldVisitor {
    type Language = JsLanguage;

    fn visited_kinds(
        &self,
        _: Option<SyntaxKindSet<JsLanguage>>,
    ) -> Option<SyntaxKindSet<JsLanguage>> {
        Some(AnyFunctionLike::KIND_SET.union(JsYieldExpression::KIND_SET))
    }

    fn visit(&mut self, event: &WalkEvent<JsSyntaxNode>, ctx: VisitorContext<JsLanguage>) {
        // ...
    }
}
```

The contract is in the trait's rustdoc (`crates/biome_analyze/src/visitor.rs`). In practice:

- The set covers every kind whose `Enter` or `Leave` changes the visitor's state or output. Check it against `visit`: every kind it casts, matches, or tests with `can_cast` is in the set.
- `visit` keeps its own filtering. It also receives events for the kinds other visitors of the phase requested.
- The root is yielded only when its kind is in the set. A visitor that acts on the root includes the root union, for example `AnyJsRoot::KIND_SET`.
- A visitor that does all its work in `start` or `finish` returns `Some(SyntaxKindSet::EMPTY)`.
- A visitor that forwards every node to `ctx.match_query` returns the `syntax_query_kinds` argument.

Return `None` only when the state truly depends on every node kind, and say why in a comment.

## Per-Node Work in Rules

`run()` executes once per queried node. Walking siblings, ancestors, or a whole list from there makes the rule quadratic on long lists or deep nesting. Compute the answer once from the containing node, for example by querying the list or the root, or from a semantic service.
