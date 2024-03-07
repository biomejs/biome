function fetchData() {
	return "data";
}

async function fetchDataAsync() {
	const response = await fetch('/data');
	const data = await response.json();
	return data;
}

async function doNothingAsync() {
}

(async function fetchDataImmediate() {
	const response = await fetch('/data');
	const data = await response.json();
	console.log(data);
})();

(async function doNothingImmediate() {
})();

const dataFetcher = {
	async fetchData() {
		const response = await fetch('/data');
		const data = await response.json();
		return data;
	},
};

const noopObject = {
	async doNothing() {
	}
};

class DataFetcherClass {
	async fetchData() {
		const response = await fetch('/data');
		const data = await response.json();
		return data;
	}
}

class NoOperationClass {
	async doNothing() {
	}
}

async function wrapperFetchData() {
	return await fetchDataAsync();
}

async function forAwaitOf () {
	let sum = 0;
	for await (const n of [1, 2, 3]) {
		sum += n
	}
	return sum;
};

async function awaitExpressionWithForOf () {
	let sum = await initialSum();
	for (const n of [1, 2, 3]) {
		sum += n
	}
	return sum;
};

