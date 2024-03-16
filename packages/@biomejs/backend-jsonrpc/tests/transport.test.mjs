import { describe, expect, it, vi } from "vitest";

import { Transport } from "../dist/transport";

function makeMessage(body) {
	const content = JSON.stringify(body);
	return Buffer.from(
		`Content-Length: ${content.length}\r\nContent-Type: application/vscode-jsonrpc;charset=utf-8\r\n\r\n${content}`,
	);
}

describe("Transport Layer", () => {
	it("should encode requests into the socket", async () => {
		let onData = null;
		const socket = {
			on(event, fn) {
				expect(event).toBe("data");
				onData = fn;
			},
			write: vi.fn(),
			destroy: vi.fn(),
		};

		const transport = new Transport(socket);

		const result = transport.request("method", "params");

		expect(socket.write).toHaveBeenCalledWith(
			makeMessage({
				jsonrpc: "2.0",
				id: 0,
				method: "method",
				params: "params",
			}),
		);

		onData(
			makeMessage({
				jsonrpc: "2.0",
				id: 0,
				result: "result",
			}),
		);

		const response = await result;
		expect(response).toMatchObject({});

		transport.destroy();
		expect(socket.destroy).toHaveBeenCalledOnce();
	});

	it("should throw on missing Content-Length headers", async () => {
		let onData = null;
		const socket = {
			on(event, fn) {
				expect(event).toBe("data");
				onData = fn;
			},
			write: vi.fn(),
			destroy: vi.fn(),
		};

		const transport = new Transport(socket);

		expect(() => onData(Buffer.from("\r\n"))).toThrowError(
			"incoming message from the remote workspace is missing the Content-Length header",
		);

		transport.destroy();
		expect(socket.destroy).toHaveBeenCalledOnce();
	});

	it("should throw on missing colon token", async () => {
		let onData = null;
		const socket = {
			on(event, fn) {
				expect(event).toBe("data");
				onData = fn;
			},
			write: vi.fn(),
			destroy: vi.fn(),
		};

		const transport = new Transport(socket);

		expect(() => onData(Buffer.from("Content-Length\r\n"))).toThrowError(
			'could not find colon token in "Content-Length\r\n"',
		);

		transport.destroy();
		expect(socket.destroy).toHaveBeenCalledOnce();
	});

	it("should throw on invalid Content-Type", async () => {
		let onData = null;
		const socket = {
			on(event, fn) {
				expect(event).toBe("data");
				onData = fn;
			},
			write: vi.fn(),
			destroy: vi.fn(),
		};

		const transport = new Transport(socket);

		expect(() =>
			onData(Buffer.from("Content-Type: text/plain\r\n")),
		).toThrowError(
			'invalid value for Content-Type expected "application/vscode-jsonrpc", got "text/plain"',
		);

		transport.destroy();
		expect(socket.destroy).toHaveBeenCalledOnce();
	});

	it("should throw on unknown request ID", async () => {
		let onData = null;
		const socket = {
			on(event, fn) {
				expect(event).toBe("data");
				onData = fn;
			},
			write: vi.fn(),
			destroy: vi.fn(),
		};

		const transport = new Transport(socket);

		expect(() =>
			onData(makeMessage({ jsonrpc: "2.0", id: 0, result: "result" })),
		).toThrowError(
			"could not find any pending request matching RPC response ID 0",
		);

		transport.destroy();
		expect(socket.destroy).toHaveBeenCalledOnce();
	});

	it("should throw on invalid messages", async () => {
		let onData = null;
		const socket = {
			on(event, fn) {
				expect(event).toBe("data");
				onData = fn;
			},
			write: vi.fn(),
			destroy: vi.fn(),
		};

		const transport = new Transport(socket);

		expect(() => onData(makeMessage({}))).toThrowError(
			'failed to deserialize incoming message from remote workspace, "{}" is not a valid JSON-RPC message body',
		);

		transport.destroy();
		expect(socket.destroy).toHaveBeenCalledOnce();
	});
});
