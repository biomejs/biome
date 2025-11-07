// unusedGetter is never used - should warn
export default class MyComponent {
  count = 0;

  get formattedCount() {
    return `Count: ${this.count}`;
  }

  get unusedGetter() {
    return 'never used';
  }

  <template>
    <div>
      <p>{{this.formattedCount}}</p>
    </div>
  </template>
}
