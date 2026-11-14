/* should not generate diagnostics */
import { queryOptions, skipToken, useQuery } from "@tanstack/react-query";

useQuery({ queryKey: ["todos"], queryFn: fetchTodos });
useQuery({ queryKey: ["entity", id], queryFn: () => api.getEntity(id) });
useQuery({ queryKey: [`entity/${id}`], queryFn: () => api.getEntity(id) });

const CONST_VAL = 1;

function ModuleConstant() {
	useQuery({ queryKey: ["foo"], queryFn: () => CONST_VAL });
}

const ModuleConstantArrow = () => {
	useQuery({ queryKey: ["foo"], queryFn: () => CONST_VAL });
};

export const queries = {
	foo: queryOptions({
		queryKey: ["foo"],
		queryFn: () => Promise.resolve(CONST_VAL),
	}),
};

const useFoo = () => {
	const api = useApi();
	return useQuery({ queryKey: ["foo", api], queryFn: () => api.fetchFoo() });
};

function CalledFunction({ todoId }) {
	const fetchTodoById = (id) => Promise.resolve(id);
	return useQuery({
		queryKey: ["todos", todoId],
		queryFn: () => fetchTodoById(todoId),
	});
}

function MethodReceiver({ todoId }) {
	const todos = useTodos();
	return useQuery({
		queryKey: ["todo", todoId],
		queryFn: () => todos.getTodo(todoId),
	});
}

function DataMethodReceiver({ items }) {
	useQuery({
		queryKey: ["items"],
		queryFn: () => items?.map((item) => item.id),
	});
}

function MemberPaths(props) {
	useQuery({ queryKey: ["entity", props.src], queryFn: () => api.get(props.src) });
	useQuery({ queryKey: ["entity", !!props.id], queryFn: () => api.get(props.id) });
	useQuery({ queryKey: ["entity", props?.id], queryFn: () => api.get(props?.id) });
	useQuery({ queryKey: ["entity", props?.id], queryFn: () => api.get(props.id) });
}

function KeyFromCallback() {
	useQuery({
		queryKey: ["foo", dep1],
		queryFn: ({ queryKey: [, dep] }) => fetch(dep),
	});
}

function useRest(arg, ...args) {
	return useQuery({
		queryKey: ["foo", arg, ...args],
		queryFn: async () => foo([arg, ...args]),
	});
}

function NewAndInstanceof({ value }) {
	class SomeClass {}
	useQuery({ queryKey: ["foo"], queryFn: async () => new SomeClass() });
	useQuery({
		queryKey: ["foo", value],
		queryFn: () => value instanceof SomeClass,
	});
	useQuery({ queryKey: ["foo"], queryFn: () => Promise.reject(new Error("1")) });
}

function Undefined() {
	useQuery({
		queryKey: [],
		queryFn: async () => {
			if (undefined) {
				return null;
			}
			return 1;
		},
	});
}

function InternalDependency() {
	useQuery({
		queryKey: ["api"],
		queryFn: async () => {
			const response = Promise.resolve([]);
			const data = await response.json();
			return data[0].name;
		},
	});
}

function WholeObject() {
	const state = { foo: "foo", bar: "bar" };
	useQuery({
		queryKey: ["state", state],
		queryFn: () => Promise.resolve({ foo: state.foo, bar: state.bar }),
	});
}

function KeyFactory({ id }, num, obj, flag) {
	useQuery({ queryKey: factory(id ?? -1), queryFn: () => Promise.resolve({ id }) });
	useQuery({ queryKey: keys.foo(num), queryFn: () => Promise.resolve(num) });
	useQuery({ queryKey: keys.foo({ x: num }), queryFn: () => Promise.resolve(num) });
	useQuery({ queryKey: keys.foo({ num }), queryFn: () => Promise.resolve(num) });
	useQuery({ queryKey: keys.foo([num]), queryFn: () => Promise.resolve(num) });
	useQuery({ queryKey: keys.foo(1, num), queryFn: () => Promise.resolve(num) });
	useQuery({ queryKey: keys.foo(obj.num), queryFn: () => Promise.resolve(obj.num) });
	useQuery({
		queryKey: keys.foo(num).detail(flag),
		queryFn: () => Promise.resolve({ num, flag }),
	});
}

