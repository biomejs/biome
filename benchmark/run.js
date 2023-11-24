const fs = require("fs");
const child_process = require("child_process");
const path = require("path");
const os = require("os");
const { dir } = require("console");

const TMP_DIRECTORY = path.resolve("./target");

function buildRome() {
	console.log("Build Biome...");

	child_process.execSync("cargo build --bin biome --release", {
		stdio: "inherit",
	});

	return path.resolve("../target/release/biome");
}

const BENCHMARKS = {
	formatter: {
		webpack: {
			repository: "https://github.com/webpack/webpack.git",
			sourceDirectories: {
				lib: ["js"],
				examples: ["js"],
				declarations: ["ts"],
				benchmark: ["js"],
			},
		},
		prettier: {
			repository: "https://github.com/prettier/prettier.git",
			sourceDirectories: {
				src: ["js"],
				scripts: ["js"],
			},
		},
	},
	linter: {
		eslint: {
			repository: "https://github.com/eslint/eslint.git",
			sourceDirectories: [
				"lib",
				"messages",
				"tests/lib",
				"tests/performance",
				"tools",
			],
		},
		webpack: {
			repository: "https://github.com/webpack/webpack.git",
			sourceDirectories: ["lib"],
		},
	},
};

function getDirsToClone(sourceDirs) {
	if (typeof sourceDirs !== 'object' || sourceDirs === null) {
		return;
	}

	if (Array.isArray(sourceDirs)) {
		return sourceDirs;
	}

	return Object.keys(sourceDirs);
}

