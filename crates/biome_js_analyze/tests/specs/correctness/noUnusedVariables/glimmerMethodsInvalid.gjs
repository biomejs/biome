// unusedMethod is never used - should warn
export default class MyComponent {
  count = 0;

  increment() {
    this.count++;
  }

  unusedMethod() {
    return 'never called';
  }

  <template>
    <button {{on "click" this.increment}}>+</button>
  </template>
}
