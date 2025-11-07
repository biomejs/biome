/* should not generate diagnostics - methods used in template */
export default class MyComponent {
  count = 0;

  increment() {
    this.count++;
  }

  decrement() {
    this.count--;
  }

  reset() {
    this.count = 0;
  }

  <template>
    <div>
      <button {{on "click" this.increment}}>+</button>
      <button {{on "click" this.decrement}}>-</button>
      <button {{on "click" this.reset}}>Reset</button>
    </div>
  </template>
}
