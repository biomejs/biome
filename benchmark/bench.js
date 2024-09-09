#!/usr/bin/node

import { execSync } from "node:child_process";
import * as fs from "node:fs";
import * as path from "node:path";
import * as process from "node:process";
import * as util from "node:util";

const REPOSITORIES = {
	eslint: {
		repository: "https://github.com/eslint/eslint.git",
		lintedDirs: ["lib", "messages", "tests/lib", "tests/performance", "tools"],
	},
	prettier: {
		repository: "https://github.com/prettier/prettier.git",
		formattedDirs: ["src", "scripts"],
	},
	webpack: {
		repository: "https://github.com/webpack/webpack.git",
		formattedDirs: ["lib", "examples", "benchmark"],
		lintedDirs: ["lib"],
	},
};

const TMP_DIR = path.resolve("./target");

function benchmarkFormatter(biomeBin, options) {
	// Run Dprint once to run the installer
	execSync("npx dprint --version");

	for (const [name, config] of Object.entries(REPOSITORIES)) {
		if (
			!config.formattedDirs ||
			("repository" in options && options.repository !== name)
		) {
			continue;
		}
		console.info(`\n⌛ repository: ${name}`);
		const projectDirectory = cloneProject(
			name,
			config.repository,
			config.formattedDirs,
		);
		console.info("");

		const dirs = config.formattedDirs.join(" ");
		const dirGlobs = config.formattedDirs.map((dir) => `"${dir}/**"`).join(" ");
		const biomeCommand = `${biomeBin} format --config-path=../../ --write --max-diagnostics=0 ${dirs}`;
		const benchCommands = {
			prettier: `../../node_modules/.bin/prettier --config=../../.prettierrc.json --ignore-path=../../.prettierignore  --write --log-level=error ${dirs}`,
			// FIXME: Parallel Prettier is crashing on Node 22
			// "parallel-prettier": `../../node_modules/.bin/pprettier --write --concurrency ${os.cpus().length} ${dirGlobs}`,
			dprint: `../../node_modules/dprint/dprint fmt --config=../../dprint.json ${dirGlobs}`,
			biome: biomeCommand,
			"biome-1-thread": withEnvVariable("RAYON_NUM_THREADS", "1", biomeCommand),
		};

		let suite = "";
		for (const benchName of Object.keys(benchCommands)) {
			if ("bench" in options && !options.bench.includes(benchName)) {
				continue;
			}
			suite += ` -n "${benchName}" '${benchCommands[benchName]}'`;
		}
		if (suite.length === 0) {
			console.error(`Benchmark '${options.bench}' doesn't exist.`);
			process.exit(1);
		}

		// Run 2 warmups to make sure the files are formatted correctly
		let hyperfineCommand = `hyperfine --warmup 2 --shell=${shellOption()} ${suite}`;
		if (options.verbose) {
			hyperfineCommand += " --show-output";
			console.info(`${hyperfineCommand}\n`);
		}

		execSync(hyperfineCommand, {
			cwd: projectDirectory,
			stdio: "inherit",
		});
	}
}

function benchmarkLinter(biomeBin, options) {
	for (const [name, config] of Object.entries(REPOSITORIES)) {
		if (
			!config.lintedDirs ||
			("repository" in options && options.repository !== name)
		) {
			continue;
		}
		console.info(`\n⌛ repository: ${name}`);
		const projectDirectory = cloneProject(
			name,
			config.repository,
			config.formattedDirs,
		);
		console.info("");

		const dirs = config.lintedDirs.map((dir) => `"${dir}"`).join(" ");
		const biomeCmd = `"${biomeBin}" lint --config-path=../../ --max-diagnostics=0 ${dirs}`;
		const benchCommands = {
			eslint: `../../node_modules/.bin/eslint --quiet --config=../../eslint.config.js --no-ignore ${dirs}`,
			"ts-eslint": `../../node_modules/.bin/eslint --quiet --config=../../ts-eslint.config.js --no-ignore ${dirs}`,
			biome: biomeCmd,
			"biome-1-thread": withEnvVariable("RAYON_NUM_THREADS", "1", biomeCmd),
			"ts-biome": `"${biomeBin}" lint --config-path=../../ts-biome.json --max-diagnostics=0 ${dirs}`,
		};

		let suite = "";
		for (const benchName of Object.keys(benchCommands)) {
			if ("bench" in options && !options.bench.includes(benchName)) {
				continue;
			}
			suite += ` -n "${benchName}" '${benchCommands[benchName]}'`;
		}
		if (suite.length === 0) {
			console.error(`Benchmark '${options.bench}' doesn't exist.`);
			process.exit(1);
		}

		let hyperfineCommand = `hyperfine --ignore-failure --shell=${shellOption()} ${suite}`;
		if (options.verbose) {
			hyperfineCommand += " --show-output";
			console.info(`${hyperfineCommand}\n`);
		}

		execSync(hyperfineCommand, {
			cwd: projectDirectory,
			stdio: "inherit",
		});
	}
}

function shellOption() {
	if (process.platform === "win32") {
		// Use Powershell so that it is possible to set an environment variable for a single command (ugh!)
		return "powershell";
	}
	return "default";
}

function withEnvVariable(name, value, command) {
	switch (process.platform) {
		case "win32": {
			return `$Env:${name}=${value}; ${command}`;
		}
		default:
			return `${name}="${value}" ${command}`;
	}
}

function withDirectory(cwd) {
	return {
		run(command, options) {
			execSync(command, {
				cwd,
				...options,
			});
		},
	};
}

function cloneProject(name, repository, dirs = []) {
	const projectDirectory = path.join(TMP_DIR, name);

	const inProjectDirectory = withDirectory(projectDirectory);

	if (fs.existsSync(projectDirectory)) {
		inProjectDirectory.run("git reset --hard @{u}");
		inProjectDirectory.run("git clean -df");
		inProjectDirectory.run("git pull --quiet --depth=1 --ff-only");
	} else {
		withDirectory(TMP_DIR).run(
			`git clone --quiet --depth=1 ${dirs.length > 0 ? "--sparse" : ""} ${repository}`,
			{
				stdio: "inherit",
			},
		);
	}

	if (dirs.length > 0) {
		console.info(
			`Adding directories ${dirs.join()} to sparse checkout in ${projectDirectory}`,
		);
		inProjectDirectory.run(`git sparse-checkout add ${dirs.join(" ")}`);
	}

	return projectDirectory;
}

function buildBiome() {
	console.info("Building Biome...");
	execSync(
		withEnvVariable(
			"BIOME_VERSION",
			"0.0.0",
			"cargo build --bin biome --release",
		),
		{
			stdio: "inherit",
		},
	);
	return path.resolve("../target/release/biome");
}

function run({ positionals, values: options }) {
	fs.mkdirSync(TMP_DIR, { recursive: true });
	let biomeBinPath;
	if ("biome" in options) {
		biomeBinPath = options.biome;
	} else {
		biomeBinPath = buildBiome();
	}
	switch (positionals[0]) {
		case "formatter":
			benchmarkFormatter(biomeBinPath, options);
			break;
		case "linter":
			benchmarkLinter(biomeBinPath, options);
			break;
		default:
			console.error(
				`A positional argument among 'formatter' and 'linter' must be passed.`,
			);
			process.exit(1);
	}
}

run(
	util.parseArgs({
		allowPositionals: true,
		options: {
			bench: {
				multiple: true,
				type: "string",
			},
			biome: {
				type: "string",
			},
			repository: {
				type: "string",
			},
			verbose: {
				type: "boolean",
			},
		},
	}),
);
