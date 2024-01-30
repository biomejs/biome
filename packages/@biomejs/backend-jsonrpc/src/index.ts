import { getCommand } from "./command";
import { createSocket } from "./socket";
import { Transport } from "./transport";
import { type Workspace, createWorkspace as wrapTransport } from "./workspace";

/**
 * Create an instance of the Workspace client connected to a remote daemon
 * instance through the JSON-RPC protocol
 *
 * @returns A Workspace client, or null if the underlying platform is not supported
 */
export async function createWorkspace(): Promise<Workspace | null> {
	const command = getCommand();
	if (!command) {
		return null;
	}

	return createWorkspaceWithBinary(command);
}

/**
 * Create an instance of the Workspace client connected to a remote daemon
 * instance through the JSON-RPC protocol, using the provided command to spawn
 * the daemon if necessary
 *
 * @param command Path to the Biome binary distribution
 * @returns A Workspace client, or null if the underlying platform is not supported
 */
export async function createWorkspaceWithBinary(
	command: string,
): Promise<Workspace> {
	const socket = await createSocket(command);
	const transport = new Transport(socket);

	await transport.request("initialize", {
		capabilities: {},
		client_info: {
			name: "@biomejs/backend-jsonrpc",
			version: "0.10.1-next",
		},
	});

	return wrapTransport(transport);
}

export * from "./workspace";
