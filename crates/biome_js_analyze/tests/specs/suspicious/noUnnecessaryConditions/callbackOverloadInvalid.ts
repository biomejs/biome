declare function schedule(handler: () => void): string;
declare function schedule(handler: () => Promise<void>): string | undefined;

export function scheduled() {
    return schedule(async () => {}) ?? "fallback";
}

type Mapper<T> = () => T;

declare function map<T>(mapper: Mapper<T>): T;

export function mapped(flag: boolean) {
    return map(() => 42) || flag;
}
