/* should not generate diagnostics */
expect(fn).toBeCalledWith();
expect(fn).toHaveBeenCalledWith();
expect(fn).toBeCalledWith(expect.anything());
expect(fn).toHaveBeenCalledWith(expect.anything());
expect(fn).toHaveBeenCalledExactlyOnceWith();
expect(fn).toBeCalledTimes(0);
expect(fn).toHaveBeenCalledTimes(0);
expect(fn).toHaveBeenLastCalledWith(1);
expect(fn).toHaveBeenNthCalledWith(1, 2);
expect(fn);

// `not` asserts that the function was never called
expect(fn).not.toBeCalled();
expect(fn).not.toHaveBeenCalled();
expect(fn).not.toHaveBeenCalledOnce();
expect(fn).rejects.not.toBeCalled();
expect(fn).resolves.not.toHaveBeenCalled();
expect(fn).not.toBeCalledWith();
expect(fn).not.toHaveBeenCalledWith();
expect(fn).resolves.not.toHaveBeenCalledWith();
expect(fn)["not"].toHaveBeenCalled();
expect.soft(fn).not.toHaveBeenCalled();

// The matcher isn't called
expect(fn).toHaveBeenCalled;
expect(fn).toBeCalled.name;

// Not an `expect()` assertion
toHaveBeenCalled();
fn.toHaveBeenCalled();
assert(fn).toHaveBeenCalled();
expect.toHaveBeenCalled();
expect.extend(fn).toHaveBeenCalled();
expect(fn).unknown.toHaveBeenCalled();
expect(fn)[modifier].toHaveBeenCalled();
expect(fn)[matcher]();
expect(fn)().toHaveBeenCalled();
wrapper.expect(fn).toHaveBeenCalled();
new expect(fn).toHaveBeenCalled();
