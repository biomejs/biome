export default class MyComponent {
  #privateCount = 0;
  #privateName = "test";

  get formattedCount() {
    return `Count: ${this.#privateCount}`;
  }

  <template>
    <div>
      {{this.#privateCount}}
      {{this.#privateName}}
      {{this.formattedCount}}
    </div>
  </template>
}
