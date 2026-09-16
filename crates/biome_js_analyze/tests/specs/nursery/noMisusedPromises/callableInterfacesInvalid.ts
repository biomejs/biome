// should generate diagnostics
interface Handler {
	(): void;
}

interface AsyncHandler {
	(): Promise<void>;
}

interface HandlerDerived extends Handler {}
interface AsyncHandlerDerived extends AsyncHandler {}

type AsyncObject = {
	(): Promise<void>;
};

declare function consumeHandler(callback: Handler): void;
declare function consumeInline(callback: () => void): void;
declare function consumeInherited(callback: HandlerDerived): void;
declare const asyncHandler: AsyncHandler;
declare const inheritedAsyncHandler: AsyncHandlerDerived;
declare const asyncObject: AsyncObject;

consumeHandler(async () => {});
consumeInline(asyncHandler);
consumeHandler(asyncHandler);
consumeInherited(inheritedAsyncHandler);
consumeInline(asyncObject);
