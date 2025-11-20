/* should not generate diagnostics - private members used in both JS and template */
export default class MyComponent {
  #count = 0;
  #increment = 1;

  updateCount() {
    this.#count += this.#increment;
  }

  <template>
    <div>
      <p>Current count: {{this.#count}}</p>
      <button {{on "click" this.updateCount}}>Update</button>
    </div>
  </template>
}
