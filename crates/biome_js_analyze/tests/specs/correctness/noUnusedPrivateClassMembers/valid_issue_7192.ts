/* should not generate diagnostics */

class App {
	#persistenceRequest: Promise<boolean> | undefined;

	saveData() {
		this.#persistenceRequest ??= navigator.storage.persist();
	}
}

new App();
