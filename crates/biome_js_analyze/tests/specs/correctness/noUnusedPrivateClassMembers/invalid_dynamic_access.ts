export class Sample {
  #prop;

  constructor() {
    this.#prop = 0;
  }

  on(action) {
    this[action]()
  }
}
