declare function consume<T>(callback: (value: T) => void): void;

consume<number[]>((values) => {
	values.indexOf(1) !== -1;
});
