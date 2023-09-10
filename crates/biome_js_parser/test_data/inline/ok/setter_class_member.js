class Setters {
  set foo(a) {}
  set static(a) {}
  static set bar(a) {}
  set "baz"(a) {}
  set ["a" + "b"](a) {}
  set 5(a) {}
  set #private(a) {}
}
class NotSetters {
  set(a) {}
  async set(a) {}
  static set(a) {}
}
