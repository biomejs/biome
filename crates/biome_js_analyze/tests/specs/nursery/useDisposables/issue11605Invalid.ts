declare function consume<T>(callback: (value: T) => void): void;

consume<Disposable>((value) => {
	const resource = value;
});
