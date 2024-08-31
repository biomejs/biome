class ValidNone {
  constructor(foo: string) {}
}

class ValidNone2 {
  constructor(readonly foo: string) {}
}

class ValidNone3 {
  name: string;
  x: number;
  getX() {
    return this.x;
  }
}

class ValidNone4 {
  name: string;
  foo?: string;
  'foo-bar'?: string;
}

class ValidNone5 {
  constructor({ x, y }: { x: number; y: number }) {}
}

class ValidNone6 {
  x = 2;
}

class ValidNone7 {
  #foo = 1;
  #bar() {}
}
