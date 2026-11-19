declare function consume<T>(callback: (value: T) => void): void;

consume<"open" | "closed">((value) => {
	switch (value) {
		case "open":
			break;
	}
});
