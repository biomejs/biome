declare function consume<T>(callback: (value: T) => void): void;

consume<"ready" | "pending">((value) => {
	if (value) {
		console.log(value);
	}
});
