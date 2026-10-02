declare function consume<T>(callback: (value: T) => void): void;

consume<number[]>((value) => {
	value.sort();
});
