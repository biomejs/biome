/* should generate diagnostics */
obj.a = jest.fn(); const test = 10;
Date['now'] = jest['fn']();
window[`${name}`] = jest[`fn`]();
obj['prop' + 1] = jest['fn']();
obj.one.two = jest.fn();
obj.a = jest.fn(() => 10,);
obj.a.b = jest.fn(() => ({})).mockReturnValue('default').mockReturnValueOnce('first call'); test();
window.fetch = jest.fn(() => ({})).one.two().three().four;
foo[bar] = jest.fn().mockReturnValue(undefined);
foo.bar = jest.fn().mockImplementation(baz => baz);
foo.bar = jest.fn(a => b).mockImplementation(baz => baz);
Date.now = vi.fn();
Date.now = vi.fn(() => 10);
window.fetch = vi.fn().mockResolvedValue(response);
obj.a = (jest.fn());
obj.a = (jest.fn()).mockReturnValue(1);
const value = (obj.a = jest.fn());
obj.a = jest.fn(...implementations);
obj.a = jest
	.fn()
	.mockReturnValue(1);
// leading comment
obj.a = jest.fn(); // trailing comment
obj.a = /* inner comment */ jest.fn();
super.method = jest.fn();
obj[a, b] = jest.fn();
(obj.a) = jest.fn();
((obj).a) = vi.fn();
