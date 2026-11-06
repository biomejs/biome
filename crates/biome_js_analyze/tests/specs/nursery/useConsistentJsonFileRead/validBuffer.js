/* should not generate diagnostics */
async function buffers() {
	JSON.parse(await fs.readFile("./package.json"));
	JSON.parse(fs.readFileSync("./package.json"));
	JSON.parse(await fs.readFile("./package.json", null));
	JSON.parse(await fs.readFile("./package.json", { encoding: null }));
}

async function notUtf8() {
	JSON.parse(await fs.readFile("./package.json", "latin1"));
	JSON.parse(await fs.readFile("./package.json", encoding));
	JSON.parse(await fs.readFile("./package.json", `${encoding}`));
}

async function otherOptions() {
	JSON.parse(await fs.readFile("./package.json", { encoding: "utf8", signal }));
}

async function variableUsedElsewhere() {
	const promise = fs.readFile("./package.json", "utf8");
	console.log(promise);
	JSON.parse(await promise);
}
