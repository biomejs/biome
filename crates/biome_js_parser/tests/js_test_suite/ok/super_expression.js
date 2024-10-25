class Test extends B {
  constructor() {
    super();
  }
  test() {
    super.test(a, b);
    super[1];
  }
}
