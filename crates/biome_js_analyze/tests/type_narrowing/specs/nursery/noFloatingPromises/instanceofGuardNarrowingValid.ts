/* should not generate diagnostics */

class Base {}
class DerivedSync extends Base {
	run(): void {}
}

// Narrowing must not turn `run` into a promise-returning method.
function syncCall(instance: Base) {
	if (instance instanceof DerivedSync) {
		instance.run();
	}
}

// A same-name class declared in the consequent could shadow the one the
// guard checked against, so narrowing is declined and `instance` stays `Base`.
class Shadow extends Base {
	run(): void {}
}

function shadowedGuardClass(instance: Base) {
	if (instance instanceof Shadow) {
		class Shadow extends Base {
			async run(): Promise<void> {}
		}
		instance.run();
	}
}
