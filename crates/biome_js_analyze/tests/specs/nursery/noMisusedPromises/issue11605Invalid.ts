declare function consume<T>(callback: (value: T) => void): void;

consume<Promise<boolean>>((value) => {
	if (value) {
		console.log("resolved");
	}
});
