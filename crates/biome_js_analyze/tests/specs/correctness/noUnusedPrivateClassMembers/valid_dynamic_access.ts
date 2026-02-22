/* should not generate diagnostics */

export class Sample {
  private add: () => void;

  constructor(private remove: () => void) {
    this.add = () => {};
    this.remove = () => {};
  }
 // this as it stands is neither read or write, so needs work on reads / writes front or a special exception here
	// but also must be considered in use readonly rule
  on(action: "add" | "remove"): void {
    this[action]();
  }
}
