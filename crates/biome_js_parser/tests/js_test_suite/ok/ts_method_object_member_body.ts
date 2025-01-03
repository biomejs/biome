({
    x<A>(maybeA: any): maybeA is A { return true },
    y(a: string): string { return "string"; },
    async *id<R>(param: Promise<R>): AsyncIterableIterator<R> { yield await param },
})
