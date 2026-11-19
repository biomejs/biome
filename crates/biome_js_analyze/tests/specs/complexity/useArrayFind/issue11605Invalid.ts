declare function consume<T>(callback: (value: T) => void): void;

consume<0>((index) => {
	[1, 2, 3].filter((value) => value > 1)[index];
});
