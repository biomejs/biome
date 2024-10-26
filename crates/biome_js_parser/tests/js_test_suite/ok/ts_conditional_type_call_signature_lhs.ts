type X<V> = V extends (...args: any[]) => any ? (...args: Parameters<V>) => void : Function;
