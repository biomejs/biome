/* should not generate diagnostics */
interface CanvasStore {
	getElementById(id: string): { id: string } | undefined;
}

const store: CanvasStore = {
	getElementById(id) {
		return { id };
	},
};

export const canvasElement = store.getElementById("COVER_IMAGE");

customApi.getElementById("foo");
