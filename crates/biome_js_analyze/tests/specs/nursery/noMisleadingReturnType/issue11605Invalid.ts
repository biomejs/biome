declare function consume<T>(callback: (value: T) => unknown): void;

consume<"ready">((value): "ready" | "pending" => value);
