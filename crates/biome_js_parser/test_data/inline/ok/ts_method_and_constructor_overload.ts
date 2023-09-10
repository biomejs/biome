class Test {
     constructor();
     constructor(a: String) // ASI
     constructor(a?: String) {}
     async method(): Promise<String>;
     method(a: String): Promise<String> // ASI
     async method(a?: String): Promise<String> { return "test" }
}
