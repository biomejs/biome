// Run `BIOME_VERSION=<version number> cargo codegen-website
// to generate a new schema
import type { InferGetStaticParamsType } from "astro";

async function enumVersions(): Promise<string[]> {
	const apiUrl = "https://registry.npmjs.org/@biomejs/biome";
	console.log("fetching version list", apiUrl);
	const response = await fetch(apiUrl, {
		headers: {
			"Accept": "application/vnd.npm.install-v1+json" // abbreviated version of the payload to reduce network impact
		}
	});

	if (!response.ok) {
		throw new Error(`Failed to retrieve all versions of @biomejs/biome: ${response.status} ${response.statusText}`);
	}

	type PackageInfoREST = {
		versions: Record<string, unknown>; // Don't care about the details of the version
	};

	const data: PackageInfoREST = await response.json(); // Eventually add a schema validation here

	return Object.keys(data.versions);
}

type PackageFile = {
	filePath: string;
	hex: string;
	packageName: string;
	version: string;
};

async function enumPackageFiles(packageName: string, version: string): Promise<Record<string, PackageFile>> {

	// ex: https://www.npmjs.com/package/@biomejs/biome/v/1.1.2/index
	const fileList = `https://www.npmjs.com/package/${packageName}/v/${version}/index`;

	console.log(`fetching files for version ${version}`, fileList);
	const response = await fetch(fileList);

	if (!response.ok) {
		throw new Error(`Failed to retrieve file list for ${packageName}: ${response.status} ${response.statusText}`);
	}

	type FileListREST = {
		files: Record<string, { hex: string; }>; // only relevant fields
	};

	const data: FileListREST = await response.json();

	return Object.entries(data.files).reduce((acc, [filePath, { hex }]) => {
		acc[filePath] = {
			filePath,
			hex,
			packageName,
			version
		};
		return acc;
	}, {} as Record<string, PackageFile>);
}

async function downloadSingleFile(file: PackageFile): Promise<string> { // Only support text files
	const { hex, packageName } = file;
	const fileContentUrl = `https://www.npmjs.com/package/${packageName}/file/${hex}`;

	console.log(`fetching ${JSON.stringify(file)}`, fileContentUrl);
	const response = await fetch(fileContentUrl);

	if (!response.ok) {
		throw new Error(`Failed to retrieve file content for ${JSON.stringify(file)}: ${response.status} ${response.statusText}`);
	}

	return response.text();
}

export async function getStaticPaths() {

	const publishedVersions = await enumVersions();

	console.log(publishedVersions);

	return publishedVersions.map((version) => {
		return { params: { tag: version } };
	});
}

type Params = InferGetStaticParamsType<typeof getStaticPaths>;
// type Props = InferGetStaticPropsType<typeof getStaticPaths>;



export async function get({ params }: { params: Params; }) {
	console.log(params);

	const files = await enumPackageFiles("@biomejs/biome", params.tag);

	const schemaFile = files["/configuration_schema.json"];
	if (!schemaFile) {
		return new Response("Not Found", {
			status: 404,
			headers: {
				"content-type": "text/plain",
			},
		});
	}

	const schema = await downloadSingleFile(schemaFile);

	return new Response(schema, {
		status: 200,
		headers: {
			"content-type": "application/json",
		},
	});
}
