console.log(@deco class Foo {})
console.log(@deco class {})

const a1 = (@deco class Foo {});
const a2 = (@deco class {});

(@deco class Foo {});
(@deco class {});

const b1 = []
;(@deco class Foo {})

const b2 = []
;(@deco class {})

(@deco class Foo {}).name;
(@deco class {}).name;

class Foo extends (@deco class Foo {}){}

class Foo extends (@deco class {}){}
