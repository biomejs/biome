/* should generate diagnostics */

// Optional chaining on a method that returns Promise — the call itself is not optional
type AsyncApi = {
	fetchData: () => Promise<string>;
};
async function callAsyncApi(api: AsyncApi) {
	api.fetchData();
}

// Non-optional call on async method with optional chaining elsewhere
type Service = {
	client?: {
		request: () => Promise<void>;
	};
};
async function callService(svc: Service) {
	// The request() call is NOT optional — if client exists, this is a floating Promise
	svc.client?.request();
}

// Record with promise-returning values — optional chaining doesn't suppress the diagnostic
type PromiseHandlers = Record<string, (() => Promise<void>) | undefined>;
const promiseHandlers: PromiseHandlers = {};
promiseHandlers?.["someHandler"]?.();

// Record with unknown-returning values combined with Promise.reject — unknown could be a Promise
const optionalObject: Record<string, (() => unknown) | undefined> = {};
optionalObject?.nonExistentMethod?.() ||
	Promise.reject("optional chaining bypass");
