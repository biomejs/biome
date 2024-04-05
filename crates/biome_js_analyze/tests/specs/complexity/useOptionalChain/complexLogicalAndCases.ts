// currently do not handle complex computed properties
foo && foo[bar as string] && foo[bar as string].baz;
foo && foo[1 + 2] && foo[1 + 2].baz;
foo && foo[typeof bar] && foo[typeof bar].baz;

// https://github.com/biomejs/biome/issues/1925
foo?.bar && foo.bar?.baz && foo.bar?.qux;

foo?.bar.baz?.qux && foo.bar.baz.qux.zoo;

foo && foo.bar.baz?.qux && foo.bar.baz.qux.zoo;
