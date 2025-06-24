// Based on: https://github.com/EndBug/version-check
// Copyright (c) 2019 Federico Grandi (MIT License)

import { readFileSync } from "node:fs";
import { join } from "node:path";

const semverReGlobal =
	/(?<major>0|[1-9]\d*)\.(?<minor>0|[1-9]\d*)\.(?<patch>0|[1-9]\d*)(?:-(?<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?/gm;

const packageFileName = process.argv[2];
if (!packageFileName) {
	console.error("Usage: node scripts/version-check.ts <path-to-package-json>");
	process.exit(1);
}

const dir = process.env.GITHUB_WORKSPACE || "/github/workspace";
const eventFile =
	process.env.GITHUB_EVENT_PATH || "/github/workflow/event.json";

async function main() {
	const { commits } = readJson(eventFile);
	await processDirectory(dir, commits);
}

function readJson(file: string) {
	const data = readFileSync(file, { encoding: "utf8" });
	if (typeof data === "string") {
		return JSON.parse(data);
	}
}

async function processDirectory(
	dir: string,
	commits: Array<LocalCommit> | Array<PartialCommitResponse>,
) {
	try {
		const packageObj = readJson(join(dir, packageFileName));

		if (!isPackageObj(packageObj)) {
			throw new Error("Can't find version field");
		}

		if (commits.length >= 20) {
			console.warn(
				"This workflow run topped the commit limit set by GitHub webhooks: " +
					"that means that commits could not appear and that the run could " +
					"not find the version change.",
			);
		}

		if (commits.length <= 0) {
			console.warn("There are no commits to look at.");
			return;
		}

		await checkCommits(commits, packageObj.version);
	} catch (error) {
		console.error(`Error checking version: ${error}`);
	}
}

async function checkCommits(
	commits: Array<LocalCommit> | Array<PartialCommitResponse>,
	version: string,
) {
	try {
		log(
			`Checking the diffs of ${commits.length} commit${
				commits.length === 1 ? "" : "s"
			}...`,
		);

		if (!isLocalCommitArray(commits)) {
			commits.sort(
				(a, b) =>
					new Date(b.commit.committer.date).getTime() -
					new Date(a.commit.committer.date).getTime(),
			);
		}

		for (const commit of commits) {
			const { message, sha } = getBasicInfo(commit);

			if (await checkDiff(sha, version)) {
				log(
					`- Found match for version ${version}: ${sha.slice(0, 7)} - ${message}`,
				);
				return true;
			}
		}

		log("No matching commit found.");
		return false;
	} catch (error) {
		console.error(`Error checking version: ${error}`);
	}
}

function getBasicInfo(commit: LocalCommit | PartialCommitResponse) {
	let message: string;
	let sha: string;

	if (isLocalCommit(commit)) {
		message = commit.message;
		sha = commit.id;
	} else {
		message = commit.commit.message;
		sha = commit.sha;
	}

	return {
		message,
		sha,
	};
}

async function checkDiff(sha: string, version: string) {
	try {
		const commit = await getCommit(sha);
		const shortSha = sha.slice(0, 7);

		const pkg = commit.files.find((f) => f.filename === packageFileName);
		if (!pkg) {
			log(`- ${shortSha}: no changes to the package file`);
			return false;
		}

		const versionLines: {
			added?: string;
			deleted?: string;
		} = {};

		const rawLines = pkg.patch
			.split("\n")
			.filter(
				(line) => line.includes('"version":') && ["+", "-"].includes(line[0]),
			);
		if (rawLines.length > 2) {
			log(`- ${shortSha}: too many version lines`);
			return false;
		}

		for (const line of rawLines)
			versionLines[line.startsWith("+") ? "added" : "deleted"] = line;
		if (!versionLines.added) {
			log(`- ${shortSha}: no "+ version" line`);
			return false;
		}

		const versions = {
			added: parseVersionLine(versionLines.added),
			deleted: !!versionLines.deleted && parseVersionLine(versionLines.deleted),
		};
		if (versions.added !== version) {
			log(
				`- ${shortSha}: added version doesn't match current one (added: "${
					versions.added
				}"; current: "${version}")`,
			);
			return false;
		}

		log(`- ${shortSha}: match found, printing version to stdout`);
		console.log(version);
		return true;
	} catch (error) {
		console.error(`An error occurred in checkDiff:\n${error}`);
		throw new ExitError(1);
	}
}

async function getCommit(sha: string): Promise<CommitResponse> {
	const url = `https://api.github.com/repos/biomejs/biome/commits/${sha}`;
	const response = await fetch(url);
	if (!response.ok) {
		throw new Error(
			`Could not fetch commit: ${response.status}: ${response.statusText}`,
		);
	}

	return (await response.json()) as CommitResponse;
}

function parseVersionLine(str: string) {
	return (str.split('"') || []).map((s) => matchVersion(s)).find((e) => !!e);
}

function matchVersion(str: string): string {
	return (str.match(semverReGlobal) || ([] as string[]))[0];
}

class ExitError extends Error {
	code?: number;

	constructor(code: number | null) {
		super(`Command failed with code ${code}`);
		if (typeof code === "number") this.code = code;
	}
}

class NeutralExitError extends Error {}

log(`Searching for version update in ${packageFileName}...`);
main().catch((err) => {
	if (err instanceof NeutralExitError) {
		process.exit(78);
	} else {
		console.error(err.message || err);
		process.exit(1);
	}
});

// Use a custom log function to write to `stderr`. We don't want ordinary logs
// to appear on `stdout`, as that will be reserved to capture the version
// output.
function log(message: string) {
	process.stderr.write(`${message}\n`);
}

interface CommitResponse extends PartialCommitResponse {
	files: {
		filename: string;
		additions: number;
		deletions: number;
		changes: number;
		status: string;
		raw_url: string;
		blob_url: string;
		patch: string;
	}[];
}

interface LocalCommit {
	id: string;
	message: string;
	author: {
		name: string;
		email: string;
	};
	url: string;
	distinct: boolean;
}

function isLocalCommit(value: unknown): value is LocalCommit {
	return (
		!!value &&
		typeof value === "object" &&
		"id" in value &&
		typeof value.id === "string"
	);
}

function isLocalCommitArray(
	value: Array<unknown>,
): value is Array<LocalCommit> {
	return isLocalCommit(value[0]);
}

interface PackageObj {
	version: string;
}

function isPackageObj(value): value is PackageObj {
	return !!value && !!value.version;
}

interface PartialCommitResponse {
	url: string;
	sha: string;
	node_id: string;
	html_url: string;
	comments_url: string;
	commit: {
		url: string;
		author: {
			name: string;
			email: string;
			date: string;
		};
		committer: {
			name: string;
			email: string;
			date: string;
		};
		message: string;
		tree: {
			url: string;
			sha: string;
		};
		comment_count: number;
		verification: {
			verified: boolean;
			reason: string;
			signature: object | null;
			payload: object | null;
		};
	};
	author: {
		login: string;
		id: number;
		node_id: string;
		avatar_url: string;
		gravatar_id: string;
		url: string;
		html_url: string;
		followers_url: string;
		following_url: string;
		gists_url: string;
		starred_url: string;
		subscriptions_url: string;
		organizations_url: string;
		repos_url: string;
		events_url: string;
		receive_events_url: string;
		type: string;
		site_admin: boolean;
	};
	committer: {
		login: string;
		id: number;
		node_id: string;
		avatar_url: string;
		gravatar_id: string;
		url: string;
		html_url: string;
		followers_url: string;
		following_url: string;
		gists_url: string;
		starred_url: string;
		subscriptions_url: string;
		organizations_url: string;
		repos_url: string;
		events_url: string;
		receive_events_url: string;
		type: string;
		site_admin: boolean;
	};
	parents: {
		url: string;
		sha: string;
	}[];
	stats: {
		additions: number;
		deletions: number;
		total: number;
	};
}
