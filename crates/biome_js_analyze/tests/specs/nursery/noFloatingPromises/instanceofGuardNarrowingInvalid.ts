// should generate diagnostics

class Base {}
class Derived extends Base {
	async run(): Promise<void> {
		console.log("run");
	}
}

function guardedCall(instance: Base) {
	if (instance instanceof Derived) {
		// `instance` is narrowed to `Derived` here, so the call floats.
		instance.run();
	}
}
