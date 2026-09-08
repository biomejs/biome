/* should not generate diagnostics */

export class Thing {
  #doThing(index: number): void;
  #doThing(point: [x: number, y: number]): void;
  #doThing(param: number | [x: number, y: number]) {
    console.info('doing thing', param);
  }
  thingA() {
    this.#doThing(1);
  }
  thingB() {
    this.#doThing([1, 2]);
  }
}
