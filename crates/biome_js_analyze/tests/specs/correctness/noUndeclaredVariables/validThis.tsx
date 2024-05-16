export class MyComponent {
  render() {
    type T = typeof this.foo;
    return <this.foo />
  }
}
