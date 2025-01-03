interface Socket {
	on(event: "data", fn: (data: Buffer) => void): void;
	write(data: Buffer): void;
	destroy(): void;
}

enum ReaderStateKind {
	Header = 0,
	Body = 1,
}

interface ReaderStateHeader {
	readonly kind: ReaderStateKind.Header;
	contentLength?: number;
	contentType?: string;
}

interface ReaderStateBody {
	readonly kind: ReaderStateKind.Body;
	readonly contentLength: number;
	readonly contentType?: string;
}

type ReaderState = ReaderStateHeader | ReaderStateBody;

interface JsonRpcRequest {
	jsonrpc: "2.0";
	id: number;
	method: string;
	params: unknown;
}

function isJsonRpcRequest(message: JsonRpcMessage): message is JsonRpcRequest {
	return (
		"id" in message &&
		typeof message.id === "number" &&
		"method" in message &&
		typeof message.method === "string" &&
		"params" in message
	);
}

interface JsonRpcNotification {
	jsonrpc: "2.0";
	method: string;
	params: unknown;
}

function isJsonRpcNotification(
	message: JsonRpcMessage,
): message is JsonRpcNotification {
	return (
		!("id" in message) &&
		"method" in message &&
		typeof message.method === "string" &&
		"params" in message
	);
}

type JsonRpcResponse =
	| {
			jsonrpc: "2.0";
			id: number;
			result: unknown;
	  }
	| {
			jsonrpc: "2.0";
			id: number;
			error: unknown;
	  };

function isJsonRpcResponse(
	message: JsonRpcMessage,
): message is JsonRpcResponse {
	return (
		"id" in message &&
		typeof message.id === "number" &&
		!("method" in message) &&
		("result" in message || "error" in message)
	);
}

type JsonRpcMessage = JsonRpcRequest | JsonRpcNotification | JsonRpcResponse;

function isJsonRpcMessage(message: unknown): message is JsonRpcMessage {
	return (
		typeof message === "object" &&
		message !== null &&
		"jsonrpc" in message &&
		message.jsonrpc === "2.0"
	);
}

interface PendingRequest {
	resolve(result: unknown): void;
	reject(error: unknown): void;
}

const MIME_JSONRPC = "application/vscode-jsonrpc";

/**
 * Implements the Biome daemon server JSON-RPC protocol over a Socket instance
 */
export class Transport {
	/**
	 * Counter incremented for each outgoing request to generate a unique ID
	 */
	private nextRequestId = 0;

	/**
	 * Storage for the promise resolver functions of pending requests,
	 * keyed by ID of the request
	 */
	private pendingRequests: Map<number, PendingRequest> = new Map();

	constructor(private socket: Socket) {
		socket.on("data", (data) => {
			this.processIncoming(data);
		});
	}

	/**
	 * Send a request to the remote server
	 *
	 * @param method Name of the remote method to call
	 * @param params Parameters object the remote method should be called with
	 * @return Promise resolving with the value returned by the remote method, or rejecting with an RPC error if the remote call failed
	 */
	// biome-ignore lint/suspicious/noExplicitAny: if i change it to Promise<unknown> typescript breaks
	request(method: string, params: unknown): Promise<any> {
		return new Promise((resolve, reject) => {
			const id = this.nextRequestId++;
			this.pendingRequests.set(id, { resolve, reject });
			this.sendMessage({
				jsonrpc: "2.0",
				id,
				method,
				params,
			});
		});
	}

	/**
	 * Send a notification message to the remote server
	 *
	 * @param method Name of the remote method to call
	 * @param params Parameters object the remote method should be called with
	 */
	notify(method: string, params: unknown) {
		this.sendMessage({
			jsonrpc: "2.0",
			method,
			params,
		});
	}

	/**
	 * Destroy the internal socket instance for this Transport
	 */
	destroy() {
		this.socket.destroy();
	}

	private sendMessage(message: JsonRpcMessage) {
		const body = Buffer.from(JSON.stringify(message));
		const headers = Buffer.from(
			`Content-Length: ${body.length}\r\nContent-Type: ${MIME_JSONRPC};charset=utf-8\r\n\r\n`,
		);
		this.socket.write(Buffer.concat([headers, body]));
	}

	private pendingData = Buffer.from("");
	private readerState: ReaderState = {
		kind: ReaderStateKind.Header,
	};

	private processIncoming(data: Buffer) {
		this.pendingData = Buffer.concat([this.pendingData, data]);

		while (this.pendingData.length > 0) {
			if (this.readerState.kind === ReaderStateKind.Header) {
				const lineBreakIndex = this.pendingData.indexOf("\n");
				if (lineBreakIndex < 0) {
					break;
				}

				const header = this.pendingData.subarray(0, lineBreakIndex + 1);
				this.pendingData = this.pendingData.subarray(lineBreakIndex + 1);
				this.processIncomingHeader(this.readerState, header.toString("utf-8"));
			} else if (this.pendingData.length >= this.readerState.contentLength) {
				const body = this.pendingData.subarray(
					0,
					this.readerState.contentLength,
				);
				this.pendingData = this.pendingData.subarray(
					this.readerState.contentLength,
				);
				this.processIncomingBody(body);

				this.readerState = {
					kind: ReaderStateKind.Header,
				};
			} else {
				break;
			}
		}
	}

	private processIncomingHeader(readerState: ReaderStateHeader, line: string) {
		if (line === "\r\n") {
			const { contentLength, contentType } = readerState;
			if (typeof contentLength !== "number") {
				throw new Error(
					"incoming message from the remote workspace is missing the Content-Length header",
				);
			}

			this.readerState = {
				kind: ReaderStateKind.Body,
				contentLength,
				contentType,
			};
			return;
		}

		const colonIndex = line.indexOf(":");
		if (colonIndex < 0) {
			throw new Error(`could not find colon token in "${line}"`);
		}

		const headerName = line.substring(0, colonIndex);
		const headerValue = line.substring(colonIndex + 1).trim();

		switch (headerName) {
			case "Content-Length": {
				const value = Number.parseInt(headerValue, 10);
				readerState.contentLength = value;
				break;
			}
			case "Content-Type": {
				if (!headerValue.startsWith(MIME_JSONRPC)) {
					throw new Error(
						`invalid value for Content-Type expected "${MIME_JSONRPC}", got "${headerValue}"`,
					);
				}

				readerState.contentType = headerValue;
				break;
			}
			default:
				console.warn(`ignoring unknown header "${headerName}"`);
		}
	}

	private processIncomingBody(buffer: Buffer) {
		const data = buffer.toString("utf-8");
		const body = JSON.parse(data);

		if (isJsonRpcMessage(body)) {
			if (isJsonRpcRequest(body)) {
				// TODO: Not implemented at the moment
				return;
			}

			if (isJsonRpcNotification(body)) {
				// TODO: Not implemented at the moment
				return;
			}

			if (isJsonRpcResponse(body)) {
				const pendingRequest = this.pendingRequests.get(body.id);
				if (pendingRequest) {
					this.pendingRequests.delete(body.id);
					const { resolve, reject } = pendingRequest;
					if ("result" in body) {
						resolve(body.result);
					} else {
						reject(body.error);
					}
				} else {
					throw new Error(
						`could not find any pending request matching RPC response ID ${body.id}`,
					);
				}
				return;
			}
		}

		throw new Error(
			`failed to deserialize incoming message from remote workspace, "${data}" is not a valid JSON-RPC message body`,
		);
	}
}