function benchmarkFormatter(rome) {
	console.log("");
	console.log("Benchmark formatter...");
	console.log("―".repeat(80));
	console.log("");

	// Run Dprint once to run the installer
	child_process.execSync("npx dprint --version");

	for (const [name, configuration] of Object.entries(BENCHMARKS.formatter)) {
		console.log(`[${name}]`);

		let projectDirectory = cloneProject(name, configuration.repository, getDirsToClone(configuration.sourceDirectories));

		const prettierPaths = Object.entries(configuration.sourceDirectories)
			.flatMap(([directory, extensions]) => {
				return extensions.map(
					(extension) => `'${path.join(directory, `**/*.${extension}`)}'`,
				);
			})
			.join(" ");

		const prettierCommand = `node '${resolvePrettier()}' ${prettierPaths} --write --loglevel=error`;
		const parallelPrettierCommand = `node '${resolveParallelPrettier()}' ${prettierPaths} --write --concurrency ${os.cpus().length}`;

		const dprintCommand = `${resolveDprint()} fmt --incremental=false --config '${require.resolve("./dprint.json")}' ${Object.keys(configuration.sourceDirectories).map(path => `'${path}/**/*'`).join(" ")}`;

		const romeCommand = `${rome} format --max-diagnostics=0 ${Object.keys(
			configuration.sourceDirectories,
		)
			.map((path) => `'${path}'`)
			.join(" ")} --write`;

		const romeSingleCoreCommand = withEnvVariable(
			"RAYON_NUM_THREADS",
			"1",
			romeCommand,
		);

		// Run 2 warmups to make sure the files are formatted correctly
		const hyperfineCommand = `hyperfine --show-output -w 2 -n Prettier "${prettierCommand}" -n "Parallel-Prettier" "${parallelPrettierCommand}" -n dprint "${dprintCommand}" -n Rome "${romeCommand}" --shell=${shellOption()} -n "Rome (1 thread)" "${romeSingleCoreCommand}"`;
		console.log(hyperfineCommand);

		child_process.execSync(hyperfineCommand, {
			cwd: projectDirectory,
			stdio: "inherit",
		});
	}
}

function resolvePrettier() {
	return path.resolve("node_modules/prettier/standalone.js");
}

function resolveParallelPrettier() {
	return path.resolve("node_modules/@mixer/parallel-prettier/dist/index.js");
}

function resolveDprint() {
	return path.resolve("node_modules/dprint/dprint");
}

function benchmarkLinter(rome) {
	console.log("");
	console.log("Benchmark linter...");
	console.log("―".repeat(80));
	console.log("");

	for (const [name, configuration] of Object.entries(BENCHMARKS.linter)) {
		console.log(`[${name}]`);

		const projectDirectory = cloneProject(name, configuration.repository, getDirsToClone(configuration.sourceDirectories));

		deleteFile(path.join(projectDirectory, ".eslintignore"));
		deleteFile(path.join(projectDirectory, "/eslintrc.js"));

		// Override eslint config
		const eslintConfig = fs.readFileSync("./bench.eslint.js");
		fs.writeFileSync(path.join(projectDirectory, "eslint.config.js"), eslintConfig);

		const romeConfig = fs.readFileSync("./bench.rome.json");
		fs.writeFileSync(path.join(projectDirectory, "rome.json"), romeConfig);

		const eslintPaths = configuration.sourceDirectories
			.map((directory) => `'${directory}/**'`)
			.join(" ");

		const eslintCommand = `node '${resolveESlint()}' --no-ignore ${eslintPaths}`;

		const romePaths = configuration.sourceDirectories
			.map((directory) => `'${directory}'`)
			.join(" ");

		// Don't compute the code frames for pulled diagnostics. ESLint doesn't do so as well.
		const romeCommand = `${rome} check --max-diagnostics=0 ${romePaths}`;

		const romeSingleCoreCommand = withEnvVariable(
			"RAYON_NUM_THREADS",
			"1",
			romeCommand,
		);

		// Run 2 warmups to make sure the files are formatted correctly
		const hyperfineCommand = `hyperfine -i -w 2 -n ESLint "${eslintCommand}" -n Rome "${romeCommand}" --shell=${shellOption()} -n "Rome (1 thread)" "${romeSingleCoreCommand}"`;
		console.log(hyperfineCommand);

		child_process.execSync(hyperfineCommand, {
			cwd: projectDirectory,
			stdio: "inherit",
		});
	}
}

function resolveESlint() {
	return path.resolve("node_modules/eslint/bin/eslint.js");
}

function shellOption() {
	switch (process.platform) {
		case "win32":
			// Use Powershell so that it is possible to set an environment variable for a single command (ugh!)
			return "powershell";
		default:
			return "default";
	}
}

function withEnvVariable(name, value, command) {
	switch (process.platform) {
		case "win32": {
			return `$Env:${name}=${value}; ${command}`;
		}
		default:
			return `${name}=\"${value}\" ${command}`;
	}
}

function withDirectory(cwd) {
	return {
		run(command, options) {
			child_process.execSync(command, {
				cwd,
				...options,
			});
		},
	};
}

function deleteFile(path) {
	if (fs.existsSync(path)) {
		fs.rmSync(path);
	}
}

function cloneProject(name, repository, dirs = []) {
	let projectDirectory = path.join(TMP_DIRECTORY, name);

	let inProjectDirectory = withDirectory(projectDirectory);

	if (fs.existsSync(projectDirectory)) {
		console.log(`Updating git repository in directory ${projectDirectory}`);
		inProjectDirectory.run("git reset --hard @{u}");
		inProjectDirectory.run("git clean -df");
		inProjectDirectory.run("git pull --depth=1 --ff-only");
	} else {
		console.log("Clone project...");

		withDirectory(TMP_DIRECTORY).run(`git clone ${dirs.length > 0 ? '--sparse' : ''} --depth=1 ${repository}`, {
			stdio: "inherit",
		});
	}

	if (dirs.length > 0) {
		console.log(`Adding directories ${dirs.join()} to sparse checkout in ${projectDirectory}`)
		inProjectDirectory.run(`git sparse-checkout add ${dirs.join(' ')}`);
	}

	return projectDirectory;
}

function run() {
	fs.mkdirSync("target", { recursive: true });

	const rome = buildRome();

	benchmarkFormatter(rome);
	benchmarkLinter(rome);
}

run();
