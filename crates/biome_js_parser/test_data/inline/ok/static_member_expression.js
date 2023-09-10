foo.bar
foo.await
foo.yield
foo.for
foo?.for
foo?.bar
class Test {
  #bar
  test(other) {
    this.#bar;
    this?.#bar;
    other.#bar;
    other?.#bar;
  }
}
