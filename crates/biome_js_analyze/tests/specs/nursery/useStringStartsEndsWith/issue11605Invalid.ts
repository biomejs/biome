declare function consume<T>(callback: (value: T) => void): void;

consume<string>((value) => {
	value[0] === "v";
});
