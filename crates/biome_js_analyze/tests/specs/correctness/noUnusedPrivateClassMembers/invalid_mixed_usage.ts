/* should generate diagnostics */
class MixedUsage {
	#writtenThenRead = 0;
	#onlyWritten = 0;
	private readLater = 0;
	private onlyUpdated = 0;
	#unused = 0;
	private destructured = 0;
	private alsoUnused = 0;

	method() {
		this.#writtenThenRead = 1;
		this.#onlyWritten = 1;
		this.onlyUpdated++;
		const { destructured } = this;
		return [this.#writtenThenRead, this.readLater, destructured];
	}
}

class ComputedAfterUse {
	private used = 0;
	private viaComputed = 0;
	#sharpUnused = 0;

	method(key: string) {
		return [this.used, this[key]];
	}
}
