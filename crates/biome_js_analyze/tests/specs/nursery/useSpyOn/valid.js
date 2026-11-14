/* should not generate diagnostics */
Date.now = () => 10;
window.fetch = jest.fn;
Date.now = fn();
obj.mock = jest.something();
obj.mock = jest.spyOn(other, "method");
const mock = jest.fn();
mock = jest.fn();
const mockObj = { mock: jest.fn() };
mockObj = { mock: jest.fn() };
window[`${name}`] = jest[`fn${expression}`]();
obj.mock = mock.fn();
obj.mock = jest.fn()();
obj.mock ??= jest.fn();
obj.mock ||= vi.fn();
class Foo {
	#method;
	constructor() {
		this.#method = jest.fn();
	}
}
jest.spyOn(Date, "now");
jest.spyOn(Date, "now").mockImplementation(() => 10);
vi.spyOn(Date, "now").mockReturnValue(10);
