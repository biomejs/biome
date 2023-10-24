type ValidIndexSignatures = {
	a: {
		[index: string]: string;
	};
	b: {
		[index: string]: string;
	};
};

interface I {
	[index: number]: string;
	[index: string]: string;
}

// See https://github.com/biomejs/biome/issues/175
const x: { [key: string]: string } = {};
let key;

let a: { [key: string]: string };
let b: { [key: string]: string };
