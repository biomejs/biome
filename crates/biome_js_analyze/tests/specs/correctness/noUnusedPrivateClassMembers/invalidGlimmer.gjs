export default class MyComponent {
  #privateCount = 0;      // Used in template - OK
  #unusedPrivate = "test"; // NOT used anywhere - should warn!

  <template>
    <div>
      {{this.#privateCount}}
    </div>
  </template>
}
