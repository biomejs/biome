/* should not generate diagnostics */

export class BadExample {
	constructor(private something: string) {}

	example() {
		const { something } = this
		return something
	}
}

export class ToastService {
	private _toastId = 0;

	show(message: string, type: string, autoClose: boolean): void {
		const id = this._toastId++;
		this.activeToasts.push({ id, message, type, autoClose });
	}
}

class TsSelfUpdate {
	private usedOnlyToUpdateItself = 5;

	method() {
		this.usedOnlyToUpdateItself++;
	}
}

export class ToastService {
	private _toastId = 0;

	show(message: string, type: string, autoClose: boolean): void {
		const id = this._toastId++;
		this.activeToasts.push({ id, message, type, autoClose });
	}
}

class TsOnlyWrite {
	private usedOnlyInWrite = 5;

	method() {
		this.usedOnlyInWrite = 21;
	}
}
