// case with a jump (i.e. a non-nullish prop)
foo && foo.bar && foo.bar.baz.buzz && bing
foo.bar && foo.bar.baz.buzz && bing

// case where for some reason there is a doubled up expression
foo && foo.bar && foo.bar.baz && foo.bar.baz && foo.bar.baz.buzz && bing
foo.bar && foo.bar.baz && foo.bar.baz && foo.bar.baz.buzz && bing

// chained members with element access
foo && foo[bar] && foo[bar].baz && foo[bar].baz.buzz && bing

// case with a jump (i.e. a non-nullish prop)
foo && foo[bar].baz && foo[bar].baz.buzz && bing

// chained calls
foo && foo.bar && foo.bar.baz && foo.bar.baz.buzz() && bing
foo && foo.bar && foo.bar.baz && foo.bar.baz.buzz && foo.bar.baz.buzz() && bing
foo.bar && foo.bar.baz && foo.bar.baz.buzz && foo.bar.baz.buzz() && bing

// case with a jump (i.e. a non-nullish prop)
foo && foo.bar && foo.bar.baz.buzz() && bing
foo.bar && foo.bar.baz.buzz() && bing

// case with a jump (i.e. a non-nullish prop)
foo && foo.bar && foo.bar.baz.buzz && foo.bar.baz.buzz() && bing

// case with a call expr inside the chain for some inefficient reason
foo && foo.bar() && foo.bar().baz && foo.bar().baz.buzz && foo.bar().baz.buzz() && bing

// chained calls with element access
foo && foo.bar && foo.bar.baz && foo.bar.baz[buzz]() && bing
foo && foo.bar && foo.bar.baz && foo.bar.baz[buzz] && foo.bar.baz[buzz]() && bing

// (partially) pre-optional chained
foo && foo?.bar && foo?.bar.baz && foo?.bar.baz[buzz] && foo?.bar.baz[buzz]() && bing
foo && foo?.bar.baz && foo?.bar.baz[buzz] && bing
foo && foo?.() && foo?.().bar && bing
foo.bar && foo.bar?.() && foo.bar?.().baz && bing

// chained members
foo && foo.bar && bing.bong
foo.bar && foo.bar.baz && bing.bong
foo && foo() && bing.bong
foo.bar && foo.bar() && bing.bong
foo && foo.bar && foo.bar.baz && foo.bar.baz.buzz && bing.bong
foo.bar && foo.bar.baz && foo.bar.baz.buzz && bing.bong

// case with a jump (i.e. a non-nullish prop)
foo && foo.bar && foo.bar.baz.buzz && bing.bong
foo.bar && foo.bar.baz.buzz && bing.bong

// case where for some reason there is a doubled up expression
foo && foo.bar && foo.bar.baz && foo.bar.baz && foo.bar.baz.buzz && bing.bong
foo.bar && foo.bar.baz && foo.bar.baz && foo.bar.baz.buzz && bing.bong

// chained members with element access
foo && foo[bar] && foo[bar].baz && foo[bar].baz.buzz && bing.bong

// case with a jump (i.e. a non-nullish prop)
foo && foo[bar].baz && foo[bar].baz.buzz && bing.bong
