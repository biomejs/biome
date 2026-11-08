/* should generate diagnostics */

// Test A — Original issue: zero versus one parameter
interface Api {
	run(): void;
	run(input: string): void;
}

type RunArgs = Parameters<Api["run"]>;

const args = ["value"] as RunArgs;
const requiredArgs: [string] = args;

console.log(requiredArgs);

// Test B — One versus two required parameters
interface Api {
	send(message: string): void;
	send(message: string, priority: number): void;
}

// Test C — Identical duplicate signatures
interface Api {
	calculate(value: number): number;
	calculate(value: number): number;
}

// Test D — Existing optional parameter
interface Api {
	update(id: number): void;
	update(id: number, name?: string): void;
}

