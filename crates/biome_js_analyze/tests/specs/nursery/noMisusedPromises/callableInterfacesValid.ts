// should not generate diagnostics
interface Handler {
	(): void;
}

interface AsyncHandler {
	(): Promise<void>;
}

interface OverloadedActual {
	(): Promise<void>;
	(value: string): Promise<void>;
}

interface MixedActual {
	(): void;
	(value: string): Promise<void>;
}

interface OverloadedExpected {
	(): void;
	(value: string): Promise<void>;
}

interface CyclicA extends CyclicB {}
interface CyclicB extends CyclicA {}

declare const overloadedActual: OverloadedActual;
declare const mixedActual: MixedActual;
declare const maybeAsync: Handler | AsyncHandler;
declare const cyclicActual: CyclicA;
declare function consumeVoid(callback: () => void): void;
declare function consumeOverloaded(callback: OverloadedExpected): void;
declare function consumeUnion(callback: Handler | AsyncHandler): void;

consumeVoid(overloadedActual);
consumeVoid(mixedActual);
consumeVoid(maybeAsync);
consumeVoid(cyclicActual);
consumeOverloaded(async () => {});
consumeUnion(async () => {});
