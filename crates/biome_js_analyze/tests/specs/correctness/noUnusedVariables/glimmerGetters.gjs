/* should not generate diagnostics - getters used in template */
export default class MyComponent {
  count = 0;

  get formattedCount() {
    return `Count: ${this.count}`;
  }

  get doubleCount() {
    return this.count * 2;
  }

  <template>
    <div>
      <p>{{this.formattedCount}}</p>
      <p>Double: {{this.doubleCount}}</p>
    </div>
  </template>
}
