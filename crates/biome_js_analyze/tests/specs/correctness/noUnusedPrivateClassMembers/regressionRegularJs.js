/* Regression test: ensure regular JS files still work correctly */
class MyClass {
  #used = 1;
  #unused = 2;

  getUsed() {
    return this.#used;
  }
}
