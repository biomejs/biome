/* should not generate diagnostics */

interface ListenOptions { port?: number; host?: string; }

interface Server {
    listen(options: ListenOptions, callback: (error: Error | null) => void): void;
    listen(options?: ListenOptions): Promise<string>;
    close(): Promise<undefined>;
    close(callback: () => void): undefined;
}

type Client = {
    connect(url: string, callback: () => void): void;
    connect(url: string): Promise<void>;
};

interface Store<T> {
    read(key: string, callback: (value: T) => void): void;
    read(key: string): Promise<T>;
}

declare const server: Server;
declare const client: Client;
declare const store: Store<number>;

// Each call selects the overload that accepts its arguments, which returns a Promise.
async function awaitPromiseOverloads(): Promise<void> {
    await server.listen({ port: 0 });
    await server.listen();
    await server.close();
    await client.connect("https://example.com");
    await store.read("key");
}
