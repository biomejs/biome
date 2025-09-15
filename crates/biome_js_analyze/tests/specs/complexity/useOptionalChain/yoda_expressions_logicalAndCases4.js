// chained calls with element access
null !== foo && null !== foo.bar && null !== foo.bar.baz && foo.bar.baz[buzz]()
null !== foo && null !== foo.bar && null !== foo.bar.baz && null !== foo.bar.baz[buzz] && foo.bar.baz[buzz]()

// (partially) pre-optional chained
null !== foo && null !== foo?.bar && null !== foo?.bar.baz && null !== foo?.bar.baz[buzz] && foo?.bar.baz[buzz]()
null !== foo && null !== foo?.bar.baz && foo?.bar.baz[buzz]
null !== foo && null !== foo?.() && foo?.().bar
null !== foo.bar && null !== foo.bar?.() && foo.bar?.().baz

// chained members
undefined !== foo && foo.bar
undefined !== foo.bar && foo.bar.baz
undefined !== foo && foo()
undefined !== foo.bar && foo.bar()
undefined !== foo && undefined !== foo.bar && undefined !== foo.bar.baz && foo.bar.baz.buzz
undefined !== foo.bar && undefined !== foo.bar.baz && foo.bar.baz.buzz

// case with a jump (i.e. a non-nullish prop)
undefined !== foo && undefined !== foo.bar && foo.bar.baz.buzz
undefined !== foo.bar && foo.bar.baz.buzz

// case where for some reason there is a doubled up expression
undefined !== foo && undefined !== foo.bar && undefined !== foo.bar.baz && undefined !== foo.bar.baz && foo.bar.baz.buzz
undefined !== foo.bar && undefined !== foo.bar.baz && undefined !== foo.bar.baz && foo.bar.baz.buzz

// chained members with element access
undefined !== foo && undefined !== foo[bar] && undefined !== foo[bar].baz && foo[bar].baz.buzz

// case with a jump (i.e. a non-nullish prop)
undefined !== foo && undefined !== foo[bar].baz && foo[bar].baz.buzz

// chained calls
undefined !== foo && undefined !== foo.bar && undefined !== foo.bar.baz && foo.bar.baz.buzz()
undefined !== foo && undefined !== foo.bar && undefined !== foo.bar.baz && undefined !== foo.bar.baz.buzz && foo.bar.baz.buzz()
undefined !== foo.bar && undefined !== foo.bar.baz && undefined !== foo.bar.baz.buzz && foo.bar.baz.buzz()

// case with a jump (i.e. a non-nullish prop)
undefined !== foo && undefined !== foo.bar && foo.bar.baz.buzz()
undefined !== foo.bar && foo.bar.baz.buzz()

// case with a jump (i.e. a non-nullish prop)
undefined !== foo && undefined !== foo.bar && undefined !== foo.bar.baz.buzz && foo.bar.baz.buzz()

// case with a call expr inside the chain for some inefficient reason
undefined !== foo && undefined !== foo.bar() && undefined !== foo.bar().baz && undefined !== foo.bar().baz.buzz && foo.bar().baz.buzz()

// chained calls with element access
undefined !== foo && undefined !== foo.bar && undefined !== foo.bar.baz && foo.bar.baz[buzz]()
undefined !== foo && undefined !== foo.bar && undefined !== foo.bar.baz && undefined !== foo.bar.baz[buzz] && foo.bar.baz[buzz]()

// (partially) pre-optional chained
undefined !== foo && undefined !== foo?.bar && undefined !== foo?.bar.baz && undefined !== foo?.bar.baz[buzz] && foo?.bar.baz[buzz]()
undefined !== foo && undefined !== foo?.bar.baz && foo?.bar.baz[buzz]
undefined !== foo && undefined !== foo?.() && foo?.().bar
undefined !== foo.bar && undefined !== foo.bar?.() && foo.bar?.().baz

// chained members
null != foo && foo.bar
null != foo.bar && foo.bar.baz
null != foo && foo()
null != foo.bar && foo.bar()
null != foo && null != foo.bar && null != foo.bar.baz && foo.bar.baz.buzz
null != foo.bar && null != foo.bar.baz && foo.bar.baz.buzz
