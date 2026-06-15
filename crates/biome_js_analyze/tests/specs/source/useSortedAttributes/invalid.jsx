// Invalid cases - should trigger the rule.
// `key` and `ref` are pinned first, in that order, then the rest are sorted.

<Hello firstName="John" key={id} lastName="Smith" />;

<Hello a="1" ref={r} key={id} b="2" />;

<Hello a="1" key={id} {...props} z="2" key={id2} b="3" />;
