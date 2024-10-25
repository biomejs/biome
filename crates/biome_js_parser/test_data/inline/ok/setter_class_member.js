class Setters {
  set foo(a) {}
  set bax(a,) {}
  set static(a) {}
  static set bar(a) {}
  static set baz(a,) {}
  set "baz"(a) {}
  set ["a" + "b"](a) {}
  set 5(a) {}
  set 6(a,) {}
  set #private(a) {}
}
class NotSetters {
  set(a) {}
  async set(a) {}
  static set(a) {}
}
