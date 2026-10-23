interface ListenOptions {
	port?: number;
}

interface Instance {
	listen(opts: ListenOptions, callback: (err: Error | null, address: string) => void): void;
	listen(opts?: ListenOptions): Promise<string>;
}

declare const app: Instance;

// The no-callback overload returns a Promise.
await app.listen({ port: 0 });
await app.listen();

// The callback overload returns void.
await app.listen({ port: 0 }, () => {});

declare const literal: {
	listen(opts: ListenOptions, callback: () => void): void;
	listen(opts?: ListenOptions): Promise<string>;
};

await literal.listen({ port: 0 });
await literal.listen({ port: 0 }, () => {});
