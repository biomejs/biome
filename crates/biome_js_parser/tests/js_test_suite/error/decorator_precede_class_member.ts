class Bar {
  public @dec get foo() {}
  static @dec foo: string;
  readonly @dec test() {}
  private @dec test() {}
  protected @dec test() {}
}
class Qux extends Bar {
  public @dec get foo() {}
  static @dec foo: string;
  readonly @dec test() {}
  private @dec test() {}
  accessor @dec test() {}
}
