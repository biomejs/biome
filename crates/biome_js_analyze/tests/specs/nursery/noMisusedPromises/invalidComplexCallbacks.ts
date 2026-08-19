/* should generate diagnostics */

type Prefix<T> = [T];

declare function consumeTuple(
  ...args: [...Prefix<number>, () => void]
): void;
const tuplePrefix: Prefix<number> = [1];
consumeTuple(...tuplePrefix, async () => {});

type Recursive<T> = [...Recursive<T>];

declare function consumeRecursive(
  ...args: [...Recursive<number>, () => void]
): void;
consumeRecursive(async () => {});

interface InterfaceConstructor {
  new (callback: () => void): object;
}
declare const InterfaceConsumer: InterfaceConstructor;
new InterfaceConsumer(async () => {});

declare const ObjectConsumer: {
  new (callback: () => void): object;
};
new ObjectConsumer(async () => {});
