// should generate diagnostics

type Source = { run: () => Promise<void>; count: number };
type Mapped<T> = { [K in keyof T]: T[K] };
type Handlers = { [K in "start" | "stop"]: () => Promise<void> };

declare const handlers: Handlers;
handlers.start();

declare const inline: { [K in keyof Source]: Source[K] };
inline.run();

declare const mapped: Mapped<Source>;
mapped.run();

function readMapped(value: Mapped<Source>) {
	value.run();
}

declare function wrap<T>(value: T): { [K in keyof T]: () => Promise<T[K]> };
wrap({ count: 1 }).count();