function KeyVariable(dep) {
	const queryKey = keyFactory(dep);
	useQuery({ queryKey, queryFn: () => Promise.resolve(dep) });

	const key = ["thing", dep];
	useQuery({ queryKey: key, queryFn: () => dep });
}

function KeyExpressions({ cond, a, b }, ids, prefix, dep1, dep2) {
	useQuery({ queryKey: ["thing", cond ? a : b], queryFn: () => (cond ? a : b) });
	useQuery({ queryKey: ["thing", a + b], queryFn: () => a + b });
	useQuery({
		queryKey: ["thing", ids.map((id) => prefix + "-" + id)],
		queryFn: () => ({ ids, prefix }),
	});
	useQuery({
		queryKey: ["thing", { ...dep1, prop: dep2 }],
		queryFn: () => dep1.prop + dep2,
	});
}

function KeyFunctions() {
	const obj = reactive({});
	useQuery({
		queryKey: ["foo", () => obj.boo],
		queryFn: () => fetch(`/mock/getSomething/${obj.boo}`),
	});
	useQuery({
		queryKey: [
			"foo",
			function () {
				return obj.boo;
			},
		],
		queryFn: () => fetch(`/mock/getSomething/${obj.boo}`),
	});
}

function SkipToken({ id }, condition) {
	useQuery({
		queryKey: [id],
		queryFn: condition ? () => Promise.resolve(id) : skipToken,
	});
	useQuery({
		queryKey: [id],
		queryFn: condition ? skipToken : () => Promise.resolve(id),
	});
}

function NullishCoalescing(options) {
	useQuery({ queryKey: ["foo", options], queryFn: () => options?.params ?? options });
}

function Callbacks() {
	const id = 1;
	useQuery({
		queryKey: ["foo", id],
		queryFn: () =>
			Promise.resolve(null)
				.then(() => id)
				.catch(() => id),
	});
	useQuery({
		queryKey: ["foo", id],
		queryFn: () => {
			try {
				return fetch(id);
			} catch (error) {
				console.error(error);
				return id;
			} finally {
				console.log("done");
			}
		},
	});
}

function useMethodCalls(a) {
	useQuery({
		queryKey: ["thing", a],
		queryFn: () => {
			a.b.foo();
			a.c.bar();
			return 1;
		},
	});
	useQuery({
		queryKey: ["thing", a.b, a.c],
		queryFn: () => {
			a.b.foo();
			a.c.bar();
			return 1;
		},
	});
	useQuery({ queryKey: ["thing"], queryFn: () => a?.foo() });
	useQuery({
		queryKey: ["thing"],
		queryFn: () => {
			a.b.foo();
			return 1;
		},
	});
}

function useKeyCalls(api, dep, obj) {
	const makeKeyPart = (value) => value;
	useQuery({ queryKey: ["thing", api.createKey()], queryFn: () => api.fetch() });
	useQuery({ queryKey: ["thing", makeKeyPart(dep)], queryFn: () => makeKeyPart(dep) });
	useQuery({
		queryKey: ["thing", obj.api.createKey()],
		queryFn: () => obj.api.fetch(),
	});
}

function Ternary(condition, a, b) {
	useQuery({
		queryKey: ["thing", a, b],
		queryFn: condition ? () => fetchA(a) : () => fetchB(b),
	});
}

function NestedMethodReceiver(props) {
	const entities = props.entities;
	useQuery({
		queryKey: ["get-stuff"],
		queryFn: () => api.fetchStuff({ ids: entities.map((o) => o.id) }),
	});
}

function NotAQueryFunction(id) {
	// The query function is not written inline, so the rule can't check it.
	useQuery({ queryKey: ["todo"], queryFn: fetchTodo });
	// Without a query key, there is nothing to compare.
	useQuery({ queryFn: () => fetchTodo(id) });
}

function LocalIndex({ cursors }) {
	useInfiniteQuery({
		queryKey: ["page", cursors],
		queryFn: ({ pageParam }) => fetchPage(cursors[pageParam]),
	});
}
