/* should not generate diagnostics */

class Base {}
class DerivedSync extends Base {
	run(): void {}
}

// The narrowed method does not return a promise.
function syncCall(instance: Base) {
	if (instance instanceof DerivedSync) {
		instance.run();
	}
}

// A same-name class declared in the consequent shadows the guard class;
// narrowing must bail out instead of resolving the guard to the inner class.
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
