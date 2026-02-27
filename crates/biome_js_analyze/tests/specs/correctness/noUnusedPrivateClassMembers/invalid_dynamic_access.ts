export class Sample {
  private member;
  #prop;

  constructor() {
    this.#prop = 0;
    this.member = 0;
  }

  method(name) {
    return this[name];
  }
}
