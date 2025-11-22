/* should not generate diagnostics */

export class Sample {
  private add: () => void;

  constructor(private remove: () => void) {
    this.add = () => {};
    this.remove = () => {};
  }

  on(action: "add" | "remove"): void {
    this[action]();
  }
}
