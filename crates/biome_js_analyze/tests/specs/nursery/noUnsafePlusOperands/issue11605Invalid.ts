declare function consume<T>(callback: (value: T) => void): void;

consume<bigint>((value) => {
	value + 1;
});
