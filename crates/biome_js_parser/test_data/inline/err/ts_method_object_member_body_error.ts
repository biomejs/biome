({
    x<>(maybeA: any): maybeA is A { return true },
    async *id<>(param: Promise<R>): AsyncIterableIterator<R> { yield await param },
})
