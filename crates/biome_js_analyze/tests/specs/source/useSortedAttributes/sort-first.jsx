<Hello firstName="John" key={id} lastName="Smith" />;

// `key` is pinned first within each group delimited by spread props.
<Hello a="1" key={id} {...props} z="2" key={id2} b="3" />;

// Already correct: `key` first, then the rest sorted naturally.
<Hello key={id} firstName="John" lastName="Smith" />;
