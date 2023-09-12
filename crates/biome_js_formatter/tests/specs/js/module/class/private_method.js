class Foo {
  a = 1;

  *#a() {
    yield bar();
  }

  #b = 2;

  get b() { return 9999; }
  set #c(x) { return x; }

  static #d() {
    return Math.random();
  }
}
