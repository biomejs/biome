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

class OtherDerived extends Base {
	async run(): Promise<void> {
		console.log("run");
	}
}

// Each guard narrows to the class it names, so a sibling subclass floats too.
function guardedCallOnSibling(instance: Base) {
	if (instance instanceof OtherDerived) {
		instance.run();
	}
}
