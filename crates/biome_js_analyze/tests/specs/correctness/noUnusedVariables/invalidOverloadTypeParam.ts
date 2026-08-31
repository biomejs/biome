/* should generate diagnostics */
function orphan<T>(): void;
function orphan(value: string): void;

function withCallback(callback: <T>() => void): void;
function withCallback(callback: () => void): void {}
withCallback(() => {});
