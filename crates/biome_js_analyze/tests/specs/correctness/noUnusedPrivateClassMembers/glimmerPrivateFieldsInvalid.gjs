// #unusedPrivate is never used - should warn
export default class MyComponent {
  #privateCount = 0;      // Used in template - OK
  #unusedPrivate = "test"; // NOT used anywhere - should warn!
  #anotherUnused = 123;   // NOT used anywhere - should warn!

  <template>
    <div>
      {{this.#privateCount}}
    </div>
  </template>
}
