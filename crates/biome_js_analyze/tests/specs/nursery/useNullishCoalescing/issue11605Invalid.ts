declare function consume<T>(callback: (value: T) => void): void;

consume<string | null>((value) => {
	value || "default";
});
