/* should not generate diagnostics - private fields used in template */
export default class MyComponent {
  #privateCount = 0;
  #privateName = "test";
  #privateData = { value: 42 };

  <template>
    <div>
      <p>Count: {{this.#privateCount}}</p>
      <p>Name: {{this.#privateName}}</p>
      <p>Data: {{this.#privateData}}</p>
    </div>
  </template>
}
