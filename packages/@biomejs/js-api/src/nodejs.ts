import { readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, extname, join, resolve } from "node:path";
import type { Configuration, Diagnostic } from "@biomejs/wasm-nodejs";
import * as moduleNodeJs from "@biomejs/wasm-nodejs";
import { BiomeCommon } from "./common";
import type {
	Module,
	OpenProjectResult,
	ProjectKey,
	Workspace,
} from "./wasm";
import { tryCatchWrapper } from "./wasm";

export type * from "./common";
export type { Configuration, Diagnostic };

interface NodeJsMemoryFileSystem {
	insert(path: string, data: Uint8Array): void;
}

type NodeJsWorkspaceConstructor = Module<Configuration, Diagnostic>["Workspace"] & {
	withFileSystem(fs: NodeJsMemoryFileSystem): Workspace<Configuration, Diagnostic>;
};

interface NodeJsModule extends Module<Configuration, Diagnostic> {
	MemoryFileSystem: new () => NodeJsMemoryFileSystem;
	Workspace: NodeJsWorkspaceConstructor;
}

interface PluginAwareConfiguration {
	plugins?: string[] | undefined;
	overrides?: Array<{
		plugins?: string[] | undefined;
	}> | undefined;
}

const nodeModule = moduleNodeJs as NodeJsModule;

function resolveProjectRoot(path = process.cwd()): string {
	const resolvedPath = resolve(path);

	try {
		return statSync(resolvedPath).isDirectory() ? resolvedPath : dirname(resolvedPath);
	} catch {
		return resolvedPath;
	}
}

export class Biome extends BiomeCommon<Configuration, Diagnostic> {
	private readonly fs: NodeJsMemoryFileSystem;
	private readonly projectRoots = new Map<ProjectKey, string>();

	constructor() {
		const fs = new nodeModule.MemoryFileSystem();
		super(nodeModule, nodeModule.Workspace.withFileSystem(fs));
		this.fs = fs;
	}

	override shutdown() {
		this.projectRoots.clear();
		super.shutdown();
	}

	override openProject(path?: string): OpenProjectResult {
		const projectRoot = resolveProjectRoot(path);
		const result = this.workspace.openProject({
			path: projectRoot,
			openUninitialized: true,
		});
		this.projectRoots.set(result.projectKey, projectRoot);
		return result;
	}

	override applyConfiguration(
		projectKey: ProjectKey,
		configuration: Configuration,
	): void {
		const projectRoot =
			this.projectRoots.get(projectKey) ?? resolveProjectRoot(process.cwd());

		this.syncPlugins(projectRoot, configuration as PluginAwareConfiguration);

		tryCatchWrapper(() => {
			this.workspace.updateSettings({
				projectKey,
				configuration,
				workspaceDirectory: projectRoot,
			});
		});
	}

	private syncPlugins(
		projectRoot: string,
		configuration: PluginAwareConfiguration,
	): void {
		const pluginEntries = new Set<string>();

		for (const pluginPath of configuration.plugins ?? []) {
			pluginEntries.add(resolve(projectRoot, pluginPath));
		}

		for (const override of configuration.overrides ?? []) {
			for (const pluginPath of override.plugins ?? []) {
				pluginEntries.add(resolve(projectRoot, pluginPath));
			}
		}

		for (const pluginPath of pluginEntries) {
			this.syncPluginPath(pluginPath);
		}
	}

	private syncPluginPath(pluginPath: string): void {
		try {
			const stats = statSync(pluginPath);

			if (stats.isDirectory()) {
				this.syncDirectory(pluginPath);
				return;
			}

			if (extname(pluginPath) === ".js" || extname(pluginPath) === ".mjs") {
				this.syncJavaScriptPlugin(pluginPath);
				return;
			}

			this.syncFile(pluginPath);
		} catch {
			// Let the wasm-side plugin loading report the configuration error.
		}
	}

	private syncDirectory(directoryPath: string): void {
		for (const entry of readdirSync(directoryPath, { withFileTypes: true })) {
			const entryPath = join(directoryPath, entry.name);
			if (entry.isDirectory()) {
				this.syncDirectory(entryPath);
			} else if (entry.isFile()) {
				this.syncFile(entryPath);
			}
		}
	}

	private syncFile(filePath: string): void {
		this.fs.insert(resolve(filePath), readFileSync(filePath));
	}

	private syncJavaScriptPlugin(
		filePath: string,
		seen = new Set<string>(),
	): void {
		const resolvedPath = resolve(filePath);
		if (seen.has(resolvedPath)) {
			return;
		}

		seen.add(resolvedPath);

		const sourceBuffer = readFileSync(resolvedPath);
		const source = sourceBuffer.toString("utf8");
		this.fs.insert(resolvedPath, sourceBuffer);

		for (const specifier of this.findRelativeImports(source)) {
			const importedPath = this.resolveImportPath(resolvedPath, specifier);
			if (!importedPath) {
				continue;
			}

			if (extname(importedPath) === ".js" || extname(importedPath) === ".mjs") {
				this.syncJavaScriptPlugin(importedPath, seen);
			} else {
				this.syncFile(importedPath);
			}
		}
	}

	private findRelativeImports(source: string): string[] {
		const imports = new Set<string>();
		const importPattern =
			/(?:import|export)\s+(?:[^"'`]*?\s+from\s+)?["']([^"'`]+)["']|import\(\s*["']([^"'`]+)["']\s*\)/g;

		for (const match of source.matchAll(importPattern)) {
			const specifier = match[1] ?? match[2];
			if (specifier?.startsWith("./") || specifier?.startsWith("../")) {
				imports.add(specifier);
			}
		}

		return [...imports];
	}

	private resolveImportPath(
		importerPath: string,
		specifier: string,
	): string | undefined {
		const basePath = resolve(dirname(importerPath), specifier);
		const candidates = [
			basePath,
			`${basePath}.js`,
			`${basePath}.mjs`,
			`${basePath}.json`,
			join(basePath, "index.js"),
			join(basePath, "index.mjs"),
		];

		for (const candidate of candidates) {
			try {
				if (statSync(candidate).isFile()) {
					return candidate;
				}
			} catch {
				continue;
			}
		}

		return undefined;
	}
}
