/* should not generate diagnostics */

interface Invoke {
  (kind: number, callback: () => void): void;
  (kind: string, callback: () => Promise<void>): void;
}

declare const invoke: Invoke;
invoke("async", async () => {});
