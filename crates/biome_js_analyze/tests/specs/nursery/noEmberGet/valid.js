// should not generate diagnostics
export default class MyComponent {
  someMethod() {
    const value = this.myProperty;
    return this.anotherProperty;
  }
}
