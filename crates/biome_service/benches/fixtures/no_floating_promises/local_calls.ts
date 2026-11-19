async function load(): Promise<number> {
	return 1;
}

class Loader {
	async load(): Promise<number> {
		return 1;
	}
}

const loader = new Loader();

load();
load().then(() => {});
loader.load();
loader.load().finally(() => {});
Promise.resolve(1);
Promise.all([load()]);

load().then(
	() => {},
	() => {},
);
load().catch(() => {});

async function handled() {
	await load();
	void load();
}

void handled();

let assigned: Promise<number>;
assigned = load();
