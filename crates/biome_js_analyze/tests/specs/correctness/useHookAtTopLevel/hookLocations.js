// Invalid as hooks cannot be used at module level.
useHook();

// Invalid as hooks cannot be called in non-component functions.
function notAComponent() {
	useHook();
}

// Valid as hook is called in a component (by naming convention).
function AComponent() {
	useHook();
}

// Invalid as hooks cannot be called in non-hook functions.
function notUseMyHook() {
	useHook();
}

// Valid as hook is called in a hook (by naming convention).
function useMyHook() {
	useHook();
}

// Valid as hooks can be called within anonymous functions (for better or worse).
test("the hook", () => {
	useHook();
});
test("the hook", function () {
	useHook();
});

// Valid as hooks can be called within nested anonymous functions.
test("more hook", () => {
	renderHook(() => useHook());
});
