/* should not generate diagnostics */

export class Sample {
  private add;
  private remove;

  constructor() {
    this.add = () => {}
    this.remove = () => {}
  }

  on(action: "add" | "remove") {
    this[action]()
  }
}
