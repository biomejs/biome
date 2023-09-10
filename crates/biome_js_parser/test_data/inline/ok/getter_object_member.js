let a = {
  get foo() {
    return foo;
  },
  get "bar"() {
    return "bar";
  },
  get ["a" + "b"]() {
    return "a" + "b"
  },
  get 5() {
    return 5;
  },
  get() {
   return "This is a method and not a getter";
  }
}
