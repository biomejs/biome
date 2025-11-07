/* should not generate diagnostics - private methods used in template */
export default class MyComponent {
  #privateHelper() {
    return "helper result";
  }

  #privateComputed() {
    return 42;
  }

  <template>
    <div>
      {{this.#privateHelper}}
      {{this.#privateComputed}}
    </div>
  </template>
}
