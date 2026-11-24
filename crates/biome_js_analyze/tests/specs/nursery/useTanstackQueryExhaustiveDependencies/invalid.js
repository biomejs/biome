/* should generate diagnostics */
import { skipToken, useQuery } from "@tanstack/react-query";

function ComputedMethod({ client, operation }) {
	useQuery({
		queryKey: ["data"],
		queryFn: () => client[operation](),
	});
}

const todoQueries = {
	list: () => ({ queryKey: ["entity"], queryFn: fetchEntities }),
	detail: (id) => ({ queryKey: ["entity"], queryFn: () => fetchEntity(id) }),
};

function NoDependencies() {
	const id = 1;
	useQuery({ queryKey: ["entity"], queryFn: () => api.getEntity(id) });
}

function TemplateLiteral() {
	const a = 1;
	const b = 2;
	useQuery({ queryKey: [`entity/${a}`], queryFn: () => api.getEntity(a, b) });
}

function StringThatLooksLikeTemplate() {
	const id = 1;
	useQuery({ queryKey: ["entity/${id}"], queryFn: () => api.getEntity(id) });
}

function StateSetter() {
	const [id] = React.useState(1);
	useQuery({
		queryKey: ["entity"],
		queryFn: () => {
			const { data } = axios.get(`.../${id}`);
			return data;
		},
	});
}

const complexKeys = {
	key: (a, b, c, d, e) => ({
		queryKey: ["entity", a, [b], { c }, 1, true],
		queryFn: () => api.getEntity(a, b, c, d, e),
	}),
	other: (dep1, dep2, dep3, dep4, dep5, dep6, dep7, dep8) => ({
		queryKey: ["foo", { dep1, dep2: dep2, bar: dep3, baz: [dep4, dep5] }, [dep6, dep7]],
		queryFn: () => api.getEntity(dep1, dep2, dep3, dep4, dep5, dep6, dep7, dep8),
	}),
};

function ComputedMember({ map, key }) {
	useQuery({ queryKey: ["key"], queryFn: () => api.get(map[key]) });
}

function KeyVariable() {
	const x = 5;
	const queryKey = ["foo"];
	useQuery({ queryKey, queryFn: () => x });
}

function KeyFactory(num, flag) {
	useQuery({ queryKey: keys.foo(), queryFn: () => Promise.resolve(num) });
	useQuery({
		queryKey: keys.foo(1).detail(flag),
		queryFn: () => Promise.resolve({ num, flag }),
	});
}

function PartialObject() {
	const state = { foo: "foo", bar: "bar" };
	useQuery({
		queryKey: ["state", state.foo],
		queryFn: () => Promise.resolve({ foo: state.foo, bar: state.bar }),
	});
}

function MethodSyntax() {
	const id = 1;
	useQuery({
		queryKey: [],
		queryFn() {
			return Promise.resolve(id);
		},
	});
}

function SkipToken({ id }, condition) {
	useQuery({
		queryKey: [],
		queryFn: condition ? () => Promise.resolve(id) : skipToken,
	});
}

function Callbacks() {
	const id = 1;
	useQuery({
		queryKey: ["foo"],
		queryFn: () =>
			Promise.resolve(null)
				.then(() => id)
				.catch(() => id),
	});
}

function ShadowedInKey(id, ids) {
	useQuery({
		queryKey: ["thing", ids.map((id) => id)],
		queryFn: () => id,
	});
}

function useTrailingComma(dep) {
	return useQuery({
		queryKey: ["thing",],
		queryFn: () => dep,
	});
}

function useMultiline(dep, other) {
	return useQuery({
		queryKey: [
			"thing",
			other, // the other value
		],
		queryFn: () => fetchThing(dep, other),
	});
}

function useEmptyKey(dep) {
	return useQuery({
		queryKey: [ ],
		queryFn: () => dep,
	});
}

function useTernary(condition, a, b) {
	useQuery({
		queryKey: ["thing", a],
		queryFn: condition ? () => fetchA(a) : () => fetchB(b),
	});
	useQuery({
		queryKey: ["thing", b],
		queryFn: condition ? () => fetchA(a) : () => fetchB(b),
	});
}

function NestedFunction(items) {
	return items.map((item) =>
		useQuery({ queryKey: ["item"], queryFn: () => fetchItem(item.id) }),
	);
}

function OptionalChain(data) {
	useQuery({ queryKey: ["query"], queryFn: () => sendQuery(data?.address) });
}

function WholeVariableAndProperty(todo) {
	useQuery({ queryKey: ["todo"], queryFn: () => fetchTodo(todo, todo.id) });
}

function Shorthand(id) {
	useQuery({ queryKey: ["todo"], queryFn: () => fetchTodo({ id }) });
}

const moduleKey = ["todo"];

function KeyDeclaredOutside(id) {
	// The key is declared where `id` doesn't exist, so no fix is offered.
	useQuery({ queryKey: moduleKey, queryFn: () => fetchTodo(id) });
}

function KeyWithLineComment(id) {
	// A fix would put `id` inside the comment, so no fix is offered.
	useQuery({
		queryKey: [
			"todo" // the todo
		],
		queryFn: () => fetchTodo(id),
	});
}

function LocalIndex({ cursors, byId, ids }) {
	// `pageParam` and `i` belong to the query function, so only the objects are missing.
	useInfiniteQuery({
		queryKey: ["page"],
		queryFn: ({ pageParam }) => fetchPage(cursors[pageParam]),
	});
	useQuery({
		queryKey: ["items", ids],
		queryFn: () => ids.map((i) => byId[i]),
	});
}

function DeclaredAfterQuery() {
	// The key is evaluated before `id` is declared, so no fix is offered.
	const query = useQuery({ queryKey: ["todo"], queryFn: () => fetchTodo(id) });
	const id = 1;
	return query;
}
