const { promises: fs } = require('fs');
const path = require('path');
const prettier = require("prettier");

if (process.argv.length < 3) {
	console.error('Usage: node prepare_tests.js <prettier root>');
	process.exit(2);
}

const defaultConfig = {
	trailingComma: 'all',
	tabWidth: 2,
	printWidth: 80,
	singleQuote: false,
	jsxSingleQuote: false,
	useTabs: false,
	embeddedLanguageFormatting: 'off'
};

const PRETTIER_ROOT = path.resolve(process.argv[2], 'tests/format');

async function extractPrettierTests(type, config) {
	const root = path.resolve(PRETTIER_ROOT, type);
	console.log('Extracting tests from %s ...', root);
	await traverseDir(root, config);
}

// Recursively traverse the test directory to search for snapshots files
async function traverseDir(dir, input_config) {
	const config = {
		...input_config,
		...defaultConfig
	};

	for (const entry of await fs.readdir(dir, { withFileTypes: true })) {
		if (entry.isDirectory()) {
			await traverseDir(path.resolve(dir, entry.name), config);
			continue;
		}

		if (entry.isFile()) {
			const file = entry.name;

			// Ignore spec files
			if (file.startsWith('format.test')) {
				continue;
			}

			// Compute a relative path from the Prettier root directory
			// to this file, then an absolute path using the biome_js_formatter
			// specs directory as a root instead
			const filePath = path.resolve(dir, file);
			const relPath = path.relative(PRETTIER_ROOT, filePath);
			const outPath = path.resolve(process.cwd(), relPath);
			const snapshotPath = path.resolve(
				dir,
				'__snapshots__',
				'format.test.js.snap'
			);
			const snapFile = path.basename(file) + '.prettier-snap';
			const snapOriginalFile = path.basename(file) + '.prettier-snap-original';

			const snapshot = require(snapshotPath);

			const key = `${file} format 1`;

			if (key in snapshot) {
				let snapshotContent = String(snapshot[key]);

				// Copy the snapshot input file, ensuring the
				// parent directory exists
				const outDir = path.resolve(outPath, '..');
				await fs.mkdir(outDir, { recursive: true });
				await fs.copyFile(filePath, outPath);
				// Extract the expected output from the snapshot text
				const INPUT =
					"=====================================input======================================";
				const OUTPUT =
					'=====================================output=====================================';
				const OPTIONS =
					"====================================options=====================================";
				const FOOTER =
					'================================================================================';

				// extract options string
				const optionsStart = snapshotContent.match(new RegExp(OPTIONS + '\\n'));
				const optionsEnd = snapshotContent.match(new RegExp('\\n' + INPUT));
				const optionsStartOffset = optionsStart.index + optionsStart[0].length;
				const optionsEndOffset = optionsEnd.index;
				const optionsContent = snapshotContent.substring(optionsStartOffset, optionsEndOffset);

				// if range options are not defined, use default value
				// https://prettier.io/docs/en/options#range
				const rangeOptions = {
					rangeStart: Number(optionsContent.match(new RegExp(/rangeStart: (\d+)/))?.[1] ?? 0),
					rangeEnd: Number(optionsContent.match(new RegExp(/rangeEnd: (\d+)/))?.[1] ?? Infinity)
				};

				// extract output string
				const outputStart = snapshotContent.match(new RegExp(OUTPUT + '\\n'));
				const outputEnd = snapshotContent.match(new RegExp('\\n' + FOOTER));

				const outputStartOffset = outputStart.index + outputStart[0].length;
				const outputEndOffset = outputEnd.index;
				snapshotContent = snapshotContent.substring(outputStartOffset, outputEndOffset);

				let originalSnapshot = snapshotContent;
				try {
					// We need to reformat prettier snapshot
					// because Biome and Prettier have different default options
					const updatedOptions = { ...rangeOptions, ...config };
					snapshotContent = await prettier.format(snapshotContent, updatedOptions);
				} catch (error) {
					console.error(`Prettier format error in ${filePath}: ${error}`);
				}

				if (originalSnapshot !== snapshotContent) {
					// Prettier has a reformat issue
					await fs.writeFile(path.resolve(outDir, snapOriginalFile), originalSnapshot);
				}

				// Write the expected output to an additional prettier-snap
				// file in the specs directory
				await fs.writeFile(path.resolve(outDir, snapFile), snapshotContent);
			} else {
				// Load content from file current fule
				const content = await fs.readFile(filePath, { encoding: 'utf8' });

				try {
					// Try to format input with prettier
					const prettierOutput = await prettier.format(content, config);

					const outDir = path.resolve(outPath, '..');
					await fs.mkdir(outDir, { recursive: true });
					await fs.copyFile(filePath, outPath);

					// Write the expected output to an additional prettier-snap
					// file in the specs directory
					await fs.writeFile(path.resolve(outDir, snapFile), prettierOutput);
				} catch (error) {
					console.error(`Prettier format error in ${filePath}: ${error}`);
				}
			}
		}
	}
}

module.exports = { extractPrettierTests };
