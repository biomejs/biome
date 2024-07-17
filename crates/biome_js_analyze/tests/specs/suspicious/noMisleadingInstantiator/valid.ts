/* should not generate diagnostics */

declare abstract class C {
  foo() {}
  get new();
  bar();
}

class C {
  constructor();
}

const foo = class {
  constructor();
};

const foo = class {
  new(): X;
};

class C {
  new() {}
}

class C {
  constructor() {}
}

const foo = class {
  new() {}
};

const foo = class {
  constructor() {}
};

interface I {
  new (): {};
}

export default class {
  constructor();
}

interface foo {
  new <T>(): bar<T>;
}

interface foo {
  new <T>(): 'x';
}

class C {
  #new(): C;
}
