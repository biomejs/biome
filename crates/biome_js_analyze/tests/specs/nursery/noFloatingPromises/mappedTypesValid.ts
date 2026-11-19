// should not generate diagnostics

type Source = { run: () => Promise<void>; count: number };
type Mapped<T> = { [K in keyof T]: T[K] };
type Sync = { [K in keyof Source]: () => void };

declare const sync: Sync;
sync.run();

declare const mapped: Mapped<Source>;
await mapped.run();
void mapped.run();
mapped.run().catch(() => {});

declare const remapped: { [K in keyof Source as Uppercase<K>]: Source[K] };
remapped.RUN();

interface Merged { run: () => Promise<void> }
declare const fromInterface: Mapped<Merged>;
fromInterface.run();
