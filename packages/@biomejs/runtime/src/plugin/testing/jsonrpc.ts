import assert from "node:assert/strict";
import { setTimeout as sleep } from "node:timers/promises";

import type { Workspace } from "@biomejs/backend-jsonrpc";

export interface WorkspaceConnection {
	/**
	 * Runs `call` against the lazily connected workspace.
	 *
	 * Connecting and `call` are each bounded by the timeout. An error response
	 * from the server rejects with {@link WorkspaceRpcError} and keeps the
	 * connection usable; any other failure disposes it, so later requests reject.
	 */
	request<T>(call: (workspace: Workspace) => Promise<T>): Promise<T>;

	dispose(): void;
}

/** An error response returned by the Biome daemon for one request. */
export class WorkspaceRpcError extends Error {
	readonly code: number;
	readonly data: unknown;

	constructor(error: { code: number; message: string; data?: unknown }) {
		super(error.message);

		this.name = "WorkspaceRpcError";
		this.code = error.code;
		this.data = error.data;
	}
}

/** The LSP `ContentModified` error code: the server cancelled the request for a write. */
const CONTENT_MODIFIED = -32801;

/** `Transport` rejects with the raw JSON-RPC error object, not an `Error`. */
function isRpcError(
	error: unknown,
): error is { code: number; message: string; data?: unknown } {
	return (
		typeof error === "object" &&
		error !== null &&
		!(error instanceof Error) &&
		typeof (error as { code?: unknown }).code === "number" &&
		typeof (error as { message?: unknown }).message === "string"
	);
}

/** Connects with `connect` on the first request; `null` means no daemon is available. */
export function createWorkspaceConnection(
	connect: () => Promise<Workspace | null>,
	timeout = 30_000,
): WorkspaceConnection {
	assert(
		Number.isSafeInteger(timeout) && timeout > 0,
		"timeout must be a positive integer",
	);

	const controller = new AbortController();

	let connection: Promise<Workspace> | undefined;
	let workspace: Workspace | undefined;

	const disposed = () => new Error("Workspace connection is disposed");
	const dispose = () => {
		controller.abort();
		workspace?.destroy?.();
		workspace = undefined;
	};

	/** Rejects `promise` on timeout or disposal, whichever comes first. */
	const bounded = async <T>(promise: Promise<T>): Promise<T> => {
		if (controller.signal.aborted) {
			throw disposed();
		}

		let reject!: (error: Error) => void;
		const limit = new Promise<never>((_, rejectLimit) => {
			reject = rejectLimit;
		});

		const timer = setTimeout(
			() => reject(new Error(`Workspace RPC timed out after ${timeout}ms`)),
			timeout,
		);

		const abort = () => reject(disposed());

		controller.signal.addEventListener("abort", abort, { once: true });

		try {
			return await Promise.race([promise, limit]);
		} finally {
			clearTimeout(timer);
			controller.signal.removeEventListener("abort", abort);
		}
	};

	const open = async (): Promise<Workspace> => {
		const client = await connect();
		if (!client) {
			throw new Error("Could not connect to the Biome daemon");
		}

		workspace = client;

		// A connection that resolves after disposal must still be closed.
		if (controller.signal.aborted) {
			dispose();
			throw disposed();
		}

		return client;
	};

	return {
		async request(call) {
			if (controller.signal.aborted) {
				throw disposed();
			}

			try {
				connection ??= open();

				const client = await bounded(connection);

				// `call` is attempted again when the server cancels it for a concurrent write, like the `WorkspaceClient`.
				for (;;) {
					try {
						return await bounded(call(client));
					} catch (error) {
						if (!isRpcError(error) || error.code !== CONTENT_MODIFIED) {
							throw error;
						}

						await sleep(1);
					}
				}
			} catch (error) {
				if (isRpcError(error)) {
					throw new WorkspaceRpcError(error);
				}

				dispose();

				throw error;
			}
		},
		dispose,
	};
}
