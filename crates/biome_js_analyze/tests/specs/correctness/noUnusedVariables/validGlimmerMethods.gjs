/* should not generate diagnostics */
export default class MyComponent {
  count = 0;

  get formattedCount() {
    return `Count: ${this.count}`;
  }

  increment() {
    this.count++;
  }

  <template>
    <div>
      {{this.formattedCount}}
      <button {{on "click" this.increment}}>
        Increment
      </button>
    </div>
  </template>
}
