class Test {
  method() {}
  async asyncMethod() {}
  async* asyncGeneratorMethod() {}
  * generatorMethod() {}
  "foo"() {}
  ["foo" + "bar"]() {}
  5() {}
  #private() {}
}
class ContextualKeywords {
   // Methods called static
  static() {}
  async static() {}
  * static() {}
  async* static() {}
  declare() {}
  get() {} // Method called get
  set() {} // Method called set
}
class Static {
  static method() {}
  static async asyncMethod() {}
  static async* asyncGeneratorMethod() {}
  static * generatorMethod() {}
  static static() {}
  static async static() {}
  static async* static() {}
  static * static() {}
}
